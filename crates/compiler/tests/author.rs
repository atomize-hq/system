use std::path::Path;
use std::sync::{Arc, Mutex};

use system_compiler::{
    author_charter_with_synthesizer, build_charter_synthesis_request,
    parse_charter_structured_input_yaml, run_setup, setup_starter_template_bytes,
    synthesize_charter_markdown_with, validate_charter_structured_input, AuthorCharterRefusalKind,
    CharterAudience, CharterBackwardCompatibility, CharterDebtTrackingInput,
    CharterDecisionRecordsInput, CharterDefaultImplicationsInput, CharterDeprecationPolicy,
    CharterDimensionInput, CharterDimensionName, CharterDomainInput, CharterExpectedLifetime,
    CharterExceptionsInput, CharterObservabilityThreshold, CharterOperationalRealityInput,
    CharterPostureInput, CharterProjectClassification, CharterProjectConstraintsInput,
    CharterProjectInput, CharterRequiredness, CharterRolloutControls,
    CharterRuntimeEnvironment, CharterStructuredInput, CharterSurface, CharterSynthesizer,
    CharterSynthesisError, CharterSynthesisRequest, SetupRequest,
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
    assert_eq!(err.kind, AuthorCharterRefusalKind::IncompleteStructuredInput);
    assert!(err.summary.contains("project.name"));
    assert!(err.summary.contains("dimensions[0].raise_the_bar_triggers"));
}

#[test]
fn author_charter_replaces_starter_template_and_writes_only_canonical_output() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());

    let synthesizer = RecordingSynthesizer::ok("# Engineering Charter — System\n");
    let result = author_charter_with_synthesizer(dir.path(), &valid_input(), &synthesizer)
        .expect("author charter");

    assert_eq!(result.canonical_repo_relative_path, ".system/charter/CHARTER.md");
    assert_eq!(
        std::fs::read_to_string(dir.path().join(".system/charter/CHARTER.md"))
            .expect("canonical charter"),
        "# Engineering Charter — System\n"
    );
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
fn shared_synthesis_request_is_used_by_future_interactive_and_deterministic_callers() {
    let dir = tempfile::tempdir().expect("tempdir");
    scaffold_repo(dir.path());
    let input = valid_input();
    let expected_request = build_charter_synthesis_request(&input).expect("request");
    let synthesizer = RecordingSynthesizer::ok("# Engineering Charter — System\n");

    let synthesized =
        synthesize_charter_markdown_with(dir.path(), &input, &synthesizer).expect("synthesize");
    assert_eq!(synthesized, "# Engineering Charter — System\n");

    let _ = author_charter_with_synthesizer(dir.path(), &input, &synthesizer)
        .expect("author charter");

    let requests = synthesizer.requests();
    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0], expected_request);
    assert_eq!(requests[1], expected_request);
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
