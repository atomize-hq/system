# Handbook Engine Extraction Phase 4 Slice 2 Packet Prompts

Task source: [handbook-engine-extraction-phase-4-slice-2-engine-migration-tasks.md](./handbook-engine-extraction-phase-4-slice-2-engine-migration-tasks.md)
Spec source: [handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md](./handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md)
Plan source: [handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md](./handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 4.2 scope.

## Packet 4.2.1 Prompt

```text
/goal Orchestrate Phase 4 Slice 2 Packet 4.2.1: Canonical Artifact Manifest Freshness And Baseline Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.2.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-tasks.md.
- Verify live repo truth before changing anything, including that Slice 4.1 already created `crates/engine`, `crates/pipeline`, and `crates/flow`, and that `handbook-engine` is still only a scaffold wrapper over `handbook-compiler`.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md
- Stay inside Packet 4.2.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.2.1 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.2.1 scope.

Packet 4.2.1 scope:
- Flip the engine/compiler dependency posture so `handbook-engine` can become the real implementation owner for the approved Slice 4.2 surfaces.
- Move the canonical artifact stack into `handbook-engine`:
  - `artifact_manifest`
  - `canonical_artifacts`
  - `freshness`
  - `baseline_validation`
- Keep any remaining compiler surfaces for those families as explicit compatibility re-exports or thin adapters only.
- Move or recreate regression coverage so the migrated canonical stack is tested from `handbook-engine` itself.
- Expected files:
  - crates/engine/Cargo.toml
  - crates/compiler/Cargo.toml
  - crates/engine/src/lib.rs
  - crates/engine/src/**
  - crates/compiler/src/lib.rs
  - crates/compiler/src/artifact_manifest.rs
  - crates/compiler/src/canonical_artifacts.rs
  - crates/compiler/src/freshness.rs
  - crates/compiler/src/baseline_validation.rs
  - crates/engine/tests/**
  - optionally crates/compiler/tests/artifact_manifest_interface.rs
  - optionally crates/compiler/tests/canonical_artifacts_ingest.rs
  - optionally crates/compiler/tests/freshness_computation.rs

Out of scope:
- moving deterministic authoring-core modules into `handbook-engine` (that is Packet 4.2.2)
- moving `setup`, `doctor`, `refusal`, `rendering`, or `template_library` into `handbook-engine`
- moving pipeline, route-state, compile/capture/handoff, setup-helper, flow, resolver, packet-result, or budget ownership
- direct `handbook-cli` rewires to `handbook-engine` as the primary runtime path
- broad public CLI wording/help changes
- Phase 4.3 pipeline migration, Phase 4.4 flow migration, or Phase 4.5 caller rewiring

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 2 Packet 4.2.1: Canonical Artifact Manifest Freshness And Baseline Migration`.
- Tell the subagent to use $incremental-implementation.
- Require live verification of current dependency and ownership truth across:
  - `crates/engine/Cargo.toml`
  - `crates/engine/src/lib.rs`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/artifact_manifest.rs`
  - `crates/compiler/src/canonical_artifacts.rs`
  - `crates/compiler/src/freshness.rs`
  - `crates/compiler/src/baseline_validation.rs`
  - `crates/compiler/tests/artifact_manifest_interface.rs`
  - `crates/compiler/tests/canonical_artifacts_ingest.rs`
  - `crates/compiler/tests/freshness_computation.rs`
- Require the implementation to:
  - eliminate the temporary `handbook-engine -> handbook-compiler` ownership posture for the canonical stack
  - keep the resulting dependency graph acyclic
  - move the canonical artifact / manifest / freshness / baseline implementation families behind `handbook-engine`
  - leave `handbook-compiler` with only narrow compatibility re-exports or thin adapters for migrated surfaces
  - add or migrate engine-owned tests so `handbook-engine` proves the moved behavior directly
- Require the implementation to keep names, schema/version constants, and behavior stable unless live repo truth proves a narrow compatibility adjustment is required.
- Require the implementation to avoid pulling author-shell, CLI wording, or adjacent engine/pipeline/flow work into this packet.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo tree -p handbook-engine -e normal`
  - `cargo tree -p handbook-compiler -e normal`
  - `cargo check --workspace`
  - `rg -n 'artifact_manifest|canonical_artifacts|freshness|baseline_validation' crates/engine crates/compiler/src`
  - `cargo test -p handbook-engine`
- Require the subagent to stop after Packet 4.2.1 acceptance is met and report touched files, impact-analysis results, dependency posture, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 2 Packet 4.2.1: Canonical Artifact Manifest Freshness And Baseline Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.2 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether the engine/compiler dependency inversion is acyclic and actually makes `handbook-engine` the real owner
  - whether `artifact_manifest`, `canonical_artifacts`, `freshness`, and `baseline_validation` now have exactly one real implementation owner
  - whether `handbook-compiler` kept only thin compatibility layers instead of duplicate implementation bodies
  - whether engine-owned tests really cover the migrated canonical stack
  - whether any author-core, setup/doctor, pipeline, flow, or CLI-rewire work leaked into Packet 4.2.1
- Require severity labels and explicit callouts if the packet leaves split-brain ownership, introduces cycles, or hides migration gaps behind oversized compiler facades.

Fix loop:
- If the review is clean, stop and report Packet 4.2.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.2.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.2.1 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.2.1 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.2.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.2.1 is review-clean and committed.
- Stop and report blocked if Packet 4.2.1 cannot be completed without widening into deterministic author-core migration, setup/doctor/rendering/refusal work, pipeline or flow migration, direct CLI rewires, or later Phase 4 slices.
```

## Packet 4.2.2 Prompt

```text
/goal Orchestrate Phase 4 Slice 2 Packet 4.2.2: Approved Authoring Core Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.2.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-tasks.md.
- Verify live repo truth before changing anything, including that Packet 4.2.1 already made `handbook-engine` the real owner of the canonical artifact / manifest / freshness / baseline stack.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-plan.md
- Stay inside Packet 4.2.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.2.2 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.2.2 scope.

Packet 4.2.2 scope:
- Move the approved deterministic authoring-core modules into `handbook-engine`:
  - `charter_core`
  - `project_context_core`
  - `environment_inventory_core`
- Export the engine-safe public types/functions from `handbook-engine`.
- Keep `crates/compiler/src/author/*.rs` as thin compatibility façades and shell/runtime owners.
- Keep template-library selection, Codex transport, canonical preflight/write flow, lock handling, refusal wording, and ambient timestamp/env resolution outside `handbook-engine`.
- Preserve public CLI behavior without a broad direct caller rewire.
- Expected files:
  - crates/engine/src/lib.rs
  - crates/engine/src/author/**
  - crates/compiler/src/lib.rs
  - crates/compiler/src/author/charter.rs
  - crates/compiler/src/author/project_context.rs
  - crates/compiler/src/author/environment_inventory.rs
  - crates/compiler/src/author/charter_core.rs
  - crates/compiler/src/author/project_context_core.rs
  - crates/compiler/src/author/environment_inventory_core.rs
  - optionally crates/compiler/src/author/*_shell.rs
  - optionally crates/engine/tests/**
  - optionally crates/compiler/tests/author.rs
  - optionally crates/cli/tests/cli_surface.rs

Out of scope:
- reopening Packet 4.2.1 canonical-stack ownership beyond tiny compile-through adjustments strictly needed to keep Packet 4.2.2 coherent
- moving `template_library`, `setup`, `doctor`, `refusal`, or `rendering` into `handbook-engine`
- moving guided interview behavior or other CLI-owned workflows into compiler or engine internals
- moving pipeline, route-state, compile/capture/handoff, flow, resolver, packet-result, or budget ownership
- direct `handbook-cli` rewires to `handbook-engine` as the primary runtime path
- Phase 4.3 pipeline migration, Phase 4.4 flow migration, or Phase 4.5 caller rewiring

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 2 Packet 4.2.2: Approved Authoring Core Migration`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 4.2.1 already landed and that the remaining approved Slice 4.2 seam is deterministic author-core ownership.
- Require live verification across:
  - `crates/engine/src/lib.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/author/charter.rs`
  - `crates/compiler/src/author/project_context.rs`
  - `crates/compiler/src/author/environment_inventory.rs`
  - `crates/compiler/src/author/charter_core.rs`
  - `crates/compiler/src/author/project_context_core.rs`
  - `crates/compiler/src/author/environment_inventory_core.rs`
  - `crates/compiler/src/author/charter_shell.rs`
  - `crates/compiler/src/author/project_context_shell.rs`
  - `crates/compiler/src/author/environment_inventory_shell.rs`
  - `crates/compiler/tests/author.rs`
  - `crates/cli/tests/cli_surface.rs`
- Require the implementation to:
  - move deterministic author-core implementations into an engine-owned namespace
  - export the engine-safe public types/functions from `handbook-engine`
  - keep compiler author façades thin and compatibility-oriented
  - keep shell/runtime/template/write responsibilities outside `handbook-engine`
  - preserve current behavior and compatibility naming unless a narrow adapter is required
- Require special care for the project-context boundary: move only explicit-input deterministic rendering/validation into the engine crate and leave ambient timestamp/env handling outside.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n 'charter_core|project_context_core|environment_inventory_core' crates/engine crates/compiler/src/author`
  - `rg -n 'use handbook_engine|pub use handbook_engine|handbook_engine::' crates/compiler/src/author crates/compiler/src/lib.rs`
  - `cargo test -p handbook-engine`
  - `cargo test -p handbook-compiler --test author`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 4.2.2 acceptance is met and report touched files, impact-analysis results, verification run, compatibility posture, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 2 Packet 4.2.2: Approved Authoring Core Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.2 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether deterministic author-core logic truly moved into `handbook-engine`
  - whether compiler author façades remained thin and shell-owned
  - whether template-library selection, Codex transport, canonical preflight/write flow, lock handling, refusal wording, and ambient timestamp/env resolution stayed outside `handbook-engine`
  - whether project-context deterministic rendering stayed explicit-input and did not drag ambient time/env state into engine ownership
  - whether author regressions and CLI surface regressions are covered by verification evidence
  - whether any setup/doctor/rendering/refusal cleanup, pipeline work, flow work, or caller rewires leaked into Packet 4.2.2
- Require severity labels and explicit callouts if the packet leaks shell behavior into the engine crate, leaves hidden duplicate ownership behind, or broadens beyond the approved author-core boundary.

Fix loop:
- If the review is clean, stop and report Packet 4.2.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.2.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.2.2 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.2.2 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.2.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.2.2 is review-clean and committed.
- Stop and report blocked if Packet 4.2.2 cannot be completed without reopening Packet 4.2.1 beyond tiny compile-through fallout, moving template-library/setup/doctor/rendering/refusal ownership into `handbook-engine`, widening into pipeline or flow migration, or rewiring direct callers as part of a later Phase 4 slice.
```
