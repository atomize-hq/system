# Handbook Engine Extraction Phase 2 Slice 1 Packet Prompts

Task source: [handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md](./handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md)
Spec source: [handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md](./handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md)
Plan source: [handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md](./handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, and preserves commit boundaries between implementation, review, and fix work.

## Packet 2.1.1 Prompt

```text
/goal Orchestrate Phase 2 Slice 1 Packet 2.1.1: Typed Target And Consumer Contract in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md
- Do not start Packet 2.1.2.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Commit changes after the implementation lands, and commit again after each fix round that changes files.
- Stay inside Packet 2.1.1 scope.

Packet 2.1.1 scope:
- Freeze the supported target contract in the slice authority docs.
- Freeze the Slice 2.1 no-runtime-adoption boundary across the authority set.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
  - docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md
  - docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md

Out of scope:
- Packet 2.1.2 registry-owner and evidence-ledger work except where existing wording must be referenced for consistency
- any production Rust behavior change
- any runtime compile/capture/handoff/provenance adoption work
- any consumer-schema design or new `core/consumers/**` tree
- any Slice 2.2 or Slice 2.3 work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 1 Packet 2.1.1: Typed Target And Consumer Contract`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to keep this packet docs-only unless a tiny behavior-neutral docs-scaffolding change is strictly necessary, and forbid widening into runtime adoption work.
- Require the spec/plan/tasks set to explicitly freeze:
  - `SupportedPipelineTarget`, `SupportedStageTarget`, and `SupportedConsumerTarget`
  - the allowed pairings for `pipeline compile`, `pipeline capture`, stage-10 provenance, and `pipeline handoff emit`
  - pipeline/stage truth as declarative catalog truth
  - consumers as code-owned validated defaults
  - the no-runtime-adoption boundary for Slice 2.1 and the deferment of runtime adoption to Slice 2.2 and template/library resolver work to Slice 2.3
- Require targeted verification with:
  - `rg -n "SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget|Allowed pairings|pipeline compile|pipeline capture|pipeline handoff emit" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`
  - `rg -n "no runtime adoption|Slice 2\.2|Slice 2\.3|Out of scope" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`
- Require the subagent to stop after Packet 2.1.1 acceptance is met and report touched files, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 1 Packet 2.1.1: Typed Target And Consumer Contract`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the slice spec, plan, tasks, and verification evidence.
- Require severity labels and explicit callouts if:
  - the supported target contract is incomplete or inconsistent
  - Packet 2.1.2 registry/evidence-ledger work leaked in
  - Slice 2.2 or Slice 2.3 work leaked in
  - the docs fail to preserve declarative pipeline/stage truth versus code-owned consumer defaults

Fix loop:
- If the review is clean, stop and report Packet 2.1.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal changes needed to close them.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.1.1 lands cleanly.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.1.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.1.1 is review-clean and committed.
- Stop and report blocked if Packet 2.1.1 cannot be completed without widening into Packet 2.1.2, Slice 2.2+, or changing the approved slice spec/plan/tasks.
```

## Packet 2.1.2 Prompt

```text
/goal Orchestrate Phase 2 Slice 1 Packet 2.1.2: Target Registry Lookup And Validation Owner in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md.
- Assume Packet 2.1.1 is already landed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md
- Do not reopen Packet 2.1.1 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 2.1.2 to land correctly.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 2.1.2 scope.

Packet 2.1.2 scope:
- Build the hardcoded-target evidence ledger for the current supported wedge.
- Freeze the `SupportedTargetRegistry` ownership boundary and validation rules.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
  - docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md
  - docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md

Out of scope:
- redefining the Packet 2.1.1 contract vocabulary unless live repo truth disproves it
- any production Rust behavior change
- any runtime compile/capture/handoff/provenance adoption work
- any route-state filename refactor
- any template/library resolver design or consumer-schema addition
- any Slice 2.2 or Slice 2.3 implementation work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 1 Packet 2.1.2: Target Registry Lookup And Validation Owner`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that the Packet 2.1.1 contract language already exists before editing.
- Require the subagent to keep this packet docs-only unless a tiny behavior-neutral registry-scaffolding clarification is strictly necessary, and forbid widening into runtime adoption.
- Require the subagent to freeze:
  - one hardcoded-target evidence ledger covering compile, capture, stage-10 provenance, handoff, CLI help surface, and adjacent runtime-state evidence
  - one `SupportedTargetRegistry` owner
  - pipeline/stage support as declarative catalog truth
  - consumers as code-owned validated defaults
  - `route_state.rs` and CLI help text as explicit non-owner surfaces
  - the boundary that Slice 2.2 adopts the registry in runtime flows and Slice 2.3 stays template/library-only
- Require targeted verification with:
  - `rg -n "pipeline\.foundation_inputs|stage\.10_feature_spec|feature-slice-decomposer" crates/compiler/src crates/cli/src`
  - `rg -n "SupportedTargetRegistry|declarative pipeline/stage|code-owned validated defaults|Slice 2\.2|Slice 2\.3" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`
  - `rg -n "route_state\.rs|CLI help|hardcoded-target evidence" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`
- Require the subagent to stop after Packet 2.1.2 acceptance is met and report touched files, verification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 1 Packet 2.1.2: Target Registry Lookup And Validation Owner`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the slice spec, plan, tasks, and verification evidence.
- Require severity labels and explicit callouts if:
  - the evidence ledger misses a current hardcoded site
  - the registry owner boundary is ambiguous or duplicated
  - `route_state.rs` or CLI help text is treated as the long-term owner
  - Slice 2.2 runtime adoption or Slice 2.3 template/library work leaked in
  - Packet 2.1.1 vocabulary was silently changed instead of minimally corrected

Fix loop:
- If the review is clean, stop and report Packet 2.1.2 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2.1.2-bounded.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2.1.2 implementation lands cleanly.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.1.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.1.2 is review-clean and committed.
- Stop and report blocked if Packet 2.1.2 requires widening into Slice 2.2+, redefining the approved Packet 2.1.1 contract beyond a tiny correctness fix, or changing the approved slice spec/plan/tasks.
```
