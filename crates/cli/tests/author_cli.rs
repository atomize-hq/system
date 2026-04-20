use std::fs;
use std::path::Path;
use std::process::{Command, Output, Stdio};

const CHARTER_SYNTHESIS_OVERRIDE_ENV_VAR: &str = "SYSTEM_AUTHOR_CHARTER_SYNTHESIS_OVERRIDE";

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

fn run_in_with_input(dir: &Path, args: &[&str], input: &str) -> Output {
    run_in_with_input_and_env(dir, args, input, &[])
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
    default_stance: "prefer automation that pays for itself"
    raise_the_bar_triggers: ["frequent repeated workflows"]
    allowed_shortcuts: ["temporary local scripts"]
    red_lines: ["manual-only release steps"]
    domain_overrides: []
  - name: ux_polish_api_usability
    level: 3
    default_stance: "clear operator and API ergonomics"
    raise_the_bar_triggers: ["external users"]
    allowed_shortcuts: ["rough internal copy while iterating"]
    red_lines: ["unclear operator failure modes"]
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

fn valid_charter_markdown() -> &'static str {
    r#"# Engineering Charter — System

## What this is
Done.

## How to use this charter
Done.

## Rubric: 1–5 rigor levels
Done.

## Project baseline posture
Done.

## Domains / areas (optional overrides)
Done.

## Posture at a glance (quick scan)
Done.

## Dimensions (details + guardrails)
Done.

## Cross-cutting red lines (global non-negotiables)
Done.

## Exceptions / overrides process
Done.

## Debt tracking expectations
Done.

## Decision Records (ADRs): how to use this charter
Done.

## Review & updates
Done.
"#
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

#[test]
fn file_inputs_refuse_existing_truth_before_parsing_malformed_yaml() {
    let dir = scaffold_repo();
    write_file(
        &dir.path().join(".system/charter/CHARTER.md"),
        "custom charter truth\n",
    );
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
        "existing charter truth should refuse before yaml parse: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: REFUSED"));
    assert!(out.contains("CATEGORY: ExistingCanonicalTruth"));
    assert!(!out.contains("CATEGORY: MalformedStructuredInput"));
}

#[test]
fn file_inputs_author_charter_successfully_with_internal_synthesis_override() {
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
        &[(CHARTER_SYNTHESIS_OVERRIDE_ENV_VAR, valid_charter_markdown())],
    );

    assert!(
        output.status.success(),
        "file inputs should succeed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"));
    assert!(out.contains("MODE: structured_inputs_file"));
    assert!(out.contains(&format!("SOURCE: {}", inputs_path.display())));
    assert_eq!(
        fs::read_to_string(dir.path().join(".system/charter/CHARTER.md")).expect("charter"),
        valid_charter_markdown()
    );
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn stdin_inputs_author_charter_successfully_with_internal_synthesis_override() {
    let dir = scaffold_repo();

    let output = run_in_with_input_and_env(
        dir.path(),
        &["author", "charter", "--from-inputs", "-"],
        valid_structured_inputs_yaml(),
        &[(CHARTER_SYNTHESIS_OVERRIDE_ENV_VAR, valid_charter_markdown())],
    );

    assert!(
        output.status.success(),
        "stdin inputs should succeed: {}",
        stdout(&output)
    );
    let out = stdout(&output);
    assert!(out.contains("OUTCOME: AUTHORED"));
    assert!(out.contains("MODE: structured_inputs_stdin"));
    assert!(out.contains("SOURCE: -"));
    assert_eq!(
        fs::read_to_string(dir.path().join(".system/charter/CHARTER.md")).expect("charter"),
        valid_charter_markdown()
    );
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}
