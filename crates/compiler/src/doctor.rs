use crate::baseline_validation::{
    baseline_artifact_validations, BaselineArtifactValidation, BaselineArtifactVerdict,
};
use crate::canonical_artifacts::{CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus};
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
    baseline_artifact_validations(artifacts)
        .into_iter()
        .map(|validation| {
            let status = classify_artifact_status(artifacts.system_root_status, &validation);
            let next_safe_action =
                artifact_next_safe_action(artifacts.system_root_status, &validation, status);
            DoctorChecklistItem {
                kind: validation.kind,
                canonical_repo_relative_path: validation.canonical_repo_relative_path,
                status,
                next_safe_action,
            }
        })
        .collect()
}

fn classify_artifact_status(
    system_root_status: SystemRootStatus,
    validation: &BaselineArtifactValidation,
) -> DoctorArtifactStatus {
    if matches!(
        system_root_status,
        SystemRootStatus::Missing | SystemRootStatus::NotDir | SystemRootStatus::SymlinkNotAllowed
    ) {
        return DoctorArtifactStatus::Missing;
    }

    match &validation.verdict {
        BaselineArtifactVerdict::Missing => DoctorArtifactStatus::Missing,
        BaselineArtifactVerdict::Empty => DoctorArtifactStatus::Empty,
        BaselineArtifactVerdict::StarterOwned => DoctorArtifactStatus::StarterOwned,
        BaselineArtifactVerdict::IngestInvalid
        | BaselineArtifactVerdict::SemanticallyInvalid { .. } => DoctorArtifactStatus::Invalid,
        BaselineArtifactVerdict::ValidCanonicalTruth { .. } => {
            DoctorArtifactStatus::ValidCanonicalTruth
        }
    }
}

fn artifact_next_safe_action(
    system_root_status: SystemRootStatus,
    validation: &BaselineArtifactValidation,
    status: DoctorArtifactStatus,
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
            if matches!(validation.verdict, BaselineArtifactVerdict::IngestInvalid) =>
        {
            Some(NextSafeAction::RunSetupRefresh)
        }
        DoctorArtifactStatus::Missing
        | DoctorArtifactStatus::Empty
        | DoctorArtifactStatus::StarterOwned
        | DoctorArtifactStatus::Invalid => Some(match validation.kind {
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
