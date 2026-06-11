# Spec: Handbook Engine Extraction Phase 3 Slice 4 (Slice 3.4) - Setup, Doctor, And Refusal Shell Split

## Assumptions

1. Phase 3 Slices 3.1 through 3.3 are complete, so the repo already accepts the Phase 3 pattern of separating reusable internal logic from product-shell behavior inside the current compiler crate before any Phase 4 crate move begins.
2. Phase 1 Slice 2 is complete, so canonical `.handbook/` root ownership, setup-owned starter-file layout, and setup init/refresh behavior are already approved contracts; Slice 3.4 must not reopen canonical layout or scaffold ownership rules.
3. Live `crates/compiler/src/setup.rs` still mixes reusable setup planning/mutation logic with handbook-specific command wording, rerun guidance, and operator-facing refusal prose.
4. Live `crates/compiler/src/doctor.rs` still mixes reusable baseline/checklist classification with handbook-specific artifact-label and author-command wording.
5. Live `crates/compiler/src/refusal.rs` already acts as a typed semantic recovery model (`RefusalCategory`, `SubjectRef`, `NextSafeAction`), while live `crates/compiler/src/rendering/shared.rs` still renders that model into handbook-specific next-safe-action and subject wording used across doctor/generate/inspect surfaces.
6. Slice 3.4 must preserve the current public compiler and CLI behavior for `handbook setup`, `handbook doctor`, generate, and inspect unless the human approves a separate wording or surface redesign first.
7. Phase 4 crate-boundary work and Phase 5 CLI command-family extraction remain out of scope even if live code shows adjacent cleanup opportunities.
8. The smallest durable seam is expected to keep current public compiler entrypoints stable while extracting local shell owners such as `setup_shell.rs`, `doctor_shell.rs`, or a small recovery-render helper. If live implementation proves a slightly different local file split is cleaner, the ownership contract in this spec still must hold.

## Objective

Separate handbook-specific recovery wording and operator-facing copy from reusable setup, doctor, and refusal/readiness logic inside the current compiler crate, without changing the public handbook command surface.

The maintainer needs this slice so future Phase 4 crate extraction can move reusable logic without dragging handbook-specific recovery prose into core crates. Success means:

- reusable setup logic stops owning inline handbook command strings and rerun prose where a shell boundary is already clear
- reusable doctor checklist/baseline logic stops owning inline handbook artifact-label and author-command wording where a shell boundary is already clear
- shared next-safe-action and subject wording are isolated behind shell-oriented rendering helpers instead of remaining mixed into reusable refusal/readiness logic
- current CLI output for setup, doctor, generate, and inspect remains behavior-stable
- no Phase 4 crate work, Phase 5 CLI extraction, or broader wording redesign leaks into the slice

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Existing authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md`
  - `docs/CLI_TONE_RULES.md`
  - `docs/CLI_OUTPUT_ANATOMY.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-compiler --test setup && cargo test -p handbook-compiler --test doctor && cargo test -p handbook-cli --test cli_surface
```

Shared recovery/rendering rails:

```bash
cargo test -p handbook-compiler --test refusal_mapping && cargo test -p handbook-compiler --test rendering_surface
```

CLI wording fallout guard:

```bash
cargo test -p handbook-cli --test help_drift_guard
```

Setup wording ownership scan:

```bash
rg -n "setup_next_safe_action|setup_mutation_refusal|setup_refusal|handbook setup|handbook doctor|repair the blocked target" crates/compiler/src/setup*.rs crates/cli/src/main.rs
```

Doctor/refusal wording ownership scan:

```bash
rg -n "doctor_artifact_label|doctor_author_command|render_next_safe_action_value|render_subject_ref|NextSafeAction|SubjectRef" crates/compiler/src/doctor.rs crates/compiler/src/doctor_shell.rs crates/compiler/src/refusal.rs crates/compiler/src/rendering/shared.rs crates/cli/src/main.rs
```

Final workspace verification wall:

```bash
cargo check --workspace
cargo test --workspace
```

## Project Structure

```text
crates/compiler/src/setup.rs             -> current setup planning/mutation owner; should become a thinner reusable setup layer after shell wording extraction
crates/compiler/src/setup_shell.rs       -> proposed shell owner for setup command wording, rerun guidance, next-safe-action wording, and setup refusal prose
crates/compiler/src/doctor.rs            -> current doctor baseline/checklist owner; should keep typed status/next-action logic while delegating handbook-facing wording
crates/compiler/src/doctor_shell.rs      -> proposed shell owner for doctor artifact labels, author-command wording, and related doctor-facing copy
crates/compiler/src/refusal.rs           -> semantic refusal/recovery model that should stay typed and reusable
crates/compiler/src/rendering/shared.rs  -> shared rendering helpers; current mixed owner of handbook-specific next-safe-action and subject wording
crates/compiler/src/lib.rs               -> compiler public surface that should remain stable for downstream callers
crates/compiler/tests/setup.rs           -> focused setup regression coverage
crates/compiler/tests/doctor.rs          -> focused doctor JSON/checklist regression coverage
crates/compiler/tests/refusal_mapping.rs -> focused semantic-refusal and next-safe-action mapping regression coverage
crates/compiler/tests/rendering_surface.rs -> focused rendering/output-model regression coverage for shared recovery rendering
crates/cli/src/main.rs                   -> CLI renderer/orchestration surface that must stay behavior-stable
crates/cli/tests/cli_surface.rs          -> CLI setup/doctor/generate/inspect output regression coverage
crates/cli/tests/help_drift_guard.rs     -> tone/output-doc drift guard if wording fallout reaches documented help surfaces
```

If live implementation proves a smaller helper layout is cleaner than `setup_shell.rs` / `doctor_shell.rs`, that is acceptable only if the same reusable-core vs product-shell split and public-surface stability hold.

## Code Style

Prefer typed reusable decisions in core logic, with handbook-specific rendering happening at a shell boundary.

```rust
let status = classify_artifact_status(system_root_status, &validation);
let next_safe_action = artifact_next_safe_action(system_root_status, &validation, status);

DoctorChecklistItem {
    artifact_label: doctor_shell::artifact_label(validation.kind),
    author_command: doctor_shell::author_command(validation.kind),
    next_safe_action,
    ..
}
```

Conventions:

- reusable setup and doctor logic must not branch on rendered handbook command strings
- shell helpers may own exact handbook commands, rerun guidance, uppercase output labels, broken-subject prose, and operator-facing recovery sentences
- `refusal.rs` should remain a semantic model layer, not a store of fully rendered handbook sentences
- preserve current public compiler return types and serialized doctor-report fields unless the human approves a contract change first
- prefer typed handoffs and one-way rendering adapters over scattered inline string assembly

## Testing Strategy

- Framework: existing Rust integration tests in `crates/compiler/tests/` plus CLI regression tests in `crates/cli/tests/`
- Primary test levels:
  - setup tests for mode resolution, mutation planning, invalid-root repair, starter-file preservation, and next-safe-action behavior
  - doctor tests for baseline-state classification, checklist item status, JSON serialization, and recovery-action selection
  - refusal-mapping and rendering-surface tests for shared semantic-model rendering behavior
  - CLI surface tests for setup/doctor/generate/inspect output stability
- Coverage focus:
  - setup planning and mutation behavior stay unchanged while handbook wording moves behind shell helpers
  - doctor baseline classification stays typed and stable while artifact-label / author-command wording moves behind shell helpers
  - shared `NEXT SAFE ACTION` and subject wording remain stable for public surfaces after extraction
  - `RefusalCategory`, `SubjectRef`, and `NextSafeAction` semantics remain stable even if their rendered wording moves behind a clearer boundary
  - nested-repo, invalid-root, symlink, starter-template, and invalid-artifact rails stay behavior-stable
- Coverage expectation:
  - Packet 3.4.1 proves setup wording separation without changing public setup behavior
  - Packet 3.4.2 proves doctor/refusal/rendering wording separation while keeping compiler + CLI regressions green

## Slice Scope

In scope:

- isolate setup command wording, rerun guidance, and operator-facing refusal prose from reusable setup planning and mutation logic
- isolate doctor artifact-label / author-command wording from reusable baseline classification and checklist decision logic
- isolate shared next-safe-action and subject wording from reusable refusal/readiness logic where the split is already clear
- preserve the current public setup/doctor/generate/inspect behavior while making the internal shell boundary explicit
- prepare reusable setup/doctor/refusal logic for later crate extraction without performing the crate move yet

Out of scope:

- changing canonical `.handbook/` layout, setup-owned starter-file ownership, or baseline-validation rules
- changing refusal categories, doctor baseline-state semantics, or supported handbook command behavior intentionally
- redesigning CLI tone, output anatomy, or help semantics beyond the minimum wording-owner extraction needed for this slice
- moving code into `handbook-engine`, `handbook-pipeline`, or `handbook-flow` before Phase 4
- widening into authoring modules, packet-body rendering design, pipeline surfaces, or Phase 5 CLI orchestration extraction
- changing doctor JSON contract fields unless a separate approved spec revision says otherwise

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Phase 1 Slice 2 authority set:
  - `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-tasks.md`
- Phase 3 Slice 1 through Slice 3 authority set:
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-3-environment-inventory-core-split-spec.md`
- CLI wording authority:
  - `docs/CLI_TONE_RULES.md`
  - `docs/CLI_OUTPUT_ANATOMY.md`
- Live slice surfaces:
  - `crates/compiler/src/setup.rs`
  - `crates/compiler/src/doctor.rs`
  - `crates/compiler/src/refusal.rs`
  - `crates/compiler/src/rendering/shared.rs`
  - `crates/cli/src/main.rs`
- Live regression evidence:
  - `crates/compiler/tests/setup.rs`
  - `crates/compiler/tests/doctor.rs`
  - `crates/compiler/tests/refusal_mapping.rs`
  - `crates/compiler/tests/rendering_surface.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`

## Current Mixed Responsibilities To Untangle

| Responsibility | Current live owner | Slice 3.4 requirement |
| --- | --- | --- |
| setup planning, mutation sequencing, and disposition classification | `plan_setup`, `run_setup`, `build_setup_execution_plan`, and `setup_disposition` in `setup.rs` | keep reusable setup behavior typed and local; do not push mutation mechanics into CLI-only code |
| setup rerun guidance, next-safe-action text, and refusal/mutation prose | `validate_request`, `setup_refusal`, `setup_mutation_refusal`, `setup_next_safe_action`, `format_repo_mutation_error`, and `format_repo_write_path_error` in `setup.rs` | move handbook command wording and operator-facing prose behind a setup shell owner where the split is already clear |
| doctor baseline and checklist classification | `baseline_checklist`, `classify_artifact_status`, `artifact_next_safe_action`, and `classify_doctor_status` in `doctor.rs` | keep baseline/report decisions typed and reusable; do not couple them to inline handbook wording |
| doctor artifact labels and author-command copy | `doctor_artifact_label` and `doctor_author_command` in `doctor.rs` | move handbook-facing label/copy ownership behind a doctor shell owner while preserving current report/CLI behavior |
| shared next-safe-action and subject wording | `render_next_safe_action_value` and `render_subject_ref` in `rendering/shared.rs` | isolate handbook-specific recovery wording behind a shell-oriented render helper instead of leaving it mixed into reusable readiness/refusal rendering |
| refusal/recovery model semantics | `RefusalCategory`, `SubjectRef`, and `NextSafeAction` in `refusal.rs` | keep semantic categories/actions stable and reusable; avoid turning the model layer into full handbook sentences |

## Boundaries

- Always:
  - preserve current public setup/doctor/generate/inspect behavior first; this slice is internal ownership cleanup
  - keep reusable setup and doctor decision logic free of inline handbook wording where a local shell boundary is already clear
  - keep shared refusal/recovery semantics model-driven and let shell helpers render handbook-specific command strings
  - prove focused compiler and CLI regressions after each packet
  - honor `docs/CLI_TONE_RULES.md` and `docs/CLI_OUTPUT_ANATOMY.md` if wording fallout reaches documented surfaces
- Ask first:
  - changing public compiler structs or serialized doctor-report fields
  - renaming `RefusalCategory`, `SubjectRef`, or `NextSafeAction` variants
  - changing CLI help text, tone rules, or documented output anatomy intentionally
  - extracting helpers that widen into authoring, pipeline, or Phase 5 CLI orchestration work
  - changing routed-command or outcome semantics for setup/doctor
- Never:
  - move actual setup mutation mechanics or doctor baseline classification into CLI-only code
  - couple reusable logic to rendered handbook command strings
  - redesign handbook commands, refusal categories, or next-safe-action phrasing as part of this slice
  - pull Phase 4 crate work or Phase 5 CLI-thinning into Slice 3.4
  - widen into authoring, pipeline, or packet-body rendering cleanup

## Success Criteria

- A local shell owner exists for setup wording, or equivalent extraction, so reusable setup planning/mutation logic no longer owns inline handbook command copy where the seam is already clear.
- A local shell owner exists for doctor wording, or equivalent extraction, so doctor baseline/checklist logic no longer owns inline handbook artifact-label / author-command copy where the seam is already clear.
- Shared next-safe-action and subject wording are isolated from reusable refusal/readiness logic through a shell-oriented render boundary.
- Public compiler and CLI behavior for setup, doctor, generate, and inspect remains stable under existing regression coverage.
- `cargo test -p handbook-compiler --test setup && cargo test -p handbook-compiler --test doctor && cargo test -p handbook-cli --test cli_surface` continue to pass.
- `cargo test -p handbook-compiler --test refusal_mapping && cargo test -p handbook-compiler --test rendering_surface` continue to pass.
- No Phase 4 crate work, Phase 5 CLI extraction, or broader wording redesign leaked into the slice.

## Open Questions

- Should setup adopt a more explicitly typed internal recovery-action boundary while still preserving the current public `String` next-safe-action fields, or is a shell-helper split sufficient for Slice 3.4?
- Should doctor continue storing `author_command` as a serialized string field after this slice, or should wording stay string-backed for now so long as the current public JSON/CLI contract stays stable?
- Is a dedicated shared recovery-render helper the smallest durable owner for `render_next_safe_action_value` / `render_subject_ref`, or should those stay under `rendering/` so long as reusable model code stops owning handbook copy?
