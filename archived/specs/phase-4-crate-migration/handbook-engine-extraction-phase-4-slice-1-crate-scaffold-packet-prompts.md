# Handbook Engine Extraction Phase 4 Slice 1 Packet Prompts

Task source: [handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md](./handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md)
Spec source: [handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md](./handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md)
Plan source: [handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md](./handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 4.1 scope.

## Packet 4.1.1 Prompt

```text
/goal Orchestrate Phase 4 Slice 1 Packet 4.1.1: Workspace Members And Crate Manifests in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md.
- Verify live repo truth before changing anything, including that the workspace currently centers on `crates/compiler` and `crates/cli` and that `crates/engine`, `crates/pipeline`, and `crates/flow` are not yet established as real workspace members.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md
- Stay inside Packet 4.1.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.1.1 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.1.1 scope.

Packet 4.1.1 scope:
- Add `crates/engine`, `crates/pipeline`, and `crates/flow` as real workspace members.
- Create valid `Cargo.toml` manifests and compile-valid `src/lib.rs` roots for each new crate.
- Keep the new crate roots minimal and scaffold-only.
- Prove the workspace recognizes the new crates cleanly.
- Expected files:
  - Cargo.toml
  - crates/engine/Cargo.toml
  - crates/engine/src/lib.rs
  - crates/pipeline/Cargo.toml
  - crates/pipeline/src/lib.rs
  - crates/flow/Cargo.toml
  - crates/flow/src/lib.rs

Out of scope:
- broad public crate surfaces or wildcard re-export design beyond the minimum compile-valid root needed for Packet 4.1.1
- `crates/compiler/src/lib.rs` compile-through wiring beyond tiny fallout that live repo truth proves unavoidable
- direct `handbook-cli` caller rewires or runtime dependency path changes
- moving major implementation modules out of `crates/compiler`
- narrowing or retiring `crates/compiler`
- Phase 4.2 engine migration, Phase 4.3 pipeline migration, Phase 4.4 flow migration, or Phase 4.5 caller rewiring
- broader workspace metadata cleanup not required for the new crate scaffolds to compile

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 1 Packet 4.1.1: Workspace Members And Crate Manifests`.
- Tell the subagent to use $incremental-implementation.
- Require live verification of current workspace truth across:
  - `Cargo.toml`
  - `crates/compiler/Cargo.toml`
  - `crates/cli/Cargo.toml`
  - the current `crates/` directory layout
- Require the implementation to:
  - add `crates/engine`, `crates/pipeline`, and `crates/flow` to workspace membership
  - create minimal valid manifests for `handbook-engine`, `handbook-pipeline`, and `handbook-flow`
  - create minimal compile-valid `src/lib.rs` roots for all three crates
  - keep the new crate roots intentionally narrow and scaffold-only
- Require the implementation to avoid broad public API design in this packet; the new `src/lib.rs` files may be skeletal so long as they are valid, intentional, and compile cleanly.
- Require package names and library crate names to align with the approved Phase 4 naming (`handbook-engine`/`handbook_engine`, `handbook-pipeline`/`handbook_pipeline`, `handbook-flow`/`handbook_flow`).
- Require dependency posture to stay simple and acyclic; do not introduce cross-crate dependency edges unless they are strictly required for a compile-valid scaffold.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `find crates -maxdepth 2 -name Cargo.toml | sort`
  - `cargo metadata --no-deps --format-version 1 | python3 -c 'import sys,json; m=json.load(sys.stdin); print("\n".join(sorted(p["name"] for p in m["packages"])))'`
  - `rg -n 'crates/(engine|pipeline|flow)' Cargo.toml crates/*/Cargo.toml`
  - `cargo check --workspace`
- Require the subagent to stop after Packet 4.1.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 1 Packet 4.1.1: Workspace Members And Crate Manifests`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.1 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether all three new crates are real workspace members with valid manifests and source roots
  - whether package names and library names match the approved Phase 4 naming
  - whether the new crate roots stayed scaffold-only rather than freezing a broad public API early
  - whether dependency posture stayed simple and acyclic
  - whether any module migration, compiler narrowing, or CLI rewire work leaked into Packet 4.1.1
- Require severity labels and explicit callouts if the packet introduced unnecessary dependency edges, broad facade exports, or adjacent-slice leakage.

Fix loop:
- If the review is clean, stop and report Packet 4.1.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.1.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.1.1 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.1.1 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.1.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.1.1 is review-clean and committed.
- Stop and report blocked if Packet 4.1.1 cannot be completed without widening into public-surface design work from Packet 4.1.2, major module moves, direct CLI rewires, compiler narrowing, or later Phase 4 migration slices.
```

## Packet 4.1.2 Prompt

```text
/goal Orchestrate Phase 4 Slice 1 Packet 4.1.2: Minimal Public Crate Surfaces And Compile-Through Wiring in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 4.1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-tasks.md.
- Verify live repo truth before changing anything, including that Packet 4.1.1 already created `crates/engine`, `crates/pipeline`, and `crates/flow` as real workspace members and that the remaining approved Slice 4.1 seam is narrow public surface definition plus compile-through wiring.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-4-slice-1-crate-scaffold-plan.md
- Stay inside Packet 4.1.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 4.1.2 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 4.1.2 scope.

Packet 4.1.2 scope:
- Define explicit minimal public surfaces for `handbook-engine`, `handbook-pipeline`, and `handbook-flow`.
- Add only the narrow compile-through wiring needed to make those surfaces intentional and reviewable.
- Keep `handbook-compiler` as the temporary implementation center.
- Prove the dependency graph stays acyclic and the CLI surface remains stable.
- Expected files:
  - crates/engine/src/lib.rs
  - crates/pipeline/src/lib.rs
  - crates/flow/src/lib.rs
  - optionally crates/compiler/src/lib.rs
  - optionally crates/engine/Cargo.toml
  - optionally crates/pipeline/Cargo.toml
  - optionally crates/flow/Cargo.toml
  - optionally crates/cli/Cargo.toml if tiny compile-through fallout is strictly required
  - optionally crates/cli/src/main.rs if tiny compile-through fallout is strictly required
  - optionally crates/cli/tests/cli_surface.rs if public-surface fallout reaches the CLI guard

Out of scope:
- moving major implementation modules into `handbook-engine`, `handbook-pipeline`, or `handbook-flow`
- rewiring `handbook-cli` to the new crates as its primary runtime dependency path
- narrowing or retiring `crates/compiler` as the monolithic implementation center
- wildcard re-export facades such as `pub use handbook_compiler::*`
- forcing `rendering`, `refusal`, or `error` into a premature crate home
- Phase 4.2 engine migration, Phase 4.3 pipeline migration, Phase 4.4 flow migration, or Phase 4.5 caller rewiring
- broader workspace refactors outside the narrow public-surface and compile-through seam

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 4 Slice 1 Packet 4.1.2: Minimal Public Crate Surfaces And Compile-Through Wiring`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 4.1.1 already landed and that the new crates currently have only scaffold roots that still need reviewed public surface definitions.
- Require the implementation to define one narrow explicit surface per new crate, aligned to its approved future ownership lane:
  - `handbook-engine` -> engine-safe marker or very small lane-aligned compile-through surface
  - `handbook-pipeline` -> pipeline-safe marker or very small lane-aligned compile-through surface
  - `handbook-flow` -> flow-safe marker or very small lane-aligned compile-through surface
- Require the implementation to prefer the smallest durable surface that proves cross-crate structure without prematurely freezing a broad API.
- Require all exports to be explicit; do not allow wildcard compiler-facade re-exports.
- Require `crates/compiler/src/lib.rs` to remain the temporary implementation center; any changes there must be narrow compile-through support only.
- Require dependency posture to stay acyclic and easy to reason about; if a cross-crate edge is introduced, the subagent must justify why it is the smallest safe option for Slice 4.1.
- Require `handbook-cli` behavior to stay stable; do not make the new crates the primary runtime dependency path in this packet.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n 'pub use|contract_version|workspace_contract_version' crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs`
  - `cargo tree -p handbook-engine -e normal`
  - `cargo tree -p handbook-pipeline -e normal`
  - `cargo tree -p handbook-flow -e normal`
  - `cargo check --workspace`
  - `cargo test -p handbook-cli --test cli_surface`
- Require the subagent to stop after Packet 4.1.2 acceptance is met and report touched files, impact-analysis results, verification run, dependency posture, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 4 Slice 1 Packet 4.1.2: Minimal Public Crate Surfaces And Compile-Through Wiring`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 4.1 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether each new crate exposes only a narrow explicit lane-aligned surface
  - whether `handbook-compiler` still remains the temporary implementation center
  - whether dependency direction stays acyclic and understandable
  - whether any broad public API freeze or wildcard facade export leaked into the packet
  - whether `handbook-cli` remained behavior-stable without becoming directly rewired to the new crates as the main runtime path
  - whether any real module migration, compiler retirement, or adjacent Phase 4 work leaked into Packet 4.1.2
- Require severity labels and explicit callouts if the packet froze too much API surface, introduced risky dependency direction, or spilled into later migration slices.

Fix loop:
- If the review is clean, stop and report Packet 4.1.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-4.1.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 4.1.2 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 4.1.2 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 4.1.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 4.1.2 is review-clean and committed.
- Stop and report blocked if Packet 4.1.2 cannot be completed without moving major implementation modules, rewiring the CLI as the main consumer of the new crates, narrowing or retiring `crates/compiler`, or spilling into Phase 4.2 through Phase 4.5 work.
```
