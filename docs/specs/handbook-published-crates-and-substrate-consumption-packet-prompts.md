# Handbook Published-Crate Readiness + Substrate Consumption Packet Prompts

Task source: [handbook-published-crates-and-substrate-consumption-tasks.md](./handbook-published-crates-and-substrate-consumption-tasks.md)
Spec source: [handbook-published-crates-and-substrate-consumption-spec.md](./handbook-published-crates-and-substrate-consumption-spec.md)
Plan source: [handbook-published-crates-and-substrate-consumption-plan.md](./handbook-published-crates-and-substrate-consumption-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one:
- starts in `/goal`
- requires a fresh **GPT-5.4 high** implementation subagent
- requires that implementation subagent to use `$incremental-implementation`
- requires a fresh **GPT-5.4 high** review subagent
- requires that review subagent to use `$code-review-and-quality`
- requires a fresh **GPT-5.4 high** fix subagent for every review round that finds issues
- requires commit boundaries between implementation, review, and each accepted fix round
- keeps execution bounded to one packet only

Unless a packet explicitly authorizes otherwise, the orchestration session must not implement, review, or fix the packet work itself. It must delegate implementation, review, and fix work to fresh subagents exactly as directed.

Preserve unrelated local edits, especially any incidental dirt in `AGENTS.md` and `CLAUDE.md`. In `system`, run GitNexus detect-changes before every commit and confirm the affected scope matches only the current packet. For any packet that edits production Rust symbols in `system`, run GitNexus impact analysis before editing those symbols and stop if the blast radius is HIGH or CRITICAL.

---

## Packet 1.1 Prompt — Publish Metadata Baseline

```text
/goal Orchestrate Packet 1.1: Publish Metadata Baseline for handbook-engine, handbook-pipeline, and handbook-flow in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md.
- Use these authorities:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-crates-and-substrate-consumption-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-crates-and-substrate-consumption-plan.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md
- Treat this as a metadata-only seam: add the first-wave publication metadata needed for the three crates without widening into dependency versioning, public API narrowing, release automation, real publication, or substrate integration.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill.
- If the review subagent finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- After each accepted fix round that changes files, commit before dispatching the next fresh review subagent.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.1 only.

Packet 1.1 scope:
- Add the agreed first-wave publication metadata to:
  - crates/engine/Cargo.toml
  - crates/pipeline/Cargo.toml
  - crates/flow/Cargo.toml
- `license = "MIT"` is already present; do not treat that as missing work.
- If supporting metadata requires a shared README or doc reference, keep it minimal and tightly scoped.
- Record any intentional first-wave metadata deferrals honestly in the docs if needed.

Out of scope:
- Packet 1.2 dependency versioning
- Lane 3 staged dry-runs / real publication
- Any `lib.rs` public API narrowing
- Any real crates.io publish step
- Any substrate repo changes
- Any AGENTS.md / CLAUDE.md incidental dirt

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 1.1: Publish Metadata Baseline for handbook-engine, handbook-pipeline, and handbook-flow`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,220p' docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`
  - `sed -n '1,220p' docs/specs/handbook-published-crates-and-substrate-consumption-plan.md`
  - `sed -n '1,120p' crates/engine/Cargo.toml`
  - `sed -n '1,120p' crates/pipeline/Cargo.toml`
  - `sed -n '1,120p' crates/flow/Cargo.toml`
  - `cargo package -p handbook-engine --allow-dirty`
- Require the implementation to add only the metadata needed for first-wave publication readiness, stop if a non-metadata blocker appears, and report exact files touched plus the engine packaging result.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 1.1: Publish Metadata Baseline`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require special attention to whether the metadata changes are minimal, honest, publication-relevant, and did not widen into Packet 1.2 or later work.
- Require severity labels on all findings.

Fix loop:
- If review is clean, stop and report Packet 1.1 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation, cite the exact findings, and constrain the fix to Packet 1.1 only.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe publication metadata baseline work.

Stop conditions:
- Stop once Packet 1.1 is review-clean, committed, and any remaining packaging blockers belong strictly to later packets.
- Stop and report blocked if honest completion requires dependency versioning, API narrowing, release automation, or substrate changes.
```

---

## Packet 1.2 Prompt — Versioned Intra-Workspace Dependencies

```text
/goal Orchestrate Packet 1.2: Convert handbook internal dependencies to publishable versioned declarations in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 1.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md.
- Use the spec/plan/tasks trio as the sole authority.
- Treat this as one narrow manifest seam: make `handbook-pipeline` and `handbook-flow` depend on `handbook-engine` in a publishable `version + path` form appropriate for local development and future crates.io publication.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill.
- After implementation completes, commit the implementation before any review round.
- Then spawn a fresh GPT-5.4 subagent on high for review using a prompt that begins with `/goal ` and explicitly uses $code-review-and-quality.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 1.2 only.

Packet 1.2 scope:
- Update only the publishable internal dependency declarations needed so pre-release `cargo package` no longer fails because `handbook-engine` lacks a dependency version in pipeline/flow packaging.
- Treat later crates.io-resolution failure for unpublished `handbook-engine` as a Lane 3 concern, not as an in-scope Packet 1.2 blocker.
- Files expected in scope:
  - crates/pipeline/Cargo.toml
  - crates/flow/Cargo.toml
- Refresh docs only if strictly needed to keep the versioning policy honest.

Out of scope:
- Publish metadata baseline beyond what Packet 1.1 already handled
- Lane 3 staged dry-runs / real publication
- Public API narrowing in any `lib.rs`
- Real crates.io publication
- Any substrate repo changes

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 1.2: Convert handbook internal dependencies to publishable versioned declarations`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,220p' docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`
  - `sed -n '1,220p' docs/specs/handbook-published-crates-and-substrate-consumption-plan.md`
  - `sed -n '1,120p' crates/pipeline/Cargo.toml`
  - `sed -n '1,120p' crates/flow/Cargo.toml`
  - `cargo package -p handbook-pipeline --allow-dirty`
  - `cargo package -p handbook-flow --allow-dirty`
- Require the implementation to adopt the minimal publishable dependency form, rerun both `cargo package` commands after the change, and report the exact before/after failure mode so the session can distinguish a resolved missing-version error from the later crates.io-resolution blocker.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 1.2: Convert handbook internal dependencies to publishable versioned declarations`.
- Tell the subagent to use $code-review-and-quality.
- Require focus on correctness of Cargo manifest semantics, scope discipline, and whether the chosen dependency form is truly publishable without widening into release-contract work.
- Require severity labels on findings.

Fix loop:
- If review is clean, stop and report Packet 1.2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation, constrained to the cited findings and Packet 1.2 only.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe versioned internal dependency hardening.

Stop conditions:
- Stop once Packet 1.2 is review-clean, committed, and `crates/pipeline/Cargo.toml` plus `crates/flow/Cargo.toml` use the minimal publishable dependency form while any remaining `cargo package` failure is limited to later crates.io resolution of unpublished `handbook-engine`.
- Stop and report blocked if honest completion requires API narrowing, release-session dry-runs, or substrate-side changes.
```

---

## Packet 2.1 Prompt — Pipeline Published API Freeze

```text
/goal Orchestrate Packet 2.1: Narrow handbook-pipeline's public Rust surface to the documented first-wave published boundary in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.1 from the published-crate readiness tasks doc.
- Use the spec/plan/tasks trio plus the archived Lane A boundary decision as authority.
- Treat this as one narrow public-surface seam: make `crates/pipeline/src/lib.rs` physically match the documented frozen subset for first-wave publication.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 high implementation subagent using a `/goal` prompt and $incremental-implementation.
- Spawn a fresh GPT-5.4 high review subagent using a `/goal` prompt and $code-review-and-quality after implementation is committed.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before editing any production symbols in `crates/pipeline/src/**`, run GitNexus impact analysis first and report the blast radius. If HIGH or CRITICAL, stop and report before editing.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.1 only.

Packet 2.1 scope:
- Narrow `crates/pipeline/src/lib.rs` so the public Rust surface matches the documented first-wave boundary.
- Refresh nearby authority docs only if needed so the physical API and the docs remain honest.
- Run the required pipeline verification wall after changes.

Out of scope:
- Broader pipeline redesign
- Engine public-surface narrowing
- Flow public-surface work beyond what later packets handle
- Real crates.io publication
- Any substrate repo changes

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 2.1: Narrow handbook-pipeline's public Rust surface to the documented first-wave boundary`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`
  - `sed -n '1,260p' docs/specs/handbook-published-crates-and-substrate-consumption-plan.md`
  - `sed -n '1,220p' docs/specs/archive/phase-6-pipeline-boundary-cleanup/README.md || true`
  - `sed -n '1,220p' crates/pipeline/src/lib.rs`
  - `cargo check --workspace`
  - `cargo test -p handbook-pipeline --test pipeline_catalog`
  - `cargo test -p handbook-pipeline --test pipeline_compile`
  - `cargo test -p handbook-pipeline --test pipeline_capture`
  - `cargo test -p handbook-pipeline --test pipeline_handoff`
- Require GitNexus impact analysis before production edits.
- Require the implementation to keep the seam minimal, rerun the listed verification after edits, and report any doc refresh needed for honesty.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 2.1: Narrow handbook-pipeline's public Rust surface to the documented first-wave boundary`.
- Tell the subagent to use $code-review-and-quality.
- Require special attention to whether the physical API now matches the documented boundary, whether any internal callers broke, whether GitNexus impact analysis was run, and whether scope leaked into broader pipeline redesign.
- Require severity labels.

Fix loop:
- If review is clean, stop and report Packet 2.1 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation constrained to those findings.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe the published API freeze for pipeline.

Stop conditions:
- Stop once Packet 2.1 is review-clean, committed, and the physical pipeline API matches the documented first-wave boundary.
- Stop and report blocked if honest completion requires widening the published contract or redesigning pipeline beyond this packet.
```

---

## Packet 2.2 Prompt — Engine Published Surface Decision

```text
/goal Orchestrate Packet 2.2: Decide and record handbook-engine's first published surface in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.2 from the published-crate readiness tasks doc.
- Use the spec/plan/tasks trio as authority.
- Treat this as one narrow decision seam: either confirm the current engine surface as the accepted first published API, or land the smallest approved engine-freeze follow-on needed before publication.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 high implementation subagent using a `/goal` prompt and $incremental-implementation.
- Spawn a fresh GPT-5.4 high review subagent using a `/goal` prompt and $code-review-and-quality after implementation is committed.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before editing any production symbols in `crates/engine/src/**`, run GitNexus impact analysis first and report the blast radius. If HIGH or CRITICAL, stop and report before editing.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.2 only.

Packet 2.2 scope:
- Determine whether `crates/engine/src/lib.rs` stays as-is for first publication or needs a narrow freeze.
- If no code change is needed, record that decision honestly in the docs.
- If a code change is needed, keep it minimal and tightly scoped to engine publication posture.

Out of scope:
- Pipeline or flow API work
- Real publication
- Substrate repo changes
- Broader engine redesign or architectural transfer

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 2.2: Decide and record handbook-engine's first published surface`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`
  - `sed -n '1,240p' crates/engine/src/lib.rs`
  - `cargo test -p handbook-engine`
- Require GitNexus impact analysis before production edits if any engine symbol changes are proposed.
- Require the subagent to choose the minimal honest path, update the relevant authority docs if needed, and report whether the packet landed as docs-only or code-plus-docs.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 2.2: Decide and record handbook-engine's first published surface`.
- Tell the subagent to use $code-review-and-quality.
- Require special attention to whether the chosen posture is supported by live engine surface truth, whether any code change was actually necessary, and whether scope leaked beyond engine publication posture.
- Require severity labels.

Fix loop:
- If review is clean, stop and report Packet 2.2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation constrained to the findings.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe the engine publication-surface decision.

Stop conditions:
- Stop once Packet 2.2 is review-clean, committed, and the engine published-surface decision is honestly recorded.
- Stop and report blocked if honest completion requires a much broader engine API redesign.
```

---

## Packet 2.3 Prompt — Flow Published-Surface Revalidation

```text
/goal Orchestrate Packet 2.3: Revalidate handbook-flow as a publishable API after manifest/versioning hardening in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 2.3 from the published-crate readiness tasks doc.
- Use the spec/plan/tasks trio plus the flow consumer contract as authority.
- Treat this as one narrow revalidation seam: confirm the cleaned flow consumer contract still matches the live publishable surface after the earlier publish-readiness changes.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 high implementation subagent using $incremental-implementation in a `/goal` prompt.
- Commit implementation before review.
- Spawn a fresh GPT-5.4 high review subagent using $code-review-and-quality in a `/goal` prompt.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before editing any production symbols in `crates/flow/src/**`, run GitNexus impact analysis first and report the blast radius. If HIGH or CRITICAL, stop and report before editing.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 2.3 only.

Packet 2.3 scope:
- Revalidate `crates/flow/src/lib.rs` and the flow consumer contract.
- Refresh the contract doc only if needed for honesty.
- Resolve only publish-surface issues revealed by the packet verification.

Out of scope:
- Reopening the broader Lane B cleanup unless a real regression is found
- Pipeline or engine API work
- Real publication
- Substrate repo changes

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 2.3: Revalidate handbook-flow as a publishable API after manifest/versioning hardening`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `sed -n '1,120p' crates/flow/src/lib.rs`
  - `cargo test -p handbook-flow`
- Require GitNexus impact analysis before production edits if any flow symbol change is proposed.
- Require the implementation to keep changes minimal, refresh docs only if needed, and report whether the packet landed as docs-only or code-plus-docs while keeping any remaining dry-run dependency on published `handbook-engine` explicitly out of scope for this lane.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 2.3: Revalidate handbook-flow as a publishable API`.
- Tell the subagent to use $code-review-and-quality.
- Require special attention to whether the live flow surface still matches the cleaned contract, whether typed semantics remain in boundary and shell wording remains out of boundary, and whether scope leaked beyond revalidation.
- Require severity labels.

Fix loop:
- If review is clean, stop and report Packet 2.3 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation constrained to those findings.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe flow publish-surface revalidation.

Stop conditions:
- Stop once Packet 2.3 is review-clean, committed, and flow remains honest as a publishable API.
- Stop and report blocked if honest completion requires reopening broader flow redesign.
```

---

## Packet 3.1 Prompt — Release Contract + Checklist

```text
/goal Orchestrate Packet 3.1: Record the first-wave release contract and checklist for handbook-engine, handbook-pipeline, and handbook-flow in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.1 from the published-crate readiness tasks doc.
- Use the spec/plan/tasks trio as authority.
- Treat this as one docs-first release seam: define the versioning policy, publish order, dependency pin semantics, and required dry-run evidence before real publication.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 high implementation subagent using $incremental-implementation in a `/goal` prompt.
- Commit implementation before review.
- Spawn a fresh GPT-5.4 high review subagent using $code-review-and-quality in a `/goal` prompt.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before every commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.1 only.

Packet 3.1 scope:
- Record the first-wave release contract and checklist.
- The artifact may live in the current spec/plan/tasks trio or a dedicated release-checklist doc if that is the cleaner narrow shape.
- This packet is planning/checklist work only.

Out of scope:
- Real crates.io publication
- Substrate repo changes
- Broad release automation / CI unless explicitly necessary and approved later

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 3.1: Record the first-wave release contract and checklist`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - `sed -n '1,260p' docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`
  - `sed -n '1,260p' docs/specs/handbook-published-crates-and-substrate-consumption-plan.md`
  - `sed -n '1,240p' docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md`
  - current manifest versions for engine/pipeline/flow
  - latest dry-run / packaging truth from earlier packets
- Require the implementation to record the release order, versioning/pin policy, and exact pre-publish evidence requirements without widening into actual publication.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 3.1: Record the first-wave release contract and checklist`.
- Tell the subagent to use $code-review-and-quality.
- Require focus on whether the release contract is executable, honest, consistent with live manifest truth, and sufficiently specific to drive the staged dry-run / publish sequence in Packet 3.2 without guessing.
- Require severity labels.

Fix loop:
- If review is clean, stop and report Packet 3.1 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation constrained to those findings.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe the first-wave release contract/checklist.

Stop conditions:
- Stop once Packet 3.1 is review-clean, committed, and the release contract is durable enough to execute the staged dry-run / publish sequence in Packet 3.2 without ambiguity.
- Stop and report blocked if honest completion requires making real publication decisions that are not yet approved.
```

---

## Packet 3.2 Prompt — Staged Dry-Run + Real crates.io Publication

```text
/goal Orchestrate Packet 3.2: Execute the staged first-wave release for handbook-engine, handbook-pipeline, and handbook-flow to crates.io in the approved order.

Mission:
- Land only Packet 3.2 from the published-crate readiness tasks doc.
- Use the published-crate readiness spec/plan/tasks trio plus the Packet 3.1 release contract/checklist as authority.
- Treat this as one high-stakes release seam: run the staged dry-run / publish order honestly, including the fact that dependent-crate dry-runs only become meaningful after the published `handbook-engine` version is resolvable from crates.io.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 high implementation subagent using $incremental-implementation in a `/goal` prompt.
- Commit any pre-publish checklist/doc updates before review.
- Spawn a fresh GPT-5.4 high review subagent using $code-review-and-quality in a `/goal` prompt.
- If review finds issues before or after publication, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before every commit in `system`, run GitNexus detect-changes and confirm the affected scope matches Packet 3.2 only.
- If explicit human authorization to run real `cargo publish` is not present in the fresh session, the orchestration agent must stop and report blocked rather than publishing.

Packet 3.2 scope:
- Run `cargo publish --dry-run -p handbook-engine`.
- Publish `handbook-engine`.
- Wait until the published `handbook-engine` version is resolvable from crates.io.
- Then run dependent dry-runs for `handbook-pipeline` and `handbook-flow`, and publish them in the approved order.
- Record exact dry-run and publish evidence needed for downstream consumers.

Out of scope:
- Broad release automation / CI
- Publishing additional crates
- Substrate repo changes (those are later packets)
- API redesign that should have been handled in earlier packets

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 3.2: Execute the staged first-wave release for handbook-engine, handbook-pipeline, and handbook-flow`.
- Tell the subagent to use $incremental-implementation.
- Require live verification with:
  - `git status --short --branch`
  - the Packet 3.1 release contract/checklist
  - `cargo publish --dry-run -p handbook-engine`
- Require the implementation subagent to confirm explicit human authorization to perform real crates.io publication remains present.
- Require the implementation subagent to stop if engine dry-run fails, stop if publication is not explicitly authorized, publish `handbook-engine` first, confirm the chosen engine version is resolvable from crates.io, then run and record:
  - `cargo publish --dry-run -p handbook-pipeline`
  - `cargo publish --dry-run -p handbook-flow`
  - `cargo publish -p handbook-pipeline`
  - `cargo publish -p handbook-flow`
- Require the implementation subagent to report exact dry-run / publish outputs and any index-resolution waiting or retry behavior.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 3.2: Execute the staged first-wave release for handbook-engine, handbook-pipeline, and handbook-flow`.
- Tell the subagent to use $code-review-and-quality.
- Require focus on whether real publication was properly authorized, whether the staged order was followed exactly, whether dependent dry-runs happened only after engine resolution, whether the recorded published versions are accurate, and whether the session stayed inside the first-wave publication seam.
- Require severity labels.

Fix loop:
- If review is clean, stop and report Packet 3.2 complete.
- If review finds issues in publish-adjacent docs/checklist artifacts, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation constrained to those findings.
- Do not republish blindly; if publication itself misfired, stop and report the exact state.

Commit policy:
- Commit any checklist/doc changes before and after publication as needed.
- Commit after each accepted fix round.
- Commit messages must clearly describe the staged first-wave crates.io release evidence.

Stop conditions:
- Stop once Packet 3.2 is review-clean and the staged dry-run / publish evidence is recorded honestly.
- Stop and report blocked if engine dry-run fails, publication is not authorized, the published engine version does not become resolvable for dependent dry-runs, or crates.io behavior contradicts the recorded release contract.
```

---

## Packet 4.1 Prompt — Downstream Dependency Wiring

```text
/goal Orchestrate Packet 4.1: Replace path/workspace-member handbook adoption with published-crate dependency wiring in /Users/spensermcconnell/__Active_Code/atomize-hq/substrate.

Mission:
- Land only Packet 4.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md.
- Use the `system` spec/plan/tasks trio and the recorded first-wave published versions as authority.
- Treat this as one narrow downstream manifest seam: switch Substrate to published crates.io dependencies for `handbook-engine`, `handbook-pipeline`, and `handbook-flow`.
- Obey any AGENTS/CLAUDE instructions present in the target `substrate` repo.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 high implementation subagent using $incremental-implementation in a `/goal` prompt.
- Commit implementation before review.
- Spawn a fresh GPT-5.4 high review subagent using $code-review-and-quality in a `/goal` prompt.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before every commit in the target repo, obey that repo's change-detection / review requirements and confirm the affected scope matches Packet 4.1 only.

Packet 4.1 scope:
- Update only the relevant Substrate manifests so the first-wave seam consumes published crates.io versions instead of sibling path dependencies.
- Keep the packet confined to manifest/dependency wiring.

Out of scope:
- Downstream adapter/call-site changes beyond what is strictly required for manifest resolution
- Full downstream verification wall beyond the packet acceptance checks
- Any new publication work back in `system`

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 4.1: Replace path/workspace-member handbook adoption with published-crate dependency wiring in substrate`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to start in `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate` and first read any repo-local AGENTS/CLAUDE instructions.
- Require live verification with:
  - `git status --short --branch`
  - relevant `Cargo.toml` files in substrate root and affected members
  - `cargo tree -p handbook-engine`
  - `cargo tree -p handbook-pipeline`
  - `cargo tree -p handbook-flow`
- Require the implementation to update only the dependency wiring needed for published-crate consumption and to report exact manifest files touched.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 4.1: Replace path/workspace-member handbook adoption with published-crate dependency wiring in substrate`.
- Tell the subagent to use $code-review-and-quality.
- Require focus on correctness of dependency source selection, scope discipline, and whether any path dependency fallback remains in the first-wave seam.
- Require severity labels.

Fix loop:
- If review is clean, stop and report Packet 4.1 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation constrained to those findings.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe published-crate dependency wiring in substrate.

Stop conditions:
- Stop once Packet 4.1 is review-clean, committed, and the downstream manifests depend on published crate versions rather than path dependencies for this seam.
- Stop and report blocked if honest completion requires adapter/call-site work that belongs to later packets.
```

---

## Packet 4.2 Prompt — Downstream Consumer Adaptation

```text
/goal Orchestrate Packet 4.2: Update Substrate call sites/adapters to consume the published handbook crate boundaries in /Users/spensermcconnell/__Active_Code/atomize-hq/substrate.

Mission:
- Land only Packet 4.2 from the published-crate readiness tasks doc.
- Use the `system` spec/plan/tasks trio, the published versions, and the already-landed Packet 4.1 manifest wiring as authority.
- Treat this as one narrow downstream code seam: adapt only the Substrate call sites/adapters needed to consume the published boundaries honestly.
- Obey any AGENTS/CLAUDE instructions present in the target `substrate` repo.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 high implementation subagent using $incremental-implementation in a `/goal` prompt.
- Commit implementation before review.
- Spawn a fresh GPT-5.4 high review subagent using $code-review-and-quality in a `/goal` prompt.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before editing production symbols in `substrate`, obey that repo's impact-analysis / safety requirements first.
- Before every commit, obey the target repo's change-detection requirements and confirm the affected scope matches Packet 4.2 only.

Packet 4.2 scope:
- Update only the downstream call sites/adapters needed to consume the published handbook crate APIs.
- Preserve caller-owned rendering / wording responsibilities in Substrate.
- Do not widen into unrelated product redesign.

Out of scope:
- Further publication work in `system`
- Broad substrate refactors beyond the affected call sites/adapters
- Full downstream verification wall beyond the packet acceptance checks

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 4.2: Update Substrate call sites/adapters to consume the published handbook crate boundaries`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to start in `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate` and read repo-local instructions first.
- Require live verification with:
  - `git status --short --branch`
  - the specific manifests and source files affected by Packet 4.1
  - `cargo check --workspace`
- Require the implementation to touch only the discovered adapter/call-site files needed for honest published-crate consumption and to report exact files changed.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 4.2: Update Substrate call sites/adapters to consume the published handbook crate boundaries`.
- Tell the subagent to use $code-review-and-quality.
- Require focus on whether Substrate now consumes only the published boundary, whether rendering ownership remains in the caller where required, and whether the adaptation stayed narrowly scoped.
- Require severity labels.

Fix loop:
- If review is clean, stop and report Packet 4.2 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation constrained to those findings.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe downstream published-crate consumer adaptation.

Stop conditions:
- Stop once Packet 4.2 is review-clean, committed, and only the minimal downstream adapters/call sites have been updated.
- Stop and report blocked if honest completion requires a broader substrate redesign.
```

---

## Packet 4.3 Prompt — Substrate Verification Wall

```text
/goal Orchestrate Packet 4.3: Pass the full Substrate verification wall against published handbook crates in /Users/spensermcconnell/__Active_Code/atomize-hq/substrate.

Mission:
- Land only Packet 4.3 from the published-crate readiness tasks doc.
- Use the `system` spec/plan/tasks trio plus the landed Packet 4.1 / 4.2 downstream changes as authority.
- Treat this as one downstream verification seam: prove that Substrate builds, lints, tests, and resolves dependency trees cleanly against the published handbook crate versions without path fallbacks.
- Obey any AGENTS/CLAUDE instructions present in the target `substrate` repo.

Hard rules:
- Do not implement, review, or fix the packet in the orchestration session yourself.
- Spawn a fresh GPT-5.4 high implementation subagent using $incremental-implementation in a `/goal` prompt.
- Commit any verification-note or minimal fix work before review.
- Spawn a fresh GPT-5.4 high review subagent using $code-review-and-quality in a `/goal` prompt.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation.
- Before every commit, obey the target repo's change-detection requirements and confirm the affected scope matches Packet 4.3 only.

Packet 4.3 scope:
- Run and record the full downstream verification wall:
  - `cargo check --workspace`
  - `cargo fmt --all -- --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo tree -p handbook-engine`
  - `cargo tree -p handbook-pipeline`
  - `cargo tree -p handbook-flow`
- Resolve only the minimal downstream issues needed so the published-consumption wall passes honestly.

Out of scope:
- New publication work in `system`
- Broad substrate redesign
- Replacing published-crate consumption with path fallbacks

Implementation subagent prompt requirements:
- Begin with `/goal Land Packet 4.3: Pass the full Substrate verification wall against published handbook crates`.
- Tell the subagent to use $incremental-implementation.
- Require the subagent to start in `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate` and read repo-local instructions first.
- Require live verification with the full wall listed above.
- Require the implementation to keep fixes minimal, report exact failing commands before changes and exact pass/fail results after changes, and explicitly confirm whether any path fallback remains.

Review subagent prompt requirements:
- Begin with `/goal Review Packet 4.3: Pass the full Substrate verification wall against published handbook crates`.
- Tell the subagent to use $code-review-and-quality.
- Require focus on whether the verification wall truly passed, whether fixes stayed minimal and bounded, and whether published-crate consumption remained intact without path fallbacks.
- Require severity labels.

Fix loop:
- If review is clean, stop and report Packet 4.3 complete.
- If review finds issues, spawn a fresh GPT-5.4 high fix subagent using $incremental-implementation constrained to those findings.
- Commit accepted fixes before re-review.

Commit policy:
- Commit after implementation if clean.
- Commit after each accepted fix round.
- Commit messages must clearly describe downstream verification-wall closure against published handbook crates.

Stop conditions:
- Stop once Packet 4.3 is review-clean, committed, and the full downstream verification wall passes against published crate versions.
- Stop and report blocked if honest completion requires falling back to path dependencies or widening into broad substrate redesign.
```
