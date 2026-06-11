use handbook_pipeline::setup::{apply_runtime_state_reset, plan_runtime_state_reset};
use std::fs;

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create parent dirs");
    }
    fs::write(path, contents).expect("write file");
}

#[test]
fn runtime_state_reset_plan_stays_inside_state_root() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/state/pipeline/pipeline.foundation_inputs.yaml"),
        b"pipeline state
",
    );
    write_file(
        &repo_root.join(".handbook/state/pipeline/capture/cache.yaml"),
        b"capture state
",
    );
    write_file(
        &repo_root.join(".handbook/custom/KEEP.md"),
        b"keep me
",
    );

    let plan = plan_runtime_state_reset(repo_root).expect("plan runtime state reset");
    assert_eq!(
        plan.paths(),
        &[
            ".handbook/state/pipeline".to_string(),
            ".handbook/state/pipeline/capture".to_string(),
            ".handbook/state/pipeline/capture/cache.yaml".to_string(),
            ".handbook/state/pipeline/pipeline.foundation_inputs.yaml".to_string(),
        ]
    );
    assert!(plan
        .paths()
        .iter()
        .all(|path| path.starts_with(".handbook/state/")));
}

#[test]
fn runtime_state_reset_apply_removes_only_runtime_state_entries() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/state/pipeline/pipeline.foundation_inputs.yaml"),
        b"pipeline state
",
    );
    write_file(
        &repo_root.join(".handbook/state/pipeline/capture/cache.yaml"),
        b"capture state
",
    );
    write_file(
        &repo_root.join(".handbook/custom/KEEP.md"),
        b"keep me
",
    );

    let plan = plan_runtime_state_reset(repo_root).expect("plan runtime state reset");
    apply_runtime_state_reset(&plan).expect("apply runtime state reset");

    assert!(!repo_root
        .join(".handbook/state/pipeline/pipeline.foundation_inputs.yaml")
        .exists());
    assert!(!repo_root
        .join(".handbook/state/pipeline/capture/cache.yaml")
        .exists());
    assert_eq!(
        fs::read(repo_root.join(".handbook/custom/KEEP.md")).expect("custom file after"),
        b"keep me
"
    );
}

#[cfg(unix)]
#[test]
fn runtime_state_reset_plan_refuses_symlink_without_partial_deletion() {
    use std::os::unix::fs::symlink;

    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".handbook/state/a.yaml"),
        b"a: 1
",
    );
    let external = tempfile::tempdir().expect("external tempdir");
    symlink(external.path(), repo_root.join(".handbook/state/z_symlink")).expect("state symlink");

    let error = plan_runtime_state_reset(repo_root).expect_err("symlink should refuse");
    assert!(error.contains("symlink"), "{error}");
    assert!(repo_root.join(".handbook/state/a.yaml").is_file());
}
