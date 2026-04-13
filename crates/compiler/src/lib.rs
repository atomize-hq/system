pub mod artifact_manifest;
pub mod blocker;
pub mod budget;
pub mod canonical_artifacts;
pub mod decision_log;
pub mod error;
pub mod freshness;
pub mod packet_result;
pub mod pipeline;
pub mod pipeline_route;
pub mod refusal;
pub mod rendering;
pub mod resolver;
pub mod route_state;

pub use artifact_manifest::{
    ArtifactManifest, ManifestError, ManifestInputs, ManifestVersion, SchemaVersion,
};
pub use blocker::{blocker_category_priority, Blocker, BlockerCategory};
pub use budget::{
    BudgetDisposition, BudgetOutcome, BudgetPolicy, BudgetReason,
    NextSafeAction as BudgetNextSafeAction,
};
pub use canonical_artifacts::{
    ArtifactIngestError, ArtifactIngestIssue, ArtifactIngestIssueKind, ArtifactPresence,
    CanonicalArtifact, CanonicalArtifactIdentity, CanonicalArtifactKind, CanonicalArtifacts,
    SystemRootStatus,
};
pub use decision_log::DecisionLog;
pub use error::CompilerError;
pub use freshness::{
    compute_freshness, FreshnessIssue, FreshnessIssueKind, FreshnessStatus, FreshnessTruth,
    InheritedDependency, OverrideTarget, OverrideWithRationale, C03_SCHEMA_VERSION,
    MANIFEST_GENERATION_VERSION,
};
pub use packet_result::PacketResult;
pub use pipeline::{
    load_pipeline_catalog, load_pipeline_catalog_metadata, load_pipeline_definition,
    render_pipeline_list, render_pipeline_show, resolve_pipeline_only_selector,
    resolve_pipeline_selector, supported_route_state_variables, ActivationClause,
    ActivationConditionSet, ActivationOperator, ActivationValidationError, PipelineBody,
    PipelineCatalog, PipelineCatalogEntry, PipelineCatalogError, PipelineCatalogStageEntry,
    PipelineDefaults, PipelineDefinition, PipelineHeader, PipelineLoadError, PipelineLookupError,
    PipelineSelection, PipelineStage, PipelineValidationError, StageActivation, StageCatalogEntry,
    StageFileValidationError,
};
pub use pipeline_route::{
    resolve_pipeline_route, ResolvedPipelineRoute, ResolvedPipelineStage, RouteEvaluationError,
    RouteStageReason, RouteStageStatus, RouteVariables,
};
pub use refusal::{NextSafeAction, Refusal, RefusalCategory, SubjectRef};
pub use rendering::{
    build_output_model, render_blocker_category, render_inspect, render_json, render_markdown,
    render_next_safe_action_value, render_subject_ref, RenderError, RenderOutputModel,
    RenderSurface,
};
pub use resolver::{
    resolve, PacketSelection, PacketSelectionStatus, ResolveRequest, ResolverResult,
};
pub use route_state::{
    load_route_state, load_route_state_with_supported_variables, set_route_state, RouteState,
    RouteStateAuditEntry, RouteStateMutation, RouteStateMutationOutcome, RouteStateMutationRefusal,
    RouteStateReadError, RouteStateRefs, RouteStateRun, RouteStateStoreError, RouteStateValue,
    ROUTE_STATE_AUDIT_LIMIT, ROUTE_STATE_SCHEMA_VERSION,
};

pub fn workspace_contract_version() -> &'static str {
    "C-02"
}
