# Spec: Handbook Engine Extraction Phase 4 Slice 2 (Slice 4.2) - Engine Crate Migration

## Assumptions

1. Phase 4 Slice 1 is complete enough in live code that `crates/engine`, `crates/pipeline`, and `crates/flow` already exist as real workspace members, and `handbook-engine` is still only a temporary compile-through scaffold.
2. Slice 4.2 is the first real ownership move inside Phase 4: `handbook-engine` must become the implementation home for engine-safe code rather than continuing to depend on `handbook-compiler` as a thin wrapper.
3. The current `handbook-engine -> handbook-compiler` scaffold dependency from Slice 4.1 is temporary and may be removed or inverted during this slice, but the end state must stay acyclic and easy to reason about.
4. The first approved engine-safe module family for migration is the canonical artifact stack: `artifact_manifest.rs`, `canonical_artifacts.rs`, `freshness.rs`, and `baseline_validation.rs`.
5. The second approved engine-safe module family for this slice is the deterministic authoring core proven in Phase 3: `charter_core.rs`, `project_context_core.rs`, and `environment_inventory_core.rs`, plus the typed public surfaces that remain free of process spawning, environment-variable reads, clock resolution, template ownership, repo mutation, and CLI wording.
6. Shell-owned authoring behavior remains outside `handbook-engine` in this slice, including template-library selection, Codex runtime transport, canonical preflight and write orchestration, lock handling, CLI-oriented refusal wording, and ambient timestamp/env resolution.
7. `handbook-cli` should remain behavior-stable and may continue to consume `handbook-compiler` as its main dependency path during Slice 4.2; direct caller rewires to `handbook-engine` remain deferred to Slice 4.5.
8. `setup`, `doctor`, `refusal`, `rendering`, `template_library`, `pipeline*`, `route_state`, `resolver`, `packet_result`, and `budget` remain out of scope unless a tiny compile-through adjustment is strictly required to keep Slice 4.2 coherent.

## Objective

Move the approved engine-safe implementation behind `handbook-engine` and prove the new crate owns canonical artifact loading, manifest generation, freshness computation, baseline validation, and the deterministic authoring-core surfaces prepared in Phase 3, while `handbook-compiler` shrinks into a temporary compatibility facade instead of remaining the true implementation center.

The maintainer needs this slice so Phase 4 stops being only workspace intent and starts becoming real crate ownership. Success means:

- `handbook-engine` owns the canonical artifact / manifest / freshness / baseline implementation stack
- `handbook-engine` owns the approved deterministic authoring-core modules from Phase 3
- `handbook-engine` no longer depends on `handbook-compiler` for those owners
- `handbook-compiler` exposes only compatibility re-exports or thin adapters for the migrated engine-safe surfaces
- `handbook-cli` behavior remains stable without a broad direct-caller rewire
- `cargo test -p handbook-engine` and `cargo test -p handbook-cli --test cli_surface` pass after the migration lands

## Tech Stack

- Rust 2021 workspace
- Current crates:
  - `handbook-compiler`
  - `handbook-cli`
  - `handbook-engine`
  - `handbook-pipeline`
  - `handbook-flow`
- Authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-engine
cargo test -p handbook-cli --test cli_surface
```

Dependency-posture verification:

```bash
cargo tree -p handbook-engine -e normal
cargo tree -p handbook-compiler -e normal
```

Workspace guard:

```bash
cargo check --workspace
```

Engine ownership scan:

```bash
rg -n 'artifact_manifest|canonical_artifacts|freshness|baseline_validation|charter_core|project_context_core|environment_inventory_core' crates/engine crates/compiler/src
```

Compiler facade scan:

```bash
rg -n 'pub use handbook_engine|use handbook_engine|handbook_engine::' crates/compiler/src/lib.rs crates/compiler/src/author
```

Shell-behavior guard during author-core migration:

```bash
cargo test -p handbook-compiler --test author
```

Final slice verification wall:

```bash
cargo check --workspace
cargo test -p handbook-engine
cargo test -p handbook-cli --test cli_surface
```

## Project Structure

```text
crates/engine/Cargo.toml                               -> Slice 4.2 turns handbook-engine into a real implementation crate instead of a scaffold wrapper
crates/engine/src/lib.rs                               -> engine public surface and re-export boundary for migrated engine-safe modules
crates/engine/src/**                                   -> new home for canonical artifact, manifest, freshness, baseline, and approved author-core modules
crates/engine/tests/**                                 -> new engine-owned regression coverage for migrated module families
crates/compiler/Cargo.toml                             -> temporary compatibility crate; may gain a dependency on handbook-engine as Slice 4.2 lands
crates/compiler/src/lib.rs                             -> temporary compatibility facade that should re-export or thinly delegate to handbook-engine
crates/compiler/src/artifact_manifest.rs               -> current engine-safe implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/canonical_artifacts.rs             -> current engine-safe implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/freshness.rs                       -> current engine-safe implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/baseline_validation.rs             -> current engine-safe implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/author/charter.rs                  -> current public charter facade; should keep shell behavior while delegating deterministic core ownership
crates/compiler/src/author/project_context.rs          -> current public project-context facade; should keep shell behavior while delegating deterministic core ownership
crates/compiler/src/author/environment_inventory.rs    -> current public environment-inventory facade; should keep shell behavior while delegating deterministic core ownership
crates/compiler/src/author/charter_core.rs             -> deterministic charter core planned to move into handbook-engine
crates/compiler/src/author/project_context_core.rs     -> deterministic project-context core planned to move into handbook-engine
crates/compiler/src/author/environment_inventory_core.rs -> deterministic environment-inventory core planned to move into handbook-engine
crates/compiler/src/author/*_shell.rs                  -> shell/runtime/template/write owners that stay outside handbook-engine in this slice
crates/compiler/tests/**                               -> existing compiler regression coverage to retain for remaining shell-owned behavior during transition
crates/cli/Cargo.toml                                  -> current CLI dependency root; direct caller rewires remain deferred
crates/cli/tests/cli_surface.rs                        -> public CLI regression guard for behavior stability during the migration
```

## Code Style

Prefer engine-owned implementations plus explicit compatibility re-exports over duplicate logic or wildcard facade cloning.

```rust
pub use handbook_engine::{
    ArtifactManifest, CanonicalArtifacts, FreshnessTruth,
    compute_freshness,
};
```

Conventions:

- `handbook-engine` owns reusable deterministic types and functions
- `handbook-compiler` may keep temporary compatibility exports, but it should not remain the true implementation home for migrated engine-safe symbols
- keep public re-export lists explicit; do not introduce `pub use handbook_engine::*`
- keep shell/runtime/template/CLI wording concerns out of `handbook-engine`
- keep the dependency graph acyclic; Slice 4.2 should reduce temporary scaffold coupling, not deepen it
- when a public compiler facade remains for compatibility, make it obviously thin and easy to retire later

## Testing Strategy

- Framework: Cargo package tests and existing CLI regression tests
- Primary test levels:
  - `handbook-engine` package tests for migrated canonical artifact, manifest, freshness, baseline, and deterministic author-core surfaces
  - `handbook-compiler` author tests as transition guards while shell-owned author behavior still lives there
  - `handbook-cli` `cli_surface` tests as the public behavior guard
- Coverage focus:
  - migrated engine-safe logic has one implementation owner in `handbook-engine`
  - dependency inversion stays acyclic after the scaffold dependency changes
  - compiler compatibility facades remain thin and behavior-stable
  - deterministic authoring-core surfaces move without dragging template-library, Codex runtime, timestamp/env resolution, or canonical write flow into `handbook-engine`
- Coverage expectation:
  - Packet 4.2.1 proves ownership of the canonical artifact stack first
  - Packet 4.2.2 proves ownership of approved authoring-core surfaces second
  - the final slice wall proves `handbook-engine` is a real tested crate while the CLI surface still behaves as before

## Slice Scope

In scope:

- move `artifact_manifest`, `canonical_artifacts`, `freshness`, and `baseline_validation` behind `handbook-engine`
- move the approved deterministic authoring-core modules behind `handbook-engine`
- create the minimum `handbook-engine` public module layout needed to own those surfaces cleanly
- update package dependencies so the engine crate becomes the implementation owner without cycles
- keep `handbook-compiler` as a temporary compatibility facade for migrated engine-safe APIs
- move or recreate regression coverage in `crates/engine/tests/**` for migrated surfaces
- preserve CLI behavior while avoiding a broad caller rewire

Out of scope:

- moving `setup`, `doctor`, `refusal`, `rendering`, or `template_library` into `handbook-engine`
- moving `pipeline`, `route_state`, compile/capture/handoff, or setup-helper ownership into `handbook-pipeline`
- moving `resolver`, `packet_result`, or `budget` into `handbook-flow`
- rewiring `handbook-cli` to depend directly on `handbook-engine` as its main runtime path
- narrowing or retiring `handbook-compiler` beyond what is strictly necessary for a temporary compatibility facade
- broad public CLI wording, help-text, or behavior changes
- Phase 4.3, Phase 4.4, or Phase 4.5 work

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 4.1 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md`
- Phase 3 authority set that defines approved authoring core vs shell boundaries:
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md`
- Live package and module truth:
  - `Cargo.toml`
  - `crates/engine/Cargo.toml`
  - `crates/engine/src/lib.rs`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/artifact_manifest.rs`
  - `crates/compiler/src/canonical_artifacts.rs`
  - `crates/compiler/src/freshness.rs`
  - `crates/compiler/src/baseline_validation.rs`
  - `crates/compiler/src/author/*.rs`
  - `crates/compiler/tests/artifact_manifest_interface.rs`
  - `crates/compiler/tests/canonical_artifacts_ingest.rs`
  - `crates/compiler/tests/freshness_computation.rs`
  - `crates/compiler/tests/author.rs`
  - `crates/cli/tests/cli_surface.rs`

## Current Ownership Gap To Close

| Surface | Current live owner | Slice 4.2 requirement |
| --- | --- | --- |
| canonical artifact model and ingest | `crates/compiler/src/canonical_artifacts.rs` | move to `handbook-engine` as the real reusable owner |
| manifest generation and version surface | `crates/compiler/src/artifact_manifest.rs` | move to `handbook-engine` with compiler-only compatibility re-exports if still needed |
| freshness computation and schema/fingerprint contract | `crates/compiler/src/freshness.rs` | move to `handbook-engine` as reusable deterministic logic |
| baseline validation over canonical artifacts | `crates/compiler/src/baseline_validation.rs` | move to `handbook-engine` alongside the canonical artifact owners it depends on |
| deterministic charter core | `crates/compiler/src/author/charter_core.rs` plus core-facing exports from `author/charter.rs` | move to `handbook-engine`; keep shell/runtime/template/write behavior outside |
| deterministic project-context core | `crates/compiler/src/author/project_context_core.rs` plus explicit-input render/validation helpers | move to `handbook-engine`; keep timestamp/env/write behavior outside |
| deterministic environment-inventory core | `crates/compiler/src/author/environment_inventory_core.rs` plus pure validation helpers | move to `handbook-engine`; keep canonical-truth loading, prompt/runtime, and write flow outside |
| author shell/runtime behavior | `crates/compiler/src/author/*_shell.rs` and shell-facing portions of `author/*.rs` | remain outside `handbook-engine` in this slice |

## Boundaries

- Always:
  - make `handbook-engine` the true implementation owner for the approved engine-safe surfaces in this slice
  - keep the dependency graph acyclic and simpler than the Slice 4.1 scaffold posture
  - keep `handbook-compiler` compatibility layers explicit and thin
  - preserve current CLI behavior during the migration
  - keep author shell/runtime/template/write responsibilities outside `handbook-engine`
  - add or move regression coverage so migrated logic is tested from the engine crate itself
- Ask first:
  - moving `setup`, `doctor`, `refusal`, `rendering`, `template_library`, or `repo_file_access` into `handbook-engine`
  - changing the public CLI contract, wording, or command help
  - renaming broad public `handbook_compiler` engine-facing types/functions instead of using temporary compatibility re-exports
  - introducing new cross-crate edges from `handbook-pipeline` or `handbook-flow` during this slice unless a tiny compile-through fix is strictly necessary
- Never:
  - keep `handbook-engine` depending on `handbook-compiler` as the real owner after Slice 4.2 lands
  - duplicate migrated implementations in both crates as a long-term compatibility strategy
  - pull Codex runtime transport, template ownership, lock/write orchestration, or CLI-oriented refusal text into `handbook-engine`
  - widen into pipeline migration, flow migration, direct caller rewires, or compiler retirement
  - let project-context ambient timestamp/env resolution become an engine-owned implicit dependency

## Success Criteria

- `handbook-engine` owns the canonical artifact, manifest, freshness, and baseline-validation implementation stack.
- `handbook-engine` owns the approved deterministic authoring-core modules without inheriting shell/runtime/template/write behavior.
- The temporary scaffold dependency from `handbook-engine` to `handbook-compiler` is removed or inverted into an acyclic compatibility posture aligned to engine ownership.
- `handbook-compiler` becomes a thin compatibility facade for the migrated engine-safe surfaces instead of remaining their real implementation home.
- Engine-owned regression coverage exists and `cargo test -p handbook-engine` passes.
- `cargo test -p handbook-cli --test cli_surface` passes without a broad caller rewire.
- No setup/doctor/rendering/refusal work, no pipeline/flow migration, and no caller-rewire leakage lands in this slice.

## Open Questions

- Should Slice 4.2 preserve the exact current `handbook_compiler` engine-facing symbol names exclusively through re-exports, or is there one narrow engine-first public-module surface worth introducing now if the CLI remains behavior-stable?
- For project-context deterministic rendering, should `handbook-engine` expose only the explicit-input core render function while `handbook-compiler` keeps the ambient-time wrapper, or does live code prove a different compatibility seam is cleaner?
