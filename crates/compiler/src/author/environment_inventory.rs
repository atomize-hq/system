use crate::baseline_validation::{baseline_artifact_validation, BaselineArtifactVerdict};
use crate::canonical_artifacts::{
    ArtifactPresence, CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
use crate::repo_file_access::{
    resolve_repo_relative_write_path, write_repo_relative_bytes, RepoRelativeMutationError,
    RepoRelativeWritePathError,
};
use std::fs::{File, OpenOptions};
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

pub const CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH: &str =
    ".system/environment_inventory/ENVIRONMENT_INVENTORY.md";

const ENVIRONMENT_INVENTORY_AUTHORING_LOCK_REPO_PATH: &str =
    ".system/state/authoring/environment_inventory.lock";
const AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN_ENV_VAR: &str =
    "SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN";
const AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL_ENV_VAR: &str =
    "SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL";
const ENVIRONMENT_INVENTORY_SYNTHESIZE_DIRECTIVE_MARKDOWN: &str = include_str!(
    "../../../../core/library/environment_inventory/environment_inventory_directive.md"
);
const ENVIRONMENT_INVENTORY_TEMPLATE_MARKDOWN: &str =
    include_str!("../../../../core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl");
const PROCESS_SUMMARY_LINE_LIMIT: usize = 3;
const PROCESS_SUMMARY_CHAR_LIMIT: usize = 600;
const PROCESS_SUMMARY_HIGH_SIGNAL_MARKERS: [&str; 5] = [
    "error:",
    "unauthorized",
    "incorrect api key",
    "missing bearer",
    "failed",
];
const REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS: [&str; 11] = [
    "## What this is",
    "## How to use",
    "## 1) Environment Variables (Inventory)",
    "## 2) External Services / Infrastructure Dependencies",
    "## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)",
    "## 4) Local Development Requirements",
    "## 5) CI Requirements",
    "## 6) Production / Deployment Requirements (even if not live yet)",
    "## 7) Dependency & Tooling Inventory (project-specific)",
    "## 8) Update Contract (non-negotiable)",
    "## 9) Known Unknowns",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorEnvironmentInventoryRefusalKind {
    MissingSystemRoot,
    InvalidSystemRoot,
    MissingRequiredCharter,
    InvalidUpstreamCanonicalTruth,
    ExistingCanonicalTruth,
    MutationRefused,
    SynthesisFailed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorEnvironmentInventoryRefusal {
    pub kind: AuthorEnvironmentInventoryRefusalKind,
    pub summary: String,
    pub broken_subject: String,
    pub next_safe_action: String,
}

impl std::fmt::Display for AuthorEnvironmentInventoryRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for AuthorEnvironmentInventoryRefusal {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorEnvironmentInventoryResult {
    pub canonical_repo_relative_path: &'static str,
    pub bytes_written: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct EnvironmentInventorySynthesisInputs {
    charter_markdown: String,
    project_context_markdown: Option<String>,
}

pub fn preflight_author_environment_inventory(
    repo_root: impl AsRef<Path>,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    let _ = prepare_environment_inventory_authoring_inputs(repo_root.as_ref())?;
    Ok(())
}

pub fn author_environment_inventory(
    repo_root: impl AsRef<Path>,
) -> Result<AuthorEnvironmentInventoryResult, AuthorEnvironmentInventoryRefusal> {
    let repo_root = repo_root.as_ref();
    let _ = prepare_environment_inventory_authoring_inputs(repo_root)?;
    let _lock = acquire_environment_inventory_authoring_lock(repo_root)?;
    let inputs = prepare_environment_inventory_authoring_inputs(repo_root)?;

    let markdown = synthesize_environment_inventory_markdown(repo_root, &inputs)?;
    write_repo_relative_bytes(
        repo_root,
        CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH,
        markdown.as_bytes(),
    )
    .map_err(|err| AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
        summary: format_repo_mutation_error(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH, err),
        broken_subject: "canonical environment inventory write target".to_string(),
        next_safe_action:
            "repair the blocked canonical environment inventory path and retry `system author environment-inventory`"
                .to_string(),
    })?;

    Ok(AuthorEnvironmentInventoryResult {
        canonical_repo_relative_path: CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH,
        bytes_written: markdown.len(),
    })
}

pub fn validate_environment_inventory_markdown(
    markdown: &str,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    let normalized = markdown.trim();
    if normalized.is_empty() {
        return Err(synthesis_refusal(
            "synthesized environment inventory markdown was empty",
        ));
    }
    if !markdown.starts_with("# Environment Inventory") {
        return Err(synthesis_refusal(
            "synthesized environment inventory markdown must start with `# Environment Inventory`",
        ));
    }
    if normalized.contains("{{") || normalized.contains("}}") {
        return Err(synthesis_refusal(
            "synthesized environment inventory markdown contains unresolved template placeholders",
        ));
    }
    if normalized.contains("${repo_root}/ENVIRONMENT_INVENTORY.md")
        || normalized.contains("artifacts/foundation/ENVIRONMENT_INVENTORY.md")
        || normalized.contains("repo/project root")
    {
        return Err(synthesis_refusal(
            "synthesized environment inventory markdown still contains legacy non-canonical path claims",
        ));
    }
    if !normalized.contains("`.system/environment_inventory/ENVIRONMENT_INVENTORY.md`") {
        return Err(synthesis_refusal(
            "synthesized environment inventory markdown must reference `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` as the canonical file",
        ));
    }
    validate_required_heading_order_result(normalized, &REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS)
        .map_err(synthesis_refusal)?;
    Ok(())
}

fn validate_required_heading_order_result(
    markdown: &str,
    required_headings: &[&str],
) -> Result<(), String> {
    let heading_lines = markdown
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            let trimmed = line.trim_end();
            required_headings
                .contains(&trimmed)
                .then_some((index, trimmed))
        })
        .collect::<Vec<_>>();

    let mut previous = 0usize;
    for heading in required_headings {
        let positions = heading_lines
            .iter()
            .filter_map(|(index, line)| (*line == *heading).then_some(*index))
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

fn prepare_environment_inventory_authoring_inputs(
    repo_root: &Path,
) -> Result<EnvironmentInventorySynthesisInputs, AuthorEnvironmentInventoryRefusal> {
    let artifacts =
        CanonicalArtifacts::load(repo_root).map_err(|err| AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
            summary: format!("failed to inspect canonical `.system` root: {err}"),
            broken_subject: "canonical `.system` root".to_string(),
            next_safe_action: "repair the canonical `.system` root and rerun `system setup`"
                .to_string(),
        })?;

    validate_environment_inventory_authoring_preconditions(repo_root, &artifacts)?;

    let charter_markdown = required_charter_markdown(&artifacts)?;
    let project_context_markdown = optional_project_context_markdown(&artifacts)?;

    Ok(EnvironmentInventorySynthesisInputs {
        charter_markdown,
        project_context_markdown,
    })
}

fn validate_environment_inventory_authoring_preconditions(
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    match artifacts.system_root_status {
        SystemRootStatus::Ok => {}
        SystemRootStatus::Missing => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::MissingSystemRoot,
                summary:
                    "canonical `.system` root is missing; environment inventory authoring requires setup first"
                        .to_string(),
                broken_subject: "canonical `.system` root".to_string(),
                next_safe_action: "run `system setup`".to_string(),
            });
        }
        SystemRootStatus::NotDir => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
                summary: "canonical `.system` root exists but is not a directory".to_string(),
                broken_subject: "canonical `.system` root".to_string(),
                next_safe_action: "repair the canonical `.system` root and rerun `system setup`"
                    .to_string(),
            });
        }
        SystemRootStatus::SymlinkNotAllowed => {
            return Err(AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::InvalidSystemRoot,
                summary: "canonical `.system` root cannot be a symlink".to_string(),
                broken_subject: "canonical `.system` root".to_string(),
                next_safe_action: "remove the `.system` symlink and rerun `system setup`"
                    .to_string(),
            });
        }
    }

    let environment_inventory = &artifacts.environment_inventory.identity;
    if environment_inventory.kind != CanonicalArtifactKind::EnvironmentInventory {
        return Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth,
            summary: "unexpected canonical artifact identity for environment inventory authoring"
                .to_string(),
            broken_subject: "canonical environment inventory truth".to_string(),
            next_safe_action:
                "inspect canonical artifact metadata and retry `system author environment-inventory`"
                    .to_string(),
        });
    }

    let existing_non_starter_truth = match environment_inventory.presence {
        ArtifactPresence::PresentNonEmpty => !environment_inventory.matches_setup_starter_template,
        ArtifactPresence::Missing | ArtifactPresence::PresentEmpty => false,
    };
    if existing_non_starter_truth {
        return Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth,
            summary:
                "canonical environment inventory truth already exists; `system author environment-inventory` only replaces missing, empty, or setup-starter content"
                    .to_string(),
            broken_subject: CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH.to_string(),
            next_safe_action: format!(
                "inspect `{}` instead of rerunning `system author environment-inventory`",
                CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH
            ),
        });
    }

    resolve_repo_relative_write_path(repo_root, CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH)
        .map_err(|err| AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
            summary: format_repo_write_path_error(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH, err),
            broken_subject: "canonical environment inventory write target".to_string(),
            next_safe_action:
                "repair the blocked canonical environment inventory path and retry `system author environment-inventory`"
                    .to_string(),
        })?;

    Ok(())
}

fn required_charter_markdown(
    artifacts: &CanonicalArtifacts,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    let validation = baseline_artifact_validation(artifacts, CanonicalArtifactKind::Charter)
        .expect("charter must be part of baseline validation");

    match validation.verdict {
        BaselineArtifactVerdict::Missing => Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter,
            summary:
                "canonical charter truth is missing; environment inventory authoring requires a completed charter first"
                    .to_string(),
            broken_subject: ".system/charter/CHARTER.md".to_string(),
            next_safe_action: "run `system author charter`".to_string(),
        }),
        BaselineArtifactVerdict::Empty => Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter,
            summary:
                "canonical charter truth is empty; environment inventory authoring requires a completed charter first"
                    .to_string(),
            broken_subject: ".system/charter/CHARTER.md".to_string(),
            next_safe_action: "run `system author charter`".to_string(),
        }),
        BaselineArtifactVerdict::StarterOwned => Err(AuthorEnvironmentInventoryRefusal {
            kind: AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter,
            summary:
                "canonical charter truth still contains the shipped starter template; environment inventory authoring requires a completed charter first"
                    .to_string(),
            broken_subject: ".system/charter/CHARTER.md".to_string(),
            next_safe_action: "run `system author charter`".to_string(),
        }),
        BaselineArtifactVerdict::IngestInvalid => Err(invalid_upstream_canonical_truth_refusal(
            ".system/charter/CHARTER.md",
            "canonical charter truth is unreadable or non-canonical; environment inventory authoring requires valid charter truth".to_string(),
            "run `system setup refresh`".to_string(),
        )),
        BaselineArtifactVerdict::SemanticallyInvalid { summary } => Err(
            invalid_upstream_canonical_truth_refusal(
                ".system/charter/CHARTER.md",
                format!("canonical charter truth is invalid: {summary}"),
                "run `system author charter`".to_string(),
            ),
        ),
        BaselineArtifactVerdict::ValidCanonicalTruth { markdown } => Ok(markdown),
    }
}

fn optional_project_context_markdown(
    artifacts: &CanonicalArtifacts,
) -> Result<Option<String>, AuthorEnvironmentInventoryRefusal> {
    let validation = baseline_artifact_validation(artifacts, CanonicalArtifactKind::ProjectContext)
        .expect("project context must be part of baseline validation");

    match validation.verdict {
        BaselineArtifactVerdict::Missing
        | BaselineArtifactVerdict::Empty
        | BaselineArtifactVerdict::StarterOwned => Ok(None),
        BaselineArtifactVerdict::IngestInvalid => Err(invalid_upstream_canonical_truth_refusal(
            ".system/project_context/PROJECT_CONTEXT.md",
            "canonical project context truth is unreadable or non-canonical; repair it or remove it before environment inventory authoring"
                .to_string(),
            "run `system setup refresh`".to_string(),
        )),
        BaselineArtifactVerdict::SemanticallyInvalid { summary } => Err(
            invalid_upstream_canonical_truth_refusal(
                ".system/project_context/PROJECT_CONTEXT.md",
                format!("canonical project context truth is invalid: {summary}"),
                "run `system author project-context`".to_string(),
            ),
        ),
        BaselineArtifactVerdict::ValidCanonicalTruth { markdown } => {
            let trimmed = markdown.trim();
            if trimmed.is_empty() {
                Ok(None)
            } else {
                Ok(Some(trimmed.to_string()))
            }
        }
    }
}

fn synthesize_environment_inventory_markdown(
    repo_root: &Path,
    inputs: &EnvironmentInventorySynthesisInputs,
) -> Result<String, AuthorEnvironmentInventoryRefusal> {
    let prompt = build_environment_inventory_synthesis_prompt(inputs);
    let output_path = synthesize_output_path();
    let codex_bin = std::env::var(AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN_ENV_VAR)
        .ok()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "codex".to_string());
    let codex_model = std::env::var(AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL_ENV_VAR)
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
            "failed to start `codex exec` for environment inventory synthesis: {err}"
        ))
    })?;

    let prompt_write_result = {
        let stdin = child.stdin.as_mut().ok_or_else(|| {
            synthesis_refusal(
                "failed to open stdin for `codex exec` environment inventory synthesis",
            )
        })?;
        stdin.write_all(prompt.as_bytes())
    };
    if let Err(err) = prompt_write_result {
        let _ = child.kill();
        let _ = child.wait();
        let _ = std::fs::remove_file(&output_path);
        return Err(synthesis_refusal(format!(
            "failed to stream environment inventory synthesis prompt into `codex exec`: {err}"
        )));
    }

    let output = child.wait_with_output().map_err(|err| {
        synthesis_refusal(format!(
            "failed while waiting for `codex exec` environment inventory synthesis: {err}"
        ))
    })?;

    if !output.status.success() {
        let command_output = summarize_process_output(&output);
        let _ = std::fs::remove_file(&output_path);
        return Err(synthesis_refusal(format!(
            "`codex exec` environment inventory synthesis exited with {}{}",
            render_exit_status(output.status.code()),
            command_output
        )));
    }

    let markdown = std::fs::read_to_string(&output_path).map_err(|err| {
        synthesis_refusal(format!(
            "failed to read synthesized environment inventory markdown from {}: {err}",
            output_path.display()
        ))
    })?;
    let _ = std::fs::remove_file(&output_path);

    let normalized = markdown.trim().to_string();
    validate_synthesized_environment_inventory_markdown(&normalized, inputs)?;
    Ok(normalized)
}

fn build_environment_inventory_synthesis_prompt(
    inputs: &EnvironmentInventorySynthesisInputs,
) -> String {
    let project_context_ref = if inputs.project_context_markdown.is_some() {
        ".system/project_context/PROJECT_CONTEXT.md"
    } else {
        "None"
    };

    let mut prompt = String::new();
    prompt.push_str("# Environment Inventory Synthesis Directive\n\n");
    prompt.push_str(ENVIRONMENT_INVENTORY_SYNTHESIZE_DIRECTIVE_MARKDOWN);
    prompt.push_str("\n\n# Canonical Write Contract\n\n");
    prompt.push_str("- Write only the canonical `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` content.\n");
    prompt.push_str(
        "- Do not describe any repo-root `ENVIRONMENT_INVENTORY.md` file as canonical.\n",
    );
    prompt.push_str("- Do not mention artifact copies as canonical or as the store of record.\n");
    prompt.push_str("- `PROJECT_CONTEXT` is optional context for this authoring flow. If it is absent, continue from the charter alone.\n");
    prompt.push_str("\n# ENVIRONMENT_INVENTORY.md Template\n\n```md\n");
    prompt.push_str(ENVIRONMENT_INVENTORY_TEMPLATE_MARKDOWN.trim());
    prompt.push_str("\n```\n\n");
    prompt.push_str("# Exact references that must be preserved verbatim\n\n");
    prompt.push_str(
        "- Canonical file reference: `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`\n",
    );
    prompt.push_str(&format!(
        "- Project context reference line: `{project_context_ref}`\n"
    ));
    prompt.push_str("\n# CHARTER.md\n\n```md\n");
    prompt.push_str(inputs.charter_markdown.trim());
    prompt.push_str("\n```\n");

    if let Some(project_context_markdown) = &inputs.project_context_markdown {
        prompt.push_str("\n# PROJECT_CONTEXT.md (optional context)\n\n```md\n");
        prompt.push_str(project_context_markdown.trim());
        prompt.push_str("\n```\n");
    } else {
        prompt.push_str(
            "\n# PROJECT_CONTEXT.md (optional context)\n\nNot present for this authoring run.\n",
        );
    }

    prompt.push_str(
        "\nReturn only the final `ENVIRONMENT_INVENTORY.md` markdown with all template placeholders resolved.\n",
    );
    prompt
}

fn validate_synthesized_environment_inventory_markdown(
    markdown: &str,
    inputs: &EnvironmentInventorySynthesisInputs,
) -> Result<(), AuthorEnvironmentInventoryRefusal> {
    validate_environment_inventory_markdown(markdown)?;

    let expected_project_context_ref = if inputs.project_context_markdown.is_some() {
        "> **Project Context Ref:** `.system/project_context/PROJECT_CONTEXT.md`"
    } else {
        "> **Project Context Ref:** None"
    };
    if !markdown.contains(expected_project_context_ref) {
        return Err(synthesis_refusal(format!(
            "synthesized environment inventory markdown must include the exact project context reference line `{expected_project_context_ref}`"
        )));
    }

    Ok(())
}

fn acquire_environment_inventory_authoring_lock(
    repo_root: &Path,
) -> Result<EnvironmentInventoryAuthoringLockGuard, AuthorEnvironmentInventoryRefusal> {
    let lock_path =
        resolve_repo_relative_write_path(repo_root, ENVIRONMENT_INVENTORY_AUTHORING_LOCK_REPO_PATH)
            .map_err(|err| AuthorEnvironmentInventoryRefusal {
                kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
                summary: format_repo_write_path_error(
                    ENVIRONMENT_INVENTORY_AUTHORING_LOCK_REPO_PATH,
                    err,
                ),
                broken_subject: "environment inventory authoring lock".to_string(),
                next_safe_action:
                    "repair the blocked environment inventory authoring lock path and retry `system author environment-inventory`"
                        .to_string(),
            })?;

    if let Some(parent) = lock_path.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|source| environment_inventory_authoring_lock_refusal(&lock_path, source))?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|source| environment_inventory_authoring_lock_refusal(&lock_path, source))?;

    lock_environment_inventory_authoring_file(&file, lock_operation_exclusive())
        .map_err(|source| environment_inventory_authoring_lock_refusal(&lock_path, source))?;

    Ok(EnvironmentInventoryAuthoringLockGuard { file, lock_path })
}

fn environment_inventory_authoring_lock_refusal(
    lock_path: &Path,
    source: std::io::Error,
) -> AuthorEnvironmentInventoryRefusal {
    AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::MutationRefused,
        summary: format!(
            "failed to acquire exclusive environment inventory authoring lock at {}: {source}",
            lock_path.display()
        ),
        broken_subject: "environment inventory authoring lock".to_string(),
        next_safe_action:
            "wait for any in-progress `system author environment-inventory` run to finish or repair the lock path, then retry `system author environment-inventory`"
                .to_string(),
    }
}

#[cfg(unix)]
fn lock_operation_exclusive() -> libc::c_int {
    libc::LOCK_EX
}

#[cfg(unix)]
fn lock_operation_unlock() -> libc::c_int {
    libc::LOCK_UN
}

#[cfg(unix)]
fn lock_environment_inventory_authoring_file(
    file: &File,
    operation: libc::c_int,
) -> Result<(), std::io::Error> {
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
fn lock_operation_exclusive() -> i32 {
    0
}

#[cfg(not(unix))]
fn lock_operation_unlock() -> i32 {
    0
}

#[cfg(not(unix))]
fn lock_environment_inventory_authoring_file(
    _file: &File,
    _operation: i32,
) -> Result<(), std::io::Error> {
    Ok(())
}

struct EnvironmentInventoryAuthoringLockGuard {
    file: File,
    lock_path: PathBuf,
}

impl Drop for EnvironmentInventoryAuthoringLockGuard {
    fn drop(&mut self) {
        let _ = lock_environment_inventory_authoring_file(&self.file, lock_operation_unlock());
        let _ = &self.lock_path;
    }
}

fn synthesize_output_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos();
    std::env::temp_dir().join(format!(
        "system-author-environment-inventory-{}-{timestamp}.md",
        std::process::id()
    ))
}

fn invalid_upstream_canonical_truth_refusal(
    broken_subject: &str,
    summary: String,
    next_safe_action: String,
) -> AuthorEnvironmentInventoryRefusal {
    AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::InvalidUpstreamCanonicalTruth,
        summary,
        broken_subject: broken_subject.to_string(),
        next_safe_action,
    }
}

fn synthesis_refusal(summary: impl Into<String>) -> AuthorEnvironmentInventoryRefusal {
    AuthorEnvironmentInventoryRefusal {
        kind: AuthorEnvironmentInventoryRefusalKind::SynthesisFailed,
        summary: summary.into(),
        broken_subject: "environment inventory synthesis runtime".to_string(),
        next_safe_action:
            "repair the environment inventory synthesis runtime or prompt inputs and retry `system author environment-inventory`"
                .to_string(),
    }
}

fn summarize_process_output(output: &Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

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

fn render_exit_status(code: Option<i32>) -> String {
    match code {
        Some(code) => format!("exit code {code}"),
        None => "signal termination".to_string(),
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

#[cfg(test)]
mod tests {
    use super::{
        author_environment_inventory, preflight_author_environment_inventory,
        AuthorEnvironmentInventoryRefusalKind, CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH,
    };
    use crate::canonical_artifacts::setup_starter_template;
    use crate::CanonicalArtifactKind;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::path::Path;
    use tempfile::tempdir;

    fn valid_charter_markdown() -> &'static str {
        "# Engineering Charter — System

## What this is
Body.

## How to use this charter
Use it.

## Rubric: 1–5 rigor levels
Levels.

## Project baseline posture
Baseline.

## Domains / areas (optional overrides)
None.

## Posture at a glance (quick scan)
Snapshot.

## Dimensions (details + guardrails)
Details.

## Cross-cutting red lines (global non-negotiables)
- Keep trust boundaries intact.

## Exceptions / overrides process
- **Approvers:** project_owner
- **Record location:** docs/exceptions.md
- **Minimum required fields:**
  - what
  - why
  - scope
  - risk
  - owner
  - expiry_or_revisit_date

## Debt tracking expectations
Tracked in issues.

## Decision Records (ADRs): how to use this charter
Use ADRs.

## Review & updates
Review monthly.
"
    }

    fn valid_project_context_markdown() -> &'static str {
        "# Project Context — System

> **File:** `PROJECT_CONTEXT.md`
> **Created (UTC):** 2026-04-21T00:00:00Z
> **Owner:** project-owner
> **Team:** system-team
> **Repo / Project:** /tmp/system
> **Charter Ref:** .system/charter/CHARTER.md

## What this is
Project reality.

## How to use this
Use this document to ground planning in reality.

## 0) Project Summary (factual, 3–6 bullets)
- Summary.

## 1) Operational Reality (the most important section)
- Runs on macOS and Linux.

## 2) Project Classification Implications (planning guardrails)
- Guardrails.

## 3) System Boundaries (what we own vs integrate with)
### What we own
- Canonical `.system/` truth.
### What we do NOT own (but may depend on)
- External delivery systems.

## 4) Integrations & Contracts (top 1–5)
- Integrations.

## 5) Environments & Delivery
- Delivery.

## 6) Data Reality
- Data.

## 7) Repo / Codebase Reality (brownfield-friendly, but safe for greenfield)
- Codebase.

## 8) Constraints
- Constraints.

## 9) Known Unknowns (explicitly tracked)
- Unknowns.

## 10) Update Triggers
- Update when reality changes.
"
    }

    #[test]
    fn preflight_requires_completed_charter() {
        let repo = tempdir().expect("tempdir");
        scaffold_environment_inventory_target(repo.path());

        let refusal = preflight_author_environment_inventory(repo.path()).expect_err("refusal");
        assert_eq!(
            refusal.kind,
            AuthorEnvironmentInventoryRefusalKind::MissingRequiredCharter
        );
        assert!(refusal.summary.contains("completed charter"));
    }

    #[test]
    fn author_writes_canonical_environment_inventory_and_reads_optional_project_context() {
        let repo = tempdir().expect("tempdir");
        scaffold_environment_inventory_target(repo.path());
        write_charter(repo.path(), valid_charter_markdown());
        write_project_context(repo.path(), valid_project_context_markdown());

        let prompt_log = repo.path().join("prompt.log");
        let fake_codex = write_fake_codex(repo.path());
        let canonical_markdown = r#"# Environment Inventory - Example

> **Canonical File:** `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
> **Project Context Ref:** `.system/project_context/PROJECT_CONTEXT.md`

## What this is
The canonical store of record for this project's environment and runtime requirements.

## How to use
- Update this file whenever runtime assumptions change.

## 1) Environment Variables (Inventory)
- None yet.

## 2) External Services / Infrastructure Dependencies
- None yet.

## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)
- None yet.

## 4) Local Development Requirements
- None yet.

## 5) CI Requirements
- None yet.

## 6) Production / Deployment Requirements (even if not live yet)
- None yet.

## 7) Dependency & Tooling Inventory (project-specific)
- None yet.

## 8) Update Contract (non-negotiable)
- Update `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` in the same change.

## 9) Known Unknowns
- None yet.
"#;

        std::env::set_var("SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN", &fake_codex);
        std::env::set_var(
            "SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_TEST_OUTPUT",
            canonical_markdown,
        );
        std::env::set_var(
            "SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_TEST_PROMPT_LOG",
            &prompt_log,
        );

        let result = author_environment_inventory(repo.path()).expect("author success");
        assert_eq!(
            result.canonical_repo_relative_path,
            CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH
        );
        assert_eq!(
            fs::read_to_string(repo.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH))
                .expect("inventory")
                .trim(),
            canonical_markdown.trim()
        );

        let prompt = fs::read_to_string(&prompt_log).expect("prompt log");
        assert!(prompt.contains(".system/environment_inventory/ENVIRONMENT_INVENTORY.md"));
        assert!(prompt.contains("Runs on macOS and Linux."));

        std::env::remove_var("SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN");
        std::env::remove_var("SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_TEST_OUTPUT");
        std::env::remove_var("SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_TEST_PROMPT_LOG");
    }

    #[test]
    fn author_refuses_to_overwrite_existing_non_starter_truth() {
        let repo = tempdir().expect("tempdir");
        scaffold_environment_inventory_target(repo.path());
        write_charter(repo.path(), valid_charter_markdown());
        fs::write(
            repo.path().join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH),
            "# Existing environment inventory\n",
        )
        .expect("write inventory");

        let refusal = author_environment_inventory(repo.path()).expect_err("refusal");
        assert_eq!(
            refusal.kind,
            AuthorEnvironmentInventoryRefusalKind::ExistingCanonicalTruth
        );
    }

    fn scaffold_environment_inventory_target(repo_root: &Path) {
        fs::create_dir_all(repo_root.join(".system/environment_inventory")).expect("mkdir");
        fs::create_dir_all(repo_root.join(".system/charter")).expect("mkdir");
        fs::write(
            repo_root.join(CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH),
            setup_starter_template(CanonicalArtifactKind::EnvironmentInventory),
        )
        .expect("starter inventory");
    }

    fn write_charter(repo_root: &Path, content: &str) {
        fs::write(repo_root.join(".system/charter/CHARTER.md"), content).expect("write charter");
    }

    fn write_project_context(repo_root: &Path, content: &str) {
        fs::create_dir_all(repo_root.join(".system/project_context")).expect("mkdir");
        fs::write(
            repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
            content,
        )
        .expect("write project context");
    }

    fn write_fake_codex(repo_root: &Path) -> String {
        let script_path = repo_root.join("fake-codex.sh");
        let script = r#"#!/bin/sh
output_path=""
while [ "$#" -gt 0 ]; do
  if [ "$1" = "--output-last-message" ]; then
    shift
    output_path="$1"
  fi
  shift
done
cat > "${SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_TEST_PROMPT_LOG}"
printf '%s' "${SYSTEM_AUTHOR_ENVIRONMENT_INVENTORY_TEST_OUTPUT}" > "$output_path"
"#;
        fs::write(&script_path, script).expect("write fake codex");
        let mut perms = fs::metadata(&script_path).expect("metadata").permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&script_path, perms).expect("chmod");
        script_path.display().to_string()
    }
}
