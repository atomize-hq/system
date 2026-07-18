#[cfg(not(unix))]
use handbook_compiler::RepositoryReadinessStatus;
use handbook_compiler::{
    doctor, doctor_with_decisions, DoctorError, DoctorErrorKind, DoctorErrorReasonCode,
    DOCTOR_REPORT_SCHEMA_ID, DOCTOR_REPORT_SCHEMA_VERSION,
};
use handbook_engine::resolve_shipped_profile_decisions;
use tempfile::tempdir;

#[test]
fn doctor_emits_the_frozen_schema_and_complete_profile_rows() {
    let repo = tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    let report = doctor_with_decisions(repo.path(), &decisions).unwrap();

    assert_eq!(report.schema_id, DOCTOR_REPORT_SCHEMA_ID);
    assert_eq!(report.schema_version, DOCTOR_REPORT_SCHEMA_VERSION);
    assert_eq!(report.profile_ref, decisions.profile_ref().as_str());
    assert_eq!(report.artifacts.len(), decisions.artifact_decisions().len());
    assert_eq!(
        report.conditions.len(),
        decisions.condition_evaluations().len()
    );
    assert_eq!(
        report.capabilities.len(),
        decisions.capability_truth().len()
    );
}

#[test]
fn default_and_injected_doctor_reports_are_identical() {
    let repo = tempdir().unwrap();
    let decisions = resolve_shipped_profile_decisions(repo.path()).unwrap();
    assert_eq!(
        doctor(repo.path()).unwrap(),
        doctor_with_decisions(repo.path(), &decisions).unwrap()
    );
}

#[test]
fn doctor_error_projection_is_exhaustive() {
    let expected = [
        (
            DoctorError::ShippedProfileUnavailable,
            DoctorErrorKind::ProfileResolution,
            DoctorErrorReasonCode::ShippedProfileUnavailable,
        ),
        (
            DoctorError::SelectedProfileDecisionInvalid,
            DoctorErrorKind::ProfileDecision,
            DoctorErrorReasonCode::SelectedProfileDecisionInvalid,
        ),
    ];
    assert_eq!(DoctorError::ALL, expected.map(|row| row.0));
    for (error, kind, reason) in expected {
        assert_eq!(error.kind(), kind);
        assert_eq!(error.reason_code(), reason);
    }
}

#[cfg(not(unix))]
#[test]
fn non_unix_doctor_reports_invalid_without_reading_artifacts() {
    let repo = tempdir().unwrap();
    let report = doctor(repo.path()).unwrap();
    assert_eq!(report.status, RepositoryReadinessStatus::Invalid);
}
