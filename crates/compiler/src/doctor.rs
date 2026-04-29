use crate::baseline_validation::{
    baseline_artifact_validations, BaselineArtifactValidation, BaselineArtifactVerdict,
};
use crate::blocker::{build_doctor_blockers, C04_RESULT_VERSION};
use crate::canonical_artifacts::{CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus};
use crate::refusal::{NextSafeAction, SubjectRef};
use crate::{ArtifactManifest, Blocker, ManifestInputs};
use serde::Serialize;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorBaselineStatus {
    Scaffolded,
    PartialBaseline,
    InvalidBaseline,
    BaselineComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DoctorArtifactStatus {
    Missing,
    Empty,
    StarterOwned,
    Invalid,
    ValidCanonicalTruth,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DoctorChecklistItem {
    pub artifact_label: &'static str,
    pub subject: SubjectRef,
    pub author_command: &'static str,
    pub kind: CanonicalArtifactKind,
    pub canonical_repo_relative_path: &'static str,
    pub status: DoctorArtifactStatus,
    pub next_safe_action: Option<NextSafeAction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DoctorReport {
    pub c04_result_version: String,
    pub c03_schema_version: String,
    pub c03_manifest_generation_version: u32,
    pub baseline_state: DoctorBaselineStatus,
    pub blockers: Vec<Blocker>,
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
    let manifest = ArtifactManifest::from_canonical_artifacts(artifacts, ManifestInputs::default());
    let checklist = baseline_checklist(artifacts);
    let baseline_state = classify_doctor_status(artifacts.system_root_status, &checklist);
    let blockers = build_doctor_blockers(&manifest, &baseline_artifact_validations(artifacts));
    let next_safe_action = blockers
        .first()
        .map(|blocker| blocker.next_safe_action.clone())
        .or_else(|| {
            checklist
                .iter()
                .find_map(|item| item.next_safe_action.clone())
        });

    DoctorReport {
        c04_result_version: C04_RESULT_VERSION.to_string(),
        c03_schema_version: manifest.version.schema.version.to_string(),
        c03_manifest_generation_version: manifest.version.generation,
        baseline_state,
        blockers,
        status: baseline_state,
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
                artifact_label: doctor_artifact_label(validation.kind),
                subject: SubjectRef::CanonicalArtifact {
                    kind: validation.kind,
                    canonical_repo_relative_path: validation.canonical_repo_relative_path,
                },
                author_command: doctor_author_command(validation.kind),
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

    let has_invalid = checklist.iter().any(|item| {
        matches!(
            item.status,
            DoctorArtifactStatus::Empty | DoctorArtifactStatus::Invalid
        )
    });
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

fn doctor_artifact_label(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "CHARTER",
        CanonicalArtifactKind::ProjectContext => "PROJECT_CONTEXT",
        CanonicalArtifactKind::EnvironmentInventory => "ENVIRONMENT_INVENTORY",
        CanonicalArtifactKind::FeatureSpec => "FEATURE_SPEC",
    }
}

fn doctor_author_command(kind: CanonicalArtifactKind) -> &'static str {
    match kind {
        CanonicalArtifactKind::Charter => "run `system author charter`",
        CanonicalArtifactKind::ProjectContext => "run `system author project-context`",
        CanonicalArtifactKind::EnvironmentInventory => "run `system author environment-inventory`",
        CanonicalArtifactKind::FeatureSpec => {
            "fill canonical artifact at .system/feature_spec/FEATURE_SPEC.md"
        }
    }
}
