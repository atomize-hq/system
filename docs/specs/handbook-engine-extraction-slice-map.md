# Handbook Engine Extraction Slice Map

## Objective

Turn Phases 1 through 5 from [HANDBOOK_ENGINE_EXTRACTION_PLAN.md](../../HANDBOOK_ENGINE_EXTRACTION_PLAN.md) into a clear Phase -> Slice -> Packet execution model.

This file is the seam map for future spec-driven execution:

- **Phase** = the root-plan migration gate
- **Slice** = the seam-sized unit that gets its own `spec.md` / `plan.md` / `tasks.md` triplet
- **Implementation packet** = the smallest unit that one agent can safely land in one session inside a slice

This file does not replace the root extraction plan. It is the companion decomposition authority for future slice triplets and packet prompts.

## Current Status

- Execution is now fully landed through Phase 5, including Phase 1 Slice 1.5, Phase 2 Slice 2.4, the refreshed Phase 4 Slice 4.5 closeout, and Phase 5 Slice 5.3.
- The extracted workspace shape is live in code: `crates/engine`, `crates/pipeline`, `crates/flow`, and `crates/cli` are the real owner layers, with `crates/compiler` retained only as the reviewed narrow compatibility/support seam.
- The current runtime wedge remains intentionally bounded while pipeline/stage truth comes from declarative catalog inputs and the retained default consumer stays code-owned and validated.
- Phase 6 in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` is now the next authoritative step.
- All Phase 1–5 and Phase 6 slice/packet artifacts have been archived under `docs/specs/archive/`. See `docs/specs/archive/README.md` for the archive index. Historical file paths referenced below should be resolved relative to the archive subdirectories.

## Authority And Assumptions

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` remains the phase-order and migration-gate authority.
- This file is the execution-decomposition authority for **Phase -> Slice -> Packet** naming and boundaries.
- Phase 0 stays a tracker/governance phase and should not become its own spec/plan/tasks triplet.
- A **slice**, not a phase or a packet, is the durable `spec.md` / `plan.md` / `tasks.md` unit.
- An **implementation packet** lives inside one slice's `tasks.md` and any packet-specific orchestration prompts.
- Slices are sequential by default. Only parallelize when the slice explicitly says it is safe.
- Packets inside a slice are sequential by default. Only parallelize when the slice explicitly says it is safe.
- Do not start any Phase 4 crate-move slice until Phases 1 through 3 are green in live code.
- Do not start any migration-to-Substrate planning from this map. That stays gated behind Phase 6 in the root plan.

## Hierarchy And Naming Convention

### Phase

Use the phase numbers from the root plan unchanged:

- `Phase 1`
- `Phase 2`
- `Phase 3`
- `Phase 4`
- `Phase 5`

Phases are governance and ordering boundaries. They do **not** get their own slice triplets.

### Slice

A slice is one named seam inside a phase. Each slice gets its own triplet under `docs/specs/`.

Use this file stem:

- `handbook-engine-extraction-phase-<phase>-slice-<slice>-<slug>-spec.md`
- `handbook-engine-extraction-phase-<phase>-slice-<slice>-<slug>-plan.md`
- `handbook-engine-extraction-phase-<phase>-slice-<slice>-<slug>-tasks.md`

Example:

- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`

Human-readable shorthand:

- `Slice 1.1: Layout Contract And Inventory`
- `Slice 2.3: Template And Library Resolver Boundary`

### Implementation Packet

A packet is the one-session execution unit inside a slice. It does **not** get its own spec/plan/tasks triplet.

Use this packet label format inside the slice tasks doc and any packet-specific prompts:

- `Packet <phase>.<slice>.<packet>: <title>`

Example:

- `Packet 1.1.1: Layout Type Family And Ownership Boundary`
- `Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze`

Packet rules:

- one packet = one agent-safe landing session
- one packet belongs to exactly one slice
- packet prompts should be one prompt per packet, not one prompt per phase or per multi-slice bundle
- if a slice later needs another packet, append a new packet number inside the same slice; do not renumber older slices

## Slice And Packet Sizing Rules

### Slice sizing rule

A slice is small enough for its own triplet only if all of the following are true:

- it has one dominant architectural seam
- it has one coherent acceptance story
- it can name one primary verification wall, even if packets use smaller rails inside that wall
- it can be described without depending on unrelated future crate or CLI decisions

### Packet sizing rule

An implementation packet is small enough only if all of the following are true:

- one agent can safely land it in one session
- it has one dominant code or doc seam
- it can name one narrow verifier wall or focused test rail
- it can stop cleanly without partially forcing the next packet to land too
- it does not require crossing multiple architectural boundaries at once

If a packet fails any of those rules, split the packet before execution.

## Phase Decomposition Summary

| Phase | Slice count | Packet count | Notes |
| --- | ---: | ---: | --- |
| Phase 1 | 5 | 10 | Layout and storage parameterization plus Slice 1.5 closeout |
| Phase 2 | 4 | 11 | Target contracts and resolver parameterization plus Slice 2.4 closeout |
| Phase 3 | 4 | 8 | In-place engine-vs-product cleanup |
| Phase 4 | 5 | 12 | Real crate-boundary creation plus refreshed Slice 4.5 closeout |
| Phase 5 | 3 | 6 | CLI thinning |

## Phase 1 Slice Map

Phase 1 in the root plan is too broad for one triplet because it mixes layout contract design, inventory, canonical/setup adoption, stateful storage adoption, and authoring-root cleanup.

### Slice 1.1: Layout Contract And Inventory

- **Triplet stem:** `handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory`
- **Seam:** freeze the layout type family and the complete reusable-internal inventory of `.handbook/**` and `.handbook/state/**` assumptions before migrations begin
- **Primary files:** `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/route_state.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`, `crates/compiler/src/setup.rs`, `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/environment_inventory.rs`
- **Slice verifier:** `rg -n "\\.handbook|\\.handbook/state" crates/compiler/src crates/cli/src`
- **Implementation packets:**
  1. `Packet 1.1.1: Layout Type Family And Ownership Boundary`
  2. `Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze`
- **Out of scope:** moving all callers to the new layout seam

### Slice 1.2: Canonical And Setup Layout Adoption

- **Triplet stem:** `handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout`
- **Seam:** move canonical artifact root ownership and setup root establishment behind the approved layout seam
- **Primary files:** `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/setup.rs`
- **Slice verifier:** `cargo test -p handbook-compiler canonical_artifacts && cargo test -p handbook-compiler --test setup`
- **Implementation packets:**
  1. `Packet 1.2.1: Canonical Artifact Root Adoption`
  2. `Packet 1.2.2: Setup Bootstrap Root Adoption`
- **Dependency:** Slice 1.1

### Slice 1.3: Stateful Storage Layout Adoption

- **Triplet stem:** `handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout`
- **Seam:** move route-state, provenance, capture, and handoff storage ownership behind the layout seam without widening into CLI wording or target-parameterization work
- **Primary files:** `crates/compiler/src/route_state.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`
- **Slice verifier:** `cargo test -p handbook-compiler --test pipeline_state_store && cargo test -p handbook-compiler --test pipeline_route_resolution && cargo test -p handbook-compiler --test pipeline_capture && cargo test -p handbook-compiler --test pipeline_handoff`
- **Implementation packets:**
  1. `Packet 1.3.1: Route-State And Stage-Provenance Layout Adoption`
  2. `Packet 1.3.2: Capture And Handoff Layout Adoption`
- **Dependency:** Slice 1.2

### Slice 1.4: Authoring Layout Adoption

- **Triplet stem:** `handbook-engine-extraction-phase-1-slice-4-authoring-layout`
- **Seam:** move authoring canonical roots and lock-file paths behind the approved layout seam while keeping product wording and prompts unchanged
- **Primary files:** `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/environment_inventory.rs`
- **Slice verifier:** `cargo test -p handbook-compiler --test author`
- **Implementation packets:**
  1. `Packet 1.4.1: Authoring Roots And Lock Paths Adoption`
- **Dependency:** Slice 1.2

### Slice 1.5: Layout Parameterization Closeout

- **Triplet stem:** `handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout`
- **Seam:** finish the Phase 1 requirement that reusable internals are parameterized instead of merely centralized behind layout helper types
- **Primary files:** `crates/engine/src/canonical_paths.rs`, `crates/pipeline/src/layout.rs`, `crates/flow/src/resolver.rs`, adjacent reusable callers that still depend on product-default layout ownership
- **Slice verifier:** `cargo test -p handbook-engine --test canonical_artifacts_ingest && cargo test -p handbook-engine --test baseline_validation && cargo test -p handbook-pipeline --test pipeline_capture && cargo test -p handbook-pipeline --test pipeline_handoff && cargo test -p handbook-flow --test resolver_core`
- **Implementation packets:**
  1. `Packet 1.5.1: Parameterized Canonical Layout Contract`
  2. `Packet 1.5.2: Parameterized Pipeline Storage Layout Adoption`
  3. `Packet 1.5.3: Remaining Reusable Caller Adoption And Residual-Literal Closeout`
- **Dependency:** Slice 1.4

## Phase 2 Slice Map

Phase 2 in the root plan is too broad for one triplet because it mixes supported-target definition, compile/capture/handoff adoption, provenance behavior, and template/library resolution.

### Slice 2.1: Supported Target Contract And Registry

- **Triplet stem:** `handbook-engine-extraction-phase-2-slice-1-supported-target-contract`
- **Seam:** define the typed supported-target contract and the one registry/lookup owner for pipeline, stage, and consumer identities
- **Primary files:** `crates/compiler/src/pipeline.rs`, `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`
- **Slice verifier:** `rg -n "pipeline\\.foundation_inputs|stage\\.10_feature_spec|feature-slice-decomposer" crates/compiler/src crates/cli/src`
- **Implementation packets:**
  1. `Packet 2.1.1: Typed Target And Consumer Contract`
  2. `Packet 2.1.2: Target Registry Lookup And Validation Owner`
- **Out of scope:** switching every caller to the new contract in the same slice

### Slice 2.2: Compile, Capture, And Handoff Target Adoption

- **Triplet stem:** `handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption`
- **Seam:** migrate compile, capture, provenance, and handoff logic onto the supported-target contract while keeping behavior-specific refusal posture local
- **Primary files:** `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`
- **Slice verifier:** `cargo test -p handbook-compiler --test pipeline_compile && cargo test -p handbook-compiler --test pipeline_capture && cargo test -p handbook-compiler --test pipeline_handoff && cargo test -p handbook-cli --test cli_surface`
- **Implementation packets:**
  1. `Packet 2.2.1: Compile Target Adoption`
  2. `Packet 2.2.2: Capture And Provenance Target Adoption`
  3. `Packet 2.2.3: Handoff Target Adoption`
- **Dependency:** Slice 2.1

### Slice 2.3: Template And Library Resolver Boundary

- **Triplet stem:** `handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver`
- **Seam:** define the typed template/library resolver boundary, shipped-default posture, and validated override rules without widening into new library content
- **Primary files:** `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/environment_inventory.rs`, possibly `crates/compiler/src/pipeline.rs`
- **Slice verifier:** `cargo test -p handbook-compiler --test author && cargo test -p handbook-compiler --test pipeline_catalog`
- **Implementation packets:**
  1. `Packet 2.3.1: Typed Resolver Contract And Shipped-Default Posture`
  2. `Packet 2.3.2: Validated Override And Selection Rules`
- **Dependency:** Slice 2.1

### Slice 2.4: Orchestration Target Parameterization Closeout

- **Triplet stem:** `handbook-engine-extraction-phase-2-slice-4-orchestration-target-parameterization-closeout`
- **Seam:** close the remaining Phase 2 gap so compile/capture/handoff/provenance behavior is driven by declared targets rather than scattered singleton runtime ownership
- **Primary files:** `core/pipelines/**`, `core/stages/**`, `crates/pipeline/src/pipeline.rs`, `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/pipeline_handoff.rs`, `crates/cli/src/pipeline_help.rs`
- **Slice verifier:** `cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-pipeline --test pipeline_compile && cargo test -p handbook-pipeline --test pipeline_capture && cargo test -p handbook-pipeline --test pipeline_handoff && cargo test -p handbook-cli --test cli_surface`
- **Implementation packets:**
  1. `Packet 2.4.1: Catalog-Backed Pipeline And Stage Target Closeout`
  2. `Packet 2.4.2: Bounded Default-Consumer Ownership`
  3. `Packet 2.4.3: CLI Help, Recovery, And Producer-Command Alignment`
  4. `Packet 2.4.4: Final Closeout Proof`
- **Dependency:** Slice 2.3

## Phase 3 Slice Map

Phase 3 in the root plan is too broad for one triplet because it spans several mixed modules whose deterministic engine logic and product-shell logic should be separated at different seams.

### Slice 3.1: Charter Deterministic Core Split

- **Triplet stem:** `handbook-engine-extraction-phase-3-slice-1-charter-core-split`
- **Seam:** separate deterministic charter parse/render/validate behavior from guided synthesis and product-shell behavior
- **Primary files:** `crates/compiler/src/author/charter.rs`
- **Slice verifier:** `cargo test -p handbook-compiler --test author`
- **Implementation packets:**
  1. `Packet 3.1.1: Charter Parse Render Validate Core Extraction`
  2. `Packet 3.1.2: Charter Synthesis And Shell Adapter Cleanup`
- **Out of scope:** changing project-context or environment-inventory in the same slice

### Slice 3.2: Project-Context Deterministic Core Split

- **Triplet stem:** `handbook-engine-extraction-phase-3-slice-2-project-context-core-split`
- **Seam:** separate deterministic project-context parsing and validation from guided synthesis and product recovery wording
- **Primary files:** `crates/compiler/src/author/project_context.rs`
- **Slice verifier:** `cargo test -p handbook-compiler --test author`
- **Implementation packets:**
  1. `Packet 3.2.1: Project-Context Deterministic Model Split`
  2. `Packet 3.2.2: Project-Context Recovery Wording And Shell Cleanup`
- **Dependency:** Slice 3.1

### Slice 3.3: Environment-Inventory Deterministic Core Split

- **Triplet stem:** `handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split`
- **Seam:** separate deterministic environment-inventory modeling from synthesis prompts, product references, and shell wording
- **Primary files:** `crates/compiler/src/author/environment_inventory.rs`
- **Slice verifier:** `cargo test -p handbook-compiler --test author`
- **Implementation packets:**
  1. `Packet 3.3.1: Environment-Inventory Deterministic Model Split`
  2. `Packet 3.3.2: Environment-Inventory Prompt And Product Cleanup`
- **Dependency:** Slice 3.2

### Slice 3.4: Setup, Doctor, And Refusal Shell Split

- **Triplet stem:** `handbook-engine-extraction-phase-3-slice-4-shell-wording-split`
- **Seam:** pull product-shell recovery wording and operator-facing copy away from reusable readiness, setup, doctor, and refusal logic where that split is already clear
- **Primary files:** `crates/compiler/src/setup.rs`, `crates/compiler/src/doctor.rs`, `crates/compiler/src/refusal.rs`, possibly `crates/compiler/src/rendering/shared.rs`
- **Slice verifier:** `cargo test -p handbook-compiler --test setup && cargo test -p handbook-compiler --test doctor && cargo test -p handbook-cli --test cli_surface`
- **Implementation packets:**
  1. `Packet 3.4.1: Setup And Readiness Shell Separation`
  2. `Packet 3.4.2: Doctor And Refusal Operator Wording Separation`
- **Dependency:** Slices 3.1 through 3.3

## Phase 4 Slice Map

Phase 4 in the root plan is too broad for one triplet because it mixes workspace wiring, crate creation, file moves, caller rewires, and compiler-facade retirement.

### Slice 4.1: Workspace And Crate Scaffold

- **Triplet stem:** `handbook-engine-extraction-phase-4-slice-1-crate-scaffold`
- **Seam:** add `crates/engine`, `crates/pipeline`, and `crates/flow`, wire Cargo metadata, and establish minimal compile-time crate boundaries before moving major logic
- **Primary files:** `Cargo.toml`, `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, `crates/flow/Cargo.toml`
- **Slice verifier:** `cargo check --workspace`
- **Implementation packets:**
  1. `Packet 4.1.1: Workspace Members And Crate Manifests`
  2. `Packet 4.1.2: Minimal Public Crate Surfaces And Compile-Through Wiring`
- **Out of scope:** moving major implementation modules in the same slice

### Slice 4.2: Engine Crate Migration

- **Triplet stem:** `handbook-engine-extraction-phase-4-slice-2-engine-migration`
- **Seam:** move engine-safe modules behind `handbook-engine` and prove the new crate owns canonical artifact, manifest, freshness, baseline validation, and approved authoring core surfaces
- **Primary files:** `crates/engine/**`, `crates/compiler/src/artifact_manifest.rs`, `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/freshness.rs`, `crates/compiler/src/baseline_validation.rs`, selected `crates/compiler/src/author/**`
- **Slice verifier:** `cargo test -p handbook-engine && cargo test -p handbook-cli --test cli_surface`
- **Implementation packets:**
  1. `Packet 4.2.1: Canonical Artifact Manifest Freshness And Baseline Migration`
  2. `Packet 4.2.2: Approved Authoring Core Migration`
- **Dependency:** Phase 3 complete and Slice 4.1

### Slice 4.3: Pipeline Crate Migration

- **Triplet stem:** `handbook-engine-extraction-phase-4-slice-3-pipeline-migration`
- **Seam:** move declarative pipeline loading, route resolution, route-state persistence, compile/capture/handoff mechanics, and approved setup helpers behind `handbook-pipeline`
- **Primary files:** `crates/pipeline/**`, `crates/compiler/src/pipeline.rs`, `crates/compiler/src/pipeline_route.rs`, `crates/compiler/src/route_state.rs`, `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/stage_10_feature_spec_provenance.rs`, `crates/compiler/src/setup.rs`
- **Slice verifier:** `cargo test -p handbook-pipeline && cargo test -p handbook-cli --test cli_surface`
- **Implementation packets:**
  1. `Packet 4.3.1: Pipeline Loading Route And Route-State Migration`
  2. `Packet 4.3.2: Compile Capture And Handoff Migration`
  3. `Packet 4.3.3: Setup Helper And Provenance Alignment`
- **Dependency:** Phase 2 complete and Slice 4.2

### Slice 4.4: Flow Crate Migration

- **Triplet stem:** `handbook-engine-extraction-phase-4-slice-4-flow-migration`
- **Seam:** move `resolver`, `packet_result`, and `budget` behind `handbook-flow` without forcing `rendering`, `refusal`, or `error` into premature ownership decisions
- **Primary files:** `crates/flow/**`, `crates/compiler/src/resolver.rs`, `crates/compiler/src/packet_result.rs`, `crates/compiler/src/budget.rs`
- **Slice verifier:** `cargo test -p handbook-flow && cargo test -p handbook-cli --test cli_surface`
- **Implementation packets:**
  1. `Packet 4.4.1: Resolver Packet-Result And Budget Migration`
- **Dependency:** Slice 4.3

### Slice 4.5: Caller Rewires And Compiler Narrowing

- **Triplet stem:** `handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing`
- **Seam:** move callers directly to `handbook-engine`, `handbook-pipeline`, and `handbook-flow`, then intentionally narrow or retire `crates/compiler` as an implementation center
- **Primary files:** `crates/compiler/src/lib.rs`, `crates/cli/src/main.rs`, crate `Cargo.toml` files, any remaining direct `handbook_compiler::*` callers
- **Slice verifier:** `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
- **Implementation packets:**
  1. `Packet 4.5.1: Residual Caller Inventory And Boundary Freeze`
  2. `Packet 4.5.2: Stale Caller Rewires To Real Owner Crates`
  3. `Packet 4.5.3: Compiler Narrow Boundary Truth`
  4. `Packet 4.5.4: Final Closeout Proof`
- **Dependency:** Slices 4.2 through 4.4

## Phase 5 Slice Map

Phase 5 in the root plan is too broad for one triplet because `crates/cli/src/main.rs` still combines command parsing, prompting, rendering, help text, fixture support, and compiler wiring.

### Slice 5.1: CLI Skeleton And Author Setup Extraction

- **Triplet stem:** `handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup`
- **Seam:** introduce CLI helper modules and move `setup` and `author` command-family orchestration out of `main.rs` first
- **Primary files:** `crates/cli/src/main.rs`, new `crates/cli/src/*` helper modules
- **Slice verifier:** `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
- **Implementation packets:**
  1. `Packet 5.1.1: CLI Module Skeleton And Shared Command Helper Wiring`
  2. `Packet 5.1.2: Author And Setup Command-Family Extraction`
- **Out of scope:** pipeline, inspect, and doctor extraction in the same slice

### Slice 5.2: Pipeline, Inspect, And Doctor Extraction

- **Triplet stem:** `handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell`
- **Seam:** move pipeline, inspect, and doctor command-family orchestration out of `main.rs` while preserving command surface behavior
- **Primary files:** `crates/cli/src/main.rs`, new `crates/cli/src/*` helper modules
- **Slice verifier:** `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
- **Implementation packets:**
  1. `Packet 5.2.1: Pipeline Command-Family Extraction`
  2. `Packet 5.2.2: Inspect And Doctor Command-Family Extraction`
- **Dependency:** Slice 5.1

### Slice 5.3: Prompting, Rendering, Help, And Exit-Code Closeout

- **Triplet stem:** `handbook-engine-extraction-phase-5-slice-3-cli-shell-closeout`
- **Seam:** isolate prompting helpers, wording/rendering helpers, help text, and exit-code decisions so `main.rs` becomes a thin product entrypoint instead of the integration bucket
- **Primary files:** `crates/cli/src/main.rs`, new `crates/cli/src/*` helper modules, possibly `crates/compiler/src/rendering/**` only if a Phase 3 decision already proved that split
- **Slice verifier:** `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
- **Implementation packets:**
  1. `Packet 5.3.1: Prompting Rendering And Help Helper Extraction`
  2. `Packet 5.3.2: Exit-Code And Final Shell Closeout`
- **Dependency:** Slice 5.2

## Recommended Execution Order

Use this order unless live repo truth forces a narrower repair slice. Inside each slice, land packets in numeric order.

1. Slice 1.1
2. Slice 1.2
3. Slice 1.3
4. Slice 1.4
5. Slice 1.5
6. Slice 2.1
7. Slice 2.2
8. Slice 2.3
9. Slice 2.4
10. Slice 3.1
11. Slice 3.2
12. Slice 3.3
13. Slice 3.4
14. Slice 4.1
15. Slice 4.2
16. Slice 4.3
17. Slice 4.4
18. Slice 4.5
19. Slice 5.1
20. Slice 5.2
21. Slice 5.3

That execution order is now fully landed through Slice 5.3; the next step is Phase 6 in the root plan, not another Phase 1–5 slice.

## Start Conditions For Writing The First Triplet

The first triplet to write should be:

- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`

The first tasks doc should then start with:

- `Packet 1.1.1: Layout Type Family And Ownership Boundary`
- `Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze`

Reason:

- every later slice depends on a frozen layout contract and a trustworthy inventory
- it is the smallest seam that reduces ambiguity instead of spreading it into later migrations
- it establishes the packeting model the rest of the extraction work should follow
