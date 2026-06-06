use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};

use rusqlite::{params, types::ValueRef, Connection, OptionalExtension};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::application::{
    AutoIntakeEvidence, BacklogAddInput, BacklogCloseInput, BrownfieldImportResult,
    CodeGraphImpactInput, CodeGraphImpactResult, ContextIngestInput, ContextIngestSummary,
    ContextPackData, DecisionAddInput, DecisionVerifyResult, FrictionAddInput, HarnessContext,
    InitResult, IntakeInput, MigrateResult, NotebookBriefInput, NotebookBriefResult, QueryTable,
    ReleaseVerifyInput, StoryAddInput, StoryUpdateInput, StoryVerifyResult, TraceInput,
};
use crate::domain::{
    normalize_token, path_has_any_segment, score_trace, ArchitectureCheckResult,
    ArchitectureConfig, ArchitectureViolation, BacklogFilter, BacklogRecord, CodeGraphMode,
    ContextIngestDiagnostic, ContextIngestGovernance, ContextIngestReport, ContextIngestStatus,
    ContextSource, ContextSourceArtifact, DecisionRecord, FrictionEventRecord, FrictionRecord,
    FrictionSeverity, FrictionType, HarnessStats, IntakeRecord, MappedContext,
    ReleaseAssetEvidence, ReleaseCheckResult, ReleaseConfig, ReleaseVerificationReport, RiskLane,
    StoryGateResult, StoryMatrixRecord, StoryVerifyStatus, TraceRecord, TraceScoreResult,
    TraceScoreSource,
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
    #[error("context ingest input is invalid: {0}")]
    InvalidContextIngest(String),
    #[error("friction event input is invalid: {0}")]
    InvalidFrictionEvent(String),
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
    fn ingest_context(&self, input: ContextIngestInput) -> Result<(PathBuf, ContextIngestReport)>;
    fn auto_intake_evidence(&self, story_id: &str) -> Result<AutoIntakeEvidence>;
    fn produce_codegraph_impact(
        &self,
        input: CodeGraphImpactInput,
    ) -> Result<CodeGraphImpactResult>;
    fn produce_notebook_brief(&self, input: NotebookBriefInput) -> Result<NotebookBriefResult>;
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
    fn add_friction_event(&self, input: FrictionAddInput) -> Result<i64>;
    fn score_trace(&self, id: Option<i64>) -> Result<TraceScoreResult>;
    fn story_verify_status(&self, id: &str) -> Result<StoryVerifyStatus>;
    fn query_matrix(&self) -> Result<Vec<StoryMatrixRecord>>;
    fn query_backlog(&self, filter: BacklogFilter) -> Result<Vec<BacklogRecord>>;
    fn query_decisions(&self) -> Result<Vec<DecisionRecord>>;
    fn query_intakes(&self) -> Result<Vec<IntakeRecord>>;
    fn query_traces(&self) -> Result<Vec<TraceRecord>>;
    fn query_friction(&self) -> Result<Vec<FrictionRecord>>;
    fn query_friction_events(&self) -> Result<Vec<FrictionEventRecord>>;
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

    fn latest_passing_mapped_context(
        &self,
        connection: &Connection,
        story_id: &str,
        source: ContextSource,
    ) -> Result<Option<MappedContext>> {
        let latest = connection
            .query_row(
                "SELECT result, report_path
                 FROM context_ingest
                 WHERE story_id=?1 AND source=?2
                 ORDER BY id DESC
                 LIMIT 1;",
                params![story_id, source.as_db_value()],
                |row| Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?)),
            )
            .optional()?;
        let Some((result, report_path)) = latest else {
            return Ok(None);
        };
        if result != ContextIngestStatus::Pass.as_db_value() {
            return Ok(None);
        }
        let path = resolve_repo_path(&self.repo_root, &report_path);
        let bytes = fs::read(&path).map_err(|error| {
            HarnessInfraError::InvalidContextIngest(format!(
                "passing {} ingest report '{}' cannot be read: {error}",
                source.as_db_value(),
                report_path
            ))
        })?;
        let report =
            serde_json::from_slice::<StoredContextIngestReport>(&bytes).map_err(|error| {
                HarnessInfraError::InvalidContextIngest(format!(
                    "passing {} ingest report '{}' is invalid: {error}",
                    source.as_db_value(),
                    report_path
                ))
            })?;
        if report.status != ContextIngestStatus::Pass {
            return Ok(None);
        }
        Ok(report.mapped_context)
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
                release_proof_required, codegraph_ingest_required,
                notebooklm_ingest_required
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9);",
            params![
                input.id,
                input.title,
                input.risk_lane.as_db_value(),
                input.contract_doc,
                input.verify_command,
                input.notes,
                input.release_proof_required.0,
                input.codegraph_ingest_required.0,
                input.notebooklm_ingest_required.0,
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
            && input.codegraph_ingest_required.is_none()
            && input.notebooklm_ingest_required.is_none()
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
                release_proof_required=COALESCE(?8, release_proof_required),
                codegraph_ingest_required=COALESCE(?9, codegraph_ingest_required),
                notebooklm_ingest_required=COALESCE(?10, notebooklm_ingest_required)
             WHERE id=?11;",
            params![
                input.status,
                input.evidence,
                input.unit.map(|value| value.0),
                input.integration.map(|value| value.0),
                input.e2e.map(|value| value.0),
                input.platform.map(|value| value.0),
                input.verify_command,
                input.release_proof_required.map(|value| value.0),
                input.codegraph_ingest_required.map(|value| value.0),
                input.notebooklm_ingest_required.map(|value| value.0),
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
                        release_proof_required, codegraph_ingest_required,
                        notebooklm_ingest_required
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
                        row.get::<_, i64>(10)?,
                        row.get::<_, i64>(11)?,
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
        for (required, source, label) in [
            (story.10, "codegraph", "CodeGraph context ingest proof"),
            (story.11, "notebooklm", "NotebookLM context ingest proof"),
        ] {
            if required == 1 {
                let ingest_pass = connection.query_row(
                    "SELECT EXISTS(
                        SELECT 1 FROM context_ingest
                        WHERE story_id=?1 AND source=?2 AND result='pass'
                     );",
                    params![id, source],
                    |row| row.get::<_, i64>(0),
                )? == 1;
                if !ingest_pass {
                    missing.push(label.to_owned());
                }
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

    fn ingest_context(&self, input: ContextIngestInput) -> Result<(PathBuf, ContextIngestReport)> {
        let connection = self.open_existing()?;
        let story_exists = connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM story WHERE id=?1);",
            params![input.story_id],
            |row| row.get::<_, i64>(0),
        )? == 1;
        if !story_exists {
            return Err(HarnessInfraError::StoryNotFound(input.story_id));
        }
        if !input.file.is_file() {
            return Err(HarnessInfraError::InvalidContextIngest(format!(
                "artifact file does not exist: {}",
                input.file.display()
            )));
        }

        let bytes = fs::read(&input.file)?;
        let artifact_sha256 = format!("{:x}", Sha256::digest(&bytes));
        let artifact_path = path_for_storage(&self.repo_root, &input.file);
        let checked_at =
            connection.query_row("SELECT strftime('%Y-%m-%dT%H:%M:%SZ','now');", [], |row| {
                row.get::<_, String>(0)
            })?;
        let mut validation = validate_context_artifact(
            input.source,
            &input.story_id,
            &bytes,
            &artifact_sha256,
            &self.repo_root,
        );
        let intake_exists = connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM intake WHERE story_id=?1);",
            params![input.story_id],
            |row| row.get::<_, i64>(0),
        )? == 1;
        if validation.status == ContextIngestStatus::Pass && !intake_exists {
            validation.status = ContextIngestStatus::Fail;
            validation.mapped_context = None;
            validation.diagnostics.push(diagnostic(
                "INTAKE_NOT_FOUND",
                "passing context cannot be mapped because the story has no linked intake",
            ));
        }
        let output_path = input.output.unwrap_or_else(|| {
            self.repo_root.join(format!(
                ".harness/context/{}-{}-ingest-result.json",
                input.story_id,
                input.source.as_db_value()
            ))
        });
        let ingest_id = uuid_from_sha256(&format!(
            "{}:{}:{}",
            input.story_id,
            input.source.as_db_value(),
            artifact_sha256
        ));
        let eligible = validation.status == ContextIngestStatus::Pass;
        let report = ContextIngestReport {
            schema_version: "1.0.0".to_owned(),
            artifact_type: "context-ingest-result".to_owned(),
            ingest_id,
            story_id: input.story_id.clone(),
            source: input.source,
            source_artifact: ContextSourceArtifact {
                artifact_type: input.source.artifact_type().to_owned(),
                artifact_id: validation.artifact_id.clone(),
                schema_version: validation.schema_version.clone(),
                path: artifact_path.clone(),
                sha256: artifact_sha256.clone(),
            },
            status: validation.status,
            checked_at,
            mapped_context: validation.mapped_context.clone(),
            diagnostics: validation.diagnostics.clone(),
            governance: ContextIngestGovernance {
                eligible_for_intake: eligible,
                eligible_for_context_pack: eligible,
                eligible_for_story_verify: eligible,
            },
        };

        write_context_ingest_report(&output_path, &report)?;
        let report_path = path_for_storage(&self.repo_root, &output_path);
        let failure = if validation.diagnostics.is_empty() {
            None
        } else {
            Some(
                validation
                    .diagnostics
                    .iter()
                    .map(|item| format!("{}: {}", item.code, item.message))
                    .collect::<Vec<_>>()
                    .join("; "),
            )
        };
        let transaction = connection.unchecked_transaction()?;
        transaction.execute(
            "INSERT INTO context_ingest (
                story_id, source, artifact_type, artifact_id, artifact_path,
                artifact_sha256, schema_version, result, provenance_status,
                summary, report_path, failure
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12);",
            params![
                input.story_id,
                input.source.as_db_value(),
                input.source.artifact_type(),
                validation.artifact_id,
                artifact_path,
                artifact_sha256,
                validation.schema_version,
                validation.status.as_db_value(),
                validation.provenance_status.as_db_value(),
                validation.summary,
                report_path,
                failure,
            ],
        )?;
        if let Some(mapped) = &validation.mapped_context {
            update_intake_from_mapped_context(&transaction, &input.story_id, mapped)?;
        }
        transaction.commit()?;

        Ok((output_path, report))
    }

    fn auto_intake_evidence(&self, story_id: &str) -> Result<AutoIntakeEvidence> {
        let connection = self.open_existing()?;
        let story_exists = connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM story WHERE id=?1);",
            params![story_id],
            |row| row.get::<_, i64>(0),
        )? == 1;
        if !story_exists {
            return Err(HarnessInfraError::StoryNotFound(story_id.to_owned()));
        }

        Ok(AutoIntakeEvidence {
            codegraph: self.latest_passing_mapped_context(
                &connection,
                story_id,
                ContextSource::Codegraph,
            )?,
            notebooklm: self.latest_passing_mapped_context(
                &connection,
                story_id,
                ContextSource::Notebooklm,
            )?,
        })
    }

    fn produce_codegraph_impact(
        &self,
        input: CodeGraphImpactInput,
    ) -> Result<CodeGraphImpactResult> {
        let connection = self.open_existing()?;
        let story_exists = connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM story WHERE id=?1);",
            params![input.story_id],
            |row| row.get::<_, i64>(0),
        )? == 1;
        if !story_exists {
            return Err(HarnessInfraError::StoryNotFound(input.story_id));
        }
        let generated_at =
            connection.query_row("SELECT strftime('%Y-%m-%dT%H:%M:%SZ','now');", [], |row| {
                row.get::<_, String>(0)
            })?;
        drop(connection);

        let artifact_path = input.output.unwrap_or_else(|| {
            self.repo_root.join(format!(
                ".harness/context/{}-codegraph-impact.json",
                input.story_id
            ))
        });
        let raw_output_path = input.raw_output.unwrap_or_else(|| {
            self.repo_root.join(format!(
                ".harness/context/{}-codegraph-provider-response.json",
                input.story_id
            ))
        });
        let repository = git_output(&self.repo_root, &["config", "--get", "remote.origin.url"])
            .unwrap_or_else(|| self.repo_root.display().to_string());
        let revision = git_output(&self.repo_root, &["rev-parse", "HEAD"])
            .unwrap_or_else(|| "unknown".to_owned());
        let provider_version =
            command_output(&input.executable, &["--version"], &self.repo_root, None)
                .ok()
                .filter(|output| output.status.success())
                .map(|output| first_non_empty_line(&output.stdout))
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| "unknown".to_owned());

        let (provider_args, stdin, request_label) = match input.mode {
            CodeGraphMode::ChangedFiles => {
                let changed_files_path = input.changed_files.ok_or_else(|| {
                    HarnessInfraError::InvalidContextIngest(
                        "CodeGraph changed-files mode requires --changed-files".to_owned(),
                    )
                })?;
                let changed_files = fs::read_to_string(&changed_files_path)?;
                if changed_files.lines().all(|line| line.trim().is_empty()) {
                    return Err(HarnessInfraError::InvalidContextIngest(
                        "CodeGraph changed-files input must contain at least one path".to_owned(),
                    ));
                }
                (
                    vec![
                        "affected".to_owned(),
                        "--path".to_owned(),
                        self.repo_root.display().to_string(),
                        "--stdin".to_owned(),
                        "--depth".to_owned(),
                        input.depth.to_string(),
                        "--json".to_owned(),
                    ],
                    Some(changed_files.into_bytes()),
                    format!(
                        "changed-files:{}",
                        path_for_storage(&self.repo_root, &changed_files_path)
                    ),
                )
            }
            CodeGraphMode::Symbol => {
                let symbol = input.symbol.ok_or_else(|| {
                    HarnessInfraError::InvalidContextIngest(
                        "CodeGraph symbol mode requires --symbol".to_owned(),
                    )
                })?;
                if symbol.trim().is_empty() {
                    return Err(HarnessInfraError::InvalidContextIngest(
                        "CodeGraph symbol must not be empty".to_owned(),
                    ));
                }
                (
                    vec![
                        "impact".to_owned(),
                        symbol.clone(),
                        "--path".to_owned(),
                        self.repo_root.display().to_string(),
                        "--depth".to_owned(),
                        input.depth.to_string(),
                        "--json".to_owned(),
                    ],
                    None,
                    format!("symbol:{symbol}"),
                )
            }
        };
        let provider_command = format!("{} {}", input.executable, provider_args.join(" "));
        let invocation_id = uuid_from_sha256(&format!(
            "{}:{}:{}:{}",
            input.story_id,
            input.mode.as_cli_value(),
            request_label,
            generated_at
        ));
        let provider_result = command_output(
            &input.executable,
            &provider_args.iter().map(String::as_str).collect::<Vec<_>>(),
            &self.repo_root,
            stdin.as_deref(),
        );

        let mut raw_path = None;
        let artifact = match provider_result {
            Err(error) => codegraph_unavailable_artifact(
                &input.story_id,
                &generated_at,
                &repository,
                &revision,
                &provider_version,
                &invocation_id,
                &format!("CodeGraph executable could not be started: {error}"),
            ),
            Ok(output) if !output.status.success() => {
                let detail = String::from_utf8_lossy(&output.stderr).trim().to_owned();
                if !output.stdout.is_empty() || !output.stderr.is_empty() {
                    write_provider_response(
                        &raw_output_path,
                        &output.stdout,
                        &output.stderr,
                        output.status.code(),
                    )?;
                    raw_path = Some(raw_output_path.clone());
                }
                codegraph_unavailable_artifact(
                    &input.story_id,
                    &generated_at,
                    &repository,
                    &revision,
                    &provider_version,
                    &invocation_id,
                    if detail.is_empty() {
                        "CodeGraph command exited non-zero"
                    } else {
                        &detail
                    },
                )
            }
            Ok(output) => {
                write_provider_response(
                    &raw_output_path,
                    &output.stdout,
                    &output.stderr,
                    output.status.code(),
                )?;
                raw_path = Some(raw_output_path.clone());
                normalize_codegraph_output(
                    input.mode,
                    &input.story_id,
                    &generated_at,
                    &repository,
                    &revision,
                    &provider_version,
                    &invocation_id,
                    &raw_output_path,
                    &path_for_storage(&self.repo_root, &raw_output_path),
                    &output.stdout,
                )
            }
        };
        write_json_value(&artifact_path, &artifact)?;
        let (ingest_report_path, ingest_report) = self.ingest_context(ContextIngestInput {
            story_id: input.story_id,
            source: ContextSource::Codegraph,
            file: artifact_path.clone(),
            output: None,
        })?;

        Ok(CodeGraphImpactResult {
            artifact_path,
            raw_output_path: raw_path,
            provider_version,
            provider_command,
            ingest_report_path,
            ingest_report,
        })
    }

    fn produce_notebook_brief(&self, input: NotebookBriefInput) -> Result<NotebookBriefResult> {
        if input.query.trim().is_empty() {
            return Err(HarnessInfraError::InvalidContextIngest(
                "NotebookLM brief query must not be empty".to_owned(),
            ));
        }
        if input.notebook.trim().is_empty() {
            return Err(HarnessInfraError::InvalidContextIngest(
                "NotebookLM notebook id must not be empty".to_owned(),
            ));
        }
        if let Some(profile) = &input.profile {
            if profile.trim().is_empty() {
                return Err(HarnessInfraError::InvalidContextIngest(
                    "NotebookLM profile must not be empty when provided".to_owned(),
                ));
            }
        }
        if let Some(timeout) = input.timeout_seconds {
            if !timeout.is_finite() || timeout <= 0.0 {
                return Err(HarnessInfraError::InvalidContextIngest(
                    "NotebookLM timeout must be a positive number".to_owned(),
                ));
            }
        }
        let connection = self.open_existing()?;
        let story_exists = connection.query_row(
            "SELECT EXISTS(SELECT 1 FROM story WHERE id=?1);",
            params![input.story_id],
            |row| row.get::<_, i64>(0),
        )? == 1;
        if !story_exists {
            return Err(HarnessInfraError::StoryNotFound(input.story_id));
        }
        let generated_at =
            connection.query_row("SELECT strftime('%Y-%m-%dT%H:%M:%SZ','now');", [], |row| {
                row.get::<_, String>(0)
            })?;
        drop(connection);

        let artifact_path = input.output.unwrap_or_else(|| {
            self.repo_root.join(format!(
                ".harness/context/{}-notebooklm-brief.json",
                input.story_id
            ))
        });
        let raw_output_path = input.raw_output.unwrap_or_else(|| {
            self.repo_root.join(format!(
                ".harness/context/{}-notebooklm-provider-response.json",
                input.story_id
            ))
        });
        let provider_version =
            command_output(&input.executable, &["--version"], &self.repo_root, None)
                .ok()
                .filter(|output| output.status.success())
                .map(|output| first_non_empty_line(&output.stdout))
                .filter(|value| !value.is_empty())
                .unwrap_or_else(|| "unknown".to_owned());

        let mut provider_args = vec![
            "query".to_owned(),
            "notebook".to_owned(),
            "--json".to_owned(),
        ];
        if let Some(profile) = &input.profile {
            provider_args.push("--profile".to_owned());
            provider_args.push(profile.clone());
        }
        if let Some(timeout) = input.timeout_seconds {
            provider_args.push("--timeout".to_owned());
            provider_args.push(format_provider_timeout(timeout));
        }
        provider_args.push(input.notebook.clone());
        provider_args.push(input.query.clone());
        let request_label = format!("notebook:{}:query:{}", input.notebook, input.query);
        let provider_command = format!("{} {}", input.executable, provider_args.join(" "));
        let invocation_id = uuid_from_sha256(&format!(
            "{}:{}:{}",
            input.story_id, request_label, generated_at
        ));
        let provider_result = command_output(
            &input.executable,
            &provider_args.iter().map(String::as_str).collect::<Vec<_>>(),
            &self.repo_root,
            None,
        );

        let mut raw_path = None;
        let artifact = match provider_result {
            Err(error) => notebook_unavailable_artifact(
                &input.story_id,
                &generated_at,
                &provider_version,
                &invocation_id,
                "provider_unavailable",
                true,
                &format!("NotebookLM executable could not be started: {error}"),
            ),
            Ok(output) if !output.status.success() => {
                let detail = notebook_provider_failure_detail(&output.stdout, &output.stderr);
                if !output.stdout.is_empty() || !output.stderr.is_empty() {
                    write_provider_response(
                        &raw_output_path,
                        &output.stdout,
                        &output.stderr,
                        output.status.code(),
                    )?;
                    raw_path = Some(raw_output_path.clone());
                }
                let reason = notebook_unavailable_reason(&detail);
                notebook_unavailable_artifact(
                    &input.story_id,
                    &generated_at,
                    &provider_version,
                    &invocation_id,
                    reason,
                    true,
                    if detail.is_empty() {
                        "NotebookLM provider exited non-zero"
                    } else {
                        &detail
                    },
                )
            }
            Ok(output) => {
                write_provider_response(
                    &raw_output_path,
                    &output.stdout,
                    &output.stderr,
                    output.status.code(),
                )?;
                raw_path = Some(raw_output_path.clone());
                normalize_notebook_output(
                    &input.story_id,
                    &generated_at,
                    &provider_version,
                    &invocation_id,
                    &path_for_storage(&self.repo_root, &raw_output_path),
                    &output.stdout,
                )
            }
        };
        write_json_value(&artifact_path, &artifact)?;
        let (ingest_report_path, ingest_report) = self.ingest_context(ContextIngestInput {
            story_id: input.story_id,
            source: ContextSource::Notebooklm,
            file: artifact_path.clone(),
            output: None,
        })?;

        Ok(NotebookBriefResult {
            artifact_path,
            raw_output_path: raw_path,
            provider_version,
            provider_command,
            ingest_report_path,
            ingest_report,
        })
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

    fn add_friction_event(&self, input: FrictionAddInput) -> Result<i64> {
        if input.summary.trim().is_empty() {
            return Err(HarnessInfraError::InvalidFrictionEvent(
                "summary is required".to_owned(),
            ));
        }
        if input.friction_type == FrictionType::ProviderUnavailable
            && input
                .provider
                .as_deref()
                .map(str::trim)
                .unwrap_or("")
                .is_empty()
        {
            return Err(HarnessInfraError::InvalidFrictionEvent(
                "provider_unavailable requires --provider".to_owned(),
            ));
        }
        if let Some(observed_at) = &input.observed_at {
            if !looks_like_rfc3339(observed_at) {
                return Err(HarnessInfraError::InvalidFrictionEvent(
                    "observed_at must be RFC 3339, for example 2026-06-07T00:00:00Z".to_owned(),
                ));
            }
        }

        let event_id = input
            .event_id
            .clone()
            .unwrap_or_else(|| generated_friction_event_id(&input));
        if !is_uuid(&event_id) {
            return Err(HarnessInfraError::InvalidFrictionEvent(
                "event_id must be a UUID".to_owned(),
            ));
        }

        let evidence_json = friction_evidence_json(&input)?;
        if input.severity == FrictionSeverity::High && evidence_json.is_none() {
            return Err(HarnessInfraError::InvalidFrictionEvent(
                "high severity requires evidence".to_owned(),
            ));
        }
        let proposed_action_json = proposed_action_json(&input)?;

        let connection = self.open_existing()?;
        connection.execute(
            "INSERT INTO friction_event (
                event_id, story_id, trace_id, friction_type, severity, source,
                summary, observed_at, provider, affected_paths, evidence_json,
                proposed_action_json, notes
             ) VALUES (
                ?1, ?2, ?3, ?4, ?5, ?6, ?7,
                COALESCE(?8, strftime('%Y-%m-%dT%H:%M:%SZ','now')),
                ?9, ?10, ?11, ?12, ?13
             );",
            params![
                event_id,
                input.story_id,
                input.trace_id,
                input.friction_type.as_db_value(),
                input.severity.as_db_value(),
                input.source.as_db_value(),
                input.summary,
                input.observed_at,
                input.provider,
                input.affected_paths.as_json_text(),
                evidence_json,
                proposed_action_json,
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

    fn query_friction_events(&self) -> Result<Vec<FrictionEventRecord>> {
        let connection = self.open_existing()?;
        let mut statement = connection.prepare(
            "SELECT id, captured_at, event_id, story_id, trace_id, friction_type,
                    severity, source, summary, provider
             FROM friction_event
             ORDER BY id DESC
             LIMIT 50;",
        )?;

        let rows = statement.query_map([], |row| {
            Ok(FrictionEventRecord {
                id: row.get(0)?,
                captured_at: row.get(1)?,
                event_id: row.get(2)?,
                story_id: row.get(3)?,
                trace_id: row.get(4)?,
                friction_type: row.get(5)?,
                severity: row.get(6)?,
                source: row.get(7)?,
                summary: row.get(8)?,
                provider: row.get(9)?,
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
        let mut ingest_statement = connection.prepare(
            "SELECT source, result, schema_version, artifact_path,
                    artifact_sha256, summary, report_path, failure, checked_at
             FROM context_ingest
             WHERE story_id=?1
               AND id IN (
                 SELECT MAX(id) FROM context_ingest
                 WHERE story_id=?1 GROUP BY source
               )
             ORDER BY source;",
        )?;
        let context_ingests =
            collect_rows(ingest_statement.query_map(params![story_id], |row| {
                Ok(ContextIngestSummary {
                    source: row.get(0)?,
                    result: row.get(1)?,
                    schema_version: row.get(2)?,
                    artifact_path: row.get(3)?,
                    artifact_sha256: row.get(4)?,
                    summary: row.get(5)?,
                    report_path: row.get(6)?,
                    failure: row.get(7)?,
                    checked_at: row.get(8)?,
                })
            })?)?;

        let mut friction_statement = connection.prepare(
            "SELECT id, captured_at, event_id, story_id, trace_id, friction_type,
                    severity, source, summary, provider
             FROM friction_event
             WHERE story_id=?1
             ORDER BY id DESC
             LIMIT 10;",
        )?;
        let friction_events =
            collect_rows(friction_statement.query_map(params![story_id], |row| {
                Ok(FrictionEventRecord {
                    id: row.get(0)?,
                    captured_at: row.get(1)?,
                    event_id: row.get(2)?,
                    story_id: row.get(3)?,
                    trace_id: row.get(4)?,
                    friction_type: row.get(5)?,
                    severity: row.get(6)?,
                    source: row.get(7)?,
                    summary: row.get(8)?,
                    provider: row.get(9)?,
                })
            })?)?;

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
            context_ingests,
            friction_events,
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

fn friction_evidence_json(input: &FrictionAddInput) -> Result<Option<String>> {
    let mut object = serde_json::Map::new();
    if let Some(command) = &input.evidence.command {
        object.insert(
            "command".to_owned(),
            serde_json::Value::String(command.clone()),
        );
    }
    if let Some(exit_code) = input.evidence.exit_code {
        object.insert(
            "exit_code".to_owned(),
            serde_json::Value::Number(exit_code.into()),
        );
    }
    if let Some(path) = &input.evidence.artifact_path {
        object.insert(
            "artifact_path".to_owned(),
            serde_json::Value::String(path.clone()),
        );
    }
    if let Some(path) = &input.evidence.report_path {
        object.insert(
            "report_path".to_owned(),
            serde_json::Value::String(path.clone()),
        );
    }
    if let Some(details) = &input.evidence.details {
        object.insert(
            "details".to_owned(),
            serde_json::Value::String(details.clone()),
        );
    }
    if let Some(trace_id) = input.trace_id {
        object.insert(
            "trace_id".to_owned(),
            serde_json::Value::Number(trace_id.into()),
        );
    }
    if object.is_empty() {
        return Ok(None);
    }
    serde_json::to_string(&object)
        .map(Some)
        .map_err(|error| HarnessInfraError::InvalidFrictionEvent(error.to_string()))
}

fn proposed_action_json(input: &FrictionAddInput) -> Result<Option<String>> {
    let has_action = input.proposed_action.action_type.is_some()
        || input.proposed_action.title.is_some()
        || input.proposed_action.target_path.is_some();
    if !has_action {
        return Ok(None);
    }
    let Some(action_type) = input.proposed_action.action_type else {
        return Err(HarnessInfraError::InvalidFrictionEvent(
            "proposed action requires --proposed-action".to_owned(),
        ));
    };

    let mut object = serde_json::Map::new();
    object.insert(
        "action_type".to_owned(),
        serde_json::Value::String(action_type.as_db_value().to_owned()),
    );
    if let Some(title) = &input.proposed_action.title {
        object.insert("title".to_owned(), serde_json::Value::String(title.clone()));
    }
    if let Some(target_path) = &input.proposed_action.target_path {
        object.insert(
            "target_path".to_owned(),
            serde_json::Value::String(target_path.clone()),
        );
    }

    serde_json::to_string(&object)
        .map(Some)
        .map_err(|error| HarnessInfraError::InvalidFrictionEvent(error.to_string()))
}

fn generated_friction_event_id(input: &FrictionAddInput) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
        .to_string();
    let mut hasher = Sha256::new();
    hasher.update(now.as_bytes());
    hasher.update(std::process::id().to_string().as_bytes());
    hasher.update(input.summary.as_bytes());
    if let Some(story_id) = &input.story_id {
        hasher.update(story_id.as_bytes());
    }
    let digest = hasher.finalize();
    let mut bytes = [0u8; 16];
    bytes.copy_from_slice(&digest[..16]);
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
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

#[derive(Clone, Debug)]
struct ContextArtifactValidation {
    artifact_id: String,
    schema_version: String,
    status: ContextIngestStatus,
    provenance_status: ContextIngestStatus,
    summary: Option<String>,
    mapped_context: Option<MappedContext>,
    diagnostics: Vec<ContextIngestDiagnostic>,
}

#[derive(Debug, Deserialize)]
struct StoredContextIngestReport {
    status: ContextIngestStatus,
    mapped_context: Option<MappedContext>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactInputRef {
    uri: String,
    sha256: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CodeGraphProvenance {
    provider: String,
    adapter: String,
    adapter_version: String,
    invocation_id: String,
    repository: String,
    revision: String,
    inputs: Vec<ArtifactInputRef>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CodeGraphAffectedFile {
    path: String,
    change_kind: String,
    reasons: Vec<String>,
    #[serde(default)]
    symbols: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CodeGraphDependencyEdge {
    from: String,
    to: String,
    kind: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactClaim {
    claim_id: String,
    statement: String,
    evidence_refs: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CodeGraphImpact {
    summary: String,
    affected_files: Vec<CodeGraphAffectedFile>,
    #[serde(default)]
    dependency_edges: Vec<CodeGraphDependencyEdge>,
    risk_flags: Vec<String>,
    claims: Vec<ArtifactClaim>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactError {
    code: String,
    message: String,
    retryable: Option<bool>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ArtifactUnavailable {
    reason: String,
    retryable: bool,
    detail: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct CodeGraphArtifact {
    schema_version: String,
    artifact_type: String,
    artifact_id: String,
    story_id: String,
    status: ContextIngestStatus,
    generated_at: String,
    provenance: CodeGraphProvenance,
    impact: Option<CodeGraphImpact>,
    errors: Option<Vec<ArtifactError>>,
    unavailable: Option<ArtifactUnavailable>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NotebookSource {
    source_id: String,
    title: String,
    uri: String,
    sha256: String,
    retrieved_at: String,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NotebookProvenance {
    provider: String,
    adapter: String,
    adapter_version: String,
    invocation_id: String,
    sources: Vec<NotebookSource>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NotebookCitation {
    source_id: String,
    locator: String,
    quote_sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NotebookClaim {
    claim_id: String,
    statement: String,
    citations: Vec<NotebookCitation>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NotebookBrief {
    summary: String,
    constraints: Vec<String>,
    open_questions: Vec<String>,
    #[serde(default)]
    affected_docs: Vec<String>,
    claims: Vec<NotebookClaim>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct NotebookArtifact {
    schema_version: String,
    artifact_type: String,
    artifact_id: String,
    story_id: String,
    status: ContextIngestStatus,
    generated_at: String,
    provenance: NotebookProvenance,
    brief: Option<NotebookBrief>,
    errors: Option<Vec<ArtifactError>>,
    unavailable: Option<ArtifactUnavailable>,
}

fn validate_context_artifact(
    source: ContextSource,
    story_id: &str,
    bytes: &[u8],
    artifact_sha256: &str,
    repo_root: &Path,
) -> ContextArtifactValidation {
    match source {
        ContextSource::Codegraph => {
            let artifact = serde_json::from_slice::<CodeGraphArtifact>(bytes);
            match artifact {
                Ok(artifact) => validate_codegraph_artifact(artifact, story_id, repo_root),
                Err(error) => invalid_artifact_validation(
                    artifact_sha256,
                    format!("CodeGraph artifact does not match schema v1.0.0: {error}"),
                ),
            }
        }
        ContextSource::Notebooklm => {
            let artifact = serde_json::from_slice::<NotebookArtifact>(bytes);
            match artifact {
                Ok(artifact) => validate_notebook_artifact(artifact, story_id),
                Err(error) => invalid_artifact_validation(
                    artifact_sha256,
                    format!("NotebookLM artifact does not match schema v1.0.0: {error}"),
                ),
            }
        }
    }
}

fn invalid_artifact_validation(hash: &str, message: String) -> ContextArtifactValidation {
    ContextArtifactValidation {
        artifact_id: uuid_from_sha256(hash),
        schema_version: "unknown".to_owned(),
        status: ContextIngestStatus::Fail,
        provenance_status: ContextIngestStatus::Fail,
        summary: None,
        mapped_context: None,
        diagnostics: vec![ContextIngestDiagnostic {
            code: "INVALID_ARTIFACT".to_owned(),
            message,
            path: None,
            retryable: Some(false),
        }],
    }
}

fn validate_codegraph_artifact(
    artifact: CodeGraphArtifact,
    expected_story: &str,
    repo_root: &Path,
) -> ContextArtifactValidation {
    let mut diagnostics = validate_common_artifact(
        &artifact.schema_version,
        &artifact.artifact_type,
        "codegraph-impact",
        &artifact.artifact_id,
        &artifact.story_id,
        expected_story,
        &artifact.generated_at,
    );
    validate_codegraph_provenance(&artifact.provenance, artifact.status, &mut diagnostics);
    validate_local_codegraph_inputs(&artifact.provenance.inputs, repo_root, &mut diagnostics);

    let (summary, mapped_context) = match artifact.status {
        ContextIngestStatus::Pass => match artifact.impact {
            Some(impact) => {
                validate_codegraph_impact(&impact, &mut diagnostics);
                let summary = impact.summary.clone();
                let mapped = MappedContext {
                    risk_flags: impact.risk_flags.clone(),
                    affected_files: impact
                        .affected_files
                        .iter()
                        .map(|file| file.path.clone())
                        .collect(),
                    affected_docs: Vec::new(),
                    code_impact_summary: Some(summary.clone()),
                    grounded_context: None,
                    claim_ids: impact
                        .claims
                        .iter()
                        .map(|claim| claim.claim_id.clone())
                        .collect(),
                };
                (Some(summary), Some(mapped))
            }
            None => {
                diagnostics.push(diagnostic(
                    "MISSING_IMPACT",
                    "passing CodeGraph artifact requires impact",
                ));
                (None, None)
            }
        },
        ContextIngestStatus::Fail => {
            validate_declared_failure(artifact.errors.as_deref(), &mut diagnostics);
            if artifact.impact.is_some() || artifact.unavailable.is_some() {
                diagnostics.push(diagnostic(
                    "INVALID_FAILURE_SHAPE",
                    "failed CodeGraph artifact must contain errors only",
                ));
            }
            (None, None)
        }
        ContextIngestStatus::Inconclusive => {
            validate_unavailable(
                artifact.unavailable.as_ref(),
                &[
                    "provider_unavailable",
                    "permission_denied",
                    "timeout",
                    "repository_unavailable",
                    "insufficient_evidence",
                ],
                &mut diagnostics,
            );
            if artifact.impact.is_some() || artifact.errors.is_some() {
                diagnostics.push(diagnostic(
                    "INVALID_INCONCLUSIVE_SHAPE",
                    "inconclusive CodeGraph artifact must contain unavailable only",
                ));
            }
            (None, None)
        }
    };
    finalize_artifact_validation(
        artifact.artifact_id,
        artifact.schema_version,
        artifact.status,
        summary,
        mapped_context,
        diagnostics,
    )
}

fn validate_notebook_artifact(
    artifact: NotebookArtifact,
    expected_story: &str,
) -> ContextArtifactValidation {
    let mut diagnostics = validate_common_artifact(
        &artifact.schema_version,
        &artifact.artifact_type,
        "notebooklm-brief",
        &artifact.artifact_id,
        &artifact.story_id,
        expected_story,
        &artifact.generated_at,
    );
    validate_notebook_provenance(&artifact.provenance, artifact.status, &mut diagnostics);

    let (summary, mapped_context) = match artifact.status {
        ContextIngestStatus::Pass => match artifact.brief {
            Some(brief) => {
                validate_notebook_brief(&brief, &artifact.provenance, &mut diagnostics);
                let summary = brief.summary.clone();
                let grounded_context = render_grounded_context(&brief);
                let mapped = MappedContext {
                    risk_flags: Vec::new(),
                    affected_files: Vec::new(),
                    affected_docs: brief.affected_docs.clone(),
                    code_impact_summary: None,
                    grounded_context: Some(grounded_context),
                    claim_ids: brief
                        .claims
                        .iter()
                        .map(|claim| claim.claim_id.clone())
                        .collect(),
                };
                (Some(summary), Some(mapped))
            }
            None => {
                diagnostics.push(diagnostic(
                    "MISSING_BRIEF",
                    "passing NotebookLM artifact requires brief",
                ));
                (None, None)
            }
        },
        ContextIngestStatus::Fail => {
            validate_declared_failure(artifact.errors.as_deref(), &mut diagnostics);
            if artifact.brief.is_some() || artifact.unavailable.is_some() {
                diagnostics.push(diagnostic(
                    "INVALID_FAILURE_SHAPE",
                    "failed NotebookLM artifact must contain errors only",
                ));
            }
            (None, None)
        }
        ContextIngestStatus::Inconclusive => {
            validate_unavailable(
                artifact.unavailable.as_ref(),
                &[
                    "provider_unavailable",
                    "permission_denied",
                    "timeout",
                    "source_unavailable",
                    "insufficient_evidence",
                ],
                &mut diagnostics,
            );
            if artifact.brief.is_some() || artifact.errors.is_some() {
                diagnostics.push(diagnostic(
                    "INVALID_INCONCLUSIVE_SHAPE",
                    "inconclusive NotebookLM artifact must contain unavailable only",
                ));
            }
            (None, None)
        }
    };
    finalize_artifact_validation(
        artifact.artifact_id,
        artifact.schema_version,
        artifact.status,
        summary,
        mapped_context,
        diagnostics,
    )
}

fn validate_common_artifact(
    schema_version: &str,
    artifact_type: &str,
    expected_type: &str,
    artifact_id: &str,
    story_id: &str,
    expected_story: &str,
    generated_at: &str,
) -> Vec<ContextIngestDiagnostic> {
    let mut diagnostics = Vec::new();
    if schema_version != "1.0.0" {
        diagnostics.push(diagnostic(
            "UNSUPPORTED_SCHEMA_VERSION",
            "schema_version must be 1.0.0",
        ));
    }
    if artifact_type != expected_type {
        diagnostics.push(diagnostic(
            "ARTIFACT_TYPE_MISMATCH",
            &format!("artifact_type must be {expected_type}"),
        ));
    }
    if !is_uuid(artifact_id) {
        diagnostics.push(diagnostic(
            "INVALID_ARTIFACT_ID",
            "artifact_id must be a UUID",
        ));
    }
    if story_id != expected_story {
        diagnostics.push(diagnostic(
            "STORY_MISMATCH",
            &format!("artifact story '{story_id}' does not match '{expected_story}'"),
        ));
    }
    if !looks_like_rfc3339(generated_at) {
        diagnostics.push(diagnostic(
            "INVALID_GENERATED_AT",
            "generated_at must be an RFC3339 UTC timestamp",
        ));
    }
    diagnostics
}

fn validate_codegraph_provenance(
    provenance: &CodeGraphProvenance,
    status: ContextIngestStatus,
    diagnostics: &mut Vec<ContextIngestDiagnostic>,
) {
    for (field, value) in [
        ("provider", provenance.provider.as_str()),
        ("adapter", provenance.adapter.as_str()),
        ("adapter_version", provenance.adapter_version.as_str()),
        ("invocation_id", provenance.invocation_id.as_str()),
        ("repository", provenance.repository.as_str()),
        ("revision", provenance.revision.as_str()),
    ] {
        require_nonempty(field, value, diagnostics);
    }
    if status == ContextIngestStatus::Pass && provenance.inputs.is_empty() {
        diagnostics.push(diagnostic(
            "MISSING_PROVENANCE_INPUT",
            "passing CodeGraph artifact requires at least one hashed input",
        ));
    }
    for input in &provenance.inputs {
        require_nonempty("provenance.inputs.uri", &input.uri, diagnostics);
        validate_sha256("provenance.inputs.sha256", &input.sha256, diagnostics);
    }
}

fn validate_local_codegraph_inputs(
    inputs: &[ArtifactInputRef],
    repo_root: &Path,
    diagnostics: &mut Vec<ContextIngestDiagnostic>,
) {
    for input in inputs {
        if input.uri.contains("://") || input.uri.starts_with("git:") {
            continue;
        }
        let candidate = PathBuf::from(&input.uri);
        let path = if candidate.is_absolute() {
            candidate
        } else {
            repo_root.join(candidate)
        };
        if !path.is_file() {
            continue;
        }
        match fs::read(&path) {
            Ok(bytes) => {
                let actual = format!("{:x}", Sha256::digest(bytes));
                if actual != input.sha256 {
                    diagnostics.push(diagnostic(
                        "PROVENANCE_SHA256_MISMATCH",
                        &format!(
                            "provenance input '{}' SHA256 does not match the referenced file",
                            input.uri
                        ),
                    ));
                }
            }
            Err(error) => diagnostics.push(diagnostic(
                "PROVENANCE_INPUT_UNREADABLE",
                &format!("cannot read provenance input '{}': {error}", input.uri),
            )),
        }
    }
}

fn validate_notebook_provenance(
    provenance: &NotebookProvenance,
    status: ContextIngestStatus,
    diagnostics: &mut Vec<ContextIngestDiagnostic>,
) {
    for (field, value) in [
        ("provider", provenance.provider.as_str()),
        ("adapter", provenance.adapter.as_str()),
        ("adapter_version", provenance.adapter_version.as_str()),
        ("invocation_id", provenance.invocation_id.as_str()),
    ] {
        require_nonempty(field, value, diagnostics);
    }
    if status == ContextIngestStatus::Pass && provenance.sources.is_empty() {
        diagnostics.push(diagnostic(
            "MISSING_PROVENANCE_SOURCE",
            "passing NotebookLM artifact requires at least one hashed source",
        ));
    }
    for source in &provenance.sources {
        require_nonempty(
            "provenance.sources.source_id",
            &source.source_id,
            diagnostics,
        );
        require_nonempty("provenance.sources.title", &source.title, diagnostics);
        require_nonempty("provenance.sources.uri", &source.uri, diagnostics);
        validate_sha256("provenance.sources.sha256", &source.sha256, diagnostics);
        if !looks_like_rfc3339(&source.retrieved_at) {
            diagnostics.push(diagnostic(
                "INVALID_RETRIEVED_AT",
                "source retrieved_at must be an RFC3339 UTC timestamp",
            ));
        }
    }
}

fn validate_codegraph_impact(
    impact: &CodeGraphImpact,
    diagnostics: &mut Vec<ContextIngestDiagnostic>,
) {
    require_nonempty("impact.summary", &impact.summary, diagnostics);
    let allowed_flags = [
        "auth",
        "authorization",
        "data_model",
        "audit_security",
        "external_systems",
        "public_contracts",
        "cross_platform",
        "existing_behavior",
        "weak_proof",
        "multi_domain",
    ];
    for flag in &impact.risk_flags {
        if !allowed_flags.contains(&flag.as_str()) {
            diagnostics.push(diagnostic(
                "INVALID_RISK_FLAG",
                &format!("unsupported risk flag '{flag}'"),
            ));
        }
    }
    if impact.claims.is_empty() {
        diagnostics.push(diagnostic(
            "MISSING_CLAIMS",
            "passing CodeGraph artifact requires claims",
        ));
    }
    for file in &impact.affected_files {
        require_nonempty("impact.affected_files.path", &file.path, diagnostics);
        if ![
            "direct",
            "transitive",
            "test",
            "configuration",
            "documentation",
        ]
        .contains(&file.change_kind.as_str())
        {
            diagnostics.push(diagnostic(
                "INVALID_CHANGE_KIND",
                &format!("unsupported change_kind '{}'", file.change_kind),
            ));
        }
        if file.reasons.is_empty() || file.reasons.iter().any(|value| value.trim().is_empty()) {
            diagnostics.push(diagnostic(
                "MISSING_FILE_REASON",
                "each affected file requires at least one reason",
            ));
        }
        if file.symbols.iter().any(|value| value.trim().is_empty()) {
            diagnostics.push(diagnostic(
                "INVALID_SYMBOL",
                "affected file symbols must not be empty",
            ));
        }
    }
    for edge in &impact.dependency_edges {
        require_nonempty("impact.dependency_edges.from", &edge.from, diagnostics);
        require_nonempty("impact.dependency_edges.to", &edge.to, diagnostics);
        if ![
            "imports",
            "calls",
            "implements",
            "reads",
            "writes",
            "configures",
            "tests",
        ]
        .contains(&edge.kind.as_str())
        {
            diagnostics.push(diagnostic(
                "INVALID_DEPENDENCY_KIND",
                &format!("unsupported dependency edge kind '{}'", edge.kind),
            ));
        }
    }
    for claim in &impact.claims {
        validate_claim(
            &claim.claim_id,
            "CG-",
            &claim.statement,
            &claim.evidence_refs,
            diagnostics,
        );
    }
}

fn validate_notebook_brief(
    brief: &NotebookBrief,
    provenance: &NotebookProvenance,
    diagnostics: &mut Vec<ContextIngestDiagnostic>,
) {
    require_nonempty("brief.summary", &brief.summary, diagnostics);
    if brief.claims.is_empty() {
        diagnostics.push(diagnostic(
            "MISSING_CLAIMS",
            "passing NotebookLM artifact requires grounded claims",
        ));
    }
    let source_ids = provenance
        .sources
        .iter()
        .map(|source| source.source_id.as_str())
        .collect::<Vec<_>>();
    for claim in &brief.claims {
        let suffix = claim.claim_id.strip_prefix("NL-").unwrap_or_default();
        if suffix.is_empty() || suffix.chars().any(|value| !value.is_ascii_digit()) {
            diagnostics.push(diagnostic(
                "INVALID_CLAIM_ID",
                &format!("invalid NotebookLM claim id '{}'", claim.claim_id),
            ));
        }
        require_nonempty("brief.claims.statement", &claim.statement, diagnostics);
        if claim.citations.is_empty() {
            diagnostics.push(diagnostic(
                "MISSING_CITATION",
                &format!("claim '{}' requires at least one citation", claim.claim_id),
            ));
        }
        for citation in &claim.citations {
            if !source_ids.contains(&citation.source_id.as_str()) {
                diagnostics.push(diagnostic(
                    "UNKNOWN_CITATION_SOURCE",
                    &format!(
                        "citation source '{}' is not in provenance",
                        citation.source_id
                    ),
                ));
            }
            require_nonempty(
                "brief.claims.citations.locator",
                &citation.locator,
                diagnostics,
            );
            if let Some(hash) = &citation.quote_sha256 {
                validate_sha256("brief.claims.citations.quote_sha256", hash, diagnostics);
            }
        }
    }
}

fn validate_claim(
    claim_id: &str,
    prefix: &str,
    statement: &str,
    evidence_refs: &[String],
    diagnostics: &mut Vec<ContextIngestDiagnostic>,
) {
    let suffix = claim_id.strip_prefix(prefix).unwrap_or_default();
    if suffix.is_empty() || suffix.chars().any(|value| !value.is_ascii_digit()) {
        diagnostics.push(diagnostic(
            "INVALID_CLAIM_ID",
            &format!("invalid claim id '{claim_id}'"),
        ));
    }
    require_nonempty("claim.statement", statement, diagnostics);
    if evidence_refs.is_empty() || evidence_refs.iter().any(|value| value.trim().is_empty()) {
        diagnostics.push(diagnostic(
            "MISSING_CLAIM_EVIDENCE",
            &format!("claim '{claim_id}' requires evidence references"),
        ));
    }
}

fn validate_declared_failure(
    errors: Option<&[ArtifactError]>,
    diagnostics: &mut Vec<ContextIngestDiagnostic>,
) {
    let Some(errors) = errors.filter(|values| !values.is_empty()) else {
        diagnostics.push(diagnostic(
            "MISSING_ERRORS",
            "failed artifact requires at least one error",
        ));
        return;
    };
    for error in errors {
        require_nonempty("errors.code", &error.code, diagnostics);
        require_nonempty("errors.message", &error.message, diagnostics);
        let _ = error.retryable;
    }
}

fn validate_unavailable(
    unavailable: Option<&ArtifactUnavailable>,
    allowed_reasons: &[&str],
    diagnostics: &mut Vec<ContextIngestDiagnostic>,
) {
    let Some(unavailable) = unavailable else {
        diagnostics.push(diagnostic(
            "MISSING_UNAVAILABLE",
            "inconclusive artifact requires unavailable details",
        ));
        return;
    };
    if !allowed_reasons.contains(&unavailable.reason.as_str()) {
        diagnostics.push(diagnostic(
            "INVALID_UNAVAILABLE_REASON",
            &format!("unsupported unavailable reason '{}'", unavailable.reason),
        ));
    }
    let _ = unavailable.retryable;
    if unavailable
        .detail
        .as_deref()
        .is_some_and(|value| value.trim().is_empty())
    {
        diagnostics.push(diagnostic(
            "INVALID_UNAVAILABLE_DETAIL",
            "unavailable detail must not be empty",
        ));
    }
}

fn finalize_artifact_validation(
    artifact_id: String,
    schema_version: String,
    declared_status: ContextIngestStatus,
    summary: Option<String>,
    mapped_context: Option<MappedContext>,
    diagnostics: Vec<ContextIngestDiagnostic>,
) -> ContextArtifactValidation {
    let contract_valid = diagnostics.is_empty();
    let status = if contract_valid {
        declared_status
    } else {
        ContextIngestStatus::Fail
    };
    ContextArtifactValidation {
        artifact_id,
        schema_version,
        status,
        provenance_status: if contract_valid {
            ContextIngestStatus::Pass
        } else {
            ContextIngestStatus::Fail
        },
        summary,
        mapped_context: if status == ContextIngestStatus::Pass {
            mapped_context
        } else {
            None
        },
        diagnostics: if contract_valid && status == ContextIngestStatus::Pass {
            Vec::new()
        } else if diagnostics.is_empty() {
            vec![ContextIngestDiagnostic {
                code: match status {
                    ContextIngestStatus::Fail => "PROVIDER_REPORTED_FAILURE",
                    ContextIngestStatus::Inconclusive => "SOURCE_INCONCLUSIVE",
                    ContextIngestStatus::Pass => unreachable!(),
                }
                .to_owned(),
                message: match status {
                    ContextIngestStatus::Fail => {
                        "provider artifact reported a deterministic failure".to_owned()
                    }
                    ContextIngestStatus::Inconclusive => {
                        "provider artifact reported unavailable or insufficient evidence".to_owned()
                    }
                    ContextIngestStatus::Pass => unreachable!(),
                },
                path: None,
                retryable: None,
            }]
        } else {
            diagnostics
        },
    }
}

fn render_grounded_context(brief: &NotebookBrief) -> String {
    let mut lines = vec![brief.summary.clone()];
    if !brief.constraints.is_empty() {
        lines.push(format!("Constraints: {}", brief.constraints.join("; ")));
    }
    if !brief.open_questions.is_empty() {
        lines.push(format!(
            "Open questions: {}",
            brief.open_questions.join("; ")
        ));
    }
    for claim in &brief.claims {
        let citations = claim
            .citations
            .iter()
            .map(|citation| format!("{}:{}", citation.source_id, citation.locator))
            .collect::<Vec<_>>()
            .join(", ");
        lines.push(format!(
            "{}: {} [{}]",
            claim.claim_id, claim.statement, citations
        ));
    }
    lines.join("\n")
}

fn update_intake_from_mapped_context(
    connection: &Connection,
    story_id: &str,
    mapped: &MappedContext,
) -> Result<()> {
    let intake = connection
        .query_row(
            "SELECT id, risk_flags, affected_docs
             FROM intake WHERE story_id=?1 ORDER BY id DESC LIMIT 1;",
            params![story_id],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                ))
            },
        )
        .optional()?
        .ok_or_else(|| {
            HarnessInfraError::InvalidContextIngest(format!(
                "story '{story_id}' has no linked intake to update"
            ))
        })?;
    let risk_flags = merge_json_string_array(intake.1.as_deref(), &mapped.risk_flags)?;
    let affected_docs = merge_json_string_array(intake.2.as_deref(), &mapped.affected_docs)?;
    connection.execute(
        "UPDATE intake SET
            risk_flags=?1,
            affected_docs=?2,
            code_impact_summary=COALESCE(?3, code_impact_summary),
            grounded_context=COALESCE(?4, grounded_context)
         WHERE id=?5;",
        params![
            risk_flags,
            affected_docs,
            mapped.code_impact_summary,
            mapped.grounded_context,
            intake.0,
        ],
    )?;
    Ok(())
}

fn merge_json_string_array(existing: Option<&str>, additions: &[String]) -> Result<Option<String>> {
    let mut values = match existing.filter(|value| !value.trim().is_empty()) {
        Some(value) => serde_json::from_str::<Vec<String>>(value).map_err(|error| {
            HarnessInfraError::InvalidContextIngest(format!(
                "stored JSON array is invalid: {error}"
            ))
        })?,
        None => Vec::new(),
    };
    for addition in additions {
        if !values.contains(addition) {
            values.push(addition.clone());
        }
    }
    if values.is_empty() {
        Ok(None)
    } else {
        Ok(Some(serde_json::to_string(&values).map_err(|error| {
            HarnessInfraError::InvalidContextIngest(error.to_string())
        })?))
    }
}

fn write_context_ingest_report(path: &Path, report: &ContextIngestReport) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(report)
        .map_err(|error| HarnessInfraError::InvalidContextIngest(error.to_string()))?;
    fs::write(path, format!("{json}\n"))?;
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeGraphAffectedOutput {
    changed_files: Vec<String>,
    affected_tests: Vec<String>,
    total_dependents_traversed: usize,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeGraphImpactOutput {
    symbol: String,
    depth: u32,
    node_count: usize,
    edge_count: usize,
    affected: Vec<CodeGraphImpactNode>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct CodeGraphImpactNode {
    name: String,
    kind: String,
    file_path: String,
    start_line: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct NotebookProviderOutput {
    #[serde(alias = "answer")]
    summary: String,
    #[serde(default)]
    constraints: Vec<String>,
    #[serde(default, alias = "openQuestions")]
    open_questions: Vec<String>,
    #[serde(default, alias = "affectedDocs")]
    affected_docs: Vec<String>,
    #[serde(default, alias = "sourcesUsed")]
    sources_used: Vec<serde_json::Value>,
    #[serde(default)]
    citations: BTreeMap<String, String>,
    #[serde(default)]
    references: Vec<NotebookProviderReference>,
    #[serde(default)]
    sources: Vec<NotebookProviderSource>,
    #[serde(default)]
    claims: Vec<NotebookProviderClaim>,
}

#[derive(Debug, Deserialize)]
struct NotebookProviderSource {
    #[serde(alias = "sourceId", alias = "source_id", alias = "id")]
    source_id: String,
    title: String,
    uri: String,
    sha256: Option<String>,
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    text: Option<String>,
    #[serde(default, alias = "retrievedAt")]
    retrieved_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct NotebookProviderClaim {
    #[serde(default, alias = "claimId")]
    claim_id: Option<String>,
    statement: String,
    #[serde(default)]
    citations: Vec<NotebookProviderCitation>,
}

#[derive(Debug, Deserialize)]
struct NotebookProviderCitation {
    #[serde(alias = "sourceId", alias = "source_id")]
    source_id: String,
    locator: String,
    #[serde(default)]
    quote: Option<String>,
    #[serde(default, alias = "quoteSha256")]
    quote_sha256: Option<String>,
}

#[derive(Debug, Deserialize)]
struct NotebookProviderReference {
    #[serde(alias = "sourceId", alias = "source_id")]
    source_id: String,
    #[serde(default, alias = "citationNumber", alias = "citation_number")]
    citation_number: Option<u32>,
    #[serde(default, alias = "citedText", alias = "cited_text")]
    cited_text: Option<String>,
}

fn command_output(
    executable: &str,
    args: &[&str],
    current_dir: &Path,
    stdin: Option<&[u8]>,
) -> std::io::Result<std::process::Output> {
    let mut command = provider_command(executable, args);
    command
        .current_dir(current_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if stdin.is_some() {
        command.stdin(Stdio::piped());
    }
    let mut child = command.spawn()?;
    if let Some(bytes) = stdin {
        child
            .stdin
            .take()
            .expect("stdin is piped when input is provided")
            .write_all(bytes)?;
    }
    child.wait_with_output()
}

fn provider_command(executable: &str, args: &[&str]) -> Command {
    #[cfg(windows)]
    {
        let resolved = resolve_windows_executable(executable).unwrap_or_else(|| executable.into());
        let extension = Path::new(&resolved)
            .extension()
            .and_then(|value| value.to_str())
            .unwrap_or_default();
        if extension.eq_ignore_ascii_case("cmd") || extension.eq_ignore_ascii_case("bat") {
            let mut command = Command::new("cmd.exe");
            command.arg("/D").arg("/C").arg(resolved).args(args);
            return command;
        }
        let mut command = Command::new(resolved);
        command.args(args);
        command
    }
    #[cfg(not(windows))]
    {
        let mut command = Command::new(executable);
        command.args(args);
        command
    }
}

#[cfg(windows)]
fn resolve_windows_executable(executable: &str) -> Option<String> {
    if Path::new(executable).extension().is_some() || Path::new(executable).components().count() > 1
    {
        return Some(executable.to_owned());
    }
    let output = Command::new("where.exe").arg(executable).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let paths = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .collect::<Vec<_>>();
    paths
        .iter()
        .find(|value| value.to_ascii_lowercase().ends_with(".cmd"))
        .cloned()
        .or_else(|| paths.first().cloned())
}

fn git_output(repo_root: &Path, args: &[&str]) -> Option<String> {
    command_output("git", args, repo_root, None)
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn first_non_empty_line(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes)
        .lines()
        .map(str::trim)
        .find(|value| !value.is_empty())
        .unwrap_or("")
        .to_owned()
}

fn write_provider_response(
    path: &Path,
    stdout: &[u8],
    stderr: &[u8],
    exit_code: Option<i32>,
) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if exit_code == Some(0) && !stdout.is_empty() {
        fs::write(path, stdout)?;
    } else {
        write_json_value(
            path,
            &serde_json::json!({
                "exit_code": exit_code,
                "stdout": String::from_utf8_lossy(stdout),
                "stderr": String::from_utf8_lossy(stderr)
            }),
        )?;
    }
    Ok(())
}

fn write_json_value(path: &Path, value: &serde_json::Value) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(value)
        .map_err(|error| HarnessInfraError::InvalidContextIngest(error.to_string()))?;
    fs::write(path, format!("{json}\n"))?;
    Ok(())
}

fn codegraph_base_artifact(
    story_id: &str,
    generated_at: &str,
    repository: &str,
    revision: &str,
    provider_version: &str,
    invocation_id: &str,
    inputs: serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "schema_version": "1.0.0",
        "artifact_type": "codegraph-impact",
        "artifact_id": uuid_from_sha256(&format!("{story_id}:{invocation_id}")),
        "story_id": story_id,
        "generated_at": generated_at,
        "provenance": {
            "provider": "codegraph-cli",
            "adapter": "harness-cli-codegraph",
            "adapter_version": format!(
                "{}; codegraph-cli {}",
                env!("CARGO_PKG_VERSION"),
                provider_version
            ),
            "invocation_id": invocation_id,
            "repository": repository,
            "revision": revision,
            "inputs": inputs
        }
    })
}

fn codegraph_unavailable_artifact(
    story_id: &str,
    generated_at: &str,
    repository: &str,
    revision: &str,
    provider_version: &str,
    invocation_id: &str,
    detail: &str,
) -> serde_json::Value {
    let mut artifact = codegraph_base_artifact(
        story_id,
        generated_at,
        repository,
        revision,
        provider_version,
        invocation_id,
        serde_json::json!([]),
    );
    let object = artifact
        .as_object_mut()
        .expect("base artifact is an object");
    object.insert("status".to_owned(), serde_json::json!("inconclusive"));
    object.insert(
        "unavailable".to_owned(),
        serde_json::json!({
            "reason": "provider_unavailable",
            "retryable": true,
            "detail": detail
        }),
    );
    artifact
}

#[allow(clippy::too_many_arguments)]
fn normalize_codegraph_output(
    mode: CodeGraphMode,
    story_id: &str,
    generated_at: &str,
    repository: &str,
    revision: &str,
    provider_version: &str,
    invocation_id: &str,
    raw_output_path: &Path,
    raw_output_uri: &str,
    stdout: &[u8],
) -> serde_json::Value {
    let raw_sha256 = format!(
        "{:x}",
        Sha256::digest(fs::read(raw_output_path).unwrap_or_else(|_| stdout.to_vec()))
    );
    let inputs = serde_json::json!([{
        "uri": raw_output_uri,
        "sha256": raw_sha256
    }]);
    let mut artifact = codegraph_base_artifact(
        story_id,
        generated_at,
        repository,
        revision,
        provider_version,
        invocation_id,
        inputs,
    );
    let impact = match mode {
        CodeGraphMode::ChangedFiles => {
            let response = match serde_json::from_slice::<CodeGraphAffectedOutput>(stdout) {
                Ok(response) => response,
                Err(error) => {
                    return codegraph_failed_artifact(
                        artifact,
                        "INVALID_PROVIDER_JSON",
                        &format!("CodeGraph affected output is invalid: {error}"),
                    );
                }
            };
            normalize_affected_impact(response, raw_output_uri)
        }
        CodeGraphMode::Symbol => {
            let response = match serde_json::from_slice::<CodeGraphImpactOutput>(stdout) {
                Ok(response) => response,
                Err(error) => {
                    return codegraph_failed_artifact(
                        artifact,
                        "INVALID_PROVIDER_JSON",
                        &format!("CodeGraph impact output is invalid: {error}"),
                    );
                }
            };
            normalize_symbol_impact(response, raw_output_uri)
        }
    };
    let object = artifact
        .as_object_mut()
        .expect("base artifact is an object");
    object.insert("status".to_owned(), serde_json::json!("pass"));
    object.insert("impact".to_owned(), impact);
    artifact
}

fn codegraph_failed_artifact(
    mut artifact: serde_json::Value,
    code: &str,
    message: &str,
) -> serde_json::Value {
    let object = artifact
        .as_object_mut()
        .expect("base artifact is an object");
    object.insert("status".to_owned(), serde_json::json!("fail"));
    object.insert(
        "errors".to_owned(),
        serde_json::json!([{
            "code": code,
            "message": message,
            "retryable": false
        }]),
    );
    artifact
}

fn normalize_affected_impact(
    response: CodeGraphAffectedOutput,
    raw_path: &str,
) -> serde_json::Value {
    let mut files = Vec::new();
    for path in &response.changed_files {
        files.push(serde_json::json!({
            "path": path,
            "change_kind": "direct",
            "reasons": ["Provided as a changed file to CodeGraph affected analysis."]
        }));
    }
    for path in &response.affected_tests {
        if !response.changed_files.contains(path) {
            files.push(serde_json::json!({
                "path": path,
                "change_kind": "test",
                "reasons": ["CodeGraph identified this test through transitive dependency analysis."]
            }));
        }
    }
    let all_paths = response
        .changed_files
        .iter()
        .chain(response.affected_tests.iter())
        .cloned()
        .collect::<Vec<_>>();
    serde_json::json!({
        "summary": format!(
            "CodeGraph traversed {} dependents for {} changed files and identified {} affected tests.",
            response.total_dependents_traversed,
            response.changed_files.len(),
            response.affected_tests.len()
        ),
        "affected_files": files,
        "dependency_edges": [],
        "risk_flags": infer_codegraph_risk_flags(&all_paths),
        "claims": [{
            "claim_id": "CG-1",
            "statement": format!(
                "CodeGraph identified {} affected tests from {} changed files.",
                response.affected_tests.len(),
                response.changed_files.len()
            ),
            "evidence_refs": [raw_path]
        }]
    })
}

fn normalize_symbol_impact(response: CodeGraphImpactOutput, raw_path: &str) -> serde_json::Value {
    let mut grouped = std::collections::BTreeMap::<String, Vec<String>>::new();
    for node in &response.affected {
        let symbol = match node.start_line {
            Some(line) => format!("{}:{}:{line}", node.kind, node.name),
            None => format!("{}:{}", node.kind, node.name),
        };
        grouped
            .entry(node.file_path.clone())
            .or_default()
            .push(symbol);
    }
    let paths = grouped.keys().cloned().collect::<Vec<_>>();
    let affected_files = grouped
        .into_iter()
        .map(|(path, symbols)| {
            serde_json::json!({
                "path": path,
                "change_kind": "transitive",
                "reasons": [format!(
                    "CodeGraph included this file in the depth-{} impact radius for '{}'.",
                    response.depth, response.symbol
                )],
                "symbols": symbols
            })
        })
        .collect::<Vec<_>>();
    serde_json::json!({
        "summary": format!(
            "CodeGraph found {} affected symbols and {} graph edges for '{}' at depth {}.",
            response.node_count, response.edge_count, response.symbol, response.depth
        ),
        "affected_files": affected_files,
        "dependency_edges": [],
        "risk_flags": infer_codegraph_risk_flags(&paths),
        "claims": [{
            "claim_id": "CG-1",
            "statement": format!(
                "Changing '{}' has a CodeGraph impact radius of {} symbols across {} files.",
                response.symbol, response.node_count, paths.len()
            ),
            "evidence_refs": [raw_path]
        }]
    })
}

fn infer_codegraph_risk_flags(paths: &[String]) -> Vec<String> {
    let mut flags = vec!["existing_behavior".to_owned()];
    for path in paths {
        let normalized = path.replace('\\', "/").to_ascii_lowercase();
        for (segments, flag) in [
            (&["auth", "authentication", "session", "jwt"][..], "auth"),
            (
                &["schema", "migration", "migrations", "database", "sql"][..],
                "data_model",
            ),
            (
                &["interface", "api", "route", "controller", "dto"][..],
                "public_contracts",
            ),
            (
                &["provider", "adapter", "integration", "webhook"][..],
                "external_systems",
            ),
        ] {
            if path_has_any_segment(&normalized, segments)
                && !flags.iter().any(|existing| existing == flag)
            {
                flags.push(flag.to_owned());
            }
        }
    }
    flags
}

fn notebook_base_artifact(
    story_id: &str,
    generated_at: &str,
    provider_version: &str,
    invocation_id: &str,
    sources: serde_json::Value,
) -> serde_json::Value {
    serde_json::json!({
        "schema_version": "1.0.0",
        "artifact_type": "notebooklm-brief",
        "artifact_id": uuid_from_sha256(&format!("{story_id}:notebooklm:{invocation_id}")),
        "story_id": story_id,
        "generated_at": generated_at,
        "provenance": {
            "provider": "notebooklm-mcp-cli",
            "adapter": "harness-cli-notebooklm",
            "adapter_version": format!(
                "{}; notebooklm-mcp-cli {}",
                env!("CARGO_PKG_VERSION"),
                provider_version
            ),
            "invocation_id": invocation_id,
            "sources": sources
        }
    })
}

fn notebook_unavailable_artifact(
    story_id: &str,
    generated_at: &str,
    provider_version: &str,
    invocation_id: &str,
    reason: &str,
    retryable: bool,
    detail: &str,
) -> serde_json::Value {
    let mut artifact = notebook_base_artifact(
        story_id,
        generated_at,
        provider_version,
        invocation_id,
        serde_json::json!([]),
    );
    let object = artifact
        .as_object_mut()
        .expect("base artifact is an object");
    object.insert("status".to_owned(), serde_json::json!("inconclusive"));
    object.insert(
        "unavailable".to_owned(),
        serde_json::json!({
            "reason": reason,
            "retryable": retryable,
            "detail": detail
        }),
    );
    artifact
}

fn normalize_notebook_output(
    story_id: &str,
    generated_at: &str,
    provider_version: &str,
    invocation_id: &str,
    raw_output_uri: &str,
    stdout: &[u8],
) -> serde_json::Value {
    let response = match serde_json::from_slice::<NotebookProviderOutput>(stdout) {
        Ok(response) => response,
        Err(error) => {
            let artifact = notebook_base_artifact(
                story_id,
                generated_at,
                provider_version,
                invocation_id,
                serde_json::json!([]),
            );
            return notebook_failed_artifact(
                artifact,
                "INVALID_PROVIDER_JSON",
                &format!("NotebookLM provider output is invalid: {error}"),
            );
        }
    };

    let source_id_map = notebook_source_id_map(&response);
    let sources = notebook_sources(&response, &source_id_map, generated_at);
    let claims = notebook_claims(&response, &source_id_map);

    let mut artifact = notebook_base_artifact(
        story_id,
        generated_at,
        provider_version,
        invocation_id,
        serde_json::json!(sources),
    );
    let object = artifact
        .as_object_mut()
        .expect("base artifact is an object");
    object.insert("status".to_owned(), serde_json::json!("pass"));
    object.insert(
        "brief".to_owned(),
        serde_json::json!({
            "summary": response.summary,
            "constraints": response.constraints,
            "open_questions": response.open_questions,
            "affected_docs": response.affected_docs,
            "claims": claims
        }),
    );
    let bytes = serde_json::to_vec(&artifact).unwrap_or_default();
    let validation = validate_context_artifact(
        ContextSource::Notebooklm,
        story_id,
        &bytes,
        &format!("{:x}", Sha256::digest(&bytes)),
        Path::new("."),
    );
    if validation.status == ContextIngestStatus::Pass {
        artifact
    } else {
        let messages = validation
            .diagnostics
            .iter()
            .map(|item| format!("{}: {}", item.code, item.message))
            .collect::<Vec<_>>()
            .join("; ");
        notebook_failed_artifact(
            notebook_base_artifact(
                story_id,
                generated_at,
                provider_version,
                invocation_id,
                serde_json::json!([{
                    "source_id": "SRC-1",
                    "title": "raw-provider-response",
                    "uri": raw_output_uri,
                    "sha256": format!("{:x}", Sha256::digest(stdout)),
                    "retrieved_at": generated_at
                }]),
            ),
            "UNGROUNDED_PROVIDER_OUTPUT",
            &messages,
        )
    }
}

fn notebook_source_id_map(response: &NotebookProviderOutput) -> BTreeMap<String, String> {
    let mut source_keys = BTreeSet::new();
    for source in &response.sources {
        if let Some(source_id) = notebook_source_key(&source.source_id) {
            source_keys.insert(source_id);
        }
    }
    for claim in &response.claims {
        for citation in &claim.citations {
            if let Some(source_id) = notebook_source_key(&citation.source_id) {
                source_keys.insert(source_id);
            }
        }
    }
    for reference in &response.references {
        if let Some(source_id) = notebook_source_key(&reference.source_id) {
            source_keys.insert(source_id);
        }
    }
    for source_id in response.citations.values() {
        if let Some(source_id) = notebook_source_key(source_id) {
            source_keys.insert(source_id);
        }
    }
    for source in &response.sources_used {
        if let Some(source_id) = notebook_source_id_from_value(source) {
            source_keys.insert(source_id);
        }
    }

    let mut used = BTreeSet::new();
    let mut next_id = 1usize;
    source_keys
        .into_iter()
        .map(|source_id| {
            let normalized = normalize_notebook_source_id(&source_id);
            let canonical =
                if is_schema_notebook_source_id(&normalized) && !used.contains(&normalized) {
                    normalized
                } else {
                    loop {
                        let candidate = format!("SRC-{next_id}");
                        next_id += 1;
                        if !used.contains(&candidate) {
                            break candidate;
                        }
                    }
                };
            used.insert(canonical.clone());
            (source_id, canonical)
        })
        .collect()
}

fn notebook_sources(
    response: &NotebookProviderOutput,
    source_id_map: &BTreeMap<String, String>,
    generated_at: &str,
) -> Vec<serde_json::Value> {
    let mut emitted = BTreeSet::new();
    let mut sources = Vec::new();

    for source in &response.sources {
        let Some(source_key) = notebook_source_key(&source.source_id) else {
            continue;
        };
        let source_id = canonical_notebook_source_id(source_id_map, &source_key);
        if !emitted.insert(source_id.clone()) {
            continue;
        }
        let title = if source.title.trim().is_empty() {
            format!("NotebookLM source {source_key}")
        } else {
            source.title.clone()
        };
        let uri = if source.uri.trim().is_empty() {
            format!("notebooklm://source/{source_key}")
        } else {
            source.uri.clone()
        };
        let hash_input = source
            .sha256
            .clone()
            .or_else(|| source.content.clone())
            .or_else(|| source.text.clone())
            .unwrap_or_else(|| format!("{}:{}:{}", source.source_id, title, uri));
        let sha256 = source
            .sha256
            .clone()
            .unwrap_or_else(|| format!("{:x}", Sha256::digest(hash_input.as_bytes())));
        sources.push(serde_json::json!({
            "source_id": source_id,
            "title": title,
            "uri": uri,
            "sha256": sha256,
            "retrieved_at": source
                .retrieved_at
                .clone()
                .unwrap_or_else(|| generated_at.to_owned())
        }));
    }

    for (source_key, source_id) in source_id_map {
        if !emitted.insert(source_id.clone()) {
            continue;
        }
        let cited_text = response
            .references
            .iter()
            .find(|reference| reference.source_id.trim() == source_key)
            .and_then(|reference| reference.cited_text.as_ref())
            .filter(|text| !text.trim().is_empty())
            .map(String::as_str)
            .unwrap_or(source_key);
        sources.push(serde_json::json!({
            "source_id": source_id,
            "title": format!("NotebookLM source {source_key}"),
            "uri": format!("notebooklm://source/{source_key}"),
            "sha256": format!("{:x}", Sha256::digest(cited_text.as_bytes())),
            "retrieved_at": generated_at
        }));
    }

    sources
}

fn notebook_claims(
    response: &NotebookProviderOutput,
    source_id_map: &BTreeMap<String, String>,
) -> Vec<serde_json::Value> {
    if !response.claims.is_empty() {
        return response
            .claims
            .iter()
            .enumerate()
            .map(|(index, claim)| {
                let citations = claim
                    .citations
                    .iter()
                    .map(|citation| {
                        let mut value = serde_json::json!({
                            "source_id": canonical_notebook_source_id(
                                source_id_map,
                                &citation.source_id
                            ),
                            "locator": citation.locator,
                        });
                        let quote_hash = citation.quote_sha256.clone().or_else(|| {
                            citation
                                .quote
                                .as_ref()
                                .map(|quote| format!("{:x}", Sha256::digest(quote.as_bytes())))
                        });
                        if let Some(hash) = quote_hash {
                            value
                                .as_object_mut()
                                .expect("citation is an object")
                                .insert("quote_sha256".to_owned(), serde_json::json!(hash));
                        }
                        value
                    })
                    .collect::<Vec<_>>();
                serde_json::json!({
                    "claim_id": claim
                        .claim_id
                        .clone()
                        .unwrap_or_else(|| format!("NL-{}", index + 1)),
                    "statement": claim.statement,
                    "citations": citations
                })
            })
            .collect();
    }

    let mut citations = response
        .references
        .iter()
        .filter_map(|reference| {
            notebook_source_key(&reference.source_id).map(|source_id| {
                let mut value = serde_json::json!({
                    "source_id": canonical_notebook_source_id(source_id_map, &source_id),
                    "locator": reference
                        .citation_number
                        .map(|number| format!("citation:{number}"))
                        .unwrap_or_else(|| "citation".to_owned()),
                });
                if let Some(cited_text) = reference
                    .cited_text
                    .as_ref()
                    .filter(|text| !text.trim().is_empty())
                {
                    value
                        .as_object_mut()
                        .expect("citation is an object")
                        .insert(
                            "quote_sha256".to_owned(),
                            serde_json::json!(format!(
                                "{:x}",
                                Sha256::digest(cited_text.as_bytes())
                            )),
                        );
                }
                value
            })
        })
        .collect::<Vec<_>>();

    if citations.is_empty() {
        citations = response
            .citations
            .iter()
            .filter_map(|(number, source_id)| {
                notebook_source_key(source_id).map(|source_id| {
                    serde_json::json!({
                        "source_id": canonical_notebook_source_id(source_id_map, &source_id),
                        "locator": format!("citation:{number}"),
                    })
                })
            })
            .collect();
    }

    if citations.is_empty() {
        Vec::new()
    } else {
        vec![serde_json::json!({
            "claim_id": "NL-1",
            "statement": response.summary,
            "citations": citations
        })]
    }
}

fn notebook_source_key(value: &str) -> Option<String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed.to_owned())
    }
}

fn notebook_source_id_from_value(value: &serde_json::Value) -> Option<String> {
    if let Some(source_id) = value.as_str() {
        return notebook_source_key(source_id);
    }
    let object = value.as_object()?;
    for key in ["source_id", "sourceId", "id"] {
        if let Some(source_id) = object.get(key).and_then(serde_json::Value::as_str) {
            return notebook_source_key(source_id);
        }
    }
    None
}

fn canonical_notebook_source_id(
    source_id_map: &BTreeMap<String, String>,
    source_id: &str,
) -> String {
    notebook_source_key(source_id)
        .and_then(|key| source_id_map.get(&key).cloned())
        .unwrap_or_else(|| normalize_notebook_source_id(source_id))
}

fn is_schema_notebook_source_id(value: &str) -> bool {
    value
        .strip_prefix("SRC-")
        .is_some_and(|suffix| !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()))
}

fn notebook_failed_artifact(
    mut artifact: serde_json::Value,
    code: &str,
    message: &str,
) -> serde_json::Value {
    let object = artifact
        .as_object_mut()
        .expect("base artifact is an object");
    object.insert("status".to_owned(), serde_json::json!("fail"));
    object.insert(
        "errors".to_owned(),
        serde_json::json!([{
            "code": code,
            "message": message,
            "retryable": false
        }]),
    );
    artifact
}

fn normalize_notebook_source_id(value: &str) -> String {
    let suffix = value
        .strip_prefix("SRC-")
        .or_else(|| value.strip_prefix("src-"))
        .unwrap_or(value)
        .trim();
    if !suffix.is_empty() && suffix.chars().all(|character| character.is_ascii_digit()) {
        format!("SRC-{suffix}")
    } else {
        value.to_owned()
    }
}

fn notebook_provider_failure_detail(stdout: &[u8], stderr: &[u8]) -> String {
    let stderr_text = String::from_utf8_lossy(stderr).trim().to_owned();
    if !stderr_text.is_empty() {
        return stderr_text;
    }
    String::from_utf8_lossy(stdout).trim().to_owned()
}

fn format_provider_timeout(timeout: f64) -> String {
    if timeout.fract() == 0.0 {
        format!("{timeout:.0}")
    } else {
        timeout.to_string()
    }
}

fn notebook_unavailable_reason(detail: &str) -> &'static str {
    let normalized = detail.to_ascii_lowercase();
    if normalized.contains("timeout") || normalized.contains("timed out") {
        "timeout"
    } else if normalized.contains("auth")
        || normalized.contains("login")
        || normalized.contains("profile")
        || normalized.contains("permission")
        || normalized.contains("denied")
        || normalized.contains("session")
    {
        "permission_denied"
    } else if normalized.contains("notebook")
        || normalized.contains("source")
        || normalized.contains("not found")
    {
        "source_unavailable"
    } else if normalized.contains("insufficient")
        || normalized.contains("citation")
        || normalized.contains("grounded")
    {
        "insufficient_evidence"
    } else {
        "provider_unavailable"
    }
}

fn diagnostic(code: &str, message: &str) -> ContextIngestDiagnostic {
    ContextIngestDiagnostic {
        code: code.to_owned(),
        message: message.to_owned(),
        path: None,
        retryable: Some(false),
    }
}

fn require_nonempty(field: &str, value: &str, diagnostics: &mut Vec<ContextIngestDiagnostic>) {
    if value.trim().is_empty() {
        diagnostics.push(diagnostic(
            "MISSING_REQUIRED_VALUE",
            &format!("{field} must not be empty"),
        ));
    }
}

fn validate_sha256(field: &str, value: &str, diagnostics: &mut Vec<ContextIngestDiagnostic>) {
    if value.len() != 64
        || !value
            .chars()
            .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    {
        diagnostics.push(diagnostic(
            "INVALID_SHA256",
            &format!("{field} must be a lowercase SHA256 digest"),
        ));
    }
}

fn is_uuid(value: &str) -> bool {
    let parts = value.split('-').collect::<Vec<_>>();
    [8, 4, 4, 4, 12]
        .into_iter()
        .zip(parts)
        .all(|(length, part)| {
            part.len() == length && part.chars().all(|character| character.is_ascii_hexdigit())
        })
}

fn looks_like_rfc3339(value: &str) -> bool {
    value.len() >= 20
        && value.as_bytes().get(4) == Some(&b'-')
        && value.as_bytes().get(7) == Some(&b'-')
        && value.as_bytes().get(10) == Some(&b'T')
        && (value.ends_with('Z')
            || value
                .get(19..)
                .is_some_and(|suffix| suffix.contains('+') || suffix.contains('-')))
}

fn uuid_from_sha256(value: &str) -> String {
    let digest = format!("{:x}", Sha256::digest(value.as_bytes()));
    format!(
        "{}-{}-4{}-8{}-{}",
        &digest[0..8],
        &digest[8..12],
        &digest[13..16],
        &digest[17..20],
        &digest[20..32]
    )
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

fn resolve_repo_path(repo_root: &Path, path: &str) -> PathBuf {
    let candidate = PathBuf::from(path);
    if candidate.is_absolute() {
        candidate
    } else {
        repo_root.join(candidate)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use super::*;
    use crate::application::{
        BacklogAddInput, BacklogCloseInput, CodeGraphImpactInput, ContextIngestInput,
        DecisionAddInput, FrictionAddInput, FrictionEvidenceInput, FrictionProposedActionInput,
        HarnessContext, HarnessService, IntakeInput, NotebookBriefInput, ReleaseVerifyInput,
        StoryAddInput, StoryUpdateInput, TraceInput,
    };
    use crate::domain::{
        BacklogFilter, BoolFlag, CodeGraphMode, ContextIngestStatus, ContextSource, CsvList,
        FrictionActionType, FrictionSeverity, FrictionSource, FrictionType, InputType, RiskLane,
        TraceQualityTier,
    };

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

    fn context_test_repository() -> (TempDir, PathBuf, SqliteHarnessRepository) {
        let temp_dir = tempfile::tempdir().unwrap();
        let repo_root = temp_dir.path().join("repo");
        fs::create_dir_all(&repo_root).unwrap();
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
        (temp_dir, repo_root, repository)
    }

    fn passing_codegraph_artifact(story_id: &str) -> serde_json::Value {
        serde_json::json!({
            "schema_version": "1.0.0",
            "artifact_type": "codegraph-impact",
            "artifact_id": "11111111-1111-4111-8111-111111111111",
            "story_id": story_id,
            "status": "pass",
            "generated_at": "2026-06-07T00:00:00Z",
            "provenance": {
                "provider": "codegraph",
                "adapter": "test-adapter",
                "adapter_version": "0.1.0",
                "invocation_id": "run-1",
                "repository": "example/repo",
                "revision": "abc123",
                "inputs": [{
                    "uri": "git:HEAD",
                    "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
                }]
            },
            "impact": {
                "summary": "Context ingest changes the CLI trust boundary.",
                "affected_files": [{
                    "path": "crates/harness-cli/src/infrastructure.rs",
                    "change_kind": "direct",
                    "reasons": ["Ingest persistence is implemented here."],
                    "symbols": ["ingest_context"]
                }],
                "dependency_edges": [],
                "risk_flags": ["external_systems", "public_contracts"],
                "claims": [{
                    "claim_id": "CG-1",
                    "statement": "The infrastructure layer persists ingest summaries.",
                    "evidence_refs": ["crates/harness-cli/src/infrastructure.rs"]
                }]
            }
        })
    }

    fn inconclusive_notebook_artifact(story_id: &str) -> serde_json::Value {
        serde_json::json!({
            "schema_version": "1.0.0",
            "artifact_type": "notebooklm-brief",
            "artifact_id": "22222222-2222-4222-8222-222222222222",
            "story_id": story_id,
            "status": "inconclusive",
            "generated_at": "2026-06-07T00:00:00Z",
            "provenance": {
                "provider": "notebooklm",
                "adapter": "test-adapter",
                "adapter_version": "0.1.0",
                "invocation_id": "run-2",
                "sources": []
            },
            "unavailable": {
                "reason": "provider_unavailable",
                "retryable": true
            }
        })
    }

    fn passing_notebook_artifact(story_id: &str) -> serde_json::Value {
        serde_json::json!({
            "schema_version": "1.0.0",
            "artifact_type": "notebooklm-brief",
            "artifact_id": "33333333-3333-4333-8333-333333333333",
            "story_id": story_id,
            "status": "pass",
            "generated_at": "2026-06-07T00:00:00Z",
            "provenance": {
                "provider": "notebooklm-mcp-cli",
                "adapter": "test-adapter",
                "adapter_version": "0.1.0",
                "invocation_id": "run-3",
                "sources": [{
                    "source_id": "SRC-1",
                    "title": "Feature intake",
                    "uri": "docs/FEATURE_INTAKE.md",
                    "sha256": "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
                    "retrieved_at": "2026-06-07T00:00:00Z"
                }]
            },
            "brief": {
                "summary": "Auto intake must use validated NotebookLM context.",
                "constraints": ["Inconclusive evidence never becomes pass."],
                "open_questions": [],
                "affected_docs": ["docs/FEATURE_INTAKE.md"],
                "claims": [{
                    "claim_id": "NL-1",
                    "statement": "NotebookLM context must be citation-backed before intake uses it.",
                    "citations": [{
                        "source_id": "SRC-1",
                        "locator": "MCP Artifact Boundary"
                    }]
                }]
            }
        })
    }

    fn add_context_story_and_intake(
        repository: &SqliteHarnessRepository,
        story_id: &str,
        codegraph_required: i64,
        notebooklm_required: i64,
    ) {
        repository
            .add_story(StoryAddInput {
                id: story_id.to_owned(),
                title: "Context ingest story".to_owned(),
                risk_lane: RiskLane::HighRisk,
                contract_doc: None,
                verify_command: Some("echo ok".to_owned()),
                notes: None,
                release_proof_required: BoolFlag(0),
                codegraph_ingest_required: BoolFlag(codegraph_required),
                notebooklm_ingest_required: BoolFlag(notebooklm_required),
            })
            .unwrap();
        repository
            .record_intake(IntakeInput {
                input_type: InputType::HarnessImprovement,
                summary: "Validate external context artifacts".to_owned(),
                risk_lane: RiskLane::HighRisk,
                risk_flags: CsvList::from_optional(None),
                affected_docs: CsvList::from_optional(None),
                story_id: Some(story_id.to_owned()),
                notes: None,
                code_impact_summary: None,
                grounded_context: None,
                auto_generated: false,
            })
            .unwrap();
    }

    #[test]
    fn init_creates_database_and_schema() {
        let (_temp_dir, repository) = test_repository();

        let result = repository.init().unwrap();

        assert!(matches!(result, InitResult::Created { .. }));
        assert_eq!(repository.query_stats().unwrap().intakes, 0);
        let connection = repository.open_existing().unwrap();
        let schema_version = SqliteHarnessRepository::schema_version(&connection).unwrap();
        assert_eq!(schema_version, 7);
        let story_columns = story_columns(&connection);
        assert!(story_columns.contains(&"verify_command".to_owned()));
        assert!(story_columns.contains(&"last_verified_at".to_owned()));
        assert!(story_columns.contains(&"last_verified_result".to_owned()));
        assert!(story_columns.contains(&"arch_check_result".to_owned()));
        assert!(story_columns.contains(&"gate_result".to_owned()));
        assert!(story_columns.contains(&"release_proof_required".to_owned()));
        assert!(story_columns.contains(&"codegraph_ingest_required".to_owned()));
        assert!(story_columns.contains(&"notebooklm_ingest_required".to_owned()));
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
        let ingest_table_exists = connection
            .query_row(
                "SELECT EXISTS(
                SELECT 1 FROM sqlite_master
                WHERE type='table' AND name='context_ingest'
             );",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap();
        assert_eq!(ingest_table_exists, 1);
        let friction_event_table_exists = connection
            .query_row(
                "SELECT EXISTS(
                SELECT 1 FROM sqlite_master
                WHERE type='table' AND name='friction_event'
             );",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap();
        assert_eq!(friction_event_table_exists, 1);
    }

    #[test]
    fn migrate_applies_all_pending_columns_to_existing_database() {
        let (_temp_dir, repository) = test_repository();
        let connection = repository.open_or_create().unwrap();
        repository.apply_schema_v1(&connection).unwrap();
        drop(connection);

        let result = repository.migrate().unwrap();

        assert_eq!(result.current_version, 1);
        assert_eq!(result.applied, vec![2, 3, 4, 5, 6, 7]);
        let connection = repository.open_existing().unwrap();
        assert_eq!(
            SqliteHarnessRepository::schema_version(&connection).unwrap(),
            7
        );
        let story_columns = story_columns(&connection);
        assert!(story_columns.contains(&"verify_command".to_owned()));
        assert!(story_columns.contains(&"last_verified_at".to_owned()));
        assert!(story_columns.contains(&"last_verified_result".to_owned()));
        assert!(story_columns.contains(&"context_pack_path".to_owned()));
        assert!(story_columns.contains(&"gate_result".to_owned()));
        assert!(story_columns.contains(&"release_proof_required".to_owned()));
        assert!(story_columns.contains(&"codegraph_ingest_required".to_owned()));
        assert!(story_columns.contains(&"notebooklm_ingest_required".to_owned()));
        let friction_event_table_exists = connection
            .query_row(
                "SELECT EXISTS(
                SELECT 1 FROM sqlite_master
                WHERE type='table' AND name='friction_event'
             );",
                [],
                |row| row.get::<_, i64>(0),
            )
            .unwrap();
        assert_eq!(friction_event_table_exists, 1);
    }

    #[test]
    fn context_ingest_pass_updates_intake_context_pack_and_governance_gate() {
        let (_temp_dir, repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-CONTEXT", 1, 0);

        let gate_before = repository.verify_story_gate("US-CONTEXT").unwrap();
        assert!(gate_before
            .missing
            .contains(&"CodeGraph context ingest proof".to_owned()));

        let artifact_path = repo_root.join("codegraph-impact.json");
        fs::write(
            &artifact_path,
            serde_json::to_vec_pretty(&passing_codegraph_artifact("US-CONTEXT")).unwrap(),
        )
        .unwrap();
        let (report_path, report) = repository
            .ingest_context(ContextIngestInput {
                story_id: "US-CONTEXT".to_owned(),
                source: ContextSource::Codegraph,
                file: artifact_path,
                output: None,
            })
            .unwrap();

        assert_eq!(report.status, ContextIngestStatus::Pass);
        assert!(report.governance.eligible_for_intake);
        assert!(report_path.is_file());
        let stored_report: serde_json::Value =
            serde_json::from_slice(&fs::read(&report_path).unwrap()).unwrap();
        assert_eq!(stored_report["status"], "pass");

        let connection = repository.open_existing().unwrap();
        let stored: (String, String) = connection
            .query_row(
                "SELECT result, provenance_status FROM context_ingest
                 WHERE story_id='US-CONTEXT' AND source='codegraph';",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(stored, ("pass".to_owned(), "pass".to_owned()));
        let intake: (String, String) = connection
            .query_row(
                "SELECT risk_flags, code_impact_summary FROM intake
                 WHERE story_id='US-CONTEXT';",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert!(intake.0.contains("external_systems"));
        assert_eq!(intake.1, "Context ingest changes the CLI trust boundary.");
        drop(connection);

        let service = HarnessService::new(HarnessContext {
            repo_root: repo_root.clone(),
            db_path: repository.db_path.clone(),
            schema_dir: repository.schema_dir.clone(),
        });
        let context_pack = service.generate_context_pack("US-CONTEXT").unwrap();
        let context_markdown = fs::read_to_string(context_pack).unwrap();
        assert!(context_markdown.contains("Validated Context Ingest Evidence"));
        assert!(context_markdown.contains("**codegraph:** pass"));

        let gate_after = repository.verify_story_gate("US-CONTEXT").unwrap();
        assert!(!gate_after
            .missing
            .contains(&"CodeGraph context ingest proof".to_owned()));
    }

    #[test]
    fn context_ingest_missing_provenance_fails_without_updating_intake() {
        let (_temp_dir, repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-INVALID", 1, 0);

        let mut artifact = passing_codegraph_artifact("US-INVALID");
        artifact.as_object_mut().unwrap().remove("provenance");
        let artifact_path = repo_root.join("invalid-codegraph-impact.json");
        fs::write(
            &artifact_path,
            serde_json::to_vec_pretty(&artifact).unwrap(),
        )
        .unwrap();

        let (_, report) = repository
            .ingest_context(ContextIngestInput {
                story_id: "US-INVALID".to_owned(),
                source: ContextSource::Codegraph,
                file: artifact_path,
                output: None,
            })
            .unwrap();

        assert_eq!(report.status, ContextIngestStatus::Fail);
        assert!(!report.governance.eligible_for_story_verify);
        assert!(report.mapped_context.is_none());
        let connection = repository.open_existing().unwrap();
        let result: String = connection
            .query_row(
                "SELECT result FROM context_ingest WHERE story_id='US-INVALID';",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(result, "fail");
        let mapped: (Option<String>, Option<String>) = connection
            .query_row(
                "SELECT risk_flags, code_impact_summary FROM intake
                 WHERE story_id='US-INVALID';",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(mapped, (None, None));
        let gate = repository.verify_story_gate("US-INVALID").unwrap();
        assert!(gate
            .missing
            .contains(&"CodeGraph context ingest proof".to_owned()));
    }

    #[test]
    fn context_ingest_unavailable_source_is_inconclusive_and_not_governance_eligible() {
        let (_temp_dir, repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-UNAVAILABLE", 0, 1);

        let artifact_path = repo_root.join("notebooklm-brief.json");
        fs::write(
            &artifact_path,
            serde_json::to_vec_pretty(&inconclusive_notebook_artifact("US-UNAVAILABLE")).unwrap(),
        )
        .unwrap();
        let (_, report) = repository
            .ingest_context(ContextIngestInput {
                story_id: "US-UNAVAILABLE".to_owned(),
                source: ContextSource::Notebooklm,
                file: artifact_path,
                output: None,
            })
            .unwrap();

        assert_eq!(report.status, ContextIngestStatus::Inconclusive);
        assert!(!report.governance.eligible_for_intake);
        assert!(!report.governance.eligible_for_context_pack);
        assert!(report.mapped_context.is_none());
        let connection = repository.open_existing().unwrap();
        let stored: (String, Option<String>) = connection
            .query_row(
                "SELECT result, summary FROM context_ingest
                 WHERE story_id='US-UNAVAILABLE';",
                [],
                |row| Ok((row.get(0)?, row.get(1)?)),
            )
            .unwrap();
        assert_eq!(stored.0, "inconclusive");
        let grounded_context: Option<String> = connection
            .query_row(
                "SELECT grounded_context FROM intake WHERE story_id='US-UNAVAILABLE';",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(grounded_context, None);
        let gate = repository.verify_story_gate("US-UNAVAILABLE").unwrap();
        assert!(gate
            .missing
            .contains(&"NotebookLM context ingest proof".to_owned()));
    }

    #[test]
    fn auto_intake_evidence_reads_latest_passing_mapped_context_only() {
        let (_temp_dir, repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-AUTO", 0, 0);

        let codegraph_path = repo_root.join("codegraph-impact.json");
        fs::write(
            &codegraph_path,
            serde_json::to_vec_pretty(&passing_codegraph_artifact("US-AUTO")).unwrap(),
        )
        .unwrap();
        repository
            .ingest_context(ContextIngestInput {
                story_id: "US-AUTO".to_owned(),
                source: ContextSource::Codegraph,
                file: codegraph_path,
                output: None,
            })
            .unwrap();

        let notebook_path = repo_root.join("notebooklm-brief.json");
        fs::write(
            &notebook_path,
            serde_json::to_vec_pretty(&passing_notebook_artifact("US-AUTO")).unwrap(),
        )
        .unwrap();
        repository
            .ingest_context(ContextIngestInput {
                story_id: "US-AUTO".to_owned(),
                source: ContextSource::Notebooklm,
                file: notebook_path,
                output: None,
            })
            .unwrap();

        let evidence = repository.auto_intake_evidence("US-AUTO").unwrap();
        let codegraph = evidence.codegraph.unwrap();
        assert_eq!(
            codegraph.code_impact_summary.as_deref(),
            Some("Context ingest changes the CLI trust boundary.")
        );
        assert!(codegraph
            .risk_flags
            .contains(&"external_systems".to_owned()));

        let notebook = evidence.notebooklm.unwrap();
        let grounded_context = notebook.grounded_context.as_deref().unwrap();
        assert!(grounded_context.contains("Auto intake must use validated NotebookLM context."));
        assert!(grounded_context
            .contains("NL-1: NotebookLM context must be citation-backed before intake uses it."));
        assert_eq!(
            notebook.affected_docs,
            vec!["docs/FEATURE_INTAKE.md".to_owned()]
        );
    }

    #[test]
    fn auto_intake_evidence_does_not_promote_inconclusive_context() {
        let (_temp_dir, repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-AUTO-INCONCLUSIVE", 0, 0);

        let pass_path = repo_root.join("notebooklm-pass.json");
        fs::write(
            &pass_path,
            serde_json::to_vec_pretty(&passing_notebook_artifact("US-AUTO-INCONCLUSIVE")).unwrap(),
        )
        .unwrap();
        repository
            .ingest_context(ContextIngestInput {
                story_id: "US-AUTO-INCONCLUSIVE".to_owned(),
                source: ContextSource::Notebooklm,
                file: pass_path,
                output: None,
            })
            .unwrap();

        let artifact_path = repo_root.join("notebooklm-brief.json");
        fs::write(
            &artifact_path,
            serde_json::to_vec_pretty(&inconclusive_notebook_artifact("US-AUTO-INCONCLUSIVE"))
                .unwrap(),
        )
        .unwrap();
        repository
            .ingest_context(ContextIngestInput {
                story_id: "US-AUTO-INCONCLUSIVE".to_owned(),
                source: ContextSource::Notebooklm,
                file: artifact_path,
                output: None,
            })
            .unwrap();

        let evidence = repository
            .auto_intake_evidence("US-AUTO-INCONCLUSIVE")
            .unwrap();
        assert!(evidence.codegraph.is_none());
        assert!(evidence.notebooklm.is_none());
    }

    #[test]
    fn context_ingest_rejects_local_provenance_checksum_mismatch() {
        let (_temp_dir, repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-CHECKSUM", 1, 0);

        let raw_path = repo_root.join("provider-response.json");
        fs::write(&raw_path, b"{\"changedFiles\":[]}").unwrap();
        let mut artifact = passing_codegraph_artifact("US-CHECKSUM");
        artifact["provenance"]["inputs"] = serde_json::json!([{
            "uri": raw_path.display().to_string(),
            "sha256": "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        }]);
        let artifact_path = repo_root.join("checksum-mismatch.json");
        fs::write(
            &artifact_path,
            serde_json::to_vec_pretty(&artifact).unwrap(),
        )
        .unwrap();

        let (_, report) = repository
            .ingest_context(ContextIngestInput {
                story_id: "US-CHECKSUM".to_owned(),
                source: ContextSource::Codegraph,
                file: artifact_path,
                output: None,
            })
            .unwrap();

        assert_eq!(report.status, ContextIngestStatus::Fail);
        assert!(report
            .diagnostics
            .iter()
            .any(|item| item.code == "PROVENANCE_SHA256_MISMATCH"));
    }

    #[test]
    fn codegraph_adapter_records_missing_executable_as_inconclusive() {
        let (_temp_dir, repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-NO-CODEGRAPH", 1, 0);
        let changed_files = repo_root.join("changed-files.txt");
        fs::write(&changed_files, "src/lib.rs\n").unwrap();

        let result = repository
            .produce_codegraph_impact(CodeGraphImpactInput {
                story_id: "US-NO-CODEGRAPH".to_owned(),
                mode: CodeGraphMode::ChangedFiles,
                changed_files: Some(changed_files),
                symbol: None,
                depth: 2,
                output: None,
                raw_output: None,
                executable: "harness-codegraph-does-not-exist".to_owned(),
            })
            .unwrap();

        assert_eq!(
            result.ingest_report.status,
            ContextIngestStatus::Inconclusive
        );
        assert!(result.artifact_path.is_file());
        assert!(!result.ingest_report.governance.eligible_for_story_verify);
        let gate = repository.verify_story_gate("US-NO-CODEGRAPH").unwrap();
        assert!(gate
            .missing
            .contains(&"CodeGraph context ingest proof".to_owned()));
    }

    #[test]
    fn codegraph_adapter_normalizes_affected_output_into_passing_artifact() {
        let temp_dir = tempfile::tempdir().unwrap();
        let raw_path = temp_dir.path().join("provider.json");
        let stdout = br#"{
            "changedFiles": ["src/lib.rs"],
            "affectedTests": ["tests/lib.test.rs"],
            "totalDependentsTraversed": 3
        }"#;
        fs::write(&raw_path, stdout).unwrap();

        let artifact = normalize_codegraph_output(
            CodeGraphMode::ChangedFiles,
            "US-999",
            "2026-06-07T00:00:00Z",
            "example/repo",
            "abc123",
            "0.9.9",
            "run-1",
            &raw_path,
            &raw_path.display().to_string(),
            stdout,
        );
        let bytes = serde_json::to_vec(&artifact).unwrap();
        let validation = validate_context_artifact(
            ContextSource::Codegraph,
            "US-999",
            &bytes,
            &format!("{:x}", Sha256::digest(&bytes)),
            temp_dir.path(),
        );

        assert_eq!(validation.status, ContextIngestStatus::Pass);
        assert!(validation.diagnostics.is_empty());
        assert_eq!(
            artifact["impact"]["affected_files"]
                .as_array()
                .unwrap()
                .len(),
            2
        );
    }

    #[test]
    fn codegraph_adapter_maps_invalid_provider_json_to_fail() {
        let temp_dir = tempfile::tempdir().unwrap();
        let raw_path = temp_dir.path().join("provider.json");
        fs::write(&raw_path, b"not-json").unwrap();

        let artifact = normalize_codegraph_output(
            CodeGraphMode::ChangedFiles,
            "US-999",
            "2026-06-07T00:00:00Z",
            "example/repo",
            "abc123",
            "0.9.9",
            "run-2",
            &raw_path,
            &raw_path.display().to_string(),
            b"not-json",
        );

        assert_eq!(artifact["status"], "fail");
        assert_eq!(artifact["errors"][0]["code"], "INVALID_PROVIDER_JSON");
    }

    #[test]
    fn notebook_adapter_records_missing_executable_as_inconclusive() {
        let (_temp_dir, _repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-NO-NOTEBOOKLM", 0, 1);

        let result = repository
            .produce_notebook_brief(NotebookBriefInput {
                story_id: "US-NO-NOTEBOOKLM".to_owned(),
                query: "Find grounded product rules.".to_owned(),
                notebook: "hios-provider-proof".to_owned(),
                profile: Some("default".to_owned()),
                timeout_seconds: Some(30.0),
                output: None,
                raw_output: None,
                executable: "harness-notebooklm-does-not-exist".to_owned(),
            })
            .unwrap();

        assert!(result
            .provider_command
            .contains("query notebook --json --profile default --timeout 30 hios-provider-proof"));
        assert_eq!(
            result.ingest_report.status,
            ContextIngestStatus::Inconclusive
        );
        assert!(result.artifact_path.is_file());
        assert!(!result.ingest_report.governance.eligible_for_story_verify);
        let gate = repository.verify_story_gate("US-NO-NOTEBOOKLM").unwrap();
        assert!(gate
            .missing
            .contains(&"NotebookLM context ingest proof".to_owned()));
    }

    #[test]
    fn notebook_adapter_normalizes_cited_provider_output_into_passing_artifact() {
        let stdout = br#"{
            "summary": "US-026 must preserve the file-based NotebookLM boundary.",
            "constraints": ["Do not store provider session secrets."],
            "openQuestions": [],
            "affectedDocs": ["docs/stories/US-026/design.md"],
            "sources": [{
                "sourceId": "1",
                "title": "US-026 design",
                "uri": "docs/stories/US-026/design.md",
                "content": "Harness stores no Google credentials, cookies, tokens, or sessions."
            }],
            "claims": [{
                "statement": "Harness must keep NotebookLM session state outside its governance database.",
                "citations": [{
                    "sourceId": "1",
                    "locator": "design.md#Interface Contract",
                    "quote": "Harness must not store Google credentials."
                }]
            }]
        }"#;

        let artifact = normalize_notebook_output(
            "US-026",
            "2026-06-07T00:00:00Z",
            "0.1.0",
            "run-1",
            ".harness/context/US-026-notebooklm-provider-response.json",
            stdout,
        );
        let bytes = serde_json::to_vec(&artifact).unwrap();
        let validation = validate_context_artifact(
            ContextSource::Notebooklm,
            "US-026",
            &bytes,
            &format!("{:x}", Sha256::digest(&bytes)),
            Path::new("."),
        );

        assert_eq!(validation.status, ContextIngestStatus::Pass);
        assert!(validation.diagnostics.is_empty());
        assert_eq!(artifact["status"], "pass");
        assert_eq!(artifact["provenance"]["provider"], "notebooklm-mcp-cli");
        assert_eq!(artifact["brief"]["claims"][0]["claim_id"], "NL-1");
        assert_eq!(
            artifact["brief"]["claims"][0]["citations"][0]["source_id"],
            "SRC-1"
        );
    }

    #[test]
    fn notebook_adapter_normalizes_real_nlm_query_json_into_passing_artifact() {
        let stdout = br#"{
            "answer": "Harness must keep NotebookLM session state outside SQLite.",
            "conversation_id": "conversation-123",
            "sources_used": ["source-abc"],
            "citations": {"1": "source-abc"},
            "references": [{
                "source_id": "source-abc",
                "citation_number": 1,
                "cited_text": "Harness never stores Google credentials, cookies, browser profiles, tokens, or provider session files."
            }]
        }"#;

        let artifact = normalize_notebook_output(
            "US-026",
            "2026-06-07T00:00:00Z",
            "0.7.1",
            "run-real",
            ".harness/context/US-026-notebooklm-provider-response.json",
            stdout,
        );
        let bytes = serde_json::to_vec(&artifact).unwrap();
        let validation = validate_context_artifact(
            ContextSource::Notebooklm,
            "US-026",
            &bytes,
            &format!("{:x}", Sha256::digest(&bytes)),
            Path::new("."),
        );

        assert_eq!(validation.status, ContextIngestStatus::Pass);
        assert_eq!(artifact["status"], "pass");
        assert_eq!(artifact["provenance"]["sources"][0]["source_id"], "SRC-1");
        assert_eq!(artifact["brief"]["claims"][0]["claim_id"], "NL-1");
        assert_eq!(
            artifact["brief"]["claims"][0]["citations"][0]["locator"],
            "citation:1"
        );
    }

    #[test]
    fn notebook_adapter_maps_invalid_provider_json_to_fail() {
        let artifact = normalize_notebook_output(
            "US-026",
            "2026-06-07T00:00:00Z",
            "0.1.0",
            "run-2",
            ".harness/context/raw.json",
            b"not-json",
        );

        assert_eq!(artifact["status"], "fail");
        assert_eq!(artifact["errors"][0]["code"], "INVALID_PROVIDER_JSON");
    }

    #[test]
    fn notebook_adapter_rejects_summary_without_cited_claims() {
        let stdout = br#"{
            "summary": "This is only a model-generated summary.",
            "sources": [{
                "sourceId": "SRC-1",
                "title": "Design",
                "uri": "docs/stories/US-026/design.md",
                "content": "Grounded claims require citations."
            }],
            "claims": [{
                "statement": "This claim has no citation.",
                "citations": []
            }]
        }"#;

        let artifact = normalize_notebook_output(
            "US-026",
            "2026-06-07T00:00:00Z",
            "0.1.0",
            "run-3",
            ".harness/context/raw.json",
            stdout,
        );

        assert_eq!(artifact["status"], "fail");
        assert_eq!(artifact["errors"][0]["code"], "UNGROUNDED_PROVIDER_OUTPUT");
        assert!(artifact["errors"][0]["message"]
            .as_str()
            .unwrap()
            .contains("MISSING_CITATION"));
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
                codegraph_ingest_required: BoolFlag(0),
                notebooklm_ingest_required: BoolFlag(0),
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
                codegraph_ingest_required: None,
                notebooklm_ingest_required: None,
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
                codegraph_ingest_required: BoolFlag(0),
                notebooklm_ingest_required: BoolFlag(0),
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
                codegraph_ingest_required: BoolFlag(0),
                notebooklm_ingest_required: BoolFlag(0),
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
                codegraph_ingest_required: BoolFlag(0),
                notebooklm_ingest_required: BoolFlag(0),
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
                codegraph_ingest_required: BoolFlag(0),
                notebooklm_ingest_required: BoolFlag(0),
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
                codegraph_ingest_required: None,
                notebooklm_ingest_required: None,
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
    fn structured_friction_capture_validates_queries_and_renders_context() {
        let (temp_dir, repo_root, repository) = context_test_repository();
        repository.init().unwrap();
        add_context_story_and_intake(&repository, "US-FRIC", 0, 0);
        let trace_id = repository
            .record_trace(TraceInput {
                task_summary: "Trace with typed friction".to_owned(),
                intake_id: None,
                story_id: Some("US-FRIC".to_owned()),
                agent: Some("codex".to_owned()),
                outcome: Some("completed".to_owned()),
                duration_seconds: None,
                token_estimate: None,
                friction: Some("Provider proof missing".to_owned()),
                notes: None,
                actions: CsvList::from_optional(Some("captured friction".to_owned())),
                files_read: CsvList::from_optional(Some("docs/FRICTION_TAXONOMY.md".to_owned())),
                files_changed: CsvList::from_optional(None),
                decisions: CsvList::from_optional(Some(
                    "kept provider outage inconclusive".to_owned(),
                )),
                errors: CsvList::from_optional(Some("none".to_owned())),
            })
            .unwrap();

        let mut valid = FrictionAddInput {
            event_id: Some("11111111-1111-4111-8111-111111111111".to_owned()),
            story_id: Some("US-FRIC".to_owned()),
            trace_id: Some(trace_id),
            friction_type: FrictionType::ProviderUnavailable,
            severity: FrictionSeverity::High,
            source: FrictionSource::Trace,
            summary: "NotebookLM profile was unavailable during proof capture.".to_owned(),
            observed_at: Some("2026-06-07T00:00:00Z".to_owned()),
            provider: Some("notebooklm-mcp-cli".to_owned()),
            affected_paths: CsvList::from_optional(Some(
                "docs/stories/US-026/validation.md".to_owned(),
            )),
            evidence: FrictionEvidenceInput {
                command: Some("nlm login --check".to_owned()),
                exit_code: Some(1),
                artifact_path: None,
                report_path: None,
                details: Some("Profile not found: default".to_owned()),
            },
            proposed_action: FrictionProposedActionInput {
                action_type: Some(FrictionActionType::ProviderPreflight),
                title: Some("Add NotebookLM provider preflight".to_owned()),
                target_path: Some("docs/stories/US-030/overview.md".to_owned()),
            },
            notes: Some("Captured by US-030 structured friction test.".to_owned()),
        };

        let mut missing_provider = valid;
        missing_provider.event_id = Some("22222222-2222-4222-8222-222222222222".to_owned());
        missing_provider.provider = None;
        assert!(repository
            .add_friction_event(missing_provider)
            .unwrap_err()
            .to_string()
            .contains("requires --provider"));

        let missing_evidence = FrictionAddInput {
            event_id: Some("33333333-3333-4333-8333-333333333333".to_owned()),
            story_id: Some("US-FRIC".to_owned()),
            trace_id: None,
            friction_type: FrictionType::WeakValidation,
            severity: FrictionSeverity::High,
            source: FrictionSource::Agent,
            summary: "Validation was too weak for a high-risk task.".to_owned(),
            observed_at: Some("2026-06-07T00:00:00Z".to_owned()),
            provider: None,
            affected_paths: CsvList::from_optional(None),
            evidence: FrictionEvidenceInput::default(),
            proposed_action: FrictionProposedActionInput::default(),
            notes: None,
        };
        assert!(repository
            .add_friction_event(missing_evidence)
            .unwrap_err()
            .to_string()
            .contains("high severity requires evidence"));

        let invalid_observed_at = FrictionAddInput {
            event_id: Some("44444444-4444-4444-8444-444444444444".to_owned()),
            story_id: Some("US-FRIC".to_owned()),
            trace_id: None,
            friction_type: FrictionType::WeakValidation,
            severity: FrictionSeverity::Medium,
            source: FrictionSource::Agent,
            summary: "Validation date was malformed.".to_owned(),
            observed_at: Some("not-a-date".to_owned()),
            provider: None,
            affected_paths: CsvList::from_optional(None),
            evidence: FrictionEvidenceInput::default(),
            proposed_action: FrictionProposedActionInput::default(),
            notes: None,
        };
        assert!(repository
            .add_friction_event(invalid_observed_at)
            .unwrap_err()
            .to_string()
            .contains("observed_at must be RFC 3339"));

        valid = FrictionAddInput {
            event_id: Some("11111111-1111-4111-8111-111111111111".to_owned()),
            story_id: Some("US-FRIC".to_owned()),
            trace_id: Some(trace_id),
            friction_type: FrictionType::ProviderUnavailable,
            severity: FrictionSeverity::High,
            source: FrictionSource::Trace,
            summary: "NotebookLM profile was unavailable during proof capture.".to_owned(),
            observed_at: Some("2026-06-07T00:00:00Z".to_owned()),
            provider: Some("notebooklm-mcp-cli".to_owned()),
            affected_paths: CsvList::from_optional(Some(
                "docs/stories/US-026/validation.md".to_owned(),
            )),
            evidence: FrictionEvidenceInput {
                command: Some("nlm login --check".to_owned()),
                exit_code: Some(1),
                artifact_path: None,
                report_path: None,
                details: Some("Profile not found: default".to_owned()),
            },
            proposed_action: FrictionProposedActionInput {
                action_type: Some(FrictionActionType::ProviderPreflight),
                title: Some("Add NotebookLM provider preflight".to_owned()),
                target_path: Some("docs/stories/US-030/overview.md".to_owned()),
            },
            notes: Some("Captured by US-030 structured friction test.".to_owned()),
        };
        let event_id = repository.add_friction_event(valid).unwrap();
        assert_eq!(event_id, 1);
        let events = repository.query_friction_events().unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].friction_type, "provider_unavailable");
        assert_eq!(events[0].severity, "high");
        assert_eq!(events[0].provider.as_deref(), Some("notebooklm-mcp-cli"));

        let service = HarnessService::new(HarnessContext {
            repo_root: repo_root.clone(),
            db_path: temp_dir.path().join("harness.db"),
            schema_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .ancestors()
                .nth(2)
                .unwrap()
                .join("scripts/schema"),
        });
        let pack = service.generate_context_pack("US-FRIC").unwrap();
        let content = fs::read_to_string(pack).unwrap();
        assert!(content.contains("## 7. Structured Friction Events"));
        assert!(content.contains("provider_unavailable"));
        assert!(content.contains("notebooklm-mcp-cli"));
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
                codegraph_ingest_required: BoolFlag(0),
                notebooklm_ingest_required: BoolFlag(0),
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
                codegraph_ingest_required: None,
                notebooklm_ingest_required: None,
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
                codegraph_ingest_required: None,
                notebooklm_ingest_required: None,
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
                codegraph_ingest_required: BoolFlag(0),
                notebooklm_ingest_required: BoolFlag(0),
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
