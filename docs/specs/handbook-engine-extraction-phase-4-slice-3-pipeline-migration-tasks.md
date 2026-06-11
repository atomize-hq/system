# Tasks: Handbook Engine Extraction Phase 4 Slice 3 (Slice 4.3) - Pipeline Crate Migration

Plan reference: [handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md](./handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md)

## Prerequisite: pipeline ownership must become real, not compile-through theater

Slice 4.3 is the first true `handbook-pipeline` ownership landing. It must make `handbook-pipeline` the implementation home for approved runtime pipeline logic while keeping `handbook-compiler` only as a temporary compatibility facade and avoiding direct CLI rewires.

- Slice 4.3 must not widen into Phase 4.4 flow migration or Phase 4.5 caller rewiring.

## Packet 4.3.1: Pipeline Loading Route And Route-State Migration

- [ ] Task: Flip the pipeline/compiler dependency posture to support real pipeline ownership
  - Acceptance: `handbook-pipeline` no longer depends on `handbook-compiler` as the real owner for Slice 4.3 surfaces, the resulting package graph is acyclic, and the narrow supporting-infra rule is implemented well enough for the pipeline crate to own its migrated modules directly.
  - Verify: `cargo tree -p handbook-pipeline -e normal && cargo tree -p handbook-compiler -e normal && cargo check --workspace`
  - Files: `crates/pipeline/Cargo.toml`, `crates/compiler/Cargo.toml`, optionally `Cargo.toml`, `crates/pipeline/src/lib.rs`, `crates/compiler/src/lib.rs`, and any narrow supporting-infra files needed for the ownership flip

- [ ] Task: Move pipeline loading, catalog, selection, and supported-target registry into `handbook-pipeline`
  - Acceptance: the `pipeline.rs` implementation family has one real owner under `crates/pipeline/src/**`, and any remaining compiler surfaces are explicit compatibility re-exports or thin adapters rather than duplicate implementation bodies.
  - Verify: `rg -n 'load_pipeline_catalog|load_pipeline_definition|load_pipeline_selection_metadata|SupportedTargetRegistry' crates/pipeline crates/compiler/src && cargo test -p handbook-pipeline && cargo test -p handbook-compiler --test pipeline_loader && cargo test -p handbook-compiler --test pipeline_catalog`
  - Files: `crates/pipeline/src/lib.rs`, `crates/pipeline/src/pipeline.rs`, any required supporting helper files, `crates/compiler/src/lib.rs`, `crates/compiler/src/pipeline.rs`, `crates/compiler/tests/pipeline_loader.rs`, `crates/compiler/tests/pipeline_catalog.rs`

- [ ] Task: Move route evaluation, route-state persistence, trusted-session validation, and reset helpers into `handbook-pipeline`
  - Acceptance: `pipeline_route.rs` and `route_state.rs` live under `crates/pipeline/src/**`, trusted-session and route-basis helpers are pipeline-owned, and any setup-facing reset helpers delegate to pipeline-owned logic instead of leaving compiler as the implementation owner.
  - Verify: `rg -n 'resolve_pipeline_route|load_route_state|persist_route_basis|load_trusted_pipeline_session|plan_runtime_state_reset|apply_runtime_state_reset' crates/pipeline crates/compiler/src && cargo test -p handbook-pipeline && cargo test -p handbook-compiler --test pipeline_route_resolution && cargo test -p handbook-compiler --test pipeline_state_store`
  - Files: `crates/pipeline/src/pipeline_route.rs`, `crates/pipeline/src/route_state.rs`, any required supporting helper files, `crates/compiler/src/pipeline_route.rs`, `crates/compiler/src/route_state.rs`, optionally `crates/compiler/src/setup.rs`, `crates/compiler/tests/pipeline_route_resolution.rs`, `crates/compiler/tests/pipeline_state_store.rs`

- [ ] Task: Move or recreate regression coverage for the migrated foundation inside the pipeline crate
  - Acceptance: `handbook-pipeline` has package-local regression coverage for loader, catalog, route evaluation, and route-state behavior, and the pipeline package can prove those surfaces without depending on compiler-owned tests as the primary contract.
  - Verify: `cargo test -p handbook-pipeline`
  - Files: `crates/pipeline/tests/**`, optionally the corresponding `crates/compiler/tests/*.rs` transition guards

## Packet 4.3.2: Compile Capture And Handoff Migration

- [ ] Task: Move compile mechanics into `handbook-pipeline`
  - Acceptance: `compile_pipeline_stage` and its related compile types/functions have one real implementation owner under `crates/pipeline/src/**`, and any remaining compiler surfaces are thin compatibility exports rather than duplicate compile logic.
  - Verify: `rg -n 'compile_pipeline_stage|render_pipeline_compile_payload|PipelineCompileResult' crates/pipeline crates/compiler/src && cargo test -p handbook-pipeline && cargo test -p handbook-compiler --test pipeline_compile`
  - Files: `crates/pipeline/src/pipeline_compile.rs`, `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/src/lib.rs`, `crates/pipeline/tests/**`, optionally `crates/compiler/tests/pipeline_compile.rs`

- [ ] Task: Move capture and handoff mechanics into `handbook-pipeline`
  - Acceptance: capture preview/apply behavior and handoff emit/validate behavior live under `crates/pipeline/src/**`, while `handbook-compiler` retains only compatibility re-exports, thin adapters, or local shell glue strictly needed to preserve current callers.
  - Verify: `rg -n 'capture_pipeline_output|preview_pipeline_capture|apply_pipeline_capture|emit_pipeline_handoff_bundle|validate_pipeline_handoff_bundle' crates/pipeline crates/compiler/src && cargo test -p handbook-pipeline && cargo test -p handbook-compiler --test pipeline_capture && cargo test -p handbook-compiler --test pipeline_handoff && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/pipeline_handoff.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/lib.rs`, `crates/pipeline/tests/**`, optionally `crates/compiler/tests/pipeline_capture.rs`, `crates/compiler/tests/pipeline_handoff.rs`, `crates/cli/tests/cli_surface.rs`

## Packet 4.3.3: Setup Helper And Provenance Alignment

- [ ] Task: Move stage-10 provenance generation and validation into `handbook-pipeline`
  - Acceptance: `stage_10_feature_spec_provenance` lives under `crates/pipeline/src/**`, and compile/capture/handoff callers rely on the pipeline-owned provenance helpers instead of compiler-owned implementation bodies.
  - Verify: `rg -n 'stage_10_feature_spec|route_basis_fingerprint_sha256|build_stage_10_feature_spec_capture_provenance' crates/pipeline crates/compiler/src && cargo test -p handbook-pipeline && cargo test -p handbook-compiler --test pipeline_capture && cargo test -p handbook-compiler --test pipeline_handoff`
  - Files: `crates/pipeline/src/stage_10_feature_spec_provenance.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`, `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/pipeline_handoff.rs`, related tests under `crates/pipeline/tests/**`, optionally `crates/compiler/tests/pipeline_capture.rs`, `crates/compiler/tests/pipeline_handoff.rs`

- [ ] Task: Move the approved reusable setup-helper seam into `handbook-pipeline` while keeping product-facing setup behavior in compiler
  - Acceptance: pipeline-safe reusable setup helpers (such as runtime-state reset planning/execution or equivalent narrow helpers) are owned by `handbook-pipeline`, while `crates/compiler/src/setup.rs` retains only product-facing request/disposition/refusal behavior and any non-pipeline starter-template logic left outside this slice.
  - Verify: `rg -n 'plan_runtime_state_reset|apply_runtime_state_reset|reset_state|SetupPlan|SetupOutcome' crates/pipeline crates/compiler/src/setup.rs crates/compiler/src/route_state.rs && cargo test -p handbook-pipeline && cargo test -p handbook-compiler --test setup && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/pipeline/src/setup.rs` or equivalent helper module, `crates/pipeline/src/route_state.rs`, `crates/compiler/src/setup.rs`, optionally `crates/compiler/src/setup_shell.rs`, `crates/pipeline/tests/**`, `crates/compiler/tests/setup.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Prove the final pipeline ownership seam with package-local coverage and public CLI guards
  - Acceptance: `handbook-pipeline` has package-local regression coverage for the final provenance/setup-helper seam, compiler compatibility remains thin, and public CLI pipeline behavior remains stable.
  - Verify: `cargo test -p handbook-pipeline && cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
  - Files: `crates/pipeline/tests/**`, optionally `crates/cli/tests/cli_surface.rs`, `crates/cli/tests/help_drift_guard.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 4.3 verification wall after all three packets land
  - Acceptance: `handbook-pipeline` is the real implementation owner for the approved Slice 4.3 surfaces, the dependency graph is acyclic, pipeline-owned tests pass, CLI behavior stays stable, and no adjacent-slice leakage appears.
  - Verify: `cargo check --workspace && cargo test -p handbook-pipeline && cargo test -p handbook-cli --test cli_surface`
  - Files: verification only

## Human Review Gate

Stop after the Slice 4.3 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
