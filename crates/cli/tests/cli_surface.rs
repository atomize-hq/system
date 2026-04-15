mod pipeline_proof_corpus_support;

use std::process::{Command, Output};

const FIXED_NOW_UTC: &str = "2026-01-28T18:35:10Z";

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

fn run_in_with_env(dir: &std::path::Path, args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut cmd = binary_in(dir);
    cmd.args(args);
    for (key, value) in envs {
        cmd.env(key, value);
    }
    cmd.output()
        .unwrap_or_else(|err| panic!("run `{}` with env: {err}", args.join(" ")))
}

fn run_in_with_input(dir: &std::path::Path, args: &[&str], input: &str) -> Output {
    use std::io::Write;
    use std::process::Stdio;

    let mut child = binary_in(dir)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap_or_else(|err| panic!("spawn `{}`: {err}", args.join(" ")));

    {
        let stdin = child.stdin.as_mut().expect("stdin");
        stdin
            .write_all(input.as_bytes())
            .unwrap_or_else(|err| panic!("write stdin for `{}`: {err}", args.join(" ")));
    }

    child
        .wait_with_output()
        .unwrap_or_else(|err| panic!("wait `{}`: {err}", args.join(" ")))
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

fn canonical_repo_root(path: &std::path::Path) -> std::path::PathBuf {
    std::fs::canonicalize(path)
        .unwrap_or_else(|err| panic!("canonicalize {}: {err}", path.display()))
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
        &root.join("pipelines/foundation.yaml"),
        b"---\nkind: pipeline\nid: pipeline.foundation\nversion: 0.1.0\ntitle: Foundation\ndescription: foundation\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );
    write_file(
        &root.join("pipelines/bad-id.yaml"),
        b"---\nkind: pipeline\nid: pipeline.bad/path\nversion: 0.1.0\ntitle: Bad Id\ndescription: bad\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );

    (dir, root)
}

fn unused_bad_stage_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    write_file(
        &root.join("core/stages/00_base.md"),
        b"---\nkind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\n---\n# base\n",
    );
    write_file(
        &root.join("core/stages/99_bad_unused.md"),
        b"---\nkind: nonsense\nid: stage.bad_unused\nversion: 0.1.0\ntitle: Bad Unused Stage\ndescription: bad\n---\n# bad\n",
    );
    write_file(
        &root.join("pipelines/foundation.yaml"),
        b"---\nkind: pipeline\nid: pipeline.foundation\nversion: 0.1.0\ntitle: Foundation\ndescription: foundation\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );

    (dir, root)
}

fn selected_broken_pipeline_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().to_path_buf();

    write_file(
        &root.join("core/stages/00_base.md"),
        b"---\nkind: stage\nid: stage.00_base\nversion: 0.1.0\ntitle: Base\ndescription: base\n---\n# base\n",
    );
    write_file(
        &root.join("core/stages/bad.md"),
        b"---\nkind: nonsense\nid: stage.bad\nversion: 0.1.0\ntitle: Bad\ndescription: bad\n---\n# bad\n",
    );
    write_file(
        &root.join("pipelines/foundation.yaml"),
        b"---\nkind: pipeline\nid: pipeline.foundation\nversion: 0.1.0\ntitle: Foundation\ndescription: foundation\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.00_base\n    file: core/stages/00_base.md\n",
    );
    write_file(
        &root.join("pipelines/broken.yaml"),
        b"---\nkind: pipeline\nid: pipeline.broken\nversion: 0.1.0\ntitle: Broken\ndescription: broken\n---\ndefaults:\n  runner: codex-cli\n  profile: python-uv\n  enable_complexity: false\nstages:\n  - id: stage.bad\n    file: core/stages/bad.md\n",
    );

    (dir, root)
}

fn write_incomplete_profile_pack(root: &std::path::Path, profile_id: &str) {
    write_file(
        &root.join(format!("profiles/{profile_id}/profile.yaml")),
        format!("kind: profile\nid: {profile_id}\n").as_bytes(),
    );
}

fn set_foundation_inputs_default_profile(root: &std::path::Path, profile_id: &str) {
    let pipeline_path = root.join("pipelines/foundation_inputs.yaml");
    let contents = std::fs::read_to_string(&pipeline_path).expect("read foundation pipeline");
    std::fs::write(
        &pipeline_path,
        contents.replace(
            "defaults:\n  runner: codex-cli\n  profile: python-uv\n",
            &format!("defaults:\n  runner: codex-cli\n  profile: {profile_id}\n"),
        ),
    )
    .expect("write foundation pipeline");
}

fn prepare_foundation_inputs_compile_ready_route_basis(root: &std::path::Path) {
    for args in [
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=true",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "charter_gaps_detected=true",
        ],
        vec!["pipeline", "resolve", "--id", "foundation_inputs"],
    ] {
        let output = run_in(root, &args);
        assert!(
            output.status.success(),
            "command should succeed: {:?}",
            args
        );
    }
}

fn prepare_foundation_inputs_full_context_route_basis(root: &std::path::Path) {
    for args in [
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.runner=codex-cli",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.profile=python-uv",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.charter_ref=artifacts/charter/CHARTER.md",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.project_context_ref=artifacts/project_context/PROJECT_CONTEXT.md",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=false",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "charter_gaps_detected=false",
        ],
        vec!["pipeline", "resolve", "--id", "foundation_inputs"],
    ] {
        let output = run_in(root, &args);
        assert!(
            output.status.success(),
            "command should succeed: {:?}",
            args
        );
    }
}

fn prepare_stage_05_capture_ready_route_basis(root: &std::path::Path) {
    let output = run_in(root, &["pipeline", "resolve", "--id", "foundation_inputs"]);
    assert!(output.status.success(), "resolve should succeed");
}

fn prepare_stage_07_capture_ready_route_basis(root: &std::path::Path) {
    for args in [
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "refs.charter_ref=artifacts/charter/CHARTER.md",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "needs_project_context=false",
        ],
        vec![
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--var",
            "charter_gaps_detected=false",
        ],
        vec!["pipeline", "resolve", "--id", "foundation_inputs"],
    ] {
        let output = run_in(root, &args);
        assert!(
            output.status.success(),
            "command should succeed: {:?}",
            args
        );
    }
}

fn stage_05_capture_input(root: &std::path::Path) -> String {
    std::fs::read_to_string(root.join("artifacts/charter/CHARTER.md")).expect("stage 05 input")
}

fn stage_07_capture_input(root: &std::path::Path) -> String {
    let outputs = [
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
        "artifacts/foundation/TEST_STRATEGY_BRIEF.md",
        "artifacts/foundation/QUALITY_GATES_SPEC.md",
        "artifacts/foundation/quality_gates.yaml",
        "artifacts/foundation/ENVIRONMENT_INVENTORY.md",
    ];
    let mut out = String::new();
    for path in outputs {
        out.push_str(&format!("--- FILE: {path} ---\n"));
        out.push_str(&std::fs::read_to_string(root.join(path)).expect("stage 07 input"));
        out.push('\n');
    }
    out
}

fn normalize_capture_id(output: &str) -> String {
    output
        .lines()
        .map(|line| {
            if line.starts_with("CAPTURE ID: ") {
                "CAPTURE ID: {{CAPTURE_ID}}".to_string()
            } else if let Some(prefix) = line
                .strip_prefix("NEXT SAFE ACTION: run `system pipeline capture apply --capture-id ")
            {
                let _ = prefix;
                "NEXT SAFE ACTION: run `system pipeline capture apply --capture-id {{CAPTURE_ID}}`"
                    .to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
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
fn version_matches_release_metadata() {
    let root = workspace_root();
    let output = run_in(root.as_path(), &["--version"]);

    assert!(output.status.success(), "version should succeed");

    let stdout = String::from_utf8(output.stdout).expect("version stdout is utf-8");
    let expected_version =
        std::fs::read_to_string(root.join("VERSION")).expect("read workspace VERSION");
    let expected = format!("system {}\n", expected_version.trim());

    assert_eq!(
        stdout, expected,
        "CLI version should match release metadata"
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
        6,
        "expected six pipeline command lines"
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
        command_lines[3].starts_with("compile "),
        "compile should be fourth: {command_lines:?}"
    );
    assert!(
        command_lines[4].starts_with("capture "),
        "capture should be fifth: {command_lines:?}"
    );
    assert!(
        command_lines[5].starts_with("state "),
        "state should be sixth: {command_lines:?}"
    );
    assert!(
        stdout.contains("explicit stage-output capture"),
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
fn pipeline_compile_help_lists_exact_m2_surface() {
    let root = workspace_root();

    let output = run_in(root.as_path(), &["pipeline", "compile", "--help"]);
    assert!(
        output.status.success(),
        "pipeline compile help should succeed"
    );

    let stdout = String::from_utf8(output.stdout).expect("help stdout is utf-8");
    assert!(
        stdout.contains("Compile one supported stage payload from persisted route basis"),
        "expected compile help summary: {stdout}"
    );
    assert!(stdout.contains("--id <ID>"), "expected --id: {stdout}");
    assert!(
        stdout.contains("--stage <STAGE>"),
        "expected --stage: {stdout}"
    );
    assert!(stdout.contains("--explain"), "expected --explain: {stdout}");
}

#[test]
fn pipeline_capture_help_lists_preview_and_apply_surface() {
    let root = workspace_root();

    let output = run_in(root.as_path(), &["pipeline", "capture", "--help"]);
    assert!(
        output.status.success(),
        "pipeline capture help should succeed"
    );

    let stdout = String::from_utf8(output.stdout).expect("help stdout is utf-8");
    assert!(
        stdout.contains(
            "Capture one supported stage output and materialize declared artifact and repo-mirror files"
        ),
        "expected capture help summary: {stdout}"
    );
    assert!(stdout.contains("--id <ID>"), "expected --id: {stdout}");
    assert!(
        stdout.contains("--stage <STAGE>"),
        "expected --stage: {stdout}"
    );
    assert!(stdout.contains("--preview"), "expected --preview: {stdout}");
    assert!(
        stdout.contains("apply"),
        "expected apply subcommand: {stdout}"
    );

    let apply = run_in(root.as_path(), &["pipeline", "capture", "apply", "--help"]);
    assert!(
        apply.status.success(),
        "pipeline capture apply help should succeed"
    );
    let apply_stdout = String::from_utf8(apply.stdout).expect("apply help stdout is utf-8");
    assert!(
        apply_stdout.contains("--capture-id <CAPTURE_ID>"),
        "expected --capture-id: {apply_stdout}"
    );
}

#[test]
fn pipeline_capture_preview_charter_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_05_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
            "--preview",
        ],
        &stage_05_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "preview should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalize_capture_id(&stdout),
        &[],
        "capture.preview.stage_05_charter_synthesize.txt",
    );
}

#[test]
fn pipeline_capture_preview_foundation_pack_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_07_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.07_foundation_pack",
            "--preview",
        ],
        &stage_07_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "preview should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalize_capture_id(&stdout),
        &[],
        "capture.preview.stage_07_foundation_pack.txt",
    );
}

#[test]
fn pipeline_capture_apply_charter_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_05_capture_ready_route_basis(root.as_path());

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
        ],
        &stage_05_capture_input(root.as_path()),
    );
    assert!(output.status.success(), "capture should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.apply.stage_05_charter_synthesize.txt",
    );
}

#[test]
fn pipeline_capture_apply_foundation_pack_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_07_capture_ready_route_basis(root.as_path());

    let preview = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.07_foundation_pack",
            "--preview",
        ],
        &stage_07_capture_input(root.as_path()),
    );
    assert!(preview.status.success(), "preview should succeed");
    let preview_stdout = String::from_utf8(preview.stdout).expect("preview stdout is utf-8");
    let capture_id = preview_stdout
        .lines()
        .find_map(|line| line.strip_prefix("CAPTURE ID: "))
        .expect("capture id");

    let apply = run_in(
        root.as_path(),
        &["pipeline", "capture", "apply", "--capture-id", capture_id],
    );
    assert!(apply.status.success(), "apply should succeed");

    let stdout = String::from_utf8(apply.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.apply.stage_07_foundation_pack.txt",
    );
}

#[test]
fn pipeline_capture_apply_refuses_missing_capture_id() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "apply",
            "--capture-id",
            "1111111111111111111111111111111111111111111111111111111111111111",
        ],
    );
    assert!(!output.status.success(), "apply should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &stdout,
        &[],
        "capture.refused.missing_capture_id.txt",
    );
}

#[cfg(unix)]
#[test]
fn pipeline_capture_preview_refuses_invalid_write_target() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_stage_05_capture_ready_route_basis(root.as_path());
    let external = tempfile::tempdir().expect("tempdir");
    let repo_mirror = root.join("CHARTER.md");

    std::fs::remove_file(&repo_mirror).expect("remove repo mirror");
    std::os::unix::fs::symlink(external.path().join("CHARTER.md"), &repo_mirror)
        .expect("replace repo mirror with symlink");

    let output = run_in_with_input(
        root.as_path(),
        &[
            "pipeline",
            "capture",
            "--id",
            "foundation_inputs",
            "--stage",
            "stage.05_charter_synthesize",
            "--preview",
        ],
        &stage_05_capture_input(root.as_path()),
    );
    assert!(!output.status.success(), "preview should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("OUTCOME: REFUSED"), "{stdout}");
    assert!(stdout.contains("PIPELINE: pipeline.foundation_inputs"), "{stdout}");
    assert!(stdout.contains("STAGE: stage.05_charter_synthesize"), "{stdout}");
    assert!(stdout.contains("invalid_write_target"), "{stdout}");
    assert!(stdout.contains("cannot be written through symlink"), "{stdout}");
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
        &["pipeline", "show", "--id", "pipeline.foundation_inputs"],
    );
    assert!(show.status.success(), "pipeline show should succeed");
    let show_stdout = String::from_utf8(show.stdout).expect("show stdout is utf-8");
    assert!(show_stdout.contains("PIPELINE: pipeline.foundation_inputs"));
    assert!(show_stdout.contains("DEFAULTS:"));
    assert!(show_stdout.contains("SOURCE: pipelines/foundation_inputs.yaml"));
    assert!(show_stdout.contains("stage.05_charter_synthesize"));
    assert!(show_stdout.contains("core/stages/05_charter_synthesize.md"));
    assert!(show_stdout.contains("sets: [needs_project_context]"));
    assert!(show_stdout.contains(
        "activation: activation.when.any [variables.charter_gaps_detected == true, variables.needs_project_context == true]"
    ));

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
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let first_resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        first_resolve.status.success(),
        "pipeline resolve should succeed"
    );
    let first_resolve_stdout = String::from_utf8(first_resolve.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &first_resolve_stdout,
        root.as_path(),
        None,
        "resolve.initial.txt",
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
    pipeline_proof_corpus_support::assert_matches_golden(
        &applied_stdout,
        root.as_path(),
        None,
        "state_set.var.needs_project_context.applied.txt",
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
    pipeline_proof_corpus_support::assert_matches_golden(
        &activation_applied_stdout,
        root.as_path(),
        None,
        "state_set.var.charter_gaps_detected.applied.txt",
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
    pipeline_proof_corpus_support::assert_matches_golden(
        &second_resolve_stdout,
        root.as_path(),
        None,
        "resolve.after_full_activation.txt",
    );
}

#[test]
fn pipeline_resolve_refuses_incomplete_default_profile_pack() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    write_incomplete_profile_pack(root.as_path(), "incomplete");
    set_foundation_inputs_default_profile(root.as_path(), "incomplete");

    let output = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        !output.status.success(),
        "pipeline resolve should refuse incomplete default profile pack"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("REFUSED: route basis build error:"),
        "{stdout}"
    );
    assert!(stdout.contains("profiles/incomplete/"), "{stdout}");
    assert!(stdout.contains("commands.yaml"), "{stdout}");
    assert!(stdout.contains("conventions.md"), "{stdout}");
    assert!(
        !stdout.contains("failed to read route_basis input"),
        "{stdout}"
    );
}

#[test]
fn pipeline_state_set_field_surfaces_accept_run_and_refs() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

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
    pipeline_proof_corpus_support::assert_matches_golden(
        &runner_stdout,
        root.as_path(),
        None,
        "state_set.field.run_runner.applied.txt",
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
    pipeline_proof_corpus_support::assert_matches_golden(
        &ref_stdout,
        root.as_path(),
        None,
        "state_set.field.refs_charter_ref.applied.txt",
    );

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(
        resolve.status.success(),
        "pipeline resolve should surface refs and run fields in route basis"
    );
    let resolve_stdout = String::from_utf8(resolve.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &resolve_stdout,
        root.as_path(),
        None,
        "resolve.after_run_and_refs.txt",
    );
}

#[test]
fn pipeline_compile_feature_spec_payload_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
        &[(
            system_compiler::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(output.status.success(), "pipeline compile should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.stage_10_feature_spec.payload.full_context.txt",
    );
    assert!(
        !stdout.contains("OUTCOME:"),
        "plain compile must stay payload-only: {stdout}"
    );
    assert!(
        !stdout.contains("NEXT SAFE ACTION:"),
        "plain compile must not include refusal framing: {stdout}"
    );
    assert!(
        !stdout.contains("ROUTE BASIS:"),
        "plain compile must not include proof sections: {stdout}"
    );
}

#[test]
fn pipeline_compile_ignores_unrelated_malformed_stage_files() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());
    std::fs::write(
        root.join("core/stages/99_bad.md"),
        r#"---
kind: nonsense
id: stage.99_bad
version: 0.1.0
title: Bad Stage
description: malformed and unrelated
---
# bad
"#,
    )
    .expect("write unrelated malformed stage");

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
        &[(
            system_compiler::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        output.status.success(),
        "pipeline compile should ignore unrelated malformed stages"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.stage_10_feature_spec.payload.full_context.txt",
    );
}

#[test]
fn pipeline_compile_feature_spec_explain_matches_shared_golden() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
            "--explain",
        ],
        &[(
            system_compiler::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        output.status.success(),
        "pipeline compile --explain should succeed"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.stage_10_feature_spec.explain.full_context.txt",
    );
    assert!(
        stdout.contains("OUTCOME: COMPILED"),
        "explain mode should render proof: {stdout}"
    );
    assert!(
        !stdout.starts_with("# stage.10_feature_spec"),
        "explain mode must not include the payload header: {stdout}"
    );
}

#[test]
fn pipeline_compile_refuses_when_required_variable_is_missing() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let stage_path = root.join("core/stages/10_feature_spec.md");
    let stage = std::fs::read_to_string(&stage_path).expect("read stage");
    let updated_stage = stage.replace("    - project_name?\n", "    - project_name\n");
    assert_ne!(
        stage, updated_stage,
        "stage fixture should be updated for the test"
    );
    std::fs::write(&stage_path, updated_stage).expect("write stage");
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
        &[(
            system_compiler::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        !output.status.success(),
        "missing required variable should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.missing_required_variable.txt",
    );
}

#[cfg(unix)]
#[test]
fn pipeline_compile_refuses_symlinked_required_artifact_input() {
    use std::os::unix::fs::symlink;

    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_full_context_route_basis(root.as_path());

    let outside_dir = tempfile::tempdir().expect("outside tempdir");
    let outside_secret = outside_dir.path().join("system-review-secret.txt");
    std::fs::write(&outside_secret, "outside-secret").expect("write secret");

    let artifact = root.join("artifacts/base/BASE_CONTEXT.md");
    std::fs::remove_file(&artifact).expect("remove artifact");
    symlink(&outside_secret, &artifact).expect("symlink artifact");

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
        &[(
            system_compiler::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        !output.status.success(),
        "symlinked required artifact should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("OUTCOME: REFUSED"));
    assert!(!stdout.contains("# stage.10_feature_spec"));
    assert!(stdout.contains("artifacts/base/BASE_CONTEXT.md"));
    assert!(!stdout.contains("outside-secret"));
}

#[test]
fn pipeline_compile_refuses_missing_route_basis() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "pipeline compile should refuse without a persisted route basis"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.missing_route_basis.txt",
    );
}

#[test]
fn pipeline_compile_refuses_inactive_stage() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(resolve.status.success(), "pipeline resolve should succeed");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "inactive-stage compile should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.inactive_stage.txt",
    );
}

#[test]
fn pipeline_compile_refuses_stale_route_basis_after_state_set() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());

    let mutation = run_in(
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
    assert!(mutation.status.success(), "state mutation should succeed");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "foundation_inputs",
            "--stage",
            "10_feature_spec",
        ],
    );
    assert!(!output.status.success(), "stale route basis should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.stale_route_basis.txt",
    );
}

#[test]
fn pipeline_compile_refuses_malformed_route_basis() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    pipeline_proof_corpus_support::install_state_seed(root.as_path(), "malformed_route_basis.yaml");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "malformed route basis should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.malformed_route_basis.txt",
    );
}

#[test]
fn pipeline_compile_refuses_forged_route_basis_status() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "foundation_inputs"],
    );
    assert!(resolve.status.success(), "pipeline resolve should succeed");

    let state_path = root
        .join(".system")
        .join("state")
        .join("pipeline")
        .join("pipeline.foundation_inputs.yaml");
    let state = std::fs::read_to_string(&state_path).expect("state file");
    let forged = state.replace(
        "  - stage_id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n    status: blocked\n    reason:\n      kind: blocked_by_unresolved_stage\n      upstream_stage_id: stage.06_project_context_interview\n      upstream_status: next\n",
        "  - stage_id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n    status: active\n    reason: null\n",
    );
    assert_ne!(
        state, forged,
        "route_basis fixture should be updated for the test"
    );
    std::fs::write(&state_path, forged).expect("write forged state");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(!output.status.success(), "forged route basis should refuse");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.malformed_route_basis_forged_status.txt",
    );
}

#[test]
fn pipeline_compile_refuses_stage_not_in_pipeline() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());

    let pipeline_path = root.join("pipelines/foundation_inputs.yaml");
    let pipeline = std::fs::read_to_string(&pipeline_path).expect("read pipeline");
    let updated_pipeline = pipeline.replace(
        "  - id: stage.10_feature_spec\n    file: core/stages/10_feature_spec.md\n",
        "",
    );
    assert_ne!(
        pipeline, updated_pipeline,
        "pipeline fixture should be updated for the test"
    );
    std::fs::write(&pipeline_path, updated_pipeline).expect("write pipeline");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
    );
    assert!(
        !output.status.success(),
        "missing declared stage should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.stage_not_in_pipeline.txt",
    );
}

#[test]
fn pipeline_compile_refuses_missing_required_artifact() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());
    std::fs::remove_file(root.join("artifacts/base/BASE_CONTEXT.md"))
        .expect("remove required artifact");

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &[(
            system_compiler::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        !output.status.success(),
        "missing required artifact should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    pipeline_proof_corpus_support::assert_matches_golden(
        &stdout,
        root.as_path(),
        None,
        "compile.refused.missing_required_artifact.txt",
    );
}

#[test]
fn pipeline_compile_allows_optional_artifacts_to_be_absent() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    prepare_foundation_inputs_compile_ready_route_basis(root.as_path());
    for path in [
        "artifacts/project_context/PROJECT_CONTEXT.md",
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
    ] {
        std::fs::remove_file(root.join(path)).expect("remove optional artifact");
    }

    let canonical_root = canonical_repo_root(root.as_path());
    let expected = system_compiler::compile_pipeline_stage_with_runtime(
        canonical_root.as_path(),
        "pipeline.foundation_inputs",
        "stage.10_feature_spec",
        &system_compiler::PipelineCompileRuntimeContext {
            now_utc_override: Some(FIXED_NOW_UTC.to_string()),
        },
    )
    .map(|result| system_compiler::render_pipeline_compile_payload(&result))
    .expect("compile should succeed");

    let output = run_in_with_env(
        root.as_path(),
        &[
            "pipeline",
            "compile",
            "--id",
            "pipeline.foundation_inputs",
            "--stage",
            "stage.10_feature_spec",
        ],
        &[(
            system_compiler::PIPELINE_COMPILE_NOW_UTC_ENV_VAR,
            FIXED_NOW_UTC,
        )],
    );
    assert!(
        output.status.success(),
        "optional artifacts should remain a success path"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_eq!(stdout, format!("{expected}\n"));
    assert!(
        !stdout.contains("OUTCOME:"),
        "plain compile must stay payload-only: {stdout}"
    );
}

#[test]
fn pipeline_state_set_field_rejects_invalid_paths_and_values() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();

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

    let derived_path = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.repo_root=/tmp/repo",
        ],
    );
    assert!(
        !derived_path.status.success(),
        "derived run.repo_root field should refuse"
    );
    let derived_path_stdout = String::from_utf8(derived_path.stdout).expect("stdout is utf-8");
    assert_eq!(
        derived_path_stdout.trim_end(),
        "REFUSED: unsupported --field path `run.repo_root`; expected one of `run.runner`, `run.profile`, `refs.charter_ref`, or `refs.project_context_ref`"
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
fn pipeline_state_set_field_rejects_incomplete_profile_pack() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    write_incomplete_profile_pack(root.as_path(), "incomplete");

    let output = run_in(
        root.as_path(),
        &[
            "pipeline",
            "state",
            "set",
            "--id",
            "foundation_inputs",
            "--field",
            "run.profile=incomplete",
        ],
    );
    assert!(
        !output.status.success(),
        "incomplete profile pack should refuse"
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("REFUSED: route state mutation error:"),
        "{stdout}"
    );
    assert!(stdout.contains("run.profile `incomplete`"), "{stdout}");
    assert!(stdout.contains("profiles/incomplete/"), "{stdout}");
    assert!(stdout.contains("commands.yaml"), "{stdout}");
    assert!(stdout.contains("conventions.md"), "{stdout}");
}

#[test]
fn pipeline_list_and_show_ignore_activation_drift_during_inventory_inspection() {
    let (_dir, root) = activation_drift_pipeline_repo();

    for args in [
        vec!["pipeline", "list"],
        vec!["pipeline", "show", "--id", "pipeline.drift"],
        vec!["pipeline", "show", "--id", "stage.00_base"],
    ] {
        let output = run_in(root.as_path(), &args);
        assert!(
            output.status.success(),
            "command should succeed: {:?}",
            args
        );

        let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
        assert!(
            !stdout.contains("has activation drift"),
            "inventory command should not surface activation drift for {:?}: {stdout}",
            args
        );
    }
}

#[test]
fn pipeline_list_and_show_ignore_unrelated_broken_stage_files_during_inventory_inspection() {
    let (_dir, root) = unused_bad_stage_repo();

    for args in [
        vec!["pipeline", "list"],
        vec!["pipeline", "show", "--id", "pipeline.foundation"],
        vec!["pipeline", "show", "--id", "stage.00_base"],
    ] {
        let output = run_in(root.as_path(), &args);
        assert!(
            output.status.success(),
            "command should succeed: {:?}",
            args
        );

        let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
        assert!(
            !stdout.contains("pipeline catalog error"),
            "unexpected refusal: {stdout}"
        );
        assert!(stdout.contains("pipeline.foundation") || stdout.contains("stage.00_base"));
    }
}

#[test]
fn pipeline_resolve_and_state_set_still_refuse_activation_drift_before_route_evaluation() {
    let (_dir, root) = activation_drift_pipeline_repo();

    for args in [
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
            "route-aware command should refuse: {:?}",
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
fn pipeline_list_omits_unrelated_invalid_pipeline_ids_but_resolve_still_refuses() {
    let (_dir, root) = invalid_pipeline_id_repo();

    let output = run_in(root.as_path(), &["pipeline", "list"]);
    assert!(output.status.success(), "pipeline list should succeed");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("PIPELINE: pipeline.foundation"));
    assert!(
        !stdout.contains("PIPELINE: pipeline.bad/path"),
        "invalid pipeline id must not be advertised: {stdout}"
    );

    let resolve = run_in(
        root.as_path(),
        &["pipeline", "resolve", "--id", "pipeline.foundation"],
    );
    assert!(
        !resolve.status.success(),
        "pipeline resolve should still refuse"
    );
    let resolve_stdout = String::from_utf8(resolve.stdout).expect("stdout is utf-8");
    assert!(
        resolve_stdout
            .contains("REFUSED: pipeline catalog error: failed to load pipeline definition"),
        "expected strict catalog refusal: {resolve_stdout}"
    );
    assert!(
        resolve_stdout.contains("field `id` has invalid canonical id `pipeline.bad/path`"),
        "expected invalid canonical id detail: {resolve_stdout}"
    );
}

#[test]
fn pipeline_show_still_refuses_when_selected_pipeline_has_broken_stage_metadata() {
    let (_dir, root) = selected_broken_pipeline_repo();

    let healthy = run_in(
        root.as_path(),
        &["pipeline", "show", "--id", "pipeline.foundation"],
    );
    assert!(
        healthy.status.success(),
        "healthy pipeline show should succeed"
    );

    let broken = run_in(
        root.as_path(),
        &["pipeline", "show", "--id", "pipeline.broken"],
    );
    assert!(
        !broken.status.success(),
        "selected broken pipeline should refuse"
    );
    let stdout = String::from_utf8(broken.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains("REFUSED: pipeline catalog error: stage front matter"),
        "expected stage metadata refusal: {stdout}"
    );
    assert!(
        stdout.contains("must declare kind `stage`, got `nonsense`"),
        "expected stage kind detail: {stdout}"
    );
}

#[test]
fn pipeline_state_set_preserves_distinct_refusals() {
    let (_dir, root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let state_path = pipeline_proof_corpus_support::install_state_seed(
        root.as_path(),
        "malformed_route_state.yaml",
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
    pipeline_proof_corpus_support::assert_matches_golden(
        &malformed_stdout,
        root.as_path(),
        Some(&state_path),
        "state_set.refused.malformed_route_state.txt",
    );

    pipeline_proof_corpus_support::install_state_seed(
        root.as_path(),
        "revision_conflict_state.yaml",
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
    pipeline_proof_corpus_support::assert_matches_golden(
        &revision_conflict_stdout,
        root.as_path(),
        None,
        "state_set.refused.revision_conflict.txt",
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
    pipeline_proof_corpus_support::assert_matches_golden(
        &unsupported_stdout,
        root.as_path(),
        None,
        "state_set.refused.unsupported_variable.txt",
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
