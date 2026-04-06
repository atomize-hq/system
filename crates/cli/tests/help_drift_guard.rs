use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

#[test]
fn system_help_matches_snapshot() {
    let exe = env!("CARGO_BIN_EXE_system");

    let output = Command::new(exe)
        .arg("--help")
        .env("CLAP_TERM_WIDTH", "120")
        .env("NO_COLOR", "1")
        .env("CLICOLOR", "0")
        .output()
        .expect("run `system --help`");

    assert!(
        output.status.success(),
        "`system --help` failed (status: {})",
        output.status
    );

    let actual = normalize_newlines(
        std::str::from_utf8(&output.stdout).expect("`system --help` stdout must be UTF-8"),
    );

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join("system-help.txt");
    let expected = normalize_newlines(
        &fs::read_to_string(&snapshot_path).expect("read help snapshot `system-help.txt`"),
    );

    assert_eq!(
        actual,
        expected,
        "Help output drifted. If intentional, update the snapshot at {}.",
        snapshot_path.display()
    );
}
