# Candidate 2 Packet Prompts

Task source: [candidate-2-trusted-pipeline-session-tasks.md](./candidate-2-trusted-pipeline-session-tasks.md)
Spec source: [candidate-2-trusted-pipeline-session-spec.md](./candidate-2-trusted-pipeline-session-spec.md)
Plan source: [candidate-2-trusted-pipeline-session-plan.md](./candidate-2-trusted-pipeline-session-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, and preserves commit boundaries between implementation, review, and fix work.

## Packet 1 Prompt

```text
/goal Orchestrate Candidate 2 Packet 1: Trusted Pipeline Session Core in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-tasks.md.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-plan.md
- Do not start Packet 2+.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Commit changes after the implementation lands, and commit again after each fix round that changes files.
- Stay inside Packet 1 scope.

Packet 1 scope:
- Add the trusted pipeline session core in the compiler.
- Add packet-local trust-seam regression coverage.
- Expected files:
  - crates/compiler/src/route_state.rs
  - crates/compiler/tests/pipeline_state_store.rs
  - crates/compiler/src/lib.rs

Out of scope:
- compile migration
- capture migration
- handoff migration
- public CLI/help changes unless required by Packet 1 tests

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 2 Packet 1: Trusted Pipeline Session Core`.
- Tell the subagent to use $incremental-implementation.
- Require the smallest viable deep seam that centralizes route-state load, canonical route-basis rebuild, freshness comparison, stage activity checks, and compile-facing normalization.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_state_store`
- Require the subagent to stop after Packet 1 acceptance is met and report touched files, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 2 Packet 1: Trusted Pipeline Session Core`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check Packet 1 against the spec, plan, tasks, and verification evidence.
- Require severity labels and explicit callouts if Packet 2+ work leaked in.

Fix loop:
- If the review is clean, stop and report Packet 1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal changes needed to close them.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1 lands cleanly.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1 change clearly and standalone.

Stop conditions:
- Stop once Packet 1 is review-clean and committed.
- Stop and report blocked if Packet 1 cannot be completed without widening into Packet 2+ or changing the approved spec/plan/tasks.
```

## Packet 2 Prompt

```text
/goal Orchestrate Candidate 2 Packet 2: Compile Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-tasks.md.
- Assume Packet 1 is already landed; verify live repo truth before changing anything.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-plan.md

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 2 scope.

Packet 2 scope:
- Migrate compile to the trusted pipeline session seam.
- Keep compile-facing refusal posture stable after migration.
- Expected files:
  - crates/compiler/src/pipeline_compile.rs
  - crates/compiler/tests/pipeline_compile.rs
  - crates/cli/tests/cli_surface.rs

Out of scope:
- new capture behavior
- new handoff behavior
- export-surface decisions unless required to compile Packet 2

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 2 Packet 2: Compile Migration`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 1 seam exists before editing.
- Require compile to consume the shared trust seam and stop owning repeated route-state load and route-basis freshness setup inline.
- Require compile refusal classifications and next-safe-action posture to stay stable unless the current spec explicitly says otherwise.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_compile`
  - `cargo test -p handbook-cli --test cli_surface`
- Require the subagent to stop after Packet 2 acceptance is met and report touched files, verification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 2 Packet 2: Compile Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review with special attention to correctness, architecture depth, refusal drift, and test coverage.
- Require the reviewer to compare behavior against Packet 2 tasks and the compile-related contract posture already documented in this repo.

Fix loop:
- If review is clean, stop and report Packet 2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2-bounded.
- Re-run:
  - `cargo test -p handbook-compiler --test pipeline_compile`
  - `cargo test -p handbook-cli --test cli_surface`
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2 implementation lands cleanly.
- Commit after each accepted fix round.

Stop conditions:
- Stop once Packet 2 is review-clean and committed.
- Stop and report blocked if Packet 2 requires capture/handoff work or spec changes to proceed safely.
```

## Packet 3 Prompt

```text
/goal Orchestrate Candidate 2 Packet 3: Capture Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-tasks.md.
- Assume Packets 1-2 are already landed; verify live repo truth first.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-plan.md

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 3 scope.

Packet 3 scope:
- Migrate capture preview/apply to the trusted pipeline session seam.
- Preserve capture apply safety after migration.
- Expected files:
  - crates/compiler/src/pipeline_capture.rs
  - crates/compiler/tests/pipeline_capture.rs
  - crates/cli/tests/cli_surface.rs

Out of scope:
- handoff migration
- export posture decisions
- broader refactors outside capture preview/apply trust setup

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 2 Packet 3: Capture Migration`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to keep capture-specific plan identity, revision-conflict handling, rollback/write behavior, and provenance behavior owned by the capture adapter.
- Require only the common trust setup to move behind the shared seam.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_capture`
  - `cargo test -p handbook-cli --test cli_surface`
- Require the subagent to stop after Packet 3 acceptance is met and report touched files, verification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 2 Packet 3: Capture Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review with emphasis on correctness, rollback safety, revision-conflict handling, provenance integrity, and scope discipline.
- Require explicit findings if capture-specific behavior leaked into the shared seam inappropriately.

Fix loop:
- If review is clean, stop and report Packet 3 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes Packet-3-bounded.
- Re-run:
  - `cargo test -p handbook-compiler --test pipeline_capture`
  - `cargo test -p handbook-cli --test cli_surface`
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 3 implementation lands cleanly.
- Commit after each accepted fix round.

Stop conditions:
- Stop once Packet 3 is review-clean and committed.
- Stop and report blocked if Packet 3 requires handoff/export work or a spec change to proceed safely.
```

## Packet 4 Prompt

```text
/goal Orchestrate Candidate 2 Packet 4: Handoff Migration And Closeout in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-tasks.md.
- Assume Packets 1-3 are already landed; verify live repo truth first.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-2-trusted-pipeline-session-plan.md

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 4 scope.

Packet 4 scope:
- Migrate handoff validation/emission to the trusted pipeline session seam.
- Decide and document the library export posture for downstream consumers.
- Run the final regression wall for the deepening.
- Expected files:
  - crates/compiler/src/pipeline_handoff.rs
  - crates/compiler/tests/pipeline_handoff.rs
  - crates/cli/tests/pipeline_handoff_refusals.rs
  - crates/compiler/src/lib.rs
  - docs/specs/candidate-2-trusted-pipeline-session-spec.md
  - docs/specs/candidate-2-trusted-pipeline-session-plan.md

Out of scope:
- reopening Packet 1-3 design unless required to fix a review-proven bug
- widening public product scope beyond Candidate 2

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 2 Packet 4: Handoff Migration And Closeout`.
- Tell the subagent to use $incremental-implementation.
- Require handoff validation/emission to consume the shared trust seam instead of rebuilding route-basis trust independently.
- Require the subagent to make and document the export-posture decision: internal seam first or small public library surface for future `substrate` consumption.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_handoff`
  - `cargo test -p handbook-cli --test pipeline_handoff_refusals`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 4 acceptance is met and report touched files, verification, export decision, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 2 Packet 4: Handoff Migration And Closeout`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review with emphasis on correctness, architecture depth, provenance/trust safety, export posture, and regression completeness.
- Require explicit findings if the final regression story is incomplete.

Fix loop:
- If review is clean, stop and report Packet 4 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-4-bounded unless a reviewer proves an earlier packet regression must be corrected.
- Re-run the relevant targeted tests plus any required final-wall commands after fixes.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 4 implementation lands cleanly.
- Commit after each accepted fix round.
- Final state should be review-clean, committed, and fully verified.

Stop conditions:
- Stop once Packet 4 is review-clean, committed, and the final regression wall passes.
- Stop and report blocked if Packet 4 cannot complete without reopening the approved spec/plan/tasks materially.
```
