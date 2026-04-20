use crate::canonical_artifacts::{
    ArtifactPresence, CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
use crate::repo_file_access::{
    resolve_repo_relative_write_path, write_repo_relative_bytes, RepoRelativeMutationError,
    RepoRelativeWritePathError,
};
use agent_api::backends::codex::{CodexBackend, CodexBackendConfig};
use agent_api::{AgentWrapperBackend, AgentWrapperCompletion, AgentWrapperRunRequest};
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

const CANONICAL_CHARTER_REPO_PATH: &str = ".system/charter/CHARTER.md";
const CHARTER_INPUTS_SCHEMA_VERSION: &str = "0.1.0";
const SYNTHESIS_DIRECTIVE: &str =
    include_str!("../../../core/library/charter/charter_synthesize_directive.md");
const AUTHORING_METHOD: &str =
    include_str!("../../../core/library/authoring/charter_authoring_method.md");
const CHARTER_TEMPLATE: &str = include_str!("../../../core/library/charter/charter.md.tmpl");

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

pub fn validate_charter_structured_input(
    input: &CharterStructuredInput,
) -> Result<(), AuthorCharterRefusal> {
    let mut issues = Vec::new();

    if input.schema_version.trim() != CHARTER_INPUTS_SCHEMA_VERSION {
        issues.push(format!(
            "schema_version must be `{CHARTER_INPUTS_SCHEMA_VERSION}`"
        ));
    }

    require_non_empty("project.name", &input.project.name, &mut issues);
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

    if input.posture.rubric_scale.trim() != "1-5" {
        issues.push("posture.rubric_scale must be `1-5`".to_string());
    }
    require_level(
        "posture.baseline_level",
        input.posture.baseline_level,
        &mut issues,
    );
    require_non_empty_list(
        "posture.baseline_rationale",
        &input.posture.baseline_rationale,
        &mut issues,
    );

    for (index, domain) in input.domains.iter().enumerate() {
        let prefix = format!("domains[{index}]");
        require_non_empty(&format!("{prefix}.name"), &domain.name, &mut issues);
        require_non_empty(
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
        require_non_empty(
            &format!("{prefix}.default_stance"),
            &dimension.default_stance,
            &mut issues,
        );
        require_non_empty_list(
            &format!("{prefix}.raise_the_bar_triggers"),
            &dimension.raise_the_bar_triggers,
            &mut issues,
        );
        require_non_empty_list(
            &format!("{prefix}.allowed_shortcuts"),
            &dimension.allowed_shortcuts,
            &mut issues,
        );
        require_non_empty_list(
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

    require_non_empty_list(
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

    require_non_empty(
        "debt_tracking.system",
        &input.debt_tracking.system,
        &mut issues,
    );
    require_non_empty(
        "debt_tracking.review_cadence",
        &input.debt_tracking.review_cadence,
        &mut issues,
    );

    if input.decision_records.enabled {
        require_non_empty(
            "decision_records.path",
            &input.decision_records.path,
            &mut issues,
        );
        require_non_empty(
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
    let prompt = format!(
        "{directive}\n\n## Charter authoring method\n```markdown\n{method}\n```\n\n## Canonical output target\n- Write only `{path}`.\n- Return only the final markdown.\n\n## Template reference\n```markdown\n{template}\n```\n\n## Structured input source of truth\n```yaml\n{yaml}\n```\n",
        directive = SYNTHESIS_DIRECTIVE.trim_end(),
        method = AUTHORING_METHOD.trim_end(),
        path = CANONICAL_CHARTER_REPO_PATH,
        template = CHARTER_TEMPLATE.trim_end(),
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
        .map_err(|err| AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::SynthesisFailed,
            summary: format!("charter synthesis failed: {}", err.message),
            broken_subject: "charter synthesis runtime".to_string(),
            next_safe_action: "repair the synthesis runtime and retry `system author charter`"
                .to_string(),
        })?;
    if markdown.trim().is_empty() {
        return Err(AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::SynthesisFailed,
            summary: "charter synthesis failed: runtime returned empty output".to_string(),
            broken_subject: "charter synthesis runtime".to_string(),
            next_safe_action: "repair the synthesis runtime and retry `system author charter`"
                .to_string(),
        });
    }
    Ok(markdown)
}

pub fn author_charter(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
    author_charter_with_synthesizer(repo_root, input, &UnifiedAgentCharterSynthesizer)
}

pub fn author_charter_with_synthesizer(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
    synthesizer: &dyn CharterSynthesizer,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
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

fn require_non_empty_list(field: &str, values: &[String], issues: &mut Vec<String>) {
    if values.iter().all(|value| value.trim().is_empty()) {
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
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|err| {
                CharterSynthesisError::new(format!("tokio runtime init failed: {err}"))
            })?;
        runtime.block_on(run_codex_charter_synthesis(repo_root, request))
    }
}

async fn run_codex_charter_synthesis(
    repo_root: &Path,
    request: CharterSynthesisRequest,
) -> Result<String, CharterSynthesisError> {
    let backend = CodexBackend::new(CodexBackendConfig {
        default_working_dir: Some(repo_root.to_path_buf()),
        ..CodexBackendConfig::default()
    });
    let handle = backend
        .run(AgentWrapperRunRequest {
            prompt: request.prompt,
            working_dir: Some(repo_root.to_path_buf()),
            timeout: None,
            env: BTreeMap::new(),
            extensions: BTreeMap::new(),
        })
        .await
        .map_err(|err| CharterSynthesisError::new(err.to_string()))?;

    let AgentWrapperCompletion {
        status, final_text, ..
    } = collect_completion(handle).await?;
    if !status.success() {
        return Err(CharterSynthesisError::new(format!(
            "codex backend exited with status {status}"
        )));
    }
    let final_text = final_text
        .map(|text| text.trim().to_string())
        .filter(|text| !text.is_empty())
        .ok_or_else(|| {
            CharterSynthesisError::new("codex backend returned no final charter text")
        })?;
    Ok(final_text)
}

async fn collect_completion(
    handle: agent_api::AgentWrapperRunHandle,
) -> Result<AgentWrapperCompletion, CharterSynthesisError> {
    let mut events = handle.events;
    let completion = handle.completion;
    let mut text_fallback = String::new();
    let mut status_messages = Vec::new();

    while let Some(event) = events.next().await {
        if let Some(text) = event.text.as_deref() {
            if !text.trim().is_empty() {
                if !text_fallback.is_empty() {
                    text_fallback.push('\n');
                }
                text_fallback.push_str(text.trim());
            }
        }
        if let Some(message) = event.message.as_deref() {
            if !message.trim().is_empty() {
                status_messages.push(message.trim().to_string());
            }
        }
    }

    let mut completion = completion
        .await
        .map_err(|err| CharterSynthesisError::new(err.to_string()))?;
    if completion
        .final_text
        .as_deref()
        .map(|text| text.trim().is_empty())
        .unwrap_or(true)
        && !text_fallback.trim().is_empty()
    {
        completion.final_text = Some(text_fallback);
    }
    if !status_messages.is_empty() && completion.final_text.is_none() {
        return Err(CharterSynthesisError::new(status_messages.join(" | ")));
    }
    Ok(completion)
}
