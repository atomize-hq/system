# Tasks: Handbook Engine Extraction Phase 5 Slice 2 (Slice 5.2) - CLI Runtime Command Shell

Plan reference: [handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-plan.md](./handbook-engine-extraction-phase-5-slice-2-cli-runtime-command-shell-plan.md)

## Prerequisite: the remaining runtime shell split must stay bounded

Slice 5.2 exists to continue thinning `handbook-cli`, not to finish the whole CLI closeout. Keep the cut limited to the `pipeline`, `inspect`, and `doctor` command families plus the smallest helper moves they require.

- Prompting, broad rendering/help cleanup, and exit-code closeout stay reserved for Slice 5.3.

## Packet 5.2.1: Pipeline Command-Family Extraction

- [ ] Task: Establish a dedicated CLI module for the pipeline command family
  - Acceptance: `main.rs` delegates `Command::Pipeline` into a pipeline shell module, and the supported list/show/resolve/compile/capture/handoff/state surface remains easy to trace.
  - Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
  - Files: `crates/cli/src/main.rs`, new pipeline module file(s) under `crates/cli/src/`

- [ ] Task: Move pipeline-local refusal, render, and route-state helper code out of `main.rs`
  - Acceptance: pipeline-specific helpers such as selector refusals, compile refusals, route-state mutation parsing/rendering, and related shell wiring no longer live inline in `main.rs`, and ownership stays obvious.
  - Verify: `cargo test -p handbook-cli --test pipeline_handoff_refusals && rg -n '^fn (pipeline|pipeline_list|pipeline_show|pipeline_resolve|pipeline_compile|pipeline_capture|pipeline_handoff|pipeline_state_set|render_pipeline_|parse_route_state_mutation)' crates/cli/src/main.rs`
  - Files: `crates/cli/src/main.rs`, new or updated pipeline-supporting module file(s) under `crates/cli/src/`

## Packet 5.2.2: Inspect And Doctor Command-Family Extraction

- [ ] Task: Move the doctor command family into its dedicated CLI module
  - Acceptance: `doctor` text/JSON readiness routing and doctor-local status/render helpers no longer live inline in `main.rs`, while baseline-state semantics and `--json` behavior remain unchanged.
  - Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
  - Files: `crates/cli/src/main.rs`, chosen doctor module file(s) under `crates/cli/src/`

- [ ] Task: Move the inspect command family into its dedicated CLI module
  - Acceptance: `inspect` packet selection, execution-demo fixture handling, and inspect-owned renderer wiring no longer live inline in `main.rs`, while ready/blocking and fixture semantics remain unchanged.
  - Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test manual_qa_fixture_checkout`
  - Files: `crates/cli/src/main.rs`, chosen inspect module file(s) under `crates/cli/src/`

- [ ] Task: Keep the extraction scoped away from Slice 5.3 shell-closeout work
  - Acceptance: the packet does not introduce unrelated prompt/render/help abstraction work, exit-code cleanup, or command-surface redesign beyond what the inspect/doctor extraction strictly requires to compile.
  - Verify: `rg -n '^fn (pipeline|inspect|doctor|render_doctor_json)' crates/cli/src/main.rs && cargo test -p handbook-cli --test help_drift_guard`
  - Files: verification against `crates/cli/src/main.rs` plus packet-local module files only

## Final Slice Verification

- [ ] Task: Run the focused Slice 5.2 verification wall after both packets land
  - Acceptance: the CLI runtime families moved out of `main.rs`, helper ownership stayed bounded, behavior/help/fixture semantics stayed stable, and the remaining shell-closeout work is still clearly Slice 5.3.
  - Verify: `cargo fmt --all -- --check && cargo clippy -p handbook-cli --all-targets -- -D warnings && cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard && cargo test -p handbook-cli --test pipeline_handoff_refusals && cargo test -p handbook-cli --test manual_qa_fixture_checkout`
  - Files: verification only

## Human Review Gate

Stop after the Slice 5.2 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
