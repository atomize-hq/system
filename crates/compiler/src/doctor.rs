use crate::profile_readiness::{
    project_profile_readiness, ProfileArtifactRow, ProfileCapabilityRow, ProfileConditionRow,
    RepositoryReadinessStatus,
};
use handbook_engine::{
    inspect_profile_repository, resolve_shipped_profile_decisions, ResolvedProfileDecisions,
    ShippedProfileDecisionError,
};
use serde::Serialize;
use std::path::Path;

pub const DOCTOR_REPORT_SCHEMA_ID: &str = "handbook.repository-doctor-report";
pub const DOCTOR_REPORT_SCHEMA_VERSION: &str = "1.0.0";

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
    let projection = project_profile_readiness(decisions, &inspection);
    Ok(DoctorReport {
        schema_id: DOCTOR_REPORT_SCHEMA_ID.to_owned(),
        schema_version: DOCTOR_REPORT_SCHEMA_VERSION.to_owned(),
        profile_ref: projection.profile_ref,
        profile_fingerprint: projection.profile_fingerprint,
        stable_role_registry_ref: projection.stable_role_registry_ref,
        stable_role_registry_fingerprint: projection.stable_role_registry_fingerprint,
        conditions: projection.conditions,
        capabilities: projection.capabilities,
        artifacts: projection.artifacts,
        status: projection.status,
    })
}

fn map_profile_error(error: ShippedProfileDecisionError) -> DoctorError {
    match error {
        ShippedProfileDecisionError::Profile(_) => DoctorError::ShippedProfileUnavailable,
        ShippedProfileDecisionError::Decision(_) => DoctorError::SelectedProfileDecisionInvalid,
    }
}
