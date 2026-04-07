use system_compiler::{
    packet_result::PacketSectionMode, resolve, BudgetDisposition, BudgetPolicy,
    PacketSelectionStatus, ResolveRequest,
};

fn write_file(path: &std::path::Path, contents: &[u8]) {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).expect("mkdirs");
    }
    std::fs::write(path, contents).expect("write");
}

fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};

    format!("{:x}", Sha256::digest(bytes))
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
    assert!(result.packet_result.sections.is_empty());
    assert!(result
        .packet_result
        .notes
        .iter()
        .any(|note| { note.text == "packet body omitted because request is not ready" }));
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

    write_file(&repo_root.join(".system/charter/CHARTER.md"), b"c");
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

    write_file(&repo_root.join(".system/charter/CHARTER.md"), b"c");
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
    assert_eq!(
        summarize.budget_outcome.disposition,
        BudgetDisposition::Summarize
    );
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
    assert_eq!(
        exclude.budget_outcome.disposition,
        BudgetDisposition::Exclude
    );
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

#[test]
fn budget_summarize_replaces_optional_body_with_summary() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"project-context-oversized",
    );

    let summarize_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: None,
            max_per_artifact_bytes: Some(10),
        },
        ..ResolveRequest::default()
    };
    let result = resolve(repo_root, summarize_req).expect("resolve summarize");

    assert_eq!(
        result.budget_outcome.disposition,
        BudgetDisposition::Summarize
    );
    assert_eq!(result.packet_result.included_sources.len(), 3);
    let summarized_section = result
        .packet_result
        .sections
        .iter()
        .find(|section| section.title == "PROJECT_CONTEXT")
        .expect("project context section");
    assert_eq!(summarized_section.mode, PacketSectionMode::Summary);
    assert!(
        summarized_section
            .contents
            .contains("budget summary: full contents omitted"),
        "expected budget summary stub: {:?}",
        summarized_section.contents
    );
    assert!(
        !summarized_section
            .contents
            .contains("project-context-oversized"),
        "full optional contents should not leak once summarized: {:?}",
        summarized_section.contents
    );
}

#[test]
fn budget_exclude_removes_optional_body_from_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let repo_root = dir.path();

    write_file(&repo_root.join(".system/charter/CHARTER.md"), b"charter");
    write_file(
        &repo_root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature",
    );
    write_file(
        &repo_root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"project-context-oversized",
    );

    let exclude_req = ResolveRequest {
        budget_policy: BudgetPolicy {
            max_total_bytes: Some(12),
            max_per_artifact_bytes: None,
        },
        ..ResolveRequest::default()
    };
    let result = resolve(repo_root, exclude_req).expect("resolve exclude");

    assert_eq!(
        result.budget_outcome.disposition,
        BudgetDisposition::Exclude
    );
    assert_eq!(result.packet_result.included_sources.len(), 2);
    assert!(
        result
            .packet_result
            .included_sources
            .iter()
            .all(|source| source.canonical_repo_relative_path
                != ".system/project_context/PROJECT_CONTEXT.md"),
        "excluded sources should not be listed as included: {:?}",
        result.packet_result.included_sources
    );
    assert_eq!(result.packet_result.sections.len(), 2);
    assert!(
        result
            .packet_result
            .sections
            .iter()
            .all(|section| section.title != "PROJECT_CONTEXT"),
        "excluded optional section should be absent from packet body: {:?}",
        result.packet_result.sections
    );
}

#[test]
fn resolver_builds_typed_packet_body_for_planning_packet() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter body");
    write_file(
        &root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"project context body",
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature spec body",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    assert!(result.packet_result.is_ready());
    assert_eq!(
        result.packet_result.variant,
        system_compiler::packet_result::PacketVariant::Planning
    );
    assert!(result.packet_result.fixture_context.is_none());
    assert_eq!(result.packet_result.included_sources.len(), 3);
    assert_eq!(result.packet_result.sections.len(), 3);
    assert_eq!(result.packet_result.sections[0].title, "CHARTER");
    assert_eq!(result.packet_result.sections[1].title, "PROJECT_CONTEXT");
    assert_eq!(result.packet_result.sections[2].title, "FEATURE_SPEC");
    assert_eq!(
        result.packet_result.sections[0].mode,
        PacketSectionMode::Verbatim
    );
    assert_eq!(result.packet_result.sections[0].contents, "charter body");
    assert_eq!(
        result.packet_result.decision_summary.ready_next_safe_action,
        "run `system inspect --packet planning.packet` for proof"
    );
    assert!(
        result
            .packet_result
            .decision_summary
            .summary_line
            .contains("READY planning.packet"),
        "expected ready summary line: {:?}",
        result.packet_result.decision_summary.summary_line
    );
}

#[test]
fn ready_packet_sections_match_included_source_metadata() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter body");
    write_file(
        &root.join(".system/project_context/PROJECT_CONTEXT.md"),
        b"project context body",
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature spec body",
    );

    let result = resolve(root, ResolveRequest::default()).expect("resolve");

    for section in result
        .packet_result
        .sections
        .iter()
        .filter(|section| section.mode == PacketSectionMode::Verbatim)
    {
        let source = result
            .packet_result
            .included_sources
            .iter()
            .find(|source| {
                source.canonical_repo_relative_path == section.canonical_repo_relative_path
            })
            .expect("matching included source");

        let bytes = section.contents.as_bytes();
        assert_eq!(source.byte_len, Some(bytes.len() as u64));
        assert_eq!(
            source.content_sha256.as_deref(),
            Some(sha256_hex(bytes).as_str())
        );
    }
}

#[test]
fn resolver_builds_fixture_context_for_execution_demo_packets() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path().join("tests/fixtures/execution_demo/basic");

    write_file(
        &root.join(".system/charter/CHARTER.md"),
        b"demo charter body",
    );
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"demo feature body",
    );

    let request = ResolveRequest {
        packet_id: "execution.demo.packet",
        ..ResolveRequest::default()
    };

    let result = resolve(&root, request).expect("resolve");

    assert!(result.packet_result.is_ready());
    assert_eq!(
        result.packet_result.variant,
        system_compiler::packet_result::PacketVariant::ExecutionDemo
    );
    let fixture_context = result
        .packet_result
        .fixture_context
        .as_ref()
        .expect("fixture context");
    assert_eq!(fixture_context.fixture_set_id, "basic");
    assert_eq!(
        fixture_context.fixture_basis_root,
        "tests/fixtures/execution_demo/basic/.system/"
    );
    assert_eq!(fixture_context.fixture_lineage.len(), 2);
    assert_eq!(
        fixture_context.fixture_lineage[0].canonical_repo_relative_path,
        ".system/charter/CHARTER.md"
    );
    assert_eq!(
        fixture_context.fixture_lineage[1].canonical_repo_relative_path,
        ".system/feature_spec/FEATURE_SPEC.md"
    );
    assert_eq!(
        result.packet_result.decision_summary.ready_next_safe_action,
        "run `system inspect --packet execution.demo.packet --fixture-set basic` for proof"
    );
}

#[test]
fn resolver_redacts_packet_body_for_unsupported_live_execution_requests() {
    let dir = tempfile::tempdir().expect("tempdir");
    let root = dir.path();

    write_file(&root.join(".system/charter/CHARTER.md"), b"charter body");
    write_file(
        &root.join(".system/feature_spec/FEATURE_SPEC.md"),
        b"feature body",
    );

    let result = resolve(
        root,
        ResolveRequest {
            packet_id: "execution.live.packet",
            ..ResolveRequest::default()
        },
    )
    .expect("resolve");

    assert_eq!(result.selection.status, PacketSelectionStatus::Blocked);
    assert!(result.refusal.is_some());
    assert!(result.packet_result.sections.is_empty());
    assert!(result
        .packet_result
        .notes
        .iter()
        .any(|note| { note.text == "packet body omitted because request is not ready" }));
}
