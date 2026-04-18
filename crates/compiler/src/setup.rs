use crate::canonical_artifacts::{
    canonical_artifact_descriptors, CanonicalArtifactDescriptor, CanonicalArtifacts,
    SystemRootStatus,
};
use crate::repo_file_access::{
    resolve_repo_relative_write_path, write_repo_relative_bytes, RepoRelativeMutationError,
    RepoRelativeWritePathError,
};
use crate::route_state::{preview_runtime_state_reset, reset_runtime_state_tree};
use std::collections::BTreeSet;
use std::path::Path;

const SYSTEM_DOCTOR_COMMAND: &str = "system doctor";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupMode {
    Auto,
    Init,
    Refresh,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupRequest {
    pub mode: SetupMode,
    pub rewrite: bool,
    pub reset_state: bool,
}

impl Default for SetupRequest {
    fn default() -> Self {
        Self {
            mode: SetupMode::Auto,
            rewrite: false,
            reset_state: false,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupActionLabel {
    Created,
    Preserved,
    Rewritten,
    Reset,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupAction {
    pub label: SetupActionLabel,
    pub path: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupPlan {
    pub requested_mode: SetupMode,
    pub resolved_mode: SetupMode,
    pub actions: Vec<SetupAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupOutcome {
    pub plan: SetupPlan,
    pub next_command: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SetupRefusalKind {
    AlreadyInitialized,
    MissingCanonicalRoot,
    InvalidCanonicalRoot,
    InvalidRequest,
    MutationRefused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetupRefusal {
    pub kind: SetupRefusalKind,
    pub summary: String,
    pub broken_subject: String,
    pub next_safe_action: String,
}

impl std::fmt::Display for SetupRefusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl std::error::Error for SetupRefusal {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PlannedMutation {
    Write {
        path: &'static str,
        bytes: &'static [u8],
    },
}

pub fn plan_setup(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
) -> Result<SetupPlan, SetupRefusal> {
    let repo_root = repo_root.as_ref();
    let artifacts = CanonicalArtifacts::load(repo_root).map_err(|err| {
        setup_mutation_refusal(request.mode, err.to_string(), "canonical `.system` root")
    })?;
    let resolved_mode = resolve_mode(request.mode, artifacts.system_root_status);
    validate_request(&artifacts, request, resolved_mode)?;

    let ingest_issue_paths = artifacts
        .ingest_issues
        .iter()
        .map(|issue| issue.canonical_repo_relative_path)
        .collect::<BTreeSet<_>>();
    let starter_actions = canonical_artifact_descriptors()
        .iter()
        .map(|descriptor| {
            plan_starter_action(
                repo_root,
                &artifacts,
                &ingest_issue_paths,
                descriptor,
                resolved_mode,
                request.rewrite,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    let mut actions = starter_actions
        .into_iter()
        .map(|planned| planned.action)
        .collect::<Vec<_>>();

    if request.reset_state {
        actions.extend(plan_state_reset(repo_root)?);
    }

    actions.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then_with(|| action_rank(a).cmp(&action_rank(b)))
    });

    Ok(SetupPlan {
        requested_mode: request.mode,
        resolved_mode,
        actions,
    })
}

pub fn run_setup(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
) -> Result<SetupOutcome, SetupRefusal> {
    let repo_root = repo_root.as_ref();
    let artifacts = CanonicalArtifacts::load(repo_root).map_err(|err| {
        setup_mutation_refusal(request.mode, err.to_string(), "canonical `.system` root")
    })?;
    let resolved_mode = resolve_mode(request.mode, artifacts.system_root_status);
    validate_request(&artifacts, request, resolved_mode)?;

    let ingest_issue_paths = artifacts
        .ingest_issues
        .iter()
        .map(|issue| issue.canonical_repo_relative_path)
        .collect::<BTreeSet<_>>();
    let planned = canonical_artifact_descriptors()
        .iter()
        .map(|descriptor| {
            plan_starter_action(
                repo_root,
                &artifacts,
                &ingest_issue_paths,
                descriptor,
                resolved_mode,
                request.rewrite,
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    for planned_action in &planned {
        if let Some(mutation) = planned_action.mutation {
            apply_mutation(repo_root, resolved_mode, mutation)?;
        }
    }

    let mut actions = planned
        .into_iter()
        .map(|planned_action| planned_action.action)
        .collect::<Vec<_>>();

    if request.reset_state {
        let reset_paths = reset_runtime_state_tree(repo_root).map_err(|reason| {
            setup_mutation_refusal(
                request.mode,
                reason,
                "runtime-state target under `.system/state/**`",
            )
        })?;
        actions.extend(reset_paths.into_iter().map(|path| SetupAction {
            label: SetupActionLabel::Reset,
            path,
        }));
    }

    actions.sort_by(|a, b| {
        a.path
            .cmp(&b.path)
            .then_with(|| action_rank(a).cmp(&action_rank(b)))
    });

    Ok(SetupOutcome {
        plan: SetupPlan {
            requested_mode: request.mode,
            resolved_mode,
            actions,
        },
        next_command: SYSTEM_DOCTOR_COMMAND.to_string(),
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct PlannedStarterAction {
    action: SetupAction,
    mutation: Option<PlannedMutation>,
}

fn resolve_mode(requested: SetupMode, system_root_status: SystemRootStatus) -> SetupMode {
    match requested {
        SetupMode::Auto => match system_root_status {
            SystemRootStatus::Ok => SetupMode::Refresh,
            SystemRootStatus::Missing
            | SystemRootStatus::NotDir
            | SystemRootStatus::SymlinkNotAllowed => SetupMode::Init,
        },
        SetupMode::Init | SetupMode::Refresh => requested,
    }
}

fn validate_request(
    artifacts: &CanonicalArtifacts,
    request: &SetupRequest,
    resolved_mode: SetupMode,
) -> Result<(), SetupRefusal> {
    match resolved_mode {
        SetupMode::Auto => {
            return Err(setup_refusal(
                SetupRefusalKind::InvalidRequest,
                "setup mode must resolve to init or refresh",
                "setup request",
                "retry `system setup`",
            ));
        }
        SetupMode::Init => {
            if request.rewrite || request.reset_state {
                return Err(setup_refusal(
                    SetupRefusalKind::InvalidRequest,
                    "setup init does not accept refresh-only flags; retry without --rewrite or --reset-state",
                    "setup request",
                    "retry `system setup init` without --rewrite or --reset-state",
                ));
            }
            if artifacts.system_root_status == SystemRootStatus::Ok {
                return Err(setup_refusal(
                    SetupRefusalKind::AlreadyInitialized,
                    "canonical .system root already exists; use `system setup refresh` instead",
                    "canonical `.system` root",
                    "run `system setup refresh`",
                ));
            }
        }
        SetupMode::Refresh => match artifacts.system_root_status {
            SystemRootStatus::Ok => {}
            SystemRootStatus::Missing => {
                return Err(setup_refusal(
                    SetupRefusalKind::MissingCanonicalRoot,
                    "canonical .system root is missing; run `system setup init` first",
                    "canonical `.system` root",
                    "run `system setup`",
                ));
            }
            SystemRootStatus::NotDir => {
                return Err(setup_refusal(
                    SetupRefusalKind::InvalidCanonicalRoot,
                    "canonical .system root is invalid; run `system setup` to re-establish it",
                    "canonical `.system` root",
                    "run `system setup`",
                ));
            }
            SystemRootStatus::SymlinkNotAllowed => {
                return Err(setup_refusal(
                    SetupRefusalKind::InvalidCanonicalRoot,
                    "canonical .system root must not be a symlink; run `system setup` to re-establish it",
                    "canonical `.system` root",
                    "run `system setup`",
                ));
            }
        },
    }

    Ok(())
}

fn plan_starter_action(
    repo_root: &Path,
    artifacts: &CanonicalArtifacts,
    ingest_issue_paths: &BTreeSet<&'static str>,
    descriptor: &CanonicalArtifactDescriptor,
    resolved_mode: SetupMode,
    rewrite: bool,
) -> Result<PlannedStarterAction, SetupRefusal> {
    let artifact = match descriptor.kind {
        crate::CanonicalArtifactKind::Charter => &artifacts.charter,
        crate::CanonicalArtifactKind::ProjectContext => &artifacts.project_context,
        crate::CanonicalArtifactKind::FeatureSpec => &artifacts.feature_spec,
    };

    if resolved_mode == SetupMode::Refresh
        && !rewrite
        && !matches!(artifact.identity.presence, crate::ArtifactPresence::Missing)
    {
        return Ok(PlannedStarterAction {
            action: SetupAction {
                label: SetupActionLabel::Preserved,
                path: descriptor.relative_path.to_string(),
            },
            mutation: None,
        });
    }

    if ingest_issue_paths.contains(descriptor.relative_path)
        || matches!(artifact.identity.presence, crate::ArtifactPresence::Missing)
        || rewrite
        || resolved_mode == SetupMode::Init
    {
        validate_write_target(repo_root, descriptor.relative_path, resolved_mode)?;
    }

    let label = if matches!(artifact.identity.presence, crate::ArtifactPresence::Missing) {
        SetupActionLabel::Created
    } else if rewrite {
        SetupActionLabel::Rewritten
    } else {
        SetupActionLabel::Created
    };

    Ok(PlannedStarterAction {
        action: SetupAction {
            label,
            path: descriptor.relative_path.to_string(),
        },
        mutation: Some(PlannedMutation::Write {
            path: descriptor.relative_path,
            bytes: starter_template_bytes(descriptor.kind),
        }),
    })
}

fn validate_write_target(
    repo_root: &Path,
    relative_path: &'static str,
    mode: SetupMode,
) -> Result<(), SetupRefusal> {
    resolve_repo_relative_write_path(repo_root, relative_path)
        .map(|_| ())
        .map_err(|err| {
            setup_mutation_refusal(
                mode,
                format_repo_write_path_error(relative_path, err),
                "setup-owned starter-file write target",
            )
        })
}

fn plan_state_reset(repo_root: &Path) -> Result<Vec<SetupAction>, SetupRefusal> {
    preview_runtime_state_reset(repo_root)
        .map(|paths| {
            paths
                .into_iter()
                .map(|path| SetupAction {
                    label: SetupActionLabel::Reset,
                    path,
                })
                .collect::<Vec<_>>()
        })
        .map_err(|reason| {
            setup_mutation_refusal(
                SetupMode::Refresh,
                reason,
                "runtime-state target under `.system/state/**`",
            )
        })
}

fn apply_mutation(
    repo_root: &Path,
    mode: SetupMode,
    mutation: PlannedMutation,
) -> Result<(), SetupRefusal> {
    match mutation {
        PlannedMutation::Write { path, bytes } => write_repo_relative_bytes(repo_root, path, bytes)
            .map_err(|err| {
                setup_mutation_refusal(
                    mode,
                    format_repo_mutation_error(path, err),
                    "setup-owned starter-file write target",
                )
            }),
    }
}

fn setup_refusal(
    kind: SetupRefusalKind,
    summary: impl Into<String>,
    broken_subject: impl Into<String>,
    next_safe_action: impl Into<String>,
) -> SetupRefusal {
    SetupRefusal {
        kind,
        summary: summary.into(),
        broken_subject: broken_subject.into(),
        next_safe_action: next_safe_action.into(),
    }
}

fn setup_mutation_refusal(
    mode: SetupMode,
    summary: impl Into<String>,
    broken_subject: impl Into<String>,
) -> SetupRefusal {
    let rerun_command = match mode {
        SetupMode::Auto | SetupMode::Init => "system setup",
        SetupMode::Refresh => "system setup refresh",
    };

    setup_refusal(
        SetupRefusalKind::MutationRefused,
        summary,
        broken_subject,
        format!("repair the blocked target and rerun `{rerun_command}`"),
    )
}

fn action_rank(action: &SetupAction) -> usize {
    match action.label {
        SetupActionLabel::Created => 0,
        SetupActionLabel::Preserved => 1,
        SetupActionLabel::Rewritten => 2,
        SetupActionLabel::Reset => 3,
    }
}

fn starter_template_bytes(kind: crate::CanonicalArtifactKind) -> &'static [u8] {
    match kind {
        crate::CanonicalArtifactKind::Charter => CHARTER_TEMPLATE.as_bytes(),
        crate::CanonicalArtifactKind::ProjectContext => PROJECT_CONTEXT_TEMPLATE.as_bytes(),
        crate::CanonicalArtifactKind::FeatureSpec => FEATURE_SPEC_TEMPLATE.as_bytes(),
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

const CHARTER_TEMPLATE: &str = "\
# Charter
\n\
Describe the durable operating rules for this system.\n\
\n\
## Purpose\n\
\n\
- TODO\n\
\n\
## Constraints\n\
\n\
- TODO\n\
\n\
## Review Cadence\n\
\n\
- TODO\n";

const FEATURE_SPEC_TEMPLATE: &str = "\
# Feature Spec
\n\
Describe the product behavior that trusted project truth should produce.\n\
\n\
## Problem\n\
\n\
- TODO\n\
\n\
## Outcomes\n\
\n\
- TODO\n\
\n\
## Scope\n\
\n\
- TODO\n";

const PROJECT_CONTEXT_TEMPLATE: &str = "\
# Project Context
\n\
Optional: capture surrounding architecture, constraints, and local context that help planning.\n\
\n\
## Current State\n\
\n\
- TODO\n\
\n\
## Constraints\n\
\n\
- TODO\n\
\n\
## Open Questions\n\
\n\
- TODO\n";
