# Handbook Engine Extraction Phase 2 Slice 2 Packet Prompts

Task source: [handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md](./handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md)
Spec source: [handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md](./handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md)
Plan source: [handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md](./handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and requires GitNexus impact analysis before production-symbol edits plus GitNexus detect-changes before each commit.

## Packet 2.2.1 Prompt

```text
/goal Orchestrate Phase 2 Slice 2 Packet 2.2.1: Compile Target Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.2.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md.
- Verify live repo truth before changing anything, including that the approved Slice 2.1 authority set already exists and that Slice 2.2 is now the runtime-adoption seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md
- Use the Slice 2.1 authority docs as contract inputs:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
- Do not start Packet 2.2.2 or Packet 2.2.3.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.2.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Stay inside Packet 2.2.1 scope.

Packet 2.2.1 scope:
- Implement the runtime supported-target owner for the approved Slice 2.1 wedge.
- Migrate compile target validation onto that runtime owner while preserving compile posture.
- Expected files:
  - crates/compiler/src/pipeline.rs
  - crates/compiler/src/pipeline_compile.rs
  - optionally crates/compiler/tests/pipeline_compile.rs
  - optionally crates/cli/tests/cli_surface.rs

Out of scope:
- capture target adoption
- stage-10 provenance target adoption
- handoff target adoption
- expanding the supported wedge beyond the approved Slice 2.1 pipeline/stage/consumer contract
- changing `route_state.rs` or CLI help into ownership surfaces
- Slice 2.3 template/library resolver work
- Phase 4 crate extraction or broad compiler cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 2 Packet 2.2.1: Compile Target Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that the Slice 2.1 authority contract already exists before editing.
- Require the subagent to keep this packet bounded to runtime target-owner introduction plus compile adoption and to explicitly preserve:
  - the approved Slice 2.1 contract vocabulary: `SupportedPipelineTarget`, `SupportedStageTarget`, `SupportedConsumerTarget`, and `SupportedTargetRegistry`
  - declarative pipeline/stage truth as the authority for pipeline and stage identities
  - consumers as code-owned validated defaults
  - compile-specific refusal classifications, summaries, and recovery posture remaining local in `pipeline_compile.rs`
  - the boundary that `route_state.rs` and CLI help stay non-owner surfaces in this slice
- Require the smallest viable runtime target owner that can support Packet 2.2.1 without widening into Packet 2.2.2 or 2.2.3.
- Require `pipeline_compile.rs` to stop acting as the supported-target owner for the approved compile target.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_compile`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `rg -n "SupportedTargetRegistry|SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget" crates/compiler/src/pipeline.rs crates/compiler/src/pipeline_compile.rs`
- Require the subagent to stop after Packet 2.2.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 2 Packet 2.2.1: Compile Target Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.2 spec, plan, tasks, the Slice 2.1 authority set, and the verification evidence.
- Require special attention to:
  - whether one runtime supported-target owner now exists instead of duplicated compile-local ownership
  - whether compile refusal posture stayed local and stable
  - whether Packet 2.2.2 or 2.2.3 work leaked in
  - whether `route_state.rs` or CLI help accidentally became an ownership surface
- Require severity labels and explicit callouts if the runtime owner is duplicated, if compile semantics drifted, or if scope widened beyond Packet 2.2.1.

Fix loop:
- If the review is clean, stop and report Packet 2.2.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.2.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.2.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.2.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.2.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.2.1 is review-clean and committed.
- Stop and report blocked if Packet 2.2.1 cannot be completed without widening into Packet 2.2.2+, changing the approved Slice 2.1 contract, or changing the approved Slice 2.2 spec/plan/tasks.
```

## Packet 2.2.2 Prompt

```text
/goal Orchestrate Phase 2 Slice 2 Packet 2.2.2: Capture And Provenance Target Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.2.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md.
- Assume Packet 2.2.1 is already landed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md
- Use the Slice 2.1 authority docs as contract inputs:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
- Do not reopen Packet 2.2.1 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 2.2.2 to land correctly.
- Do not start Packet 2.2.3.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.2.2 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 2.2.2 scope.

Packet 2.2.2 scope:
- Migrate capture supported-target validation and supported-stage rendering onto the runtime owner.
- Migrate stage-10 provenance target validation onto the same runtime owner.
- Expected files:
  - crates/compiler/src/pipeline.rs
  - crates/compiler/src/pipeline_capture.rs
  - crates/compiler/src/stage_10_feature_spec_provenance.rs
  - crates/compiler/tests/pipeline_capture.rs
  - optionally crates/cli/tests/cli_surface.rs

Out of scope:
- handoff target adoption
- expanding the approved capture stage set or supported wedge
- changing capture input-shape, apply/preview, or state posture except where required to preserve current behavior after adoption
- changing provenance schema, persisted file layout, or hash/path guarantees except where required to preserve current behavior after adoption
- Slice 2.3 template/library resolver work
- broader runtime-state or CLI/help ownership changes

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 2 Packet 2.2.2: Capture And Provenance Target Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 2.2.1 already landed and that the runtime target owner is present before editing.
- Require the subagent to keep this packet bounded to capture and stage-10 provenance adoption and to explicitly preserve:
  - the approved Slice 2.1 capture stage wedge for `pipeline.foundation_inputs`
  - capture-specific state, input-shape, apply/preview, and refusal posture remaining local in `pipeline_capture.rs`
  - provenance schema, feature-spec path, route-basis, template, and payload-hash guarantees remaining local in `stage_10_feature_spec_provenance.rs`
  - `route_state.rs` and CLI help remaining non-owner observer surfaces
- Require `pipeline_capture.rs` to stop owning the authoritative supported pipeline/stage set locally.
- Require `stage_10_feature_spec_provenance.rs` to stop owning a private supported-target validator if the runtime owner can provide the same contract.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_capture`
  - `cargo test -p handbook-cli --test cli_surface`
  - `rg -n "SupportedTargetRegistry|SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget" crates/compiler/src/pipeline.rs crates/compiler/src/pipeline_capture.rs crates/compiler/src/stage_10_feature_spec_provenance.rs`
- Require the subagent to stop after Packet 2.2.2 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 2 Packet 2.2.2: Capture And Provenance Target Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.2 spec, plan, tasks, the Slice 2.1 authority set, and the verification evidence.
- Require special attention to:
  - whether capture now consumes the runtime owner instead of remaining a competing supported-target owner
  - whether stage-10 provenance uses the same target contract as compile/capture
  - whether capture or provenance behavior drifted while ownership moved
  - whether Packet 2.2.3 handoff work leaked in
- Require severity labels and explicit callouts if supported-stage ownership is still duplicated, if stage-10 provenance guarantees drifted, or if scope widened beyond Packet 2.2.2.

Fix loop:
- If review is clean, stop and report Packet 2.2.2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2.2.2-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2.2.2 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.2.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.2.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.2.2 is review-clean and committed.
- Stop and report blocked if Packet 2.2.2 requires widening into Packet 2.2.3, expanding the approved supported wedge, or changing the approved Slice 2.2 spec/plan/tasks.
```

## Packet 2.2.3 Prompt

```text
/goal Orchestrate Phase 2 Slice 2 Packet 2.2.3: Handoff Target Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.2.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md.
- Assume Packets 2.2.1 and 2.2.2 are already landed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md
- Use the Slice 2.1 authority docs as contract inputs:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
- Do not reopen Packets 2.2.1 or 2.2.2 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 2.2.3 to land correctly.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.2.3 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 2.2.3 scope.

Packet 2.2.3 scope:
- Migrate handoff consumer and manifest target validation onto the runtime owner.
- Preserve handoff-specific provenance, trust, and write-failure posture after adoption.
- Expected files:
  - crates/compiler/src/pipeline.rs
  - crates/compiler/src/pipeline_handoff.rs
  - crates/compiler/tests/pipeline_handoff.rs
  - crates/cli/tests/cli_surface.rs
  - crates/cli/tests/help_drift_guard.rs

Out of scope:
- changing handoff bundle layout beyond what current behavior already requires
- changing handoff trust-class posture or provenance-matching rules beyond preserving current behavior after owner adoption
- expanding the approved consumer or supported wedge
- Slice 2.3 template/library resolver work
- Phase 4 crate extraction or broader compiler/CLI restructuring

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 2 Packet 2.2.3: Handoff Target Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packets 2.2.1 and 2.2.2 already landed and that the runtime target owner is present before editing.
- Require the subagent to keep this packet bounded to handoff adoption and to explicitly preserve:
  - the approved handoff wedge `pipeline.foundation_inputs` -> `stage.10_feature_spec` -> `feature-slice-decomposer`
  - handoff-specific provenance matching, trust-class behavior, bundle layout, and write-failure posture remaining local in `pipeline_handoff.rs`
  - CLI help and `route_state.rs` remaining non-owner observer surfaces
- Require `pipeline_handoff.rs` to stop owning the authoritative supported pipeline/consumer/stage target checks if the runtime owner can provide the same contract.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_handoff`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `rg -n "SupportedTargetRegistry|SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget" crates/compiler/src/pipeline.rs crates/compiler/src/pipeline_handoff.rs`
- Require the subagent to stop after Packet 2.2.3 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 2 Packet 2.2.3: Handoff Target Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.2 spec, plan, tasks, the Slice 2.1 authority set, and the verification evidence.
- Require special attention to:
  - whether handoff now consumes the runtime owner instead of keeping competing supported-target checks
  - whether provenance matching, trust-class behavior, bundle layout, and write-failure posture stayed local and stable
  - whether any unsupported wedge expansion leaked in
  - whether CLI/help fallout drifted unexpectedly
- Require severity labels and explicit callouts if supported-target ownership remains duplicated, if handoff safety posture regressed, or if scope widened beyond Packet 2.2.3.

Fix loop:
- If review is clean, stop and report Packet 2.2.3 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2.2.3-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2.2.3 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.2.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.2.3 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.2.3 is review-clean and committed.
- Stop and report blocked if Packet 2.2.3 requires widening beyond the approved supported wedge, reopening earlier packets beyond tiny corrective edits, or changing the approved Slice 2.2 spec/plan/tasks.
```
