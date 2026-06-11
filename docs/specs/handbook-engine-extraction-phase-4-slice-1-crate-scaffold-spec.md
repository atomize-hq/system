# Spec: Handbook Engine Extraction Phase 4 Slice 1 (Slice 4.1) - Workspace And Crate Scaffold

## Assumptions

1. Phase 3 Slices 3.1 through 3.4 are complete enough that Phase 4 crate-boundary planning is now the active next gate, even though the `Status` block in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` still lags behind the latest slice progression.
2. Live workspace truth currently contains only `crates/compiler` and `crates/cli`; `crates/engine`, `crates/pipeline`, and `crates/flow` do not exist yet.
3. Slice 4.1 is scaffold-only: it may add workspace members, crate manifests, and minimal compile-time public surfaces, but it must not move major implementation modules out of `crates/compiler` yet.
4. `crates/compiler/src/lib.rs` remains the temporary implementation center during this slice; any new crate surface introduced here is a deliberately narrow compile-through boundary, not the final ownership landing for engine, pipeline, or flow logic.
5. `handbook-cli` should remain behavior-stable and may keep its current dependency posture during Slice 4.1 unless a tiny compile-through adjustment is strictly required to prove the new crates participate cleanly in the workspace.
6. The intended package names are `handbook-engine`, `handbook-pipeline`, and `handbook-flow`, with Rust library crate names `handbook_engine`, `handbook_pipeline`, and `handbook_flow`.
7. Rendering, refusal, and error ownership are still intentionally unresolved at this point; Slice 4.1 must not force those modules into a premature crate home.

## Objective

Create the first real Phase 4 workspace boundary by adding `crates/engine`, `crates/pipeline`, and `crates/flow` to the Rust workspace, defining minimal crate manifests and minimal public compile-through surfaces that make the future split structurally real without moving major logic yet.

The maintainer needs this slice so later Phase 4 migration slices can move code into already-existing workspace homes instead of mixing crate creation, ownership decisions, and logic migration in the same landing. Success means:

- the workspace declares `crates/engine`, `crates/pipeline`, and `crates/flow` as real members
- each new crate has a minimal `Cargo.toml` and `src/lib.rs`
- each new crate exposes only a narrow, explicit surface aligned to its future ownership lane
- `crates/compiler` remains the temporary implementation center for now
- no major module moves, caller rewires, or compiler-facade retirement leak into this slice
- `cargo check --workspace` passes after the scaffold lands

## Tech Stack

- Rust 2021 workspace
- Current crates:
  - `handbook-compiler`
  - `handbook-cli`
- New scaffold crates introduced by this slice:
  - `handbook-engine`
  - `handbook-pipeline`
  - `handbook-flow`
- Authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md`

## Commands

Primary slice verifier:

```bash
cargo check --workspace
```

Workspace-member verification:

```bash
cargo metadata --no-deps --format-version 1 | python3 -c 'import sys,json; m=json.load(sys.stdin); print("\n".join(sorted(p["name"] for p in m["packages"])))'
```

Workspace declaration scan:

```bash
rg -n 'crates/(engine|pipeline|flow)' Cargo.toml crates/*/Cargo.toml
```

Dependency-posture scan:

```bash
cargo tree -p handbook-engine -e normal
cargo tree -p handbook-pipeline -e normal
cargo tree -p handbook-flow -e normal
```

Minimal public-surface scan:

```bash
rg -n 'pub use|contract_version|workspace_contract_version' crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs
```

Behavior-stability guard:

```bash
cargo test -p handbook-cli --test cli_surface
```

Final extraction wall for the slice:

```bash
cargo check --workspace
cargo test --workspace
```

## Project Structure

```text
Cargo.toml                     -> workspace membership authority; Slice 4.1 adds engine/pipeline/flow members here
crates/compiler/Cargo.toml     -> current monolithic implementation crate that remains the temporary source of compile-through truth
crates/compiler/src/lib.rs     -> current oversized public facade; may receive only narrow compile-through adjustments if required
crates/engine/Cargo.toml       -> new package manifest for handbook-engine
crates/engine/src/lib.rs       -> minimal engine-facing public surface for later engine migration work
crates/pipeline/Cargo.toml     -> new package manifest for handbook-pipeline
crates/pipeline/src/lib.rs     -> minimal pipeline-facing public surface for later pipeline migration work
crates/flow/Cargo.toml         -> new package manifest for handbook-flow
crates/flow/src/lib.rs         -> minimal flow-facing public surface for later flow migration work
crates/cli/Cargo.toml          -> current CLI dependency root; should stay stable unless tiny compile-through fallout is unavoidable
crates/cli/src/main.rs         -> current caller surface; direct caller rewires are deferred to Slice 4.5
crates/cli/tests/cli_surface.rs -> public CLI regression guard if any narrow compile-through fallout reaches the binary crate
```

## Code Style

Prefer explicit, narrow compile-through surfaces over wildcard facade cloning.

```rust
pub use handbook_compiler::{ArtifactManifest, ManifestInputs};

pub fn engine_contract_version() -> &'static str {
    handbook_compiler::workspace_contract_version()
}
```

Conventions:

- use explicit `pub use` lists only; do not introduce `pub use handbook_compiler::*`
- keep each new crate surface aligned to its future ownership lane
- prefer one clearly named contract-version helper over broad placeholder modules when only a compile-through proof is needed
- avoid circular dependencies among the new crates during the scaffold slice
- treat any symbol re-export in Slice 4.1 as intentionally temporary unless a later slice formally promotes it

## Testing Strategy

- Framework: existing Cargo workspace checks and integration tests
- Primary test levels:
  - workspace compilation proof via `cargo check --workspace`
  - dependency graph inspection via `cargo metadata` and `cargo tree`
  - CLI behavior regression via `cargo test -p handbook-cli --test cli_surface`
- Coverage focus:
  - new crates exist as real workspace members
  - manifests resolve cleanly without circular or accidental broad dependency edges
  - minimal public crate surfaces compile without forcing major logic movement
  - the existing CLI still compiles and behaves as before
- Coverage expectation:
  - Packet 4.1.1 proves workspace/member and manifest scaffolding only
  - Packet 4.1.2 proves minimal public surfaces and compile-through wiring only
  - full workspace tests remain optional during packet work but become part of the final slice verification wall

## Slice Scope

In scope:

- add `crates/engine`, `crates/pipeline`, and `crates/flow` as real workspace members
- add minimal manifests and `src/lib.rs` files for those crates
- define a temporary dependency posture that keeps the scaffold compile-valid without forcing early ownership decisions
- expose minimal, explicit public crate surfaces aligned to the Phase 4 ownership map
- keep `crates/compiler` as the temporary implementation center while the new crates bootstrap
- preserve existing CLI behavior and avoid direct caller rewires

Out of scope:

- moving major implementation modules out of `crates/compiler`
- rewiring `crates/cli` or tests to consume the new crates as their primary runtime dependency path
- narrowing or retiring `crates/compiler` as the main facade
- deciding final ownership for `rendering`, `refusal`, or `error`
- changing public handbook command semantics, help text, or output behavior
- updating the broader root plan, slice map, or packet-prompt artifacts as part of this slice

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Phase 3 authority slices that prepared engine-shaped seams:
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md`
- Live workspace truth:
  - `Cargo.toml`
  - `crates/compiler/Cargo.toml`
  - `crates/cli/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `crates/cli/src/main.rs`
  - `crates/cli/tests/cli_surface.rs`

## Initial Ownership Map For The Scaffold Slice

| Future crate | Slice 4.1 posture | Expected later migration owner |
| --- | --- | --- |
| `handbook-engine` | may expose only narrow compile-through engine-safe symbols or contract markers | canonical artifacts, manifest, freshness, baseline validation, approved authoring core |
| `handbook-pipeline` | may expose only narrow compile-through pipeline-safe symbols or contract markers | declarative pipeline loading, routing, route state, compile/capture/handoff, approved setup helpers |
| `handbook-flow` | may expose only narrow compile-through flow-safe symbols or contract markers | resolver, packet result, budget, and related higher-level flow composition |
| `handbook-compiler` | remains the temporary implementation center in Slice 4.1 | intentionally narrowed or retired later in Slice 4.5 |
| `handbook-cli` | remains the product shell and behavior guard | direct caller rewires deferred until later Phase 4/5 work |

## Boundaries

- Always:
  - keep Slice 4.1 scaffold-only
  - make the new crates real workspace members with minimal compile-valid manifests and source roots
  - keep public surfaces explicit and narrow
  - keep `handbook-compiler` as the temporary implementation center for this slice
  - preserve current CLI behavior and use `cli_surface` as the behavior guard if any narrow fallout appears
  - keep the dependency graph acyclic and easy to reason about
- Ask first:
  - rewiring `handbook-cli` to depend directly on the new crates as its primary runtime path
  - promoting a broad set of compiler symbols into any new crate facade
  - introducing shared workspace dependency tables or package metadata refactors beyond what the scaffold strictly needs
  - forcing `rendering`, `refusal`, or `error` into one of the new crates early
- Never:
  - move major implementation modules in Slice 4.1
  - retire or significantly narrow `crates/compiler` in this slice
  - use wildcard facade exports from `handbook-compiler`
  - introduce circular dependencies among `handbook-engine`, `handbook-pipeline`, `handbook-flow`, and `handbook-compiler`
  - widen into Slice 4.2 engine migration, Slice 4.3 pipeline migration, Slice 4.4 flow migration, or Slice 4.5 caller rewiring

## Success Criteria

- `Cargo.toml` declares `crates/engine`, `crates/pipeline`, and `crates/flow` as workspace members.
- `crates/engine`, `crates/pipeline`, and `crates/flow` each contain a valid `Cargo.toml` and `src/lib.rs`.
- The new crates compile as part of the workspace and expose only narrow, explicit public surfaces.
- The scaffold dependency posture is acyclic and does not force premature ownership decisions.
- `crates/compiler` remains the temporary implementation center, with any adjustments limited to compile-through support.
- `cargo check --workspace` passes.
- No major module moves, caller rewires, or compiler-facade retirement leaked into the slice.

## Open Questions

- Should Slice 4.1 expose only contract-version markers from the new crates, or should it also expose a very small set of lane-aligned re-exports to prove cross-crate compilation ergonomics before Slice 4.2 begins?
- Should the scaffold keep all three new crates depending only on `handbook-compiler` initially, or is there value in introducing a one-way `handbook-pipeline -> handbook-engine` dependency immediately if it stays compile-through only?
- Does Slice 4.1 need any dedicated smoke tests inside the new crates, or is `cargo check --workspace` plus existing CLI coverage sufficient until real module migration starts?
