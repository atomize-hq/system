use system_compiler::{build_output_model, resolve, RenderSurface, ResolveRequest};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[test]
fn build_output_model_is_pure_for_identical_resolver_results() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let first = build_output_model(&result).expect("model");
    let second = build_output_model(&result).expect("model again");

    assert_eq!(first, second);
    assert_eq!(RenderSurface::Markdown.order(), 0);
    assert_eq!(RenderSurface::Json.order(), 1);
    assert_eq!(RenderSurface::Inspect.order(), 2);
}

#[test]
fn build_output_model_rejects_invalid_packet_state_without_touching_resolver_truth() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let original = result.clone();

    let mut invalid = result;
    invalid.selection.packet_id.clear();

    let err = build_output_model(&invalid).expect_err("model should fail");
    assert_eq!(format!("{err}"), "presentation failure: empty packet id");
    assert_eq!(original.selection.packet_id, "planning.packet");
}
