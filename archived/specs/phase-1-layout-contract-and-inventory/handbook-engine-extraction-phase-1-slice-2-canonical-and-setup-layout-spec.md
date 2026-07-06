# Spec: Handbook Engine Extraction Phase 1 Slice 2 - Canonical And Setup Layout

## Assumptions

1. Slice 1.1 is already the accepted authority for the separate-layout-types contract and the Phase 1 ownership-domain names.
2. Slice 1.2 is the first **code-adoption** slice in Phase 1, but it is still a narrow seam: only canonical root ownership and setup root establishment move behind the approved layout boundary here.
3. This slice may introduce `crates/compiler/src/layout.rs` as the initial compiler-local home for `RepoLayoutRoot` and `CanonicalLayout`; later slices may extend that family, but Slice 1.2 must not collapse the contract into one monolithic layout object.
4. Runtime-state reset behavior in `setup.rs` remains local for now; Slice 1.2 only adopts the canonical `.handbook` root ownership in setup, not the `.handbook/state/**` ownership reserved for Slice 1.3.
5. Canonical artifact identity semantics must stay unchanged: repo-relative paths, namespace directories, required-vs-optional posture, setup starter-template behavior, and refusal posture all remain the same after adoption.
6. CLI/product-shell wording, route-state adoption, authoring-root adoption, and Phase 2 target/template work remain out of scope.

## Objective

Land the first behavior-neutral layout adoption slice for Phase 1 by moving canonical artifact root ownership and setup bootstrap root establishment behind the approved canonical layout seam.

The user is the maintainer of the handbook workspace and the future reviewer of the extraction sequence. The immediate outcome is not a product behavior change. The immediate outcome is a narrow internal adoption that proves the Slice 1.1 layout contract can replace scattered canonical-root literals without widening into runtime-state or authoring work.

Success means all of the following are true:

- canonical `.handbook` root ownership has one typed compiler-local owner
- `canonical_artifacts.rs` consumes that owner instead of owning duplicated root/namespace rules
- `setup.rs` consumes that same owner for canonical-root inspection, repair, and starter-path planning
- runtime-state reset ownership remains deferred to Slice 1.3
- current canonical artifact and setup behavior remains intact

## Slice Scope

In scope:

- introduce the first compiler-local canonical layout owner surface
- move canonical `.handbook` root and canonical artifact namespace ownership behind that surface
- adopt `crates/compiler/src/canonical_artifacts.rs` onto the canonical layout owner
- adopt the canonical-root parts of `crates/compiler/src/setup.rs` onto the same owner
- preserve existing canonical artifact and setup semantics while improving ownership boundaries

Out of scope:

- moving `.handbook/state/**` ownership behind the runtime-state layout
- changing route-state, capture, provenance, or handoff path owners
- changing authoring root or lock-path ownership
- changing CLI/operator wording
- widening into Phase 2 target/template parameterization
- changing repo-relative canonical file locations

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`
- Current Slice 1.2 implementation corpus:
  - `crates/compiler/src/canonical_artifacts.rs`
  - `crates/compiler/src/setup.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/tests/canonical_artifacts_ingest.rs`
  - `crates/compiler/tests/setup.rs`

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- compiler-internal typed layout ownership surface under `crates/compiler/src/layout.rs`
- repo-local canonical storage under `.handbook/**`
- repo-local runtime state under `.handbook/state/**`, still owned outside this slice

## Commands

Slice 1.2 inventory query:

```bash
rg -n "\\.handbook|SYSTEM_ROOT_RELATIVE|relative_path|namespace_dir|join\\(\"\\.handbook\"\\)" \
  crates/compiler/src/canonical_artifacts.rs \
  crates/compiler/src/setup.rs
```

Primary packet verification rails:

```bash
cargo test -p handbook-compiler --test canonical_artifacts_ingest
cargo test -p handbook-compiler --test setup
```

Behavior-neutral compile rail:

```bash
cargo check -p handbook-compiler
```

Repo verification wall for a landed Slice 1.2 packet:

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-compiler --all-targets -- -D warnings
cargo test -p handbook-compiler --test canonical_artifacts_ingest
cargo test -p handbook-compiler --test setup
cargo check -p handbook-compiler
```

## Project Structure

```text
HANDBOOK_ENGINE_EXTRACTION_PLAN.md                                                   -> Root phase-order authority
docs/specs/handbook-engine-extraction-slice-map.md                                   -> Phase -> Slice -> Packet authority
docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-*.md
                                                                                     -> Slice 1.1 contract and inventory authority
docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-*.md
                                                                                     -> Slice 1.2 authority set
crates/compiler/src/layout.rs                                                        -> Initial compiler-local canonical layout owner introduced by this slice
crates/compiler/src/canonical_artifacts.rs                                           -> Canonical artifact identities, ingest, and descriptor consumption
crates/compiler/src/setup.rs                                                         -> Setup bootstrap/refresh flow that consumes canonical-root ownership
crates/compiler/src/lib.rs                                                           -> Compiler module wiring and any reviewed exports needed by this slice
crates/compiler/tests/canonical_artifacts_ingest.rs                                  -> Canonical root and artifact-ingest regression coverage
crates/compiler/tests/setup.rs                                                       -> Setup init/refresh/repair regression coverage
```

## Code Style

Prefer a narrow typed canonical owner over duplicated string literals or ad hoc helper pairs.

```rust
let repo = RepoLayoutRoot::new(repo_root)?;
let canonical = CanonicalLayout::new(&repo);

let system_root = canonical.system_root();
let descriptor = canonical.artifact(CanonicalArtifactKind::Charter);
```

Conventions for this slice:

- `CanonicalLayout` owns canonical root and canonical artifact path derivation
- `canonical_artifacts.rs` keeps ingest/presence semantics local but consumes layout accessors
- `setup.rs` keeps setup mode, refusal, and runtime-state reset flow local but consumes canonical-layout accessors for canonical-root ownership
- preserve existing repo-relative string identities in outputs and test expectations
- do not introduce a “global layout” type that also claims runtime-state, handoff, or authoring ownership in this slice

## Testing Strategy

Primary verification for this slice is **behavior-preserving adoption**, not new product functionality.

Test levels:

- targeted integration coverage in `crates/compiler/tests/canonical_artifacts_ingest.rs`
- targeted integration coverage in `crates/compiler/tests/setup.rs`
- `cargo check -p handbook-compiler` after the layout owner is integrated

Coverage expectations:

- canonical root status classification remains unchanged
- canonical artifact descriptor and identity semantics remain unchanged
- setup init/refresh/repair behavior remains unchanged
- setup reset-state behavior remains unchanged while still deferring runtime-state ownership to Slice 1.3
- no Slice 1.3 or Slice 1.4 adoption is required to land Slice 1.2

## Boundaries

- Always:
  - preserve the Slice 1.1 separate-layout-types contract
  - keep Slice 1.2 limited to Canonical root layout adoption in `canonical_artifacts.rs` and `setup.rs`
  - preserve canonical artifact identity semantics and setup outcome semantics
  - leave runtime-state reset ownership local until Slice 1.3
- Ask first:
  - changing canonical repo-relative paths or artifact names
  - widening into `route_state.rs`, `pipeline_capture.rs`, `pipeline_handoff.rs`, `stage_10_feature_spec_provenance.rs`, or `author/**`
  - changing CLI/operator wording or doctor guidance
  - introducing new crates, dependencies, or public API promises beyond what this slice needs
- Never:
  - collapse the layout family into one monolithic layout object
  - adopt `.handbook/state/**` ownership in this slice
  - change Feature Spec scaffold policy as part of canonical-root adoption
  - start Slice 1.3 or Slice 1.4 work from inside Slice 1.2

## Success Criteria

- Slice 1.2 introduces one compiler-local canonical layout owner surface in `crates/compiler/src/layout.rs`.
- `canonical_artifacts.rs` consumes that owner for canonical root and canonical artifact namespace/path derivation.
- `setup.rs` consumes that same owner for canonical-root inspection, repair, and setup-owned starter-file planning.
- Existing canonical artifact identities, setup starter-template behavior, and refusal behavior remain unchanged.
- `cargo test -p handbook-compiler --test canonical_artifacts_ingest` passes.
- `cargo test -p handbook-compiler --test setup` passes.
- `cargo check -p handbook-compiler` passes.
- Slice 1.3 and Slice 1.4 ownership adoption remains deferred.

## Open Questions

- Should the initial `CanonicalLayout` types remain compiler-internal after Slice 1.2, or should a later slice expose a reviewed public surface?
- Should canonical artifact descriptor helpers live entirely in `layout.rs`, or should `canonical_artifacts.rs` keep the descriptor table while consuming layout-owned root accessors?

## Packet Breakdown

### Packet 1.2.1: Canonical Artifact Root Adoption

Goal:

- introduce the canonical layout owner and move canonical artifact root ownership behind it

Required outcome:

- `crates/compiler/src/layout.rs` owns canonical root and canonical artifact path derivation
- `crates/compiler/src/canonical_artifacts.rs` becomes a consumer of that owner without changing artifact identity semantics

### Packet 1.2.2: Setup Bootstrap Root Adoption

Goal:

- route canonical-root setup behavior through the canonical layout owner while leaving runtime-state ownership local

Required outcome:

- `crates/compiler/src/setup.rs` no longer owns duplicated canonical-root literals such as direct `repo_root.join(".handbook")`
- setup bootstrap/refresh behavior still passes the existing setup verification wall
