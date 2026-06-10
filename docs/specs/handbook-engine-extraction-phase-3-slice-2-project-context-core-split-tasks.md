# Tasks: Handbook Engine Extraction Phase 3 Slice 2 (Slice 3.2) - Project-Context Deterministic Core Split

Plan reference: [handbook-engine-extraction-phase-3-slice-2-project-context-core-split-plan.md](./handbook-engine-extraction-phase-3-slice-2-project-context-core-split-plan.md)

## Prerequisite: Slice 3.1 split pattern and the public project-context contract stay frozen

Slice 3.1 already established the approved in-place split pattern for authoring surfaces. Slice 3.2 must stay boundary-cleanup-only: it cannot redesign the CLI interview, change markdown semantics, or reopen adjacent authoring surfaces.

- Slice 3.2 must split deterministic project-context core from shell/runtime behavior, not redesign project-context authoring.

## Packet 3.2.1: Project-Context Deterministic Model Split

- [ ] Task: Introduce a deterministic project-context-core owner for structured-input types, normalization, and validation
  - Acceptance: One project-context-core boundary owns the structured-input model plus normalization, factuality checks, render-safety checks, and markdown validation helpers, and that boundary does not read environment variables, derive current time, acquire authoring locks, or mutate repo state.
  - Verify: `cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/project_context_core.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`

- [ ] Task: Move deterministic project-context markdown rendering behind the same core boundary
  - Acceptance: Compiler-owned project-context rendering and required-heading / placeholder validation are owned by the project-context core, while outward parse/render/validate entrypoints remain available through the current public compiler surface and render deterministically once shell-provided timestamp metadata is supplied.
  - Verify: `cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/project_context_core.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`

## Packet 3.2.2: Project-Context Recovery Wording And Shell Cleanup

- [ ] Task: Extract project-context render timestamp resolution and authoring mutation flow into shell-owned helpers
  - Acceptance: Timestamp resolution, canonical-root inspection, baseline eligibility checks, lock acquisition, canonical-write validation, and repo mutation move out of the deterministic core and are owned by shell/runtime helpers instead.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/project_context_shell.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`, `crates/cli/tests/author_cli.rs`

- [ ] Task: Thin project-context preflight and authoring entrypoints around the new core and shell boundaries
  - Acceptance: `preflight_author_project_context`, `author_project_context`, and `author_project_context_from_input` become thin orchestrators that preserve guided-CLI ownership, canonical write behavior, refusal posture, and public compiler/CLI behavior without resuming ownership of deterministic project-context logic.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/project_context_shell.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`, `crates/cli/tests/author_cli.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 3.2 verification wall after both packets land
  - Acceptance: Deterministic project-context logic is isolated from timestamp/runtime/mutation behavior, public compiler and CLI behavior remain stable, and workspace checks pass without adjacent-slice leakage.
  - Verify: `cargo check --workspace && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 3.2 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
