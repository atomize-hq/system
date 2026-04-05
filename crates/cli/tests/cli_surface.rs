use std::process::Command;

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_system"))
}

fn binary_in(dir: &std::path::Path) -> Command {
    let mut cmd = binary();
    cmd.current_dir(dir);
    cmd
}

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[test]
fn help_lists_setup_first() {
    let output = binary()
        .arg("--help")
        .output()
        .expect("help should run");

    assert!(output.status.success(), "help should succeed");

    let stdout = String::from_utf8(output.stdout).expect("help is utf-8");
    let command_lines = command_section_lines(&stdout);

    assert_eq!(command_lines.len(), 4, "expected four command lines in help");
    assert!(command_lines[0].starts_with("setup "), "setup should be first: {command_lines:?}");
    assert!(command_lines[1].starts_with("generate "), "generate should be second: {command_lines:?}");
    assert!(command_lines[2].starts_with("inspect "), "inspect should be third: {command_lines:?}");
    assert!(command_lines[3].starts_with("doctor "), "doctor should be fourth: {command_lines:?}");
}

#[test]
fn setup_prints_placeholder_and_fails() {
    assert_placeholder("setup", "reserved setup entrypoint");
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
    assert!(stdout.contains("## REFUSAL"), "expected refusal section: {stdout}");
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
fn inspect_prints_placeholder_and_fails() {
    assert_placeholder("inspect", "reserved proof-surface command");
}

#[test]
fn doctor_blocks_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = binary_in(dir.path())
        .arg("doctor")
        .output()
        .expect("doctor should run");

    assert!(!output.status.success(), "doctor should return nonzero when blocked");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(stdout.contains("BLOCKED"), "expected blocked header: {stdout}");
    assert!(
        stdout.contains("SystemRootMissing"),
        "expected SystemRootMissing category: {stdout}"
    );
}

#[test]
fn generate_resolves_but_remains_unimplemented_when_ready() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        b"charter",
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let output = binary_in(root)
        .arg("generate")
        .output()
        .expect("generate should run");

    assert!(!output.status.success(), "generate should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert_first_three_lines(
        &stdout,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: render packet body once implemented (SEAM-5)",
        ],
    );
    assert!(
        stdout.contains("## PACKET BODY"),
        "expected packet body section: {stdout}"
    );
    assert!(
        stdout.contains("Packet body rendering is not implemented yet"),
        "expected honest note: {stdout}"
    );
}

#[test]
fn doctor_reports_ready_when_required_artifacts_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        b"charter",
    );
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

fn assert_placeholder(command: &str, expected_phrase: &str) {
    let output = binary()
        .arg(command)
        .output()
        .expect("command should run");

    assert!(!output.status.success(), "{command} should return nonzero");

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf-8");
    assert!(
        stdout.contains(expected_phrase),
        "expected placeholder phrase in stdout for {command}: {stdout}"
    );
    assert!(
        stdout.contains("reduced v1 behavior is not implemented yet"),
        "expected honest non-implementation message for {command}: {stdout}"
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
