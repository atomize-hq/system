use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use system_compiler::{
    author_charter_with_synthesizer, build_charter_synthesis_request,
    parse_charter_structured_input_yaml, preflight_author_charter, run_setup,
    setup_starter_template_bytes, synthesize_charter_markdown_with,
    validate_charter_structured_input, AuthorCharterRefusalKind, CharterAudience,
    CharterBackwardCompatibility, CharterDebtTrackingInput, CharterDecisionRecordsInput,
    CharterDefaultImplicationsInput, CharterDeprecationPolicy, CharterDimensionInput,
    CharterDimensionName, CharterDomainInput, CharterExceptionsInput, CharterExpectedLifetime,
    CharterObservabilityThreshold, CharterOperationalRealityInput, CharterPostureInput,
    CharterProjectClassification, CharterProjectConstraintsInput, CharterProjectInput,
    CharterRequiredness, CharterRolloutControls, CharterRuntimeEnvironment, CharterStructuredInput,
    CharterSurface, CharterSynthesisError, CharterSynthesisRequest, CharterSynthesizer,
    SetupRequest,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[derive(Clone)]
struct RecordingSynthesizer {
    requests: Arc<Mutex<Vec<CharterSynthesisRequest>>>,
    response: Result<String, String>,
}

impl RecordingSynthesizer {
    fn ok(markdown: &str) -> Self {
        Self {
            requests: Arc::new(Mutex::new(Vec::new())),
            response: Ok(markdown.to_string()),
        }
    }

    fn err(message: &str) -> Self {
        Self {
            requests: Arc::new(Mutex::new(Vec::new())),
            response: Err(message.to_string()),
        }
    }

    fn requests(&self) -> Vec<CharterSynthesisRequest> {
        self.requests.lock().expect("requests").clone()
    }
}

impl CharterSynthesizer for RecordingSynthesizer {
    fn synthesize(
        &self,
        _repo_root: &Path,
        request: CharterSynthesisRequest,
    ) -> Result<String, CharterSynthesisError> {
        self.requests.lock().expect("requests").push(request);
        match &self.response {
            Ok(markdown) => Ok(markdown.clone()),
            Err(message) => Err(CharterSynthesisError {
                message: message.clone(),
            }),
        }
    }
}

struct BlockingSynthesizer {
    entered_tx: Mutex<Option<mpsc::Sender<()>>>,
    release_rx: Mutex<mpsc::Receiver<()>>,
    requests: Arc<Mutex<Vec<CharterSynthesisRequest>>>,
    markdown: String,
}

impl BlockingSynthesizer {
    fn new(markdown: &str, entered_tx: mpsc::Sender<()>, release_rx: mpsc::Receiver<()>) -> Self {
        Self {
            entered_tx: Mutex::new(Some(entered_tx)),
            release_rx: Mutex::new(release_rx),
            requests: Arc::new(Mutex::new(Vec::new())),
            markdown: markdown.to_string(),
        }
    }

    fn requests(&self) -> Vec<CharterSynthesisRequest> {
        self.requests.lock().expect("requests").clone()
    }
}

impl CharterSynthesizer for BlockingSynthesizer {
    fn synthesize(
        &self,
        _repo_root: &Path,
        request: CharterSynthesisRequest,
    ) -> Result<String, CharterSynthesisError> {
        self.requests.lock().expect("requests").push(request);
        if let Some(entered_tx) = self.entered_tx.lock().expect("entered tx").take() {
            entered_tx.send(()).expect("notify synth entered");
        }
        self.release_rx
            .lock()
            .expect("release rx")
            .recv()
            .expect("release synth");
        Ok(self.markdown.clone())
    }
}

fn valid_input() -> CharterStructuredInput {
    CharterStructuredInput {
        schema_version: "0.1.0".to_string(),
        project: CharterProjectInput {
            name: "System".to_string(),
            classification: CharterProjectClassification::Greenfield,
            team_size: 2,
            users: CharterAudience::Internal,
            expected_lifetime: CharterExpectedLifetime::Months,
            surfaces: vec![CharterSurface::Cli, CharterSurface::Api],
            runtime_environments: vec![CharterRuntimeEnvironment::Server],
            constraints: CharterProjectConstraintsInput {
                deadline: "".to_string(),
                budget: "".to_string(),
                experience_notes: "small team".to_string(),
                must_use_tech: vec!["rust".to_string()],
            },
            operational_reality: CharterOperationalRealityInput {
                in_production_today: false,
                prod_users_or_data: "".to_string(),
                external_contracts_to_preserve: Vec::new(),
                uptime_expectations: "".to_string(),
            },
            default_implications: CharterDefaultImplicationsInput {
                backward_compatibility: CharterBackwardCompatibility::NotRequired,
                migration_planning: CharterRequiredness::NotRequired,
                rollout_controls: CharterRolloutControls::Lightweight,
                deprecation_policy: CharterDeprecationPolicy::NotRequiredYet,
                observability_threshold: CharterObservabilityThreshold::Standard,
            },
        },
        posture: CharterPostureInput {
            rubric_scale: "1-5".to_string(),
            baseline_level: 3,
            baseline_rationale: vec![
                "internal operators".to_string(),
                "moderate blast radius".to_string(),
            ],
        },
        domains: vec![CharterDomainInput {
            name: "planning".to_string(),
            blast_radius: "medium".to_string(),
            touches: vec!["internal".to_string()],
            constraints: vec!["trust product".to_string()],
        }],
        dimensions: vec![
            dimension(CharterDimensionName::SpeedVsQuality),
            dimension(CharterDimensionName::TypeSafetyStaticAnalysis),
            dimension(CharterDimensionName::TestingRigor),
            dimension(CharterDimensionName::ScalabilityPerformance),
            dimension(CharterDimensionName::ReliabilityOperability),
            dimension(CharterDimensionName::SecurityPrivacy),
            dimension(CharterDimensionName::Observability),
            dimension(CharterDimensionName::DxToolingAutomation),
            dimension(CharterDimensionName::UxPolishApiUsability),
        ],
        exceptions: CharterExceptionsInput {
            approvers: vec!["project_owner".to_string()],
            record_location: "CHARTER.md#exceptions".to_string(),
            minimum_fields: vec![
                "what".to_string(),
                "why".to_string(),
                "scope".to_string(),
                "risk".to_string(),
                "owner".to_string(),
                "expiry_or_revisit_date".to_string(),
            ],
        },
        debt_tracking: CharterDebtTrackingInput {
            system: "issues".to_string(),
            labels: vec!["debt".to_string()],
            review_cadence: "monthly".to_string(),
        },
        decision_records: CharterDecisionRecordsInput {
            enabled: true,
            path: "docs/decisions".to_string(),
            format: "md".to_string(),
        },
    }
}

fn dimension(name: CharterDimensionName) -> CharterDimensionInput {
    CharterDimensionInput {
        name,
        level: Some(3),
        default_stance: format!("default stance for {:?}", name),
        raise_the_bar_triggers: vec!["production data".to_string()],
        allowed_shortcuts: vec!["throwaway exploration".to_string()],
        red_lines: vec!["ship without review".to_string()],
        domain_overrides: Vec::new(),
    }
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

fn valid_charter_markdown_with_appendix(appendix: &str) -> String {
    format!("{}{}", valid_charter_markdown(), appendix)
}

fn scaffold_repo(root: &Path) {
    run_setup(root, &SetupRequest::default()).expect("setup scaffold");
}

#[test]
fn author_charter_refuses_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok("# Charter\n"),
    )
    .expect_err("missing system root should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::MissingSystemRoot);
    assert_eq!(err.next_safe_action, "run `system setup`");
}

#[test]
fn author_charter_refuses_when_system_root_is_invalid() {
    let dir = tempfile::tempdir().expect("tempdir");
    write_file(&dir.path().join(".system"), b"not a directory\n");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok("# Charter\n"),
    )
    .expect_err("invalid system root should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::InvalidSystemRoot);
}

#[test]
fn parse_structured_input_refuses_on_malformed_yaml() {
    let err = parse_charter_structured_input_yaml("not: [valid")
        .expect_err("malformed yaml should refuse");
    assert_eq!(err.kind, AuthorCharterRefusalKind::MalformedStructuredInput);
}

#[test]
fn validate_structured_input_refuses_on_incomplete_required_fields() {
    let mut input = valid_input();
    input.project.name.clear();
    input.dimensions[0].raise_the_bar_triggers.clear();

    let err = validate_charter_structured_input(&input)
        .expect_err("incomplete structured input should refuse");
    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("project.name"));
    assert!(err.summary.contains("dimensions[0].raise_the_bar_triggers"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_scalar_field() {
    let mut input = valid_input();
    input.project.constraints.experience_notes = "tbd".to_string();

    let err =
        validate_charter_structured_input(&input).expect_err("placeholder scalar should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("project.constraints.experience_notes"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_list_field() {
    let mut input = valid_input();
    input.posture.baseline_rationale = vec!["various".to_string()];

    let err =
        validate_charter_structured_input(&input).expect_err("placeholder list item should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("posture.baseline_rationale[0]"));
}

#[test]
fn validate_structured_input_refuses_placeholder_required_dimension_field() {
    let mut input = valid_input();
    input.dimensions[0].default_stance = "normal".to_string();

    let err = validate_charter_structured_input(&input)
        .expect_err("placeholder dimension field should refuse");

    assert_eq!(
        err.kind,
        AuthorCharterRefusalKind::IncompleteStructuredInput
    );
    assert!(err.summary.contains("dimensions[0].default_stance"));
}

#[test]
fn author_charter_replaces_starter_template_and_writes_only_canonical_output() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());

    let synthesizer = RecordingSynthesizer::ok(valid_charter_markdown());
    let result = author_charter_with_synthesizer(dir.path(), &valid_input(), &synthesizer)
        .expect("author charter");

    assert_eq!(
        result.canonical_repo_relative_path,
        ".system/charter/CHARTER.md"
    );
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".system/charter/CHARTER.md"))
            .expect("canonical charter"),
        valid_charter_markdown()
    );
    let mut charter_entries = std::fs::read_dir(dir.path().join(".system/charter"))
        .expect("read charter dir")
        .map(|entry| {
            entry
                .expect("charter dir entry")
                .file_name()
                .into_string()
                .expect("utf8 charter entry")
        })
        .collect::<Vec<_>>();
    charter_entries.sort();
    assert_eq!(charter_entries, vec!["CHARTER.md"]);
    assert!(!dir.path().join(".system/charter/CHARTER.md.lock").exists());
    assert!(!dir.path().join("artifacts/charter/CHARTER.md").exists());
    assert!(!dir.path().join("CHARTER.md").exists());
    assert_eq!(synthesizer.requests().len(), 1);
}

#[test]
fn author_charter_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    write_file(
        &dir.path().join(".system/charter/CHARTER.md"),
        b"custom charter truth\n",
    );

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok("# should not write\n"),
    )
    .expect_err("existing charter truth should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::ExistingCanonicalTruth);
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".system/charter/CHARTER.md"))
            .expect("existing charter"),
        "custom charter truth\n"
    );
}

#[test]
fn author_charter_rechecks_existing_truth_under_lock_before_writing() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());

    let first_markdown = valid_charter_markdown().replace("Done.", "First authored truth.");
    let second_markdown = valid_charter_markdown().replace("Done.", "Second authored truth.");
    let (entered_tx, entered_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let first_synthesizer = BlockingSynthesizer::new(&first_markdown, entered_tx, release_rx);
    let second_synthesizer = RecordingSynthesizer::ok(&second_markdown);
    let input = valid_input();

    thread::scope(|scope| {
        let first_run =
            scope.spawn(|| author_charter_with_synthesizer(dir.path(), &input, &first_synthesizer));

        entered_rx.recv().expect("first synthesis started");

        let second_run = scope
            .spawn(|| author_charter_with_synthesizer(dir.path(), &input, &second_synthesizer));

        release_tx.send(()).expect("release first synthesis");

        let first_result = first_run.join().expect("first author thread");
        let second_result = second_run.join().expect("second author thread");

        assert_eq!(
            first_result
                .expect("first author succeeds")
                .canonical_repo_relative_path,
            ".system/charter/CHARTER.md"
        );

        let second_err = second_result.expect_err("second author should refuse");
        assert_eq!(
            second_err.kind,
            AuthorCharterRefusalKind::ExistingCanonicalTruth
        );
    });

    assert_eq!(
        std::fs::read_to_string(dir.path().join(".system/charter/CHARTER.md"))
            .expect("final charter"),
        first_markdown
    );
    assert!(!dir.path().join(".system/charter/CHARTER.md.lock").exists());
    assert_eq!(first_synthesizer.requests().len(), 1);
    assert_eq!(second_synthesizer.requests().len(), 0);
}

#[test]
fn preflight_author_charter_refuses_when_non_starter_canonical_truth_exists() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    write_file(
        &dir.path().join(".system/charter/CHARTER.md"),
        b"custom charter truth\n",
    );

    let err = preflight_author_charter(dir.path())
        .expect_err("existing charter truth should refuse during preflight");

    assert_eq!(err.kind, AuthorCharterRefusalKind::ExistingCanonicalTruth);
    assert!(err
        .summary
        .contains("canonical charter truth already exists"));
}

#[test]
fn author_charter_does_not_partially_write_when_synthesis_fails() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let before = std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
        .expect("starter charter bytes");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::err("backend failed"),
    )
    .expect_err("synthesis failure should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::SynthesisFailed);
    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
            .expect("charter after failure"),
        before
    );
}

#[test]
fn author_charter_rejects_synthesis_output_with_commentary_preamble() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let before = std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
        .expect("starter charter bytes");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok(
            "Here is your charter:\n# Engineering Charter — System\n\n## What this is\n",
        ),
    )
    .expect_err("commentary preamble should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::SynthesisFailed);
    assert!(err
        .summary
        .contains("does not start with `# Engineering Charter`"));
    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
            .expect("charter after failure"),
        before
    );
}

#[test]
fn author_charter_rejects_synthesis_output_missing_required_headings() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let before = std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
        .expect("starter charter bytes");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok(
            "# Engineering Charter — System\n\n## What this is\nShort body.\n",
        ),
    )
    .expect_err("partial charter output should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::SynthesisFailed);
    assert!(err
        .summary
        .contains("does not satisfy the shipped charter template"));
    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
            .expect("charter after failure"),
        before
    );
}

#[test]
fn author_charter_rejects_synthesis_output_with_unresolved_placeholders() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let before = std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
        .expect("starter charter bytes");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok(
            "# Engineering Charter — {{PROJECT_NAME}}\n\n## What this is\nDone.\n## How to use this charter\nDone.\n## Rubric: 1–5 rigor levels\nDone.\n## Project baseline posture\nDone.\n## Domains / areas (optional overrides)\nDone.\n## Posture at a glance (quick scan)\nDone.\n## Dimensions (details + guardrails)\nDone.\n## Cross-cutting red lines (global non-negotiables)\nDone.\n## Exceptions / overrides process\nDone.\n## Debt tracking expectations\nDone.\n## Decision Records (ADRs): how to use this charter\nDone.\n## Review & updates\nDone.\n",
        ),
    )
    .expect_err("unresolved placeholders should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::SynthesisFailed);
    assert!(err
        .summary
        .contains("unresolved charter template placeholders"));
    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
            .expect("charter after failure"),
        before
    );
}

#[test]
fn author_charter_rejects_synthesis_output_with_template_comments() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let before = std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
        .expect("starter charter bytes");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok(
            "# Engineering Charter — System\n\n## What this is\nDone.\n<!-- template note -->\n## How to use this charter\nDone.\n## Rubric: 1–5 rigor levels\nDone.\n## Project baseline posture\nDone.\n## Domains / areas (optional overrides)\nDone.\n## Posture at a glance (quick scan)\nDone.\n## Dimensions (details + guardrails)\nDone.\n## Cross-cutting red lines (global non-negotiables)\nDone.\n## Exceptions / overrides process\nDone.\n## Debt tracking expectations\nDone.\n## Decision Records (ADRs): how to use this charter\nDone.\n## Review & updates\nDone.\n",
        ),
    )
    .expect_err("template comments should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::SynthesisFailed);
    assert!(err
        .summary
        .contains("charter template commentary instead of final markdown"));
    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
            .expect("charter after failure"),
        before
    );
}

#[test]
fn author_charter_rejects_required_headings_inside_fenced_block_appendix() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let before = std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
        .expect("starter charter bytes");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok(
            "# Engineering Charter — System\n\n## What this is\nDone.\n\n```markdown\n## How to use this charter\nDone.\n## Rubric: 1–5 rigor levels\nDone.\n## Project baseline posture\nDone.\n## Domains / areas (optional overrides)\nDone.\n## Posture at a glance (quick scan)\nDone.\n## Dimensions (details + guardrails)\nDone.\n## Cross-cutting red lines (global non-negotiables)\nDone.\n## Exceptions / overrides process\nDone.\n## Debt tracking expectations\nDone.\n## Decision Records (ADRs): how to use this charter\nDone.\n## Review & updates\nDone.\n```\n",
        ),
    )
    .expect_err("fenced headings should not satisfy the template");

    assert_eq!(err.kind, AuthorCharterRefusalKind::SynthesisFailed);
    assert!(err
        .summary
        .contains("does not satisfy the shipped charter template"));
    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
            .expect("charter after failure"),
        before
    );
}

#[test]
fn author_charter_rejects_duplicated_required_heading_in_appendix() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let before = std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
        .expect("starter charter bytes");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok(&valid_charter_markdown_with_appendix(
            "\n## What this is\nRepeated appendix heading.\n",
        )),
    )
    .expect_err("duplicate required heading should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::SynthesisFailed);
    assert!(err
        .summary
        .contains("does not satisfy the shipped charter template"));
    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
            .expect("charter after failure"),
        before
    );
}

#[test]
fn author_charter_rejects_out_of_order_required_heading() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let before = std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
        .expect("starter charter bytes");

    let err = author_charter_with_synthesizer(
        dir.path(),
        &valid_input(),
        &RecordingSynthesizer::ok(
            "# Engineering Charter — System\n\n## What this is\nDone.\n\n## Rubric: 1–5 rigor levels\nDone.\n\n## How to use this charter\nDone.\n\n## Project baseline posture\nDone.\n\n## Domains / areas (optional overrides)\nDone.\n\n## Posture at a glance (quick scan)\nDone.\n\n## Dimensions (details + guardrails)\nDone.\n\n## Cross-cutting red lines (global non-negotiables)\nDone.\n\n## Exceptions / overrides process\nDone.\n\n## Debt tracking expectations\nDone.\n\n## Decision Records (ADRs): how to use this charter\nDone.\n\n## Review & updates\nDone.\n",
        ),
    )
    .expect_err("out-of-order required headings should refuse");

    assert_eq!(err.kind, AuthorCharterRefusalKind::SynthesisFailed);
    assert!(err
        .summary
        .contains("does not satisfy the shipped charter template"));
    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md"))
            .expect("charter after failure"),
        before
    );
}

#[test]
fn shared_synthesis_request_is_used_by_future_interactive_and_deterministic_callers() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let input = valid_input();
    let expected_request = build_charter_synthesis_request(&input).expect("request");
    let synthesizer = RecordingSynthesizer::ok(valid_charter_markdown());

    let synthesized =
        synthesize_charter_markdown_with(dir.path(), &input, &synthesizer).expect("synthesize");
    assert_eq!(synthesized, valid_charter_markdown());

    let _ =
        author_charter_with_synthesizer(dir.path(), &input, &synthesizer).expect("author charter");

    let requests = synthesizer.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0], expected_request);
    assert_eq!(requests[1], expected_request);
}

#[test]
fn synthesis_request_embeds_method_artifact_and_closes_yaml_fence_on_its_own_line() {
    let request = build_charter_synthesis_request(&valid_input()).expect("request");

    assert!(request.prompt.contains("## Charter authoring method"));
    assert!(request.prompt.contains("# Charter Authoring Method"));
    assert!(request
        .prompt
        .contains("## Structured input source of truth"));
    assert!(request.prompt.contains("decision_records:"));
    assert!(request.prompt.contains("format: md\n```\n"));
}

#[test]
fn starter_template_fixture_remains_the_pre_write_state_for_scaffolded_authoring() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());

    assert_eq!(
        std::fs::read(dir.path().join(".system/charter/CHARTER.md")).expect("starter bytes"),
        setup_starter_template_bytes(system_compiler::CanonicalArtifactKind::Charter)
    );
}
