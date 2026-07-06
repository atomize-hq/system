# Handbook Engine Extraction Phase 1 Slice 4 Packet Prompts

Task source: [handbook-engine-extraction-phase-1-slice-4-authoring-layout-tasks.md](./handbook-engine-extraction-phase-1-slice-4-authoring-layout-tasks.md)
Spec source: [handbook-engine-extraction-phase-1-slice-4-authoring-layout-spec.md](./handbook-engine-extraction-phase-1-slice-4-authoring-layout-spec.md)
Plan source: [handbook-engine-extraction-phase-1-slice-4-authoring-layout-plan.md](./handbook-engine-extraction-phase-1-slice-4-authoring-layout-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 1.4 scope.

## Packet 1.4.1 Prompt

```text
/goal Orchestrate Phase 1 Slice 4 Packet 1.4.1: Authoring Roots And Lock Paths Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.4.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-4-authoring-layout-tasks.md.
- Verify live repo truth before changing anything, including that Slice 1.2 already landed the canonical layout owner, Slice 1.3 already landed the stateful-storage owners, and Slice 1.4 is the remaining Phase 1 authoring adoption seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-4-authoring-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-4-authoring-layout-plan.md
- Stay inside Packet 1.4.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.4.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Stay inside Packet 1.4.1 scope.

Packet 1.4.1 scope:
- Introduce compiler-local authoring layout owners.
- Adopt `author/charter.rs` onto the authoring layout owner.
- Adopt `author/project_context.rs` and `author/environment_inventory.rs` onto the authoring layout owner.
- Preserve Slice 1.4 boundaries while integrating authoring ownership.
- Expected files:
  - crates/compiler/src/layout.rs
  - crates/compiler/src/author/mod.rs
  - crates/compiler/src/author/charter.rs
  - crates/compiler/src/author/project_context.rs
  - crates/compiler/src/author/environment_inventory.rs
  - crates/compiler/src/lib.rs
  - crates/compiler/tests/author.rs

Out of scope:
- any prompt wording, template text, heading-order, structured-input-schema, or operator-wording cleanup
- any canonical artifact identity or repo-relative path redesign beyond routing existing ownership through layout accessors
- any runtime-state, capture-provenance, handoff-bundle, setup, doctor, refusal, or pipeline work beyond narrow compile-through wiring proven necessary by live code truth
- any changes to `DEFAULT_EXCEPTION_RECORD_LOCATION` or other prompt-facing path strings except where live code truth proves a tiny compile-through adjustment is strictly required
- any changes to test-only prompt-capture paths such as `.handbook/state/authoring/last_prompt.txt`
- any Phase 2 target/template work or Phase 3 shell-wording cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 4 Packet 1.4.1: Authoring Roots And Lock Paths Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Slice 1.2 and Slice 1.3 already landed and that the current compiler-local layout family is present before editing.
- Require the subagent to keep this packet bounded to authoring canonical-target and authoring lock-path ownership and to explicitly preserve:
  - charter guided authoring, structured-input validation, default exception location semantics, and refusal behavior
  - project-context metadata rendering, structured-input validation, and refusal behavior
  - environment-inventory upstream canonical-truth checks, synthesis validation, and refusal behavior
  - prompt wording, template text, heading validation, and user-facing guidance remaining local for later slices
  - canonical artifact identities already frozen by Slice 1.2
- Require the subagent to remove direct storage ownership such as `CANONICAL_CHARTER_REPO_PATH`, `CANONICAL_PROJECT_CONTEXT_REPO_PATH`, `CANONICAL_ENVIRONMENT_INVENTORY_REPO_PATH`, and `.handbook/state/authoring/*.lock` path derivation in favor of layout accessors, unless live code truth proves equivalent narrow helpers are already present.
- Require the subagent to introduce only the narrowest compiler-local authoring layout owners needed for Packet 1.4.1, preferably by extending `crates/compiler/src/layout.rs`, and to make the authoring modules consume them without widening into prompt cleanup, canonical identity redesign, or helper unification beyond what the packet strictly needs.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "\\.handbook/(charter|project_context|environment_inventory)|\\.handbook/state/authoring|CANONICAL_(CHARTER|PROJECT_CONTEXT|ENVIRONMENT_INVENTORY)_REPO_PATH|LOCK_REPO_PATH" crates/compiler/src/layout.rs crates/compiler/src/author/mod.rs crates/compiler/src/author/charter.rs crates/compiler/src/author/project_context.rs crates/compiler/src/author/environment_inventory.rs`
  - `cargo test -p handbook-compiler --test author`
  - `cargo check -p handbook-compiler`
- Require the subagent to stop after Packet 1.4.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 4 Packet 1.4.1: Authoring Roots And Lock Paths Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 1.4 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether `layout.rs` stayed narrow rather than becoming a monolithic all-storage object
  - whether `author/charter.rs` now consumes authoring layout ownership cleanly
  - whether `author/project_context.rs` and `author/environment_inventory.rs` now consume authoring layout ownership cleanly
  - whether guided authoring, synthesis validation, canonical-truth gates, or refusal semantics regressed while storage ownership was being adopted
  - whether any prompt/template cleanup, Phase 2 target work, or Phase 3 shell-wording cleanup leaked in
- Require severity labels and explicit callouts if direct authoring target or lock-path ownership remains duplicated, if the layout owners are too broad, or if authoring semantics drifted.

Fix loop:
- If the review is clean, stop and report Packet 1.4.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.4.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.4.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.4.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1.4.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.4.1 is review-clean and committed.
- Stop and report blocked if Packet 1.4.1 cannot be completed without widening into Phase 2+, Phase 3 shell cleanup, regressing Slice 1.2 canonical semantics, or changing the approved Slice 1.4 spec/plan/tasks.
```
