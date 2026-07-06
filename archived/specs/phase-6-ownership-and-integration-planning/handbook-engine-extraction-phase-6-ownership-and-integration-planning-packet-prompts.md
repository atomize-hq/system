# Handbook Engine Extraction Phase 6 Ownership And Integration Planning Packet Prompts

Task source: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md)
Spec source: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md)
Plan source: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation and fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps this family strictly at ownership/integration planning scope rather than code execution, crate publication, or Substrate integration implementation.

These packets are expected to be planning/documentation work only. Do not edit production code, move crates, publish crates, or wire Substrate consumption from these prompts. If live repo truth suggests production code must change before an honest ownership call can be made, capture that as a blocker or deferred downstream seam rather than silently implementing it here.

The orchestration session must not implement, review, or fix the packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed in each packet prompt.

Do not advance to the next packet until the current packet is review-clean and committed.

## Packet 1 Prompt

```text
/goal Orchestrate Phase 6 Ownership And Integration Planning Packet 1: Freeze Current Authority And Scope Guard in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md.
- Use the planning family docs at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md
- Use the prerequisite authorities at:
  - /Users/spensermcconnell/__Active_Code/system/HANDBOOK_ENGINE_EXTRACTION_PLAN.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-slice-map.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md
- Treat this packet as docs-only planning work that establishes the current authority chain and hard scope guard for the new family.
- Do not start Packet 2, 3, or 4.

Hard rules:
- Do not implement, review, or fix Packet 1 work in the orchestration session yourself. Orchestrate it through the required fresh subagents.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- This packet is expected to be docs/planning-only. Do not edit production code.
- If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1 only.
- Stay inside Packet 1 scope.

Packet 1 scope:
- Record the current branch / HEAD / working-tree posture for this planning family.
- Record the docs-only delta from `aa882af42792a250cc02a6740bd1e2123178caff` to current HEAD.
- Re-state the root ownership decision rule and the family’s hard boundaries.
- Ensure the triplet makes explicit that this family starts after the READY Phase 6 reassessment and remains planning-only.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md

Out of scope:
- any per-crate final ownership call beyond the scope guard needed to frame later packets
- any code implementation
- any crate publication, versioning, or crates.io work
- any Substrate integration implementation
- any packet-prompt authoring beyond landing Packet 1 itself

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Ownership And Integration Planning Packet 1: Freeze Current Authority And Scope Guard`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `git rev-parse HEAD`
  - `git log --oneline --decorate -20`
  - `git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..HEAD`
  - `git diff --name-only aa882af42792a250cc02a6740bd1e2123178caff..HEAD`
  - `rg -n "Phase 6|Final ownership decision rule|Exit criteria|Open Questions" HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `rg -n "Phase 6|fully landed through Slice 5.3|next authoritative step" docs/specs/handbook-engine-extraction-slice-map.md`
  - `rg -n "Phase 6|next authoritative step|Set 3|Set 4" docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
  - `rg -n "READY|Packet 6\.1\.4|ownership-and-integration-planning|default_canonical_layout_contract" docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
- Require the subagent to explicitly record:
  - current branch / HEAD truth
  - whether the delta from `aa882af...` to HEAD is docs-only
  - that Phase 6 Slice 1 is already READY and is the prerequisite gate
  - the root ownership rule and planning-only boundaries
  - any contradiction between the new triplet and the prerequisite authority set
- Require the subagent to stop after Packet 1 acceptance is met and report touched files, verification outputs, contradiction list if any, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Ownership And Integration Planning Packet 1: Freeze Current Authority And Scope Guard`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, applied to the packet’s planning/docs updates.
- Require the reviewer to review the packet against the new family spec/plan/tasks plus the prerequisite Phase 6 Slice 1 authority docs.
- Require special attention to:
  - whether the READY prerequisite gate is stated concretely rather than implied
  - whether the docs-only delta from `aa882af...` to HEAD is described honestly
  - whether the root ownership rule is repeated accurately
  - whether the packet stayed planning-only and did not jump into per-crate execution work
- Require severity labels and explicit callouts if the authority chain is incomplete, if assumptions are unstated, or if the packet widened beyond scope.

Fix loop:
- If the review is clean, stop and report Packet 1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1-bounded changes needed to close them.
- Re-run only the verification commands affected by the fix. Do not rerun unchanged verification commands just for reassurance.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1 authority/scope-guard change clearly and standalone.

Stop conditions:
- Stop once Packet 1 is review-clean, committed, and the family’s starting authority/scope guard is explicit.
- Stop and report blocked if Packet 1 finds an authority contradiction that prevents an honest planning baseline.
```

## Packet 2 Prompt

```text
/goal Orchestrate Phase 6 Ownership And Integration Planning Packet 2: Decide Handbook-Owned Imported-Core Boundaries in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md.
- Assume Packet 1 is already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the planning family docs at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md
- Use the prerequisite authorities and evidence at:
  - /Users/spensermcconnell/__Active_Code/system/HANDBOOK_ENGINE_EXTRACTION_PLAN.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md
  - /Users/spensermcconnell/__Active_Code/system/crates/engine/src/lib.rs
  - /Users/spensermcconnell/__Active_Code/system/crates/engine/src/canonical_paths.rs
  - /Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/lib.rs
  - /Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_catalog.rs
- Do not reopen Packet 1 except where a tiny documentation correction is strictly required for Packet 2 to land correctly.
- Do not start Packet 3 or 4.

Hard rules:
- Do not implement, review, or fix Packet 2 work in the orchestration session yourself. Orchestrate it through the required fresh subagents.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- This packet is expected to be docs/analysis-only. Do not edit production code.
- If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2 only.
- Stay inside Packet 2 scope.

Packet 2 scope:
- Lock the `handbook-engine` ownership/import posture.
- Lock the `handbook-pipeline` ownership/import posture.
- State whether handbook remains the architectural owner for each, what Substrate’s posture is, and what the intended import boundary is.
- Name any remaining pipeline-specific deferred cleanup seam instead of hiding it in the ownership call.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md

Out of scope:
- `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` final posture work beyond tiny cross-references strictly required for consistency
- any code implementation
- any crate publication or crates.io planning
- any Substrate integration implementation
- any claim that pipeline is fully decoupled if the current evidence still shows bounded compiler-backed fixture/support coupling

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Ownership And Integration Planning Packet 2: Decide Handbook-Owned Imported-Core Boundaries`.
- Tell the subagent to use $incremental-implementation.
- Require evidence gathering with:
  - `rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs`
  - `rg -n "default_canonical_layout_contract|workspace_contract_version" crates/engine/src/lib.rs crates/engine/src/canonical_paths.rs`
  - `rg -n "PipelineCapture|PipelineHandoff|RouteState|template_library|stage_10_feature_spec" crates/pipeline/src crates/pipeline/tests`
  - `cargo tree -p handbook-engine`
  - `cargo tree -p handbook-pipeline`
  - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
- Require the subagent to produce explicit calls for `handbook-engine` and `handbook-pipeline` that answer:
  - who the architectural owner is
  - whether Substrate should import through the current public surface or whether a thinner adapter is still needed
  - what exact repo-level boundary text should be recorded
  - what residual cleanup belongs in a later bounded seam rather than this packet
- Require the subagent to keep the engine and pipeline calls distinct rather than collapsing them into one generic “publish/import” verdict.
- Require the subagent to stop after Packet 2 acceptance is met and report touched files, evidence used, per-crate decisions, deferred seam list, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Ownership And Integration Planning Packet 2: Decide Handbook-Owned Imported-Core Boundaries`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, applied to the packet’s ownership-boundary documentation.
- Require the reviewer to review the packet against the new family spec/plan/tasks, the root Phase 6 ownership rule, and the live engine/pipeline evidence.
- Require special attention to:
  - whether `handbook-engine` and `handbook-pipeline` each have explicit owner / posture / boundary / deferred-seam treatment
  - whether the engine boundary reflects the post-`aa882af...` generic default layout contract truth
  - whether pipeline coupling is described honestly rather than either ignored or overstated as a blocker
  - whether the packet stayed out of publication/execution planning
- Require severity labels and explicit callouts if the two crates are collapsed together, if evidence is missing, or if the docs overclaim portability.

Fix loop:
- If the review is clean, stop and report Packet 2 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2-bounded changes needed to close them.
- Re-run only the evidence/verification commands affected by the fix.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2 implementation if it lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2 engine/pipeline ownership-boundary decision clearly and standalone.

Stop conditions:
- Stop once Packet 2 is review-clean, committed, and `handbook-engine` plus `handbook-pipeline` each have an explicit owner / import / boundary posture.
- Stop and report blocked if Packet 2 cannot make an honest imported-core decision without first repairing a concrete earlier-seam regression.
```

## Packet 3 Prompt

```text
/goal Orchestrate Phase 6 Ownership And Integration Planning Packet 3: Decide Handbook-Side Deferred Boundaries And Non-Targets in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md.
- Assume Packets 1 and 2 are already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the planning family docs at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md
- Use the live evidence at:
  - /Users/spensermcconnell/__Active_Code/system/crates/flow/src/lib.rs
  - /Users/spensermcconnell/__Active_Code/system/crates/flow/src/resolver.rs
  - /Users/spensermcconnell/__Active_Code/system/crates/compiler/src/lib.rs
  - /Users/spensermcconnell/__Active_Code/system/crates/cli/src/main.rs
  - /Users/spensermcconnell/__Active_Code/system/crates/cli/src/
- Do not reopen Packets 1 or 2 except where a tiny documentation correction is strictly required for Packet 3 to land correctly.
- Do not start Packet 4.

Hard rules:
- Do not implement, review, or fix Packet 3 work in the orchestration session yourself. Orchestrate it through the required fresh subagents.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- This packet is expected to be docs/analysis-only. Do not edit production code.
- If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3 only.
- Stay inside Packet 3 scope.

Packet 3 scope:
- Lock the `handbook-flow` ownership posture without overclaiming a move target.
- Lock the `handbook-cli` product-shell boundary explicitly.
- Lock retained `handbook-compiler` as transition glue rather than ownership target.
- State what future narrower import boundary would have to prove for flow, without claiming that proof already exists.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md

Out of scope:
- changing the imported-core decision for `handbook-engine` or `handbook-pipeline` beyond tiny consistency corrections
- any code implementation
- any crate publication or crates.io planning
- any Substrate integration implementation
- any broad CLI redesign or retained-compiler retirement work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Ownership And Integration Planning Packet 3: Decide Handbook-Side Deferred Boundaries And Non-Targets`.
- Tell the subagent to use $incremental-implementation.
- Require evidence gathering with:
  - `rg -n "pub mod|pub use|resolve|PacketResult|BudgetOutcome" crates/flow/src/lib.rs crates/flow/src`
  - `cargo tree -p handbook-flow`
  - `cargo test -p handbook-flow --test resolver_core`
  - `rg -n "CommandFactory|ExitCode|pipeline_help|doctor|rendering|prompt" crates/cli/src/main.rs crates/cli/src`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `rg -n "rendering|refusal|doctor|setup|template_library|pub use" crates/compiler/src/lib.rs crates/compiler/src`
  - `cargo tree -p handbook-compiler`
  - `cargo check --workspace`
- Require the subagent to produce explicit calls that answer:
  - whether `handbook-flow` remains handbook-owned longer-term and what a future importable slice would need to prove
  - why `handbook-cli` is handbook-owned product shell and not an import target
  - why retained `handbook-compiler` is transition glue and not a future ownership target
  - which support surfaces stay deferred to later seams rather than being resolved here
- Require the subagent to keep `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` distinct rather than bundling them into a generic “later” bucket.
- Require the subagent to stop after Packet 3 acceptance is met and report touched files, evidence used, per-crate decisions, deferred seam list, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Ownership And Integration Planning Packet 3: Decide Handbook-Side Deferred Boundaries And Non-Targets`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, applied to the packet’s ownership-boundary documentation.
- Require the reviewer to review the packet against the new family spec/plan/tasks and the live flow/CLI/compiler evidence.
- Require special attention to:
  - whether `handbook-flow` is explicit but not overclaimed as a move target
  - whether `handbook-cli` is explicit product shell rather than implied reusable core
  - whether retained `handbook-compiler` is explicit transition glue rather than future owner target
  - whether the packet stayed out of CLI redesign, compiler retirement, and execution planning
- Require severity labels and explicit callouts if any of the three surfaces are left implicit, if shell/glue language is vague, or if the docs quietly widen scope.

Fix loop:
- If the review is clean, stop and report Packet 3 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3-bounded changes needed to close them.
- Re-run only the evidence/verification commands affected by the fix.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 3 implementation if it lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 3 flow/CLI/compiler boundary decisions clearly and standalone.

Stop conditions:
- Stop once Packet 3 is review-clean, committed, and `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` each have explicit, bounded posture.
- Stop and report blocked if Packet 3 cannot make an honest deferred/non-target call without first repairing a concrete earlier-seam regression.
```

## Packet 4 Prompt

```text
/goal Orchestrate Phase 6 Ownership And Integration Planning Packet 4: Define Downstream Execution Seams Without Starting Them in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md.
- Assume Packets 1, 2, and 3 are already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the planning family docs at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md
- Use the prerequisite authorities and packet outcomes from Packets 1 through 3.
- Do not reopen earlier packets except where a tiny documentation correction is strictly required for Packet 4 to land correctly.
- This packet should finish the planning family by naming bounded downstream seams and restating the human review gate.

Hard rules:
- Do not implement, review, or fix Packet 4 work in the orchestration session yourself. Orchestrate it through the required fresh subagents.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- This packet is expected to be docs/planning-only. Do not edit production code.
- If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4 only.
- Stay inside Packet 4 scope.

Packet 4 scope:
- Name the bounded follow-on execution seams per crate/owner area.
- Make explicit that those seams are not started by this planning family.
- Finish with an explicit human review gate before any execution work.
- Ensure the planning family does not read as approval to publish crates or integrate Substrate yet.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md
  - docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md

Out of scope:
- any code implementation
- any crate publication or crates.io work
- any Substrate integration implementation
- any packet prompt authoring beyond landing Packet 4 itself
- any widening of the downstream seams into actual implementation tasks in this pass

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Ownership And Integration Planning Packet 4: Define Downstream Execution Seams Without Starting Them`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to preserve the Packet 1–3 ownership calls and to add only the bounded downstream seam map plus the final review gate language.
- Require verification with:
  - `rg -n "follow-on|downstream execution seams|adapter|boundary cleanup|narrowing|review gate" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-*.md`
  - manual review that Packet 1 through 3 decisions remain intact and that Packet 4 does not convert those seams into implementation approval
- Require the subagent to name, at minimum:
  - any `handbook-engine` adapter/boundary freeze seam if still needed
  - the `handbook-pipeline` boundary cleanup seam
  - the `handbook-flow` ownership clarification seam
  - the retained `handbook-compiler` narrowing seam
  - the CLI shell/support clarification seam
- Require the subagent to make explicit that none of these seams start here, and that publication / crates.io / Substrate consumption are still later decisions.
- Require the subagent to stop after Packet 4 acceptance is met and report touched files, seam map, review-gate wording, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Ownership And Integration Planning Packet 4: Define Downstream Execution Seams Without Starting Them`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, applied to the packet’s downstream-seam and review-gate documentation.
- Require the reviewer to review the packet against the new family spec/plan/tasks and the already-landed Packet 1–3 decisions.
- Require special attention to:
  - whether downstream seams are bounded and distinct rather than vague “follow up later” notes
  - whether Packet 4 clearly stops short of implementation approval
  - whether the final review gate is explicit enough that a later agent cannot miss it
  - whether the docs avoid overstating readiness for crates.io publication or Substrate integration
- Require severity labels and explicit callouts if downstream seams are fuzzy, if the review gate is weak, or if the docs imply execution approval too early.

Fix loop:
- If the review is clean, stop and report Packet 4 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4-bounded changes needed to close them.
- Re-run only the verification commands affected by the fix.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 4 implementation if it lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 4 downstream-seam / review-gate change clearly and standalone.

Stop conditions:
- Stop once Packet 4 is review-clean, committed, and the planning family ends with bounded downstream seams plus an explicit human review gate.
- Stop and report blocked if Packet 4 cannot define later seams honestly without reopening unresolved contradictions from earlier packets.
```
