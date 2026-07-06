# Spec: Handbook Engine Extraction Phase 1 Slice 3 - Stateful Storage Layout

## Assumptions

1. Slice 1.1 is already the accepted authority for the separate-layout-types contract and the Phase 1 ownership-domain names.
2. Slice 1.2 has already landed the initial compiler-local canonical layout owner in `crates/compiler/src/layout.rs`, and Slice 1.3 should extend that family rather than introduce a competing layout entrypoint.
3. Slice 1.3 is the next **code-adoption** slice in Phase 1: it adopts runtime-state, capture-provenance, and handoff-bundle storage ownership, but it must stay behavior-neutral.
4. Setup/operator wording around runtime-state remains product-shell-local here; Slice 1.3 moves storage ownership only, not CLI/help-text cleanup.
5. Capture, provenance, and handoff target identities remain frozen in this slice; removing hardcoded pipeline/stage/consumer ids belongs to Phase 2.
6. Canonical artifact semantics already frozen by Slice 1.2 must remain unchanged while storage owners underneath capture and handoff are adopted.

## Objective

Land the second behavior-neutral layout adoption slice for Phase 1 by moving runtime-state, capture-provenance, and handoff-bundle storage ownership behind the approved layout family without widening into target-contract or shell-wording work.

The user is the maintainer of the handbook workspace and the reviewer of the extraction sequence. The immediate outcome is not a product-surface change. The immediate outcome is a narrow internal adoption that proves the Slice 1.1 ownership domains for runtime state, capture provenance, and handoff bundle roots can replace scattered storage literals while preserving current routing, capture, provenance, and handoff behavior.

Success means all of the following are true:

- runtime-state storage ownership has one typed compiler-local owner
- capture-provenance storage ownership has a narrower typed owner instead of remaining embedded in `route_state.rs` or `pipeline_capture.rs`
- handoff bundle root ownership moves behind the same layout family without changing bundle contents or trust semantics
- `route_state.rs`, `stage_10_feature_spec_provenance.rs`, `pipeline_capture.rs`, and `pipeline_handoff.rs` consume layout accessors instead of owning duplicated storage roots
- current state-reset, route-basis, capture-cache, provenance, and handoff behavior remains intact

## Slice Scope

In scope:

- extend the compiler-local layout family for runtime-state, capture-provenance, and handoff-bundle ownership
- move `.handbook/state/**` root and per-pipeline route-state file ownership behind that surface
- move stage-capture provenance and capture-cache path ownership behind that surface
- move feature-slice handoff bundle root ownership behind that surface
- preserve existing route-state, capture, provenance, and handoff semantics while improving ownership boundaries

Out of scope:

- changing CLI/operator wording or next-safe-action copy
- changing supported pipeline, stage, or consumer identity rules
- changing canonical artifact identities or canonical layout ownership already frozen by Slice 1.2
- changing authoring roots or authoring lock ownership
- widening into Phase 2 target/template parameterization
- changing handoff manifest/read-allowlist/trust-matrix semantics beyond storage-root adoption

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-tasks.md`
- Current Slice 1.3 implementation corpus:
  - `crates/compiler/src/layout.rs`
  - `crates/compiler/src/route_state.rs`
  - `crates/compiler/src/stage_10_feature_spec_provenance.rs`
  - `crates/compiler/src/pipeline_capture.rs`
  - `crates/compiler/src/pipeline_handoff.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/tests/pipeline_state_store.rs`
  - `crates/compiler/tests/pipeline_route_resolution.rs`
  - `crates/compiler/tests/pipeline_capture.rs`
  - `crates/compiler/tests/pipeline_handoff.rs`

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- compiler-internal typed layout ownership surface under `crates/compiler/src/layout.rs`
- repo-local runtime state under `.handbook/state/**`
- repo-local stage-capture provenance under `.handbook/state/pipeline/stage_capture/**`
- repo-local capture cache under `.handbook/state/pipeline/capture/**`
- repo-local handoff bundles under `artifacts/handoff/feature_slice/**`

## Commands

Slice 1.3 inventory query:

```bash
rg -n "\\.handbook/state|artifacts/handoff/feature_slice|route_state_path|STAGE_10_FEATURE_SPEC_CAPTURE_PROVENANCE_PATH|capture_cache_repo_relative_path|bundle_root" \
  crates/compiler/src/layout.rs \
  crates/compiler/src/route_state.rs \
  crates/compiler/src/stage_10_feature_spec_provenance.rs \
  crates/compiler/src/pipeline_capture.rs \
  crates/compiler/src/pipeline_handoff.rs
```

Primary packet verification rails:

```bash
cargo test -p handbook-compiler --test pipeline_state_store
cargo test -p handbook-compiler --test pipeline_route_resolution
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
```

Behavior-neutral compile rail:

```bash
cargo check -p handbook-compiler
```

Repo verification wall for a landed Slice 1.3 packet:

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-compiler --all-targets -- -D warnings
cargo test -p handbook-compiler --test pipeline_state_store
cargo test -p handbook-compiler --test pipeline_route_resolution
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
cargo check -p handbook-compiler
```

## Project Structure

```text
HANDBOOK_ENGINE_EXTRACTION_PLAN.md                                                   -> Root phase-order authority
docs/specs/handbook-engine-extraction-slice-map.md                                   -> Phase -> Slice -> Packet authority
docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-*.md
                                                                                     -> Slice 1.1 ownership-domain authority
docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-*.md
                                                                                     -> Slice 1.2 canonical adoption authority
docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-*.md  -> Slice 1.3 authority set
crates/compiler/src/layout.rs                                                        -> Compiler-internal layout family extended by this slice
crates/compiler/src/route_state.rs                                                   -> Runtime-state storage, route-state persistence, reset traversal
crates/compiler/src/stage_10_feature_spec_provenance.rs                              -> Stage-capture provenance persistence and validation
crates/compiler/src/pipeline_capture.rs                                              -> Capture-cache storage and capture apply/rollback flow
crates/compiler/src/pipeline_handoff.rs                                              -> Feature-slice handoff bundle root ownership and bundle validation
crates/compiler/tests/pipeline_state_store.rs                                        -> Runtime-state persistence/reset regression coverage
crates/compiler/tests/pipeline_route_resolution.rs                                   -> Route-basis / route-state read semantics regression coverage
crates/compiler/tests/pipeline_capture.rs                                            -> Capture-cache and capture provenance regression coverage
crates/compiler/tests/pipeline_handoff.rs                                            -> Handoff bundle root and provenance regression coverage
```

## Code Style

Prefer narrow typed storage owners over ad hoc path assembly inside feature modules.

```rust
let repo = RepoLayoutRoot::new(repo_root);
let runtime_state = repo.runtime_state();
let capture = repo.capture_provenance();
let handoff = repo.handoff_bundle();

let state_file = runtime_state.pipeline_state_path(pipeline_id)?;
let capture_cache = capture.capture_cache_path(capture_id)?;
let provenance = capture.stage_10_feature_spec_capture_path();
let bundle_root = handoff.feature_slice_bundle_root(feature_id)?;
```

Conventions for this slice:

- `layout.rs` owns runtime-state root derivation, capture-provenance path derivation, and handoff bundle root derivation
- `route_state.rs` keeps route-state validation, serialization, reset ordering, and audit semantics local while consuming runtime-state layout accessors
- `stage_10_feature_spec_provenance.rs` keeps schema and provenance-match semantics local while consuming capture-provenance layout accessors
- `pipeline_capture.rs` keeps capture planning, refusal posture, rollback, and write ordering local while consuming capture-provenance layout accessors
- `pipeline_handoff.rs` keeps manifest/read-allowlist/trust semantics local while consuming handoff bundle layout accessors
- do not introduce a “global layout” type that simultaneously claims canonical, runtime-state, authoring, and Phase 2 target ownership in this slice

## Testing Strategy

Primary verification for this slice is **behavior-preserving storage adoption**, not new product functionality.

Test levels:

- targeted integration coverage in `crates/compiler/tests/pipeline_state_store.rs`
- targeted integration coverage in `crates/compiler/tests/pipeline_route_resolution.rs`
- targeted integration coverage in `crates/compiler/tests/pipeline_capture.rs`
- targeted integration coverage in `crates/compiler/tests/pipeline_handoff.rs`
- `cargo check -p handbook-compiler` after the storage owners are integrated

Coverage expectations:

- route-state file placement and runtime-state reset semantics remain unchanged
- route-basis and route-resolution behavior remains unchanged
- stage-10 feature-spec capture provenance schema and match semantics remain unchanged
- capture-cache identity, apply, rollback, and refusal semantics remain unchanged
- handoff bundle root placement, manifest/read-allowlist generation, and validation semantics remain unchanged
- no Phase 2 target-contract or Phase 3 shell-wording work is required to land Slice 1.3

## Boundaries

- Always:
  - preserve the Slice 1.1 separate-layout-types contract and Slice 1.2 canonical-layout decisions
  - keep Slice 1.3 limited to runtime-state, capture-provenance, and handoff-bundle storage ownership adoption
  - preserve route-state, capture, provenance, and handoff behavior/output semantics
  - keep target ids, consumer ids, and operator wording local until their approved later slices
- Ask first:
  - changing `.handbook/state/**` relative identities or handoff bundle placement under `artifacts/handoff/**`
  - widening into `setup.rs`, `doctor.rs`, `refusal.rs`, `pipeline_compile.rs`, `pipeline.rs`, or `author/**` beyond narrow compile-through wiring
  - changing CLI/operator wording or next-safe-action strings
  - introducing new crates, dependencies, or public API promises beyond what this slice needs
- Never:
  - collapse the layout family into one monolithic layout object
  - remove or reclassify supported pipeline/stage/consumer ids as part of this slice
  - change canonical artifact identity semantics as a side effect of storage adoption
  - start Slice 1.4, Phase 2, or Phase 3 shell cleanup from inside Slice 1.3

## Success Criteria

- Slice 1.3 extends the compiler-local layout family with typed runtime-state, capture-provenance, and handoff-bundle ownership surfaces.
- `route_state.rs` consumes the runtime-state layout owner for `.handbook/state/**` root and route-state file derivation.
- `stage_10_feature_spec_provenance.rs` and `pipeline_capture.rs` consume capture-provenance layout accessors for stage-capture and capture-cache storage.
- `pipeline_handoff.rs` consumes the handoff bundle layout owner for feature-slice bundle root derivation and bundle-relative writes/validation.
- Existing route-state reset, capture provenance, capture cache, and handoff bundle semantics remain unchanged.
- `cargo test -p handbook-compiler --test pipeline_state_store` passes.
- `cargo test -p handbook-compiler --test pipeline_route_resolution` passes.
- `cargo test -p handbook-compiler --test pipeline_capture` passes.
- `cargo test -p handbook-compiler --test pipeline_handoff` passes.
- `cargo check -p handbook-compiler` passes.
- Slice 1.4 and Phase 2 adoption remains deferred.

## Open Questions

- Should `RuntimeStateLayout` own temp route-state file placement as well as stable state-root paths, or should temp-file helpers remain local to `route_state.rs` while only stable storage roots move behind the layout seam?
- Should handoff bundle root normalization live entirely in the handoff layout owner, or should `pipeline_handoff.rs` keep normalization/validation local while consuming only the root-derivation accessors?

## Packet Breakdown

### Packet 1.3.1: Route-State And Stage-Provenance Layout Adoption

Goal:

- introduce the runtime-state and capture-provenance layout owners and move route-state / stage-capture provenance storage ownership behind them

Required outcome:

- `crates/compiler/src/layout.rs` owns runtime-state root, per-pipeline route-state file, and stage-capture provenance path derivation
- `crates/compiler/src/route_state.rs` and `crates/compiler/src/stage_10_feature_spec_provenance.rs` become consumers of those owners without changing reset, route-basis, or provenance-match semantics

### Packet 1.3.2: Capture And Handoff Layout Adoption

Goal:

- route capture-cache and handoff-bundle storage ownership through the extended layout family while leaving target identities and shell wording untouched

Required outcome:

- `crates/compiler/src/pipeline_capture.rs` no longer owns direct capture-cache path derivation such as `.handbook/state/pipeline/capture/{capture_id}.yaml`
- `crates/compiler/src/pipeline_handoff.rs` no longer owns direct feature-slice bundle-root derivation such as `artifacts/handoff/feature_slice/{feature_id}`
- capture and handoff behavior still passes the existing verification wall
