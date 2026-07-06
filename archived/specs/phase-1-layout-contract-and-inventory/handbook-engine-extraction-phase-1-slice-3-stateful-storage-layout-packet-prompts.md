# Handbook Engine Extraction Phase 1 Slice 3 Packet Prompts

Task source: [handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-tasks.md](./handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-tasks.md)
Spec source: [handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md](./handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md)
Plan source: [handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-plan.md](./handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 1.3 scope.

## Packet 1.3.1 Prompt

```text
/goal Orchestrate Phase 1 Slice 3 Packet 1.3.1: Route-State And Stage-Provenance Layout Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.3.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-tasks.md.
- Verify live repo truth before changing anything, including that Slice 1.2 already landed the initial compiler-local canonical layout owner and that Slice 1.3 is the next code-adoption seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-plan.md
- Do not start Packet 1.3.2.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.3.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Stay inside Packet 1.3.1 scope.

Packet 1.3.1 scope:
- Introduce compiler-local runtime-state and capture-provenance layout owners.
- Adopt `route_state.rs` and `stage_10_feature_spec_provenance.rs` onto those layout owners.
- Expected files:
  - crates/compiler/src/layout.rs
  - crates/compiler/src/lib.rs
  - crates/compiler/src/route_state.rs
  - crates/compiler/src/stage_10_feature_spec_provenance.rs
  - crates/compiler/tests/pipeline_state_store.rs
  - crates/compiler/tests/pipeline_route_resolution.rs

Out of scope:
- Packet 1.3.2 capture or handoff adoption beyond minimal compile-through wiring
- any canonical artifact or setup ownership changes
- any target-id, stage-id, consumer-id, or Phase 2 registry/contract changes
- any CLI/operator wording or next-safe-action wording changes
- any authoring-root or authoring-lock adoption
- any Phase 1 Slice 1.4 / Phase 2 / Phase 3 shell-cleanup work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 3 Packet 1.3.1: Route-State And Stage-Provenance Layout Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Slice 1.2 already landed and that the initial compiler-local layout owner is present before editing.
- Require the subagent to keep this packet bounded to runtime-state and stage-provenance ownership and to explicitly preserve:
  - separate layout types rather than one global layout object
  - route-state reset traversal and route-basis persistence semantics
  - stage-10 capture provenance schema and provenance-match semantics
  - supported pipeline and stage identities remaining frozen for Phase 2
  - canonical artifact semantics already frozen by Slice 1.2
- Require the subagent to remove direct runtime-state and stage-provenance storage ownership such as `repo_root.join(".handbook").join("state")...` and `STAGE_10_FEATURE_SPEC_CAPTURE_PROVENANCE_PATH` literals in favor of layout accessors, unless live code truth proves an equivalent narrow helper is already present.
- Require the subagent to introduce only the narrowest compiler-local runtime-state and capture-provenance layout owners needed for Packet 1.3.1, preferably by extending `crates/compiler/src/layout.rs`, and to make `route_state.rs` plus `stage_10_feature_spec_provenance.rs` consume them without widening into capture-cache or handoff-bundle adoption.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "RuntimeState|CaptureProvenance|route_state_path|STAGE_10_FEATURE_SPEC_CAPTURE_PROVENANCE_PATH|\\.handbook/state" crates/compiler/src/layout.rs crates/compiler/src/route_state.rs crates/compiler/src/stage_10_feature_spec_provenance.rs`
  - `cargo test -p handbook-compiler --test pipeline_state_store`
  - `cargo test -p handbook-compiler --test pipeline_route_resolution`
  - `cargo check -p handbook-compiler`
- Require the subagent to stop after Packet 1.3.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 3 Packet 1.3.1: Route-State And Stage-Provenance Layout Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 1.3 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether `layout.rs` stayed narrow rather than becoming a monolithic all-storage object
  - whether `route_state.rs` now consumes runtime-state layout ownership cleanly
  - whether stage-10 capture provenance storage moved behind layout accessors without semantic drift
  - whether Packet 1.3.2 capture/handoff adoption leaked in
  - whether any target-contract or shell-wording cleanup accidentally started
- Require severity labels and explicit callouts if the layout owners are too broad, if route-state or provenance semantics drifted, or if capture/handoff adoption leaked beyond compile-through wiring.

Fix loop:
- If the review is clean, stop and report Packet 1.3.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.3.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.3.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.3.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1.3.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.3.1 is review-clean and committed.
- Stop and report blocked if Packet 1.3.1 cannot be completed without widening into Packet 1.3.2, Slice 1.4+, Phase 2+, or changing the approved Slice 1.3 spec/plan/tasks.
```

## Packet 1.3.2 Prompt

```text
/goal Orchestrate Phase 1 Slice 3 Packet 1.3.2: Capture And Handoff Layout Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.3.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-tasks.md.
- Assume Packet 1.3.1 is already landed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-plan.md
- Do not reopen Packet 1.3.1 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 1.3.2 to land correctly.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.3.2 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 1.3.2 scope.

Packet 1.3.2 scope:
- Adopt `pipeline_capture.rs` onto the capture-provenance layout owner.
- Adopt `pipeline_handoff.rs` onto the handoff bundle layout owner and preserve Slice 1.3 boundaries.
- Expected files:
  - crates/compiler/src/layout.rs
  - crates/compiler/src/pipeline_capture.rs
  - crates/compiler/src/pipeline_handoff.rs
  - crates/compiler/tests/pipeline_capture.rs
  - crates/compiler/tests/pipeline_handoff.rs

Out of scope:
- any Packet 1.3.1 reopening beyond a tiny corrective compile-through change proven necessary by live code truth
- any canonical artifact, setup, route-state, or authoring ownership changes beyond what is strictly required for Packet 1.3.2 to compile cleanly
- any supported pipeline, stage, or consumer identity redesign
- any CLI/operator wording or next-safe-action wording changes
- any Phase 1 Slice 1.4 / Phase 2 / Phase 3 shell-cleanup work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 3 Packet 1.3.2: Capture And Handoff Layout Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 1.3.1 already landed and that the runtime-state / capture-provenance layout owners are present before editing.
- Require the subagent to keep this packet bounded to capture-cache and handoff-bundle storage adoption and to explicitly preserve:
  - capture input parsing, cache-integrity checks, rollback, and refusal behavior
  - handoff manifest, read-allowlist, trust-matrix, and validation semantics
  - supported pipeline, stage, and consumer identities remaining frozen for Phase 2
  - shell wording and next-safe-action wording remaining local for later slices
  - canonical artifact and route-state semantics already frozen by earlier packets
- Require the subagent to remove direct storage ownership such as `.handbook/state/pipeline/capture/{capture_id}.yaml` and `artifacts/handoff/feature_slice/{feature_id}` derivation in favor of layout accessors, unless live code truth proves equivalent narrow helpers are already present.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "CaptureProvenance|HandoffBundle|capture_cache_repo_relative_path|bundle_root|artifacts/handoff/feature_slice" crates/compiler/src/layout.rs crates/compiler/src/pipeline_capture.rs crates/compiler/src/pipeline_handoff.rs`
  - `cargo test -p handbook-compiler --test pipeline_capture`
  - `cargo test -p handbook-compiler --test pipeline_handoff`
  - `cargo test -p handbook-compiler --test pipeline_state_store`
  - `cargo test -p handbook-compiler --test pipeline_route_resolution`
  - `cargo check -p handbook-compiler`
- Require the subagent to stop after Packet 1.3.2 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 3 Packet 1.3.2: Capture And Handoff Layout Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 1.3 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether `pipeline_capture.rs` now consumes capture-provenance layout ownership cleanly
  - whether `pipeline_handoff.rs` now consumes handoff bundle layout ownership cleanly
  - whether capture-cache, provenance, or handoff semantics regressed while storage ownership was being adopted
  - whether any target-contract, route-state redesign, or shell-wording cleanup leaked in
  - whether Packet 1.3.1 semantics regressed while Packet 1.3.2 was landing
- Require severity labels and explicit callouts if direct capture-cache or handoff-bundle ownership remains duplicated, if Phase 2 target work leaked in, or if Packet 1.3.1 semantics regressed.

Fix loop:
- If review is clean, stop and report Packet 1.3.2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-1.3.2-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 1.3.2 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.3.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1.3.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.3.2 is review-clean and committed.
- Stop and report blocked if Packet 1.3.2 requires widening into Slice 1.4+, Phase 2+, Phase 3 shell cleanup, regressing Packet 1.3.1 semantics, or changing the approved Slice 1.3 spec/plan/tasks.
```
