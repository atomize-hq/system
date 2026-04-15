#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use std::fs;
use std::path::Path;

use system_compiler::{
    apply_pipeline_capture, capture_pipeline_output, preview_pipeline_capture,
    render_pipeline_capture_apply_result, render_pipeline_capture_preview,
    render_pipeline_capture_refusal, set_route_state, PipelineCaptureRefusalClassification,
    PipelineCaptureRequest, RouteStateMutation, RouteStateMutationOutcome,
};

const PIPELINE_ID: &str = pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID;
const STAGE_05_ID: &str = pipeline_proof_corpus_support::STAGE_05_CHARTER_SYNTHESIZE_ID;
const STAGE_07_ID: &str = pipeline_proof_corpus_support::STAGE_07_FOUNDATION_PACK_ID;

fn stage_05_request(input: String) -> PipelineCaptureRequest {
    PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_05_ID.to_string(),
        input,
    }
}

fn stage_07_request(input: String) -> PipelineCaptureRequest {
    PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_07_ID.to_string(),
        input,
    }
}

fn stage_05_capture_input() -> String {
    pipeline_proof_corpus_support::read_committed_fixture("artifacts/charter/CHARTER.md")
}

fn stage_07_capture_input() -> String {
    let outputs = [
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
        "artifacts/foundation/TEST_STRATEGY_BRIEF.md",
        "artifacts/foundation/QUALITY_GATES_SPEC.md",
        "artifacts/foundation/quality_gates.yaml",
        "artifacts/foundation/ENVIRONMENT_INVENTORY.md",
    ];
    let mut out = String::new();
    for path in outputs {
        out.push_str(&format!("--- FILE: {path} ---\n"));
        out.push_str(&pipeline_proof_corpus_support::read_committed_fixture(path));
        out.push('\n');
    }
    out
}

fn normalize_capture_id(output: &str, capture_id: &str) -> String {
    output.replace(capture_id, "{{CAPTURE_ID}}")
}

fn apply_mutation(repo_root: &Path, mutation: RouteStateMutation) {
    let (definition, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(repo_root);
    let state = system_compiler::load_route_state_with_supported_variables(
        repo_root,
        PIPELINE_ID,
        &supported_variables,
    )
    .expect("load route state");
    let outcome = set_route_state(
        repo_root,
        PIPELINE_ID,
        supported_variables.iter().map(String::as_str),
        mutation,
        state.revision,
    )
    .expect("set route state");
    match outcome {
        RouteStateMutationOutcome::Applied(_) => {}
        RouteStateMutationOutcome::Refused(refusal) => {
            panic!("expected mutation to apply, got {refusal:?}")
        }
    }
    let _ = definition;
}

#[test]
fn capture_preview_charter_matches_shared_golden() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let rendered = render_pipeline_capture_preview(&preview);
    let normalized = normalize_capture_id(&rendered, &preview.plan.capture_id);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalized,
        &[],
        "capture.preview.stage_05_charter_synthesize.txt",
    );
    assert!(pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id
    )
    .is_file());
}

#[test]
fn capture_preview_foundation_pack_matches_shared_golden() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_07_request(stage_07_capture_input()))
        .expect("preview");
    let rendered = render_pipeline_capture_preview(&preview);
    let normalized = normalize_capture_id(&rendered, &preview.plan.capture_id);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &normalized,
        &[],
        "capture.preview.stage_07_foundation_pack.txt",
    );
}

#[test]
fn capture_apply_charter_matches_shared_golden_and_writes_repo_mirror() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let result = capture_pipeline_output(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.apply.stage_05_charter_synthesize.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        stage_05_capture_input()
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("repo mirror"),
        stage_05_capture_input()
    );
}

#[test]
fn capture_apply_foundation_pack_matches_shared_golden_and_uses_cached_preview() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_07_request(stage_07_capture_input()))
        .expect("preview");
    let cache_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id,
    );
    assert!(cache_path.is_file(), "preview should write capture cache");

    let result = apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect("apply");
    let rendered = render_pipeline_capture_apply_result(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.apply.stage_07_foundation_pack.txt",
    );
    assert!(
        !cache_path.exists(),
        "successful apply should clear cached preview"
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("ENVIRONMENT_INVENTORY.md")).expect("repo mirror"),
        pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/foundation/ENVIRONMENT_INVENTORY.md"
        )
    );
}

#[test]
fn capture_refuses_single_file_with_file_wrapper() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let wrapped = format!(
        "--- FILE: artifacts/charter/CHARTER.md ---\n{}",
        stage_05_capture_input()
    );
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_05_request(wrapped)).expect_err("refusal");
    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_05_ID)),
        &[],
        "capture.refused.single_file_with_file_wrapper.txt",
    );
}

#[test]
fn capture_refuses_missing_declared_block() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let missing = [
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
        "artifacts/foundation/TEST_STRATEGY_BRIEF.md",
        "artifacts/foundation/QUALITY_GATES_SPEC.md",
        "artifacts/foundation/quality_gates.yaml",
    ]
    .into_iter()
    .map(|path| {
        format!(
            "--- FILE: {path} ---\n{}",
            pipeline_proof_corpus_support::read_committed_fixture(path)
        )
    })
    .collect::<Vec<_>>()
    .join("\n");
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_07_request(missing)).expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.missing_declared_block.txt",
    );
}

#[test]
fn capture_refuses_duplicate_declared_block() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let mut input = stage_07_capture_input();
    input.push_str(&format!(
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\n{}",
        pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/foundation/ENVIRONMENT_INVENTORY.md"
        )
    ));
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_07_request(input)).expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.duplicate_declared_block.txt",
    );
}

#[test]
fn capture_refuses_undeclared_block() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let mut input = stage_07_capture_input();
    input.push_str("--- FILE: artifacts/foundation/EXTRA_NOT_DECLARED.md ---\nextra\n");
    let refusal =
        preview_pipeline_capture(&repo_root, &stage_07_request(input)).expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.undeclared_block.txt",
    );
}

#[test]
fn capture_refuses_stale_route_basis_after_preview() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    apply_mutation(
        &repo_root,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: true,
        },
    );
    let refusal =
        apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect_err("refusal");
    let rendered = render_pipeline_capture_refusal(&refusal, None, None);
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &rendered,
        &[],
        "capture.refused.stale_route_basis.txt",
    );
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &preview.plan.capture_id
        )
        .exists(),
        "refused apply should keep cached preview"
    );
}

#[test]
fn capture_refuses_inactive_stage() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let refusal = preview_pipeline_capture(&repo_root, &stage_07_request(stage_07_capture_input()))
        .expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.inactive_stage.txt",
    );
}

#[test]
fn capture_apply_refuses_missing_capture_id() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let refusal = apply_pipeline_capture(
        &repo_root,
        "1111111111111111111111111111111111111111111111111111111111111111",
    )
    .expect_err("refusal");
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, None, None),
        &[],
        "capture.refused.missing_capture_id.txt",
    );
}
