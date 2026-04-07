use system_compiler::{
    build_output_model, render_inspect, render_json, render_markdown, resolve, ResolveRequest,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[test]
fn render_markdown_keeps_trust_header_first_for_ready_result() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
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

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"context",
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
            "NEXT SAFE ACTION: create canonical .system root at .system",
        ]
    );
    assert!(rendered.contains("## REFUSAL"));
    assert!(rendered.contains("CATEGORY: SystemRootMissing"));
    assert!(rendered.contains("BROKEN SUBJECT: policy system_root"));
    assert!(rendered.contains("NEXT SAFE ACTION: create canonical .system root at .system"));
    assert!(
        !rendered.contains("## PACKET BODY"),
        "refusal output should stay compact: {rendered}"
    );
}

#[test]
fn render_json_is_deterministic_for_identical_models() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
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
fn render_inspect_is_deterministic_and_includes_json_fallback() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
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
            "NEXT SAFE ACTION: run `system inspect --packet planning.packet` for proof",
        ]
    );
    assert!(first.contains("## JSON FALLBACK"));
    assert!(first.contains("\"packet_id\": \"planning.packet\""));
    assert!(first.contains("## PACKET OVERVIEW"));
    assert!(first.contains("## PACKET BODY"));
    assert!(first.contains("### CHARTER (.system/charter/CHARTER.md)"));
}

#[test]
fn render_markdown_includes_execution_demo_fixture_context_and_ready_next_action() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("tests/fixtures/execution_demo/basic");
    write_file(&root.join(".system/charter/CHARTER.md"), b"demo charter");
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
    assert!(rendered.contains("demo charter"));
    assert!(rendered.contains("demo feature"));
}
