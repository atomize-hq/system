# HANDBOOK ENGINE EXTRACTION PLAN

## Status

This file is the root tracking artifact for the handbook engine extraction work.

It exists to track:

- the intended crate split
- the ordered implementation phases
- the concrete file/module seams
- the progress state of each phase
- the verification gates required before migration into Substrate

Current status:

- research and seam mapping completed
- no extraction implementation started yet
- migration into Substrate is explicitly blocked until the prerequisite phases below are complete

## Objective

Land a safe structural split of the current Rust workspace into three clearer layers:

- `handbook-engine`: canonical truth and validation core
- `handbook-pipeline`: reusable pipeline, state, compile, capture, and handoff mechanics
- `handbook-flow`: higher-level application and flow composition that does not belong in engine or pipeline
- `handbook-cli`: thin product shell

Success means all of the following are true:

- reusable engine code is separated from handbook-product shell behavior
- reusable pipeline code is separated from handbook-product shell behavior
- path/storage assumptions are parameterized instead of being baked into reusable internals
- orchestration targets are declared rather than hardcoded to one product wedge
- the engine portion is clean enough to move into Substrate without carrying handbook-product assumptions with it

## Non-Goals

- Do not migrate code into Substrate yet.
- Do not make `substrate-context` become handbook.
- Do not discard the pipeline model.
- Do not introduce compatibility aliases as a long-term architecture substitute.
- Do not require the existing CLI to keep working through a compatibility phase while the split lands.
- Do not widen this plan into unrelated CLI/product redesign work.

## Architectural Target

### Intended steady state

- `handbook-engine`
  - canonical artifact model
  - canonical truth parsing/rendering/validation
  - freshness and manifest generation
  - baseline validation and readiness computation
  - typed engine-facing contracts for authored truth
- `handbook-pipeline`
  - declarative pipeline loading
  - route resolution
  - route-state persistence
  - compile/capture/handoff mechanics
  - trusted session and provenance handling
  - reusable orchestration target definitions
  - reusable layout/state-root handling
- `handbook-flow`
  - higher-level flow composition
  - resolver-style application behavior
  - packet/proof composition that does not cleanly belong in engine or pipeline
  - transitional home for middle-layer logic while the split settles
- `handbook-cli`
  - argument parsing
  - interactive prompting
  - product wording and help text
  - operator-facing rendering and exit-code decisions

### Migration rule

Do not move any crate into Substrate until:

1. storage/layout assumptions are parameterized
2. orchestration targets are parameterized
3. the engine/pipeline/flow split is real in this repo
4. the CLI is no longer acting as an oversized integration bucket

## Root Problems To Solve First

### 1. Path and storage assumptions are still embedded

These assumptions currently appear throughout reusable internals:

- `.handbook/**`
- `.handbook/state/**`
- canonical artifact locations
- route-basis persistence locations
- stage-capture provenance locations
- handoff bundle layout

This must be replaced by a typed layout boundary rather than scattered constants.

### 2. Orchestration targets are still hardcoded

The orchestration layer still bakes in a single supported product wedge:

- `pipeline.foundation_inputs`
- `stage.10_feature_spec`
- `feature-slice-decomposer`

This must become a declared target contract rather than compiler logic frozen to one flow.

### 3. The current compiler surface is too flat

`crates/compiler/src/lib.rs` currently re-exports most of the crate, which is convenient for the current CLI but too broad to be a durable architecture boundary.

### 4. The CLI is still too thick

`crates/cli/src/main.rs` currently contains product shell behavior, interactive behavior, rendering choices, and a large amount of compiler-specific wiring in one file.

## Ordered Phases

## Phase 0: Freeze The Boundary Contract

Goal:

- define the target split and the implementation order
- prevent ad hoc extraction work from drifting

Deliverables:

- this file
- stable phase ordering
- explicit migration gates

Checklist:

- [x] create the root extraction tracker
- [ ] confirm this file is the active extraction authority
- [ ] keep status/progress current as packets land

## Phase 1: Parameterize Layout And Storage

Goal:

- remove baked-in `.handbook/**` and state-root assumptions from reusable internals

Required result:

- reusable code consumes a typed layout object instead of relying on scattered path constants

Likely seam:

- introduce something like `HandbookLayout`
- centralize canonical artifact roots
- centralize state roots
- centralize route-state locations
- centralize capture provenance locations
- centralize handoff bundle roots

Primary files likely involved:

- `crates/compiler/src/canonical_artifacts.rs`
- `crates/compiler/src/setup.rs`
- `crates/compiler/src/route_state.rs`
- `crates/compiler/src/stage_10_feature_spec_provenance.rs`
- `crates/compiler/src/pipeline_capture.rs`
- `crates/compiler/src/pipeline_handoff.rs`
- `crates/compiler/src/author/charter.rs`
- `crates/compiler/src/author/project_context.rs`
- `crates/compiler/src/author/environment_inventory.rs`

Checklist:

- [ ] design the typed layout object and its ownership boundary
- [ ] inventory all reusable-internal references to `.handbook/**`
- [ ] inventory all reusable-internal references to `.handbook/state/**`
- [ ] move canonical artifact path ownership behind the layout seam
- [ ] move route-state file ownership behind the layout seam
- [ ] move stage-capture provenance file ownership behind the layout seam
- [ ] move handoff bundle root ownership behind the layout seam
- [ ] keep CLI/product wording out of the layout seam

Exit criteria:

- reusable internals no longer depend on scattered hardcoded storage roots
- all remaining path literals are either product-shell-only or intentionally temporary compatibility glue

## Phase 2: Parameterize Orchestration Targets And Template Resolution

Goal:

- replace hardcoded supported flow identities with declared supported target definitions
- introduce a validated template/library resolver boundary instead of leaving selection frozen in implementation details

Required result:

- orchestration logic supports declared target contracts rather than one frozen product wedge
- target and template resolution use a hybrid model: configurable by users, but validated and bounded by code-owned primitives

Likely seam:

- introduce something like `PipelineTarget`, `HandoffTarget`, or `OrchestrationTarget`
- define supported pipeline/stage/consumer contracts in one place
- let compile/capture/handoff resolve through those definitions
- introduce a typed template/library resolver boundary
- keep shipped templates and library content as repo-owned defaults initially
- allow configuration to select or override within validated rules

Primary files likely involved:

- `crates/compiler/src/pipeline_compile.rs`
- `crates/compiler/src/pipeline_capture.rs`
- `crates/compiler/src/pipeline_handoff.rs`
- `crates/compiler/src/stage_10_feature_spec_provenance.rs`
- possibly `crates/compiler/src/pipeline.rs`
- `crates/compiler/src/author/charter.rs`
- `crates/compiler/src/author/environment_inventory.rs`

Checklist:

- [ ] define a typed supported-target contract
- [ ] remove direct hardcoding of `pipeline.foundation_inputs`
- [ ] remove direct hardcoding of `stage.10_feature_spec`
- [ ] remove direct hardcoding of `feature-slice-decomposer`
- [ ] route recovery/help text through target definitions where appropriate
- [ ] ensure targets determine inputs, outputs, provenance rules, and materialization rules
- [ ] define the typed template/library resolver boundary
- [ ] keep shipped templates/library content as validated defaults initially
- [ ] allow configuration to select or override targets/templates within validated bounds

Exit criteria:

- compile/capture/handoff no longer rely on hardcoded product identities in core orchestration logic
- target/template resolution is configurable within bounded validated rules instead of being fully hardcoded or fully unbounded

## Phase 3: Split Mixed Engine And Product Logic In Place

Goal:

- identify and separate pure engine logic from product-shell behavior before introducing new crates

Required result:

- mixed modules are cleaner, even before file moves happen

Primary files likely involved:

- `crates/compiler/src/author/charter.rs`
- `crates/compiler/src/author/project_context.rs`
- `crates/compiler/src/author/environment_inventory.rs`
- `crates/compiler/src/doctor.rs`
- `crates/compiler/src/setup.rs`

Checklist:

- [ ] separate deterministic parse/render/validate from guided synthesis behavior
- [ ] separate lock-file/storage mechanics from canonical truth modeling where practical
- [ ] separate product command wording from readiness/report logic
- [ ] separate product-shell recovery wording from reusable refusal classification where practical

Exit criteria:

- candidate engine modules have a mostly reusable shape before crate extraction starts

## Phase 4: Introduce Real Crate Boundaries

Goal:

- create the actual engine/pipeline/flow split inside this repo

Required result:

- new crates exist and the implementation is no longer centered in the current monolithic compiler crate

Target workspace shape:

- `crates/engine`
- `crates/pipeline`
- `crates/flow`
- `crates/cli`

Checklist:

- [ ] add `crates/engine`
- [ ] add `crates/pipeline`
- [ ] add `crates/flow`
- [ ] move engine code behind `handbook-engine`
- [ ] move pipeline code behind `handbook-pipeline`
- [ ] move middle-layer flow/application code behind `handbook-flow`
- [ ] move callers directly to the new crates without relying on a compiler facade
- [ ] narrow or retire `crates/compiler` intentionally rather than preserving it as the implementation center

Exit criteria:

- the split is structurally real in the workspace
- the old compiler crate is no longer the true implementation home

## Phase 5: Thin The CLI

Goal:

- make `handbook-cli` a real product shell instead of a large integration bucket

Required result:

- CLI owns parsing, prompting, wording, and rendering
- engine/pipeline/flow crates own the reusable logic
- CLI thinning reaches the medium target rather than a minimal import rewrite or a maximal shell-purity rewrite

Primary files likely involved:

- `crates/cli/src/main.rs`
- targeted CLI helper modules to be introduced as needed

Checklist:

- [ ] reduce `main.rs` responsibility
- [ ] split CLI by command/helper areas where useful
- [ ] isolate interactive prompting helpers
- [ ] isolate product wording/rendering helpers
- [ ] stop pulling broad internal types directly through product code where avoidable
- [ ] keep command surface behavior unchanged unless explicitly intended

Exit criteria:

- CLI boundary is thin enough that engine migration does not drag the product shell with it

## Phase 6: Reassess Migration Readiness

Goal:

- decide the final ownership model after extraction:
  - handbook-owned crates imported by Substrate
  - or crates moved into Substrate ownership

Checklist:

- [ ] confirm `handbook-engine` is reusable without handbook-product assumptions
- [ ] confirm `handbook-pipeline` is either reusable as-is or intentionally stays external for now
- [ ] confirm `handbook-flow` is either reusable as-is or intentionally stays handbook-owned longer than engine/pipeline
- [ ] confirm Substrate should consume engine surfaces rather than absorb the CLI shell
- [ ] decide per crate whether handbook remains the architectural owner or whether Substrate becomes the owner
- [ ] write the migration/import plan only after the extraction prerequisites are truly complete

Exit criteria:

- the repo is ready for a separate ownership and integration plan

### Final ownership decision rule

Do not assume up front that extracted crates belong inside Substrate.

After parameterization and decoupling, evaluate each crate with this rule:

- if a crate is still fundamentally handbook-domain, handbook should own it and Substrate should import it
- only move a crate into Substrate if its real center of gravity becomes substrate-specific

Questions to answer for each crate:

- does this encode handbook concepts or substrate concepts?
- is the handbook CLI still the primary product surface for it?
- can Substrate consume it through a clean adapter boundary?
- would substrate-specific pressure distort the crate if handbook kept owning it?
- would moving it into Substrate create a cleaner dependency graph, or create hidden coupling?

Current bias:

- `handbook-engine`: likely handbook-owned and imported by Substrate
- `handbook-pipeline`: depends on whether it remains truly generic after parameterization
- `handbook-flow`: least clear; likely stays handbook-owned longer until its real center of gravity is proven

## Initial Module Classification

### Likely `handbook-engine`

- `crates/compiler/src/canonical_artifacts.rs`
- `crates/compiler/src/artifact_manifest.rs`
- `crates/compiler/src/freshness.rs`
- `crates/compiler/src/baseline_validation.rs`
- engine-safe portions of `crates/compiler/src/author/**`
- most of `crates/compiler/src/doctor.rs`

### Likely `handbook-pipeline`

- `crates/compiler/src/declarative_roots.rs`
- `crates/compiler/src/pipeline.rs`
- `crates/compiler/src/pipeline_route.rs`
- `crates/compiler/src/route_state.rs`
- `crates/compiler/src/pipeline_compile.rs`
- `crates/compiler/src/pipeline_capture.rs`
- `crates/compiler/src/pipeline_handoff.rs`
- `crates/compiler/src/stage_10_feature_spec_provenance.rs`
- `crates/compiler/src/repo_file_access.rs`
- reusable parts of `crates/compiler/src/setup.rs`

### Likely `handbook-flow`

- `crates/compiler/src/resolver.rs`
- `crates/compiler/src/packet_result.rs`
- `crates/compiler/src/budget.rs`

### Hold Or Reassess Later

- `crates/compiler/src/rendering/**`
- `crates/compiler/src/refusal.rs`
- `crates/compiler/src/error.rs`

These should not be forced into an early split without proving whether they are engine, pipeline, flow, or product-shell concerns.

## Verification Wall

Use this wall as the default extraction proof unless a smaller packet explicitly justifies a narrower subset:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Useful focused rails during extraction:

```bash
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-compiler --test author
cargo test -p handbook-compiler --test setup
cargo test -p handbook-compiler --test doctor
cargo test -p handbook-compiler --test pipeline_loader
cargo test -p handbook-compiler --test pipeline_catalog
cargo test -p handbook-compiler --test pipeline_route_resolution
cargo test -p handbook-compiler --test pipeline_compile
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
cargo test -p handbook-compiler --test pipeline_state_store
```

## Progress Log

### 2026-06-07

- created the root extraction tracker
- recorded the ordered phase plan
- recorded the pre-migration prerequisites
- recorded the initial module classification

## Open Questions

- After extraction, which crates should be handbook-owned and imported by Substrate versus moved into Substrate ownership?
- Should `handbook-pipeline` eventually move into Substrate too, or remain handbook-owned longer than the engine?
- Which parts of `rendering`, `refusal`, and `error` belong in CLI versus `handbook-flow`?
- Should authoring templates remain compile-time embedded defaults long-term, or later move further behind a configurable library resolver?
- At what point should `crates/compiler` be retired entirely instead of remaining as a temporary transition artifact?

## Locked Decisions

- Callers move directly to `handbook-engine`, `handbook-pipeline`, and `handbook-flow`; do not rely on a compiler facade.
- Use separate layout types rather than one global layout object.
- Use a hybrid configuration model for orchestration targets:
  - code owns primitives, validators, and invariants
  - configuration declares supported and active target instances within validated bounds
- Use the same hybrid direction for template/library resolution:
  - shipped defaults remain repo-owned initially
  - configuration may select or override within validated rules
- Treat `resolver`, `packet_result`, and `budget` as the initial `handbook-flow` candidates rather than forcing them into engine or pipeline.
- Use the medium CLI-thinning target:
  - move reusable domain logic out of CLI
  - split CLI by command/helper areas as needed
  - keep prompting, wording, rendering, and exit-code decisions in CLI

## Migration Gate

Do not start the Substrate move until all of the following are true:

- [ ] Phase 1 complete
- [ ] Phase 2 complete
- [ ] Phase 3 complete
- [ ] Phase 4 complete
- [ ] Phase 5 complete

If any of those boxes are still open, migration planning is premature.
