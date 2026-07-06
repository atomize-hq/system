# Handbook Engine Extraction Phase 1 Slice 5 Packet Prompts

Task source: [handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-tasks.md](./handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-tasks.md)
Spec source: [handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md](./handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md)
Plan source: [handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md](./handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 1.5 scope.

## Packet 1.5.1 Prompt

```text
/goal Orchestrate Phase 1 Slice 5 Packet 1.5.1: Parameterized Canonical Layout Contract in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.5.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-tasks.md.
- Verify live repo truth before changing anything, including that Slice 1.5 is a parameterization closeout slice, that the current product-default paths remain `.handbook/**`, `.handbook/state/**`, and `artifacts/handoff/feature_slice/**`, and that making those paths changeable through typed parameters/default owners is a hard requirement even though the effective defaults must not change in this packet.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md
- Stay inside Packet 1.5.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.5.1 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 1.5.1 scope.

Packet 1.5.1 scope:
- Freeze the typed reusable-layout contract and residual-literal policy.
- Parameterize engine canonical path ownership.
- Make canonical path values changeable via typed parameters/default owners in principle.
- Keep the current effective canonical product-default paths unchanged in this packet.
- Expected files:
  - crates/engine/src/canonical_paths.rs
  - crates/engine/src/canonical_artifacts.rs
  - optionally crates/engine/src/lib.rs
  - optionally narrow shared support module(s) only if live code truth proves they are strictly required
  - optionally narrow compile-through alignment in crates/compiler/src/** only if required to avoid drift
  - optionally crates/engine/tests/canonical_artifacts_ingest.rs
  - optionally crates/engine/tests/freshness_computation.rs

Out of scope:
- pipeline runtime-state/capture/provenance/handoff parameterization work reserved for Packet 1.5.2
- reusable-caller adoption and residual-literal closeout reserved for Packet 1.5.3
- any actual product path change away from the current defaults
- any free-form user-configurable layout surface, environment-variable override surface, or multi-consumer platform work
- any orchestration-target parameterization, compiler retirement/narrowing decision, or CLI wording/help cleanup
- any broad crate-boundary redesign outside the narrow canonical-parameterization seam

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 5 Packet 1.5.1: Parameterized Canonical Layout Contract`.
- Tell the subagent to use $incremental-implementation.
- Require live verification across:
  - `crates/engine/src/canonical_paths.rs`
  - `crates/engine/src/canonical_artifacts.rs`
  - `crates/engine/src/lib.rs`
  - `crates/compiler/src/layout.rs`
  - `crates/flow/src/resolver.rs`
  - `crates/engine/tests/canonical_artifacts_ingest.rs`
  - `crates/engine/tests/freshness_computation.rs`
- Require the implementation to:
  - land one typed canonical layout contract/default owner that makes canonical path values changeable in principle through parameters/default owners
  - ensure the current `.handbook/**` canonical layout is produced by the newly landed parameterization rather than by scattered reusable-internal literals
  - preserve canonical artifact identities, manifest behavior, and freshness semantics for the current default layout
  - keep the implementation narrow and avoid inventing a broad shared layout platform unless live code truth proves a tiny shared support seam is strictly required
  - preserve the hard rule that this packet does not actually change the effective canonical paths used by the product today
- Require the subagent to explicitly classify any residual fixed canonical literals left behind as product-shell/doc/test-only or justify why they still must exist at this packet boundary.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n --glob '!**/tests/**' --glob '!**/*test*' "\\.handbook|artifacts/handoff|state/pipeline|feature_slice" crates/engine/src crates/pipeline/src crates/flow/src crates/compiler/src`
  - `rg -n "SYSTEM_ROOT_RELATIVE|RUNTIME_STATE_ROOT_RELATIVE|HANDOFF_FEATURE_SLICE_DIR_RELATIVE|HANDBOOK_ROOT_PATH" crates/engine/src crates/pipeline/src crates/flow/src crates/compiler/src`
  - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - `cargo test -p handbook-engine --test freshness_computation`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 1.5.1 acceptance is met and report touched files, impact-analysis results, the parameterization shape chosen, verification run, residual literals still present, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 5 Packet 1.5.1: Parameterized Canonical Layout Contract`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 1.5 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether path changeability via typed parameters/default owners is actually real, not merely centralized literals under a new name
  - whether the current canonical default layout is now produced by the new parameterized owner rather than remaining an implicit reusable-engine assumption
  - whether canonical artifact identities, manifest generation, and freshness semantics remained stable
  - whether the packet avoided widening into Packet 1.5.2 storage work, Packet 1.5.3 caller work, or broader platform/config work
  - whether any residual fixed canonical literals are truthfully bounded and justified
- Require severity labels and explicit callouts if the packet still hardcodes canonical contract ownership inside reusable engine internals, fakes parameterization with no real changeability, or leaks broader architectural work into Packet 1.5.1.

Fix loop:
- If the review is clean, stop and report Packet 1.5.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.5.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.5.1 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.5.1 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 1.5.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.5.1 is review-clean and committed.
- Stop and report blocked if Packet 1.5.1 cannot be completed without changing the current effective product-default canonical paths, widening into pipeline-storage/caller-closeout work, or introducing a broader config/platform surface than the approved slice allows.
```

## Packet 1.5.2 Prompt

```text
/goal Orchestrate Phase 1 Slice 5 Packet 1.5.2: Parameterized Pipeline Storage Layout Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.5.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-tasks.md.
- Assume Packet 1.5.1 is already landed; verify live repo truth before changing anything, including that the canonical parameterization contract/default owner is already in place and that this packet must make runtime-state, capture, provenance, and handoff roots changeable through typed parameters/default owners in principle while keeping the current effective defaults unchanged.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md
- Stay inside Packet 1.5.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.5.2 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 1.5.2 scope.

Packet 1.5.2 scope:
- Parameterize runtime-state, capture, provenance, and handoff layout ownership in `handbook-pipeline`.
- Keep the current effective `.handbook/state/**` and `artifacts/handoff/feature_slice/**` defaults unchanged while making them changeable through typed parameters/default owners in principle.
- Preserve current route-state, capture, provenance, and handoff behavior.
- Keep this packet scoped away from orchestration-target work.
- Expected files:
  - crates/pipeline/src/layout.rs
  - crates/pipeline/src/route_state.rs
  - crates/pipeline/src/pipeline_capture.rs
  - crates/pipeline/src/stage_10_feature_spec_provenance.rs
  - crates/pipeline/src/pipeline_handoff.rs
  - optionally narrow shared support or compile-through alignment files only if live code truth proves they are strictly required
  - optionally crates/pipeline/tests/pipeline_state_store.rs
  - optionally crates/pipeline/tests/pipeline_capture.rs
  - optionally crates/pipeline/tests/pipeline_handoff.rs

Out of scope:
- canonical-path parameterization work reserved for Packet 1.5.1 except for narrow compile-through reuse
- reusable-caller adoption and residual-literal closeout reserved for Packet 1.5.3
- any actual change to the current effective runtime-state or handoff product paths
- any change to supported pipeline ids, stage ids, consumer ids, target-registry posture, or catalog truth
- any free-form user-configurable layout/config surface, compiler-retirement decision, or CLI wording/help cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 5 Packet 1.5.2: Parameterized Pipeline Storage Layout Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification across:
  - `crates/pipeline/src/layout.rs`
  - `crates/pipeline/src/route_state.rs`
  - `crates/pipeline/src/pipeline_capture.rs`
  - `crates/pipeline/src/stage_10_feature_spec_provenance.rs`
  - `crates/pipeline/src/pipeline_handoff.rs`
  - `crates/compiler/src/layout.rs`
  - `crates/pipeline/tests/pipeline_state_store.rs`
  - `crates/pipeline/tests/pipeline_capture.rs`
  - `crates/pipeline/tests/pipeline_handoff.rs`
- Require the implementation to:
  - make runtime-state, capture, provenance, and handoff root derivation changeable in principle via typed parameters/default owners
  - ensure the current `.handbook/state/**` and `artifacts/handoff/feature_slice/**` locations are produced by the newly landed parameterized owners/defaults rather than by fixed reusable-pipeline literals
  - preserve route-state persistence/reset behavior, capture-cache and provenance behavior, and handoff bundle behavior for the current defaults
  - keep supported pipeline/stage/consumer identities untouched and avoid widening into Set 2 orchestration-target parameterization
  - keep the implementation narrow rather than inventing a broad layout/config platform
- Require the subagent to explicitly report any remaining residual fixed storage literals and classify them as acceptable product-shell/doc/test-only or explain why they still remain.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-pipeline --test pipeline_state_store`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
  - `rg -n "pipeline\.foundation_inputs|stage\.10_feature_spec|feature-slice-decomposer" crates/pipeline/src crates/compiler/src`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 1.5.2 acceptance is met and report touched files, impact-analysis results, the storage-parameterization shape chosen, verification run, residual literals still present, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 5 Packet 1.5.2: Parameterized Pipeline Storage Layout Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 1.5 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether runtime-state/capture/provenance/handoff paths are actually changeable through typed parameters/default owners rather than merely centralized literals
  - whether the current effective defaults are now produced by the new owners/defaults without changing behavior
  - whether route-state, capture, provenance, and handoff semantics remained stable
  - whether any target-registry, pipeline-id, stage-id, or consumer-id behavior drifted
  - whether the packet avoided widening into Packet 1.5.3 caller work or Set 2 orchestration-target work
  - whether any residual fixed storage literals are truthfully bounded and justified
- Require severity labels and explicit callouts if the packet still hardcodes reusable-pipeline storage ownership, quietly changes current path behavior, or leaks target-parameterization/platform work into Packet 1.5.2.

Fix loop:
- If the review is clean, stop and report Packet 1.5.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.5.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.5.2 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.5.2 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 1.5.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.5.2 is review-clean and committed.
- Stop and report blocked if Packet 1.5.2 cannot be completed without changing the current effective runtime-state/handoff defaults, altering supported target identity behavior, or widening into Packet 1.5.3 or Set 2 work.
```

## Packet 1.5.3 Prompt

```text
/goal Orchestrate Phase 1 Slice 5 Packet 1.5.3: Remaining Reusable Caller Adoption And Residual-Literal Closeout in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.5.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-tasks.md.
- Assume Packets 1.5.1 and 1.5.2 are already landed; verify live repo truth before changing anything, including that canonical and pipeline layout owners/defaults are already parameterized and that this packet must move remaining reusable callers onto those owners/defaults while truthfully bounding any residual fixed literals.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md
- Stay inside Packet 1.5.3 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.5.3 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 1.5.3 scope.

Packet 1.5.3 scope:
- Adopt remaining reusable callers onto the approved parameterized layout owners/defaults.
- Bound and justify any remaining fixed literals after reusable adoption lands.
- Preserve current resolver refusal/budget behavior for the current product defaults.
- Minimize compatibility-layer drift without widening into compiler narrowing/retirement work.
- Expected files:
  - crates/flow/src/resolver.rs
  - optionally narrow adapter/alignment files in crates/compiler/src/** only if required for compile-through truth
  - optionally very small source-touch follow-through in crates/engine/src/** or crates/pipeline/src/** only if live code truth proves the reusable owner contract needs a tiny final exposure adjustment
  - optionally crates/flow/tests/resolver_core.rs if a narrow test update is strictly required by truthful residual-literal assertions

Out of scope:
- reopening Packet 1.5.1 or 1.5.2 architecture beyond tiny compile-through follow-through
- any actual change to the current effective product-default paths
- any compiler narrowing/retirement decision reserved for Set 3
- any orchestration-target parameterization reserved for Set 2
- any CLI wording/help cleanup or broad doc rewrite beyond truthful residual-literal classification where strictly required

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 5 Packet 1.5.3: Remaining Reusable Caller Adoption And Residual-Literal Closeout`.
- Tell the subagent to use $incremental-implementation.
- Require live verification across:
  - `crates/flow/src/resolver.rs`
  - `crates/compiler/src/layout.rs`
  - any engine/pipeline owner files the caller should now consume
  - `crates/flow/tests/resolver_core.rs`
  - the output of `rg -n --glob '!**/tests/**' --glob '!**/*test*' "\\.handbook|artifacts/handoff|state/pipeline|feature_slice" crates/engine/src crates/pipeline/src crates/flow/src crates/compiler/src crates/cli/src`
- Require the implementation to:
  - remove remaining reusable-caller fallback ownership where approved parameterized owners/defaults already exist
  - make `crates/flow/src/resolver.rs` and any other touched reusable caller consume the newly landed parameterized owners/defaults rather than its own separate root literals
  - preserve current refusal, blocker, budget, and packet-selection behavior for the current product defaults
  - keep actual product-default paths unchanged in this packet
  - leave compiler posture decisions, target parameterization, and CLI-shell work untouched
- Require the subagent to produce an explicit residual-literal inventory at the end of the packet that distinguishes:
  - acceptable product-shell literals
  - acceptable doc/fixture/test literals
  - any remaining compatibility-layer literals that still need future work
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-flow --test resolver_core`
  - `cargo check --workspace`
  - `rg -n --glob '!**/tests/**' --glob '!**/*test*' "\\.handbook|artifacts/handoff|state/pipeline|feature_slice" crates/engine/src crates/pipeline/src crates/flow/src crates/compiler/src crates/cli/src`
- Require the subagent to stop after Packet 1.5.3 acceptance is met and report touched files, impact-analysis results, residual-literal inventory, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 5 Packet 1.5.3: Remaining Reusable Caller Adoption And Residual-Literal Closeout`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 1.5 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether remaining reusable callers truly stopped owning separate fallback root literals where approved owners/defaults exist
  - whether resolver refusal/blocker/budget behavior stayed stable for the current product defaults
  - whether residual fixed literals are now truthfully bounded rather than silently left in reusable ownership positions
  - whether compiler alignment stayed narrow and avoided leaking into Set 3 posture decisions
  - whether the packet avoided reopening Packet 1.5.1/1.5.2 architecture except where a tiny truthful exposure fix was strictly necessary
- Require severity labels and explicit callouts if reusable callers still own independent fallback roots, if residual literals are misclassified, or if the packet leaks into compiler-narrowing, target-parameterization, or CLI-shell work.

Fix loop:
- If the review is clean, stop and report Packet 1.5.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.5.3-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.5.3 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.5.3 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 1.5.3 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.5.3 is review-clean and committed.
- Stop and report blocked if Packet 1.5.3 cannot be completed without changing the current effective defaults, widening into compiler posture work, or reopening broader architecture outside the approved Slice 1.5 packet boundary.
```
