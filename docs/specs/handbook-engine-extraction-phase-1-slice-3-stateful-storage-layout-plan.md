# Plan: Handbook Engine Extraction Phase 1 Slice 3 - Stateful Storage Layout

## Objective

Extend the compiler-local layout family so `route_state.rs`, `stage_10_feature_spec_provenance.rs`, `pipeline_capture.rs`, and `pipeline_handoff.rs` consume typed runtime-state, capture-provenance, and handoff-bundle storage owners without changing current behavior.

Spec reference: [handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md](./handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md)

## Major Artifacts

1. Runtime-state layout owner surface
   - lives in `crates/compiler/src/layout.rs`
   - owns `.handbook/state` root access and per-pipeline route-state file derivation

2. Capture-provenance layout owner surface
   - lives in the same compiler-internal layout family
   - owns stage-capture provenance and capture-cache repo-relative path derivation under runtime state

3. Handoff-bundle layout owner surface
   - lives in the same compiler-internal layout family
   - owns feature-slice handoff bundle root derivation under `artifacts/handoff/feature_slice/**`

4. Consumer adoptions
   - updates `crates/compiler/src/route_state.rs` and `crates/compiler/src/stage_10_feature_spec_provenance.rs` for Packet 1.3.1
   - updates `crates/compiler/src/pipeline_capture.rs` and `crates/compiler/src/pipeline_handoff.rs` for Packet 1.3.2
   - preserves current validation, serialization, refusal, and write-order behavior inside those modules

5. Regression coverage
   - keeps `pipeline_state_store`, `pipeline_route_resolution`, `pipeline_capture`, and `pipeline_handoff` as the primary safety wall
   - extends or adjusts tests only where the new owner seam needs direct proof

## Dependencies And Order

### Packet 1.3.1 first: Route-State And Stage-Provenance Layout Adoption

Why first:

- `route_state.rs` is the primary current owner of `.handbook/state/**` roots, so the runtime-state owner has to be defined there first
- stage-capture provenance lives under the runtime-state subtree and should consume the same family before capture/handoff adoption begins
- this packet proves the typed storage owners can replace runtime-state literals without pulling capture-cache or handoff bundle logic in immediately

Output:

- one extended `layout.rs` family that defines runtime-state and capture-provenance owner surfaces
- one `route_state.rs` adoption for state-root and per-pipeline state-file ownership
- one `stage_10_feature_spec_provenance.rs` adoption for stage-capture provenance path ownership
- unchanged route-state reset, route-basis, and provenance-match semantics

### Packet 1.3.2 second: Capture And Handoff Layout Adoption

Why second:

- capture should consume the already-landed runtime-state/capture-provenance owners instead of inventing parallel path helpers
- handoff depends on the stage-capture provenance contract and should adopt only after the provenance root is stable
- this packet can stay narrow if bundle-root ownership is routed through a dedicated layout owner instead of mixed into target-contract work

Output:

- one `pipeline_capture.rs` adoption that consumes capture-provenance layout accessors for cache and related write-target planning
- one `pipeline_handoff.rs` adoption that consumes the handoff bundle layout owner for feature-slice bundle roots and bundle-relative writes/validation
- unchanged capture refusal, rollback, manifest, read-allowlist, and trust semantics

## Risks And Mitigations

### Risk: Slice 1.3 drifts into target-contract or shell-wording work

Mitigation:

- keep supported pipeline/stage/consumer ids frozen and local in the existing modules
- reject changes whose primary purpose is CLI/help-text cleanup, next-safe-action wording changes, or target registry work

### Risk: runtime-state adoption widens into canonical or authoring ownership

Mitigation:

- preserve the Slice 1.2 canonical owner as-is
- limit new layout family additions to runtime-state, capture-provenance, and handoff-bundle ownership only
- leave authoring roots/locks for Slice 1.4

### Risk: capture-provenance semantics drift during storage adoption

Mitigation:

- preserve exact repo-relative provenance and cache identities recorded in the Slice 1.1 inventory
- keep `pipeline_capture` and `pipeline_handoff` as required verification rails because both rely on the stored provenance contract

### Risk: handoff bundle ownership overreaches into bundle-content redesign

Mitigation:

- keep the handoff owner narrowly responsible for bundle-root derivation only
- leave manifest contents, read-allowlist policy, trust-matrix content, and consumer posture untouched

### Risk: the layout family becomes a monolithic all-storage object too early

Mitigation:

- preserve the separate layout type-family contract from Slice 1.1
- prefer dedicated owners for runtime state, capture provenance, and handoff bundle roots instead of one mega-struct
- keep the layout module compiler-internal until a later slice proves the outward API we want to freeze

### Risk: tests stay green while Slice 1.3 boundaries drift

Mitigation:

- pair the test wall with explicit boundary review against the spec
- treat any required change to `pipeline.rs`, `pipeline_compile.rs`, `setup.rs`, or `author/**` as out-of-scope leakage unless it is proven compile-through wiring only

## Parallel Vs Sequential

Sequential:

- Packet 1.3.1 before Packet 1.3.2
- runtime-state / capture-provenance owner introduction before capture and handoff adoption
- route-state and stage-provenance verification before capture/handoff packet lands

Not parallel:

- do not split runtime-state owner creation and `route_state.rs` adoption across simultaneous packets
- do not start Phase 2 target-contract work from `pipeline_capture.rs` or `pipeline_handoff.rs`
- do not mix authoring-path adoption or shell-wording cleanup into this slice

## Verification Checkpoints

### Checkpoint 1: Runtime-state and stage-provenance owner adoption complete

Confirm the compiler now has runtime-state / capture-provenance owners and the first consumers use them.

Suggested verification:

```bash
rg -n "RuntimeState|CaptureProvenance|route_state_path|STAGE_10_FEATURE_SPEC_CAPTURE_PROVENANCE_PATH|\.handbook/state" \
  crates/compiler/src/layout.rs \
  crates/compiler/src/route_state.rs \
  crates/compiler/src/stage_10_feature_spec_provenance.rs

cargo test -p handbook-compiler --test pipeline_state_store
cargo test -p handbook-compiler --test pipeline_route_resolution
cargo check -p handbook-compiler
```

### Checkpoint 2: Capture and handoff adoption complete

Confirm `pipeline_capture.rs` and `pipeline_handoff.rs` now consume the extended layout family for storage ownership.

Suggested verification:

```bash
rg -n "CaptureProvenance|HandoffBundle|capture_cache_repo_relative_path|bundle_root|artifacts/handoff/feature_slice" \
  crates/compiler/src/layout.rs \
  crates/compiler/src/pipeline_capture.rs \
  crates/compiler/src/pipeline_handoff.rs

cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
cargo check -p handbook-compiler
```

### Checkpoint 3: Slice boundary remains intact

Confirm the slice stayed within the approved stateful-storage corpus and still compiles cleanly.

Suggested verification:

```bash
cargo test -p handbook-compiler --test pipeline_state_store
cargo test -p handbook-compiler --test pipeline_route_resolution
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
cargo check -p handbook-compiler
```

## Exit Conditions

Slice 1.3 is ready for human review when:

- runtime-state, capture-provenance, and handoff-bundle ownership each have a typed compiler-local owner
- `route_state.rs`, `stage_10_feature_spec_provenance.rs`, `pipeline_capture.rs`, and `pipeline_handoff.rs` all consume those owners
- route-state, capture, provenance, and handoff semantics remain unchanged
- target ids and shell wording still clearly belong to later approved slices
- the targeted test wall passes cleanly

Slice 1.3 is ready for implementation only after the human reviews and accepts the spec/plan/tasks set.
