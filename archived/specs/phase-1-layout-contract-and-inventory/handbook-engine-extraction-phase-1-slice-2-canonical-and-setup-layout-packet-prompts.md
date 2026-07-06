# Handbook Engine Extraction Phase 1 Slice 2 Packet Prompts

Task source: [handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-tasks.md](./handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-tasks.md)
Spec source: [handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md](./handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md)
Plan source: [handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md](./handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 1.2 scope.

## Packet 1.2.1 Prompt

```text
/goal Orchestrate Phase 1 Slice 2 Packet 1.2.1: Canonical Artifact Root Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.2.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-tasks.md.
- Verify live repo truth before changing anything, including that Slice 1.1 already froze the separate-layout-types contract and that Slice 1.2 is the next code-adoption seam.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md
- Do not start Packet 1.2.2.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.2.1 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round that changes files.
- Stay inside Packet 1.2.1 scope.

Packet 1.2.1 scope:
- Introduce the compiler-local canonical root layout owner.
- Adopt `canonical_artifacts.rs` onto the canonical layout owner.
- Expected files:
  - crates/compiler/src/layout.rs
  - crates/compiler/src/lib.rs
  - crates/compiler/src/canonical_artifacts.rs
  - crates/compiler/tests/canonical_artifacts_ingest.rs

Out of scope:
- Packet 1.2.2 setup adoption beyond minimal compile-through wiring
- any `.handbook/state/**` ownership adoption
- any `setup.rs` canonical-root refactor beyond what is strictly required for Packet 1.2.1 to compile cleanly
- any `route_state.rs`, `pipeline_capture.rs`, `pipeline_handoff.rs`, `stage_10_feature_spec_provenance.rs`, or `author/**` work
- any CLI/product wording change
- any Slice 1.3 / 1.4 / Phase 2 work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 2 Packet 1.2.1: Canonical Artifact Root Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that the Slice 1.1 contract language already exists before editing.
- Require the subagent to keep this packet bounded to canonical-root ownership and to explicitly preserve:
  - separate layout types rather than one global layout object
  - canonical artifact identity semantics
  - repo-relative path and namespace-dir semantics
  - setup starter-template behavior
  - the deferment of runtime-state ownership to Slice 1.3
- Require the subagent to introduce only the narrowest compiler-local canonical layout owner needed for Packet 1.2.1, preferably in `crates/compiler/src/layout.rs`, and to make `canonical_artifacts.rs` consume it without widening into broader layout-family adoption.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "CanonicalLayout|RepoLayoutRoot|SYSTEM_ROOT_RELATIVE|relative_path|namespace_dir" crates/compiler/src/layout.rs crates/compiler/src/canonical_artifacts.rs crates/compiler/src/lib.rs`
  - `cargo test -p handbook-compiler --test canonical_artifacts_ingest`
  - `cargo check -p handbook-compiler`
- Require the subagent to stop after Packet 1.2.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 2 Packet 1.2.1: Canonical Artifact Root Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 1.2 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - preservation of canonical artifact identity semantics
  - whether `layout.rs` stayed narrow rather than becoming a monolithic layout object
  - whether Packet 1.2.2 setup work leaked in
  - whether any `.handbook/state/**` ownership adoption accidentally started
- Require severity labels and explicit callouts if the layout owner is too broad, if canonical semantics drifted, or if setup adoption leaked beyond compile-through wiring.

Fix loop:
- If the review is clean, stop and report Packet 1.2.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-1.2.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 1.2.1 lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.2.1 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1.2.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.2.1 is review-clean and committed.
- Stop and report blocked if Packet 1.2.1 cannot be completed without widening into Packet 1.2.2, Slice 1.3+, or changing the approved Slice 1.2 spec/plan/tasks.
```

## Packet 1.2.2 Prompt

```text
/goal Orchestrate Phase 1 Slice 2 Packet 1.2.2: Setup Bootstrap Root Adoption in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.2.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-tasks.md.
- Assume Packet 1.2.1 is already landed; verify live repo truth before changing anything.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md
- Do not reopen Packet 1.2.1 except where live repo truth proves a tiny corrective adjustment is strictly required for Packet 1.2.2 to land correctly.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them with `$incremental-implementation`.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 1.2.2 only.
- Commit changes after the implementation lands, and commit again after each accepted fix round.
- Stay inside Packet 1.2.2 scope.

Packet 1.2.2 scope:
- Route setup canonical-root establishment and repair through the canonical layout owner.
- Preserve Slice 1.2 boundaries while integrating canonical and setup ownership.
- Expected files:
  - crates/compiler/src/layout.rs
  - crates/compiler/src/canonical_artifacts.rs
  - crates/compiler/src/setup.rs
  - crates/compiler/src/lib.rs
  - crates/compiler/tests/canonical_artifacts_ingest.rs
  - crates/compiler/tests/setup.rs

Out of scope:
- any `.handbook/state/**` ownership adoption beyond preserving the existing local reset flow
- any new route-state, capture, provenance, handoff, or authoring layout adoption
- changing canonical artifact identity semantics as a side effect
- changing CLI/operator wording or doctor guidance
- any Slice 1.3 / 1.4 / Phase 2 work

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 1 Slice 2 Packet 1.2.2: Setup Bootstrap Root Adoption`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that Packet 1.2.1 already landed and that the canonical layout owner is present before editing.
- Require the subagent to keep this packet bounded to setup-side canonical-root adoption and to explicitly preserve:
  - setup init/refresh/refusal behavior
  - setup starter-template behavior
  - runtime-state reset behavior remaining local and deferred to Slice 1.3
  - canonical artifact semantics already frozen by Packet 1.2.1
- Require the subagent to remove direct setup-owned canonical-root derivation such as `repo_root.join(".handbook")` in favor of canonical-layout accessors, unless live code truth proves an equivalent narrow helper is already present.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report the impacted callers/processes.
- Require targeted verification with:
  - `rg -n "CanonicalLayout|RepoLayoutRoot|join\\(\"\\.handbook\"\\)|reset_state|runtime-state" crates/compiler/src/layout.rs crates/compiler/src/setup.rs`
  - `cargo test -p handbook-compiler --test setup`
  - `cargo test -p handbook-compiler --test canonical_artifacts_ingest`
  - `cargo check -p handbook-compiler`
- Require the subagent to stop after Packet 1.2.2 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 1 Slice 2 Packet 1.2.2: Setup Bootstrap Root Adoption`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 1.2 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether `setup.rs` now consumes the canonical layout owner cleanly
  - whether runtime-state ownership remained local instead of leaking into Slice 1.3 work
  - whether canonical artifact semantics regressed while setup was being adopted
  - whether any broader authoring or stateful-storage adoption leaked in
- Require severity labels and explicit callouts if `.handbook/state/**` ownership adoption leaked in, if direct canonical-root ownership remains duplicated in setup, or if Packet 1.2.1 semantics regressed.

Fix loop:
- If review is clean, stop and report Packet 1.2.2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using `$incremental-implementation`.
- Keep fixes minimal and Packet-1.2.2-bounded.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after Packet 1.2.2 implementation lands cleanly.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.2.2 only.
- Commit after each accepted fix round.
- Commit messages must describe the Packet 1.2.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 1.2.2 is review-clean and committed.
- Stop and report blocked if Packet 1.2.2 requires widening into Slice 1.3+, regressing Packet 1.2.1 semantics, or changing the approved Slice 1.2 spec/plan/tasks.
```
