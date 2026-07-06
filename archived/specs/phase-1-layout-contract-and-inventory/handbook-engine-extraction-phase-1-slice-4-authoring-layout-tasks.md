# Tasks: Handbook Engine Extraction Phase 1 Slice 4 - Authoring Layout

Plan reference: [handbook-engine-extraction-phase-1-slice-4-authoring-layout-plan.md](./handbook-engine-extraction-phase-1-slice-4-authoring-layout-plan.md)

## Packet 1.4.1: Authoring Roots And Lock Paths Adoption

Packet 1.4.1 is the authoring ownership adoption for Slice 1.4: extend the compiler-local layout family for authoring canonical targets and authoring lock paths, keep separate layout types rather than one global layout object, and move the authoring modules behind those owners without changing prompt, validation, or refusal semantics.

- [ ] Task: Introduce compiler-local authoring layout owners
  - Acceptance: `crates/compiler/src/layout.rs` defines the authoring canonical-target and authoring lock-path accessors that Slice 1.4 needs, and the boundary stays specific to authoring ownership rather than becoming a monolithic all-layout object.
  - Verify: `cargo check -p handbook-compiler`
  - Files: `crates/compiler/src/layout.rs`, `crates/compiler/src/lib.rs`

- [ ] Task: Adopt `author/charter.rs` onto the authoring layout owner
  - Acceptance: charter canonical write-target and authoring lock ownership flow through the authoring layout owner while guided authoring, structured-input validation, default exception location, and refusal semantics remain unchanged.
  - Verify: `cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/author/charter.rs`, `crates/compiler/tests/author.rs`

- [ ] Task: Adopt `author/project_context.rs` and `author/environment_inventory.rs` onto the authoring layout owner
  - Acceptance: project-context and environment-inventory canonical write-target and authoring lock ownership flow through the authoring layout owner while metadata rendering, upstream canonical-truth checks, prompt text, synthesis behavior, and refusal semantics remain unchanged.
  - Verify: `cargo test -p handbook-compiler --test author`
  - Files: `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/tests/author.rs`

- [ ] Task: Preserve Slice 1.4 boundaries while integrating authoring ownership
  - Acceptance: canonical artifact identities remain unchanged, prompt/template/operator wording stays local, no Phase 2 target work or Phase 3 shell cleanup is required, and the full Slice 1.4 verification wall passes.
  - Verify: `cargo test -p handbook-compiler --test author && cargo check -p handbook-compiler`
  - Files: `crates/compiler/src/layout.rs`, `crates/compiler/src/author/mod.rs`, `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/project_context.rs`, `crates/compiler/src/author/environment_inventory.rs`, `crates/compiler/src/lib.rs`, `crates/compiler/tests/author.rs`

## Human Review Gate

Do not start Slice 1.4 implementation work until the human has reviewed and approved this Slice 1.4 spec/plan/tasks set.
