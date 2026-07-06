# Plan: Handbook Substrate Parameterization — Set 3: Import-Surface Default / Validation Honesty Cleanup

Spec reference: [handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md](./handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md)

Upstream authority carried forward from Set 1 and Set 2:
- [handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md)
- [handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md)
- [handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md)
- [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-spec.md)
- [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-plan.md)
- [handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md](./handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md)

## Objective

Translate the final three-set seam into a bounded packet plan that cleans or explicitly bounds the remaining import-target default / validation / refusal wording so the reusable-import story matches the actual landed contracts from Set 1 and Set 2.

## Overview

Set 3 is the residual honesty-cleanup set from the three-set map. The structural layout seams are already landed:

```text
Packet 3.1 (flow residual cleanup)
  -> Packet 3.2 (pipeline validation/refusal cleanup)
  -> Packet 3.3 (engine residual bounding + final proof)
```

This set should stay smaller than Set 1 and Set 2. It is not a contract-invention set; it is a boundary-honesty set that must derive from live repo truth rather than guessing ahead.

## Current State (live repo truth)

1. Set 1 is closed with a reviewed proof wall and a recorded residual-default inventory in `docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md`.
2. Set 2 is closed with a reviewed proof wall and a recorded residual-default inventory in `docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md`.
3. `crates/pipeline/src/pipeline.rs` still contains validation-reason helpers whose display text is derived from `handbook_product_declarative_roots()` rather than the active declarative contract:
   - `configured_pipeline_root_display()`
   - `pipeline_yaml_root_reason()`
   - `pipeline_yaml_extension_reason()`
4. `crates/engine/src/canonical_artifacts.rs` still renders user-visible ingest errors such as:
   - `missing canonical .handbook root ...`
   - `canonical .handbook root is not a directory ...`
   - `canonical .handbook root must not be a symlink ...`
5. `crates/engine/src/author/environment_inventory_core.rs` and `crates/engine/src/author/charter_core.rs` still contain fixed `.handbook/**` references that may be either:
   - product-default authoring truths to keep explicitly bounded
   - or residual misleading defaults to clean
6. `crates/flow/src/resolver.rs` still contains residual `.handbook/**`-shaped helper text / fixture surfaces even though the supported import-facing path is now contract-aware.
7. CLI/compiler surfaces remain broader and out of scope. Set 3 should finish the import-target crate story only.

## Planned Seam Shape

The default planned shape for Set 3 is:

1. Clean import-facing wording or fallback behavior when it still contradicts the active layout contract.
2. Preserve explicit handbook-product default helpers where they are intentionally product-default behavior.
3. Explicitly bound any remaining import-target defaults that are still code-owned after the cleanup.
4. Stop once the final import-target story is honest; do not spill into CLI/compiler product-shell cleanup.

## Components

### 1. Flow residual refusal / fallback cleanup

Clean or explicitly bound the remaining flow surfaces that still imply handbook-product canonical-root truth.

This component includes:
- refusal / blocker / summary text that still names `.handbook` when the active contract differs
- result / proof surfaces such as fixture-context basis paths if they are part of the downstream-visible packet story
- preserving the explicit default wrapper path in `resolve(...)`

This component does **not** include:
- redesigning the Set 2 public API
- general flow rendering cleanup outside the import-target contract story

### 2. Pipeline validation / refusal wording cleanup

Make pipeline validation reasons and refusal text match the active declarative roots.

This component includes:
- import-facing pipeline YAML root / extension reasons
- stage-path validation wording that must name the active stage root
- focused tests proving non-default declarative roots produce honest reasons

This component does **not** include:
- reopening the structural stage-root or storage-layout seams from Set 1
- removing explicit default-helper constants that are still valid as handbook-product defaults

### 3. Engine residual default bounding

Decide which remaining engine-side `.handbook/**` references must become contract-derived and which should remain explicitly bounded as handbook-product defaults.

This component includes:
- ingest/default-root user-visible text in `canonical_artifacts.rs`
- authoring-core constants / validation expectations in `author/*.rs`
- any final notes or boundary-doc refresh needed so the import-target contract story stays honest

This component does **not** include:
- widening into CLI/compiler authoring/rendering surfaces
- blanket removal of every `.handbook/**` literal from engine tests or product-default helpers

### 4. Final proof and residual-default inventory

After the cleanup packets land:
- run the full verification wall
- inspect the remaining residual-default references across the import-target crates
- distinguish acceptable bounded defaults from misleading defaults that should have been removed
- record the final closeout notes without turning Packet 3.3 into an implementation sink

## Packet Plan

## Packet 3.1 — Flow Residual Refusal And Fallback Cleanup

### Goal

Make the remaining flow-facing default-root wording and fallback/result surfaces honest relative to the active canonical layout contract.

### Work

- inspect the remaining `.handbook/**` flow hits after Set 2
- clean or explicitly bound refusal/blocker/summary text that still misstates the active contract
- clean or explicitly bound any packet-result / fixture-context surfaces that still imply handbook-product canonical-root truth
- preserve the explicit default wrapper path for handbook-product callers
- keep broader wording sweeps deferred if they do not affect the import-facing contract story

### Verification checkpoint

```bash
git status --short --branch
sed -n '430,760p' crates/flow/src/resolver.rs
sed -n '1,260p' crates/flow/tests/resolver_core.rs
cargo test -p handbook-flow --test resolver_core
cargo test -p handbook-flow
rg -n "missing canonical \.handbook root|canonical \.handbook root|\.handbook/" crates/flow/src crates/flow/tests
```

### Exit condition

The remaining flow-facing contract story is honest for both default and non-default canonical-root callers, without reopening the Set 2 public API design.

## Packet 3.2 — Pipeline Validation / Refusal Wording Cleanup

### Goal

Make pipeline validation / refusal wording follow the active declarative roots instead of handbook-product root displays.

### Work

- replace handbook-product-derived import-facing validation reasons with active-contract-derived wording where required
- preserve explicit handbook-product default helpers as explicit defaults rather than hidden assumptions
- update only the tests whose assertions are inseparable from the cleaned wording
- avoid reopening the structural storage/declarative seams from Set 1

### Verification checkpoint

```bash
git status --short --branch
sed -n '2920,3015p' crates/pipeline/src/pipeline.rs
sed -n '1,240p' crates/pipeline/tests/pipeline_loader.rs
sed -n '1,240p' crates/pipeline/tests/pipeline_compile.rs
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_route_resolution
rg -n "core/stages/|core/pipelines/|must live under|pipeline YAML must" crates/pipeline/src/pipeline.rs crates/pipeline/tests
```

### Exit condition

Import-facing pipeline validation / refusal wording is contract-honest for active declarative roots, and any remaining `core/**` literals are clearly bounded as default-helper or fixture behavior.

## Packet 3.3 — Engine Residual Default Bounding And Final Proof

### Goal

Close the remaining engine-side default-story gap, then run the final Set 3 proof wall and record the bounded-default inventory honestly.

### Work

- inspect engine ingest/default-root wording and authoring-core canonical-path references
- clean contract-facing misleading defaults or explicitly bound product-default authoring surfaces
- refresh adjacent boundary docs only if the final import-target story would otherwise be misstated
- run the full Set 3 verification wall
- record pass/fail and final residual-default inventory in the Set 3 tasks doc

### Verification checkpoint

```bash
git status --short --branch
sed -n '340,380p' crates/engine/src/canonical_artifacts.rs
sed -n '1,120p' crates/engine/src/author/environment_inventory_core.rs
sed -n '1,80p' crates/engine/src/author/charter_core.rs
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test baseline_validation
cargo test -p handbook-engine --test author_core
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
rg -n "missing canonical \.handbook root|canonical \.handbook root|core/stages/|core/pipelines/|\.handbook/" crates/engine/src crates/pipeline/src crates/flow/src
```

### Exit condition

The set is reviewable as a complete import-target honesty-cleanup story, and the tasks doc clearly separates acceptable retained product defaults from misleading defaults that should not survive the reusable import promise.

## Implementation Order

1. **Packet 3.1 first** because flow still has import-facing contract surfaces from Set 2 and should be made honest before the final closeout narrative hardens.
2. **Packet 3.2 second** because pipeline validation/refusal wording should follow the already-landed Set 1 contracts before the final residual-default inventory is written.
3. **Packet 3.3 last** because engine default bounding depends on the upstream flow/pipeline honesty story already being real, and the final proof should reflect the whole import-target picture.

## Verification Checkpoints And Stop Boundaries

- After **Packet 3.1**, stop if a truthful fix requires redesigning `handbook-flow`'s public canonical-layout injection surface rather than cleaning residual wording/fallback behavior.
- After **Packet 3.2**, stop if the remaining pipeline wording problem reveals a structural Set 1 gap instead of residual honesty cleanup.
- During **Packet 3.3**, stop if the cleanest engine answer requires widening into CLI/compiler product-shell cleanup or broader authoring-surface redesign outside the import-target contract story.

## Risks And Mitigations

### Risk 1: Set 3 accidentally reopens structural work from Set 1 or Set 2

- **Why it matters:** the three-set map depends on Set 3 being a residual honesty-cleanup seam, not a hidden structural reimplementation.
- **Mitigation:** limit changes to wording, fallback behavior, result surfaces, and explicit default bounding unless live contradictory proof forces an earlier-set reopen.

### Risk 2: over-cleaning removes valid handbook-product defaults

- **Why it matters:** explicit default helpers and product-default authoring surfaces are allowed to remain if they are honestly bounded.
- **Mitigation:** preserve clearly named default helpers and document the retained defaults explicitly in the final residual inventory.

### Risk 3: under-cleaning leaves the final import-target story misleading

- **Why it matters:** if import-target crates still tell downstream users to use `.handbook/**` or `core/**` as active truth, the structural sets are not honestly consumable.
- **Mitigation:** use focused rg sweeps plus source inspection to distinguish import-facing misleading text from test-only or product-default references.

### Risk 4: Packet 3.3 becomes a catch-all sink

- **Why it matters:** proof packets that silently absorb unfinished cleanup make the authority docs untrustworthy.
- **Mitigation:** keep explicit packet exits and reopen/ask-first boundaries when proof reveals work outside the intended residual seam.

## Verification Wall

Use this as the set-level proof wall for Packet 3.3:

```bash
cargo test -p handbook-flow --test resolver_core
cargo test -p handbook-flow
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_route_resolution
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test baseline_validation
cargo test -p handbook-engine --test author_core
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
rg -n "missing canonical \.handbook root|canonical \.handbook root|core/stages/|core/pipelines/|\.handbook/" crates/engine/src crates/pipeline/src crates/flow/src
```

## Stop Boundary

Stop after Packet 3.3 for this set. Do not:
- reopen Set 1 or Set 2 without explicit contradictory proof
- widen into CLI/compiler cleanup
- execute actual Substrate import work
- automatically open a further family after Set 3 closes; return to live repo truth first
