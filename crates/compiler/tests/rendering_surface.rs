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
            "NEXT SAFE ACTION: render packet body once implemented (SEAM-5)",
        ]
    );
    assert_eq!(render_markdown(&model), rendered);
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
            "NEXT SAFE ACTION: render packet body once implemented (SEAM-5)",
        ]
    );
    assert!(first.contains("## JSON FALLBACK"));
    assert!(first.contains("\"packet_id\": \"planning.packet\""));
}
