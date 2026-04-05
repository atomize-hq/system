pub mod decision_log;
pub mod canonical_artifacts;
pub mod error;
pub mod freshness;
pub mod packet_result;
pub mod refusal;

pub use canonical_artifacts::{
    ArtifactIngestError, ArtifactPresence, CanonicalArtifact, CanonicalArtifactIdentity,
    CanonicalArtifactKind, CanonicalArtifacts,
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

pub fn workspace_contract_version() -> &'static str {
    "C-02"
}
