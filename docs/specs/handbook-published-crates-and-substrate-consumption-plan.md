# Plan: Handbook Published-Crate Readiness + Substrate Consumption

Spec reference: [handbook-published-crates-and-substrate-consumption-spec.md](./handbook-published-crates-and-substrate-consumption-spec.md)

## Overview

This follow-on is a four-lane execution sequence:

```text
Lane 1: Manifest + packaging hardening in system
    ↓
Lane 2: Physical public-boundary hardening in system
    ↓
Lane 3: Release / publish choreography in system
    ↓
Lane 4: Published-crate consumption in substrate
```

The order is intentional:
- publication metadata and dependency versioning must exist before dry-runs can pass
- physical public-surface hardening must land before publication, otherwise crates.io would freeze the wrong API
- actual publication must happen before downstream substrate can honestly prove published-crate consumption

## Lane 1: Manifest + Packaging Hardening

### Goal

Make the three crate manifests structurally publishable.

### Current State (live repo truth, 2026-06-21)

- `handbook-engine` packages successfully, but Cargo warns that the manifest has no description/documentation/homepage/repository metadata.
- `handbook-pipeline` fails `cargo package` because `handbook-engine` is declared as `path = "../engine"` with no version.
- `handbook-flow` fails `cargo package` for the same reason.
- The Phase 6 docs already landed `license = "MIT"`, so the license prerequisite is no longer the blocker.

### Components

1. **Manifest metadata hardening**
   - add the minimum metadata needed for first-wave publication
   - decide whether readme/homepage/documentation URLs are required now or can stay minimal

2. **Versioned internal dependency contract**
   - convert intra-workspace handbook dependencies to publishable `version + path` declarations
   - record the coordinated versioning policy for the first release wave

3. **Packaging wall**
   - all three crates must pass `cargo package`
   - all three crates must pass `cargo publish --dry-run`

### Risks

- Cargo packaging may surface missing metadata, package-content surprises, or publish-specific dependency issues not visible in workspace builds.
- If internal version coordination is sloppy, the first release wave may publish an incoherent set of crate versions.

### Verification Checkpoint

```bash
cargo package -p handbook-engine --allow-dirty
cargo package -p handbook-pipeline --allow-dirty
cargo package -p handbook-flow --allow-dirty
cargo publish --dry-run -p handbook-engine
cargo publish --dry-run -p handbook-pipeline
cargo publish --dry-run -p handbook-flow
```

## Lane 2: Physical Public-Boundary Hardening

### Goal

Make the actual published Rust surfaces match the intended supported contract.

### Current State (live repo truth, 2026-06-21)

- `handbook-flow` already has an explicit consumer contract and cleaned public surface.
- `handbook-engine` currently looks narrow enough, but the Phase 6 docs still treated a stricter engine freeze as optional.
- `handbook-pipeline` is not yet physically aligned with the documented frozen subset: `crates/pipeline/src/lib.rs` still publicly exposes `declarative_roots`, `setup`, and layout re-exports even though those are documented as out-of-boundary for first-wave import.

### Components

1. **Pipeline physical API freeze**
   - narrow `crates/pipeline/src/lib.rs` so the published API matches the documented frozen subset
   - if a broader API is truly needed, update the authority docs explicitly instead of silently publishing the broader surface

2. **Engine publication posture decision**
   - either confirm the current engine surface is the accepted first published API
   - or activate the optional engine boundary freeze before publication

3. **Flow publication revalidation**
   - confirm that the publishable flow surface still matches the cleaned consumer contract once manifests and versions are publication-ready

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

2. **Dry-run wall**
   - after Lane 1 + Lane 2 land, all three crates pass `cargo publish --dry-run`

3. **Real publication step**
   - publish `handbook-engine`
   - then `handbook-pipeline`
   - then `handbook-flow`
   - record exact published versions and any release notes/checklist items needed for downstream consumers

### Risks

- crates.io publication is irreversible for each version, so dry-run evidence must be complete before the real publish step.
- If engine publishes but pipeline/flow still fail dry-run or depend on the wrong engine version, the release train becomes inconsistent.

### Verification Checkpoint

- Human review of the publish checklist and dry-run evidence before the first real `cargo publish`.
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
Lane 1 (publishable manifests) ─────► Lane 2 (publishable public surfaces)
Lane 2 (boundary hardening) ────────► Lane 3 (dry-run + publish)
Lane 3 (real publish) ──────────────► Lane 4 (published-crate substrate consumption)
```

Notes:
- Lane 2 can start some review work in parallel with Lane 1, but real dry-runs should wait until both are done.
- Lane 4 should not claim success until Lane 3 has produced real published versions.

## Execution Summary

| Lane | Status | Blocks next lane? | Est. effort |
|------|--------|-------------------|-------------|
| 1 | Not started | Yes | One focused system-repo session |
| 2 | Not started | Yes | One or more system-repo sessions |
| 3 | Not started | Yes | One release session after dry-runs are green |
| 4 | Not started | — | One substrate integration session |
