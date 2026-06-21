# Plan: Handbook Published-Crate Readiness + Substrate Consumption

Spec reference: [handbook-published-crates-and-substrate-consumption-spec.md](./handbook-published-crates-and-substrate-consumption-spec.md)

## Overview

This follow-on is a four-lane execution sequence:

```text
Lane 1: Manifest hardening in system
    ↓
Lane 2: Physical public-boundary hardening in system
    ↓
Lane 3: Release / publish choreography in system
    ↓
Lane 4: Published-crate consumption in substrate
```

The order is intentional:
- publication metadata and dependency versioning must exist before the release session can stage dry-runs and publication honestly
- physical public-surface hardening must land before publication, otherwise crates.io would freeze the wrong API
- dependent-crate dry-runs are registry-resolved checks, so they belong with the staged release choreography after `handbook-engine` publication, not as Packet 1.2 acceptance
- actual publication must happen before downstream substrate can honestly prove published-crate consumption

## Lane 1: Manifest Hardening

### Goal

Make the three crate manifests structurally publishable and remove manifest-only blockers.

### Current State (live repo truth, 2026-06-21)

- The first-wave metadata baseline is now present in `handbook-engine`, `handbook-pipeline`, and `handbook-flow`.
- `handbook-engine` packages successfully.
- `handbook-pipeline` and `handbook-flow` now declare `handbook-engine` as `version + path` during local development.
- `cargo package -p handbook-pipeline --allow-dirty` and `cargo package -p handbook-flow --allow-dirty` no longer fail on a missing dependency version; the remaining failure is that `handbook-engine` is not yet published/resolvable from the crates.io index.

### Components

1. **Manifest metadata hardening**
   - keep the minimum metadata needed for first-wave publication honest
   - defer optional publication polish (`readme`, `keywords`, `categories`) unless a later packet explicitly pulls it in

2. **Versioned internal dependency contract**
   - keep intra-workspace handbook dependencies in publishable `version + path` form
   - treat removal of the missing-version failure as the Packet 1.2 completion condition

3. **Registry-resolved release proof handoff**
   - do not require dependent-crate dry-runs or successful dependent packaging as a standalone Lane 1 proof
   - carry the remaining crates.io-resolution proof into Lane 3's staged release choreography

### Risks

- Cargo packaging can still surface real manifest or package-content issues, so Lane 1 must distinguish manifest blockers from later registry-resolution blockers honestly.
- If internal version coordination drifts from the release contract, the first release wave may publish an incoherent set of crate versions.

### Verification Checkpoint

```bash
cargo package -p handbook-engine --allow-dirty
cargo package -p handbook-pipeline --allow-dirty
cargo package -p handbook-flow --allow-dirty
```

Interpretation:
- `handbook-engine` should pass.
- `handbook-pipeline` and `handbook-flow` may still fail before release execution, but only because Cargo cannot yet resolve the published `handbook-engine` version from crates.io.
- A missing dependency version is no longer an acceptable failure mode after Packet 1.2.

## Lane 2: Physical Public-Boundary Hardening

### Goal

Make the actual published Rust surfaces match the intended supported contract.

### Current State (live repo truth, 2026-06-21)

- `handbook-flow` already has an explicit consumer contract and cleaned public surface.
- `handbook-engine` currently looks narrow enough, but the Phase 6 docs still treated a stricter engine freeze as optional.
- `handbook-pipeline` is now physically aligned with the documented frozen subset: `crates/pipeline/src/lib.rs` exposes only the approved first-wave modules plus `pipeline_contract_version()`, and callers/tests now consume those items through the approved module paths.

### Components

1. **Pipeline physical API freeze**
   - narrow `crates/pipeline/src/lib.rs` so the published API matches the documented frozen subset
   - if a broader API is truly needed, update the authority docs explicitly instead of silently publishing the broader surface

2. **Engine publication posture decision**
   - either confirm the current engine surface is the accepted first published API
   - or activate the optional engine boundary freeze before publication

3. **Flow publication revalidation**
   - confirm that the publishable flow surface still matches the cleaned consumer contract once manifest hardening lands
   - keep registry-resolved dry-run proof for flow in Lane 3 rather than this lane

### Risks

- Narrowing the pipeline API may break current internal callers or tests.
- A real downstream consumer may reveal that the currently documented boundaries are too narrow or too broad.

### Verification Checkpoint

```bash
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p handbook-engine
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-flow
```

Plus required source inspection of:
- `crates/engine/src/lib.rs`
- `crates/pipeline/src/lib.rs`
- `crates/flow/src/lib.rs`

## Lane 3: Release / Publish Choreography

### Goal

Define and execute the first crates.io release wave without guessing.

### Components

1. **Release contract**
   - choose the versioning scheme for the first wave
   - define engine → pipeline → flow publish order
   - define the dependency pin policy (`version`, exact pin, or compatible range) for intra-wave coordination

2. **Staged dry-run sequence**
   - run `cargo publish --dry-run -p handbook-engine` before the first real publish
   - publish `handbook-engine`
   - wait until the chosen `handbook-engine` version is resolvable from crates.io
   - then run `cargo publish --dry-run -p handbook-pipeline` and `cargo publish --dry-run -p handbook-flow`

3. **Real publication step**
   - publish `handbook-pipeline`
   - then `handbook-flow`
   - record exact published versions and any release notes/checklist items needed for downstream consumers

### Risks

- crates.io publication is irreversible for each version, so the release contract must define the staged dry-run / publish order explicitly.
- If the crates.io index lags after engine publication, the dependent dry-runs may fail transiently even when the manifests are correct.
- If pipeline/flow depend on the wrong engine version, the release train becomes inconsistent.

### Verification Checkpoint

- Human review of the publish checklist before the first real `cargo publish`.
- `cargo publish --dry-run -p handbook-engine` succeeds before engine publication.
- After engine publication, `handbook-pipeline` and `handbook-flow` both pass `cargo publish --dry-run` only once the published engine version is resolvable.
- After publication, crates.io versions must match the documented release contract.

## Lane 4: Published-Crate Consumption in Substrate

### Goal

Replace the current path/workspace-member consumption assumption with honest proof that Substrate can consume the published crates from crates.io.

### Current State

- The current Phase 6 adoption plan intentionally recommends workspace-member/path dependency consumption.
- That is valid for the Phase 6 plan, but it is not proof of published-crate consumption readiness.

### Components

1. **Downstream dependency wiring**
   - update Substrate manifests to depend on the published handbook crate versions from crates.io
   - remove any fallback to sibling path dependencies for this seam

2. **Consumer adaptation**
   - update only the downstream call sites / adapters needed to use the published boundaries
   - keep rendering and product-shell wording in Substrate where the published flow contract expects caller ownership

3. **Downstream verification wall**
   - build, lint, and test Substrate against the published crate versions
   - confirm no path dependency remains in the first-wave consumption path

### Risks

- Downstream substrate may reveal a real need for a narrower adapter/facade after all, especially around `handbook-pipeline`.
- Published-crate consumption may expose version-selection or lockfile issues not visible in path-based development.

### Verification Checkpoint

Run in `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate`:

```bash
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow
```

## Cross-Lane Dependencies

```text
Lane 1 (manifest hardening) ───────► Lane 2 (publishable public surfaces)
Lane 1 + Lane 2 ───────────────────► Lane 3 (release contract → staged dry-runs → publish)
Lane 3 (real publish) ─────────────► Lane 4 (published-crate substrate consumption)
```

Notes:
- Lane 2 can start some review work in parallel with Lane 1, but the release-session dry-runs should wait until both lanes are done.
- Lane 4 should not claim success until Lane 3 has produced real published versions.

## Execution Summary

| Lane | Status | Blocks next lane? | Est. effort |
|------|--------|-------------------|-------------|
| 1 | Packets 1.1-1.2 landed; remaining proof handed to Lane 3 | Yes | Mostly already landed docs/manifests |
| 2 | Packet 2.1 landed; Packets 2.2-2.3 remain | Yes | One or more system-repo sessions |
| 3 | Not started | Yes | One staged release session after Lane 2 is green |
| 4 | Not started | — | One substrate integration session |
