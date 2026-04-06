use system_compiler::{resolve, BudgetDisposition, BudgetPolicy, RefusalCategory, ResolveRequest};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[test]
fn refusal_system_root_missing_is_highest_priority() {
    let dir = tempfile::tempdir().expect("tempdir");
    let result = resolve(dir.path(), ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::SystemRootMissing);
}

#[test]
fn refusal_required_artifact_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    std::fs::create_dir_all(root.join(".system/feature_spec")).expect("mkdirs");
    write_file(&root.join(".system/feature_spec/FEATURE_SPEC.md"), b"spec");

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactMissing);
}

#[test]
fn refusal_required_artifact_empty() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"");
    write_file(&root.join(".system/feature_spec/FEATURE_SPEC.md"), b"spec");

    let result = resolve(root, ResolveRequest::default()).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::RequiredArtifactEmpty);
}

#[test]
fn refusal_budget_refused_is_selected_when_other_inputs_ok() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature spec that is longer than one byte",
    );

    let request = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(1),
        },
        packet_id: "planning.packet",
    };

    let result = resolve(root, request).expect("resolve");
    assert_eq!(result.budget_outcome.disposition, BudgetDisposition::Refuse);
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::BudgetRefused);
}

#[test]
fn refusal_unsupported_request_is_selected_for_live_execution_packet_when_other_inputs_ok() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(&root.join(".system/feature_spec/FEATURE_SPEC.md"), b"feature");

    let request = ResolveRequest {
        budget_policy: BudgetPolicy::default(),
        packet_id: "execution.live.packet",
    };

    let result = resolve(root, request).expect("resolve");
    let refusal = result.refusal.expect("refusal");
    assert_eq!(refusal.category, RefusalCategory::UnsupportedRequest);
    assert!(
        refusal.summary.contains("fixture-backed"),
        "expected boundary statement mentioning fixture-backed demos: {:?}",
        refusal.summary
    );
    assert!(
        refusal.summary.contains("planning"),
        "expected boundary statement mentioning planning packets: {:?}",
        refusal.summary
    );
}
