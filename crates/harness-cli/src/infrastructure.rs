use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, types::ValueRef, Connection, OptionalExtension};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::application::{
    BacklogAddInput, BacklogCloseInput, BrownfieldImportResult, ContextPackData, DecisionAddInput,
    DecisionVerifyResult, HarnessContext, InitResult, IntakeInput, MigrateResult, QueryTable,
    ReleaseVerifyInput, StoryAddInput, StoryUpdateInput, StoryVerifyResult, TraceInput,
};
use crate::domain::{
    normalize_token, score_trace, ArchitectureCheckResult, ArchitectureConfig,
    ArchitectureViolation, BacklogFilter, BacklogRecord, DecisionRecord, FrictionRecord,
    HarnessStats, IntakeRecord, ReleaseAssetEvidence, ReleaseCheckResult, ReleaseConfig,
    ReleaseVerificationReport, RiskLane, StoryGateResult, StoryMatrixRecord, StoryVerifyStatus,
    TraceRecord, TraceScoreResult, TraceScoreSource,
};

pub type Result<T> = std::result::Result<T, HarnessInfraError>;
type ContextIntakeRow = (
    String,
    String,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
);

#[derive(Debug, Error)]
pub enum HarnessInfraError {
    #[error("database not found at {0}. Run: harness init")]
    MissingDatabase(String),
    #[error("schema file missing: {0}")]
    MissingSchema(String),
    #[error("brownfield import: missing {0}")]
    MissingBrownfieldPath(String),
    #[error("decision {0} has no verify_command. Configure one with: harness-cli decision add --id {0} --title <title> --verify \"<command>\"")]
    MissingDecisionVerifyCommand(String),
    #[error("story {0} has no verify_command. Configure one with: harness-cli story update --id {0} --verify \"<command>\"")]
    MissingStoryVerifyCommand(String),
    #[error("story update: story '{0}' not found")]
    StoryNotFound(String),
    #[error("architecture config missing: {0}")]
    MissingArchitectureConfig(String),
    #[error("architecture config is invalid: {0}")]
    InvalidArchitectureConfig(String),
    #[error("release config missing: {0}")]
    MissingReleaseConfig(String),
    #[error("release config is invalid: {0}")]
    InvalidReleaseConfig(String),
    #[error("release verification input is invalid: {0}")]
    InvalidReleaseInput(String),
    #[error("backlog close: backlog item '{0}' not found")]
    BacklogNotFound(i64),
    #[error("trace '{0}' not found")]
    TraceNotFound(i64),
    #[error("no traces found")]
    NoTraces,
    #[error("story update: nothing to update")]
    EmptyStoryUpdate,
    #[error("sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

pub trait HarnessRepository {
    fn init(&self) -> Result<InitResult>;
    fn migrate(&self) -> Result<MigrateResult>;
    fn import_brownfield(&self) -> Result<BrownfieldImportResult>;
    fn record_intake(&self, input: IntakeInput) -> Result<i64>;
    fn add_story(&self, input: StoryAddInput) -> Result<()>;
    fn update_story(&self, input: StoryUpdateInput) -> Result<()>;
    fn verify_story(&self, id: &str) -> Result<StoryVerifyResult>;
    fn verify_story_gate(&self, id: &str) -> Result<StoryGateResult>;
    fn verify_release(
        &self,
        input: ReleaseVerifyInput,
    ) -> Result<(PathBuf, ReleaseVerificationReport)>;
    fn check_architecture(
        &self,
        config_path: Option<PathBuf>,
        story_id: Option<&str>,
    ) -> Result<ArchitectureCheckResult>;
    fn add_decision(&self, input: DecisionAddInput) -> Result<()>;
    fn verify_decision(&self, id: &str) -> Result<DecisionVerifyResult>;
    fn add_backlog(&self, input: BacklogAddInput) -> Result<i64>;
    fn close_backlog(&self, input: BacklogCloseInput) -> Result<()>;
    fn record_trace(&self, input: TraceInput) -> Result<i64>;
    fn score_trace(&self, id: Option<i64>) -> Result<TraceScoreResult>;
    fn story_verify_status(&self, id: &str) -> Result<StoryVerifyStatus>;
    fn query_matrix(&self) -> Result<Vec<StoryMatrixRecord>>;
    fn query_backlog(&self, filter: BacklogFilter) -> Result<Vec<BacklogRecord>>;
    fn query_decisions(&self) -> Result<Vec<DecisionRecord>>;
    fn query_intakes(&self) -> Result<Vec<IntakeRecord>>;
    fn query_traces(&self) -> Result<Vec<TraceRecord>>;
    fn query_friction(&self) -> Result<Vec<FrictionRecord>>;
    fn query_stats(&self) -> Result<HarnessStats>;
    fn query_sql(&self, sql: &str) -> Result<QueryTable>;
    fn get_context_pack_data(&self, story_id: &str) -> Result<ContextPackData>;
    fn update_story_context_pack_path(&self, id: &str, path: &str) -> Result<()>;
    fn repo_root(&self) -> PathBuf;
}

#[derive(Debug)]
pub struct SqliteHarnessRepository {
    repo_root: PathBuf,
    db_path: PathBuf,
    schema_dir: PathBuf,
}

impl SqliteHarnessRepository {
    pub fn new(repo_root: PathBuf, db_path: PathBuf, schema_dir: PathBuf) -> Self {
        Self {
            repo_root,
            db_path,
            schema_dir,
        }
    }

    fn open_existing(&self) -> Result<Connection> {
        if !self.db_path.exists() {
            return Err(HarnessInfraError::MissingDatabase(
                self.db_path.display().to_string(),
            ));
        }

        let connection = Connection::open(&self.db_path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        Ok(connection)
    }

    fn open_or_create(&self) -> Result<Connection> {
        let connection = Connection::open(&self.db_path)?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        Ok(connection)
    }

    fn schema_version(connection: &Connection) -> Result<i64> {
        let version = connection
            .query_row(
                "SELECT COALESCE(MAX(version),0) FROM schema_version;",
                [],
                |row| row.get::<_, i64>(0),
            )
            .optional()?
            .unwrap_or(0);
        Ok(version)
    }

    fn apply_schema_v1(&self, connection: &Connection) -> Result<()> {
        let schema_path = self.schema_dir.join("001-init.sql");
        if !schema_path.exists() {
            return Err(HarnessInfraError::MissingSchema(
                schema_path.display().to_string(),
            ));
        }

        let schema = fs::read_to_string(schema_path)?;
        connection.execute_batch(&schema)?;
        Ok(())
    }

    fn apply_pending_migrations(
        &self,
        connection: &Connection,
        current_version: i64,
    ) -> Result<Vec<i64>> {
        let mut applied = Vec::new();
        for (version, path) in self.migration_files()? {
            if version > current_version {
                let sql = fs::read_to_string(path)?;
                connection.execute_batch(&sql)?;
                applied.push(version);
            }
        }
        Ok(applied)
    }

    fn migration_files(&self) -> Result<Vec<(i64, PathBuf)>> {
        let mut files = Vec::new();
        for entry in fs::read_dir(&self.schema_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("sql") {
                continue;
            }
            let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
                continue;
            };
            let Some(prefix) = file_name.split('-').next() else {
                continue;
            };
            let Ok(version) = prefix.trim_start_matches('0').parse::<i64>() else {
                continue;
            };
            files.push((version, path));
        }
        files.sort_by_key(|(version, _)| *version);
        Ok(files)
    }

    fn import_matrix(&self, connection: &Connection) -> Result<usize> {
        let matrix_path = self.repo_root.join("docs/TEST_MATRIX.md");
        if !matrix_path.exists() {
            return Err(HarnessInfraError::MissingBrownfieldPath(
                matrix_path.display().to_string(),
            ));
        }

        let content = fs::read_to_string(matrix_path)?;
        let mut story_count = 0;
        let mut columns: Option<MatrixColumns> = None;

        for line in content.lines() {
            if !line.trim_start().starts_with('|') {
                continue;
            }

            let fields = markdown_table_fields(line);
            if fields.len() < 2 {
                continue;
            }

            if columns.is_none() {
                let candidate = MatrixColumns::from_header(&fields);
                if candidate.story.is_some() && candidate.status.is_some() {
                    columns = Some(candidate);
                }
                continue;
            }

            let columns = columns.as_ref().expect("matrix columns discovered");
            let id = field_at(&fields, columns.story).unwrap_or_default();
            let token = normalize_token(&id);
            if matches!(
                token.as_str(),
                "" | "story" | "tbd" | "todo" | "example" | "examples"
            ) || id.chars().all(|character| character == '-')
            {
                continue;
            }

            let mut title = field_at(&fields, columns.contract).unwrap_or_else(|| id.clone());
            if title.is_empty() {
                title = id.clone();
            }

            let status =
                normalize_story_status(&field_at(&fields, columns.status).unwrap_or_default());
            let unit = proof_from_cell(&field_at(&fields, columns.unit).unwrap_or_default());
            let integration =
                proof_from_cell(&field_at(&fields, columns.integration).unwrap_or_default());
            let e2e = proof_from_cell(&field_at(&fields, columns.e2e).unwrap_or_default());
            let platform =
                proof_from_cell(&field_at(&fields, columns.platform).unwrap_or_default());
            let evidence = columns
                .evidence
                .and_then(|index| evidence_from_fields(&fields, index));

            connection.execute(
                "INSERT INTO story (
                    id, title, risk_lane, contract_doc, status,
                    unit_proof, integration_proof, e2e_proof, platform_proof,
                    evidence, notes
                 ) VALUES (?1, ?2, 'high_risk', ?3, ?4, ?5, ?6, ?7, ?8, ?9,
                    'Imported from docs/TEST_MATRIX.md by harness import brownfield.'
                 )
                 ON CONFLICT(id) DO UPDATE SET
                    title=excluded.title,
                    contract_doc=excluded.contract_doc,
                    status=excluded.status,
                    unit_proof=excluded.unit_proof,
                    integration_proof=excluded.integration_proof,
                    e2e_proof=excluded.e2e_proof,
                    platform_proof=excluded.platform_proof,
                    evidence=excluded.evidence,
                    notes=excluded.notes;",
                params![
                    id,
                    title,
                    field_at(&fields, columns.contract),
                    status,
                    unit,
                    integration,
                    e2e,
                    platform,
                    evidence,
                ],
            )?;
            story_count += 1;
        }

        Ok(story_count)
    }

    fn import_decisions(&self, connection: &Connection) -> Result<usize> {
        let decisions_dir = self.repo_root.join("docs/decisions");
        if !decisions_dir.is_dir() {
            return Err(HarnessInfraError::MissingBrownfieldPath(
                decisions_dir.display().to_string(),
            ));
        }

        let mut files = Vec::new();
        for entry in fs::read_dir(&decisions_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|value| value.to_str()) != Some("md") {
                continue;
            }
            let Some(file_name) = path.file_name().and_then(|value| value.to_str()) else {
                continue;
            };
            if is_decision_file_name(file_name) {
                files.push(path);
            }
        }
        files.sort();

        let mut decision_count = 0;
        for path in files {
            let content = fs::read_to_string(&path)?;
            let stem = path
                .file_stem()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_owned();
            let title = content
                .lines()
                .next()
                .and_then(|line| line.strip_prefix("# "))
                .map(str::trim)
                .filter(|value| !value.is_empty())
                .unwrap_or(&stem)
                .to_owned();
            let status =
                normalize_decision_status(&markdown_section_first_value(&content, "Status"));
            let doc_path = format!(
                "docs/decisions/{}",
                path.file_name()
                    .and_then(|value| value.to_str())
                    .unwrap_or_default()
            );

            connection.execute(
                "INSERT INTO decision (id, title, status, doc_path, notes)
                 VALUES (?1, ?2, ?3, ?4,
                    'Imported from docs/decisions by harness import brownfield.'
                 )
                 ON CONFLICT(id) DO UPDATE SET
                    title=excluded.title,
                    status=excluded.status,
                    doc_path=excluded.doc_path,
                    notes=excluded.notes;",
                params![stem, title, status, doc_path],
            )?;
            decision_count += 1;
        }

        Ok(decision_count)
    }

    fn import_backlog(&self, connection: &Connection) -> Result<usize> {
        let backlog_path = self.repo_root.join("docs/HARNESS_BACKLOG.md");
        if !backlog_path.exists() {
            return Ok(0);
        }

        let content = fs::read_to_string(backlog_path)?;
        let items = backlog_items(&content);
        let mut imported = 0;
        for item in items {
            if item.title.is_empty() || item.title == "Short name." {
                continue;
            }

            let risk = if item.risk.is_empty() {
                None
            } else {
                RiskLane::from_str(&item.risk)
                    .ok()
                    .map(|value| value.as_db_value().to_owned())
            };
            let status = normalize_backlog_status(&item.status);
            let discovered = empty_to_none(item.discovered_while);
            let pain = empty_to_none(item.current_pain);
            let suggestion = empty_to_none(item.suggested_improvement);

            connection.execute(
                "INSERT INTO backlog (
                    title, discovered_while, current_pain, suggested_improvement,
                    risk, status, notes
                 )
                 SELECT ?1, ?2, ?3, ?4, ?5, ?6,
                    'Imported from docs/HARNESS_BACKLOG.md by harness import brownfield.'
                 WHERE NOT EXISTS (
                    SELECT 1 FROM backlog WHERE title=?1
                 );",
                params![item.title, discovered, pain, suggestion, risk, status],
            )?;
            imported += 1;
        }

        Ok(imported)
    }
}

impl HarnessRepository for SqliteHarnessRepository {
    fn init(&self) -> Result<InitResult> {
        if self.db_path.exists() {
            let connection = self.open_existing()?;
            let current = Self::schema_version(&connection).unwrap_or(0);
            if current == 0 {
                self.apply_schema_v1(&connection)?;
                self.apply_pending_migrations(&connection, 1)?;
                return Ok(InitResult::MigratedExisting {
                    db_path: self.db_path.clone(),
                });
            }

            return Ok(InitResult::Existing {
                db_path: self.db_path.clone(),
                version: current,
            });
        }

        let connection = self.open_or_create()?;
        self.apply_schema_v1(&connection)?;
        self.apply_pending_migrations(&connection, 1)?;
        Ok(InitResult::Created {
            db_path: self.db_path.clone(),
        })
    }

    fn migrate(&self) -> Result<MigrateResult> {
        let connection = self.open_existing()?;
        let current_version = Self::schema_version(&connection).unwrap_or(0);
        let applied = self.apply_pending_migrations(&connection, current_version)?;

        Ok(MigrateResult {
            current_version,
            applied,
        })
    }

    fn import_brownfield(&self) -> Result<BrownfieldImportResult> {
        let connection = self.open_existing()?;
        let stories = self.import_matrix(&connection)?;
        let decisions = self.import_decisions(&connection)?;
        let backlog_items = self.import_backlog(&connection)?;

        Ok(BrownfieldImportResult {
            stories,
            decisions,
            backlog_items,
        })
    }

    fn record_intake(&self, input: IntakeInput) -> Result<i64> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO intake (
                input_type, summary, risk_lane, risk_flags, affected_docs, story_id, notes,
                code_impact_summary, grounded_context, auto_generated
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10);",
            params![
                input.input_type.as_db_value(),
                input.summary,
                input.risk_lane.as_db_value(),
                input.risk_flags.as_json_text(),
                input.affected_docs.as_json_text(),
                input.story_id,
                input.notes,
                input.code_impact_summary,
                input.grounded_context,
                i64::from(input.auto_generated),
            ],
        )?;

        Ok(connection.last_insert_rowid())
    }

    fn add_story(&self, input: StoryAddInput) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO story (
                id, title, risk_lane, contract_doc, verify_command, notes,
                release_proof_required
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                input.id,
                input.title,
                input.risk_lane.as_db_value(),
                input.contract_doc,
                input.verify_command,
                input.notes,
                input.release_proof_required.0,
            ],
        )?;
        Ok(())
    }

    fn update_story(&self, input: StoryUpdateInput) -> Result<()> {
        if input.status.is_none()
            && input.evidence.is_none()
            && input.unit.is_none()
            && input.integration.is_none()
            && input.e2e.is_none()
            && input.platform.is_none()
            && input.verify_command.is_none()
            && input.release_proof_required.is_none()
        {
            return Err(HarnessInfraError::EmptyStoryUpdate);
        }

        let connection = self.open_existing()?;
        connection.execute(
            "UPDATE story SET
                status=COALESCE(?1, status),
                evidence=COALESCE(?2, evidence),
                unit_proof=COALESCE(?3, unit_proof),
                integration_proof=COALESCE(?4, integration_proof),
                e2e_proof=COALESCE(?5, e2e_proof),
                platform_proof=COALESCE(?6, platform_proof),
                verify_command=COALESCE(?7, verify_command),
                release_proof_required=COALESCE(?8, release_proof_required)
             WHERE id=?9;",
            params![
                input.status,
                input.evidence,
                input.unit.map(|value| value.0),
                input.integration.map(|value| value.0),
                input.e2e.map(|value| value.0),
                input.platform.map(|value| value.0),
                input.verify_command,
                input.release_proof_required.map(|value| value.0),
                input.id,
            ],
        )?;

        if connection.changes() == 0 {
            return Err(HarnessInfraError::StoryNotFound(input.id));
        }
        Ok(())
    }

    fn verify_story(&self, id: &str) -> Result<StoryVerifyResult> {
        let connection = self.open_existing()?;
        let verify_command = connection
            .query_row(
                "SELECT verify_command FROM story WHERE id=?1;",
                params![id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?
            .flatten()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| HarnessInfraError::MissingStoryVerifyCommand(id.to_owned()))?;

        let (shell, flag) = verifier_shell();
        let output = Command::new(shell)
            .arg(flag)
            .arg(&verify_command)
            .current_dir(&self.repo_root)
            .output()?;
        let result = if output.status.success() {
            "pass"
        } else {
            "fail"
        }
        .to_owned();
        connection.execute(
            "UPDATE story
             SET last_verified_at=datetime('now'), last_verified_result=?1
             WHERE id=?2;",
            params![result, id],
        )?;

        Ok(StoryVerifyResult {
            command: verify_command,
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
            result,
        })
    }

    fn verify_story_gate(&self, id: &str) -> Result<StoryGateResult> {
        let connection = self.open_existing()?;
        let story = connection
            .query_row(
                "SELECT risk_lane, context_pack_path, arch_check_result,
                        last_verified_result, evidence, unit_proof,
                        integration_proof, e2e_proof, platform_proof,
                        release_proof_required
                 FROM story WHERE id=?1;",
                params![id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, Option<String>>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, i64>(5)?,
                        row.get::<_, i64>(6)?,
                        row.get::<_, i64>(7)?,
                        row.get::<_, i64>(8)?,
                        row.get::<_, i64>(9)?,
                    ))
                },
            )
            .optional()?
            .ok_or_else(|| HarnessInfraError::StoryNotFound(id.to_owned()))?;

        let intake = connection
            .query_row(
                "SELECT auto_generated, code_impact_summary
                 FROM intake WHERE story_id=?1 ORDER BY id DESC LIMIT 1;",
                params![id],
                |row| Ok((row.get::<_, i64>(0)?, row.get::<_, Option<String>>(1)?)),
            )
            .optional()?;
        let trace_exists = connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM trace WHERE story_id=?1);",
            params![id],
            |row| row.get::<_, i64>(0),
        )? == 1;

        let mut missing = Vec::new();
        if intake.is_none() {
            missing.push("intake".to_owned());
        }
        let context_pack_exists = story
            .1
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .is_some_and(|value| self.repo_root.join(value).is_file());
        if !context_pack_exists {
            missing.push("context pack".to_owned());
        }
        if story.2.as_deref() != Some("pass") {
            missing.push("architecture check result".to_owned());
        }
        if story.3.as_deref() != Some("pass") {
            missing.push("validation command proof".to_owned());
        }
        if let Some((auto_generated, code_impact_summary)) = &intake {
            if *auto_generated == 1
                && code_impact_summary
                    .as_deref()
                    .map(str::trim)
                    .unwrap_or("")
                    .is_empty()
            {
                missing.push("code impact summary".to_owned());
            }
        }
        if story.0 == "high_risk" {
            let has_proof_flag = [story.5, story.6, story.7, story.8]
                .into_iter()
                .any(|value| value == 1);
            if !has_proof_flag || story.4.as_deref().map(str::trim).unwrap_or("").is_empty() {
                missing.push("validation proof".to_owned());
            }
        }
        if !trace_exists {
            missing.push("trace".to_owned());
        }
        if story.9 == 1 {
            let release_pass = connection.query_row(
                "SELECT EXISTS(
                    SELECT 1 FROM release_verification
                    WHERE story_id=?1 AND result='pass'
                 );",
                params![id],
                |row| row.get::<_, i64>(0),
            )? == 1;
            if !release_pass {
                missing.push("release verification proof".to_owned());
            }
        }

        let passed = missing.is_empty();
        connection.execute(
            "UPDATE story
             SET gate_checked_at=datetime('now'), gate_result=?1
             WHERE id=?2;",
            params![if passed { "pass" } else { "fail" }, id],
        )?;

        Ok(StoryGateResult {
            id: id.to_owned(),
            passed,
            missing,
        })
    }

    fn verify_release(
        &self,
        input: ReleaseVerifyInput,
    ) -> Result<(PathBuf, ReleaseVerificationReport)> {
        validate_release_version(&input.version)?;
        let config_path = self.repo_root.join("harness-release.toml");
        if !config_path.is_file() {
            return Err(HarnessInfraError::MissingReleaseConfig(
                config_path.display().to_string(),
            ));
        }
        let config_text = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<ReleaseConfig>(&config_text)
            .map_err(|error| HarnessInfraError::InvalidReleaseConfig(error.to_string()))?;
        validate_release_origin(&config.origin)?;
        if config.tag_prefix.trim().is_empty() {
            return Err(HarnessInfraError::InvalidReleaseConfig(
                "tag_prefix must not be empty".to_owned(),
            ));
        }

        if let Some(story_id) = input.story_id.as_deref() {
            let connection = self.open_existing()?;
            let exists = connection.query_row(
                "SELECT EXISTS(SELECT 1 FROM story WHERE id=?1);",
                params![story_id],
                |row| row.get::<_, i64>(0),
            )? == 1;
            if !exists {
                return Err(HarnessInfraError::StoryNotFound(story_id.to_owned()));
            }
        }

        let origin = input.origin.unwrap_or_else(|| config.origin.clone());
        validate_release_origin(&origin)?;
        let platform = input.platform.unwrap_or_else(host_release_platform);
        let binary_asset = binary_asset_for_platform(&platform)?.to_owned();
        let checksum_asset = format!("{binary_asset}.sha256");
        let tag = format!("{}{}", config.tag_prefix, input.version);
        let output_path = input.output.unwrap_or_else(|| {
            self.repo_root
                .join(format!(".harness/release/{tag}-release-verify.json"))
        });

        let mut report = ReleaseVerificationReport {
            checked_at_unix: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            version: input.version.clone(),
            canonical_origin: config.origin.clone(),
            origin: origin.clone(),
            tag: tag.clone(),
            platform: platform.clone(),
            assets_checked: 0,
            assets: Vec::new(),
            binary_asset: binary_asset.clone(),
            checksum_asset: checksum_asset.clone(),
            expected_hash: None,
            actual_hash: None,
            download: ReleaseCheckResult::Inconclusive,
            checksum: ReleaseCheckResult::Inconclusive,
            version_check: ReleaseCheckResult::Inconclusive,
            smoke_install: ReleaseCheckResult::Inconclusive,
            version_output: None,
            smoke_output: None,
            failures: Vec::new(),
            result: ReleaseCheckResult::Inconclusive,
        };

        if input.story_id.is_some() && origin != config.origin {
            report.result = ReleaseCheckResult::Fail;
            report.failures.push(format!(
                "story-linked verification origin '{origin}' does not match canonical origin '{}'",
                config.origin
            ));
        } else {
            run_release_checks(&origin, &tag, &platform, &mut report)?;
        }

        write_release_report(&output_path, &report)?;
        let report_path = path_for_storage(&self.repo_root, &output_path);
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO release_verification (
                version, origin, tag, platform, result, report_path,
                assets_checked, failure, story_id
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
            params![
                report.version,
                report.origin,
                report.tag,
                report.platform,
                report.result.as_db_value(),
                report_path,
                report.assets_checked as i64,
                if report.failures.is_empty() {
                    None
                } else {
                    Some(report.failures.join("; "))
                },
                input.story_id,
            ],
        )?;

        Ok((output_path, report))
    }

    fn check_architecture(
        &self,
        config_path: Option<PathBuf>,
        story_id: Option<&str>,
    ) -> Result<ArchitectureCheckResult> {
        let config_path = match config_path {
            Some(path) if path.is_absolute() => path,
            Some(path) => self.repo_root.join(path),
            None => self.repo_root.join("harness-architecture.toml"),
        };
        if !config_path.is_file() {
            return Err(HarnessInfraError::MissingArchitectureConfig(
                config_path.display().to_string(),
            ));
        }
        let config_text = fs::read_to_string(&config_path)?;
        let config = toml::from_str::<ArchitectureConfig>(&config_text)
            .map_err(|error| HarnessInfraError::InvalidArchitectureConfig(error.to_string()))?;
        if config.layer.is_empty() {
            return Err(HarnessInfraError::InvalidArchitectureConfig(
                "at least one [[layer]] rule is required".to_owned(),
            ));
        }

        let mut scanned_files = 0;
        let mut violations = Vec::new();
        for layer in &config.layer {
            let layer_root = self.repo_root.join(&layer.path);
            if !layer_root.is_dir() {
                continue;
            }
            scan_architecture_layer(
                &self.repo_root,
                &layer_root,
                &layer.name,
                &layer.files,
                &layer.forbidden_imports,
                &mut scanned_files,
                &mut violations,
            )?;
        }
        let passed = violations.is_empty();
        if let Some(story_id) = story_id {
            let connection = self.open_existing()?;
            connection.execute(
                "UPDATE story
                 SET arch_check_result=?1, arch_checked_at=datetime('now')
                 WHERE id=?2;",
                params![if passed { "pass" } else { "fail" }, story_id],
            )?;
            if connection.changes() == 0 {
                return Err(HarnessInfraError::StoryNotFound(story_id.to_owned()));
            }
        }

        Ok(ArchitectureCheckResult {
            passed,
            scanned_files,
            violations,
        })
    }

    fn add_decision(&self, input: DecisionAddInput) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO decision (id, title, status, doc_path, verify_command, predicted_impact, notes)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                input.id,
                input.title,
                input.status,
                input.doc_path,
                input.verify_command,
                input.predicted_impact,
                input.notes,
            ],
        )?;
        Ok(())
    }

    fn verify_decision(&self, id: &str) -> Result<DecisionVerifyResult> {
        let connection = self.open_existing()?;
        let verify_command = connection
            .query_row(
                "SELECT verify_command FROM decision WHERE id=?1;",
                params![id],
                |row| row.get::<_, Option<String>>(0),
            )
            .optional()?
            .flatten()
            .filter(|value| !value.is_empty())
            .ok_or_else(|| HarnessInfraError::MissingDecisionVerifyCommand(id.to_owned()))?;

        let (shell, flag) = verifier_shell();
        let status = Command::new(shell)
            .arg(flag)
            .arg(&verify_command)
            .current_dir(&self.repo_root)
            .status()?;
        let result = if status.success() { "pass" } else { "fail" }.to_owned();
        connection.execute(
            "UPDATE decision
             SET last_verified_at=datetime('now'), last_verified_result=?1
             WHERE id=?2;",
            params![result, id],
        )?;

        Ok(DecisionVerifyResult {
            command: verify_command,
            result,
        })
    }

    fn add_backlog(&self, input: BacklogAddInput) -> Result<i64> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO backlog (
                title, discovered_while, current_pain, suggested_improvement,
                risk, predicted_impact, notes
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);",
            params![
                input.title,
                input.discovered_while,
                input.current_pain,
                input.suggestion,
                input.risk.map(|value| value.as_db_value().to_owned()),
                input.predicted_impact,
                input.notes,
            ],
        )?;
        Ok(connection.last_insert_rowid())
    }

    fn close_backlog(&self, input: BacklogCloseInput) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute(
            "UPDATE backlog
             SET status=?1, actual_outcome=?2, implemented_at=datetime('now')
             WHERE id=?3;",
            params![input.status, input.actual_outcome, input.id],
        )?;

        if connection.changes() == 0 {
            return Err(HarnessInfraError::BacklogNotFound(input.id));
        }
        Ok(())
    }

    fn record_trace(&self, input: TraceInput) -> Result<i64> {
        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO trace (
                task_summary, intake_id, story_id, agent,
                actions_taken, files_read, files_changed, decisions_made, errors,
                outcome, duration_seconds, token_estimate, harness_friction, notes
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14);",
            params![
                input.task_summary,
                input.intake_id,
                input.story_id,
                input.agent,
                input.actions.as_json_text(),
                input.files_read.as_json_text(),
                input.files_changed.as_json_text(),
                input.decisions.as_json_text(),
                input.errors.as_json_text(),
                input.outcome,
                input.duration_seconds,
                input.token_estimate,
                input.friction,
                input.notes,
            ],
        )?;
        Ok(connection.last_insert_rowid())
    }

    fn score_trace(&self, id: Option<i64>) -> Result<TraceScoreResult> {
        let connection = self.open_existing()?;
        let sql = match id {
            Some(_) => {
                "SELECT
                    trace.id,
                    trace.task_summary,
                    trace.intake_id,
                    intake.risk_lane,
                    trace.agent,
                    trace.actions_taken,
                    trace.files_read,
                    trace.files_changed,
                    trace.decisions_made,
                    trace.errors,
                    trace.outcome,
                    trace.duration_seconds,
                    trace.token_estimate,
                    trace.harness_friction,
                    trace.notes
                 FROM trace
                 LEFT JOIN intake ON intake.id = trace.intake_id
                 WHERE trace.id = ?1"
            }
            None => {
                "SELECT
                    trace.id,
                    trace.task_summary,
                    trace.intake_id,
                    intake.risk_lane,
                    trace.agent,
                    trace.actions_taken,
                    trace.files_read,
                    trace.files_changed,
                    trace.decisions_made,
                    trace.errors,
                    trace.outcome,
                    trace.duration_seconds,
                    trace.token_estimate,
                    trace.harness_friction,
                    trace.notes
                 FROM trace
                 LEFT JOIN intake ON intake.id = trace.intake_id
                 ORDER BY trace.id DESC
                 LIMIT 1"
            }
        };

        let source = if let Some(id) = id {
            connection
                .query_row(sql, params![id], trace_score_source_from_row)
                .optional()?
                .ok_or(HarnessInfraError::TraceNotFound(id))?
        } else {
            connection
                .query_row(sql, [], trace_score_source_from_row)
                .optional()?
                .ok_or(HarnessInfraError::NoTraces)?
        };

        Ok(score_trace(source))
    }

    fn story_verify_status(&self, id: &str) -> Result<StoryVerifyStatus> {
        let connection = self.open_existing()?;
        connection
            .query_row(
                "SELECT id, verify_command, last_verified_result FROM story WHERE id=?1;",
                params![id],
                |row| {
                    Ok(StoryVerifyStatus {
                        id: row.get(0)?,
                        verify_command: row.get(1)?,
                        last_verified_result: row.get(2)?,
                    })
                },
            )
            .optional()?
            .ok_or_else(|| HarnessInfraError::StoryNotFound(id.to_owned()))
    }

    fn query_matrix(&self) -> Result<Vec<StoryMatrixRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, title, status, unit_proof, integration_proof, e2e_proof, platform_proof, evidence
             FROM story ORDER BY id;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(StoryMatrixRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                unit: row.get(3)?,
                integration: row.get(4)?,
                e2e: row.get(5)?,
                platform: row.get(6)?,
                evidence: row.get(7)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_backlog(&self, filter: BacklogFilter) -> Result<Vec<BacklogRecord>> {
        let connection = self.open_existing()?;
        let where_clause = match filter {
            BacklogFilter::All => "",
            BacklogFilter::Open => "WHERE status IN ('proposed', 'accepted')",
            BacklogFilter::Closed => "WHERE status IN ('implemented', 'rejected')",
        };
        let sql = format!(
            "SELECT id, title, status, risk, predicted_impact, actual_outcome
             FROM backlog {where_clause} ORDER BY status, id;"
        );
        let mut statement = connection.prepare(&sql)?;

        let rows = statement.query_map([], |row| {
            Ok(BacklogRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                risk: row.get(3)?,
                predicted_impact: row.get(4)?,
                actual_outcome: row.get(5)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_decisions(&self) -> Result<Vec<DecisionRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, title, status, last_verified_at, last_verified_result
             FROM decision ORDER BY id;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(DecisionRecord {
                id: row.get(0)?,
                title: row.get(1)?,
                status: row.get(2)?,
                last_verified_at: row.get(3)?,
                last_verified_result: row.get(4)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_intakes(&self) -> Result<Vec<IntakeRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, created_at, input_type, risk_lane, summary
             FROM intake ORDER BY id DESC LIMIT 20;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(IntakeRecord {
                id: row.get(0)?,
                created_at: row.get(1)?,
                input_type: row.get(2)?,
                risk_lane: row.get(3)?,
                summary: row.get(4)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_traces(&self) -> Result<Vec<TraceRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, created_at, outcome, task_summary, harness_friction
             FROM trace ORDER BY id DESC LIMIT 20;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(TraceRecord {
                id: row.get(0)?,
                created_at: row.get(1)?,
                outcome: row.get(2)?,
                task_summary: row.get(3)?,
                harness_friction: row.get(4)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_friction(&self) -> Result<Vec<FrictionRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT
                trace.id,
                trace.created_at,
                intake.risk_lane,
                intake.input_type,
                trace.task_summary,
                trace.harness_friction
             FROM trace
             LEFT JOIN intake ON intake.id = trace.intake_id
             WHERE trace.harness_friction IS NOT NULL
             ORDER BY trace.id DESC;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(FrictionRecord {
                id: row.get(0)?,
                created_at: row.get(1)?,
                risk_lane: row.get(2)?,
                input_type: row.get(3)?,
                task_summary: row.get(4)?,
                harness_friction: row.get(5)?,
            })
        })?;

        collect_rows(rows)
    }

    fn query_stats(&self) -> Result<HarnessStats> {
        let connection = self.open_existing()?;
        connection
            .query_row(
                "SELECT
                    (SELECT COUNT(*) FROM intake) AS intakes,
                    (SELECT COUNT(*) FROM story) AS stories,
                    (SELECT COUNT(*) FROM decision) AS decisions,
                    (SELECT COUNT(*) FROM backlog) AS backlog_items,
                    (SELECT COUNT(*) FROM trace) AS traces;",
                [],
                |row| {
                    Ok(HarnessStats {
                        intakes: row.get(0)?,
                        stories: row.get(1)?,
                        decisions: row.get(2)?,
                        backlog_items: row.get(3)?,
                        traces: row.get(4)?,
                    })
                },
            )
            .map_err(HarnessInfraError::from)
    }

    fn query_sql(&self, sql: &str) -> Result<QueryTable> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(sql)?;
        let headers = statement
            .column_names()
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>();
        let column_count = statement.column_count();
        let rows = statement.query_map([], |row| {
            let mut values = Vec::new();
            for index in 0..column_count {
                values.push(sql_value_to_string(row.get_ref(index)?));
            }
            Ok(values)
        })?;

        Ok(QueryTable {
            headers,
            rows: collect_rows(rows)?,
        })
    }

    fn get_context_pack_data(&self, story_id: &str) -> Result<ContextPackData> {
        let connection = self.open_existing()?;

        let story: (String, String, String, String, Option<String>, Option<String>, Option<String>) = connection
            .query_row(
                "SELECT id, title, status, risk_lane, contract_doc, verify_command, notes FROM story WHERE id=?1;",
                params![story_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, String>(2)?,
                        row.get::<_, String>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<String>>(6)?,
                    ))
                },
            )
            .optional()?
            .ok_or_else(|| HarnessInfraError::StoryNotFound(story_id.to_owned()))?;

        let intake: Option<ContextIntakeRow> = connection
            .query_row(
                "SELECT input_type, summary, risk_flags, affected_docs, notes, code_impact_summary, grounded_context 
                 FROM intake WHERE story_id=?1 ORDER BY id DESC LIMIT 1;",
                params![story_id],
                |row| {
                    Ok((
                        row.get::<_, String>(0)?,
                        row.get::<_, String>(1)?,
                        row.get::<_, Option<String>>(2)?,
                        row.get::<_, Option<String>>(3)?,
                        row.get::<_, Option<String>>(4)?,
                        row.get::<_, Option<String>>(5)?,
                        row.get::<_, Option<String>>(6)?,
                    ))
                },
            )
            .optional()?;

        let (
            intake_input_type,
            intake_summary,
            intake_risk_flags,
            intake_affected_docs,
            intake_notes,
            code_impact_summary,
            grounded_context,
        ) = match intake {
            Some(i) => (Some(i.0), Some(i.1), i.2, i.3, i.4, i.5, i.6),
            None => (None, None, None, None, None, None, None),
        };

        Ok(ContextPackData {
            story_id: story.0,
            story_title: story.1,
            story_status: story.2,
            story_risk_lane: story.3,
            story_contract_doc: story.4,
            story_verify_command: story.5,
            story_notes: story.6,
            intake_input_type,
            intake_summary,
            intake_risk_flags,
            intake_affected_docs,
            intake_notes,
            code_impact_summary,
            grounded_context,
        })
    }

    fn update_story_context_pack_path(&self, id: &str, path: &str) -> Result<()> {
        let connection = self.open_existing()?;
        connection.execute(
            "UPDATE story SET context_pack_path=?1 WHERE id=?2;",
            params![path, id],
        )?;
        Ok(())
    }

    fn repo_root(&self) -> PathBuf {
        self.repo_root.clone()
    }
}

impl From<HarnessContext> for SqliteHarnessRepository {
    fn from(context: HarnessContext) -> Self {
        Self::new(context.repo_root, context.db_path, context.schema_dir)
    }
}

#[derive(Debug)]
struct MatrixColumns {
    story: Option<usize>,
    contract: Option<usize>,
    unit: Option<usize>,
    integration: Option<usize>,
    e2e: Option<usize>,
    platform: Option<usize>,
    status: Option<usize>,
    evidence: Option<usize>,
}

#[derive(Debug, Default)]
struct BacklogMarkdownItem {
    title: String,
    discovered_while: String,
    current_pain: String,
    suggested_improvement: String,
    risk: String,
    status: String,
}

impl MatrixColumns {
    fn from_header(fields: &[String]) -> Self {
        let mut columns = Self {
            story: None,
            contract: None,
            unit: None,
            integration: None,
            e2e: None,
            platform: None,
            status: None,
            evidence: None,
        };

        for (index, field) in fields.iter().enumerate() {
            match normalize_token(field).as_str() {
                "story" => columns.story = Some(index),
                "contract" => columns.contract = Some(index),
                "unit" => columns.unit = Some(index),
                "integration" => columns.integration = Some(index),
                "e2e" => columns.e2e = Some(index),
                "platform" => columns.platform = Some(index),
                "status" => columns.status = Some(index),
                "evidence" => columns.evidence = Some(index),
                _ => {}
            }
        }

        columns
    }
}

fn collect_rows<T>(
    rows: rusqlite::MappedRows<'_, impl FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<T>>,
) -> Result<Vec<T>> {
    rows.collect::<std::result::Result<Vec<_>, _>>()
        .map_err(HarnessInfraError::from)
}

fn scan_architecture_layer(
    repo_root: &Path,
    current: &Path,
    layer_name: &str,
    included_files: &[String],
    forbidden_imports: &[String],
    scanned_files: &mut usize,
    violations: &mut Vec<ArchitectureViolation>,
) -> Result<()> {
    for entry in fs::read_dir(current)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if !is_ignored_directory(&path) {
                scan_architecture_layer(
                    repo_root,
                    &path,
                    layer_name,
                    included_files,
                    forbidden_imports,
                    scanned_files,
                    violations,
                )?;
            }
            continue;
        }
        if !is_supported_source_file(&path) || !is_included_file(&path, included_files) {
            continue;
        }

        *scanned_files += 1;
        let content = fs::read_to_string(&path)?;
        for import in source_imports(&content) {
            for forbidden in forbidden_imports {
                if import_matches_path(&import, forbidden) {
                    let relative = path
                        .strip_prefix(repo_root)
                        .unwrap_or(&path)
                        .to_string_lossy()
                        .replace('\\', "/");
                    violations.push(ArchitectureViolation {
                        file: relative,
                        import: import.clone(),
                        rule: format!(
                            "{} must not depend on {}",
                            layer_name,
                            forbidden.replace('\\', "/")
                        ),
                    });
                }
            }
        }
    }
    Ok(())
}

fn is_ignored_directory(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|value| value.to_str()),
        Some(".git" | ".harness" | "node_modules" | "target" | "dist" | "build")
    )
}

fn is_supported_source_file(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|value| value.to_str()),
        Some("rs" | "ts" | "tsx" | "js" | "jsx" | "py" | "go" | "java" | "kt" | "cs")
    )
}

fn is_included_file(path: &Path, included_files: &[String]) -> bool {
    included_files.is_empty()
        || path
            .file_name()
            .and_then(|value| value.to_str())
            .is_some_and(|value| included_files.iter().any(|item| item == value))
}

fn source_imports(content: &str) -> Vec<String> {
    content
        .lines()
        .filter_map(source_import)
        .collect::<Vec<_>>()
}

fn source_import(line: &str) -> Option<String> {
    let trimmed = line.trim();
    if let Some(require_index) = trimmed.find("require(") {
        return clean_import_target(&trimmed[require_index + "require(".len()..]);
    }
    if let Some(value) = trimmed.strip_prefix("#include") {
        return clean_import_target(value);
    }
    if let Some(value) = trimmed
        .strip_prefix("pub use ")
        .or_else(|| trimmed.strip_prefix("use "))
    {
        return Some(value.trim_end_matches(';').trim().to_owned());
    }
    if let Some(value) = trimmed.strip_prefix("from ") {
        return value.split_whitespace().next().map(str::to_owned);
    }
    if let Some(value) = trimmed.strip_prefix("import ") {
        if let Some((_, module)) = value.rsplit_once(" from ") {
            return clean_import_target(module);
        }
        return clean_import_target(value);
    }
    None
}

fn clean_import_target(value: &str) -> Option<String> {
    let target = value
        .trim()
        .trim_matches(|character| matches!(character, '"' | '\'' | '<' | '>' | ';' | ')' | '('))
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_matches(|character| matches!(character, '"' | '\'' | '<' | '>' | ';' | ')' | '('));
    (!target.is_empty()).then(|| target.to_owned())
}

fn import_matches_path(import: &str, forbidden: &str) -> bool {
    let import_segments = path_segments(import);
    let forbidden_segments = path_segments(forbidden);
    !forbidden_segments.is_empty()
        && import_segments
            .windows(forbidden_segments.len())
            .any(|window| window == forbidden_segments)
}

fn path_segments(value: &str) -> Vec<String> {
    value
        .split(|character: char| {
            matches!(character, '/' | '\\' | ':' | '.')
                || !(character.is_ascii_alphanumeric() || character == '_')
        })
        .filter(|segment| !segment.is_empty())
        .map(|segment| segment.to_ascii_lowercase())
        .collect()
}

fn trace_score_source_from_row(row: &rusqlite::Row<'_>) -> rusqlite::Result<TraceScoreSource> {
    Ok(TraceScoreSource {
        id: row.get(0)?,
        task_summary: row.get(1)?,
        intake_id: row.get(2)?,
        risk_lane: row.get(3)?,
        agent: row.get(4)?,
        actions_taken: row.get(5)?,
        files_read: row.get(6)?,
        files_changed: row.get(7)?,
        decisions_made: row.get(8)?,
        errors: row.get(9)?,
        outcome: row.get(10)?,
        duration_seconds: row.get(11)?,
        token_estimate: row.get(12)?,
        harness_friction: row.get(13)?,
        notes: row.get(14)?,
    })
}

fn markdown_table_fields(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    let trimmed = trimmed.strip_prefix('|').unwrap_or(trimmed);
    let trimmed = trimmed.strip_suffix('|').unwrap_or(trimmed);
    trimmed
        .split('|')
        .map(|field| field.trim().to_owned())
        .collect()
}

fn field_at(fields: &[String], index: Option<usize>) -> Option<String> {
    index
        .and_then(|value| fields.get(value))
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn evidence_from_fields(fields: &[String], start_index: usize) -> Option<String> {
    fields
        .get(start_index..)
        .map(|values| values.join(" | "))
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn proof_from_cell(value: &str) -> i64 {
    match normalize_token(value).as_str() {
        ""
        | "no"
        | "none"
        | "n_a"
        | "na"
        | "planned"
        | "pending"
        | "blocked"
        | "not_attempted"
        | "not_operator_reviewed" => 0,
        token
            if token.starts_with("no_")
                || token.starts_with("pending")
                || token.starts_with("blocked")
                || token.contains("pending")
                || token.contains("blocked")
                || token.contains("not_attempted")
                || token.contains("not_operator_reviewed") =>
        {
            0
        }
        _ => 1,
    }
}

fn normalize_story_status(value: &str) -> String {
    match normalize_token(value).as_str() {
        "planned" => "planned",
        "in_progress" => "in_progress",
        "implemented" => "implemented",
        "changed" => "changed",
        "retired" => "retired",
        _ => "planned",
    }
    .to_owned()
}

fn normalize_decision_status(value: &str) -> String {
    let token = normalize_token(value);
    match token.as_str() {
        "proposed" => "proposed",
        "accepted" => "accepted",
        "superseded" => "superseded",
        "rejected" => "rejected",
        token if token.starts_with("superseded_") => "superseded",
        _ => "accepted",
    }
    .to_owned()
}

fn normalize_backlog_status(value: &str) -> String {
    match normalize_token(value).as_str() {
        "proposed" => "proposed",
        "accepted" => "accepted",
        "implemented" => "implemented",
        "rejected" => "rejected",
        _ => "proposed",
    }
    .to_owned()
}

fn markdown_section_first_value(content: &str, heading: &str) -> String {
    let target = format!("## {heading}");
    let mut found = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if found && !trimmed.is_empty() {
            return trimmed.to_owned();
        }
        if trimmed == target {
            found = true;
        }
    }
    String::new()
}

fn backlog_items(content: &str) -> Vec<BacklogMarkdownItem> {
    let mut in_items = false;
    let mut current_heading = String::new();
    let mut current = BacklogMarkdownItem::default();
    let mut items = Vec::new();

    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed == "## Items" {
            in_items = true;
            current_heading.clear();
            continue;
        }
        if !in_items {
            continue;
        }

        if let Some(heading) = trimmed.strip_prefix("### ") {
            let normalized = normalize_token(heading);
            if normalized == "title" && !current.title.is_empty() {
                items.push(current);
                current = BacklogMarkdownItem::default();
            }
            current_heading = normalized;
            continue;
        }

        if trimmed.is_empty() || current_heading.is_empty() {
            continue;
        }

        let target = match current_heading.as_str() {
            "title" => &mut current.title,
            "discovered_while" => &mut current.discovered_while,
            "current_pain" => &mut current.current_pain,
            "suggested_improvement" => &mut current.suggested_improvement,
            "risk" => &mut current.risk,
            "status" => &mut current.status,
            _ => continue,
        };
        if target.is_empty() {
            *target = trimmed.to_owned();
        }
    }

    if !current.title.is_empty() {
        items.push(current);
    }
    items
}

fn empty_to_none(value: String) -> Option<String> {
    if value.is_empty() {
        None
    } else {
        Some(value)
    }
}

fn verifier_shell() -> (&'static str, &'static str) {
    if cfg!(windows) {
        ("cmd", "/C")
    } else {
        ("sh", "-c")
    }
}

fn is_decision_file_name(file_name: &str) -> bool {
    let Some((prefix, _)) = file_name.split_once('-') else {
        return false;
    };
    prefix.len() == 4 && prefix.chars().all(|character| character.is_ascii_digit())
}

fn sql_value_to_string(value: ValueRef<'_>) -> String {
    match value {
        ValueRef::Null => String::new(),
        ValueRef::Integer(value) => value.to_string(),
        ValueRef::Real(value) => value.to_string(),
        ValueRef::Text(value) => String::from_utf8_lossy(value).into_owned(),
        ValueRef::Blob(value) => format!("<{} bytes>", value.len()),
    }
}

#[derive(Debug, Deserialize)]
struct GithubRelease {
    assets: Vec<GithubAsset>,
}

#[derive(Debug, Deserialize)]
struct GithubAsset {
    name: String,
    browser_download_url: String,
}

struct ExternalFailure {
    result: ReleaseCheckResult,
    message: String,
}

fn validate_release_version(version: &str) -> Result<()> {
    let parts = version.split('.').collect::<Vec<_>>();
    if parts.len() != 3
        || parts
            .iter()
            .any(|part| part.is_empty() || !part.chars().all(|value| value.is_ascii_digit()))
    {
        return Err(HarnessInfraError::InvalidReleaseInput(format!(
            "version '{version}' must use numeric major.minor.patch form"
        )));
    }
    Ok(())
}

fn validate_release_origin(origin: &str) -> Result<()> {
    let parts = origin.split('/').collect::<Vec<_>>();
    if parts.len() != 2
        || parts
            .iter()
            .any(|part| part.is_empty() || !part.chars().all(is_release_origin_character))
    {
        return Err(HarnessInfraError::InvalidReleaseInput(format!(
            "origin '{origin}' must use owner/repository form"
        )));
    }
    Ok(())
}

fn is_release_origin_character(value: char) -> bool {
    value.is_ascii_alphanumeric() || matches!(value, '-' | '_' | '.')
}

fn host_release_platform() -> String {
    match (std::env::consts::OS, std::env::consts::ARCH) {
        ("windows", "x86_64") => "windows-x64",
        ("linux", "x86_64") => "linux-x64",
        ("linux", "aarch64") => "linux-arm64",
        ("macos", "x86_64") => "macos-x64",
        ("macos", "aarch64") => "macos-arm64",
        (os, arch) => return format!("{os}-{arch}"),
    }
    .to_owned()
}

fn binary_asset_for_platform(platform: &str) -> Result<&'static str> {
    match platform {
        "windows-x64" => Ok("harness-cli-windows-x64.exe"),
        "linux-x64" => Ok("harness-cli-linux-x64"),
        "linux-arm64" => Ok("harness-cli-linux-arm64"),
        "macos-x64" => Ok("harness-cli-macos-x64"),
        "macos-arm64" => Ok("harness-cli-macos-arm64"),
        _ => Err(HarnessInfraError::InvalidReleaseInput(format!(
            "unsupported platform '{platform}'. Use windows-x64, linux-x64, linux-arm64, macos-x64, or macos-arm64"
        ))),
    }
}

fn expected_release_assets() -> Vec<String> {
    [
        "harness-cli-linux-arm64",
        "harness-cli-linux-x64",
        "harness-cli-macos-arm64",
        "harness-cli-macos-x64",
        "harness-cli-windows-x64.exe",
    ]
    .into_iter()
    .flat_map(|binary| [binary.to_owned(), format!("{binary}.sha256")])
    .collect()
}

fn run_release_checks(
    origin: &str,
    tag: &str,
    platform: &str,
    report: &mut ReleaseVerificationReport,
) -> Result<()> {
    let api_url = format!("https://api.github.com/repos/{origin}/releases/tags/{tag}");
    let release = match fetch_github_release(&api_url) {
        Ok(release) => release,
        Err(failure) => {
            report.result = failure.result;
            report.failures.push(failure.message);
            return Ok(());
        }
    };

    report.assets = release
        .assets
        .iter()
        .map(|asset| ReleaseAssetEvidence {
            name: asset.name.clone(),
            download_url: asset.browser_download_url.clone(),
        })
        .collect();
    report.assets_checked = report.assets.len();

    let expected = expected_release_assets();
    let observed = release
        .assets
        .iter()
        .map(|asset| asset.name.clone())
        .collect::<Vec<_>>();
    let missing = missing_expected_assets(&expected, &observed);
    if !missing.is_empty() {
        report.result = ReleaseCheckResult::Fail;
        report.failures.push(format!(
            "release is missing expected assets: {}",
            missing.join(", ")
        ));
        return Ok(());
    }

    let binary = release
        .assets
        .iter()
        .find(|asset| asset.name == report.binary_asset)
        .expect("complete asset contract includes selected binary");
    let checksum = release
        .assets
        .iter()
        .find(|asset| asset.name == report.checksum_asset)
        .expect("complete asset contract includes selected checksum");
    let temp_dir = tempfile::tempdir()?;
    let binary_path = temp_dir.path().join(&binary.name);

    let binary_bytes = match download_public_asset(&binary.browser_download_url) {
        Ok(bytes) => bytes,
        Err(failure) => {
            report.download = failure.result;
            report.result = failure.result;
            report.failures.push(failure.message);
            return Ok(());
        }
    };
    let checksum_bytes = match download_public_asset(&checksum.browser_download_url) {
        Ok(bytes) => bytes,
        Err(failure) => {
            report.download = failure.result;
            report.result = failure.result;
            report.failures.push(failure.message);
            return Ok(());
        }
    };
    report.download = ReleaseCheckResult::Pass;
    fs::write(&binary_path, &binary_bytes)?;

    let expected_hash = match parse_sha256(&checksum_bytes) {
        Ok(hash) => hash,
        Err(message) => {
            report.checksum = ReleaseCheckResult::Fail;
            report.result = ReleaseCheckResult::Fail;
            report.failures.push(message);
            return Ok(());
        }
    };
    let actual_hash = format!("{:x}", Sha256::digest(&binary_bytes));
    report.expected_hash = Some(expected_hash.clone());
    report.actual_hash = Some(actual_hash.clone());
    if expected_hash != actual_hash {
        report.checksum = ReleaseCheckResult::Fail;
        report.result = ReleaseCheckResult::Fail;
        report.failures.push(format!(
            "checksum mismatch for {}: expected {expected_hash}, got {actual_hash}",
            binary.name
        ));
        return Ok(());
    }
    report.checksum = ReleaseCheckResult::Pass;

    if platform != host_release_platform() {
        report.result = ReleaseCheckResult::Inconclusive;
        report.failures.push(format!(
            "platform '{platform}' cannot be executed on host '{}'",
            host_release_platform()
        ));
        return Ok(());
    }

    if let Err(error) = make_executable(&binary_path) {
        report.version_check = ReleaseCheckResult::Fail;
        report.result = ReleaseCheckResult::Fail;
        report.failures.push(format!(
            "downloaded binary could not be made executable: {error}"
        ));
        return Ok(());
    }
    let version_output = match Command::new(&binary_path).arg("--version").output() {
        Ok(output) => output,
        Err(error) => {
            report.version_check = ReleaseCheckResult::Fail;
            report.result = ReleaseCheckResult::Fail;
            report.failures.push(format!(
                "downloaded binary could not run --version: {error}"
            ));
            return Ok(());
        }
    };
    let version_text = combined_output(&version_output);
    report.version_output = Some(version_text.clone());
    if !version_output.status.success()
        || version_text.trim() != format!("harness-cli {}", report.version)
    {
        report.version_check = ReleaseCheckResult::Fail;
        report.result = ReleaseCheckResult::Fail;
        report.failures.push(format!(
            "version check expected 'harness-cli {}', got '{}'",
            report.version,
            version_text.trim()
        ));
        return Ok(());
    }
    report.version_check = ReleaseCheckResult::Pass;

    let smoke_output = match Command::new(&binary_path)
        .args(["arch-check", "--help"])
        .output()
    {
        Ok(output) => output,
        Err(error) => {
            report.smoke_install = ReleaseCheckResult::Fail;
            report.result = ReleaseCheckResult::Fail;
            report
                .failures
                .push(format!("smoke command could not start: {error}"));
            return Ok(());
        }
    };
    let smoke_text = combined_output(&smoke_output);
    report.smoke_output = Some(smoke_text);
    if !smoke_output.status.success() {
        report.smoke_install = ReleaseCheckResult::Fail;
        report.result = ReleaseCheckResult::Fail;
        report
            .failures
            .push("smoke command 'arch-check --help' failed".to_owned());
        return Ok(());
    }

    report.smoke_install = ReleaseCheckResult::Pass;
    report.result = ReleaseCheckResult::Pass;
    Ok(())
}

fn fetch_github_release(url: &str) -> std::result::Result<GithubRelease, ExternalFailure> {
    let response = ureq::get(url)
        .set("Accept", "application/vnd.github+json")
        .set("User-Agent", "harness-cli-release-verify")
        .call()
        .map_err(|error| classify_http_error("release metadata", error))?;
    response
        .into_json::<GithubRelease>()
        .map_err(|error| ExternalFailure {
            result: ReleaseCheckResult::Fail,
            message: format!("release metadata is invalid JSON: {error}"),
        })
}

fn download_public_asset(url: &str) -> std::result::Result<Vec<u8>, ExternalFailure> {
    let response = ureq::get(url)
        .set("User-Agent", "harness-cli-release-verify")
        .call()
        .map_err(|error| classify_http_error("public asset download", error))?;
    let mut bytes = Vec::new();
    response
        .into_reader()
        .read_to_end(&mut bytes)
        .map_err(|error| ExternalFailure {
            result: ReleaseCheckResult::Inconclusive,
            message: format!("public asset download interrupted: {error}"),
        })?;
    Ok(bytes)
}

fn classify_http_error(context: &str, error: ureq::Error) -> ExternalFailure {
    match error {
        ureq::Error::Status(status, _) => {
            let result = http_status_result(status);
            ExternalFailure {
                result,
                message: if result == ReleaseCheckResult::Inconclusive {
                    format!("{context} unavailable with HTTP {status}")
                } else {
                    format!("{context} rejected with HTTP {status}")
                },
            }
        }
        ureq::Error::Transport(error) => ExternalFailure {
            result: ReleaseCheckResult::Inconclusive,
            message: format!("{context} unavailable: {error}"),
        },
    }
}

fn http_status_result(status: u16) -> ReleaseCheckResult {
    if status >= 500 || status == 429 {
        ReleaseCheckResult::Inconclusive
    } else {
        ReleaseCheckResult::Fail
    }
}

fn missing_expected_assets(expected: &[String], observed: &[String]) -> Vec<String> {
    expected
        .iter()
        .filter(|name| !observed.contains(name))
        .cloned()
        .collect()
}

fn parse_sha256(bytes: &[u8]) -> std::result::Result<String, String> {
    let text = String::from_utf8_lossy(bytes);
    let hash = text.split_whitespace().next().unwrap_or_default();
    if hash.len() != 64 || !hash.chars().all(|value| value.is_ascii_hexdigit()) {
        return Err("checksum asset does not start with a valid SHA256 digest".to_owned());
    }
    Ok(hash.to_ascii_lowercase())
}

fn combined_output(output: &std::process::Output) -> String {
    let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
    text.push_str(&String::from_utf8_lossy(&output.stderr));
    text
}

#[cfg(unix)]
fn make_executable(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

#[cfg(not(unix))]
fn make_executable(_path: &Path) -> Result<()> {
    Ok(())
}

fn write_release_report(path: &Path, report: &ReleaseVerificationReport) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("json.tmp");
    fs::write(
        &temporary,
        serde_json::to_vec_pretty(report).map_err(|error| {
            HarnessInfraError::InvalidReleaseInput(format!("could not serialize report: {error}"))
        })?,
    )?;
    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(temporary, path)?;
    Ok(())
}

fn path_for_storage(repo_root: &Path, path: &Path) -> String {
    path.strip_prefix(repo_root)
        .unwrap_or(path)
        .display()
        .to_string()
        .replace('\\', "/")
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::application::{
        BacklogAddInput, BacklogCloseInput, DecisionAddInput, IntakeInput, ReleaseVerifyInput,
        StoryAddInput, StoryUpdateInput, TraceInput,
    };
    use crate::domain::{BacklogFilter, BoolFlag, CsvList, InputType, RiskLane, TraceQualityTier};

    fn test_repository() -> (TempDir, SqliteHarnessRepository) {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .to_path_buf();
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            repo_root.join("scripts/schema"),
        );
        (temp_dir, repository)
    }

    fn story_columns(connection: &Connection) -> Vec<String> {
        let mut statement = connection.prepare("PRAGMA table_info(story);").unwrap();
        let rows = statement
            .query_map([], |row| row.get::<_, String>(1))
            .unwrap();
        rows.collect::<std::result::Result<Vec<_>, _>>().unwrap()
    }

    #[test]
    fn init_creates_database_and_schema() {
        let (_temp_dir, repository) = test_repository();

        let result = repository.init().unwrap();

        assert!(matches!(result, InitResult::Created { .. }));
        assert_eq!(repository.query_stats().unwrap().intakes, 0);
        let connection = repository.open_existing().unwrap();
        let schema_version = SqliteHarnessRepository::schema_version(&connection).unwrap();
        assert_eq!(schema_version, 5);
        let story_columns = story_columns(&connection);
        assert!(story_columns.contains(&"verify_command".to_owned()));
        assert!(story_columns.contains(&"last_verified_at".to_owned()));
        assert!(story_columns.contains(&"last_verified_result".to_owned()));
        assert!(story_columns.contains(&"arch_check_result".to_owned()));
        assert!(story_columns.contains(&"gate_result".to_owned()));
        assert!(story_columns.contains(&"release_proof_required".to_owned()));
        let release_table_exists = connection
            .query_row(
                "SELECT EXISTS(
                SELECT 1 FROM sqlite_master
                WHERE type='table' AND name='release_verification'
             );",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap();
        assert_eq!(release_table_exists, 1);
    }

    #[test]
    fn migrate_applies_all_pending_columns_to_existing_database() {
        let (_temp_dir, repository) = test_repository();
        let connection = repository.open_or_create().unwrap();
        repository.apply_schema_v1(&connection).unwrap();
        drop(connection);

        let result = repository.migrate().unwrap();

        assert_eq!(result.current_version, 1);
        assert_eq!(result.applied, vec![2, 3, 4, 5]);
        let connection = repository.open_existing().unwrap();
        assert_eq!(
            SqliteHarnessRepository::schema_version(&connection).unwrap(),
            5
        );
        let story_columns = story_columns(&connection);
        assert!(story_columns.contains(&"verify_command".to_owned()));
        assert!(story_columns.contains(&"last_verified_at".to_owned()));
        assert!(story_columns.contains(&"last_verified_result".to_owned()));
        assert!(story_columns.contains(&"context_pack_path".to_owned()));
        assert!(story_columns.contains(&"gate_result".to_owned()));
        assert!(story_columns.contains(&"release_proof_required".to_owned()));
    }

    #[test]
    fn records_and_queries_intake() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        let id = repository
            .record_intake(IntakeInput {
                input_type: InputType::HarnessImprovement,
                summary: "Port one CLI slice".to_owned(),
                risk_lane: RiskLane::HighRisk,
                risk_flags: CsvList::from_optional(Some("public contracts".to_owned())),
                affected_docs: CsvList::from_optional(None),
                story_id: Some("US-002".to_owned()),
                notes: None,
                code_impact_summary: None,
                grounded_context: None,
                auto_generated: false,
            })
            .unwrap();

        let intakes = repository.query_intakes().unwrap();
        assert_eq!(id, 1);
        assert_eq!(intakes[0].summary, "Port one CLI slice");
        assert_eq!(intakes[0].input_type, "harness_improvement");
        assert_eq!(intakes[0].risk_lane, "high_risk");

        let connection = repository.open_existing().unwrap();
        let missing_lists_are_null: (bool, bool) = connection
            .query_row(
                "SELECT risk_flags IS NULL, affected_docs IS NULL FROM intake WHERE id=?1;",
                params![id],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(missing_lists_are_null, (false, true));
    }

    #[test]
    fn decision_verify_runs_from_repo_root() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(&repo_root).unwrap();
        let schema_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .to_path_buf()
            .join("scripts/schema");
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            schema_root,
        );
        repository.init().unwrap();

        let pwd_output = repo_root.join("verify-pwd.txt");
        let verify_command = if cfg!(windows) {
            "cd > verify-pwd.txt".to_owned()
        } else {
            "pwd > verify-pwd.txt".to_owned()
        };
        repository
            .add_decision(DecisionAddInput {
                id: "0001-test".to_owned(),
                title: "Verify from root".to_owned(),
                status: "accepted".to_owned(),
                doc_path: None,
                verify_command: Some(verify_command),
                predicted_impact: None,
                notes: None,
            })
            .unwrap();

        let result = repository.verify_decision("0001-test").unwrap();

        assert_eq!(result.result, "pass");
        assert_eq!(
            fs::canonicalize(fs::read_to_string(pwd_output).unwrap().trim()).unwrap(),
            fs::canonicalize(repo_root).unwrap()
        );
    }

    #[test]
    fn story_add_update_and_verify_status_store_verify_command() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        repository
            .add_story(StoryAddInput {
                id: "US-VERIFY".to_owned(),
                title: "Verify command story".to_owned(),
                risk_lane: RiskLane::Normal,
                contract_doc: None,
                verify_command: Some("echo ok".to_owned()),
                notes: None,
                release_proof_required: BoolFlag(0),
            })
            .unwrap();
        assert_eq!(
            repository
                .story_verify_status("US-VERIFY")
                .unwrap()
                .verify_command
                .as_deref(),
            Some("echo ok")
        );

        repository
            .update_story(StoryUpdateInput {
                id: "US-VERIFY".to_owned(),
                status: None,
                evidence: None,
                unit: None,
                integration: None,
                e2e: None,
                platform: None,
                verify_command: Some("npm test".to_owned()),
                release_proof_required: None,
            })
            .unwrap();

        assert_eq!(
            repository
                .story_verify_status("US-VERIFY")
                .unwrap()
                .verify_command
                .as_deref(),
            Some("npm test")
        );
    }

    #[test]
    fn story_verify_records_pass_fail_and_missing_command() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(&repo_root).unwrap();
        let schema_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .to_path_buf()
            .join("scripts/schema");
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            schema_root,
        );
        repository.init().unwrap();

        let pwd_output = repo_root.join("story-verify-pwd.txt");
        let verify_command = if cfg!(windows) {
            "cd > story-verify-pwd.txt".to_owned()
        } else {
            "pwd > story-verify-pwd.txt".to_owned()
        };
        repository
            .add_story(StoryAddInput {
                id: "US-PASS".to_owned(),
                title: "Passing story".to_owned(),
                risk_lane: RiskLane::Normal,
                contract_doc: None,
                verify_command: Some(verify_command),
                notes: None,
                release_proof_required: BoolFlag(0),
            })
            .unwrap();
        let pass = repository.verify_story("US-PASS").unwrap();
        assert_eq!(pass.result, "pass");
        assert_eq!(
            fs::canonicalize(fs::read_to_string(pwd_output).unwrap().trim()).unwrap(),
            fs::canonicalize(repo_root).unwrap()
        );
        assert_eq!(
            repository
                .story_verify_status("US-PASS")
                .unwrap()
                .last_verified_result
                .as_deref(),
            Some("pass")
        );

        repository
            .add_story(StoryAddInput {
                id: "US-FAIL".to_owned(),
                title: "Failing story".to_owned(),
                risk_lane: RiskLane::Normal,
                contract_doc: None,
                verify_command: Some("exit 1".to_owned()),
                notes: None,
                release_proof_required: BoolFlag(0),
            })
            .unwrap();
        let fail = repository.verify_story("US-FAIL").unwrap();
        assert_eq!(fail.result, "fail");
        assert_eq!(
            repository
                .story_verify_status("US-FAIL")
                .unwrap()
                .last_verified_result
                .as_deref(),
            Some("fail")
        );

        repository
            .add_story(StoryAddInput {
                id: "US-MISSING".to_owned(),
                title: "Missing command story".to_owned(),
                risk_lane: RiskLane::Normal,
                contract_doc: None,
                verify_command: None,
                notes: None,
                release_proof_required: BoolFlag(0),
            })
            .unwrap();
        assert!(matches!(
            repository.verify_story("US-MISSING"),
            Err(HarnessInfraError::MissingStoryVerifyCommand(id)) if id == "US-MISSING"
        ));
    }

    #[test]
    fn story_backlog_trace_and_queries_work() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        repository
            .add_story(StoryAddInput {
                id: "US-T".to_owned(),
                title: "Test story".to_owned(),
                risk_lane: RiskLane::Normal,
                contract_doc: None,
                verify_command: None,
                notes: None,
                release_proof_required: BoolFlag(0),
            })
            .unwrap();
        repository
            .update_story(StoryUpdateInput {
                id: "US-T".to_owned(),
                status: Some("implemented".to_owned()),
                evidence: Some("unit test".to_owned()),
                unit: Some(BoolFlag(1)),
                integration: None,
                e2e: None,
                platform: None,
                verify_command: None,
                release_proof_required: None,
            })
            .unwrap();
        assert_eq!(repository.query_matrix().unwrap()[0].unit, 1);

        let backlog_id = repository
            .add_backlog(BacklogAddInput {
                title: "Improve CLI".to_owned(),
                discovered_while: None,
                current_pain: Some("manual SQL".to_owned()),
                suggestion: None,
                risk: Some(RiskLane::HighRisk),
                predicted_impact: None,
                notes: None,
            })
            .unwrap();
        repository
            .close_backlog(BacklogCloseInput {
                id: backlog_id,
                status: "implemented".to_owned(),
                actual_outcome: Some("done".to_owned()),
            })
            .unwrap();
        assert_eq!(
            repository.query_backlog(BacklogFilter::All).unwrap()[0]
                .actual_outcome
                .as_deref(),
            Some("done")
        );

        let trace_id = repository
            .record_trace(TraceInput {
                task_summary: "Test trace".to_owned(),
                intake_id: None,
                story_id: Some("US-T".to_owned()),
                agent: Some("test".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: Some("none".to_owned()),
                notes: None,
                actions: CsvList::from_optional(Some("one,two".to_owned())),
                files_read: CsvList::from_optional(None),
                files_changed: CsvList::from_optional(None),
                decisions: CsvList::from_optional(None),
                errors: CsvList::from_optional(None),
            })
            .unwrap();
        assert_eq!(trace_id, 1);
        assert_eq!(
            repository.query_traces().unwrap()[0].task_summary,
            "Test trace"
        );
        assert_eq!(
            repository.query_friction().unwrap()[0].harness_friction,
            "none"
        );
    }

    #[test]
    fn friction_query_includes_intake_context_and_filters_null_friction() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();
        let intake_id = repository
            .record_intake(IntakeInput {
                input_type: InputType::ChangeRequest,
                summary: "Friction query context".to_owned(),
                risk_lane: RiskLane::Normal,
                risk_flags: CsvList::from_optional(None),
                affected_docs: CsvList::from_optional(None),
                story_id: None,
                notes: None,
                code_impact_summary: None,
                grounded_context: None,
                auto_generated: false,
            })
            .unwrap();
        repository
            .record_trace(TraceInput {
                task_summary: "Trace without friction".to_owned(),
                intake_id: Some(intake_id),
                story_id: None,
                agent: Some("codex".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: None,
                notes: None,
                actions: CsvList::from_optional(None),
                files_read: CsvList::from_optional(None),
                files_changed: CsvList::from_optional(None),
                decisions: CsvList::from_optional(None),
                errors: CsvList::from_optional(None),
            })
            .unwrap();
        repository
            .record_trace(TraceInput {
                task_summary: "Trace with linked friction".to_owned(),
                intake_id: Some(intake_id),
                story_id: None,
                agent: Some("codex".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: Some("Linked friction".to_owned()),
                notes: None,
                actions: CsvList::from_optional(None),
                files_read: CsvList::from_optional(None),
                files_changed: CsvList::from_optional(None),
                decisions: CsvList::from_optional(None),
                errors: CsvList::from_optional(None),
            })
            .unwrap();
        repository
            .record_trace(TraceInput {
                task_summary: "Trace with unlinked friction".to_owned(),
                intake_id: None,
                story_id: None,
                agent: Some("codex".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: Some("Unlinked friction".to_owned()),
                notes: None,
                actions: CsvList::from_optional(None),
                files_read: CsvList::from_optional(None),
                files_changed: CsvList::from_optional(None),
                decisions: CsvList::from_optional(None),
                errors: CsvList::from_optional(None),
            })
            .unwrap();

        let friction = repository.query_friction().unwrap();

        assert_eq!(friction.len(), 2);
        assert_eq!(friction[0].risk_lane, None);
        assert_eq!(friction[0].input_type, None);
        assert_eq!(friction[1].risk_lane.as_deref(), Some("normal"));
        assert_eq!(friction[1].input_type.as_deref(), Some("change_request"));
    }

    #[test]
    fn import_brownfield_seeds_markdown_state_idempotently() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(repo_root.join("docs/decisions")).unwrap();
        fs::write(
            repo_root.join("docs/TEST_MATRIX.md"),
            r#"# Test Matrix

| Story | Contract | Unit | Integration | E2E | Platform | Status | Evidence |
| --- | --- | --- | --- | --- | --- | --- | --- |
| US-010 | docs/product/tasks.md | yes | pending | no | mac smoke | implemented | cargo test |
"#,
        )
        .unwrap();
        fs::write(
            repo_root.join("docs/decisions/0007-test-decision.md"),
            r#"# Test Decision

## Status

Accepted
"#,
        )
        .unwrap();
        fs::write(
            repo_root.join("docs/HARNESS_BACKLOG.md"),
            r#"# Harness Backlog

## Items

### Title

Import existing docs

### Discovered While

Testing brownfield import

### Current Pain

Existing Harness v0 repos have markdown truth.

### Suggested Improvement

Seed the durable database.

### Risk

normal

### Status

accepted

### Title

Keep installer checksum

### Discovered While

Testing release install

### Current Pain

Downloads need verification.

### Suggested Improvement

Verify sha256 files.

### Risk

high-risk

### Status

implemented
"#,
        )
        .unwrap();

        let source_repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .to_path_buf();
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            source_repo_root.join("scripts/schema"),
        );
        repository.init().unwrap();

        let first = repository.import_brownfield().unwrap();
        let second = repository.import_brownfield().unwrap();

        assert_eq!(
            first,
            BrownfieldImportResult {
                stories: 1,
                decisions: 1,
                backlog_items: 2,
            }
        );
        assert_eq!(second.backlog_items, 2);

        let matrix = repository.query_matrix().unwrap();
        assert_eq!(matrix[0].id, "US-010");
        assert_eq!(matrix[0].title, "docs/product/tasks.md");
        assert_eq!(matrix[0].status, "implemented");
        assert_eq!(matrix[0].unit, 1);
        assert_eq!(matrix[0].integration, 0);
        assert_eq!(matrix[0].platform, 1);

        let decisions = repository.query_decisions().unwrap();
        assert_eq!(decisions[0].id, "0007-test-decision");
        assert_eq!(decisions[0].status, "accepted");

        let backlog = repository.query_backlog(BacklogFilter::All).unwrap();
        assert_eq!(backlog.len(), 2);
        assert!(backlog
            .iter()
            .any(|item| item.title == "Import existing docs"
                && item.status == "accepted"
                && item.risk.as_deref() == Some("normal")));
        assert!(backlog
            .iter()
            .any(|item| item.title == "Keep installer checksum"
                && item.status == "implemented"
                && item.risk.as_deref() == Some("high_risk")));
    }

    #[test]
    fn filters_open_and_closed_backlog_items() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();

        let proposed_id = repository
            .add_backlog(BacklogAddInput {
                title: "Proposed item".to_owned(),
                discovered_while: None,
                current_pain: None,
                suggestion: None,
                risk: Some(RiskLane::Tiny),
                predicted_impact: Some("Should improve trace review.".to_owned()),
                notes: None,
            })
            .unwrap();
        let implemented_id = repository
            .add_backlog(BacklogAddInput {
                title: "Implemented item".to_owned(),
                discovered_while: None,
                current_pain: None,
                suggestion: None,
                risk: Some(RiskLane::Normal),
                predicted_impact: Some("Should reduce missing proof.".to_owned()),
                notes: None,
            })
            .unwrap();
        repository
            .close_backlog(BacklogCloseInput {
                id: implemented_id,
                status: "implemented".to_owned(),
                actual_outcome: Some("Proof gaps were found earlier.".to_owned()),
            })
            .unwrap();

        let all = repository.query_backlog(BacklogFilter::All).unwrap();
        let open = repository.query_backlog(BacklogFilter::Open).unwrap();
        let closed = repository.query_backlog(BacklogFilter::Closed).unwrap();

        assert_eq!(all.len(), 2);
        assert_eq!(open.len(), 1);
        assert_eq!(open[0].id, proposed_id);
        assert_eq!(closed.len(), 1);
        assert_eq!(closed[0].id, implemented_id);
        assert_eq!(
            closed[0].actual_outcome.as_deref(),
            Some("Proof gaps were found earlier.")
        );
    }

    #[test]
    fn scores_latest_and_specific_trace_with_lane_lookup() {
        let (_temp_dir, repository) = test_repository();
        repository.init().unwrap();
        let intake_id = repository
            .record_intake(IntakeInput {
                input_type: InputType::HarnessImprovement,
                summary: "High risk trace quality test".to_owned(),
                risk_lane: RiskLane::HighRisk,
                risk_flags: CsvList::from_optional(None),
                affected_docs: CsvList::from_optional(None),
                story_id: None,
                notes: None,
                code_impact_summary: None,
                grounded_context: None,
                auto_generated: false,
            })
            .unwrap();
        let first_trace = repository
            .record_trace(TraceInput {
                task_summary: "Minimal trace test".to_owned(),
                intake_id: None,
                story_id: None,
                agent: None,
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: None,
                notes: None,
                actions: CsvList::from_optional(None),
                files_read: CsvList::from_optional(None),
                files_changed: CsvList::from_optional(None),
                decisions: CsvList::from_optional(None),
                errors: CsvList::from_optional(None),
            })
            .unwrap();
        repository
            .record_trace(TraceInput {
                task_summary: "Standard trace linked to high risk intake".to_owned(),
                intake_id: Some(intake_id),
                story_id: None,
                agent: Some("codex".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: Some("none".to_owned()),
                notes: None,
                actions: CsvList::from_optional(Some("read,patched".to_owned())),
                files_read: CsvList::from_optional(Some("PHASE3.md".to_owned())),
                files_changed: CsvList::from_optional(Some(
                    "crates/harness-cli/src/domain.rs".to_owned(),
                )),
                decisions: CsvList::from_optional(None),
                errors: CsvList::from_optional(None),
            })
            .unwrap();

        let latest = repository.score_trace(None).unwrap();
        assert_eq!(latest.achieved, TraceQualityTier::Standard);
        assert_eq!(latest.required, Some(TraceQualityTier::Detailed));
        assert!(!latest.meets_requirement);
        assert!(latest
            .missing_detailed
            .iter()
            .any(|field| field.starts_with("decisions_made")));

        let specific = repository.score_trace(Some(first_trace)).unwrap();
        assert_eq!(specific.trace_id, first_trace);
        assert_eq!(specific.achieved, TraceQualityTier::Minimal);
        assert_eq!(specific.required, None);
        assert!(specific.meets_requirement);
    }

    #[test]
    fn architecture_check_reports_segment_matched_imports_and_ignores_author_names() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(repo_root.join("src/domain")).unwrap();
        fs::write(
            repo_root.join("src/domain/user.rs"),
            "use crate::infrastructure::db;\nuse crate::author::profile;\n",
        )
        .unwrap();
        fs::write(
            repo_root.join("harness-architecture.toml"),
            r#"
[[layer]]
name = "domain"
path = "src/domain"
forbidden_imports = ["infrastructure", "auth"]
"#,
        )
        .unwrap();
        let schema_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .join("scripts/schema");
        let repository = SqliteHarnessRepository::new(
            repo_root,
            temp_dir.path().join("harness.db"),
            schema_root,
        );

        let result = repository.check_architecture(None, None).unwrap();

        assert!(!result.passed);
        assert_eq!(result.scanned_files, 1);
        assert_eq!(result.violations.len(), 1);
        assert_eq!(result.violations[0].import, "crate::infrastructure::db");
    }

    #[test]
    fn source_import_extracts_common_language_forms() {
        assert_eq!(
            source_import(r#"import { db } from "./infrastructure/db";"#).as_deref(),
            Some("./infrastructure/db")
        );
        assert_eq!(
            source_import("from infrastructure.db import connect").as_deref(),
            Some("infrastructure.db")
        );
        assert_eq!(
            source_import(r#"const db = require("./infrastructure/db");"#).as_deref(),
            Some("./infrastructure/db")
        );
        assert_eq!(
            source_import("use crate::{domain::User, infrastructure::Db};").as_deref(),
            Some("crate::{domain::User, infrastructure::Db}")
        );
    }

    #[test]
    fn story_gate_requires_governance_evidence_and_passes_when_complete() {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(repo_root.join(".harness/context")).unwrap();
        fs::write(
            repo_root.join("harness-architecture.toml"),
            r#"
[[layer]]
name = "domain"
path = "src/domain"
forbidden_imports = ["infrastructure"]
"#,
        )
        .unwrap();
        let schema_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .unwrap()
            .join("scripts/schema");
        let repository = SqliteHarnessRepository::new(
            repo_root.clone(),
            temp_dir.path().join("harness.db"),
            schema_root,
        );
        repository.init().unwrap();
        repository
            .add_story(StoryAddInput {
                id: "US-GATE".to_owned(),
                title: "Governance gate".to_owned(),
                risk_lane: RiskLane::HighRisk,
                contract_doc: None,
                verify_command: Some("exit 0".to_owned()),
                notes: None,
                release_proof_required: BoolFlag(0),
            })
            .unwrap();

        let incomplete = repository.verify_story_gate("US-GATE").unwrap();
        assert!(!incomplete.passed);
        assert!(incomplete.missing.contains(&"intake".to_owned()));
        assert!(incomplete.missing.contains(&"context pack".to_owned()));
        assert!(incomplete
            .missing
            .contains(&"architecture check result".to_owned()));
        assert!(incomplete.missing.contains(&"validation proof".to_owned()));
        assert!(incomplete.missing.contains(&"trace".to_owned()));

        repository
            .record_intake(IntakeInput {
                input_type: InputType::HarnessImprovement,
                summary: "Complete governance gate evidence".to_owned(),
                risk_lane: RiskLane::HighRisk,
                risk_flags: CsvList::from_optional(Some("architecture".to_owned())),
                affected_docs: CsvList::from_optional(None),
                story_id: Some("US-GATE".to_owned()),
                notes: None,
                code_impact_summary: Some("{\"affected_files\":[]}".to_owned()),
                grounded_context: None,
                auto_generated: true,
            })
            .unwrap();
        fs::write(
            repo_root.join(".harness/context/US-GATE-context.md"),
            "# Context",
        )
        .unwrap();
        repository
            .update_story_context_pack_path("US-GATE", ".harness/context/US-GATE-context.md")
            .unwrap();
        repository
            .check_architecture(None, Some("US-GATE"))
            .unwrap();
        repository.verify_story("US-GATE").unwrap();
        repository
            .update_story(StoryUpdateInput {
                id: "US-GATE".to_owned(),
                status: Some("implemented".to_owned()),
                evidence: Some("cargo test --workspace".to_owned()),
                unit: Some(BoolFlag(1)),
                integration: None,
                e2e: None,
                platform: None,
                verify_command: None,
                release_proof_required: None,
            })
            .unwrap();
        repository
            .record_trace(TraceInput {
                task_summary: "Completed governance gate evidence".to_owned(),
                intake_id: None,
                story_id: Some("US-GATE".to_owned()),
                agent: Some("test".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: Some("none".to_owned()),
                notes: None,
                actions: CsvList::from_optional(Some("verify".to_owned())),
                files_read: CsvList::from_optional(Some("story".to_owned())),
                files_changed: CsvList::from_optional(Some("proof".to_owned())),
                decisions: CsvList::from_optional(Some("gate".to_owned())),
                errors: CsvList::from_optional(Some("none".to_owned())),
            })
            .unwrap();

        let complete = repository.verify_story_gate("US-GATE").unwrap();
        assert!(complete.passed, "{:?}", complete.missing);

        repository
            .update_story(StoryUpdateInput {
                id: "US-GATE".to_owned(),
                status: None,
                evidence: None,
                unit: None,
                integration: None,
                e2e: None,
                platform: None,
                verify_command: None,
                release_proof_required: Some(BoolFlag(1)),
            })
            .unwrap();
        let release_missing = repository.verify_story_gate("US-GATE").unwrap();
        assert!(!release_missing.passed);
        assert!(release_missing
            .missing
            .contains(&"release verification proof".to_owned()));

        let connection = repository.open_existing().unwrap();
        connection
            .execute(
                "INSERT INTO release_verification (
                    version, origin, tag, platform, result, report_path,
                    assets_checked, story_id
                 ) VALUES (
                    '0.2.0', 'ntu254/Harness-Intelligence-OS',
                    'harness-cli-v0.2.0', 'windows-x64', 'pass',
                    '.harness/release/test.json', 10, 'US-GATE'
                 );",
                [],
            )
            .unwrap();
        let release_complete = repository.verify_story_gate("US-GATE").unwrap();
        assert!(release_complete.passed, "{:?}", release_complete.missing);
    }

    #[test]
    fn release_contract_validates_versions_assets_and_checksums() {
        assert!(validate_release_version("0.2.0").is_ok());
        assert!(validate_release_version("v0.2.0").is_err());
        assert!(validate_release_origin("ntu254/Harness-Intelligence-OS").is_ok());
        assert!(validate_release_origin("not-an-origin").is_err());
        assert_eq!(expected_release_assets().len(), 10);
        assert!(
            expected_release_assets().contains(&"harness-cli-windows-x64.exe.sha256".to_owned())
        );
        assert_eq!(
            parse_sha256(
                b"0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef  binary"
            )
            .unwrap(),
            "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
        );
        assert!(parse_sha256(b"not-a-hash").is_err());
        assert_eq!(http_status_result(404), ReleaseCheckResult::Fail);
        assert_eq!(http_status_result(429), ReleaseCheckResult::Inconclusive);
        assert_eq!(http_status_result(503), ReleaseCheckResult::Inconclusive);
        let expected = expected_release_assets();
        let mut observed = expected.clone();
        observed.retain(|name| name != "harness-cli-linux-x64.sha256");
        assert_eq!(
            missing_expected_assets(&expected, &observed),
            vec!["harness-cli-linux-x64.sha256".to_owned()]
        );
    }

    #[test]
    fn story_linked_release_override_mismatch_fails_without_network() {
        let (temp_dir, repository) = test_repository();
        repository.init().unwrap();
        repository
            .add_story(StoryAddInput {
                id: "US-RELEASE".to_owned(),
                title: "Release evidence".to_owned(),
                risk_lane: RiskLane::HighRisk,
                contract_doc: None,
                verify_command: None,
                notes: None,
                release_proof_required: BoolFlag(1),
            })
            .unwrap();

        let output = temp_dir.path().join("release-report.json");
        let (_, report) = repository
            .verify_release(ReleaseVerifyInput {
                version: "0.2.0".to_owned(),
                origin: Some("example/not-canonical".to_owned()),
                platform: Some("windows-x64".to_owned()),
                output: Some(output.clone()),
                story_id: Some("US-RELEASE".to_owned()),
            })
            .unwrap();

        assert_eq!(report.result, ReleaseCheckResult::Fail);
        assert!(output.is_file());
        let connection = repository.open_existing().unwrap();
        let stored = connection
            .query_row(
                "SELECT result FROM release_verification WHERE story_id='US-RELEASE';",
                [],
                |row| row.get::<_, String>(0),
            )
            .unwrap();
        assert_eq!(stored, "fail");
        assert!(repository
            .verify_story_gate("US-RELEASE")
            .unwrap()
            .missing
            .contains(&"release verification proof".to_owned()));
    }
}
