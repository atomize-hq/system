# Handbook Engine Extraction Phase 6 Slice 1 Packet Prompts

Task source: [handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md](./handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md)
Spec source: [handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md](./handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md)
Plan source: [handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md](./handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps Phase 6 strictly at reassessment/planning scope rather than ownership/import implementation.

These packets are expected to be validation/planning/documentation work. If live repo truth suggests a production-code change is required, do not silently “fix” the code inside Phase 6. Capture that as a blocker and route it back to the owning earlier seam.

Do not advance to the next packet until the current packet is review-clean and committed.

## Packet 6.1.1 Prompt

```text
/goal Orchestrate Phase 6 Slice 1 Packet 6.1.1: Freeze Live Repo Truth And Revalidate The Migration Gate in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
- Use the root authorities at:
  - /Users/spensermcconnell/__Active_Code/system/HANDBOOK_ENGINE_EXTRACTION_PLAN.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-slice-map.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Treat the current working tree as the truth surface and explicitly record whether Packet 6.1.1 is validating committed truth or local-only truth.
- Do not start Packet 6.1.2, 6.1.3, or 6.1.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- This packet is expected to be docs/validation-only. Do not edit production code. If live truth proves a code regression exists, capture it as a blocker rather than repairing it here.
- If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.1.1 only.
- Stay inside Packet 6.1.1 scope.

Packet 6.1.1 scope:
- Record branch / HEAD / working-tree truth.
- Re-read the root authorities and confirm they still name Phase 6 as the next authoritative step.
- Re-run the representative proof rails and full verification wall.
- Update only the Phase 6 slice docs and any narrowly necessary packet-local evidence note under `docs/specs/` if the agent needs one.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md
  - optionally one narrow Phase-6-local evidence note under `docs/specs/`

Out of scope:
- any ownership/import planning
- any crate move or integration implementation
- any repair to earlier extraction code
- any authority rewrite outside narrow truth corrections strictly required by Packet 6.1.1

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Slice 1 Packet 6.1.1: Freeze Live Repo Truth And Revalidate The Migration Gate`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to treat this as validation/planning/documentation work only.
- Require live verification with:
  - `git status --short --branch`
  - `git log --oneline --decorate -20`
  - `rg -n "Phase 6|Migration Gate|Exit criteria|Open Questions|next authoritative step" HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `rg -n "Phase 6|next step|fully landed through Slice 5.3|Authority And Assumptions" docs/specs/handbook-engine-extraction-slice-map.md`
  - `rg -n "Phase 6|verification wall|four-set closeout|next authoritative step" docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
  - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
- Require the subagent to explicitly record:
  - whether the tree was clean or dirty
  - whether validation was against committed HEAD truth or unstaged local truth
  - whether the root plan, slice map, and closeout map still agree that Phase 6 is next
  - whether the representative rails and full wall passed
  - whether any failure is a true blocker
- Require the subagent to stop after Packet 6.1.1 acceptance is met and report touched files, verification outputs, blocker list if any, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Slice 1 Packet 6.1.1: Freeze Live Repo Truth And Revalidate The Migration Gate`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, applied to the packet’s validation story and any docs changes.
- Require the reviewer to review the packet against the Slice 6.1 spec, plan, tasks, and the live command outputs.
- Require special attention to:
  - whether committed-vs-local truth was stated explicitly
  - whether the verification wall was actually rerun rather than assumed
  - whether failures, if any, were classified honestly as blockers vs non-blocking notes
  - whether the packet stayed docs/validation-only
- Require severity labels and explicit callouts if the verification story is incomplete, if the repo-truth call is overstated, or if the packet widened into code repair or ownership planning.

Fix loop:
- If the review is clean, stop and report Packet 6.1.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.1.1-bounded changes needed to close them.
- Re-run only the verification commands affected by the fix. Do not rerun unchanged verification commands just for reassurance.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 6.1.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.1.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 6.1.1 truth-freeze / migration-gate revalidation change clearly and standalone.

Stop conditions:
- Stop once Packet 6.1.1 is review-clean, committed, and the truth-freeze plus verification wall evidence is recorded cleanly.
- Stop and report blocked if Packet 6.1.1 finds a real failing verification rail or authority contradiction that prevents an honest Phase 6 baseline.
```

## Packet 6.1.2 Prompt

```text
/goal Orchestrate Phase 6 Slice 1 Packet 6.1.2: Reassess Extracted Crate Boundaries Against The Ownership Rule in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md.
- Assume Packet 6.1.1 is already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
- Use the root authorities at:
  - /Users/spensermcconnell/__Active_Code/system/HANDBOOK_ENGINE_EXTRACTION_PLAN.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-slice-map.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not reopen Packet 6.1.1 except where a tiny documentation correction is strictly required for Packet 6.1.2 to land correctly.
- Do not start Packet 6.1.3 or 6.1.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- This packet is expected to be docs/analysis-only. Do not edit production code. If a real code blocker is discovered, capture it and route it back to the owning earlier seam.
- If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.1.2 only.
- Stay inside Packet 6.1.2 scope.

Packet 6.1.2 scope:
- Reassess `handbook-engine`, `handbook-pipeline`, `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` against the root ownership decision rule.
- Produce a crate-by-crate ownership/readiness matrix grounded in live surfaces.
- Keep the packet limited to analysis and documentation of ownership posture.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md
  - optionally one narrow Phase-6-local ownership matrix or evidence note under `docs/specs/`

Out of scope:
- writing the follow-on ownership/import plan
- moving crates into Substrate
- code repair work in engine, pipeline, flow, compiler, or CLI
- rewriting earlier closeout docs beyond tiny truth corrections strictly needed for Packet 6.1.2

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Slice 1 Packet 6.1.2: Reassess Extracted Crate Boundaries Against The Ownership Rule`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 6.1.1 already landed and that the migration-gate baseline is recorded before editing.
- Require the subagent to keep this packet bounded to ownership/readiness assessment and to explicitly preserve:
  - no assumption that extracted crates automatically belong in Substrate
  - the bounded current runtime wedge being treated as intentional unless live evidence proves otherwise
  - `handbook-cli` remaining a product shell rather than a move target
  - retained `handbook-compiler` being assessed honestly as compatibility/support glue, not the implementation center
- Require evidence gathering with:
  - `rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs crates/cli/src/main.rs`
  - `cargo tree -p handbook-engine`
  - `cargo tree -p handbook-pipeline`
  - `cargo tree -p handbook-flow`
  - `cargo tree -p handbook-compiler`
- Require the subagent to produce an explicit matrix that answers, for each crate:
  - handbook-domain vs substrate-domain center of gravity
  - whether Substrate should likely import it through a clean boundary
  - whether ownership should remain handbook-side longer
  - whether any handbook-product assumptions remain relevant
- Require the subagent not to rerun Packet 6.1.1’s full wall unless files affecting the wall changed or the subagent has concrete reason that the baseline is stale.
- Require the subagent to stop after Packet 6.1.2 acceptance is met and report touched files, evidence used, the crate-by-crate matrix, any discovered blocker seams, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Slice 1 Packet 6.1.2: Reassess Extracted Crate Boundaries Against The Ownership Rule`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, applied to the ownership assessment and any docs changes.
- Require the reviewer to review the packet against the Slice 6.1 spec, plan, tasks, Packet 6.1.1 baseline, and the live evidence.
- Require special attention to:
  - whether every crate got an explicit call rather than implied treatment
  - whether handbook-domain vs substrate-domain reasoning is evidence-backed rather than speculative
  - whether `handbook-flow` and retained `handbook-compiler` were handled honestly rather than waved through
  - whether the packet stayed out of follow-on ownership/import planning
- Require severity labels and explicit callouts if the matrix is incomplete, if assumptions are unstated, or if the packet widened into planning or code repair.

Fix loop:
- If the review is clean, stop and report Packet 6.1.2 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.1.2-bounded changes needed to close them.
- Re-run only the evidence/verification commands affected by the fix.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 6.1.2 implementation if it lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.1.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 6.1.2 ownership-boundary reassessment clearly and standalone.

Stop conditions:
- Stop once Packet 6.1.2 is review-clean, committed, and every extracted crate has an explicit ownership/readiness posture.
- Stop and report blocked if Packet 6.1.2 cannot produce an honest crate-by-crate matrix without first repairing a concrete earlier-seam regression.
```

## Packet 6.1.3 Prompt

```text
/goal Orchestrate Phase 6 Slice 1 Packet 6.1.3: Resolve The Readiness Verdict And Explicit Deferrals in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.1.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md.
- Assume Packets 6.1.1 and 6.1.2 are already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
- Use the root authorities at:
  - /Users/spensermcconnell/__Active_Code/system/HANDBOOK_ENGINE_EXTRACTION_PLAN.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-slice-map.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Preserve the Packet 6.1.1 baseline and Packet 6.1.2 ownership matrix; do not reopen them except where a tiny documentation correction is strictly required for Packet 6.1.3 to land correctly.
- Do not start Packet 6.1.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- This packet is expected to be docs/verdict-only. Do not edit production code. If a real code blocker is discovered, capture it and route it back to the owning earlier seam.
- If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.1.3 only.
- Stay inside Packet 6.1.3 scope.

Packet 6.1.3 scope:
- Produce one final crate-by-crate ownership/readiness matrix with explicit blockers vs open questions.
- Resolve the Phase 6 verdict for whether the repo is ready for a separate ownership/integration planning family.
- Make deferrals explicit without starting the next family.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md
  - optionally one narrow Phase-6-local verdict / matrix note under `docs/specs/`

Out of scope:
- Packet 6.1.4 next-boundary naming
- authoring the follow-on ownership/import plan
- any code repair or earlier-seam implementation
- broad edits to the root plan or slice map beyond tiny truth corrections strictly needed for this verdict packet

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Slice 1 Packet 6.1.3: Resolve The Readiness Verdict And Explicit Deferrals`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packets 6.1.1 and 6.1.2 already landed and that the baseline plus crate matrix exist before editing.
- Require the subagent to keep this packet bounded to verdict/deferral writing and to explicitly preserve:
  - blockers being separated from non-blocking open questions
  - handbook-owned/imported vs candidate-future-move vs explicitly deferred postures being stated per crate
  - no premature authorship of the next ownership/import planning family
  - no code repair hidden inside “readiness cleanup”
- Require the subagent to base the verdict on:
  - the Packet 6.1.1 truth-freeze and verification wall
  - the Packet 6.1.2 crate-by-crate matrix
  - the root Phase 6 checklist in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- Require the subagent to state clearly:
  - whether the repo is ready for a separate ownership/integration planning family
  - what remains explicitly deferred
  - whether retained `handbook-compiler` posture is merely temporary glue or still a readiness ambiguity
- Require the subagent not to rerun unchanged verification commands unless the docs edits or new evidence genuinely require it.
- Require the subagent to stop after Packet 6.1.3 acceptance is met and report touched files, final verdict, blocker list, open-question list, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Slice 1 Packet 6.1.3: Resolve The Readiness Verdict And Explicit Deferrals`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, applied to the verdict logic and any docs changes.
- Require the reviewer to review the packet against the Slice 6.1 spec, plan, tasks, Packet 6.1.1 baseline, Packet 6.1.2 matrix, and the root Phase 6 checklist.
- Require special attention to:
  - whether blockers and open questions are clearly separated
  - whether every crate posture is reflected in the verdict
  - whether the packet overclaims readiness because tests were green
  - whether retained `handbook-compiler` ambiguity was handled honestly
  - whether the packet stayed out of Packet 6.1.4 next-boundary work
- Require severity labels and explicit callouts if the verdict is not evidence-backed, if blocker/open-question categories blur together, or if the packet widened beyond verdict/deferral resolution.

Fix loop:
- If the review is clean, stop and report Packet 6.1.3 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.1.3-bounded changes needed to close them.
- Re-run only the evidence/verification commands affected by the fix.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 6.1.3 implementation if it lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.1.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 6.1.3 readiness verdict / deferral resolution clearly and standalone.

Stop conditions:
- Stop once Packet 6.1.3 is review-clean, committed, and the Phase 6 readiness verdict is explicit and honest.
- Stop and report blocked if Packet 6.1.3 cannot resolve the verdict because Packet 6.1.1 or 6.1.2 evidence is still incomplete or because a real earlier-seam blocker surfaced.
```

## Packet 6.1.4 Prompt

```text
/goal Orchestrate Phase 6 Slice 1 Packet 6.1.4: Name The Next Planning Boundary Without Starting It in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 6.1.4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md.
- Assume Packets 6.1.1, 6.1.2, and 6.1.3 are already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
- Use the root authorities at:
  - /Users/spensermcconnell/__Active_Code/system/HANDBOOK_ENGINE_EXTRACTION_PLAN.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-slice-map.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Preserve the Packet 6.1.1 baseline, Packet 6.1.2 matrix, and Packet 6.1.3 verdict; do not reopen them except where a tiny documentation correction is strictly required for Packet 6.1.4 to land correctly.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- This packet is expected to be docs/boundary-only. Do not edit production code. If a real code blocker is discovered, capture it and route it back to the owning earlier seam.
- If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 6.1.4 only.
- Stay inside Packet 6.1.4 scope.

Packet 6.1.4 scope:
- If Packet 6.1.3 concluded READY, name the exact follow-on ownership/integration planning family without generating it here.
- If Packet 6.1.3 concluded NOT READY, route the blocker back to a narrow earlier seam instead of vague “more Phase 6”.
- Keep this packet strictly at the planning-boundary line.
- Expected files:
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md
  - docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md
  - optionally one narrow Phase-6-local next-boundary note under `docs/specs/`

Out of scope:
- authoring the next ownership/integration spec/plan/tasks family
- implementing any ownership/import work
- broad restatement of earlier packet evidence beyond what is required to name the next boundary
- repairing earlier extraction code inside this packet

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Slice 1 Packet 6.1.4: Name The Next Planning Boundary Without Starting It`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packets 6.1.1 through 6.1.3 already landed and that the final verdict is explicit before editing.
- Require the subagent to keep this packet bounded to next-boundary naming and to explicitly preserve:
  - no silent continuation into the follow-on family
  - if READY, only naming the next family and its scope
  - if NOT READY, only naming the narrow blocker seam and its scope
  - no code repair or ownership/import implementation
- Require the subagent to write down:
  - the exact next family name if READY
  - why that family is the next honest step
  - or, if NOT READY, the exact earlier seam owner and why it blocks follow-on planning
- Require the subagent not to rerun unchanged verification commands unless the packet discovers a contradiction that actually requires revalidation.
- Require the subagent to stop after Packet 6.1.4 acceptance is met and report touched files, next-boundary statement, blocker seam if any, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Slice 1 Packet 6.1.4: Name The Next Planning Boundary Without Starting It`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance, applied to the planning-boundary logic and any docs changes.
- Require the reviewer to review the packet against the Slice 6.1 spec, plan, tasks, the Packet 6.1.3 verdict, and the earlier packet evidence.
- Require special attention to:
  - whether the next boundary is explicit enough for a future session to start cleanly
  - whether READY vs NOT READY branching is handled correctly
  - whether the packet accidentally started authoring the follow-on family
  - whether blockers are routed to a named earlier seam rather than vague “more reassessment”
- Require severity labels and explicit callouts if the next boundary is ambiguous, if the packet widened into follow-on planning, or if blocker routing is too vague to be actionable.

Fix loop:
- If the review is clean, stop and report Packet 6.1.4 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-6.1.4-bounded changes needed to close them.
- Re-run only the evidence/verification commands affected by the fix.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 6.1.4 implementation if it lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 6.1.4 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 6.1.4 next-boundary naming clearly and standalone.

Stop conditions:
- Stop once Packet 6.1.4 is review-clean, committed, and the next planning boundary is named without starting it.
- Stop and report blocked if the packet cannot name the next boundary because the Packet 6.1.3 verdict is still ambiguous or because a real earlier-seam blocker surfaced.
```
