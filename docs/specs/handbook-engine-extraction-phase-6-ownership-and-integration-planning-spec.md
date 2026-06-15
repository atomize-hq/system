# Spec: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

## Status

- Planned planning-only family.
- This draft is the SPECIFY stage artifact for the next Phase 6 family; implementation remains blocked until the SPEC/PLAN/TASKS triplet is reviewed and approved by the human.
- This family is the explicit follow-on boundary named by Packet 6.1.4 in the Phase 6 Slice 1 reassessment triplet.
- Phase 6 Slice 1 is already revalidated as **READY** in live repo truth after `aa882af42792a250cc02a6740bd1e2123178caff` (`Slice 1.5: remove handbook-product engine layout boundary`) and `a883d168fe42aabb1504fed7397cdb03ff9874ad` (`Revalidate Phase 6 readiness after layout boundary removal`).
- `git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..a883d168fe42aabb1504fed7397cdb03ff9874ad` is docs-only, so the current crate/code truth remains the readiness-validated workspace.
- This family must stop at ownership/integration planning. It must not begin code implementation, packet-prompt authoring, crate moves, runtime rewrites, CLI redesign, or retained-compiler retirement work.

## Assumptions I'm Making

Correct these before execution if current repo truth, human intent, or Phase 6 authority has changed:

1. The root migration authority remains `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, and its Phase 6 rule still governs all ownership decisions in this family.
2. The Phase 6 Slice 1 reassessment triplet and Packet 6.1.2 ownership matrix are current starting inputs, but Packet 6.1.2 alone is not the final verdict because it predates the later layout-boundary fix.
3. The extracted workspace shape is real in live code: `crates/engine`, `crates/pipeline`, `crates/flow`, and `crates/cli` are the active owner layers, while `crates/compiler` is retained as a narrow compatibility/support seam rather than the implementation center.
4. No in-scope crate currently proves a substrate-specific center of gravity strongly enough to skip explicit planning; this family exists to make those calls explicit and reviewable.
5. Success for this family is a human-reviewable ownership/import framework with explicit follow-on execution seams, not code changes or migration execution.
6. If this family uncovers a real contradiction with the landed Phase 1 through Phase 5 closeout posture, it should name the contradiction and its owning seam explicitly rather than silently reopening older work.

## Objective

Define the final ownership/import decision framework after extraction so a later approved execution slice can act on explicit, evidence-backed crate boundaries instead of implicit assumptions.

The maintainer needs this family because the extraction work is already landed and Phase 6 Slice 1 is already READY; the remaining honest question is not “did extraction happen?” but “what should handbook own, what should Substrate import through a clean boundary, and what must remain handbook-owned shell or transition glue for now?”

This family succeeds only when all of the following are true:

- each in-scope crate has an explicit architectural-owner call
- each in-scope crate has an explicit Substrate posture call (`import through boundary`, `not an import target`, or `defer any move until a later narrower seam`)
- each in-scope crate has an intended import/integration boundary named in repo terms
- `handbook-cli` is handled explicitly as product shell versus reusable core
- retained `handbook-compiler` is handled explicitly as transition glue versus future ownership target
- the family names follow-on execution seams without starting them
- the output is reviewable without silently widening into implementation

## Tech Stack

- Rust 2021 workspace
- in-scope crates:
  - `handbook-engine`
  - `handbook-pipeline`
  - `handbook-flow`
  - `handbook-cli`
  - retained `handbook-compiler`
- planning authorities:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`
- live code/test evidence:
  - `crates/engine/src/**`, `crates/engine/tests/**`
  - `crates/pipeline/src/**`, `crates/pipeline/tests/**`
  - `crates/flow/src/**`, `crates/flow/tests/**`
  - `crates/cli/src/**`, `crates/cli/tests/**`
  - `crates/compiler/src/**`, `crates/compiler/tests/**`

## Commands

Truth-surface freeze:

```bash
git status --short --branch
git rev-parse HEAD
git log --oneline --decorate -20
git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..HEAD
git diff --name-only aa882af42792a250cc02a6740bd1e2123178caff..HEAD
```

Authority sweep:

```bash
rg -n "Phase 6|Final ownership decision rule|Exit criteria|Open Questions" HANDBOOK_ENGINE_EXTRACTION_PLAN.md
rg -n "Phase 6|fully landed through Slice 5.3|next authoritative step" docs/specs/handbook-engine-extraction-slice-map.md
rg -n "Phase 6|next authoritative step|Set 3|Set 4" docs/specs/handbook-engine-extraction-closeout-four-set-map.md
rg -n "READY|Packet 6\.1\.4|ownership-and-integration-planning|default_canonical_layout_contract" docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md
```

Ownership-surface sweep:

```bash
rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs crates/cli/src/main.rs
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow
cargo tree -p handbook-compiler
```

Boundary-specific evidence rails:

```bash
rg -n "default_canonical_layout_contract|CanonicalLayoutContract" crates/engine/src crates/flow/src
rg -n "stage_10_feature_spec|PipelineHandoff|PipelineCapture|RouteState|template_library" crates/pipeline/src crates/pipeline/tests
rg -n "rendering|refusal|doctor|setup|template_library" crates/compiler/src
rg -n "CommandFactory|ExitCode|pipeline_help|doctor|rendering|prompt" crates/cli/src/main.rs crates/cli/src
```

Representative verification rails:

```bash
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-flow --test resolver_core
cargo test -p handbook-cli --test help_drift_guard
cargo check --workspace
```

Inherited full verification wall before later execution work:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Project Structure

```text
HANDBOOK_ENGINE_EXTRACTION_PLAN.md                                                                    -> root migration authority and Phase 6 ownership decision rule
docs/specs/handbook-engine-extraction-slice-map.md                                                    -> Phase -> Slice -> Packet authority through the landed extraction work
docs/specs/handbook-engine-extraction-closeout-four-set-map.md                                        -> bounded closeout authority for what Phases 1 through 5 actually finished
docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md          -> READY verdict and named follow-on family authority
docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md               -> pre-fix ownership matrix input that must be updated against current repo truth
docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md              -> this family's scope, evidence rules, and decision framework
docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md              -> this family's ordered planning approach and downstream seam map
docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md             -> this family's bounded reviewable task ledger
docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-packet-prompts.md    -> approved orchestration prompts for landing each packet after human request; does not by itself authorize implementation
Cargo.toml                                                                                             -> workspace member list (`crates/compiler`, `crates/cli`, `crates/engine`, `crates/pipeline`, `crates/flow`)
crates/engine/src/lib.rs                                                                               -> reusable engine exports and canonical layout contract surface
crates/pipeline/src/lib.rs                                                                             -> reusable pipeline exports and bounded orchestration/runtime surface
crates/flow/src/lib.rs                                                                                 -> middle-layer composition exports (`budget`, `packet_result`, `resolver`)
crates/compiler/src/lib.rs                                                                             -> retained compatibility/support seam above the extracted crates
crates/cli/src/main.rs                                                                                 -> product-shell entrypoint and CLI boundary proof
crates/pipeline/tests/pipeline_catalog.rs                                                              -> evidence of remaining compiler-backed fixture coupling in pipeline tests
crates/engine/tests/canonical_artifacts_ingest.rs, crates/flow/tests/resolver_core.rs, crates/cli/tests/help_drift_guard.rs -> representative proof rails
```

## Code Style

Prefer explicit decision records over optimistic prose. Every crate call should name the owner, the import posture, the concrete boundary, the evidence, and the deferred execution seam.

```rust
OwnershipCall {
    crate_name: "handbook-pipeline",
    architectural_owner: ArchitecturalOwner::Handbook,
    substrate_posture: SubstratePosture::ImportThroughReviewedBoundary,
    import_boundary: "public exports from crates/pipeline/src/lib.rs",
    retained_handbook_surface: "compiler-backed fixture coupling remains transition-only",
    follow_on_execution_seam: Some("narrow pipeline fixture/support coupling before any move discussion"),
}
```

Conventions:

- ground every claim in live repo files or command output
- distinguish current posture from future execution work
- prefer “defer explicitly” over “probably fine”
- state handbook-owned shell and handbook-owned glue honestly instead of treating all extracted crates as equally reusable
- do not convert Phase 6 planning into a hidden implementation brief

## Testing Strategy

- Framework: existing Rust unit/integration tests plus authority-doc consistency checks and live dependency-surface inspection
- Primary evidence levels:
  - authority alignment (`HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, slice map, closeout map, reassessment triplet)
  - crate-surface inspection (`lib.rs`/`main.rs` exports and CLI shell shape)
  - dependency-graph inspection (`cargo tree`)
  - representative regression rails (`canonical_artifacts_ingest`, `pipeline_catalog`, `resolver_core`, `help_drift_guard`, `cargo check --workspace`)
  - inherited full verification wall before later execution slices
- Coverage focus:
  - the post-`aa882af...` engine boundary fix is reflected honestly in the ownership story
  - pipeline still records its reviewed boundary and remaining compiler-backed fixture/support coupling
  - flow remains explicit as handbook-side composition rather than an implied import target
  - CLI remains explicit product shell, not reusable imported core
  - retained compiler remains explicit transition glue, not the real owner layer
- Honesty rule:
  - if the code/tests/docs contradict the planning posture, record the contradiction explicitly
  - if a question remains open but non-blocking, defer it explicitly instead of inflating certainty

## Planning Scope

In scope:

- decide the per-crate ownership/import decision framework for `handbook-engine`, `handbook-pipeline`, `handbook-flow`, `handbook-cli`, and retained `handbook-compiler`
- define the intended import/integration boundary for each in-scope crate
- state what remains handbook-owned reusable core versus handbook-owned product shell versus handbook-owned transition glue
- update the Packet 6.1.2-style crate posture against current repo truth after `aa882af...` and `a883d16...`
- name the bounded follow-on execution seams that would come after planning approval

Out of scope:

- moving crates between repos
- rewriting runtime behavior
- broad CLI redesign
- retiring retained `handbook-compiler`
- production code changes of any kind
- packet-prompt authoring
- reopening Phase 1 through Phase 5 closeout implementation without concrete regression evidence

## Current Repo-Truth Starting Posture This Family Must Finalize

| Crate | Current repo-truth starting posture | Intended planning question |
| --- | --- | --- |
| `handbook-engine` | handbook-domain core with a generic `default_canonical_layout_contract` export and foundational dependency shape | keep handbook as architectural owner and make Substrate import the current engine surface, or define a thinner adapter boundary later? |
| `handbook-pipeline` | handbook-domain reusable pipeline with a reviewed runtime boundary plus remaining compiler-backed fixture/support coupling | keep handbook as owner and import the reviewed pipeline boundary, or prove a narrower/more isolated surface before any later ownership move discussion? |
| `handbook-flow` | handbook-side middle-layer composition with only `budget`, `packet_result`, and `resolver` exported | keep handbook ownership longer-term, or define a small future import boundary without claiming a current move target? |
| `handbook-cli` | handbook product shell and entrypoint, not reusable core | keep fully handbook-owned and explicitly outside any Substrate import target? |
| retained `handbook-compiler` | handbook-side compatibility/support seam above the extracted crates | keep as transition glue and non-target, while naming the later narrowing/retirement seam separately? |

## Required Outputs

This family should leave behind, inside this triplet only:

- one explicit per-crate ownership/import decision table
- one explicit per-crate intended boundary description
- one explicit classification for handbook-owned reusable core vs handbook-owned shell vs handbook-owned transition glue
- one explicit downstream seam map for later approved execution work
- one explicit human review gate before any implementation

After later explicit human approval, this family may also carry:

- one packet-prompts artifact that orchestrates packet landings without authorizing implementation by itself

## Boundaries

- Always:
  - use current repo truth after `aa882af...` and `a883d16...`
  - keep the family docs-only and planning-only
  - surface assumptions and contradictions explicitly instead of silently inheriting stale wording
  - make every crate explicit; do not leave `handbook-flow`, `handbook-cli`, or retained `handbook-compiler` implied
  - state downstream execution seams separately from current planning conclusions
- Ask first:
  - any production code change
  - any new planning artifact beyond the triplet and the explicitly approved `handbook-engine-extraction-phase-6-ownership-and-integration-planning-packet-prompts.md`
  - any reopening of a landed Phase 1 through Phase 5 closeout seam without hard regression evidence
  - any new or widened prompt artifact beyond the explicitly approved packet-prompts file
- Never:
  - treat Packet 6.1.2 alone as sufficient current truth without the later layout-boundary fix and reassessment verdict
  - widen into runtime behavior changes, crate moves, CLI redesign, or retained-compiler retirement work
  - treat `handbook-cli` or retained `handbook-compiler` as default Substrate ownership targets
  - call the planning family complete while import boundaries remain implicit

## Success Criteria

- the planning triplet exists and is internally consistent
- the family is grounded in current repo truth after the READY Phase 6 reassessment
- every in-scope crate has a clear ownership/import question and intended boundary to settle
- the triplet is explicit about handbook-owned reusable core vs product shell vs transition glue
- downstream execution seams are named without starting implementation
- the approved packet-prompts artifact, if present, matches the triplet and still does not authorize implementation by itself
- the output is ready for human review and approval before any execution work begins

## Open Questions

- Does `handbook-engine` need a thinner adapter boundary for Substrate consumption, or is the current engine export surface already the honest import seam?
- Should `handbook-pipeline` keep handbook ownership longer because of the compiler-backed fixture/support seam, even if its runtime surface is already importable?
- Does `handbook-flow` ever earn a future move discussion, or is the honest long-term posture still handbook-owned composition?
- Which `rendering`, `refusal`, or error-facing surfaces belong in a future CLI/support narrowing seam versus a future flow/pipeline seam?
- When later execution happens, what should be narrowed first: pipeline fixture/support coupling, flow ownership clarification, or retained compiler support surfaces?
