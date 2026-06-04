# Tasks: Candidate 2 Trusted Pipeline Session Deepening

Plan reference: [candidate-2-trusted-pipeline-session-plan.md](./candidate-2-trusted-pipeline-session-plan.md)

## Packet 1: Trusted Pipeline Session Core

- [ ] Task: Add the trusted pipeline session core in the compiler
  - Acceptance: One compiler-owned module or module section produces a typed trust result that covers route-state load, canonical route-basis rebuild, freshness comparison, stage activity, and compile-facing normalization without caller-side reconstruction.
  - Verify: `cargo test -p handbook-compiler --test pipeline_state_store`
  - Files: `crates/compiler/src/route_state.rs`, `crates/compiler/tests/pipeline_state_store.rs`, `crates/compiler/src/lib.rs`

- [ ] Task: Add packet-local trust-seam regression coverage
  - Acceptance: Compiler tests explicitly prove the new seam handles missing, stale, malformed, and inactive route-basis cases without relying on compile/capture/handoff-specific ladders.
  - Verify: `cargo test -p handbook-compiler --test pipeline_state_store`
  - Files: `crates/compiler/tests/pipeline_state_store.rs`, optionally `crates/compiler/src/lib.rs`

## Packet 2: Compile Migration

- [ ] Task: Migrate compile to the trusted pipeline session seam
  - Acceptance: `pipeline_compile.rs` consumes the shared trust result and no longer owns repeated route-state load plus canonical route-basis freshness setup inline.
  - Verify: `cargo test -p handbook-compiler --test pipeline_compile && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/tests/pipeline_compile.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Keep compile-facing refusal posture stable after migration
  - Acceptance: Existing compile refusal classifications and next-safe-action posture remain stable after the trust migration unless an explicit spec update says otherwise.
  - Verify: `cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/pipeline_compile.rs`, `crates/cli/tests/cli_surface.rs`

## Packet 3: Capture Migration

- [ ] Task: Migrate capture preview/apply to the trusted pipeline session seam
  - Acceptance: `pipeline_capture.rs` delegates common trust setup to the shared seam while retaining capture-specific plan identity, revision-conflict, write, rollback, and provenance behavior.
  - Verify: `cargo test -p handbook-compiler --test pipeline_capture && cargo test -p handbook-cli --test cli_surface`
  - Files: `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/tests/pipeline_capture.rs`, `crates/cli/tests/cli_surface.rs`

- [ ] Task: Preserve capture apply safety after migration
  - Acceptance: Preview/apply flows still refuse stale route-basis and revision-conflict cases correctly, and capture-specific rollback/write behavior remains owned by the capture adapter rather than being pushed into unrelated callers.
  - Verify: `cargo test -p handbook-compiler --test pipeline_capture`
  - Files: `crates/compiler/src/pipeline_capture.rs`, `crates/compiler/tests/pipeline_capture.rs`

## Packet 4: Handoff Migration And Closeout

- [ ] Task: Migrate handoff validation/emission to the trusted pipeline session seam
  - Acceptance: `pipeline_handoff.rs` stops independently rebuilding route-basis trust and instead consumes the shared trust implementation for validation inputs and provenance binding.
  - Verify: `cargo test -p handbook-compiler --test pipeline_handoff && cargo test -p handbook-cli --test pipeline_handoff_refusals`
  - Files: `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/tests/pipeline_handoff.rs`, `crates/cli/tests/pipeline_handoff_refusals.rs`

- [ ] Task: Decide and document the library export posture for downstream consumers
  - Acceptance: The implementation either keeps the trusted pipeline session internal with clear future-export notes or exposes a small reviewed library surface suitable for `substrate`, and the choice is reflected in the spec/plan docs.
  - Verify: `cargo check --workspace`
  - Files: `crates/compiler/src/lib.rs`, `docs/specs/candidate-2-trusted-pipeline-session-spec.md`, `docs/specs/candidate-2-trusted-pipeline-session-plan.md`

- [ ] Task: Run the final regression wall for the deepening
  - Acceptance: Workspace formatting, lint, compiler tests, CLI tests, and full workspace tests pass with the new trust seam in place.
  - Verify: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets --all-features -- -D warnings && cargo test --workspace && cargo check --workspace`
  - Files: no new source files; verification only
