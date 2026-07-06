# Spec: Handbook Engine Extraction Phase 4 Slice 3 (Slice 4.3) - Pipeline Crate Migration

## Assumptions

1. Phase 4 Slice 2 is complete enough in live code that `handbook-engine` already owns the approved engine-safe implementation families, while `handbook-pipeline` is still only a scaffold crate that forwards through `handbook-compiler`.
2. Slice 4.3 is the first real ownership move for reusable pipeline-safe runtime logic: declarative pipeline loading, supported-target registry, route resolution, route-state persistence, trusted-session validation, compile/capture/handoff mechanics, and stage-10 provenance should stop living primarily in `handbook-compiler`.
3. `handbook-cli` should remain behavior-stable and may continue to depend on `handbook-compiler` as its main compatibility path during Slice 4.3; direct caller rewires to `handbook-pipeline` remain deferred to Slice 4.5.
4. Phase 2 Slice 1 and Slice 2 remain the authority for supported target vocabulary and compile/capture/handoff behavior; Slice 4.3 relocates ownership but must not redesign the approved runtime wedge.
5. Phase 1 layout and stateful-storage decisions remain authoritative for `.handbook/state/**`, capture provenance paths, and handoff bundle locations; Slice 4.3 may move code ownership, but it must not silently redefine those layouts.
6. The approved setup-helper seam for this slice is narrow: move only reusable pipeline-safe helper logic that currently couples `setup.rs` to runtime-state reset and adjacent pipeline mechanics. Canonical starter-template ownership and other engine-safe setup behavior stay outside `handbook-pipeline` unless a tiny compatibility adjustment is strictly required.
7. Because current pipeline modules depend on `declarative_roots.rs`, `repo_file_access.rs`, and pipeline-specific portions of `layout.rs`, narrow supporting-infra moves or splits are allowed if they are strictly required to make `handbook-pipeline` a real owner without depending on `handbook-compiler`.
8. `resolver`, `packet_result`, `budget`, `rendering`, `refusal`, `error`, and broad CLI shell wording remain out of scope unless a tiny compatibility shim is strictly required to keep Slice 4.3 coherent.

## Objective

Move the approved reusable pipeline implementation behind `handbook-pipeline` and prove the new crate owns declarative pipeline loading, supported-target registry, route resolution, route-state persistence, trusted pipeline-session validation, compile/capture/handoff mechanics, stage-10 provenance, and approved setup helpers, while `handbook-compiler` shrinks into a temporary compatibility facade instead of remaining the true implementation center.

The maintainer needs this slice so the Phase 4 crate split becomes real for runtime pipeline behavior, not just for engine-safe logic. Success means:

- `handbook-pipeline` owns pipeline loading, route resolution, and route-state runtime logic
- `handbook-pipeline` owns compile/capture/handoff mechanics and stage-10 provenance
- `handbook-pipeline` owns the approved reusable setup-helper seam without absorbing full setup product behavior
- `handbook-pipeline` no longer depends on `handbook-compiler` as the real implementation owner for Slice 4.3 surfaces
- `handbook-compiler` exposes only compatibility re-exports or thin adapters for migrated pipeline-safe APIs
- `cargo test -p handbook-pipeline` and `cargo test -p handbook-cli --test cli_surface` pass after the migration lands

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
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-pipeline
cargo test -p handbook-cli --test cli_surface
```

Dependency-posture verification:

```bash
cargo tree -p handbook-pipeline -e normal
cargo tree -p handbook-compiler -e normal
```

Foundation regression guards during migration:

```bash
cargo test -p handbook-compiler --test pipeline_loader
cargo test -p handbook-compiler --test pipeline_catalog
cargo test -p handbook-compiler --test pipeline_route_resolution
cargo test -p handbook-compiler --test pipeline_state_store
```

Runtime migration regression guards:

```bash
cargo test -p handbook-compiler --test pipeline_compile
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
cargo test -p handbook-compiler --test setup
```

Ownership scan:

```bash
rg -n 'load_pipeline_catalog|resolve_pipeline_route|load_route_state|persist_route_basis|load_trusted_pipeline_session|compile_pipeline_stage|capture_pipeline_output|emit_pipeline_handoff_bundle|stage_10_feature_spec' crates/pipeline crates/compiler/src
```

Supporting-infra scan:

```bash
rg -n 'CompilerWorkspace|NormalizedRepoRelativePath|RepoLayoutRoot|pipeline_root|profile_root|runner_root' crates/pipeline crates/compiler/src
```

Optional public help fallout guard:

```bash
cargo test -p handbook-cli --test help_drift_guard
```

Final slice verification wall:

```bash
cargo check --workspace
cargo test -p handbook-pipeline
cargo test -p handbook-cli --test cli_surface
```

## Project Structure

```text
crates/pipeline/Cargo.toml                                  -> Slice 4.3 turns handbook-pipeline into a real implementation crate instead of a scaffold wrapper
crates/pipeline/src/lib.rs                                  -> pipeline public surface and re-export boundary for migrated runtime modules
crates/pipeline/src/pipeline.rs                             -> new owner for declarative pipeline loading, catalog, selection, and supported-target registry logic
crates/pipeline/src/pipeline_route.rs                       -> new owner for route evaluation and activation reasoning
crates/pipeline/src/route_state.rs                          -> new owner for persisted route-state, route-basis, trusted-session, and reset helpers
crates/pipeline/src/pipeline_compile.rs                     -> new owner for pipeline compile mechanics
crates/pipeline/src/pipeline_capture.rs                     -> new owner for pipeline capture planning/apply mechanics
crates/pipeline/src/pipeline_handoff.rs                     -> new owner for handoff emit/validate mechanics
crates/pipeline/src/stage_10_feature_spec_provenance.rs     -> new owner for stage-10 provenance generation and validation
crates/pipeline/src/setup.rs or equivalent helper namespace -> new home for the approved reusable setup-helper seam if a dedicated module is required
crates/pipeline/src/{declarative_roots,repo_file_access,layout}.rs -> narrow supporting infra if required to keep handbook-pipeline independent from handbook-compiler
crates/pipeline/tests/**                                    -> new pipeline-owned regression coverage for migrated runtime families
crates/compiler/Cargo.toml                                  -> temporary compatibility crate that should depend on handbook-pipeline instead of owning Slice 4.3 implementations directly
crates/compiler/src/lib.rs                                  -> temporary compatibility facade that should re-export or thinly delegate to handbook-pipeline
crates/compiler/src/pipeline.rs                             -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/pipeline_route.rs                       -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/route_state.rs                          -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/pipeline_compile.rs                     -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/pipeline_capture.rs                     -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/pipeline_handoff.rs                     -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/stage_10_feature_spec_provenance.rs     -> current implementation owner to retire or replace with a thin compatibility layer
crates/compiler/src/setup.rs                                -> product-facing setup facade that should retain only thin setup orchestration plus any engine-owned starter-template behavior left outside the pipeline seam
crates/compiler/tests/{pipeline_loader,pipeline_catalog,pipeline_route_resolution,pipeline_state_store,pipeline_compile,pipeline_capture,pipeline_handoff,setup}.rs -> existing compiler transition guards while compatibility surfaces remain
crates/cli/tests/cli_surface.rs                             -> public CLI regression guard for behavior stability during the migration
crates/cli/tests/help_drift_guard.rs                        -> public help wording guard for pipeline-facing CLI fallout
```

## Code Style

Prefer pipeline-owned implementations plus explicit compatibility re-exports over duplicate logic or wildcard facade cloning.

```rust
pub use handbook_pipeline::{
    compile_pipeline_stage, capture_pipeline_output, emit_pipeline_handoff_bundle,
    load_pipeline_catalog, load_route_state, persist_route_basis,
};
```

Conventions:

- `handbook-pipeline` owns reusable declarative/runtime pipeline logic
- `handbook-compiler` may keep temporary compatibility exports, but it should not remain the true implementation home for migrated Slice 4.3 symbols
- keep public re-export lists explicit; do not introduce `pub use handbook_pipeline::*`
- allow narrow supporting-infra moves only when required to break compiler ownership cleanly
- keep full setup product behavior, flow-layer logic, and direct caller rewires out of Slice 4.3
- keep the dependency graph acyclic; Slice 4.3 should remove scaffold indirection, not deepen it

## Testing Strategy

- Framework: Cargo package tests and existing compiler/CLI regression tests
- Primary test levels:
  - `handbook-pipeline` package tests for migrated loader, route, route-state, compile/capture/handoff, provenance, and approved setup-helper surfaces
  - `handbook-compiler` integration tests as transition guards while compatibility facades still exist
  - `handbook-cli` `cli_surface` and `help_drift_guard` tests as public behavior guards
- Coverage focus:
  - migrated runtime logic has one implementation owner in `handbook-pipeline`
  - route-state, route-basis, and trusted-session behavior remain stable after migration
  - compile/capture/handoff and stage-10 provenance behavior remain stable after migration
  - approved setup-helper alignment does not drag starter-template or engine-safe ownership into `handbook-pipeline`
  - compiler compatibility facades remain thin and easy to retire later
- Coverage expectation:
  - Packet 4.3.1 proves loader/route/route-state ownership first
  - Packet 4.3.2 proves compile/capture/handoff ownership second
  - Packet 4.3.3 proves provenance and setup-helper alignment third
  - the final slice wall proves `handbook-pipeline` is a real tested crate while the CLI surface still behaves as before

## Slice Scope

In scope:

- move `pipeline`, `pipeline_route`, and `route_state` behind `handbook-pipeline`
- move compile/capture/handoff mechanics behind `handbook-pipeline`
- move stage-10 provenance and approved reusable setup helpers behind `handbook-pipeline`
- move or split the minimum supporting `declarative_roots`, `repo_file_access`, and pipeline-specific `layout` helpers required to keep `handbook-pipeline` independent from `handbook-compiler`
- create the minimum `handbook-pipeline` public module layout needed to own those surfaces cleanly
- update package dependencies so the pipeline crate becomes the implementation owner without cycles
- keep `handbook-compiler` as a temporary compatibility facade for migrated pipeline-safe APIs
- move or recreate regression coverage in `crates/pipeline/tests/**` for migrated surfaces
- preserve CLI behavior while avoiding a broad direct-caller rewire

Out of scope:

- moving `resolver`, `packet_result`, or `budget` into `handbook-flow`
- moving `rendering`, `refusal`, or `error` into `handbook-pipeline`
- broad setup-shell redesign, prompt wording cleanup, or starter-template ownership changes beyond tiny compatibility plumbing
- rewiring `handbook-cli` to depend directly on `handbook-pipeline` as its main runtime path
- narrowing or retiring `handbook-compiler` beyond what is strictly necessary for a temporary compatibility facade
- changing the approved Phase 2 supported-target wedge or redesigning route-state schemas/layouts
- Phase 4.4 or Phase 4.5 work

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 4.1 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md`
- Slice 4.2 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-tasks.md`
- Phase 2 authority that defines supported-target and runtime behavior expectations:
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md`
- Phase 1 authority that defines runtime-state and setup layout expectations:
  - `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md`
- Live package and module truth:
  - `Cargo.toml`
  - `crates/pipeline/Cargo.toml`
  - `crates/pipeline/src/lib.rs`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/declarative_roots.rs`
  - `crates/compiler/src/repo_file_access.rs`
  - `crates/compiler/src/layout.rs`
  - `crates/compiler/src/pipeline.rs`
  - `crates/compiler/src/pipeline_route.rs`
  - `crates/compiler/src/route_state.rs`
  - `crates/compiler/src/pipeline_compile.rs`
  - `crates/compiler/src/pipeline_capture.rs`
  - `crates/compiler/src/pipeline_handoff.rs`
  - `crates/compiler/src/stage_10_feature_spec_provenance.rs`
  - `crates/compiler/src/setup.rs`
  - `crates/compiler/tests/pipeline_loader.rs`
  - `crates/compiler/tests/pipeline_catalog.rs`
  - `crates/compiler/tests/pipeline_route_resolution.rs`
  - `crates/compiler/tests/pipeline_state_store.rs`
  - `crates/compiler/tests/pipeline_compile.rs`
  - `crates/compiler/tests/pipeline_capture.rs`
  - `crates/compiler/tests/pipeline_handoff.rs`
  - `crates/compiler/tests/setup.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`

## Current Ownership Gap To Close

| Surface | Current live owner | Slice 4.3 requirement |
| --- | --- | --- |
| declarative pipeline loading and supported-target registry | `crates/compiler/src/pipeline.rs` | move to `handbook-pipeline` as the real reusable owner |
| route evaluation and activation reasoning | `crates/compiler/src/pipeline_route.rs` | move to `handbook-pipeline` alongside pipeline loading |
| route-state persistence, route-basis, trusted sessions, and reset helpers | `crates/compiler/src/route_state.rs` with setup-facing use from `crates/compiler/src/setup.rs` | move to `handbook-pipeline`; keep compiler setup as a thin consumer |
| pipeline compile mechanics | `crates/compiler/src/pipeline_compile.rs` | move to `handbook-pipeline` while keeping compiler compatibility exports thin |
| pipeline capture mechanics | `crates/compiler/src/pipeline_capture.rs` | move to `handbook-pipeline` while keeping compiler compatibility exports thin |
| handoff emit and validation mechanics | `crates/compiler/src/pipeline_handoff.rs` | move to `handbook-pipeline` while keeping compiler compatibility exports thin |
| stage-10 provenance generation and validation | `crates/compiler/src/stage_10_feature_spec_provenance.rs` | move to `handbook-pipeline` and align compile/capture/handoff callers there |
| approved reusable setup helpers | `crates/compiler/src/setup.rs` plus helpers in `route_state.rs` | move only the reusable pipeline-safe seam to `handbook-pipeline`; keep starter-template and product-facing setup behavior outside |
| supporting repo/declarative/layout helpers | currently rooted in `crates/compiler/src/{declarative_roots,repo_file_access,layout}.rs` | move or split only the subset needed for real pipeline ownership without creating duplicate long-term implementations |

## Boundaries

- Always:
  - make `handbook-pipeline` the true implementation owner for the approved Slice 4.3 surfaces
  - keep the dependency graph acyclic and simpler than the Slice 4.1 scaffold posture
  - keep `handbook-compiler` compatibility layers explicit and thin
  - preserve current CLI behavior during the migration
  - keep full setup product behavior, flow-layer logic, and caller rewires outside this slice
  - add or move regression coverage so migrated logic is tested from the pipeline crate itself
- Ask first:
  - moving `resolver`, `packet_result`, `budget`, `rendering`, `refusal`, or `error` into `handbook-pipeline`
  - changing the public CLI contract, wording, or command help
  - broadening setup migration beyond the approved reusable helper seam
  - making `handbook-cli` depend directly on `handbook-pipeline` during Slice 4.3
  - introducing a wider shared-infra crate instead of a narrow supporting move/split
- Never:
  - keep `handbook-pipeline` depending on `handbook-compiler` as the real owner after Slice 4.3 lands
  - duplicate migrated implementations in both crates as a long-term compatibility strategy
  - redefine the approved Phase 2 supported-target wedge or silently change route-state layout/schema semantics
  - widen into flow migration, direct caller rewires, or compiler retirement
  - let setup starter-template or canonical artifact ownership drift into `handbook-pipeline` by accident

## Success Criteria

- `handbook-pipeline` owns the declarative pipeline loading, route, route-state, compile/capture/handoff, and stage-10 provenance implementation stack.
- `handbook-pipeline` owns the approved reusable setup-helper seam without absorbing full setup product behavior.
- The temporary scaffold dependency from `handbook-pipeline` to `handbook-compiler` is removed and replaced with an acyclic compatibility posture aligned to pipeline ownership.
- `handbook-compiler` becomes a thin compatibility facade for the migrated pipeline-safe surfaces instead of remaining their real implementation home.
- Pipeline-owned regression coverage exists and `cargo test -p handbook-pipeline` passes.
- `cargo test -p handbook-cli --test cli_surface` passes without a broad caller rewire.
- No flow migration, caller rewire work, or broad setup-shell redesign leaks into this slice.

## Open Questions

- Should `declarative_roots`, `repo_file_access`, and pipeline-focused `layout` helpers move wholesale into `handbook-pipeline`, or should Slice 4.3 land a narrower pipeline-owned subset plus compiler compatibility re-exports for remaining non-pipeline consumers like `template_library.rs`?
- Should the approved setup-helper seam live under `handbook-pipeline::setup`, `handbook-pipeline::route_state`, or another narrow helper module as long as full setup product behavior and starter-template ownership remain outside the crate?
