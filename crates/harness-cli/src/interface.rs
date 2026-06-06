use std::env;
use std::path::PathBuf;
use std::str::FromStr;

use clap::{Args, Parser, Subcommand};
use thiserror::Error;

use crate::application::{
    BacklogAddInput, BacklogCloseInput, BrownfieldImportResult, CodeGraphImpactInput,
    ContextIngestInput, DecisionAddInput, HarnessContext, HarnessService, InitResult, IntakeInput,
    MigrateResult, NotebookBriefInput, QueryTable, ReleaseVerifyInput, StoryAddInput,
    StoryUpdateInput, TraceInput,
};
use crate::domain::{
    parse_optional_integer, path_has_any_segment, proof_display, BacklogFilter, BacklogRecord,
    BoolFlag, CodeGraphMode, ContextIngestStatus, ContextSource, CsvList, DecisionRecord,
    FrictionRecord, HarnessStats, InputType, IntakeRecord, MappedContext, ReleaseCheckResult,
    RiskLane, StoryMatrixRecord, TraceQualityTier, TraceRecord, TraceScoreResult, RISK_LANE_HELP,
};

#[derive(Parser, Debug)]
#[command(name = "harness-cli")]
#[command(about = "durable layer for the project harness", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create the harness database if it does not already exist.
    Init,
    /// Apply schema migrations.
    Migrate,
    /// Seed or refresh the database from existing markdown state.
    Import(ImportArgs),
    /// Record a feature intake classification.
    Intake(IntakeArgs),
    /// Add or update a story.
    Story(StoryArgs),
    /// Add a decision or run its verification.
    Decision(DecisionArgs),
    /// Add or close a backlog item.
    Backlog(BacklogArgs),
    /// Record an agent execution trace.
    Trace(TraceArgs),
    /// Score a trace against the trace quality tiers.
    ScoreTrace(ScoreTraceArgs),
    /// Query harness data.
    Query(QueryArgs),
    /// Generate a context pack for a story.
    Context(ContextArgs),
    /// Produce and ingest CodeGraph impact evidence.
    Codegraph(CodeGraphArgs),
    /// Produce and ingest NotebookLM grounded brief evidence.
    Notebooklm(NotebookLmArgs),
    /// Check configured architecture dependency boundaries.
    ArchCheck(ArchCheckArgs),
    /// Verify the trusted public release distribution chain.
    Release(ReleaseArgs),
}

#[derive(Args, Debug)]
#[command(after_help = RISK_LANE_HELP)]
struct IntakeArgs {
    #[arg(long = "type")]
    input_type: Option<String>,
    #[arg(long)]
    summary: String,
    #[arg(long, value_name = "tiny|normal|high-risk")]
    lane: Option<String>,
    #[arg(long)]
    flags: Option<String>,
    #[arg(long)]
    docs: Option<String>,
    #[arg(long)]
    story: Option<String>,
    #[arg(long)]
    notes: Option<String>,
    #[arg(long)]
    auto: bool,
    #[arg(long = "impact-report")]
    impact_report: Option<String>,
    #[arg(long = "business-context")]
    business_context: Option<String>,
}

#[derive(Args, Debug)]
struct ImportArgs {
    #[command(subcommand)]
    source: ImportSource,
}

#[derive(Subcommand, Debug)]
enum ImportSource {
    /// Import TEST_MATRIX, decisions, and backlog markdown.
    Brownfield,
}

#[derive(Args, Debug)]
struct StoryArgs {
    #[command(subcommand)]
    action: StoryAction,
}

#[derive(Subcommand, Debug)]
enum StoryAction {
    #[command(after_help = RISK_LANE_HELP)]
    Add(StoryAddArgs),
    #[command(
        after_help = "Proof flags use numeric booleans: --unit 1 --integration 1 --e2e 0 --platform 0. Do not use yes/no."
    )]
    Update(StoryUpdateArgs),
    #[command(
        after_help = "story verify runs the configured proof command, then enforces the governance gate. Record intake, context, architecture, proof, and trace evidence first."
    )]
    Verify {
        /// Story id to verify.
        id: String,
    },
}

#[derive(Args, Debug)]
struct StoryAddArgs {
    #[arg(long)]
    id: String,
    #[arg(long)]
    title: String,
    #[arg(long, value_name = "tiny|normal|high-risk")]
    lane: String,
    #[arg(long)]
    contract: Option<String>,
    #[arg(long)]
    verify: Option<String>,
    #[arg(long)]
    notes: Option<String>,
    #[arg(long = "release-proof", value_name = "0|1", default_value = "0")]
    release_proof: String,
    #[arg(long = "codegraph-ingest", value_name = "0|1", default_value = "0")]
    codegraph_ingest: String,
    #[arg(long = "notebooklm-ingest", value_name = "0|1", default_value = "0")]
    notebooklm_ingest: String,
}

#[derive(Args, Debug)]
struct StoryUpdateArgs {
    #[arg(long)]
    id: String,
    #[arg(long)]
    status: Option<String>,
    #[arg(long)]
    evidence: Option<String>,
    #[arg(long, value_name = "0|1")]
    unit: Option<String>,
    #[arg(long, value_name = "0|1")]
    integration: Option<String>,
    #[arg(long, value_name = "0|1")]
    e2e: Option<String>,
    #[arg(long, value_name = "0|1")]
    platform: Option<String>,
    #[arg(long)]
    verify: Option<String>,
    #[arg(long = "release-proof", value_name = "0|1")]
    release_proof: Option<String>,
    #[arg(long = "codegraph-ingest", value_name = "0|1")]
    codegraph_ingest: Option<String>,
    #[arg(long = "notebooklm-ingest", value_name = "0|1")]
    notebooklm_ingest: Option<String>,
}

#[derive(Args, Debug)]
struct ReleaseArgs {
    #[command(subcommand)]
    action: ReleaseAction,
}

#[derive(Subcommand, Debug)]
enum ReleaseAction {
    /// Verify public assets, checksum, binary version, and smoke execution.
    Verify(ReleaseVerifyArgs),
}

#[derive(Args, Debug)]
struct ReleaseVerifyArgs {
    #[arg(long)]
    version: String,
    #[arg(long)]
    origin: Option<String>,
    #[arg(long)]
    platform: Option<String>,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long)]
    story: Option<String>,
}

#[derive(Args, Debug)]
struct DecisionArgs {
    #[command(subcommand)]
    action: DecisionAction,
}

#[derive(Subcommand, Debug)]
enum DecisionAction {
    Add(DecisionAddArgs),
    Verify { id: String },
}

#[derive(Args, Debug)]
struct DecisionAddArgs {
    #[arg(long)]
    id: String,
    #[arg(long)]
    title: String,
    #[arg(long, default_value = "accepted")]
    status: String,
    #[arg(long)]
    doc: Option<String>,
    #[arg(long)]
    verify: Option<String>,
    #[arg(long)]
    predicted: Option<String>,
    #[arg(long)]
    notes: Option<String>,
}

#[derive(Args, Debug)]
struct BacklogArgs {
    #[command(subcommand)]
    action: BacklogAction,
}

#[derive(Subcommand, Debug)]
enum BacklogAction {
    #[command(after_help = RISK_LANE_HELP)]
    Add(BacklogAddArgs),
    Close(BacklogCloseArgs),
}

#[derive(Args, Debug)]
struct BacklogAddArgs {
    #[arg(long)]
    title: String,
    #[arg(long = "while")]
    discovered_while: Option<String>,
    #[arg(long)]
    pain: Option<String>,
    #[arg(long)]
    suggestion: Option<String>,
    #[arg(long, value_name = "tiny|normal|high-risk")]
    risk: Option<String>,
    #[arg(long)]
    predicted: Option<String>,
    #[arg(long)]
    notes: Option<String>,
}

#[derive(Args, Debug)]
struct BacklogCloseArgs {
    #[arg(long)]
    id: String,
    #[arg(long, default_value = "implemented")]
    status: String,
    #[arg(long)]
    outcome: Option<String>,
}

#[derive(Args, Debug)]
struct TraceArgs {
    #[arg(long)]
    summary: String,
    #[arg(long)]
    intake: Option<String>,
    #[arg(long)]
    story: Option<String>,
    #[arg(long)]
    agent: Option<String>,
    #[arg(long)]
    outcome: Option<String>,
    #[arg(long)]
    duration: Option<String>,
    #[arg(long)]
    tokens: Option<String>,
    #[arg(long)]
    friction: Option<String>,
    #[arg(long)]
    actions: Option<String>,
    #[arg(long = "read")]
    files_read: Option<String>,
    #[arg(long = "changed")]
    files_changed: Option<String>,
    #[arg(long)]
    decisions: Option<String>,
    #[arg(long)]
    errors: Option<String>,
    #[arg(long)]
    notes: Option<String>,
}

#[derive(Args, Debug)]
struct ScoreTraceArgs {
    /// Score a specific trace id. Defaults to the latest trace.
    #[arg(long)]
    id: Option<String>,
}

#[derive(Args, Debug)]
struct QueryArgs {
    #[command(subcommand)]
    view: QueryView,
}

#[derive(Args, Debug)]
#[command(args_conflicts_with_subcommands = true, subcommand_negates_reqs = true)]
struct ContextArgs {
    #[command(subcommand)]
    action: Option<ContextAction>,
    /// Story ID to generate context pack for (e.g. US-001).
    #[arg(long, required = true)]
    story: Option<String>,
}

#[derive(Subcommand, Debug)]
enum ContextAction {
    /// Validate and ingest a versioned external intelligence artifact.
    Ingest(ContextIngestArgs),
}

#[derive(Args, Debug)]
struct ContextIngestArgs {
    #[arg(long)]
    story: String,
    #[arg(long, value_name = "codegraph|notebooklm")]
    source: String,
    #[arg(long)]
    file: PathBuf,
    #[arg(long)]
    output: Option<PathBuf>,
}

#[derive(Args, Debug)]
struct CodeGraphArgs {
    #[command(subcommand)]
    action: CodeGraphAction,
}

#[derive(Subcommand, Debug)]
enum CodeGraphAction {
    /// Run CodeGraph analysis, normalize the artifact, and ingest it.
    Impact(CodeGraphImpactArgs),
}

#[derive(Args, Debug)]
struct CodeGraphImpactArgs {
    #[arg(long)]
    story: String,
    #[arg(
        long,
        value_name = "changed-files|symbol",
        default_value = "changed-files"
    )]
    mode: String,
    #[arg(long = "changed-files")]
    changed_files: Option<PathBuf>,
    #[arg(long)]
    symbol: Option<String>,
    #[arg(long, default_value = "2")]
    depth: String,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long = "raw-output")]
    raw_output: Option<PathBuf>,
    #[arg(long, default_value = "codegraph")]
    executable: String,
}

#[derive(Args, Debug)]
struct NotebookLmArgs {
    #[command(subcommand)]
    action: NotebookLmAction,
}

#[derive(Subcommand, Debug)]
enum NotebookLmAction {
    /// Run NotebookLM provider query, normalize the grounded brief, and ingest it.
    Brief(NotebookLmBriefArgs),
}

#[derive(Args, Debug)]
struct NotebookLmBriefArgs {
    #[arg(long)]
    story: String,
    #[arg(long)]
    query: String,
    #[arg(long)]
    notebook: String,
    #[arg(long)]
    profile: Option<String>,
    #[arg(long, default_value = "120")]
    timeout: String,
    #[arg(long)]
    output: Option<PathBuf>,
    #[arg(long = "raw-output")]
    raw_output: Option<PathBuf>,
    #[arg(long, default_value = "nlm")]
    executable: String,
}

#[derive(Args, Debug)]
struct ArchCheckArgs {
    /// Architecture rules file. Defaults to harness-architecture.toml.
    #[arg(long)]
    config: Option<PathBuf>,
    /// Story id that should receive the durable pass/fail result.
    #[arg(long)]
    story: Option<String>,
}

#[derive(Args, Debug)]
struct MatrixQueryArgs {
    /// Render proof flags as CLI input values, 1 and 0, instead of yes and no.
    #[arg(long)]
    numeric: bool,
}

#[derive(Args, Debug)]
struct BacklogQueryArgs {
    /// Show only proposed and accepted backlog items.
    #[arg(long, conflicts_with = "closed")]
    open: bool,
    /// Show only implemented and rejected backlog items.
    #[arg(long)]
    closed: bool,
}

#[derive(Subcommand, Debug)]
enum QueryView {
    /// Test matrix.
    Matrix(MatrixQueryArgs),
    /// Harness improvement proposals.
    Backlog(BacklogQueryArgs),
    /// Decision records.
    Decisions,
    /// Recent intake classifications.
    Intakes,
    /// Recent traces.
    Traces,
    /// Traces with harness friction.
    Friction,
    /// Summary counts.
    Stats,
    /// Run arbitrary SQL.
    Sql { query: Vec<String> },
}

#[derive(Debug, Error)]
pub enum InterfaceError {
    #[error("{0}")]
    ParseHarnessValue(#[from] crate::domain::ParseHarnessValueError),
    #[error("{0}")]
    Infrastructure(#[from] crate::infrastructure::HarnessInfraError),
    #[error("could not determine current directory: {0}")]
    CurrentDir(std::io::Error),
    #[error("query sql requires a SQL statement")]
    EmptySql,
    #[error("{0}")]
    InvalidNumber(String),
    #[error(
        "impact report must be a JSON object with top-level string fields and string arrays: {0}"
    )]
    InvalidImpactReport(String),
}

pub fn run(cli: Cli) -> Result<(), InterfaceError> {
    let service = HarnessService::new(resolve_context()?);

    match cli.command {
        Command::Init => print_init_result(service.init()?),
        Command::Migrate => print_migrate_result(service.migrate()?),
        Command::Import(args) => match args.source {
            ImportSource::Brownfield => {
                print_brownfield_import_result(service.import_brownfield()?)
            }
        },
        Command::Intake(args) => {
            let mut input_type_str = args.input_type.clone();
            let mut lane_str = args.lane.clone();
            let mut flags = Vec::new();
            let mut affected_docs = Vec::new();
            let mut code_impact_summary = None;
            let mut grounded_context = None;

            if args.auto {
                let mut has_ingested_codegraph = false;
                let mut has_ingested_notebooklm = false;
                if let Some(story_id) = &args.story {
                    let evidence = service.auto_intake_evidence(story_id)?;
                    if let Some(mapped) = evidence.codegraph {
                        merge_mapped_context_for_auto_intake(
                            &mapped,
                            &mut flags,
                            &mut affected_docs,
                            &mut code_impact_summary,
                            &mut grounded_context,
                        );
                        has_ingested_codegraph = true;
                    }
                    if let Some(mapped) = evidence.notebooklm {
                        merge_mapped_context_for_auto_intake(
                            &mapped,
                            &mut flags,
                            &mut affected_docs,
                            &mut code_impact_summary,
                            &mut grounded_context,
                        );
                        has_ingested_notebooklm = true;
                    }
                }

                let report_content = if !has_ingested_codegraph {
                    if let Some(path_or_json) = &args.impact_report {
                        if std::path::Path::new(path_or_json).exists() {
                            std::fs::read_to_string(path_or_json).ok()
                        } else {
                            Some(path_or_json.clone())
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };

                if let Some(json) = &report_content {
                    code_impact_summary = Some(json.clone());
                    let report = parse_impact_report(json)?;

                    if lane_str.is_none() {
                        if let Some(lane_val) = report.lane {
                            lane_str = Some(lane_val);
                        }
                    }

                    for flag in report.risk_flags {
                        merge_flag(&mut flags, &flag);
                    }

                    for file in report.affected_files {
                        merge_affected_file_for_auto_intake(&file, &mut flags, &mut affected_docs);
                    }
                }

                let brief_content = if !has_ingested_notebooklm {
                    if let Some(path_or_text) = &args.business_context {
                        if std::path::Path::new(path_or_text).exists() {
                            std::fs::read_to_string(path_or_text).ok()
                        } else {
                            Some(path_or_text.clone())
                        }
                    } else {
                        None
                    }
                } else {
                    None
                };
                if let Some(text) = brief_content {
                    grounded_context = Some(text);
                }

                if input_type_str.is_none() {
                    input_type_str = Some("change_request".to_owned());
                }

                if lane_str.is_none() {
                    let calculated_lane = if flags.contains(&"auth".to_owned())
                        || flags.contains(&"authorization".to_owned())
                        || flags.contains(&"data_model".to_owned())
                        || flags.contains(&"audit_security".to_owned())
                        || flags.len() >= 4
                    {
                        "high-risk"
                    } else if flags.len() >= 2 {
                        "normal"
                    } else {
                        "tiny"
                    };
                    lane_str = Some(calculated_lane.to_owned());
                }
            }

            let input_type_val = input_type_str.ok_or_else(|| {
                InterfaceError::ParseHarnessValue(crate::domain::ParseHarnessValueError::InputType(
                    "missing input type".to_owned(),
                ))
            })?;
            let lane_val = lane_str.ok_or_else(|| {
                InterfaceError::ParseHarnessValue(crate::domain::ParseHarnessValueError::RiskLane(
                    "missing risk lane".to_owned(),
                ))
            })?;

            let merged_flags = if let Some(user_flags) = &args.flags {
                let mut f = flags.clone();
                for flag in user_flags.split(',') {
                    let flag = crate::domain::normalize_token(flag);
                    if !flag.is_empty() && !f.contains(&flag) {
                        f.push(flag);
                    }
                }
                Some(f.join(","))
            } else if !flags.is_empty() {
                Some(flags.join(","))
            } else {
                None
            };

            let merged_docs = if let Some(user_docs) = &args.docs {
                let mut d = affected_docs.clone();
                for doc in user_docs.split(',') {
                    let doc = doc.trim().to_owned();
                    if !doc.is_empty() && !d.contains(&doc) {
                        d.push(doc);
                    }
                }
                Some(d.join(","))
            } else if !affected_docs.is_empty() {
                Some(affected_docs.join(","))
            } else {
                None
            };

            let id = service.record_intake(IntakeInput {
                input_type: InputType::from_str(&input_type_val)?,
                summary: args.summary,
                risk_lane: RiskLane::from_str(&lane_val)?,
                risk_flags: CsvList::from_optional(merged_flags),
                affected_docs: CsvList::from_optional(merged_docs),
                story_id: args.story,
                notes: args.notes,
                code_impact_summary,
                grounded_context,
                auto_generated: args.auto,
            })?;
            println!("Intake #{id} recorded.");
        }
        Command::Context(args) => {
            if let Some(action) = args.action {
                match action {
                    ContextAction::Ingest(args) => {
                        let (path, report) = service.ingest_context(ContextIngestInput {
                            story_id: args.story,
                            source: ContextSource::from_str(&args.source)?,
                            file: args.file,
                            output: args.output,
                        })?;
                        println!("Context source: {}", report.source.as_db_value());
                        println!("Artifact SHA256: {}", report.source_artifact.sha256);
                        for diagnostic in &report.diagnostics {
                            println!("Diagnostic {}: {}", diagnostic.code, diagnostic.message);
                        }
                        println!("Evidence: {}", path.display());
                        println!("Context ingest: {}", report.status.as_db_value());
                        if report.status != ContextIngestStatus::Pass {
                            std::process::exit(1);
                        }
                    }
                }
            } else {
                let story = args
                    .story
                    .expect("clap requires --story without a subcommand");
                let path = service.generate_context_pack(&story)?;
                println!("Context pack generated successfully at {}", path.display());
            }
        }
        Command::Codegraph(args) => match args.action {
            CodeGraphAction::Impact(args) => {
                let depth = args.depth.parse::<u32>().map_err(|_| {
                    InterfaceError::ParseHarnessValue(
                        crate::domain::ParseHarnessValueError::Integer(
                            "codegraph impact: --depth".to_owned(),
                        ),
                    )
                })?;
                if !(1..=10).contains(&depth) {
                    return Err(InterfaceError::ParseHarnessValue(
                        crate::domain::ParseHarnessValueError::Integer(
                            "codegraph impact: --depth must be between 1 and 10".to_owned(),
                        ),
                    ));
                }
                let result = service.produce_codegraph_impact(CodeGraphImpactInput {
                    story_id: args.story,
                    mode: CodeGraphMode::from_str(&args.mode)?,
                    changed_files: args.changed_files,
                    symbol: args.symbol,
                    depth,
                    output: args.output,
                    raw_output: args.raw_output,
                    executable: args.executable,
                })?;
                println!("Provider: codegraph-cli {}", result.provider_version);
                println!("Command: {}", result.provider_command);
                if let Some(path) = result.raw_output_path {
                    println!("Raw response: {}", path.display());
                }
                println!("Artifact: {}", result.artifact_path.display());
                println!("Ingest evidence: {}", result.ingest_report_path.display());
                for diagnostic in &result.ingest_report.diagnostics {
                    println!("Diagnostic {}: {}", diagnostic.code, diagnostic.message);
                }
                println!(
                    "CodeGraph impact: {}",
                    result.ingest_report.status.as_db_value()
                );
                if result.ingest_report.status != ContextIngestStatus::Pass {
                    std::process::exit(1);
                }
            }
        },
        Command::Notebooklm(args) => match args.action {
            NotebookLmAction::Brief(args) => {
                let result = service.produce_notebook_brief(NotebookBriefInput {
                    story_id: args.story,
                    query: args.query,
                    notebook: args.notebook,
                    profile: args.profile,
                    timeout_seconds: Some(parse_positive_float(
                        "notebooklm brief: --timeout",
                        &args.timeout,
                    )?),
                    output: args.output,
                    raw_output: args.raw_output,
                    executable: args.executable,
                })?;
                println!("Provider: notebooklm-mcp-cli {}", result.provider_version);
                println!("Command: {}", result.provider_command);
                if let Some(path) = result.raw_output_path {
                    println!("Raw response: {}", path.display());
                }
                println!("Artifact: {}", result.artifact_path.display());
                println!("Ingest evidence: {}", result.ingest_report_path.display());
                for diagnostic in &result.ingest_report.diagnostics {
                    println!("Diagnostic {}: {}", diagnostic.code, diagnostic.message);
                }
                println!(
                    "NotebookLM brief: {}",
                    result.ingest_report.status.as_db_value()
                );
                if result.ingest_report.status != ContextIngestStatus::Pass {
                    std::process::exit(1);
                }
            }
        },
        Command::ArchCheck(args) => {
            let result = service.check_architecture(args.config, args.story.as_deref())?;
            println!("Architecture files scanned: {}", result.scanned_files);
            if result.passed {
                println!("Architecture check passed.");
            } else {
                println!("Architecture check failed.");
                for violation in &result.violations {
                    println!("Violation: {} imports {}", violation.file, violation.import);
                    println!("Rule: {}", violation.rule);
                }
                std::process::exit(1);
            }
        }
        Command::Release(args) => match args.action {
            ReleaseAction::Verify(args) => {
                let (path, report) = service.verify_release(ReleaseVerifyInput {
                    version: args.version,
                    origin: args.origin,
                    platform: args.platform,
                    output: args.output,
                    story_id: args.story,
                })?;
                println!("Release tag: {}", report.tag);
                println!("Origin: {}", report.origin);
                println!("Platform: {}", report.platform);
                println!("Assets checked: {}", report.assets_checked);
                println!("Download: {}", report.download.as_db_value());
                println!("Checksum: {}", report.checksum.as_db_value());
                println!("Version: {}", report.version_check.as_db_value());
                println!("Smoke install: {}", report.smoke_install.as_db_value());
                for failure in &report.failures {
                    println!("Failure: {failure}");
                }
                println!("Evidence: {}", path.display());
                println!("Release verification: {}", report.result.as_db_value());
                if report.result != ReleaseCheckResult::Pass {
                    std::process::exit(1);
                }
            }
        },
        Command::Story(args) => match args.action {
            StoryAction::Add(args) => {
                service.add_story(StoryAddInput {
                    id: args.id.clone(),
                    title: args.title,
                    risk_lane: RiskLane::from_str(&args.lane)?,
                    contract_doc: args.contract,
                    verify_command: args.verify,
                    notes: args.notes,
                    release_proof_required: BoolFlag::parse(
                        "story add: --release-proof",
                        &args.release_proof,
                    )?,
                    codegraph_ingest_required: BoolFlag::parse(
                        "story add: --codegraph-ingest",
                        &args.codegraph_ingest,
                    )?,
                    notebooklm_ingest_required: BoolFlag::parse(
                        "story add: --notebooklm-ingest",
                        &args.notebooklm_ingest,
                    )?,
                })?;
                println!("Story {} added.", args.id);
            }
            StoryAction::Update(args) => {
                service.update_story(StoryUpdateInput {
                    id: args.id.clone(),
                    status: args.status,
                    evidence: args.evidence,
                    unit: parse_optional_bool("story update: --unit", args.unit)?,
                    integration: parse_optional_bool(
                        "story update: --integration",
                        args.integration,
                    )?,
                    e2e: parse_optional_bool("story update: --e2e", args.e2e)?,
                    platform: parse_optional_bool("story update: --platform", args.platform)?,
                    verify_command: args.verify,
                    release_proof_required: parse_optional_bool(
                        "story update: --release-proof",
                        args.release_proof,
                    )?,
                    codegraph_ingest_required: parse_optional_bool(
                        "story update: --codegraph-ingest",
                        args.codegraph_ingest,
                    )?,
                    notebooklm_ingest_required: parse_optional_bool(
                        "story update: --notebooklm-ingest",
                        args.notebooklm_ingest,
                    )?,
                })?;
                println!("Story {} updated.", args.id);
            }
            StoryAction::Verify { id } => {
                let result = service.verify_story(&id)?;
                println!("Running: {}", result.command);
                print!("{}", result.stdout);
                print!("{}", result.stderr);
                println!("Story {id} verification: {}", result.result);
                if result.result == "fail" {
                    std::process::exit(1);
                }
                let gate = service.verify_story_gate(&id)?;
                if gate.passed {
                    println!("Story {id} governance gate: pass");
                } else {
                    println!("Story {id} governance gate: fail");
                    println!("Missing:");
                    for item in gate.missing {
                        println!("  - {item}");
                    }
                    std::process::exit(1);
                }
            }
        },
        Command::Decision(args) => match args.action {
            DecisionAction::Add(args) => {
                service.add_decision(DecisionAddInput {
                    id: args.id.clone(),
                    title: args.title,
                    status: args.status,
                    doc_path: args.doc,
                    verify_command: args.verify,
                    predicted_impact: args.predicted,
                    notes: args.notes,
                })?;
                println!("Decision {} added.", args.id);
            }
            DecisionAction::Verify { id } => {
                let result = service.verify_decision(&id)?;
                println!("Running: {}", result.command);
                println!("Decision {id} verification: {}", result.result);
                if result.result == "fail" {
                    std::process::exit(1);
                }
            }
        },
        Command::Backlog(args) => match args.action {
            BacklogAction::Add(args) => {
                let id = service.add_backlog(BacklogAddInput {
                    title: args.title,
                    discovered_while: args.discovered_while,
                    current_pain: args.pain,
                    suggestion: args.suggestion,
                    risk: args
                        .risk
                        .map(|value| RiskLane::from_str(&value))
                        .transpose()?,
                    predicted_impact: args.predicted,
                    notes: args.notes,
                })?;
                println!("Backlog #{id} added.");
            }
            BacklogAction::Close(args) => {
                let id = parse_optional_integer("backlog close: --id", Some(args.id))?
                    .expect("value provided");
                let status = args.status;
                service.close_backlog(BacklogCloseInput {
                    id,
                    status: status.clone(),
                    actual_outcome: args.outcome,
                })?;
                println!("Backlog #{id} closed as {status}.");
            }
        },
        Command::Trace(args) => {
            let story_id = args.story.clone();
            let id = service.record_trace(TraceInput {
                task_summary: args.summary,
                intake_id: parse_optional_integer("trace: --intake", args.intake)?,
                story_id: args.story,
                agent: args.agent,
                outcome: args.outcome,
                duration_seconds: parse_optional_integer("trace: --duration", args.duration)?,
                token_estimate: parse_optional_integer("trace: --tokens", args.tokens)?,
                friction: args.friction,
                notes: args.notes,
                actions: CsvList::from_optional(args.actions),
                files_read: CsvList::from_optional(args.files_read),
                files_changed: CsvList::from_optional(args.files_changed),
                decisions: CsvList::from_optional(args.decisions),
                errors: CsvList::from_optional(args.errors),
            })?;
            println!("Trace #{id} recorded.");
            let result = service.score_trace(Some(id))?;
            print_trace_score(&result, false);
            if let Some(story_id) = story_id {
                print_story_verify_warning(&service, &story_id)?;
            }
        }
        Command::ScoreTrace(args) => {
            let id = parse_optional_integer("score-trace: --id", args.id)?;
            let result = service.score_trace(id)?;
            print_trace_score(&result, id.is_none());
            if !result.meets_requirement {
                std::process::exit(1);
            }
        }
        Command::Query(args) => match args.view {
            QueryView::Matrix(args) => print_matrix(&service.query_matrix()?, args.numeric),
            QueryView::Backlog(args) => {
                print_backlog(&service.query_backlog(backlog_filter(&args))?)
            }
            QueryView::Decisions => print_decisions(&service.query_decisions()?),
            QueryView::Intakes => print_intakes(&service.query_intakes()?),
            QueryView::Traces => print_traces(&service.query_traces()?),
            QueryView::Friction => print_friction(&service.query_friction()?),
            QueryView::Stats => print_stats(&service.query_stats()?),
            QueryView::Sql { query } => {
                if query.is_empty() {
                    return Err(InterfaceError::EmptySql);
                }
                print_query_table(&service.query_sql(&query.join(" "))?);
            }
        },
    }

    Ok(())
}

fn print_trace_score(result: &TraceScoreResult, latest: bool) {
    if latest {
        println!("Trace #{} (latest):", result.trace_id);
    } else {
        println!("Trace #{}:", result.trace_id);
    }
    println!(
        "  Tier achieved: {} ({}/3)",
        result.achieved.label(),
        result.achieved.score()
    );

    match (&result.risk_lane, result.required) {
        (Some(lane), Some(required)) => {
            println!(
                "  Lane: {} -> required tier: {} ({}/3)",
                lane,
                required.label(),
                required.score()
            );
            if result.meets_requirement {
                println!("  MEETS REQUIREMENT");
            } else {
                println!("  BELOW REQUIREMENT");
            }
        }
        _ => {
            println!("  Lane: unknown (no linked intake)");
        }
    }

    print_missing_fields(
        "minimal",
        TraceQualityTier::Minimal,
        &result.missing_minimal,
    );
    print_missing_fields(
        "standard",
        TraceQualityTier::Standard,
        &result.missing_standard,
    );
    print_missing_fields(
        "detailed",
        TraceQualityTier::Detailed,
        &result.missing_detailed,
    );
}

fn print_story_verify_warning(
    service: &HarnessService,
    story_id: &str,
) -> Result<(), InterfaceError> {
    let status = service.story_verify_status(story_id)?;
    let has_command = status
        .verify_command
        .as_deref()
        .map(str::trim)
        .is_some_and(|value| !value.is_empty());
    if has_command && status.last_verified_result.as_deref() != Some("pass") {
        println!();
        println!(
            "Warning: Story {} has verify_command but verification has not passed.",
            status.id
        );
        println!("Run: harness-cli story verify {}", status.id);
    }
    Ok(())
}

fn print_missing_fields(label: &str, tier: TraceQualityTier, fields: &[String]) {
    if fields.is_empty() {
        return;
    }
    println!();
    println!("  Missing for {label}:");
    for field in fields {
        println!("    - {field}");
    }
    if tier == TraceQualityTier::Detailed {
        println!();
    }
}

fn backlog_filter(args: &BacklogQueryArgs) -> BacklogFilter {
    if args.open {
        BacklogFilter::Open
    } else if args.closed {
        BacklogFilter::Closed
    } else {
        BacklogFilter::All
    }
}

fn print_brownfield_import_result(result: BrownfieldImportResult) {
    println!("Brownfield import complete.");
    println!("Stories imported or updated: {}", result.stories);
    println!("Decisions imported or updated: {}", result.decisions);
    println!("Backlog items discovered: {}", result.backlog_items);
}

fn parse_optional_bool(
    label: &str,
    value: Option<String>,
) -> Result<Option<BoolFlag>, InterfaceError> {
    value
        .map(|inner| BoolFlag::parse(label, &inner))
        .transpose()
        .map_err(InterfaceError::from)
}

fn parse_positive_float(label: &str, value: &str) -> Result<f64, InterfaceError> {
    let parsed = value
        .parse::<f64>()
        .map_err(|_| InterfaceError::InvalidNumber(format!("{label} must be a positive number")))?;
    if parsed.is_finite() && parsed > 0.0 {
        Ok(parsed)
    } else {
        Err(InterfaceError::InvalidNumber(format!(
            "{label} must be a positive number"
        )))
    }
}

fn merge_mapped_context_for_auto_intake(
    mapped: &MappedContext,
    flags: &mut Vec<String>,
    affected_docs: &mut Vec<String>,
    code_impact_summary: &mut Option<String>,
    grounded_context: &mut Option<String>,
) {
    for flag in &mapped.risk_flags {
        merge_flag(flags, flag);
    }
    for file in &mapped.affected_files {
        merge_affected_file_for_auto_intake(file, flags, affected_docs);
    }
    for doc in &mapped.affected_docs {
        merge_doc(affected_docs, doc);
    }
    if let Some(summary) = &mapped.code_impact_summary {
        *code_impact_summary = Some(summary.clone());
    }
    if let Some(context) = &mapped.grounded_context {
        *grounded_context = Some(context.clone());
    }
}

fn merge_flag(flags: &mut Vec<String>, flag: &str) {
    let flag = crate::domain::normalize_token(flag);
    if !flag.is_empty() && !flags.contains(&flag) {
        flags.push(flag);
    }
}

fn merge_doc(affected_docs: &mut Vec<String>, doc: &str) {
    let doc = doc.trim();
    if !doc.is_empty() && !affected_docs.iter().any(|existing| existing == doc) {
        affected_docs.push(doc.to_owned());
    }
}

fn merge_affected_file_for_auto_intake(
    file: &str,
    flags: &mut Vec<String>,
    affected_docs: &mut Vec<String>,
) {
    let file_lower = file.to_lowercase();
    if path_has_any_segment(
        &file_lower,
        &["auth", "authentication", "login", "session", "jwt"],
    ) {
        merge_flag(flags, "auth");
        merge_flag(flags, "authorization");
    }
    if path_has_any_segment(
        &file_lower,
        &[
            "db",
            "database",
            "migration",
            "migrations",
            "schema",
            "sql",
            "prisma",
        ],
    ) {
        merge_flag(flags, "data_model");
    }
    if path_has_any_segment(
        &file_lower,
        &["route", "routes", "controller", "controllers", "dto", "api"],
    ) {
        merge_flag(flags, "public_contracts");
    }
    if path_has_any_segment(&file_lower, &["audit", "security", "privacy"]) {
        merge_flag(flags, "audit_security");
    }
    if file_lower.ends_with(".md") && file_lower.contains("docs/") {
        merge_doc(affected_docs, file);
    }
}

fn print_init_result(result: InitResult) {
    match result {
        InitResult::Created { db_path } => {
            println!("Creating harness database at {}", db_path.display());
            println!("Schema applied.");
        }
        InitResult::Existing { db_path, version } => {
            println!("Database already exists at {}", db_path.display());
            println!("Current schema version: {version}");
        }
        InitResult::MigratedExisting { db_path } => {
            println!("Database already exists at {}", db_path.display());
            println!("No schema version found. Applying schema.");
            println!("Schema applied.");
        }
    }
}

fn print_migrate_result(result: MigrateResult) {
    println!("Current schema version: {}", result.current_version);
    if result.applied.is_empty() {
        println!("Already up to date.");
    } else {
        for version in &result.applied {
            println!("Applying migration {version}...");
        }
        println!("Applied {} migration(s).", result.applied.len());
    }
}

fn resolve_context() -> Result<HarnessContext, InterfaceError> {
    let repo_root = match env::var_os("HARNESS_REPO_ROOT") {
        Some(path) => PathBuf::from(path),
        None => env::current_dir().map_err(InterfaceError::CurrentDir)?,
    };
    let db_path = env::var_os("HARNESS_DB")
        .map(PathBuf::from)
        .unwrap_or_else(|| repo_root.join("harness.db"));

    let schema_dir = repo_root.join("scripts/schema");

    Ok(HarnessContext {
        repo_root,
        db_path,
        schema_dir,
    })
}

fn print_matrix(records: &[StoryMatrixRecord], numeric: bool) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.clone(),
                record.title.clone(),
                record.status.clone(),
                proof_display(record.unit, numeric),
                proof_display(record.integration, numeric),
                proof_display(record.e2e, numeric),
                proof_display(record.platform, numeric),
                record.evidence.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id", "title", "status", "unit", "integ", "e2e", "plat", "evidence",
        ],
        &rows,
    );
}

fn print_backlog(records: &[BacklogRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.title.clone(),
                record.status.clone(),
                record.risk.clone().unwrap_or_default(),
                record.predicted_impact.clone().unwrap_or_default(),
                record.actual_outcome.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "title",
            "status",
            "risk",
            "predicted_impact",
            "actual_outcome",
        ],
        &rows,
    );
}

fn print_decisions(records: &[DecisionRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.clone(),
                record.title.clone(),
                record.status.clone(),
                record.last_verified_at.clone().unwrap_or_default(),
                record.last_verified_result.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "title",
            "status",
            "last_verified_at",
            "last_verified_result",
        ],
        &rows,
    );
}

fn print_intakes(records: &[IntakeRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.created_at.clone(),
                record.input_type.clone(),
                record.risk_lane.clone(),
                record.summary.clone(),
            ]
        })
        .collect::<Vec<_>>();

    print_table(
        &["id", "created_at", "input_type", "risk_lane", "summary"],
        &rows,
    );
}

fn print_traces(records: &[TraceRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.created_at.clone(),
                record.outcome.clone().unwrap_or_default(),
                record.task_summary.clone(),
                record.harness_friction.clone().unwrap_or_default(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "created_at",
            "outcome",
            "task_summary",
            "harness_friction",
        ],
        &rows,
    );
}

fn print_friction(records: &[FrictionRecord]) {
    let rows = records
        .iter()
        .map(|record| {
            vec![
                record.id.to_string(),
                record.created_at.clone(),
                record.risk_lane.clone().unwrap_or_else(|| "-".to_owned()),
                record.input_type.clone().unwrap_or_else(|| "-".to_owned()),
                record.task_summary.clone(),
                record.harness_friction.clone(),
            ]
        })
        .collect::<Vec<_>>();
    print_table(
        &[
            "id",
            "created_at",
            "risk_lane",
            "input_type",
            "task_summary",
            "harness_friction",
        ],
        &rows,
    );
}

fn print_stats(stats: &HarnessStats) {
    println!("=== Harness Stats ===");
    print_table(
        &["intakes", "stories", "decisions", "backlog_items", "traces"],
        &[vec![
            stats.intakes.to_string(),
            stats.stories.to_string(),
            stats.decisions.to_string(),
            stats.backlog_items.to_string(),
            stats.traces.to_string(),
        ]],
    );
}

fn print_query_table(table: &QueryTable) {
    let headers = table.headers.iter().map(String::as_str).collect::<Vec<_>>();
    print_table(&headers, &table.rows);
}

fn print_table(headers: &[&str], rows: &[Vec<String>]) {
    let widths = headers
        .iter()
        .enumerate()
        .map(|(index, header)| {
            rows.iter()
                .filter_map(|row| row.get(index))
                .map(String::len)
                .chain(std::iter::once(header.len()))
                .max()
                .unwrap_or(header.len())
        })
        .collect::<Vec<_>>();

    print_row(
        &headers
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>(),
        &widths,
    );
    print_row(
        &widths
            .iter()
            .map(|width| "-".repeat(*width))
            .collect::<Vec<_>>(),
        &widths,
    );
    for row in rows {
        print_row(row, &widths);
    }
}

fn print_row(values: &[String], widths: &[usize]) {
    for (index, width) in widths.iter().enumerate() {
        if index > 0 {
            print!("  ");
        }
        let value = values.get(index).map(String::as_str).unwrap_or("");
        print!("{value:<width$}");
    }
    println!();
}

#[derive(Debug, PartialEq, Eq)]
struct ImpactReport {
    lane: Option<String>,
    risk_flags: Vec<String>,
    affected_files: Vec<String>,
}

fn parse_impact_report(json: &str) -> Result<ImpactReport, InterfaceError> {
    let value = serde_json::from_str::<serde_json::Value>(json)
        .map_err(|error| InterfaceError::InvalidImpactReport(error.to_string()))?;
    let object = value.as_object().ok_or_else(|| {
        InterfaceError::InvalidImpactReport("top-level value is not an object".to_owned())
    })?;

    Ok(ImpactReport {
        lane: optional_string_field(object, "lane")?,
        risk_flags: string_array_field(object, "risk_flags")?,
        affected_files: string_array_field(object, "affected_files")?,
    })
}

fn optional_string_field(
    object: &serde_json::Map<String, serde_json::Value>,
    field: &str,
) -> Result<Option<String>, InterfaceError> {
    match object.get(field) {
        None | Some(serde_json::Value::Null) => Ok(None),
        Some(serde_json::Value::String(value)) => Ok(Some(value.clone())),
        Some(_) => Err(InterfaceError::InvalidImpactReport(format!(
            "'{field}' must be a string"
        ))),
    }
}

fn string_array_field(
    object: &serde_json::Map<String, serde_json::Value>,
    field: &str,
) -> Result<Vec<String>, InterfaceError> {
    let Some(value) = object.get(field) else {
        return Ok(Vec::new());
    };
    let array = value.as_array().ok_or_else(|| {
        InterfaceError::InvalidImpactReport(format!("'{field}' must be a string array"))
    })?;
    array
        .iter()
        .map(|item| {
            item.as_str().map(str::to_owned).ok_or_else(|| {
                InterfaceError::InvalidImpactReport(format!("'{field}' must contain only strings"))
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::CommandFactory;

    #[test]
    fn cli_definition_is_valid() {
        Cli::command().debug_assert();
    }

    #[test]
    fn story_help_documents_proof_command_shape() {
        let mut command = Cli::command();
        let story = command.find_subcommand_mut("story").unwrap();

        let update_help = story
            .find_subcommand_mut("update")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(update_help.contains("--unit <0|1>"));
        assert!(update_help.contains("--integration <0|1>"));
        assert!(update_help.contains("Proof flags use numeric booleans"));

        let verify_help = story
            .find_subcommand_mut("verify")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(verify_help.contains("enforces the governance gate"));
        assert!(verify_help.contains("Record intake, context, architecture"));
    }

    #[test]
    fn command_help_documents_lane_values_and_version() {
        let mut command = Cli::command();
        assert!(command.render_long_help().to_string().contains("--version"));

        let intake_help = command
            .find_subcommand_mut("intake")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(intake_help.contains("--lane <tiny|normal|high-risk>"));
        assert!(intake_help.contains("Use tiny instead of low"));

        let story_add_help = command
            .find_subcommand_mut("story")
            .unwrap()
            .find_subcommand_mut("add")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(story_add_help.contains("--lane <tiny|normal|high-risk>"));

        let backlog_add_help = command
            .find_subcommand_mut("backlog")
            .unwrap()
            .find_subcommand_mut("add")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(backlog_add_help.contains("--risk <tiny|normal|high-risk>"));
        assert!(backlog_add_help.contains("Accepted lanes"));

        let matrix_help = command
            .find_subcommand_mut("query")
            .unwrap()
            .find_subcommand_mut("matrix")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(matrix_help.contains("--numeric"));

        let release_help = command
            .find_subcommand_mut("release")
            .unwrap()
            .find_subcommand_mut("verify")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(release_help.contains("--version <VERSION>"));
        assert!(release_help.contains("--origin <ORIGIN>"));
        assert!(release_help.contains("--story <STORY>"));

        let notebook_help = command
            .find_subcommand_mut("notebooklm")
            .unwrap()
            .find_subcommand_mut("brief")
            .unwrap()
            .render_long_help()
            .to_string();
        assert!(notebook_help.contains("--query <QUERY>"));
        assert!(notebook_help.contains("--notebook <NOTEBOOK>"));
        assert!(notebook_help.contains("--profile <PROFILE>"));
        assert!(notebook_help.contains("--timeout <TIMEOUT>"));
        assert!(notebook_help.contains("--raw-output <RAW_OUTPUT>"));
    }

    #[test]
    fn impact_report_parser_handles_missing_empty_and_nested_fields() {
        assert_eq!(
            parse_impact_report(r#"{"risk_flags":[]}"#).unwrap(),
            ImpactReport {
                lane: None,
                risk_flags: Vec::new(),
                affected_files: Vec::new(),
            }
        );
        assert_eq!(
            parse_impact_report(r#"{"report":{"affected_files":["src/auth/login.ts"]}}"#)
                .unwrap()
                .affected_files,
            Vec::<String>::new()
        );
        assert!(parse_impact_report("{not json").is_err());
        assert!(parse_impact_report(r#"{"affected_files":"src/auth/login.ts"}"#).is_err());
    }

    #[test]
    fn auto_intake_merge_uses_mapped_context_flags_docs_and_grounding() {
        let mapped = MappedContext {
            risk_flags: vec!["external_systems".to_owned()],
            affected_files: vec![
                "src/auth/session.rs".to_owned(),
                "docs/FEATURE_INTAKE.md".to_owned(),
            ],
            affected_docs: vec!["docs/HARNESS.md".to_owned()],
            code_impact_summary: Some("CodeGraph summary".to_owned()),
            grounded_context: Some("NotebookLM grounded context".to_owned()),
            claim_ids: vec!["CG-1".to_owned(), "NL-1".to_owned()],
        };
        let mut flags = Vec::new();
        let mut docs = Vec::new();
        let mut impact = None;
        let mut grounded = None;

        merge_mapped_context_for_auto_intake(
            &mapped,
            &mut flags,
            &mut docs,
            &mut impact,
            &mut grounded,
        );

        assert!(flags.contains(&"external_systems".to_owned()));
        assert!(flags.contains(&"auth".to_owned()));
        assert!(flags.contains(&"authorization".to_owned()));
        assert_eq!(
            docs,
            vec![
                "docs/FEATURE_INTAKE.md".to_owned(),
                "docs/HARNESS.md".to_owned()
            ]
        );
        assert_eq!(impact.as_deref(), Some("CodeGraph summary"));
        assert_eq!(grounded.as_deref(), Some("NotebookLM grounded context"));
    }
}
