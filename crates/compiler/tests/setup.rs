use std::fs;

use system_compiler::{
    plan_setup, render_next_safe_action_value, resolve, run_setup, setup_starter_template_bytes,
    ResolveRequest, SetupActionLabel, SetupMode, SetupRefusalKind, SetupRequest,
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

fn starter_template_bytes_for_path(path: &str) -> &'static [u8] {
    match path {
        ".system/charter/CHARTER.md" => {
            setup_starter_template_bytes(system_compiler::CanonicalArtifactKind::Charter)
        }
        ".system/feature_spec/FEATURE_SPEC.md" => {
            setup_starter_template_bytes(system_compiler::CanonicalArtifactKind::FeatureSpec)
        }
        ".system/project_context/PROJECT_CONTEXT.md" => {
            setup_starter_template_bytes(system_compiler::CanonicalArtifactKind::ProjectContext)
        }
        _ => panic!("unexpected starter path: {path}"),
    }
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
            bytes == starter_template_bytes_for_path(path),
            "starter file should match shipped template: {path}"
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
    assert_eq!(refusal.broken_subject, "canonical `.system` root");
    assert_eq!(refusal.next_safe_action, "run `system setup refresh`");
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
    assert_eq!(
        refusal.broken_subject,
        "setup-owned starter-file write target"
    );
    assert_eq!(
        refusal.next_safe_action,
        "repair the blocked target and rerun `system setup`"
    );
    assert!(
        !external.path().join("CHARTER.md").exists(),
        "setup must not write through a repo-escaping symlink"
    );
}

#[test]
fn setup_init_repairs_file_backed_invalid_system_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".system"), b"not a directory\n");

    let outcome = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Init,
            rewrite: false,
            reset_state: false,
        },
    )
    .expect("setup init should repair invalid root");

    assert_eq!(outcome.plan.resolved_mode, SetupMode::Init);
    assert!(repo_root.join(".system").is_dir());
    for path in starter_paths() {
        assert!(
            repo_root.join(path).is_file(),
            "missing starter file: {path}"
        );
    }
    assert!(outcome
        .plan
        .actions
        .iter()
        .all(|action| action.label == SetupActionLabel::Created));
}

#[cfg(unix)]
#[test]
fn setup_auto_repairs_symlinked_invalid_system_root() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let external = tempfile::tempdir().expect("external tempdir");
    let repo_root = dir.path();

    symlink(external.path(), repo_root.join(".system")).expect("system symlink");

    let outcome = run_setup(repo_root, &SetupRequest::default())
        .expect("auto setup should repair symlinked invalid root");

    assert_eq!(outcome.plan.resolved_mode, SetupMode::Init);
    assert!(repo_root.join(".system").is_dir());
    assert!(
        external
            .path()
            .read_dir()
            .expect("external dir")
            .next()
            .is_none(),
        "repair must unlink the blocking symlink without touching its target"
    );
    for path in starter_paths() {
        assert!(
            repo_root.join(path).is_file(),
            "missing starter file: {path}"
        );
    }
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
        assert_eq!(current, starter_template_bytes_for_path(path));
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

#[cfg(unix)]
#[test]
fn setup_refresh_reset_state_refuses_without_partial_deletion() {
    use std::os::unix::fs::symlink;

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
    write_file(&repo_root.join(".system/state/a.yaml"), b"a: 1\n");
    let external = tempfile::tempdir().expect("external tempdir");
    symlink(external.path(), repo_root.join(".system/state/z_symlink")).expect("state symlink");

    let refusal = run_setup(
        repo_root,
        &SetupRequest {
            mode: SetupMode::Refresh,
            rewrite: false,
            reset_state: true,
        },
    )
    .expect_err("reset-state should refuse on symlink");

    assert_eq!(refusal.kind, SetupRefusalKind::MutationRefused);
    assert!(refusal.summary.contains("symlink"), "{}", refusal.summary);
    assert!(
        repo_root.join(".system/state/a.yaml").is_file(),
        "preflight refusal must leave earlier files intact"
    );
}

#[test]
fn plan_setup_and_run_setup_agree_on_reset_state_actions() {
    let plan_dir = tempfile::tempdir().expect("plan tempdir");
    let plan_root = plan_dir.path();
    write_file(&plan_root.join(".system/charter/CHARTER.md"), b"charter\n");
    write_file(
        &plan_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature\n",
    );
    write_file(
        &plan_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"context\n",
    );
    write_file(
        &plan_root.join(".system/state/pipeline/pipeline.foundation_inputs.yaml"),
        b"pipeline state\n",
    );
    write_file(
        &plan_root.join(".system/state/pipeline/capture/cache.yaml"),
        b"capture state\n",
    );

    let request = SetupRequest {
        mode: SetupMode::Refresh,
        rewrite: false,
        reset_state: true,
    };
    let planned = plan_setup(plan_root, &request).expect("plan setup");

    let run_dir = tempfile::tempdir().expect("run tempdir");
    let run_root = run_dir.path();
    write_file(&run_root.join(".system/charter/CHARTER.md"), b"charter\n");
    write_file(
        &run_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature\n",
    );
    write_file(
        &run_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"context\n",
    );
    write_file(
        &run_root.join(".system/state/pipeline/pipeline.foundation_inputs.yaml"),
        b"pipeline state\n",
    );
    write_file(
        &run_root.join(".system/state/pipeline/capture/cache.yaml"),
        b"capture state\n",
    );

    let outcome = run_setup(run_root, &request).expect("run setup");

    assert_eq!(planned.actions, outcome.plan.actions);
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

#[test]
fn required_artifact_read_error_points_to_setup_refresh() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".system/charter/CHARTER.md")).expect("charter dir");
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature\n",
    );

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("required read-error refusal");
    assert_eq!(
        render_next_safe_action_value(&refusal.next_safe_action),
        "run `system setup refresh`"
    );
}

#[cfg(unix)]
#[test]
fn symlinked_required_artifact_points_to_setup_refresh() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    std::fs::create_dir_all(repo_root.join(".system/charter")).expect("charter dir");
    std::fs::create_dir_all(repo_root.join(".system/feature_spec")).expect("feature dir");
    let real = repo_root.join("real_charter.md");
    write_file(&real, b"charter\n");
    symlink(&real, repo_root.join(".system/charter/CHARTER.md")).expect("charter symlink");
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature\n",
    );

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("symlink refusal");
    assert_eq!(
        render_next_safe_action_value(&refusal.next_safe_action),
        "run `system setup refresh`"
    );
}
