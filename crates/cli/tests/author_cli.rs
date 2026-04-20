use std::fs;
use std::io::{Read, Write};
use std::path::Path;
use std::process::{Command, Output, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use portable_pty::{native_pty_system, CommandBuilder, PtySize};

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

fn expected_file_inputs_markdown() -> String {
    let input =
        system_compiler::parse_charter_structured_input_yaml(valid_structured_inputs_yaml())
            .expect("parse valid structured input yaml");
    system_compiler::render_charter_markdown(&input).expect("render valid structured input")
}

fn guided_expected_input() -> system_compiler::CharterStructuredInput {
    let baseline_level = 3;
    let project_name = "System".to_string();
    let in_production_today = false;

    system_compiler::CharterStructuredInput {
        schema_version: "0.1.0".to_string(),
        project: system_compiler::CharterProjectInput {
            name: project_name.clone(),
            classification: system_compiler::CharterProjectClassification::Greenfield,
            team_size: 2,
            users: system_compiler::CharterAudience::Internal,
            expected_lifetime: system_compiler::CharterExpectedLifetime::Months,
            surfaces: vec![
                system_compiler::CharterSurface::Cli,
                system_compiler::CharterSurface::Api,
            ],
            runtime_environments: vec![system_compiler::CharterRuntimeEnvironment::Server],
            constraints: system_compiler::CharterProjectConstraintsInput {
                deadline: String::new(),
                budget: String::new(),
                experience_notes: "small team".to_string(),
                must_use_tech: vec!["rust".to_string()],
            },
            operational_reality: system_compiler::CharterOperationalRealityInput {
                in_production_today,
                prod_users_or_data: String::new(),
                external_contracts_to_preserve: Vec::new(),
                uptime_expectations: "best effort".to_string(),
            },
            default_implications: system_compiler::CharterDefaultImplicationsInput {
                backward_compatibility: system_compiler::CharterBackwardCompatibility::NotRequired,
                migration_planning: system_compiler::CharterRequiredness::NotRequired,
                rollout_controls: system_compiler::CharterRolloutControls::Lightweight,
                deprecation_policy: system_compiler::CharterDeprecationPolicy::NotRequiredYet,
                observability_threshold: system_compiler::CharterObservabilityThreshold::Standard,
            },
        },
        posture: system_compiler::CharterPostureInput {
            rubric_scale: "1-5".to_string(),
            baseline_level,
            baseline_rationale: vec![
                "internal operators".to_string(),
                "moderate blast radius".to_string(),
            ],
        },
        domains: vec![system_compiler::CharterDomainInput {
            name: "planning".to_string(),
            blast_radius: "medium".to_string(),
            touches: vec!["internal operators".to_string()],
            constraints: vec!["preserve trust boundaries".to_string()],
        }],
        dimensions: all_dimension_names()
            .iter()
            .copied()
            .map(|name| {
                default_dimension_input(name, baseline_level, &project_name, in_production_today)
            })
            .collect(),
        exceptions: system_compiler::CharterExceptionsInput {
            approvers: vec!["project_owner".to_string()],
            record_location: "CHARTER.md#exceptions".to_string(),
            minimum_fields: default_exception_minimum_fields(),
        },
        debt_tracking: system_compiler::CharterDebtTrackingInput {
            system: "issues".to_string(),
            labels: vec!["debt".to_string()],
            review_cadence: "monthly".to_string(),
        },
        decision_records: system_compiler::CharterDecisionRecordsInput {
            enabled: true,
            path: "docs/decisions".to_string(),
            format: "md".to_string(),
        },
    }
}

fn guided_expected_markdown() -> String {
    system_compiler::render_charter_markdown(&guided_expected_input())
        .expect("render guided expected input")
}

fn all_dimension_names() -> [system_compiler::CharterDimensionName; 9] {
    [
        system_compiler::CharterDimensionName::SpeedVsQuality,
        system_compiler::CharterDimensionName::TypeSafetyStaticAnalysis,
        system_compiler::CharterDimensionName::TestingRigor,
        system_compiler::CharterDimensionName::ScalabilityPerformance,
        system_compiler::CharterDimensionName::ReliabilityOperability,
        system_compiler::CharterDimensionName::SecurityPrivacy,
        system_compiler::CharterDimensionName::Observability,
        system_compiler::CharterDimensionName::DxToolingAutomation,
        system_compiler::CharterDimensionName::UxPolishApiUsability,
    ]
}

fn default_exception_minimum_fields() -> Vec<String> {
    [
        "what",
        "why",
        "scope",
        "risk",
        "owner",
        "expiry_or_revisit_date",
    ]
    .into_iter()
    .map(str::to_string)
    .collect()
}

fn default_dimension_input(
    name: system_compiler::CharterDimensionName,
    baseline_level: u8,
    project_name: &str,
    in_production_today: bool,
) -> system_compiler::CharterDimensionInput {
    let dimension_label = dimension_label(name);
    let production_trigger = if in_production_today {
        "changes touching live users, data, or uptime"
    } else {
        "changes that create irreversible migration or trust-boundary cost"
    };

    system_compiler::CharterDimensionInput {
        name,
        level: Some(baseline_level),
        default_stance: format!(
            "{project_name} defaults to level {baseline_level} on {dimension_label}; raise the bar whenever blast radius, trust boundaries, or recovery cost increases."
        ),
        raise_the_bar_triggers: vec![
            production_trigger.to_string(),
            "new external interfaces or contracts".to_string(),
        ],
        allowed_shortcuts: vec![
            "time-boxed exploration before merge".to_string(),
            "fixture-backed or local-only iteration with explicit follow-up".to_string(),
        ],
        red_lines: vec![
            format!("do not waive {dimension_label} expectations on shipped work"),
            "do not hide known risk without recording an exception".to_string(),
        ],
        domain_overrides: Vec::new(),
    }
}

fn dimension_label(name: system_compiler::CharterDimensionName) -> &'static str {
    match name {
        system_compiler::CharterDimensionName::SpeedVsQuality => "speed vs quality",
        system_compiler::CharterDimensionName::TypeSafetyStaticAnalysis => {
            "type safety and static analysis"
        }
        system_compiler::CharterDimensionName::TestingRigor => "testing rigor",
        system_compiler::CharterDimensionName::ScalabilityPerformance => {
            "scalability and performance"
        }
        system_compiler::CharterDimensionName::ReliabilityOperability => {
            "reliability and operability"
        }
        system_compiler::CharterDimensionName::SecurityPrivacy => "security and privacy",
        system_compiler::CharterDimensionName::Observability => "observability",
        system_compiler::CharterDimensionName::DxToolingAutomation => {
            "developer tooling and automation"
        }
        system_compiler::CharterDimensionName::UxPolishApiUsability => {
            "ux polish and api usability"
        }
    }
}

fn guided_prompt_answers() -> [(&'static str, &'static str); 35] {
    [
        ("Project name:", "System"),
        (
            "Project classification [greenfield|brownfield|integration|modernization|hardening]:",
            "greenfield",
        ),
        ("Team size (> 0):", "2"),
        ("Audience [internal|external|mixed]:", "internal"),
        ("Expected lifetime [days|weeks|months|years]:", "months"),
        ("Surfaces [web_app, api, cli, lib, infra, ml]:", "cli, api"),
        (
            "Runtime environments [browser, server, cloud, on_prem, edge]:",
            "server",
        ),
        ("Deadline or delivery window:", ""),
        ("Budget notes:", ""),
        ("Experience notes:", "small team"),
        ("Must-use tech (comma-separated, optional):", "rust"),
        ("In production today? [yes|no]:", "no"),
        ("Production users or data notes:", ""),
        (
            "External contracts to preserve (comma-separated, optional):",
            "",
        ),
        ("Uptime expectations:", "best effort"),
        ("Baseline rubric level [1-5]:", "3"),
        (
            "Baseline rationale (comma-separated, at least one):",
            "internal operators, moderate blast radius",
        ),
        (
            "Backward compatibility [required|not_required|boundary_only]:",
            "not_required",
        ),
        (
            "Migration planning [required|not_required]:",
            "not_required",
        ),
        (
            "Rollout controls [none|lightweight|required]:",
            "lightweight",
        ),
        (
            "Deprecation policy [required|not_required_yet]:",
            "not_required_yet",
        ),
        (
            "Observability threshold [minimal|standard|high|regulated]:",
            "standard",
        ),
        ("Primary domain name (optional):", "planning"),
        ("Primary domain blast radius:", "medium"),
        (
            "Primary domain touches (comma-separated, optional):",
            "internal operators",
        ),
        (
            "Primary domain constraints (comma-separated, optional):",
            "preserve trust boundaries",
        ),
        (
            "Exception approvers (comma-separated, at least one):",
            "project_owner",
        ),
        ("Exception record location [CHARTER.md#exceptions]:", ""),
        (
            "Exception minimum fields (comma-separated; press enter for standard fields):",
            "",
        ),
        ("Debt tracking system:", "issues"),
        ("Debt tracking labels (comma-separated, optional):", "debt"),
        ("Debt tracking review cadence:", "monthly"),
        ("Decision records enabled? [yes|no]:", "yes"),
        ("Decision records path:", "docs/decisions"),
        ("Decision records format:", "md"),
    ]
}

fn wait_for_transcript(transcript: &Arc<Mutex<String>>, needle: &str, timeout: Duration) {
    let deadline = Instant::now() + timeout;
    loop {
        let snapshot = transcript.lock().expect("transcript").clone();
        if snapshot.contains(needle) {
            return;
        }
        if Instant::now() >= deadline {
            panic!("timed out waiting for `{needle}` in transcript:\n{snapshot}");
        }
        thread::sleep(Duration::from_millis(10));
    }
}

fn run_guided_author_under_pty(dir: &Path) -> (String, portable_pty::ExitStatus) {
    let pty_system = native_pty_system();
    let pair = pty_system
        .openpty(PtySize {
            rows: 48,
            cols: 120,
            pixel_width: 0,
            pixel_height: 0,
        })
        .expect("open pty");

    let mut command = CommandBuilder::new(env!("CARGO_BIN_EXE_system"));
    command.cwd(dir);
    command.arg("author");
    command.arg("charter");

    let mut child = pair
        .slave
        .spawn_command(command)
        .expect("spawn author in pty");
    drop(pair.slave);

    let mut reader = pair.master.try_clone_reader().expect("clone pty reader");
    let mut writer = pair.master.take_writer().expect("take pty writer");

    let transcript = Arc::new(Mutex::new(String::new()));
    let transcript_reader = Arc::clone(&transcript);
    let reader_thread = thread::spawn(move || {
        let mut buffer = [0u8; 4096];
        loop {
            match reader.read(&mut buffer) {
                Ok(0) => break,
                Ok(read) => {
                    let text = String::from_utf8_lossy(&buffer[..read]);
                    transcript_reader
                        .lock()
                        .expect("transcript")
                        .push_str(&text);
                }
                Err(err) if err.kind() == std::io::ErrorKind::Interrupted => continue,
                Err(err) => panic!("read pty output: {err}"),
            }
        }
    });

    wait_for_transcript(
        &transcript,
        "Guided charter interview",
        Duration::from_secs(5),
    );
    for (prompt, answer) in guided_prompt_answers() {
        wait_for_transcript(&transcript, prompt, Duration::from_secs(5));
        writer
            .write_all(answer.as_bytes())
            .unwrap_or_else(|err| panic!("write answer for `{prompt}`: {err}"));
        writer
            .write_all(b"\n")
            .unwrap_or_else(|err| panic!("write newline for `{prompt}`: {err}"));
        writer
            .flush()
            .unwrap_or_else(|err| panic!("flush answer for `{prompt}`: {err}"));
    }
    wait_for_transcript(&transcript, "OUTCOME: AUTHORED", Duration::from_secs(10));
    drop(writer);

    let status = child.wait().expect("wait for author");
    drop(pair.master);
    reader_thread.join().expect("reader thread");
    let output = transcript.lock().expect("transcript").clone();
    (output, status)
}

fn make_repo_generation_ready(dir: &Path) {
    fs::remove_file(dir.join(".system/feature_spec/FEATURE_SPEC.md")).expect("remove feature spec");
    fs::remove_file(dir.join(".system/project_context/PROJECT_CONTEXT.md"))
        .expect("remove optional project context");
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
fn file_inputs_author_charter_successfully_with_deterministic_rendering() {
    let dir = scaffold_repo();
    let inputs_path = dir.path().join("charter-inputs.yaml");
    write_file(&inputs_path, valid_structured_inputs_yaml());

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
        expected_file_inputs_markdown()
    );
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn stdin_inputs_author_charter_successfully_with_deterministic_rendering() {
    let dir = scaffold_repo();

    let output = run_in_with_input(
        dir.path(),
        &["author", "charter", "--from-inputs", "-"],
        valid_structured_inputs_yaml(),
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
        expected_file_inputs_markdown()
    );
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
}

#[test]
fn guided_tty_author_charter_succeeds_via_real_binary_path() {
    let dir = scaffold_repo();

    let (output, status) = run_guided_author_under_pty(dir.path());

    assert!(
        status.success(),
        "guided PTY author should succeed: {output}"
    );
    assert!(output.contains("Guided charter interview"), "{output}");
    assert!(output.contains("Project name:"), "{output}");
    assert!(output.contains("Decision records format:"), "{output}");
    assert!(output.contains("OUTCOME: AUTHORED"), "{output}");
    assert!(output.contains("MODE: guided_interview"), "{output}");
    assert_eq!(
        fs::read_to_string(dir.path().join(".system/charter/CHARTER.md")).expect("charter"),
        guided_expected_markdown()
    );
}

#[test]
fn guided_tty_author_charter_unblocks_doctor_and_generate_once_optional_starters_are_removed() {
    let dir = scaffold_repo();

    let (author_output, status) = run_guided_author_under_pty(dir.path());
    assert!(
        status.success(),
        "guided PTY author should succeed: {author_output}"
    );
    make_repo_generation_ready(dir.path());

    let doctor = run_in(dir.path(), &["doctor"]);
    assert!(
        doctor.status.success(),
        "doctor should succeed after authoring: {}",
        stdout(&doctor)
    );
    assert_eq!(stdout(&doctor).trim(), "READY");

    let generate = run_in(dir.path(), &["generate"]);
    assert!(
        generate.status.success(),
        "generate should succeed after authoring: {}",
        stdout(&generate)
    );
    let generate_stdout = stdout(&generate);
    assert!(
        generate_stdout.contains("OUTCOME: READY"),
        "{generate_stdout}"
    );
    assert!(
        generate_stdout.contains("### CHARTER (.system/charter/CHARTER.md)"),
        "{generate_stdout}"
    );
    assert!(
        generate_stdout.contains("# Engineering Charter — System"),
        "{generate_stdout}"
    );
}
