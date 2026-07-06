# Tasks: Handbook Engine Extraction Phase 1 Slice 3 - Stateful Storage Layout

Plan reference: [handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-plan.md](./handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-plan.md)

## Packet 1.3.1: Route-State And Stage-Provenance Layout Adoption

Packet 1.3.1 is the runtime-state owner adoption for Slice 1.3: extend the compiler-local layout family for runtime-state and capture-provenance ownership, keep separate layout types rather than one global layout object, and move `route_state.rs` plus `stage_10_feature_spec_provenance.rs` behind those owners without changing route-state or provenance semantics.

- [ ] Task: Introduce compiler-local runtime-state and capture-provenance layout owners
  - Acceptance: `crates/compiler/src/layout.rs` defines the `.handbook/state` root, per-pipeline route-state file, stage-capture provenance path, and capture-cache accessors that Slice 1.3 needs, and the boundary stays specific to stateful-storage ownership rather than becoming a monolithic all-layout object.
  - Verify: `cargo test -p handbook-compiler --test pipeline_state_store`
  - Files: `crates/compiler/src/layout.rs`, `crates/compiler/src/lib.rs`

- [ ] Task: Adopt `route_state.rs` and stage-10 capture provenance onto the new layout owners
  - Acceptance: runtime-state root/file derivation and stage-10 capture provenance path ownership flow through the layout family while reset traversal, route-basis persistence, provenance schema, and provenance-match semantics remain unchanged.
  - Verify: `cargo test -p handbook-compiler --test pipeline_state_store && cargo test -p handbook-compiler --test pipeline_route_resolution && cargo check -p handbook-compiler`
  - Files: `crates/compiler/src/route_state.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`, `crates/compiler/tests/pipeline_state_store.rs`, `crates/compiler/tests/pipeline_route_resolution.rs`

## Packet 1.3.2: Capture And Handoff Layout Adoption

Packet 1.3.2 is the capture/handoff storage-owner adoption for Slice 1.3: make `pipeline_capture.rs` and `pipeline_handoff.rs` consume the stateful-storage layout family while leaving supported target ids, consumer posture, and shell wording local for later slices.

- [ ] Task: Adopt `pipeline_capture.rs` onto the capture-provenance layout owner
  - Acceptance: capture-cache path ownership and related write-target planning flow through the capture-provenance layout owner while capture input parsing, rollback, cache-integrity checks, and refusal semantics remain unchanged.
  - Verify: `cargo test -p handbook-compiler --test pipeline_capture`
  - Files: `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/tests/pipeline_capture.rs`

- [ ] Task: Adopt `pipeline_handoff.rs` onto the handoff bundle layout owner and preserve Slice 1.3 boundaries
  - Acceptance: feature-slice bundle root ownership and bundle-relative writes/validation flow through the handoff layout owner; manifest/read-allowlist/trust semantics remain unchanged; no Phase 2 target adoption or Phase 3 shell-wording cleanup is required; and the full Slice 1.3 verification wall passes.
  - Verify: `cargo test -p handbook-compiler --test pipeline_state_store && cargo test -p handbook-compiler --test pipeline_route_resolution && cargo test -p handbook-compiler --test pipeline_capture && cargo test -p handbook-compiler --test pipeline_handoff && cargo check -p handbook-compiler`
  - Files: `crates/compiler/src/layout.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/tests/pipeline_capture.rs`, `crates/compiler/tests/pipeline_handoff.rs`

## Human Review Gate

Do not start Slice 1.3 implementation work until the human has reviewed and approved this Slice 1.3 spec/plan/tasks set.
