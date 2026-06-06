use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum ParseHarnessValueError {
    #[error("unknown intake type '{0}'. Use: new spec, spec slice, change request, new initiative, maintenance request, or harness improvement")]
    InputType(String),
    #[error("unknown lane '{0}'. Use: tiny, normal, or high-risk. Use tiny instead of low.")]
    RiskLane(String),
    #[error("unknown context source '{0}'. Use: codegraph or notebooklm")]
    ContextSource(String),
    #[error("{0} must be an integer")]
    Integer(String),
    #[error("{0} must be 0 or 1. Example: --unit 1 --integration 1 --e2e 0 --platform 0")]
    BoolFlag(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InputType {
    NewSpec,
    SpecSlice,
    ChangeRequest,
    NewInitiative,
    Maintenance,
    HarnessImprovement,
}

impl InputType {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::NewSpec => "new_spec",
            Self::SpecSlice => "spec_slice",
            Self::ChangeRequest => "change_request",
            Self::NewInitiative => "new_initiative",
            Self::Maintenance => "maintenance",
            Self::HarnessImprovement => "harness_improvement",
        }
    }
}

impl FromStr for InputType {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_token(value);
        match normalized.as_str() {
            "new_spec" => Ok(Self::NewSpec),
            "spec_slice" => Ok(Self::SpecSlice),
            "change_request" => Ok(Self::ChangeRequest),
            "new_initiative" => Ok(Self::NewInitiative),
            "maintenance" | "maintenance_request" => Ok(Self::Maintenance),
            "harness_improvement" => Ok(Self::HarnessImprovement),
            _ => Err(ParseHarnessValueError::InputType(value.to_owned())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RiskLane {
    Tiny,
    Normal,
    HighRisk,
}

impl RiskLane {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::Tiny => "tiny",
            Self::Normal => "normal",
            Self::HighRisk => "high_risk",
        }
    }
}

impl FromStr for RiskLane {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let normalized = normalize_token(value);
        match normalized.as_str() {
            "tiny" => Ok(Self::Tiny),
            "normal" => Ok(Self::Normal),
            "high_risk" => Ok(Self::HighRisk),
            _ => Err(ParseHarnessValueError::RiskLane(value.to_owned())),
        }
    }
}

pub const RISK_LANE_HELP: &str =
    "Accepted lanes: tiny, normal, high-risk. Use tiny instead of low.";

#[derive(Debug, PartialEq, Eq)]
pub struct IntakeRecord {
    pub id: i64,
    pub created_at: String,
    pub input_type: String,
    pub risk_lane: String,
    pub summary: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryMatrixRecord {
    pub id: String,
    pub title: String,
    pub status: String,
    pub unit: i64,
    pub integration: i64,
    pub e2e: i64,
    pub platform: i64,
    pub evidence: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryVerifyStatus {
    pub id: String,
    pub verify_command: Option<String>,
    pub last_verified_result: Option<String>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ArchitectureConfig {
    #[serde(default)]
    pub layer: Vec<ArchitectureLayer>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ArchitectureLayer {
    pub name: String,
    pub path: String,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub forbidden_imports: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArchitectureViolation {
    pub file: String,
    pub import: String,
    pub rule: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct ArchitectureCheckResult {
    pub passed: bool,
    pub scanned_files: usize,
    pub violations: Vec<ArchitectureViolation>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct StoryGateResult {
    pub id: String,
    pub passed: bool,
    pub missing: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BacklogRecord {
    pub id: i64,
    pub title: String,
    pub status: String,
    pub risk: Option<String>,
    pub predicted_impact: Option<String>,
    pub actual_outcome: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BacklogFilter {
    All,
    Open,
    Closed,
}

#[derive(Debug, PartialEq, Eq)]
pub struct DecisionRecord {
    pub id: String,
    pub title: String,
    pub status: String,
    pub last_verified_at: Option<String>,
    pub last_verified_result: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraceRecord {
    pub id: i64,
    pub created_at: String,
    pub outcome: Option<String>,
    pub task_summary: String,
    pub harness_friction: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TraceQualityTier {
    Incomplete = 0,
    Minimal = 1,
    Standard = 2,
    Detailed = 3,
}

impl TraceQualityTier {
    pub fn label(self) -> &'static str {
        match self {
            Self::Incomplete => "incomplete",
            Self::Minimal => "minimal",
            Self::Standard => "standard",
            Self::Detailed => "detailed",
        }
    }

    pub fn score(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraceScoreSource {
    pub id: i64,
    pub task_summary: String,
    pub intake_id: Option<i64>,
    pub risk_lane: Option<String>,
    pub agent: Option<String>,
    pub actions_taken: Option<String>,
    pub files_read: Option<String>,
    pub files_changed: Option<String>,
    pub decisions_made: Option<String>,
    pub errors: Option<String>,
    pub outcome: Option<String>,
    pub duration_seconds: Option<i64>,
    pub token_estimate: Option<i64>,
    pub harness_friction: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraceScoreResult {
    pub trace_id: i64,
    pub achieved: TraceQualityTier,
    pub risk_lane: Option<String>,
    pub required: Option<TraceQualityTier>,
    pub meets_requirement: bool,
    pub missing_minimal: Vec<String>,
    pub missing_standard: Vec<String>,
    pub missing_detailed: Vec<String>,
}

pub fn required_trace_tier_for_lane(risk_lane: &str) -> Option<TraceQualityTier> {
    match risk_lane {
        "tiny" => Some(TraceQualityTier::Minimal),
        "normal" => Some(TraceQualityTier::Standard),
        "high_risk" => Some(TraceQualityTier::Detailed),
        _ => None,
    }
}

pub fn score_trace(source: TraceScoreSource) -> TraceScoreResult {
    let missing_minimal = missing_minimal_fields(&source);
    let missing_standard = if missing_minimal.is_empty() {
        missing_standard_fields(&source)
    } else {
        Vec::new()
    };
    let missing_detailed = if missing_minimal.is_empty() && missing_standard.is_empty() {
        missing_detailed_fields(&source)
    } else {
        Vec::new()
    };

    let achieved = if !missing_minimal.is_empty() {
        TraceQualityTier::Incomplete
    } else if !missing_standard.is_empty() {
        TraceQualityTier::Minimal
    } else if !missing_detailed.is_empty() {
        TraceQualityTier::Standard
    } else {
        TraceQualityTier::Detailed
    };
    let required = source
        .risk_lane
        .as_deref()
        .and_then(required_trace_tier_for_lane);
    let meets_requirement = required.is_none_or(|tier| achieved >= tier);

    TraceScoreResult {
        trace_id: source.id,
        achieved,
        risk_lane: source.risk_lane,
        required,
        meets_requirement,
        missing_minimal,
        missing_standard,
        missing_detailed,
    }
}

fn missing_minimal_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if source.task_summary.trim().len() < 10 {
        missing.push("task_summary: missing or shorter than 10 characters".to_owned());
    }
    if blank(&source.outcome) {
        missing.push("outcome: null".to_owned());
    }
    missing
}

fn missing_standard_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if blank(&source.agent) {
        missing.push("agent: empty".to_owned());
    }
    if short_json_list(&source.actions_taken) {
        missing.push("actions_taken: empty".to_owned());
    }
    if short_json_list(&source.files_read) {
        missing.push("files_read: empty".to_owned());
    }
    if source.files_changed.is_none() {
        missing.push("files_changed: null".to_owned());
    }
    if source.errors.is_none() && source.harness_friction.is_none() {
        missing.push("errors or harness_friction: both null".to_owned());
    }
    missing
}

fn missing_detailed_fields(source: &TraceScoreSource) -> Vec<String> {
    let mut missing = Vec::new();
    if short_json_list(&source.decisions_made) {
        missing.push("decisions_made: empty".to_owned());
    }
    if source.errors.is_none() {
        missing.push("errors: null".to_owned());
    }
    if source.harness_friction.is_none() {
        missing.push("harness_friction: null".to_owned());
    }
    if source.duration_seconds.is_none() && !notes_explain_missing(&source.notes, "duration") {
        missing.push("duration_seconds: null (no explanation in notes)".to_owned());
    }
    if source.token_estimate.is_none() && !notes_explain_missing(&source.notes, "token") {
        missing.push("token_estimate: null (no explanation in notes)".to_owned());
    }
    missing
}

fn blank(value: &Option<String>) -> bool {
    value.as_deref().map(str::trim).unwrap_or("").is_empty()
}

fn short_json_list(value: &Option<String>) -> bool {
    value.as_deref().map(str::trim).unwrap_or("").len() <= 2
}

fn notes_explain_missing(notes: &Option<String>, field: &str) -> bool {
    let Some(notes) = notes.as_deref() else {
        return false;
    };
    let lower = notes.to_ascii_lowercase();
    lower.contains(field)
        && (lower.contains("unavailable")
            || lower.contains("not available")
            || lower.contains("unknown"))
}

#[derive(Debug, PartialEq, Eq)]
pub struct FrictionRecord {
    pub id: i64,
    pub created_at: String,
    pub risk_lane: Option<String>,
    pub input_type: Option<String>,
    pub task_summary: String,
    pub harness_friction: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct HarnessStats {
    pub intakes: i64,
    pub stories: i64,
    pub decisions: i64,
    pub backlog_items: i64,
    pub traces: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CsvList(pub Option<String>);

impl CsvList {
    pub fn from_optional(value: Option<String>) -> Self {
        Self(value.filter(|item| !item.is_empty()))
    }

    pub fn as_json_text(&self) -> Option<String> {
        self.0.as_ref().map(|value| {
            let escaped_items = value
                .split(',')
                .map(|item| format!("\"{}\"", escape_json_string(item.trim())))
                .collect::<Vec<_>>()
                .join(",");
            format!("[{escaped_items}]")
        })
    }

    pub fn as_json_text_or_null_literal(&self) -> String {
        self.as_json_text().unwrap_or_else(|| "null".to_owned())
    }
}

impl fmt::Display for CsvList {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.as_json_text_or_null_literal())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoolFlag(pub i64);

impl BoolFlag {
    pub fn parse(label: &str, value: &str) -> Result<Self, ParseHarnessValueError> {
        match value {
            "0" => Ok(Self(0)),
            "1" => Ok(Self(1)),
            _ => Err(ParseHarnessValueError::BoolFlag(label.to_owned())),
        }
    }
}

pub fn parse_optional_integer(
    label: &str,
    value: Option<String>,
) -> Result<Option<i64>, ParseHarnessValueError> {
    value
        .map(|inner| {
            inner
                .parse::<i64>()
                .map_err(|_| ParseHarnessValueError::Integer(label.to_owned()))
        })
        .transpose()
}

fn escape_json_string(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub fn normalize_token(value: &str) -> String {
    let mut normalized = String::new();
    let mut last_was_separator = false;

    for character in value.trim().chars().flat_map(char::to_lowercase) {
        if character.is_ascii_alphanumeric() {
            normalized.push(character);
            last_was_separator = false;
        } else if !last_was_separator && !normalized.is_empty() {
            normalized.push('_');
            last_was_separator = true;
        }
    }

    while normalized.ends_with('_') {
        normalized.pop();
    }

    normalized
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContextSource {
    Codegraph,
    Notebooklm,
}

impl ContextSource {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::Codegraph => "codegraph",
            Self::Notebooklm => "notebooklm",
        }
    }

    pub fn artifact_type(&self) -> &'static str {
        match self {
            Self::Codegraph => "codegraph-impact",
            Self::Notebooklm => "notebooklm-brief",
        }
    }
}

impl FromStr for ContextSource {
    type Err = ParseHarnessValueError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match normalize_token(value).as_str() {
            "codegraph" => Ok(Self::Codegraph),
            "notebooklm" => Ok(Self::Notebooklm),
            _ => Err(ParseHarnessValueError::ContextSource(value.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ContextIngestStatus {
    Pass,
    Fail,
    Inconclusive,
}

impl ContextIngestStatus {
    pub fn as_db_value(&self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Inconclusive => "inconclusive",
        }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ContextSourceArtifact {
    pub artifact_type: String,
    pub artifact_id: String,
    pub schema_version: String,
    pub path: String,
    pub sha256: String,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct MappedContext {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub risk_flags: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affected_files: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub affected_docs: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_impact_summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grounded_context: Option<String>,
    pub claim_ids: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ContextIngestDiagnostic {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retryable: Option<bool>,
}

#[derive(Clone, Debug, Serialize)]
pub struct ContextIngestGovernance {
    pub eligible_for_intake: bool,
    pub eligible_for_context_pack: bool,
    pub eligible_for_story_verify: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct ContextIngestReport {
    pub schema_version: String,
    pub artifact_type: String,
    pub ingest_id: String,
    pub story_id: String,
    pub source: ContextSource,
    pub source_artifact: ContextSourceArtifact,
    pub status: ContextIngestStatus,
    pub checked_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapped_context: Option<MappedContext>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<ContextIngestDiagnostic>,
    pub governance: ContextIngestGovernance,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct ReleaseConfig {
    pub origin: String,
    pub tag_prefix: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ReleaseCheckResult {
    Pass,
    Fail,
    Inconclusive,
}

impl ReleaseCheckResult {
    pub fn as_db_value(self) -> &'static str {
        match self {
            Self::Pass => "pass",
            Self::Fail => "fail",
            Self::Inconclusive => "inconclusive",
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReleaseAssetEvidence {
    pub name: String,
    pub download_url: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ReleaseVerificationReport {
    pub checked_at_unix: u64,
    pub version: String,
    pub canonical_origin: String,
    pub origin: String,
    pub tag: String,
    pub platform: String,
    pub assets_checked: usize,
    pub assets: Vec<ReleaseAssetEvidence>,
    pub binary_asset: String,
    pub checksum_asset: String,
    pub expected_hash: Option<String>,
    pub actual_hash: Option<String>,
    pub download: ReleaseCheckResult,
    pub checksum: ReleaseCheckResult,
    pub version_check: ReleaseCheckResult,
    pub smoke_install: ReleaseCheckResult,
    pub version_output: Option<String>,
    pub smoke_output: Option<String>,
    pub failures: Vec<String>,
    pub result: ReleaseCheckResult,
}

pub fn path_has_any_segment(path: &str, candidates: &[&str]) -> bool {
    path.split(|character: char| {
        matches!(character, '/' | '\\' | ':' | '.')
            || !(character.is_ascii_alphanumeric() || character == '_')
    })
    .filter(|segment| !segment.is_empty())
    .any(|segment| {
        candidates
            .iter()
            .any(|candidate| segment.eq_ignore_ascii_case(candidate))
    })
}

pub fn yes_no(value: i64) -> String {
    if value == 1 {
        "yes".to_owned()
    } else {
        "no".to_owned()
    }
}

pub fn proof_display(value: i64, numeric: bool) -> String {
    if numeric {
        value.to_string()
    } else {
        yes_no(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_input_type_aliases() {
        assert_eq!("new_spec".parse::<InputType>().unwrap(), InputType::NewSpec);
        assert_eq!(
            "maintenance request".parse::<InputType>().unwrap(),
            InputType::Maintenance
        );
        assert_eq!(
            "Harness improvement".parse::<InputType>().unwrap(),
            InputType::HarnessImprovement
        );
    }

    #[test]
    fn parses_high_risk_lane_alias() {
        assert_eq!("high-risk".parse::<RiskLane>().unwrap(), RiskLane::HighRisk);
    }

    #[test]
    fn renders_csv_as_json_text() {
        assert_eq!(
            CsvList::from_optional(Some("auth, data model".to_owned()))
                .as_json_text_or_null_literal(),
            "[\"auth\",\"data model\"]"
        );
        assert_eq!(
            CsvList::from_optional(None).as_json_text_or_null_literal(),
            "null"
        );
    }

    #[test]
    fn parses_bool_flags() {
        assert_eq!(BoolFlag::parse("--unit", "1").unwrap(), BoolFlag(1));
        assert!(BoolFlag::parse("--unit", "yes").is_err());
    }

    #[test]
    fn path_segments_avoid_auth_substring_false_positives() {
        assert!(path_has_any_segment(
            r"src\authentication\session.ts",
            &["auth", "authentication", "login", "session"]
        ));
        assert!(!path_has_any_segment(
            "src/author/profile.ts",
            &["auth", "authentication", "login", "session"]
        ));
    }

    fn trace_source() -> TraceScoreSource {
        TraceScoreSource {
            id: 7,
            task_summary: "Completed a useful task".to_owned(),
            intake_id: None,
            risk_lane: None,
            agent: None,
            actions_taken: None,
            files_read: None,
            files_changed: None,
            decisions_made: None,
            errors: None,
            outcome: Some("completed".to_owned()),
            duration_seconds: None,
            token_estimate: None,
            harness_friction: None,
            notes: None,
        }
    }

    #[test]
    fn scores_minimal_standard_and_detailed_traces() {
        let minimal = score_trace(trace_source());
        assert_eq!(minimal.achieved, TraceQualityTier::Minimal);

        let mut standard_source = trace_source();
        standard_source.agent = Some("codex".to_owned());
        standard_source.actions_taken = Some("[\"read\",\"patched\"]".to_owned());
        standard_source.files_read = Some("[\"PHASE3.md\"]".to_owned());
        standard_source.files_changed = Some("[\"docs/TRACE_SPEC.md\"]".to_owned());
        standard_source.harness_friction = Some("none".to_owned());
        let standard = score_trace(standard_source);
        assert_eq!(standard.achieved, TraceQualityTier::Standard);

        let mut detailed_source = trace_source();
        detailed_source.agent = Some("codex".to_owned());
        detailed_source.actions_taken = Some("[\"read\",\"patched\"]".to_owned());
        detailed_source.files_read = Some("[\"PHASE3.md\"]".to_owned());
        detailed_source.files_changed = Some("[\"docs/TRACE_SPEC.md\"]".to_owned());
        detailed_source.decisions_made = Some("[\"kept schema unchanged\"]".to_owned());
        detailed_source.errors = Some("[\"none\"]".to_owned());
        detailed_source.harness_friction = Some("none".to_owned());
        detailed_source.duration_seconds = Some(120);
        detailed_source.token_estimate = Some(2000);
        let detailed = score_trace(detailed_source);
        assert_eq!(detailed.achieved, TraceQualityTier::Detailed);
    }

    #[test]
    fn compares_trace_score_to_lane_requirement() {
        let mut source = trace_source();
        source.risk_lane = Some("high_risk".to_owned());
        source.agent = Some("codex".to_owned());
        source.actions_taken = Some("[\"read\",\"patched\"]".to_owned());
        source.files_read = Some("[\"PHASE3.md\"]".to_owned());
        source.files_changed = Some("[\"docs/TRACE_SPEC.md\"]".to_owned());
        source.harness_friction = Some("none".to_owned());

        let result = score_trace(source);

        assert_eq!(result.achieved, TraceQualityTier::Standard);
        assert_eq!(result.required, Some(TraceQualityTier::Detailed));
        assert!(!result.meets_requirement);
        assert!(result
            .missing_detailed
            .iter()
            .any(|field| field.starts_with("decisions_made")));
    }
}
