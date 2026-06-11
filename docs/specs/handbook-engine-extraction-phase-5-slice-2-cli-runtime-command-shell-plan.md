# Plan: Handbook Engine Extraction Phase 5 Slice 2 (Slice 5.2) - CLI Runtime Command Shell

## Objective

Continue Phase 5 by extracting the remaining runtime command families—`pipeline`, `inspect`, and `doctor`—out of `crates/cli/src/main.rs` so the CLI shell keeps converging toward a thin product entrypoint.

Spec reference: [handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-spec.md](./handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-spec.md)

## Major Modules

1. Runtime command dispatch boundary
   - `crates/cli/src/main.rs`
   - decides where top-level clap registration stops and runtime-family modules begin

2. Pipeline command-family module
   - current `pipeline` list/show/resolve/compile/capture/handoff/state orchestration
   - absorbs the pipeline-local refusal/render/state helper functions now living in `main.rs`

3. Doctor command-family module
   - current `doctor` text/JSON rendering flow and status-name helpers
   - becomes the CLI-local home for baseline-readiness command orchestration

4. Inspect command-family module
   - current `inspect` packet selection / fixture handling / renderer wiring flow
   - absorbs inspect-owned helper functions needed to preserve proof behavior

5. Minimal shared shell helpers
   - only helpers that are truly cross-family after the extraction
   - should stay intentionally small so Slice 5.2 does not turn into Slice 5.3 early

6. Verification surfaces
   - `crates/cli/tests/cli_surface.rs`
   - `crates/cli/tests/help_drift_guard.rs`
   - `crates/cli/tests/pipeline_handoff_refusals.rs`
   - `crates/cli/tests/manual_qa_fixture_checkout.rs`

## Dependencies And Order

### Prerequisite: preserve the Slice 5.1 shell split and freeze the runtime scope

Why first:

- Slice 5.2 depends on the setup/author extraction already being stable
- the runtime extraction should build on the existing module skeleton rather than re-opening Slice 5.1 structure
- freezing the scope up front prevents the slice from leaking into the help/render/exit cleanup reserved for Slice 5.3

Output:

- one agreed runtime module layout for `pipeline`, `inspect`, and `doctor`
- one agreed rule for helper placement: family-local by default, shared only when immediately justified
- one agreed fence that broad prompt/render/help/exit cleanup stays out of this slice

### Packet 5.2.1: Pipeline Command-Family Extraction

Why first:

- `pipeline` is the largest remaining runtime family in `main.rs`, so moving it yields the biggest shell-thinning step
- it already has strong command-surface and help-drift coverage
- extracting it first lets Packet 5.2.2 focus on the proof/readiness families without mixing too many kinds of runtime behavior at once

Output:

- `main.rs` delegates `Command::Pipeline` into a dedicated pipeline shell module
- pipeline refusal/render/state helpers move out of `main.rs` with the family that owns them
- the supported list/show/resolve/compile/capture/handoff/state surface remains stable

### Packet 5.2.2: Inspect And Doctor Command-Family Extraction

Why second:

- once the pipeline family is out, the remaining work is the proof/readiness layer
- `inspect` and `doctor` share the general concern of operator-facing result rendering, but still need to stay distinct enough to preserve their contract boundaries
- delaying these extractions until after the pipeline cut keeps the review surface easier to reason about

Output:

- `main.rs` delegates `Command::Inspect` into a dedicated inspect shell module
- `main.rs` delegates `Command::Doctor` into a dedicated doctor shell module
- inspect-owned packet/fixture helpers move with inspect, and doctor-owned JSON/text render helpers move with doctor
- ready/blocking semantics, JSON output, and fixture behavior remain stable

## Risks And Mitigations

### Risk: the runtime split widens into the full Phase 5 CLI closeout

Mitigation:

- keep Packet 5.2.1 limited to the pipeline family
- keep Packet 5.2.2 limited to inspect/doctor and the smallest required helpers
- reject broad prompt/render/help/exit cleanup unless it is a direct compile blocker

### Risk: helper placement becomes muddled between family-local files and `shell_shared.rs`

Mitigation:

- default helper ownership to the family that uses it
- move a helper into `shell_shared.rs` only when there is live two-family reuse now
- avoid creating generic “utils” files that hide ownership

### Risk: pipeline help or refusal semantics drift during extraction

Mitigation:

- use `help_drift_guard` throughout Packet 5.2.1, not only at the end
- use `pipeline_handoff_refusals` as a focused regression wall for important refusal paths
- keep pipeline render/refusal helpers with the pipeline shell instead of scattering them

### Risk: inspect or doctor extraction accidentally changes proof/readiness contracts

Mitigation:

- preserve the existing inspect/doctor separation defined by `C-02` and `C-04`
- use `cli_surface` and `manual_qa_fixture_checkout` to hold inspect fixture semantics steady
- keep doctor JSON/text rendering logic behavior-preserving and avoid shape changes

## Parallel Vs Sequential

Sequential:

- confirm the runtime module layout before moving the families
- extract `pipeline` before extracting `inspect` and `doctor`
- keep the final slice verifier until all runtime families are out of `main.rs`

Parallel opportunities after Packet 5.2.1 lands:

- inspect and doctor extraction subtasks can proceed independently once the pipeline cut and any helper-boundary decisions are stable
- focused runtime tests can run in parallel during final cleanup

## Verification Checkpoints

### Checkpoint 1: pipeline extraction preserves the runtime command surface

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-cli --test pipeline_handoff_refusals
```

### Checkpoint 2: inspect and doctor extraction preserve proof/readiness behavior

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test manual_qa_fixture_checkout
```

### Checkpoint 3: `main.rs` is materially thinner and runtime families are no longer inline

```bash
wc -l crates/cli/src/main.rs
rg -n '^fn (pipeline|pipeline_list|pipeline_show|pipeline_resolve|pipeline_compile|pipeline_capture|pipeline_handoff|pipeline_state_set|inspect|doctor|render_doctor_json)' crates/cli/src/main.rs
```

### Final checkpoint

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-cli --all-targets -- -D warnings
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-cli --test pipeline_handoff_refusals
cargo test -p handbook-cli --test manual_qa_fixture_checkout
```

## Exit Conditions

The slice is ready for human review when:

- `main.rs` delegates `pipeline`, `inspect`, and `doctor` into dedicated CLI modules
- the remaining runtime-family helper code no longer lives inline in `main.rs`
- helper ownership is still obvious and bounded
- help posture and command behavior remain stable
- Slice 5.3 prompt/render/help/exit-code cleanup has not leaked into the slice
- the focused runtime verification wall passes
