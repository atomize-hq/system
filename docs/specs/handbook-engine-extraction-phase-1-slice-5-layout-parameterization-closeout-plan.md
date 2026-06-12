# Plan: Handbook Engine Extraction Phase 1 Slice 5 (Slice 1.5) - Layout Parameterization Closeout

## Objective

Close the remaining Phase 1 root-plan gap by parameterizing reusable layout/storage owners in `handbook-engine`, `handbook-pipeline`, and any remaining reusable callers such that path values are changeable via parameters in principle, while preserving the current handbook product layout as one explicit validated default for this slice.

Spec reference: [handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md](./handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md)

## Major Modules

1. Current-product default layout owner
   - `crates/engine/src/canonical_paths.rs`
   - `crates/pipeline/src/layout.rs`
   - optionally a narrow shared support surface if compile-through reality requires it
   - defines where the typed reusable owners obtain today’s `.handbook/**`, `.handbook/state/**`, and `artifacts/handoff/feature_slice/**` defaults

2. Engine canonical-path adoption
   - `crates/engine/src/canonical_paths.rs`
   - `crates/engine/src/canonical_artifacts.rs`
   - possibly `crates/engine/src/lib.rs`
   - moves canonical path derivation off fixed internal literals and onto a typed contract/default owner

3. Pipeline storage adoption
   - `crates/pipeline/src/layout.rs`
   - `crates/pipeline/src/route_state.rs`
   - `crates/pipeline/src/pipeline_capture.rs`
   - `crates/pipeline/src/stage_10_feature_spec_provenance.rs`
   - `crates/pipeline/src/pipeline_handoff.rs`
   - parameterizes runtime-state, capture, provenance, and handoff path ownership without changing the current product contract

4. Remaining reusable caller adoption
   - `crates/flow/src/resolver.rs`
   - optional narrow compile-through adapters in `crates/compiler/src/**`
   - removes duplicate fallback ownership from reusable callers and keeps any residual product-only literals clearly bounded

5. Verification surfaces
   - `crates/engine/tests/canonical_artifacts_ingest.rs`
   - `crates/engine/tests/freshness_computation.rs`
   - `crates/pipeline/tests/pipeline_state_store.rs`
   - `crates/pipeline/tests/pipeline_capture.rs`
   - `crates/pipeline/tests/pipeline_handoff.rs`
   - `crates/flow/tests/resolver_core.rs`

## Dependencies And Order

### Prerequisite: freeze the parameterization contract and the residual-literal policy first

Why first:

- the slice needs one explicit rule for what counts as “parameterized enough” before code moves begin
- without that rule, work will oscillate between over-generalizing into new config surfaces and under-delivering with more centralized literals
- the closeout must distinguish reusable path ownership from acceptable product-shell, doc, and test literals up front

Output:

- one approved typed-layout posture for reusable engine/pipeline owners
- one approved current-product default layout posture for `.handbook/**`, `.handbook/state/**`, and `artifacts/handoff/feature_slice/**`
- one explicit requirement that the landed typed owners/defaults can produce different paths via parameters in principle, even though this slice keeps today’s defaults unchanged
- one explicit residual-literal classification matrix: reusable-owner forbidden, product-shell/docs/tests allowed when justified

### Packet 1.5.1: Parameterized Canonical Layout Contract

Why first:

- engine canonical path ownership is the narrowest and most foundational remaining parameterization seam
- flow and compiler-facing compatibility surfaces still refer to canonical roots, so engine truth should settle first
- a clean engine canonical contract reduces the risk that later packets keep inventing ad hoc canonical-root fallbacks

Output:

- `handbook-engine` exposes or consumes a typed canonical layout contract/default owner instead of fixed contract literals alone
- canonical artifact loading, manifest generation, and freshness consumers keep current behavior for the default layout
- the current `.handbook/**` product layout becomes an explicit default produced by the new parameterized owner, not an implicit reusable-crate assumption

### Packet 1.5.2: Parameterized Pipeline Storage Layout Adoption

Why second:

- pipeline runtime-state, capture, provenance, and handoff ownership is broader than engine canonical-path ownership and benefits from the same contract vocabulary
- it has the biggest remaining reusable storage surface, so isolating it in its own packet keeps the review wall tractable
- deferring it until after Packet 1.5.1 avoids building two incompatible parameterization patterns

Output:

- `handbook-pipeline` derives runtime-state, capture, provenance, and handoff roots through typed layout owners/defaults
- route-state, capture, provenance, and handoff behavior stays unchanged for the current product default
- those owners/defaults are capable of representing different storage roots in principle
- target ids, consumer ids, and catalog behavior remain untouched

### Packet 1.5.3: Remaining Reusable Caller Adoption And Residual-Literal Closeout

Why third:

- only after the reusable owners are parameterized is it clear which remaining fallback literals are truly redundant versus legitimately product-owned
- keeping caller adoption last prevents `handbook-flow` or compatibility adapters from dictating the reusable-owner design
- the closeout needs one final pass that proves the repo’s residual-literal story is honest

Output:

- `handbook-flow` and any other remaining reusable callers consume the approved typed owners/defaults where appropriate
- duplicate fallback roots are removed or downgraded to explicitly product-only contexts
- compatibility-layer drift is minimized without widening into compiler retirement work

## Risks And Mitigations

### Risk: parameterization accidentally changes the current handbook product layout

Mitigation:

- freeze the current default layout contract before edits
- require the implementation to make that contract changeable via typed parameters/default owners, not merely centralized
- keep all focused test rails asserting today’s relative paths
- treat any path relocation as out of scope unless the user explicitly approves it

### Risk: the slice over-generalizes into user-configurable layout or multi-consumer platform work

Mitigation:

- define parameterization as typed reusable-owner inputs plus one validated default, not free-form config
- reject environment-variable overrides, config files, and consumer-platform abstractions in this slice
- leave target/consumer generalization to Set 2

### Risk: compiler compatibility surfaces pull the slice into Set 3 work

Mitigation:

- treat `crates/compiler/**` as compile-through only
- only change compiler files when they must align with the parameterized owners or avoid truth drift
- defer compiler narrowing/retirement decisions explicitly

### Risk: residual literals become a game of whack-a-mole across source, tests, and docs

Mitigation:

- classify literals by ownership type before editing
- focus the reusable-owner sweep on non-test source files first
- allow product-shell/docs/tests literals when they are no longer the reusable owner of path derivation

### Risk: flow keeps its own fallback root even after reusable owners are fixed

Mitigation:

- make Packet 1.5.3 explicitly responsible for reusable-caller adoption
- use `resolver_core` to preserve current refusal/budget behavior while changing where root truth comes from
- require an explicit justification for any remaining reusable caller fallback literal

## Parallel Vs Sequential

Sequential:

- freeze the contract and residual-literal policy before code edits
- land engine canonical parameterization before pipeline storage parameterization
- land reusable-caller adoption only after the owners it should consume are stable
- run the full workspace wall last

Parallel opportunities after Packet 1.5.2 lands:

- targeted engine and pipeline test rails can run in parallel during final cleanup
- residual-literal source sweeps and flow-adoption cleanup can proceed in parallel once engine/pipeline contracts are stable

## Verification Checkpoints

### Checkpoint 1: reusable owner constants and fallback roots are materially reduced

```bash
rg -n --glob '!**/tests/**' --glob '!**/*test*' "\\.handbook|artifacts/handoff|state/pipeline|feature_slice" \
  crates/engine/src \
  crates/pipeline/src \
  crates/flow/src \
  crates/compiler/src
rg -n "SYSTEM_ROOT_RELATIVE|RUNTIME_STATE_ROOT_RELATIVE|HANDOFF_FEATURE_SLICE_DIR_RELATIVE|HANDBOOK_ROOT_PATH" \
  crates/engine/src \
  crates/pipeline/src \
  crates/flow/src \
  crates/compiler/src
```

### Checkpoint 2: engine canonical behavior is unchanged for the current default

```bash
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test freshness_computation
```

### Checkpoint 3: pipeline storage behavior is unchanged for the current default

```bash
cargo test -p handbook-pipeline --test pipeline_state_store
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
```

### Checkpoint 4: remaining reusable callers preserve current resolver behavior

```bash
cargo test -p handbook-flow --test resolver_core
cargo check --workspace
```

### Final checkpoint

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- reusable engine and pipeline owners are parameterized rather than hardcoded to the current product layout
- the landed owners/defaults make path changes possible in principle without another ownership redesign
- the current handbook product layout still resolves to the same default relative paths
- remaining reusable callers consume the approved owners/defaults or are explicitly justified if they still retain a literal
- any residual fixed literals are truthfully bounded to product-shell, docs, fixtures, or tests
- the focused engine/pipeline/flow verification rails pass
- the slice has not widened into Set 2, Set 3, or Set 4 work
