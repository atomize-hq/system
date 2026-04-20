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

fn run_in_with_env(dir: &Path, args: &[&str], envs: &[(&str, &str)]) -> Output {
    let mut cmd = binary_in(dir);
    cmd.args(args);
    for (key, value) in envs {
        cmd.env(key, value);
    }
    cmd.output()
        .unwrap_or_else(|err| panic!("run `{}` with env: {err}", args.join(" ")))
}

fn run_in_with_input_and_env(
    dir: &Path,
    args: &[&str],
    input: &str,
    envs: &[(&str, &str)],
) -> Output {
    use std::io::Write;

    let mut cmd = binary_in(dir);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    for (key, value) in envs {
        cmd.env(key, value);
    }

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

fn valid_structured_inputs_yaml() -> &'static str {
    r#"schema_version: "0.1.0"
project:
  name: "System"
  classification: greenfield
  team_size: 2
  users: internal
  expected_lifetime: months
  surfaces:
    - cli
    - api
  runtime_environments:
    - server
  constraints:
    deadline: ""
    budget: ""
    experience_notes: "small team"
    must_use_tech:
      - rust
  operational_reality:
    in_production_today: false
    prod_users_or_data: ""
    external_contracts_to_preserve: []
    uptime_expectations: "best effort"
  default_implications:
    backward_compatibility: not_required
    migration_planning: not_required
    rollout_controls: lightweight
    deprecation_policy: not_required_yet
    observability_threshold: standard
posture:
  rubric_scale: "1-5"
  baseline_level: 3
  baseline_rationale:
    - "internal operators"
    - "moderate blast radius"
domains:
  - name: "planning"
    blast_radius: "medium"
    touches:
      - "internal operators"
    constraints:
      - "preserve trust boundaries"
dimensions:
  - name: speed_vs_quality
    level: 3
    default_stance: "optimize for durability over shortcuts"
    raise_the_bar_triggers: ["production data"]
    allowed_shortcuts: ["time-boxed exploration"]
    red_lines: ["ship without review"]
    domain_overrides: []
  - name: type_safety_static_analysis
    level: 3
    default_stance: "type-safe by default"
    raise_the_bar_triggers: ["cross-boundary interfaces"]
    allowed_shortcuts: ["fixture-backed exploration"]
    red_lines: ["unchecked public contracts"]
    domain_overrides: []
  - name: testing_rigor
    level: 3
    default_stance: "test the shipped path"
    raise_the_bar_triggers: ["regression risk"]
    allowed_shortcuts: ["manual validation for throwaway work"]
    red_lines: ["merge without exercising the path"]
    domain_overrides: []
  - name: scalability_performance
    level: 3
    default_stance: "track obvious bottlenecks"
    raise_the_bar_triggers: ["user-visible latency"]
    allowed_shortcuts: ["defer micro-optimizations"]
    red_lines: ["ignore known load cliffs"]
    domain_overrides: []
  - name: reliability_operability
    level: 3
    default_stance: "prefer recoverable changes"
    raise_the_bar_triggers: ["long-lived state changes"]
    allowed_shortcuts: ["local-only iteration"]
    red_lines: ["unrecoverable migrations without a plan"]
    domain_overrides: []
  - name: security_privacy
    level: 3
    default_stance: "protect boundaries by default"
    raise_the_bar_triggers: ["credentials or user data"]
    allowed_shortcuts: ["synthetic data in local dev"]
    red_lines: ["plaintext secrets"]
    domain_overrides: []
  - name: observability
    level: 3
    default_stance: "emit enough proof to debug production issues"
    raise_the_bar_triggers: ["background jobs"]
    allowed_shortcuts: ["manual logs for local-only work"]
    red_lines: ["silent failures"]
    domain_overrides: []
  - name: dx_tooling_automation
    level: 3
    default_stance: "prefer repeatable tooling"
    raise_the_bar_triggers: ["frequent operator workflows"]
    allowed_shortcuts: ["small scripts while learning"]
    red_lines: ["undocumented one-off release steps"]
    domain_overrides: []
  - name: ux_polish_api_usability
    level: 3
    default_stance: "optimize for clear operator surfaces"
    raise_the_bar_triggers: ["new primary workflows"]
    allowed_shortcuts: ["rough edges on internal prototypes"]
    red_lines: ["ambiguous operator-facing behavior"]
    domain_overrides: []
exceptions:
  approvers:
    - project_owner
  record_location: "CHARTER.md#exceptions"
  minimum_fields:
    - what
    - why
    - scope
    - risk
    - owner
    - expiry_or_revisit_date
debt_tracking:
  system: "issues"
  labels:
    - debt
  review_cadence: "monthly"
decision_records:
  enabled: false
  path: ""
  format: ""
"#
}

fn guided_interview_answers() -> String {
    [
        "System",
        "greenfield",
        "2",
        "internal",
        "months",
        "cli, api",
        "server",
        "",
        "",
        "small team",
        "rust",
        "no",
        "",
        "",
        "best effort",
        "3",
        "internal operators, moderate blast radius",
        "not_required",
        "not_required",
        "lightweight",
        "not_required_yet",
        "standard",
        "",
        "project_owner",
        "",
        "",
        "issues",
        "debt",
        "monthly",
        "no",
    ]
    .join("\n")
        + "\n"
}

#[test]
fn interactive_happy_path_writes_canonical_charter() {
    let dir = scaffold_repo();

    let output = run_in_with_input_and_env(
        dir.path(),
        &["author", "charter"],
        &guided_interview_answers(),
        &[
            ("SYSTEM_AUTHOR_FORCE_TTY", "1"),
            (
                "SYSTEM_AUTHOR_TEST_SYNTHESIS_OUTPUT",
                "# Engineering Charter\n\ninteractive\n",
            ),
        ],
    );

    assert!(
        output.status.success(),
        "interactive author failed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"));
    assert!(out.contains("OBJECT: author charter"));
    assert!(out.contains("NEXT SAFE ACTION: run `system doctor`"));
    assert!(out.contains("MODE: guided_interview"));
    assert_eq!(
        fs::read_to_string(dir.path().join(".system/charter/CHARTER.md")).expect("charter"),
        "# Engineering Charter\n\ninteractive\n"
    );
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn authored_charter_clears_doctor_and_generate_without_feature_spec_truth() {
    let dir = scaffold_repo();

    let author = run_in_with_input_and_env(
        dir.path(),
        &["author", "charter"],
        &guided_interview_answers(),
        &[
            ("SYSTEM_AUTHOR_FORCE_TTY", "1"),
            (
                "SYSTEM_AUTHOR_TEST_SYNTHESIS_OUTPUT",
                "# Engineering Charter\n\nauthored\n",
            ),
        ],
    );

    assert!(
        author.status.success(),
        "author failed: {}",
        stdout(&author)
    );

    let doctor = run_in(dir.path(), &["doctor"]);
    assert!(
        doctor.status.success(),
        "doctor failed: {}",
        stdout(&doctor)
    );
    assert_eq!(stdout(&doctor).trim(), "READY");

    let generate = run_in(dir.path(), &["generate"]);
    assert!(
        generate.status.success(),
        "generate failed without feature spec truth: {}",
        stdout(&generate)
    );
    let generate_stdout = stdout(&generate);
    assert!(generate_stdout.contains("OUTCOME: READY"));
    assert!(generate_stdout.contains("OBJECT: planning.packet"));
    assert!(!generate_stdout.contains("### FEATURE_SPEC (.system/feature_spec/FEATURE_SPEC.md)"));
    assert!(generate_stdout.contains(
        "optional source omitted: .system/feature_spec/FEATURE_SPEC.md (shipped starter template)"
    ));
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
fn file_input_writes_only_canonical_charter_path() {
    let dir = scaffold_repo();
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, valid_structured_inputs_yaml());

    let output = run_in_with_env(
        dir.path(),
        &[
            "author",
            "charter",
            "--from-inputs",
            inputs_path.to_str().expect("utf-8 path"),
        ],
        &[(
            "SYSTEM_AUTHOR_TEST_SYNTHESIS_OUTPUT",
            "# Engineering Charter\n\nfile\n",
        )],
    );

    assert!(
        output.status.success(),
        "file-input author failed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("MODE: structured_inputs_file"));
    assert!(out.contains("SOURCE:"));
    assert_eq!(
        fs::read_to_string(dir.path().join(".system/charter/CHARTER.md")).expect("charter"),
        "# Engineering Charter\n\nfile\n"
    );
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn stdin_inputs_are_supported_with_dash() {
    let dir = scaffold_repo();

    let output = run_in_with_input_and_env(
        dir.path(),
        &["author", "charter", "--from-inputs", "-"],
        valid_structured_inputs_yaml(),
        &[(
            "SYSTEM_AUTHOR_TEST_SYNTHESIS_OUTPUT",
            "# Engineering Charter\n\nstdin\n",
        )],
    );

    assert!(
        output.status.success(),
        "stdin author failed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("MODE: structured_inputs_stdin"));
    assert!(out.contains("SOURCE: -"));
    assert_eq!(
        fs::read_to_string(dir.path().join(".system/charter/CHARTER.md")).expect("charter"),
        "# Engineering Charter\n\nstdin\n"
    );
}

#[test]
fn guided_interview_asks_one_follow_up_for_vague_required_text_and_accepts_concrete_retry() {
    let dir = scaffold_repo();
    let answers = [
        "System",
        "greenfield",
        "2",
        "internal",
        "months",
        "cli, api",
        "server",
        "",
        "",
        "idk",
        "small team with Rust and release pressure",
        "rust",
        "no",
        "",
        "",
        "best effort",
        "3",
        "internal operators, moderate blast radius",
        "not_required",
        "not_required",
        "lightweight",
        "not_required_yet",
        "standard",
        "",
        "project_owner",
        "",
        "",
        "issues",
        "debt",
        "monthly",
        "no",
    ]
    .join("\n")
        + "\n";

    let output = run_in_with_input_and_env(
        dir.path(),
        &["author", "charter"],
        &answers,
        &[
            ("SYSTEM_AUTHOR_FORCE_TTY", "1"),
            (
                "SYSTEM_AUTHOR_TEST_SYNTHESIS_OUTPUT",
                "# Engineering Charter\n\ninteractive\n",
            ),
        ],
    );

    assert!(
        output.status.success(),
        "interactive author failed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains(
        "Experience notes need a concrete summary of team experience or delivery constraints"
    ));
    assert_eq!(
        out.matches(
            "Experience notes need a concrete summary of team experience or delivery constraints"
        )
        .count(),
        1
    );
}

#[test]
fn guided_interview_refuses_after_second_vague_required_answer() {
    let dir = scaffold_repo();
    let answers = [
        "System",
        "greenfield",
        "2",
        "internal",
        "months",
        "cli, api",
        "server",
        "",
        "",
        "idk",
        "good quality",
    ]
    .join("\n")
        + "\n";

    let output = run_in_with_input_and_env(
        dir.path(),
        &["author", "charter"],
        &answers,
        &[
            ("SYSTEM_AUTHOR_FORCE_TTY", "1"),
            (
                "SYSTEM_AUTHOR_TEST_SYNTHESIS_OUTPUT",
                "# Engineering Charter\n\ninteractive\n",
            ),
        ],
    );

    assert!(!output.status.success(), "interactive author should refuse");
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: InterviewIncomplete"));
    assert!(out.contains(
        "guided charter interview could not normalize a concrete answer for experience notes"
    ));
    assert_eq!(
        out.matches(
            "Experience notes need a concrete summary of team experience or delivery constraints"
        )
        .count(),
        1
    );
}
