# Handbook Engine Extraction Slice Map

## Objective

Turn Phases 1 through 5 from [HANDBOOK_ENGINE_EXTRACTION_PLAN.md](../../HANDBOOK_ENGINE_EXTRACTION_PLAN.md) into packet-sized boundaries that are safe to represent as separate `spec.md` / `plan.md` / `tasks.md` triplets.

This file does not replace the root extraction plan. It is the companion packet map for future spec-driven execution.

## Authority And Assumptions

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` remains the phase-order and migration-gate authority.
- Phase 0 stays a tracker/governance phase and should not become its own spec/plan/tasks triplet.
- Each packet below is intended to be one dominant seam with one dominant verifier wall.
- Packets are sequential by default. Only parallelize when the packet explicitly says it is safe.
- Do not start any Phase 4 crate-move packet until Phases 1 through 3 are green in live code.
- Do not start any migration-to-Substrate planning from this map. That stays gated behind Phase 6 in the root plan.

## Naming Convention

For each packet below, create a triplet under `docs/specs/` using this stem:

- `handbook-engine-extraction-phase-<phase>-packet-<packet>-<slug>-spec.md`
- `handbook-engine-extraction-phase-<phase>-packet-<packet>-<slug>-plan.md`
- `handbook-engine-extraction-phase-<phase>-packet-<packet>-<slug>-tasks.md`

Example:

- `docs/specs/handbook-engine-extraction-phase-1-packet-1-layout-contract-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-packet-1-layout-contract-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-packet-1-layout-contract-tasks.md`

## Packet Sizing Rule

A packet is small enough for its own triplet only if all of the following are true:

- it has one primary contract or migration seam
- it can name one narrow verification wall
- it does not require changing more than one architectural boundary at once
- it can stop cleanly without forcing later packets to be partially landed

## Phase Verdicts

- Phase 1: decompose into five packets before writing triplets
- Phase 2: decompose into five packets before writing triplets
- Phase 3: decompose into four packets before writing triplets
- Phase 4: decompose into five packets before writing triplets
- Phase 5: decompose into three packets before writing triplets

## Phase 1 Packet Map

Phase 1 in the root plan is too broad for one triplet because it combines contract design, inventory, canonical artifact ownership, runtime-state ownership, capture/handoff storage, and authoring-path cleanup.

### Packet 1: Layout Contract And Storage Inventory

- Triplet stem: `handbook-engine-extraction-phase-1-packet-1-layout-contract`
- Scope: freeze the layout type family, ownership boundaries, and a complete reusable-internal inventory of `.handbook/**` and `.handbook/state/**` assumptions before migrations begin
- Primary files: `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/route_state.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`, `crates/compiler/src/setup.rs`, `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/environment_inventory.rs`
- Verify: `rg -n "\\.handbook|\\.handbook/state" crates/compiler/src crates/cli/src`
- Out of scope: moving any caller to the new layout seam

### Packet 2: Canonical Artifact And Setup Layout Adoption

- Triplet stem: `handbook-engine-extraction-phase-1-packet-2-canonical-layout`
- Scope: move canonical artifact root ownership and setup root establishment behind the approved layout seam
- Primary files: `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/setup.rs`
- Verify: `cargo test -p handbook-compiler canonical_artifacts && cargo test -p handbook-compiler --test setup`
- Dependency: Packet 1

### Packet 3: Route-State And Stage-Provenance Layout Adoption

- Triplet stem: `handbook-engine-extraction-phase-1-packet-3-route-state-layout`
- Scope: move route-state storage roots and stage-capture provenance paths behind the approved layout seam without widening into capture/handoff behavior
- Primary files: `crates/compiler/src/route_state.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`
- Verify: `cargo test -p handbook-compiler --test pipeline_state_store && cargo test -p handbook-compiler --test pipeline_route_resolution`
- Dependency: Packet 2

### Packet 4: Capture And Handoff Layout Adoption

- Triplet stem: `handbook-engine-extraction-phase-1-packet-4-capture-handoff-layout`
- Scope: move pipeline capture and handoff storage ownership behind the layout seam after route-state semantics are stable
- Primary files: `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`
- Verify: `cargo test -p handbook-compiler --test pipeline_capture && cargo test -p handbook-compiler --test pipeline_handoff`
- Dependency: Packet 3

### Packet 5: Authoring Roots And Lock Paths

- Triplet stem: `handbook-engine-extraction-phase-1-packet-5-authoring-layout`
- Scope: move authoring canonical roots and lock-file paths behind the layout seam while keeping product wording and prompts unchanged
- Primary files: `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/environment_inventory.rs`
- Verify: `cargo test -p handbook-compiler --test author`
- Dependency: Packet 2

## Phase 2 Packet Map

Phase 2 in the root plan is too broad for one triplet because it combines supported-target parameterization, compile/capture/handoff adoption, provenance behavior, and template/library resolution.

### Packet 1: Supported Target Contract

- Triplet stem: `handbook-engine-extraction-phase-2-packet-1-target-contract`
- Scope: define the typed supported-target contract and one registry/lookup owner for pipeline, stage, and consumer identities
- Primary files: `crates/compiler/src/pipeline.rs`, `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`
- Verify: `rg -n "pipeline\\.foundation_inputs|stage\\.10_feature_spec|feature-slice-decomposer" crates/compiler/src crates/cli/src`
- Out of scope: switching all callers to the new contract in the same packet

### Packet 2: Compile Target Adoption

- Triplet stem: `handbook-engine-extraction-phase-2-packet-2-compile-targets`
- Scope: migrate compile logic to consume the supported-target contract rather than local hardcoded pipeline/stage identities
- Primary files: `crates/compiler/src/pipeline_compile.rs`
- Verify: `cargo test -p handbook-compiler --test pipeline_compile && cargo test -p handbook-cli --test cli_surface`
- Dependency: Packet 1

### Packet 3: Capture And Provenance Target Adoption

- Triplet stem: `handbook-engine-extraction-phase-2-packet-3-capture-targets`
- Scope: migrate capture and stage-provenance logic to the supported-target contract while keeping capture-specific refusal posture local
- Primary files: `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`
- Verify: `cargo test -p handbook-compiler --test pipeline_capture`
- Dependency: Packet 2

### Packet 4: Handoff Target Adoption

- Triplet stem: `handbook-engine-extraction-phase-2-packet-4-handoff-targets`
- Scope: migrate handoff emission and validation onto the supported-target contract, including consumer identity ownership
- Primary files: `crates/compiler/src/pipeline_handoff.rs`
- Verify: `cargo test -p handbook-compiler --test pipeline_handoff`
- Dependency: Packet 3

### Packet 5: Template And Library Resolver Boundary

- Triplet stem: `handbook-engine-extraction-phase-2-packet-5-template-resolver`
- Scope: define the typed template/library resolver boundary, shipped-default posture, and validated override rules without widening into new library content
- Primary files: `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/environment_inventory.rs`, possibly `crates/compiler/src/pipeline.rs`
- Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-compiler --test pipeline_catalog`
- Dependency: Packet 1

## Phase 3 Packet Map

Phase 3 in the root plan is too broad for one triplet because it spans multiple large mixed modules with different refactor goals.

### Packet 1: Charter Deterministic Core Split

- Triplet stem: `handbook-engine-extraction-phase-3-packet-1-charter-core-split`
- Scope: separate deterministic charter parse/render/validate behavior from guided synthesis and product-shell behavior
- Primary files: `crates/compiler/src/author/charter.rs`
- Verify: `cargo test -p handbook-compiler --test author`
- Out of scope: changing project-context or environment-inventory in the same packet

### Packet 2: Project-Context Deterministic Core Split

- Triplet stem: `handbook-engine-extraction-phase-3-packet-2-project-context-core-split`
- Scope: separate deterministic project-context parsing/validation from guided synthesis and product recovery wording
- Primary files: `crates/compiler/src/author/project_context.rs`
- Verify: `cargo test -p handbook-compiler --test author`
- Dependency: Packet 1

### Packet 3: Environment-Inventory Deterministic Core Split

- Triplet stem: `handbook-engine-extraction-phase-3-packet-3-environment-inventory-core-split`
- Scope: separate deterministic environment-inventory modeling from synthesis prompts, product references, and shell wording
- Primary files: `crates/compiler/src/author/environment_inventory.rs`
- Verify: `cargo test -p handbook-compiler --test author`
- Dependency: Packet 2

### Packet 4: Setup, Doctor, And Refusal Shell Split

- Triplet stem: `handbook-engine-extraction-phase-3-packet-4-shell-wording-split`
- Scope: pull product-shell recovery wording and operator-facing copy away from reusable readiness/setup/refusal logic where that split is already clear
- Primary files: `crates/compiler/src/setup.rs`, `crates/compiler/src/doctor.rs`, `crates/compiler/src/refusal.rs`, possibly `crates/compiler/src/rendering/shared.rs`
- Verify: `cargo test -p handbook-compiler --test setup && cargo test -p handbook-compiler --test doctor && cargo test -p handbook-cli --test cli_surface`
- Dependency: Packets 1 through 3

## Phase 4 Packet Map

Phase 4 in the root plan is too broad for one triplet because it combines workspace wiring, crate creation, file moves, caller rewires, and compiler-facade retirement.

### Packet 1: Workspace And Crate Scaffolding

- Triplet stem: `handbook-engine-extraction-phase-4-packet-1-crate-scaffold`
- Scope: add `crates/engine`, `crates/pipeline`, and `crates/flow`, wire Cargo metadata, and establish minimal compile-time crate boundaries without moving major logic yet
- Primary files: `Cargo.toml`, `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, `crates/flow/Cargo.toml`
- Verify: `cargo check --workspace`
- Out of scope: moving implementation modules behind the new crates in the same packet

### Packet 2: Engine Crate Migration

- Triplet stem: `handbook-engine-extraction-phase-4-packet-2-engine-migration`
- Scope: move engine-safe modules behind `handbook-engine` and prove the new crate owns canonical artifact, manifest, freshness, baseline validation, and approved authoring core surfaces
- Primary files: `crates/engine/**`, `crates/compiler/src/artifact_manifest.rs`, `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/freshness.rs`, `crates/compiler/src/baseline_validation.rs`, selected `crates/compiler/src/author/**`
- Verify: `cargo test -p handbook-engine && cargo test -p handbook-cli --test cli_surface`
- Dependency: Phase 3 complete and Packet 1

### Packet 3: Pipeline Crate Migration

- Triplet stem: `handbook-engine-extraction-phase-4-packet-3-pipeline-migration`
- Scope: move declarative pipeline loading, route resolution, route-state persistence, compile/capture/handoff mechanics, and approved setup helpers behind `handbook-pipeline`
- Primary files: `crates/pipeline/**`, `crates/compiler/src/pipeline.rs`, `crates/compiler/src/pipeline_route.rs`, `crates/compiler/src/route_state.rs`, `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`, `crates/compiler/src/setup.rs`
- Verify: `cargo test -p handbook-pipeline && cargo test -p handbook-cli --test cli_surface`
- Dependency: Phase 2 complete and Packet 2

### Packet 4: Flow Crate Migration

- Triplet stem: `handbook-engine-extraction-phase-4-packet-4-flow-migration`
- Scope: move `resolver`, `packet_result`, and `budget` behind `handbook-flow` without forcing `rendering`, `refusal`, or `error` into premature ownership decisions
- Primary files: `crates/flow/**`, `crates/compiler/src/resolver.rs`, `crates/compiler/src/packet_result.rs`, `crates/compiler/src/budget.rs`
- Verify: `cargo test -p handbook-flow && cargo test -p handbook-cli --test cli_surface`
- Dependency: Packet 3

### Packet 5: Caller Rewires And Compiler Narrowing

- Triplet stem: `handbook-engine-extraction-phase-4-packet-5-caller-rewire`
- Scope: move callers directly to `handbook-engine`, `handbook-pipeline`, and `handbook-flow`, then intentionally narrow or retire `crates/compiler` as an implementation center
- Primary files: `crates/compiler/src/lib.rs`, `crates/cli/src/main.rs`, crate `Cargo.toml` files, any remaining direct `handbook_compiler::*` callers
- Verify: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
- Dependency: Packets 2 through 4

## Phase 5 Packet Map

Phase 5 in the root plan is too broad for one triplet because `crates/cli/src/main.rs` still combines command parsing, prompting, rendering, help text, fixture support, and compiler wiring.

### Packet 1: CLI Module Skeleton And Author Setup Extraction

- Triplet stem: `handbook-engine-extraction-phase-5-packet-1-cli-skeleton`
- Scope: introduce CLI helper modules and move `setup` and `author` command-family orchestration out of `main.rs` first
- Primary files: `crates/cli/src/main.rs`, new `crates/cli/src/*` helper modules
- Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
- Out of scope: pipeline, inspect, and doctor command-family extraction in the same packet

### Packet 2: Pipeline, Inspect, And Doctor Extraction

- Triplet stem: `handbook-engine-extraction-phase-5-packet-2-cli-pipeline-shell`
- Scope: move pipeline, inspect, and doctor command-family orchestration out of `main.rs` while preserving command surface behavior
- Primary files: `crates/cli/src/main.rs`, new `crates/cli/src/*` helper modules
- Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
- Dependency: Packet 1

### Packet 3: Prompting, Rendering, Help, And Exit-Code Closeout

- Triplet stem: `handbook-engine-extraction-phase-5-packet-3-cli-closeout`
- Scope: isolate prompting helpers, wording/rendering helpers, and exit-code decisions so `main.rs` becomes a thin product entrypoint instead of the integration bucket
- Primary files: `crates/cli/src/main.rs`, new `crates/cli/src/*` helper modules, possibly `crates/compiler/src/rendering/**` only if a Phase 3 decision already proved that split
- Verify: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
- Dependency: Packet 2

## Recommended Execution Order

Use this order unless live repo truth forces a narrower repair packet:

1. Phase 1 Packet 1
2. Phase 1 Packet 2
3. Phase 1 Packet 3
4. Phase 1 Packet 4
5. Phase 1 Packet 5
6. Phase 2 Packet 1
7. Phase 2 Packet 2
8. Phase 2 Packet 3
9. Phase 2 Packet 4
10. Phase 2 Packet 5
11. Phase 3 Packet 1
12. Phase 3 Packet 2
13. Phase 3 Packet 3
14. Phase 3 Packet 4
15. Phase 4 Packet 1
16. Phase 4 Packet 2
17. Phase 4 Packet 3
18. Phase 4 Packet 4
19. Phase 4 Packet 5
20. Phase 5 Packet 1
21. Phase 5 Packet 2
22. Phase 5 Packet 3

## Start Conditions For Writing The First Triplet

The first triplet to write should be:

- `docs/specs/handbook-engine-extraction-phase-1-packet-1-layout-contract-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-packet-1-layout-contract-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-packet-1-layout-contract-tasks.md`

Reason:

- every later packet depends on a frozen layout contract and a trustworthy inventory
- it is the smallest packet that reduces ambiguity instead of spreading it into later migrations
