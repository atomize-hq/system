use crate::CanonicalArtifactKind;
use serde::Serialize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RefusalCategory {
    NonCanonicalInputAttempt,
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    RequiredArtifactStarterTemplate,
    RequiredArtifactInvalid,
    ArtifactReadError,
    FreshnessInvalid,
    BudgetRefused,
    UnsupportedRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(untagged)]
pub enum SubjectRef {
    CanonicalArtifact {
        kind: CanonicalArtifactKind,
        canonical_repo_relative_path: &'static str,
    },
    InheritedDependency {
        dependency_id: String,
        version: Option<String>,
    },
    Policy {
        policy_id: &'static str,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum NextSafeAction {
    RunSetup,
    RunSetupInit,
    RunSetupRefresh,
    RunAuthorCharter,
    RunAuthorProjectContext,
    RunAuthorEnvironmentInventory,
    CreateSystemRoot {
        canonical_repo_relative_path: &'static str,
    },
    EnsureSystemRootIsDirectory {
        canonical_repo_relative_path: &'static str,
    },
    RemoveSystemRootSymlink {
        canonical_repo_relative_path: &'static str,
    },
    CreateCanonicalArtifact {
        canonical_repo_relative_path: &'static str,
    },
    FillCanonicalArtifact {
        canonical_repo_relative_path: &'static str,
    },
    ReduceCanonicalArtifactSize {
        canonical_repo_relative_path: &'static str,
    },
    RunGenerate {
        packet_id: &'static str,
    },
    RunDoctor,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Refusal {
    pub category: RefusalCategory,
    pub summary: String,
    pub broken_subject: SubjectRef,
    pub next_safe_action: NextSafeAction,
}
