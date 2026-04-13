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
fn system_pipeline_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "--help"],
        "system-pipeline-help.txt",
        "system pipeline --help",
    );
}

#[test]
fn system_pipeline_state_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "state", "--help"],
        "system-pipeline-state-help.txt",
        "system pipeline state --help",
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

    let pipeline_required_phrases = ["`pipeline`", "pipeline resolve", "pipeline state set"];
    assert!(
        top_help_text.contains("pipeline"),
        "top-level help snapshot missing pipeline entry"
    );
    for phrase in pipeline_required_phrases {
        assert!(
            docs_text.contains(phrase),
            "docs missing pipeline phrase `{phrase}`"
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

#[test]
fn cli_tone_rules_doc_locks_core_tone() {
    let root = workspace_root();
    let tone_path = root.join("docs/CLI_TONE_RULES.md");
    let tone_text = fs::read_to_string(&tone_path)
        .unwrap_or_else(|err| panic!("read {}: {}", tone_path.display(), err));

    let required_phrases = [
        "The default tone is **strict but guided**.",
        "Do not use:",
        "something went wrong",
        "unable to process request",
        "Do not celebrate normal success.",
        "Refusal output should:",
        "Proof output should:",
        "Recovery output should:",
    ];

    for phrase in required_phrases {
        assert!(
            tone_text.contains(phrase),
            "CLI tone rules doc missing phrase `{phrase}`"
        );
    }
}

#[test]
fn cli_output_anatomy_doc_locks_section_order_rules() {
    let root = workspace_root();
    let anatomy_path = root.join("docs/CLI_OUTPUT_ANATOMY.md");
    let anatomy_text = fs::read_to_string(&anatomy_path)
        .unwrap_or_else(|err| panic!("read {}: {}", anatomy_path.display(), err));

    let required_phrases = [
        "`generate` and `inspect` start with the same three-line trust header:",
        "## `generate` Anatomy",
        "## `inspect` Anatomy",
        "`doctor` is still transitional.",
        "docs must not claim that it already shares the full trust-header anatomy",
        "`setup` is placeholder-only in current reduced v1.",
        "## Presentation Failure And Parse-Validation Output",
    ];

    for phrase in required_phrases {
        assert!(
            anatomy_text.contains(phrase),
            "CLI output anatomy doc missing phrase `{phrase}`"
        );
    }
}

#[test]
fn design_doc_locks_cli_interaction_contract() {
    let root = workspace_root();
    let design_path = root.join("DESIGN.md");
    let design_text = fs::read_to_string(&design_path)
        .unwrap_or_else(|err| panic!("read {}: {}", design_path.display(), err));

    let required_phrases = [
        "This file is the canonical interaction contract for the reduced-v1 CLI product.",
        "the packet is the product",
        "`doctor` is the only canonical recovery verb",
        "`setup` remains placeholder-only in the Rust CLI",
        "`doctor` still uses a transitional output anatomy",
        "`inspect` currently emits a self-referential ready-path next action",
        "update the relevant D1-D4 source document",
    ];

    for phrase in required_phrases {
        assert!(
            design_text.contains(phrase),
            "DESIGN.md missing phrase `{phrase}`"
        );
    }
}

#[test]
fn cli_operator_journey_doc_locks_revision_findings() {
    let root = workspace_root();
    let journey_path = root.join("docs/CLI_OPERATOR_JOURNEY.md");
    let journey_text = fs::read_to_string(&journey_path)
        .unwrap_or_else(|err| panic!("read {}: {}", journey_path.display(), err));

    let required_phrases = [
        "Does the shipped reduced-v1 product actually produce the confidence -> momentum -> controlled caution arc",
        "The command is functionally correct and productically wrong.",
        "The front door is named correctly, but the shipped command still stops one step before usefulness.",
        "## Revision Backlog",
        "R1, Align `doctor` to the interaction contract",
        "R2, Fix `inspect` ready-path next-action semantics",
        "R3, Make the setup placeholder hand off to a real guided entry path",
    ];

    for phrase in required_phrases {
        assert!(
            journey_text.contains(phrase),
            "CLI operator journey doc missing phrase `{phrase}`"
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
