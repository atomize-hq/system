#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use std::fs;
use std::path::{Path, PathBuf};

use handbook_pipeline::{
    pipeline_capture::{
        capture_pipeline_output, capture_pipeline_output_with_storage_layout,
        PipelineCaptureRequest,
    },
    pipeline_handoff::{
        emit_pipeline_handoff_bundle, emit_pipeline_handoff_bundle_with_storage_layout,
        validate_pipeline_handoff_bundle, validate_pipeline_handoff_bundle_with_storage_layout,
        PipelineHandoffEmitRequest, PipelineHandoffManifest, PipelineHandoffRefusalClassification,
        PipelineHandoffTrustClass, PipelineHandoffValidatedBundle,
        PipelineHandoffValidationFailureClassification,
    },
    route_state::{route_state_path_with_storage_layout, PipelineStorageLayoutContract},
};

const PIPELINE_ID: &str = pipeline_proof_corpus_support::FOUNDATION_INPUTS_PIPELINE_ID;
const STAGE_ID: &str = pipeline_proof_corpus_support::STAGE_10_FEATURE_SPEC_ID;
const CONSUMER_ID: &str = "feature-slice-decomposer";
const STAGE_10_CAPTURE_PROVENANCE_PATH: &str =
    ".handbook/state/pipeline/stage_capture/pipeline.foundation_inputs.stage.10_feature_spec.json";
const CUSTOM_ROUTE_STATE_PATH: &str =
    ".handbook/custom_state/pipeline/pipeline.foundation_inputs.yaml";
const CUSTOM_STAGE_10_CAPTURE_PROVENANCE_PATH: &str =
    ".handbook/custom_state/pipeline/custom_stage_capture/pipeline.foundation_inputs.stage.10_feature_spec.json";
const CUSTOM_HANDOFF_ROOT: &str = "artifacts/custom_handoff/feature_slice";

fn custom_handoff_storage_layout() -> PipelineStorageLayoutContract {
    PipelineStorageLayoutContract::try_from_paths(
        ".handbook/custom_state",
        ".handbook/custom_state/pipeline",
        ".handbook/custom_state/pipeline/custom_stage_capture",
        ".handbook/custom_state/pipeline/custom_capture",
        CUSTOM_HANDOFF_ROOT,
    )
    .expect("custom handoff storage layout should validate")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TestConsumerRefusalClassification {
    UndeclaredRepoReread,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TestConsumerRefusal {
    classification: TestConsumerRefusalClassification,
    summary: String,
}

fn write_file(path: &Path, body: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("mkdirs");
    }
    fs::write(path, body).expect("write file");
}

fn bundle_path(repo_root: &Path, bundle_root: &str, relative_path: &str) -> PathBuf {
    repo_root.join(bundle_root).join(relative_path)
}

fn install_canonical_inputs(repo_root: &Path) {
    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        &pipeline_proof_corpus_support::read_committed_fixture("artifacts/charter/CHARTER.md"),
    );
    write_file(
        &repo_root.join(".handbook/project_context/PROJECT_CONTEXT.md"),
        &pipeline_proof_corpus_support::read_committed_fixture(
            "artifacts/project_context/PROJECT_CONTEXT.md",
        ),
    );
    write_file(
        &repo_root.join(".handbook/feature_spec/FEATURE_SPEC.md"),
        &pipeline_proof_corpus_support::read_committed_model_output("stage_10_feature_spec.md"),
    );
}

fn capture_feature_spec(repo_root: &Path) {
    let request = PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_ID.to_string(),
        input: pipeline_proof_corpus_support::read_committed_model_output(
            "stage_10_feature_spec.md",
        ),
    };
    capture_pipeline_output(repo_root, &request).expect("capture feature spec");
}

fn capture_feature_spec_with_storage_layout(
    repo_root: &Path,
    storage_layout: PipelineStorageLayoutContract,
) {
    let request = PipelineCaptureRequest {
        pipeline_selector: PIPELINE_ID.to_string(),
        stage_selector: STAGE_ID.to_string(),
        input: pipeline_proof_corpus_support::read_committed_model_output(
            "stage_10_feature_spec.md",
        ),
    };
    capture_pipeline_output_with_storage_layout(repo_root, &request, storage_layout)
        .expect("capture feature spec with custom storage layout");
}

fn emit_valid_bundle(
    repo_root: &Path,
) -> (
    String,
    PipelineHandoffValidatedBundle,
    PipelineHandoffManifest,
) {
    let result = emit_pipeline_handoff_bundle(
        repo_root,
        &PipelineHandoffEmitRequest {
            pipeline_selector: PIPELINE_ID.to_string(),
            consumer_selector: CONSUMER_ID.to_string(),
            producer_command: format!(
                "handbook pipeline handoff emit --id {PIPELINE_ID} --consumer {CONSUMER_ID}"
            ),
            producer_version: "test-suite".to_string(),
        },
    )
    .expect("emit handoff bundle");
    let validated =
        validate_pipeline_handoff_bundle(repo_root, &result.bundle_root).expect("validate bundle");
    (result.bundle_root, validated, result.manifest)
}

fn prepare_emitted_bundle_repo() -> (
    tempfile::TempDir,
    PathBuf,
    String,
    PipelineHandoffValidatedBundle,
    PipelineHandoffManifest,
) {
    let (dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    install_canonical_inputs(&repo_root);
    capture_feature_spec(&repo_root);
    let (bundle_root, validated, manifest) = emit_valid_bundle(&repo_root);
    (dir, repo_root, bundle_root, validated, manifest)
}

fn copy_dir_all(source: &Path, dest: &Path) {
    fs::create_dir_all(dest).expect("mkdirs");
    for entry in fs::read_dir(source).expect("read dir") {
        let entry = entry.expect("dir entry");
        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());
        let metadata = entry.metadata().expect("metadata");
        if metadata.is_dir() {
            copy_dir_all(&source_path, &dest_path);
        } else {
            fs::copy(&source_path, &dest_path).unwrap_or_else(|err| {
                panic!(
                    "copy {} -> {}: {err}",
                    source_path.display(),
                    dest_path.display()
                )
            });
        }
    }
}

fn move_route_state_to_custom_layout(
    repo_root: &Path,
    storage_layout: PipelineStorageLayoutContract,
) -> PathBuf {
    let default_state_path = pipeline_proof_corpus_support::pipeline_state_path(repo_root);
    let custom_state_path =
        route_state_path_with_storage_layout(repo_root, PIPELINE_ID, storage_layout)
            .expect("custom route state path");
    fs::create_dir_all(
        custom_state_path
            .parent()
            .expect("custom route state parent"),
    )
    .expect("create custom route state parent");
    fs::rename(&default_state_path, &custom_state_path).expect("move route state to custom layout");
    custom_state_path
}

fn test_consumer_read_bundle_path(
    repo_root: &Path,
    bundle: &PipelineHandoffValidatedBundle,
    requested_path: &str,
) -> Result<String, TestConsumerRefusal> {
    let requested_path = requested_path.trim().trim_start_matches('/');
    if !bundle
        .read_allowlist
        .allow_read_paths
        .iter()
        .any(|allowed| allowed == requested_path)
    {
        return Err(TestConsumerRefusal {
            classification: TestConsumerRefusalClassification::UndeclaredRepoReread,
            summary: format!(
                "repo reread `{requested_path}` is outside bundle `{}`; downstream consumers must stay within the declared allowlist",
                bundle.manifest.bundle_root
            ),
        });
    }

    let full_path = repo_root
        .join(&bundle.manifest.bundle_root)
        .join(requested_path);
    fs::read_to_string(&full_path).map_err(|err| TestConsumerRefusal {
        classification: TestConsumerRefusalClassification::UndeclaredRepoReread,
        summary: format!(
            "declared bundle path `{requested_path}` is unreadable at {}: {err}",
            full_path.display()
        ),
    })
}

#[test]
fn handoff_validation_refuses_stale_canonical_provenance() {
    let (_dir, repo_root, bundle_root, _validated, _manifest) = prepare_emitted_bundle_repo();

    write_file(
        &repo_root.join(".handbook/charter/CHARTER.md"),
        "# tampered canonical charter\n",
    );

    let failure = validate_pipeline_handoff_bundle(&repo_root, &bundle_root)
        .expect_err("stale canonical provenance should refuse");
    assert_eq!(
        failure.classification,
        PipelineHandoffValidationFailureClassification::StaleCanonicalProvenance
    );
    assert!(
        failure.summary.contains("canonical manifest fingerprint")
            || failure.summary.contains("canonical artifact"),
        "{}",
        failure.summary
    );
}

#[test]
fn handoff_emit_refuses_stale_stage_10_feature_spec_capture_provenance() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    install_canonical_inputs(&repo_root);
    capture_feature_spec(&repo_root);

    write_file(
        &repo_root.join("artifacts/foundation/FOUNDATION_STRATEGY.md"),
        "# drifted foundation strategy\n",
    );

    let refusal = emit_pipeline_handoff_bundle(
        &repo_root,
        &PipelineHandoffEmitRequest {
            pipeline_selector: PIPELINE_ID.to_string(),
            consumer_selector: CONSUMER_ID.to_string(),
            producer_command: format!(
                "handbook pipeline handoff emit --id {PIPELINE_ID} --consumer {CONSUMER_ID}"
            ),
            producer_version: "test-suite".to_string(),
        },
    )
    .expect_err("stale stage-10 capture provenance should refuse");
    assert_eq!(
        refusal.classification,
        PipelineHandoffRefusalClassification::InvalidProvenance
    );
    assert!(
        refusal.summary.contains("payload_sha256")
            || refusal.summary.contains("route-basis revision/hash"),
        "{}",
        refusal.summary
    );
}

#[test]
fn handoff_emit_refuses_missing_or_corrupt_stage_10_capture_provenance() {
    for case in ["missing", "corrupt"] {
        let (_dir, repo_root) =
            pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
        install_canonical_inputs(&repo_root);
        capture_feature_spec(&repo_root);

        let provenance_path = repo_root.join(STAGE_10_CAPTURE_PROVENANCE_PATH);
        match case {
            "missing" => {
                fs::remove_file(&provenance_path).expect("remove stage-10 capture provenance");
            }
            "corrupt" => {
                write_file(&provenance_path, "{not-valid-json");
            }
            _ => unreachable!("unexpected case"),
        }

        let refusal = emit_pipeline_handoff_bundle(
            &repo_root,
            &PipelineHandoffEmitRequest {
                pipeline_selector: PIPELINE_ID.to_string(),
                consumer_selector: CONSUMER_ID.to_string(),
                producer_command: format!(
                    "handbook pipeline handoff emit --id {PIPELINE_ID} --consumer {CONSUMER_ID}"
                ),
                producer_version: "test-suite".to_string(),
            },
        )
        .expect_err("missing or corrupt stage-10 provenance should refuse");
        assert_eq!(
            refusal.classification,
            PipelineHandoffRefusalClassification::InvalidProvenance
        );
        assert!(
            refusal.summary.contains("stage-10 capture provenance"),
            "{}",
            refusal.summary
        );
    }
}

#[test]
fn handoff_emit_with_storage_layout_reads_custom_capture_provenance_and_writes_custom_bundle_root()
{
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_stage_10_capture_ready_repo();
    install_canonical_inputs(&repo_root);

    let storage_layout = custom_handoff_storage_layout();
    let custom_state_path = move_route_state_to_custom_layout(&repo_root, storage_layout);
    assert_eq!(
        custom_state_path,
        repo_root.join(CUSTOM_ROUTE_STATE_PATH),
        "route-state should move under the custom runtime-state root"
    );

    capture_feature_spec_with_storage_layout(&repo_root, storage_layout);
    assert!(
        !repo_root.join(STAGE_10_CAPTURE_PROVENANCE_PATH).exists(),
        "default stage-10 provenance path should stay unused for the custom storage layout"
    );
    assert!(
        repo_root
            .join(CUSTOM_STAGE_10_CAPTURE_PROVENANCE_PATH)
            .exists(),
        "custom stage-10 provenance should persist under the custom capture root"
    );

    let result = emit_pipeline_handoff_bundle_with_storage_layout(
        &repo_root,
        &PipelineHandoffEmitRequest {
            pipeline_selector: PIPELINE_ID.to_string(),
            consumer_selector: CONSUMER_ID.to_string(),
            producer_command: format!(
                "handbook pipeline handoff emit --id {PIPELINE_ID} --consumer {CONSUMER_ID}"
            ),
            producer_version: "test-suite".to_string(),
        },
        storage_layout,
    )
    .expect("emit handoff bundle with custom storage layout");

    assert_eq!(result.manifest.bundle_root, result.bundle_root);
    assert!(
        result.bundle_root.starts_with(CUSTOM_HANDOFF_ROOT),
        "custom handoff bundle root should stay under {CUSTOM_HANDOFF_ROOT}, got {}",
        result.bundle_root
    );

    let validated = validate_pipeline_handoff_bundle_with_storage_layout(
        &repo_root,
        &result.bundle_root,
        storage_layout,
    )
    .expect("validate custom-root handoff bundle");
    assert_eq!(validated.manifest.bundle_root, result.bundle_root);
    assert_eq!(validated.read_allowlist.bundle_root, result.bundle_root);
    assert!(
        result
            .written_files
            .iter()
            .all(|path| path.starts_with(&result.bundle_root)),
        "all emitted bundle files should stay within the custom handoff root"
    );
}

#[test]
fn handoff_validation_refuses_tampered_derived_input() {
    let (_dir, repo_root, bundle_root, _validated, manifest) = prepare_emitted_bundle_repo();
    let tampered_input = manifest
        .inputs
        .iter()
        .find(|input| input.trust_class == PipelineHandoffTrustClass::CompilerDerived)
        .expect("compiler-derived handoff input");

    write_file(
        &bundle_path(&repo_root, &bundle_root, &tampered_input.bundle_path),
        "# tampered derived input\n",
    );

    let failure = validate_pipeline_handoff_bundle(&repo_root, &bundle_root)
        .expect_err("tampered derived input should refuse");
    assert_eq!(
        failure.classification,
        PipelineHandoffValidationFailureClassification::TamperedDerivedInput
    );
    assert!(
        failure.summary.contains(&tampered_input.bundle_path),
        "{}",
        failure.summary
    );
    assert!(failure.summary.contains("sha256"), "{}", failure.summary);
}

#[test]
fn handoff_validation_refuses_missing_or_corrupt_provenance() {
    let (_dir, repo_root, bundle_root, _validated, _manifest) = prepare_emitted_bundle_repo();

    fs::remove_file(bundle_path(
        &repo_root,
        &bundle_root,
        "handoff_manifest.json",
    ))
    .expect("remove manifest");

    let failure = validate_pipeline_handoff_bundle(&repo_root, &bundle_root)
        .expect_err("missing handoff manifest should refuse");
    assert_eq!(
        failure.classification,
        PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance
    );
    assert!(
        failure
            .summary
            .contains("handoff manifest is missing or unreadable"),
        "{}",
        failure.summary
    );
}

#[test]
fn handoff_validation_refuses_trust_class_mismatch() {
    let (_dir, repo_root, bundle_root, _validated, _manifest) = prepare_emitted_bundle_repo();
    let manifest_path = bundle_path(&repo_root, &bundle_root, "handoff_manifest.json");
    let mut manifest: PipelineHandoffManifest =
        serde_json::from_str(&fs::read_to_string(&manifest_path).expect("read manifest"))
            .expect("parse manifest");
    let mismatched_input = manifest
        .inputs
        .iter_mut()
        .find(|input| input.source_path.starts_with("core/"))
        .expect("canonical input for trust-class mismatch");
    let source_path = mismatched_input.source_path.clone();
    mismatched_input.trust_class = PipelineHandoffTrustClass::CompilerDerived;
    let rewritten_manifest = serde_json::to_string_pretty(&manifest).expect("serialize manifest");
    write_file(&manifest_path, &rewritten_manifest);

    let failure = validate_pipeline_handoff_bundle(&repo_root, &bundle_root)
        .expect_err("trust-class mismatch should refuse");
    assert_eq!(
        failure.classification,
        PipelineHandoffValidationFailureClassification::TrustClassMismatch
    );
    assert!(
        failure.summary.contains(&source_path),
        "{}",
        failure.summary
    );
    assert!(
        failure
            .summary
            .contains("does not match expected `canonical`"),
        "{}",
        failure.summary
    );
}

#[test]
fn handoff_validation_accepts_same_root_aliases_and_refuses_relocated_copy() {
    let (_dir, repo_root, bundle_root, _validated, _manifest) = prepare_emitted_bundle_repo();

    validate_pipeline_handoff_bundle(&repo_root, &format!("./{bundle_root}"))
        .expect("same-root alias with leading ./ should validate");
    validate_pipeline_handoff_bundle(&repo_root, &format!("{bundle_root}/"))
        .expect("same-root alias with trailing slash should validate");

    let relocated_bundle_root = "artifacts/handoff/feature_slice/copied-root";
    copy_dir_all(
        &repo_root.join(&bundle_root),
        &repo_root.join(relocated_bundle_root),
    );

    let failure = validate_pipeline_handoff_bundle(&repo_root, relocated_bundle_root)
        .expect_err("relocated bundle copy should refuse");
    assert_eq!(
        failure.classification,
        PipelineHandoffValidationFailureClassification::MissingOrCorruptProvenance
    );
    assert!(
        failure
            .summary
            .contains("does not match handoff manifest bundle_root"),
        "{}",
        failure.summary
    );
}

#[test]
fn test_consumer_refuses_undeclared_repo_reread_outside_bundle_allowlist() {
    let (_dir, repo_root, _bundle_root, validated, _manifest) = prepare_emitted_bundle_repo();

    let manifest_body =
        test_consumer_read_bundle_path(&repo_root, &validated, "handoff_manifest.json")
            .expect("declared bundle path should read");
    assert!(
        manifest_body.contains("\"schema_version\""),
        "{manifest_body}"
    );

    let refusal =
        test_consumer_read_bundle_path(&repo_root, &validated, "core/stages/10_feature_spec.md")
            .expect_err("undeclared repo reread should refuse");
    assert_eq!(
        refusal.classification,
        TestConsumerRefusalClassification::UndeclaredRepoReread
    );
    assert!(
        refusal.summary.contains("core/stages/10_feature_spec.md"),
        "{}",
        refusal.summary
    );
    assert!(
        refusal.summary.contains("declared allowlist"),
        "{}",
        refusal.summary
    );
}
