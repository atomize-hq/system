use crate::{CanonicalArtifactKind, FreshnessIssueKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusalCategory {
    NonCanonicalInputAttempt,
    SystemRootMissing,
    SystemRootNotDir,
    SystemRootSymlinkNotAllowed,
    RequiredArtifactMissing,
    RequiredArtifactEmpty,
    ArtifactReadError,
    FreshnessInvalid,
    BudgetRefused,
    UnsupportedRequest,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubjectRef {
    CanonicalArtifact {
        kind: CanonicalArtifactKind,
        canonical_repo_relative_path: &'static str,
    },
    FreshnessIssue { kind: FreshnessIssueKind },
    Policy { policy_id: &'static str },
    SystemRoot { canonical_repo_relative_path: &'static str },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NextSafeAction {
    CreateSystemRoot { canonical_repo_relative_path: &'static str },
    EnsureSystemRootIsDirectory { canonical_repo_relative_path: &'static str },
    RemoveSystemRootSymlink { canonical_repo_relative_path: &'static str },
    CreateCanonicalArtifact { canonical_repo_relative_path: &'static str },
    FillCanonicalArtifact { canonical_repo_relative_path: &'static str },
    ReduceCanonicalArtifactSize { canonical_repo_relative_path: &'static str },
    RunDoctor,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    pub category: RefusalCategory,
    pub summary: String,
    pub broken_subject: SubjectRef,
    pub next_safe_action: NextSafeAction,
}
