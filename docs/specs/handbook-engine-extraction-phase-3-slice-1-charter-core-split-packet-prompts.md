# Handbook Engine Extraction Phase 3 Slice 1 Packet Prompts

Task source: [handbook-engine-extraction-phase-3-slice-1-charter-core-split-tasks.md](./handbook-engine-extraction-phase-3-slice-1-charter-core-split-tasks.md)
Spec source: [handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md](./handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md)
Plan source: [handbook-engine-extraction-phase-3-slice-1-charter-core-split-plan.md](./handbook-engine-extraction-phase-3-slice-1-charter-core-split-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 3.1 scope.

## Packet 3.1.1 Prompt

```text
/goal Orchestrate Phase 3 Slice 1 Packet 3.1.1: Charter Parse Render Validate Core Extraction in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-tasks.md.
- Verify live repo truth before changing anything, including that Phase 2 Slice 3 already landed the template-library boundary and that Packet 3.1.1 is the first approved Phase 3 charter seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-plan.md
- Stay inside Packet 3.1.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.1.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Stay inside Packet 3.1.1 scope.

Packet 3.1.1 scope:
- Introduce one deterministic charter-core owner for structured-input types, normalization, and validation.
- Move deterministic charter markdown rendering and markdown validation behind that same core boundary.
- Keep the public charter parse/render/validate surface stable through the current compiler exports.
- Keep guided synthesis, Codex runtime transport, env-var overrides, temp-output handling, synthesized-markdown validation, and shell-oriented refusal wording out of Packet 3.1.1 unless a tiny compile-through adjustment is strictly required.
- Expected files:
  - crates/compiler/src/author/charter.rs
  - crates/compiler/src/author/charter_core.rs
  - crates/compiler/src/author/mod.rs
  - optionally crates/compiler/src/lib.rs if narrow compile-through wiring is required
  - crates/compiler/tests/author.rs

Out of scope:
- Packet 3.1.2 shell/runtime cleanup, including extracting guided synthesis transport into `charter_shell.rs`
- changes to `project_context`, `environment_inventory`, `setup`, `doctor`, `refusal`, or CLI thinning
- changes to charter prompt wording, template text, synthesized-output policy, or template-library selection rules
- new public CLI flags, new authoring configuration, or canonical-write-contract redesign
- Phase 4 crate moves or any new crate boundary work
- broad helper unification across authoring surfaces

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 3 Slice 1 Packet 3.1.1: Charter Parse Render Validate Core Extraction`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Slice 2.3 already landed the template-library boundary and that `crates/compiler/src/author/charter.rs` still mixes deterministic charter logic with shell/runtime behavior before editing.
- Require one deterministic charter-core owner for:
  - `CharterStructuredInput` and related structured-input types
  - normalization helpers such as `normalize_charter_free_text`
  - validation helpers such as `parse_charter_structured_input_yaml` and `validate_charter_structured_input`
  - compiler-owned markdown rendering and markdown validation
  - render-safety and heading-validation helpers needed by the deterministic render path
- Require the deterministic core to stay free of:
  - `Command::new`, `Stdio`, process spawning, or Codex transport
  - runtime env-var reads such as `HANDBOOK_AUTHOR_CHARTER_CODEX_*`
  - temp-output creation or cleanup
  - repo mutation, lock handling, or canonical write orchestration
  - direct shipped asset selection through `template_library.rs`
- Require the implementation to preserve the current public charter parse/render/validate API through `crates/compiler/src/author/mod.rs` and any narrow compile-through wiring that live code truth proves necessary.
- Require guided synthesis, preflight, lock/write orchestration, and shell-oriented refusal wording to remain behavior-stable and packet-deferred rather than being cleaned up broadly in Packet 3.1.1.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "parse_charter_structured_input_yaml|normalize_charter_free_text|validate_charter_structured_input|render_charter_markdown|validate_charter_markdown" crates/compiler/src/author/charter*.rs crates/compiler/src/author/mod.rs`
  - `rg -n "Command::new|Stdio|AUTHOR_CHARTER_CODEX_|std::env::var|temp_dir" crates/compiler/src/author/charter_core.rs crates/compiler/src/author/charter.rs`
  - `cargo test -p handbook-compiler --test author`
  - `cargo check -p handbook-compiler`
- Require the subagent to stop after Packet 3.1.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 3 Slice 1 Packet 3.1.1: Charter Parse Render Validate Core Extraction`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 3.1 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether all deterministic charter modeling, normalization, validation, rendering, and markdown validation now live behind one clear core boundary
  - whether runtime/process/env/temp-file dependencies remained outside the deterministic core
  - whether the implementation preserved the public parse/render/validate compiler surface
  - whether Packet 3.1.2 shell/runtime cleanup leaked into Packet 3.1.1
  - whether template-library ownership from Slice 2.3 stayed outside the deterministic core
  - whether authoring semantics or charter markdown validation behavior drifted while moving deterministic logic
- Require severity labels and explicit callouts if shell/runtime dependencies remain in the core, if Packet 3.1.2 work leaked in, or if public charter behavior drifted.

Fix loop:
- If the review is clean, stop and report Packet 3.1.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.1.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.1.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.1.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 3.1.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 3.1.1 is review-clean and committed.
- Stop and report blocked if Packet 3.1.1 cannot be completed without widening into Packet 3.1.2 shell/runtime cleanup, changing the approved Slice 3.1 spec/plan/tasks, or spilling into project-context, environment-inventory, setup, doctor, refusal, or Phase 4 crate work.
```

## Packet 3.1.2 Prompt

```text
/goal Orchestrate Phase 3 Slice 1 Packet 3.1.2: Charter Synthesis And Shell Adapter Cleanup in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-tasks.md.
- Verify live repo truth before changing anything, including that Packet 3.1.1 already landed the deterministic charter-core split and that Packet 3.1.2 is now the remaining approved charter shell/runtime cleanup seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-plan.md
- Stay inside Packet 3.1.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.1.2 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Stay inside Packet 3.1.2 scope.

Packet 3.1.2 scope:
- Extract guided charter synthesis prompt assembly and runtime transport into shell-owned helpers.
- Thin charter preflight and canonical-write orchestration around the deterministic core and shell boundaries.
- Keep the deterministic charter core from Packet 3.1.1 intact while moving shell/runtime ownership out of the monolithic facade.
- Preserve existing public compiler and CLI charter behavior, including canonical write targets, lock behavior, refusal semantics, and CLI-visible success/refusal flows.
- Expected files:
  - crates/compiler/src/author/charter.rs
  - crates/compiler/src/author/charter_shell.rs
  - crates/compiler/src/author/mod.rs
  - optionally crates/compiler/src/lib.rs if narrow compile-through wiring is required
  - crates/compiler/tests/author.rs
  - crates/cli/tests/author_cli.rs

Out of scope:
- reopening Packet 3.1.1 deterministic-core ownership except for tiny compile-through adjustments required by the shell split
- changes to `project_context`, `environment_inventory`, `setup`, `doctor`, `refusal`, or CLI thinning
- changes to charter template text, template-library selection rules, structured-input schema, or charter markdown semantics
- new public CLI flags, new authoring configuration, or broad authoring-surface helper unification
- Phase 4 crate moves or any new crate-boundary work
- broad cleanup beyond the minimal shell/runtime extraction needed for Packet 3.1.2

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 3 Slice 1 Packet 3.1.2: Charter Synthesis And Shell Adapter Cleanup`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 3.1.1 already landed and that the remaining mixed ownership in `crates/compiler/src/author/charter.rs` is now guided synthesis, shell/runtime transport, preflight/lock/write orchestration, and shell-oriented refusal wording.
- Require one shell/runtime owner, preferably `crates/compiler/src/author/charter_shell.rs`, for:
  - guided synthesis prompt assembly
  - shipped asset consumption through `template_library.rs`
  - `codex exec` transport, `Command::new`, `Stdio`, and env-var overrides such as `HANDBOOK_AUTHOR_CHARTER_CODEX_*`
  - temp-output creation and cleanup
  - synthesized-markdown validation
  - shell-oriented refusal helpers and next-safe-action wording
- Require `preflight_author_charter`, `preflight_author_charter_from_input`, `author_charter`, and `author_charter_guided` to become thin orchestrators over the deterministic core and shell/runtime helpers without changing the public compiler surface.
- Require the deterministic charter core from Packet 3.1.1 to remain free of process spawning, env-var reads, temp files, repo mutation, and direct template-library selection.
- Require preservation of:
  - current canonical write targets and lock semantics
  - current refusal classifications and CLI-facing next-safe-action posture
  - current guided-authoring behavior and shipped-template-library resolution semantics
  - current compiler and CLI author regression behavior unless live repo truth proves a tiny compile-through adjustment is strictly required
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "Command::new|Stdio|AUTHOR_CHARTER_CODEX_|std::env::var|temp_dir|next_safe_action|handbook author charter" crates/compiler/src/author/charter*.rs crates/compiler/src/author/mod.rs`
  - `cargo test -p handbook-compiler --test author`
  - `cargo test -p handbook-cli --test author_cli`
  - `cargo check -p handbook-compiler`
  - `cargo check -p handbook-cli`
- Require the subagent to stop after Packet 3.1.2 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 3 Slice 1 Packet 3.1.2: Charter Synthesis And Shell Adapter Cleanup`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 3.1 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether guided synthesis prompt assembly and runtime transport now live behind one shell/runtime owner
  - whether preflight and canonical-write orchestration became thin facades instead of remaining monolithic mixed owners
  - whether Packet 3.1.1 deterministic-core boundaries remained intact
  - whether template-library ownership stayed shell-side rather than leaking back into the core
  - whether CLI-visible charter behavior, refusal posture, lock semantics, and canonical write behavior regressed
  - whether any `project_context`, `environment_inventory`, `setup`, `doctor`, or CLI-thinning work leaked in
- Require severity labels and explicit callouts if shell/runtime ownership is still duplicated, if deterministic-core purity regressed, or if public charter behavior drifted.

Fix loop:
- If the review is clean, stop and report Packet 3.1.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.1.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.1.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.1.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 3.1.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 3.1.2 is review-clean and committed.
- Stop and report blocked if Packet 3.1.2 cannot be completed without reopening Packet 3.1.1 beyond tiny compile-through adjustments, changing the approved Slice 3.1 spec/plan/tasks, or spilling into project-context, environment-inventory, setup, doctor, refusal, CLI thinning, or Phase 4 crate work.
```
