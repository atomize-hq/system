# Handbook Engine Extraction Phase 2 Slice 3 Packet Prompts

Task source: [handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md](./handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md)
Spec source: [handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md](./handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md)
Plan source: [handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md](./handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, fix, and packet advancement, and requires GitNexus impact analysis before production-symbol edits plus GitNexus detect-changes before each commit.

Do not start Packet 2.3.2 until Packet 2.3.1 is review-clean and committed.

## Packet 2.3.1 Prompt

```text
/goal Orchestrate Phase 2 Slice 3 Packet 2.3.1: Typed Resolver Contract And Shipped-Default Posture in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.3.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md.
- Verify live repo truth before changing anything, including that Slice 2.2 is already landed and that Slice 2.3 is the template/library-only seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md
- Use the Slice 2.2 authority docs as dependency inputs:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md
- Do not start Packet 2.3.2.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- Spawn a fresh GPT-5.4 subagent on high for review after implementation completes.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.3.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Stay inside Packet 2.3.1 scope.

Packet 2.3.1 scope:
- Define the typed template/library resolver contract for the approved authoring asset families.
- Migrate charter authoring onto the resolver-backed shipped defaults.
- Migrate environment-inventory authoring onto the same resolver-backed shipped defaults.
- Expected files:
  - crates/compiler/src/template_library.rs
  - crates/compiler/src/author/mod.rs
  - crates/compiler/src/author/charter.rs
  - crates/compiler/src/author/environment_inventory.rs
  - crates/compiler/tests/author.rs
  - crates/compiler/tests/pipeline_catalog.rs
  - crates/cli/tests/author_cli.rs
  - optionally crates/compiler/src/pipeline.rs if live repo truth requires observer wiring only

Out of scope:
- validated override and alternate-selection rules from Packet 2.3.2
- setup or canonical-artifact semantic changes except for tiny compatibility plumbing that is strictly required to preserve shipped-default behavior
- new public CLI flags, new repo-level config files, or unbounded user-provided path selection
- new library content, new templates, or prompt/template rewrites
- reopening Slice 2.2 supported-target adoption or starting Phase 3 shell-wording cleanup
- changing stage front matter or demoting `core/stages/**` / `core/library/**` from authoritative repo-owned truth

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 3 Packet 2.3.1: Typed Resolver Contract And Shipped-Default Posture`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Slice 2.2 already landed and that Packet 2.3.1 is the current boundary before editing.
- Require the subagent to keep this packet bounded to shipped-default resolver introduction plus adopter migration and to explicitly preserve:
  - zero-config shipped-default behavior for charter and environment-inventory authoring
  - declarative stage/library truth in `core/stages/**` and `core/library/**` remaining authoritative
  - charter-specific validation, heading guarantees, and guided-authoring wording staying local in `author/charter.rs`
  - environment-inventory-specific validation, optional project-context posture, and canonical-path checks staying local in `author/environment_inventory.rs`
  - Packet 2.3.2 override behavior staying deferred
- Require the smallest viable typed resolver boundary that can own shipped-default asset selection without redesigning stage compilation or broadening into setup/canonical-artifact refactors.
- Require `author/charter.rs` and `author/environment_inventory.rs` to stop owning raw shipped asset selection locally if the new resolver can provide the same contract.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test author`
  - `cargo test -p handbook-compiler --test pipeline_catalog`
  - `cargo test -p handbook-cli --test author_cli`
  - `rg -n "include_str!\\(|template_library|core/library/.+tmpl|core/library/.+directive" crates/compiler/src/author crates/compiler/src/template_library.rs crates/compiler/src/pipeline.rs`
- If Packet 2.3.1 ends up touching setup/canonical-artifact compatibility code, also require:
  - `cargo test -p handbook-compiler --test setup`
- Require the subagent to stop after Packet 2.3.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 3 Packet 2.3.1: Typed Resolver Contract And Shipped-Default Posture`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.3 spec, plan, tasks, the Slice 2.2 authority set, and the verification evidence.
- Require special attention to:
  - whether one typed shipped-default resolver owner now exists instead of duplicated local selection ownership in authoring modules
  - whether zero-config behavior stayed stable for charter and environment-inventory authoring
  - whether declarative stage/library truth stayed authoritative instead of becoming a second-class observer
  - whether Packet 2.3.2 override work leaked in
  - whether any new public configuration or path-selection surface leaked in
- Require severity labels and explicit callouts if shipped-default ownership remains duplicated, if shipped-default behavior drifted, or if scope widened beyond Packet 2.3.1.

Fix loop:
- If the review is clean, stop and report Packet 2.3.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-2.3.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 2.3.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.3.1 only.
- Commit after each accepted fix round.
- Do not advance to Packet 2.3.2 until Packet 2.3.1 is review-clean and committed.
- Commit messages must describe the Packet 2.3.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.3.1 is review-clean and committed.
- Stop and report blocked if Packet 2.3.1 cannot be completed without widening into Packet 2.3.2, adding new public config/CLI surface, changing the approved Slice 2.3 spec/plan/tasks, or reopening Slice 2.2 runtime-target work.
```

## Packet 2.3.2 Prompt

```text
/goal Orchestrate Phase 2 Slice 3 Packet 2.3.2: Validated Override And Selection Rules in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.3.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md.
- Assume Packet 2.3.1 is already landed, review-clean, and committed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md
- Use Packet 2.3.1’s landed resolver contract as the immediate dependency surface.
- Do not reopen Packet 2.3.1 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 2.3.2 to land correctly.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- Spawn a fresh GPT-5.4 subagent on high for review after implementation completes.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 2.3.2 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 2.3.2 scope.

Packet 2.3.2 scope:
- Add validated override and alternate-selection rules for approved asset families.
- Preserve shipped-default and starter-template compatibility after override support lands.
- Expected files:
  - crates/compiler/src/template_library.rs
  - crates/compiler/src/author/charter.rs
  - crates/compiler/src/author/environment_inventory.rs
  - crates/compiler/tests/author.rs
  - crates/compiler/tests/pipeline_catalog.rs
  - crates/compiler/src/canonical_artifacts.rs
  - crates/compiler/src/setup.rs
  - crates/compiler/tests/setup.rs
  - crates/cli/tests/author_cli.rs

Out of scope:
- new public CLI flags, repo-level manifests, or unbounded user-editable config files for template selection
- new library content, renamed library files, or stage front-matter rewrites
- broad setup/canonical-artifact redesign beyond minimal compatibility work
- reopening Packet 2.3.1 except for tiny corrective edits strictly required by live repo truth
- reopening Slice 2.2 or starting Phase 3 shell cleanup
- arbitrary absolute paths, traversal, or any out-of-root file-selection mechanism

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 2 Slice 3 Packet 2.3.2: Validated Override And Selection Rules`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 2.3.1 already landed and that the shipped-default resolver contract exists before editing.
- Require the subagent to keep this packet bounded to override validation and compatibility guardrails and to explicitly preserve:
  - zero-config shipped-default behavior when no override is supplied
  - approved asset-family boundaries for charter and environment-inventory only
  - refusal posture for absolute paths, traversal, out-of-root selections, missing files, and asset-kind mismatches
  - starter-template and setup/status semantics remaining stable unless a tiny compatibility adjustment is strictly required
  - declarative stage/library truth remaining authoritative and unchanged
- Require override handling to stay typed and bounded rather than becoming a generic file-path escape hatch.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo test -p handbook-compiler --test author`
  - `cargo test -p handbook-compiler --test pipeline_catalog`
  - `cargo test -p handbook-compiler --test setup`
  - `cargo test -p handbook-cli --test author_cli`
  - `rg -n "include_str!\\(|template_library|core/library/.+tmpl|core/library/.+directive|setup_starter_template" crates/compiler/src/author crates/compiler/src/template_library.rs crates/compiler/src/canonical_artifacts.rs crates/compiler/src/setup.rs`
- Require the subagent to stop after Packet 2.3.2 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 2 Slice 3 Packet 2.3.2: Validated Override And Selection Rules`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 2.3 spec, plan, tasks, the landed Packet 2.3.1 contract, and the verification evidence.
- Require special attention to:
  - whether override support is typed and bounded instead of acting as arbitrary path selection
  - whether zero-config shipped-default behavior remained stable
  - whether starter-template or setup/status semantics regressed
  - whether authoring modules reintroduced competing asset-selection ownership
  - whether any public config/CLI surface, new library content, or stage-truth drift leaked in
- Require severity labels and explicit callouts if override safety is incomplete, if compatibility regressed, or if scope widened beyond Packet 2.3.2.

Fix loop:
- If review is clean, stop and report Packet 2.3.2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-2.3.2-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 2.3.2 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.3.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 2.3.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 2.3.2 is review-clean and committed.
- Stop and report blocked if Packet 2.3.2 requires widening beyond approved asset families, introducing new public configuration surfaces, changing the approved Slice 2.3 spec/plan/tasks, or performing broad setup/canonical-artifact redesign.
```
