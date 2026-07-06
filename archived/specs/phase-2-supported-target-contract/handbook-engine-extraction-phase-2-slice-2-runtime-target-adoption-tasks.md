# Tasks: Handbook Engine Extraction Phase 2 Slice 2 - Runtime Target Adoption

Plan reference: [handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md](./handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md)

## Prerequisite: Slice 2.1 Authority

Phase 2 Slice 1 already froze the supported-target contract for this work: `SupportedPipelineTarget`, `SupportedStageTarget`, `SupportedConsumerTarget`, `SupportedTargetRegistry`, declarative pipeline/stage truth, code-owned validated default consumers, the approved allowed pairings, the non-owner posture of `route_state.rs` and CLI help, and the deferment of Slice 2.3 template/library resolver work.

- Slice 2.2 must adopt that authority in runtime code without redefining it.

## Packet 2.2.1: Compile Target Adoption

- [ ] Task: Implement the runtime supported-target owner for the approved Slice 2.1 wedge
  - Acceptance: Live compiler code exposes one runtime owner for `SupportedPipelineTarget`, `SupportedStageTarget`, `SupportedConsumerTarget`, and the approved pairings, and that owner preserves declarative pipeline/stage truth plus code-owned validated default consumers.
  - Verify: `cargo test -p handbook-compiler --test pipeline_compile`
  - Files: `crates/compiler/src/pipeline.rs`, `crates/compiler/src/pipeline_compile.rs`, optionally `crates/compiler/tests/pipeline_compile.rs`

- [ ] Task: Migrate compile target validation onto the runtime owner while preserving compile posture
  - Acceptance: `pipeline_compile.rs` no longer acts as the supported-target owner for the approved compile target, and compile-specific refusal classifications, summaries, and recovery guidance remain stable after adoption.
  - Verify: `cargo test -p handbook-compiler --test pipeline_compile && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/tests/pipeline_compile.rs`, `crates/cli/tests/cli_surface.rs`

## Packet 2.2.2: Capture And Provenance Target Adoption

- [ ] Task: Migrate capture supported-target validation and supported-stage rendering onto the runtime owner
  - Acceptance: `pipeline_capture.rs` consumes the runtime supported-target owner for the approved capture wedge instead of owning the authoritative supported pipeline/stage set locally, while capture-specific state, input-shape, and apply/preview behavior remain local.
  - Verify: `cargo test -p handbook-compiler --test pipeline_capture && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline.rs`, `crates/compiler/tests/pipeline_capture.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Migrate stage-10 provenance target validation onto the same runtime owner
  - Acceptance: `stage_10_feature_spec_provenance.rs` validates supported targets through the shared runtime contract rather than a private stage-10 hardcoded validator, while provenance schema, path, and hash guarantees stay unchanged.
  - Verify: `cargo test -p handbook-compiler --test pipeline_capture`
  - Files: `crates/compiler/src/stage_10_feature_spec_provenance.rs`, `crates/compiler/src/pipeline.rs`, `crates/compiler/tests/pipeline_capture.rs`

## Packet 2.2.3: Handoff Target Adoption

- [ ] Task: Migrate handoff consumer and manifest target validation onto the runtime owner
  - Acceptance: `pipeline_handoff.rs` stops owning the authoritative supported pipeline/consumer/stage target checks and instead consumes the shared runtime contract for handoff emit and manifest validation.
  - Verify: `cargo test -p handbook-compiler --test pipeline_handoff && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/pipeline.rs`, `crates/compiler/tests/pipeline_handoff.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Preserve handoff-specific provenance, trust, and write-failure posture after adoption
  - Acceptance: Handoff provenance matching, trust-class enforcement, bundle layout, and write-failure behavior remain local and regression-covered after the supported-target owner changes.
  - Verify: `cargo test -p handbook-compiler --test pipeline_handoff && cargo test -p handbook-cli --test help_drift_guard`
  - Files: `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/tests/pipeline_handoff.rs`, `crates/cli/tests/help_drift_guard.rs`

## Final Slice Verification

- [ ] Task: Run the full slice verification wall after all three packets land
  - Acceptance: The runtime supported-target owner is the only approved owner across compile, capture, provenance, and handoff, the supported wedge remains unchanged, and the full workspace checks pass.
  - Verify: `cargo check --workspace && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 2.2 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
