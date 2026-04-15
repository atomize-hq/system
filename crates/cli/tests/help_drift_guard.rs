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
fn system_pipeline_compile_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "compile", "--help"],
        "system-pipeline-compile-help.txt",
        "system pipeline compile --help",
    );
}

#[test]
fn system_pipeline_capture_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "capture", "--help"],
        "system-pipeline-capture-help.txt",
        "system pipeline capture --help",
    );
}

#[test]
fn system_pipeline_capture_apply_help_matches_snapshot() {
    assert_help_matches_snapshot(
        &["pipeline", "capture", "apply", "--help"],
        "system-pipeline-capture-apply-help.txt",
        "system pipeline capture apply --help",
    );
}

#[test]
fn support_story_docs_match_help_snapshots() {
    let root = workspace_root();
    let root_readme_path = root.join("README.md");
    let root_readme_text = fs::read_to_string(&root_readme_path)
        .unwrap_or_else(|err| panic!("read {}: {}", root_readme_path.display(), err));
    let docs = [
        root_readme_path.clone(),
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
    let pipeline_help_text = read_help_snapshot("system-pipeline-help.txt");
    let compile_help_text = read_help_snapshot("system-pipeline-compile-help.txt");
    let capture_help_text = read_help_snapshot("system-pipeline-capture-help.txt");
    let capture_apply_help_text = read_help_snapshot("system-pipeline-capture-apply-help.txt");

    let top_level_required_phrases = [
        "planning packet generation",
        "canonical repo-local `.system/`",
        "fixture-backed execution demo",
        "execution.demo.packet",
        "live execution is explicitly refused",
        "`inspect` is the packet proof surface",
        "`doctor` is the recovery surface",
        "`setup` is still a placeholder",
        "explicit stage compilation",
        "explicit stage-output capture",
    ];
    let root_readme_required_phrases = [
        "pipeline capture --preview",
        "pipeline capture apply --capture-id <capture-id>",
        "stage.04_charter_inputs",
        "stage.05_charter_synthesize",
        "stage.06_project_context_interview",
        "stage.07_foundation_pack",
        "stage.10_feature_spec",
        "only supported stage-output writer surface",
        "payload-only",
        "compile-to-capture handoff",
        "`system`-coordinated single-writer flows",
    ];

    for phrase in root_readme_required_phrases {
        assert!(
            root_readme_text.contains(phrase),
            "root README missing capture boundary phrase `{phrase}`"
        );
    }

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

    let pipeline_required_phrases = [
        "`pipeline`",
        "pipeline resolve",
        "pipeline compile",
        "pipeline capture",
        "pipeline capture --preview",
        "pipeline capture apply --capture-id",
        "pipeline state set",
        "pipeline compile --explain",
        "payload-only stdout",
        "proof-only stdout",
        "re-run `pipeline resolve`",
    ];
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

    assert!(
        pipeline_help_text.contains("compile"),
        "pipeline help snapshot missing compile entry"
    );
    assert!(
        pipeline_help_text.contains("capture"),
        "pipeline help snapshot missing capture entry"
    );
    for phrase in ["--stage", "--explain"] {
        assert!(
            compile_help_text.contains(phrase),
            "compile help snapshot missing phrase `{phrase}`"
        );
    }
    for phrase in ["--id", "--stage", "--preview"] {
        assert!(
            capture_help_text.contains(phrase),
            "capture help snapshot missing phrase `{phrase}`"
        );
    }
    for phrase in [
        "stage.04_charter_inputs",
        "stage.06_project_context_interview",
        "stage.10_feature_spec",
    ] {
        assert!(
            docs_text.contains(phrase),
            "docs missing capture target phrase `{phrase}`"
        );
        assert!(
            capture_help_text.contains(phrase),
            "capture help snapshot missing capture target phrase `{phrase}`"
        );
    }
    for phrase in ["--capture-id", "Deterministic capture id"] {
        assert!(
            capture_apply_help_text.contains(phrase),
            "capture apply help snapshot missing phrase `{phrase}`"
        );
    }
    assert!(
        !inspect_help_text.contains("compile"),
        "inspect help snapshot must remain packet-proof only"
    );
    assert!(
        !inspect_help_text.contains("capture"),
        "inspect help snapshot must remain packet-proof only"
    );
}

#[test]
fn m1_activation_contract_docs_remain_boolean_only() {
    let root = workspace_root();
    let plan_path = root.join("PLAN.md");
    let todos_path = root.join("TODOS.md");
    let system_model_path = root.join("docs/legacy/SYSTEM_MODEL.md");
    let contract_path = root.join("docs/contracts/pipeline-route-and-state-core.md");

    let plan_text = fs::read_to_string(&plan_path)
        .unwrap_or_else(|err| panic!("read {}: {}", plan_path.display(), err));
    let todos_text = fs::read_to_string(&todos_path)
        .unwrap_or_else(|err| panic!("read {}: {}", todos_path.display(), err));
    let system_model_text = fs::read_to_string(&system_model_path)
        .unwrap_or_else(|err| panic!("read {}: {}", system_model_path.display(), err));
    let contract_text = fs::read_to_string(&contract_path)
        .unwrap_or_else(|err| panic!("read {}: {}", contract_path.display(), err));

    assert!(
        plan_text.contains("variables.<name> == true|false"),
        "PLAN must describe the boolean-only M1 activation clause shape"
    );
    assert!(
        !plan_text.contains("     - quoted strings\n     - numbers"),
        "PLAN must not advertise quoted-string or numeric activation support for shipped M1"
    );

    assert!(
        todos_text.contains("variable-path equality against boolean literals"),
        "TODOS must describe M1 activation as boolean-only"
    );
    assert!(
        todos_text.contains("future pipelines may eventually need string or numeric equality"),
        "TODOS must treat string and numeric activation as future work"
    );

    assert!(
        system_model_text.contains(
            "only boolean equality in the form `variables.<name> == true|false` is supported"
        ),
        "legacy system model must call out the boolean-only reduced-v1 boundary"
    );
    assert!(
        system_model_text.contains("legacy harness reference supported simple equality checks"),
        "legacy system model must preserve the legacy-harness note"
    );

    assert!(
        contract_text.contains("Activation values MUST be boolean literals only."),
        "route/state contract must remain authoritative for the boolean-only M1 activation subset"
    );
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
        "`inspect` is the packet proof surface",
        "`doctor` is the recovery surface",
        "`setup` is still a placeholder",
        "`pipeline compile --explain`",
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
        "`pipeline compile --id <pipeline-id> --stage <stage-id>`",
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
        "Plain `pipeline compile` success should stay payload-only.",
        "`pipeline compile --explain` should stay proof-only.",
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
        "## `pipeline compile` Anatomy",
        "plain `pipeline compile` success is payload-only stdout",
        "`pipeline compile --explain` success is proof-only stdout",
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
