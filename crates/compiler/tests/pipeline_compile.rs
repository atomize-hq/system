#[allow(dead_code)]
#[path = "support/pipeline_proof_corpus_support.rs"]
mod pipeline_proof_corpus_support;

use std::fs;
use std::path::Path;

use system_compiler::{
    build_route_basis, compile_pipeline_stage, compile_pipeline_stage_with_runtime,
    load_pipeline_definition, load_route_state_with_supported_variables, persist_route_basis,
    render_pipeline_compile_explain, render_pipeline_compile_payload, resolve_pipeline_route,
    set_route_state, supported_route_state_variables, PipelineCompileRuntimeContext,
    RouteBasisPersistOutcome, RouteStateMutation, RouteStateMutationOutcome, RouteVariables,
};

const PIPELINE_ID: &str = "pipeline.foundation_inputs";
const STAGE_ID: &str = "stage.10_feature_spec";
const FIXED_NOW_UTC: &str = "2026-01-28T18:35:10Z";

fn pipeline_definition(repo_root: &Path) -> system_compiler::PipelineDefinition {
    load_pipeline_definition(repo_root, "pipelines/foundation_inputs.yaml")
        .expect("pipeline fixture")
}

fn supported_variables(
    repo_root: &Path,
) -> (
    system_compiler::PipelineDefinition,
    std::collections::BTreeSet<String>,
) {
    let definition = pipeline_definition(repo_root);
    let supported_variables = supported_route_state_variables(&definition);
    (definition, supported_variables)
}

fn apply_state_mutation(
    repo_root: &Path,
    supported_variables: &std::collections::BTreeSet<String>,
    mutation: RouteStateMutation,
) {
    let state =
        load_route_state_with_supported_variables(repo_root, PIPELINE_ID, supported_variables)
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
            panic!("expected route-state mutation to apply, got {refusal:?}")
        }
    }
}

fn persist_route_basis_for_current_state(repo_root: &Path) {
    let (definition, supported_variables) = supported_variables(repo_root);
    let state =
        load_route_state_with_supported_variables(repo_root, PIPELINE_ID, &supported_variables)
            .expect("load route state");
    let route_variables = RouteVariables::new(state.routing.clone()).expect("route variables");
    let route = resolve_pipeline_route(&definition, &route_variables).expect("resolved route");
    let route_basis =
        build_route_basis(repo_root, &definition, &state, &route).expect("route basis");
    let outcome =
        persist_route_basis(repo_root, PIPELINE_ID, route_basis).expect("persist route basis");
    match outcome {
        RouteBasisPersistOutcome::Applied(_) => {}
        RouteBasisPersistOutcome::Refused(refusal) => {
            panic!("expected route basis persist to apply, got {refusal:?}")
        }
    }
}

fn prepare_compile_ready_repo() -> (tempfile::TempDir, std::path::PathBuf) {
    let (dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: false,
        },
    );
    persist_route_basis_for_current_state(&repo_root);
    (dir, repo_root)
}

fn fixed_runtime() -> PipelineCompileRuntimeContext {
    PipelineCompileRuntimeContext {
        now_utc_override: Some(FIXED_NOW_UTC.to_string()),
    }
}

#[test]
fn compile_success_matches_shared_payload_and_explain_goldens() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile result");
    let payload = render_pipeline_compile_payload(&result);
    let explain = render_pipeline_compile_explain(&result);

    pipeline_proof_corpus_support::assert_matches_golden_with_placeholders(
        &payload,
        &repo_root,
        &[],
        "compile.stage_10_feature_spec.payload.full_context.txt",
    );
    pipeline_proof_corpus_support::assert_matches_golden_with_placeholders(
        &explain,
        &repo_root,
        &[],
        "compile.stage_10_feature_spec.explain.full_context.txt",
    );
}

#[test]
fn compile_refuses_when_route_basis_is_missing() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        system_compiler::PipelineCompileRefusalClassification::MissingRouteBasis
    );
    assert!(err.summary.contains("route_basis"));
    assert!(err.recovery.contains("pipeline resolve"));
}

#[test]
fn compile_refuses_when_required_artifact_is_missing() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    fs::remove_file(repo_root.join("artifacts/base/BASE_CONTEXT.md"))
        .expect("remove required artifact");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        system_compiler::PipelineCompileRefusalClassification::MissingRequiredInput
    );
    assert!(err.summary.contains("artifacts/base/BASE_CONTEXT.md"));
}

#[test]
fn compile_refuses_when_required_variable_is_missing() {
    let (_dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let stage_path = repo_root.join("core/stages/10_feature_spec.md");
    let stage = fs::read_to_string(&stage_path).expect("read stage");
    let updated_stage = stage.replace("    - project_name?\n", "    - project_name\n");
    assert_ne!(
        stage, updated_stage,
        "stage fixture should be updated for the test"
    );
    fs::write(&stage_path, updated_stage).expect("write stage");

    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefCharterRef {
            value: "artifacts/charter/CHARTER.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RefProjectContextRef {
            value: "artifacts/project_context/PROJECT_CONTEXT.md".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: false,
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "charter_gaps_detected".to_string(),
            value: false,
        },
    );
    persist_route_basis_for_current_state(&repo_root);

    let err =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect_err("compile refusal");

    assert_eq!(
        err.classification,
        system_compiler::PipelineCompileRefusalClassification::MissingRequiredInput
    );
    assert!(err
        .summary
        .contains("required compile variable `project_name`"));
}

#[test]
fn compile_succeeds_when_optional_artifacts_are_absent() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    for path in [
        "artifacts/project_context/PROJECT_CONTEXT.md",
        "artifacts/foundation/FOUNDATION_STRATEGY.md",
        "artifacts/foundation/TECH_ARCH_BRIEF.md",
    ] {
        fs::remove_file(repo_root.join(path)).expect("remove optional artifact");
    }

    let result =
        compile_pipeline_stage_with_runtime(&repo_root, PIPELINE_ID, STAGE_ID, &fixed_runtime())
            .expect("compile success");

    assert!(
        result.documents.iter().any(|document| {
            document.path == "artifacts/project_context/PROJECT_CONTEXT.md"
                && document.status
                    == system_compiler::PipelineCompileDocumentStatus::MissingOptional
        }),
        "expected optional project-context artifact to be marked missing"
    );
}

#[test]
fn compile_refuses_when_route_basis_is_stale_after_state_mutation() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RoutingVariable {
            variable: "needs_project_context".to_string(),
            value: true,
        },
    );

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        system_compiler::PipelineCompileRefusalClassification::StaleRouteBasis
    );
    assert!(err.recovery.contains("pipeline resolve"));
}

#[test]
fn compile_refuses_when_persisted_route_basis_marks_stage_inactive() {
    let (dir, repo_root) = pipeline_proof_corpus_support::install_foundation_inputs_repo();
    let (_, supported_variables) = supported_variables(&repo_root);
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunRunner {
            value: "codex-cli".to_string(),
        },
    );
    apply_state_mutation(
        &repo_root,
        &supported_variables,
        RouteStateMutation::RunProfile {
            value: "python-uv".to_string(),
        },
    );
    persist_route_basis_for_current_state(&repo_root);

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        system_compiler::PipelineCompileRefusalClassification::InactiveStage
    );
    assert!(err.summary.contains("not active"));
    drop(dir);
}

#[test]
fn compile_refuses_when_route_basis_is_malformed() {
    let (_dir, repo_root) = prepare_compile_ready_repo();
    let state_path = pipeline_proof_corpus_support::pipeline_state_path(&repo_root);
    let state = fs::read_to_string(&state_path).expect("state file");
    let malformed = state.replace(
        "schema_version: m2-route-basis-v1",
        "schema_version: bad-route-basis",
    );
    fs::write(&state_path, malformed).expect("write malformed state");

    let err =
        compile_pipeline_stage(&repo_root, PIPELINE_ID, STAGE_ID).expect_err("compile refusal");

    assert_eq!(
        err.classification,
        system_compiler::PipelineCompileRefusalClassification::MalformedRouteBasis
    );
    assert!(err.summary.contains("route_basis"));
    assert!(err.recovery.contains("pipeline resolve"));
}
