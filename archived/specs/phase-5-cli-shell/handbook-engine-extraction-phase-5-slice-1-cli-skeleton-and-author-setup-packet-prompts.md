# Handbook Engine Extraction Phase 5 Slice 1 Packet Prompts

Task source: [handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-tasks.md](./handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-tasks.md)
Spec source: [handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-spec.md](./handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-spec.md)
Plan source: [handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-plan.md](./handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 5.1 scope.

## Packet 5.1.1 Prompt

```text
/goal Orchestrate Phase 5 Slice 1 Packet 5.1.1: CLI Module Skeleton And Shared Command Helper Wiring in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 5.1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-tasks.md.
- Verify live repo truth before changing anything, including that `crates/cli/src/main.rs` is still the large integration bucket and that Slice 5.1 is limited to the first CLI shell split.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-plan.md
- Stay inside Packet 5.1.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 5.1.1 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 5.1.1 scope.

Packet 5.1.1 scope:
- Establish the first CLI module skeleton under `crates/cli/src/`.
- Extract only the shared shell helpers that setup and author both need immediately.
- Keep top-level clap registration and later families easy to find in `main.rs`.
- Expected files:
  - crates/cli/src/main.rs
  - new crates/cli/src/*.rs helper modules for the chosen skeleton

Out of scope:
- moving the full `setup` command family out of `main.rs`
- moving the full `author` command family out of `main.rs`
- pipeline, inspect, or doctor extraction
- broad prompting/rendering/exit-code cleanup
- command-surface wording changes or help-order changes
- new runtime features or crate-boundary redesign

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 5 Slice 1 Packet 5.1.1: CLI Module Skeleton And Shared Command Helper Wiring`.
- Tell the subagent to use $incremental-implementation.
- Require live verification of current shell truth across:
  - `crates/cli/src/main.rs`
  - `crates/cli/Cargo.toml`
  - `crates/cli/tests/author_cli.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- Require the implementation to:
  - introduce a real CLI module skeleton under `crates/cli/src/`
  - keep `main.rs` as the obvious top-level entrypoint for clap registration and dispatch
  - extract only the minimal shared shell helpers that the future `setup` and `author` module split actually needs now
  - avoid creating a generic dumping-ground helper module or speculative future-facing abstraction stack
  - keep `pipeline`, `inspect`, and `doctor` inline for now
- Require the implementation to avoid moving full setup/author implementations in this packet unless tiny structural fallout is strictly required to make the skeleton compile cleanly.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-cli`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `wc -l crates/cli/src/main.rs`
  - `rg -n '^fn (setup|author|execute_author_|render_setup_)' crates/cli/src/main.rs`
- Require the subagent to stop after Packet 5.1.1 acceptance is met and report touched files, impact-analysis results, module layout chosen, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 5 Slice 1 Packet 5.1.1: CLI Module Skeleton And Shared Command Helper Wiring`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 5.1 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether the new module skeleton makes `main.rs` a clearer shell boundary without obscuring the command surface
  - whether extracted shared helpers are truly minimal and justified by the setup/author split
  - whether later families stayed untouched
  - whether help posture and top-level CLI behavior stayed stable
  - whether the packet avoided drifting into full setup/author extraction or broader Phase 5 cleanup
- Require severity labels and explicit callouts if the packet introduces speculative abstractions, weakens discoverability of the CLI surface, or leaks later-slice work into Packet 5.1.1.

Fix loop:
- If the review is clean, stop and report Packet 5.1.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-5.1.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 5.1.1 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 5.1.1 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 5.1.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 5.1.1 is review-clean and committed.
- Stop and report blocked if Packet 5.1.1 cannot be completed without moving full setup/author command-family bodies, altering command/help behavior, or widening into later Phase 5 family extraction work.
```

## Packet 5.1.2 Prompt

```text
/goal Orchestrate Phase 5 Slice 1 Packet 5.1.2: Author And Setup Command-Family Extraction in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 5.1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-tasks.md.
- Assume Packet 5.1.1 is already landed; verify live repo truth before changing anything, including that the initial CLI skeleton exists and that this packet is limited to moving `setup` and `author` out of `main.rs` while leaving later families untouched.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-plan.md
- Stay inside Packet 5.1.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 5.1.2 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 5.1.2 scope.

Packet 5.1.2 scope:
- Move the `setup` command family into its dedicated CLI module.
- Move the `author` command family into its dedicated CLI module.
- Leave `pipeline`, `inspect`, and `doctor` untouched while completing the first shell split.
- Preserve current help posture and behavior for:
  - `handbook setup`
  - `handbook setup init`
  - `handbook setup refresh`
  - `handbook author charter`
  - `handbook author project-context`
  - `handbook author environment-inventory`
- Expected files:
  - crates/cli/src/main.rs
  - chosen setup module file(s) under `crates/cli/src/`
  - chosen author module file(s) under `crates/cli/src/`
  - any Packet-5.1.1-introduced shared helper module(s) that need tiny follow-through changes
  - optionally crates/cli/tests/help_drift_guard.rs only if snapshot/test harness fallout makes a tiny update strictly necessary

Out of scope:
- pipeline, inspect, or doctor extraction
- broad prompting/rendering/help/exit-code cleanup beyond what the setup/author move strictly requires
- changing supported verb names, help ordering, or user-visible wording
- reopening Phase 4 crate-boundary decisions
- new runtime features, new dependencies, or speculative cross-family abstractions

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 5 Slice 1 Packet 5.1.2: Author And Setup Command-Family Extraction`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 5.1.1 already landed and that the current module skeleton is ready to receive the `setup` and `author` command families.
- Require live verification across:
  - `crates/cli/src/main.rs`
  - setup/author module files created or referenced by Packet 5.1.1
  - `crates/cli/tests/author_cli.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- Require the implementation to:
  - move `setup`, `setup init`, and `setup refresh` routing plus setup success/refusal rendering out of inline `main.rs` bodies
  - move `author charter`, `author project-context`, and `author environment-inventory` shell orchestration out of inline `main.rs` bodies
  - keep guided and deterministic author behavior stable
  - keep help posture stable
  - leave `pipeline`, `inspect`, and `doctor` in `main.rs`
  - keep shared helper extraction minimal and avoid broad module churn outside the packet boundary
- Require the implementation to avoid turning this packet into the broader Phase 5.3 prompting/rendering/help closeout.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-cli --test author_cli`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `cargo test -p handbook-cli`
  - `rg -n '^fn (pipeline|inspect|doctor)' crates/cli/src/main.rs`
  - `rg -n '^fn (setup|author|execute_author_|render_setup_)' crates/cli/src/main.rs`
- Require the subagent to stop after Packet 5.1.2 acceptance is met and report touched files, impact-analysis results, remaining inline command-family inventory, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 5 Slice 1 Packet 5.1.2: Author And Setup Command-Family Extraction`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 5.1 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether `setup` and `author` really no longer live inline in `main.rs`
  - whether guided and deterministic author flows stayed behavior-stable
  - whether help output and command ordering remained unchanged
  - whether shared helpers stayed narrow and comprehensible
  - whether `pipeline`, `inspect`, and `doctor` remained out of scope
  - whether the packet avoided drifting into Phase 5.3 cleanup or broader CLI redesign
- Require severity labels and explicit callouts if the packet leaves large family-local logic stranded in `main.rs`, changes user-visible behavior, or introduces speculative abstractions that make later family extraction harder.

Fix loop:
- If the review is clean, stop and report Packet 5.1.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-5.1.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 5.1.2 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 5.1.2 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 5.1.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 5.1.2 is review-clean and committed.
- Stop and report blocked if Packet 5.1.2 cannot be completed without changing supported help/command behavior, moving later command families, or widening into the broader prompting/rendering/help closeout reserved for later Slice 5.x work.
```
