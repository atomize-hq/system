# Tasks: Handbook Engine Extraction Phase 6 Slice 1 (Slice 6.1) - Migration Readiness Reassessment

Plan reference: [handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md](./handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md)

## Status

- Planned reassessment task ledger.
- This checklist exists to execute Phase 6 validation and produce the final planning boundary.
- No code implementation or ownership/import planning should begin from this file without a later explicit approval step.

## Implementation Authority Used

Before execution, this slice is grounded in:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- the landed Phase 1 Slice 1.5 closeout triplet
- the landed Phase 2 Slice 2.4 closeout triplet
- the landed Phase 4 Slice 4.5 closeout triplet
- the landed Phase 5 Slice 5.3 closeout triplet

This slice is a reassessment seam. It should validate readiness and define the next planning boundary without silently widening into implementation or into the follow-on ownership/import planning family.

## Packet 6.1.1: Freeze Live Repo Truth And Revalidate The Migration Gate

- [ ] Task: Record working-tree truth and authority-doc truth before making any readiness call
  - Acceptance: Branch, HEAD, and working-tree posture are recorded; the root plan, slice map, and closeout map still name Phase 6 as the next authoritative step; the execution clearly states whether it is validating committed truth or local-only truth.
  - Verify: `git status --short --branch && git log --oneline --decorate -20 && rg -n "Phase 6|Migration Gate|next authoritative step" HANDBOOK_ENGINE_EXTRACTION_PLAN.md docs/specs/handbook-engine-extraction-slice-map.md docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
  - Files: `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, `docs/specs/handbook-engine-extraction-slice-map.md`, `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`

- [ ] Task: Re-run the representative proof rails and full workspace wall before any final verdict
  - Acceptance: The representative closeout rails and the full verification wall are green, or any failure is captured as a concrete blocker rather than omitted from the Phase 6 call.
  - Verify: `cargo test -p handbook-engine --test canonical_artifacts_ingest && cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-cli --test help_drift_guard && cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
  - Files: `crates/engine/tests/canonical_artifacts_ingest.rs`, `crates/pipeline/tests/pipeline_catalog.rs`, `crates/cli/tests/help_drift_guard.rs`

## Packet 6.1.2: Reassess Extracted Crate Boundaries Against The Ownership Rule

- [ ] Task: Reassess `handbook-engine` and `handbook-pipeline` against handbook-domain versus substrate-domain center of gravity
  - Acceptance: Each crate has an explicit evidence-backed posture that answers whether Substrate should import it through a clean boundary or whether moving ownership would be cleaner later; intentional boundedness is not misreported as a blocker by itself.
  - Verify: `rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs && cargo tree -p handbook-engine && cargo tree -p handbook-pipeline`
  - Files: `crates/engine/src/lib.rs`, `crates/pipeline/src/lib.rs`, `crates/engine/tests/canonical_artifacts_ingest.rs`, `crates/pipeline/tests/pipeline_catalog.rs`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`

- [ ] Task: Reassess `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` posture explicitly
  - Acceptance: `handbook-flow` is not left as an implied afterthought, `handbook-cli` is treated honestly as a product shell rather than a move target, and retained `handbook-compiler` posture is classified as either temporary glue to retire later or a still-blocking ambiguity.
  - Verify: `rg -n "pub use|pub mod|mod " crates/flow/src/lib.rs crates/compiler/src/lib.rs crates/cli/src/main.rs && cargo tree -p handbook-flow && cargo tree -p handbook-compiler`
  - Files: `crates/flow/src/lib.rs`, `crates/compiler/src/lib.rs`, `crates/cli/src/main.rs`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`

## Packet 6.1.3: Resolve The Readiness Verdict And Explicit Deferrals

- [ ] Task: Produce one crate-by-crate ownership/readiness matrix with explicit blockers versus open questions
  - Acceptance: Every crate is assigned an explicit posture such as handbook-owned/imported, candidate future move, or explicitly deferred; blockers are separated from non-blocking open questions; no crate is left implicit.
  - Verify: Manual review against `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` Phase 6 checklist and the live crate surfaces named in the spec.
  - Files: `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md`

- [ ] Task: Write the final Phase 6 verdict without beginning the follow-on planning family
  - Acceptance: The slice ends with an explicit readiness call for separate ownership/integration planning, or with a named narrow blocker seam if the repo is not ready; the output does not silently spill into authoring the next family.
  - Verify: Manual review against the Phase 6 success criteria and planned exit conditions in the spec/plan.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`

## Packet 6.1.4: Name The Next Planning Boundary

- [ ] Task: If READY, name the exact next planning family without starting it
  - Acceptance: The output names the follow-on ownership/integration planning family clearly enough for a future session to start it cleanly, but does not generate that family inside Slice 6.1.
  - Verify: Manual review that the named next family is distinct from this reassessment slice and does not reopen a closed earlier seam.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`

- [ ] Task: If NOT READY, route the blocker back to a narrow earlier seam rather than vague “more Phase 6”
  - Acceptance: Any blocking regression is attached to a concrete earlier seam owner (for example layout parameterization, target parameterization, caller/compiler narrowing, or CLI shell closeout) instead of being left as generic unfinished reassessment work.
  - Verify: Manual review against `docs/specs/handbook-engine-extraction-closeout-four-set-map.md` and the relevant landed closeout triplet.
  - Files: `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`, `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`

## Human Review Gate

Do not start the follow-on ownership/import planning family from this task ledger until the human reviews the Phase 6 verdict and explicitly approves that next step.
