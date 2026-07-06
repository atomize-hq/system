# Handbook Engine Extraction Phase 4 Slice 5 Packet Prompts

Task source: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md)
Spec source: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md)
Plan source: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and requires GitNexus impact analysis before production-symbol edits plus GitNexus detect-changes before each commit.

Do not advance to the next packet until the current packet is review-clean and committed.

## Packet 4.5.1 Prompt

```text
/goal Orchestrate Phase 4 Slice 5 Packet 4.5.1: Residual Caller Inventory And Boundary Freeze in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.5.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md.
- Verify live repo truth before changing anything, including that Set 3 / Slice 4.5 is now a closeout refresh rather than a fresh compiler-retirement slice.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not start Packet 4.5.2, 4.5.3, or 4.5.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation changes before review.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before re-reviewing.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.5.1 only.
- Stay inside Packet 4.5.1 scope.

Packet 4.5.1 scope:
- Inventory every remaining `handbook_compiler::*` caller in CLI-adjacent code and classify it by ownership.
- Preserve the already-landed owner-rooted surfaces while freezing the residual rewire list.
- Expected files:
  - crates/cli/src/main.rs
  - crates/cli/src/author.rs
  - crates/cli/src/setup.rs
  - crates/cli/src/doctor.rs
  - crates/cli/src/rendering.rs
  - crates/flow/src/lib.rs
  - crates/cli/tests/author_cli.rs
  - crates/cli/tests/cli_surface.rs
  - optionally adjacent doc notes only if strictly required to record the inventory truth

Out of scope:
- landing the actual stale caller rewires from Packet 4.5.2
- compiler-root export cleanup or ownership-doc rewrites from Packet 4.5.3
- final verification wall or Set 4 deferral ledger work from Packet 4.5.4
- broad CLI cleanup, shell refactors, copy rewrites, or compiler retirement work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 5 Packet 4.5.1: Residual Caller Inventory And Boundary Freeze`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that the refreshed Slice 4.5 spec/plan/tasks already treat `handbook-compiler` as an already-narrowed seam that still needs caller/dependency honesty closeout.
- Require the subagent to keep this packet bounded to inventory/classification work and to explicitly preserve:
  - `crates/flow/src/lib.rs` staying free of compiler forwarding
  - already-owner-rooted test imports staying on direct owner crates
  - no speculative Packet 4.5.2 rewires except tiny edits strictly required to make the inventory durable
  - no Packet 4.5.3 doc-truth refresh beyond tiny notes strictly required to record the frozen inventory
- Require the subagent to classify every remaining compiler-root caller as either:
  - stale extracted-logic indirection that should move in Packet 4.5.2, or
  - legitimate retained narrow support-seam usage that remains allowed for now
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli/src crates/cli/tests crates/flow crates/compiler/src/lib.rs`
  - `cargo tree -p handbook-cli -e normal`
  - `cargo test -p handbook-flow`
  - `cargo test -p handbook-cli --test author_cli`
  - `cargo test -p handbook-cli --test cli_surface`
- Require the subagent to stop after Packet 4.5.1 acceptance is met and report touched files, impact-analysis results, the classified residual caller inventory, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 5 Packet 4.5.1: Residual Caller Inventory And Boundary Freeze`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.5 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether every remaining compiler-root caller is classified clearly and exhaustively
  - whether the packet preserved already-owner-rooted surfaces like `handbook-flow`
  - whether any actual caller rewires or compiler-boundary redesign leaked in early
  - whether the inventory is concrete enough to make Packet 4.5.2 and 4.5.3 hard to mis-scope
- Require severity labels and explicit callouts if the inventory is incomplete, if current boundaries are misstated, or if the packet widened beyond Packet 4.5.1.

Fix loop:
- If the review is clean, stop and report Packet 4.5.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.5.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after the implementation round if Packet 4.5.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.5.1 only.
- Commit after each accepted fix round that changes files, before re-review.
- Commit messages must describe the Packet 4.5.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.5.1 is review-clean and committed.
- Stop and report blocked if Packet 4.5.1 cannot be completed without prematurely performing Packet 4.5.2 rewires, Packet 4.5.3 boundary redesign, or Set 4 CLI shell work.
```

## Packet 4.5.2 Prompt

```text
/goal Orchestrate Phase 4 Slice 5 Packet 4.5.2: Stale Caller Rewires To Real Owner Crates in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.5.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md.
- Assume Packet 4.5.1 is already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not reopen Packet 4.5.1 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 4.5.2 to land correctly.
- Do not start Packet 4.5.3 or 4.5.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation changes before review.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before re-reviewing.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.5.2 only.
- Stay inside Packet 4.5.2 scope.

Packet 4.5.2 scope:
- Rewire stale extracted-logic callers directly to `handbook-engine`, `handbook-pipeline`, or `handbook-flow`.
- Keep `handbook-cli` dependency posture honest after the rewires.
- Expected files:
  - crates/cli/src/main.rs
  - crates/cli/src/author.rs
  - crates/cli/src/rendering.rs
  - crates/cli/tests/author_cli.rs
  - crates/cli/tests/cli_surface.rs
  - crates/cli/Cargo.toml
  - optionally Cargo.toml
  - optionally crates/compiler/Cargo.toml

Out of scope:
- reopening Packet 4.5.1 inventory work except tiny corrective carry-forward
- compiler-root export cleanup and ownership-doc guard work from Packet 4.5.3
- final workspace proof and Set 4 deferral ledger work from Packet 4.5.4
- broad CLI shell decomposition, setup/doctor redesign, or compiler retirement

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 5 Packet 4.5.2: Stale Caller Rewires To Real Owner Crates`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 4.5.1 already landed and that the residual caller inventory clearly distinguishes stale extracted-logic indirection from legitimate retained compiler support seams.
- Require the subagent to keep this packet bounded to stale caller rewires and to explicitly preserve:
  - `handbook-compiler` remaining available for the legitimate narrow support seams that still exist
  - `crates/flow/src/lib.rs` staying free of compiler forwarding
  - CLI behavior and test expectations remaining stable
  - no Packet 4.5.3 doc-truth cleanup except tiny edits strictly required by the rewires
  - no Phase 5 shell refactor work
- Require the implementation to move only the stale extracted-logic callers identified in Packet 4.5.1 to direct `handbook-engine`, `handbook-pipeline`, or `handbook-flow` imports.
- Require manifest edits to reflect the real owner graph after the rewire.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli/src crates/cli/tests`
  - `cargo tree -p handbook-cli -e normal`
  - `cargo test -p handbook-cli --test author_cli`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-engine`
  - `cargo test -p handbook-pipeline`
  - `cargo test -p handbook-flow`
- Require the subagent to stop after Packet 4.5.2 acceptance is met and report touched files, impact-analysis results, which stale callers were rewired, any remaining compiler-root usages with justification, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 5 Packet 4.5.2: Stale Caller Rewires To Real Owner Crates`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.5 spec, plan, tasks, the landed Packet 4.5.1 state, and the verification evidence.
- Require special attention to:
  - whether stale extracted-logic callers truly moved to the real owner crates
  - whether legitimate retained compiler support seams were preserved rather than broken or over-removed
  - whether `handbook-cli` dependency posture now matches the real owner graph
  - whether scope drifted into Packet 4.5.3 boundary redesign or Phase 5 shell cleanup
- Require severity labels and explicit callouts if stale facade imports remain, if direct-owner rewires are semantically wrong, or if scope widened beyond Packet 4.5.2.

Fix loop:
- If the review is clean, stop and report Packet 4.5.2 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-4.5.2-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 4.5.2 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.5.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 4.5.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.5.2 is review-clean and committed.
- Stop and report blocked if Packet 4.5.2 requires reopening Packet 4.5.1 substantially, broadening the retained compiler seam, or widening into Packet 4.5.3/Phase 5 work.
```

## Packet 4.5.3 Prompt

```text
/goal Orchestrate Phase 4 Slice 5 Packet 4.5.3: Compiler Narrow Boundary Truth in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.5.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md.
- Assume Packets 4.5.1 and 4.5.2 are already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not reopen Packets 4.5.1 or 4.5.2 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 4.5.3 to land correctly.
- Do not start Packet 4.5.4.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation changes before review.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before re-reviewing.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.5.3 only.
- Stay inside Packet 4.5.3 scope.

Packet 4.5.3 scope:
- Keep `handbook-compiler` limited to the reviewed narrow compatibility/support seam.
- Align repo-facing ownership docs and help guards to the retained compiler seam.
- Expected files:
  - crates/compiler/src/lib.rs
  - crates/compiler/Cargo.toml
  - README.md
  - docs/README.md
  - docs/contracts/C-02-rust-workspace-and-cli-command-surface.md
  - optionally crates/compiler/tests/author.rs
  - optionally crates/compiler/tests/doctor.rs
  - optionally crates/compiler/tests/setup.rs
  - optionally crates/cli/tests/help_drift_guard.rs

Out of scope:
- substantial caller rewire work from Packet 4.5.2 except tiny corrective adjustments strictly required to keep the retained seam coherent
- final full workspace verification wall and explicit Set 4 deferral ledger work from Packet 4.5.4
- broad CLI help/copy cleanup, shell decomposition, or compiler retirement

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 5 Packet 4.5.3: Compiler Narrow Boundary Truth`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packets 4.5.1 and 4.5.2 already landed and that the remaining `handbook-compiler` surface is now the true retained narrow seam rather than a mixed facade.
- Require the subagent to keep this packet bounded to compiler-boundary truth and to explicitly preserve:
  - no re-expansion of `handbook-compiler` into an umbrella implementation center
  - no retirement push unless live code proves it is unavoidable and the orchestrator stops for scope approval instead
  - no Phase 5 shell redesign
  - doc/help truth matching the actual retained support seam and direct-owner caller graph
- Require the implementation to make `crates/compiler/src/lib.rs` and adjacent docs/tests reviewably consistent with the retained narrow seam.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo tree -p handbook-compiler -e normal`
  - `cargo test -p handbook-compiler --test author`
  - `cargo test -p handbook-compiler --test doctor`
  - `cargo test -p handbook-compiler --test setup`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `rg -n 'narrow compatibility/support|direct owner|umbrella' README.md docs/README.md docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- Require the subagent to stop after Packet 4.5.3 acceptance is met and report touched files, impact-analysis results, the retained compiler seam posture, updated doc/help truth, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 5 Packet 4.5.3: Compiler Narrow Boundary Truth`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.5 spec, plan, tasks, the landed Packet 4.5.1/4.5.2 state, and the verification evidence.
- Require special attention to:
  - whether `handbook-compiler` remains narrow and non-umbrella in both code and docs
  - whether docs and help guards describe the same boundary the code now implements
  - whether any hidden caller rewires or CLI shell redesign leaked in
  - whether the packet understates remaining retained compiler responsibilities
- Require severity labels and explicit callouts if the compiler seam still behaves like an umbrella, if docs are stale or misleading, or if scope widened beyond Packet 4.5.3.

Fix loop:
- If the review is clean, stop and report Packet 4.5.3 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-4.5.3-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 4.5.3 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.5.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 4.5.3 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.5.3 is review-clean and committed.
- Stop and report blocked if Packet 4.5.3 requires compiler retirement, broad CLI redesign, or reopening earlier packets beyond tiny corrective changes.
```

## Packet 4.5.4 Prompt

```text
/goal Orchestrate Phase 4 Slice 5 Packet 4.5.4: Final Closeout Proof And Set 4 Deferral Ledger in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.5.4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md.
- Assume Packets 4.5.1, 4.5.2, and 4.5.3 are already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md
- Use the closeout seam map at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-closeout-four-set-map.md
- Do not reopen earlier packets except where live repo truth proves a tiny closeout fix is strictly required to make Packet 4.5.4 honestly green.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, if files changed, commit the implementation changes before review.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- After each accepted fix round that changes files, commit before re-reviewing.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.5.4 only.
- If the packet lands with no repo changes, do not create an empty commit; report that the packet was a verification-only closeout.
- Stay inside Packet 4.5.4 scope.

Packet 4.5.4 scope:
- Run the full Slice 4.5 refresh verification wall.
- Preserve explicit deferrals for remaining Set 4 CLI shell closeout work.
- Expected files:
  - verification only by default
  - optionally minimal follow-up doc notes if a tiny explicit Set 4 deferral clarification is strictly required

Out of scope:
- new structural caller rewires that belong to Packet 4.5.2
- new compiler-boundary redesign that belongs to Packet 4.5.3
- broad CLI shell cleanup that belongs to Set 4
- any speculative follow-on work beyond making the Slice 4.5 closeout verdict honest

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 5 Packet 4.5.4: Final Closeout Proof And Set 4 Deferral Ledger`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packets 4.5.1 through 4.5.3 already landed and that Slice 4.5 is now at the final proof stage.
- Require the subagent to keep this packet bounded to final verification and explicit deferral truth and to explicitly preserve:
  - no reopening earlier packets except for tiny fixes strictly required to make the verification wall honestly green
  - no hidden Set 4 CLI shell cleanup under the guise of “just fixing the tests”
  - no compiler-retirement scope change
- Require targeted verification with:
  - `rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli/src crates/cli/tests crates/flow crates/compiler/src/lib.rs`
  - `cargo tree -p handbook-cli -e normal`
  - `cargo tree -p handbook-compiler -e normal`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
- Require the subagent to report whether Packet 4.5.4 was verification-only or required tiny corrective edits, and if edits were required, to list them and justify why they were the smallest honest closeout changes.
- Require the subagent to stop after Packet 4.5.4 acceptance is met and report verification evidence, any final touched files, impact-analysis results for any production-symbol edits, the explicit Set 4 deferral ledger, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 5 Packet 4.5.4: Final Closeout Proof And Set 4 Deferral Ledger`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.5 spec, plan, tasks, the landed Packet 4.5.1/4.5.2/4.5.3 state, and the final verification evidence.
- Require special attention to:
  - whether the final closeout verdict is honest about residual compiler-root usage and retained narrow seam responsibilities
  - whether the workspace wall actually proves the slice is green
  - whether any “tiny closeout fix” quietly widened into earlier-packet work or Set 4 shell work
  - whether the Set 4 deferrals are explicit enough that the next agent will not miss them
- Require severity labels and explicit callouts if the final proof is incomplete, if verification evidence is weak, or if deferrals are not explicit enough.

Fix loop:
- If the review is clean, stop and report Packet 4.5.4 complete.
- If the review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-4.5.4-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- If Packet 4.5.4 required code or doc changes, commit once after the implementation round lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.5.4 only.
- Commit after each accepted fix round that changes files.
- If Packet 4.5.4 is verification-only, do not create an empty commit.
- Commit messages must describe the Packet 4.5.4 closeout change clearly and standalone when commits are needed.

Stop conditions:
- Stop once Packet 4.5.4 is review-clean and either committed or explicitly reported as a no-change verification-only closeout.
- Stop and report blocked if Packet 4.5.4 cannot be made honestly green without substantial reopening of earlier packets or widening into Set 4 CLI shell work.
```
