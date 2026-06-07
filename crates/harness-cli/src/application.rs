use std::path::PathBuf;

use crate::domain::{
    ArchitectureCheckResult, BacklogFilter, BacklogRecord, BacklogSuggestionRecord, BoolFlag,
    CodeGraphMode, ContextIngestReport, ContextSource, CsvList, DecisionRecord, FrictionActionType,
    FrictionEventRecord, FrictionRecord, FrictionSeverity, FrictionSource, FrictionType,
    GovernanceReport, HarnessStats, HiosIdentity, InputType, IntakeRecord, MappedContext,
    ReleaseVerificationReport, RiskLane, RuleProposalRecord, StoryGateResult, StoryMatrixRecord,
    StoryVerifyStatus, TraceRecord, TraceScoreResult,
};
use crate::infrastructure::{HarnessRepository, SqliteHarnessRepository};

#[derive(Debug)]
pub struct HarnessContext {
    pub repo_root: PathBuf,
    pub db_path: PathBuf,
    pub schema_dir: PathBuf,
}

#[derive(Debug)]
pub struct IntakeInput {
    pub input_type: InputType,
    pub summary: String,
    pub risk_lane: RiskLane,
    pub risk_flags: CsvList,
    pub affected_docs: CsvList,
    pub story_id: Option<String>,
    pub notes: Option<String>,
    pub code_impact_summary: Option<String>,
    pub grounded_context: Option<String>,
    pub auto_generated: bool,
}

#[derive(Debug)]
pub struct ContextPackData {
    pub story_id: String,
    pub story_title: String,
    pub story_status: String,
    pub story_risk_lane: String,
    pub story_contract_doc: Option<String>,
    pub story_verify_command: Option<String>,
    pub story_notes: Option<String>,
    pub intake_input_type: Option<String>,
    pub intake_summary: Option<String>,
    pub intake_risk_flags: Option<String>,
    pub intake_affected_docs: Option<String>,
    pub intake_notes: Option<String>,
    pub code_impact_summary: Option<String>,
    pub grounded_context: Option<String>,
    pub context_ingests: Vec<ContextIngestSummary>,
    pub friction_events: Vec<FrictionEventRecord>,
}

#[derive(Debug)]
pub struct ContextIngestSummary {
    pub source: String,
    pub result: String,
    pub schema_version: Option<String>,
    pub artifact_path: String,
    pub artifact_sha256: String,
    pub summary: Option<String>,
    pub report_path: String,
    pub failure: Option<String>,
    pub checked_at: String,
}

#[derive(Debug)]
pub struct ContextIngestInput {
    pub story_id: String,
    pub source: ContextSource,
    pub file: PathBuf,
    pub output: Option<PathBuf>,
}

#[derive(Debug, Default)]
pub struct AutoIntakeEvidence {
    pub codegraph: Option<MappedContext>,
    pub notebooklm: Option<MappedContext>,
}

#[derive(Debug)]
pub struct CodeGraphImpactInput {
    pub story_id: String,
    pub mode: CodeGraphMode,
    pub changed_files: Option<PathBuf>,
    pub symbol: Option<String>,
    pub depth: u32,
    pub output: Option<PathBuf>,
    pub raw_output: Option<PathBuf>,
    pub executable: String,
}

#[derive(Debug)]
pub struct CodeGraphImpactResult {
    pub artifact_path: PathBuf,
    pub raw_output_path: Option<PathBuf>,
    pub provider_version: String,
    pub provider_command: String,
    pub ingest_report_path: PathBuf,
    pub ingest_report: ContextIngestReport,
}

#[derive(Debug)]
pub struct NotebookBriefInput {
    pub story_id: String,
    pub query: String,
    pub notebook: String,
    pub profile: Option<String>,
    pub timeout_seconds: Option<f64>,
    pub output: Option<PathBuf>,
    pub raw_output: Option<PathBuf>,
    pub executable: String,
}

#[derive(Debug)]
pub struct NotebookBriefResult {
    pub artifact_path: PathBuf,
    pub raw_output_path: Option<PathBuf>,
    pub provider_version: String,
    pub provider_command: String,
    pub ingest_report_path: PathBuf,
    pub ingest_report: ContextIngestReport,
}

#[derive(Debug)]
pub struct StoryAddInput {
    pub id: String,
    pub title: String,
    pub risk_lane: RiskLane,
    pub contract_doc: Option<String>,
    pub verify_command: Option<String>,
    pub notes: Option<String>,
    pub release_proof_required: BoolFlag,
    pub codegraph_ingest_required: BoolFlag,
    pub notebooklm_ingest_required: BoolFlag,
}

#[derive(Debug)]
pub struct StoryUpdateInput {
    pub id: String,
    pub status: Option<String>,
    pub evidence: Option<String>,
    pub unit: Option<BoolFlag>,
    pub integration: Option<BoolFlag>,
    pub e2e: Option<BoolFlag>,
    pub platform: Option<BoolFlag>,
    pub verify_command: Option<String>,
    pub release_proof_required: Option<BoolFlag>,
    pub codegraph_ingest_required: Option<BoolFlag>,
    pub notebooklm_ingest_required: Option<BoolFlag>,
}

#[derive(Debug)]
pub struct ReleaseVerifyInput {
    pub version: String,
    pub origin: Option<String>,
    pub platform: Option<String>,
    pub output: Option<PathBuf>,
    pub story_id: Option<String>,
}

#[derive(Debug)]
pub struct GovernanceReportInput {
    pub output: Option<PathBuf>,
}

#[derive(Debug)]
pub struct GovernanceReportResult {
    pub report_path: PathBuf,
    pub report: GovernanceReport,
}

#[derive(Debug)]
pub struct GovernanceDashboardInput {
    pub report: Option<PathBuf>,
    pub output: Option<PathBuf>,
}

#[derive(Debug)]
pub struct GovernanceDashboardResult {
    pub dashboard_path: PathBuf,
    pub report: GovernanceReport,
}

#[derive(Debug)]
pub struct DecisionAddInput {
    pub id: String,
    pub title: String,
    pub status: String,
    pub doc_path: Option<String>,
    pub verify_command: Option<String>,
    pub predicted_impact: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct BacklogAddInput {
    pub title: String,
    pub discovered_while: Option<String>,
    pub current_pain: Option<String>,
    pub suggestion: Option<String>,
    pub risk: Option<RiskLane>,
    pub predicted_impact: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug)]
pub struct BacklogCloseInput {
    pub id: i64,
    pub status: String,
    pub actual_outcome: Option<String>,
}

#[derive(Debug)]
pub struct BacklogSuggestInput {
    pub story_id: Option<String>,
    pub friction_type: Option<FrictionType>,
    pub min_severity: FrictionSeverity,
    pub limit: usize,
}

#[derive(Debug)]
pub struct RuleSuggestInput {
    pub story_id: Option<String>,
    pub friction_type: Option<FrictionType>,
    pub min_severity: FrictionSeverity,
    pub limit: usize,
}

#[derive(Debug)]
pub struct TraceInput {
    pub task_summary: String,
    pub intake_id: Option<i64>,
    pub story_id: Option<String>,
    pub agent: Option<String>,
    pub outcome: Option<String>,
    pub duration_seconds: Option<i64>,
    pub token_estimate: Option<i64>,
    pub friction: Option<String>,
    pub notes: Option<String>,
    pub actions: CsvList,
    pub files_read: CsvList,
    pub files_changed: CsvList,
    pub decisions: CsvList,
    pub errors: CsvList,
}

#[derive(Debug)]
pub struct FrictionAddInput {
    pub event_id: Option<String>,
    pub story_id: Option<String>,
    pub trace_id: Option<i64>,
    pub friction_type: FrictionType,
    pub severity: FrictionSeverity,
    pub source: FrictionSource,
    pub summary: String,
    pub observed_at: Option<String>,
    pub provider: Option<String>,
    pub affected_paths: CsvList,
    pub evidence: FrictionEvidenceInput,
    pub proposed_action: FrictionProposedActionInput,
    pub notes: Option<String>,
}

#[derive(Debug, Default)]
pub struct FrictionEvidenceInput {
    pub command: Option<String>,
    pub exit_code: Option<i64>,
    pub artifact_path: Option<String>,
    pub report_path: Option<String>,
    pub details: Option<String>,
}

#[derive(Debug, Default)]
pub struct FrictionProposedActionInput {
    pub action_type: Option<FrictionActionType>,
    pub title: Option<String>,
    pub target_path: Option<String>,
}

pub struct HarnessService {
    repository: SqliteHarnessRepository,
}

impl HarnessService {
    pub fn new(context: HarnessContext) -> Self {
        Self {
            repository: SqliteHarnessRepository::new(
                context.repo_root,
                context.db_path,
                context.schema_dir,
            ),
        }
    }

    pub fn init(&self) -> crate::infrastructure::Result<InitResult> {
        self.repository.init()
    }

    pub fn migrate(&self) -> crate::infrastructure::Result<MigrateResult> {
        self.repository.migrate()
    }

    pub fn import_brownfield(&self) -> crate::infrastructure::Result<BrownfieldImportResult> {
        self.repository.import_brownfield()
    }

    pub fn record_intake(&self, input: IntakeInput) -> crate::infrastructure::Result<i64> {
        self.repository.record_intake(input)
    }

    pub fn add_story(&self, input: StoryAddInput) -> crate::infrastructure::Result<()> {
        self.repository.add_story(input)
    }

    pub fn update_story(&self, input: StoryUpdateInput) -> crate::infrastructure::Result<()> {
        self.repository.update_story(input)
    }

    pub fn verify_story(&self, id: &str) -> crate::infrastructure::Result<StoryVerifyResult> {
        self.repository.verify_story(id)
    }

    pub fn verify_story_gate(&self, id: &str) -> crate::infrastructure::Result<StoryGateResult> {
        self.repository.verify_story_gate(id)
    }

    pub fn verify_release(
        &self,
        input: ReleaseVerifyInput,
    ) -> crate::infrastructure::Result<(PathBuf, ReleaseVerificationReport)> {
        self.repository.verify_release(input)
    }

    pub fn identity(&self) -> crate::infrastructure::Result<HiosIdentity> {
        self.repository.identity()
    }

    pub fn generate_governance_report(
        &self,
        input: GovernanceReportInput,
    ) -> crate::infrastructure::Result<GovernanceReportResult> {
        let (report_path, report) = self.repository.generate_governance_report(input)?;
        Ok(GovernanceReportResult {
            report_path,
            report,
        })
    }

    pub fn export_governance_dashboard(
        &self,
        input: GovernanceDashboardInput,
    ) -> crate::infrastructure::Result<GovernanceDashboardResult> {
        let (dashboard_path, report) = self.repository.export_governance_dashboard(input)?;
        Ok(GovernanceDashboardResult {
            dashboard_path,
            report,
        })
    }

    pub fn ingest_context(
        &self,
        input: ContextIngestInput,
    ) -> crate::infrastructure::Result<(PathBuf, ContextIngestReport)> {
        self.repository.ingest_context(input)
    }

    pub fn auto_intake_evidence(
        &self,
        story_id: &str,
    ) -> crate::infrastructure::Result<AutoIntakeEvidence> {
        self.repository.auto_intake_evidence(story_id)
    }

    pub fn produce_codegraph_impact(
        &self,
        input: CodeGraphImpactInput,
    ) -> crate::infrastructure::Result<CodeGraphImpactResult> {
        self.repository.produce_codegraph_impact(input)
    }

    pub fn produce_notebook_brief(
        &self,
        input: NotebookBriefInput,
    ) -> crate::infrastructure::Result<NotebookBriefResult> {
        self.repository.produce_notebook_brief(input)
    }

    pub fn check_architecture(
        &self,
        config_path: Option<PathBuf>,
        story_id: Option<&str>,
    ) -> crate::infrastructure::Result<ArchitectureCheckResult> {
        self.repository.check_architecture(config_path, story_id)
    }

    pub fn add_decision(&self, input: DecisionAddInput) -> crate::infrastructure::Result<()> {
        self.repository.add_decision(input)
    }

    pub fn verify_decision(&self, id: &str) -> crate::infrastructure::Result<DecisionVerifyResult> {
        self.repository.verify_decision(id)
    }

    pub fn add_backlog(&self, input: BacklogAddInput) -> crate::infrastructure::Result<i64> {
        self.repository.add_backlog(input)
    }

    pub fn close_backlog(&self, input: BacklogCloseInput) -> crate::infrastructure::Result<()> {
        self.repository.close_backlog(input)
    }

    pub fn record_trace(&self, input: TraceInput) -> crate::infrastructure::Result<i64> {
        self.repository.record_trace(input)
    }

    pub fn add_friction_event(
        &self,
        input: FrictionAddInput,
    ) -> crate::infrastructure::Result<i64> {
        self.repository.add_friction_event(input)
    }

    pub fn score_trace(&self, id: Option<i64>) -> crate::infrastructure::Result<TraceScoreResult> {
        self.repository.score_trace(id)
    }

    pub fn story_verify_status(
        &self,
        id: &str,
    ) -> crate::infrastructure::Result<StoryVerifyStatus> {
        self.repository.story_verify_status(id)
    }

    pub fn query_matrix(&self) -> crate::infrastructure::Result<Vec<StoryMatrixRecord>> {
        self.repository.query_matrix()
    }

    pub fn query_backlog(
        &self,
        filter: BacklogFilter,
    ) -> crate::infrastructure::Result<Vec<BacklogRecord>> {
        self.repository.query_backlog(filter)
    }

    pub fn suggest_backlog(
        &self,
        input: BacklogSuggestInput,
    ) -> crate::infrastructure::Result<Vec<BacklogSuggestionRecord>> {
        self.repository.suggest_backlog(input)
    }

    pub fn suggest_rules(
        &self,
        input: RuleSuggestInput,
    ) -> crate::infrastructure::Result<Vec<RuleProposalRecord>> {
        self.repository.suggest_rules(input)
    }

    pub fn query_decisions(&self) -> crate::infrastructure::Result<Vec<DecisionRecord>> {
        self.repository.query_decisions()
    }

    pub fn query_intakes(&self) -> crate::infrastructure::Result<Vec<IntakeRecord>> {
        self.repository.query_intakes()
    }

    pub fn query_traces(&self) -> crate::infrastructure::Result<Vec<TraceRecord>> {
        self.repository.query_traces()
    }

    pub fn query_friction(&self) -> crate::infrastructure::Result<Vec<FrictionRecord>> {
        self.repository.query_friction()
    }

    pub fn query_friction_events(&self) -> crate::infrastructure::Result<Vec<FrictionEventRecord>> {
        self.repository.query_friction_events()
    }

    pub fn query_stats(&self) -> crate::infrastructure::Result<HarnessStats> {
        self.repository.query_stats()
    }

    pub fn query_sql(&self, sql: &str) -> crate::infrastructure::Result<QueryTable> {
        self.repository.query_sql(sql)
    }

    pub fn generate_context_pack(&self, story_id: &str) -> crate::infrastructure::Result<PathBuf> {
        let data = self.repository.get_context_pack_data(story_id)?;

        let context_dir = self.repository.repo_root().join(".harness/context");
        std::fs::create_dir_all(&context_dir)?;

        let filepath = context_dir.join(format!("{}-context.md", story_id));

        let mut markdown = String::new();
        markdown.push_str(&format!("# Context Pack for Story {}\n\n", data.story_id));
        markdown.push_str("## 1. Story Overview\n");
        markdown.push_str(&format!("*   **Title:** {}\n", data.story_title));
        markdown.push_str(&format!("*   **Status:** {}\n", data.story_status));
        markdown.push_str(&format!("*   **Risk Lane:** {}\n", data.story_risk_lane));
        if let Some(doc) = &data.story_contract_doc {
            markdown.push_str(&format!(
                "*   **Product Contract:** [{doc}](file:///{})\n",
                self.repository
                    .repo_root()
                    .join(doc)
                    .display()
                    .to_string()
                    .replace('\\', "/")
            ));
        }
        if let Some(cmd) = &data.story_verify_command {
            markdown.push_str(&format!("*   **Verify Command:** `{}`\n", cmd));
        }
        if let Some(notes) = &data.story_notes {
            markdown.push_str(&format!("*   **Notes:** {}\n", notes));
        }
        markdown.push('\n');

        markdown.push_str("## 2. Intake Information\n");
        if let Some(input_type) = &data.intake_input_type {
            markdown.push_str(&format!("*   **Input Type:** {}\n", input_type));
        }
        if let Some(summary) = &data.intake_summary {
            markdown.push_str(&format!("*   **Intake Summary:** {}\n", summary));
        }
        if let Some(flags) = &data.intake_risk_flags {
            markdown.push_str(&format!("*   **Risk Flags:** {}\n", flags));
        }
        if let Some(docs) = &data.intake_affected_docs {
            markdown.push_str(&format!("*   **Affected Docs:** {}\n", docs));
        }
        if let Some(notes) = &data.intake_notes {
            markdown.push_str(&format!("*   **Intake Notes:** {}\n", notes));
        }
        markdown.push('\n');

        markdown.push_str("## 3. CodeGraph Impact Analysis\n");
        if let Some(impact) = &data.code_impact_summary {
            markdown.push_str(&format!("{}\n", impact));
        } else {
            markdown.push_str("*No CodeGraph impact data available.*\n");
        }
        markdown.push('\n');

        markdown.push_str("## 4. NotebookLM Grounded Context\n");
        if let Some(context) = &data.grounded_context {
            markdown.push_str(&format!("{}\n", context));
        } else {
            markdown.push_str("*No NotebookLM grounded context available.*\n");
        }
        markdown.push('\n');

        markdown.push_str("## 5. Architecture Constraints (docs/ARCHITECTURE.md)\n");
        markdown.push_str(
            "*   Inner layers must not depend on outer layers \
             (Domain <- Application <- Infrastructure <- Interface).\n",
        );
        markdown.push_str(
            "*   Unknown data must be parsed at boundaries before entering inner code.\n",
        );
        markdown.push('\n');

        markdown.push_str("## 6. Validated Context Ingest Evidence\n");
        if data.context_ingests.is_empty() {
            markdown.push_str("*No validated context ingest evidence available.*\n");
        } else {
            for ingest in &data.context_ingests {
                markdown.push_str(&format!(
                    "*   **{}:** {} (schema {}, artifact `{}`, SHA256 `{}`, report `{}`, checked {})\n",
                    ingest.source,
                    ingest.result,
                    ingest.schema_version.as_deref().unwrap_or("unknown"),
                    ingest.artifact_path,
                    ingest.artifact_sha256,
                    ingest.report_path,
                    ingest.checked_at
                ));
                if let Some(summary) = &ingest.summary {
                    markdown.push_str(&format!("    * Summary: {}\n", summary));
                }
                if let Some(failure) = &ingest.failure {
                    markdown.push_str(&format!("    * Diagnostics: {}\n", failure));
                }
            }
        }
        markdown.push('\n');

        markdown.push_str("## 7. Structured Friction Events\n");
        if data.friction_events.is_empty() {
            markdown.push_str("*No structured friction events captured for this story.*\n");
        } else {
            for event in &data.friction_events {
                markdown.push_str(&format!(
                    "*   **{}:** {} severity `{}` from `{}` (event `{}`, captured {})\n",
                    event.friction_type,
                    event.summary,
                    event.severity,
                    event.source,
                    event.event_id,
                    event.captured_at
                ));
                if let Some(trace_id) = event.trace_id {
                    markdown.push_str(&format!("    * Trace: #{}\n", trace_id));
                }
                if let Some(provider) = &event.provider {
                    markdown.push_str(&format!("    * Provider: {}\n", provider));
                }
            }
        }

        std::fs::write(&filepath, markdown)?;

        self.repository.update_story_context_pack_path(
            story_id,
            &format!(".harness/context/{}-context.md", story_id),
        )?;

        Ok(filepath)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum InitResult {
    Created { db_path: PathBuf },
    Existing { db_path: PathBuf, version: i64 },
    MigratedExisting { db_path: PathBuf },
}

#[derive(Debug, PartialEq, Eq)]
pub struct MigrateResult {
    pub current_version: i64,
    pub applied: Vec<i64>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BrownfieldImportResult {
    pub stories: usize,
    pub decisions: usize,
    pub backlog_items: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecisionVerifyResult {
    pub command: String,
    pub result: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryVerifyResult {
    pub command: String,
    pub stdout: String,
    pub stderr: String,
    pub result: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct QueryTable {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
