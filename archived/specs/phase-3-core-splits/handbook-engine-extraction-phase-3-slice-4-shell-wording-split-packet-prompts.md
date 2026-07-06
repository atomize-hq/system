# Handbook Engine Extraction Phase 3 Slice 4 Packet Prompts

Task source: [handbook-engine-extraction-phase-3-slice-4-shell-wording-split-tasks.md](./handbook-engine-extraction-phase-3-slice-4-shell-wording-split-tasks.md)
Spec source: [handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md](./handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md)
Plan source: [handbook-engine-extraction-phase-3-slice-4-shell-wording-split-plan.md](./handbook-engine-extraction-phase-3-slice-4-shell-wording-split-plan.md)

These prompts are ready to paste into fresh orchestration sessions. Each one starts in `/goal`, requires fresh `GPT-5.4` `high` subagents, uses `$incremental-implementation` for implementation/fix rounds, uses `$code-review-and-quality` for review rounds, preserves commit boundaries between implementation, review, and fix work, and keeps packet execution bounded to the approved Slice 3.4 scope.

## Packet 3.4.1 Prompt

```text
/goal Orchestrate Phase 3 Slice 4 Packet 3.4.1: Setup And Readiness Shell Separation in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.4.1 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-tasks.md.
- Verify live repo truth before changing anything, including that `crates/compiler/src/setup.rs` still mixes reusable setup planning/mutation logic with handbook-specific command wording, rerun guidance, and operator-facing refusal prose.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-plan.md
- Stay inside Packet 3.4.1 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.4.1 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 3.4.1 scope.

Packet 3.4.1 scope:
- Introduce a shell-owned setup wording boundary for handbook commands, rerun guidance, and mutation/refusal prose.
- Thin setup outcome/refusal construction around reusable setup decisions.
- Keep mode resolution, starter-action planning, write-target validation, invalid-root repair, runtime-state reset, and disposition classification in reusable setup code.
- Preserve the current public `SetupOutcome` / `SetupRefusal` behavior and current CLI-visible setup behavior.
- Expected files:
  - crates/compiler/src/setup.rs
  - crates/compiler/src/setup_shell.rs
  - optionally crates/compiler/src/lib.rs if narrow compile-through wiring is required
  - crates/compiler/tests/setup.rs
  - crates/cli/tests/cli_surface.rs

Out of scope:
- Packet 3.4.2 doctor wording extraction, refusal/rendering extraction, or shared next-safe-action/subject rendering cleanup beyond tiny compile-through adjustments strictly required by the setup split
- changes to doctor baseline classification, doctor JSON contract fields, or doctor checklist semantics
- changes to `crates/compiler/src/refusal.rs`, `crates/compiler/src/rendering/shared.rs`, or `crates/cli/src/main.rs` beyond narrow fallout needed to preserve current setup behavior
- changes to canonical `.handbook/` layout, setup-owned starter-file ownership, or baseline-validation rules
- changes to handbook command surface, setup outcome/refusal semantics, or CLI output shape
- Phase 4 crate moves or Phase 5 CLI command-family extraction
- broad wording redesign or adjacent cleanup outside Packet 3.4.1

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 3 Slice 4 Packet 3.4.1: Setup And Readiness Shell Separation`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that `crates/compiler/src/setup.rs` still owns mixed responsibilities across:
  - mode resolution and request validation
  - starter-action planning and mutation sequencing
  - invalid-root repair and runtime-state reset wiring
  - handbook command wording, rerun guidance, next-safe-action strings, and operator-facing refusal/mutation prose
- Require one setup shell owner, preferably `crates/compiler/src/setup_shell.rs`, for handbook-specific wording such as:
  - setup command strings like `handbook setup`, `handbook setup refresh`, and `handbook doctor`
  - rerun guidance such as "repair the blocked target and rerun ..."
  - setup refusal summary/recovery copy where the split is already clear
  - setup next-safe-action wording and other operator-facing setup prose
- Require reusable setup behavior to remain in `setup.rs`, including:
  - `resolve_mode`
  - `validate_request`
  - starter-action planning and mutation sequencing
  - invalid-root repair
  - runtime-state reset planning/application wiring
  - setup disposition classification
- Require the implementation to preserve the current public `SetupOutcome` / `SetupRefusal` behavior and current CLI setup output semantics.
- Require any new helper boundary to remain semantic-input -> handbook-copy-output only; do not move setup mutation mechanics into CLI-only code.
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n "setup_next_safe_action|setup_mutation_refusal|setup_refusal|handbook setup|handbook doctor|repair the blocked target" crates/compiler/src/setup*.rs crates/cli/src/main.rs`
  - `cargo test -p handbook-compiler --test setup`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo check -p handbook-compiler`
  - `cargo check -p handbook-cli`
- Require the subagent to stop after Packet 3.4.1 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 3 Slice 4 Packet 3.4.1: Setup And Readiness Shell Separation`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 3.4 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether reusable setup planning/mutation/disposition behavior remained in reusable setup code
  - whether handbook-specific setup wording now lives behind one clear shell boundary
  - whether `SetupOutcome` / `SetupRefusal` behavior and current CLI setup output stayed stable
  - whether Packet 3.4.2 doctor/refusal/rendering work leaked into Packet 3.4.1
  - whether Phase 4 reusable-setup-for-pipeline work or Phase 5 CLI extraction leaked into the packet
- Require severity labels and explicit callouts if reusable setup logic still depends on rendered handbook strings, if public setup behavior drifted, or if doctor/refusal/rendering cleanup leaked into the packet.

Fix loop:
- If the review is clean, stop and report Packet 3.4.1 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.4.1-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.4.1 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.4.1 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 3.4.1 change clearly and standalone.

Stop conditions:
- Stop once Packet 3.4.1 is review-clean and committed.
- Stop and report blocked if Packet 3.4.1 cannot be completed without widening into Packet 3.4.2 doctor/refusal/rendering work, changing the approved Slice 3.4 spec/plan/tasks, or spilling into canonical-layout redesign, public setup semantics changes, Phase 4 crate work, or Phase 5 CLI extraction.
```

## Packet 3.4.2 Prompt

```text
/goal Orchestrate Phase 3 Slice 4 Packet 3.4.2: Doctor And Refusal Operator Wording Separation in /Users/spensermcconnell/__Active_Code/system.

Mission:
- Land only Packet 3.4.2 from /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-tasks.md.
- Verify live repo truth before changing anything, including that Packet 3.4.1 already separated setup wording ownership and that the remaining approved Slice 3.4 seam is doctor wording plus shared refusal/rendering wording separation.
- Use the slice spec and plan at:
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md
  - /Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-engine-extraction-phase-3-slice-4-shell-wording-split-plan.md
- Stay inside Packet 3.4.2 only.

Hard rules:
- Spawn a fresh GPT-5.4 subagent on high for implementation.
- The implementation subagent prompt must begin with `/goal ` and must explicitly use the $incremental-implementation skill from /Users/spensermcconnell/.agents/skills/incremental-implementation/SKILL.md.
- After implementation completes, spawn a fresh GPT-5.4 subagent on high for review.
- The review subagent prompt must begin with `/goal ` and must explicitly use the $code-review-and-quality skill from /Users/spensermcconnell/.agents/skills/code-review-and-quality/SKILL.md.
- If review finds issues, spawn a fresh GPT-5.4 subagent on high to fix them.
- Every fix subagent prompt must begin with `/goal ` and must explicitly use $incremental-implementation.
- Before editing any function, method, or other production symbol, run GitNexus impact analysis on the touched symbol(s) and report the blast radius. If GitNexus reports a stale index, refresh it first.
- Before every commit, run GitNexus detect-changes and verify the affected scope matches Packet 3.4.2 only.
- Commit after the implementation round before sending the change to review.
- Commit again after each accepted fix round that changes files, before re-reviewing.
- Do not amend away prior packet-round commits during the loop.
- Stay inside Packet 3.4.2 scope.

Packet 3.4.2 scope:
- Extract doctor artifact-label and handbook author-command wording into shell-owned helpers.
- Isolate shared next-safe-action and subject wording from reusable refusal/readiness models.
- Keep doctor baseline/checklist classification typed and reusable.
- Keep `RefusalCategory`, `SubjectRef`, and `NextSafeAction` semantic and reusable rather than turning them into full handbook sentences.
- Preserve current doctor JSON contract, current compiler behavior, and current CLI-visible doctor/generate/inspect/setup recovery wording behavior.
- Expected files:
  - crates/compiler/src/doctor.rs
  - crates/compiler/src/doctor_shell.rs
  - crates/compiler/src/refusal.rs
  - crates/compiler/src/rendering/shared.rs
  - optionally crates/compiler/src/lib.rs if narrow compile-through wiring is required
  - crates/compiler/tests/doctor.rs
  - crates/compiler/tests/refusal_mapping.rs
  - crates/compiler/tests/rendering_surface.rs
  - crates/cli/tests/cli_surface.rs
  - optionally crates/cli/tests/help_drift_guard.rs if wording fallout reaches documented help/tone rails

Out of scope:
- reopening Packet 3.4.1 setup wording work beyond tiny compile-through adjustments strictly required by shared rendering changes
- changes to setup planning/mutation semantics, canonical `.handbook/` layout, or setup-owned starter-file rules
- changes to doctor baseline-state semantics, blocker-selection logic, or doctor JSON field set
- changing `RefusalCategory`, `SubjectRef`, or `NextSafeAction` semantics into new public contracts
- changes to CLI tone/output policy beyond the minimum extraction needed to preserve current behavior
- Phase 4 crate moves or Phase 5 CLI command-family extraction
- broad authoring, pipeline, or packet-body rendering cleanup outside Packet 3.4.2

Implementation subagent prompt requirements:
- Begin with `/goal Land Phase 3 Slice 4 Packet 3.4.2: Doctor And Refusal Operator Wording Separation`.
- Tell the subagent to use $incremental-implementation.
- Require live verification that the remaining mixed ownership is now centered in:
  - `doctor_artifact_label` and `doctor_author_command` inside `crates/compiler/src/doctor.rs`
  - `render_next_safe_action_value` and `render_subject_ref` inside `crates/compiler/src/rendering/shared.rs`
  - any remaining handbook-facing recovery wording that is still mixed into reusable readiness/refusal logic
- Require one doctor shell owner, preferably `crates/compiler/src/doctor_shell.rs`, for:
  - doctor artifact labels such as `CHARTER`, `PROJECT_CONTEXT`, and `ENVIRONMENT_INVENTORY`
  - handbook author-command wording such as `run \`handbook author charter\``
  - other doctor-facing operator wording that does not belong in reusable baseline/checklist logic
- Require a shell-oriented recovery-render boundary for:
  - `render_next_safe_action_value`
  - `render_subject_ref`
  - any directly related handbook-facing recovery text that is currently mixed into reusable readiness/refusal rendering
- Require `doctor.rs` to keep baseline/checklist classification and next-safe-action selection typed and reusable.
- Require `refusal.rs` to stay semantic and reusable; do not convert its model layer into a bag of full handbook sentences.
- Require preservation of:
  - current doctor JSON contract fields and meanings
  - current checklist status classification and next-safe-action semantics
  - current generate/inspect/doctor/setup recovery wording behavior at the public surface
  - current compiler and CLI regression behavior unless live repo truth proves a tiny compile-through adjustment is strictly required
- Require GitNexus impact analysis before editing touched production symbols and require the subagent to report impacted callers/processes.
- Require targeted verification with:
  - `rg -n "doctor_artifact_label|doctor_author_command|render_next_safe_action_value|render_subject_ref|NextSafeAction|SubjectRef" crates/compiler/src/doctor.rs crates/compiler/src/doctor_shell.rs crates/compiler/src/refusal.rs crates/compiler/src/rendering/shared.rs crates/cli/src/main.rs`
  - `cargo test -p handbook-compiler --test doctor`
  - `cargo test -p handbook-compiler --test refusal_mapping`
  - `cargo test -p handbook-compiler --test rendering_surface`
  - `cargo test -p handbook-cli --test cli_surface`
  - `cargo test -p handbook-cli --test help_drift_guard`
  - `cargo check -p handbook-compiler`
  - `cargo check -p handbook-cli`
- Require the subagent to stop after Packet 3.4.2 acceptance is met and report touched files, impact-analysis results, verification run, and residual risks.

Review subagent prompt requirements:
- Begin with `/goal Review Phase 3 Slice 4 Packet 3.4.2: Doctor And Refusal Operator Wording Separation`.
- Tell the subagent to use $code-review-and-quality.
- Require findings-first review across correctness, readability, architecture, security, and performance.
- Require the reviewer to review the packet against the Slice 3.4 spec, plan, tasks, and verification evidence.
- Require special attention to:
  - whether doctor baseline/checklist classification stayed typed and reusable
  - whether doctor artifact-label / author-command copy now lives behind one clear shell owner
  - whether shared next-safe-action and subject wording are now isolated from reusable refusal/readiness logic
  - whether `RefusalCategory`, `SubjectRef`, and `NextSafeAction` remained semantic and stable
  - whether current doctor JSON contract and CLI-visible recovery behavior regressed
  - whether Packet 3.4.1 setup work, Phase 4 crate work, or Phase 5 CLI extraction leaked into the packet
- Require severity labels and explicit callouts if reusable model logic still owns handbook strings, if doctor JSON or CLI recovery behavior drifted, or if adjacent slice work leaked into the packet.

Fix loop:
- If the review is clean, stop and report Packet 3.4.2 complete.
- If the review finds issues, spawn one fresh GPT-5.4 high fix subagent per review round using `$incremental-implementation`.
- The fix prompt must cite the exact review findings and require only the minimal Packet-3.4.2-bounded changes needed to close them.
- Re-run GitNexus impact analysis if new production symbols are touched.
- Re-run the bounded verification after each fix round.
- Re-run a fresh review subagent after fixes.

Commit policy:
- Commit once after implementation if Packet 3.4.2 lands cleanly, before review.
- Before each commit, run GitNexus detect-changes and confirm the affected scope matches Packet 3.4.2 only.
- Commit after each accepted fix round, before re-review.
- Commit messages must describe the Packet 3.4.2 change clearly and standalone.

Stop conditions:
- Stop once Packet 3.4.2 is review-clean and committed.
- Stop and report blocked if Packet 3.4.2 cannot be completed without reopening Packet 3.4.1 beyond tiny compile-through adjustments, changing the approved Slice 3.4 spec/plan/tasks, or spilling into setup semantics changes, Phase 4 crate work, Phase 5 CLI extraction, or broader wording redesign.
```
