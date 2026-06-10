use super::{
    acquire_authoring_lock, baseline_authoring_eligibility, canonical_artifact_identity,
    charter_core::{
        compiler_owned_charter_markdown as compiler_owned_charter_markdown_core,
        find_charter_template_scaffold_line, normalized_charter_structured_input,
        parse_charter_structured_input_yaml as parse_charter_structured_input_yaml_core,
        render_charter_markdown as render_charter_markdown_core, sanitize_charter_template,
        validate_charter_structured_input as validate_charter_structured_input_core,
        validate_required_heading_order_result, CharterCoreError, CharterCoreErrorKind,
    },
    format_repo_mutation_error, format_repo_write_path_error, render_exit_status,
    summarize_process_output,
    template_library::resolve_template_library,
    template_library::TemplateLibraryRequest,
    template_library::TemplateLibraryResolveRequest,
    template_library::TemplateLibrarySelection,
    validate_canonical_write_target, validate_system_root_for_authoring, AuthoringLockError,
    BaselineAuthoringEligibility, SystemRootAuthoringError,
};
use crate::canonical_artifacts::{CanonicalArtifactKind, CanonicalArtifacts};
use crate::layout::{RepoLayoutRoot, CANONICAL_CHARTER_RELATIVE_PATH};
use crate::repo_file_access::write_repo_relative_bytes;
use std::fmt::Write as _;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub use super::charter_core::{
    is_unusably_vague_charter_text, normalize_charter_free_text, validate_charter_markdown,
    CharterAudience, CharterBackwardCompatibility, CharterDebtTrackingInput,
    CharterDecisionRecordsInput, CharterDefaultImplicationsInput, CharterDeprecationPolicy,
    CharterDimensionInput, CharterDimensionName, CharterDomainInput, CharterExceptionsInput,
    CharterExpectedLifetime, CharterObservabilityThreshold, CharterOperationalRealityInput,
    CharterPostureInput, CharterProjectClassification, CharterProjectConstraintsInput,
    CharterProjectInput, CharterRequiredness, CharterRolloutControls, CharterRuntimeEnvironment,
    CharterStructuredInput, CharterSurface, DEFAULT_EXCEPTION_RECORD_LOCATION,
};

pub const CANONICAL_CHARTER_REPO_PATH: &str = CANONICAL_CHARTER_RELATIVE_PATH;
// Tests can override the codex binary path without changing the shipped CLI surface.
const AUTHOR_CHARTER_CODEX_BIN_ENV_VAR: &str = "HANDBOOK_AUTHOR_CHARTER_CODEX_BIN";
const AUTHOR_CHARTER_CODEX_MODEL_ENV_VAR: &str = "HANDBOOK_AUTHOR_CHARTER_CODEX_MODEL";
// Command paths:
// `handbook author charter --from-inputs <path|->`
// -> normalized `CharterStructuredInput`
// -> compiler-owned deterministic render
// -> guarded write to `.handbook/charter/CHARTER.md`
//
// `handbook author charter`
// -> normalized `CharterStructuredInput`
// -> Codex-backed guided synthesis
// -> guarded write to `.handbook/charter/CHARTER.md`

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

pub fn parse_charter_structured_input_yaml(
    yaml: &str,
) -> Result<CharterStructuredInput, AuthorCharterRefusal> {
    parse_charter_structured_input_yaml_core(yaml).map_err(map_charter_core_error)
}

pub fn validate_charter_structured_input(
    input: &CharterStructuredInput,
) -> Result<(), AuthorCharterRefusal> {
    validate_charter_structured_input_core(input).map_err(map_charter_core_error)
}

pub fn render_charter_markdown(
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    render_charter_markdown_core(input).map_err(map_charter_core_error)
}

fn compiler_owned_charter_markdown(
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    compiler_owned_charter_markdown_core(input).map_err(map_charter_core_error)
}

pub fn preflight_author_charter_from_input(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<(), AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    compiler_owned_charter_markdown(input)?;
    preflight_author_charter(repo_root)?;
    Ok(())
}

fn map_charter_core_error(err: CharterCoreError) -> AuthorCharterRefusal {
    match err.kind {
        CharterCoreErrorKind::MalformedStructuredInput => AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::MalformedStructuredInput,
            summary: err.summary,
            broken_subject: "structured charter input".to_string(),
            next_safe_action:
                "repair the structured charter input and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        },
        CharterCoreErrorKind::IncompleteStructuredInput => AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::IncompleteStructuredInput,
            summary: err.summary,
            broken_subject: "structured charter input".to_string(),
            next_safe_action:
                "repair the structured charter input and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        },
        CharterCoreErrorKind::DeterministicRenderFailed => AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::SynthesisFailed,
            summary: err.summary,
            broken_subject: "final charter render".to_string(),
            next_safe_action:
                "repair the structured charter input or compiler-owned charter render path and retry `handbook author charter --from-inputs <path|->`"
                    .to_string(),
        },
    }
}

pub fn author_charter(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    let charter_layout = RepoLayoutRoot::new(repo_root).authoring().charter();
    preflight_author_charter_from_input(repo_root, input)?;
    let _lock =
        acquire_authoring_lock(repo_root, charter_layout.lock_path().as_str()).map_err(|err| {
            match err {
                AuthoringLockError::WritePath(path_err) => AuthorCharterRefusal {
                    kind: AuthorCharterRefusalKind::MutationRefused,
                    summary: format_repo_write_path_error(
                        charter_layout.lock_relative_path(),
                        path_err,
                    ),
                    broken_subject: "charter authoring lock".to_string(),
                    next_safe_action:
                        "repair the blocked charter authoring lock path and retry `handbook author charter`"
                            .to_string(),
                },
                AuthoringLockError::Io { lock_path, source } => AuthorCharterRefusal {
                    kind: AuthorCharterRefusalKind::MutationRefused,
                    summary: format!(
                        "failed to acquire exclusive charter authoring lock at {}: {source}",
                        lock_path.display()
                    ),
                    broken_subject: "charter authoring lock".to_string(),
                    next_safe_action:
                        "wait for any in-progress `handbook author charter` run to finish or repair the lock path, then retry `handbook author charter`"
                            .to_string(),
                },
            }
        })?;
    preflight_author_charter_from_input(repo_root, input)?;

    let markdown = compiler_owned_charter_markdown(input)?;
    write_repo_relative_bytes(
        repo_root,
        charter_layout.canonical_target().as_str(),
        markdown.as_bytes(),
    )
    .map_err(|err| AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::MutationRefused,
        summary: format_repo_mutation_error(charter_layout.canonical_target_relative(), err),
        broken_subject: "canonical charter write target".to_string(),
        next_safe_action:
            "repair the blocked canonical charter path and retry `handbook author charter`"
                .to_string(),
    })?;

    Ok(AuthorCharterResult {
        canonical_repo_relative_path: charter_layout.canonical_target_relative(),
        bytes_written: markdown.len(),
    })
}

pub fn author_charter_guided(
    repo_root: impl AsRef<Path>,
    input: &CharterStructuredInput,
) -> Result<AuthorCharterResult, AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    let charter_layout = RepoLayoutRoot::new(repo_root).authoring().charter();
    validate_charter_structured_input(input)?;
    preflight_author_charter(repo_root)?;
    let _lock =
        acquire_authoring_lock(repo_root, charter_layout.lock_path().as_str()).map_err(|err| {
            match err {
                AuthoringLockError::WritePath(path_err) => AuthorCharterRefusal {
                    kind: AuthorCharterRefusalKind::MutationRefused,
                    summary: format_repo_write_path_error(
                        charter_layout.lock_relative_path(),
                        path_err,
                    ),
                    broken_subject: "charter authoring lock".to_string(),
                    next_safe_action:
                        "repair the blocked charter authoring lock path and retry `handbook author charter`"
                            .to_string(),
                },
                AuthoringLockError::Io { lock_path, source } => AuthorCharterRefusal {
                    kind: AuthorCharterRefusalKind::MutationRefused,
                    summary: format!(
                        "failed to acquire exclusive charter authoring lock at {}: {source}",
                        lock_path.display()
                    ),
                    broken_subject: "charter authoring lock".to_string(),
                    next_safe_action:
                        "wait for any in-progress `handbook author charter` run to finish or repair the lock path, then retry `handbook author charter`"
                            .to_string(),
                },
            }
        })?;
    validate_charter_structured_input(input)?;
    preflight_author_charter(repo_root)?;

    let markdown = synthesize_charter_markdown(repo_root, input)?;
    write_repo_relative_bytes(
        repo_root,
        charter_layout.canonical_target().as_str(),
        markdown.as_bytes(),
    )
    .map_err(|err| AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::MutationRefused,
        summary: format_repo_mutation_error(charter_layout.canonical_target_relative(), err),
        broken_subject: "canonical charter write target".to_string(),
        next_safe_action:
            "repair the blocked canonical charter path and retry `handbook author charter`"
                .to_string(),
    })?;

    Ok(AuthorCharterResult {
        canonical_repo_relative_path: charter_layout.canonical_target_relative(),
        bytes_written: markdown.len(),
    })
}

pub fn preflight_author_charter(repo_root: impl AsRef<Path>) -> Result<(), AuthorCharterRefusal> {
    let repo_root = repo_root.as_ref();
    let artifacts = CanonicalArtifacts::load(repo_root).map_err(|err| AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::InvalidSystemRoot,
        summary: format!("failed to inspect canonical `.handbook` root: {err}"),
        broken_subject: "canonical `.handbook` root".to_string(),
        next_safe_action: "repair the canonical `.handbook` root and rerun `handbook setup`"
            .to_string(),
    })?;
    validate_authoring_preconditions(repo_root, &artifacts)?;
    Ok(())
}

fn validate_authoring_preconditions(
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
) -> Result<(), AuthorCharterRefusal> {
    let charter_layout = RepoLayoutRoot::new(repo_root).authoring().charter();
    match validate_system_root_for_authoring(artifacts) {
        Ok(()) => {}
        Err(SystemRootAuthoringError::Missing) => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::MissingSystemRoot,
                summary:
                    "canonical `.handbook` root is missing; charter authoring requires setup first"
                        .to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "run `handbook setup`".to_string(),
            })
        }
        Err(SystemRootAuthoringError::NotDir) => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root exists but is not a directory".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action:
                    "repair the canonical `.handbook` root and rerun `handbook setup`".to_string(),
            })
        }
        Err(SystemRootAuthoringError::SymlinkNotAllowed) => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::InvalidSystemRoot,
                summary: "canonical `.handbook` root cannot be a symlink".to_string(),
                broken_subject: "canonical `.handbook` root".to_string(),
                next_safe_action: "remove the `.handbook` symlink and rerun `handbook setup`"
                    .to_string(),
            })
        }
    }

    let charter = canonical_artifact_identity(artifacts, CanonicalArtifactKind::Charter);
    if charter.kind != CanonicalArtifactKind::Charter {
        return Err(AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::ExistingCanonicalTruth,
            summary: "unexpected canonical artifact identity for charter authoring".to_string(),
            broken_subject: "canonical charter truth".to_string(),
            next_safe_action:
                "inspect canonical artifact metadata and retry `handbook author charter`"
                    .to_string(),
        });
    }

    match baseline_authoring_eligibility(artifacts, CanonicalArtifactKind::Charter) {
        BaselineAuthoringEligibility::Authorable => {}
        BaselineAuthoringEligibility::ExistingValidCanonicalTruth => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::ExistingCanonicalTruth,
                summary:
                    "canonical charter truth already exists as valid non-starter truth; `handbook author charter` refuses to overwrite authored canonical truth"
                        .to_string(),
                broken_subject: charter_layout.canonical_target_relative().to_string(),
                next_safe_action: format!(
                    "inspect `{}` instead of rerunning `handbook author charter`",
                    charter_layout.canonical_target_relative()
                ),
            });
        }
        BaselineAuthoringEligibility::RequiresSetupRefresh => {
            return Err(AuthorCharterRefusal {
                kind: AuthorCharterRefusalKind::MutationRefused,
                summary:
                    "canonical charter truth is unreadable or path-invalid; repair it with `handbook setup refresh` before rerunning `handbook author charter`"
                        .to_string(),
                broken_subject: charter_layout.canonical_target_relative().to_string(),
                next_safe_action: "run `handbook setup refresh`".to_string(),
            });
        }
    }

    validate_canonical_write_target(repo_root, charter_layout.canonical_target().as_str())
        .map_err(|err| AuthorCharterRefusal {
            kind: AuthorCharterRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(charter_layout.canonical_target_relative(), err),
            broken_subject: "canonical charter write target".to_string(),
            next_safe_action:
                "repair the blocked canonical charter path and retry `handbook author charter`"
                    .to_string(),
        })?;

    Ok(())
}

fn synthesis_refusal(summary: impl Into<String>) -> AuthorCharterRefusal {
    AuthorCharterRefusal {
        kind: AuthorCharterRefusalKind::SynthesisFailed,
        summary: summary.into(),
        broken_subject: "final charter synthesis".to_string(),
        next_safe_action:
            "repair the charter synthesis runtime or prompt inputs and retry `handbook author charter`"
                .to_string(),
    }
}

fn synthesize_charter_markdown(
    repo_root: &Path,
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    validate_charter_structured_input(input)?;

    let prompt = build_charter_synthesis_prompt(repo_root, input)?;
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
    validate_synthesized_charter_markdown(&normalized, input)?;
    Ok(normalized)
}

fn build_charter_synthesis_prompt(
    repo_root: &Path,
    input: &CharterStructuredInput,
) -> Result<String, AuthorCharterRefusal> {
    let selection = match resolve_template_library(
        repo_root,
        &TemplateLibraryResolveRequest::new(TemplateLibraryRequest::CharterAuthoring),
    )
    .map_err(|err| {
        synthesis_refusal(format!("failed to resolve charter authoring assets: {err}"))
    })? {
        TemplateLibrarySelection::Charter(selection) => selection,
        TemplateLibrarySelection::EnvironmentInventory(_) => {
            unreachable!("charter authoring must resolve charter template-library assets")
        }
    };
    let normalized_input = normalized_charter_structured_input(input);
    let structured_yaml = serde_yaml_bw::to_string(&normalized_input).map_err(|err| {
        synthesis_refusal(format!(
            "failed to serialize normalized charter inputs for synthesis: {err}"
        ))
    })?;
    let sanitized_template = sanitize_charter_template(selection.template().contents());
    if let Some(leaked_line) = find_charter_template_scaffold_line(&sanitized_template) {
        return Err(synthesis_refusal(format!(
            "sanitized charter template still contains author-facing scaffold: `{}`",
            leaked_line.trim()
        )));
    }

    let mut prompt = String::new();
    writeln!(prompt, "# Repo-Owned Charter Authoring Method").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "{}", selection.authoring_method().contents()).unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "# Charter Synthesis Directive").unwrap();
    writeln!(prompt).unwrap();
    writeln!(prompt, "{}", selection.synthesize_directive().contents()).unwrap();
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
        "## Exact structured values that must appear verbatim"
    )
    .unwrap();
    writeln!(prompt).unwrap();
    writeln!(
        prompt,
        "- In `## Exceptions / overrides process`, render the exact `exceptions.record_location` string from `CHARTER_INPUTS.yaml`: `{}`",
        normalized_input.exceptions.record_location
    )
    .unwrap();
    writeln!(prompt).unwrap();
    writeln!(
        prompt,
        "Return only the final `CHARTER.md` markdown and preserve the template heading order exactly once."
    )
    .unwrap();

    Ok(prompt)
}

fn synthesize_output_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "handbook-author-charter-{}-{timestamp}.md",
        std::process::id()
    ))
}

fn validate_synthesized_charter_markdown(
    markdown: &str,
    input: &CharterStructuredInput,
) -> Result<(), AuthorCharterRefusal> {
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
    let expected_exception_record_location =
        normalize_charter_free_text(&input.exceptions.record_location);
    if !markdown.contains(&expected_exception_record_location) {
        return Err(synthesis_refusal(format!(
            "synthesized charter markdown must include the exact exception record location `{expected_exception_record_location}`"
        )));
    }
    Ok(())
}
