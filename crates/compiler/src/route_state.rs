use crate::pipeline_route::RouteVariables;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

pub const ROUTE_STATE_SCHEMA_VERSION: &str = "m1-pipeline-state-v1";
pub const ROUTE_STATE_AUDIT_LIMIT: usize = 50;

static TEMP_FILE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteState {
    pub schema_version: String,
    pub pipeline_id: String,
    pub revision: u64,
    pub variables: BTreeMap<String, bool>,
    pub audit: Vec<RouteStateAuditEntry>,
}

impl RouteState {
    pub fn empty(pipeline_id: impl Into<String>) -> Self {
        Self {
            schema_version: ROUTE_STATE_SCHEMA_VERSION.to_string(),
            pipeline_id: pipeline_id.into(),
            revision: 0,
            variables: BTreeMap::new(),
            audit: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RouteStateAuditEntry {
    pub revision: u64,
    pub variable: String,
    pub value: bool,
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
    load_route_state_at_path(&state_path, pipeline_id, None)
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
    load_route_state_at_path(&state_path, pipeline_id, Some(supported_variables))
}

pub fn set_route_state_variable<I, S>(
    repo_root: impl AsRef<Path>,
    pipeline_id: impl AsRef<str>,
    supported_variables: I,
    variable: impl AsRef<str>,
    value: bool,
    expected_revision: u64,
) -> Result<RouteStateMutationOutcome, RouteStateStoreError>
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let pipeline_id = pipeline_id.as_ref();
    let variable = variable.as_ref();
    validate_pipeline_id(pipeline_id).map_err(|reason| {
        RouteStateStoreError::InvalidPipelineId {
            pipeline_id: pipeline_id.to_string(),
            reason,
        }
    })?;

    let supported_variables = normalize_supported_variables(supported_variables)?;
    validate_variable_name(variable)
        .map_err(|reason| RouteStateStoreError::InvalidSupportedVariables { reason })?;

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
    let mut state =
        match load_route_state_at_path(&state_path, pipeline_id, Some(&supported_variables)) {
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

    if !supported_variables.contains(variable) {
        return Ok(RouteStateMutationOutcome::Refused(
            RouteStateMutationRefusal::UnsupportedVariable {
                variable: variable.to_string(),
            },
        ));
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
    state.variables.insert(variable.to_string(), value);
    state.audit.push(RouteStateAuditEntry {
        revision: state.revision,
        variable: variable.to_string(),
        value,
    });
    trim_audit_history(&mut state.audit);

    persist_route_state(&state_path, &state)?;

    Ok(RouteStateMutationOutcome::Applied(state))
}

fn load_route_state_at_path(
    state_path: &Path,
    pipeline_id: &str,
    supported_variables: Option<&BTreeSet<String>>,
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

    validate_loaded_state(&state, pipeline_id, supported_variables, state_path)?;

    Ok(state)
}

fn validate_loaded_state(
    state: &RouteState,
    expected_pipeline_id: &str,
    supported_variables: Option<&BTreeSet<String>>,
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

    validate_variable_map(&state.variables).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason,
        }
    })?;
    validate_audit_entries(&state.audit, supported_variables).map_err(|reason| {
        RouteStateReadError::MalformedState {
            path: state_path.to_path_buf(),
            reason,
        }
    })?;

    if let Some(supported_variables) = supported_variables {
        for variable in state.variables.keys() {
            if !supported_variables.contains(variable) {
                return Err(RouteStateReadError::MalformedState {
                    path: state_path.to_path_buf(),
                    reason: format!("unsupported variable `{variable}` in persisted state"),
                });
            }
        }
    }

    Ok(())
}

fn validate_audit_entries(
    audit: &[RouteStateAuditEntry],
    supported_variables: Option<&BTreeSet<String>>,
) -> Result<(), String> {
    for entry in audit {
        validate_variable_name(&entry.variable)?;
        if let Some(supported_variables) = supported_variables {
            if !supported_variables.contains(&entry.variable) {
                return Err(format!(
                    "unsupported audit variable `{}` in persisted state",
                    entry.variable
                ));
            }
        }
    }

    Ok(())
}

fn validate_variable_map(values: &BTreeMap<String, bool>) -> Result<(), String> {
    let values = values.iter().map(|(name, value)| (name.clone(), *value));
    RouteVariables::new(values.collect()).map_err(|err| err.to_string())?;
    Ok(())
}

fn validate_variable_name(variable: &str) -> Result<(), String> {
    let mut values = BTreeMap::new();
    values.insert(variable.to_string(), false);
    RouteVariables::new(values)
        .map(|_| ())
        .map_err(|err| err.to_string())
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
        (Some(std::path::Component::Normal(_)), None) => Ok(()),
        (Some(std::path::Component::CurDir), None) => {
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
