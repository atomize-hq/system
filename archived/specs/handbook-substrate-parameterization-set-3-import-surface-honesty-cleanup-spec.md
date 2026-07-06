# Spec: Handbook Substrate Parameterization — Set 3: Import-Surface Default / Validation Honesty Cleanup

## Assumptions I'm Making

1. Set 1 and Set 2 have both reached reviewed stopping points, so Set 3 can now open even though the older status text in `docs/specs/handbook-substrate-parameterization-three-set-map.md` still says Set 3 is blocked on earlier sets.
2. The structural seams from Set 1 and Set 2 are already real enough that this set should **not** reopen their public contract design unless live proof finds a documented contradiction.
3. The intended downstream integration target is still a Substrate-owned namespace such as `.substrate/handbook/**`, so import-target crate wording and fallback behavior must stop implying handbook-product defaults as the only honest layout story.
4. `handbook-cli` and `handbook-compiler` remain out of scope even if they still contain many `.handbook/**` or `core/**` assumptions.
5. Not every remaining `.handbook/**` or `core/**` literal must disappear from the repo; explicit handbook-product default helpers, product-default authoring surfaces, and test fixtures may remain if their boundary is honest and clearly bounded.
6. This set should finish with a final bounded-default inventory that distinguishes removed misleading defaults from intentionally retained handbook-product defaults inside the import-target crates.

## Objective

Create the third implementation set from the three-set map: clean or explicitly bound the remaining import-target-crate defaults, validation text, refusal text, and fallback wording so the final reusable-import story is honest.

The live residual gaps after Set 1 and Set 2 are now narrower and more specific:

1. `handbook-flow` still contains residual handbook-product-root wording / proof helpers that can imply `.handbook/**` even after the public contract-aware path landed.
2. `handbook-pipeline` still contains import-facing validation reasons that derive display text from handbook-product declarative roots instead of the active declarative contract.
3. `handbook-engine` still contains user-visible ingest/default text such as `missing canonical .handbook root ...` plus authoring-core constants that hardcode `.handbook/**` references.
4. The import-target crates still need one final explicit statement of which handbook-product defaults remain intentionally code-owned after the structural sets are done.

This set is about **honesty and boundary cleanup**, not about inventing new layout models, reopening the structural contracts from Set 1 / Set 2, or widening into product-shell cleanup outside the import-target crates.

## Tech Stack

- Rust 2021 workspace
- Primary crates under change:
  - `crates/flow`
  - `crates/pipeline`
  - `crates/engine`
- Existing upstream authorities:
  - `docs/specs/handbook-substrate-parameterization-three-set-map.md`
  - `docs/specs/handbook-substrate-parameterization-gap-map.md`
  - `docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-{spec,plan,tasks}.md`
  - `docs/specs/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-{spec,plan,tasks}.md`
- Primary verification surfaces:
  - `crates/flow/src/resolver.rs`
  - `crates/flow/tests/resolver_core.rs`
  - `crates/pipeline/src/pipeline.rs`
  - `crates/pipeline/tests/pipeline_loader.rs`
  - `crates/pipeline/tests/pipeline_compile.rs`
  - `crates/pipeline/tests/pipeline_route_resolution.rs`
  - `crates/engine/src/canonical_artifacts.rs`
  - `crates/engine/src/canonical_paths.rs`
  - `crates/engine/src/author/*.rs`
  - `crates/engine/tests/author_core.rs`

## Commands

```bash
# Workspace baseline
cargo check --workspace

# Focused flow proof
cargo test -p handbook-flow --test resolver_core
cargo test -p handbook-flow

# Focused pipeline proof
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_route_resolution

# Focused engine proof
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test baseline_validation
cargo test -p handbook-engine --test author_core

# Lint / format wall
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Live residual-default evidence sweep
rg -n "missing canonical \.handbook root|canonical \.handbook root|core/stages/|core/pipelines/|\.handbook/" crates/engine/src crates/pipeline/src crates/flow/src
```

## Project Structure

```text
docs/specs/
  handbook-substrate-parameterization-three-set-map.md                             → set sequencing authority
  handbook-substrate-parameterization-gap-map.md                                   → repo-truth gap inventory
  handbook-substrate-import-adoption-plan.md                                       → downstream import/adoption posture
  handbook-substrate-parameterization-set-1-pipeline-import-layout-*.md            → landed Set 1 authority
  handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-*.md   → landed Set 2 authority
  handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md → this file
  handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-plan.md → set 3 plan
  handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md
                                                                                  → set 3 packets/tasks

crates/flow/src/
  resolver.rs            → residual refusal/blocker/default-root wording and fixture-context surfaces

crates/flow/tests/
  resolver_core.rs       → default-vs-non-default resolver proof surfaces

crates/pipeline/src/
  pipeline.rs            → validation / refusal reason helpers and import-facing path checks
  declarative_roots.rs   → explicit handbook-product default declarative roots
  layout.rs              → explicit handbook-product default storage roots

crates/pipeline/tests/
  pipeline_catalog.rs          → declarative-root behavior and product-default proofs
  pipeline_loader.rs           → loader validation semantics
  pipeline_compile.rs          → compile-stage validation / routing semantics
  pipeline_route_resolution.rs → route/path behavior against active roots

crates/engine/src/
  canonical_artifacts.rs             → ingest errors / default-root display strings
  canonical_paths.rs                 → explicit default canonical layout helper
  author/charter_core.rs             → product-default charter references
  author/environment_inventory_core.rs → product-default environment inventory references

crates/engine/tests/
  canonical_artifacts_ingest.rs → ingest/default-root coverage
  baseline_validation.rs        → loaded canonical-artifact validation coverage
  author_core.rs                → authoring-core canonical-path wording coverage
```

## Code Style

Prefer deriving user-visible reasons from the active contract when the surface is part of the reusable import promise, while keeping explicit handbook-product default helpers clearly named and bounded:

```rust
fn stage_file_outside_directory_reason(stage_root: &str) -> String {
    format!("must live under `{stage_root}/`")
}

pub fn resolve(
    repo_root: impl AsRef<Path>,
    request: ResolveRequest,
) -> Result<ResolverResult, ManifestError> {
    resolve_with_contract(repo_root, request, *default_canonical_layout_contract())
}
```

Code-shape expectations for this set:

- prefer contract-derived wording for import-facing validation/refusal surfaces
- keep handbook-product defaults available only through explicit default helpers or clearly bounded product-default authoring surfaces
- do **not** create a new layout abstraction or reopen the structural public APIs from Set 1 / Set 2
- update only the text, fallback behavior, and boundary notes needed to make the final import-target story honest

## Testing Strategy

This set is implemented as three sequential packets inside one set triplet:

1. **Packet 3.1 — Flow Residual Refusal And Fallback Cleanup**
   - clean or explicitly bound remaining flow surfaces that still imply handbook-product canonical-root truth after the contract-aware path landed
   - keep the explicit default wrapper path intact
   - primary verification:
     - `cargo test -p handbook-flow --test resolver_core`
     - `cargo test -p handbook-flow`

2. **Packet 3.2 — Pipeline Validation / Refusal Wording Cleanup**
   - make import-facing pipeline validation / refusal wording follow the active declarative roots rather than handbook-product displays
   - preserve explicit default helpers and default-path proofs where they remain intentionally bounded
   - primary verification:
     - `cargo test -p handbook-pipeline --test pipeline_catalog`
     - `cargo test -p handbook-pipeline --test pipeline_loader`
     - `cargo test -p handbook-pipeline --test pipeline_compile`
     - `cargo test -p handbook-pipeline --test pipeline_route_resolution`

3. **Packet 3.3 — Engine Residual Default Bounding And Final Proof**
   - clean or explicitly bound remaining engine-side import-target defaults
   - run the full verification wall and record the final bounded-default inventory honestly
   - primary verification:
     - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
     - `cargo test -p handbook-engine --test baseline_validation`
     - `cargo test -p handbook-engine --test author_core`
     - `cargo check --workspace`
     - `cargo fmt --all -- --check`
     - `cargo clippy --workspace --all-targets -- -D warnings`
     - targeted `rg` sweep for remaining residual-default wording

## Boundaries

- **Always:**
  - Keep Set 3 focused on import-target-crate honesty cleanup inside `handbook-flow`, `handbook-pipeline`, and `handbook-engine`.
  - Preserve the distinction between an explicit handbook-product default helper and a reusable import-facing contract.
  - Ground every cleanup decision in landed Set 1 / Set 2 behavior and current repo truth.
  - Record any intentionally retained defaults explicitly instead of pretending every handbook-product literal disappeared.

- **Ask first:**
  - If a cleanup appears to require redesigning the public import-facing contracts from Set 1 or Set 2.
  - If the cleanest engine fix would require parameterizing authoring/product-default surfaces that may actually be intentionally outside the reusable import promise.
  - If a truthful closeout seems to require widening into `handbook-cli`, `handbook-compiler`, or actual Substrate import execution.

- **Never:**
  - Reopen Set 1 or Set 2 structural seams without explicit contradictory proof.
  - Widen into CLI/compiler product-shell cleanup.
  - Execute the actual Substrate import.
  - Treat this set as permission to build a generalized multi-layout plugin system.

## Success Criteria

1. Import-target crate validation/refusal/default wording no longer misrepresents the active layout contract story for the supported reusable-import paths.
2. `handbook-flow` no longer exposes residual refusal/fallback/result surfaces that force non-default callers back onto `.handbook/**` semantics unless those surfaces are explicitly bounded as product-default or test-only.
3. `handbook-pipeline` import-facing validation text no longer hardcodes handbook-product `core/pipelines/` or `core/stages/` roots when the active declarative roots differ.
4. `handbook-engine` import-target default strings either derive from the active contract where required or are explicitly bounded as handbook-product defaults outside the reusable import promise.
5. Any remaining handbook-product defaults inside the import-target crates are explicitly bounded and justified in the final residual-default inventory.
6. The verification wall passes and the Set 3 tasks doc records the final honest boundary without widening into CLI/compiler work.

## Open Questions

1. **Engine authoring-core posture**
   - Default assumption for this set: engine authoring-core constants that still name `.handbook/**` may remain if they are explicitly bounded as handbook-product authoring surfaces rather than reusable import-facing contract surfaces.

2. **Flow execution-demo fixture context**
   - Default assumption for this set: any flow packet-result / fixture-context surface that is visible to downstream import consumers should follow the active contract or be explicitly bounded as test/proof-only behavior.

3. **Boundary-doc refreshes**
   - Default assumption for this set: refresh adjacent boundary docs only if the final bounded-default story for import-target consumers changes in a way the current docs would otherwise misstate.
