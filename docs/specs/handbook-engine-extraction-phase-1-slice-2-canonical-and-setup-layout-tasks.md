# Tasks: Handbook Engine Extraction Phase 1 Slice 2 - Canonical And Setup Layout

Plan reference: [handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md](./handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md)

## Packet 1.2.1: Canonical Artifact Root Adoption

Packet 1.2.1 is the canonical-root owner adoption for Slice 1.2: introduce the compiler-local canonical layout owner, keep separate layout types rather than one global layout object, and move `canonical_artifacts.rs` behind that owner without changing canonical artifact identities.

- [ ] Task: Introduce the compiler-local canonical root layout owner
  - Acceptance: `crates/compiler/src/layout.rs` defines the canonical root and canonical artifact path accessors that Slice 1.2 needs, and the boundary stays specific to canonical ownership rather than becoming a monolithic all-layout object.
  - Verify: `cargo test -p handbook-compiler --test canonical_artifacts_ingest`
  - Files: `crates/compiler/src/layout.rs`, `crates/compiler/src/lib.rs`

- [ ] Task: Adopt `canonical_artifacts.rs` onto the canonical layout owner
  - Acceptance: canonical root, namespace-dir derivation, and canonical artifact path ownership flow through the canonical layout owner while artifact identity, ingest-issue, and setup-template semantics remain unchanged.
  - Verify: `cargo test -p handbook-compiler --test canonical_artifacts_ingest`
  - Files: `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/tests/canonical_artifacts_ingest.rs`

## Packet 1.2.2: Setup Bootstrap Root Adoption

Packet 1.2.2 is the setup-side canonical-root adoption for Slice 1.2: make `setup.rs` consume the canonical layout owner for canonical-root establishment and repair while leaving runtime-state reset ownership local for Slice 1.3.

- [ ] Task: Route setup canonical-root establishment and repair through the canonical layout owner
  - Acceptance: `setup.rs` uses canonical layout accessors for canonical-root inspection, repair, and setup-owned starter-file write planning; direct canonical-root ownership such as `repo_root.join(".handbook")` no longer remains setup-owned logic.
  - Verify: `cargo test -p handbook-compiler --test setup`
  - Files: `crates/compiler/src/setup.rs`, `crates/compiler/tests/setup.rs`

- [ ] Task: Preserve Slice 1.2 boundaries while integrating canonical and setup ownership
  - Acceptance: runtime-state reset behavior remains local and unadopted, no Slice 1.3 or Slice 1.4 caller migration is required, and the full Slice 1.2 verification wall passes.
  - Verify: `cargo test -p handbook-compiler --test canonical_artifacts_ingest && cargo test -p handbook-compiler --test setup && cargo check -p handbook-compiler`
  - Files: `crates/compiler/src/layout.rs`, `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/setup.rs`, `crates/compiler/src/lib.rs`, `crates/compiler/tests/canonical_artifacts_ingest.rs`, `crates/compiler/tests/setup.rs`

## Human Review Gate

Do not start Slice 1.2 implementation work until the human has reviewed and approved this Slice 1.2 spec/plan/tasks set.
