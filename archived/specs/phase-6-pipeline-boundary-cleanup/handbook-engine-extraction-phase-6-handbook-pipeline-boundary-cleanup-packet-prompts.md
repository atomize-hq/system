# Handbook Engine Extraction Phase 6 `handbook-pipeline` Boundary Cleanup Packet Prompts

Task source: [handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md](./handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md)
Spec source: [handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md](./handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md)
Plan source: [handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md](./handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation and fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps execution bounded to the approved Phase 6 `handbook-pipeline` boundary-cleanup seam.

This slice is approved for one bounded implementation packet only. Do not widen into retained `handbook-compiler` retirement, CLI shell/support reassignment, publication, crates.io work, Substrate consumption, or broader integration implementation. If the narrow packet cannot land honestly without such widening, stop and report the blocker instead of silently expanding scope.

The orchestration session must not implement, review, or fix the packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

## Implementation Packet 1 Prompt

```text
/goal Orchestrate Phase 6 Handbook-Pipeline Boundary Cleanup Implementation Packet 1: Pipeline Catalog Fixture/Support Decoupling in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Implementation Packet 1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved seam authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md
- Treat the packet as one narrow decoupling seam: move the `pipeline_catalog` proof off compiler-owned template-library support, prefer a pipeline-owned test/support fixture or literal source first, and remove the now-unneeded compiler-backed dev-dependency if no remaining in-scope pipeline-owned test still needs it.
- Stay inside Implementation Packet 1 only.

Hard rules:
- Do not implement, review, or fix Packet 1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before editing any production symbol or helper under `crates/pipeline/src/**`, run GitNexus impact analysis first and report the blast radius. If the packet can close inside `crates/pipeline/tests/**` plus `crates/pipeline/Cargo.toml`, do not widen into production-symbol edits just for elegance.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Implementation Packet 1 only.
- Stay inside Implementation Packet 1 scope.

Implementation Packet 1 scope:
- Remove the `handbook_compiler::author::template_library` dependency from `crates/pipeline/tests/pipeline_catalog.rs`.
- Replace the proof source with a pipeline-owned test/support fixture or literal source rooted in the declared stage-library contract.
- Remove `handbook-compiler` from `crates/pipeline/Cargo.toml` dev-dependencies if no remaining in-scope pipeline-owned test still needs it.
- Preserve the reviewed supported-target wedge and do not widen into CLI/setup ownership, flow proof, compiler retirement, publication, crates.io work, or Substrate integration.
- Expected files:
  - crates/pipeline/tests/pipeline_catalog.rs
  - optionally crates/pipeline/tests/support/**
  - optionally crates/pipeline/src/**
  - crates/pipeline/Cargo.toml

Out of scope:
- retained `handbook-compiler` retirement or authoring-stack relocation
- any CLI shell/support ownership change
- any `handbook-flow` proof or boundary work
- any `setup` ownership reassignment
- publication or crates.io work
- Substrate consumption or broader integration implementation
- any new packet authoring beyond this approved packet prompt artifact

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 6 Handbook-Pipeline Boundary Cleanup Implementation Packet 1: Pipeline Catalog Fixture/Support Decoupling`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,120p' crates/pipeline/Cargo.toml`
  - `rg -n "handbook_compiler::author::template_library|resolve_shipped_template_library|TemplateLibraryRequest|TemplateLibrarySelection" crates/pipeline/tests crates/pipeline/src`
  - `sed -n '240,330p' crates/pipeline/tests/pipeline_catalog.rs`
  - `cargo tree -p handbook-pipeline`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
- Require the implementation to:
  - keep the reviewed supported-target importer boundary unchanged
  - prefer a pipeline-owned test/support fixture or literal source in `crates/pipeline/tests/**` before introducing any compiler-neutral helper or new public pipeline API
  - remove the compiler-owned proof dependency from `pipeline_catalog`
  - remove `handbook-compiler` from pipeline dev-dependencies if the decoupling leaves no remaining in-scope need
  - avoid widening into retained `handbook-compiler` retirement, CLI/setup work, flow work, publication, crates.io work, or Substrate integration
- Require GitNexus impact analysis before editing any touched production symbol and require the subagent to report impacted callers/processes.
- Require packet verification with:
  - `rg -n "handbook_compiler::author::template_library|resolve_shipped_template_library|TemplateLibraryRequest|TemplateLibrarySelection" crates/pipeline/tests crates/pipeline/src`
  - `cargo tree -p handbook-pipeline`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_compile`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
  - `cargo test -p handbook-compiler --test author`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 1 acceptance is met and report touched files, proof-source posture, dependency posture, verification run, impact-analysis results if any, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 6 Handbook-Pipeline Boundary Cleanup Implementation Packet 1: Pipeline Catalog Fixture/Support Decoupling`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the approved seam spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether `pipeline_catalog` no longer imports compiler-owned template-library support
  - whether the replacement proof source stays pipeline-owned first and remains narrow
  - whether `crates/pipeline/Cargo.toml` honestly drops the compiler-backed dev-dependency if no remaining in-scope test needs it
  - whether the reviewed supported-target wedge stayed unchanged
  - whether any CLI/setup/flow/compiler-retirement/publication/Substrate work leaked into the packet
- Require severity labels and explicit callouts if the packet over-engineers the replacement proof source, leaves the compiler dev-dependency behind without justification, or widens beyond scope.

Fix loop:
- If the review is clean, stop and report Implementation Packet 1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run only the verification commands affected by the fix.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Implementation Packet 1 only.
- Commit after each accepted fix round.
- Commit messages must describe the pipeline-catalog proof decoupling and dev-dependency cleanup clearly and standalone.

Stop conditions:
- Stop once Implementation Packet 1 is review-clean, committed, and the full verification wall is green.
- Stop and report blocked if the packet cannot close honestly without widening into retained `handbook-compiler` retirement, CLI/setup work, publication, crates.io work, Substrate consumption, or broader integration implementation.
```
