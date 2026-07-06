# Handbook Engine Extraction Phase 4 Slice 3 Packet Prompts

Task source: [handbook-engine-extraction-phase-4-slice-3-pipeline-migration-tasks.md](./handbook-engine-extraction-phase-4-slice-3-pipeline-migration-tasks.md)
Spec source: [handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md](./handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md)
Plan source: [handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md](./handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 4.3 scope.

## Packet 4.3.1 Prompt

```text
/goal Orchestrate Phase 4 Slice 3 Packet 4.3.1: Pipeline Loading Route And Route-State Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.3.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-tasks.md.
- Verify live repo truth before changing anything, including that Slice 4.2 already made `handbook-engine` the real owner of the approved engine-safe surfaces and that `handbook-pipeline` is still only a scaffold wrapper over `handbook-compiler`.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md
- Stay inside Packet 4.3.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.3.1 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.3.1 scope.

Packet 4.3.1 scope:
- Flip the pipeline/compiler dependency posture so `handbook-pipeline` can become the real implementation owner for the approved Slice 4.3 foundation surfaces.
- Move pipeline loading, catalog, selection, and supported-target registry into `handbook-pipeline`.
- Move route evaluation, route-state persistence, trusted-session validation, route-basis helpers, and runtime-state reset helpers into `handbook-pipeline`.
- Move or split only the narrow supporting-infra needed to make those moves real:
  - `declarative_roots`
  - `repo_file_access`
  - pipeline-focused portions of `layout`
- Move or recreate regression coverage so the migrated foundation is tested from `handbook-pipeline` itself.
- Expected files:
  - crates/pipeline/Cargo.toml
  - crates/compiler/Cargo.toml
  - crates/pipeline/src/lib.rs
  - crates/pipeline/src/pipeline.rs
  - crates/pipeline/src/pipeline_route.rs
  - crates/pipeline/src/route_state.rs
  - any narrow supporting helper files required under crates/pipeline/src/**
  - crates/compiler/src/lib.rs
  - crates/compiler/src/pipeline.rs
  - crates/compiler/src/pipeline_route.rs
  - crates/compiler/src/route_state.rs
  - optionally crates/compiler/src/declarative_roots.rs
  - optionally crates/compiler/src/repo_file_access.rs
  - optionally crates/compiler/src/layout.rs
  - crates/pipeline/tests/**
  - optionally crates/compiler/tests/pipeline_loader.rs
  - optionally crates/compiler/tests/pipeline_catalog.rs
  - optionally crates/compiler/tests/pipeline_route_resolution.rs
  - optionally crates/compiler/tests/pipeline_state_store.rs

Out of scope:
- moving compile/capture/handoff execution mechanics (that is Packet 4.3.2)
- moving stage-10 provenance or setup-helper alignment work (that is Packet 4.3.3)
- moving `resolver`, `packet_result`, or `budget` into `handbook-flow`
- moving `rendering`, `refusal`, or `error` into `handbook-pipeline`
- broad setup-shell redesign or starter-template ownership changes
- direct `handbook-cli` rewires to `handbook-pipeline` as the primary runtime path
- Phase 4.4 flow migration or Phase 4.5 caller rewiring

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 3 Packet 4.3.1: Pipeline Loading Route And Route-State Migration`.
- Tell the subagent to use $incremental-implementation.
- Require live verification of current dependency and ownership truth across:
  - `crates/pipeline/Cargo.toml`
  - `crates/pipeline/src/lib.rs`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/pipeline.rs`
  - `crates/compiler/src/pipeline_route.rs`
  - `crates/compiler/src/route_state.rs`
  - `crates/compiler/src/declarative_roots.rs`
  - `crates/compiler/src/repo_file_access.rs`
  - `crates/compiler/src/layout.rs`
  - `crates/compiler/tests/pipeline_loader.rs`
  - `crates/compiler/tests/pipeline_catalog.rs`
  - `crates/compiler/tests/pipeline_route_resolution.rs`
  - `crates/compiler/tests/pipeline_state_store.rs`
- Require the implementation to:
  - eliminate the temporary `handbook-pipeline -> handbook-compiler` ownership posture for the Packet 4.3.1 foundation stack
  - keep the resulting dependency graph acyclic
  - move pipeline loading / supported-target registry / route / route-state logic behind `handbook-pipeline`
  - move or split only the supporting-infra subset strictly required for real pipeline ownership
  - leave `handbook-compiler` with only narrow compatibility re-exports or thin adapters for migrated surfaces
  - add or migrate pipeline-owned tests so `handbook-pipeline` proves the moved behavior directly
- Require the implementation to keep supported-target behavior, route-state schema/layout behavior, and trusted-session semantics stable unless live repo truth proves a narrow compatibility adjustment is required.
- Require the implementation to avoid pulling compile/capture/handoff, provenance, setup-shell redesign, flow work, or CLI rewires into this packet.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `cargo tree -p handbook-pipeline -e normal`
  - `cargo tree -p handbook-compiler -e normal`
  - `cargo check --workspace`
  - `rg -n 'load_pipeline_catalog|resolve_pipeline_route|load_route_state|persist_route_basis|load_trusted_pipeline_session|plan_runtime_state_reset|apply_runtime_state_reset' crates/pipeline crates/compiler/src`
  - `cargo test -p handbook-pipeline`
  - `cargo test -p handbook-compiler --test pipeline_loader`
  - `cargo test -p handbook-compiler --test pipeline_catalog`
  - `cargo test -p handbook-compiler --test pipeline_route_resolution`
  - `cargo test -p handbook-compiler --test pipeline_state_store`
- Require the subagent to stop after Packet 4.3.1 acceptance is met and report touched files, impact-analysis results, dependency posture, supporting-infra posture, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 3 Packet 4.3.1: Pipeline Loading Route And Route-State Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.3 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether the pipeline/compiler dependency inversion is acyclic and actually makes `handbook-pipeline` the real owner
  - whether `pipeline.rs`, `pipeline_route.rs`, and `route_state.rs` now have exactly one real implementation owner
  - whether any `declarative_roots` / `repo_file_access` / `layout` movement stayed narrow and justified instead of becoming a broad infrastructure rewrite
  - whether `handbook-compiler` kept only thin compatibility layers instead of duplicate implementation bodies
  - whether pipeline-owned tests really cover the migrated foundation stack
  - whether any compile/capture/handoff, provenance/setup alignment, flow, or CLI-rewire work leaked into Packet 4.3.1
- Require severity labels and explicit callouts if the packet leaves split-brain ownership, introduces cycles, or hides migration gaps behind oversized compiler facades.

Fix loop:
- If the review is clean, stop and report Packet 4.3.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.3.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.3.1 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.3.1 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.3.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.3.1 is review-clean and committed.
- Stop and report blocked if Packet 4.3.1 cannot be completed without widening into compile/capture/handoff migration, provenance/setup-helper alignment, flow migration, direct CLI rewires, or later Phase 4 slices.
```

## Packet 4.3.2 Prompt

```text
/goal Orchestrate Phase 4 Slice 3 Packet 4.3.2: Compile Capture And Handoff Migration in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.3.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-tasks.md.
- Verify live repo truth before changing anything, including that Packet 4.3.1 already made `handbook-pipeline` the real owner of pipeline loading, route, route-state, and trusted-session behavior.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md
- Stay inside Packet 4.3.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.3.2 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.3.2 scope.

Packet 4.3.2 scope:
- Move compile mechanics into `handbook-pipeline`.
- Move capture mechanics into `handbook-pipeline`.
- Move handoff mechanics into `handbook-pipeline`.
- Keep `handbook-compiler` runtime surfaces for those families as explicit compatibility re-exports or thin adapters only.
- Move or recreate regression coverage so the migrated runtime execution surfaces are tested from `handbook-pipeline` itself.
- Preserve CLI behavior without a broad direct caller rewire.
- Expected files:
  - crates/pipeline/src/lib.rs
  - crates/pipeline/src/pipeline_compile.rs
  - crates/pipeline/src/pipeline_capture.rs
  - crates/pipeline/src/pipeline_handoff.rs
  - any supporting helper files needed under crates/pipeline/src/**
  - crates/compiler/src/lib.rs
  - crates/compiler/src/pipeline_compile.rs
  - crates/compiler/src/pipeline_capture.rs
  - crates/compiler/src/pipeline_handoff.rs
  - crates/pipeline/tests/**
  - optionally crates/compiler/tests/pipeline_compile.rs
  - optionally crates/compiler/tests/pipeline_capture.rs
  - optionally crates/compiler/tests/pipeline_handoff.rs
  - optionally crates/cli/tests/cli_surface.rs

Out of scope:
- reopening Packet 4.3.1 foundation ownership beyond tiny compile-through adjustments strictly needed to keep Packet 4.3.2 coherent
- moving stage-10 provenance or setup-helper alignment work (that is Packet 4.3.3)
- moving `resolver`, `packet_result`, or `budget` into `handbook-flow`
- moving `rendering`, `refusal`, or `error` into `handbook-pipeline`
- broad setup-shell redesign
- direct `handbook-cli` rewires to `handbook-pipeline` as the primary runtime path
- Phase 4.4 flow migration or Phase 4.5 caller rewiring

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 3 Packet 4.3.2: Compile Capture And Handoff Migration`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 4.3.1 already landed and that the remaining approved Slice 4.3 seam is runtime execution ownership.
- Require live verification across:
  - `crates/pipeline/src/lib.rs`
  - `crates/pipeline/src/pipeline_compile.rs`
  - `crates/pipeline/src/pipeline_capture.rs`
  - `crates/pipeline/src/pipeline_handoff.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/pipeline_compile.rs`
  - `crates/compiler/src/pipeline_capture.rs`
  - `crates/compiler/src/pipeline_handoff.rs`
  - `crates/compiler/tests/pipeline_compile.rs`
  - `crates/compiler/tests/pipeline_capture.rs`
  - `crates/compiler/tests/pipeline_handoff.rs`
  - `crates/cli/tests/cli_surface.rs`
- Require the implementation to:
  - move compile/capture/handoff implementations into a pipeline-owned namespace
  - keep compiler compatibility surfaces thin and explicit
  - preserve supported-target behavior, refusal posture, route-basis behavior, provenance inputs, and handoff trust/write behavior unless a narrow compatibility adjustment is required
  - add or migrate pipeline-owned tests so `handbook-pipeline` proves the moved behavior directly
- Require the implementation to avoid pulling stage-10 provenance ownership, setup-helper migration, flow work, or CLI rewires into this packet.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n 'compile_pipeline_stage|capture_pipeline_output|preview_pipeline_capture|emit_pipeline_handoff_bundle|validate_pipeline_handoff_bundle' crates/pipeline crates/compiler/src`
  - `cargo test -p handbook-pipeline`
  - `cargo test -p handbook-compiler --test pipeline_compile`
  - `cargo test -p handbook-compiler --test pipeline_capture`
  - `cargo test -p handbook-compiler --test pipeline_handoff`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 4.3.2 acceptance is met and report touched files, impact-analysis results, verification run, compatibility posture, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 3 Packet 4.3.2: Compile Capture And Handoff Migration`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.3 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether compile/capture/handoff runtime logic truly moved into `handbook-pipeline`
  - whether `handbook-compiler` kept only thin compatibility layers instead of duplicate implementation bodies
  - whether route-state / trusted-session / supported-target behavior remained stable after the ownership move
  - whether pipeline-owned tests really cover the migrated runtime execution surfaces
  - whether CLI surface verification is sufficient for public fallout
  - whether any provenance/setup-helper work, flow work, or caller rewires leaked into Packet 4.3.2
- Require severity labels and explicit callouts if the packet leaves hidden duplicate ownership behind, weakens behavior guarantees, or broadens beyond the approved runtime execution boundary.

Fix loop:
- If the review is clean, stop and report Packet 4.3.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.3.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.3.2 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.3.2 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.3.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.3.2 is review-clean and committed.
- Stop and report blocked if Packet 4.3.2 cannot be completed without reopening Packet 4.3.1 beyond tiny compatibility fallout, widening into provenance/setup-helper alignment, flow migration, direct CLI rewires, or later Phase 4 slices.
```

## Packet 4.3.3 Prompt

```text
/goal Orchestrate Phase 4 Slice 3 Packet 4.3.3: Setup Helper And Provenance Alignment in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.3.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-tasks.md.
- Verify live repo truth before changing anything, including that Packet 4.3.2 already made `handbook-pipeline` the real owner of compile/capture/handoff runtime mechanics.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-plan.md
- Stay inside Packet 4.3.3 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.3.3 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.3.3 scope.

Packet 4.3.3 scope:
- Move stage-10 provenance generation and validation into `handbook-pipeline`.
- Move the approved reusable setup-helper seam into `handbook-pipeline` while keeping product-facing setup behavior in compiler.
- Prove the final provenance/setup-helper seam with package-local coverage and public CLI guards.
- Expected files:
  - crates/pipeline/src/lib.rs
  - crates/pipeline/src/stage_10_feature_spec_provenance.rs
  - crates/pipeline/src/route_state.rs
  - crates/pipeline/src/setup.rs or equivalent helper module
  - crates/pipeline/src/pipeline_capture.rs
  - crates/pipeline/src/pipeline_handoff.rs
  - crates/compiler/src/lib.rs
  - crates/compiler/src/stage_10_feature_spec_provenance.rs
  - crates/compiler/src/setup.rs
  - optionally crates/compiler/src/setup_shell.rs
  - crates/pipeline/tests/**
  - optionally crates/compiler/tests/pipeline_capture.rs
  - optionally crates/compiler/tests/pipeline_handoff.rs
  - optionally crates/compiler/tests/setup.rs
  - optionally crates/cli/tests/cli_surface.rs
  - optionally crates/cli/tests/help_drift_guard.rs

Out of scope:
- reopening Packet 4.3.1 or Packet 4.3.2 beyond tiny compatibility adjustments strictly needed to keep Packet 4.3.3 coherent
- broad setup-shell redesign, prompt wording cleanup, or starter-template ownership migration
- moving `resolver`, `packet_result`, or `budget` into `handbook-flow`
- moving `rendering`, `refusal`, or `error` into `handbook-pipeline`
- direct `handbook-cli` rewires to `handbook-pipeline` as the primary runtime path
- Phase 4.4 flow migration or Phase 4.5 caller rewiring

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 3 Packet 4.3.3: Setup Helper And Provenance Alignment`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 4.3.2 already landed and that the remaining approved Slice 4.3 seam is provenance plus the narrow reusable setup-helper boundary.
- Require live verification across:
  - `crates/pipeline/src/lib.rs`
  - `crates/pipeline/src/stage_10_feature_spec_provenance.rs`
  - `crates/pipeline/src/route_state.rs`
  - `crates/pipeline/src/setup.rs` or equivalent helper module if it exists
  - `crates/pipeline/src/pipeline_capture.rs`
  - `crates/pipeline/src/pipeline_handoff.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/stage_10_feature_spec_provenance.rs`
  - `crates/compiler/src/setup.rs`
  - `crates/compiler/src/setup_shell.rs`
  - `crates/compiler/tests/pipeline_capture.rs`
  - `crates/compiler/tests/pipeline_handoff.rs`
  - `crates/compiler/tests/setup.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
- Require the implementation to:
  - move stage-10 provenance generation/validation into a pipeline-owned namespace
  - move only the approved reusable setup-helper seam into `handbook-pipeline`
  - keep `crates/compiler/src/setup.rs` as the product-facing facade for request/disposition/refusal behavior and any non-pipeline starter-template logic left outside this slice
  - preserve provenance behavior, route-basis fingerprinting, setup reset semantics, and CLI behavior unless a narrow compatibility adjustment is required
  - add or migrate pipeline-owned tests so `handbook-pipeline` proves the final provenance/setup-helper seam directly
- Require the implementation to avoid pulling broad setup-shell work, flow work, or direct CLI rewires into this packet.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n 'stage_10_feature_spec|route_basis_fingerprint_sha256|build_stage_10_feature_spec_capture_provenance|plan_runtime_state_reset|apply_runtime_state_reset|reset_state' crates/pipeline crates/compiler/src`
  - `cargo test -p handbook-pipeline`
  - `cargo test -p handbook-compiler --test pipeline_capture`
  - `cargo test -p handbook-compiler --test pipeline_handoff`
  - `cargo test -p handbook-compiler --test setup`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 4.3.3 acceptance is met and report touched files, impact-analysis results, verification run, compatibility posture, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 3 Packet 4.3.3: Setup Helper And Provenance Alignment`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.3 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether stage-10 provenance truly moved into `handbook-pipeline`
  - whether the approved reusable setup-helper seam moved into `handbook-pipeline` without dragging broad setup-shell behavior or starter-template ownership with it
  - whether `crates/compiler/src/setup.rs` stayed a thin product-facing facade instead of remaining the real implementation owner
  - whether pipeline-owned tests really cover the final provenance/setup-helper seam
  - whether CLI surface/help verification is sufficient for public fallout
  - whether any flow work or caller rewires leaked into Packet 4.3.3
- Require severity labels and explicit callouts if the packet broadens beyond the approved narrow setup-helper boundary, leaves hidden compiler ownership behind, or regresses provenance/setup behavior.

Fix loop:
- If the review is clean, stop and report Packet 4.3.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.3.3-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.3.3 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.3.3 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.3.3 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.3.3 is review-clean and committed.
- Stop and report blocked if Packet 4.3.3 cannot be completed without reopening earlier packets beyond tiny compatibility fallout, widening into flow migration, broad setup redesign, direct CLI rewires, or later Phase 4 slices.
```
