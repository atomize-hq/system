# Tasks: Handbook Engine Extraction Phase 4 Slice 2 (Slice 4.2) - Engine Crate Migration

Plan reference: [handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md](./handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md)

## Prerequisite: engine ownership must become real, not compile-through theater

Slice 4.2 is the first true engine crate ownership landing. It must make `handbook-engine` the implementation home for approved engine-safe logic while keeping `handbook-compiler` only as a temporary compatibility facade and avoiding direct CLI rewires.

- Slice 4.2 must not widen into Phase 4.3 pipeline migration, Phase 4.4 flow migration, or Phase 4.5 caller rewiring.

## Packet 4.2.1: Canonical Artifact Manifest Freshness And Baseline Migration

- [ ] Task: Flip the engine/compiler dependency posture to support real engine ownership
  - Acceptance: `handbook-engine` no longer depends on `handbook-compiler` as the real owner for Slice 4.2 surfaces, the resulting package graph is acyclic, and `handbook-compiler` can act as a temporary compatibility facade over engine-owned APIs.
  - Verify: `cargo tree -p handbook-engine -e normal && cargo tree -p handbook-compiler -e normal && cargo check --workspace`
  - Files: `crates/engine/Cargo.toml`, `crates/compiler/Cargo.toml`, optionally `Cargo.toml`

- [ ] Task: Move the canonical artifact stack into `handbook-engine`
  - Acceptance: `artifact_manifest`, `canonical_artifacts`, `freshness`, and `baseline_validation` have one real implementation owner under `crates/engine/src/**`, and any remaining compiler surfaces are explicit compatibility re-exports or thin adapters rather than duplicate implementation bodies.
  - Verify: `rg -n 'artifact_manifest|canonical_artifacts|freshness|baseline_validation' crates/engine crates/compiler/src && cargo test -p handbook-engine`
  - Files: `crates/engine/src/lib.rs`, `crates/engine/src/**`, `crates/compiler/src/lib.rs`, `crates/compiler/src/artifact_manifest.rs`, `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/freshness.rs`, `crates/compiler/src/baseline_validation.rs`

- [ ] Task: Move or recreate regression coverage for the migrated canonical stack in the engine crate
  - Acceptance: `handbook-engine` has package-local regression coverage for the canonical artifact / manifest / freshness / baseline family, and the engine package can prove those surfaces without depending on compiler-owned tests as the primary contract.
  - Verify: `cargo test -p handbook-engine`
  - Files: `crates/engine/tests/**`, optionally `crates/compiler/tests/artifact_manifest_interface.rs`, `crates/compiler/tests/canonical_artifacts_ingest.rs`, `crates/compiler/tests/freshness_computation.rs`

## Packet 4.2.2: Approved Authoring Core Migration

- [ ] Task: Move deterministic authoring-core modules into `handbook-engine`
  - Acceptance: the approved deterministic authoring cores (`charter_core`, `project_context_core`, and `environment_inventory_core`) live under `crates/engine/src/author/**` or an equivalent engine-owned namespace, and their engine-safe public types/functions are exported from `handbook-engine` without pulling shell/runtime/template/write behavior into the crate.
  - Verify: `rg -n 'charter_core|project_context_core|environment_inventory_core' crates/engine crates/compiler/src/author && cargo test -p handbook-engine`
  - Files: `crates/engine/src/lib.rs`, `crates/engine/src/author/**`, `crates/compiler/src/author/charter_core.rs`, `crates/compiler/src/author/project_context_core.rs`, `crates/compiler/src/author/environment_inventory_core.rs`

- [ ] Task: Keep compiler author facades thin and shell-owned while delegating core ownership to `handbook-engine`
  - Acceptance: `crates/compiler/src/author/*.rs` retain only compatibility re-exports, thin adapters, or shell/runtime orchestration, while template-library selection, Codex transport, canonical preflight/write flow, lock handling, refusal wording, and ambient timestamp/env resolution remain outside `handbook-engine`.
  - Verify: `rg -n 'use handbook_engine|pub use handbook_engine|handbook_engine::' crates/compiler/src/author crates/compiler/src/lib.rs && cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/lib.rs`, `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/src/author/*_shell.rs`

- [ ] Task: Prove the migration preserves public CLI behavior without direct caller rewires
  - Acceptance: `handbook-cli` remains behavior-stable through the compiler compatibility facade, and no broad direct dependency shift to `handbook-engine` is required for Slice 4.2.
  - Verify: `cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/cli/Cargo.toml`, `crates/cli/src/main.rs`, optionally `crates/compiler/Cargo.toml`, `crates/compiler/src/lib.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 4.2 verification wall after both packets land
  - Acceptance: `handbook-engine` is the real implementation owner for the approved Slice 4.2 surfaces, the dependency graph is acyclic, engine-owned tests pass, CLI behavior stays stable, and no adjacent-slice leakage appears.
  - Verify: `cargo check --workspace && cargo test -p handbook-engine && cargo test -p handbook-cli --test cli_surface`
  - Files: verification only

## Human Review Gate

Stop after the Slice 4.2 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
