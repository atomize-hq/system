use std::fs;

use system_compiler::{
    render_next_safe_action_value, resolve, run_setup, ResolveRequest, SetupActionLabel, SetupMode,
    SetupRefusalKind, SetupRequest,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, contents).expect("write file");
}

fn starter_paths() -> [&'static str; 3] {
    [
        ".system/charter/CHARTER.md",
        ".system/feature_spec/FEATURE_SPEC.md",
        ".system/project_context/PROJECT_CONTEXT.md",
    ]
}

#[test]
fn setup_init_creates_scaffold_and_starter_files_on_uninitialized_repo() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let outcome = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
        },
    )
    .expect("setup init");

    assert_eq!(outcome.plan.resolved_mode, SetupMode::Init);
    assert_eq!(outcome.next_command, "system doctor");
    assert_eq!(
        outcome
            .plan
            .actions
            .iter()
            .map(|action| action.path.as_str())
            .collect::<Vec<_>>(),
        starter_paths().to_vec()
    );
    assert!(outcome
        .plan
        .actions
        .iter()
        .all(|action| action.label == SetupActionLabel::Created));

    assert!(repo_root.join(".system").is_dir());
    assert!(repo_root.join(".system/charter").is_dir());
    assert!(repo_root.join(".system/feature_spec").is_dir());
    assert!(repo_root.join(".system/project_context").is_dir());
    for path in starter_paths() {
        let bytes = fs::read(repo_root.join(path)).expect("starter bytes");
        assert!(
            !bytes.is_empty(),
            "starter file should not be empty: {path}"
        );
    }
}

#[test]
fn setup_init_refuses_when_canonical_system_already_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();
    fs::create_dir_all(repo_root.join(".system/charter")).expect("charter dir");

    let refusal = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
        },
    )
    .expect_err("init should refuse");

    assert_eq!(refusal.kind, SetupRefusalKind::AlreadyInitialized);
    assert!(refusal.summary.contains("system setup refresh"));
}

#[cfg(unix)]
#[test]
fn setup_mutation_refuses_symlinked_or_escaping_paths() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let external = tempfile::tempdir().expect("external tempdir");
    let repo_root = dir.path();

    fs::create_dir_all(repo_root.join(".system")).expect("system root");
    symlink(external.path(), repo_root.join(".system/charter")).expect("symlink parent");

    let refusal = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
        },
    )
    .expect_err("symlinked parent should refuse");

    assert_eq!(refusal.kind, SetupRefusalKind::MutationRefused);
    assert!(refusal.summary.contains("symlink"), "{}", refusal.summary);
    assert!(
        !external.path().join("CHARTER.md").exists(),
        "setup must not write through a repo-escaping symlink"
    );
}

#[test]
fn setup_refresh_preserves_existing_canonical_file_bytes_by_default() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        b"custom charter\n",
    );
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"custom spec\n",
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"custom context\n",
    );

    let before = starter_paths()
        .into_iter()
        .map(|path| (path, fs::read(repo_root.join(path)).expect("read before")))
        .collect::<Vec<_>>();

    let outcome = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: false,
        },
    )
    .expect("setup refresh");

    assert!(outcome
        .plan
        .actions
        .iter()
        .all(|action| action.label == SetupActionLabel::Preserved));
    for (path, expected) in before {
        assert_eq!(
            fs::read(repo_root.join(path)).expect("read after"),
            expected
        );
    }
}

#[test]
fn setup_refresh_repairs_missing_setup_owned_scaffold_pieces_without_rewriting_preserved_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        b"keep this charter\n",
    );

    let outcome = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: false,
        },
    )
    .expect("setup refresh");

    assert_eq!(
        fs::read(repo_root.join(".system/charter/CHARTER.md")).expect("charter after"),
        b"keep this charter\n"
    );
    assert!(repo_root
        .join(".system/feature_spec/FEATURE_SPEC.md")
        .is_file());
    assert!(repo_root
        .join(".system/project_context/PROJECT_CONTEXT.md")
        .is_file());

    let labels_by_path = outcome
        .plan
        .actions
        .iter()
        .map(|action| (action.path.as_str(), action.label))
        .collect::<std::collections::BTreeMap<_, _>>();
    assert_eq!(
        labels_by_path.get(".system/charter/CHARTER.md"),
        Some(&SetupActionLabel::Preserved)
    );
    assert_eq!(
        labels_by_path.get(".system/feature_spec/FEATURE_SPEC.md"),
        Some(&SetupActionLabel::Created)
    );
    assert_eq!(
        labels_by_path.get(".system/project_context/PROJECT_CONTEXT.md"),
        Some(&SetupActionLabel::Created)
    );
}

#[test]
fn setup_refresh_rewrite_rewrites_only_setup_owned_starter_files() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    for path in starter_paths() {
        write_file(&repo_root.join(path), format!("custom {path}\n").as_bytes());
    }
    write_file(
        &repo_root.join(".system/state/pipeline/pipeline.foundation_inputs.yaml"),
        b"state: keep\n",
    );
    write_file(&repo_root.join(".system/custom/KEEP.md"), b"keep me\n");

    let extra_before = fs::read(repo_root.join(".system/custom/KEEP.md")).expect("extra before");
    let state_before =
        fs::read(repo_root.join(".system/state/pipeline/pipeline.foundation_inputs.yaml"))
            .expect("state before");

    let outcome = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: true,
            reset_state: false,
        },
    )
    .expect("setup refresh rewrite");

    assert!(outcome
        .plan
        .actions
        .iter()
        .all(|action| action.label == SetupActionLabel::Rewritten));
    for path in starter_paths() {
        let current = fs::read(repo_root.join(path)).expect("starter after");
        assert_ne!(current, format!("custom {path}\n").into_bytes());
    }
    assert_eq!(
        fs::read(repo_root.join(".system/custom/KEEP.md")).expect("extra after"),
        extra_before
    );
    assert_eq!(
        fs::read(repo_root.join(".system/state/pipeline/pipeline.foundation_inputs.yaml"))
            .expect("state after"),
        state_before
    );
}

#[test]
fn setup_refresh_reset_state_mutates_only_system_state() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".system/charter/CHARTER.md"), b"charter\n");
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature\n",
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"context\n",
    );
    write_file(
        &repo_root.join(".system/state/pipeline/pipeline.foundation_inputs.yaml"),
        b"pipeline state\n",
    );
    write_file(
        &repo_root.join(".system/state/pipeline/capture/cache.yaml"),
        b"capture state\n",
    );
    write_file(&repo_root.join(".system/custom/KEEP.md"), b"keep me\n");

    let charter_before = fs::read(repo_root.join(".system/charter/CHARTER.md")).expect("charter");
    let feature_before =
        fs::read(repo_root.join(".system/feature_spec/FEATURE_SPEC.md")).expect("feature");
    let context_before =
        fs::read(repo_root.join(".system/project_context/PROJECT_CONTEXT.md")).expect("context");

    let outcome = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true,
        },
    )
    .expect("setup refresh reset-state");

    assert_eq!(
        fs::read(repo_root.join(".system/charter/CHARTER.md")).expect("charter after"),
        charter_before
    );
    assert_eq!(
        fs::read(repo_root.join(".system/feature_spec/FEATURE_SPEC.md")).expect("feature after"),
        feature_before
    );
    assert_eq!(
        fs::read(repo_root.join(".system/project_context/PROJECT_CONTEXT.md"))
            .expect("context after"),
        context_before
    );
    assert_eq!(
        fs::read(repo_root.join(".system/custom/KEEP.md")).expect("keep after"),
        b"keep me\n"
    );
    assert!(!repo_root
        .join(".system/state/pipeline/pipeline.foundation_inputs.yaml")
        .exists());
    assert!(!repo_root
        .join(".system/state/pipeline/capture/cache.yaml")
        .exists());

    for action in outcome
        .plan
        .actions
        .iter()
        .filter(|action| action.label == SetupActionLabel::Reset)
    {
        assert!(
            action.path.starts_with(".system/state/"),
            "reset path escaped runtime state: {}",
            action.path
        );
    }
}

#[test]
fn next_safe_action_mapping_for_missing_invalid_canonical_truth_points_to_setup_family_commands() {
    let missing_root = tempfile::tempdir().expect("missing root tempdir");
    let missing_result = resolve(missing_root.path(), ResolveRequest::default()).expect("resolve");
    let missing_refusal = missing_result.refusal.expect("missing root refusal");
    assert_eq!(
        render_next_safe_action_value(&missing_refusal.next_safe_action),
        "run `system setup`"
    );

    let invalid_root = tempfile::tempdir().expect("invalid root tempdir");
    write_file(&invalid_root.path().join(".system"), b"not a directory");
    let invalid_result = resolve(invalid_root.path(), ResolveRequest::default()).expect("resolve");
    let invalid_refusal = invalid_result.refusal.expect("invalid root refusal");
    assert_eq!(
        render_next_safe_action_value(&invalid_refusal.next_safe_action),
        "run `system setup`"
    );

    let missing_artifact = tempfile::tempdir().expect("missing artifact tempdir");
    write_file(
        &missing_artifact
            .path()
            .join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature\n",
    );
    let artifact_result =
        resolve(missing_artifact.path(), ResolveRequest::default()).expect("resolve");
    let artifact_refusal = artifact_result.refusal.expect("missing artifact refusal");
    assert_eq!(
        render_next_safe_action_value(&artifact_refusal.next_safe_action),
        "run `system setup refresh`"
    );
}
