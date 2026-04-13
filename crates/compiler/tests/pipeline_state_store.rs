#[allow(dead_code)]
#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use system_compiler::{
    build_route_basis, effective_route_basis_run, load_pipeline_definition, load_route_state,
    load_route_state_with_supported_variables, persist_route_basis, resolve_pipeline_route,
    set_route_state, supported_route_state_variables, RouteBasisPersistOutcome, RouteState,
    RouteStateMutation, RouteStateMutationOutcome, RouteStateMutationRefusal, RouteStateReadError,
    RouteStateStoreError, RouteStateValue, RouteVariables, ROUTE_STATE_AUDIT_LIMIT,
    ROUTE_STATE_SCHEMA_VERSION,
};

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn seed_run_inventory(repo_root: &Path) {
    write_file(&repo_root.join("runners/codex-cli.md"), "# runner");
    write_file(&repo_root.join("runners/*.md"), "# ignored noise");
    write_file(
        &repo_root.join("profiles/python-uv/profile.yaml"),
        "kind: profile\nid: python-uv\n",
    );
    write_file(
        &repo_root.join("profiles/_template/profile.yaml"),
        "kind: profile\nid: _template\n",
    );
    write_file(&repo_root.join("profiles/.DS_Store"), "noise");
}

fn repo_root_string(repo_root: &Path) -> String {
    repo_root.to_string_lossy().into_owned()
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
        .truncate(false)
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
fn route_basis_defaults_runner_and_profile_into_run_snapshot() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");

    let route_basis = build_route_basis(&repo_root, &definition, &state, &route).expect("basis");

    assert_eq!(route_basis.run.runner.as_deref(), Some("codex-cli"));
    assert_eq!(route_basis.run.profile.as_deref(), Some("python-uv"));
    assert_eq!(
        route_basis.run,
        effective_route_basis_run(&repo_root, &definition, &state)
    );
}

#[test]
fn persist_route_basis_accepts_defaulted_run_snapshot_against_unset_state_run() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");
    let route_basis = build_route_basis(&repo_root, &definition, &state, &route).expect("basis");

    let outcome =
        persist_route_basis(&repo_root, &definition.header.id, route_basis).expect("persist");

    match outcome {
        RouteBasisPersistOutcome::Applied(state) => {
            assert_eq!(
                state
                    .route_basis
                    .as_ref()
                    .and_then(|basis| basis.run.runner.as_deref()),
                Some("codex-cli")
            );
            assert_eq!(
                state
                    .route_basis
                    .as_ref()
                    .and_then(|basis| basis.run.profile.as_deref()),
                Some("python-uv")
            );
        }
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected route basis persist to apply, got {refusal:?}")
        }
    }
}

#[test]
fn route_basis_round_trips_when_written_by_resolve() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let definition = load_pipeline_definition(&repo_root, "pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture");
    let supported_variables = supported_route_state_variables(&definition);
    let state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("state");
    let route = resolve_pipeline_route(
        &definition,
        &RouteVariables::new(state.routing.clone()).expect("route variables"),
    )
    .expect("route");
    let route_basis = build_route_basis(&repo_root, &definition, &state, &route).expect("basis");

    let outcome = persist_route_basis(&repo_root, &definition.header.id, route_basis.clone())
        .expect("persist");

    let persisted_state = match outcome {
        RouteBasisPersistOutcome::Applied(state) => *state,
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected route basis persist to apply, got {refusal:?}")
        }
    };
    assert_eq!(persisted_state.route_basis.as_ref(), Some(&route_basis));

    let reloaded_state = load_route_state_with_supported_variables(
        &repo_root,
        &definition.header.id,
        &supported_variables,
    )
    .expect("reload state");
    assert_eq!(reloaded_state.route_basis.as_ref(), Some(&route_basis));
}

#[test]
fn missing_state_loads_as_empty_and_round_trips_mixed_fields() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let expected_repo_root = repo_root_string(repo_root);
    let pipeline_id = "pipeline.route_state";
    seed_run_inventory(repo_root);

    let empty = load_route_state(repo_root, pipeline_id).expect("empty state");
    assert_eq!(empty, RouteState::empty(pipeline_id));
    assert!(empty.routing.is_empty());
    assert_eq!(empty.refs.charter_ref, None);
    assert_eq!(empty.refs.project_context_ref, None);
    assert_eq!(empty.run.runner, None);
    assert_eq!(empty.run.profile, None);
    assert_eq!(empty.run.repo_root, None);

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
        0,
    )
    .expect("routing mutation");
    let mut state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.schema_version, ROUTE_STATE_SCHEMA_VERSION);
    assert_eq!(state.pipeline_id, pipeline_id);
    assert_eq!(state.revision, 1);
    assert_eq!(state.routing.get("needs_project_context"), Some(&true));
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );
    assert_eq!(state.audit.len(), 1);
    assert_eq!(state.audit[0].field_path, "routing.needs_project_context");
    assert_eq!(state.audit[0].value, RouteStateValue::Bool(true));

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
        1,
    )
    .expect("run mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 2);
    assert_eq!(state.run.runner.as_deref(), Some("codex-cli"));
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );
    assert_eq!(state.audit[1].field_path, "run.runner");
    assert_eq!(
        state.audit[1].value,
        RouteStateValue::String("codex-cli".to_string())
    );

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
        2,
    )
    .expect("profile mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 3);
    assert_eq!(state.run.profile.as_deref(), Some("python-uv"));
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
        3,
    )
    .expect("charter ref mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 4);
    assert_eq!(
        state.refs.charter_ref.as_deref(),
        Some("artifacts/charter/CHARTER.md")
    );
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context", "charter_gaps_detected"],
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
        4,
    )
    .expect("project context ref mutation");
    state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 5);
    assert_eq!(
        state.refs.project_context_ref.as_deref(),
        Some("artifacts/project_context/PROJECT_CONTEXT.md")
    );
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );

    let loaded = load_route_state(repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded, state);
}

#[test]
fn legacy_m1_state_without_route_basis_still_loads() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 2
routing:
  needs_project_context: false
refs:
  charter_ref: artifacts/charter/CHARTER.md
run:
  runner: codex-cli
  profile: python-uv
audit:
  - revision: 1
    field_path: run.runner
    value: codex-cli
  - revision: 2
    field_path: routing.needs_project_context
    value: false
"#,
    );

    let state = load_route_state(repo_root, pipeline_id).expect("legacy state loads");
    assert_eq!(state.schema_version, "m1-pipeline-state-v2");
    assert_eq!(state.route_basis, None);
    assert_eq!(state.run.runner.as_deref(), Some("codex-cli"));
    assert_eq!(state.run.profile.as_deref(), Some("python-uv"));
    assert_eq!(state.routing.get("needs_project_context"), Some(&false));
}

#[test]
fn legacy_state_without_repo_root_backfills_on_next_successful_mutation() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let expected_repo_root = repo_root_string(repo_root);
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run:
  runner: codex-cli
  profile: python-uv
audit:
  - revision: 1
    field_path: run.runner
    value: codex-cli
"#,
    );

    let loaded = load_route_state(repo_root, pipeline_id).expect("legacy state loads");
    assert_eq!(loaded.run.repo_root, None);

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
        1,
    )
    .expect("mutation");
    let state = match outcome {
        RouteStateMutationOutcome::Applied(state) => *state,
        other => panic!("expected success, got {other:?}"),
    };
    assert_eq!(state.revision, 2);
    assert_eq!(state.run.runner.as_deref(), Some("codex-cli"));
    assert_eq!(state.run.profile.as_deref(), Some("python-uv"));
    assert_eq!(
        state.run.repo_root.as_deref(),
        Some(expected_repo_root.as_str())
    );

    let reloaded = load_route_state(repo_root, pipeline_id).expect("reloaded state");
    assert_eq!(reloaded, state);
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
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run: {}
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
fn malformed_state_refuses_wrong_nested_scalar_types() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing:
  needs_project_context: true
refs:
  charter_ref: true
run: {}
audit:
  - revision: 1
    field_path: run.runner
    value: true
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
fn malformed_state_refuses_invalid_repo_root_shape() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run:
  repo_root: relative/root
audit: []
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(reason.contains("run.repo_root"), "{reason}");
            assert!(reason.contains("must be absolute"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn malformed_route_basis_is_distinct_from_malformed_route_state() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let malformed_route_basis_path =
        pipeline_proof_corpus_support::install_state_seed(&repo_root, "malformed_route_basis.yaml");
    let malformed_route_basis_err = load_route_state_with_supported_variables(
        &repo_root,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        &std::collections::BTreeSet::from([
            "needs_project_context".to_string(),
            "charter_gaps_detected".to_string(),
        ]),
    )
    .expect_err("malformed route basis should refuse");

    match malformed_route_basis_err {
        RouteStateReadError::MalformedState { path, reason } => {
            assert_eq!(path, malformed_route_basis_path);
            assert!(reason.contains("route_basis"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }

    let malformed_route_state_path =
        pipeline_proof_corpus_support::install_state_seed(&repo_root, "malformed_route_state.yaml");
    let malformed_route_state_err = load_route_state_with_supported_variables(
        &repo_root,
        pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID,
        &std::collections::BTreeSet::from([
            "needs_project_context".to_string(),
            "charter_gaps_detected".to_string(),
        ]),
    )
    .expect_err("malformed route state should refuse");

    match malformed_route_state_err {
        RouteStateReadError::MalformedState { path, reason } => {
            assert_eq!(path, malformed_route_state_path);
            assert!(!reason.contains("route_basis"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn load_with_supported_variables_refuses_unsupported_persisted_routing_variables() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing:
  charter_gaps_detected: true
refs: {}
run: {}
audit:
  - revision: 1
    field_path: routing.charter_gaps_detected
    value: true
"#,
    );

    let err = load_route_state_with_supported_variables(
        repo_root,
        pipeline_id,
        &std::collections::BTreeSet::from(["needs_project_context".to_string()]),
    )
    .expect_err("malformed state");

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

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: true,
        },
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
fn invalid_ref_value_is_rejected_before_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";

    let err = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RefCharterRef {
            value: "/tmp/CHARTER.md".to_string(),
        },
        0,
    )
    .expect_err("invalid mutation");

    match err {
        RouteStateStoreError::InvalidMutation { reason } => {
            assert!(reason.contains("must not be absolute"), "{reason}");
        }
        other => panic!("expected invalid mutation error, got {other:?}"),
    }
}

#[test]
fn invalid_runner_value_is_rejected_before_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    seed_run_inventory(repo_root);

    let err = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RunRunner {
            value: "not-a-runner".to_string(),
        },
        0,
    )
    .expect_err("invalid mutation");

    match err {
        RouteStateStoreError::InvalidMutation { reason } => {
            assert!(reason.contains("run.runner"), "{reason}");
            assert!(reason.contains("runners/"), "{reason}");
            assert!(reason.contains("not-a-runner"), "{reason}");
        }
        other => panic!("expected invalid mutation error, got {other:?}"),
    }
}

#[test]
fn invalid_profile_value_is_rejected_before_write() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    seed_run_inventory(repo_root);

    let err = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RunProfile {
            value: "not-a-profile".to_string(),
        },
        0,
    )
    .expect_err("invalid mutation");

    match err {
        RouteStateStoreError::InvalidMutation { reason } => {
            assert!(reason.contains("run.profile"), "{reason}");
            assert!(reason.contains("profiles/"), "{reason}");
            assert!(reason.contains("not-a-profile"), "{reason}");
        }
        other => panic!("expected invalid mutation error, got {other:?}"),
    }
}

#[test]
fn revision_conflict_refuses_without_overwrite() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 2
routing:
  needs_project_context: true
refs: {}
run:
  runner: codex-cli
audit:
  - revision: 1
    field_path: routing.needs_project_context
    value: true
  - revision: 2
    field_path: run.runner
    value: codex-cli
"#,
    );

    let outcome = set_route_state(
        repo_root,
        pipeline_id,
        ["needs_project_context"],
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
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
    assert_eq!(loaded.routing.get("needs_project_context"), Some(&true));
    assert_eq!(loaded.run.runner.as_deref(), Some("codex-cli"));
}

#[test]
fn audit_history_trims_oldest_first_across_mixed_fields() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let mut expected_revision = 0;
    seed_run_inventory(repo_root);

    for index in 0..(ROUTE_STATE_AUDIT_LIMIT + 3) {
        let mutation = match index % 4 {
            0 => RouteStateMutation::RoutingVariable {
                variable: "needs_project_context".to_string(),
                value: index % 2 == 0,
            },
            1 => RouteStateMutation::RunRunner {
                value: "codex-cli".to_string(),
            },
            2 => RouteStateMutation::RunProfile {
                value: "python-uv".to_string(),
            },
            _ => RouteStateMutation::RefCharterRef {
                value: format!("artifacts/charter/CHARTER-{index}.md"),
            },
        };

        let outcome = set_route_state(
            repo_root,
            pipeline_id,
            ["needs_project_context"],
            mutation,
            expected_revision,
        )
        .expect("mutation");

        let state = match outcome {
            RouteStateMutationOutcome::Applied(state) => *state,
            other => panic!("expected success, got {other:?}"),
        };
        expected_revision = state.revision;
    }

    let loaded = load_route_state(repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded.audit.len(), ROUTE_STATE_AUDIT_LIMIT);
    assert_eq!(loaded.revision, (ROUTE_STATE_AUDIT_LIMIT + 3) as u64);
    assert_eq!(
        loaded.audit.first().expect("first audit").revision,
        loaded.revision - ROUTE_STATE_AUDIT_LIMIT as u64 + 1
    );
    assert_eq!(
        loaded.audit.last().expect("last audit").revision,
        loaded.revision
    );
}

#[test]
fn malformed_state_refuses_unallowlisted_run_runner() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 1
routing: {}
refs: {}
run:
  runner: not-a-runner
audit:
  - revision: 1
    field_path: run.runner
    value: not-a-runner
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(reason.contains("run.runner"), "{reason}");
            assert!(reason.contains("not-a-runner"), "{reason}");
            assert!(reason.contains("runners/"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}

#[test]
fn malformed_state_refuses_unallowlisted_run_profile_audit_entry() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    let pipeline_id = "pipeline.route_state";
    let path = state_path(repo_root, pipeline_id);
    seed_run_inventory(repo_root);
    write_file(
        &path,
        r#"---
schema_version: m1-pipeline-state-v2
pipeline_id: pipeline.route_state
revision: 2
routing: {}
refs: {}
run:
  profile: python-uv
audit:
  - revision: 1
    field_path: run.profile
    value: not-a-profile
  - revision: 2
    field_path: run.profile
    value: python-uv
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("malformed state");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(reason.contains("run.profile"), "{reason}");
            assert!(reason.contains("not-a-profile"), "{reason}");
            assert!(reason.contains("profiles/"), "{reason}");
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
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
        let outcome = set_route_state(
            &thread_repo_root,
            pipeline_id,
            ["needs_project_context"],
            RouteStateMutation::RoutingVariable {
                variable: "needs_project_context".to_string(),
                value: true,
            },
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
            assert_eq!(state.routing.get("needs_project_context"), Some(&true));
        }
        other => panic!("expected success, got {other:?}"),
    }

    let loaded = load_route_state(&repo_root, pipeline_id).expect("loaded state");
    assert_eq!(loaded.revision, 1);
    assert!(path.exists());

    let dir_entries = std::fs::read_dir(path.parent().expect("parent"))
        .expect("read dir")
        .map(|entry| {
            entry
                .expect("entry")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .collect::<Vec<_>>();
    assert!(dir_entries
        .iter()
        .any(|entry| entry == "pipeline.route_state.yaml"));
    assert!(dir_entries
        .iter()
        .any(|entry| entry == "pipeline.route_state.lock"));
    assert!(
        dir_entries.iter().all(|entry| !entry.contains(".tmp-")),
        "temp file should not remain after atomic replace"
    );
}

#[test]
fn legacy_flat_schema_refuses_explicitly() {
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
"#,
    );

    let err = load_route_state(repo_root, pipeline_id).expect_err("legacy schema should refuse");
    match err {
        RouteStateReadError::MalformedState { reason, .. } => {
            assert!(
                reason.contains("unknown field `variables`")
                    || reason.contains("unexpected schema_version"),
                "{reason}"
            );
        }
        other => panic!("expected malformed-state refusal, got {other:?}"),
    }
}
