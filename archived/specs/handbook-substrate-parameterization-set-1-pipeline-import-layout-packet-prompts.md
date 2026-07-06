# Handbook Substrate Parameterization — Set 1 Packet Prompts

Task source: [handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md)
Spec source: [handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md)
Plan source: [handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation and fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps execution bounded to Set 1 only.

This set is approved only for the `handbook-pipeline` import-layout parameterization seam captured in this artifact: Packet 1.1 (declarative root contract), Packet 1.2 (stage-root adoption), Packet 1.3 (public storage-layout injection), and Packet 1.4 (final set proof). Do not widen into Set 2, Set 3, CLI/compiler cleanup, publication, or actual Substrate import work. If the narrow packet cannot land honestly without such widening, stop and report the blocker instead of silently expanding scope.

The orchestration session must not implement, review, or fix the packet work itself first. It must delegate implementation, review, and any fix rounds to fresh subagents exactly as directed below.

---

## Packet 1.1 Prompt — Declarative Root Contract And Owner Boundary

```text
/goal Orchestrate Set 1 Packet 1.1: Declarative Root Contract And Owner Boundary in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md
- Treat the packet as one narrow seam: introduce the supported public/import-facing declarative root contract and adopt it at the root-owner boundary while preserving handbook-product defaults through an explicit helper.
- Stay inside Packet 1.1 only.

Hard rules:
- Do not implement, review, or fix Packet 1.1 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before editing any production symbol in `crates/pipeline/src/declarative_roots.rs`, `crates/pipeline/src/lib.rs`, or `crates/pipeline/src/pipeline.rs`, run GitNexus impact analysis first and report the blast radius. If GitNexus says the index is stale, run `npx gitnexus analyze` first. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.1 only.
- Stay inside Packet 1.1 scope.

Packet 1.1 scope:
- Introduce the supported declarative root contract for pipeline/profile/runner/stage roots.
- Preserve handbook-product defaults through an explicit default helper.
- Adopt the contract in the root-owner helper surface and any minimal entry-point wiring required for `pipeline_catalog` / `pipeline_loader` proof.
- Re-export only the supported public surface needed for downstream importers.
- Update only the minimal tests needed for this packet.
- Expected files:
  - crates/pipeline/src/declarative_roots.rs
  - crates/pipeline/src/lib.rs
  - crates/pipeline/src/pipeline.rs
  - crates/pipeline/tests/pipeline_catalog.rs
  - crates/pipeline/tests/pipeline_loader.rs

Out of scope — do NOT touch:
- Packet 1.2 stage-source/discovery/validation adoption beyond the minimal root-owner wiring needed to keep Packet 1.1 coherent
- Packet 1.3 storage-layout injection
- Packet 1.4 final set proof
- Set 2 or Set 3 work
- CLI/compiler/product-shell cleanup
- actual Substrate import execution
- free-form multi-consumer generalization

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 1 Packet 1.1: Declarative Root Contract And Owner Boundary`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,220p' crates/pipeline/src/declarative_roots.rs`
  - `sed -n '1,220p' crates/pipeline/src/lib.rs`
  - `sed -n '1,260p' crates/pipeline/src/pipeline.rs`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_loader`
- Require the implementation to:
  - run GitNexus impact analysis before editing production symbols and report the blast radius
  - introduce the supported public/import-facing declarative root contract
  - preserve handbook-product defaults through an explicit default helper
  - adopt the contract in the root-owner boundary without widening into stage-source or storage-layout work that belongs to later packets
  - update only the minimal tests needed for Packet 1.1
  - stop after Packet 1.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 1 Packet 1.1: Declarative Root Contract And Owner Boundary`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the public/import-facing declarative contract is coherent and minimal
  - whether handbook-product defaults stayed explicit rather than hidden fallback behavior
  - whether any scope leaked into Packet 1.2 or Packet 1.3 work
  - whether the changed tests prove the intended boundary rather than a wider redesign
- Require severity labels and explicit callouts for scope drift, unnecessary abstraction, or missing proof.

Fix loop:
- If the review is clean, stop and report Packet 1.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.1-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the declarative-root contract work clearly and standalone.

Stop conditions:
- Stop once Packet 1.1 is review-clean, committed, and the supported declarative root contract exists with explicit handbook-product defaults preserved.
- Stop and report blocked if the packet cannot close honestly without widening into stage-root adoption, storage-layout injection, CLI/compiler cleanup, or later sets.
```

---

## Packet 1.2 Prompt — Stage-Root Discovery And Validation Adoption

```text
/goal Orchestrate Set 1 Packet 1.2: Stage-Root Discovery And Validation Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md
- Treat the packet as one narrow seam: move supported stage-source assumptions, stage discovery, and inseparable stage/pipeline path validation onto the active declarative contract.
- Packet 1.1 must already be complete.
- Stay inside Packet 1.2 only.

Hard rules:
- Do not implement, review, or fix Packet 1.2 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Preserve unrelated local edits, including any dirt already present in `AGENTS.md` and `CLAUDE.md`.
- Before editing any production symbol in `crates/pipeline/src/pipeline.rs`, run GitNexus impact analysis first and report the blast radius. If GitNexus says the index is stale, run `npx gitnexus analyze` first. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.2 only.
- Stay inside Packet 1.2 scope.

Packet 1.2 scope:
- Replace raw structural ownership of `core/stages/**` and handbook-product default-root truth with contract-driven behavior in supported-target/source-path derivation and catalog discovery.
- Drive stage discovery from the active stage root instead of handbook-product default discovery behavior.
- Update stage/pipeline path validation only where the active contract must be used for structural correctness.
- Remove or replace the old loader proof that codifies rejection of non-default stage roots, and replace it with positive contract-driven acceptance coverage.
- Update only the minimal tests needed for Packet 1.2.
- Expected files:
  - crates/pipeline/src/pipeline.rs
  - crates/pipeline/tests/pipeline_catalog.rs
  - crates/pipeline/tests/pipeline_loader.rs
  - crates/pipeline/tests/pipeline_compile.rs
  - crates/pipeline/tests/pipeline_route_resolution.rs

Out of scope — do NOT touch:
- Packet 1.1 declarative-contract design beyond the minimal changes required by this packet
- Packet 1.3 storage-layout injection
- Packet 1.4 final set proof
- Set 2 or Set 3 work
- CLI/compiler/product-shell cleanup beyond inseparable contract-derived path wording
- consumer-ownership generalization
- actual Substrate import execution

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 1 Packet 1.2: Stage-Root Discovery And Validation Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `npx gitnexus status`
  - `sed -n '1,260p' crates/pipeline/src/declarative_roots.rs`
  - `sed -n '1,320p' crates/pipeline/src/pipeline.rs`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_loader`
  - `cargo test -p handbook-pipeline --test pipeline_compile`
  - `cargo test -p handbook-pipeline --test pipeline_route_resolution`
- Require the implementation to:
  - verify Packet 1.1 landed first
  - run `npx gitnexus analyze` if `npx gitnexus status` reports a stale index, then run GitNexus impact analysis before editing production symbols and report the blast radius
  - move supported stage-source assumptions and discovery onto the active declarative contract rather than handbook-product default roots
  - update only the validation/refusal behavior that is inseparable from the structural root change
  - remove or replace the old loader proof that codifies rejection of non-default stage roots with positive proof for contract-driven acceptance
  - avoid widening into broader wording cleanup or storage-layout work
  - stop after Packet 1.2 acceptance is met and report touched files, impact-analysis results, verification run, residual risks, and the exact commit hash/message if one is created

Review subagent prompt requirements:
- Begin with `/goal Review Set 1 Packet 1.2: Stage-Root Discovery And Validation Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether stage-root behavior is truly contract-driven rather than just renamed literals
  - whether handbook-product default roots are still acting as active truth in discovery or validation
  - whether the old loader blocker semantics were actually removed and replaced with positive proof
  - whether validation changes stayed inseparable from the structural seam instead of drifting into Set 3 cleanup
  - whether scope leaked beyond `pipeline.rs` + the targeted tests
- Require severity labels and explicit callouts for false proof, widened scope, or incomplete adoption.

Fix loop:
- If the review is clean, stop and report Packet 1.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.2-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.2 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the stage-root adoption work clearly and standalone.

Stop conditions:
- Stop once Packet 1.2 is review-clean, committed, and stage-root behavior is structurally contract-driven.
- Stop and report blocked if the packet cannot close honestly without widening into storage-layout injection, broader wording cleanup, CLI/compiler work, or later sets.
```

---

## Packet 1.3 Prompt — Public Pipeline Storage Layout Injection

```text
/goal Orchestrate Set 1 Packet 1.3: Public Pipeline Storage Layout Injection in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.3 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md
- Treat the packet as one narrow seam: promote the pipeline storage layout contract to a supported public/import-facing boundary and adopt it across route-state, capture, and handoff entry points.
- Packets 1.1 and 1.2 must already be complete.
- Stay inside Packet 1.3 only.

Hard rules:
- Do not implement, review, or fix Packet 1.3 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before editing any production symbol in `crates/pipeline/src/layout.rs`, `crates/pipeline/src/route_state.rs`, `crates/pipeline/src/pipeline_capture.rs`, or `crates/pipeline/src/pipeline_handoff.rs`, run GitNexus impact analysis first and report the blast radius. If GitNexus says the index is stale, run `npx gitnexus analyze` first. If the blast radius is HIGH or CRITICAL, stop and report it before editing.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.3 only.
- Stay inside Packet 1.3 scope.

Packet 1.3 scope:
- Make the storage layout contract supported/public/import-facing.
- Preserve handbook-product defaults through an explicit default helper.
- Adopt the contract in route-state entry points.
- Adopt the contract in capture provenance/cache entry points.
- Adopt the contract in handoff bundle entry points.
- Update only the minimal tests needed for Packet 1.3.
- Expected files:
  - crates/pipeline/src/layout.rs
  - crates/pipeline/src/lib.rs
  - crates/pipeline/src/route_state.rs
  - crates/pipeline/src/pipeline_capture.rs
  - crates/pipeline/src/pipeline_handoff.rs
  - crates/pipeline/tests/pipeline_state_store.rs
  - crates/pipeline/tests/pipeline_capture.rs
  - crates/pipeline/tests/pipeline_handoff.rs

Out of scope — do NOT touch:
- Packet 1.1 or Packet 1.2 seams except for strictly necessary wiring
- Packet 1.4 final set proof
- Set 2 or Set 3 work
- `setup` becoming part of the import contract unless explicitly required and approved
- CLI/compiler/product-shell cleanup
- actual Substrate import execution

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 1 Packet 1.3: Public Pipeline Storage Layout Injection`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' crates/pipeline/src/layout.rs`
  - `sed -n '1,220p' crates/pipeline/src/lib.rs`
  - `sed -n '1,220p' crates/pipeline/src/route_state.rs`
  - `sed -n '1,260p' crates/pipeline/src/pipeline_capture.rs`
  - `sed -n '1,260p' crates/pipeline/src/pipeline_handoff.rs`
  - `cargo test -p handbook-pipeline --test pipeline_state_store`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
- Require the implementation to:
  - verify Packets 1.1 and 1.2 landed first
  - run GitNexus impact analysis before editing production symbols and report the blast radius
  - promote the storage layout contract to the supported public/import-facing boundary
  - preserve handbook-product defaults via an explicit helper rather than hidden fallback behavior
  - adopt the contract in route-state, capture, and handoff entry points
  - keep `setup` out of the import contract unless an approved blocker forces escalation
  - stop after Packet 1.3 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks

Review subagent prompt requirements:
- Begin with `/goal Review Set 1 Packet 1.3: Public Pipeline Storage Layout Injection`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether downstream importers can really use the public storage layout seam without crate-private access
  - whether handbook-product defaults stayed explicit
  - whether containment validation still protects runtime-state-owned paths
  - whether route-state/capture/handoff adoption stayed bounded without widening into setup or CLI/compiler work
- Require severity labels and explicit callouts for hidden fallback behavior, overexposed APIs, or scope drift.

Fix loop:
- If the review is clean, stop and report Packet 1.3 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.3-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.3 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.3 only.
- Commit after each accepted fix round.
- Commit messages must describe the storage-layout boundary work clearly and standalone.

Stop conditions:
- Stop once Packet 1.3 is review-clean, committed, and the public/import-facing storage layout seam is usable across route-state, capture, and handoff.
- Stop and report blocked if the packet cannot close honestly without widening into setup ownership, CLI/compiler work, actual Substrate import, or later sets.
```

---

## Packet 1.4 Prompt — Final Set Proof

```text
/goal Orchestrate Set 1 Packet 1.4: Final Set Proof in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.4 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md.
- Verify live repo truth before changing anything.
- Use the approved set authority at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md
- Treat the packet as one narrow proof seam: run the verification wall and record the bounded residual-default inventory honestly.
- Packets 1.1, 1.2, and 1.3 must already be complete.
- Stay inside Packet 1.4 only.

Hard rules:
- Do not implement, review, or fix Packet 1.4 work in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.4 only.
- Stay inside Packet 1.4 scope.

Packet 1.4 scope:
- Run the full Set 1 verification wall.
- Record pass/fail in the completion notes of the tasks doc.
- Record the bounded residual-default inventory honestly.
- Update docs only for the proof/notes unless a minimal packet-scoped fix is required.
- Expected files:
  - docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md

Out of scope — do NOT touch:
- reopening Packets 1.1–1.3 silently
- Set 2 or Set 3 work
- CLI/compiler/product-shell cleanup
- actual Substrate import execution
- speculative new code beyond the minimal fix needed to resolve a proof failure already inside Set 1 scope

Implementation subagent prompt requirements:
- Begin with `/goal Land Set 1 Packet 1.4: Final Set Proof`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_loader`
  - `cargo test -p handbook-pipeline --test pipeline_compile`
  - `cargo test -p handbook-pipeline --test pipeline_route_resolution`
  - `cargo test -p handbook-pipeline --test pipeline_state_store`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
  - `cargo check --workspace`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `rg -n "core/pipelines|core/profiles|core/runners|core/stages|\.handbook/state|artifacts/handoff/feature_slice" crates/pipeline/src crates/pipeline/tests`
- Require the implementation to:
  - verify Packets 1.1–1.3 landed first
  - run the full verification wall and record exact pass/fail results in the Packet 1.4 completion notes
  - distinguish acceptable retained handbook-product defaults from structural import blockers
  - avoid silently reopening earlier packets; if structural work is still missing, report it as a blocker and cite the packet that must be reopened
  - stop after Packet 1.4 acceptance is met and report touched files, verification run, residual defaults, and any blockers

Review subagent prompt requirements:
- Begin with `/goal Review Set 1 Packet 1.4: Final Set Proof`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to check:
  - whether the verification wall actually ran and was recorded accurately
  - whether the residual-default inventory is honest and bounded
  - whether the packet avoided silently absorbing unfinished structural work from Packets 1.1–1.3
  - whether the completion notes are internally consistent with the spec, plan, tasks, and live verification evidence
- Require severity labels and explicit callouts for incomplete proof, inaccurate recording, or hidden scope widening.

Fix loop:
- If the review is clean, stop and report Packet 1.4 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.4-bounded changes needed to close them.
- Commit accepted fixes before dispatching the next fresh review subagent.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.4 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.4 only.
- Commit after each accepted fix round.
- Commit messages must describe the Set 1 proof work clearly and standalone.

Stop conditions:
- Stop once Packet 1.4 is review-clean, committed, and the tasks doc records both the verification wall and the bounded residual-default inventory honestly.
- Stop and report blocked if verification reveals unfinished structural work from Packets 1.1–1.3 that must be reopened explicitly.
```
