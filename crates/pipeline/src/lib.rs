pub mod declarative_roots;
mod layout;
pub mod pipeline;
pub mod pipeline_route;
mod repo_file_access;
pub mod route_state;

pub use pipeline::{
    load_pipeline_catalog, load_pipeline_catalog_metadata, load_pipeline_definition,
    load_pipeline_selection_metadata, load_stage_compile_definition, render_pipeline_list,
    render_pipeline_show, resolve_pipeline_only_selector, resolve_pipeline_selector,
    supported_route_state_variables, ActivationClause, ActivationConditionSet, ActivationOperator,
    ActivationValidationError, CompileStageDefinition, CompileStageGating, CompileStageInput,
    CompileStageInputs, CompileStageLoadError, CompileStageOutput, CompileStageOutputs,
    CompileStageVariable, PipelineBody, PipelineCatalog, PipelineCatalogEntry,
    PipelineCatalogError, PipelineCatalogStageEntry, PipelineDefaults, PipelineDefinition,
    PipelineHeader, PipelineLoadError, PipelineLookupError, PipelineMetadataSelectionError,
    PipelineSelection, PipelineStage, PipelineValidationError, StageActivation, StageCatalogEntry,
    StageFileValidationError,
};
pub use pipeline_route::{
    resolve_pipeline_route, ResolvedPipelineRoute, ResolvedPipelineStage, RouteEvaluationError,
    RouteStageReason, RouteStageStatus, RouteVariables,
};
pub use route_state::{
    build_route_basis, effective_route_basis_run, load_route_state,
    load_route_state_with_supported_variables, persist_route_basis, set_route_state, RouteBasis,
    RouteBasisActivationOperator, RouteBasisBuildError, RouteBasisPersistOutcome,
    RouteBasisPersistRefusal, RouteBasisProfilePack, RouteBasisResolvedStage, RouteBasisRunner,
    RouteBasisStageReason, RouteBasisStageStatus, RouteState, RouteStateAuditEntry,
    RouteStateMutation, RouteStateMutationOutcome, RouteStateMutationRefusal, RouteStateReadError,
    RouteStateRefs, RouteStateRun, RouteStateStoreError, RouteStateValue,
    TrustedPipelineSessionRefusal, ROUTE_BASIS_REPO_ROOT_SENTINEL, ROUTE_BASIS_SCHEMA_VERSION,
    ROUTE_STATE_AUDIT_LIMIT, ROUTE_STATE_SCHEMA_VERSION,
};

pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
