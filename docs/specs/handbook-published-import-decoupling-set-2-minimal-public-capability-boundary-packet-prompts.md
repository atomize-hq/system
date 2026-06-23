# Handbook Published-Import Decoupling — Set 2 Packet Prompts

Task source: [handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md](./handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md)
Spec source: [handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md](./handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md)
Plan source: [handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md](./handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md)
Set 1 authority source: [handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md](./handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md)
Map source: [MAP.md](./MAP.md)

These prompts are ready to paste into fresh orchestration sessions. Each one:
- starts in `/goal`
- requires a fresh **GPT-5.4 high** implementation subagent
- requires that implementation subagent to use `$incremental-implementation`
- requires a fresh **GPT-5.4 high** review subagent after the implementation lands
- requires that review subagent to use `$code-review-and-quality`
- requires a fresh **GPT-5.4 high** fix subagent for every review round that finds issues
- requires commit boundaries between implementation, review, and each accepted fix round
- keeps execution bounded to one Set 2 packet only

Set 2 is implementation work in `/Users/spensermcconnell/__Active_Code/system`, but the orchestration session must not implement, review, or fix the packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

Preserve unrelated local edits, especially incidental dirt in `AGENTS.md` and `CLAUDE.md`. In `system`, run `npx gitnexus detect-changes --repo system` before every commit and confirm the affected scope matches only the current packet. Do not widen beyond the Set 2 retained/dropped matrix without reopening authority first. Do not silently drift into Set 3 released-consumer proof, downstream Substrate source-touching proof, CLI/compiler/product-shell redesign, or unrelated cleanup.

---

## Packet 2.1 Prompt — Public Contract Owners

```text
/goal Orchestrate Set 2 Packet 2.1: Public Contract Owners in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
- Treat this as one narrow implementation seam: promote only the reviewed public declarative-roots and storage-layout contract owners, keep nested helper structs and repo-layout plumbing private, and avoid whole-module publication.
- Stay inside Packet 2.1 only.

Hard rules:
- Do not implement, review, or fix Packet 2.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.1 only.
- Stay inside Packet 2.1 scope.

Packet 2.1 scope:
- Make `PipelineDeclarativeRootsContract` public with validated public construction and stable read accessors.
- Make `PipelineStorageLayoutContract` public with validated public construction and stable read accessors.
- Keep `RuntimeStateLayoutContract`, `CaptureStorageLayoutContract`, `HandoffBundleLayoutContract`, and `RepoLayoutRoot` private.
- Do not make `handbook_pipeline::declarative_roots` or `handbook_pipeline::layout` public wholesale.
- Expected files:
  - crates/pipeline/src/declarative_roots.rs
  - crates/pipeline/src/layout.rs
  - crates/pipeline/src/lib.rs
  - optionally tightly related tests if required for honest acceptance

Out of scope — do NOT touch:
- Packet 2.2 declarative-root façade entrypoints
- Packet 2.3 route-state storage-layout façade
- Packet 2.4 capture/handoff façade
- Packet 2.5 external consumer proof harness
- Set 3 proof, release, or downstream Substrate work
- CLI/compiler/product-shell redesign
- unrelated cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 2 Packet 2.1: Public Contract Owners`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,220p' docs/specs/MAP.md`
  - `sed -n '254,340p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md`
  - `sed -n '1,220p' crates/pipeline/src/declarative_roots.rs`
  - `sed -n '1,260p' crates/pipeline/src/layout.rs`
  - `sed -n '1,120p' crates/pipeline/src/lib.rs`
- Require the implementation to:
  - keep the change limited to the reviewed public contract owners
  - add validation/constructor APIs if needed for honest downstream construction
  - keep nested helpers and module-level raw internals private
  - run any targeted tests needed for the touched code plus `cargo check --workspace`
  - stop after Packet 2.1 acceptance is met and report exact files touched, exact verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 2 Packet 2.1: Public Contract Owners`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the public contract owners match the retained/dropped matrix
  - whether construction/validation is safe and specific
  - whether nested helpers stayed private
  - whether the crate avoided whole-module publication
  - whether the packet stayed inside 2.1 scope
- Require severity labels and explicit callouts for accidental overexposure, weak validation, or scope drift.

Fix loop:
- If the review is clean, stop and report Packet 2.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.1 lands cleanly.
- Before each commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the public contract-owner work clearly and standalone.

Stop conditions:
- Stop once Packet 2.1 is review-clean, committed, and the reviewed public contract owners are landed without whole-module publication.
- Stop and report blocked if honest completion requires widening beyond the Set 2 retained/dropped matrix or drifting into later packets.
```

---

## Packet 2.2 Prompt — Declarative-Root Public Façade

```text
/goal Orchestrate Set 2 Packet 2.2: Declarative-Root Public Façade in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
- Treat this as one narrow implementation seam: expose only the retained declarative-root-aware entrypoints and keep dropped seams private unless authority is explicitly reopened.
- Packet 2.1 must already be complete.
- Stay inside Packet 2.2 only.

Hard rules:
- Do not implement, review, or fix Packet 2.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.2 only.
- Stay inside Packet 2.2 scope.

Packet 2.2 scope:
- Expose only retained declarative-root-aware entrypoints for:
  - `load_pipeline_catalog_metadata`
  - `load_pipeline_selection_metadata`
  - `load_pipeline_definition`
  - `load_selected_pipeline_definition`
- Keep `SupportedTargetRegistry::load` and route-aware `load_pipeline_catalog` private unless the retained/dropped matrix is explicitly reopened first.
- Prove custom declarative roots through package-local tests that use only public APIs.
- Expected files:
  - crates/pipeline/src/pipeline.rs
  - crates/pipeline/src/lib.rs
  - crates/pipeline/tests/pipeline_catalog.rs
  - crates/pipeline/tests/pipeline_loader.rs
  - optionally tightly related test support files

Out of scope — do NOT touch:
- Packet 2.1 contract-owner shape beyond minimal carry-forward use
- Packet 2.3 route-state storage-layout façade
- Packet 2.4 capture/handoff façade
- Packet 2.5 external consumer proof harness
- Set 3 proof, release, or downstream Substrate work
- widening to dropped declarative-root candidate seams
- unrelated cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 2 Packet 2.2: Declarative-Root Public Façade`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md`
  - `sed -n '1,220p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md`
  - `sed -n '1380,1548p' crates/pipeline/src/pipeline.rs`
  - `sed -n '2590,2635p' crates/pipeline/src/pipeline.rs`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_loader`
- Require the implementation to:
  - expose only retained declarative-root-aware public façade seams
  - avoid publicizing dropped seams from the matrix
  - keep tests public-API-only with no private module imports
  - run targeted tests plus `cargo check --workspace`
  - stop after Packet 2.2 acceptance is met and report exact files touched, exact verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 2 Packet 2.2: Declarative-Root Public Façade`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether each landed public seam matches a retained row in the matrix
  - whether dropped declarative-root seams stayed private
  - whether custom-root tests are public-API-only and meaningful
  - whether the naming and boundary shape preserve the MAP intent
  - whether the packet stayed inside 2.2 scope
- Require severity labels and explicit callouts for overexposure, ambiguous API shape, or weak proof coverage.

Fix loop:
- If the review is clean, stop and report Packet 2.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.2-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.2 lands cleanly.
- Before each commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the declarative-root façade work clearly and standalone.

Stop conditions:
- Stop once Packet 2.2 is review-clean, committed, and the retained declarative-root-aware façade is landed with public-API-only proof.
- Stop and report blocked if honest completion requires promoting dropped seams or widening into later packets.
```

---

## Packet 2.3 Prompt — Route-State Storage-Layout Public Façade

```text
/goal Orchestrate Set 2 Packet 2.3: Route-State Storage-Layout Public Façade in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
- Treat this as one narrow implementation seam: expose only the retained storage-layout-aware route-state entrypoints and keep repo-layout plumbing and extra convenience surfaces private.
- Packets 2.1 and 2.2 must already be complete.
- Stay inside Packet 2.3 only.

Hard rules:
- Do not implement, review, or fix Packet 2.3 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.3 only.
- Stay inside Packet 2.3 scope.

Packet 2.3 scope:
- Expose only retained storage-layout-aware public seams for:
  - `load_route_state`
  - `set_route_state`
  - `load_trusted_pipeline_session`
  - `persist_route_basis`
- Keep repo-layout plumbing private.
- Prove custom storage-layout route-state behavior via package-local tests using only public APIs.
- Expected files:
  - crates/pipeline/src/route_state.rs
  - crates/pipeline/src/lib.rs
  - crates/pipeline/tests/pipeline_state_store.rs
  - crates/pipeline/tests/pipeline_route_resolution.rs
  - optionally tightly related test support files

Out of scope — do NOT touch:
- Packet 2.2 declarative-root façade beyond minimal carry-forward use
- Packet 2.4 capture/handoff façade
- Packet 2.5 external consumer proof harness
- Set 3 proof, release, or downstream Substrate work
- widening to dropped/private helper surfaces
- unrelated cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 2 Packet 2.3: Route-State Storage-Layout Public Façade`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md`
  - `sed -n '1,220p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md`
  - `sed -n '438,760p' crates/pipeline/src/route_state.rs`
  - `sed -n '1140,1268p' crates/pipeline/src/route_state.rs`
  - `cargo test -p handbook-pipeline --test pipeline_state_store`
  - `cargo test -p handbook-pipeline --test pipeline_route_resolution`
- Require the implementation to:
  - expose only retained route-state storage-layout seams
  - keep helper plumbing private
  - add public-API-only tests for non-default storage roots
  - run targeted tests plus `cargo check --workspace`
  - stop after Packet 2.3 acceptance is met and report exact files touched, exact verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 2 Packet 2.3: Route-State Storage-Layout Public Façade`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the landed public seams match the retained matrix rows
  - whether non-default storage-layout behavior is actually tested
  - whether helper plumbing stayed private
  - whether Packet 4.2 is still treated as separate `engine + flow` proof only
  - whether the packet stayed inside 2.3 scope
- Require severity labels and explicit callouts for overexposure, broken storage-layout invariants, or weak test coverage.

Fix loop:
- If the review is clean, stop and report Packet 2.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.3-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.3 lands cleanly.
- Before each commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the route-state storage-layout façade work clearly and standalone.

Stop conditions:
- Stop once Packet 2.3 is review-clean, committed, and the retained route-state storage-layout façade is landed with public-API-only proof.
- Stop and report blocked if honest completion requires exposing dropped/private helper surfaces or drifting into later packets.
```

---

## Packet 2.4 Prompt — Capture + Handoff Storage-Layout Public Façade

```text
/goal Orchestrate Set 2 Packet 2.4: Capture + Handoff Storage-Layout Public Façade in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
- Treat this as one narrow implementation seam: expose only the retained capture and handoff storage-layout-aware entrypoints and keep dropped convenience seams private.
- Packets 2.1 through 2.3 must already be complete.
- Stay inside Packet 2.4 only.

Hard rules:
- Do not implement, review, or fix Packet 2.4 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.4 only.
- Stay inside Packet 2.4 scope.

Packet 2.4 scope:
- Expose only retained storage-layout-aware public seams for:
  - `preview_pipeline_capture`
  - `apply_pipeline_capture`
  - `emit_pipeline_handoff_bundle`
  - `validate_pipeline_handoff_bundle`
- Keep `capture_pipeline_output_with_storage_layout` and `load_pipeline_capture_cache_entry_with_storage_layout` private unless authority is explicitly reopened first.
- Prove custom capture and handoff roots through package-local tests using only public APIs.
- Expected files:
  - crates/pipeline/src/pipeline_capture.rs
  - crates/pipeline/src/pipeline_handoff.rs
  - crates/pipeline/src/lib.rs
  - crates/pipeline/tests/pipeline_capture.rs
  - crates/pipeline/tests/pipeline_handoff.rs
  - optionally tightly related test support files

Out of scope — do NOT touch:
- Packet 2.5 external consumer proof harness
- Set 3 proof, release, or downstream Substrate work
- widening to dropped convenience seams
- handbook product-shell wording behavior beyond what retained APIs already expose
- unrelated cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 2 Packet 2.4: Capture + Handoff Storage-Layout Public Façade`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md`
  - `sed -n '1,220p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md`
  - `sed -n '180,310p' crates/pipeline/src/pipeline_capture.rs`
  - `sed -n '220,260p' crates/pipeline/src/pipeline_handoff.rs`
  - `sed -n '536,560p' crates/pipeline/src/pipeline_handoff.rs`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
- Require the implementation to:
  - expose only retained capture/handoff public façade seams
  - keep dropped convenience seams private
  - keep tests public-API-only with non-default storage-layout proof
  - run targeted tests plus `cargo check --workspace`
  - stop after Packet 2.4 acceptance is met and report exact files touched, exact verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 2 Packet 2.4: Capture + Handoff Storage-Layout Public Façade`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the landed public seams match retained matrix rows only
  - whether dropped convenience seams stayed private
  - whether custom capture/handoff root behavior is actually proved in tests
  - whether the packet preserved the intended ownership split and avoided wording leakage
  - whether the packet stayed inside 2.4 scope
- Require severity labels and explicit callouts for overexposure, weak proof coverage, or hidden scope drift.

Fix loop:
- If the review is clean, stop and report Packet 2.4 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.4-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.4 lands cleanly.
- Before each commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.4 only.
- Commit after each accepted fix round.
- Commit messages must describe the capture/handoff storage-layout façade work clearly and standalone.

Stop conditions:
- Stop once Packet 2.4 is review-clean, committed, and the retained capture/handoff storage-layout façade is landed with public-API-only proof.
- Stop and report blocked if honest completion requires promoting dropped convenience seams or drifting into later packets.
```

---

## Packet 2.5 Prompt — Release-Candidate External Proof + Closeout

```text
/goal Orchestrate Set 2 Packet 2.5: Release-Candidate External Proof + Closeout in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.5 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
- Treat this as one narrow implementation/proof seam: add the release-candidate external consumer proof harness, prove every retained capability family from outside the crate source tree, and close Set 2 honestly without overclaiming Set 3 work.
- Packets 2.1 through 2.4 must already be complete.
- Stay inside Packet 2.5 only.

Hard rules:
- Do not implement, review, or fix Packet 2.5 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.5 only.
- Stay inside Packet 2.5 scope.

Packet 2.5 scope:
- Add a packaged external consumer proof harness that constructs non-default declarative-roots and storage-layout contracts and exercises every retained capability family through public APIs only.
- Run `cargo package -p handbook-pipeline --allow-dirty`, `cargo publish --dry-run -p handbook-pipeline`, and the proof driver.
- Write honest Set 2 closeout notes that preserve:
  - Packet 4.2 remains only `engine + flow` proof
  - Set 3 still owns released-crate proof, downstream Substrate proof, and guard rails
  - no downstream Substrate source-touching proof happened inside Set 2
- Expected files:
  - tools/proof/handbook_pipeline_minimal_boundary.sh
  - tests/fixtures/external_consumers/handbook_pipeline_minimal_boundary/Cargo.toml
  - tests/fixtures/external_consumers/handbook_pipeline_minimal_boundary/src/main.rs
  - optionally a small helper/README under the same proof directory
  - docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md

Out of scope — do NOT touch:
- downstream Substrate source code
- Set 3 released-crate publication or downstream worktree proof
- widening the retained public API matrix
- unrelated release automation or CI redesign
- unrelated cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 2 Packet 2.5: Release-Candidate External Proof + Closeout`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md`
  - `sed -n '1,220p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md`
  - `cargo package -p handbook-pipeline --allow-dirty`
  - `cargo publish --dry-run -p handbook-pipeline`
  - `bash tools/proof/handbook_pipeline_minimal_boundary.sh`
  - `cargo check --workspace`
- Require the implementation to:
  - keep the proof harness external-consumer-shaped and public-API-only
  - avoid sibling-path accidents and private module imports
  - record closeout notes that do not overclaim Set 3 work
  - stop after Packet 2.5 acceptance is met and report exact files touched, exact verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 2 Packet 2.5: Release-Candidate External Proof + Closeout`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the proof harness actually exercises every retained capability family
  - whether the proof is external-consumer-shaped and public-API-only
  - whether closeout notes preserve Packet 4.2 and Set 3 boundaries honestly
  - whether no downstream Substrate source-touching work was smuggled into Set 2
  - whether the packet stayed inside 2.5 scope
- Require severity labels and explicit callouts for false-complete proof claims, private import leakage, or weak closeout language.

Fix loop:
- If the review is clean, stop and report Packet 2.5 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.5-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.5 lands cleanly.
- Before each commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 2.5 only.
- Commit after each accepted fix round.
- Commit messages must describe the release-candidate external proof / Set 2 closeout work clearly and standalone.

Stop conditions:
- Stop once Packet 2.5 is review-clean, committed, and Set 2 has a passing release-candidate external proof plus honest closeout notes.
- Stop and report blocked if honest completion requires downstream Substrate source changes, released-crate publication work, or widening beyond the retained public API matrix.
```
