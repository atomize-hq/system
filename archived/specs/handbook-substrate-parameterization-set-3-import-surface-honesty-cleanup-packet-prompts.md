# Handbook Substrate Parameterization — Set 3 Packet Prompts

Task source: [handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md](./handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md)
Spec source: [handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md](./handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md)
Plan source: [handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-plan.md](./handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation and fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps execution bounded to Set 3 only.

This set is approved only for the `handbook-flow`, `handbook-pipeline`, and `handbook-engine` import-surface honesty-cleanup seam captured in this artifact: Packet 3.1 (flow residual refusal and fallback cleanup), Packet 3.2 (pipeline validation/refusal wording cleanup), and Packet 3.3 (engine residual default bounding and final proof). Do not reopen Set 1 or Set 2 without explicit contradictory proof, do not widen into CLI/compiler cleanup, and do not execute actual Substrate import work. If a packet cannot land honestly without widening beyond that seam, stop and report the blocker instead of silently expanding scope.

The orchestration session must not implement, review, or fix packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

---

## Packet 3.1 Prompt — Flow Residual Refusal And Fallback Cleanup

```text
/goal Orchestrate Set 3 Packet 3.1: Flow Residual Refusal And Fallback Cleanup in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md
- Treat the packet as one narrow seam: clean or explicitly bound the remaining flow refusal/blocker/result surfaces that still encode handbook-product canonical-root wording after Set 2 landed.
- Set 1 and Set 2 must already be complete.
- Stay inside Packet 3.1 only.

Hard rules:
- Do not implement, review, or fix Packet 3.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Preserve unrelated local edits, including any dirt already present in `AGENTS.md` and `CLAUDE.md`.
- Before editing any production symbol in `crates/flow/src/resolver.rs`, run GitNexus impact analysis first and report the blast radius. If GitNexus says the index is stale, run `npx gitnexus analyze` first. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.1 only.
- Stay inside Packet 3.1 scope.

Packet 3.1 scope:
- Clean or explicitly bound residual flow refusal/blocker/result surfaces that still encode handbook-product canonical-root wording.
- Preserve the explicit default wrapper path in `resolve(...)`.
- Keep downstream-visible proof/result surfaces honest when a non-default canonical layout contract is in use.
- Update only the minimal test coverage needed to prove the cleaned behavior.
- Expected files:
  - crates/flow/src/resolver.rs
  - crates/flow/tests/resolver_core.rs

Out of scope — do NOT touch:
- Packet 3.2 pipeline validation/refusal wording cleanup
- Packet 3.3 engine residual default bounding or final proof notes
- Set 1 or Set 2 structural contract design
- CLI/compiler/product-shell cleanup
- actual Substrate import execution
- broader flow API redesign beyond the residual honesty seam

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 3 Packet 3.1: Flow Residual Refusal And Fallback Cleanup`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '430,760p' crates/flow/src/resolver.rs`
  - `sed -n '1040,1125p' crates/flow/src/resolver.rs`
  - `sed -n '1,260p' crates/flow/tests/resolver_core.rs`
  - `cargo test -p handbook-flow --test resolver_core`
  - `cargo test -p handbook-flow`
  - `rg -n "missing canonical \.handbook root|canonical \.handbook root|\.handbook/" crates/flow/src crates/flow/tests`
- Require the implementation to:
  - verify Set 1 and Set 2 are already complete enough for Set 3 to proceed
  - run GitNexus impact analysis before editing production symbols and report the blast radius
  - clean or explicitly bound only the residual flow surfaces that still misstate the active contract story
  - preserve `resolve(...)` as the explicit handbook-product default wrapper path
  - keep proof/result surfaces honest for non-default canonical layout contracts
  - avoid widening into pipeline wording cleanup, engine cleanup, or a broader flow API redesign
  - stop after Packet 3.1 acceptance is met and report touched files, impact-analysis results, verification run, remaining bounded defaults, and any blockers

Review subagent prompt requirements:
- Begin with `/goal Review Set 3 Packet 3.1: Flow Residual Refusal And Fallback Cleanup`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the remaining flow-facing contract story is honest for both default and non-default canonical-root callers
  - whether `resolve(...)` stayed an explicit default wrapper instead of hidden fallback behavior
  - whether any remaining `.handbook/**` references in flow are clearly bounded as default-helper or proof-only behavior
  - whether scope leaked into Packet 3.2, Packet 3.3, or Set 2 structural redesign
- Require severity labels and explicit callouts for hidden default fallback, false residual bounding, or scope drift.

Fix loop:
- If the review is clean, stop and report Packet 3.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the flow residual honesty cleanup clearly and standalone.

Stop conditions:
- Stop once Packet 3.1 is review-clean, committed, and the remaining flow refusal/blocker/result surfaces are honest relative to the active canonical layout contract.
- Stop and report blocked if truthful cleanup requires reopening Set 2 public API design or widening into later packets.
```

---

## Packet 3.2 Prompt — Pipeline Validation / Refusal Wording Cleanup

```text
/goal Orchestrate Set 3 Packet 3.2: Pipeline Validation / Refusal Wording Cleanup in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md
- Treat the packet as one narrow seam: make import-facing pipeline validation and refusal wording follow the active declarative roots instead of handbook-product root displays.
- Set 1 and Set 2 must already be complete.
- Stay inside Packet 3.2 only.

Hard rules:
- Do not implement, review, or fix Packet 3.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Preserve unrelated local edits, including any dirt already present in `AGENTS.md` and `CLAUDE.md`.
- Before editing any production symbol in `crates/pipeline/src/pipeline.rs`, run GitNexus impact analysis first and report the blast radius. If GitNexus says the index is stale, run `npx gitnexus analyze` first. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.2 only.
- Stay inside Packet 3.2 scope.

Packet 3.2 scope:
- Derive import-facing pipeline validation and refusal wording from the active declarative roots.
- Keep explicit handbook-product default-helper surfaces bounded while removing misleading validation text from the reusable import story.
- Update only the tests whose assertions are inseparable from the cleaned wording.
- Expected files:
  - crates/pipeline/src/pipeline.rs
  - crates/pipeline/tests/pipeline_loader.rs
  - crates/pipeline/tests/pipeline_compile.rs
  - crates/pipeline/tests/pipeline_route_resolution.rs
  - crates/pipeline/tests/pipeline_catalog.rs
  - crates/pipeline/src/declarative_roots.rs (only if explicit default-helper wording needs alignment)
  - crates/pipeline/src/layout.rs (only if explicit default bounding notes need alignment)

Out of scope — do NOT touch:
- Packet 3.1 flow residual cleanup
- Packet 3.3 engine residual default bounding or final proof notes
- Set 1 structural declarative/storage-layout contract design
- Set 2 structural flow contract design
- CLI/compiler/product-shell cleanup
- actual Substrate import execution

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 3 Packet 3.2: Pipeline Validation / Refusal Wording Cleanup`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '2920,3015p' crates/pipeline/src/pipeline.rs`
  - `sed -n '1,240p' crates/pipeline/tests/pipeline_loader.rs`
  - `sed -n '1,240p' crates/pipeline/tests/pipeline_compile.rs`
  - `sed -n '1,260p' crates/pipeline/tests/pipeline_route_resolution.rs`
  - `sed -n '1,220p' crates/pipeline/tests/pipeline_catalog.rs`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_loader`
  - `cargo test -p handbook-pipeline --test pipeline_compile`
  - `cargo test -p handbook-pipeline --test pipeline_route_resolution`
  - `rg -n "core/stages/|core/pipelines/|must live under|pipeline YAML must" crates/pipeline/src/pipeline.rs crates/pipeline/tests`
- Require the implementation to:
  - verify Set 1 and Set 2 are already complete enough for Set 3 to proceed
  - run GitNexus impact analysis before editing production symbols and report the blast radius
  - make import-facing validation/refusal wording follow the active declarative roots
  - preserve explicit handbook-product default helpers as explicit defaults rather than hidden assumptions
  - update only the inseparable tests needed to prove the new wording behavior
  - avoid reopening Set 1 structural seams or widening into engine/flow/CLI/compiler cleanup
  - stop after Packet 3.2 acceptance is met and report touched files, impact-analysis results, verification run, remaining bounded defaults, and any blockers

Review subagent prompt requirements:
- Begin with `/goal Review Set 3 Packet 3.2: Pipeline Validation / Refusal Wording Cleanup`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether import-facing pipeline/stage validation text now follows the active declarative roots
  - whether explicit handbook-product defaults remain clearly bounded rather than silently reasserted as the only supported layout
  - whether the updated tests actually prove contract-honest wording instead of only default-path behavior
  - whether any scope leaked into Set 1 structural rewiring, flow cleanup, or engine cleanup
- Require severity labels and explicit callouts for false wording proof, misleading default-helper behavior, or scope drift.

Fix loop:
- If the review is clean, stop and report Packet 3.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.2-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the pipeline validation/refusal wording cleanup clearly and standalone.

Stop conditions:
- Stop once Packet 3.2 is review-clean, committed, and import-facing pipeline validation/refusal wording is honest relative to the active declarative roots.
- Stop and report blocked if truthful cleanup requires reopening Set 1 structural contracts or widening into other packets.
```

---

## Packet 3.3 Prompt — Engine Residual Default Bounding And Final Proof

```text
/goal Orchestrate Set 3 Packet 3.3: Engine Residual Default Bounding And Final Proof in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md
- Treat the packet as one narrow seam: clean or explicitly bound the remaining engine-side import-target defaults, then run the full Set 3 proof wall and record the final bounded-default inventory honestly.
- Packets 3.1 and 3.2 must already be complete.
- Stay inside Packet 3.3 only.

Hard rules:
- Do not implement, review, or fix Packet 3.3 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Preserve unrelated local edits, including any dirt already present in `AGENTS.md` and `CLAUDE.md`.
- Before editing any production symbol in `crates/engine/src/canonical_artifacts.rs`, `crates/engine/src/canonical_paths.rs`, or `crates/engine/src/author/*.rs`, run GitNexus impact analysis first and report the blast radius. If GitNexus says the index is stale, run `npx gitnexus analyze` first. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.3 only.
- Stay inside Packet 3.3 scope.

Packet 3.3 scope:
- Clean or explicitly bound engine-side residual default references that affect the import-target contract story.
- Run the full Set 3 verification wall.
- Record pass/fail in the completion notes of the Set 3 tasks doc.
- Record the final bounded-default inventory honestly.
- Refresh adjacent boundary docs only if the final import-target story would otherwise be misstated.
- Expected files:
  - crates/engine/src/canonical_artifacts.rs
  - crates/engine/src/canonical_paths.rs
  - crates/engine/src/author/charter_core.rs
  - crates/engine/src/author/environment_inventory_core.rs
  - crates/engine/tests/author_core.rs
  - docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md
  - adjacent boundary docs only if strictly necessary for honest closeout

Out of scope — do NOT touch:
- Packet 3.1 flow cleanup beyond confirming it already landed
- Packet 3.2 pipeline cleanup beyond confirming it already landed
- Set 1 or Set 2 structural contract design
- CLI/compiler/product-shell cleanup
- actual Substrate import execution
- speculative contract redesign beyond the residual default-bounding seam

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 3 Packet 3.3: Engine Residual Default Bounding And Final Proof`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '340,380p' crates/engine/src/canonical_artifacts.rs`
  - `sed -n '1,120p' crates/engine/src/author/environment_inventory_core.rs`
  - `sed -n '1,80p' crates/engine/src/author/charter_core.rs`
  - `cargo test -p handbook-flow --test resolver_core`
  - `cargo test -p handbook-flow`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_loader`
  - `cargo test -p handbook-pipeline --test pipeline_compile`
  - `cargo test -p handbook-pipeline --test pipeline_route_resolution`
  - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - `cargo test -p handbook-engine --test baseline_validation`
  - `cargo test -p handbook-engine --test author_core`
  - `cargo check --workspace`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `rg -n "missing canonical \.handbook root|canonical \.handbook root|core/stages/|core/pipelines/|\.handbook/" crates/engine/src crates/pipeline/src crates/flow/src`
- Require the implementation to:
  - verify Packets 3.1 and 3.2 landed first
  - run GitNexus impact analysis before editing production symbols and report the blast radius
  - clean contract-sensitive engine defaults or explicitly bound remaining handbook-product authoring/default surfaces
  - run the full verification wall and record exact pass/fail results in the Packet 3.3 completion notes
  - distinguish acceptable retained product defaults from misleading defaults that should not survive the reusable import promise
  - avoid widening into CLI/compiler cleanup or reopening structural Set 1 / Set 2 work unless contradictory proof forces a blocker report
  - stop after Packet 3.3 acceptance is met and report touched files, impact-analysis results, verification run, residual bounded defaults, boundary-doc changes, and any blockers

Review subagent prompt requirements:
- Begin with `/goal Review Set 3 Packet 3.3: Engine Residual Default Bounding And Final Proof`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether engine-side default text is now honest relative to the final import-target contract story
  - whether any retained `.handbook/**` references are clearly bounded as handbook-product authoring/default behavior rather than reusable import contract truth
  - whether the full Set 3 verification wall actually ran and was recorded accurately
  - whether the final bounded-default inventory is honest about what remains intentionally code-owned
  - whether any scope leaked into CLI/compiler cleanup or structural redesign from earlier sets
- Require severity labels and explicit callouts for false bounding, inaccurate proof notes, or hidden scope widening.

Fix loop:
- If the review is clean, stop and report Packet 3.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.3-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.3 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the engine residual default bounding / Set 3 final proof work clearly and standalone.

Stop conditions:
- Stop once Packet 3.3 is review-clean, committed, and the Set 3 tasks doc records both the full verification wall and the final bounded-default inventory honestly.
- Stop and report blocked if truthful cleanup requires widening into CLI/compiler cleanup or reopening earlier structural sets.
```
