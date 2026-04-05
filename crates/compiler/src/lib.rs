pub mod decision_log;
pub mod artifact_manifest;
pub mod budget;
pub mod canonical_artifacts;
pub mod error;
pub mod freshness;
pub mod packet_result;
pub mod refusal;
pub mod resolver;

pub use artifact_manifest::{
    ArtifactManifest, ManifestError, ManifestInputs, ManifestVersion, SchemaVersion,
};
pub use budget::{BudgetDisposition, BudgetOutcome, BudgetPolicy, BudgetReason, NextSafeAction};
pub use canonical_artifacts::{
    ArtifactIngestError, ArtifactPresence, CanonicalArtifact, CanonicalArtifactIdentity,
    CanonicalArtifactKind, CanonicalArtifacts, SystemRootStatus,
};
pub use decision_log::DecisionLog;
pub use error::CompilerError;
pub use freshness::{
    compute_freshness, FreshnessIssue, FreshnessIssueKind, FreshnessStatus, FreshnessTruth,
    InheritedDependency, OverrideTarget, OverrideWithRationale, C03_SCHEMA_VERSION,
    MANIFEST_GENERATION_VERSION,
};
pub use packet_result::PacketResult;
pub use refusal::RefusalPlaceholder;
pub use resolver::{resolve, PacketSelection, PacketSelectionStatus, ResolverResult, ResolveRequest};

pub fn workspace_contract_version() -> &'static str {
    "C-02"
}
