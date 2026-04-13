use crate::pipeline::{
    load_pipeline_catalog, load_stage_compile_definition, resolve_pipeline_only_selector,
    supported_route_state_variables, CompileStageDefinition, CompileStageInput,
    CompileStageLoadError, CompileStageVariable, PipelineCatalogEntry, PipelineDefinition,
};
use crate::route_state::{
    load_route_state_with_supported_variables, RouteBasis, RouteBasisStageReason,
    RouteBasisStageStatus, RouteState, RouteStateReadError,
};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fmt;
use std::path::{Component, Path, PathBuf};

const SUPPORTED_PIPELINE_ID: &str = "pipeline.foundation_inputs";
const SUPPORTED_STAGE_ID: &str = "stage.10_feature_spec";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileResult {
    pub target: PipelineCompileTarget,
    pub basis: RouteBasis,
    pub variables: Vec<PipelineCompileVariable>,
    pub documents: Vec<PipelineCompileDocument>,
    pub outputs: Vec<PipelineCompileOutput>,
    pub gating: PipelineCompileGatingSummary,
    pub stage_body: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileTarget {
    pub pipeline_id: String,
    pub stage_id: String,
    pub stage_file: String,
    pub title: String,
    pub description: String,
    pub work_level: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileVariable {
    pub name: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileDocument {
    pub kind: PipelineCompileDocumentKind,
    pub path: String,
    pub required: bool,
    pub status: PipelineCompileDocumentStatus,
    pub content: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileDocumentKind {
    Include,
    Runner,
    Profile,
    Library,
    Artifact,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileDocumentStatus {
    Present,
    MissingOptional,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileOutput {
    pub kind: PipelineCompileOutputKind,
    pub path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileOutputKind {
    Artifact,
    RepoFile,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileGatingSummary {
    pub mode: Option<String>,
    pub fail_on: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PipelineCompileRefusal {
    pub classification: PipelineCompileRefusalClassification,
    pub summary: String,
    pub pipeline_id: Option<String>,
    pub stage_id: Option<String>,
    pub recovery: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineCompileRefusalClassification {
    UnsupportedTarget,
    InvalidDefinition,
    InvalidState,
    MissingRouteBasis,
    MalformedRouteBasis,
    StaleRouteBasis,
    InactiveStage,
    MissingRequiredInput,
    EmptyRequiredInput,
}

pub fn compile_pipeline_stage(
    repo_root: impl AsRef<Path>,
    pipeline_selector: &str,
    stage_selector: &str,
) -> Result<PipelineCompileResult, PipelineCompileRefusal> {
    let repo_root = repo_root.as_ref();
    let catalog = load_pipeline_catalog(repo_root).map_err(|err| PipelineCompileRefusal {
        classification: PipelineCompileRefusalClassification::InvalidDefinition,
        summary: format!("failed to load pipeline catalog: {err}"),
        pipeline_id: None,
        stage_id: None,
        recovery: "fix the pipeline/stage definitions and retry `pipeline compile`".to_string(),
    })?;

    let pipeline = resolve_pipeline_only_selector(&catalog, pipeline_selector).map_err(|err| {
        PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::UnsupportedTarget,
            summary: err.to_string(),
            pipeline_id: None,
            stage_id: None,
            recovery: "retry with the canonical pipeline id `pipeline.foundation_inputs`"
                .to_string(),
        }
    })?;
    let resolved_stage_id =
        resolve_stage_selector(&pipeline, stage_selector).map_err(|summary| {
            PipelineCompileRefusal {
                classification: PipelineCompileRefusalClassification::UnsupportedTarget,
                summary,
                pipeline_id: Some(pipeline.definition.header.id.clone()),
                stage_id: Some(stage_selector.trim().to_string()),
                recovery: "retry with the canonical stage id `stage.10_feature_spec`".to_string(),
            }
        })?;

    if pipeline.definition.header.id != SUPPORTED_PIPELINE_ID
        || resolved_stage_id != SUPPORTED_STAGE_ID
    {
        return Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::UnsupportedTarget,
            summary: format!(
                "M2 compile currently supports only `{SUPPORTED_PIPELINE_ID}` + `{SUPPORTED_STAGE_ID}`"
            ),
            pipeline_id: Some(pipeline.definition.header.id.clone()),
            stage_id: Some(resolved_stage_id),
            recovery: format!(
                "retry with `pipeline compile --id {SUPPORTED_PIPELINE_ID} --stage {SUPPORTED_STAGE_ID}`"
            ),
        });
    }

    let stage_id = SUPPORTED_STAGE_ID.to_string();
    let supported_variables = supported_route_state_variables(&pipeline.definition);
    let state = load_route_state_with_supported_variables(
        repo_root,
        &pipeline.definition.header.id,
        &supported_variables,
    )
    .map_err(|err| classify_state_read_refusal(err, &pipeline.definition.header.id, &stage_id))?;
    let route_basis = state
        .route_basis
        .clone()
        .ok_or_else(|| PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::MissingRouteBasis,
            summary: "persisted route_basis is missing for the selected pipeline".to_string(),
            pipeline_id: Some(pipeline.definition.header.id.clone()),
            stage_id: Some(stage_id.clone()),
            recovery: "re-run `pipeline resolve` and retry `pipeline compile`".to_string(),
        })?;

    check_route_basis_freshness(
        repo_root,
        &pipeline.definition,
        &state,
        &route_basis,
        &stage_id,
    )?;

    let basis_stage = route_basis
        .route
        .iter()
        .find(|stage| stage.stage_id == stage_id)
        .ok_or_else(|| PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::UnsupportedTarget,
            summary: format!(
                "selected stage `{stage_id}` is absent from the persisted resolved route"
            ),
            pipeline_id: Some(pipeline.definition.header.id.clone()),
            stage_id: Some(stage_id.clone()),
            recovery: "re-run `pipeline resolve` and confirm the selected stage is declared in the pipeline".to_string(),
        })?;

    if basis_stage.status != RouteBasisStageStatus::Active {
        return Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InactiveStage,
            summary: format!(
                "selected stage `{stage_id}` is not active in the persisted route: {}",
                render_route_basis_reason(basis_stage.reason.as_ref())
            ),
            pipeline_id: Some(pipeline.definition.header.id.clone()),
            stage_id: Some(stage_id.clone()),
            recovery: "re-run `pipeline resolve`, adjust route state if needed, and retry `pipeline compile`".to_string(),
        });
    }

    let stage_definition =
        load_stage_compile_definition(repo_root, &pipeline.definition, &stage_id).map_err(
            |err| classify_compile_stage_error(err, &pipeline.definition.header.id, &stage_id),
        )?;
    let work_level = stage_definition
        .work_level
        .clone()
        .filter(|value| !value.trim().is_empty())
        .unwrap_or_else(|| "L1".to_string());
    let variable_values =
        resolve_compile_variables(&route_basis, &stage_definition, repo_root, &work_level);
    let declared_variables = render_declared_variables(
        &stage_definition.inputs.variables,
        &variable_values,
        &work_level,
    );
    let documents =
        assemble_documents(repo_root, &stage_definition, &route_basis, &variable_values)?;
    let outputs = render_outputs(&stage_definition, &variable_values);

    Ok(PipelineCompileResult {
        target: PipelineCompileTarget {
            pipeline_id: pipeline.definition.header.id.clone(),
            stage_id,
            stage_file: stage_definition.stage.file.clone(),
            title: stage_definition.title.clone(),
            description: stage_definition.description.clone(),
            work_level,
            tags: stage_definition.tags.clone(),
        },
        basis: route_basis,
        variables: declared_variables,
        documents,
        outputs,
        gating: PipelineCompileGatingSummary {
            mode: stage_definition.gating.mode.clone(),
            fail_on: stage_definition.gating.fail_on.clone(),
            notes: stage_definition.gating.notes.clone(),
        },
        stage_body: stage_definition.body.clone(),
    })
}

pub fn render_pipeline_compile_payload(result: &PipelineCompileResult) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "# {} - {}\n",
        result.target.stage_id, result.target.title
    ));
    if !result.target.description.trim().is_empty() {
        out.push('\n');
        out.push_str(result.target.description.trim());
        out.push('\n');
    }

    out.push_str("\n## Run Variables\n");
    for variable in &result.variables {
        out.push_str(&format!(
            "- {}: {}\n",
            variable.name,
            variable.value.as_deref().unwrap_or("<unset>")
        ));
    }

    render_document_section(
        &mut out,
        "Selected Runner",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Runner),
    );
    render_document_section(
        &mut out,
        "Selected Profile",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Profile),
    );
    render_document_section(
        &mut out,
        "Includes",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Include),
    );
    render_document_section(
        &mut out,
        "Library Inputs",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Library),
    );
    render_document_section(
        &mut out,
        "Artifact Inputs",
        result
            .documents
            .iter()
            .filter(|doc| doc.kind == PipelineCompileDocumentKind::Artifact),
    );

    out.push_str("\n## Outputs\n");
    render_output_section(
        &mut out,
        "Artifacts",
        result
            .outputs
            .iter()
            .filter(|output| output.kind == PipelineCompileOutputKind::Artifact),
    );
    render_output_section(
        &mut out,
        "Repo Files",
        result
            .outputs
            .iter()
            .filter(|output| output.kind == PipelineCompileOutputKind::RepoFile),
    );

    if result.gating.mode.is_some()
        || !result.gating.fail_on.is_empty()
        || !result.gating.notes.is_empty()
    {
        out.push_str("\n## Gating\n");
        out.push_str(&format!(
            "mode: {}\n",
            result.gating.mode.as_deref().unwrap_or("<unset>")
        ));
        if result.gating.fail_on.is_empty() {
            out.push_str("fail_on:\n- <none>\n");
        } else {
            out.push_str("fail_on:\n");
            for item in &result.gating.fail_on {
                out.push_str(&format!("- {item}\n"));
            }
        }
        if !result.gating.notes.is_empty() {
            out.push_str("notes:\n");
            for note in &result.gating.notes {
                out.push_str(&format!("- {note}\n"));
            }
        }
    }

    if let Some(stage_body) = &result.stage_body {
        out.push_str("\n## Stage Body\n");
        out.push_str(stage_body.trim());
        out.push('\n');
    }

    normalize_rendered_output(&out)
}

pub fn render_pipeline_compile_explain(result: &PipelineCompileResult) -> String {
    let mut out = String::new();
    out.push_str("OUTCOME: COMPILED\n");
    out.push_str("TARGET:\n");
    out.push_str(&format!("  pipeline = {}\n", result.target.pipeline_id));
    out.push_str(&format!("  stage = {}\n", result.target.stage_id));
    out.push_str(&format!("  stage_file = {}\n", result.target.stage_file));
    out.push_str(&format!("  work_level = {}\n", result.target.work_level));
    out.push_str("ROUTE BASIS:\n");
    out.push_str(&format!(
        "  schema_version = {}\n",
        result.basis.schema_version
    ));
    out.push_str(&format!(
        "  state_revision = {}\n",
        result.basis.state_revision
    ));
    out.push_str(&format!(
        "  pipeline_file = {}\n",
        result.basis.pipeline_file
    ));
    out.push_str(&format!(
        "  pipeline_file_sha256 = {}\n",
        result.basis.pipeline_file_sha256
    ));
    out.push_str("  routing:\n");
    if result.basis.routing.is_empty() {
        out.push_str("    <empty>\n");
    } else {
        for (name, value) in &result.basis.routing {
            out.push_str(&format!("    {} = {}\n", name, value));
        }
    }
    out.push_str("  refs:\n");
    render_optional_basis_value(
        &mut out,
        "charter_ref",
        result.basis.refs.charter_ref.as_deref(),
    );
    render_optional_basis_value(
        &mut out,
        "project_context_ref",
        result.basis.refs.project_context_ref.as_deref(),
    );
    out.push_str("  run:\n");
    render_optional_basis_value(&mut out, "runner", result.basis.run.runner.as_deref());
    render_optional_basis_value(&mut out, "profile", result.basis.run.profile.as_deref());
    render_optional_basis_value(&mut out, "repo_root", result.basis.run.repo_root.as_deref());
    out.push_str("  runner:\n");
    out.push_str(&format!("    id = {}\n", result.basis.runner.id));
    out.push_str(&format!("    file = {}\n", result.basis.runner.file));
    out.push_str(&format!(
        "    file_sha256 = {}\n",
        result.basis.runner.file_sha256
    ));
    out.push_str("  profile:\n");
    out.push_str(&format!("    id = {}\n", result.basis.profile.id));
    out.push_str(&format!(
        "    profile.yaml.sha256 = {}\n",
        result.basis.profile.profile_yaml_sha256
    ));
    out.push_str(&format!(
        "    commands.yaml.sha256 = {}\n",
        result.basis.profile.commands_yaml_sha256
    ));
    out.push_str(&format!(
        "    conventions.md.sha256 = {}\n",
        result.basis.profile.conventions_md_sha256
    ));
    out.push_str("ROUTE SNAPSHOT:\n");
    for (index, stage) in result.basis.route.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {} | {}\n",
            index + 1,
            stage.stage_id,
            render_route_basis_status(stage.status),
            stage.file
        ));
        if let Some(reason) = &stage.reason {
            out.push_str(&format!(
                "     REASON: {}\n",
                render_route_basis_reason(Some(reason))
            ));
        }
    }
    out.push_str("VARIABLES:\n");
    for variable in &result.variables {
        out.push_str(&format!(
            "  {} = {}\n",
            variable.name,
            variable.value.as_deref().unwrap_or("<unset>")
        ));
    }
    out.push_str("DOCUMENTS:\n");
    for (index, document) in result.documents.iter().enumerate() {
        out.push_str(&format!(
            "  {}. {} | {} | required={} | status={}\n",
            index + 1,
            render_document_kind(document.kind),
            document.path,
            document.required,
            render_document_status(document.status)
        ));
    }
    out.push_str("OUTPUTS:\n");
    for output in &result.outputs {
        out.push_str(&format!(
            "  {} | {}\n",
            render_output_kind(output.kind),
            output.path
        ));
    }
    out.push_str("GATING:\n");
    out.push_str(&format!(
        "  mode = {}\n",
        result.gating.mode.as_deref().unwrap_or("<unset>")
    ));
    if result.gating.fail_on.is_empty() {
        out.push_str("  fail_on = <none>\n");
    } else {
        out.push_str(&format!(
            "  fail_on = {}\n",
            result.gating.fail_on.join(", ")
        ));
    }
    if result.gating.notes.is_empty() {
        out.push_str("  notes = <none>\n");
    } else {
        for note in &result.gating.notes {
            out.push_str(&format!("  note = {note}\n"));
        }
    }
    out.push_str(&format!(
        "STAGE BODY: {}\n",
        if result.stage_body.is_some() {
            "present"
        } else {
            "absent"
        }
    ));

    normalize_rendered_output(&out)
}

fn classify_state_read_refusal(
    err: RouteStateReadError,
    pipeline_id: &str,
    stage_id: &str,
) -> PipelineCompileRefusal {
    match err {
        RouteStateReadError::MalformedState { reason, .. } if reason.contains("route_basis") => {
            PipelineCompileRefusal {
                classification: PipelineCompileRefusalClassification::MalformedRouteBasis,
                summary: format!("persisted route_basis is malformed: {reason}"),
                pipeline_id: Some(pipeline_id.to_string()),
                stage_id: Some(stage_id.to_string()),
                recovery: "re-run `pipeline resolve` and retry `pipeline compile`".to_string(),
            }
        }
        other => PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidState,
            summary: other.to_string(),
            pipeline_id: Some(pipeline_id.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the persisted route state and retry `pipeline compile`".to_string(),
        },
    }
}

fn classify_compile_stage_error(
    err: CompileStageLoadError,
    pipeline_id: &str,
    stage_id: &str,
) -> PipelineCompileRefusal {
    let classification = match err {
        CompileStageLoadError::StageNotDeclared { .. } => {
            PipelineCompileRefusalClassification::UnsupportedTarget
        }
        _ => PipelineCompileRefusalClassification::InvalidDefinition,
    };

    PipelineCompileRefusal {
        classification,
        summary: err.to_string(),
        pipeline_id: Some(pipeline_id.to_string()),
        stage_id: Some(stage_id.to_string()),
        recovery: "fix the selected stage definition and retry `pipeline compile`".to_string(),
    }
}

fn resolve_stage_selector(
    pipeline: &PipelineCatalogEntry,
    selector: &str,
) -> Result<String, String> {
    let selector = selector.trim();
    if selector.is_empty() {
        return Err("stage selector must not be empty".to_string());
    }
    if selector_is_path_like(selector) {
        return Err(format!(
            "unsupported stage selector `{selector}`: raw file paths are evidence only; use a canonical stage id"
        ));
    }

    if pipeline
        .definition
        .declared_stages()
        .iter()
        .any(|stage| stage.id == selector)
    {
        return Ok(selector.to_string());
    }

    let matches = pipeline
        .definition
        .declared_stages()
        .iter()
        .filter(|stage| stage.id.strip_prefix("stage.") == Some(selector))
        .map(|stage| stage.id.clone())
        .collect::<Vec<_>>();

    if matches.len() > 1 {
        return Err(format!(
            "ambiguous stage selector `{selector}` matched multiple canonical ids: {}",
            matches.join(", ")
        ));
    }

    matches
        .into_iter()
        .next()
        .ok_or_else(|| format!("unknown stage selector `{selector}` for the selected pipeline"))
}

fn selector_is_path_like(selector: &str) -> bool {
    let path = Path::new(selector);
    path.is_absolute()
        || path
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
        || selector.contains('/')
}

fn check_route_basis_freshness(
    repo_root: &Path,
    pipeline: &PipelineDefinition,
    state: &RouteState,
    basis: &RouteBasis,
    stage_id: &str,
) -> Result<(), PipelineCompileRefusal> {
    if state.revision != basis.state_revision {
        return Err(stale_basis_refusal(
            &pipeline.header.id,
            stage_id,
            format!(
                "route state revision {} does not match persisted route_basis revision {}",
                state.revision, basis.state_revision
            ),
        ));
    }
    if state.routing != basis.routing || state.refs != basis.refs || state.run != basis.run {
        return Err(stale_basis_refusal(
            &pipeline.header.id,
            stage_id,
            "route_state routing/refs/run no longer match the persisted route_basis".to_string(),
        ));
    }
    if fingerprint_repo_relative_file(repo_root, &pipeline.source_path.to_string_lossy())
        .map_err(|detail| stale_basis_refusal(&pipeline.header.id, stage_id, detail))?
        != basis.pipeline_file_sha256
    {
        return Err(stale_basis_refusal(
            &pipeline.header.id,
            stage_id,
            "pipeline definition fingerprint drifted after the last `pipeline resolve`".to_string(),
        ));
    }

    let stage_basis = basis
        .route
        .iter()
        .find(|stage| stage.stage_id == stage_id)
        .ok_or_else(|| {
            stale_basis_refusal(
                &pipeline.header.id,
                stage_id,
                "selected stage is missing from the persisted route_basis".to_string(),
            )
        })?;
    let current_stage_sha = fingerprint_repo_relative_file(repo_root, &stage_basis.file)
        .map_err(|detail| stale_basis_refusal(&pipeline.header.id, stage_id, detail))?;
    if current_stage_sha != stage_basis.file_sha256 {
        return Err(stale_basis_refusal(
            &pipeline.header.id,
            stage_id,
            format!(
                "stage file `{}` changed after the last `pipeline resolve`",
                stage_basis.file
            ),
        ));
    }

    let current_runner_sha = fingerprint_repo_relative_file(repo_root, &basis.runner.file)
        .map_err(|detail| stale_basis_refusal(&pipeline.header.id, stage_id, detail))?;
    if current_runner_sha != basis.runner.file_sha256 {
        return Err(stale_basis_refusal(
            &pipeline.header.id,
            stage_id,
            format!(
                "runner document `{}` changed after the last `pipeline resolve`",
                basis.runner.file
            ),
        ));
    }

    let profile_yaml = format!("profiles/{}/profile.yaml", basis.profile.id);
    let commands_yaml = format!("profiles/{}/commands.yaml", basis.profile.id);
    let conventions_md = format!("profiles/{}/conventions.md", basis.profile.id);
    if fingerprint_repo_relative_file(repo_root, &profile_yaml)
        .map_err(|detail| stale_basis_refusal(&pipeline.header.id, stage_id, detail))?
        != basis.profile.profile_yaml_sha256
        || fingerprint_repo_relative_file(repo_root, &commands_yaml)
            .map_err(|detail| stale_basis_refusal(&pipeline.header.id, stage_id, detail))?
            != basis.profile.commands_yaml_sha256
        || fingerprint_repo_relative_file(repo_root, &conventions_md)
            .map_err(|detail| stale_basis_refusal(&pipeline.header.id, stage_id, detail))?
            != basis.profile.conventions_md_sha256
    {
        return Err(stale_basis_refusal(
            &pipeline.header.id,
            stage_id,
            "selected profile pack changed after the last `pipeline resolve`".to_string(),
        ));
    }

    Ok(())
}

fn stale_basis_refusal(
    pipeline_id: &str,
    stage_id: &str,
    summary: String,
) -> PipelineCompileRefusal {
    PipelineCompileRefusal {
        classification: PipelineCompileRefusalClassification::StaleRouteBasis,
        summary,
        pipeline_id: Some(pipeline_id.to_string()),
        stage_id: Some(stage_id.to_string()),
        recovery: "re-run `pipeline resolve` and retry `pipeline compile`".to_string(),
    }
}

fn resolve_compile_variables(
    basis: &RouteBasis,
    stage_definition: &CompileStageDefinition,
    repo_root: &Path,
    work_level: &str,
) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    for (name, value) in &basis.routing {
        values.insert(name.clone(), value.to_string());
    }
    if let Some(value) = &basis.refs.charter_ref {
        values.insert("charter_ref".to_string(), value.clone());
    }
    if let Some(value) = &basis.refs.project_context_ref {
        values.insert("project_context_ref".to_string(), value.clone());
    }
    values.insert("runner".to_string(), basis.runner.id.clone());
    values.insert("profile".to_string(), basis.profile.id.clone());
    values.insert(
        "repo_root".to_string(),
        basis
            .run
            .repo_root
            .clone()
            .unwrap_or_else(|| repo_root.to_string_lossy().into_owned()),
    );
    values.insert("work_level".to_string(), work_level.to_string());

    for variable in &stage_definition.inputs.variables {
        values
            .entry(variable.name.clone())
            .or_insert_with(String::new);
    }

    values
}

fn render_declared_variables(
    declared: &[CompileStageVariable],
    values: &BTreeMap<String, String>,
    work_level: &str,
) -> Vec<PipelineCompileVariable> {
    let mut variables = declared
        .iter()
        .map(|variable| PipelineCompileVariable {
            name: variable.name.clone(),
            value: normalize_variable_value(values.get(&variable.name)),
        })
        .collect::<Vec<_>>();

    if !variables
        .iter()
        .any(|variable| variable.name == "work_level")
    {
        variables.push(PipelineCompileVariable {
            name: "work_level".to_string(),
            value: Some(work_level.to_string()),
        });
    }

    variables
}

fn normalize_variable_value(value: Option<&String>) -> Option<String> {
    value
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn assemble_documents(
    repo_root: &Path,
    stage_definition: &CompileStageDefinition,
    basis: &RouteBasis,
    variables: &BTreeMap<String, String>,
) -> Result<Vec<PipelineCompileDocument>, PipelineCompileRefusal> {
    let mut documents = Vec::new();
    let runner_path = basis.runner.file.clone();
    let profile_paths = [
        format!("profiles/{}/profile.yaml", basis.profile.id),
        format!("profiles/{}/commands.yaml", basis.profile.id),
        format!("profiles/{}/conventions.md", basis.profile.id),
    ];

    for include in &stage_definition.includes {
        let path = substitute_variables(include, variables);
        let kind = if path == runner_path {
            PipelineCompileDocumentKind::Runner
        } else if profile_paths.iter().any(|candidate| candidate == &path) {
            PipelineCompileDocumentKind::Profile
        } else {
            PipelineCompileDocumentKind::Include
        };
        documents.push(load_required_document(
            repo_root,
            &stage_definition.id,
            kind,
            &path,
        )?);
    }

    for input in &stage_definition.inputs.library {
        documents.push(load_declared_input(
            repo_root,
            &stage_definition.id,
            PipelineCompileDocumentKind::Library,
            input,
            variables,
        )?);
    }

    for input in &stage_definition.inputs.artifacts {
        documents.push(load_declared_input(
            repo_root,
            &stage_definition.id,
            PipelineCompileDocumentKind::Artifact,
            input,
            variables,
        )?);
    }

    Ok(documents)
}

fn load_required_document(
    repo_root: &Path,
    stage_id: &str,
    kind: PipelineCompileDocumentKind,
    path: &str,
) -> Result<PipelineCompileDocument, PipelineCompileRefusal> {
    match load_repo_relative_document(repo_root, path, true) {
        Ok(content) => Ok(PipelineCompileDocument {
            kind,
            path: path.to_string(),
            required: true,
            status: PipelineCompileDocumentStatus::Present,
            content: Some(content),
        }),
        Err(DocumentLoadError::Missing { .. }) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::MissingRequiredInput,
            summary: format!("required compile-shaping input `{path}` is missing for `{stage_id}`"),
            pipeline_id: Some(SUPPORTED_PIPELINE_ID.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "restore the missing input and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::Empty { .. }) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::EmptyRequiredInput,
            summary: format!("required compile-shaping input `{path}` is empty for `{stage_id}`"),
            pipeline_id: Some(SUPPORTED_PIPELINE_ID.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fill the required input and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::InvalidPath(reason)) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidDefinition,
            summary: format!("compile input path `{path}` is invalid: {reason}"),
            pipeline_id: Some(SUPPORTED_PIPELINE_ID.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the stage definition and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::ReadFailure(err_path, source)) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidState,
            summary: format!(
                "failed to read compile input {}: {source}",
                err_path.display()
            ),
            pipeline_id: Some(SUPPORTED_PIPELINE_ID.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the unreadable input and retry `pipeline compile`".to_string(),
        }),
    }
}

fn load_declared_input(
    repo_root: &Path,
    stage_id: &str,
    kind: PipelineCompileDocumentKind,
    input: &CompileStageInput,
    variables: &BTreeMap<String, String>,
) -> Result<PipelineCompileDocument, PipelineCompileRefusal> {
    let path = substitute_variables(&input.path, variables);
    match load_repo_relative_document(
        repo_root,
        &path,
        matches!(kind, PipelineCompileDocumentKind::Artifact),
    ) {
        Ok(content) => Ok(PipelineCompileDocument {
            kind,
            path,
            required: input.required,
            status: PipelineCompileDocumentStatus::Present,
            content: Some(content),
        }),
        Err(DocumentLoadError::Missing { .. }) if !input.required => Ok(PipelineCompileDocument {
            kind,
            path,
            required: false,
            status: PipelineCompileDocumentStatus::MissingOptional,
            content: None,
        }),
        Err(DocumentLoadError::Empty { .. }) if !input.required => Ok(PipelineCompileDocument {
            kind,
            path,
            required: false,
            status: PipelineCompileDocumentStatus::MissingOptional,
            content: None,
        }),
        Err(DocumentLoadError::Missing { .. }) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::MissingRequiredInput,
            summary: format!("required input `{path}` is missing for `{stage_id}`"),
            pipeline_id: Some(SUPPORTED_PIPELINE_ID.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "restore the required input and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::Empty { .. }) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::EmptyRequiredInput,
            summary: format!("required input `{path}` is empty for `{stage_id}`"),
            pipeline_id: Some(SUPPORTED_PIPELINE_ID.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fill the required input and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::InvalidPath(reason)) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidDefinition,
            summary: format!("compile input path `{path}` is invalid: {reason}"),
            pipeline_id: Some(SUPPORTED_PIPELINE_ID.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the stage definition and retry `pipeline compile`".to_string(),
        }),
        Err(DocumentLoadError::ReadFailure(err_path, source)) => Err(PipelineCompileRefusal {
            classification: PipelineCompileRefusalClassification::InvalidState,
            summary: format!(
                "failed to read compile input {}: {source}",
                err_path.display()
            ),
            pipeline_id: Some(SUPPORTED_PIPELINE_ID.to_string()),
            stage_id: Some(stage_id.to_string()),
            recovery: "fix the unreadable input and retry `pipeline compile`".to_string(),
        }),
    }
}

fn render_outputs(
    stage_definition: &CompileStageDefinition,
    variables: &BTreeMap<String, String>,
) -> Vec<PipelineCompileOutput> {
    let mut outputs = Vec::new();
    for output in &stage_definition.outputs.artifacts {
        outputs.push(PipelineCompileOutput {
            kind: PipelineCompileOutputKind::Artifact,
            path: substitute_variables(&output.path, variables),
        });
    }
    for output in &stage_definition.outputs.repo_files {
        outputs.push(PipelineCompileOutput {
            kind: PipelineCompileOutputKind::RepoFile,
            path: substitute_variables(&output.path, variables),
        });
    }
    outputs
}

enum DocumentLoadError {
    Missing,
    Empty,
    InvalidPath(String),
    ReadFailure(PathBuf, std::io::Error),
}

fn load_repo_relative_document(
    repo_root: &Path,
    relative_path: &str,
    apply_scoping: bool,
) -> Result<String, DocumentLoadError> {
    validate_repo_relative_path(relative_path).map_err(DocumentLoadError::InvalidPath)?;
    let path = repo_root.join(relative_path);
    let contents = std::fs::read_to_string(&path).map_err(|source| match source.kind() {
        std::io::ErrorKind::NotFound => DocumentLoadError::Missing,
        _ => DocumentLoadError::ReadFailure(path.clone(), source),
    })?;

    let filtered = if apply_scoping {
        filter_scoped_blocks(&contents)
    } else {
        normalize_text(&contents)
    };
    if filtered.trim().is_empty() {
        return Err(DocumentLoadError::Empty);
    }

    Ok(filtered)
}

fn validate_repo_relative_path(path: &str) -> Result<(), String> {
    let trimmed = path.trim();
    if trimmed.is_empty() {
        return Err("path must not be empty".to_string());
    }
    let path = Path::new(trimmed);
    if path.is_absolute() {
        return Err("path must be repo-relative".to_string());
    }
    for component in path.components() {
        match component {
            Component::Normal(_) => {}
            Component::CurDir => {}
            Component::ParentDir => return Err("path must not escape the repo root".to_string()),
            Component::RootDir | Component::Prefix(_) => {
                return Err("path must be repo-relative".to_string())
            }
        }
    }
    Ok(())
}

fn fingerprint_repo_relative_file(repo_root: &Path, relative_path: &str) -> Result<String, String> {
    validate_repo_relative_path(relative_path)?;
    let path = repo_root.join(relative_path);
    let contents = std::fs::read_to_string(&path)
        .map_err(|source| format!("failed to read {}: {source}", path.display()))?;
    let mut hasher = Sha256::new();
    hasher.update(contents.as_bytes());
    Ok(format!("{:x}", hasher.finalize()))
}

fn substitute_variables(input: &str, values: &BTreeMap<String, String>) -> String {
    let mut out = String::new();
    let mut rest = input;
    while let Some(start) = rest.find("${") {
        out.push_str(&rest[..start]);
        let after = &rest[start + 2..];
        let Some(end) = after.find('}') else {
            out.push_str(&rest[start..]);
            return out;
        };
        let key = &after[..end];
        match values.get(key).filter(|value| !value.is_empty()) {
            Some(value) => out.push_str(value),
            None => {
                out.push_str("${");
                out.push_str(key);
                out.push('}');
            }
        }
        rest = &after[end + 1..];
    }
    out.push_str(rest);
    out
}

fn filter_scoped_blocks(text: &str) -> String {
    let mut out_lines = Vec::new();
    let mut include = true;

    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(scopes) = parse_scope_start(trimmed) {
            include = scopes.iter().any(|scope| scope == "ALL" || scope == "L1");
            continue;
        }
        if trimmed == "<!-- END_SCOPE -->" {
            include = true;
            continue;
        }
        if include {
            out_lines.push(line.trim_end().to_string());
        }
    }

    normalize_text(&out_lines.join("\n"))
}

fn parse_scope_start(line: &str) -> Option<Vec<String>> {
    let inner = line
        .strip_prefix("<!-- SCOPE:")
        .and_then(|value| value.strip_suffix("-->"))?;
    Some(
        inner
            .split(',')
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
            .collect(),
    )
}

fn normalize_text(text: &str) -> String {
    let normalized = text.replace("\r\n", "\n");
    let lines = normalized
        .lines()
        .map(str::trim_end)
        .collect::<Vec<_>>()
        .join("\n");
    let trimmed = lines.trim();
    if trimmed.is_empty() {
        String::new()
    } else {
        format!("{trimmed}\n")
    }
}

fn normalize_rendered_output(text: &str) -> String {
    let normalized = normalize_text(text);
    if normalized.is_empty() {
        String::new()
    } else {
        normalized
    }
}

fn render_document_section<'a>(
    out: &mut String,
    title: &str,
    docs: impl Iterator<Item = &'a PipelineCompileDocument>,
) {
    let docs = docs
        .filter(|doc| doc.status == PipelineCompileDocumentStatus::Present)
        .collect::<Vec<_>>();
    out.push_str(&format!("\n## {title}\n"));
    if docs.is_empty() {
        out.push_str("(none)\n");
        return;
    }
    for doc in docs {
        out.push_str(&format!("\n### {}\n", doc.path));
        if let Some(content) = &doc.content {
            out.push_str(content.trim_end());
            out.push('\n');
        }
    }
}

fn render_output_section<'a>(
    out: &mut String,
    title: &str,
    outputs: impl Iterator<Item = &'a PipelineCompileOutput>,
) {
    let outputs = outputs.collect::<Vec<_>>();
    out.push_str(&format!("\n### {title}\n"));
    if outputs.is_empty() {
        out.push_str("(none declared)\n");
        return;
    }
    for output in outputs {
        out.push_str(&format!("- {}\n", output.path));
    }
}

fn render_optional_basis_value(out: &mut String, name: &str, value: Option<&str>) {
    match value {
        Some(value) => out.push_str(&format!("    {} = {}\n", name, value)),
        None => out.push_str(&format!("    {} = <unset>\n", name)),
    }
}

fn render_route_basis_status(status: RouteBasisStageStatus) -> &'static str {
    match status {
        RouteBasisStageStatus::Active => "active",
        RouteBasisStageStatus::Skipped => "skipped",
        RouteBasisStageStatus::Blocked => "blocked",
        RouteBasisStageStatus::Next => "next",
    }
}

fn render_route_basis_reason(reason: Option<&RouteBasisStageReason>) -> String {
    match reason {
        Some(RouteBasisStageReason::SkippedActivationFalse {
            unsatisfied_variables,
            ..
        }) => format!(
            "activation evaluated false for variables: {}",
            unsatisfied_variables.join(", ")
        ),
        Some(RouteBasisStageReason::NextMissingRouteVariables {
            missing_variables, ..
        }) => format!("missing route variables: {}", missing_variables.join(", ")),
        Some(RouteBasisStageReason::BlockedByUnresolvedStage {
            upstream_stage_id,
            upstream_status,
        }) => format!(
            "blocked by unresolved stage {} ({})",
            upstream_stage_id,
            render_route_basis_status(*upstream_status)
        ),
        None => "no persisted reason".to_string(),
    }
}

fn render_document_kind(kind: PipelineCompileDocumentKind) -> &'static str {
    match kind {
        PipelineCompileDocumentKind::Include => "include",
        PipelineCompileDocumentKind::Runner => "runner",
        PipelineCompileDocumentKind::Profile => "profile",
        PipelineCompileDocumentKind::Library => "library",
        PipelineCompileDocumentKind::Artifact => "artifact",
    }
}

fn render_document_status(status: PipelineCompileDocumentStatus) -> &'static str {
    match status {
        PipelineCompileDocumentStatus::Present => "present",
        PipelineCompileDocumentStatus::MissingOptional => "missing_optional",
    }
}

fn render_output_kind(kind: PipelineCompileOutputKind) -> &'static str {
    match kind {
        PipelineCompileOutputKind::Artifact => "artifact",
        PipelineCompileOutputKind::RepoFile => "repo_file",
    }
}

impl fmt::Display for PipelineCompileRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary)
    }
}

impl fmt::Display for PipelineCompileRefusalClassification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            PipelineCompileRefusalClassification::UnsupportedTarget => "unsupported_target",
            PipelineCompileRefusalClassification::InvalidDefinition => "invalid_definition",
            PipelineCompileRefusalClassification::InvalidState => "invalid_state",
            PipelineCompileRefusalClassification::MissingRouteBasis => "missing_route_basis",
            PipelineCompileRefusalClassification::MalformedRouteBasis => "malformed_route_basis",
            PipelineCompileRefusalClassification::StaleRouteBasis => "stale_route_basis",
            PipelineCompileRefusalClassification::InactiveStage => "inactive_stage",
            PipelineCompileRefusalClassification::MissingRequiredInput => "missing_required_input",
            PipelineCompileRefusalClassification::EmptyRequiredInput => "empty_required_input",
        };
        write!(f, "{value}")
    }
}
