# Spec: Handbook Engine Extraction Phase 1 Slice 5 (Slice 1.5) - Layout Parameterization Closeout

## Assumptions

1. Slice 1.5 is a **remediation closeout** slice: Slices 1.1 through 1.4 already centralized ownership seams, but they did **not** fully satisfy the root-plan requirement that reusable internals be parameterized instead of carrying handbook-product defaults as fixed contract literals.
2. The current product-owned on-disk contract remains the same during this slice unless the user explicitly changes scope: canonical truth stays under `.handbook/**`, runtime state stays under `.handbook/state/**`, and feature-slice handoff bundles stay under `artifacts/handoff/feature_slice/**`.
3. For this slice, **parameterized** is a hard requirement: reusable crates must consume typed layout definitions or validated default owners such that the underlying paths are changeable via parameters in principle, rather than being embedded as fixed product literals in reusable internals.
4. This slice does **not** need to exercise a path change. It needs to land the new parameterized ownership so the current product layout is produced by the newly landed parameters/default owners, while the effective paths remain unchanged in this slice.
5. Fixed path strings may remain in CLI/product-shell copy, docs, tests, and explicitly product-owned edge adapters when they are describing or asserting the current contract. They should not remain as the active source of truth for reusable path derivation inside `handbook-engine`, `handbook-pipeline`, or other reusable internals.
6. `crates/compiler/**` may be touched only when needed for compile-through adapters or to prevent drift with the new parameterized owners. Final compiler narrowing/retirement remains Set 3 / Slice 4.5 refresh work.
7. Orchestration-target parameterization, direct-caller rewires, and CLI shell closeout remain separate closeout sets and must not be widened into Slice 1.5.

## Objective

Finish the remaining Phase 1 root-plan gap by replacing fixed layout/storage assumptions inside reusable internals with typed parameterized layout ownership, with explicit support for path changes via parameters in principle, while preserving the current handbook product layout as one explicit validated default for this slice.

The user is the maintainer of the handbook workspace and the reviewer of the extraction closeout. The immediate outcome is not a new product layout. The immediate outcome is a truthful closeout slice that makes the current crate split structurally reusable:

- `handbook-engine` stops treating the current canonical `.handbook/**` tree as an unchangeable internal constant contract
- `handbook-pipeline` stops treating `.handbook/state/**` and `artifacts/handoff/**` roots as unchangeable internal constant contracts
- remaining reusable callers stop inventing their own fallback layout roots when approved owners already exist
- one code-owned parameter/default posture continues to describe today’s handbook product layout without changing behavior in this slice

Success means the repo can honestly claim that reusable internals are parameterized, that the path contract is changeable through typed parameters/default owners in principle, and that the current handbook product still uses the same effective storage layout in this slice.

## Slice Scope

In scope:

- define the approved parameterized layout closeout boundary for the current product layout
- introduce or refine typed default layout owners so reusable crates consume validated layout inputs instead of fixed product literals
- make it explicit in code/design that those typed owners can produce different paths via parameters, even though this slice keeps the current defaults unchanged
- parameterize canonical artifact path ownership in `handbook-engine`
- parameterize runtime-state, capture-provenance, and handoff-bundle path ownership in `handbook-pipeline`
- adopt any remaining reusable-internal callers that still assume fixed `.handbook/**`, `.handbook/state/**`, or `artifacts/**` roots
- explicitly classify which residual path literals are acceptable after this slice and which are not

Out of scope:

- changing the current repo-relative handbook product contract
- adding free-form user-configurable layout files, environment-variable overrides, or a generic consumer framework
- orchestration-target parameterization and de-hardcoding of `feature-slice-decomposer`
- compiler retirement/narrowing beyond narrow compile-through adjustments
- CLI wording/help cleanup, prompt cleanup, or operator-facing copy rewrites
- moving artifacts just to make the tree look cleaner

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- existing Phase 1 slice triplets:
  - `handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-*`
  - `handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-*`
  - `handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-*`
  - `handbook-engine-extraction-phase-1-slice-4-authoring-layout-*`
- live reusable-layout owners and callers:
  - `crates/engine/src/canonical_paths.rs`
  - `crates/engine/src/canonical_artifacts.rs`
  - `crates/pipeline/src/layout.rs`
  - `crates/pipeline/src/route_state.rs`
  - `crates/pipeline/src/pipeline_capture.rs`
  - `crates/pipeline/src/stage_10_feature_spec_provenance.rs`
  - `crates/pipeline/src/pipeline_handoff.rs`
  - `crates/flow/src/resolver.rs`
- adjacent compatibility surfaces that may need compile-through alignment only:
  - `crates/compiler/src/layout.rs`
  - `crates/compiler/src/setup.rs`
  - `crates/compiler/src/author/*_shell.rs`

## Tech Stack

- Rust 2021 workspace
- `handbook-engine` crate for canonical truth and validation
- `handbook-pipeline` crate for runtime-state, capture, handoff, and route-state behavior
- `handbook-flow` crate for higher-level packet-selection and resolver behavior
- current product-owned default layout:
  - canonical truth under `.handbook/**`
  - runtime state under `.handbook/state/**`
  - handoff bundles under `artifacts/handoff/feature_slice/**`

## Commands

Live evidence sweep for reusable fixed-path ownership:

```bash
rg -n --glob '!**/tests/**' --glob '!**/*test*' "\\.handbook|artifacts/handoff|state/pipeline|feature_slice" \
  crates/engine/src \
  crates/pipeline/src \
  crates/flow/src \
  crates/compiler/src
```

Targeted owner/constant sweep:

```bash
rg -n "SYSTEM_ROOT_RELATIVE|RUNTIME_STATE_ROOT_RELATIVE|HANDOFF_FEATURE_SLICE_DIR_RELATIVE|HANDBOOK_ROOT_PATH" \
  crates/engine/src \
  crates/pipeline/src \
  crates/flow/src \
  crates/compiler/src
```

Focused verification rails:

```bash
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test freshness_computation
cargo test -p handbook-pipeline --test pipeline_state_store
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-flow --test resolver_core
```

Compile-through rail:

```bash
cargo check --workspace
```

Final verification wall:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Project Structure

```text
HANDBOOK_ENGINE_EXTRACTION_PLAN.md                                                       -> Root phase-order authority
docs/specs/handbook-engine-extraction-slice-map.md                                       -> Phase -> Slice -> Packet authority
docs/specs/handbook-engine-extraction-closeout-four-set-map.md                           -> Closeout-set authority for Slice 1.5
docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-*.md
                                                                                         -> Slice 1.5 authority set
crates/engine/src/canonical_paths.rs                                                     -> Canonical-path owner that still hardcodes the current product layout
crates/engine/src/canonical_artifacts.rs                                                 -> Canonical artifact loading and manifest-facing canonical path consumption
crates/pipeline/src/layout.rs                                                            -> Runtime-state, capture, and handoff path owners that still hardcode the current product layout
crates/pipeline/src/route_state.rs                                                       -> Runtime-state consumer
crates/pipeline/src/pipeline_capture.rs                                                  -> Capture-cache and provenance consumer
crates/pipeline/src/stage_10_feature_spec_provenance.rs                                  -> Stage-capture provenance consumer
crates/pipeline/src/pipeline_handoff.rs                                                  -> Feature-slice handoff-bundle consumer
crates/flow/src/resolver.rs                                                              -> Higher-level reusable caller that still carries `.handbook` fallback assumptions
crates/compiler/src/layout.rs                                                            -> Compatibility/product-owned layout duplication; touch only if needed for compile-through truth
crates/cli/src/**                                                                        -> Product shell and operator copy; not the primary owner for this slice
```

## Code Style

Prefer typed layout definitions plus one validated current-product default over fixed product literals embedded directly in reusable crates.

```rust
let layout = HandbookProductLayout::current_default();
let canonical = layout.canonical();
let pipeline = layout.pipeline();

let artifacts = CanonicalArtifacts::load_with_layout(repo_root, canonical)?;
let route_state = pipeline.runtime_state().route_state_relative_path(pipeline_id);
let handoff_root = pipeline.handoff().feature_slice_bundle_root(feature_id);
```

Conventions for this slice:

- reusable crates consume typed layout values or owners rather than reading raw product literals as their own source of truth
- one explicit validated default owner represents today’s `.handbook/**` / `.handbook/state/**` / `artifacts/handoff/**` posture
- that owner contract must be capable of representing different path values through parameters in principle, even though this slice keeps the current defaults unchanged
- do not thread raw strings through reusable call graphs when a typed layout owner can carry the contract
- literal path strings may remain in tests, docs, and product-shell copy when they are asserting or describing the current contract
- do not introduce free-form end-user layout configuration or a broad new platform surface in this slice

## Testing Strategy

Primary verification for this slice is **behavior-preserving reusable-layout parameterization**, not a storage-contract redesign.

Test levels:

- source sweeps proving reusable owners stopped hardcoding the current layout contract as active internal truth
- targeted engine regression coverage in `crates/engine/tests/canonical_artifacts_ingest.rs`
- targeted engine regression coverage in `crates/engine/tests/freshness_computation.rs`
- targeted pipeline regression coverage in `crates/pipeline/tests/pipeline_state_store.rs`
- targeted pipeline regression coverage in `crates/pipeline/tests/pipeline_capture.rs`
- targeted pipeline regression coverage in `crates/pipeline/tests/pipeline_handoff.rs`
- targeted flow regression coverage in `crates/flow/tests/resolver_core.rs`
- full workspace compile/test wall at the end

Coverage expectations:

- canonical artifact identities and manifest behavior remain unchanged
- runtime-state, capture, provenance, and handoff relative locations remain unchanged for the current product default
- resolver refusal/budget behavior remains unchanged for the current product default
- reusable crates no longer need fixed path literals as their active contract owner
- the landed typed owners/defaults make path changes possible in principle without further architectural redesign
- residual literals that remain are explicitly classed as product-shell, docs, or tests

## Boundaries

- Always:
  - keep Slice 1.5 about reusable layout/storage parameterization closeout
  - preserve the current product-owned default layout unless the user explicitly changes scope
  - distinguish reusable internal owners from product-shell copy, docs, fixtures, and tests
  - name any acceptable residual fixed literals explicitly
- Ask first:
  - changing the repo-relative product layout contract
  - introducing a new user-facing config surface for layout overrides
  - adding a new shared crate solely for layout ownership
  - widening into orchestration-target work, compiler retirement, or CLI wording cleanup
- Never:
  - redesign the product layout model in this slice
  - introduce free-form consumer/layout configuration
  - hide remaining fixed product literals inside reusable crate internals
  - use Slice 1.5 as cover for Set 2, Set 3, or Set 4 work

## Success Criteria

- `handbook-engine` no longer depends on fixed `.handbook/**` literals as its reusable internal contract owner for canonical path derivation.
- `handbook-pipeline` no longer depends on fixed `.handbook/state/**` or `artifacts/handoff/**` literals as its reusable internal contract owner for runtime-state, capture, provenance, and handoff path derivation.
- any remaining reusable-internal callers consume approved typed layout owners/defaults instead of inventing their own fallback root literals.
- the landed parameterized owners/defaults make path changes possible in principle without another ownership redesign.
- the current handbook product layout still resolves to the same effective relative paths after the slice lands.
- acceptable remaining literals are bounded to product-shell copy, tests, fixtures, or docs and are explicitly justified.
- `cargo test -p handbook-engine --test canonical_artifacts_ingest` passes.
- `cargo test -p handbook-engine --test freshness_computation` passes.
- `cargo test -p handbook-pipeline --test pipeline_state_store` passes.
- `cargo test -p handbook-pipeline --test pipeline_capture` passes.
- `cargo test -p handbook-pipeline --test pipeline_handoff` passes.
- `cargo test -p handbook-flow --test resolver_core` passes.
- `cargo test --workspace` passes.
- the slice stays bounded to three implementation packets:
  - `Packet 1.5.1: Parameterized Canonical Layout Contract`
  - `Packet 1.5.2: Parameterized Pipeline Storage Layout Adoption`
  - `Packet 1.5.3: Remaining Reusable Caller Adoption And Residual-Literal Closeout`

## Open Questions

- Should the current-product default layout owner live as a narrow shared support surface reused by `handbook-engine` and `handbook-pipeline`, or should each crate accept typed layout inputs while the product default stays composed one layer up?
- Should `crates/flow/src/resolver.rs` consume engine-owned canonical layout accessors directly for its fallback/reporting behavior, or should it receive already-derived canonical paths from a thinner adapter?
- If `crates/compiler/src/layout.rs` duplicates current-product defaults after reusable owners are parameterized, should Slice 1.5 deduplicate that definition minimally for drift prevention, or leave the duplication intact until Set 3 narrows compiler ownership?

## Packet Breakdown

### Packet 1.5.1: Parameterized Canonical Layout Contract

Goal:

- replace fixed canonical-path ownership in reusable engine internals with typed parameterized layout ownership while preserving the current `.handbook/**` default for this slice

Required outcome:

- `handbook-engine` has a reviewable typed canonical layout contract or owner
- current canonical artifact identities remain unchanged for the current product default
- the contract can represent different canonical path values through parameters in principle
- the active canonical path contract is no longer owned by fixed engine literals alone

### Packet 1.5.2: Parameterized Pipeline Storage Layout Adoption

Goal:

- replace fixed runtime-state, capture, provenance, and handoff layout ownership in reusable pipeline internals with typed parameterized layout ownership while preserving the current product default for this slice

Required outcome:

- `handbook-pipeline` derives `.handbook/state/**` and `artifacts/handoff/feature_slice/**` through typed layout owners/defaults
- runtime-state, capture, provenance, and handoff behavior remains unchanged for the current product default
- the typed layout owners/defaults can represent different storage roots in principle
- target-parameterization work stays deferred

### Packet 1.5.3: Remaining Reusable Caller Adoption And Residual-Literal Closeout

Goal:

- move remaining reusable callers onto the approved parameterized owners/defaults and bound any residual fixed literals to explicitly acceptable product-shell, docs, or test contexts

Required outcome:

- remaining reusable callers such as `handbook-flow` stop owning separate fallback root literals where reusable owners already exist
- any compatibility-layer duplication is either removed or explicitly justified
- the repo has a truthful residual-literal story for the current product default
