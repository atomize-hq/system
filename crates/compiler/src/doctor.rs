use crate::profile_readiness::{
    project_profile_readiness, ProfileArtifactRow, ProfileCapabilityRow, ProfileConditionRow,
    RepositoryReadinessStatus,
};
use handbook_engine::{
    inspect_profile_repository, resolve_shipped_profile_decisions, ProfileInspectionReport,
    ResolvedProfileDecisions, ShippedProfileDecisionError,
};
use serde::Serialize;
use std::path::Path;

pub const DOCTOR_REPORT_SCHEMA_ID: &str = "handbook.repository-doctor-report";
pub const DOCTOR_REPORT_SCHEMA_VERSION: &str = "1.1.0";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DoctorProjectContextRow {
    pub instance_id: String,
    pub kind_ref: String,
    pub canonical_path: String,
    pub source_fingerprint: String,
    pub rendered_output_fingerprint: String,
    pub rendered_media_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DoctorReport {
    pub schema_id: String,
    pub schema_version: String,
    pub profile_ref: String,
    pub profile_fingerprint: String,
    pub stable_role_registry_ref: String,
    pub stable_role_registry_fingerprint: String,
    pub conditions: Vec<ProfileConditionRow>,
    pub capabilities: Vec<ProfileCapabilityRow>,
    pub artifacts: Vec<ProfileArtifactRow>,
    pub project_context: Option<DoctorProjectContextRow>,
    pub status: RepositoryReadinessStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoctorErrorKind {
    ProfileResolution,
    ProfileDecision,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoctorErrorReasonCode {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoctorError {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
}

impl DoctorError {
    pub const ALL: [Self; 2] = [
        Self::ShippedProfileUnavailable,
        Self::SelectedProfileDecisionInvalid,
    ];

    pub fn kind(&self) -> DoctorErrorKind {
        match self {
            Self::ShippedProfileUnavailable => DoctorErrorKind::ProfileResolution,
            Self::SelectedProfileDecisionInvalid => DoctorErrorKind::ProfileDecision,
        }
    }

    pub fn reason_code(&self) -> DoctorErrorReasonCode {
        match self {
            Self::ShippedProfileUnavailable => DoctorErrorReasonCode::ShippedProfileUnavailable,
            Self::SelectedProfileDecisionInvalid => {
                DoctorErrorReasonCode::SelectedProfileDecisionInvalid
            }
        }
    }
}

impl std::fmt::Display for DoctorError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "doctor refused: {:?}", self)
    }
}

impl std::error::Error for DoctorError {}

pub fn doctor(repo_root: impl AsRef<Path>) -> Result<DoctorReport, DoctorError> {
    let repo_root = repo_root.as_ref();
    let decisions = resolve_shipped_profile_decisions(repo_root).map_err(map_profile_error)?;
    doctor_with_decisions(repo_root, &decisions)
}

pub fn doctor_with_decisions(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
) -> Result<DoctorReport, DoctorError> {
    let inspection = inspect_profile_repository(repo_root, decisions);
    Ok(doctor_report_from_inspection(decisions, &inspection))
}

fn doctor_report_from_inspection(
    decisions: &ResolvedProfileDecisions,
    inspection: &ProfileInspectionReport,
) -> DoctorReport {
    let project_context = inspection
        .artifacts()
        .iter()
        .find(|artifact| artifact.instance_id().as_str() == "project_context")
        .and_then(|artifact| {
            if artifact.status() != handbook_engine::ArtifactInspectionStatus::StructurallyValid
                || artifact.reason()
                    != handbook_engine::ArtifactInspectionReason::PresentAndStructurallyValid
            {
                return None;
            }
            artifact
                .project_context_projection()
                .map(|projection| DoctorProjectContextRow {
                    instance_id: artifact.instance_id().as_str().to_owned(),
                    kind_ref: decisions
                        .artifact_decisions()
                        .iter()
                        .find(|decision| decision.instance_id() == artifact.instance_id())
                        .expect("inspection row retains selected decision")
                        .kind_ref()
                        .as_str()
                        .to_owned(),
                    canonical_path: projection.canonical_path().to_owned(),
                    source_fingerprint: projection.source_fingerprint().as_str().to_owned(),
                    rendered_output_fingerprint: projection
                        .rendered_output_fingerprint()
                        .as_str()
                        .to_owned(),
                    rendered_media_type: "text/markdown".to_owned(),
                })
        });
    let projection = project_profile_readiness(decisions, inspection);
    DoctorReport {
        schema_id: DOCTOR_REPORT_SCHEMA_ID.to_owned(),
        schema_version: DOCTOR_REPORT_SCHEMA_VERSION.to_owned(),
        profile_ref: projection.profile_ref,
        profile_fingerprint: projection.profile_fingerprint,
        stable_role_registry_ref: projection.stable_role_registry_ref,
        stable_role_registry_fingerprint: projection.stable_role_registry_fingerprint,
        conditions: projection.conditions,
        capabilities: projection.capabilities,
        artifacts: projection.artifacts,
        project_context,
        status: projection.status,
    }
}

fn map_profile_error(error: ShippedProfileDecisionError) -> DoctorError {
    match error {
        ShippedProfileDecisionError::Profile(_) => DoctorError::ShippedProfileUnavailable,
        ShippedProfileDecisionError::Decision(_) => DoctorError::SelectedProfileDecisionInvalid,
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use handbook_engine::{
        inspect_profile_repository_with_stability_hook, ArtifactInspectionReason,
        ArtifactInspectionStatus,
    };

    const PROJECT_CONTEXT_YAML: &str = concat!(
        "schema_id: \"handbook.artifact.project-context\"\n",
        "schema_version: \"1.0\"\n",
        "record_id: \"handbook.project-context\"\n",
        "summary: \"Doctor stability.\"\n",
        "system_boundaries:\n",
        "  - \"API\"\n",
        "ownership:\n",
        "  - \"Platform\"\n",
        "authoritative_references: []\n",
        "known_unknowns: []\n",
    );

    #[test]
    fn doctor_nulls_project_context_and_is_invalid_after_substitution_or_inode_aba() {
        for identical_replacement in [false, true] {
            let repo = tempfile::tempdir().unwrap();
            let selected = repo.path().join(".handbook/project/context.yaml");
            std::fs::create_dir_all(selected.parent().unwrap()).unwrap();
            std::fs::write(&selected, PROJECT_CONTEXT_YAML).unwrap();
            let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
            let backup = repo.path().join("initial-project-context.yaml");
            let replacement = if identical_replacement {
                PROJECT_CONTEXT_YAML.to_owned()
            } else {
                PROJECT_CONTEXT_YAML.replace("Doctor stability.", "Changed stability.")
            };

            let inspection =
                inspect_profile_repository_with_stability_hook(repo.path(), &decisions, || {
                    if identical_replacement {
                        std::fs::rename(&selected, &backup).unwrap();
                    }
                    std::fs::write(&selected, replacement).unwrap();
                });
            let report = doctor_report_from_inspection(&decisions, &inspection);
            let row = report
                .artifacts
                .iter()
                .find(|artifact| artifact.instance_id == "project_context")
                .unwrap();

            assert_eq!(row.inspection_status, ArtifactInspectionStatus::Unreadable);
            assert_eq!(
                row.inspection_reason,
                ArtifactInspectionReason::ObservationChangedDuringInspection
            );
            assert!(report.project_context.is_none());
            assert_eq!(report.status, RepositoryReadinessStatus::Invalid);
        }
    }
}
