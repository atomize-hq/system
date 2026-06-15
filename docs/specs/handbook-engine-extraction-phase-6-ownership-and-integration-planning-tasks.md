# Tasks: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

Plan reference: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md)

## Status

- Planned planning-only task ledger.
- This file is the TASKS stage artifact for the family. It is reviewable planning work only and does not authorize implementation until the human approves the SPEC/PLAN/TASKS triplet.
- This checklist exists to finish the ownership/integration planning family named by Packet 6.1.4.
- This ledger is intentionally docs-only: no production code changes or implementation work should start from it without a later explicit approval step. The separately approved packet-prompts artifact may orchestrate packet landings, but it still does not authorize implementation by itself.
- The family starts from live repo truth after `aa882af42792a250cc02a6740bd1e2123178caff` and `a883d168fe42aabb1504fed7397cdb03ff9874ad`.
- Unrelated local edits in `/Users/spensermcconnell/__Active_Code/system/AGENTS.md` and `/Users/spensermcconnell/__Active_Code/system/CLAUDE.md`, if present in a future execution session, must remain preserved and out of scope.

## Implementation Authority Used

Before execution, this family is grounded in:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`
- `crates/engine/src/lib.rs`
- `crates/pipeline/src/lib.rs`
- `crates/flow/src/lib.rs`
- `crates/compiler/src/lib.rs`
- `crates/cli/src/main.rs`

This family should make the ownership/import decision framework explicit without silently widening into code execution.

## Task Group 1: Freeze Current Authority And Scope Guard

- [ ] Task: Record the current repo-truth starting gate for this planning family
  - Acceptance: The triplet states the current branch/HEAD posture, the docs-only delta from `aa882af...` to current HEAD, and the fact that Phase 6 Slice 1 is already READY before this family begins.
  - Verify: `git status --short --branch && git rev-parse HEAD && git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..HEAD && rg -n "READY|Packet 6\.1\.4|ownership-and-integration-planning" docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

- [ ] Task: Re-state the root ownership rule and hard planning boundaries inside the triplet
  - Acceptance: The triplet makes explicit that handbook stays owner when the center of gravity is handbook-domain, Substrate only becomes owner if the surface is truly substrate-specific, and implementation stays out of scope.
  - Verify: `rg -n "architectural owner|Substrate|product shell|transition glue|Never:" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-*.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

## Task Group 2: Decide Handbook-Owned Imported-Core Boundaries

- [ ] Task: Lock the `handbook-engine` ownership/import posture
  - Acceptance: The triplet records whether handbook remains the architectural owner, whether Substrate should import the engine surface through the current public exports or a thinner adapter later, and what evidence supports that call.
  - Verify: `rg -n "pub use|default_canonical_layout_contract|workspace_contract_version" crates/engine/src/lib.rs crates/engine/src/canonical_paths.rs && cargo tree -p handbook-engine && cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

- [ ] Task: Lock the `handbook-pipeline` ownership/import posture
  - Acceptance: The triplet records whether handbook remains the architectural owner, what the intended pipeline import boundary is, and which remaining compiler-backed fixture/support seams stay deferred to later execution instead of being hidden in this planning call.
  - Verify: `rg -n "PipelineCapture|PipelineHandoff|RouteState|template_library|stage_10_feature_spec" crates/pipeline/src crates/pipeline/tests && cargo tree -p handbook-pipeline && cargo test -p handbook-pipeline --test pipeline_catalog`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

## Task Group 3: Decide Handbook-Side Deferred Boundaries And Non-Targets

- [ ] Task: Lock the `handbook-flow` ownership posture without overclaiming a move target
  - Acceptance: The triplet states whether `handbook-flow` remains handbook-owned longer-term, what a future narrower import boundary would have to prove, and why the current surface does or does not justify more than that.
  - Verify: `rg -n "pub mod|pub use|resolve|PacketResult|BudgetOutcome" crates/flow/src/lib.rs crates/flow/src && cargo tree -p handbook-flow && cargo test -p handbook-flow --test resolver_core`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

- [ ] Task: Lock the `handbook-cli` product-shell boundary explicitly
  - Acceptance: The triplet states that CLI remains handbook-owned product shell, names the shell boundary in repo terms, and does not leave CLI as an implied import target.
  - Verify: `rg -n "CommandFactory|ExitCode|pipeline_help|doctor|rendering|prompt" crates/cli/src/main.rs crates/cli/src && cargo test -p handbook-cli --test help_drift_guard`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

- [ ] Task: Lock retained `handbook-compiler` as transition glue rather than ownership target
  - Acceptance: The triplet states the retained compiler's intended role, names the support/glue surfaces that still justify its existence, and defers any narrowing/retirement work to a later bounded seam.
  - Verify: `rg -n "rendering|refusal|doctor|setup|template_library|pub use" crates/compiler/src/lib.rs crates/compiler/src && cargo tree -p handbook-compiler && cargo check --workspace`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

## Task Group 4: Define Downstream Execution Seams Without Starting Them

- [ ] Task: Name the bounded follow-on execution seams per crate/owner area
  - Acceptance: The triplet names later seams for engine boundary freeze (if needed), pipeline boundary cleanup, flow ownership clarification, retained compiler narrowing, and CLI shell/support clarification without turning any of them into implementation tasks in this pass.
  - Verify: `rg -n "follow-on|downstream execution seams|adapter|boundary cleanup|narrowing|review gate" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-*.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

- [ ] Task: Finish with an explicit human review gate before any execution work
  - Acceptance: The triplet ends with clear language that this family is review-ready but not execution-approved, and any approved packet-prompts artifact is treated as orchestration support rather than code-task approval.
  - Verify: Manual review against the Status, Boundaries, Planned Exit Conditions, and task ledger language in this triplet.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

## Human Review Gate

Do not begin implementation, packet-prompt authoring, crate moves, runtime behavior changes, CLI redesign, or retained-compiler retirement work from this ledger until the human reviews and approves this planning family.
