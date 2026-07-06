# Tasks: Handbook Engine Extraction Phase 2 Slice 3 (Slice 2.3) - Template And Library Resolver Boundary

Plan reference: [handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md](./handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md)

## Prerequisite: Slice 2.2 Authority

Phase 2 Slice 2 already landed runtime supported-target adoption. Slice 2.3 must stay template/library-only: it cannot reopen supported-target ownership, change the approved runtime wedge, or widen into Phase 3 shell cleanup.

- Slice 2.3 must replace local shipped-asset ownership with a typed resolver boundary, not redesign runtime targets.

## Packet 2.3.1: Typed Resolver Contract And Shipped-Default Posture

- [ ] Task: Define the typed template/library resolver contract for the approved authoring asset families
  - Acceptance: Live compiler code exposes one typed owner for the approved charter and environment-inventory asset identities, shipped-default selection posture, and canonical repo-relative metadata, without changing zero-config behavior.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-compiler --test pipeline_catalog`
  - Files: `crates/compiler/src/template_library.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`, `crates/compiler/tests/pipeline_catalog.rs`

- [ ] Task: Migrate charter authoring onto the resolver-backed shipped defaults
  - Acceptance: `author/charter.rs` stops owning raw shipped authoring-method/directive/template selection and instead consumes the typed resolver while preserving charter-specific validation, heading guarantees, and guided-authoring output behavior.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/author/charter.rs`, `crates/compiler/src/template_library.rs`, `crates/compiler/tests/author.rs`, `crates/cli/tests/author_cli.rs`

- [ ] Task: Migrate environment-inventory authoring onto the same resolver-backed shipped defaults
  - Acceptance: `author/environment_inventory.rs` stops owning raw shipped directive/template selection and instead consumes the same typed resolver while preserving optional project-context posture, canonical-path validation, and synthesis-result validation.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/src/template_library.rs`, `crates/compiler/tests/author.rs`, `crates/cli/tests/author_cli.rs`

## Packet 2.3.2: Validated Override And Selection Rules

- [ ] Task: Add validated override and alternate-selection rules for approved asset families
  - Acceptance: The resolver can accept optional typed override requests for the approved charter and environment-inventory asset families, and it refuses absolute paths, traversal, out-of-root selections, missing files, and asset-kind mismatches.
  - Verify: `cargo test -p handbook-compiler --test author && cargo test -p handbook-compiler --test pipeline_catalog`
  - Files: `crates/compiler/src/template_library.rs`, `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/tests/author.rs`, `crates/compiler/tests/pipeline_catalog.rs`

- [ ] Task: Preserve shipped-default and starter-template compatibility after override support lands
  - Acceptance: Without explicit overrides, setup and authoring still behave as they do today; any starter-template/status semantics touched by the slice remain stable unless the approved spec is updated first.
  - Verify: `cargo test -p handbook-compiler --test setup && cargo test -p handbook-cli --test author_cli`
  - Files: `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/setup.rs`, `crates/compiler/src/template_library.rs`, `crates/compiler/tests/setup.rs`, `crates/cli/tests/author_cli.rs`

## Final Slice Verification

- [ ] Task: Run the full Slice 2.3 verification wall after both packets land
  - Acceptance: The typed resolver is the only approved owner for shipped authoring asset selection, zero-config behavior remains stable, override rules are bounded and refusal-tested, declarative stage library truth remains authoritative, and the workspace checks pass.
  - Verify: `cargo check --workspace && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 2.3 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
