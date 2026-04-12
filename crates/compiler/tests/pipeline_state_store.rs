use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use system_compiler::{
    load_route_state, set_route_state_variable, RouteState, RouteStateMutationOutcome,
    RouteStateMutationRefusal, RouteStateReadError, ROUTE_STATE_AUDIT_LIMIT,
    ROUTE_STATE_SCHEMA_VERSION,
};

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn state_path(repo_root: &Path, pipeline_id: &str) -> PathBuf {
    repo_root
        .join(".system")
        .join("state")
        .join("pipeline")
        .join(format!("{pipeline_id}.yaml"))
}

fn lock_path(state_path: &Path) -> PathBuf {
    state_path.with_extension("lock")
}

fn acquire_unix_lock(path: &Path) -> std::fs::File {
    use std::os::unix::io::AsRawFd;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdir lock parent");
    }

    let file = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .expect("open lock file");

    let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX) };
    assert_eq!(rc, 0, "acquire lock");
    file
}

fn release_unix_lock(file: &std::fs::File) {
    use std::os::unix::io::AsRawFd;

    let rc = unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_UN) };
    assert_eq!(rc, 0, "release lock");
}

#[test]
fn missing_state_loads_as_empty_and_round_trips_revisioned_variables() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";

    let empty = load_route_state(repo_root, pipeline_id).expect("empty state");
    assert_eq!(empty, RouteState::empty(pipeline_id));

    let outcome = set_route_state_variable(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        "needs_project_context",
        true,
        0,
    )
    .expect("mutation");
    let mut state = match outcome {
        RouteStateMutationOutcome::Applied(state) => state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.schema_version, ROUTE_STATE_SCHEMA_VERSION);
    assert_eq!(state.pipeline_id, pipeline_id);
    assert_eq!(state.revision, 1);
    assert_eq!(state.variables.get("needs_project_context"), Some(&true));
    assert_eq!(state.audit.len(), 1);

    let outcome = set_route_state_variable(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        "charter_gaps_detected",
        false,
        1,
    )
    .expect("mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 2);
    assert_eq!(state.variables.get("needs_project_context"), Some(&true));
    assert_eq!(state.variables.get("charter_gaps_detected"), Some(&false));
    assert_eq!(state.audit.len(), 2);
    assert_eq!(state.audit[0].revision, 1);
    assert_eq!(state.audit[1].revision, 2);

    let loaded = load_route_state(repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded, state);
}

#[test]
fn malformed_state_refuses_unknown_top_level_keys() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v1
pipeline_id: pipeline.route_state
revision: 1
variables:
  needs_project_context: true
audit: []
unexpected: true
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { path: err_path, .. } => {
            assert_eq!(err_path, path);
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn malformed_state_refuses_non_boolean_values() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v1
pipeline_id: pipeline.route_state
revision: 1
variables:
  needs_project_context: maybe
audit: []
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { path: err_path, .. } => {
            assert_eq!(err_path, path);
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn unsupported_variable_refuses_without_overwrite() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);

    let outcome = set_route_state_variable(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        "charter_gaps_detected",
        true,
        0,
    )
    .expect("mutation");

    match outcome {
        RouteStateMutationOutcome::Refused(RouteStateMutationRefusal::UnsupportedVariable {
            variable,
        }) => assert_eq!(variable, "charter_gaps_detected"),
        other => panic!("expected unsupported-variable refusal, got {other:?}"),
    }
    assert!(!path.exists());
}

#[test]
fn revision_conflict_refuses_without_overwrite() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v1
pipeline_id: pipeline.route_state
revision: 2
variables:
  needs_project_context: true
audit:
  - revision: 1
    variable: needs_project_context
    value: true
  - revision: 2
    variable: needs_project_context
    value: true
"#,
    );

    let outcome = set_route_state_variable(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        "needs_project_context",
        false,
        1,
    )
    .expect("mutation");

    match outcome {
        RouteStateMutationOutcome::Refused(RouteStateMutationRefusal::RevisionConflict {
            expected_revision,
            actual_revision,
        }) => {
            assert_eq!(expected_revision, 1);
            assert_eq!(actual_revision, 2);
        }
        other => panic!("expected revision-conflict refusal, got {other:?}"),
    }

    let loaded = load_route_state(repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded.revision, 2);
    assert_eq!(loaded.variables.get("needs_project_context"), Some(&true));
}

#[test]
fn audit_history_trims_oldest_first() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let mut expected_revision = 0;
    let mut value = false;

    for _ in 0..(ROUTE_STATE_AUDIT_LIMIT + 3) {
        let outcome = set_route_state_variable(
            repo_root,
            pipeline_id,
            ["needs_project_context"],
            "needs_project_context",
            value,
            expected_revision,
        )
        .expect("mutation");

        let state = match outcome {
            RouteStateMutationOutcome::Applied(state) => state,
            other => panic!("expected success, got {other:?}"),
        };
        expected_revision = state.revision;
        value = !value;
    }

    let loaded = load_route_state(repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded.audit.len(), ROUTE_STATE_AUDIT_LIMIT);
    assert_eq!(loaded.revision, (ROUTE_STATE_AUDIT_LIMIT + 3) as u64);
    assert_eq!(
        loaded.audit.first().expect("first audit").revision,
        loaded.revision - ROUTE_STATE_AUDIT_LIMIT as u64 + 1
    );
    assert_eq!(loaded.audit.last().expect("last audit").revision, loaded.revision);
}

#[test]
fn atomic_replace_happens_under_lock() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    let lock = lock_path(&path);
    let guard = acquire_unix_lock(&lock);
    let (tx, rx) = mpsc::channel();

    let repo_root = repo_root.to_path_buf();
    let thread_repo_root = repo_root.clone();
    thread::spawn(move || {
        let outcome = set_route_state_variable(
            &thread_repo_root,
            pipeline_id,
            ["needs_project_context"],
            "needs_project_context",
            true,
            0,
        );
        tx.send(outcome).expect("send");
    });

    thread::sleep(Duration::from_millis(100));
    assert!(rx.try_recv().is_err(), "mutation should block on lock");

    release_unix_lock(&guard);
    let outcome = rx
        .recv_timeout(Duration::from_secs(2))
        .expect("mutation result")
        .expect("mutation");

    match outcome {
        RouteStateMutationOutcome::Applied(state) => {
            assert_eq!(state.revision, 1);
            assert_eq!(state.variables.get("needs_project_context"), Some(&true));
        }
        other => panic!("expected success, got {other:?}"),
    }

    let loaded = load_route_state(&repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded.revision, 1);
    assert!(path.exists());

    let dir_entries = std::fs::read_dir(path.parent().expect("parent"))
        .expect("read dir")
        .map(|entry| entry.expect("entry").file_name().to_string_lossy().to_string())
        .collect::<Vec<_>>();
    assert!(dir_entries.iter().any(|entry| entry == "pipeline.route_state.yaml"));
    assert!(dir_entries.iter().any(|entry| entry == "pipeline.route_state.lock"));
    assert!(
        dir_entries
            .iter()
            .all(|entry| !entry.contains(".tmp-")),
        "temp file should not remain after atomic replace"
    );
}
