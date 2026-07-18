use crate::canonical_repo_support::{CanonicalWorkspace, RepoRelativeFileAccessError};
use crate::definition_identity::{
    parse_definition_yaml, ExactDefinitionRef, RegistryLoadErrorKind, MAX_SOURCE_DOCUMENT_BYTES,
    MAX_TOTAL_SOURCE_BYTES,
};
use crate::instance_profile::SymbolicId;
use crate::profile_decision::{ArtifactApplicability, ResolvedProfileDecisions};
use serde::Serialize;
use std::path::Path;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactInspectionStatus {
    Missing,
    StructurallyValid,
    StructurallyInvalid,
    UnsafePath,
    Unreadable,
    NotInspected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactInspectionReason {
    PresentAndStructurallyValid,
    RequiredPathMissing,
    OptionalPathMissing,
    ConditionalEvidenceUnavailablePathMissing,
    ConditionalEvidenceUnavailablePathPresent,
    YamlSyntaxInvalid,
    DuplicateYamlKey,
    DocumentNotObject,
    StructuralValidationFailed,
    DocumentLimitExceeded,
    AggregateReadLimitExceeded,
    SymlinkRefused,
    NonRegularFileRefused,
    UnsafeRepositoryPath,
    UnsupportedPlatformStrictRead,
    RepositoryReadFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactInspection {
    instance_id: SymbolicId,
    canonical_path: String,
    applicability: ArtifactApplicability,
    status: ArtifactInspectionStatus,
    reason: ArtifactInspectionReason,
}

impl ArtifactInspection {
    pub fn instance_id(&self) -> &SymbolicId {
        &self.instance_id
    }
    pub fn canonical_path(&self) -> &str {
        &self.canonical_path
    }
    pub fn applicability(&self) -> ArtifactApplicability {
        self.applicability
    }
    pub fn status(&self) -> ArtifactInspectionStatus {
        self.status
    }
    pub fn reason(&self) -> ArtifactInspectionReason {
        self.reason
    }
}

#[derive(Clone, Debug)]
pub struct ProfileInspectionReport {
    profile_ref: ExactDefinitionRef,
    profile_fingerprint: crate::DefinitionFingerprint,
    artifacts: Vec<ArtifactInspection>,
}

impl ProfileInspectionReport {
    pub fn profile_ref(&self) -> &ExactDefinitionRef {
        &self.profile_ref
    }
    pub fn profile_fingerprint(&self) -> &crate::DefinitionFingerprint {
        &self.profile_fingerprint
    }
    pub fn artifacts(&self) -> &[ArtifactInspection] {
        &self.artifacts
    }
}

pub fn inspect_profile_repository(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
) -> ProfileInspectionReport {
    let workspace = CanonicalWorkspace::new(repo_root.as_ref());
    let mut consumed_bytes = 0usize;
    let mut aggregate_exhausted = false;
    let mut artifacts = Vec::with_capacity(decisions.artifact_decisions().len());

    for decision in decisions.artifact_decisions() {
        if consumed_bytes >= MAX_TOTAL_SOURCE_BYTES {
            aggregate_exhausted = true;
        }
        let (status, reason) = if aggregate_exhausted {
            aggregate_limit()
        } else {
            let normalized = match workspace.normalize_repo_relative(decision.canonical_path()) {
                Ok(normalized) => normalized,
                Err(_) => {
                    artifacts.push(inspection(
                        decision.instance_id().clone(),
                        decision.canonical_path(),
                        decision.applicability(),
                        ArtifactInspectionStatus::UnsafePath,
                        ArtifactInspectionReason::UnsafeRepositoryPath,
                    ));
                    continue;
                }
            };
            match workspace.trusted_read_strict(&normalized) {
                Err(error) => map_open_error(error, decision.applicability()),
                Ok(file) => {
                    let remaining = MAX_TOTAL_SOURCE_BYTES.saturating_sub(consumed_bytes);
                    if remaining == 0 {
                        aggregate_exhausted = true;
                        aggregate_limit()
                    } else {
                        let read_limit = remaining.min(MAX_SOURCE_DOCUMENT_BYTES);
                        match file.read_bytes_bounded(read_limit) {
                            Err(_) => repository_read_failed(),
                            Ok((_bytes, exceeded))
                                if exceeded && remaining < MAX_SOURCE_DOCUMENT_BYTES =>
                            {
                                aggregate_exhausted = true;
                                aggregate_limit()
                            }
                            Ok((_bytes, true)) => {
                                consumed_bytes = consumed_bytes
                                    .saturating_add(MAX_SOURCE_DOCUMENT_BYTES)
                                    .min(MAX_TOTAL_SOURCE_BYTES);
                                (
                                    ArtifactInspectionStatus::Unreadable,
                                    ArtifactInspectionReason::DocumentLimitExceeded,
                                )
                            }
                            Ok((bytes, false)) => {
                                consumed_bytes += bytes.len();
                                inspect_bytes(
                                    decisions,
                                    decision.instance_id(),
                                    decision.applicability(),
                                    &bytes,
                                )
                            }
                        }
                    }
                }
            }
        };
        artifacts.push(inspection(
            decision.instance_id().clone(),
            decision.canonical_path(),
            decision.applicability(),
            status,
            reason,
        ));
    }

    ProfileInspectionReport {
        profile_ref: decisions.profile_ref().clone(),
        profile_fingerprint: decisions.profile_fingerprint().clone(),
        artifacts,
    }
}

fn inspect_bytes(
    decisions: &ResolvedProfileDecisions,
    instance_id: &SymbolicId,
    applicability: ArtifactApplicability,
    bytes: &[u8],
) -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    let value = match parse_definition_yaml(bytes) {
        Ok(value) => value,
        Err(error) if error.kind() == RegistryLoadErrorKind::DuplicateKey => {
            return (
                ArtifactInspectionStatus::StructurallyInvalid,
                ArtifactInspectionReason::DuplicateYamlKey,
            )
        }
        Err(_) => {
            return (
                ArtifactInspectionStatus::StructurallyInvalid,
                ArtifactInspectionReason::YamlSyntaxInvalid,
            )
        }
    };
    if !value.is_object() {
        return (
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::DocumentNotObject,
        );
    }
    if decisions
        .registry()
        .validate_json(instance_id, &value)
        .is_err()
    {
        return (
            ArtifactInspectionStatus::StructurallyInvalid,
            ArtifactInspectionReason::StructuralValidationFailed,
        );
    }
    (
        ArtifactInspectionStatus::StructurallyValid,
        match applicability {
            ArtifactApplicability::Indeterminate => {
                ArtifactInspectionReason::ConditionalEvidenceUnavailablePathPresent
            }
            ArtifactApplicability::Required | ArtifactApplicability::Optional => {
                ArtifactInspectionReason::PresentAndStructurallyValid
            }
        },
    )
}

fn map_open_error(
    error: RepoRelativeFileAccessError,
    applicability: ArtifactApplicability,
) -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    match error {
        RepoRelativeFileAccessError::Missing(_) => match applicability {
            ArtifactApplicability::Required => (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::RequiredPathMissing,
            ),
            ArtifactApplicability::Optional => (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::OptionalPathMissing,
            ),
            ArtifactApplicability::Indeterminate => (
                ArtifactInspectionStatus::NotInspected,
                ArtifactInspectionReason::ConditionalEvidenceUnavailablePathMissing,
            ),
        },
        RepoRelativeFileAccessError::SymlinkNotAllowed(_) => (
            ArtifactInspectionStatus::UnsafePath,
            ArtifactInspectionReason::SymlinkRefused,
        ),
        RepoRelativeFileAccessError::NotRegularFile(_) => (
            ArtifactInspectionStatus::UnsafePath,
            ArtifactInspectionReason::NonRegularFileRefused,
        ),
        RepoRelativeFileAccessError::InvalidPath(_) => {
            #[cfg(unix)]
            let reason = ArtifactInspectionReason::UnsafeRepositoryPath;
            #[cfg(not(unix))]
            let reason = ArtifactInspectionReason::UnsupportedPlatformStrictRead;
            (ArtifactInspectionStatus::UnsafePath, reason)
        }
        RepoRelativeFileAccessError::ReadFailure { .. } => repository_read_failed(),
    }
}

fn inspection(
    instance_id: SymbolicId,
    canonical_path: &str,
    applicability: ArtifactApplicability,
    status: ArtifactInspectionStatus,
    reason: ArtifactInspectionReason,
) -> ArtifactInspection {
    ArtifactInspection {
        instance_id,
        canonical_path: canonical_path.to_owned(),
        applicability,
        status,
        reason,
    }
}

fn aggregate_limit() -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    (
        ArtifactInspectionStatus::Unreadable,
        ArtifactInspectionReason::AggregateReadLimitExceeded,
    )
}

fn repository_read_failed() -> (ArtifactInspectionStatus, ArtifactInspectionReason) {
    (
        ArtifactInspectionStatus::Unreadable,
        ArtifactInspectionReason::RepositoryReadFailed,
    )
}

#[cfg(test)]
mod mapping_tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn required_missing_open_maps_to_required_path_missing() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::Missing(PathBuf::from("missing")),
                ArtifactApplicability::Required,
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::RequiredPathMissing,
            )
        );
    }

    #[test]
    fn optional_missing_open_maps_to_optional_path_missing() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::Missing(PathBuf::from("missing")),
                ArtifactApplicability::Optional,
            ),
            (
                ArtifactInspectionStatus::Missing,
                ArtifactInspectionReason::OptionalPathMissing,
            )
        );
    }

    #[test]
    fn indeterminate_missing_open_maps_to_condition_unavailable() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::Missing(PathBuf::from("missing")),
                ArtifactApplicability::Indeterminate,
            ),
            (
                ArtifactInspectionStatus::NotInspected,
                ArtifactInspectionReason::ConditionalEvidenceUnavailablePathMissing,
            )
        );
    }

    #[test]
    fn symlink_open_refusal_maps_to_symlink_refused() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::SymlinkNotAllowed(PathBuf::from("linked")),
                ArtifactApplicability::Required,
            ),
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::SymlinkRefused,
            )
        );
    }

    #[test]
    fn non_regular_open_refusal_maps_to_non_regular_file_refused() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::NotRegularFile(PathBuf::from("directory")),
                ArtifactApplicability::Required,
            ),
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::NonRegularFileRefused,
            )
        );
    }

    #[test]
    fn invalid_repository_path_maps_to_platform_specific_unsafe_reason() {
        let actual = map_open_error(
            RepoRelativeFileAccessError::InvalidPath("invalid".to_owned()),
            ArtifactApplicability::Required,
        );
        #[cfg(unix)]
        assert_eq!(
            actual,
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::UnsafeRepositoryPath,
            )
        );
        #[cfg(not(unix))]
        assert_eq!(
            actual,
            (
                ArtifactInspectionStatus::UnsafePath,
                ArtifactInspectionReason::UnsupportedPlatformStrictRead,
            )
        );
    }

    #[test]
    fn read_failure_maps_to_bounded_repository_read_failed() {
        assert_eq!(
            map_open_error(
                RepoRelativeFileAccessError::ReadFailure {
                    path: PathBuf::from("unreadable"),
                    source: std::io::Error::new(std::io::ErrorKind::PermissionDenied, "secret"),
                },
                ArtifactApplicability::Required,
            ),
            (
                ArtifactInspectionStatus::Unreadable,
                ArtifactInspectionReason::RepositoryReadFailed,
            )
        );
    }
}
