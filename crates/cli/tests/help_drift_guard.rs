use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn normalize_newlines(value: &str) -> String {
    value.replace("\r\n", "\n")
}

fn help_snapshot_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("snapshots")
        .join(filename)
}

fn run_help(args: &[&str], command_name: &str) -> String {
    let exe = env!("CARGO_BIN_EXE_system");

    let output = Command::new(exe)
        .args(args)
        .env("CLAP_TERM_WIDTH", "120")
        .env("NO_COLOR", "1")
        .env("CLICOLOR", "0")
        .output()
        .unwrap_or_else(|err| panic!("run `{command_name}`: {err}"));

    assert!(
        output.status.success(),
        "`{command_name}` failed (status: {})",
        output.status
    );

    normalize_newlines(
        std::str::from_utf8(&output.stdout)
            .unwrap_or_else(|err| panic!("`{command_name}` stdout must be UTF-8: {err}")),
    )
}

fn read_help_snapshot(filename: &str) -> String {
    let snapshot_path = help_snapshot_path(filename);
    normalize_newlines(
        &fs::read_to_string(&snapshot_path)
            .unwrap_or_else(|err| panic!("read {}: {}", snapshot_path.display(), err)),
    )
}

fn assert_help_matches_snapshot(args: &[&str], snapshot_filename: &str, command_name: &str) {
    let actual = run_help(args, command_name);
    let snapshot_path = help_snapshot_path(snapshot_filename);
    let expected = normalize_newlines(
        &fs::read_to_string(&snapshot_path)
            .unwrap_or_else(|err| panic!("read {}: {}", snapshot_path.display(), err)),
    );

    assert_eq!(
        actual,
        expected,
        "{command_name} output drifted. If intentional, update the snapshot at {}.",
        snapshot_path.display()
    );
}

#[test]
fn system_help_matches_snapshot() {
    assert_help_matches_snapshot(&["--help"], "system-help.txt", "system --help");
}

#[test]
fn system_generate_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["generate", "--help"],
        "system-generate-help.txt",
        "system generate --help",
    );
}

#[test]
fn system_inspect_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["inspect", "--help"],
        "system-inspect-help.txt",
        "system inspect --help",
    );
}

#[test]
fn support_story_docs_match_help_snapshots() {
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

    let top_help_text = read_help_snapshot("system-help.txt");
    let generate_help_text = read_help_snapshot("system-generate-help.txt");
    let inspect_help_text = read_help_snapshot("system-inspect-help.txt");

    let top_level_required_phrases = [
        "planning packet generation",
        "canonical repo-local `.system/`",
        "fixture-backed execution demo",
        "execution.demo.packet",
        "live execution is explicitly refused",
        "`inspect` is the proof surface",
        "`doctor` is the recovery surface",
        "`setup` is still a placeholder",
    ];

    for phrase in top_level_required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing supported-story phrase `{phrase}`"
        );
        assert!(
            top_help_text.contains(phrase),
            "top-level help snapshot missing supported-story phrase `{phrase}`"
        );
    }

    let generate_required_phrases = ["execution.demo.packet"];
    for phrase in generate_required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing generate-help phrase `{phrase}`"
        );
        assert!(
            generate_help_text.contains(phrase),
            "generate help snapshot missing phrase `{phrase}`"
        );
    }

    let inspect_required_phrases = [
        "execution.demo.packet",
        "packet composition and decision evidence",
    ];
    for phrase in inspect_required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing inspect-help phrase `{phrase}`"
        );
        assert!(
            inspect_help_text.contains(phrase),
            "inspect help snapshot missing phrase `{phrase}`"
        );
    }
}

#[test]
fn cli_product_vocabulary_doc_locks_core_terms() {
    let root = workspace_root();
    let vocab_path = root.join("docs/CLI_PRODUCT_VOCABULARY.md");
    let vocab_text = fs::read_to_string(&vocab_path)
        .unwrap_or_else(|err| panic!("read {}: {}", vocab_path.display(), err));

    let required_phrases = [
        "planning packet generation",
        "canonical repo-local `.system/` inputs",
        "`inspect` is the proof surface",
        "`doctor` is the recovery surface",
        "`setup` is still a placeholder",
        "next safe action",
        "bootstrap",
        "hydrate",
    ];

    for phrase in required_phrases {
        assert!(
            vocab_text.contains(phrase),
            "CLI vocabulary doc missing phrase `{phrase}`"
        );
    }
}

#[test]
fn cli_command_hierarchy_doc_locks_front_door_rules() {
    let root = workspace_root();
    let hierarchy_path = root.join("docs/CLI_COMMAND_HIERARCHY.md");
    let hierarchy_text = fs::read_to_string(&hierarchy_path)
        .unwrap_or_else(|err| panic!("read {}: {}", hierarchy_path.display(), err));

    let required_phrases = [
        "The front door is a guided setup experience.",
        "The stable operation name remains `setup`.",
        "`generate` is the default ready-path command.",
        "Commands anchor to the enclosing git root when one exists.",
        "A nested git repo boundary wins over a parent managed repo.",
        "`doctor` is the recovery and readiness command",
    ];

    for phrase in required_phrases {
        assert!(
            hierarchy_text.contains(phrase),
            "CLI command hierarchy doc missing phrase `{phrase}`"
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
