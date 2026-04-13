use crate::pipeline_route::RouteVariables;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub const ROUTE_STATE_SCHEMA_VERSION: &str = "m1-pipeline-state-v2";
pub const ROUTE_STATE_AUDIT_LIMIT: usize = 50;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

const FIELD_REFS_CHARTER_REF: &str = "refs.charter_ref";
const FIELD_REFS_PROJECT_CONTEXT_REF: &str = "refs.project_context_ref";
const FIELD_RUN_RUNNER: &str = "run.runner";
const FIELD_RUN_PROFILE: &str = "run.profile";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteState {
    pub schema_version: String,
    pub pipeline_id: String,
    pub revision: u64,
    pub routing: BTreeMap<String, bool>,
    pub refs: RouteStateRefs,
    pub run: RouteStateRun,
    pub audit: Vec<RouteStateAuditEntry>,
}

impl RouteState {
    pub fn empty(pipeline_id: impl Into<String>) -> Self {
        Self {
            schema_version: ROUTE_STATE_SCHEMA_VERSION.to_string(),
            pipeline_id: pipeline_id.into(),
            revision: 0,
            routing: BTreeMap::new(),
            refs: RouteStateRefs::default(),
            run: RouteStateRun::default(),
            audit: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteStateRefs {
    pub charter_ref: Option<String>,
    pub project_context_ref: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteStateRun {
    pub runner: Option<String>,
    pub profile: Option<String>,
    pub repo_root: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct RouteStateRunInventory {
    runners: BTreeSet<String>,
    profiles: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteStateAuditEntry {
    pub revision: u64,
    pub field_path: String,
    pub value: RouteStateValue,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RouteStateValue {
    Bool(bool),
    String(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteStateMutation {
    RoutingVariable { variable: String, value: bool },
    RefCharterRef { value: String },
    RefProjectContextRef { value: String },
    RunRunner { value: String },
    RunProfile { value: String },
}

impl RouteStateMutation {
    fn field_path(&self) -> String {
        match self {
            Self::RoutingVariable { variable, .. } => format!("routing.{variable}"),
            Self::RefCharterRef { .. } => FIELD_REFS_CHARTER_REF.to_string(),
            Self::RefProjectContextRef { .. } => FIELD_REFS_PROJECT_CONTEXT_REF.to_string(),
            Self::RunRunner { .. } => FIELD_RUN_RUNNER.to_string(),
            Self::RunProfile { .. } => FIELD_RUN_PROFILE.to_string(),
        }
    }

    fn value(&self) -> RouteStateValue {
        match self {
            Self::RoutingVariable { value, .. } => RouteStateValue::Bool(*value),
            Self::RefCharterRef { value }
            | Self::RefProjectContextRef { value }
            | Self::RunRunner { value }
            | Self::RunProfile { value } => RouteStateValue::String(value.clone()),
        }
    }

    fn apply(&self, state: &mut RouteState) {
        match self {
            Self::RoutingVariable { variable, value } => {
                state.routing.insert(variable.clone(), *value);
            }
            Self::RefCharterRef { value } => {
                state.refs.charter_ref = Some(value.clone());
            }
            Self::RefProjectContextRef { value } => {
                state.refs.project_context_ref = Some(value.clone());
            }
            Self::RunRunner { value } => {
                state.run.runner = Some(value.clone());
            }
            Self::RunProfile { value } => {
                state.run.profile = Some(value.clone());
            }
        }
    }
}

enum RouteStateFieldPath<'a> {
    Routing(&'a str),
    RefsCharterRef,
    RefsProjectContextRef,
    RunRunner,
    RunProfile,
}

#[derive(Debug)]
pub enum RouteStateReadError {
    InvalidPipelineId {
        pipeline_id: String,
        reason: &'static str,
    },
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    MalformedState {
        path: PathBuf,
        reason: String,
    },
}

#[derive(Debug)]
pub enum RouteStateStoreError {
    InvalidPipelineId {
        pipeline_id: String,
        reason: &'static str,
    },
    InvalidSupportedVariables {
        reason: String,
    },
    InvalidMutation {
        reason: String,
    },
    ReadFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    LockFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    WriteFailure {
        path: PathBuf,
        source: std::io::Error,
    },
    SerializationFailure {
        path: PathBuf,
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteStateMutationOutcome {
    Applied(RouteState),
    Refused(RouteStateMutationRefusal),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RouteStateMutationRefusal {
    MalformedState {
        reason: String,
    },
    UnsupportedVariable {
        variable: String,
    },
    RevisionConflict {
        expected_revision: u64,
        actual_revision: u64,
    },
}

#[derive(Debug)]
struct RouteStateInventoryLoadError {
    path: PathBuf,
    source: std::io::Error,
}

pub fn load_route_state(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
) -> Result<RouteState, RouteStateReadError> {
    let pipeline_id = pipeline_id.as_ref();
    let state_path = route_state_path(repo_root.as_ref(), pipeline_id).map_err(|reason| {
        RouteStateReadError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;
    let run_inventory =
        load_run_inventory(repo_root.as_ref()).map_err(|err| RouteStateReadError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    load_route_state_at_path(&state_path, pipeline_id, None, &run_inventory)
}

pub fn load_route_state_with_supported_variables(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    supported_variables: &BTreeSet<String>,
) -> Result<RouteState, RouteStateReadError> {
    let pipeline_id = pipeline_id.as_ref();
    let state_path = route_state_path(repo_root.as_ref(), pipeline_id).map_err(|reason| {
        RouteStateReadError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;
    let run_inventory =
        load_run_inventory(repo_root.as_ref()).map_err(|err| RouteStateReadError::ReadFailure {
            path: err.path,
            source: err.source,
        })?;
    load_route_state_at_path(
        &state_path,
        pipeline_id,
        Some(supported_variables),
        &run_inventory,
    )
}

pub fn set_route_state(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    supported_variables: impl IntoIterator<Item = impl AsRef<str>>,
    mutation: RouteStateMutation,
    expected_revision: u64,
) -> Result<RouteStateMutationOutcome, RouteStateStoreError> {
    let pipeline_id = pipeline_id.as_ref();
    validate_pipeline_id(pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;

    let supported_variables = normalize_supported_variables(supported_variables)?;
    let run_inventory = load_run_inventory(repo_root.as_ref()).map_err(|err| {
        RouteStateStoreError::ReadFailure {
            path: err.path,
            source: err.source,
        }
    })?;
    validate_mutation(&mutation, &supported_variables, &run_inventory)?;

    let state_path = route_state_path(repo_root.as_ref(), pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;
    ensure_state_parent_dir(&state_path).map_err(|source| RouteStateStoreError::ReadFailure {
        path: state_path.clone(),
        source,
    })?;

    let _lock = acquire_advisory_lock(&state_path)?;
    let mut state = match load_route_state_at_path(
        &state_path,
        pipeline_id,
        Some(&supported_variables),
        &run_inventory,
    ) {
        Ok(state) => state,
        Err(RouteStateReadError::MalformedState { reason, .. }) => {
            return Ok(RouteStateMutationOutcome::Refused(
                RouteStateMutationRefusal::MalformedState { reason },
            ));
        }
        Err(RouteStateReadError::InvalidPipelineId {
            pipeline_id,
            reason,
        }) => {
            return Err(RouteStateStoreError::InvalidPipelineId {
                pipeline_id,
                reason,
            });
        }
        Err(RouteStateReadError::ReadFailure { path, source }) => {
            return Err(RouteStateStoreError::ReadFailure { path, source });
        }
    };

    if let RouteStateMutation::RoutingVariable { variable, .. } = &mutation {
        if !supported_variables.contains(variable) {
            return Ok(RouteStateMutationOutcome::Refused(
                RouteStateMutationRefusal::UnsupportedVariable {
                    variable: variable.clone(),
                },
            ));
        }
    }

    if state.revision != expected_revision {
        return Ok(RouteStateMutationOutcome::Refused(
            RouteStateMutationRefusal::RevisionConflict {
                expected_revision,
                actual_revision: state.revision,
            },
        ));
    }

    state.revision = state.revision.saturating_add(1);
    mutation.apply(&mut state);
    state.run.repo_root = Some(derived_repo_root(repo_root.as_ref()));
    state.audit.push(RouteStateAuditEntry {
        revision: state.revision,
        field_path: mutation.field_path(),
        value: mutation.value(),
    });
    trim_audit_history(&mut state.audit);

    persist_route_state(&state_path, &state)?;

    Ok(RouteStateMutationOutcome::Applied(state))
}

fn load_route_state_at_path(
    state_path: &Path,
    pipeline_id: &str,
    supported_variables: Option<&BTreeSet<String>>,
    run_inventory: &RouteStateRunInventory,
) -> Result<RouteState, RouteStateReadError> {
    let file_meta = match fs::symlink_metadata(state_path) {
        Ok(meta) => Some(meta),
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => None,
        Err(source) => {
            return Err(RouteStateReadError::ReadFailure {
                path: state_path.to_path_buf(),
                source,
            });
        }
    };

    if file_meta.is_none() {
        let state = RouteState::empty(pipeline_id);
        return Ok(state);
    }

    let meta = file_meta.expect("file_meta present");
    if !meta.is_file() {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: "state file must be a regular file".to_string(),
        });
    }

    let contents =
        read_file_no_follow(state_path).map_err(|source| RouteStateReadError::ReadFailure {
            path: state_path.to_path_buf(),
            source,
        })?;

    let state: RouteState = serde_yaml_bw::from_str(&contents).map_err(|source| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: source.to_string(),
        }
    })?;

    validate_loaded_state(
        &state,
        pipeline_id,
        supported_variables,
        run_inventory,
        state_path,
    )?;

    Ok(state)
}

fn validate_loaded_state(
    state: &RouteState,
    expected_pipeline_id: &str,
    supported_variables: Option<&BTreeSet<String>>,
    run_inventory: &RouteStateRunInventory,
    state_path: &Path,
) -> Result<(), RouteStateReadError> {
    if state.schema_version != ROUTE_STATE_SCHEMA_VERSION {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!(
                "unexpected schema_version `{}`; expected `{}`",
                state.schema_version, ROUTE_STATE_SCHEMA_VERSION
            ),
        });
    }

    if state.pipeline_id != expected_pipeline_id {
        return Err(RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason: format!(
                "pipeline_id `{}` does not match expected `{}`",
                state.pipeline_id, expected_pipeline_id
            ),
        });
    }

    validate_routing_map(&state.routing).map_err(|reason| RouteStateReadError::MalformedState {
        path: state_path.to_path_buf(),
        reason,
    })?;
    validate_refs(&state.refs).map_err(|reason| RouteStateReadError::MalformedState {
        path: state_path.to_path_buf(),
        reason,
    })?;
    validate_run(&state.run, run_inventory).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason,
        }
    })?;
    validate_audit_entries(&state.audit, supported_variables, run_inventory).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason,
        }
    })?;

    if let Some(supported_variables) = supported_variables {
        for variable in state.routing.keys() {
            if !supported_variables.contains(variable) {
                return Err(RouteStateReadError::MalformedState {
                    path: state_path.to_path_buf(),
                    reason: format!("unsupported routing variable `{variable}` in persisted state"),
                });
            }
        }
    }

    Ok(())
}

fn validate_mutation(
    mutation: &RouteStateMutation,
    supported_variables: &BTreeSet<String>,
    run_inventory: &RouteStateRunInventory,
) -> Result<(), RouteStateStoreError> {
    match mutation {
        RouteStateMutation::RoutingVariable { variable, .. } => {
            validate_variable_name(variable)
                .map_err(|reason| RouteStateStoreError::InvalidMutation { reason })?;

            let values = supported_variables
                .iter()
                .map(|variable| (variable.clone(), false))
                .collect::<BTreeMap<_, _>>();
            RouteVariables::new(values).map_err(|err| {
                RouteStateStoreError::InvalidSupportedVariables {
                    reason: err.to_string(),
                }
            })?;
        }
        RouteStateMutation::RefCharterRef { value }
        | RouteStateMutation::RefProjectContextRef { value } => {
            validate_repo_relative_ref(value)
                .map_err(|reason| RouteStateStoreError::InvalidMutation { reason })?
        }
        RouteStateMutation::RunRunner { value } => {
            validate_inventory_value(value, FIELD_RUN_RUNNER, "runners/", &run_inventory.runners)
                .map_err(|reason| RouteStateStoreError::InvalidMutation { reason })?
        }
        RouteStateMutation::RunProfile { value } => validate_inventory_value(
            value,
            FIELD_RUN_PROFILE,
            "profiles/",
            &run_inventory.profiles,
        )
        .map_err(|reason| RouteStateStoreError::InvalidMutation { reason })?,
    }

    Ok(())
}

fn validate_audit_entries(
    audit: &[RouteStateAuditEntry],
    supported_variables: Option<&BTreeSet<String>>,
    run_inventory: &RouteStateRunInventory,
) -> Result<(), String> {
    for entry in audit {
        let field = parse_route_state_field_path(&entry.field_path)?;
        match field {
            RouteStateFieldPath::Routing(variable) => {
                match &entry.value {
                    RouteStateValue::Bool(_) => {}
                    RouteStateValue::String(_) => {
                        return Err(format!(
                            "audit field `{}` must use a boolean value",
                            entry.field_path
                        ));
                    }
                }

                if let Some(supported_variables) = supported_variables {
                    if !supported_variables.contains(variable) {
                        return Err(format!(
                            "unsupported audit routing variable `{variable}` in persisted state"
                        ));
                    }
                }
            }
            RouteStateFieldPath::RefsCharterRef | RouteStateFieldPath::RefsProjectContextRef => {
                match &entry.value {
                    RouteStateValue::String(value) => validate_repo_relative_ref(value)?,
                    RouteStateValue::Bool(_) => {
                        return Err(format!(
                            "audit field `{}` must use a string value",
                            entry.field_path
                        ));
                    }
                }
            }
            RouteStateFieldPath::RunRunner | RouteStateFieldPath::RunProfile => {
                match &entry.value {
                    RouteStateValue::String(value) => match field {
                        RouteStateFieldPath::RunRunner => validate_inventory_value(
                            value,
                            &entry.field_path,
                            "runners/",
                            &run_inventory.runners,
                        )?,
                        RouteStateFieldPath::RunProfile => validate_inventory_value(
                            value,
                            &entry.field_path,
                            "profiles/",
                            &run_inventory.profiles,
                        )?,
                        _ => unreachable!("run field match already constrained"),
                    },
                    RouteStateValue::Bool(_) => {
                        return Err(format!(
                            "audit field `{}` must use a string value",
                            entry.field_path
                        ));
                    }
                }
            }
        }
    }

    Ok(())
}

fn validate_routing_map(values: &BTreeMap<String, bool>) -> Result<(), String> {
    let values = values.iter().map(|(name, value)| (name.clone(), *value));
    RouteVariables::new(values.collect()).map_err(|err| err.to_string())?;
    Ok(())
}

fn validate_refs(refs: &RouteStateRefs) -> Result<(), String> {
    if let Some(value) = &refs.charter_ref {
        validate_repo_relative_ref(value)?;
    }
    if let Some(value) = &refs.project_context_ref {
        validate_repo_relative_ref(value)?;
    }
    Ok(())
}

fn validate_run(run: &RouteStateRun, run_inventory: &RouteStateRunInventory) -> Result<(), String> {
    if let Some(value) = &run.runner {
        validate_inventory_value(value, FIELD_RUN_RUNNER, "runners/", &run_inventory.runners)?;
    }
    if let Some(value) = &run.profile {
        validate_inventory_value(
            value,
            FIELD_RUN_PROFILE,
            "profiles/",
            &run_inventory.profiles,
        )?;
    }
    if let Some(value) = &run.repo_root {
        validate_repo_root(value)?;
    }
    Ok(())
}

fn validate_variable_name(variable: &str) -> Result<(), String> {
    let mut values = BTreeMap::new();
    values.insert(variable.to_string(), false);
    RouteVariables::new(values)
        .map(|_| ())
        .map_err(|err| err.to_string())
}

fn validate_non_empty_string(value: &str) -> Result<(), String> {
    if value.trim().is_empty() {
        Err("value must not be empty".to_string())
    } else {
        Ok(())
    }
}

fn validate_inventory_value(
    value: &str,
    field_path: &str,
    inventory_root: &str,
    allowlisted_ids: &BTreeSet<String>,
) -> Result<(), String> {
    validate_non_empty_string(value)?;
    if allowlisted_ids.contains(value) {
        return Ok(());
    }

    Err(format!(
        "{field_path} `{value}` is not declared under `{inventory_root}`"
    ))
}

fn validate_repo_relative_ref(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("repo-relative ref must not be empty".to_string());
    }

    let path = Path::new(trimmed);
    if path.is_absolute() {
        return Err(format!(
            "repo-relative ref `{trimmed}` must not be absolute"
        ));
    }

    let mut saw_normal = false;
    for component in path.components() {
        match component {
            Component::Normal(_) => saw_normal = true,
            Component::ParentDir => {
                return Err(format!(
                    "repo-relative ref `{trimmed}` must not escape the repo root"
                ));
            }
            Component::CurDir | Component::RootDir | Component::Prefix(_) => {
                return Err(format!(
                    "repo-relative ref `{trimmed}` must be a clean repo-relative path"
                ));
            }
        }
    }

    if !saw_normal {
        return Err(format!(
            "repo-relative ref `{trimmed}` must include at least one path component"
        ));
    }

    Ok(())
}

fn validate_repo_root(value: &str) -> Result<(), String> {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return Err("run.repo_root must not be empty".to_string());
    }

    let path = Path::new(trimmed);
    if !path.is_absolute() {
        return Err(format!("run.repo_root `{trimmed}` must be absolute"));
    }

    let mut saw_normal = false;
    for component in path.components() {
        match component {
            Component::Normal(_) => saw_normal = true,
            Component::RootDir => {}
            Component::ParentDir | Component::CurDir | Component::Prefix(_) => {
                return Err(format!(
                    "run.repo_root `{trimmed}` must be a clean absolute path"
                ));
            }
        }
    }

    if !saw_normal {
        return Err(format!(
            "run.repo_root `{trimmed}` must include at least one path component"
        ));
    }

    Ok(())
}

fn derived_repo_root(repo_root: &Path) -> String {
    repo_root.to_string_lossy().into_owned()
}

fn parse_route_state_field_path(input: &str) -> Result<RouteStateFieldPath<'_>, String> {
    if let Some(variable) = input.strip_prefix("routing.") {
        validate_variable_name(variable)?;
        return Ok(RouteStateFieldPath::Routing(variable));
    }

    match input {
        FIELD_REFS_CHARTER_REF => Ok(RouteStateFieldPath::RefsCharterRef),
        FIELD_REFS_PROJECT_CONTEXT_REF => Ok(RouteStateFieldPath::RefsProjectContextRef),
        FIELD_RUN_RUNNER => Ok(RouteStateFieldPath::RunRunner),
        FIELD_RUN_PROFILE => Ok(RouteStateFieldPath::RunProfile),
        _ => Err(format!("unsupported route-state field path `{input}`")),
    }
}

fn normalize_supported_variables<I, S>(
    supported_variables: I,
) -> Result<BTreeSet<String>, RouteStateStoreError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut values = BTreeMap::new();
    for variable in supported_variables {
        values.insert(variable.as_ref().to_string(), false);
    }

    let supported_variables = values.keys().cloned().collect::<BTreeSet<_>>();

    RouteVariables::new(values).map_err(|err| RouteStateStoreError::InvalidSupportedVariables {
        reason: err.to_string(),
    })?;

    Ok(supported_variables)
}

fn load_run_inventory(
    repo_root: &Path,
) -> Result<RouteStateRunInventory, RouteStateInventoryLoadError> {
    Ok(RouteStateRunInventory {
        runners: load_runner_inventory(repo_root)?,
        profiles: load_profile_inventory(repo_root)?,
    })
}

fn load_runner_inventory(
    repo_root: &Path,
) -> Result<BTreeSet<String>, RouteStateInventoryLoadError> {
    let inventory_dir = repo_root.join("runners");
    let entries = match fs::read_dir(&inventory_dir) {
        Ok(entries) => entries,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeSet::new()),
        Err(source) => {
            return Err(RouteStateInventoryLoadError {
                path: inventory_dir,
                source,
            });
        }
    };

    let mut ids = BTreeSet::new();
    for entry in entries {
        let entry = entry.map_err(|source| RouteStateInventoryLoadError {
            path: inventory_dir.clone(),
            source,
        })?;
        let entry_path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| RouteStateInventoryLoadError {
                path: entry_path.clone(),
                source,
            })?;
        if !file_type.is_file() {
            continue;
        }

        let Some(name) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        let Some(id) = name.strip_suffix(".md") else {
            continue;
        };
        if !is_inventory_id(id) {
            continue;
        }

        ids.insert(id.to_string());
    }

    Ok(ids)
}

fn load_profile_inventory(
    repo_root: &Path,
) -> Result<BTreeSet<String>, RouteStateInventoryLoadError> {
    let inventory_dir = repo_root.join("profiles");
    let entries = match fs::read_dir(&inventory_dir) {
        Ok(entries) => entries,
        Err(source) if source.kind() == std::io::ErrorKind::NotFound => return Ok(BTreeSet::new()),
        Err(source) => {
            return Err(RouteStateInventoryLoadError {
                path: inventory_dir,
                source,
            });
        }
    };

    let mut ids = BTreeSet::new();
    for entry in entries {
        let entry = entry.map_err(|source| RouteStateInventoryLoadError {
            path: inventory_dir.clone(),
            source,
        })?;
        let entry_path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|source| RouteStateInventoryLoadError {
                path: entry_path.clone(),
                source,
            })?;
        if !file_type.is_dir() {
            continue;
        }

        let Some(id) = entry.file_name().to_str().map(str::to_owned) else {
            continue;
        };
        if !is_inventory_id(&id) {
            continue;
        }

        let profile_yaml = entry_path.join("profile.yaml");
        let metadata = match fs::symlink_metadata(&profile_yaml) {
            Ok(metadata) => metadata,
            Err(source) if source.kind() == std::io::ErrorKind::NotFound => continue,
            Err(source) => {
                return Err(RouteStateInventoryLoadError {
                    path: profile_yaml,
                    source,
                });
            }
        };
        if !metadata.is_file() {
            continue;
        }

        ids.insert(id);
    }

    Ok(ids)
}

fn is_inventory_id(value: &str) -> bool {
    let Some(first) = value.chars().next() else {
        return false;
    };
    if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
        return false;
    }

    value
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
}

fn trim_audit_history(audit: &mut Vec<RouteStateAuditEntry>) {
    if audit.len() <= ROUTE_STATE_AUDIT_LIMIT {
        return;
    }

    let overflow = audit.len() - ROUTE_STATE_AUDIT_LIMIT;
    audit.drain(0..overflow);
}

fn route_state_path(repo_root: &Path, pipeline_id: &str) -> Result<PathBuf, &'static str> {
    validate_pipeline_id(pipeline_id)?;

    Ok(repo_root
        .join(".system")
        .join("state")
        .join("pipeline")
        .join(format!("{pipeline_id}.yaml")))
}

fn ensure_state_parent_dir(state_path: &Path) -> Result<(), std::io::Error> {
    if let Some(parent) = state_path.parent() {
        fs::create_dir_all(parent)?;
    }

    Ok(())
}

fn persist_route_state(state_path: &Path, state: &RouteState) -> Result<(), RouteStateStoreError> {
    let serialized = serde_yaml_bw::to_string(state).map_err(|source| {
        RouteStateStoreError::SerializationFailure {
            path: state_path.to_path_buf(),
            reason: source.to_string(),
        }
    })?;

    let temp_path = temp_route_state_path(state_path);
    let result = (|| -> Result<(), RouteStateStoreError> {
        let mut file = open_new_temp_file(&temp_path)?;
        file.write_all(serialized.as_bytes()).map_err(|source| {
            RouteStateStoreError::WriteFailure {
                path: temp_path.clone(),
                source,
            }
        })?;
        file.sync_all()
            .map_err(|source| RouteStateStoreError::WriteFailure {
                path: temp_path.clone(),
                source,
            })?;
        drop(file);
        fs::rename(&temp_path, state_path).map_err(|source| {
            RouteStateStoreError::WriteFailure {
                path: state_path.to_path_buf(),
                source,
            }
        })?;
        sync_parent_dir(state_path)?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    result
}

fn open_new_temp_file(path: &Path) -> Result<File, RouteStateStoreError> {
    let mut options = OpenOptions::new();
    options.create_new(true).write(true).read(true);

    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        options.mode(0o600);
    }

    options
        .open(path)
        .map_err(|source| RouteStateStoreError::WriteFailure {
            path: path.to_path_buf(),
            source,
        })
}

fn sync_parent_dir(path: &Path) -> Result<(), RouteStateStoreError> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };

    let dir = File::open(parent).map_err(|source| RouteStateStoreError::WriteFailure {
        path: parent.to_path_buf(),
        source,
    })?;
    dir.sync_all()
        .map_err(|source| RouteStateStoreError::WriteFailure {
            path: parent.to_path_buf(),
            source,
        })?;
    Ok(())
}

fn temp_route_state_path(state_path: &Path) -> PathBuf {
    let counter = TEMP_FILE_COUNTER.fetch_add(1, Ordering::Relaxed);
    let pid = std::process::id();
    let file_name = state_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("state.yaml");
    state_path.with_file_name(format!("{file_name}.tmp-{pid}-{counter}"))
}

fn read_file_no_follow(path: &Path) -> Result<String, std::io::Error> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;

        let mut file = OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    #[cfg(not(unix))]
    {
        fs::read_to_string(path)
    }
}

fn validate_pipeline_id(pipeline_id: &str) -> Result<(), &'static str> {
    if pipeline_id.trim().is_empty() {
        return Err("pipeline id must not be empty");
    }

    let mut components = Path::new(pipeline_id).components();
    match (components.next(), components.next()) {
        (Some(Component::Normal(_)), None) => Ok(()),
        (Some(Component::CurDir), None) => {
            Err("pipeline id must be a single repo-relative path component")
        }
        _ => Err("pipeline id must be a single repo-relative path component"),
    }
}

fn acquire_advisory_lock(state_path: &Path) -> Result<RouteStateLockGuard, RouteStateStoreError> {
    let lock_path = state_path.with_extension("lock");
    if let Some(parent) = lock_path.parent() {
        fs::create_dir_all(parent).map_err(|source| RouteStateStoreError::LockFailure {
            path: lock_path.clone(),
            source,
        })?;
    }

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(&lock_path)
        .map_err(|source| RouteStateStoreError::LockFailure {
            path: lock_path.clone(),
            source,
        })?;

    #[cfg(unix)]
    {
        lock_file(&file, libc::LOCK_EX).map_err(|source| RouteStateStoreError::LockFailure {
            path: lock_path.clone(),
            source,
        })?;
    }

    Ok(RouteStateLockGuard { file, lock_path })
}

#[cfg(unix)]
fn lock_file(file: &File, operation: libc::c_int) -> Result<(), std::io::Error> {
    use std::os::unix::io::AsRawFd;

    loop {
        let result = unsafe { libc::flock(file.as_raw_fd(), operation) };
        if result == 0 {
            return Ok(());
        }

        let error = std::io::Error::last_os_error();
        if error.kind() == std::io::ErrorKind::Interrupted {
            continue;
        }

        return Err(error);
    }
}

#[cfg(not(unix))]
fn lock_file(_file: &File, _operation: libc::c_int) -> Result<(), std::io::Error> {
    Ok(())
}

struct RouteStateLockGuard {
    file: File,
    lock_path: PathBuf,
}

impl Drop for RouteStateLockGuard {
    fn drop(&mut self) {
        #[cfg(unix)]
        let _ = lock_file(&self.file, libc::LOCK_UN);
        let _ = &self.lock_path;
    }
}

impl fmt::Display for RouteStateValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateValue::Bool(value) => write!(f, "{value}"),
            RouteStateValue::String(value) => write!(f, "{value}"),
        }
    }
}

impl fmt::Display for RouteStateReadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateReadError::InvalidPipelineId {
                pipeline_id,
                reason,
            } => {
                write!(
                    f,
                    "route state pipeline id `{pipeline_id}` is invalid: {reason}"
                )
            }
            RouteStateReadError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read route state at {}: {source}",
                    path.display()
                )
            }
            RouteStateReadError::MalformedState { path, reason } => {
                write!(f, "malformed route state at {}: {reason}", path.display())
            }
        }
    }
}

impl std::error::Error for RouteStateReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RouteStateReadError::ReadFailure { source, .. } => Some(source),
            RouteStateReadError::InvalidPipelineId { .. }
            | RouteStateReadError::MalformedState { .. } => None,
        }
    }
}

impl fmt::Display for RouteStateStoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateStoreError::InvalidPipelineId {
                pipeline_id,
                reason,
            } => {
                write!(
                    f,
                    "route state pipeline id `{pipeline_id}` is invalid: {reason}"
                )
            }
            RouteStateStoreError::InvalidSupportedVariables { reason } => {
                write!(f, "supported route-state variables are invalid: {reason}")
            }
            RouteStateStoreError::InvalidMutation { reason } => {
                write!(f, "route state mutation is invalid: {reason}")
            }
            RouteStateStoreError::ReadFailure { path, source } => {
                write!(
                    f,
                    "failed to read route state at {}: {source}",
                    path.display()
                )
            }
            RouteStateStoreError::LockFailure { path, source } => {
                write!(
                    f,
                    "failed to acquire route state lock at {}: {source}",
                    path.display()
                )
            }
            RouteStateStoreError::WriteFailure { path, source } => {
                write!(
                    f,
                    "failed to write route state at {}: {source}",
                    path.display()
                )
            }
            RouteStateStoreError::SerializationFailure { path, reason } => {
                write!(
                    f,
                    "failed to serialize route state for {}: {reason}",
                    path.display()
                )
            }
        }
    }
}

impl std::error::Error for RouteStateStoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            RouteStateStoreError::ReadFailure { source, .. }
            | RouteStateStoreError::LockFailure { source, .. }
            | RouteStateStoreError::WriteFailure { source, .. } => Some(source),
            RouteStateStoreError::InvalidPipelineId { .. }
            | RouteStateStoreError::InvalidSupportedVariables { .. }
            | RouteStateStoreError::InvalidMutation { .. }
            | RouteStateStoreError::SerializationFailure { .. } => None,
        }
    }
}

impl fmt::Display for RouteStateMutationOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateMutationOutcome::Applied(state) => {
                write!(f, "applied route state revision {}", state.revision)
            }
            RouteStateMutationOutcome::Refused(refusal) => write!(f, "{refusal}"),
        }
    }
}

impl fmt::Display for RouteStateMutationRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RouteStateMutationRefusal::MalformedState { reason } => {
                write!(f, "malformed route state: {reason}")
            }
            RouteStateMutationRefusal::UnsupportedVariable { variable } => {
                write!(f, "unsupported route-state variable `{variable}`")
            }
            RouteStateMutationRefusal::RevisionConflict {
                expected_revision,
                actual_revision,
            } => write!(
                f,
                "revision conflict: expected {expected_revision}, found {actual_revision}"
            ),
        }
    }
}
