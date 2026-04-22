use system_compiler::{
    build_output_model, render_inspect, render_json, render_markdown, resolve, BudgetPolicy,
    ResolveRequest,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn valid_charter_markdown() -> &'static str {
    "# Engineering Charter — System

## What this is
Body.

## How to use this charter
Use it.

## Rubric: 1–5 rigor levels
Levels.

## Project baseline posture
Baseline.

## Domains / areas (optional overrides)
None.

## Posture at a glance (quick scan)
Snapshot.

## Dimensions (details + guardrails)
Details.

## Cross-cutting red lines (global non-negotiables)
- Keep trust boundaries intact.

## Exceptions / overrides process
- **Approvers:** project_owner
- **Record location:** docs/exceptions.md
- **Minimum required fields:**
  - what
  - why
  - scope
  - risk
  - owner
  - expiry_or_revisit_date

## Debt tracking expectations
Tracked in issues.

## Decision Records (ADRs): how to use this charter
Use ADRs.

## Review & updates
Review monthly.
"
}

fn valid_project_context_markdown() -> &'static str {
    "# Project Context — System

> **File:** `PROJECT_CONTEXT.md`
> **Created (UTC):** 2026-04-21T00:00:00Z
> **Owner:** project-owner
> **Team:** system-team
> **Repo / Project:** /tmp/system
> **Charter Ref:** .system/charter/CHARTER.md

## What this is
Project reality.

## How to use this
Use this document to ground planning in reality.

## 0) Project Summary (factual, 3–6 bullets)
- Summary.

## 1) Operational Reality (the most important section)
- Operations.

## 2) Project Classification Implications (planning guardrails)
- Guardrails.

## 3) System Boundaries (what we own vs integrate with)
### What we own
- Canonical `.system/` truth.
### What we do NOT own (but may depend on)
- External delivery systems.

## 4) Integrations & Contracts (top 1–5)
- Integrations.

## 5) Environments & Delivery
- Delivery.

## 6) Data Reality
- Data.

## 7) Repo / Codebase Reality (brownfield-friendly, but safe for greenfield)
- Codebase.

## 8) Constraints
- Constraints.

## 9) Known Unknowns (explicitly tracked)
- Unknowns.

## 10) Update Triggers
- Update when reality changes.
"
}

fn oversized_valid_project_context_markdown() -> String {
    format!("{}\n{}", valid_project_context_markdown(), "x".repeat(256))
}

#[test]
fn render_markdown_keeps_trust_header_first_for_ready_result() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    let lines: Vec<&str> = rendered.lines().take(3).collect();

    assert_eq!(
        lines,
        [
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system inspect --packet planning.packet` for proof",
        ]
    );
    assert_eq!(render_markdown(&model), rendered);
    assert!(rendered.contains("## PACKET OVERVIEW"));
    assert!(rendered.contains("PACKET VARIANT: planning.packet"));
    assert!(rendered.contains("## INCLUDED SOURCES"));
    assert!(rendered.contains("Charter [.system/charter/CHARTER.md]"));
    assert!(rendered.contains("FeatureSpec [.system/feature_spec/FEATURE_SPEC.md]"));
    assert!(rendered.contains("## OMISSIONS AND BUDGET"));
    assert!(rendered.contains("## DECISION SUMMARY"));
    assert!(rendered.contains("## PACKET BODY"));
    assert!(rendered.contains("### CHARTER (.system/charter/CHARTER.md)"));
    assert!(rendered.contains("### FEATURE_SPEC (.system/feature_spec/FEATURE_SPEC.md)"));
    assert!(
        !rendered.contains("render packet body once implemented"),
        "placeholder body text should be gone: {rendered}"
    );
}

#[test]
fn render_markdown_keeps_optional_project_context_in_order_when_present() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/project_context/PROJECT_CONTEXT.md"),
        valid_project_context_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    let pos_charter = rendered
        .find("### CHARTER (.system/charter/CHARTER.md)")
        .expect("charter section");
    let pos_context = rendered
        .find("### PROJECT_CONTEXT (.system/project_context/PROJECT_CONTEXT.md)")
        .expect("project context section");
    let pos_feature = rendered
        .find("### FEATURE_SPEC (.system/feature_spec/FEATURE_SPEC.md)")
        .expect("feature spec section");

    assert!(
        pos_charter < pos_context && pos_context < pos_feature,
        "expected section order charter -> project_context -> feature_spec: {rendered}"
    );
    assert!(rendered.contains("ProjectContext [.system/project_context/PROJECT_CONTEXT.md]"));
}

#[test]
fn render_markdown_omits_optional_feature_spec_starter_template_from_ready_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        system_compiler::setup_starter_template_bytes(
            system_compiler::CanonicalArtifactKind::FeatureSpec,
        ),
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");
    let rendered = render_markdown(&model);

    assert!(rendered.contains("OUTCOME: READY"));
    assert!(rendered.contains("Charter [.system/charter/CHARTER.md]"));
    assert!(!rendered.contains("FeatureSpec [.system/feature_spec/FEATURE_SPEC.md]"));
    assert!(!rendered.contains("### FEATURE_SPEC (.system/feature_spec/FEATURE_SPEC.md)"));
    assert!(rendered.contains(
        "optional source omitted: .system/feature_spec/FEATURE_SPEC.md (shipped starter template)"
    ));
}

#[test]
fn render_markdown_keeps_trust_header_first_for_refusal_result() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    let lines: Vec<&str> = rendered.lines().take(3).collect();

    assert_eq!(
        lines,
        [
            "OUTCOME: REFUSED",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system setup`",
        ]
    );
    assert!(rendered.contains("## REFUSAL"));
    assert!(rendered.contains("CATEGORY: SystemRootMissing"));
    assert!(rendered.contains("BROKEN SUBJECT: policy system_root"));
    assert!(rendered.contains("NEXT SAFE ACTION: run `system setup`"));
    assert!(
        !rendered.contains("## PACKET BODY"),
        "refusal output should stay compact: {rendered}"
    );
}

#[test]
fn render_json_is_deterministic_for_identical_models() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let first = render_json(&model);
    let second = render_json(&model);

    assert_eq!(first, second);
    assert!(first.contains("\"budget_outcome\""));
    assert!(first.contains("\"decision_log_entries\""));
    assert!(first.contains("\"packet_result\""));
    assert!(first.contains("\"ready_next_safe_action\""));
    assert!(first.contains("\"sections\""));
}

#[test]
fn render_json_redacts_packet_body_for_refused_live_execution_requests() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature-body",
    );

    let result = resolve(
        root,
        ResolveRequest {
            packet_id: "execution.live.packet",
            ..Default::default()
        },
    )
    .expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_json(&model);

    assert!(rendered.contains("\"packet_result\""));
    assert!(rendered.contains("\"sections\": [\n    ]"));
    assert!(rendered.contains("\"packet body omitted because request is not ready\""));
    assert!(!rendered.contains(valid_charter_markdown()));
    assert!(!rendered.contains("\"contents\": \"feature-body\""));
}

#[test]
fn render_json_does_not_mislabel_optional_read_error_as_omission() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature-body",
    );
    std::fs::create_dir_all(root.join(".system/project_context/PROJECT_CONTEXT.md"))
        .expect("project_context dir");

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_json(&model);

    assert!(rendered.contains("\"packet_result\""));
    assert!(rendered.contains("\"packet body omitted because request is not ready\""));
    assert!(rendered.contains("\"category\": \"ArtifactReadError\""));
    assert!(!rendered.contains(
        "\"text\": \"optional source omitted: .system/project_context/PROJECT_CONTEXT.md\""
    ));
}

#[test]
fn render_inspect_is_deterministic_and_includes_json_fallback() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let first = render_inspect(&model);
    let second = render_inspect(&model);

    assert_eq!(first, second);
    assert_eq!(
        first.lines().take(3).collect::<Vec<_>>(),
        vec![
            "OUTCOME: READY",
            "OBJECT: planning.packet",
            "NEXT SAFE ACTION: run `system generate --packet planning.packet`",
        ]
    );
    assert!(first.contains("## JSON FALLBACK"));
    assert!(first.contains("\"packet_id\": \"planning.packet\""));
    assert!(
        !first.contains("run `system inspect --packet planning.packet` for proof"),
        "inspect ready path should not loop back into inspect: {first}"
    );
    assert!(first.contains("## PACKET OVERVIEW"));
    assert!(first.contains("## PACKET BODY"));
    assert!(first.contains("### CHARTER (.system/charter/CHARTER.md)"));
}

#[test]
fn render_markdown_includes_execution_demo_fixture_context_and_ready_next_action() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("tests/fixtures/execution_demo/basic");
    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"demo feature",
    );

    let request = system_compiler::ResolveRequest {
        packet_id: "execution.demo.packet",
        ..Default::default()
    };
    let result = resolve(&root, request).expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    assert_eq!(
        rendered.lines().take(3).collect::<Vec<_>>(),
        vec![
            "OUTCOME: READY",
            "OBJECT: execution.demo.packet",
            "NEXT SAFE ACTION: run `system inspect --packet execution.demo.packet --fixture-set basic` for proof",
        ]
    );
    assert!(rendered.contains("## FIXTURE DEMO"));
    assert!(rendered.contains("MODE: fixture-backed execution demo"));
    assert!(rendered.contains("FIXTURE SET: basic"));
    assert!(rendered.contains("FIXTURE BASIS ROOT: tests/fixtures/execution_demo/basic/.system/"));
    assert!(rendered.contains("FIXTURE LINEAGE:"));
    assert!(rendered.contains("Charter [.system/charter/CHARTER.md]"));
    assert!(rendered.contains("FeatureSpec [.system/feature_spec/FEATURE_SPEC.md]"));
    assert!(rendered.contains("### CHARTER (.system/charter/CHARTER.md)"));
    assert!(rendered.contains("# Engineering Charter — System"));
    assert!(rendered.contains("demo feature"));
}

#[test]
fn render_markdown_marks_budget_summarized_sections_without_leaking_body() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_file(
        &root.join(".system/project_context/PROJECT_CONTEXT.md"),
        oversized_valid_project_context_markdown().as_bytes(),
    );

    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: None,
                max_per_artifact_bytes: Some(valid_charter_markdown().len() as u64),
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    assert!(rendered.contains("### PROJECT_CONTEXT (.system/project_context/PROJECT_CONTEXT.md)"));
    assert!(rendered.contains("MODE: summarized due to budget"));
    assert!(rendered.contains("budget summary: full contents omitted"));
    assert!(!rendered.contains(valid_project_context_markdown()));
}

#[test]
fn render_markdown_omits_budget_excluded_optional_sections() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        valid_charter_markdown().as_bytes(),
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_file(
        &root.join(".system/project_context/PROJECT_CONTEXT.md"),
        oversized_valid_project_context_markdown().as_bytes(),
    );

    let result = resolve(
        root,
        ResolveRequest {
            budget_policy: BudgetPolicy {
                max_total_bytes: Some((valid_charter_markdown().len() + "feature".len()) as u64),
                max_per_artifact_bytes: None,
            },
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");
    let model = build_output_model(&result).expect("model");

    let rendered = render_markdown(&model);
    assert!(!rendered.contains("ProjectContext [.system/project_context/PROJECT_CONTEXT.md]"));
    assert!(!rendered.contains("### PROJECT_CONTEXT (.system/project_context/PROJECT_CONTEXT.md)"));
    assert!(rendered.contains(
        "optional source excluded due to budget: .system/project_context/PROJECT_CONTEXT.md"
    ));
}
