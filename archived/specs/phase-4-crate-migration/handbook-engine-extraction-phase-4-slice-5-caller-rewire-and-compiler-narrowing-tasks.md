# Tasks: Handbook Engine Extraction Phase 4 Slice 5 (Set 3 / Slice 4.5 Refresh) - Direct Caller Rewires + Compiler Narrowing Closeout

Plan reference: [handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md](./handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md)

## Prerequisite Authority

Before implementation, re-read:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- the existing Slice 4.5 triplet
- the landed Slice 4.2, 4.3, and 4.4 triplets

This refresh is closeout/remediation work. It should finish the remaining caller/dependency honesty gap without reopening Phase 2 target-parameterization work or widening into Set 4 CLI shell cleanup.

## Packet 4.5.1: Residual Caller Inventory And Boundary Freeze

- [ ] Task: Inventory every remaining `handbook_compiler::*` caller in CLI-adjacent code and classify it by ownership
  - Acceptance: Every remaining compiler-root caller in `crates/cli/src/**`, `crates/cli/tests/**`, `crates/flow/**`, and `crates/compiler/src/lib.rs` is classified as either stale extracted-logic indirection or legitimate retained compatibility/support usage.
  - Verify: `rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli/src crates/cli/tests crates/flow crates/compiler/src/lib.rs && cargo tree -p handbook-cli -e normal`
  - Files: `crates/cli/src/main.rs`, `crates/cli/src/author.rs`, `crates/cli/src/setup.rs`, `crates/cli/src/doctor.rs`, `crates/cli/src/rendering.rs`

- [ ] Task: Preserve the already-landed owner-rooted surfaces while freezing the residual rewire list
  - Acceptance: `crates/flow/src/lib.rs` remains free of compiler forwarding, and existing owner-rooted test surfaces stay on direct engine/pipeline/flow imports rather than regressing back to compiler-facade usage.
  - Verify: `rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/flow crates/cli/tests && cargo test -p handbook-flow && cargo test -p handbook-cli --test author_cli && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/flow/src/lib.rs`, `crates/cli/tests/author_cli.rs`, `crates/cli/tests/cli_surface.rs`

## Packet 4.5.2: Stale Caller Rewires To Real Owner Crates

- [ ] Task: Rewire stale extracted-logic callers directly to `handbook-engine`, `handbook-pipeline`, or `handbook-flow`
  - Acceptance: Any caller still using `handbook-compiler` only as a convenience facade for extracted logic is moved to the real owner crate, while legitimate retained support-seam usages remain explicit and reviewable.
  - Verify: `cargo test -p handbook-cli --test author_cli && cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-engine && cargo test -p handbook-pipeline && cargo test -p handbook-flow`
  - Files: `crates/cli/src/main.rs`, `crates/cli/src/author.rs`, `crates/cli/src/rendering.rs`, `crates/cli/tests/author_cli.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Keep `handbook-cli` dependency posture honest after the rewires
  - Acceptance: `crates/cli` depends directly on the owner crates for extracted logic, and any retained `handbook-compiler` dependency is justified solely by the narrow compatibility/support seam that still exists in live code.
  - Verify: `cargo tree -p handbook-cli -e normal && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/cli/Cargo.toml`, optionally `Cargo.toml`, optionally `crates/compiler/Cargo.toml`

## Packet 4.5.3: Compiler Narrow Boundary Truth

- [ ] Task: Keep `handbook-compiler` limited to the reviewed narrow compatibility/support seam
  - Acceptance: `crates/compiler/src/lib.rs` exports only the reviewed support surface that still legitimately spans CLI-facing seams; it does not regain broad umbrella ownership for engine-, pipeline-, or flow-owned logic.
  - Verify: `cargo tree -p handbook-compiler -e normal && cargo test -p handbook-compiler --test author && cargo test -p handbook-compiler --test doctor && cargo test -p handbook-compiler --test setup`
  - Files: `crates/compiler/src/lib.rs`, `crates/compiler/Cargo.toml`, optionally `crates/compiler/tests/author.rs`, `crates/compiler/tests/doctor.rs`, `crates/compiler/tests/setup.rs`

- [ ] Task: Align repo-facing ownership docs and help guards to the retained compiler seam
  - Acceptance: README, docs, contract language, and help guards describe the same direct-owner plus narrow-compiler boundary that the live code implements, without promising compiler retirement or broader Phase 5 cleanup in this slice.
  - Verify: `cargo test -p handbook-cli --test help_drift_guard && rg -n 'narrow compatibility/support|direct owner|umbrella' README.md docs/README.md docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - Files: `README.md`, `docs/README.md`, `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`, optionally `crates/cli/tests/help_drift_guard.rs`

## Packet 4.5.4: Final Closeout Proof

- [ ] Task: Run the full Slice 4.5 refresh verification wall and preserve explicit Set 4 deferrals
  - Acceptance: Residual caller classifications, direct-owner rewires, compiler narrowing truth, and repo-facing ownership docs all agree; the workspace is format-clean, clippy-clean, and test-green; remaining CLI shell finish work is explicitly deferred to Set 4.
  - Verify: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the refreshed Slice 4.5 spec, plan, and tasks are reviewed. Do not widen into packet prompts, implementation, or Set 4 CLI shell closeout unless the human explicitly asks for the next artifact.
