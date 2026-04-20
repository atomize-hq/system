use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};

fn binary() -> Command {
    Command::new(env!("CARGO_BIN_EXE_system"))
}

fn binary_in(dir: &Path) -> Command {
    let mut cmd = binary();
    cmd.current_dir(dir);
    cmd
}

fn run_in(dir: &Path, args: &[&str]) -> Output {
    binary_in(dir)
        .args(args)
        .output()
        .unwrap_or_else(|err| panic!("run `{}`: {err}", args.join(" ")))
}

fn run_in_with_input(dir: &Path, args: &[&str], input: &str) -> Output {
    use std::io::Write;

    let mut cmd = binary_in(dir);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd
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

fn stdout(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).expect("stdout utf-8")
}

fn write_file(path: &Path, contents: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("mkdirs");
    }
    fs::write(path, contents).expect("write");
}

fn scaffold_repo() -> tempfile::TempDir {
    let dir = tempfile::tempdir().expect("tempdir");
    let output = run_in(dir.path(), &["setup"]);
    assert!(
        output.status.success(),
        "setup failed: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    dir
}

#[test]
fn non_tty_author_refuses_and_points_to_deterministic_path() {
    let dir = scaffold_repo();

    let output = run_in(dir.path(), &["author", "charter"]);

    assert!(!output.status.success(), "non-tty author should refuse");
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: NonInteractiveRefusal"));
    assert!(out.contains("TTY-only guided interview"));
    assert!(out.contains("system author charter --from-inputs <path|->"));
}

#[test]
fn file_inputs_refuse_when_yaml_is_malformed() {
    let dir = scaffold_repo();
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, "project: [not valid");

    let output = run_in(
        dir.path(),
        &[
            "author",
            "charter",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
        ],
    );

    assert!(
        !output.status.success(),
        "malformed yaml should refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: MalformedStructuredInput"));
}

#[test]
fn stdin_inputs_refuse_when_yaml_is_malformed() {
    let dir = scaffold_repo();

    let output = run_in_with_input(
        dir.path(),
        &["author", "charter", "--from-inputs", "-"],
        "schema_version: [broken\n",
    );

    assert!(
        !output.status.success(),
        "malformed stdin yaml should refuse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: MalformedStructuredInput"));
    assert!(out.contains("OBJECT: author charter"));
}
