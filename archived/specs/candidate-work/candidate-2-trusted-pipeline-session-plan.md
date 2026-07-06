# Plan: Candidate 2 Trusted Pipeline Session Deepening

## Objective

Implement one deep compiler-owned trust seam so compile, capture, and handoff consume shared route-basis trust instead of repeating trust validation inline.

Spec reference: [candidate-2-trusted-pipeline-session-spec.md](./candidate-2-trusted-pipeline-session-spec.md)

## Major Modules

1. Trust session core
   - likely centered near `crates/compiler/src/route_state.rs`
   - owns state load, canonical route-basis rebuild, freshness checks, stage activity checks, and normalization needed by downstream callers

2. Compile adapter
   - `crates/compiler/src/pipeline_compile.rs`
   - consumes the trust session result and focuses on compile-only concerns: stage metadata, required inputs, payload assembly, explain rendering

3. Capture adapter
   - `crates/compiler/src/pipeline_capture.rs`
   - consumes the trust session result for preview/apply flows and keeps capture-only concerns local: cached plan identity, write validation, state updates, rollback

4. Handoff adapter
   - `crates/compiler/src/pipeline_handoff.rs`
   - consumes the same trust session implementation for bundle validation and provenance checks

5. Export and regression layer
   - `crates/compiler/src/lib.rs`
   - compiler tests and CLI tests that prove public behavior did not drift

## Dependencies And Order

### Phase 1: Define the deep seam

Build the trust-session core first.

Why first:

- every caller currently depends on the same repeated ladder
- without a stable shared result shape, caller migrations will fork again

Output:

- one typed trust result
- one typed trust refusal mapping entrypoint
- one place for route-basis freshness and normalization

### Phase 2: Migrate compile

Compile is the smallest downstream adapter with the clearest existing trust posture.

Why second:

- it already has a bounded target surface
- it proves the seam can support a library-first consumer before touching write paths

Output:

- `pipeline_compile.rs` stops owning route-state reload and route-basis freshness details

### Phase 3: Migrate capture

Capture adds revision checks and apply-time revalidation.

Why third:

- it exercises the deeper seam against preview/apply semantics
- it proves the seam can be reused across both read and write orchestrators

Output:

- `pipeline_capture.rs` keeps capture-specific plan/apply logic but delegates trust setup

### Phase 4: Migrate handoff

Handoff should consume the same trust implementation after compile/capture prove it.

Why fourth:

- it validates that downstream provenance checks can stand on the same trust base
- it closes the last repeated route-basis rebuild path in the compiler

Output:

- `pipeline_handoff.rs` no longer reconstructs trust independently

### Phase 5: Stabilize exports and docs

After all callers migrate:

- decide whether to export the seam publicly now or keep it internal
- refresh docs/spec references if implementation decisions changed

## Risks And Mitigations

### Risk: trust seam becomes a pass-through

If the new module only moves repeated code without shrinking caller knowledge, the refactor stays shallow.

Mitigation:

- require callers to consume one typed trust result
- forbid caller-side route-basis freshness reconstruction after migration

### Risk: capture apply loses revision-conflict nuance

Capture has additional checks beyond plain route-basis freshness.

Mitigation:

- keep revision-conflict handling as an explicit capture-layer concern if it does not belong in the shared trust seam
- test preview/apply mismatch cases directly

### Risk: public refusal wording drifts

Compile/capture/handoff each currently surface specific classifications and recovery actions.

Mitigation:

- keep typed refusal classifications stable
- run CLI regression tests after each caller migration

### Risk: premature public API hardening

The eventual `substrate` library consumer matters, but exporting the wrong interface too early freezes a weak seam.

Mitigation:

- land the deep implementation seam first
- decide public export only after compile/capture/handoff all use it cleanly

## Parallel Vs Sequential

Sequential:

- trust-session core before any caller migration
- compile before capture/handoff
- export decision after all migrations

Potentially parallel after Phase 1:

- capture migration and handoff migration can be split if both consume the same stabilized trust result shape
- CLI regression updates can run alongside compiler test updates

## Verification Checkpoints

### Checkpoint 1: Trust-session core

- `cargo test -p handbook-compiler --test pipeline_state_store`
- targeted compiler tests proving freshness/malformed/inactive trust outcomes through the new seam

### Checkpoint 2: Compile migration

- `cargo test -p handbook-compiler --test pipeline_compile`
- `cargo test -p handbook-cli --test cli_surface`

### Checkpoint 3: Capture migration

- `cargo test -p handbook-compiler --test pipeline_capture`
- `cargo test -p handbook-cli --test cli_surface`

### Checkpoint 4: Handoff migration

- `cargo test -p handbook-compiler --test pipeline_handoff`
- `cargo test -p handbook-cli --test pipeline_handoff_refusals`

### Final checkpoint

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace`
- `cargo check --workspace`

## Exit Conditions

The plan is complete when:

- one deep trust seam owns route-basis trust for compile/capture/handoff
- each caller reads as an adapter over that seam rather than a second trust owner
- tests prove no public reduced-v1 behavior drift
- the codebase is in a better position for direct library consumption from `substrate`
