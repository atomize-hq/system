use crate::canonical_artifacts::{
    ArtifactPresence, CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
use crate::repo_file_access::{
    resolve_repo_relative_write_path, write_repo_relative_bytes, RepoRelativeMutationError,
    RepoRelativeWritePathError,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

const CANONICAL_CHARTER_REPO_PATH: &str = ".system/charter/CHARTER.md";
const CHARTER_AUTHORING_LOCK_REPO_PATH: &str = ".system/state/authoring/charter.lock";
const CHARTER_INPUTS_SCHEMA_VERSION: &str = "0.1.0";
const SYNTHESIS_DIRECTIVE: &str =
    include_str!("../../../core/library/charter/charter_synthesize_directive.md");
const AUTHORING_METHOD: &str =
    include_str!("../../../core/library/authoring/charter_authoring_method.md");
const CHARTER_TEMPLATE: &str = include_str!("../../../core/library/charter/charter.md.tmpl");
const CHARTER_SYNTHESIS_OVERRIDE_ENV_VAR: &str = "SYSTEM_AUTHOR_CHARTER_SYNTHESIS_OVERRIDE";
const REQUIRED_CHARTER_TOP_LEVEL_HEADINGS: [&str; 12] = [
    "## What this is",
    "## How to use this charter",
    "## Rubric: 1–5 rigor levels",
    "## Project baseline posture",
    "## Domains / areas (optional overrides)",
    "## Posture at a glance (quick scan)",
    "## Dimensions (details + guardrails)",
    "## Cross-cutting red lines (global non-negotiables)",
    "## Exceptions / overrides process",
    "## Debt tracking expectations",
    "## Decision Records (ADRs): how to use this charter",
    "## Review & updates",
];

// Command path:
// `system author charter` or `system author charter --from-inputs <path|->`
// -> normalized `CharterStructuredInput`
// -> one shared synthesis request
// -> guarded write to `.system/charter/CHARTER.md`

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorCharterRefusalKind {
    MissingSystemRoot,
    InvalidSystemRoot,
    MalformedStructuredInput,
    IncompleteStructuredInput,
    ExistingCanonicalTruth,
    MutationRefused,
    SynthesisFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorCharterRefusal {
    pub kind: AuthorCharterRefusalKind,
    pub summary: String,
    pub broken_subject: String,
    pub next_safe_action: String,
}

impl std::fmt::Display for AuthorCharterRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for AuthorCharterRefusal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorCharterResult {
    pub canonical_repo_relative_path: &'static str,
    pub bytes_written: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharterSynthesisRequest {
    pub canonical_repo_relative_path: &'static str,
    pub inputs_yaml: String,
    pub prompt: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharterSynthesisError {
    pub message: String,
}

impl CharterSynthesisError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

pub trait CharterSynthesizer: Send + Sync {
    fn synthesize(
        &self,
        repo_root: &Path,
        request: CharterSynthesisRequest,
    ) -> Result<String, CharterSynthesisError>;
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterStructuredInput {
    pub schema_version: String,
    pub project: CharterProjectInput,
    pub posture: CharterPostureInput,
    #[serde(default)]
    pub domains: Vec<CharterDomainInput>,
    pub dimensions: Vec<CharterDimensionInput>,
    pub exceptions: CharterExceptionsInput,
    pub debt_tracking: CharterDebtTrackingInput,
    pub decision_records: CharterDecisionRecordsInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterProjectInput {
    pub name: String,
    pub classification: CharterProjectClassification,
    pub team_size: u32,
    pub users: CharterAudience,
    pub expected_lifetime: CharterExpectedLifetime,
    pub surfaces: Vec<CharterSurface>,
    pub runtime_environments: Vec<CharterRuntimeEnvironment>,
    pub constraints: CharterProjectConstraintsInput,
    pub operational_reality: CharterOperationalRealityInput,
    pub default_implications: CharterDefaultImplicationsInput,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterProjectConstraintsInput {
    pub deadline: String,
    pub budget: String,
    pub experience_notes: String,
    #[serde(default)]
    pub must_use_tech: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterOperationalRealityInput {
    pub in_production_today: bool,
    pub prod_users_or_data: String,
    #[serde(default)]
    pub external_contracts_to_preserve: Vec<String>,
    pub uptime_expectations: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterDefaultImplicationsInput {
    pub backward_compatibility: CharterBackwardCompatibility,
    pub migration_planning: CharterRequiredness,
    pub rollout_controls: CharterRolloutControls,
    pub deprecation_policy: CharterDeprecationPolicy,
    pub observability_threshold: CharterObservabilityThreshold,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterPostureInput {
    pub rubric_scale: String,
    pub baseline_level: u8,
    #[serde(default)]
    pub baseline_rationale: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterDomainInput {
    pub name: String,
    pub blast_radius: String,
    #[serde(default)]
    pub touches: Vec<String>,
    #[serde(default)]
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterDimensionInput {
    pub name: CharterDimensionName,
    pub level: Option<u8>,
    pub default_stance: String,
    #[serde(default)]
    pub raise_the_bar_triggers: Vec<String>,
    #[serde(default)]
    pub allowed_shortcuts: Vec<String>,
    #[serde(default)]
    pub red_lines: Vec<String>,
    #[serde(default)]
    pub domain_overrides: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterExceptionsInput {
    #[serde(default)]
    pub approvers: Vec<String>,
    pub record_location: String,
    #[serde(default)]
    pub minimum_fields: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterDebtTrackingInput {
    pub system: String,
    #[serde(default)]
    pub labels: Vec<String>,
    pub review_cadence: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CharterDecisionRecordsInput {
    pub enabled: bool,
    pub path: String,
    pub format: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterProjectClassification {
    Greenfield,
    Brownfield,
    Integration,
    Modernization,
    Hardening,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterAudience {
    Internal,
    External,
    Mixed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterExpectedLifetime {
    Days,
    Weeks,
    Months,
    Years,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterSurface {
    WebApp,
    Api,
    Cli,
    Lib,
    Infra,
    Ml,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterRuntimeEnvironment {
    Browser,
    Server,
    Cloud,
    OnPrem,
    Edge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterBackwardCompatibility {
    Required,
    NotRequired,
    BoundaryOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterRequiredness {
    Required,
    NotRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterRolloutControls {
    None,
    Lightweight,
    Required,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterDeprecationPolicy {
    Required,
    NotRequiredYet,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterObservabilityThreshold {
    Minimal,
    Standard,
    High,
    Regulated,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharterDimensionName {
    SpeedVsQuality,
    TypeSafetyStaticAnalysis,
    TestingRigor,
    ScalabilityPerformance,
    ReliabilityOperability,
    SecurityPrivacy,
    Observability,
    DxToolingAutomation,
    UxPolishApiUsability,
}

impl CharterDimensionName {
    fn all() -> &'static [Self; 9] {
        &[
            Self::SpeedVsQuality,
            Self::TypeSafetyStaticAnalysis,
            Self::TestingRigor,
            Self::ScalabilityPerformance,
            Self::ReliabilityOperability,
            Self::SecurityPrivacy,
            Self::Observability,
            Self::DxToolingAutomation,
            Self::UxPolishApiUsability,
        ]
    }

    fn field_slug(self) -> &'static str {
        match self {
            Self::SpeedVsQuality => "speed_vs_quality",
            Self::TypeSafetyStaticAnalysis => "type_safety_static_analysis",
            Self::TestingRigor => "testing_rigor",
            Self::ScalabilityPerformance => "scalability_performance",
            Self::ReliabilityOperability => "reliability_operability",
            Self::SecurityPrivacy => "security_privacy",
            Self::Observability => "observability",
            Self::DxToolingAutomation => "dx_tooling_automation",
            Self::UxPolishApiUsability => "ux_polish_api_usability",
        }
    }
}

pub fn parse_charter_structured_input_yaml(
    yaml: &str,
) -> Result<CharterStructuredInput, AuthorCharterRefusal> {
    let parsed = serde_yaml_bw::from_str::<CharterStructuredInput>(yaml).map_err(|err| {
        structured_input_refusal(
            AuthorCharterRefusalKind::MalformedStructuredInput,
            format!("structured charter input is malformed: {err}"),
        )
    })?;
    validate_charter_structured_input(&parsed)?;
    Ok(parsed)
}

pub fn normalize_charter_free_text(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn is_unusably_vague_charter_text(value: &str) -> bool {
    let normalized = normalize_charter_free_text(value);
    if normalized.is_empty() {
        return true;
    }

    let lower = normalized.to_ascii_lowercase();
    matches!(
        lower.as_str(),
        "idk"
            | "i don't know"
            | "dont know"
            | "unknown"
            | "n/a"
            | "na"
            | "tbd"
            | "todo"
            | "unsure"
            | "not sure"
            | "good quality"
            | "good"
            | "quality"
            | "standard"
            | "normal"
            | "stuff"
            | "things"
            | "misc"
            | "various"
            | "whatever"
    )
}

pub fn validate_charter_structured_input(
    input: &CharterStructuredInput,
) -> Result<(), AuthorCharterRefusal> {
    let mut issues = Vec::new();

    if input.schema_version.trim() != CHARTER_INPUTS_SCHEMA_VERSION {
        issues.push(format!(
            "schema_version must be `{CHARTER_INPUTS_SCHEMA_VERSION}`"
        ));
    }

    require_concrete_text("project.name", &input.project.name, &mut issues);
    if input.project.team_size == 0 {
        issues.push("project.team_size must be greater than 0".to_string());
    }
    if input.project.surfaces.is_empty() {
        issues.push("project.surfaces must include at least one surface".to_string());
    }
    if input.project.runtime_environments.is_empty() {
        issues.push(
            "project.runtime_environments must include at least one runtime environment"
                .to_string(),
        );
    }
    require_concrete_text(
        "project.constraints.experience_notes",
        &input.project.constraints.experience_notes,
        &mut issues,
    );

    if input.posture.rubric_scale.trim() != "1-5" {
        issues.push("posture.rubric_scale must be `1-5`".to_string());
    }
    require_level(
        "posture.baseline_level",
        input.posture.baseline_level,
        &mut issues,
    );
    require_concrete_list(
        "posture.baseline_rationale",
        &input.posture.baseline_rationale,
        &mut issues,
    );

    for (index, domain) in input.domains.iter().enumerate() {
        let prefix = format!("domains[{index}]");
        require_non_empty(&format!("{prefix}.name"), &domain.name, &mut issues);
        require_concrete_text(
            &format!("{prefix}.blast_radius"),
            &domain.blast_radius,
            &mut issues,
        );
    }

    let mut seen = BTreeSet::new();
    for (index, dimension) in input.dimensions.iter().enumerate() {
        let prefix = format!("dimensions[{index}]");
        if let Some(level) = dimension.level {
            require_level(&format!("{prefix}.level"), level, &mut issues);
        }
        if !seen.insert(dimension.name) {
            issues.push(format!(
                "{prefix}.name `{}` is duplicated",
                dimension.name.field_slug()
            ));
        }
        require_concrete_text(
            &format!("{prefix}.default_stance"),
            &dimension.default_stance,
            &mut issues,
        );
        require_concrete_list(
            &format!("{prefix}.raise_the_bar_triggers"),
            &dimension.raise_the_bar_triggers,
            &mut issues,
        );
        require_concrete_list(
            &format!("{prefix}.allowed_shortcuts"),
            &dimension.allowed_shortcuts,
            &mut issues,
        );
        require_concrete_list(
            &format!("{prefix}.red_lines"),
            &dimension.red_lines,
            &mut issues,
        );
    }
    for required in CharterDimensionName::all() {
        if !seen.contains(required) {
            issues.push(format!(
                "dimensions must include `{}`",
                required.field_slug()
            ));
        }
    }

    require_concrete_list(
        "exceptions.approvers",
        &input.exceptions.approvers,
        &mut issues,
    );
    require_non_empty(
        "exceptions.record_location",
        &input.exceptions.record_location,
        &mut issues,
    );
    require_non_empty_list(
        "exceptions.minimum_fields",
        &input.exceptions.minimum_fields,
        &mut issues,
    );

    require_concrete_text(
        "debt_tracking.system",
        &input.debt_tracking.system,
        &mut issues,
    );
    require_concrete_text(
        "debt_tracking.review_cadence",
        &input.debt_tracking.review_cadence,
        &mut issues,
    );

    if input.decision_records.enabled {
        require_concrete_text(
            "decision_records.path",
            &input.decision_records.path,
            &mut issues,
        );
        require_concrete_text(
            "decision_records.format",
            &input.decision_records.format,
            &mut issues,
        );
    }

    if issues.is_empty() {
        Ok(())
    } else {
        Err(structured_input_refusal(
            AuthorCharterRefusalKind::IncompleteStructuredInput,
            format!(
                "structured charter input is incomplete: {}",
                issues.join("; ")
            ),
        ))
    }
}

pub fn build_charter_synthesis_request(
    input: &CharterStructuredInput,
) -> Result<CharterSynthesisRequest, AuthorCharterRefusal> {
    validate_charter_structured_input(input)?;
    let inputs_yaml = serde_yaml_bw::to_string(input).map_err(|err| AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::MalformedStructuredInput,
        summary: format!("failed to serialize structured charter input: {err}"),
        broken_subject: "structured charter input".to_string(),
        next_safe_action:
            "repair the structured charter input and retry `system author charter --from-inputs <path|->`"
                .to_string(),
    })?;
    let inputs_yaml_block = inputs_yaml.trim_end_matches('\n');
    let render_template = charter_render_template_for_synthesis();
    let prompt = format!(
        "{directive}\n\n## Charter authoring method\n```markdown\n{method}\n```\n\n## Canonical output target\n- Write only `{path}`.\n- Return only the final markdown.\n\n## Template reference\n```markdown\n{template}\n```\n\n## Structured input source of truth\n```yaml\n{yaml}\n```\n",
        directive = SYNTHESIS_DIRECTIVE.trim_end(),
        method = AUTHORING_METHOD.trim_end(),
        path = CANONICAL_CHARTER_REPO_PATH,
        template = render_template,
        yaml = inputs_yaml_block,
    );
    Ok(CharterSynthesisRequest {
        canonical_repo_relative_path: CANONICAL_CHARTER_REPO_PATH,
        inputs_yaml,
        prompt,
    })
}

pub fn synthesize_charter_markdown(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    synthesize_charter_markdown_with(repo_root, input, &UnifiedAgentCharterSynthesizer)
}

pub fn synthesize_charter_markdown_with(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
    synthesizer: &dyn CharterSynthesizer,
) -> Result<String, AuthorCharterRefusal> {
    // Both future entrypoints converge here before the final LLM pass.
    let request = build_charter_synthesis_request(input)?;
    let markdown = synthesizer
        .synthesize(repo_root.as_ref(), request)
        .map_err(|err| {
            synthesis_failed_refusal(format!("charter synthesis failed: {}", err.message))
        })?;
    if markdown.trim().is_empty() {
        return Err(synthesis_failed_refusal(
            "charter synthesis failed: runtime returned empty output",
        ));
    }
    validate_synthesized_charter_markdown(&markdown)?;
    Ok(markdown)
}

pub fn author_charter(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
    author_charter_with_synthesizer(repo_root, input, &UnifiedAgentCharterSynthesizer)
}

pub fn preflight_author_charter(repo_root: impl AsRef<Path>) -> Result<(), AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    let artifacts = CanonicalArtifacts::load(repo_root).map_err(|err| AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::InvalidSystemRoot,
        summary: format!("failed to inspect canonical `.system` root: {err}"),
        broken_subject: "canonical `.system` root".to_string(),
        next_safe_action: "repair the canonical `.system` root and rerun `system setup`"
            .to_string(),
    })?;
    validate_authoring_preconditions(&artifacts)?;
    validate_charter_write_target(repo_root)?;
    Ok(())
}

pub fn author_charter_with_synthesizer(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
    synthesizer: &dyn CharterSynthesizer,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    preflight_author_charter(repo_root)?;
    let _lock = acquire_charter_authoring_lock(repo_root)?;
    preflight_author_charter(repo_root)?;

    let markdown = synthesize_charter_markdown_with(repo_root, input, synthesizer)?;
    write_repo_relative_bytes(repo_root, CANONICAL_CHARTER_REPO_PATH, markdown.as_bytes())
        .map_err(|err| AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::MutationRefused,
            summary: format_repo_mutation_error(CANONICAL_CHARTER_REPO_PATH, err),
            broken_subject: "canonical charter write target".to_string(),
            next_safe_action:
                "repair the blocked canonical charter path and retry `system author charter`"
                    .to_string(),
        })?;

    Ok(AuthorCharterResult {
        canonical_repo_relative_path: CANONICAL_CHARTER_REPO_PATH,
        bytes_written: markdown.len(),
    })
}

fn validate_authoring_preconditions(
    artifacts: &CanonicalArtifacts,
) -> Result<(), AuthorCharterRefusal> {
    match artifacts.system_root_status {
        SystemRootStatus::Ok => {}
        SystemRootStatus::Missing => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::MissingSystemRoot,
                summary:
                    "canonical `.system` root is missing; charter authoring requires setup first"
                        .to_string(),
                broken_subject: "canonical `.system` root".to_string(),
                next_safe_action: "run `system setup`".to_string(),
            })
        }
        SystemRootStatus::NotDir => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::InvalidSystemRoot,
                summary: "canonical `.system` root exists but is not a directory".to_string(),
                broken_subject: "canonical `.system` root".to_string(),
                next_safe_action: "repair the canonical `.system` root and rerun `system setup`"
                    .to_string(),
            })
        }
        SystemRootStatus::SymlinkNotAllowed => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::InvalidSystemRoot,
                summary: "canonical `.system` root cannot be a symlink".to_string(),
                broken_subject: "canonical `.system` root".to_string(),
                next_safe_action: "remove the `.system` symlink and rerun `system setup`"
                    .to_string(),
            })
        }
    }

    let charter = &artifacts.charter.identity;
    if charter.kind != CanonicalArtifactKind::Charter {
        return Err(AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::ExistingCanonicalTruth,
            summary: "unexpected canonical artifact identity for charter authoring".to_string(),
            broken_subject: "canonical charter truth".to_string(),
            next_safe_action:
                "inspect canonical artifact metadata and retry `system author charter`".to_string(),
        });
    }

    let existing_non_starter_truth = match charter.presence {
        ArtifactPresence::PresentNonEmpty => !charter.matches_setup_starter_template,
        ArtifactPresence::Missing | ArtifactPresence::PresentEmpty => false,
    };
    if existing_non_starter_truth {
        return Err(AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::ExistingCanonicalTruth,
            summary:
                "canonical charter truth already exists; `system author charter` only replaces missing, empty, or setup-starter content"
                    .to_string(),
            broken_subject: CANONICAL_CHARTER_REPO_PATH.to_string(),
            next_safe_action: format!(
                "inspect `{}` instead of rerunning `system author charter`",
                CANONICAL_CHARTER_REPO_PATH
            ),
        });
    }

    Ok(())
}

fn charter_render_template_for_synthesis() -> String {
    let mut rendered_lines = Vec::new();
    let mut inside_comment = false;
    let mut skip_nested_instruction_block = false;

    for line in CHARTER_TEMPLATE.lines() {
        let trimmed = line.trim();

        if inside_comment {
            if trimmed.contains("-->") {
                inside_comment = false;
            }
            continue;
        }

        if trimmed.starts_with("<!--") {
            if !trimmed.contains("-->") {
                inside_comment = true;
            }
            continue;
        }

        if skip_nested_instruction_block {
            if line.starts_with("    - ") || line.starts_with("  - ") {
                continue;
            }
            skip_nested_instruction_block = false;
        }

        if trimmed.starts_with('>') {
            continue;
        }

        if trimmed == "- Options (choose one):" {
            skip_nested_instruction_block = true;
            continue;
        }

        if trimmed.starts_with("- e.g.,") || trimmed.starts_with("- Examples:") {
            continue;
        }

        rendered_lines.push(line);
    }

    rendered_lines.join("\n").trim().to_string()
}

fn validate_synthesized_charter_markdown(markdown: &str) -> Result<(), AuthorCharterRefusal> {
    if markdown.contains("{{") || markdown.contains("}}") {
        return Err(synthesis_failed_refusal(
            "charter synthesis failed: runtime returned unresolved charter template placeholders",
        ));
    }

    if markdown.contains("<!--") || markdown.contains("-->") {
        return Err(synthesis_failed_refusal(
            "charter synthesis failed: runtime returned charter template commentary instead of final markdown",
        ));
    }

    if has_non_empty_content_before_first_heading(markdown) {
        return Err(synthesis_failed_refusal(
            "charter synthesis failed: runtime returned output that does not start with `# Engineering Charter`",
        ));
    }

    let headings = collect_structural_markdown_headings(markdown);
    let Some(first_heading) = headings.first() else {
        return Err(synthesis_failed_refusal(
            "charter synthesis failed: runtime returned no structural markdown headings",
        ));
    };

    if first_heading.level != 1 || !first_heading.text.starts_with("Engineering Charter") {
        return Err(synthesis_failed_refusal(
            "charter synthesis failed: runtime returned output whose first structural heading is not `# Engineering Charter`",
        ));
    }

    let required_headings = REQUIRED_CHARTER_TOP_LEVEL_HEADINGS
        .iter()
        .map(|heading| heading.trim_start_matches("## ").trim())
        .collect::<Vec<_>>();
    let mut next_required_index = 0usize;
    let mut seen_required = vec![false; required_headings.len()];

    for heading in headings.iter().filter(|heading| heading.level == 2) {
        let Some(required_index) = required_headings
            .iter()
            .position(|required| heading.text == *required)
        else {
            continue;
        };

        if seen_required[required_index] || required_index != next_required_index {
            return Err(synthesis_failed_refusal(
                "charter synthesis failed: runtime returned output that does not satisfy the shipped charter template",
            ));
        }

        seen_required[required_index] = true;
        next_required_index += 1;
    }

    if next_required_index != required_headings.len() {
        return Err(synthesis_failed_refusal(
            "charter synthesis failed: runtime returned output that does not satisfy the shipped charter template",
        ));
    }

    Ok(())
}

fn synthesis_failed_refusal(message: impl Into<String>) -> AuthorCharterRefusal {
    AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::SynthesisFailed,
        summary: message.into(),
        broken_subject: "charter synthesis runtime".to_string(),
        next_safe_action: "repair the synthesis runtime and retry `system author charter`"
            .to_string(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MarkdownHeading {
    level: usize,
    text: String,
}

fn collect_structural_markdown_headings(markdown: &str) -> Vec<MarkdownHeading> {
    let mut headings = Vec::new();
    let mut active_fence: Option<(char, usize)> = None;

    for line in markdown.lines() {
        if let Some((fence_char, fence_len)) = parse_fence_delimiter(line) {
            match active_fence {
                Some((active_char, active_len))
                    if active_char == fence_char && fence_len >= active_len =>
                {
                    active_fence = None;
                    continue;
                }
                None => {
                    active_fence = Some((fence_char, fence_len));
                    continue;
                }
                _ => continue,
            }
        }

        if active_fence.is_some() {
            continue;
        }

        if let Some(heading) = parse_atx_heading(line) {
            headings.push(heading);
        }
    }

    headings
}

fn has_non_empty_content_before_first_heading(markdown: &str) -> bool {
    let mut active_fence: Option<(char, usize)> = None;

    for line in markdown.lines() {
        if let Some((fence_char, fence_len)) = parse_fence_delimiter(line) {
            match active_fence {
                Some((active_char, active_len))
                    if active_char == fence_char && fence_len >= active_len =>
                {
                    active_fence = None;
                    continue;
                }
                None => {
                    active_fence = Some((fence_char, fence_len));
                }
                _ => {}
            }
        }

        if active_fence.is_some() {
            return true;
        }

        if parse_atx_heading(line).is_some() {
            return false;
        }

        if !line.trim().is_empty() {
            return true;
        }
    }

    false
}

fn parse_fence_delimiter(line: &str) -> Option<(char, usize)> {
    let indent = line.chars().take_while(|ch| *ch == ' ').count();
    if indent > 3 {
        return None;
    }

    let trimmed = &line[indent..];
    let mut chars = trimmed.chars();
    let fence_char = chars.next()?;
    if fence_char != '`' && fence_char != '~' {
        return None;
    }

    let fence_len = trimmed.chars().take_while(|ch| *ch == fence_char).count();
    if fence_len < 3 {
        return None;
    }

    Some((fence_char, fence_len))
}

fn parse_atx_heading(line: &str) -> Option<MarkdownHeading> {
    let indent = line.chars().take_while(|ch| *ch == ' ').count();
    if indent > 3 {
        return None;
    }

    let trimmed = &line[indent..];
    if trimmed.starts_with('>') {
        return None;
    }

    let level = trimmed.chars().take_while(|ch| *ch == '#').count();
    if level == 0 || level > 6 {
        return None;
    }

    let remainder = trimmed[level..].trim_start();
    if remainder.is_empty() {
        return None;
    }

    let text = remainder
        .trim_end()
        .trim_end_matches('#')
        .trim_end()
        .to_string();
    if text.is_empty() {
        return None;
    }

    Some(MarkdownHeading { level, text })
}

fn validate_charter_write_target(repo_root: &Path) -> Result<(), AuthorCharterRefusal> {
    resolve_repo_relative_write_path(repo_root, CANONICAL_CHARTER_REPO_PATH).map_err(|err| {
        AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(CANONICAL_CHARTER_REPO_PATH, err),
            broken_subject: "canonical charter write target".to_string(),
            next_safe_action:
                "repair the blocked canonical charter path and retry `system author charter`"
                    .to_string(),
        }
    })?;
    Ok(())
}

fn acquire_charter_authoring_lock(
    repo_root: &Path,
) -> Result<CharterAuthoringLockGuard, AuthorCharterRefusal> {
    let lock_path = resolve_repo_relative_write_path(repo_root, CHARTER_AUTHORING_LOCK_REPO_PATH)
        .map_err(|err| AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::MutationRefused,
        summary: format_repo_write_path_error(CHARTER_AUTHORING_LOCK_REPO_PATH, err),
        broken_subject: "charter authoring lock".to_string(),
        next_safe_action:
            "repair the blocked charter authoring lock path and retry `system author charter`"
                .to_string(),
    })?;

    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|source| charter_authoring_lock_refusal(&lock_path, source))?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|source| charter_authoring_lock_refusal(&lock_path, source))?;

    lock_charter_authoring_file(&file, libc::LOCK_EX)
        .map_err(|source| charter_authoring_lock_refusal(&lock_path, source))?;

    Ok(CharterAuthoringLockGuard { file, lock_path })
}

fn charter_authoring_lock_refusal(
    lock_path: &Path,
    source: std::io::Error,
) -> AuthorCharterRefusal {
    AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::MutationRefused,
        summary: format!(
            "failed to acquire exclusive charter authoring lock at {}: {source}",
            lock_path.display()
        ),
        broken_subject: "charter authoring lock".to_string(),
        next_safe_action:
            "wait for any in-progress `system author charter` run to finish or repair the lock path, then retry `system author charter`"
                .to_string(),
    }
}

#[cfg(unix)]
fn lock_charter_authoring_file(file: &File, operation: libc::c_int) -> Result<(), std::io::Error> {
    use std::os::unix::io::AsRawFd;

    loop {
        let result = unsafe { libc::flock(file.as_raw_fd(), operation) };
        if result == 0 {
            return Ok(());
        }

        let error = std::io::Error::last_os_error();
        if error.kind() == std::io::ErrorKind::Interrupted {
            continue;
        }

        return Err(error);
    }
}

#[cfg(not(unix))]
fn lock_charter_authoring_file(
    _file: &File,
    _operation: libc::c_int,
) -> Result<(), std::io::Error> {
    Ok(())
}

struct CharterAuthoringLockGuard {
    file: File,
    lock_path: PathBuf,
}

impl Drop for CharterAuthoringLockGuard {
    fn drop(&mut self) {
        #[cfg(unix)]
        let _ = lock_charter_authoring_file(&self.file, libc::LOCK_UN);
        let _ = &self.lock_path;
    }
}

fn structured_input_refusal(
    kind: AuthorCharterRefusalKind,
    summary: String,
) -> AuthorCharterRefusal {
    AuthorCharterRefusal {
        kind,
        summary,
        broken_subject: "structured charter input".to_string(),
        next_safe_action:
            "repair the structured charter input and retry `system author charter --from-inputs <path|->`"
                .to_string(),
    }
}

fn require_non_empty(field: &str, value: &str, issues: &mut Vec<String>) {
    if value.trim().is_empty() {
        issues.push(format!("{field} must not be empty"));
    }
}

fn require_concrete_text(field: &str, value: &str, issues: &mut Vec<String>) {
    let normalized = normalize_charter_free_text(value);
    if normalized.is_empty() {
        issues.push(format!("{field} must not be empty"));
        return;
    }
    if is_unusably_vague_charter_text(&normalized) {
        issues.push(format!("{field} must be concrete, not a placeholder"));
    }
}

fn require_non_empty_list(field: &str, values: &[String], issues: &mut Vec<String>) {
    if values.iter().all(|value| value.trim().is_empty()) {
        issues.push(format!("{field} must include at least one non-empty value"));
    }
}

fn require_concrete_list(field: &str, values: &[String], issues: &mut Vec<String>) {
    if values.is_empty() {
        issues.push(format!("{field} must include at least one non-empty value"));
        return;
    }

    let mut saw_concrete = false;
    for (index, value) in values.iter().enumerate() {
        let normalized = normalize_charter_free_text(value);
        if normalized.is_empty() {
            issues.push(format!("{field}[{index}] must not be empty"));
            continue;
        }
        if is_unusably_vague_charter_text(&normalized) {
            issues.push(format!(
                "{field}[{index}] must be concrete, not a placeholder"
            ));
            continue;
        }
        saw_concrete = true;
    }

    if !saw_concrete {
        issues.push(format!("{field} must include at least one non-empty value"));
    }
}

fn require_level(field: &str, value: u8, issues: &mut Vec<String>) {
    if !(1..=5).contains(&value) {
        issues.push(format!("{field} must be between 1 and 5"));
    }
}

fn format_repo_mutation_error(path: &str, err: RepoRelativeMutationError) -> String {
    match err {
        RepoRelativeMutationError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        RepoRelativeMutationError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        RepoRelativeMutationError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeMutationError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeMutationError::ReadFailure {
            path: found,
            source,
        }
        | RepoRelativeMutationError::WriteFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to mutate write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}

fn format_repo_write_path_error(path: &str, err: RepoRelativeWritePathError) -> String {
    match err {
        RepoRelativeWritePathError::InvalidPath(reason) => {
            format!("write target `{path}` is invalid: {reason}")
        }
        RepoRelativeWritePathError::ParentNotDirectory(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a directory",
                found.display()
            )
        }
        RepoRelativeWritePathError::NotRegularFile(found) => {
            format!(
                "write target `{path}` cannot be written because {} is not a regular file target",
                found.display()
            )
        }
        RepoRelativeWritePathError::SymlinkNotAllowed(found) => {
            format!(
                "write target `{path}` cannot be written through symlink {}",
                found.display()
            )
        }
        RepoRelativeWritePathError::ReadFailure {
            path: found,
            source,
        } => {
            format!(
                "failed to inspect write target `{path}` at {}: {source}",
                found.display()
            )
        }
    }
}

#[derive(Debug, Default)]
struct UnifiedAgentCharterSynthesizer;

impl CharterSynthesizer for UnifiedAgentCharterSynthesizer {
    fn synthesize(
        &self,
        repo_root: &Path,
        request: CharterSynthesisRequest,
    ) -> Result<String, CharterSynthesisError> {
        if let Ok(override_markdown) = std::env::var(CHARTER_SYNTHESIS_OVERRIDE_ENV_VAR) {
            return Ok(override_markdown);
        }

        run_codex_charter_synthesis(repo_root, request)
    }
}

fn run_codex_charter_synthesis(
    repo_root: &Path,
    request: CharterSynthesisRequest,
) -> Result<String, CharterSynthesisError> {
    let output_path = temp_output_last_message_path()?;
    let mut command = Command::new("codex");
    command
        .arg("exec")
        .arg("--color")
        .arg("never")
        .arg("--skip-git-repo-check")
        .arg("--json")
        .arg("--sandbox")
        .arg("workspace-write")
        .arg("--output-last-message")
        .arg(&output_path)
        .current_dir(repo_root)
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|err| {
        CharterSynthesisError::new(format!("failed to spawn `codex exec`: {err}"))
    })?;

    {
        let mut stdin = child
            .stdin
            .take()
            .ok_or_else(|| CharterSynthesisError::new("failed to open stdin for `codex exec`"))?;
        stdin
            .write_all(request.prompt.as_bytes())
            .and_then(|_| stdin.write_all(b"\n"))
            .map_err(|err| {
                CharterSynthesisError::new(format!("failed to write prompt to `codex exec`: {err}"))
            })?;
    }

    let output = child.wait_with_output().map_err(|err| {
        CharterSynthesisError::new(format!("failed waiting for `codex exec`: {err}"))
    })?;

    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if !output.status.success() {
        let detail = if stderr.is_empty() {
            format!("`codex exec` exited with status {}", output.status)
        } else {
            format!(
                "`codex exec` exited with status {}: {}",
                output.status, stderr
            )
        };
        let _ = std::fs::remove_file(&output_path);
        return Err(CharterSynthesisError::new(detail));
    }

    let final_text = std::fs::read_to_string(&output_path).map_err(|err| {
        CharterSynthesisError::new(format!(
            "failed to read `codex exec` final markdown from {}: {err}",
            output_path.display()
        ))
    })?;
    let _ = std::fs::remove_file(&output_path);
    let final_text = final_text.trim().to_string();
    if final_text.is_empty() {
        return Err(CharterSynthesisError::new(
            "`codex exec` returned no final charter text",
        ));
    }

    Ok(final_text)
}

fn temp_output_last_message_path() -> Result<PathBuf, CharterSynthesisError> {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| CharterSynthesisError::new(format!("system clock error: {err}")))?
        .as_nanos();
    Ok(std::env::temp_dir().join(format!(
        "system-charter-last-message-{}-{nanos}.md",
        std::process::id()
    )))
}
