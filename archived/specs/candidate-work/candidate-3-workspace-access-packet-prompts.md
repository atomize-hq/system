# Candidate 3 Packet Prompts

Task source: [candidate-3-workspace-access-tasks.md](./candidate-3-workspace-access-tasks.md)
Spec source: [candidate-3-workspace-access-spec.md](./candidate-3-workspace-access-spec.md)
Plan source: [candidate-3-workspace-access-plan.md](./candidate-3-workspace-access-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, and preserves commit boundaries between implementation, review, and fix work.

## Packet 1 Prompt

```text
/goal Orchestrate Candidate 3 Packet 1: Workspace Access Core in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-tasks.md.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-plan.md
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
- Add the deep workspace access seam in the compiler.
- Add packet-local regression coverage for workspace invariants.
- Expected files:
  - crates/compiler/src/repo_file_access.rs
  - crates/compiler/src/lib.rs
  - optionally crates/compiler/tests/*

Out of scope:
- canonical artifact migration
- pipeline migration
- route-state cleanup
- export posture decisions

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 3 Packet 1: Workspace Access Core`.
- Tell the subagent to use $incremental-implementation.
- Require live repo grounding before editing.
- Require one compiler-owned seam for typed repo-relative normalization, validation, trusted no-follow reads, and trusted writes without forcing callers to reconstruct those rules inline.
- Require the smallest viable deep seam and forbid widening into Packet 2+.
- Require targeted verification with:
  - `cargo test -p handbook-compiler repo_file_access`
- Require the subagent to stop after Packet 1 acceptance is met and report touched files, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 3 Packet 1: Workspace Access Core`.
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
/goal Orchestrate Candidate 3 Packet 2: Canonical Artifact Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-tasks.md.
- Assume Packet 1 is already landed; verify live repo truth before changing anything.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-plan.md

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 2 scope.

Packet 2 scope:
- Migrate canonical artifact ingest onto the workspace seam.
- Preserve canonical artifact ingest posture after migration.
- Expected files:
  - crates/compiler/src/canonical_artifacts.rs
  - crates/compiler/src/repo_file_access.rs
  - crates/compiler/tests/resolver_core.rs

Out of scope:
- pipeline migration
- route-state cleanup
- public CLI/help changes unless required by Packet 2 tests
- export posture decisions

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 3 Packet 2: Canonical Artifact Migration`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 1 seam exists before editing.
- Require canonical artifact ingest to consume the shared workspace seam for `.handbook/` discovery and no-follow reads instead of keeping a second read helper.
- Require missing, empty, starter-template, and ingest-error behavior to stay stable unless the current spec explicitly says otherwise.
- Require targeted verification with:
  - `cargo test -p handbook-compiler canonical_artifacts`
  - `cargo test -p handbook-compiler --test resolver_core`
- Require the subagent to stop after Packet 2 acceptance is met and report touched files, verification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 3 Packet 2: Canonical Artifact Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review with special attention to correctness, architecture depth, path/discovery semantics, and regression coverage.
- Require the reviewer to compare behavior against Packet 2 tasks and the canonical-artifact-related contract posture already documented in this repo.

Fix loop:
- If review is clean, stop and report Packet 2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2-bounded.
- Re-run:
  - `cargo test -p handbook-compiler canonical_artifacts`
  - `cargo test -p handbook-compiler --test resolver_core`
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2 implementation lands cleanly.
- Commit after each accepted fix round.

Stop conditions:
- Stop once Packet 2 is review-clean and committed.
- Stop and report blocked if Packet 2 requires pipeline/route-state/export work or spec changes to proceed safely.
```

## Packet 3 Prompt

```text
/goal Orchestrate Candidate 3 Packet 3: Pipeline Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-tasks.md.
- Assume Packets 1-2 are already landed; verify live repo truth first.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-plan.md

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 3 scope.

Packet 3 scope:
- Migrate pipeline path validation and stage-file access onto the workspace seam.
- Keep pipeline-facing behavior stable after migration.
- Expected files:
  - crates/compiler/src/pipeline.rs
  - crates/compiler/src/repo_file_access.rs
  - crates/compiler/tests/pipeline_loader.rs
  - crates/compiler/tests/pipeline_catalog.rs
  - crates/cli/tests/cli_surface.rs

Out of scope:
- route-state cleanup
- export posture decisions
- broader refactors outside pipeline path validation and stage/front-matter access

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 3 Packet 3: Pipeline Migration`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to verify live repo truth for Packets 1-2 before editing.
- Require `pipeline.rs` to stop owning duplicate repo-relative stage-file validation where the workspace seam can own it.
- Require stage and front-matter reads to consume shared trusted file access.
- Require pipeline and stage load failures to keep classifying invalid paths and invalid files correctly.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_loader`
  - `cargo test -p handbook-compiler --test pipeline_catalog`
  - `cargo test -p handbook-cli --test cli_surface`
- Require the subagent to stop after Packet 3 acceptance is met and report touched files, verification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 3 Packet 3: Pipeline Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review with emphasis on correctness, architecture depth, path validation behavior, front-matter loading, and scope discipline.
- Require explicit findings if Packet 3 changes public CLI behavior beyond what the tests/spec justify.

Fix loop:
- If review is clean, stop and report Packet 3 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes Packet-3-bounded.
- Re-run:
  - `cargo test -p handbook-compiler --test pipeline_loader`
  - `cargo test -p handbook-compiler --test pipeline_catalog`
  - `cargo test -p handbook-cli --test cli_surface`
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 3 implementation lands cleanly.
- Commit after each accepted fix round.

Stop conditions:
- Stop once Packet 3 is review-clean and committed.
- Stop and report blocked if Packet 3 requires Packet 4+ work or a spec change to proceed safely.
```

## Packet 4 Prompt

```text
/goal Orchestrate Candidate 3 Packet 4: Route-State Workspace Cleanup in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-tasks.md.
- Assume Packets 1-3 are already landed; verify live repo truth first.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-plan.md

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 4 scope.

Packet 4 scope:
- Migrate overlapping route-state filesystem access onto the workspace seam.
- Preserve runtime-state reset and inventory behavior after migration.
- Expected files:
  - crates/compiler/src/route_state.rs
  - crates/compiler/src/repo_file_access.rs
  - crates/compiler/tests/pipeline_state_store.rs
  - crates/compiler/tests/pipeline_route_resolution.rs

Out of scope:
- export posture decisions
- final closeout wall beyond Packet 4 verification
- broader route-state redesign not required by workspace seam ownership

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 3 Packet 4: Route-State Workspace Cleanup`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to verify live repo truth for Packets 1-3 before editing.
- Require only the repo-relative read and traversal rules that belong to workspace ownership to move under the shared seam.
- Require route-state-specific domain behavior to stay local.
- Require runtime-state reset planning and inventory enumeration to keep their current safety posture and error behavior.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test pipeline_state_store`
  - `cargo test -p handbook-compiler --test pipeline_route_resolution`
- Require the subagent to stop after Packet 4 acceptance is met and report touched files, verification, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 3 Packet 4: Route-State Workspace Cleanup`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review with emphasis on correctness, architecture depth, traversal safety, inventory/reset behavior, and scope discipline.
- Require explicit findings if route-state domain behavior was moved into the workspace seam unnecessarily.

Fix loop:
- If review is clean, stop and report Packet 4 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes Packet-4-bounded.
- Re-run:
  - `cargo test -p handbook-compiler --test pipeline_state_store`
  - `cargo test -p handbook-compiler --test pipeline_route_resolution`
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 4 implementation lands cleanly.
- Commit after each accepted fix round.

Stop conditions:
- Stop once Packet 4 is review-clean and committed.
- Stop and report blocked if Packet 4 requires Packet 5 work or a material spec change to proceed safely.
```

## Packet 5 Prompt

```text
/goal Orchestrate Candidate 3 Packet 5: Export Posture And Closeout in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 5 from /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-tasks.md.
- Assume Packets 1-4 are already landed; verify live repo truth first.
- Use the spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/candidate-3-workspace-access-plan.md

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 5 scope.

Packet 5 scope:
- Decide and document the library export posture for the workspace seam.
- Run the final regression wall for the deepening.
- Expected files:
  - crates/compiler/src/lib.rs
  - docs/specs/candidate-3-workspace-access-spec.md
  - docs/specs/candidate-3-workspace-access-plan.md
  - plus any minimal closeout adjustments required by final verification

Out of scope:
- reopening Packet 1-4 design unless required to fix a review-proven bug
- widening public product scope beyond Candidate 3

Implementation subagent prompt requirements:
- Begin with `/goal Land Candidate 3 Packet 5: Export Posture And Closeout`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to make and document the export-posture decision: keep the workspace seam internal with clear future-export notes, or expose a small reviewed library surface suitable for downstream consumers.
- Require the subagent to keep the decision grounded in the landed Packet 1-4 code, not hypothetical abstractions.
- Require targeted verification with:
  - `cargo check --workspace`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets --all-features -- -D warnings`
  - `cargo test --workspace`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 5 acceptance is met and report touched files, verification, export decision, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Candidate 3 Packet 5: Export Posture And Closeout`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review with emphasis on correctness, architecture depth, export posture, and regression completeness.
- Require explicit findings if the final regression story is incomplete or the documented export decision does not match the landed code.

Fix loop:
- If review is clean, stop and report Packet 5 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-5-bounded unless a reviewer proves an earlier packet regression must be corrected.
- Re-run the relevant final-wall commands after fixes.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 5 implementation lands cleanly.
- Commit after each accepted fix round.
- Final state should be review-clean, committed, and fully verified.

Stop conditions:
- Stop once Packet 5 is review-clean, committed, and the final regression wall passes.
- Stop and report blocked if Packet 5 cannot complete without materially reopening the approved spec/plan/tasks.
```
