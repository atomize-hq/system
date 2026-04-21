use crate::canonical_artifacts::{
    ArtifactPresence, CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
use crate::repo_file_access::{
    resolve_repo_relative_write_path, write_repo_relative_bytes, RepoRelativeMutationError,
    RepoRelativeWritePathError,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs::{File, OpenOptions};
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub const CANONICAL_CHARTER_REPO_PATH: &str = ".system/charter/CHARTER.md";
pub const DEFAULT_EXCEPTION_RECORD_LOCATION: &str = ".system/charter/CHARTER.md#exceptions";
const CHARTER_AUTHORING_LOCK_REPO_PATH: &str = ".system/state/authoring/charter.lock";
const CHARTER_INPUTS_SCHEMA_VERSION: &str = "0.1.0";
const AUTHORING_METHOD_MARKDOWN: &str =
    include_str!("../../../core/library/authoring/charter_authoring_method.md");
const CHARTER_SYNTHESIZE_DIRECTIVE_MARKDOWN: &str =
    include_str!("../../../core/library/charter/charter_synthesize_directive.md");
const CHARTER_TEMPLATE_MARKDOWN: &str =
    include_str!("../../../core/library/charter/charter.md.tmpl");
// Tests can override the codex binary path without changing the shipped CLI surface.
const AUTHOR_CHARTER_CODEX_BIN_ENV_VAR: &str = "SYSTEM_AUTHOR_CHARTER_CODEX_BIN";
const AUTHOR_CHARTER_CODEX_MODEL_ENV_VAR: &str = "SYSTEM_AUTHOR_CHARTER_CODEX_MODEL";
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
const CHARTER_TEMPLATE_SCAFFOLD_MARKERS: [&str; 18] = [
    "> **file:** `charter.md`",
    "> **created (utc):**",
    "> **owner:**",
    "> **team:**",
    "> **repo / project:**",
    "- examples: auth, payments, pii, regulated data, critical uptime, model inference, supply chain risk",
    "- options (choose one):",
    "- e.g., prod today?, live users?, existing data?, slas/slos?, external contracts?",
    "> use this section for **coarse areas** (domains/services) like auth/identity, pii/privacy, billing, ml inference, customer ux, admin tools, integrations, deployment pipeline.",
    "> **not** per-class or per-function.",
    "> if a field below is blank, it inherits the baseline level.",
    "> **format per dimension:**",
    "> - default stance (level)",
    "> - raise-the-bar triggers",
    "> - allowed shortcuts",
    "> - non-negotiables (red lines)",
    "> - domain/area overrides (only where needed)",
    "- e.g., production launch, first external customers, incident, scope change, new domain added",
];
const CHARTER_TEMPLATE_SCAFFOLD_PREFIX_MARKERS: [&str; 6] = [
    "- e.g., ts `strict`,",
    "- e.g., unit vs integration vs e2e;",
    "- e.g., basic slos, rollback expectations, on-call/ownership",
    "- e.g., structured logs, metrics, traces, alerts",
    "- e.g., ci, formatting, linting, release automation, local dev scripts",
    "- e.g., accessibility baseline, performance perception, error messaging clarity",
];
const PROCESS_SUMMARY_LINE_LIMIT: usize = 3;
const PROCESS_SUMMARY_CHAR_LIMIT: usize = 600;
const PROCESS_SUMMARY_HIGH_SIGNAL_MARKERS: [&str; 5] = [
    "error:",
    "unauthorized",
    "incorrect api key",
    "missing bearer",
    "failed",
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

#[derive(Clone, Copy)]
struct RubricLevel {
    level: u8,
    label: &'static str,
    meaning: &'static str,
}

const RUBRIC_LEVELS: [RubricLevel; 5] = [
    RubricLevel {
        level: 1,
        label: "Exploratory",
        meaning: "throwaway ok; optimize learning; minimal gates",
    },
    RubricLevel {
        level: 2,
        label: "Prototype",
        meaning: "demoable/internal use; some structure; still speed-first",
    },
    RubricLevel {
        level: 3,
        label: "Product",
        meaning: "real users; balanced; maintainability matters",
    },
    RubricLevel {
        level: 4,
        label: "Production",
        meaning: "GA/customer-facing; strong quality/reliability/security defaults",
    },
    RubricLevel {
        level: 5,
        label: "Hardened",
        meaning: "critical/regulated/high blast radius; strict gates; defense-in-depth",
    },
];

#[derive(Clone, Copy)]
struct DimensionMetadata {
    title: &'static str,
    table_label: &'static str,
}

fn rubric_level(level: u8) -> RubricLevel {
    RUBRIC_LEVELS[(level.saturating_sub(1)) as usize]
}

fn dimension_metadata(name: CharterDimensionName) -> DimensionMetadata {
    match name {
        CharterDimensionName::SpeedVsQuality => DimensionMetadata {
            title: "1) Speed vs Quality",
            table_label: "Speed vs Quality",
        },
        CharterDimensionName::TypeSafetyStaticAnalysis => DimensionMetadata {
            title: "2) Type safety / static analysis",
            table_label: "Type safety / static analysis",
        },
        CharterDimensionName::TestingRigor => DimensionMetadata {
            title: "3) Testing rigor",
            table_label: "Testing rigor",
        },
        CharterDimensionName::ScalabilityPerformance => DimensionMetadata {
            title: "4) Scalability & performance",
            table_label: "Scalability & performance",
        },
        CharterDimensionName::ReliabilityOperability => DimensionMetadata {
            title: "5) Reliability & operability",
            table_label: "Reliability & operability",
        },
        CharterDimensionName::SecurityPrivacy => DimensionMetadata {
            title: "6) Security & privacy",
            table_label: "Security & privacy",
        },
        CharterDimensionName::Observability => DimensionMetadata {
            title: "7) Observability",
            table_label: "Observability",
        },
        CharterDimensionName::DxToolingAutomation => DimensionMetadata {
            title: "8) Developer experience (DX)",
            table_label: "Developer experience (DX)",
        },
        CharterDimensionName::UxPolishApiUsability => DimensionMetadata {
            title: "9) UX polish / API usability",
            table_label: "UX polish / API usability",
        },
    }
}

impl CharterProjectClassification {
    fn display_name(self) -> &'static str {
        match self {
            Self::Greenfield => "greenfield",
            Self::Brownfield => "brownfield",
            Self::Integration => "integration",
            Self::Modernization => "modernization",
            Self::Hardening => "hardening",
        }
    }
}

impl CharterAudience {
    fn display_name(self) -> &'static str {
        match self {
            Self::Internal => "internal",
            Self::External => "external",
            Self::Mixed => "mixed",
        }
    }
}

impl CharterExpectedLifetime {
    fn display_name(self) -> &'static str {
        match self {
            Self::Days => "days",
            Self::Weeks => "weeks",
            Self::Months => "months",
            Self::Years => "years",
        }
    }
}

impl CharterSurface {
    fn display_name(self) -> &'static str {
        match self {
            Self::WebApp => "web app",
            Self::Api => "api",
            Self::Cli => "cli",
            Self::Lib => "library",
            Self::Infra => "infrastructure",
            Self::Ml => "ml",
        }
    }
}

impl CharterRuntimeEnvironment {
    fn display_name(self) -> &'static str {
        match self {
            Self::Browser => "browser",
            Self::Server => "server",
            Self::Cloud => "cloud",
            Self::OnPrem => "on-prem",
            Self::Edge => "edge",
        }
    }
}

impl CharterBackwardCompatibility {
    fn display_name(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::NotRequired => "not required",
            Self::BoundaryOnly => "boundary only",
        }
    }
}

impl CharterRequiredness {
    fn display_name(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::NotRequired => "not required",
        }
    }
}

impl CharterRolloutControls {
    fn display_name(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Lightweight => "lightweight",
            Self::Required => "required",
        }
    }
}

impl CharterDeprecationPolicy {
    fn display_name(self) -> &'static str {
        match self {
            Self::Required => "required",
            Self::NotRequiredYet => "not required yet",
        }
    }
}

impl CharterObservabilityThreshold {
    fn display_name(self) -> &'static str {
        match self {
            Self::Minimal => "minimal",
            Self::Standard => "standard",
            Self::High => "high",
            Self::Regulated => "regulated",
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

    collect_render_safety_issues(input, &mut issues);

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

pub fn render_charter_markdown(
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    validate_charter_structured_input(input)?;

    let project_name = normalize_charter_free_text(&input.project.name);
    let baseline = rubric_level(input.posture.baseline_level);
    let surfaces = join_display(
        input
            .project
            .surfaces
            .iter()
            .map(|surface| surface.display_name()),
    );
    let runtimes = join_display(
        input
            .project
            .runtime_environments
            .iter()
            .map(|runtime| runtime.display_name()),
    );
    let must_use_tech =
        render_inline_list_or_default(&input.project.constraints.must_use_tech, "none declared");
    let production_state = if input.project.operational_reality.in_production_today {
        "yes"
    } else {
        "no"
    };
    let external_contracts = render_inline_list_or_default(
        &input
            .project
            .operational_reality
            .external_contracts_to_preserve,
        "none declared",
    );
    let debt_labels = render_inline_list_or_default(&input.debt_tracking.labels, "none");

    let mut out = String::new();
    writeln!(out, "# Engineering Charter — {project_name}").unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## What this is").unwrap();
    writeln!(
        out,
        "This engineering charter is the canonical decision surface for {project_name}. It turns the project's stated baseline posture, domain constraints, and dimension-specific guardrails into the default rules for day-to-day engineering work."
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## How to use this charter").unwrap();
    writeln!(
        out,
        "- Default to the project baseline of level {} ({}) unless a dimension or domain section explicitly raises or lowers the bar.",
        baseline.level, baseline.label
    )
    .unwrap();
    writeln!(
        out,
        "- Use the dimension sections below to decide when to raise rigor, what shortcuts remain acceptable, and which red lines are non-negotiable."
    )
    .unwrap();
    writeln!(
        out,
        "- Record approved exceptions in {} before deviating from these defaults.",
        normalize_charter_free_text(&input.exceptions.record_location)
    )
    .unwrap();
    writeln!(
        out,
        "- Revisit this charter when the project classification, risk profile, or operating environment changes."
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Rubric: 1–5 rigor levels").unwrap();
    writeln!(
        out,
        "| Level | Label | Meaning |\n|------:|-------|---------|"
    )
    .unwrap();
    for level in RUBRIC_LEVELS {
        writeln!(
            out,
            "| {} | {} | {} |",
            level.level, level.label, level.meaning
        )
        .unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "## Project baseline posture").unwrap();
    writeln!(
        out,
        "- **Baseline level:** {} ({})",
        baseline.level, baseline.label
    )
    .unwrap();
    writeln!(out, "- **Baseline rationale:**").unwrap();
    for rationale in &input.posture.baseline_rationale {
        writeln!(out, "  - {}", normalize_charter_free_text(rationale)).unwrap();
    }
    writeln!(
        out,
        "- **Project classification:** {}",
        input.project.classification.display_name()
    )
    .unwrap();
    writeln!(out, "- **Users:** {}", input.project.users.display_name()).unwrap();
    writeln!(
        out,
        "- **Expected lifetime:** {}",
        input.project.expected_lifetime.display_name()
    )
    .unwrap();
    writeln!(out, "- **Team size:** {}", input.project.team_size).unwrap();
    writeln!(out, "- **Surfaces:** {surfaces}").unwrap();
    writeln!(out, "- **Runtime environments:** {runtimes}").unwrap();
    writeln!(
        out,
        "- **Deadline:** {}",
        render_inline_text_or_default(&input.project.constraints.deadline, "none declared")
    )
    .unwrap();
    writeln!(
        out,
        "- **Budget notes:** {}",
        render_inline_text_or_default(&input.project.constraints.budget, "none declared")
    )
    .unwrap();
    writeln!(
        out,
        "- **Experience notes:** {}",
        normalize_charter_free_text(&input.project.constraints.experience_notes)
    )
    .unwrap();
    writeln!(out, "- **Required technologies:** {must_use_tech}").unwrap();
    writeln!(out, "- **In production today:** {production_state}").unwrap();
    writeln!(
        out,
        "- **Production users or data:** {}",
        render_inline_text_or_default(
            &input.project.operational_reality.prod_users_or_data,
            "none declared"
        )
    )
    .unwrap();
    writeln!(
        out,
        "- **External contracts to preserve:** {external_contracts}"
    )
    .unwrap();
    writeln!(
        out,
        "- **Uptime expectations:** {}",
        render_inline_text_or_default(
            &input.project.operational_reality.uptime_expectations,
            "none declared"
        )
    )
    .unwrap();
    writeln!(
        out,
        "- **Backward compatibility default:** {}",
        input
            .project
            .default_implications
            .backward_compatibility
            .display_name()
    )
    .unwrap();
    writeln!(
        out,
        "- **Migration planning default:** {}",
        input
            .project
            .default_implications
            .migration_planning
            .display_name()
    )
    .unwrap();
    writeln!(
        out,
        "- **Rollout controls default:** {}",
        input
            .project
            .default_implications
            .rollout_controls
            .display_name()
    )
    .unwrap();
    writeln!(
        out,
        "- **Deprecation policy default:** {}",
        input
            .project
            .default_implications
            .deprecation_policy
            .display_name()
    )
    .unwrap();
    writeln!(
        out,
        "- **Observability threshold default:** {}",
        input
            .project
            .default_implications
            .observability_threshold
            .display_name()
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Domains / areas (optional overrides)").unwrap();
    if input.domains.is_empty() {
        writeln!(out, "None — baseline applies everywhere.").unwrap();
    } else {
        writeln!(
            out,
            "These domains add context for where the baseline posture needs extra care."
        )
        .unwrap();
        writeln!(out).unwrap();
        for domain in &input.domains {
            writeln!(out, "### {}", normalize_charter_free_text(&domain.name)).unwrap();
            writeln!(
                out,
                "- **Blast radius:** {}",
                normalize_charter_free_text(&domain.blast_radius)
            )
            .unwrap();
            writeln!(
                out,
                "- **Touches / trust boundary:** {}",
                render_inline_list_or_default(&domain.touches, "none declared")
            )
            .unwrap();
            writeln!(
                out,
                "- **Special constraints:** {}",
                render_inline_list_or_default(&domain.constraints, "none declared")
            )
            .unwrap();
            writeln!(
                out,
                "- **Default posture:** baseline applies unless a dimension override below says otherwise."
            )
            .unwrap();
            writeln!(out).unwrap();
        }
    }
    writeln!(out).unwrap();
    writeln!(out, "## Posture at a glance (quick scan)").unwrap();
    writeln!(
        out,
        "| Dimension | Default level (1–5) | Notes / intent |\n|---|---:|---|"
    )
    .unwrap();
    for name in CharterDimensionName::all() {
        let dimension = dimension_lookup(input, *name);
        let level = effective_dimension_level(dimension, input.posture.baseline_level);
        let metadata = dimension_metadata(*name);
        writeln!(
            out,
            "| {} | {} | {} |",
            metadata.table_label,
            level,
            escape_table_cell(&normalize_charter_free_text(&dimension.default_stance))
        )
        .unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "## Dimensions (details + guardrails)").unwrap();
    writeln!(
        out,
        "Each dimension inherits the project baseline unless an explicit level is set below."
    )
    .unwrap();
    writeln!(out).unwrap();
    for name in CharterDimensionName::all() {
        let dimension = dimension_lookup(input, *name);
        let level = effective_dimension_level(dimension, input.posture.baseline_level);
        let rubric = rubric_level(level);
        let metadata = dimension_metadata(*name);
        writeln!(out, "### {}", metadata.title).unwrap();
        writeln!(
            out,
            "- **Default stance (level):** {} ({})",
            rubric.level, rubric.label
        )
        .unwrap();
        writeln!(
            out,
            "- **Intent:** {}",
            normalize_charter_free_text(&dimension.default_stance)
        )
        .unwrap();
        writeln!(out, "**Raise the bar when:**").unwrap();
        push_bullets(&mut out, &dimension.raise_the_bar_triggers);
        writeln!(out).unwrap();
        writeln!(out, "**Allowed shortcuts when:**").unwrap();
        push_bullets(&mut out, &dimension.allowed_shortcuts);
        writeln!(out).unwrap();
        writeln!(out, "**Non-negotiables / red lines:**").unwrap();
        push_bullets(&mut out, &dimension.red_lines);
        writeln!(out).unwrap();
        writeln!(out, "**Domain overrides (if any):**").unwrap();
        if dimension.domain_overrides.is_empty() {
            writeln!(out, "- None — baseline applies.").unwrap();
        } else {
            push_bullets(&mut out, &dimension.domain_overrides);
        }
        writeln!(out).unwrap();
    }
    writeln!(out, "## Cross-cutting red lines (global non-negotiables)").unwrap();
    writeln!(
        out,
        "The following rules apply across the project unless an approved exception is recorded:"
    )
    .unwrap();
    for red_line in collect_cross_cutting_red_lines(input) {
        writeln!(out, "- {red_line}").unwrap();
    }
    writeln!(
        out,
        "- Preserve the project-wide defaults for {} and {} unless a feature-specific plan explicitly changes them.",
        input
            .project
            .default_implications
            .backward_compatibility
            .display_name(),
        input.project.default_implications.rollout_controls.display_name()
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Exceptions / overrides process").unwrap();
    writeln!(
        out,
        "- **Approvers:** {}",
        render_inline_list_or_default(&input.exceptions.approvers, "none declared")
    )
    .unwrap();
    writeln!(
        out,
        "- **Record location:** {}",
        normalize_charter_free_text(&input.exceptions.record_location)
    )
    .unwrap();
    writeln!(out, "- **Minimum required fields:**").unwrap();
    push_bullets(&mut out, &input.exceptions.minimum_fields);
    writeln!(out).unwrap();
    writeln!(out, "## Debt tracking expectations").unwrap();
    writeln!(
        out,
        "- **Tracking system:** {}",
        normalize_charter_free_text(&input.debt_tracking.system)
    )
    .unwrap();
    writeln!(out, "- **Labels:** {debt_labels}").unwrap();
    writeln!(
        out,
        "- **Review cadence:** {}",
        normalize_charter_free_text(&input.debt_tracking.review_cadence)
    )
    .unwrap();
    writeln!(out).unwrap();
    writeln!(out, "## Decision Records (ADRs): how to use this charter").unwrap();
    if input.decision_records.enabled {
        writeln!(
            out,
            "- Record major design decisions in {} using {} files.",
            normalize_charter_free_text(&input.decision_records.path),
            normalize_charter_free_text(&input.decision_records.format)
        )
        .unwrap();
        writeln!(
            out,
            "- Use ADRs when a change alters the project baseline, crosses a listed red line, or introduces a lasting domain override."
        )
        .unwrap();
    } else {
        writeln!(out, "- ADRs are not mandatory by default for this project.").unwrap();
        writeln!(
            out,
            "- Capture any material exception or posture change in {} instead.",
            normalize_charter_free_text(&input.exceptions.record_location)
        )
        .unwrap();
    }
    writeln!(out).unwrap();
    writeln!(out, "## Review & updates").unwrap();
    writeln!(
        out,
        "- Review this charter on a {} cadence.",
        normalize_charter_free_text(&input.debt_tracking.review_cadence)
    )
    .unwrap();
    writeln!(
        out,
        "- Update it when the project classification, domains, runtime environments, or production reality change."
    )
    .unwrap();
    writeln!(
        out,
        "- Re-run impacted plans when any update changes a default level, a red line, or an exception process."
    )
    .unwrap();

    let markdown = out.trim_end().to_string();
    validate_required_heading_order(&markdown);
    Ok(markdown)
}

pub fn author_charter(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    preflight_author_charter(repo_root)?;
    let _lock = acquire_charter_authoring_lock(repo_root)?;
    preflight_author_charter(repo_root)?;

    let markdown = synthesize_charter_markdown(repo_root, input)?;
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

fn collect_render_safety_issues(input: &CharterStructuredInput, issues: &mut Vec<String>) {
    require_render_safe_text("project.name", &input.project.name, issues);
    require_render_safe_text(
        "project.constraints.deadline",
        &input.project.constraints.deadline,
        issues,
    );
    require_render_safe_text(
        "project.constraints.budget",
        &input.project.constraints.budget,
        issues,
    );
    require_render_safe_text(
        "project.constraints.experience_notes",
        &input.project.constraints.experience_notes,
        issues,
    );
    require_render_safe_list(
        "project.constraints.must_use_tech",
        &input.project.constraints.must_use_tech,
        issues,
    );
    require_render_safe_text(
        "project.operational_reality.prod_users_or_data",
        &input.project.operational_reality.prod_users_or_data,
        issues,
    );
    require_render_safe_list(
        "project.operational_reality.external_contracts_to_preserve",
        &input
            .project
            .operational_reality
            .external_contracts_to_preserve,
        issues,
    );
    require_render_safe_text(
        "project.operational_reality.uptime_expectations",
        &input.project.operational_reality.uptime_expectations,
        issues,
    );
    require_render_safe_list(
        "posture.baseline_rationale",
        &input.posture.baseline_rationale,
        issues,
    );

    for (index, domain) in input.domains.iter().enumerate() {
        let prefix = format!("domains[{index}]");
        require_render_safe_text(&format!("{prefix}.name"), &domain.name, issues);
        require_render_safe_text(
            &format!("{prefix}.blast_radius"),
            &domain.blast_radius,
            issues,
        );
        require_render_safe_list(&format!("{prefix}.touches"), &domain.touches, issues);
        require_render_safe_list(
            &format!("{prefix}.constraints"),
            &domain.constraints,
            issues,
        );
    }

    for (index, dimension) in input.dimensions.iter().enumerate() {
        let prefix = format!("dimensions[{index}]");
        require_render_safe_text(
            &format!("{prefix}.default_stance"),
            &dimension.default_stance,
            issues,
        );
        require_render_safe_list(
            &format!("{prefix}.raise_the_bar_triggers"),
            &dimension.raise_the_bar_triggers,
            issues,
        );
        require_render_safe_list(
            &format!("{prefix}.allowed_shortcuts"),
            &dimension.allowed_shortcuts,
            issues,
        );
        require_render_safe_list(&format!("{prefix}.red_lines"), &dimension.red_lines, issues);
        require_render_safe_list(
            &format!("{prefix}.domain_overrides"),
            &dimension.domain_overrides,
            issues,
        );
    }

    require_render_safe_list("exceptions.approvers", &input.exceptions.approvers, issues);
    require_render_safe_text(
        "exceptions.record_location",
        &input.exceptions.record_location,
        issues,
    );
    require_render_safe_list(
        "exceptions.minimum_fields",
        &input.exceptions.minimum_fields,
        issues,
    );
    require_render_safe_text("debt_tracking.system", &input.debt_tracking.system, issues);
    require_render_safe_list("debt_tracking.labels", &input.debt_tracking.labels, issues);
    require_render_safe_text(
        "debt_tracking.review_cadence",
        &input.debt_tracking.review_cadence,
        issues,
    );
    require_render_safe_text(
        "decision_records.path",
        &input.decision_records.path,
        issues,
    );
    require_render_safe_text(
        "decision_records.format",
        &input.decision_records.format,
        issues,
    );
}

fn require_render_safe_list(field: &str, values: &[String], issues: &mut Vec<String>) {
    for (index, value) in values.iter().enumerate() {
        require_render_safe_text(&format!("{field}[{index}]"), value, issues);
    }
}

fn require_render_safe_text(field: &str, value: &str, issues: &mut Vec<String>) {
    if value.trim().is_empty() {
        return;
    }

    let normalized = normalize_charter_free_text(value);
    let trimmed = normalized.trim_start();
    let has_unsafe_prefix = trimmed.starts_with('#')
        || trimmed.starts_with('>')
        || trimmed.starts_with("```")
        || trimmed.starts_with("~~~")
        || trimmed.starts_with("<!--")
        || trimmed.starts_with("- ")
        || trimmed.starts_with("* ");
    let has_unsafe_token = normalized.contains("```")
        || normalized.contains("~~~")
        || normalized.contains("<!--")
        || normalized.contains("-->");

    if has_unsafe_prefix || has_unsafe_token {
        issues.push(format!(
            "{field} must not include markdown control syntax such as headings, blockquotes, fences, comments, or list markers"
        ));
    }
}

fn push_bullets(out: &mut String, values: &[String]) {
    for value in values {
        writeln!(out, "- {}", normalize_charter_free_text(value)).unwrap();
    }
}

fn render_inline_text_or_default(value: &str, default: &str) -> String {
    let normalized = normalize_charter_free_text(value);
    if normalized.is_empty() {
        default.to_string()
    } else {
        normalized
    }
}

fn render_inline_list_or_default(values: &[String], default: &str) -> String {
    if values.is_empty() {
        return default.to_string();
    }

    let normalized = values
        .iter()
        .map(|value| normalize_charter_free_text(value))
        .filter(|value| !value.is_empty())
        .collect::<Vec<_>>();
    if normalized.is_empty() {
        default.to_string()
    } else {
        normalized.join(", ")
    }
}

fn join_display<'a>(values: impl Iterator<Item = &'a str>) -> String {
    values.collect::<Vec<_>>().join(", ")
}

fn escape_table_cell(value: &str) -> String {
    value.replace('\\', "\\\\").replace('|', "\\|")
}

fn dimension_lookup(
    input: &CharterStructuredInput,
    name: CharterDimensionName,
) -> &CharterDimensionInput {
    input
        .dimensions
        .iter()
        .find(|dimension| dimension.name == name)
        .expect("validated charter input includes all dimensions")
}

fn effective_dimension_level(dimension: &CharterDimensionInput, baseline_level: u8) -> u8 {
    dimension.level.unwrap_or(baseline_level)
}

fn collect_cross_cutting_red_lines(input: &CharterStructuredInput) -> Vec<String> {
    let mut seen = BTreeSet::new();
    let mut lines = Vec::new();

    for name in CharterDimensionName::all() {
        for red_line in &dimension_lookup(input, *name).red_lines {
            let normalized = normalize_charter_free_text(red_line);
            if seen.insert(normalized.clone()) {
                lines.push(normalized);
            }
        }
    }

    lines
}

fn validate_required_heading_order(markdown: &str) {
    if let Err(summary) = validate_required_heading_order_result(markdown) {
        panic!("rendered charter heading validation failed: {summary}");
    }
}

fn validate_required_heading_order_result(markdown: &str) -> Result<(), String> {
    let heading_lines = markdown
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim_end();
            REQUIRED_CHARTER_TOP_LEVEL_HEADINGS
                .contains(&trimmed)
                .then_some((index, trimmed))
        })
        .collect::<Vec<_>>();

    let mut previous = 0usize;
    for heading in REQUIRED_CHARTER_TOP_LEVEL_HEADINGS {
        let positions = heading_lines
            .iter()
            .filter_map(|(index, line)| (*line == heading).then_some(*index))
            .collect::<Vec<_>>();
        if positions.is_empty() {
            return Err(format!("missing required heading `{heading}`"));
        }
        if positions.len() != 1 {
            return Err(format!(
                "required heading `{heading}` must appear exactly once"
            ));
        }
        let position = positions[0];
        if position < previous {
            return Err(format!("required heading `{heading}` is out of order"));
        }
        previous = position;
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

fn synthesis_refusal(summary: impl Into<String>) -> AuthorCharterRefusal {
    AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::SynthesisFailed,
        summary: summary.into(),
        broken_subject: "final charter synthesis".to_string(),
        next_safe_action:
            "repair the charter synthesis runtime or prompt inputs and retry `system author charter`"
                .to_string(),
    }
}

fn synthesize_charter_markdown(
    repo_root: &Path,
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    validate_charter_structured_input(input)?;

    let prompt = build_charter_synthesis_prompt(input)?;
    let output_path = synthesize_output_path();
    let codex_bin = std::env::var(AUTHOR_CHARTER_CODEX_BIN_ENV_VAR)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "codex".to_string());
    let codex_model = std::env::var(AUTHOR_CHARTER_CODEX_MODEL_ENV_VAR)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty());

    let mut command = Command::new(&codex_bin);
    command
        .current_dir(repo_root)
        .arg("exec")
        .arg("--skip-git-repo-check")
        .arg("--sandbox")
        .arg("read-only")
        .arg("--color")
        .arg("never");
    if let Some(model) = codex_model.as_deref() {
        command.arg("--model").arg(model);
    }
    command
        .arg("--output-last-message")
        .arg(&output_path)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn().map_err(|err| {
        synthesis_refusal(format!(
            "failed to start `codex exec` for charter synthesis: {err}"
        ))
    })?;

    let prompt_write_result = {
        let stdin = child.stdin.as_mut().ok_or_else(|| {
            synthesis_refusal("failed to open stdin for `codex exec` charter synthesis")
        })?;
        stdin.write_all(prompt.as_bytes())
    };
    if let Err(err) = prompt_write_result {
        let _ = child.kill();
        let _ = child.wait();
        let _ = std::fs::remove_file(&output_path);
        return Err(synthesis_refusal(format!(
            "failed to stream charter synthesis prompt into `codex exec`: {err}"
        )));
    }

    let output = child.wait_with_output().map_err(|err| {
        synthesis_refusal(format!(
            "failed while waiting for `codex exec` charter synthesis: {err}"
        ))
    })?;

    if !output.status.success() {
        let command_output = summarize_process_output(&output.stdout, &output.stderr);
        let _ = std::fs::remove_file(&output_path);
        return Err(synthesis_refusal(format!(
            "`codex exec` charter synthesis exited with {}{}",
            render_exit_status(output.status.code()),
            command_output
        )));
    }

    let markdown = std::fs::read_to_string(&output_path).map_err(|err| {
        synthesis_refusal(format!(
            "failed to read synthesized charter markdown from {}: {err}",
            output_path.display()
        ))
    })?;
    let _ = std::fs::remove_file(&output_path);

    let normalized = markdown.trim().to_string();
    validate_synthesized_charter_markdown(&normalized)?;
    Ok(normalized)
}

fn build_charter_synthesis_prompt(
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    let normalized_input = normalized_charter_structured_input(input);
    let structured_yaml = serde_yaml_bw::to_string(&normalized_input).map_err(|err| {
        synthesis_refusal(format!(
            "failed to serialize normalized charter inputs for synthesis: {err}"
        ))
    })?;
    let sanitized_template = sanitize_charter_template(CHARTER_TEMPLATE_MARKDOWN);
    if let Some(leaked_line) = find_charter_template_scaffold_line(&sanitized_template) {
        return Err(synthesis_refusal(format!(
            "sanitized charter template still contains author-facing scaffold: `{}`",
            leaked_line.trim()
        )));
    }

    let mut prompt = String::new();
    writeln!(prompt, "# Repo-Owned Charter Authoring Method").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "{AUTHORING_METHOD_MARKDOWN}").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "# Charter Synthesis Directive").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "{CHARTER_SYNTHESIZE_DIRECTIVE_MARKDOWN}").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "# Sanitized charter.md.tmpl").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "```md").unwrap();
    writeln!(prompt, "{sanitized_template}").unwrap();
    writeln!(prompt, "```").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "# CHARTER_INPUTS.yaml").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "```yaml").unwrap();
    write!(prompt, "{structured_yaml}").unwrap();
    if !structured_yaml.ends_with('\n') {
        writeln!(prompt).unwrap();
    }
    writeln!(prompt, "```").unwrap();
    writeln!(prompt).unwrap();
    writeln!(
        prompt,
        "Return only the final `CHARTER.md` markdown and preserve the template heading order exactly once."
    )
    .unwrap();

    Ok(prompt)
}

fn normalized_charter_structured_input(input: &CharterStructuredInput) -> CharterStructuredInput {
    let mut normalized = input.clone();

    normalized.project.name = normalize_charter_free_text(&normalized.project.name);
    normalized.project.constraints.deadline =
        normalize_charter_free_text(&normalized.project.constraints.deadline);
    normalized.project.constraints.budget =
        normalize_charter_free_text(&normalized.project.constraints.budget);
    normalized.project.constraints.experience_notes =
        normalize_charter_free_text(&normalized.project.constraints.experience_notes);
    normalize_string_list(&mut normalized.project.constraints.must_use_tech);

    normalized.project.operational_reality.prod_users_or_data =
        normalize_charter_free_text(&normalized.project.operational_reality.prod_users_or_data);
    normalize_string_list(
        &mut normalized
            .project
            .operational_reality
            .external_contracts_to_preserve,
    );
    normalized.project.operational_reality.uptime_expectations =
        normalize_charter_free_text(&normalized.project.operational_reality.uptime_expectations);

    normalized.posture.rubric_scale = normalize_charter_free_text(&normalized.posture.rubric_scale);
    normalize_string_list(&mut normalized.posture.baseline_rationale);

    for domain in &mut normalized.domains {
        domain.name = normalize_charter_free_text(&domain.name);
        domain.blast_radius = normalize_charter_free_text(&domain.blast_radius);
        normalize_string_list(&mut domain.touches);
        normalize_string_list(&mut domain.constraints);
    }

    for dimension in &mut normalized.dimensions {
        dimension.default_stance = normalize_charter_free_text(&dimension.default_stance);
        normalize_string_list(&mut dimension.raise_the_bar_triggers);
        normalize_string_list(&mut dimension.allowed_shortcuts);
        normalize_string_list(&mut dimension.red_lines);
        normalize_string_list(&mut dimension.domain_overrides);
    }

    normalize_string_list(&mut normalized.exceptions.approvers);
    normalized.exceptions.record_location =
        normalize_charter_free_text(&normalized.exceptions.record_location);
    normalize_string_list(&mut normalized.exceptions.minimum_fields);

    normalized.debt_tracking.system = normalize_charter_free_text(&normalized.debt_tracking.system);
    normalize_string_list(&mut normalized.debt_tracking.labels);
    normalized.debt_tracking.review_cadence =
        normalize_charter_free_text(&normalized.debt_tracking.review_cadence);

    normalized.decision_records.path =
        normalize_charter_free_text(&normalized.decision_records.path);
    normalized.decision_records.format =
        normalize_charter_free_text(&normalized.decision_records.format);

    normalized
}

fn normalize_string_list(values: &mut Vec<String>) {
    for value in values {
        *value = normalize_charter_free_text(value);
    }
}

fn normalize_charter_template_scaffold_line(line: &str) -> String {
    line.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn is_charter_template_scaffold_line(line: &str) -> bool {
    let normalized = normalize_charter_template_scaffold_line(line);
    if normalized.is_empty() {
        return false;
    }

    CHARTER_TEMPLATE_SCAFFOLD_MARKERS
        .iter()
        .any(|marker| normalized.starts_with(marker))
        || CHARTER_TEMPLATE_SCAFFOLD_PREFIX_MARKERS
            .iter()
            .any(|marker| normalized.starts_with(marker))
}

fn find_charter_template_scaffold_line(document: &str) -> Option<&str> {
    document
        .lines()
        .find(|line| is_charter_template_scaffold_line(line))
}

fn sanitize_charter_template(template: &str) -> String {
    let mut sanitized = String::with_capacity(template.len());
    let mut remaining = template;
    let mut in_comment = false;

    while !remaining.is_empty() {
        if in_comment {
            if let Some(index) = remaining.find("-->") {
                remaining = &remaining[index + 3..];
                in_comment = false;
            } else {
                break;
            }
            continue;
        }

        if let Some(index) = remaining.find("<!--") {
            sanitized.push_str(&remaining[..index]);
            remaining = &remaining[index + 4..];
            in_comment = true;
        } else {
            sanitized.push_str(remaining);
            break;
        }
    }

    let mut collapsed = String::new();
    let mut blank_run = 0usize;
    for line in sanitized.lines() {
        let trimmed_end = line.trim_end();
        if is_charter_template_scaffold_line(trimmed_end) {
            continue;
        }
        if trimmed_end.is_empty() {
            blank_run += 1;
            if blank_run > 1 {
                continue;
            }
            collapsed.push('\n');
            continue;
        }
        blank_run = 0;
        collapsed.push_str(trimmed_end);
        collapsed.push('\n');
    }

    collapsed.trim().to_string()
}

fn synthesize_output_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "system-author-charter-{}-{timestamp}.md",
        std::process::id()
    ))
}

fn validate_synthesized_charter_markdown(markdown: &str) -> Result<(), AuthorCharterRefusal> {
    if markdown.trim().is_empty() {
        return Err(synthesis_refusal("synthesized charter markdown was empty"));
    }
    if !markdown.starts_with("# Engineering Charter — ") {
        return Err(synthesis_refusal(
            "synthesized charter markdown must start with `# Engineering Charter — `",
        ));
    }
    if markdown.contains("{{") || markdown.contains("}}") {
        return Err(synthesis_refusal(
            "synthesized charter markdown contains unresolved template placeholders",
        ));
    }
    if let Some(leaked_line) = find_charter_template_scaffold_line(markdown) {
        return Err(synthesis_refusal(format!(
            "synthesized charter markdown contains leaked author-facing scaffold: `{}`",
            leaked_line.trim()
        )));
    }
    if let Err(summary) = validate_required_heading_order_result(markdown) {
        return Err(synthesis_refusal(format!(
            "synthesized charter markdown failed heading validation: {summary}"
        )));
    }
    Ok(())
}

fn summarize_process_output(stdout: &[u8], stderr: &[u8]) -> String {
    let stdout = String::from_utf8_lossy(stdout);
    let stderr = String::from_utf8_lossy(stderr);

    let stderr = summarize_stderr_for_refusal(stderr.trim());
    if !stderr.is_empty() {
        return format!("; stderr: {stderr}");
    }

    let stdout = summarize_stream_tail(stdout.trim());
    if stdout.is_empty() {
        String::new()
    } else {
        format!("; stdout: {stdout}")
    }
}

fn truncate_for_summary(value: &str) -> String {
    if value.chars().count() <= PROCESS_SUMMARY_CHAR_LIMIT {
        value.to_string()
    } else {
        let tail = value
            .chars()
            .rev()
            .take(PROCESS_SUMMARY_CHAR_LIMIT)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<String>();
        format!("...{tail}")
    }
}

fn summarize_stderr_for_refusal(stderr: &str) -> String {
    let high_signal = collect_stream_summary_lines(stderr, true);
    if !high_signal.is_empty() {
        return truncate_for_summary(&high_signal.join(" | "));
    }

    summarize_stream_tail(stderr)
}

fn summarize_stream_tail(stream: &str) -> String {
    let lines = collect_stream_summary_lines(stream, false);
    if lines.is_empty() {
        String::new()
    } else {
        truncate_for_summary(&lines.join(" | "))
    }
}

fn collect_stream_summary_lines(stream: &str, prefer_high_signal: bool) -> Vec<String> {
    let mut selected = Vec::new();
    let mut seen = Vec::new();

    for raw_line in stream.lines().rev() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }

        let normalized = normalize_process_summary_line(line);
        if normalized.is_empty() || seen.iter().any(|existing| existing == &normalized) {
            continue;
        }
        if prefer_high_signal && !is_high_signal_process_summary_line(&normalized) {
            continue;
        }

        seen.push(normalized);
        selected.push(line.to_string());
        if selected.len() == PROCESS_SUMMARY_LINE_LIMIT {
            break;
        }
    }

    selected.reverse();
    selected
}

fn normalize_process_summary_line(line: &str) -> String {
    line.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_ascii_lowercase()
}

fn is_high_signal_process_summary_line(normalized_line: &str) -> bool {
    PROCESS_SUMMARY_HIGH_SIGNAL_MARKERS
        .iter()
        .any(|marker| normalized_line.contains(marker))
}

fn render_exit_status(code: Option<i32>) -> String {
    match code {
        Some(code) => format!("exit code {code}"),
        None => "signal termination".to_string(),
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
