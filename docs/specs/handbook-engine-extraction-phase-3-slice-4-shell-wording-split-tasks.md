# Tasks: Handbook Engine Extraction Phase 3 Slice 4 (Slice 3.4) - Setup, Doctor, And Refusal Shell Split

Plan reference: [handbook-engine-extraction-phase-3-slice-4-shell-wording-split-plan.md](./handbook-engine-extraction-phase-3-slice-4-shell-wording-split-plan.md)

## Prerequisite: the public setup/doctor/refusal contract and wording authority stay frozen

Slice 3.4 is internal ownership cleanup only. It must preserve the current public behavior for `handbook setup`, `handbook doctor`, generate, and inspect while moving handbook-specific recovery wording behind clearer shell boundaries.

- Slice 3.4 must separate reusable readiness/setup/doctor logic from handbook-specific recovery wording, not redesign CLI semantics or output shape.

## Packet 3.4.1: Setup And Readiness Shell Separation

- [ ] Task: Introduce a shell-owned setup wording boundary for handbook commands, rerun guidance, and mutation/refusal prose
  - Acceptance: setup planning and mutation logic no longer owns inline handbook command strings or rerun prose where the shell boundary is already clear; a local shell helper owns that wording while current `SetupOutcome` / `SetupRefusal` behavior remains stable.
  - Verify: `cargo test -p handbook-compiler --test setup && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/setup.rs`, `crates/compiler/src/setup_shell.rs`, `crates/compiler/tests/setup.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Thin setup outcome/refusal construction around reusable setup decisions
  - Acceptance: mode resolution, starter-action planning, write-target validation, invalid-root repair, runtime-state reset, and disposition classification remain in reusable setup code, while shell adapters translate those typed decisions into the current handbook-facing next-safe-action and summary strings.
  - Verify: `cargo test -p handbook-compiler --test setup && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/setup.rs`, `crates/compiler/src/setup_shell.rs`, `crates/compiler/tests/setup.rs`, `crates/cli/tests/cli_surface.rs`

## Packet 3.4.2: Doctor And Refusal Operator Wording Separation

- [ ] Task: Extract doctor artifact-label and handbook author-command wording into shell-owned helpers
  - Acceptance: doctor baseline/checklist classification no longer hardcodes uppercase artifact labels or handbook author-command copy inline; current doctor report and CLI checklist behavior remain stable through a shell-owned wording boundary.
  - Verify: `cargo test -p handbook-compiler --test doctor && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/doctor.rs`, `crates/compiler/src/doctor_shell.rs`, `crates/compiler/tests/doctor.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Isolate shared next-safe-action and subject wording from reusable refusal/readiness models
  - Acceptance: shared handbook recovery wording, including `NEXT SAFE ACTION` and subject rendering, moves behind a shell-oriented render helper instead of remaining mixed into reusable readiness/refusal logic, while `RefusalCategory`, `SubjectRef`, and `NextSafeAction` semantics stay behavior-stable.
  - Verify: `cargo test -p handbook-compiler --test refusal_mapping && cargo test -p handbook-compiler --test rendering_surface && cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
  - Files: `crates/compiler/src/refusal.rs`, `crates/compiler/src/rendering/shared.rs`, `crates/compiler/src/doctor.rs`, `crates/compiler/src/doctor_shell.rs`, `crates/compiler/tests/refusal_mapping.rs`, `crates/compiler/tests/rendering_surface.rs`, `crates/cli/tests/cli_surface.rs`, `crates/cli/tests/help_drift_guard.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 3.4 verification wall after both packets land
  - Acceptance: reusable setup/doctor/refusal logic is separated from handbook-specific recovery wording where the seam is clear, public compiler and CLI behavior remain stable, and workspace checks pass without adjacent-slice leakage.
  - Verify: `cargo test -p handbook-compiler --test setup && cargo test -p handbook-compiler --test doctor && cargo test -p handbook-compiler --test refusal_mapping && cargo test -p handbook-compiler --test rendering_surface && cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard && cargo check --workspace && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 3.4 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
