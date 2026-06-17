# Handbook Engine Extraction Phase 6 Remaining Work Packet Prompts

Task source: [handbook-engine-extraction-phase-6-remaining-work-tasks.md](./handbook-engine-extraction-phase-6-remaining-work-tasks.md)
Spec source: [handbook-engine-extraction-phase-6-remaining-work-spec.md](./handbook-engine-extraction-phase-6-remaining-work-spec.md)
Plan source: [handbook-engine-extraction-phase-6-remaining-work-plan.md](./handbook-engine-extraction-phase-6-remaining-work-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation and fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps execution bounded to the approved Phase 6 remaining-work seam.

This slice is approved for Lane B (flow import-boundary proof) and Lane D (final Substrate import plan) only. Do not widen into executing the actual Substrate import, reopening Lane A, CLI shell redesign, compiler retirement, publication, crates.io work, or making `substrate-context` become handbook. If the narrow packet cannot land honestly without such widening, stop and report the blocker instead of silently expanding scope.

The orchestration session must not implement, review, or fix the packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

---

## Packet 6.B.1 Prompt — Gather Evidence

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.B.1: Gather Flow Import-Boundary Evidence in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.B.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow evidence-gathering seam: capture `cargo tree` output, `rg` coupling-exclusion output, transitive type-dependency traces, and exclusion-of-CLI/compiler/doctor/setup/pipeline evidence — all recorded into the consumer contract doc's evidence section.
- Stay inside Packet 6.B.1 only.

Hard rules:
- Do not implement, review, or fix Packet 6.B.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before editing any production symbol or helper under `crates/flow/src/**`, run GitNexus impact analysis first and report the blast radius. If the packet can close inside `docs/specs/handbook-flow-import-boundary-consumer-contract.md` plus `crates/flow/src/**` (read-only inspection only), do not widen into production-symbol edits.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.B.1 only.
- Stay inside Packet 6.B.1 scope.

Packet 6.B.1 scope:
- Capture `cargo tree -p handbook-flow` output and record it, showing only `handbook-engine` as intra-workspace dependency.
- Capture `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` output and record zero matches.
- Trace transitive type dependencies for all in-boundary public symbols from `resolver`, `budget`, `packet_result` — confirm each resolves only to `handbook-engine` types or std types. Flag any symbol whose implementation pulls in engine types beyond the engine public surface.
- Confirm exclusion of CLI/compiler/doctor/setup/pipeline concerns by source inspection of `crates/flow/src/*.rs` — confirm `use` statements reference only `crate::*`, `handbook_engine::*`, and std.
- Record all evidence into the evidence section of `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (create the file with the evidence section if it does not yet exist; the full contract formalization is Packet 6.B.2, so only write the evidence section now).
- Expected files:
  - docs/specs/handbook-flow-import-boundary-consumer-contract.md (evidence section only)

Out of scope — do NOT touch:
- Writing the full consumer contract (frozen symbol set, contract version function, exclusions section) — that is Packet 6.B.2
- Running the verification wall — that is Packet 6.B.3
- Lane C (engine optional boundary freeze)
- Lane D (import/adoption plan)
- any production code changes in `crates/flow/src/**` or any other crate source
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.B.1: Gather Flow Import-Boundary Evidence`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `cargo tree -p handbook-flow`
  - `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/`
  - `sed -n '1,60p' crates/flow/src/resolver.rs`
  - `sed -n '1,60p' crates/flow/src/budget.rs`
  - `sed -n '1,60p' crates/flow/src/packet_result.rs`
  - `sed -n '1,40p' crates/engine/src/lib.rs`
- Require the implementation to:
  - run `cargo tree -p handbook-flow` and record the exact output
  - run `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` and record the zero-match result
  - inspect each public symbol in `resolver`, `budget`, `packet_result` and trace its type dependencies to confirm engine-only or std
  - inspect all `use` statements in `crates/flow/src/*.rs` and confirm they reference only `crate::*`, `handbook_engine::*`, and std
  - record all evidence into the evidence section of `docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - avoid widening into full contract formalization, verification wall, Lane C, Lane D, or any production code changes
- Require the subagent to stop after Packet 6.B.1 acceptance is met and report touched files, evidence captured, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.B.1: Gather Flow Import-Boundary Evidence`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether `cargo tree` output was captured accurately and shows only `handbook-engine` as intra-workspace dependency
  - whether `rg` output was captured accurately and shows zero matches
  - whether transitive type-dependency traces are complete for all in-boundary public symbols
  - whether exclusion proof covers CLI shell, compiler glue, doctor/setup, and pipeline surfaces
  - whether any evidence was paraphrased rather than recorded from live output
  - whether the evidence section is internally consistent with live source
- Require severity labels and explicit callouts if evidence is incomplete, inaccurate, or if scope leaked beyond Packet 6.B.1.

Fix loop:
- If the review is clean, stop and report Packet 6.B.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.B.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.B.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.B.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the evidence-gathering work clearly and standalone.

Stop conditions:
- Stop once Packet 6.B.1 is review-clean, committed, and all four evidence tasks have verifiable output.
- Stop and report blocked if the packet cannot close honestly without widening into full contract formalization, verification wall, production code changes, Lane C, Lane D, or broader integration implementation.
```

---

## Packet 6.B.2 Prompt — Formalize Consumer Contract

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.B.2: Formalize Flow Import-Boundary Consumer Contract in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.B.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow contract-formalization seam: write the standalone consumer contract document at `docs/specs/handbook-flow-import-boundary-consumer-contract.md` that records the frozen in-boundary symbol set, their transitive type dependencies, explicit exclusions, the contract version function, and evidence references.
- This packet depends on Packet 6.B.1 being complete (evidence section already populated). Verify Packet 6.B.1 evidence exists before starting.
- Stay inside Packet 6.B.2 only.

Hard rules:
- Do not implement, review, or fix Packet 6.B.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.B.2 only.
- Stay inside Packet 6.B.2 scope.

Packet 6.B.2 scope:
- Write the full standalone consumer contract document at `docs/specs/handbook-flow-import-boundary-consumer-contract.md` that records:
  - The frozen in-boundary symbol set — all public re-exports from `budget`, `packet_result`, `resolver`:
    - From `resolver`: `resolve`, `ResolveRequest`, `ResolverResult`, `ResolverRefusal`, `ResolverRefusalCategory`, `ResolverBlocker`, `ResolverBlockerCategory`, `ResolverNextSafeAction`, `ResolverSubjectRef`, `PacketSelection`, `PacketSelectionStatus`, `C04_RESULT_VERSION`
    - From `budget`: `evaluate_budget`, `BudgetDisposition`, `BudgetOutcome`, `BudgetPolicy`, `BudgetReason`, `BudgetTarget`, `NextSafeAction`
    - From `packet_result`: `PacketResult`, `PacketSection`, `PacketSectionMode`, `PacketBodyNote`, `PacketBodyNoteKind`, `PacketDecisionSummary`, `PacketFixtureContext`, `PacketSourceSummary`, `PacketVariant`
  - Their transitive type dependencies (all engine-only or std) — carried from Packet 6.B.1 evidence
  - Explicit exclusions (CLI shell, compiler glue, doctor/setup, pipeline surfaces)
  - The contract version function (`flow_contract_version()`) and its delegation to `handbook_engine::workspace_contract_version()`
  - Evidence references (cargo tree output, rg output, source inspection conclusions from Packet 6.B.1)
- Preserve the evidence section from Packet 6.B.1 — do not overwrite or remove it; integrate it into the full document.
- Expected files:
  - docs/specs/handbook-flow-import-boundary-consumer-contract.md (full document)

Out of scope — do NOT touch:
- Running the verification wall — that is Packet 6.B.3
- Any production code changes in `crates/flow/src/**` or any other crate source
- Lane C (engine optional boundary freeze)
- Lane D (import/adoption plan)
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.B.2: Formalize Flow Import-Boundary Consumer Contract`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,80p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `rg -n "pub (fn|struct|enum|type|const)" crates/flow/src/resolver.rs crates/flow/src/budget.rs crates/flow/src/packet_result.rs`
  - `rg -n "flow_contract_version" crates/flow/src/`
  - `rg -n "workspace_contract_version" crates/engine/src/`
- Require the implementation to:
  - verify Packet 6.B.1 evidence exists in the doc before writing the full contract
  - record every public symbol from `resolver`, `budget`, `packet_result` exactly as they appear in live source
  - trace each symbol's type dependencies and confirm they resolve to engine-only or std
  - record explicit exclusions for CLI shell, compiler glue, doctor/setup, and pipeline surfaces
  - record the contract version function and its delegation chain
  - reference the Packet 6.B.1 evidence (cargo tree, rg, source inspection)
  - ensure the document is internally consistent with live source — no paraphrased or invented symbol names
  - avoid widening into verification wall, production code changes, Lane C, Lane D, or broader integration
- Require the subagent to stop after Packet 6.B.2 acceptance is met and report touched files, contract contents summary, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.B.2: Formalize Flow Import-Boundary Consumer Contract`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, and Packet 6.B.1 evidence.
- Require special attention to:
  - whether every public symbol from `resolver`, `budget`, `packet_result` is recorded and matches live source exactly
  - whether transitive type dependencies are complete and accurate (no missing symbols, no invented symbols)
  - whether explicit exclusions cover CLI shell, compiler glue, doctor/setup, and pipeline surfaces
  - whether the contract version function and its delegation chain are correctly recorded
  - whether the Packet 6.B.1 evidence section was preserved and integrated, not overwritten
  - whether the document is internally consistent — no contradictions between symbol set, type dependencies, and exclusions
  - whether any scope leaked beyond Packet 6.B.2
- Require severity labels and explicit callouts if the contract is incomplete, inaccurate, or if scope leaked.

Fix loop:
- If the review is clean, stop and report Packet 6.B.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.B.2-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.B.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.B.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the consumer contract formalization clearly and standalone.

Stop conditions:
- Stop once Packet 6.B.2 is review-clean, committed, and the consumer contract doc is internally consistent with live source.
- Stop and report blocked if the packet cannot close honestly without widening into verification wall, production code changes, Lane C, Lane D, or broader integration implementation.
```

---

## Packet 6.B.3 Prompt — Verification Wall

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.B.3: Run Flow Import-Boundary Verification Wall in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.B.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow verification seam: run the full Lane B verification wall, record pass/fail for each command, and record completion notes in the tasks doc.
- This packet depends on Packets 6.B.1 and 6.B.2 being complete. Verify both are landed before starting.
- Stay inside Packet 6.B.3 only.

Hard rules:
- Do not implement, review, or fix Packet 6.B.3 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.B.3 only.
- Stay inside Packet 6.B.3 scope.

Packet 6.B.3 scope:
- Run each of the following verification commands and record pass/fail:
  - `cargo tree -p handbook-flow` — must show only `handbook-engine` as intra-workspace dependency
  - `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` — must return zero matches
  - `cargo test -p handbook-flow` — must pass
  - `cargo check --workspace` — must pass
  - `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` — must pass
- Record completion notes in `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (Packet 6.B.3 section) with pass/fail status for each command.
- If any command fails, stop and report the blocker — do not attempt to fix the underlying issue in this packet without explicit direction.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md (completion notes section)

Out of scope — do NOT touch:
- Any production code changes in any crate
- Lane C (engine optional boundary freeze)
- Lane D (import/adoption plan)
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.B.3: Run Flow Import-Boundary Verification Wall`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,40p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
- Require the implementation to:
  - verify Packets 6.B.1 and 6.B.2 are complete (evidence section and full contract exist in the consumer contract doc)
  - run `cargo tree -p handbook-flow` and record the exact output with pass/fail
  - run `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` and record the result with pass/fail
  - run `cargo test -p handbook-flow` and record pass/fail
  - run `cargo check --workspace` and record pass/fail
  - run `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` and record pass/fail
  - record all results in the Packet 6.B.3 completion notes section of `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`
  - if any command fails, stop and report the exact failure output — do not attempt fixes
  - avoid widening into production code changes, Lane C, Lane D, or broader integration
- Require the subagent to stop after Packet 6.B.3 acceptance is met and report touched files, verification results, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.B.3: Run Flow Import-Boundary Verification Wall`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, and the recorded verification results.
- Require special attention to:
  - whether all five verification commands were run and their output was recorded accurately
  - whether pass/fail status is honest and matches actual command output
  - whether any command output was paraphrased or omitted
  - whether the completion notes are consistent with the consumer contract doc and live source
  - whether any scope leaked beyond Packet 6.B.3
- Require severity labels and explicit callouts if any verification result is missing, inaccurate, or if scope leaked.

Fix loop:
- If the review is clean, stop and report Packet 6.B.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.B.3-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.B.3 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.B.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the verification wall results clearly and standalone.

Stop conditions:
- Stop once Packet 6.B.3 is review-clean, committed, and all five verification commands have honest pass/fail records.
- Stop and report blocked if any verification command fails and the failure cannot be resolved within Packet 6.B.3 scope (i.e., the failure requires production code changes or widening into another lane).
```

---

## Packet 6.C.1 Prompt — Defer Or Activate (Decision Task)

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.C.1: Record Lane C Deferral Decision in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.C.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow decision-recording seam: explicitly record that Lane C (engine optional boundary freeze) is deferred, with rationale that engine's current public surface is the working boundary and Lane C can be activated later if Lane D's import plan indicates a narrower surface is needed.
- Stay inside Packet 6.C.1 only.

Hard rules:
- Do not implement, review, or fix Packet 6.C.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.C.1 only.
- Stay inside Packet 6.C.1 scope.

Packet 6.C.1 scope:
- Record in the Lane C section of `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` that Lane C is deferred.
- Include rationale: engine's current public surface is the working boundary; if Lane D's import plan indicates a narrower surface is needed, Lane C can be activated at that time.
- Reference the spec's Lane C section for the activation condition.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md (Lane C section)

Out of scope — do NOT touch:
- Activating Lane C (gathering evidence, writing a consumer contract, running verification wall)
- Any production code changes in any crate
- Lane B or Lane D work
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.C.1: Record Lane C Deferral Decision`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '/Lane C/,/Lane D/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`
  - `sed -n '/Lane C/,/Lane D/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
- Require the implementation to:
  - read the Lane C section in both the tasks doc and spec doc
  - record an explicit "deferred" decision with rationale in the tasks doc Lane C section
  - reference the activation condition from the spec
  - avoid widening into Lane C activation, Lane B, Lane D, or any production code changes
- Require the subagent to stop after Packet 6.C.1 acceptance is met and report touched files, decision recorded, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.C.1: Record Lane C Deferral Decision`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, and tasks.
- Require special attention to:
  - whether the deferral decision is explicit and unambiguous
  - whether the rationale is consistent with the spec's Lane C section
  - whether the activation condition is referenced correctly
  - whether any scope leaked beyond Packet 6.C.1
- Require severity labels and explicit callouts if the decision is ambiguous, inconsistent with the spec, or if scope leaked.

Fix loop:
- If the review is clean, stop and report Packet 6.C.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.C.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.C.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.C.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Lane C deferral decision clearly and standalone.

Stop conditions:
- Stop once Packet 6.C.1 is review-clean, committed, and the deferral decision is explicit with rationale.
- Stop and report blocked if the packet cannot close honestly without widening into Lane C activation, Lane B, Lane D, or broader integration implementation.
```

---

## Packet 6.D.1 Prompt — Write Import/Adoption Plan

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.D.1: Write Final Substrate Import/Adoption Plan in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.D.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as one narrow planning-artifact seam: write a standalone import/adoption plan doc at `docs/specs/handbook-substrate-import-adoption-plan.md` that records import order, rationale, per-crate frozen boundary summaries, adapter/facade assessment, import verification gates, and Substrate-side constraints.
- This packet depends on Lane B being complete (Packets 6.B.1, 6.B.2, 6.B.3 all landed) and Lane C being deferred (Packet 6.C.1 landed). Verify all are complete before starting.
- Stay inside Packet 6.D.1 only.

Hard rules:
- Do not implement, review, or fix Packet 6.D.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.D.1 only.
- Stay inside Packet 6.D.1 scope.

Packet 6.D.1 scope:
- Write a standalone doc at `docs/specs/handbook-substrate-import-adoption-plan.md` that records:
  - Import order: engine first (no intra-workspace deps), then pipeline (depends on engine), then flow (depends on engine)
  - Rationale for the phased order
  - Per-crate frozen boundary summary:
    - Engine: current public surface (Lane C deferred) — modules: `artifact_manifest`, `author`, `baseline_validation`, `canonical_artifacts`, `freshness`, plus `workspace_contract_version()` and `engine_contract_version()`
    - Pipeline: documented frozen subset from Lane A closeout — in-boundary modules: `pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, plus `pipeline_contract_version()`
    - Flow: Lane B consumer contract — in-boundary symbols from `resolver`, `budget`, `packet_result` (reference the consumer contract doc)
  - Adapter/facade assessment (current evidence: none needed; record the assessment with evidence)
  - Import verification gate per phase (what checks Substrate must pass after importing each crate)
  - Substrate-side constraints (resolved from live repo inspection, 2026-06-17):
    - License field: add `license = "MIT"` to the three crate Cargo.toml files before import
    - Workspace integration: recommend workspace member pattern (path deps) vs external dep
    - YAML crate divergence: `serde_yaml_bw` (handbook) vs `serde_yaml` (substrate) — record keep-both or migrate decision
    - No feature flags needed; edition/resolver/sha2/libc/serde all compatible
- Reference the Lane B consumer contract doc, Lane A closeout archive, and the spec's substrate-side constraints table as provenance.
- Expected files:
  - docs/specs/handbook-substrate-import-adoption-plan.md (full document)

Out of scope — do NOT touch:
- Executing the actual Substrate import — this is a planning artifact only
- Any production code changes in any crate
- Lane B or Lane C work
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Remaining Work Packet 6.D.1: Write Final Substrate Import/Adoption Plan`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,60p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `ls docs/specs/archive/phase-6-pipeline-boundary-cleanup/`
  - `sed -n '/Substrate-Side Constraints/,/Resolved Open Questions/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
  - `rg -n "pub (fn|struct|enum|mod)" crates/engine/src/lib.rs`
  - `rg -n "pub (fn|struct|enum|mod)" crates/pipeline/src/lib.rs`
- Require the implementation to:
  - verify Lane B (Packets 6.B.1–6.B.3) and Lane C (Packet 6.C.1) are complete before writing the plan
  - record the import order with rationale (engine first, then pipeline, then flow)
  - record per-crate frozen boundary summaries consistent with live source and the Lane A/B closeout artifacts
  - record the adapter/facade assessment with evidence (current evidence: none needed)
  - record the import verification gate per phase (what checks Substrate must pass after importing each crate)
  - record all four Substrate-side constraints from the spec's constraints table
  - reference the Lane B consumer contract doc, Lane A closeout archive, and spec constraints table as provenance
  - ensure the document is internally consistent with live crate surfaces and frozen boundaries
  - avoid widening into actual import execution, Lane B/C work, or broader integration
- Require the subagent to stop after Packet 6.D.1 acceptance is met and report touched files, plan contents summary, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.D.1: Write Final Substrate Import/Adoption Plan`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, Lane A closeout archive, and Lane B consumer contract.
- Require special attention to:
  - whether the import order and rationale are correct and consistent with dependency truth (engine has no intra-workspace deps; pipeline depends on engine; flow depends on engine)
  - whether per-crate frozen boundary summaries match live source and Lane A/B closeout artifacts exactly
  - whether the adapter/facade assessment is honest and evidence-backed
  - whether the import verification gates are concrete and testable
  - whether all four Substrate-side constraints are recorded with correct values from the spec
  - whether the document is internally consistent — no contradictions between crate summaries, import order, and verification gates
  - whether the plan stays a planning artifact and does not execute any import
  - whether any scope leaked beyond Packet 6.D.1
- Require severity labels and explicit callouts if the plan is incomplete, inaccurate, contradicts frozen boundaries, or if scope leaked.

Fix loop:
- If the review is clean, stop and report Packet 6.D.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.D.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.D.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.D.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the import/adoption plan authoring clearly and standalone.

Stop conditions:
- Stop once Packet 6.D.1 is review-clean, committed, and the import plan doc is internally consistent with live crate surfaces and frozen boundaries.
- Stop and report blocked if the packet cannot close honestly without widening into actual import execution, Lane B/C work, or broader integration implementation.
```

---

## Packet 6.D.2 Prompt — Human Review Gate

```text
/goal Orchestrate Phase 6 Remaining Work Packet 6.D.2: Human Review Gate for Final Substrate Import/Adoption Plan in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.D.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md
- Treat the packet as a human-review-gate seam: prepare the import plan for human review, present it to the user for sign-off, and address any review feedback the user provides.
- This packet depends on Packet 6.D.1 being complete (import plan doc written and review-clean). Verify Packet 6.D.1 is landed before starting.
- Stay inside Packet 6.D.2 only.

Hard rules:
- Do not implement, review, or fix Packet 6.D.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for the preparation phase.
- The preparation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After preparation completes, commit any preparation changes before presenting to the user.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the automated review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- After the automated review is clean, the orchestrator must present the import plan to the user for human sign-off.
- If the human reviewer provides feedback, spawn a fresh GPT-5.4 subagent on high to address the feedback using $incremental-implementation.
- Commit after each accepted feedback-fix round.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.D.2 only.
- Stay inside Packet 6.D.2 scope.

Packet 6.D.2 scope:
- Verify Packet 6.D.1 is complete (import plan doc exists and is review-clean).
- Run a fresh automated review of the import plan against:
  - The three crate surfaces (engine, pipeline, flow)
  - The frozen boundaries (Lane A closeout, Lane B consumer contract)
  - The root plan's migration gate
  - The spec's substrate-side constraints
- Present the import plan to the user for human sign-off.
- Record human sign-off (or feedback) in the Packet 6.D.2 completion notes section of `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`.
- If the human reviewer provides feedback, address it with a fix subagent and re-present.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md (completion note)
  - docs/specs/handbook-substrate-import-adoption-plan.md (if feedback requires edits)

Out of scope — do NOT touch:
- Executing the actual Substrate import
- Any production code changes in any crate
- Lane B or Lane C work
- Reopening Lane A
- any CLI shell, compiler, doctor/setup, or pipeline surface changes
- publication or crates.io work
- any new packet authoring beyond this approved packet prompt artifact

Preparation subagent prompt requirements:
- Begin with `/goal Prepare Phase 6 Remaining Work Packet 6.D.2: Human Review Gate for Final Substrate Import/Adoption Plan`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,200p' docs/specs/handbook-substrate-import-adoption-plan.md`
  - `sed -n '/Lane A status/,/Tech Stack/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
  - `sed -n '/Substrate-Side Constraints/,/Resolved Open Questions/p' docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
- Require the preparation to:
  - verify Packet 6.D.1 is complete (import plan doc exists)
  - verify the import plan is internally consistent with the three crate surfaces, frozen boundaries, and substrate-side constraints
  - prepare a concise review summary for the human reviewer highlighting: import order, per-crate boundary posture, adapter assessment, verification gates, and substrate-side constraints
  - flag any inconsistencies or gaps found during preparation for the human reviewer's attention
  - avoid widening into actual import execution, Lane B/C work, or broader integration
- Require the subagent to stop after preparation is complete and report the review summary, any flagged issues, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Remaining Work Packet 6.D.2: Human Review Gate for Final Substrate Import/Adoption Plan`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the import plan against:
  - the three crate surfaces (engine, pipeline, flow) — verify boundary summaries match live source
  - the frozen boundaries (Lane A closeout archive, Lane B consumer contract)
  - the root plan's migration gate
  - the spec's substrate-side constraints table
- Require special attention to:
  - whether the plan is consistent with live crate surfaces and frozen boundaries
  - whether the import order and rationale are correct
  - whether the substrate-side constraints are accurately recorded
  - whether the plan stays a planning artifact and does not execute any import
  - whether any scope leaked beyond Packet 6.D.2
- Require severity labels and explicit callouts if the plan contradicts frozen boundaries, has missing constraints, or if scope leaked.

Human review gate:
- After the automated review is clean, the orchestrator must present the import plan to the user.
- The presentation must include: the plan doc path, a concise summary of import order/boundaries/gates/constraints, and any flagged issues from the automated review.
- Wait for the user's sign-off or feedback.
- If the user provides feedback, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation` to address the feedback.
- The fix prompt must cite the exact human feedback and require only the minimal Packet-6.D.2-bounded changes needed to close it.
- Commit accepted feedback-fixes.
- Re-present the updated plan to the user for final sign-off.
- Record the human sign-off in the Packet 6.D.2 completion notes section of `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`.

Fix loop (automated review):
- If the automated review is clean, proceed to the human review gate.
- If the automated review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Fix loop (human feedback):
- If the human reviewer provides feedback, spawn one fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Commit accepted fixes.
- Re-present the updated plan to the user.

Commit policy:
- Commit once after preparation if Packet 6.D.2 preparation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.D.2 only.
- Commit after each accepted fix round (automated or human-feedback).
- Commit messages must describe the human review gate preparation, feedback addressing, or sign-off recording clearly and standalone.

Stop conditions:
- Stop once Packet 6.D.2 is complete: human sign-off is recorded in the tasks doc and any feedback has been addressed and committed.
- Stop and report blocked if the human reviewer rejects the plan and the feedback cannot be addressed within Packet 6.D.2 scope (i.e., the feedback requires reopening Lane A/B/C, production code changes, or actual import execution).
```
