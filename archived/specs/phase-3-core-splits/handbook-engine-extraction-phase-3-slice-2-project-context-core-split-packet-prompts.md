# Handbook Engine Extraction Phase 3 Slice 2 Packet Prompts

Task source: [handbook-engine-extraction-phase-3-slice-2-project-context-core-split-tasks.md](./handbook-engine-extraction-phase-3-slice-2-project-context-core-split-tasks.md)
Spec source: [handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md](./handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md)
Plan source: [handbook-engine-extraction-phase-3-slice-2-project-context-core-split-plan.md](./handbook-engine-extraction-phase-3-slice-2-project-context-core-split-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 3.2 scope.

## Packet 3.2.1 Prompt

```text
/goal Orchestrate Phase 3 Slice 2 Packet 3.2.1: Project-Context Deterministic Model Split in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.2.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-tasks.md.
- Verify live repo truth before changing anything, including that Slice 3.1 already established the in-place deterministic-core split pattern and that `crates/compiler/src/author/project_context.rs` still mixes deterministic project-context logic with shell/runtime behavior.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-plan.md
- Stay inside Packet 3.2.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.2.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 3.2.1 scope.

Packet 3.2.1 scope:
- Introduce one deterministic project-context-core owner for structured-input types, normalization, and validation.
- Move deterministic project-context markdown rendering and markdown validation behind that same core boundary.
- Keep the public project-context parse/render/validate surface stable through the current compiler exports.
- Keep render timestamp resolution, canonical authoring preflight, lock/write orchestration, guided CLI interviewing, and shell-oriented refusal wording out of Packet 3.2.1 unless a tiny compile-through adjustment is strictly required.
- Expected files:
  - crates/compiler/src/author/project_context.rs
  - crates/compiler/src/author/project_context_core.rs
  - crates/compiler/src/author/mod.rs
  - optionally crates/compiler/src/lib.rs if narrow compile-through wiring is required
  - crates/compiler/tests/author.rs

Out of scope:
- Packet 3.2.2 shell/runtime cleanup, including extracting timestamp resolution and authoring mutation flow into `project_context_shell.rs`
- changes to `environment_inventory`, `setup`, `doctor`, `refusal`, or CLI thinning
- changes to `crates/cli/src/main.rs` guided project-context interview behavior except for tiny compile-through fallout if live repo truth proves it is unavoidable
- changes to project-context markdown semantics, required headings, metadata-line semantics, refusal policy, or CLI-visible wording
- new public CLI flags, new authoring configuration, or canonical-write-contract redesign
- Phase 4 crate moves or any new crate boundary work
- broad helper unification across authoring surfaces

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 3 Slice 2 Packet 3.2.1: Project-Context Deterministic Model Split`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Slice 3.1 already landed the deterministic-core split pattern and that `crates/compiler/src/author/project_context.rs` still owns mixed deterministic and shell/runtime behavior before editing.
- Require one deterministic project-context-core owner for:
  - `ProjectContextStructuredInput` and related structured-input types
  - normalization helpers such as `normalized_project_context_structured_input` and `normalize_project_context_text`
  - validation helpers such as `parse_project_context_structured_input_yaml`, `validate_project_context_structured_input`, `require_factual_*`, `collect_render_safety_issues`, and `validate_known_fake_project_context_markers`
  - compiler-owned markdown rendering and markdown validation
  - heading-order and placeholder/boilerplate validation helpers needed by the deterministic render path
- Require the deterministic core to stay free of:
  - env-var reads such as `AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR`
  - current-time resolution or `OffsetDateTime::now_utc()` ownership
  - lock handling, canonical-root inspection, or repo mutation
  - canonical write validation and write execution
  - CLI-guided interview logic or operator-facing refusal wording
- Require the implementation to preserve the current public project-context parse/render/validate API through `crates/compiler/src/author/mod.rs` and any narrow compile-through wiring that live code truth proves necessary.
- Require timestamp resolution, preflight, lock/write orchestration, and shell-oriented refusal wording to remain behavior-stable and packet-deferred rather than being cleaned up broadly in Packet 3.2.1.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "parse_project_context_structured_input_yaml|validate_project_context_structured_input|render_project_context_markdown|validate_project_context_markdown|normalized_project_context_structured_input|collect_render_safety_issues|validate_known_fake_project_context_markers" crates/compiler/src/author/project_context*.rs crates/compiler/src/author/mod.rs`
  - `rg -n "AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR|resolve_project_context_now_utc|OffsetDateTime::now_utc|next_safe_action|write_repo_relative_bytes|acquire_authoring_lock|CanonicalArtifacts::load" crates/compiler/src/author/project_context_core.rs crates/compiler/src/author/project_context.rs`
  - `cargo test -p handbook-compiler --test author`
  - `cargo check -p handbook-compiler`
- Require the subagent to stop after Packet 3.2.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 3 Slice 2 Packet 3.2.1: Project-Context Deterministic Model Split`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 3.2 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether all deterministic project-context modeling, normalization, validation, rendering, and markdown validation now live behind one clear core boundary
  - whether env/clock/lock/write/refusal dependencies remained outside the deterministic core
  - whether the implementation preserved the public parse/render/validate compiler surface
  - whether Packet 3.2.2 shell/runtime cleanup leaked into Packet 3.2.1
  - whether CLI-guided interview ownership stayed in CLI rather than drifting into compiler internals
  - whether project-context markdown validation or required-heading semantics drifted while moving deterministic logic
- Require severity labels and explicit callouts if shell/runtime dependencies remain in the core, if Packet 3.2.2 work leaked in, or if public project-context behavior drifted.

Fix loop:
- If the review is clean, stop and report Packet 3.2.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.2.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.2.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.2.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 3.2.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 3.2.1 is review-clean and committed.
- Stop and report blocked if Packet 3.2.1 cannot be completed without widening into Packet 3.2.2 shell/runtime cleanup, changing the approved Slice 3.2 spec/plan/tasks, or spilling into environment-inventory, setup, doctor, refusal, CLI thinning, or Phase 4 crate work.
```

## Packet 3.2.2 Prompt

```text
/goal Orchestrate Phase 3 Slice 2 Packet 3.2.2: Project-Context Recovery Wording And Shell Cleanup in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.2.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-tasks.md.
- Verify live repo truth before changing anything, including that Packet 3.2.1 already landed the deterministic project-context-core split and that Packet 3.2.2 is now the remaining approved shell/runtime cleanup seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-plan.md
- Stay inside Packet 3.2.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.2.2 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 3.2.2 scope.

Packet 3.2.2 scope:
- Extract project-context render timestamp resolution and authoring mutation flow into shell-owned helpers.
- Thin project-context preflight and authoring entrypoints around the deterministic core and shell boundaries.
- Keep the deterministic project-context core from Packet 3.2.1 intact while moving shell/runtime ownership out of the monolithic facade.
- Preserve existing public compiler and CLI project-context behavior, including guided interview ownership, canonical write targets, lock behavior, refusal semantics, timestamp format, and CLI-visible success/refusal flows.
- Expected files:
  - crates/compiler/src/author/project_context.rs
  - crates/compiler/src/author/project_context_shell.rs
  - crates/compiler/src/author/mod.rs
  - optionally crates/compiler/src/lib.rs if narrow compile-through wiring is required
  - crates/compiler/tests/author.rs
  - crates/cli/tests/author_cli.rs

Out of scope:
- reopening Packet 3.2.1 deterministic-core ownership except for tiny compile-through adjustments required by the shell split
- changes to `environment_inventory`, `setup`, `doctor`, `refusal`, or CLI thinning
- pulling guided interview collection from `crates/cli/src/main.rs` into compiler internals
- changes to project-context markdown semantics, required headings, metadata-line policy, or structured-input schema
- new public CLI flags, new authoring configuration, or broad authoring-surface helper unification
- Phase 4 crate moves or any new crate-boundary work
- broad cleanup beyond the minimal shell/runtime extraction needed for Packet 3.2.2

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 3 Slice 2 Packet 3.2.2: Project-Context Recovery Wording And Shell Cleanup`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 3.2.1 already landed and that the remaining mixed ownership in `crates/compiler/src/author/project_context.rs` is now render timestamp resolution, authoring preflight, lock/write flow, and shell-oriented refusal wording.
- Require one shell/runtime owner, preferably `crates/compiler/src/author/project_context_shell.rs`, for:
  - render timestamp resolution, including env override handling and fallback current-time derivation
  - canonical `.handbook` root inspection and baseline eligibility checks
  - authoring lock acquisition and canonical-write-target validation
  - canonical project-context write execution and mutation-refusal construction
  - shell-oriented refusal helpers and next-safe-action wording that reference `handbook author project-context`
- Require `preflight_author_project_context`, `author_project_context`, and `author_project_context_from_input` to become thin orchestrators over the deterministic core and shell/runtime helpers without changing the public compiler surface.
- Require the deterministic project-context core from Packet 3.2.1 to remain free of env reads, current-time resolution, lock handling, canonical writes, and CLI-guided interview logic.
- Require preservation of:
  - current canonical write targets and lock semantics
  - current refusal classifications and CLI-facing next-safe-action posture
  - current timestamp format and rendered metadata lines
  - current guided-authoring CLI behavior and `--from-inputs` behavior
  - current compiler and CLI author regression behavior unless live repo truth proves a tiny compile-through adjustment is strictly required
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR|resolve_project_context_now_utc|OffsetDateTime::now_utc|next_safe_action|handbook author project-context|acquire_authoring_lock|write_repo_relative_bytes|validate_canonical_write_target|CanonicalArtifacts::load" crates/compiler/src/author/project_context*.rs crates/compiler/src/author/mod.rs`
  - `cargo test -p handbook-compiler --test author`
  - `cargo test -p handbook-cli --test author_cli`
  - `cargo check -p handbook-compiler`
  - `cargo check -p handbook-cli`
- Require the subagent to stop after Packet 3.2.2 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 3 Slice 2 Packet 3.2.2: Project-Context Recovery Wording And Shell Cleanup`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 3.2 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether render timestamp resolution and authoring mutation flow now live behind one shell/runtime owner
  - whether preflight and authoring entrypoints became thin facades instead of remaining monolithic mixed owners
  - whether Packet 3.2.1 deterministic-core boundaries remained intact
  - whether guided CLI interviewing stayed in CLI rather than leaking into compiler internals
  - whether CLI-visible project-context behavior, refusal posture, timestamp behavior, lock semantics, and canonical write behavior regressed
  - whether any `environment_inventory`, `setup`, `doctor`, `refusal`, or CLI-thinning work leaked in
- Require severity labels and explicit callouts if shell/runtime ownership is still duplicated, if deterministic-core purity regressed, or if public project-context behavior drifted.

Fix loop:
- If the review is clean, stop and report Packet 3.2.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.2.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.2.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.2.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 3.2.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 3.2.2 is review-clean and committed.
- Stop and report blocked if Packet 3.2.2 cannot be completed without reopening Packet 3.2.1 beyond tiny compile-through adjustments, changing the approved Slice 3.2 spec/plan/tasks, or spilling into environment-inventory, setup, doctor, refusal, CLI thinning, or Phase 4 crate work.
```
