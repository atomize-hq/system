# Plan: Handbook Engine Extraction Phase 3 Slice 4 (Slice 3.4) - Setup, Doctor, And Refusal Shell Split

## Objective

Split handbook-specific recovery wording and operator-facing copy away from reusable setup, doctor, and refusal/readiness logic inside the current compiler crate, while preserving the existing public `handbook setup`, `handbook doctor`, generate, and inspect behavior.

Spec reference: [handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md](./handbook-engine-extraction-phase-3-slice-4-shell-wording-split-spec.md)

## Major Modules

1. Reusable setup planning and mutation layer
   - `crates/compiler/src/setup.rs`
   - owns mode resolution, request validation, starter-action planning, mutation sequencing, invalid-root repair, and disposition classification

2. Setup shell wording layer
   - proposed as `crates/compiler/src/setup_shell.rs`
   - owns handbook command wording, rerun guidance, next-safe-action wording, and setup refusal/mutation prose that does not belong in reusable setup logic

3. Reusable doctor baseline and checklist layer
   - `crates/compiler/src/doctor.rs`
   - owns baseline classification, checklist decision logic, blocker selection, and typed next-safe-action selection

4. Doctor / recovery shell wording layer
   - proposed as `crates/compiler/src/doctor_shell.rs` plus a small shared recovery-render helper if needed
   - owns artifact labels, handbook author-command wording, shared next-safe-action wording, subject wording, and related operator-facing recovery copy that reusable readiness logic should not own

5. Shared semantic recovery model
   - `crates/compiler/src/refusal.rs`
   - stays as the semantic category/subject/action model used by reusable logic and shell renderers

6. Public caller and regression layer
   - `crates/cli/src/main.rs`
   - `crates/compiler/tests/setup.rs`
   - `crates/compiler/tests/doctor.rs`
   - `crates/compiler/tests/refusal_mapping.rs`
   - `crates/compiler/tests/rendering_surface.rs`
   - `crates/cli/tests/cli_surface.rs`
   - `crates/cli/tests/help_drift_guard.rs`
   - must preserve current public behavior while internal wording ownership moves

## Dependencies And Order

### Prerequisite: Freeze the current setup/doctor/refusal public contract and wording authority

Why first:

- Slice 3.4 is internal ownership cleanup only
- setup/doctor output already has broad CLI regression coverage and doc-backed wording expectations
- later packets should move wording owners without changing outcome categories, next-safe-action semantics, checklist meaning, or documented surface behavior

Output:

- one frozen statement that setup/doctor/generate/inspect public behavior stays stable
- one frozen wording authority grounded in `docs/CLI_TONE_RULES.md`, `docs/CLI_OUTPUT_ANATOMY.md`, and current CLI regression coverage

### Packet 3.4.1: Setup and readiness shell separation

Why first:

- setup is the smallest contained Slice 3.4 surface with a clear split between reusable mutation logic and handbook-specific wording
- Phase 4 later expects reusable parts of setup to move toward pipeline ownership, so separating wording now reduces later crate-move drag
- proving the setup shell split first establishes the local pattern for doctor/refusal wording extraction

Output:

- one setup shell owner for handbook command wording, rerun guidance, next-safe-action wording, and setup refusal prose
- a thinner `setup.rs` that keeps planning/mutation/disposition behavior while delegating operator-facing wording where the split is already clear
- preserved public setup behavior under compiler and CLI regressions

### Packet 3.4.2: Doctor and refusal operator wording separation

Why second:

- doctor/refusal wording touches broader shared rendering helpers and more public surfaces than setup does
- shared next-safe-action and subject rendering are safer to extract after the setup split pattern is already proven locally
- doctor baseline classification should wrap a stable shell wording boundary, not migrate simultaneously with setup wording work

Output:

- one doctor shell owner for artifact-label and author-command wording
- one shell-oriented recovery-render owner for next-safe-action and subject wording, or equivalent extraction, that keeps semantic refusal/readiness models free of handbook copy
- preserved doctor/generate/inspect/setup public behavior after the internal split

## Risks And Mitigations

### Risk: the split changes public setup/doctor/CLI behavior

Mitigation:

- preserve current public compiler structs and CLI output first
- use setup, doctor, refusal-mapping, rendering-surface, CLI-surface, and help-drift rails as required verification
- treat contract changes as out of scope unless the spec changes first

### Risk: reusable logic still depends on rendered handbook strings after the split

Mitigation:

- keep setup/doctor decisions typed and render handbook copy only at shell boundaries
- use ownership scans after each packet to confirm inline handbook wording is concentrated in shell owners
- keep `refusal.rs` semantic rather than sentence-oriented

### Risk: Packet 3.4.2 widens into Phase 5 CLI extraction or broader wording redesign

Mitigation:

- keep the work scoped to compiler-side wording ownership and the minimum CLI fallout needed to preserve behavior
- defer command-family extraction in `main.rs`, help redesign, and tone-rule rewrites to later approved slices

### Risk: shared recovery rendering becomes duplicated or cyclic across setup, doctor, and rendering helpers

Mitigation:

- choose one shared shell-oriented recovery-render owner and route handbook next-safe-action / subject wording through it
- keep that helper semantic-input -> handbook-copy-output only; do not move core decision logic into it

### Risk: setup shell separation accidentally weakens mutation safety or invalid-root repair rails

Mitigation:

- keep mutation planning, write-target validation, invalid-root repair, and runtime-state reset logic in reusable setup code
- prove setup behavior with focused compiler and CLI rails before touching doctor/refusal helpers

## Parallel Vs Sequential

Sequential:

- freeze public contracts before any extraction
- setup shell split before doctor/refusal shell split
- shared recovery-render extraction before final verification wall

Parallel opportunities after Packet 3.4.1 starts:

- compiler-side setup regression updates and CLI setup-surface assertions can proceed in parallel once the setup shell owner is explicit
- after the shared recovery-render boundary is explicit, doctor regression updates and refusal/rendering regression updates can proceed in parallel as long as they do not reopen setup ownership

## Verification Checkpoints

### Checkpoint 1: Setup behavior is stable after wording moves

```bash
cargo test -p handbook-compiler --test setup
cargo test -p handbook-cli --test cli_surface
```

### Checkpoint 2: Doctor and shared recovery rendering are stable after wording extraction

```bash
cargo test -p handbook-compiler --test doctor
cargo test -p handbook-compiler --test refusal_mapping
cargo test -p handbook-compiler --test rendering_surface
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
```

### Wording ownership spot-check

```bash
rg -n "setup_next_safe_action|setup_mutation_refusal|setup_refusal|handbook setup|handbook doctor|repair the blocked target|doctor_artifact_label|doctor_author_command|render_next_safe_action_value|render_subject_ref" crates/compiler/src/setup*.rs crates/compiler/src/doctor*.rs crates/compiler/src/refusal.rs crates/compiler/src/rendering/shared.rs crates/cli/src/main.rs
```

### Typed-model spot-check

```bash
rg -n "NextSafeAction|RefusalCategory|SubjectRef|DoctorBaselineStatus|DoctorArtifactStatus|SetupDisposition|SetupRefusalKind" crates/compiler/src/setup.rs crates/compiler/src/doctor.rs crates/compiler/src/refusal.rs crates/compiler/src/rendering/shared.rs
```

### Final checkpoint

```bash
cargo test -p handbook-compiler --test setup && cargo test -p handbook-compiler --test doctor && cargo test -p handbook-compiler --test refusal_mapping && cargo test -p handbook-compiler --test rendering_surface && cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard
cargo check --workspace
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- setup planning/mutation logic is clearly separated from handbook command wording and rerun prose
- doctor baseline/checklist logic is clearly separated from handbook artifact-label / author-command copy
- shared next-safe-action and subject wording are isolated behind a shell-oriented render boundary
- public setup/doctor/generate/inspect behavior remains stable under focused regressions
- no Phase 4 crate work, Phase 5 CLI extraction, or broader wording redesign leaked in
