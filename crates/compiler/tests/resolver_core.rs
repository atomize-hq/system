use system_compiler::{
    resolve, BudgetDisposition, BudgetPolicy, PacketSelectionStatus, ResolveRequest,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

#[test]
fn resolver_returns_typed_result_when_system_root_missing() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    let result = resolve(repo_root, ResolveRequest::default()).expect("resolve");

    assert_eq!(result.c04_result_version, "reduced-v1");
    assert_eq!(result.c03_schema_version, "reduced-v1");
    assert_eq!(result.c03_manifest_generation_version, 1);
    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    assert_eq!(result.c03_fingerprint_sha256.len(), 64);
    assert!(result
        .c03_fingerprint_sha256
        .chars()
        .all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn resolver_is_deterministic_for_identical_inputs() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        b"c",
    );
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"f",
    );

    let req = ResolveRequest::default();
    let a = resolve(repo_root, req.clone()).expect("resolve a");
    let b = resolve(repo_root, req).expect("resolve b");

    assert_eq!(a, b);
}

#[test]
fn budget_next_safe_action_is_only_present_on_refuse() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(
        &repo_root.join(".system/charter/CHARTER.md"),
        b"c",
    );
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"f",
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"cc",
    );

    // Summarize optional.
    let summarize_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(1),
        },
        ..ResolveRequest::default()
    };
    let summarize = resolve(repo_root, summarize_req).expect("resolve summarize");
    assert_eq!(summarize.budget_outcome.disposition, BudgetDisposition::Summarize);
    assert!(summarize.budget_outcome.next_safe_action.is_none());

    // Exclude optional.
    let exclude_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: Some(2),
            max_per_artifact_bytes: None,
        },
        ..ResolveRequest::default()
    };
    let exclude = resolve(repo_root, exclude_req).expect("resolve exclude");
    assert_eq!(exclude.budget_outcome.disposition, BudgetDisposition::Exclude);
    assert!(exclude.budget_outcome.next_safe_action.is_none());

    // Refuse required.
    let refuse_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(0),
        },
        ..ResolveRequest::default()
    };
    let refuse = resolve(repo_root, refuse_req).expect("resolve refuse");
    assert_eq!(refuse.budget_outcome.disposition, BudgetDisposition::Refuse);
    assert!(refuse.budget_outcome.next_safe_action.is_some());
}

