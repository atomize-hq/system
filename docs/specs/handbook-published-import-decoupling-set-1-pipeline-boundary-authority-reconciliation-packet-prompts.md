# Handbook Published-Import Decoupling — Set 1 Packet Prompts

Task source: [handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md](./handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md)
Spec source: [handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md](./handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md)
Plan source: [handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md](./handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md)
Map source: [MAP.md](./MAP.md)

These prompts are ready to paste into fresh orchestration sessions. Each one:
- starts in `/goal`
- requires a fresh **GPT-5.4 high** implementation subagent
- requires that implementation subagent to use `$incremental-implementation`
- requires a fresh **GPT-5.4 high** review subagent
- requires that review subagent to use `$code-review-and-quality`
- requires a fresh **GPT-5.4 high** fix subagent for every review round that finds issues
- requires commit boundaries between implementation, review, and each accepted fix round
- keeps execution bounded to one packet only

Set 1 is docs-only. The orchestration session must not implement, review, or fix the packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

Preserve unrelated local edits, especially incidental dirt in `AGENTS.md` and `CLAUDE.md`. In `system`, run GitNexus detect-changes before every commit and confirm the affected scope matches only the current packet. Because Set 1 is docs-only, do not widen into Rust implementation work, CLI/compiler/product-shell redesign, or downstream Substrate source-touching work.

---

## Packet 1.1 Prompt — Current-State Evidence Matrix

```text
/goal Orchestrate Set 1 Packet 1.1: Current-State Evidence Matrix in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md
  - /Users/spensermcconnell/__Active_Code/system/docs/ideas/handbook-substrate-packet-4-2-proof-findings.md
- Treat this as one narrow docs/evidence seam: produce the side-by-side claim matrix and reproduce the positive/negative external-consumer proofs that distinguish the proven `engine + flow` seam from the still-private `pipeline` seam.
- Stay inside Packet 1.1 only.

Hard rules:
- Do not implement, review, or fix Packet 1.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.1 only.
- Stay inside Packet 1.1 scope.

Packet 1.1 scope:
- Add the side-by-side claim matrix to the active Set 1 docs.
- Reproduce the positive external-consumer compile for `handbook-engine = "=0.1.1"` + `handbook-flow = "=0.1.1"`.
- Reproduce the negative external-consumer compile for `handbook-pipeline = "=0.1.1"` importing `handbook_pipeline::layout::PipelineStorageLayoutContract`.
- Record the evidence honestly in the active Set 1 docs.
- Expected files:
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - optionally HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md if a bounded completion note is truly needed

Out of scope — do NOT touch:
- Packet 1.2 boundary-shape decision content beyond whatever Packet 1.1 evidence must support
- Packet 1.3 active-authority supersession wording beyond minimal Packet 1.1 evidence notes
- Packet 1.4 closeout notes
- Any Rust production files
- Any Set 2 or Set 3 implementation/proof work
- CLI/compiler/product-shell redesign
- Downstream Substrate source-touching work

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 1 Packet 1.1: Current-State Evidence Matrix`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/MAP.md`
  - `sed -n '1,260p' HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
  - `sed -n '1,260p' docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`
  - `sed -n '1,120p' crates/pipeline/src/lib.rs`
  - `sed -n '1,220p' crates/pipeline/src/declarative_roots.rs`
  - `sed -n '1,260p' crates/pipeline/src/layout.rs`
  - `cargo check --workspace`
- Require the implementation to:
  - keep the packet docs-only
  - add the explicit claim matrix aligned to the MAP objective/intent
  - reproduce and record the positive `engine + flow` proof and the negative `pipeline` proof
  - stop after Packet 1.1 acceptance is met and report exact files touched, exact verification run, and any residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 1 Packet 1.1: Current-State Evidence Matrix`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, interpreted for a docs/evidence packet.
- Require the reviewer to check:
  - whether the matrix is specific and source-grounded
  - whether the positive/negative proof story is honest and reproducible
  - whether the packet stayed docs-only
  - whether any content drifted into Packet 1.2/1.3/1.4 territory
  - whether the MAP objective/intent is reflected explicitly
- Require severity labels and explicit callouts for stale claims, weak evidence, or scope drift.

Fix loop:
- If the review is clean, stop and report Packet 1.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the evidence-matrix / external-proof work clearly and standalone.

Stop conditions:
- Stop once Packet 1.1 is review-clean, committed, and the active Set 1 docs contain the required evidence matrix plus the reproduced proof story.
- Stop and report blocked if honest completion requires Rust implementation work, broader authority rewrites, or downstream Substrate source changes.
```

---

## Packet 1.2 Prompt — Boundary-Shape Decision And Set 2 Target

```text
/goal Orchestrate Set 1 Packet 1.2: Boundary-Shape Decision And Set 2 Target in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
- Treat this as one narrow decision seam: lock that full reusable `handbook-pipeline` capability for Substrate is required, then define the narrowest stable public boundary shape Set 2 should implement.
- Packet 1.1 must already be complete.
- Stay inside Packet 1.2 only.

Hard rules:
- Do not implement, review, or fix Packet 1.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.2 only.
- Stay inside Packet 1.2 scope.

Packet 1.2 scope:
- Lock that full reusable `handbook-pipeline` capability for Substrate is required.
- Explain why Packet 4.2 does not yet satisfy that requirement.
- Decide whether Set 2 should satisfy the requirement through direct module/type promotion or a narrower public façade.
- Define the Set 2 acceptance wall for the minimal public capability boundary.
- Expected files:
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md

Out of scope — do NOT touch:
- Packet 1.1 evidence matrix content except for minimal carry-forward references
- Packet 1.3 supersession wiring
- Packet 1.4 closeout notes
- Any Rust production files
- Any Set 2 implementation or Set 3 proof execution
- CLI/compiler/product-shell redesign
- Downstream Substrate source-touching work

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 1 Packet 1.2: Boundary-Shape Decision And Set 2 Target`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/MAP.md`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`
  - `sed -n '1,220p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`
  - `rg -n "full reusable|minimum reviewed public surface|Packet 4.2|Set 2|handbook-pipeline" docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md`
- Require the implementation to:
  - keep the packet docs-only
  - lock the required capability explicitly
  - define the narrowest stable Set 2 boundary shape honestly
  - define the Set 2 acceptance wall without pretending implementation/proof already exists
  - stop after Packet 1.2 acceptance is met and report touched files, decision taken, and unresolved questions

Review subagent prompt requirements:
- Begin with `/goal Review Set 1 Packet 1.2: Boundary-Shape Decision And Set 2 Target`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, interpreted for a planning/decision packet.
- Require the reviewer to check:
  - whether the capability requirement is explicit and aligned to the MAP
  - whether the boundary-shape decision avoids unnecessary public surface
  - whether the Set 2 acceptance wall is specific enough to execute
  - whether Packet 4.2 is still classified honestly
  - whether scope drifted into actual Set 2 implementation
- Require severity labels and explicit callouts for vague decisions, overexposure risk, or missing proof criteria.

Fix loop:
- If the review is clean, stop and report Packet 1.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.2-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the boundary-shape / Set 2 target work clearly and standalone.

Stop conditions:
- Stop once Packet 1.2 is review-clean, committed, and Set 2 has a clear required capability target plus a concrete acceptance wall.
- Stop and report blocked if honest completion requires real Rust implementation, broader authority rewrites, or downstream Substrate source changes.
```

---

## Packet 1.3 Prompt — Active Authority Reconciliation

```text
/goal Orchestrate Set 1 Packet 1.3: Active Authority Reconciliation in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md
- Treat this as one narrow authority seam: make `docs/specs/MAP.md` plus the active Set 1 triplet the obvious main authority, preserve the archive as provenance only, and bound the stale `9b83` Substrate notes as non-canonical context.
- Packets 1.1 and 1.2 must already be complete.
- Stay inside Packet 1.3 only.

Hard rules:
- Do not implement, review, or fix Packet 1.3 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.3 only.
- Stay inside Packet 1.3 scope.

Packet 1.3 scope:
- Make the active authority hierarchy explicit.
- Add any needed supersession wording to the active Set 1 docs and, only if truly needed, a bounded note in the root audit.
- Preserve archive materials as provenance rather than execution authority.
- Bound the `9b83` Substrate notes as stale but useful non-canonical context.
- Expected files:
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - optionally HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md if a narrow supersession note is truly required

Out of scope — do NOT touch:
- Packet 1.4 completion notes beyond minimal scaffolding already present
- Any Rust production files
- Any Set 2 implementation or Set 3 proof execution
- CLI/compiler/product-shell redesign
- Downstream Substrate source-touching work

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 1 Packet 1.3: Active Authority Reconciliation`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/MAP.md`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`
  - `sed -n '1,220p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`
  - `rg -n "docs/specs/MAP.md|supersede|superseded|archive|provenance|non-canonical|9b83" docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-*.md HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
- Require the implementation to:
  - keep the packet docs-only
  - make the authority stack explicit and easy for a fresh session to follow
  - avoid rewriting archive history as if it never happened
  - label stale `9b83` context as non-canonical
  - stop after Packet 1.3 acceptance is met and report touched files plus any remaining authority ambiguities

Review subagent prompt requirements:
- Begin with `/goal Review Set 1 Packet 1.3: Active Authority Reconciliation`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, interpreted for authority/planning docs.
- Require the reviewer to check:
  - whether `docs/specs/MAP.md` plus the active Set 1 triplet now read as the main authority
  - whether archive materials are treated as provenance only
  - whether any supersession wording is precise and non-destructive
  - whether stale Substrate context is clearly bounded as non-canonical
  - whether scope drifted into Packet 1.4 or Set 2 work
- Require severity labels and explicit callouts for ambiguous authority, over-claiming, or unnecessary rewriting.

Fix loop:
- If the review is clean, stop and report Packet 1.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.3-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.3 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the active-authority reconciliation clearly and standalone.

Stop conditions:
- Stop once Packet 1.3 is review-clean, committed, and `docs/specs/MAP.md` plus the active Set 1 triplet are the obvious main authority for this seam.
- Stop and report blocked if honest completion requires Rust implementation work, broader archive surgery, or downstream Substrate source changes.
```

---

## Packet 1.4 Prompt — Final Set Proof And Handoff

```text
/goal Orchestrate Set 1 Packet 1.4: Final Set Proof And Handoff in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
- Treat this as one narrow closeout seam: rerun the docs-only proof wall, record the final Set 1 completion state honestly, and leave explicit Set 2 plus Set 3 stop boundaries.
- Packets 1.1, 1.2, and 1.3 must already be complete.
- Stay inside Packet 1.4 only.

Hard rules:
- Do not implement, review, or fix Packet 1.4 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.4 only.
- Stay inside Packet 1.4 scope.

Packet 1.4 scope:
- Re-run the Set 1 docs-only proof wall.
- Confirm the positive and negative external-consumer proofs are still reflected honestly.
- Confirm no Rust source changed as part of Set 1.
- Record final completion notes in the tasks doc.
- Record explicit Set 2 and Set 3 stop boundaries.
- Expected files:
  - docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - optionally docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md if stop-boundary wording truly needs adjustment
  - optionally HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md if a bounded closeout pointer is truly needed

Out of scope — do NOT touch:
- Reopening Packet 1.1, 1.2, or 1.3 except to record honest contradictions that block closeout
- Any Rust production files
- Any Set 2 implementation or Set 3 proof execution
- CLI/compiler/product-shell redesign
- Downstream Substrate source-touching work

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 1 Packet 1.4: Final Set Proof And Handoff`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/MAP.md`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`
  - `sed -n '1,240p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`
  - `cargo check --workspace`
  - `git diff -- docs/specs/MAP.md docs/specs HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
- Require the implementation to:
  - keep the packet docs-only
  - record final closeout notes honestly
  - explicitly state what Set 2 may do and what it must not silently widen into
  - explicitly state what Set 3 must prove and guard against
  - stop after Packet 1.4 acceptance is met and report touched files, proof wall run, and any remaining blockers to closeout

Review subagent prompt requirements:
- Begin with `/goal Review Set 1 Packet 1.4: Final Set Proof And Handoff`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, interpreted for closeout docs.
- Require the reviewer to check:
  - whether the closeout notes are honest and specific
  - whether Set 2 and Set 3 boundaries are explicit and executable
  - whether any unresolved contradiction from Packets 1.1–1.3 was hidden instead of reported
  - whether the packet stayed docs-only
- Require severity labels and explicit callouts for hidden blockers, weak stop-boundary language, or false completeness.

Fix loop:
- If the review is clean, stop and report Packet 1.4 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.4-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.4 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.4 only.
- Commit after each accepted fix round.
- Commit messages must describe the Set 1 closeout / handoff work clearly and standalone.

Stop conditions:
- Stop once Packet 1.4 is review-clean, committed, and Set 1 has an honest closeout plus explicit Set 2 and Set 3 handoff boundaries.
- Stop and report blocked if honest closeout requires Rust implementation work, reopening earlier packets, or downstream Substrate source changes.
```
