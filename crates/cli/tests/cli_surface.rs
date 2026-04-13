use std::process::{Command, Output};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_system"))
}

fn binary_in(dir: &std::path::Path) -> Command {
    let mut cmd = binary();
    cmd.current_dir(dir);
    cmd
}

fn run_in(dir: &std::path::Path, args: &[&str]) -> Output {
    binary_in(dir)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("run `{}`: {err}", args.join(" ")))
}

fn workspace_root() -> std::path::PathBuf {
    let start = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in start.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if !cargo_toml.is_file() {
            continue;
        }
        let Ok(contents) = std::fs::read_to_string(&cargo_toml) else {
            continue;
        };
        if contents.contains("[workspace]") {
            return ancestor.to_path_buf();
        }
    }

    panic!(
        "failed to locate workspace root from CARGO_MANIFEST_DIR={}",
        env!("CARGO_MANIFEST_DIR")
    );
}

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn planning_ready_repo_with_nested_cwd() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let nested = root.join("work/nested");
    std::fs::create_dir_all(&nested).expect("nested cwd");

    (dir, nested)
}

fn nested_git_repo_with_nested_cwd() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".git")).expect("git root");
    let nested = root.join("work/nested");
    std::fs::create_dir_all(&nested).expect("nested cwd");

    (dir, nested)
}

fn execution_demo_repo_with_nested_cwd() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".git")).expect("git root");
    write_file(
        &root.join("tests/fixtures/execution_demo/basic/.system/charter/CHARTER.md"),
        b"demo charter",
    );
    write_file(
        &root.join("tests/fixtures/execution_demo/basic/.system/feature_spec/FEATURE_SPEC.md"),
        b"demo feature",
    );

    let nested = root.join("work/nested");
    std::fs::create_dir_all(&nested).expect("nested cwd");

    (dir, nested)
}

fn committed_execution_demo_fixture_dir() -> std::path::PathBuf {
    workspace_root().join("tests/fixtures/execution_demo/basic")
}

fn repair_to_ready(root: &std::path::Path) {
    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
}

fn partial_system_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    write_file(
        dir.path().join(".system/charter/CHARTER.md").as_path(),
        b"charter",
    );
    dir
}

fn malformed_optional_project_context_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    repair_to_ready(root);
    std::fs::create_dir_all(root.join(".system/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context directory");

    dir
}

fn nested_git_repo_inside_managed_parent_with_nested_cwd() -> (tempfile::TempDir, std::path::PathBuf)
{
    let dir = tempfile::tempdir().expect("tempdir");
    let parent = dir.path().join("parent");
    let child = parent.join("child");
    let nested = child.join("work/nested");

    write_file(
        &parent.join(".system/charter/CHARTER.md"),
        b"parent charter",
    );
    write_file(
        &parent.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"parent feature",
    );
    std::fs::create_dir_all(child.join(".git")).expect("child git root");
    std::fs::create_dir_all(&nested).expect("nested cwd");

    (dir, nested)
}

fn copy_committed_file(repo_root: &std::path::Path, relative_path: &str) {
    let source = workspace_root().join(relative_path);
    let target = repo_root.join(relative_path);

    if let Some(parent) = target.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }

    std::fs::copy(&source, &target)
        .unwrap_or_else(|err| panic!("copy {} -> {}: {err}", source.display(), target.display()));
}

fn shared_pipeline_proof_corpus_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    for path in [
        "pipelines/foundation_inputs.yaml",
        "core/stages/00_base.md",
        "core/stages/04_charter_inputs.md",
        "core/stages/05_charter_synthesize.md",
        "core/stages/06_project_context_interview.md",
        "core/stages/07_foundation_pack.md",
    ] {
        copy_committed_file(&root, path);
    }

    (dir, root)
}

fn activation_drift_pipeline_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    write_file(
        &root.join("core/stages/00_base.md"),
        b"---\nkind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\nactivation:\n  when:\n    any:\n      - variables.needs_project_context == true\n---\n# base\n",
    );
    write_file(
        &root.join("pipelines/drift.yaml"),
        b"---\nkind: pipeline\nid: pipeline.drift\nversion: 0.1.0\ntitle: Drift\ndescription: drift\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );

    (dir, root)
}

fn invalid_pipeline_id_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    write_file(
        &root.join("core/stages/00_base.md"),
        b"---\nkind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\n---\n# base\n",
    );
    write_file(
        &root.join("pipelines/bad-id.yaml"),
        b"---\nkind: pipeline\nid: pipeline.bad/path\nversion: 0.1.0\ntitle: Bad Id\ndescription: bad\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );

    (dir, root)
}

#[test]
fn help_lists_setup_first() {
    let output = binary().arg("--help").output().expect("help should run");

    assert!(output.status.success(), "help should succeed");

    let stdout = String::from_utf8(output.stdout).expect("help is utf-8");
    let command_lines = command_section_lines(&stdout);

    assert_eq!(
        command_lines.len(),
        5,
        "expected five command lines in help"
    );
    assert!(
        command_lines[0].starts_with("setup "),
        "setup should be first: {command_lines:?}"
    );
    assert!(
        command_lines[1].starts_with("pipeline "),
        "pipeline should be second: {command_lines:?}"
    );
    assert!(
        command_lines[2].starts_with("generate "),
        "generate should be third: {command_lines:?}"
    );
    assert!(
        command_lines[3].starts_with("inspect "),
        "inspect should be fourth: {command_lines:?}"
    );
    assert!(
        command_lines[4].starts_with("doctor "),
        "doctor should be fifth: {command_lines:?}"
    );
}

#[test]
fn pipeline_help_lists_supported_surface() {
    let root = workspace_root();

    let output = run_in(root.as_path(), &["pipeline", "--help"]);
    assert!(output.status.success(), "pipeline help should succeed");

    let stdout = String::from_utf8(output.stdout).expect("help stdout is utf-8");
    let command_lines = command_section_lines(&stdout);

    assert_eq!(
        command_lines.len(),
        4,
        "expected four pipeline command lines"
    );
    assert!(
        command_lines[0].starts_with("list "),
        "list should be first: {command_lines:?}"
    );
    assert!(
        command_lines[1].starts_with("show "),
        "show should be second: {command_lines:?}"
    );
    assert!(
        command_lines[2].starts_with("resolve "),
        "resolve should be third: {command_lines:?}"
    );
    assert!(
        command_lines[3].starts_with("state "),
        "state should be fourth: {command_lines:?}"
    );
    assert!(
        stdout.contains("Pipeline operator surface"),
        "expected pipeline help title: {stdout}"
    );
}

#[test]
fn pipeline_state_help_lists_set() {
    let root = workspace_root();

    let output = run_in(root.as_path(), &["pipeline", "state", "--help"]);
    assert!(
        output.status.success(),
        "pipeline state help should succeed"
    );

    let stdout = String::from_utf8(output.stdout).expect("help stdout is utf-8");
    let command_lines = command_section_lines(&stdout);

    assert_eq!(
        command_lines.len(),
        1,
        "expected one pipeline state command line"
    );
    assert!(
        command_lines[0].starts_with("set "),
        "set should be the only state subcommand: {command_lines:?}"
    );
    assert!(
        stdout.contains("Set one supported route-state field"),
        "expected pipeline state help text: {stdout}"
    );
}

#[test]
fn pipeline_list_and_show_use_canonical_id_discovery() {
    let root = workspace_root();

    let list = run_in(root.as_path(), &["pipeline", "list"]);
    assert!(list.status.success(), "pipeline list should succeed");
    let list_stdout = String::from_utf8(list.stdout).expect("list stdout is utf-8");
    assert!(list_stdout.contains("PIPELINE INVENTORY"));
    assert!(list_stdout.contains("PIPELINE COUNT: 4"));
    assert!(list_stdout.contains("PIPELINE: pipeline.foundation"));
    assert!(list_stdout.contains("SOURCE: pipelines/foundation.yaml"));

    let show = run_in(
        root.as_path(),
        &["pipeline", "show", "--id", "pipeline.foundation"],
    );
    assert!(show.status.success(), "pipeline show should succeed");
    let show_stdout = String::from_utf8(show.stdout).expect("show stdout is utf-8");
    assert!(show_stdout.contains("PIPELINE: pipeline.foundation"));
    assert!(show_stdout.contains("DEFAULTS:"));
    assert!(show_stdout.contains("stage.05_charter_interview"));
    assert!(show_stdout.contains("core/stages/05_charter_interview.md"));

    let shorthand = run_in(root.as_path(), &["pipeline", "show", "--id", "00_base"]);
    assert!(shorthand.status.success(), "stage shorthand should resolve");
    let shorthand_stdout = String::from_utf8(shorthand.stdout).expect("stdout is utf-8");
    assert!(shorthand_stdout.contains("STAGE: stage.00_base"));
    assert!(shorthand_stdout.contains("pipeline.foundation"));

    let ambiguous_repo = tempfile::tempdir().expect("tempdir");
    let ambiguous_root = ambiguous_repo.path();
    write_file(
        &ambiguous_root.join("core/stages/alpha.md"),
        b"---\nkind: stage\nid: stage.alpha\nversion: 0.1.0\ntitle: Alpha Stage\ndescription: alpha\n---\n# alpha\n",
    );
    write_file(
        &ambiguous_root.join("pipelines/alpha.yaml"),
        b"---\nkind: pipeline\nid: pipeline.alpha\nversion: 0.1.0\ntitle: Alpha Pipeline\ndescription: alpha\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.alpha\n    file: core/stages/alpha.md\n",
    );

    let ambiguous = run_in(ambiguous_root, &["pipeline", "show", "--id", "alpha"]);
    assert!(
        !ambiguous.status.success(),
        "ambiguous shorthand should refuse"
    );
    let ambiguous_stdout = String::from_utf8(ambiguous.stdout).expect("stdout is utf-8");
    assert!(ambiguous_stdout.contains("ambiguous selector `alpha`"));
    assert!(ambiguous_stdout.contains("pipeline.alpha"));
    assert!(ambiguous_stdout.contains("stage.alpha"));
    assert!(ambiguous_stdout.contains("NEXT SAFE ACTION"));
    assert!(ambiguous_stdout.contains("rename the conflicting ids"));

    let unknown = run_in(root.as_path(), &["pipeline", "show", "--id", "missing-id"]);
    assert!(!unknown.status.success(), "unknown selector should refuse");
    let unknown_stdout = String::from_utf8(unknown.stdout).expect("stdout is utf-8");
    assert!(unknown_stdout.contains("unknown pipeline selector `missing-id`"));
    assert!(unknown_stdout.contains("pipeline list"));
    assert!(unknown_stdout.contains("NEXT SAFE ACTION"));
    assert!(unknown_stdout.contains("full canonical id"));

    let path_like = run_in(
        root.as_path(),
        &["pipeline", "show", "--id", "pipelines/foundation.yaml"],
    );
    assert!(
        !path_like.status.success(),
        "path-like selector should refuse"
    );
    let path_like_stdout = String::from_utf8(path_like.stdout).expect("stdout is utf-8");
    assert!(path_like_stdout.contains("raw file paths are evidence only"));
    assert!(path_like_stdout.contains("NEXT SAFE ACTION"));
    assert!(path_like_stdout.contains("canonical pipeline or stage id"));
}

#[test]
fn pipeline_resolve_and_state_set_use_compiler_route_state_handoff() {
    let (_dir, root) = shared_pipeline_proof_corpus_repo();

    let first_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        first_resolve.status.success(),
        "pipeline resolve should succeed"
    );
    let first_resolve_stdout = String::from_utf8(first_resolve.stdout).expect("stdout is utf-8");
    assert_eq!(
        first_resolve_stdout.trim_end(),
        concat!(
            "OUTCOME: RESOLVED\n",
            "PIPELINE: pipeline.foundation_inputs\n",
            "STATE REVISION: 0\n",
            "ROUTE:\n",
            "  1. stage.00_base | active\n",
            "  2. stage.04_charter_inputs | active\n",
            "  3. stage.05_charter_synthesize | active\n",
            "  4. stage.06_project_context_interview | next\n",
            "     REASON: missing route variables: charter_gaps_detected, needs_project_context\n",
            "  5. stage.07_foundation_pack | blocked\n",
            "     REASON: blocked by unresolved stage stage.06_project_context_interview (next)"
        )
    );

    let applied = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
    );
    assert!(
        applied.status.success(),
        "pipeline state set should succeed"
    );
    let applied_stdout = String::from_utf8(applied.stdout).expect("stdout is utf-8");
    assert_eq!(
        applied_stdout.trim_end(),
        concat!(
            "OUTCOME: APPLIED\n",
            "PIPELINE: pipeline.foundation_inputs\n",
            "REVISION: 1\n",
            "ROUTING:\n",
            "  needs_project_context = true\n",
            "REFS:\n",
            "  charter_ref = <unset>\n",
            "  project_context_ref = <unset>\n",
            "RUN:\n",
            "  runner = <unset>\n",
            "  profile = <unset>"
        )
    );

    let activation_applied = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "charter_gaps_detected=true",
        ],
    );
    assert!(
        activation_applied.status.success(),
        "activation-only route variable should be supported"
    );
    let activation_applied_stdout =
        String::from_utf8(activation_applied.stdout).expect("stdout is utf-8");
    assert_eq!(
        activation_applied_stdout.trim_end(),
        concat!(
            "OUTCOME: APPLIED\n",
            "PIPELINE: pipeline.foundation_inputs\n",
            "REVISION: 2\n",
            "ROUTING:\n",
            "  charter_gaps_detected = true\n",
            "  needs_project_context = true\n",
            "REFS:\n",
            "  charter_ref = <unset>\n",
            "  project_context_ref = <unset>\n",
            "RUN:\n",
            "  runner = <unset>\n",
            "  profile = <unset>"
        )
    );

    let second_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        second_resolve.status.success(),
        "pipeline resolve should succeed after mutation"
    );
    let second_resolve_stdout = String::from_utf8(second_resolve.stdout).expect("stdout is utf-8");
    assert_eq!(
        second_resolve_stdout.trim_end(),
        concat!(
            "OUTCOME: RESOLVED\n",
            "PIPELINE: pipeline.foundation_inputs\n",
            "STATE REVISION: 2\n",
            "ROUTE:\n",
            "  1. stage.00_base | active\n",
            "  2. stage.04_charter_inputs | active\n",
            "  3. stage.05_charter_synthesize | active\n",
            "  4. stage.06_project_context_interview | active\n",
            "  5. stage.07_foundation_pack | active"
        )
    );
}

#[test]
fn pipeline_state_set_field_surfaces_accept_run_and_refs() {
    let (_dir, root) = shared_pipeline_proof_corpus_repo();

    let runner_applied = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.runner=codex-cli",
        ],
    );
    assert!(
        runner_applied.status.success(),
        "run.runner field should succeed"
    );
    let runner_stdout = String::from_utf8(runner_applied.stdout).expect("stdout is utf-8");
    assert_eq!(
        runner_stdout.trim_end(),
        concat!(
            "OUTCOME: APPLIED\n",
            "PIPELINE: pipeline.foundation_inputs\n",
            "REVISION: 1\n",
            "ROUTING:\n",
            "  <empty>\n",
            "REFS:\n",
            "  charter_ref = <unset>\n",
            "  project_context_ref = <unset>\n",
            "RUN:\n",
            "  runner = codex-cli\n",
            "  profile = <unset>"
        )
    );

    let ref_applied = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.charter_ref=artifacts/charter/CHARTER.md",
        ],
    );
    assert!(
        ref_applied.status.success(),
        "refs.charter_ref field should succeed"
    );
    let ref_stdout = String::from_utf8(ref_applied.stdout).expect("stdout is utf-8");
    assert_eq!(
        ref_stdout.trim_end(),
        concat!(
            "OUTCOME: APPLIED\n",
            "PIPELINE: pipeline.foundation_inputs\n",
            "REVISION: 2\n",
            "ROUTING:\n",
            "  <empty>\n",
            "REFS:\n",
            "  charter_ref = artifacts/charter/CHARTER.md\n",
            "  project_context_ref = <unset>\n",
            "RUN:\n",
            "  runner = codex-cli\n",
            "  profile = <unset>"
        )
    );
}

#[test]
fn pipeline_state_set_field_rejects_invalid_paths_and_values() {
    let (_dir, root) = shared_pipeline_proof_corpus_repo();

    let invalid_path = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.unknown=artifacts/charter/CHARTER.md",
        ],
    );
    assert!(
        !invalid_path.status.success(),
        "invalid field path should refuse"
    );
    let invalid_path_stdout = String::from_utf8(invalid_path.stdout).expect("stdout is utf-8");
    assert_eq!(
        invalid_path_stdout.trim_end(),
        "REFUSED: unsupported --field path `refs.unknown`; expected one of `run.runner`, `run.profile`, `refs.charter_ref`, or `refs.project_context_ref`"
    );

    let invalid_value = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.charter_ref=/tmp/CHARTER.md",
        ],
    );
    assert!(
        !invalid_value.status.success(),
        "invalid field value should refuse"
    );
    let invalid_value_stdout = String::from_utf8(invalid_value.stdout).expect("stdout is utf-8");
    assert_eq!(
        invalid_value_stdout.trim_end(),
        "REFUSED: route state mutation error: route state mutation is invalid: repo-relative ref `/tmp/CHARTER.md` must not be absolute"
    );
}

#[test]
fn pipeline_commands_refuse_activation_drift_before_operating() {
    let (_dir, root) = activation_drift_pipeline_repo();

    for args in [
        vec!["pipeline", "list"],
        vec!["pipeline", "show", "--id", "pipeline.drift"],
        vec!["pipeline", "resolve", "--id", "pipeline.drift"],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "pipeline.drift",
            "--var",
            "needs_project_context=true",
        ],
    ] {
        let output = run_in(root.as_path(), &args);
        assert!(
            !output.status.success(),
            "command should refuse: {:?}",
            args
        );

        let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
        assert!(
            stdout.contains("REFUSED: pipeline catalog error: failed to load pipeline definition"),
            "expected catalog refusal for {:?}: {stdout}",
            args
        );
        assert!(
            stdout.contains("has activation drift"),
            "expected activation drift for {:?}: {stdout}",
            args
        );
        assert!(
            stdout.contains("stage `stage.00_base` file `core/stages/00_base.md`"),
            "expected stage identification for {:?}: {stdout}",
            args
        );
    }
}

#[test]
fn pipeline_list_refuses_invalid_canonical_ids_before_advertising_inventory() {
    let (_dir, root) = invalid_pipeline_id_repo();

    let output = run_in(root.as_path(), &["pipeline", "list"]);
    assert!(
        !output.status.success(),
        "pipeline list should refuse invalid canonical ids"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("REFUSED: pipeline catalog error: failed to load pipeline definition"),
        "expected catalog refusal: {stdout}"
    );
    assert!(
        stdout.contains("field `id` has invalid canonical id `pipeline.bad/path`"),
        "expected invalid canonical id detail: {stdout}"
    );
    assert!(
        stdout.contains("canonical ids must not look like raw repo-relative paths"),
        "expected recovery guidance: {stdout}"
    );
    assert!(
        !stdout.contains("PIPELINE: pipeline.bad/path"),
        "unreachable id must not be advertised: {stdout}"
    );
}

#[test]
fn pipeline_state_set_preserves_distinct_refusals() {
    let (_dir, root) = shared_pipeline_proof_corpus_repo();
    let state_path = root.join(".system/state/pipeline/pipeline.foundation_inputs.yaml");

    write_file(
        &state_path,
        b"---\nschema_version: m1-pipeline-state-v2\npipeline_id: pipeline.foundation_inputs\nrevision: 1\nrouting: {}\nrefs: {}\nrun: {}\naudit:\n  - revision: 1\n    field_path: routing.unsupported_flag\n    value: true\n",
    );

    let malformed = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
    );
    assert!(
        !malformed.status.success(),
        "malformed route state should refuse"
    );
    let malformed_stdout = String::from_utf8(malformed.stdout).expect("stdout is utf-8");
    let malformed_state_path = std::fs::canonicalize(&state_path).expect("canonical state path");
    assert_eq!(
        malformed_stdout.trim_end(),
        format!(
            "REFUSED: malformed route state at {}: unsupported audit routing variable `unsupported_flag` in persisted state",
            malformed_state_path.display()
        )
    );

    write_file(
        &state_path,
        b"---\nschema_version: m1-pipeline-state-v2\npipeline_id: pipeline.foundation_inputs\nrevision: 1\nrouting:\n  needs_project_context: true\nrefs: {}\nrun: {}\naudit:\n  - revision: 1\n    field_path: routing.needs_project_context\n    value: true\n",
    );

    let revision_conflict = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=false",
            "--expected-revision",
            "0",
        ],
    );
    let revision_conflict_stdout =
        String::from_utf8(revision_conflict.stdout).expect("stdout is utf-8");
    assert!(
        !revision_conflict.status.success(),
        "revision conflict should refuse: {revision_conflict_stdout}"
    );
    assert_eq!(
        revision_conflict_stdout.trim_end(),
        concat!(
            "OUTCOME: REFUSED\n",
            "PIPELINE: pipeline.foundation_inputs\n",
            "REASON: revision conflict: expected 0, found 1"
        )
    );

    let unsupported = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "unsupported_flag=true",
        ],
    );
    assert!(
        !unsupported.status.success(),
        "unsupported variable should refuse"
    );
    let unsupported_stdout = String::from_utf8(unsupported.stdout).expect("stdout is utf-8");
    assert_eq!(
        unsupported_stdout.trim_end(),
        concat!(
            "OUTCOME: REFUSED\n",
            "PIPELINE: pipeline.foundation_inputs\n",
            "REASON: unsupported route-state variable `unsupported_flag`"
        )
    );
}

#[test]
fn setup_prints_placeholder_and_fails() {
    assert_placeholder("setup", "placeholder-only entrypoint");
}

#[test]
fn generate_retry_after_repair_clears_missing_root_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let first = run_in(root, &["generate"]);
    assert!(!first.status.success(), "initial generate should fail");
    let first_stdout = String::from_utf8(first.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &first_stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical .system root at .system",
        ],
    );

    repair_to_ready(root);

    let second = run_in(root, &["generate"]);
    assert!(
        second.status.success(),
        "generate should succeed after repair"
    );
    let second_stdout = String::from_utf8(second.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &second_stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system inspect --packet planning.packet` for proof",
        ],
    );
    assert!(second_stdout.contains("## PACKET BODY"));
    assert!(second_stdout.contains("### CHARTER"));
    assert!(second_stdout.contains("### FEATURE_SPEC"));
}

#[test]
fn inspect_retry_after_repair_clears_missing_root_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let first = run_in(root, &["inspect"]);
    assert!(!first.status.success(), "initial inspect should fail");
    let first_stdout = String::from_utf8(first.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &first_stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical .system root at .system",
        ],
    );

    repair_to_ready(root);

    let second = run_in(root, &["inspect"]);
    assert!(
        second.status.success(),
        "inspect should succeed after repair"
    );
    let second_stdout = String::from_utf8(second.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &second_stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system generate --packet planning.packet`",
        ],
    );
    assert!(second_stdout.contains("## JSON FALLBACK"));
    assert!(
        !second_stdout.contains("run `system inspect --packet planning.packet` for proof"),
        "inspect ready path should not loop back into inspect: {second_stdout}"
    );
    assert!(second_stdout.contains("## PACKET BODY"));
    assert!(second_stdout.contains("### CHARTER"));
    assert!(second_stdout.contains("### FEATURE_SPEC"));
}

#[test]
fn doctor_retry_after_repair_reports_ready_after_repair() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let first = run_in(root, &["doctor"]);
    assert!(!first.status.success(), "initial doctor should fail");
    let first_stdout = String::from_utf8(first.stdout).expect("stdout is utf-8");
    assert!(first_stdout.contains("BLOCKED"));
    assert!(first_stdout.contains("SystemRootMissing"));

    repair_to_ready(root);

    let second = run_in(root, &["doctor"]);
    assert!(
        second.status.success(),
        "doctor should succeed after repair"
    );
    let second_stdout = String::from_utf8(second.stdout).expect("stdout is utf-8");
    assert_eq!(second_stdout.trim(), "READY");
}

#[test]
fn generate_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical .system root at .system",
        ],
    );
    assert!(
        stdout.contains("## REFUSAL"),
        "expected refusal section: {stdout}"
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootMissing"),
        "expected SystemRootMissing category: {stdout}"
    );
    assert!(
        stdout.contains("BROKEN SUBJECT: policy system_root"),
        "expected broken subject line: {stdout}"
    );
}

#[test]
fn generate_refuses_when_feature_spec_missing_in_partial_system_tree() {
    let dir = partial_system_repo();

    let output = run_in(dir.path(), &["generate"]);
    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical artifact at .system/feature_spec/FEATURE_SPEC.md",
        ],
    );
    assert!(stdout.contains("CATEGORY: RequiredArtifactMissing"));
}

#[test]
fn inspect_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical .system root at .system",
        ],
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected JSON fallback: {stdout}"
    );
}

#[test]
fn inspect_refuses_when_feature_spec_missing_in_partial_system_tree() {
    let dir = partial_system_repo();

    let output = run_in(dir.path(), &["inspect"]);
    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical artifact at .system/feature_spec/FEATURE_SPEC.md",
        ],
    );
    assert!(stdout.contains("CATEGORY: RequiredArtifactMissing"));
}

#[test]
fn doctor_blocks_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(
        !output.status.success(),
        "doctor should return nonzero when blocked"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("BLOCKED"),
        "expected blocked header: {stdout}"
    );
    assert!(
        stdout.contains("SystemRootMissing"),
        "expected SystemRootMissing category: {stdout}"
    );
    assert!(
        stdout.contains("SUBJECT: policy system_root"),
        "expected human-facing subject: {stdout}"
    );
    assert!(
        stdout.contains("NEXT SAFE ACTION: create canonical .system root at .system"),
        "expected human-facing next action: {stdout}"
    );
    assert!(
        !stdout.contains("NEXT ACTION:"),
        "doctor should use NEXT SAFE ACTION phrasing: {stdout}"
    );
}

#[test]
fn doctor_blocks_when_feature_spec_missing_in_partial_system_tree() {
    let dir = partial_system_repo();

    let output = run_in(dir.path(), &["doctor"]);
    assert!(!output.status.success(), "doctor should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("BLOCKED"));
    assert!(stdout.contains("RequiredArtifactMissing"));
    assert!(stdout.contains(".system/feature_spec/FEATURE_SPEC.md"));
    assert!(stdout.contains(
        "NEXT SAFE ACTION: create canonical artifact at .system/feature_spec/FEATURE_SPEC.md"
    ));
}

#[test]
fn generate_refuses_against_repo_root_when_nested_git_repo_has_invalid_system_root() {
    let (_dir, nested) = nested_git_repo_with_nested_cwd();
    write_file(&nested.join("../../.system"), b"not a directory");

    let output = binary_in(&nested)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: ensure canonical .system root is a directory at .system",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootNotDir"),
        "expected SystemRootNotDir category: {stdout}"
    );
}

#[test]
fn inspect_refuses_against_repo_root_when_nested_git_repo_has_invalid_system_root() {
    let (_dir, nested) = nested_git_repo_with_nested_cwd();
    write_file(&nested.join("../../.system"), b"not a directory");

    let output = binary_in(&nested)
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: ensure canonical .system root is a directory at .system",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootNotDir"),
        "expected SystemRootNotDir category: {stdout}"
    );
}

#[test]
fn doctor_blocks_against_repo_root_when_nested_git_repo_has_invalid_system_root() {
    let (_dir, nested) = nested_git_repo_with_nested_cwd();
    write_file(&nested.join("../../.system"), b"not a directory");

    let output = binary_in(&nested)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(
        !output.status.success(),
        "doctor should return nonzero when blocked"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("BLOCKED"),
        "expected blocked header: {stdout}"
    );
    assert!(
        stdout.contains("SystemRootNotDir"),
        "expected SystemRootNotDir category: {stdout}"
    );
}

#[test]
fn doctor_does_not_cross_nested_git_repo_boundary_into_parent_system_root() {
    let (_dir, nested) = nested_git_repo_inside_managed_parent_with_nested_cwd();

    let output = binary_in(&nested)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(
        !output.status.success(),
        "doctor should block against the child repo root"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("BLOCKED"),
        "expected blocked header: {stdout}"
    );
    assert!(
        stdout.contains("SystemRootMissing"),
        "expected SystemRootMissing category: {stdout}"
    );
}

#[test]
fn generate_emits_real_packet_body_when_ready() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let output = binary_in(root)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system inspect --packet planning.packet` for proof",
        ],
    );
    assert!(
        stdout.contains("## INCLUDED SOURCES"),
        "expected included-sources section: {stdout}"
    );
    assert!(
        stdout.contains("## OMISSIONS AND BUDGET"),
        "expected omissions section: {stdout}"
    );
    assert!(
        stdout.contains("## DECISION SUMMARY"),
        "expected decision-summary section: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET OVERVIEW"),
        "expected packet overview section: {stdout}"
    );
    assert!(
        stdout.contains("PACKET VARIANT: planning.packet"),
        "expected packet variant line: {stdout}"
    );
    assert!(
        stdout.contains("STATUS: Selected"),
        "expected selected status: {stdout}"
    );
    assert!(
        stdout.contains("SUMMARY: READY planning.packet:"),
        "expected ready summary line: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet-body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(
        stdout.contains("charter"),
        "expected charter contents: {stdout}"
    );
    assert!(
        stdout.contains("### FEATURE_SPEC"),
        "expected feature body section: {stdout}"
    );
    assert!(
        stdout.contains("feature"),
        "expected feature contents: {stdout}"
    );
}

#[test]
fn generate_succeeds_from_nested_directory_inside_ready_repo() {
    let (_dir, nested) = planning_ready_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system inspect --packet planning.packet` for proof",
        ],
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body: {stdout}"
    );
    assert!(
        stdout.contains("charter"),
        "expected charter contents: {stdout}"
    );
    assert!(
        stdout.contains("feature"),
        "expected feature contents: {stdout}"
    );
}

#[test]
fn doctor_reports_ready_when_required_artifacts_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let output = binary_in(root)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(output.status.success(), "doctor should succeed when ready");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("READY"), "expected ready header: {stdout}");
}

#[test]
fn doctor_succeeds_from_nested_directory_inside_ready_repo() {
    let (_dir, nested) = planning_ready_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(output.status.success(), "doctor should succeed when ready");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("READY"), "expected ready header: {stdout}");
}

#[test]
fn inspect_reports_ready_when_required_artifacts_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let output = binary_in(root)
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should succeed when ready");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system generate --packet planning.packet`",
        ],
    );
    assert!(
        !stdout.contains("run `system inspect --packet planning.packet` for proof"),
        "inspect ready path should not loop back into inspect: {stdout}"
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected JSON fallback: {stdout}"
    );
    assert!(
        stdout.contains("## DECISION LOG"),
        "expected decision log section: {stdout}"
    );
    assert!(
        stdout.contains("## BUDGET OUTCOME"),
        "expected budget outcome section: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(
        stdout.contains("selection packet_id=planning.packet status=Selected"),
        "expected selected decision summary: {stdout}"
    );
}

#[test]
fn inspect_succeeds_from_nested_directory_inside_ready_repo() {
    let (_dir, nested) = planning_ready_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system generate --packet planning.packet`",
        ],
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected JSON fallback: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body: {stdout}"
    );
}

#[test]
fn generate_refuses_when_demo_packet_selected_without_fixture_set() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .args(["generate", "--packet", "execution.demo.packet"])
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("--fixture-set is required"),
        "expected explicit missing fixture-set refusal: {stdout}"
    );
}

#[test]
fn inspect_blocks_when_demo_packet_selected_without_fixture_set() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .args(["inspect", "--packet", "execution.demo.packet"])
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("--fixture-set is required"),
        "expected explicit missing fixture-set refusal: {stdout}"
    );
}

#[test]
fn generate_resolves_execution_demo_packet_from_fixture_set() {
    let root = workspace_root();
    assert!(
        root.join("tests/fixtures/execution_demo/basic/.system/charter/CHARTER.md")
            .is_file(),
        "expected committed execution demo fixtures to exist under tests/fixtures/execution_demo/basic"
    );
    assert!(
        root.join("tests/fixtures/execution_demo/basic/.system/feature_spec/FEATURE_SPEC.md")
            .is_file(),
        "expected committed execution demo fixtures to exist under tests/fixtures/execution_demo/basic"
    );

    let output = binary_in(&root)
        .args([
            "generate",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `system inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture demo section: {stdout}"
    );
    assert!(
        stdout.contains("MODE: fixture-backed execution demo"),
        "expected fixture-backed label near top: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: basic"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("## INCLUDED SOURCES"),
        "expected included-sources section: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage block: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(
        stdout.contains("### FEATURE_SPEC"),
        "expected feature body section: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET OVERVIEW"),
        "expected packet overview section: {stdout}"
    );
    assert!(
        stdout.contains("PACKET VARIANT: execution.demo.packet"),
        "expected packet variant line: {stdout}"
    );
    assert!(
        stdout.contains("STATUS: Selected"),
        "expected selected status: {stdout}"
    );
    assert!(
        stdout.contains("SUMMARY: READY execution.demo.packet"),
        "expected ready summary line: {stdout}"
    );
}

#[test]
fn generate_resolves_execution_demo_packet_from_nested_directory_inside_repo() {
    let (_dir, nested) = execution_demo_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .args([
            "generate",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `system inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture demo section: {stdout}"
    );
    assert!(
        stdout.contains("MODE: fixture-backed execution demo"),
        "expected fixture-backed label near top: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: basic"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage block: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(
        stdout.contains("### FEATURE_SPEC"),
        "expected feature body section: {stdout}"
    );
}

#[test]
fn inspect_includes_fixture_section_for_execution_demo_packet() {
    let root = workspace_root();
    assert!(
        root.join("tests/fixtures/execution_demo/basic/.system/charter/CHARTER.md")
            .is_file(),
        "expected committed execution demo fixtures to exist under tests/fixtures/execution_demo/basic"
    );
    assert!(
        root.join("tests/fixtures/execution_demo/basic/.system/feature_spec/FEATURE_SPEC.md")
            .is_file(),
        "expected committed execution demo fixtures to exist under tests/fixtures/execution_demo/basic"
    );

    let output = binary_in(&root)
        .args([
            "inspect",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should succeed when ready");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `system generate --packet execution.demo.packet --fixture-set basic`",
        ],
    );
    assert!(
        !stdout.contains(
            "run `system inspect --packet execution.demo.packet --fixture-set basic` for proof"
        ),
        "inspect ready path should not loop back into inspect: {stdout}"
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture section: {stdout}"
    );
    assert!(
        stdout.contains("## DECISION LOG"),
        "expected decision log section: {stdout}"
    );
    assert!(
        stdout.contains("## BUDGET OUTCOME"),
        "expected budget outcome section: {stdout}"
    );
    assert!(
        stdout.contains("MODE: fixture-backed execution demo"),
        "expected fixture-backed label near top: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: basic"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage list: {stdout}"
    );
    assert!(
        stdout.contains("## INCLUDED SOURCES"),
        "expected included-sources section: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(
        stdout.contains("### FEATURE_SPEC"),
        "expected feature body section: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET OVERVIEW"),
        "expected packet overview section: {stdout}"
    );
    assert!(
        stdout.contains("PACKET VARIANT: execution.demo.packet"),
        "expected packet variant line: {stdout}"
    );
    assert!(
        stdout.contains("STATUS: Selected"),
        "expected selected status: {stdout}"
    );
    assert!(
        stdout.contains("SUMMARY: READY execution.demo.packet"),
        "expected ready summary line: {stdout}"
    );
    let pos_charter = stdout
        .find("Charter [.system/charter/CHARTER.md]")
        .expect("charter should be listed");
    let pos_feature = stdout
        .find("FeatureSpec [.system/feature_spec/FEATURE_SPEC.md]")
        .expect("feature spec should be listed");
    assert!(
        pos_charter < pos_feature,
        "expected deterministic ordering (charter before feature): {stdout}"
    );
}

#[test]
fn inspect_resolves_execution_demo_packet_from_nested_directory_inside_repo() {
    let (_dir, nested) = execution_demo_repo_with_nested_cwd();

    let output = binary_in(&nested)
        .args([
            "inspect",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `system generate --packet execution.demo.packet --fixture-set basic`",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture section: {stdout}"
    );
    assert!(
        stdout.contains("MODE: fixture-backed execution demo"),
        "expected fixture-backed label near top: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE SET: basic"),
        "expected fixture set id: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE LINEAGE:"),
        "expected fixture lineage list: {stdout}"
    );
    assert!(
        stdout.contains("## JSON FALLBACK"),
        "expected JSON fallback: {stdout}"
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("### CHARTER"),
        "expected charter body section: {stdout}"
    );
    assert!(
        stdout.contains("### FEATURE_SPEC"),
        "expected feature body section: {stdout}"
    );
}

#[test]
fn generate_from_committed_fixture_dir_refuses_against_git_root() {
    let fixture_dir = committed_execution_demo_fixture_dir();

    let output = binary_in(&fixture_dir)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical .system root at .system",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootMissing"),
        "expected git-root refusal instead of fixture-root success: {stdout}"
    );
    assert!(
        !stdout.contains("Fixture demo charter (basic)"),
        "planning flow should not read fixture bodies as live inputs: {stdout}"
    );
}

#[test]
fn inspect_from_committed_fixture_dir_refuses_against_git_root() {
    let fixture_dir = committed_execution_demo_fixture_dir();

    let output = binary_in(&fixture_dir)
        .arg("inspect")
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical .system root at .system",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: SystemRootMissing"),
        "expected git-root refusal instead of fixture-root success: {stdout}"
    );
    assert!(
        !stdout.contains("Fixture demo charter (basic)"),
        "planning flow should not read fixture bodies as live inputs: {stdout}"
    );
}

#[test]
fn doctor_from_committed_fixture_dir_blocks_against_git_root() {
    let fixture_dir = committed_execution_demo_fixture_dir();

    let output = binary_in(&fixture_dir)
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(!output.status.success(), "doctor should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("BLOCKED"),
        "expected blocked header: {stdout}"
    );
    assert!(
        stdout.contains("SystemRootMissing"),
        "expected git-root blocker instead of fixture-root success: {stdout}"
    );
}

#[test]
fn generate_resolves_execution_demo_packet_from_committed_fixture_dir() {
    let fixture_dir = committed_execution_demo_fixture_dir();

    let output = binary_in(&fixture_dir)
        .args([
            "generate",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("generate should run");

    assert!(output.status.success(), "generate should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `system inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture demo section: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE BASIS ROOT: tests/fixtures/execution_demo/basic/.system/"),
        "expected fixture basis root to resolve exactly once: {stdout}"
    );
}

#[test]
fn inspect_resolves_execution_demo_packet_from_committed_fixture_dir() {
    let fixture_dir = committed_execution_demo_fixture_dir();

    let output = binary_in(&fixture_dir)
        .args([
            "inspect",
            "--packet",
            "execution.demo.packet",
            "--fixture-set",
            "basic",
        ])
        .output()
        .expect("inspect should run");

    assert!(output.status.success(), "inspect should return zero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `system generate --packet execution.demo.packet --fixture-set basic`",
        ],
    );
    assert!(
        stdout.contains("## FIXTURE DEMO"),
        "expected fixture section: {stdout}"
    );
    assert!(
        stdout.contains("FIXTURE BASIS ROOT: tests/fixtures/execution_demo/basic/.system/"),
        "expected fixture basis root to resolve exactly once: {stdout}"
    );
}

#[test]
fn generate_refuses_for_live_execution_packet_when_other_inputs_ok() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let output = binary_in(root)
        .args(["generate", "--packet", "execution.live.packet"])
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: execution.live.packet",
            "NEXT SAFE ACTION: run `system generate --packet planning.packet`",
        ],
    );
    assert!(
        stdout.contains("CATEGORY: UnsupportedRequest"),
        "expected UnsupportedRequest category: {stdout}"
    );
    assert!(
        stdout.contains("fixture-backed execution demos"),
        "expected boundary statement to mention fixture-backed demos: {stdout}"
    );
}

#[test]
fn inspect_redacts_packet_body_for_live_execution_refusal() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter-body");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature-body",
    );

    let output = binary_in(root)
        .args(["inspect", "--packet", "execution.live.packet"])
        .output()
        .expect("inspect should run");

    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: REFUSED",
            "OBJECT: execution.live.packet",
            "NEXT SAFE ACTION: run `system generate --packet planning.packet`",
        ],
    );
    assert!(stdout.contains("## JSON FALLBACK"));
    assert!(stdout.contains("\"packet_result\""));
    assert!(stdout.contains("\"packet body omitted because request is not ready\""));
    assert!(!stdout.contains("charter-body"));
    assert!(!stdout.contains("feature-body"));
}

#[test]
fn generate_blocks_when_optional_project_context_path_is_malformed() {
    let dir = malformed_optional_project_context_repo();

    let output = run_in(dir.path(), &["generate"]);
    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: BLOCKED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical artifact at .system/project_context/PROJECT_CONTEXT.md",
        ],
    );
    assert!(stdout.contains("CATEGORY: ArtifactReadError"));
    assert!(!stdout.contains("## PACKET BODY"));
}

#[test]
fn inspect_blocks_when_optional_project_context_path_is_malformed() {
    let dir = malformed_optional_project_context_repo();

    let output = run_in(dir.path(), &["inspect"]);
    assert!(!output.status.success(), "inspect should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: BLOCKED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: create canonical artifact at .system/project_context/PROJECT_CONTEXT.md",
        ],
    );
    assert!(stdout.contains("CATEGORY: ArtifactReadError"));
    assert!(stdout.contains("## JSON FALLBACK"));
    assert!(!stdout.contains("## PACKET BODY"));
}

#[test]
fn doctor_blocks_when_optional_project_context_path_is_malformed() {
    let dir = malformed_optional_project_context_repo();

    let output = run_in(dir.path(), &["doctor"]);
    assert!(!output.status.success(), "doctor should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("BLOCKED"));
    assert!(stdout.contains("ArtifactReadError"));
    assert!(stdout.contains(".system/project_context/PROJECT_CONTEXT.md"));
}

fn assert_placeholder(command: &str, expected_phrase: &str) {
    let output = binary().arg(command).output().expect("command should run");

    assert!(!output.status.success(), "{command} should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains(expected_phrase),
        "expected placeholder phrase in stdout for {command}: {stdout}"
    );
    assert!(
        stdout.contains(
            "planning packet generation, `inspect`, and `doctor` are implemented in reduced v1"
        ),
        "expected honest placeholder-only message for {command}: {stdout}"
    );
}

fn command_section_lines(help: &str) -> Vec<&str> {
    let mut in_commands = false;
    let mut lines = Vec::new();

    for line in help.lines() {
        if !in_commands {
            if line.trim() == "Commands:" {
                in_commands = true;
            }
            continue;
        }

        if line.trim().is_empty() {
            break;
        }

        if line.starts_with("  ") {
            lines.push(line.trim_start());
        }
    }

    lines
}

fn assert_first_three_lines(stdout: &str, expected: [&str; 3]) {
    let lines: Vec<&str> = stdout.lines().take(3).collect();
    assert_eq!(lines, expected, "unexpected trust header: {stdout}");
}
