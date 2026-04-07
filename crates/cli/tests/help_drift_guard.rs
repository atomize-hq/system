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

#[test]
fn support_story_docs_match_help_snapshot() {
    let root = workspace_root();
    let docs = [
        root.join("README.md"),
        root.join("docs/START_HERE.md"),
        root.join("docs/SUPPORTED_COMMANDS.md"),
    ];
    let docs_text = docs
        .iter()
        .map(|path| {
            fs::read_to_string(path)
                .unwrap_or_else(|err| panic!("read {}: {}", path.display(), err))
        })
        .collect::<Vec<_>>()
        .join("\n");

    let snapshot_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join("system-help.txt");
    let help_text = fs::read_to_string(&snapshot_path)
        .unwrap_or_else(|err| panic!("read {}: {}", snapshot_path.display(), err));

    let required_phrases = [
        "planning packet generation",
        "canonical repo-local `.system/`",
        "fixture-backed execution demo",
        "execution.demo.packet",
        "live execution is explicitly refused",
        "`inspect` is the proof surface",
        "`doctor` is the recovery surface",
        "`setup` is still a placeholder",
    ];

    for phrase in required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing supported-story phrase `{phrase}`"
        );
        assert!(
            help_text.contains(phrase),
            "help snapshot missing supported-story phrase `{phrase}`"
        );
    }
}

fn workspace_root() -> PathBuf {
    let start = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for ancestor in start.ancestors() {
        let cargo_toml = ancestor.join("Cargo.toml");
        if !cargo_toml.is_file() {
            continue;
        }

        let Ok(contents) = fs::read_to_string(&cargo_toml) else {
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
