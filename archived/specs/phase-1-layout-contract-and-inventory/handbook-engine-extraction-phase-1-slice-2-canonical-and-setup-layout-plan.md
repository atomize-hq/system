# Plan: Handbook Engine Extraction Phase 1 Slice 2 - Canonical And Setup Layout

## Objective

Introduce the first compiler-local canonical layout owner and adopt `canonical_artifacts.rs` plus the canonical-root portion of `setup.rs` onto it without changing existing behavior.

Spec reference: [handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md](./handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md)

## Major Artifacts

1. Canonical layout owner surface
   - lives in `crates/compiler/src/layout.rs`
   - owns repo-root normalization, canonical `.handbook` root access, and canonical artifact path derivation

2. Canonical artifact adoption
   - updates `crates/compiler/src/canonical_artifacts.rs` to consume the canonical layout owner
   - preserves existing artifact descriptor and identity semantics

3. Setup adoption
   - updates `crates/compiler/src/setup.rs` to consume the same canonical layout owner for canonical-root inspection, repair, and write-target planning
   - keeps runtime-state reset logic local and deferred to Slice 1.3

4. Regression coverage
   - keeps `crates/compiler/tests/canonical_artifacts_ingest.rs` and `crates/compiler/tests/setup.rs` as the primary safety wall
   - extends or adjusts tests only where the new owner seam needs direct proof

## Dependencies And Order

### Packet 1.2.1 first: Canonical Artifact Root Adoption

Why first:

- `setup.rs` already depends on `CanonicalArtifacts`, so canonical ownership must move there first
- the canonical layout owner needs a stable consumer before setup can adopt it
- this packet proves the typed owner can replace duplicated canonical-root literals without touching runtime-state flow

Output:

- one compiler-local `layout.rs` module that defines the canonical root owner surface
- one `canonical_artifacts.rs` adoption that routes root and namespace ownership through that surface
- unchanged canonical artifact identities and ingest semantics

### Packet 1.2.2 second: Setup Bootstrap Root Adoption

Why second:

- setup should consume the already-landed canonical owner instead of inventing a second root helper
- setup touches both canonical root and runtime-state flow, so it must adopt only after the canonical boundary is stable
- this packet can stay narrow if the canonical-root seam already exists

Output:

- one `setup.rs` adoption that consumes the canonical layout owner for canonical-root establishment and repair
- one preserved boundary where runtime-state reset logic remains local and deferred to Slice 1.3
- unchanged setup init/refresh/refusal semantics

## Risks And Mitigations

### Risk: Slice 1.2 drifts into runtime-state ownership work

Mitigation:

- keep `.handbook/state/**` ownership explicitly deferred to Slice 1.3
- allow `setup.rs` to keep calling current runtime-state reset helpers without moving that ownership into `layout.rs`

### Risk: canonical path semantics drift during adoption

Mitigation:

- preserve exact repo-relative path strings and namespace directories from the Slice 1.1 inventory
- keep `canonical_artifacts_ingest` as a required verification wall for Packet 1.2.1

### Risk: setup introduces a second source of truth for canonical root ownership

Mitigation:

- require `setup.rs` to consume the same canonical layout owner used by `canonical_artifacts.rs`
- reject packet language that leaves `repo_root.join(".handbook")` or duplicate canonical-root derivation as setup-owned logic

### Risk: the new owner becomes a monolithic all-layout object too early

Mitigation:

- keep `layout.rs` limited to Canonical root layout concerns in Slice 1.2
- defer runtime-state, capture, handoff, and authoring ownership to their approved follow-on slices

### Risk: tests stay green while slice boundaries drift

Mitigation:

- pair the test wall with explicit boundary review against the spec
- treat any required change to `route_state.rs`, `pipeline_capture.rs`, `pipeline_handoff.rs`, or `author/**` as out-of-scope leakage

## Parallel Vs Sequential

Sequential:

- Packet 1.2.1 before Packet 1.2.2
- canonical layout owner introduction before setup adoption
- canonical artifact verification before the setup packet lands

Not parallel:

- do not split canonical owner creation and `canonical_artifacts.rs` adoption across separate simultaneous packets
- do not start Slice 1.3 runtime-state adoption work from `setup.rs`
- do not mix authoring-path adoption into the same slice

## Verification Checkpoints

### Checkpoint 1: Canonical owner and canonical-artifacts adoption complete

Confirm the compiler now has a canonical layout owner and `canonical_artifacts.rs` consumes it.

Suggested verification:

```bash
rg -n "CanonicalLayout|RepoLayoutRoot|SYSTEM_ROOT_RELATIVE|relative_path|namespace_dir" \
  crates/compiler/src/layout.rs \
  crates/compiler/src/canonical_artifacts.rs

cargo test -p handbook-compiler --test canonical_artifacts_ingest
```

### Checkpoint 2: Setup adoption complete

Confirm `setup.rs` now consumes the canonical owner for canonical-root ownership while leaving runtime-state reset local.

Suggested verification:

```bash
rg -n "CanonicalLayout|RepoLayoutRoot|join\\(\"\\.handbook\"\\)|reset_state|runtime-state" \
  crates/compiler/src/layout.rs \
  crates/compiler/src/setup.rs

cargo test -p handbook-compiler --test setup
```

### Checkpoint 3: Slice boundary remains intact

Confirm the slice stayed within the approved canonical/setup corpus and still compiles cleanly.

Suggested verification:

```bash
cargo test -p handbook-compiler --test canonical_artifacts_ingest
cargo test -p handbook-compiler --test setup
cargo check -p handbook-compiler
```

## Exit Conditions

Slice 1.2 is ready for human review when:

- canonical root ownership has one typed compiler-local owner
- `canonical_artifacts.rs` and `setup.rs` both consume that owner
- canonical artifact and setup semantics remain unchanged
- runtime-state ownership still clearly belongs to Slice 1.3
- the targeted test wall passes cleanly

Slice 1.2 is ready for implementation only after the human reviews and accepts the spec/plan/tasks set.
