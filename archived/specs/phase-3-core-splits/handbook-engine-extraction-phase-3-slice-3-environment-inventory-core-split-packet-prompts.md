# Handbook Engine Extraction Phase 3 Slice 3 Packet Prompts

Task source: [handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-tasks.md](./handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-tasks.md)
Spec source: [handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md](./handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md)
Plan source: [handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-plan.md](./handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 3.3 scope.

## Packet 3.3.1 Prompt

```text
/goal Orchestrate Phase 3 Slice 3 Packet 3.3.1: Environment-Inventory Deterministic Model Split in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.3.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-tasks.md.
- Verify live repo truth before changing anything, including that Slice 3.2 already established the in-place deterministic-core split pattern and that `crates/compiler/src/author/environment_inventory.rs` still mixes deterministic environment-inventory contract logic with shell/runtime behavior.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-plan.md
- Stay inside Packet 3.3.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.3.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 3.3.1 scope.

Packet 3.3.1 scope:
- Introduce one deterministic environment-inventory-core owner for markdown contract validation.
- Preserve the current public environment-inventory validation API while moving pure contract checks behind that core boundary.
- Keep the public `validate_environment_inventory_markdown` compiler surface stable through current exports.
- Keep upstream truth loading, template-library prompt construction, `codex exec` synthesis runtime, env-var overrides, temp output/process-summary handling, canonical authoring preflight, lock/write orchestration, and shell-oriented refusal wording out of Packet 3.3.1 unless a tiny compile-through adjustment is strictly required.
- Expected files:
  - crates/compiler/src/author/environment_inventory.rs
  - crates/compiler/src/author/environment_inventory_core.rs
  - crates/compiler/src/author/mod.rs
  - optionally crates/compiler/src/lib.rs if narrow compile-through wiring is required
  - crates/compiler/tests/author.rs

Out of scope:
- Packet 3.3.2 shell/runtime cleanup, including extracting prompt construction and synthesis runtime into `environment_inventory_shell.rs`
- changes to template-library selection rules, shipped directive/template content, or starter asset ownership
- changes to `project_context`, `setup`, `doctor`, `refusal`, or CLI thinning
- changes to required headings, canonical reference lines, document semantics, refusal policy, or CLI-visible wording
- new guided TTY or `--from-inputs` flows for environment-inventory authoring
- new public CLI flags, new authoring configuration, or canonical-write-contract redesign
- Phase 4 crate moves or any new crate boundary work
- broad helper unification across authoring surfaces

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 3 Slice 3 Packet 3.3.1: Environment-Inventory Deterministic Model Split`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Slice 3.2 already landed the deterministic-core split pattern and that `crates/compiler/src/author/environment_inventory.rs` still owns mixed deterministic and shell/runtime behavior before editing.
- Require one deterministic environment-inventory-core owner for:
  - markdown contract validation exposed through `validate_environment_inventory_markdown`
  - required heading ordering such as `validate_required_heading_order_result` and `REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS`
  - canonical-file assertions for `.handbook/environment_inventory/ENVIRONMENT_INVENTORY.md`
  - legacy-path rejection for repo-root `ENVIRONMENT_INVENTORY.md`, `artifacts/foundation/ENVIRONMENT_INVENTORY.md`, and similar non-canonical references
  - exact `Project Context Ref` contract validation needed to prove the optional project-context posture stays stable
  - any other pure contract/model helpers needed to keep environment-inventory validation deterministic and shell-independent
- Require the deterministic core to stay free of:
  - env-var reads such as `HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_*`
  - `Command::new`, `codex exec`, `Stdio`, or other process-spawn ownership
  - temp-output creation, cleanup, or process-summary formatting
  - repo-root inspection, canonical artifact loading, baseline gating, lock handling, or repo mutation
  - template-library prompt assembly and shell-oriented refusal wording
- Require the implementation to preserve the current public environment-inventory validation API through `crates/compiler/src/author/mod.rs` and any narrow compile-through wiring that live code truth proves necessary.
- Require upstream truth loading, prompt construction, synthesis transport, preflight, lock/write orchestration, and shell-oriented refusal wording to remain behavior-stable and packet-deferred rather than being cleaned up broadly in Packet 3.3.1.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "validate_environment_inventory_markdown|validate_required_heading_order_result|REQUIRED_ENVIRONMENT_INVENTORY_HEADINGS|Project Context Ref|legacy non-canonical path claims" crates/compiler/src/author/environment_inventory*.rs crates/compiler/src/author/mod.rs`
  - `rg -n "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_|Command::new|codex exec|Stdio|next_safe_action|acquire_environment_inventory_authoring_lock|write_repo_relative_bytes|prepare_environment_inventory_authoring_inputs|build_environment_inventory_synthesis_prompt|summarize_process_output" crates/compiler/src/author/environment_inventory_core.rs crates/compiler/src/author/environment_inventory.rs`
  - `cargo test -p handbook-compiler --test author`
  - `cargo check -p handbook-compiler`
- Require the subagent to stop after Packet 3.3.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 3 Slice 3 Packet 3.3.1: Environment-Inventory Deterministic Model Split`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 3.3 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether all deterministic environment-inventory contract logic now lives behind one clear core boundary
  - whether env/process/temp-file/repo-mutation dependencies remained outside the deterministic core
  - whether the implementation preserved the public `validate_environment_inventory_markdown` compiler surface
  - whether Packet 3.3.2 shell/runtime cleanup leaked into Packet 3.3.1
  - whether Slice 2.3 template-library ownership stayed outside the deterministic core
  - whether required headings, canonical path assertions, or `Project Context Ref` validation semantics drifted while moving deterministic logic
- Require severity labels and explicit callouts if shell/runtime dependencies remain in the core, if Packet 3.3.2 work leaked in, or if public environment-inventory behavior drifted.

Fix loop:
- If the review is clean, stop and report Packet 3.3.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.3.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.3.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.3.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 3.3.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 3.3.1 is review-clean and committed.
- Stop and report blocked if Packet 3.3.1 cannot be completed without widening into Packet 3.3.2 shell/runtime cleanup, changing the approved Slice 3.3 spec/plan/tasks, or spilling into template-library redesign, project-context, setup, doctor, refusal, CLI thinning, or Phase 4 crate work.
```

## Packet 3.3.2 Prompt

```text
/goal Orchestrate Phase 3 Slice 3 Packet 3.3.2: Environment-Inventory Prompt And Product Cleanup in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.3.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-tasks.md.
- Verify live repo truth before changing anything, including that Packet 3.3.1 already landed the deterministic environment-inventory-core split and that Packet 3.3.2 is now the remaining approved shell/runtime cleanup seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-plan.md
- Stay inside Packet 3.3.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.3.2 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 3.3.2 scope.

Packet 3.3.2 scope:
- Extract environment-inventory prompt construction and synthesis runtime into shell-owned helpers.
- Thin environment-inventory preflight and authoring entrypoints around the deterministic core and shell boundaries.
- Keep the deterministic environment-inventory core from Packet 3.3.1 intact while moving shell/runtime ownership out of the monolithic facade.
- Preserve existing public compiler and CLI environment-inventory behavior, including required charter truth, optional project-context posture, canonical write target behavior, lock semantics, refusal semantics, runtime override posture, and CLI-visible success/refusal flows.
- Expected files:
  - crates/compiler/src/author/environment_inventory.rs
  - crates/compiler/src/author/environment_inventory_shell.rs
  - crates/compiler/src/author/mod.rs
  - optionally crates/compiler/src/lib.rs if narrow compile-through wiring is required
  - crates/compiler/tests/author.rs
  - crates/cli/tests/author_cli.rs

Out of scope:
- reopening Packet 3.3.1 deterministic-core ownership except for tiny compile-through adjustments required by the shell split
- changing template-library selection rules, shipped directive/template content, or starter asset ownership
- changes to `project_context`, `setup`, `doctor`, `refusal`, or CLI thinning
- changes to required headings, canonical reference lines, document semantics, or refusal categories
- inventing guided TTY or `--from-inputs` environment-inventory flows
- new public CLI flags, new authoring configuration, or broad authoring-surface helper unification
- Phase 4 crate moves or any new crate-boundary work
- broad cleanup beyond the minimal shell/runtime extraction needed for Packet 3.3.2

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 3 Slice 3 Packet 3.3.2: Environment-Inventory Prompt And Product Cleanup`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 3.3.1 already landed and that the remaining mixed ownership in `crates/compiler/src/author/environment_inventory.rs` is now upstream truth loading, template-library prompt assembly, `codex exec` synthesis runtime, env-var override handling, temp-output/process-summary handling, canonical preflight, lock/write flow, and shell-oriented refusal wording.
- Require one shell/runtime owner, preferably `crates/compiler/src/author/environment_inventory_shell.rs`, for:
  - upstream charter/project-context truth loading and baseline gating
  - template-library prompt assembly
  - `codex exec` transport, `Command::new`, `Stdio`, and env-var overrides such as `HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_*`
  - temp-output creation/cleanup and process-summary formatting
  - canonical `.handbook` root inspection, authoring lock acquisition, write-path validation, and canonical write execution
  - shell-oriented refusal helpers and next-safe-action wording that reference `handbook author environment-inventory`
- Require `preflight_author_environment_inventory` and `author_environment_inventory` to become thin orchestrators over the deterministic core and shell/runtime helpers without changing the public compiler surface.
- Require the deterministic environment-inventory core from Packet 3.3.1 to remain free of env reads, process spawning, temp files, repo inspection, lock handling, canonical writes, and template-library prompt ownership.
- Require preservation of:
  - current required charter truth and optional project-context posture
  - current canonical write target and lock semantics
  - current refusal classifications and CLI-facing next-safe-action posture
  - current runtime override behavior for `HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_BIN` and `HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_MODEL`
  - current compiler and CLI author regression behavior unless live repo truth proves a tiny compile-through adjustment is strictly required
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "HANDBOOK_AUTHOR_ENVIRONMENT_INVENTORY_CODEX_|Command::new|codex exec|Stdio|next_safe_action|acquire_environment_inventory_authoring_lock|write_repo_relative_bytes|prepare_environment_inventory_authoring_inputs|build_environment_inventory_synthesis_prompt|summarize_process_output" crates/compiler/src/author/environment_inventory*.rs crates/compiler/src/author/mod.rs`
  - `cargo test -p handbook-compiler --test author`
  - `cargo test -p handbook-cli --test author_cli`
  - `cargo check -p handbook-compiler`
  - `cargo check -p handbook-cli`
- Require the subagent to stop after Packet 3.3.2 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 3 Slice 3 Packet 3.3.2: Environment-Inventory Prompt And Product Cleanup`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 3.3 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether upstream truth loading, prompt assembly, and synthesis runtime now live behind one shell/runtime owner
  - whether preflight and authoring entrypoints became thin facades instead of remaining monolithic mixed owners
  - whether Packet 3.3.1 deterministic-core boundaries remained intact
  - whether Slice 2.3 template-library ownership stayed shell-side rather than leaking back into the core
  - whether CLI-visible environment-inventory behavior, refusal posture, lock semantics, runtime override behavior, and canonical write behavior regressed
  - whether any `project_context`, `setup`, `doctor`, `refusal`, or CLI-thinning work leaked in
- Require severity labels and explicit callouts if shell/runtime ownership is still duplicated, if deterministic-core purity regressed, or if public environment-inventory behavior drifted.

Fix loop:
- If the review is clean, stop and report Packet 3.3.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.3.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.3.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.3.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 3.3.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 3.3.2 is review-clean and committed.
- Stop and report blocked if Packet 3.3.2 cannot be completed without reopening Packet 3.3.1 beyond tiny compile-through adjustments, changing the approved Slice 3.3 spec/plan/tasks, or spilling into template-library redesign, project-context, setup, doctor, refusal, CLI thinning, or Phase 4 crate work.
```
