#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use std::fs;
use std::path::Path;

use sha2::{Digest, Sha256};
use system_compiler::{
    apply_pipeline_capture, capture_pipeline_output, load_pipeline_capture_cache_entry,
    load_route_state_with_supported_variables, preview_pipeline_capture,
    render_pipeline_capture_apply_result, render_pipeline_capture_preview,
    render_pipeline_capture_refusal, set_route_state, PipelineCaptureCacheEntry,
    PipelineCapturePlan, PipelineCaptureRefusalClassification, PipelineCaptureRequest,
    PipelineCaptureStateUpdate, PipelineCaptureStateValue, RouteState, RouteStateMutation,
    RouteStateMutationOutcome,
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

fn capture_next_safe_action(rendered: &str) -> &str {
    rendered
        .lines()
        .find_map(|line| line.strip_prefix("NEXT SAFE ACTION: "))
        .expect("rendered capture output should include next safe action")
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

fn load_route_state(repo_root: &Path) -> RouteState {
    let (_, supported_variables) =
        pipeline_proof_corpus_support::load_foundation_inputs_definition(repo_root);
    load_route_state_with_supported_variables(repo_root, PIPELINE_ID, &supported_variables)
        .expect("load route state")
}

fn assert_no_capture_cache_entries(repo_root: &Path) {
    let capture_dir = repo_root
        .join(".system")
        .join("state")
        .join("pipeline")
        .join("capture");
    if !capture_dir.exists() {
        return;
    }
    assert!(
        fs::read_dir(&capture_dir)
            .expect("capture cache dir")
            .next()
            .is_none(),
        "refused capture should not leave cached preview entries"
    );
}

fn recompute_capture_id(plan: &PipelineCapturePlan) -> String {
    let mut identity_plan = plan.clone();
    identity_plan.capture_id.clear();
    let serialized = serde_yaml_bw::to_string(&identity_plan).expect("serialize capture plan");
    let mut hasher = Sha256::new();
    hasher.update(serialized.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn rewrite_tampered_capture_cache(
    repo_root: &Path,
    original_capture_id: &str,
    mutate: impl FnOnce(&mut PipelineCaptureCacheEntry),
) -> String {
    let mut cache_entry =
        load_pipeline_capture_cache_entry(repo_root, original_capture_id).expect("load cache");
    mutate(&mut cache_entry);
    cache_entry.plan.capture_id = recompute_capture_id(&cache_entry.plan);
    cache_entry.capture_id = cache_entry.plan.capture_id.clone();

    let original_path =
        pipeline_proof_corpus_support::pipeline_capture_cache_path(repo_root, original_capture_id);
    let next_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        repo_root,
        &cache_entry.capture_id,
    );
    let serialized = serde_yaml_bw::to_string(&cache_entry).expect("serialize tampered cache");
    if next_path != original_path {
        fs::remove_file(&original_path).expect("remove original cache file");
    }
    fs::write(&next_path, serialized).expect("write tampered cache file");
    cache_entry.capture_id
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
    let next_safe_action = capture_next_safe_action(&rendered);

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
    assert!(
        next_safe_action.contains(
            "system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>"
        ),
        "stage-05 apply should tell the operator to set needs_project_context"
    );
    assert!(
        next_safe_action.contains("system pipeline resolve --id pipeline.foundation_inputs"),
        "stage-05 apply should tell the operator to refresh route truth"
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
fn capture_preview_refuses_empty_single_file_body_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let refusal = preview_pipeline_capture(&repo_root, &stage_05_request("\n".to_string()))
        .expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_05_ID)),
        &[],
        "capture.refused.empty_single_file_body.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_apply_refuses_empty_single_file_body_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let refusal = capture_pipeline_output(&repo_root, &stage_05_request("\r\n".to_string()))
        .expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "single-file capture stages must receive a non-empty body"
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
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
fn capture_preview_refuses_empty_declared_block_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact");
    let initial_repo_mirror =
        fs::read_to_string(repo_root.join("ENVIRONMENT_INVENTORY.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);
    let mut input = stage_07_capture_input();
    let target_block = format!(
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\n{}",
        pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/foundation/ENVIRONMENT_INVENTORY.md"
        )
    );
    input = input.replace(
        &target_block,
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\n",
    );

    let refusal =
        preview_pipeline_capture(&repo_root, &stage_07_request(input)).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_explicit_placeholders(
        &render_pipeline_capture_refusal(&refusal, Some(PIPELINE_ID), Some(STAGE_07_ID)),
        &[],
        "capture.refused.empty_declared_block.txt",
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("ENVIRONMENT_INVENTORY.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[test]
fn capture_apply_refuses_empty_declared_block_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_07_capture_ready_repo();
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact");
    let initial_repo_mirror =
        fs::read_to_string(repo_root.join("ENVIRONMENT_INVENTORY.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);
    let mut input = stage_07_capture_input();
    let target_block = format!(
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\n{}",
        pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/foundation/ENVIRONMENT_INVENTORY.md"
        )
    );
    input = input.replace(
        &target_block,
        "--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---\r\n",
    );

    let refusal =
        capture_pipeline_output(&repo_root, &stage_07_request(input)).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::InvalidCaptureInput
    );
    assert_eq!(
        refusal.summary,
        "declared artifact block `artifacts/foundation/ENVIRONMENT_INVENTORY.md` must contain a non-empty body"
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/foundation/ENVIRONMENT_INVENTORY.md"))
            .expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("ENVIRONMENT_INVENTORY.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
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

#[cfg(unix)]
#[test]
fn capture_apply_refuses_symlinked_cache_entry_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let cache_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id,
    );
    let target_path = repo_root.join("CHARTER.md");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(&target_path).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    fs::remove_file(&cache_path).expect("remove cached preview file");
    std::os::unix::fs::symlink(&target_path, &cache_path).expect("replace cache with symlink");

    let refusal =
        apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert!(
        refusal.summary.contains("capture cache path"),
        "expected cache-path summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(&target_path).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        fs::symlink_metadata(&cache_path)
            .expect("cache metadata")
            .file_type()
            .is_symlink(),
        "refused apply should leave the symlinked cache entry in place"
    );
}

#[cfg(unix)]
#[test]
fn capture_preview_refuses_cache_path_when_system_root_is_symlinked() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let external_root = tempfile::tempdir().expect("external tempdir");
    let system_root = repo_root.join(".system");
    let redirected_system_root = external_root.path().join("redirected-system");
    let initial_state = load_route_state(&repo_root);

    fs::rename(&system_root, &redirected_system_root).expect("move .system");
    std::os::unix::fs::symlink(&redirected_system_root, &system_root)
        .expect("replace .system with symlink");

    let refusal = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect_err("preview refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::CacheFailure
    );
    assert!(
        refusal.summary.contains("capture cache path"),
        "expected cache-path summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[cfg(unix)]
#[test]
fn capture_preview_refuses_cache_path_when_capture_parent_is_symlinked() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let external_root = tempfile::tempdir().expect("external tempdir");
    let capture_dir = repo_root
        .join(".system")
        .join("state")
        .join("pipeline")
        .join("capture");
    let redirected_capture_dir = external_root.path().join("redirected-capture");
    let initial_state = load_route_state(&repo_root);

    std::os::unix::fs::symlink(&redirected_capture_dir, &capture_dir).expect("symlink capture dir");

    let refusal = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect_err("preview refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::CacheFailure
    );
    assert!(
        refusal.summary.contains("capture cache path"),
        "expected cache-path summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert_no_capture_cache_entries(&repo_root);
}

#[cfg(unix)]
#[test]
fn capture_apply_refuses_symlinked_cache_parent_chain_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let external_root = tempfile::tempdir().expect("external tempdir");
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let cache_dir = repo_root
        .join(".system")
        .join("state")
        .join("pipeline")
        .join("capture");
    let redirected_capture_dir = external_root.path().join("redirected-capture");
    let cache_path = pipeline_proof_corpus_support::pipeline_capture_cache_path(
        &repo_root,
        &preview.plan.capture_id,
    );
    let target_path = repo_root.join("CHARTER.md");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(&target_path).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    fs::rename(&cache_dir, &redirected_capture_dir).expect("move capture dir");
    std::os::unix::fs::symlink(&redirected_capture_dir, &cache_dir)
        .expect("replace capture dir with symlink");

    let refusal =
        apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert!(
        refusal.summary.contains("capture cache path"),
        "expected cache-path summary, got: {}",
        refusal.summary
    );
    assert!(
        refusal
            .summary
            .contains("cannot be written through symlink"),
        "expected symlink refusal, got: {}",
        refusal.summary
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(&target_path).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        fs::symlink_metadata(&cache_dir)
            .expect("cache dir metadata")
            .file_type()
            .is_symlink(),
        "refused apply should leave the symlinked cache directory in place"
    );
    assert!(
        cache_path
            .parent()
            .expect("cache parent")
            .ends_with("capture"),
        "test should keep the redirected cache file under the capture directory"
    );
}

#[test]
fn capture_apply_stage_05_guidance_warns_about_resolve_before_follow_up_capture() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let result = capture_pipeline_output(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);
    let next_safe_action = capture_next_safe_action(&rendered);

    assert!(
        next_safe_action.contains(
            "system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>"
        ),
        "stage-05 apply guidance should include the manual route variable step"
    );
    assert!(
        next_safe_action.contains("system pipeline resolve --id pipeline.foundation_inputs"),
        "stage-05 apply guidance should include route refresh before follow-up work"
    );

    apply_mutation(
        &repo_root,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );

    let refusal = preview_pipeline_capture(&repo_root, &stage_07_request(stage_07_capture_input()))
        .expect_err("stale route basis refusal");
    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::StaleRouteBasis
    );
    assert!(
        refusal
            .recovery
            .contains("system pipeline resolve --id pipeline.foundation_inputs"),
        "follow-up capture without resolve should still require route refresh"
    );
}

#[test]
fn capture_apply_stage_05_guidance_survives_preexisting_manual_route_value() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    apply_mutation(
        &repo_root,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);

    let result = capture_pipeline_output(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("capture");
    let rendered = render_pipeline_capture_apply_result(&result);
    let next_safe_action = capture_next_safe_action(&rendered);

    assert!(
        next_safe_action.contains(
            "system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>"
        ),
        "stage-05 apply guidance should still include the manual route variable step after re-capture"
    );
    assert!(
        next_safe_action.contains("system pipeline resolve --id pipeline.foundation_inputs"),
        "stage-05 apply guidance should still include route refresh after re-capture"
    );
}

#[test]
fn capture_preview_and_cached_apply_stage_05_guidance_survive_preexisting_manual_route_value() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    apply_mutation(
        &repo_root,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );
    let _ = pipeline_proof_corpus_support::persist_foundation_inputs_route_basis(&repo_root);

    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    assert_eq!(
        preview.plan.post_apply_next_safe_action.as_deref(),
        Some(
            "run `system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>`, then run `system pipeline resolve --id pipeline.foundation_inputs` before the next compile or capture"
        )
    );

    let result = apply_pipeline_capture(&repo_root, &preview.plan.capture_id).expect("apply");
    let rendered = render_pipeline_capture_apply_result(&result);
    let next_safe_action = capture_next_safe_action(&rendered);

    assert!(
        next_safe_action.contains(
            "system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>"
        ),
        "cached stage-05 apply guidance should still include the manual route variable step after re-capture"
    );
    assert!(
        next_safe_action.contains("system pipeline resolve --id pipeline.foundation_inputs"),
        "cached stage-05 apply guidance should still include route refresh after re-capture"
    );
}

#[test]
fn capture_apply_refuses_tampered_artifact_path_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let tampered_capture_id =
        rewrite_tampered_capture_cache(&repo_root, &preview.plan.capture_id, |cache_entry| {
            cache_entry.plan.artifact_writes[0].path = "docs/EVIL.md".to_string();
        });

    let refusal = apply_pipeline_capture(&repo_root, &tampered_capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &tampered_capture_id
        )
        .exists(),
        "refused apply should keep the tampered cache entry for inspection"
    );
}

#[test]
fn capture_apply_refuses_tampered_repo_mirror_writes_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let tampered_capture_id =
        rewrite_tampered_capture_cache(&repo_root, &preview.plan.capture_id, |cache_entry| {
            cache_entry.plan.repo_mirror_writes[0].path = "README.md".to_string();
        });

    let refusal = apply_pipeline_capture(&repo_root, &tampered_capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &tampered_capture_id
        )
        .exists(),
        "refused apply should keep the tampered cache entry for inspection"
    );
}

#[test]
fn capture_apply_refuses_tampered_state_updates_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let tampered_capture_id =
        rewrite_tampered_capture_cache(&repo_root, &preview.plan.capture_id, |cache_entry| {
            cache_entry
                .plan
                .state_updates
                .push(PipelineCaptureStateUpdate {
                    field_path: "refs.project_context_ref".to_string(),
                    value: PipelineCaptureStateValue::String(
                        "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
                    ),
                });
        });

    let refusal = apply_pipeline_capture(&repo_root, &tampered_capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::TamperedCaptureCache
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &tampered_capture_id
        )
        .exists(),
        "refused apply should keep the tampered cache entry for inspection"
    );
}

#[test]
fn capture_apply_refuses_unsupported_stage_id_from_tampered_cache_without_side_effects() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_05_capture_ready_repo();
    let preview = preview_pipeline_capture(&repo_root, &stage_05_request(stage_05_capture_input()))
        .expect("preview");
    let initial_artifact =
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact");
    let initial_repo_mirror = fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror");
    let initial_state = load_route_state(&repo_root);

    let tampered_capture_id =
        rewrite_tampered_capture_cache(&repo_root, &preview.plan.capture_id, |cache_entry| {
            cache_entry.plan.target.stage_id = "stage.99_not_supported".to_string();
        });

    let refusal = apply_pipeline_capture(&repo_root, &tampered_capture_id).expect_err("refusal");

    assert_eq!(
        refusal.classification,
        PipelineCaptureRefusalClassification::UnsupportedTarget
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("artifacts/charter/CHARTER.md")).expect("artifact"),
        initial_artifact
    );
    assert_eq!(
        fs::read_to_string(repo_root.join("CHARTER.md")).expect("mirror"),
        initial_repo_mirror
    );
    assert_eq!(load_route_state(&repo_root), initial_state);
    assert!(
        pipeline_proof_corpus_support::pipeline_capture_cache_path(
            &repo_root,
            &tampered_capture_id
        )
        .exists(),
        "refused apply should keep the tampered cache entry for inspection"
    );
}
