# Plan: Handbook Engine Extraction Phase 4 Slice 1 (Slice 4.1) - Workspace And Crate Scaffold

## Objective

Make the Phase 4 crate split structurally real by adding `handbook-engine`, `handbook-pipeline`, and `handbook-flow` as workspace members with minimal manifests and minimal compile-through public surfaces, while keeping `handbook-compiler` as the temporary implementation center.

Spec reference: [handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md](./handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md)

## Major Modules

1. Workspace root
   - `Cargo.toml`
   - owns workspace membership and is the first place the split becomes structurally real

2. Engine scaffold crate
   - `crates/engine/Cargo.toml`
   - `crates/engine/src/lib.rs`
   - owns the minimal `handbook-engine` package definition and its narrow compile-through surface

3. Pipeline scaffold crate
   - `crates/pipeline/Cargo.toml`
   - `crates/pipeline/src/lib.rs`
   - owns the minimal `handbook-pipeline` package definition and its narrow compile-through surface

4. Flow scaffold crate
   - `crates/flow/Cargo.toml`
   - `crates/flow/src/lib.rs`
   - owns the minimal `handbook-flow` package definition and its narrow compile-through surface

5. Temporary compiler and caller boundary
   - `crates/compiler/src/lib.rs`
   - `crates/cli/Cargo.toml`
   - `crates/cli/src/main.rs`
   - remains stable unless narrow compile-through support requires a tiny adjustment

## Dependencies And Order

### Prerequisite: freeze the scaffold ownership and dependency posture

Why first:

- Slice 4.1 should create crate homes without accidentally starting Slice 4.2 through Slice 4.5 work
- the repo needs one explicit rule for what “minimal public surface” means before any new crate exports appear
- dependency direction must stay simple enough to avoid cycles and future cleanup churn

Output:

- one agreed scaffold posture: new crates are real workspace members, `handbook-compiler` stays temporary implementation center, and public surfaces stay narrow
- one agreed dependency rule: no circular edges and no broad caller rewires in this slice

### Packet 4.1.1: Workspace Members And Crate Manifests

Why first:

- the workspace must recognize the new crates before any public surface or compile-through proof can exist
- manifest creation is the smallest durable landing that turns Phase 4 from planning intent into workspace reality
- later migration slices need stable package names and source roots already on disk

Output:

- `Cargo.toml` includes `crates/engine`, `crates/pipeline`, and `crates/flow`
- each new crate has a valid manifest and `src/lib.rs`
- `cargo metadata` and `cargo check --workspace` see the new members cleanly

### Packet 4.1.2: Minimal Public Crate Surfaces And Compile-Through Wiring

Why second:

- once manifests exist, the repo can prove minimal public surfaces without mixing them into crate-creation churn
- narrow compile-through surfaces establish the first reviewed API seams for the later migration slices
- proving the surfaces after scaffold creation keeps caller rewires and real module moves clearly out of scope

Output:

- each new crate exposes a minimal explicit surface or contract marker aligned to its future ownership lane
- any compile-through support added in `crates/compiler/src/lib.rs` stays narrow and temporary
- existing caller behavior remains stable and no direct caller rewire becomes the primary integration path

## Risks And Mitigations

### Risk: Slice 4.1 accidentally freezes a broad public API too early

Mitigation:

- keep new crate exports explicit and minimal
- prefer contract markers or a very small set of lane-aligned re-exports over broad facade cloning
- treat all Slice 4.1 public surfaces as temporary unless a later slice explicitly promotes them

### Risk: the scaffold introduces circular or confusing dependency edges

Mitigation:

- keep dependency direction one-way and simple during Slice 4.1
- inspect each new crate with `cargo tree -p ... -e normal`
- avoid adding cross-links among new crates unless the value is explicit and still compile-through only

### Risk: workspace scaffolding widens into module migration work

Mitigation:

- keep Packet 4.1.1 manifest- and source-root-only
- keep Packet 4.1.2 limited to minimal public surfaces and compile-through support
- treat any real module move as Phase 4.2, 4.3, or 4.4 leakage unless the approved docs change first

### Risk: callers start rewiring to the new crates before the migration slices are ready

Mitigation:

- keep `handbook-cli` behavior-stable and leave direct caller rewires deferred to Slice 4.5
- allow only tiny compile-through fallout adjustments if strictly required
- use `cargo test -p handbook-cli --test cli_surface` as the public behavior guard

### Risk: the compiler crate gets unintentionally narrowed or destabilized too soon

Mitigation:

- keep `handbook-compiler` as the temporary implementation center in Slice 4.1
- limit any compiler changes to narrow compile-through support only
- defer real compiler narrowing or retirement to Slice 4.5

## Parallel Vs Sequential

Sequential:

- freeze the scaffold posture before editing manifests
- add workspace members and manifests before adding any public surfaces
- verify workspace shape before attempting compile-through wiring

Parallel opportunities after Packet 4.1.1 lands:

- `crates/engine/src/lib.rs`, `crates/pipeline/src/lib.rs`, and `crates/flow/src/lib.rs` can be refined in parallel once the package names and dependency rules are fixed
- dependency-graph inspection and CLI regression verification can run in parallel after the public surfaces are in place

## Verification Checkpoints

### Checkpoint 1: the workspace sees the new crates

```bash
cargo metadata --no-deps --format-version 1 | python3 -c 'import sys,json; m=json.load(sys.stdin); print("\n".join(sorted(p["name"] for p in m["packages"])))'
rg -n 'crates/(engine|pipeline|flow)' Cargo.toml crates/*/Cargo.toml
cargo check --workspace
```

### Checkpoint 2: minimal public surfaces stay narrow and acyclic

```bash
rg -n 'pub use|contract_version|workspace_contract_version' crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs
cargo tree -p handbook-engine -e normal
cargo tree -p handbook-pipeline -e normal
cargo tree -p handbook-flow -e normal
cargo check --workspace
```

### Checkpoint 3: public caller behavior stays stable

```bash
cargo test -p handbook-cli --test cli_surface
```

### Final checkpoint

```bash
cargo check --workspace
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- the workspace contains real `handbook-engine`, `handbook-pipeline`, and `handbook-flow` crates
- each new crate has a valid manifest and minimal `src/lib.rs`
- the scaffold public surfaces are explicit, narrow, and lane-aligned
- the dependency posture is acyclic and does not imply premature ownership decisions
- `handbook-compiler` remains the temporary implementation center
- no major module moves, caller rewires, or compiler retirement leaked into the slice
