# Handbook Engine Extraction Phase 1 Slice 1 Packet Prompts

Task source: [handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md](./handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md)
Spec source: [handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md](./handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md)
Plan source: [handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md](./handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, and preserves commit boundaries between implementation, review, and fix work.

## Packet 1.1.1 Prompt

```text
/goal Orchestrate Phase 1 Slice 1 Packet 1.1.1: Layout Type Family And Ownership Boundary in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md
- Do not start Packet 1.1.2.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Commit changes after the implementation lands, and commit again after each fix round that changes files.
- Stay inside Packet 1.1.1 scope.

Packet 1.1.1 scope:
- Freeze the layout ownership domains in the slice authority docs.
- Freeze the Slice 1.1 no-migration boundary across the authority set.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md
  - docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md
  - docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md

Out of scope:
- Packet 1.1.2 inventory-table work
- any caller migration
- any production Rust behavior change
- any Slice 1.2 / 1.3 / 1.4 work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 1 Packet 1.1.1: Layout Type Family And Ownership Boundary`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to keep this packet docs-only unless a tiny behavior-neutral docs-scaffolding change is strictly necessary, and forbid widening into source-level adoption work.
- Require the spec/plan/tasks set to explicitly freeze:
  - separate layout types rather than one global layout object
  - the canonical root, runtime state, capture provenance, handoff bundle, and authoring ownership domains
  - the no-caller-migration boundary for Slice 1.1
  - the handoff to Slices 1.2, 1.3, and 1.4
- Require targeted verification with:
  - `rg -n "separate layout types|no caller migration|Ownership domain" docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`
- Require the subagent to stop after Packet 1.1.1 acceptance is met and report touched files, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 1 Packet 1.1.1: Layout Type Family And Ownership Boundary`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the slice spec, plan, tasks, and verification evidence.
- Require severity labels and explicit callouts if Packet 1.1.2 work leaked in or if the docs drift back toward one global layout object.

Fix loop:
- If the review is clean, stop and report Packet 1.1.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal changes needed to close them.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.1.1 lands cleanly.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1.1.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.1.1 is review-clean and committed.
- Stop and report blocked if Packet 1.1.1 cannot be completed without widening into Packet 1.1.2, Slice 1.2+, or changing the approved slice spec/plan/tasks.
```

## Packet 1.1.2 Prompt

```text
/goal Orchestrate Phase 1 Slice 1 Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md.
- Assume Packet 1.1.1 is already landed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 1.1.2 scope.

Packet 1.1.2 scope:
- Build the reusable-internal inventory table for the targeted compiler corpus.
- Freeze explicit exclusions and temporary exceptions.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md
  - docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md
  - docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md

Out of scope:
- Packet 1.1.1 contract-boundary redefinition unless live repo truth proves the existing wording is wrong
- any caller migration
- any production Rust behavior change
- any Slice 1.2 / 1.3 / 1.4 adoption work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 1 Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that the Packet 1.1.1 contract language already exists before editing.
- Require the subagent to keep this packet docs-only and to freeze:
  - one inventory row per targeted compiler file
  - follow-on slice ownership mapping for each row
  - explicit CLI/product-shell exclusions
  - `pipeline_handoff.rs` as an indirect/no-direct-hit dependency for this slice
  - temporary exceptions for non-primary verifier items such as non-`.handbook` future owners
- Require targeted verification with:
  - `rg -n "\.handbook|\.handbook/state" crates/compiler/src/canonical_artifacts.rs crates/compiler/src/route_state.rs crates/compiler/src/pipeline_capture.rs crates/compiler/src/pipeline_handoff.rs crates/compiler/src/stage_10_feature_spec_provenance.rs crates/compiler/src/setup.rs crates/compiler/src/author/charter.rs crates/compiler/src/author/project_context.rs crates/compiler/src/author/environment_inventory.rs`
  - `rg -n "product-shell exclusion|indirect dependency|temporary exceptions|non-.handbook|no direct \\.handbook" docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`
- Require the subagent to stop after Packet 1.1.2 acceptance is met and report touched files, verification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 1 Packet 1.1.2: Storage Inventory And Temporary-Exception Freeze`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review with special attention to corpus completeness, exclusion honesty, slice-boundary discipline, and consistency with the already-frozen Packet 1.1.1 contract.
- Require severity labels and explicit callouts if any targeted compiler file is missing, if exclusions are silent, or if Slice 1.2+ work leaked in.

Fix loop:
- If review is clean, stop and report Packet 1.1.2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-1.1.2-bounded.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 1.1.2 implementation lands cleanly.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1.1.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.1.2 is review-clean and committed.
- Stop and report blocked if Packet 1.1.2 requires changing the Packet 1.1.1 contract, widening into Slice 1.2+, or changing the approved slice spec/plan/tasks.
```
