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

#[test]
fn help_lists_setup_first() {
    let output = binary().arg("--help").output().expect("help should run");

    assert!(output.status.success(), "help should succeed");

    let stdout = String::from_utf8(output.stdout).expect("help is utf-8");
    let command_lines = command_section_lines(&stdout);

    assert_eq!(
        command_lines.len(),
        4,
        "expected four command lines in help"
    );
    assert!(
        command_lines[0].starts_with("setup "),
        "setup should be first: {command_lines:?}"
    );
    assert!(
        command_lines[1].starts_with("generate "),
        "generate should be second: {command_lines:?}"
    );
    assert!(
        command_lines[2].starts_with("inspect "),
        "inspect should be third: {command_lines:?}"
    );
    assert!(
        command_lines[3].starts_with("doctor "),
        "doctor should be fourth: {command_lines:?}"
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
            "NEXT SAFE ACTION: run `system inspect --packet planning.packet` for proof",
        ],
    );
    assert!(second_stdout.contains("## JSON FALLBACK"));
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
            "NEXT SAFE ACTION: run `system inspect --packet planning.packet` for proof",
        ],
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
            "NEXT SAFE ACTION: run `system inspect --packet planning.packet` for proof",
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
            "NEXT SAFE ACTION: run `system inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ],
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
            "NEXT SAFE ACTION: run `system inspect --packet execution.demo.packet --fixture-set basic` for proof",
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
            "NEXT SAFE ACTION: run `system inspect --packet execution.demo.packet --fixture-set basic` for proof",
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
