# Tasks: Handbook Engine Extraction Phase 3 Slice 1 (Slice 3.1) - Charter Deterministic Core Split

Plan reference: [handbook-engine-extraction-phase-3-slice-1-charter-core-split-plan.md](./handbook-engine-extraction-phase-3-slice-1-charter-core-split-plan.md)

## Prerequisite: Phase 2 Charter Asset Ownership Stays Frozen

Phase 2 Slice 3 already made `template_library.rs` the approved shipped-default charter asset owner. Slice 3.1 must stay boundary-cleanup-only: it cannot reopen template selection rules, shipped-default posture, or general authoring-surface cleanup.

- Slice 3.1 must split deterministic charter core from shell/runtime behavior, not redesign asset ownership.

## Packet 3.1.1: Charter Parse Render Validate Core Extraction

- [ ] Task: Introduce a deterministic charter-core owner for structured-input types, normalization, and validation
  - Acceptance: One charter-core boundary owns the structured-input model plus normalization and validation helpers, and that boundary does not spawn processes, read runtime env overrides, create temp files, or mutate repo state.
  - Verify: `cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/charter_core.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`

- [ ] Task: Move deterministic charter markdown rendering and markdown validation behind the same core boundary
  - Acceptance: Compiler-owned charter rendering, heading/order validation, and render-safety helpers are owned by the charter core, while outward parse/render/validate entrypoints remain available through the current public compiler surface.
  - Verify: `cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/charter_core.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`

## Packet 3.1.2: Charter Synthesis And Shell Adapter Cleanup

- [ ] Task: Extract guided charter synthesis prompt assembly and runtime transport into shell-owned helpers
  - Acceptance: Prompt assembly, shipped template-library selection, `codex exec` transport, env-var overrides, temp-output handling, and synthesized-markdown validation move out of the deterministic core and are owned by shell/runtime helpers instead.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/charter_shell.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`, `crates/cli/tests/author_cli.rs`

- [ ] Task: Thin charter preflight and canonical-write orchestration around the new core and shell boundaries
  - Acceptance: `preflight_author_charter`, `preflight_author_charter_from_input`, `author_charter`, and `author_charter_guided` become thin orchestrators that preserve canonical write, lock, refusal, and CLI behavior without resuming ownership of deterministic charter logic.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/charter_shell.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`, `crates/cli/tests/author_cli.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 3.1 verification wall after both packets land
  - Acceptance: Deterministic charter logic is isolated from guided synthesis and product-shell behavior, public compiler and CLI behavior remain stable, and the workspace checks pass without adjacent-slice leakage.
  - Verify: `cargo check --workspace && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 3.1 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
