# Handbook Published-Import Decoupling — Set 3 Packet Prompts

Task source: [handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md](./handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md)
Spec source: [handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md](./handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md)
Plan source: [handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md](./handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md)
Set 1 authority source: [handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md](./handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md)
Set 2 authority source: [handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md](./handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md)
Map source: [MAP.md](./MAP.md)

These prompts are ready to paste into fresh orchestration sessions. Each one:
- starts in `/goal`
- requires a fresh **GPT-5.4 high** implementation subagent
- requires that implementation subagent to use `$incremental-implementation`
- requires a fresh **GPT-5.4 high** review subagent after the implementation lands
- requires that review subagent to use `$code-review-and-quality`
- requires a fresh **GPT-5.4 high** fix subagent for every review round that finds issues
- requires commit boundaries between implementation, review, and each accepted fix round
- keeps execution bounded to one Set 3 packet only

Set 3 is proof and guard-rail work across `/Users/spensermcconnell/__Active_Code/system` and, for Packet 3.3, a dedicated Substrate worktree under `/Users/spensermcconnell/.codex/worktrees/`. The orchestration session must not implement, review, or fix the packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

Preserve unrelated local edits, especially incidental dirt in `AGENTS.md` and `CLAUDE.md`. In `system`, run `npx gitnexus detect-changes --repo system` before every commit and confirm the affected scope matches only the current packet. Do not widen beyond the Set 2 retained/dropped matrix without reopening authority first. Do not silently reclassify Packet 4.2 as `handbook-pipeline` proof, do not treat Set 2 packaged proof as released-crate proof, and do not drift into unrelated CLI/compiler/product-shell redesign.

---

## Packet 3.1 Prompt — Released-Proof Harness + Release Preparation

```text
/goal Orchestrate Set 3 Packet 3.1: Released-Proof Harness + Release Preparation in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md
- Treat this as one narrow implementation seam: add the exact-published-version released proof harness and record the smallest honest publish target for the Set 2 boundary.
- Set 2 must already be complete.
- Stay inside Packet 3.1 only.

Hard rules:
- Do not implement, review, or fix Packet 3.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit in `system`, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 3.1 only.
- Stay inside Packet 3.1 scope.

Packet 3.1 scope:
- Add a released-crate proof harness separate from the Set 2 packaged proof harness.
- Pin the released external consumer fixture to an exact crates.io version rather than a packaged path dependency.
- Preserve the Set 2 packaged proof harness as a distinct proof tier.
- Record the smallest honest publish target needed because `handbook-pipeline 0.1.1` is insufficient for released proof.
- Expected files:
  - tools/proof/handbook_pipeline_released_boundary.sh
  - tests/fixtures/external_consumers/handbook_pipeline_released_boundary/Cargo.toml
  - tests/fixtures/external_consumers/handbook_pipeline_released_boundary/src/main.rs
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md

Out of scope — do NOT touch:
- Packet 3.2 actual crates.io publish or released proof execution against a real published version
- Packet 3.3 downstream Substrate source-touching proof
- Packet 3.4 guard rails and closeout
- widening the public API beyond the Set 2 retained/dropped matrix
- Packet 4.2 downstream seam classification
- unrelated cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 3 Packet 3.1: Released-Proof Harness + Release Preparation`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,340p' docs/specs/MAP.md`
  - `sed -n '170,190p' docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`
  - `sed -n '368,378p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md`
  - `sed -n '223,228p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md`
  - `sed -n '117,122p' docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md`
  - `sed -n '1,260p' tools/proof/handbook_pipeline_minimal_boundary.sh`
  - `cargo package -p handbook-pipeline --allow-dirty`
  - `cargo publish --dry-run -p handbook-pipeline`
- Require the implementation to:
  - keep the released proof tier distinct from the Set 2 packaged proof tier
  - pin the released fixture to exact crates.io versions only
  - avoid path-dependency fallback, sibling-path accidents, or direct source-tree reach-in
  - record the exact reason `handbook-pipeline 0.1.1` is insufficient
  - stop after Packet 3.1 acceptance is met and report exact files touched, exact verification run, and residual open questions

Review subagent prompt requirements:
- Begin with `/goal Review Set 3 Packet 3.1: Released-Proof Harness + Release Preparation`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the new proof harness is truly released-version-oriented rather than packaged-path-oriented
  - whether the fixture is exact-version pinned and fail-fast against fallback accidents
  - whether the docs record the smallest honest publish target clearly
  - whether Packet 3.1 stayed inside scope without widening public API or drifting into publish/downstream work
- Require severity labels and explicit callouts for weak exact-version proofing, fallback risk, or scope drift.

Fix loop:
- If the review is clean, stop and report Packet 3.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.1 lands cleanly.
- Before each commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 3.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the released-proof harness / release-prep work clearly and standalone.

Stop conditions:
- Stop once Packet 3.1 is review-clean, committed, and the released-proof harness plus smallest publish-target docs are landed.
- Stop and report blocked if honest completion requires actual publish work, downstream Substrate work, or widening beyond the Set 2 retained/dropped matrix.
```

---

## Packet 3.2 Prompt — Published Crates.io Proof

```text
/goal Orchestrate Set 3 Packet 3.2: Published Crates.io Proof in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md
- Treat this as one narrow implementation/proof seam: publish the first crates.io version that actually contains the Set 2 `handbook-pipeline` boundary, then prove the released external consumer against that exact published version, while preserving Packet 4.2 as `engine + flow` only.
- Packet 3.1 must already be complete.
- Stay inside Packet 3.2 only.

Hard rules:
- Do not implement, review, or fix Packet 3.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit in `system`, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 3.2 only.
- Do not run the actual `cargo publish -p handbook-pipeline` command without explicit human approval immediately before that command.
- Stay inside Packet 3.2 scope.

Packet 3.2 scope:
- Publish the first crates.io version that actually contains the Set 2 `handbook-pipeline` public boundary.
- Run the released external consumer proof against that exact published version.
- Record honest released-proof classification and preserve Packet 4.2 as `engine + flow` only.
- Expected files:
  - crates/pipeline/Cargo.toml
  - any tightly required version-coupled workspace manifests only if needed for the publish target
  - tools/proof/handbook_pipeline_released_boundary.sh
  - tests/fixtures/external_consumers/handbook_pipeline_released_boundary/Cargo.toml
  - tests/fixtures/external_consumers/handbook_pipeline_released_boundary/src/main.rs
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md

Out of scope — do NOT touch:
- Packet 3.1 released-proof harness shape beyond minimal carry-forward fixes required by review
- Packet 3.3 downstream Substrate source-touching proof
- Packet 3.4 guard rails and closeout
- widening the public API beyond the Set 2 retained/dropped matrix
- unrelated release automation redesign
- unrelated cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 3 Packet 3.2: Published Crates.io Proof`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,280p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md`
  - `sed -n '1,260p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md`
  - `sed -n '1,220p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md`
  - `cargo package -p handbook-pipeline --allow-dirty`
  - `cargo publish --dry-run -p handbook-pipeline`
  - `sed -n '1,260p' tools/proof/handbook_pipeline_released_boundary.sh`
- Require the implementation to:
  - keep actual publish work tightly scoped to the smallest honest version target
  - stop and ask the human for explicit approval immediately before running `cargo publish -p handbook-pipeline`
  - after publish, run `bash tools/proof/handbook_pipeline_released_boundary.sh --version <published_version>`
  - update proof-classification notes so Set 2 packaged proof and Set 3 released proof stay distinct
  - keep Packet 4.2 classified only as `engine + flow` proof
  - stop after Packet 3.2 acceptance is met and report exact files touched, published version, exact verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 3 Packet 3.2: Published Crates.io Proof`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the published version really contains the Set 2 boundary
  - whether the released external proof is exact-version-based and public-API-only
  - whether Packet 4.2 remained explicitly `engine + flow` only
  - whether Packet 3.2 stayed inside scope without drifting into downstream Substrate work or public-surface widening
- Require severity labels and explicit callouts for release-proof overclaiming, publish/version mistakes, or scope drift.

Fix loop:
- If the review is clean, stop and report Packet 3.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.2-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.2 lands cleanly.
- Before each commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 3.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the published-crate proof work clearly and standalone.

Stop conditions:
- Stop once Packet 3.2 is review-clean, committed, the released external proof passes against the exact published version, and Packet 4.2 remains clearly `engine + flow` only.
- Stop and report blocked if honest completion requires downstream Substrate work, public-surface widening, or publish actions the human does not approve.
```

---

## Packet 3.3 Prompt — Downstream Substrate Published-Boundary Proof

```text
/goal Orchestrate Set 3 Packet 3.3: Downstream Substrate Published-Boundary Proof using a dedicated Substrate worktree plus the active authority docs in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md
- Treat this as one narrow downstream proof seam: choose one minimal Substrate production seam in a dedicated worktree, pin the exact published version proven in Packet 3.2, and prove real `handbook-pipeline` capability while preserving Substrate-owned wording and runtime behavior.
- Packet 3.2 must already be complete.
- Stay inside Packet 3.3 only.

Hard rules:
- Do not implement, review, or fix Packet 3.3 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Do not use the main Substrate checkout as the write path. Use a dedicated worktree under `/Users/spensermcconnell/.codex/worktrees/` only.
- In `system`, if Packet 3.3 changes the active docs, run `npx gitnexus detect-changes --repo system` before every `system` commit and confirm the affected scope matches Packet 3.3 only.
- In the downstream worktree, follow that repo's local instructions and verify the affected scope matches Packet 3.3 only before every downstream commit.
- Stay inside Packet 3.3 scope.

Packet 3.3 scope:
- Choose one narrow downstream proof seam in a dedicated Substrate worktree.
- Pin the exact published `handbook-pipeline` version proved in Packet 3.2.
- Implement only the smallest downstream seam needed to prove real `handbook-pipeline` capability.
- Record an explicit downstream capability map so the proof does not overclaim family usage.
- Expected files:
  - one narrow downstream file group in the dedicated Substrate worktree, ideally no more than ~5 files
  - tightly related downstream tests only if needed for honest proof
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md

Out of scope — do NOT touch:
- Packet 3.1 released-proof harness design
- Packet 3.2 publish/released-proof mechanics beyond minimal carry-forward use
- Packet 3.4 guard rails and closeout
- the main Substrate checkout
- broad Substrate redesign, unrelated runtime cleanup, or public-surface widening
- reclassifying Packet 4.2 as `handbook-pipeline` proof

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 3 Packet 3.3: Downstream Substrate Published-Boundary Proof`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch` in `system`
  - `git status --short --branch` in the dedicated Substrate worktree
  - `sed -n '1,320p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md`
  - `sed -n '1,280p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md`
  - `sed -n '1,220p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md`
  - `cargo tree -p handbook-engine`
  - `cargo tree -p handbook-flow`
  - `cargo tree -p handbook-pipeline`
  - `cargo check --workspace`
- Require the implementation to:
  - choose the narrowest honest downstream seam
  - pin the exact published version from Packet 3.2 rather than any path override
  - preserve Substrate-owned wording and runtime behavior
  - keep the proof honest about which retained `handbook-pipeline` capability families are actually used downstream now
  - avoid widening into a broader downstream redesign or into Set 3.4 guard rails
  - stop after Packet 3.3 acceptance is met and report exact files touched, exact proof seam, exact verification run, capability families consumed downstream, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 3 Packet 3.3: Downstream Substrate Published-Boundary Proof`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the downstream seam really uses the published `handbook-pipeline` boundary rather than path or sibling overrides
  - whether final wording and runtime behavior remain Substrate-owned
  - whether the proof is narrow and honest instead of overclaiming full family usage
  - whether the work stayed in a dedicated Substrate worktree
  - whether scope leaked into Packet 3.4 or broader Substrate redesign
- Require severity labels and explicit callouts for path-fallback risk, ownership-boundary drift, or proof overclaiming.

Fix loop:
- If the review is clean, stop and report Packet 3.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.3-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.3 lands cleanly.
- Before each `system` commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 3.3 only.
- Before each downstream worktree commit, verify the affected scope matches Packet 3.3 only and follow that repo's local instructions.
- Commit after each accepted fix round.
- Commit messages must describe the downstream published-boundary proof seam clearly and standalone.

Stop conditions:
- Stop once Packet 3.3 is review-clean, committed, the downstream worktree proves real `handbook-pipeline` usage against the exact published version, and the capability map is explicit and honest.
- Stop and report blocked if honest completion requires using the main Substrate checkout, widening the public boundary, or redesigning a broader downstream surface.
```

---

## Packet 3.4 Prompt — Guard Rails + Honest Closeout

```text
/goal Orchestrate Set 3 Packet 3.4: Guard Rails + Honest Closeout in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-tasks.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md
- Treat this as one narrow closeout seam: add the released-boundary proof/update guard rails, lock the truth-classification guard rails so Packet 4.2 cannot be mistaken for `handbook-pipeline` proof, and close Set 3 honestly against the MAP objective.
- Packets 3.1, 3.2, and 3.3 must already be complete.
- Stay inside Packet 3.4 only.

Hard rules:
- Do not implement, review, or fix Packet 3.4 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit in `system`, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 3.4 only.
- Stay inside Packet 3.4 scope.

Packet 3.4 scope:
- Add release/update guard rails that re-run the released-boundary proof and reject path-fallback proof.
- Add truth-classification guard rails so Packet 4.2 cannot be mistaken for `handbook-pipeline` proof.
- Close Set 3 honestly against the MAP objective and exact intent.
- Expected files:
  - tools/proof/handbook_pipeline_released_boundary.sh
  - optionally `justfile` or a tightly scoped CI/release helper if needed
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md
  - docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md
  - optionally `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md` only if a final audit addendum is required

Out of scope — do NOT touch:
- Packet 3.1 released-proof harness architecture beyond minimal guard-rail carry-forward
- Packet 3.2 publish logic beyond minimal rerun hooks
- Packet 3.3 downstream implementation beyond classification/closeout references
- widening the public surface beyond the Set 2 retained/dropped matrix
- unrelated CI/release automation redesign
- unrelated cleanup

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 3 Packet 3.4: Guard Rails + Honest Closeout`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,340p' docs/specs/MAP.md`
  - `sed -n '1,320p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-spec.md`
  - `sed -n '1,280p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-plan.md`
  - `sed -n '1,240p' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-tasks.md`
  - `bash tools/proof/handbook_pipeline_released_boundary.sh --version <published_version>`
  - `rg -n "engine \+ flow|handbook-pipeline proof|Packet 4\.2|packaged proof|released proof" docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-*.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-*.md`
- Require the implementation to:
  - add only the minimum proof/update guard rails needed to keep released proof honest
  - make truth-classification drift around Packet 4.2 impossible to state honestly in the active docs
  - preserve the distinction between Set 2 packaged proof, Set 3 released proof, and Set 3 downstream proof
  - keep closeout language tied to the exact proofs that actually passed
  - stop after Packet 3.4 acceptance is met and report exact files touched, exact guard rails added, exact verification run, and any remaining limitations

Review subagent prompt requirements:
- Begin with `/goal Review Set 3 Packet 3.4: Guard Rails + Honest Closeout`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the new guard rails actually protect against released-proof drift and path fallback
  - whether Packet 4.2 classification is locked clearly as `engine + flow` only
  - whether the closeout notes satisfy the MAP exact objective and exact intent without overclaiming
  - whether Packet 3.4 stayed inside scope without reopening Set 2 API design or broader release automation redesign
- Require severity labels and explicit callouts for weak guard rails, proof misclassification, or closeout overclaiming.

Fix loop:
- If the review is clean, stop and report Packet 3.4 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.4-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.4 lands cleanly.
- Before each commit, run `npx gitnexus detect-changes --repo system` and confirm the affected scope matches Packet 3.4 only.
- Commit after each accepted fix round.
- Commit messages must describe the Set 3 guard-rail / closeout work clearly and standalone.

Stop conditions:
- Stop once Packet 3.4 is review-clean, committed, the released-proof/update guard rails are in place, Packet 4.2 classification is locked honestly, and the closeout notes satisfy the MAP exact objective and exact intent.
- Stop and report blocked if honest closeout requires widening public surface, redesigning unrelated release automation, or changing the historical truth of Packet 4.2.
```
