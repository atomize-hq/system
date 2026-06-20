# Spec: Handbook Substrate Parameterization — Set 1: Pipeline Import Layout

## Assumptions I'm Making

1. This is the **first active set** from `docs/specs/handbook-substrate-parameterization-three-set-map.md`, so this triplet should stand alone and should not pre-author Set 2 or Set 3 work.
2. The target outcome is a supported import-facing layout seam for `handbook-pipeline`, not a redesign of handbook's product-default layout model.
3. The desired downstream layout is still a Substrate-owned namespace such as:
   - `.substrate/handbook/core/pipelines/**`
   - `.substrate/handbook/core/profiles/**`
   - `.substrate/handbook/core/runners/**`
   - `.substrate/handbook/core/stages/**`
   - `.substrate/handbook/state/pipeline/**`
   - `.substrate/handbook/artifacts/handoff/feature_slice/**`
4. `handbook-cli` and `handbook-compiler` remain out of scope for this set even if they still contain handbook-product literals.
5. The bounded default consumer id (`feature-slice-decomposer`) remains code-owned and is **not** generalized in this set.
6. Handbook-product defaults may remain available through explicit default helpers, but downstream importers must no longer need crate-private access or repo-level `core/**` mirroring to use `handbook-pipeline` honestly.

## Objective

Create the first implementation set required by the three-set map: make `handbook-pipeline` support a real import-facing layout contract for Substrate-owned handbook placement.

This set closes the biggest remaining structural import gap by addressing three closely related seams inside `handbook-pipeline`:

1. **Declarative roots** are currently fixed to repo-level `core/pipelines`, `core/profiles`, and `core/runners`.
2. **Stage-root ownership** is currently fixed to repo-level `core/stages/**` across supported-target assumptions, discovery, and validation.
3. **Pipeline storage layout** already exists as a typed internal contract, but it is not yet available through a supported public/import-facing seam.

The goal is **not** to eliminate every handbook-product default literal everywhere in the crate. The goal is to establish the honest reusable import-facing boundary needed for `.substrate/handbook/**`, while explicitly bounding any remaining handbook-product defaults that stay as product-default behavior. For Packet 1.2 specifically, removing a literal alone is insufficient: discovery and validation behavior must adopt the active contract as runtime truth.

## Tech Stack

- Rust 2021 workspace
- Crate under change: `crates/pipeline`
- Adjacent dependency boundary:
  - `handbook-engine` (only handbook intra-workspace dependency)
- Key dependencies in `crates/pipeline/Cargo.toml`:
  - `serde = "1"`
  - `serde_json = "1"`
  - `serde_yaml_bw = "2.5.4"`
  - `sha2 = "0.10"`
  - `time = "0.3"`
  - `libc = "0.2"`
- Verification surfaces:
  - `crates/pipeline/tests/pipeline_catalog.rs`
  - `crates/pipeline/tests/pipeline_loader.rs`
  - `crates/pipeline/tests/pipeline_compile.rs`
  - `crates/pipeline/tests/pipeline_route_resolution.rs`
  - `crates/pipeline/tests/pipeline_state_store.rs`
  - `crates/pipeline/tests/pipeline_capture.rs`
  - `crates/pipeline/tests/pipeline_handoff.rs`

## Commands

```bash
# Workspace baseline
cargo check --workspace

# Focused handbook-pipeline tests for declarative roots + stage-root behavior
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_route_resolution

# Focused handbook-pipeline tests for storage layout behavior
cargo test -p handbook-pipeline --test pipeline_state_store
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff

# Full crate run when needed
cargo test -p handbook-pipeline

# Lint / format wall
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings

# Live evidence sweep for remaining fixed roots
rg -n "core/pipelines|core/profiles|core/runners|core/stages|\.handbook/state|artifacts/handoff/feature_slice" crates/pipeline/src crates/pipeline/tests
```

## Project Structure

```text
docs/specs/
  handbook-substrate-parameterization-three-set-map.md                         → set sequencing authority
  handbook-substrate-parameterization-gap-map.md                               → repo-truth gap inventory
  handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md     → this file
  handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md     → set 1 plan
  handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md    → set 1 packets/tasks
  handbook-substrate-parameterization-set-1-pipeline-import-layout-packet-prompts.md
                                                                               → one orchestration prompt per packet

crates/pipeline/src/
  lib.rs                → public re-exports and crate-boundary ownership
  declarative_roots.rs  → current repo-level declarative root assumptions
  pipeline.rs           → catalog loading, discovery, validation, supported-target registry
  layout.rs             → typed storage-layout contract (currently internal)
  route_state.rs        → runtime state persistence behavior
  pipeline_capture.rs   → capture storage / provenance behavior
  pipeline_handoff.rs   → handoff bundle storage behavior

crates/pipeline/tests/
  pipeline_catalog.rs         → catalog/discovery assertions
  pipeline_loader.rs          → pipeline/stage validation behavior
  pipeline_compile.rs         → compile-stage path behavior
  pipeline_route_resolution.rs→ route resolution against declared pipeline/stage truth
  pipeline_state_store.rs     → route-state persistence behavior
  pipeline_capture.rs         → stage-capture persistence behavior
  pipeline_handoff.rs         → handoff bundle behavior
```

## Code Style

Maintain the crate's current explicit typed-contract style and default-helper posture:

```rust
pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}

pub fn pipeline_root() -> &'static Path {
    Path::new(PIPELINES_ROOT)
}

pub(crate) const HANDBOOK_PRODUCT_PIPELINE_STORAGE_LAYOUT: PipelineStorageLayoutContract =
    PipelineStorageLayoutContract::new(
        RuntimeStateLayoutContract::new(".handbook/state", ".handbook/state/pipeline"),
        CaptureStorageLayoutContract::new(
            ".handbook/state/pipeline/stage_capture",
            ".handbook/state/pipeline/capture",
        ),
        HandoffBundleLayoutContract::new("artifacts/handoff/feature_slice"),
    );
```

Code-shape expectations for this set:

- prefer explicit typed contracts over scattered path literals
- keep handbook-product defaults available through clearly named default helpers rather than hidden fallback behavior
- preserve narrow crate-boundary exports in `lib.rs`
- do not introduce speculative abstraction for future product-shell concerns outside this set

## Testing Strategy

This set is implemented as four sequential packets inside one set triplet:

1. **Packet 1.1 — Declarative Root Contract And Owner Boundary**
   - Introduce the public/import-facing declarative root contract.
   - Prove catalog/loader entry points can derive roots from that contract without changing default behavior.
   - Primary verification:
     - `cargo test -p handbook-pipeline --test pipeline_catalog`
     - `cargo test -p handbook-pipeline --test pipeline_loader`

2. **Packet 1.2 — Stage-Root Discovery And Validation Adoption**
   - Move supported stage-source assumptions, stage discovery, and inseparable stage/pipeline path validation onto the active contract.
   - This packet is not complete while loader validation still codifies rejection of non-default stage roots; the proof must flip to positive contract-driven acceptance.
   - Primary verification:
     - `cargo test -p handbook-pipeline --test pipeline_catalog`
     - `cargo test -p handbook-pipeline --test pipeline_loader`
     - `cargo test -p handbook-pipeline --test pipeline_compile`
     - `cargo test -p handbook-pipeline --test pipeline_route_resolution`

3. **Packet 1.3 — Public Pipeline Storage Layout Injection**
   - Promote the storage contract to a supported public/import-facing seam.
   - Adopt it across route-state, capture, and handoff entry points while preserving handbook-product defaults.
   - Primary verification:
     - `cargo test -p handbook-pipeline --test pipeline_state_store`
     - `cargo test -p handbook-pipeline --test pipeline_capture`
     - `cargo test -p handbook-pipeline --test pipeline_handoff`

4. **Packet 1.4 — Final Set Proof**
   - Run the verification wall, record bounded residual defaults, and confirm the triplet matches live repo truth.
   - Primary verification:
     - `cargo test -p handbook-pipeline`
     - `cargo check --workspace`
     - `cargo fmt --all -- --check`
     - `cargo clippy --workspace --all-targets -- -D warnings`
     - targeted `rg` sweep for remaining fixed roots

## Boundaries

- **Always:**
  - Keep Set 1 focused on `handbook-pipeline` import-layout parameterization only.
  - Preserve the difference between a **public/import-facing contract** and a **handbook-product default helper**.
  - Ground acceptance in live crate behavior, focused tests, and source inspection.
  - Keep packets sequential: 1.1 → 1.2 → 1.3 → 1.4.
  - Record any remaining fixed handbook-product defaults explicitly instead of pretending they disappeared.

- **Ask first:**
  - If the cleanest implementation would require widening into `handbook-flow`, `handbook-cli`, or `handbook-compiler`.
  - If stage-root or storage-layout adoption appears to require generalizing consumer ownership beyond the bounded default already accepted in the repo.
  - If the public import-facing surface would need a broader facade or a breaking rename to stay coherent.
  - If implementation reveals that `setup` must become part of the import contract rather than staying product-default behavior.

- **Never:**
  - Execute the actual Substrate import from this set.
  - Widen into CLI/compiler product-shell cleanup.
  - Treat this set as permission to implement Set 2 (`handbook-flow`) or Set 3 (honesty cleanup) early.
  - Introduce compatibility shims as a long-term substitute for a real import-facing contract.
  - Generalize the bounded default consumer into a free-form multi-consumer system.

## Success Criteria

1. `handbook-pipeline` exposes a supported declarative layout seam that can represent non-default pipeline/profile/runner/stage roots.
2. Supported stage-source assumptions, stage discovery, and inseparable path validation no longer depend on raw repo-level `core/stages/**` ownership or handbook-product default roots as active truth.
3. `handbook-pipeline` exposes a supported public/import-facing storage layout seam for runtime state, stage-capture provenance, capture cache, and handoff bundle roots.
4. Route-state, capture, and handoff entry points can honor the import-facing storage layout seam without downstream callers reaching into crate-private implementation details.
5. Handbook-product defaults remain available only as explicit default helpers or explicitly bounded product-default behavior.
6. The verification wall passes and the tasks doc records any remaining bounded defaults honestly.
7. The set finishes without widening into CLI/compiler work or the later Set 2 / Set 3 seams.

## Open Questions

1. **Single declarative contract vs split root contracts**
   - Default assumption for this set: keep pipeline/profile/runner/stage roots in one coherent declarative-layout owner so downstream importers do not need to stitch multiple partial contracts together.

2. **Handoff bundle root containment**
   - Default assumption for this set: keep handoff bundle root separate from runtime state containment rules, matching the current `artifacts/handoff/feature_slice` model, unless implementation reveals a concrete import-boundary problem.

3. **`setup` posture**
   - Default assumption for this set: `setup` remains outside the reusable import contract. If implementation proves otherwise, stop and ask before widening.
