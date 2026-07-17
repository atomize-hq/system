use crate::definition_identity::{
    ExactDefinitionRef, RegistryLoadError, RegistryLoadErrorKind, SourceByteBudget,
};
use crate::stable_role_registry::read_trusted_repo_source;
use std::collections::BTreeSet;
use std::fmt;
use std::path::Path;

#[allow(dead_code)] // Activated by the Task 16 resolver increment.
const MAX_SYMBOLIC_ID_BYTES: usize = 64;
#[allow(dead_code)] // Activated by the Task 16 resolver increment.
const MAX_PROFILE_SOURCES: usize = 64;
#[allow(dead_code)] // Activated by the Task 16 resolver increment.
const MAX_DEFINITION_SOURCE_BINDINGS: usize = 512;
#[allow(dead_code)] // Activated by the Task 16 resolver increment.
const MAX_ALLOWED_SCHEMA_ROOTS: usize = 32;
#[allow(dead_code)] // Activated by the Task 16 resolver increment.
const MAX_REPOSITORY_PATH_BYTES: usize = 1024;
#[allow(dead_code)] // Activated by the Task 16 resolver increment.
const MAX_REPOSITORY_PATH_COMPONENTS: usize = 64;
const MAX_PROFILE_ERROR_LOCATION_BYTES: usize = 240;

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SymbolicId(String);

impl SymbolicId {
    pub fn parse(value: &str) -> Result<Self, ProfileLoadError> {
        let bytes = value.as_bytes();
        let valid = (1..=MAX_SYMBOLIC_ID_BYTES).contains(&bytes.len())
            && value.is_ascii()
            && bytes.first().is_some_and(u8::is_ascii_lowercase)
            && value.split('_').all(|segment| {
                !segment.is_empty()
                    && segment
                        .bytes()
                        .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
            });
        if !valid {
            return Err(ProfileLoadError::new(
                ProfileLoadErrorKind::InvalidSymbolicId,
                "symbolic ID violates the 1-64-byte lowercase snake-case grammar",
            ));
        }
        Ok(Self(value.to_owned()))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SymbolicId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefinitionSource {
    BuiltIn(ExactDefinitionRef),
    RepositoryPath(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionSourceBinding {
    pub definition_ref: ExactDefinitionRef,
    pub source: DefinitionSource,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileSelectionRequest {
    pub selected_profile_ref: ExactDefinitionRef,
    pub profile_sources: Vec<DefinitionSourceBinding>,
    pub stable_role_registry_sources: Vec<DefinitionSourceBinding>,
    pub schema_entry_sources: Vec<DefinitionSourceBinding>,
    pub artifact_kind_sources: Vec<DefinitionSourceBinding>,
    pub semantic_capability_sources: Vec<DefinitionSourceBinding>,
    pub semantic_validator_sources: Vec<DefinitionSourceBinding>,
    pub project_condition_sources: Vec<DefinitionSourceBinding>,
    pub vocabulary_sources: Vec<DefinitionSourceBinding>,
    pub context_resolution_sources: Vec<DefinitionSourceBinding>,
    pub context_resolution_policy_sources: Vec<DefinitionSourceBinding>,
    pub allowed_schema_roots: Vec<String>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileLoadErrorKind {
    InvalidSymbolicId,
    ProfileSourceLimitExceeded,
    DefinitionBindingLimitExceeded,
    SchemaRootLimitExceeded,
    DuplicateSourceBinding,
    DuplicateSchemaRoot,
    SourceIdentityMismatch,
    InvalidSourcePath,
    InvalidSchemaRoot,
    MissingSource,
    SymlinkSource,
    NonRegularSource,
    SourceReadFailure,
    SourceLimitExceeded,
    AggregateLimitExceeded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileLoadError {
    kind: ProfileLoadErrorKind,
    location: Option<String>,
    detail: String,
}

impl ProfileLoadError {
    pub fn new(kind: ProfileLoadErrorKind, detail: impl Into<String>) -> Self {
        Self {
            kind,
            location: None,
            detail: detail.into(),
        }
    }

    pub fn at(
        kind: ProfileLoadErrorKind,
        location: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        let location = location.into();
        let looks_absolute = location.starts_with('/')
            || location.starts_with('\\')
            || matches!(location.as_bytes(), [drive, b':', ..] if drive.is_ascii_alphabetic());
        let location = if location.len() > MAX_PROFILE_ERROR_LOCATION_BYTES
            || location.chars().any(char::is_control)
            || looks_absolute
        {
            "profile_location".to_string()
        } else {
            location
        };
        Self {
            kind,
            location: Some(location),
            detail: detail.into(),
        }
    }

    pub fn kind(&self) -> ProfileLoadErrorKind {
        self.kind
    }

    pub fn location(&self) -> Option<&str> {
        self.location.as_deref()
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }
}

impl fmt::Display for ProfileLoadError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(location) = &self.location {
            write!(formatter, "{location}: {}", self.detail)
        } else {
            formatter.write_str(&self.detail)
        }
    }
}

impl std::error::Error for ProfileLoadError {}

#[allow(dead_code)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DefinitionClass {
    Profile,
    StableRoleRegistry,
    SchemaEntry,
    ArtifactKind,
    SemanticCapability,
    SemanticValidator,
    ProjectCondition,
    Vocabulary,
    ContextResolution,
    ContextResolutionPolicy,
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
enum AdmittedDefinitionSource {
    BuiltIn {
        class: DefinitionClass,
        definition_ref: ExactDefinitionRef,
    },
    Repository {
        class: DefinitionClass,
        definition_ref: ExactDefinitionRef,
        normalized_path: String,
        bytes: Vec<u8>,
    },
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub(crate) struct AdmittedProfileSelectionRequest {
    request: ProfileSelectionRequest,
    sources: Vec<AdmittedDefinitionSource>,
    source_bytes: usize,
}

impl AdmittedProfileSelectionRequest {
    #[allow(dead_code)] // Activated by the Task 16 resolver increment.
    pub(crate) fn source_bytes(&self) -> usize {
        self.source_bytes
    }
}

#[allow(dead_code)] // Activated by the Task 16 resolver increment.
pub(crate) fn admit_selection_request(
    repo_root: &Path,
    request: ProfileSelectionRequest,
) -> Result<AdmittedProfileSelectionRequest, ProfileLoadError> {
    if request.profile_sources.len() > MAX_PROFILE_SOURCES {
        return Err(ProfileLoadError::at(
            ProfileLoadErrorKind::ProfileSourceLimitExceeded,
            "profile_sources",
            "profile source count exceeds 64",
        ));
    }

    let collections = source_collections(&request);
    let total_bindings = collections.iter().map(|(_, bindings)| bindings.len()).sum();
    if total_bindings > MAX_DEFINITION_SOURCE_BINDINGS {
        return Err(ProfileLoadError::new(
            ProfileLoadErrorKind::DefinitionBindingLimitExceeded,
            "definition source binding count exceeds 512",
        ));
    }
    if request.allowed_schema_roots.len() > MAX_ALLOWED_SCHEMA_ROOTS {
        return Err(ProfileLoadError::at(
            ProfileLoadErrorKind::SchemaRootLimitExceeded,
            "allowed_schema_roots",
            "allowed schema root count exceeds 32",
        ));
    }

    let mut declared_refs = BTreeSet::new();
    for (_, bindings) in &collections {
        for binding in *bindings {
            if !declared_refs.insert(binding.definition_ref.clone()) {
                return Err(ProfileLoadError::new(
                    ProfileLoadErrorKind::DuplicateSourceBinding,
                    "definition source exact ref appears more than once",
                ));
            }
            if let DefinitionSource::BuiltIn(source_ref) = &binding.source {
                if source_ref != &binding.definition_ref {
                    return Err(ProfileLoadError::new(
                        ProfileLoadErrorKind::SourceIdentityMismatch,
                        "built-in source ref must equal its declared binding ref",
                    ));
                }
            }
        }
    }

    let mut roots = BTreeSet::new();
    for root in &request.allowed_schema_roots {
        validate_normalized_repository_path(root, ProfileLoadErrorKind::InvalidSchemaRoot)?;
        if !roots.insert(root) {
            return Err(ProfileLoadError::at(
                ProfileLoadErrorKind::DuplicateSchemaRoot,
                "allowed_schema_roots",
                "allowed schema root is duplicated",
            ));
        }
    }
    for (_, bindings) in &collections {
        for binding in *bindings {
            if let DefinitionSource::RepositoryPath(path) = &binding.source {
                validate_normalized_repository_path(path, ProfileLoadErrorKind::InvalidSourcePath)?;
            }
        }
    }

    let mut budget = SourceByteBudget::default();
    let mut sources = Vec::with_capacity(total_bindings);
    for (class, bindings) in collections {
        for binding in bindings {
            let admitted = match &binding.source {
                DefinitionSource::BuiltIn(_) => AdmittedDefinitionSource::BuiltIn {
                    class,
                    definition_ref: binding.definition_ref.clone(),
                },
                DefinitionSource::RepositoryPath(path) => {
                    let (normalized_path, bytes) =
                        read_trusted_repo_source(repo_root, path, &mut budget)
                            .map_err(map_source_error)?;
                    if normalized_path != *path {
                        return Err(ProfileLoadError::at(
                            ProfileLoadErrorKind::InvalidSourcePath,
                            "source_path",
                            "repository source path must already be normalized",
                        ));
                    }
                    AdmittedDefinitionSource::Repository {
                        class,
                        definition_ref: binding.definition_ref.clone(),
                        normalized_path,
                        bytes,
                    }
                }
            };
            sources.push(admitted);
        }
    }

    Ok(AdmittedProfileSelectionRequest {
        request,
        sources,
        source_bytes: budget.total_bytes(),
    })
}

#[allow(dead_code)] // Activated by the Task 16 resolver increment.
fn source_collections(
    request: &ProfileSelectionRequest,
) -> [(DefinitionClass, &[DefinitionSourceBinding]); 10] {
    [
        (DefinitionClass::Profile, &request.profile_sources),
        (
            DefinitionClass::StableRoleRegistry,
            &request.stable_role_registry_sources,
        ),
        (DefinitionClass::SchemaEntry, &request.schema_entry_sources),
        (
            DefinitionClass::ArtifactKind,
            &request.artifact_kind_sources,
        ),
        (
            DefinitionClass::SemanticCapability,
            &request.semantic_capability_sources,
        ),
        (
            DefinitionClass::SemanticValidator,
            &request.semantic_validator_sources,
        ),
        (
            DefinitionClass::ProjectCondition,
            &request.project_condition_sources,
        ),
        (DefinitionClass::Vocabulary, &request.vocabulary_sources),
        (
            DefinitionClass::ContextResolution,
            &request.context_resolution_sources,
        ),
        (
            DefinitionClass::ContextResolutionPolicy,
            &request.context_resolution_policy_sources,
        ),
    ]
}

#[allow(dead_code)] // Activated by the Task 16 resolver increment.
fn validate_normalized_repository_path(
    path: &str,
    kind: ProfileLoadErrorKind,
) -> Result<(), ProfileLoadError> {
    let components = path.split('/').collect::<Vec<_>>();
    let invalid = path.trim() != path
        || !(1..=MAX_REPOSITORY_PATH_BYTES).contains(&path.len())
        || !(1..=MAX_REPOSITORY_PATH_COMPONENTS).contains(&components.len())
        || path.starts_with('/')
        || path.ends_with('/')
        || path.contains('\\')
        || path.contains('\0')
        || path.contains("://")
        || components
            .iter()
            .any(|component| component.is_empty() || *component == "." || *component == "..");
    if invalid {
        return Err(ProfileLoadError::at(
            kind,
            "repository_path",
            "repository path must be normalized, repo-relative, and within byte/component limits",
        ));
    }
    Ok(())
}

#[allow(dead_code)] // Activated by the Task 16 resolver increment.
fn map_source_error(error: RegistryLoadError) -> ProfileLoadError {
    let kind = match error.kind() {
        RegistryLoadErrorKind::InvalidSourcePath => ProfileLoadErrorKind::InvalidSourcePath,
        RegistryLoadErrorKind::MissingSource => ProfileLoadErrorKind::MissingSource,
        RegistryLoadErrorKind::SymlinkSource => ProfileLoadErrorKind::SymlinkSource,
        RegistryLoadErrorKind::NonRegularSource => ProfileLoadErrorKind::NonRegularSource,
        RegistryLoadErrorKind::SourceReadFailure => ProfileLoadErrorKind::SourceReadFailure,
        RegistryLoadErrorKind::SourceLimitExceeded => ProfileLoadErrorKind::SourceLimitExceeded,
        RegistryLoadErrorKind::AggregateLimitExceeded => {
            ProfileLoadErrorKind::AggregateLimitExceeded
        }
        _ => ProfileLoadErrorKind::SourceReadFailure,
    };
    if let Some(location) = error.location() {
        ProfileLoadError::at(kind, location, error.detail())
    } else {
        ProfileLoadError::new(kind, error.detail())
    }
}

#[cfg(test)]
mod request_admission_tests {
    use super::{
        admit_selection_request, DefinitionSource, DefinitionSourceBinding, ProfileLoadErrorKind,
        ProfileSelectionRequest, SymbolicId,
    };
    use crate::{ExactDefinitionRef, MAX_SOURCE_DOCUMENT_BYTES, MAX_TOTAL_SOURCE_BYTES};

    fn exact_ref(index: usize) -> ExactDefinitionRef {
        ExactDefinitionRef::parse(&format!("example.profile.source-{index}@1.0.0")).unwrap()
    }

    fn builtin_binding(index: usize) -> DefinitionSourceBinding {
        let exact_ref = exact_ref(index);
        DefinitionSourceBinding {
            definition_ref: exact_ref.clone(),
            source: DefinitionSource::BuiltIn(exact_ref),
        }
    }

    fn empty_request() -> ProfileSelectionRequest {
        ProfileSelectionRequest {
            selected_profile_ref: exact_ref(0),
            profile_sources: Vec::new(),
            stable_role_registry_sources: Vec::new(),
            schema_entry_sources: Vec::new(),
            artifact_kind_sources: Vec::new(),
            semantic_capability_sources: Vec::new(),
            semantic_validator_sources: Vec::new(),
            project_condition_sources: Vec::new(),
            vocabulary_sources: Vec::new(),
            context_resolution_sources: Vec::new(),
            context_resolution_policy_sources: Vec::new(),
            allowed_schema_roots: Vec::new(),
        }
    }

    #[test]
    fn symbolic_ids_accept_internal_underscores_and_the_exact_boundary() {
        for value in ["a", "constitutional_root", &format!("a{}", "0".repeat(63))] {
            assert_eq!(SymbolicId::parse(value).unwrap().as_str(), value);
        }
    }

    #[test]
    fn symbolic_ids_refuse_every_unfrozen_repair_or_shape() {
        for value in [
            "",
            &format!("a{}", "0".repeat(64)),
            "Upper",
            "has-hyphen",
            "has.dot",
            "café",
            "has\ncontrol",
            "_leading",
            "trailing_",
            "double__underscore",
            " trim",
        ] {
            let error = SymbolicId::parse(value).expect_err(value);
            assert_eq!(error.kind(), ProfileLoadErrorKind::InvalidSymbolicId);
        }
    }

    #[test]
    fn request_counts_and_declared_source_identity_fail_before_reads() {
        let repo = tempfile::tempdir().unwrap();

        let mut admitted = empty_request();
        admitted.profile_sources = (0..64).map(builtin_binding).collect();
        admitted.stable_role_registry_sources = (64..512).map(builtin_binding).collect();
        admitted.allowed_schema_roots = (0..32).map(|index| format!("schemas/{index}")).collect();
        admit_selection_request(repo.path(), admitted).expect("N boundaries admit");

        let mut too_many_profiles = empty_request();
        too_many_profiles.profile_sources = (0..65).map(builtin_binding).collect();
        assert_eq!(
            admit_selection_request(repo.path(), too_many_profiles)
                .unwrap_err()
                .kind(),
            ProfileLoadErrorKind::ProfileSourceLimitExceeded
        );

        let mut too_many_bindings = empty_request();
        too_many_bindings.profile_sources = (0..64).map(builtin_binding).collect();
        too_many_bindings.stable_role_registry_sources = (64..513).map(builtin_binding).collect();
        assert_eq!(
            admit_selection_request(repo.path(), too_many_bindings)
                .unwrap_err()
                .kind(),
            ProfileLoadErrorKind::DefinitionBindingLimitExceeded
        );

        let mut too_many_roots = empty_request();
        too_many_roots.allowed_schema_roots =
            (0..33).map(|index| format!("schemas/{index}")).collect();
        assert_eq!(
            admit_selection_request(repo.path(), too_many_roots)
                .unwrap_err()
                .kind(),
            ProfileLoadErrorKind::SchemaRootLimitExceeded
        );

        let mut duplicate = empty_request();
        duplicate.profile_sources.push(builtin_binding(1));
        duplicate.artifact_kind_sources.push(builtin_binding(1));
        assert_eq!(
            admit_selection_request(repo.path(), duplicate)
                .unwrap_err()
                .kind(),
            ProfileLoadErrorKind::DuplicateSourceBinding
        );

        let mut mismatched = empty_request();
        mismatched.profile_sources.push(DefinitionSourceBinding {
            definition_ref: exact_ref(1),
            source: DefinitionSource::BuiltIn(exact_ref(2)),
        });
        assert_eq!(
            admit_selection_request(repo.path(), mismatched)
                .unwrap_err()
                .kind(),
            ProfileLoadErrorKind::SourceIdentityMismatch
        );
    }

    #[test]
    fn paths_and_roots_enforce_exact_bytes_components_and_normalization() {
        let repo = tempfile::tempdir().unwrap();
        let path_1024 = format!("{}/{}", vec!["a".repeat(15); 63].join("/"), "b".repeat(16));
        assert_eq!(path_1024.len(), 1024);
        let path_1025 = format!("{path_1024}x");
        let components_65 = vec!["a"; 65].join("/");

        for path in [&path_1024, "missing.yaml"] {
            let mut request = empty_request();
            request.profile_sources.push(DefinitionSourceBinding {
                definition_ref: exact_ref(1),
                source: DefinitionSource::RepositoryPath(path.to_string()),
            });
            assert_eq!(
                admit_selection_request(repo.path(), request)
                    .unwrap_err()
                    .kind(),
                ProfileLoadErrorKind::MissingSource
            );
        }

        for path in [
            path_1025.as_str(),
            components_65.as_str(),
            " leading.yaml",
            "trailing.yaml ",
            "a//b.yaml",
            "a/./b.yaml",
            "a/../b.yaml",
            "a\\b.yaml",
            "/absolute.yaml",
            "file://source.yaml",
        ] {
            let mut request = empty_request();
            request.profile_sources.push(DefinitionSourceBinding {
                definition_ref: exact_ref(1),
                source: DefinitionSource::RepositoryPath(path.to_string()),
            });
            assert_eq!(
                admit_selection_request(repo.path(), request)
                    .unwrap_err()
                    .kind(),
                ProfileLoadErrorKind::InvalidSourcePath,
                "{path}"
            );
        }

        let mut duplicate_root = empty_request();
        duplicate_root.allowed_schema_roots = vec!["schemas".into(), "schemas".into()];
        assert_eq!(
            admit_selection_request(repo.path(), duplicate_root)
                .unwrap_err()
                .kind(),
            ProfileLoadErrorKind::DuplicateSchemaRoot
        );
    }

    #[test]
    fn repository_sources_stop_at_exact_per_source_and_aggregate_sentinels() {
        let repo = tempfile::tempdir().unwrap();
        let mut exact = empty_request();
        for index in 0..8 {
            let path = format!("sources/{index}.yaml");
            std::fs::create_dir_all(repo.path().join("sources")).unwrap();
            std::fs::write(
                repo.path().join(&path),
                vec![b'a'; MAX_SOURCE_DOCUMENT_BYTES],
            )
            .unwrap();
            exact.profile_sources.push(DefinitionSourceBinding {
                definition_ref: exact_ref(index),
                source: DefinitionSource::RepositoryPath(path),
            });
        }
        let admitted = admit_selection_request(repo.path(), exact).expect("8 MiB admits");
        assert_eq!(admitted.source_bytes(), MAX_TOTAL_SOURCE_BYTES);

        let per_source = tempfile::tempdir().unwrap();
        std::fs::write(
            per_source.path().join("oversize.yaml"),
            vec![b'a'; MAX_SOURCE_DOCUMENT_BYTES + 1],
        )
        .unwrap();
        let mut over = empty_request();
        over.profile_sources.push(DefinitionSourceBinding {
            definition_ref: exact_ref(1),
            source: DefinitionSource::RepositoryPath("oversize.yaml".into()),
        });
        assert_eq!(
            admit_selection_request(per_source.path(), over)
                .unwrap_err()
                .kind(),
            ProfileLoadErrorKind::SourceLimitExceeded
        );

        std::fs::write(repo.path().join("sources/extra.yaml"), b"x").unwrap();
        let mut aggregate = empty_request();
        for index in 0..8 {
            aggregate.profile_sources.push(DefinitionSourceBinding {
                definition_ref: exact_ref(index),
                source: DefinitionSource::RepositoryPath(format!("sources/{index}.yaml")),
            });
        }
        aggregate.profile_sources.push(DefinitionSourceBinding {
            definition_ref: exact_ref(20),
            source: DefinitionSource::RepositoryPath("sources/extra.yaml".into()),
        });
        assert_eq!(
            admit_selection_request(repo.path(), aggregate)
                .unwrap_err()
                .kind(),
            ProfileLoadErrorKind::AggregateLimitExceeded
        );
    }
}
