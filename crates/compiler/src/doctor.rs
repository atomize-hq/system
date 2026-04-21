use crate::author::{
    validate_charter_markdown, validate_environment_inventory_markdown,
    validate_project_context_markdown,
};
use crate::canonical_artifacts::{
    canonical_artifact_descriptors, ArtifactIngestIssueKind, CanonicalArtifactDescriptor,
    CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
use crate::refusal::NextSafeAction;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoctorBaselineStatus {
    Scaffolded,
    PartialBaseline,
    InvalidBaseline,
    BaselineComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoctorArtifactStatus {
    Missing,
    Empty,
    StarterOwned,
    Invalid,
    ValidCanonicalTruth,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorChecklistItem {
    pub kind: CanonicalArtifactKind,
    pub canonical_repo_relative_path: &'static str,
    pub status: DoctorArtifactStatus,
    pub next_safe_action: Option<NextSafeAction>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoctorReport {
    pub status: DoctorBaselineStatus,
    pub system_root_status: SystemRootStatus,
    pub checklist: Vec<DoctorChecklistItem>,
    pub next_safe_action: Option<NextSafeAction>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArtifactInvalidReason {
    IngestIssue,
}

pub fn doctor(repo_root: impl AsRef<Path>) -> Result<DoctorReport, crate::ArtifactIngestError> {
    let artifacts = CanonicalArtifacts::load(repo_root)?;
    Ok(doctor_from_artifacts(&artifacts))
}

pub fn doctor_from_artifacts(artifacts: &CanonicalArtifacts) -> DoctorReport {
    let checklist = baseline_checklist(artifacts);
    let status = classify_doctor_status(artifacts.system_root_status, &checklist);
    let next_safe_action = checklist
        .iter()
        .find_map(|item| match item.next_safe_action {
            Some(NextSafeAction::RunSetupRefresh) => Some(NextSafeAction::RunSetupRefresh),
            _ => None,
        })
        .or_else(|| {
            checklist
                .iter()
                .find_map(|item| item.next_safe_action.clone())
        });

    DoctorReport {
        status,
        system_root_status: artifacts.system_root_status,
        checklist,
        next_safe_action,
    }
}

fn baseline_checklist(artifacts: &CanonicalArtifacts) -> Vec<DoctorChecklistItem> {
    canonical_artifact_descriptors()
        .iter()
        .filter(|descriptor| descriptor.baseline_required)
        .map(|descriptor| {
            let invalid_reason = artifact_invalid_reason(artifacts, descriptor);
            let status = classify_artifact_status(artifacts, descriptor, invalid_reason);
            let next_safe_action = artifact_next_safe_action(
                artifacts.system_root_status,
                descriptor.kind,
                status,
                invalid_reason,
            );
            DoctorChecklistItem {
                kind: descriptor.kind,
                canonical_repo_relative_path: descriptor.relative_path,
                status,
                next_safe_action,
            }
        })
        .collect()
}

fn classify_artifact_status(
    artifacts: &CanonicalArtifacts,
    descriptor: &CanonicalArtifactDescriptor,
    invalid_reason: Option<ArtifactInvalidReason>,
) -> DoctorArtifactStatus {
    if matches!(
        artifacts.system_root_status,
        SystemRootStatus::Missing | SystemRootStatus::NotDir | SystemRootStatus::SymlinkNotAllowed
    ) {
        return DoctorArtifactStatus::Missing;
    }

    if invalid_reason == Some(ArtifactInvalidReason::IngestIssue) {
        return DoctorArtifactStatus::Invalid;
    }

    let artifact = match descriptor.kind {
        CanonicalArtifactKind::Charter => &artifacts.charter,
        CanonicalArtifactKind::ProjectContext => &artifacts.project_context,
        CanonicalArtifactKind::EnvironmentInventory => &artifacts.environment_inventory,
        CanonicalArtifactKind::FeatureSpec => unreachable!("feature spec is not baseline"),
    };

    match artifact.identity.presence {
        crate::ArtifactPresence::Missing => DoctorArtifactStatus::Missing,
        crate::ArtifactPresence::PresentEmpty => DoctorArtifactStatus::Empty,
        crate::ArtifactPresence::PresentNonEmpty
            if artifact.identity.matches_setup_starter_template =>
        {
            DoctorArtifactStatus::StarterOwned
        }
        crate::ArtifactPresence::PresentNonEmpty => {
            let markdown = artifact
                .bytes
                .as_ref()
                .and_then(|bytes| String::from_utf8(bytes.clone()).ok());
            match markdown {
                Some(markdown)
                    if validate_artifact_markdown(descriptor.kind, &markdown).is_ok() =>
                {
                    DoctorArtifactStatus::ValidCanonicalTruth
                }
                _ => DoctorArtifactStatus::Invalid,
            }
        }
    }
}

fn artifact_invalid_reason(
    artifacts: &CanonicalArtifacts,
    descriptor: &CanonicalArtifactDescriptor,
) -> Option<ArtifactInvalidReason> {
    if has_ingest_issue_for_artifact(
        artifacts,
        descriptor.kind,
        descriptor.relative_path,
        ArtifactIngestIssueKind::CanonicalArtifactReadError,
    ) || has_ingest_issue_for_artifact(
        artifacts,
        descriptor.kind,
        descriptor.relative_path,
        ArtifactIngestIssueKind::CanonicalArtifactSymlinkNotAllowed,
    ) {
        Some(ArtifactInvalidReason::IngestIssue)
    } else {
        None
    }
}

fn has_ingest_issue_for_artifact(
    artifacts: &CanonicalArtifacts,
    kind: CanonicalArtifactKind,
    canonical_repo_relative_path: &'static str,
    issue_kind: ArtifactIngestIssueKind,
) -> bool {
    artifacts.ingest_issues.iter().any(|issue| {
        issue.kind == issue_kind
            && issue.artifact_kind == kind
            && issue.canonical_repo_relative_path == canonical_repo_relative_path
    })
}

fn validate_artifact_markdown(kind: CanonicalArtifactKind, markdown: &str) -> Result<(), String> {
    match kind {
        CanonicalArtifactKind::Charter => validate_charter_markdown(markdown),
        CanonicalArtifactKind::ProjectContext => {
            validate_project_context_markdown(markdown).map_err(|err| err.to_string())
        }
        CanonicalArtifactKind::EnvironmentInventory => {
            validate_environment_inventory_markdown(markdown).map_err(|err| err.to_string())
        }
        CanonicalArtifactKind::FeatureSpec => Ok(()),
    }
}

fn artifact_next_safe_action(
    system_root_status: SystemRootStatus,
    kind: CanonicalArtifactKind,
    status: DoctorArtifactStatus,
    invalid_reason: Option<ArtifactInvalidReason>,
) -> Option<NextSafeAction> {
    if matches!(
        system_root_status,
        SystemRootStatus::Missing | SystemRootStatus::NotDir | SystemRootStatus::SymlinkNotAllowed
    ) {
        return Some(NextSafeAction::RunSetup);
    }
    match status {
        DoctorArtifactStatus::ValidCanonicalTruth => None,
        DoctorArtifactStatus::Invalid
            if invalid_reason == Some(ArtifactInvalidReason::IngestIssue) =>
        {
            Some(NextSafeAction::RunSetupRefresh)
        }
        DoctorArtifactStatus::Missing
        | DoctorArtifactStatus::Empty
        | DoctorArtifactStatus::StarterOwned
        | DoctorArtifactStatus::Invalid => Some(match kind {
            CanonicalArtifactKind::Charter => NextSafeAction::RunAuthorCharter,
            CanonicalArtifactKind::ProjectContext => NextSafeAction::RunAuthorProjectContext,
            CanonicalArtifactKind::EnvironmentInventory => {
                NextSafeAction::RunAuthorEnvironmentInventory
            }
            CanonicalArtifactKind::FeatureSpec => NextSafeAction::FillCanonicalArtifact {
                canonical_repo_relative_path: ".system/feature_spec/FEATURE_SPEC.md",
            },
        }),
    }
}

fn classify_doctor_status(
    system_root_status: SystemRootStatus,
    checklist: &[DoctorChecklistItem],
) -> DoctorBaselineStatus {
    if matches!(
        system_root_status,
        SystemRootStatus::NotDir | SystemRootStatus::SymlinkNotAllowed
    ) {
        return DoctorBaselineStatus::InvalidBaseline;
    }

    let has_invalid = checklist
        .iter()
        .any(|item| item.status == DoctorArtifactStatus::Invalid);
    if has_invalid {
        return DoctorBaselineStatus::InvalidBaseline;
    }

    let valid_count = checklist
        .iter()
        .filter(|item| item.status == DoctorArtifactStatus::ValidCanonicalTruth)
        .count();

    if valid_count == checklist.len() {
        return DoctorBaselineStatus::BaselineComplete;
    }

    if valid_count > 0 {
        return DoctorBaselineStatus::PartialBaseline;
    }

    DoctorBaselineStatus::Scaffolded
}
