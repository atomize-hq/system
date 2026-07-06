# Tasks: Handbook Engine Extraction Phase 4 Slice 1 (Slice 4.1) - Workspace And Crate Scaffold

Plan reference: [handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md](./handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md)

## Prerequisite: the scaffold posture stays narrow and temporary

Slice 4.1 creates real crate homes, not real module migration. It must add workspace members and minimal compile-through public surfaces while keeping `handbook-compiler` as the temporary implementation center and deferring direct caller rewires.

- Slice 4.1 must not widen into Phase 4.2 engine moves, Phase 4.3 pipeline moves, Phase 4.4 flow moves, or Phase 4.5 caller rewiring.

## Packet 4.1.1: Workspace Members And Crate Manifests

- [ ] Task: Add the new workspace members and create the initial engine crate scaffold
  - Acceptance: `Cargo.toml` lists `crates/engine` as a workspace member, and `crates/engine` contains a valid `Cargo.toml` plus `src/lib.rs` with a minimal compile-valid library root.
  - Verify: `cargo metadata --no-deps --format-version 1 | python3 -c 'import sys,json; m=json.load(sys.stdin); print("\n".join(sorted(p["name"] for p in m["packages"])))' && cargo check --workspace`
  - Files: `Cargo.toml`, `crates/engine/Cargo.toml`, `crates/engine/src/lib.rs`

- [ ] Task: Create the initial pipeline and flow crate scaffolds with the same minimal manifest/source-root posture
  - Acceptance: `crates/pipeline` and `crates/flow` each contain valid manifests and `src/lib.rs` roots, and the workspace recognizes all three new crates without introducing circular dependency errors.
  - Verify: `rg -n 'crates/(engine|pipeline|flow)' Cargo.toml crates/*/Cargo.toml && cargo check --workspace`
  - Files: `crates/pipeline/Cargo.toml`, `crates/pipeline/src/lib.rs`, `crates/flow/Cargo.toml`, `crates/flow/src/lib.rs`

## Packet 4.1.2: Minimal Public Crate Surfaces And Compile-Through Wiring

- [ ] Task: Define explicit minimal public surfaces for the new crates
  - Acceptance: `handbook-engine`, `handbook-pipeline`, and `handbook-flow` each expose only a narrow explicit surface or contract marker aligned to their future ownership lane, with no wildcard compiler-facade re-exports.
  - Verify: `rg -n 'pub use|contract_version|workspace_contract_version' crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs && cargo check --workspace`
  - Files: `crates/engine/src/lib.rs`, `crates/pipeline/src/lib.rs`, `crates/flow/src/lib.rs`, optionally `crates/compiler/src/lib.rs`

- [ ] Task: Prove the scaffold wiring does not force premature caller rewires or dependency churn
  - Acceptance: the new crates participate cleanly in the workspace dependency graph, `handbook-compiler` remains the temporary implementation center, and any CLI or compiler edits are limited to tiny compile-through fallout rather than a real migration.
  - Verify: `cargo tree -p handbook-engine -e normal && cargo tree -p handbook-pipeline -e normal && cargo tree -p handbook-flow -e normal && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, `crates/flow/Cargo.toml`, optionally `crates/compiler/src/lib.rs`, `crates/cli/Cargo.toml`, `crates/cli/src/main.rs`, `crates/cli/tests/cli_surface.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 4.1 verification wall after both packets land
  - Acceptance: the new crates are real workspace members, their public surfaces remain narrow, the dependency graph stays acyclic, `handbook-compiler` remains the temporary implementation center, and the workspace remains green without adjacent-slice leakage.
  - Verify: `cargo check --workspace && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 4.1 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
