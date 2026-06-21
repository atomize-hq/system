# Spec: Handbook Published-Crate Readiness + Substrate Consumption

## Assumptions I'm Making

1. `system` remains the publication owner for `handbook-engine`, `handbook-pipeline`, and `handbook-flow`; this follow-on does not transfer architectural ownership into Substrate.
2. The first published wave still includes **only** `handbook-engine`, `handbook-pipeline`, and `handbook-flow` — not `handbook-cli` or `handbook-compiler`.
3. The target end state is **published-crate consumption from crates.io**, not the sibling path/workspace-member integration shape documented in the current Phase 6 adoption plan.
4. The first publication wave should use a coordinated release train for the three crates so downstream Substrate adoption can pin one coherent version set.
5. This triplet is a **post-Phase-6 follow-on authority**: it builds on the Lane A / Lane B / Lane D decisions already recorded, but it does not reopen the extraction architecture unless publication or published consumption proves a real blocker.

## Objective

Produce the single navigable planning authority for the remaining work required to make `handbook-engine`, `handbook-pipeline`, and `handbook-flow`:

1. **crates.io publish-ready** from `/Users/spensermcconnell/__Active_Code/system`, and
2. **Substrate-consume-ready via published crates** in `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate`.

This follow-on exists because live repo truth now shows a real gap between the completed Phase 6 import-planning posture and actual publication / published-consumption readiness:

- `cargo package -p handbook-engine --allow-dirty` currently passes.
- `handbook-pipeline` and `handbook-flow` now declare `handbook-engine` in publishable local-development form (`version + path`).
- `cargo package -p handbook-pipeline --allow-dirty` and `cargo package -p handbook-flow --allow-dirty` no longer fail on a **missing dependency version**; they now fail because Cargo resolves `handbook-engine` through the crates.io index during packaging, and that crate version is not yet published/resolvable there.
- `crates/pipeline/src/lib.rs` still publicly exposes items outside the documented frozen subset (`declarative_roots`, `setup`, and layout re-exports), so the published API surface is broader than the Phase 6 contract.
- `docs/specs/handbook-substrate-import-adoption-plan.md` still recommends **workspace-member/path dependency** consumption, which is intentionally different from the published-crate consumption target for this follow-on.

## Tech Stack

- Rust 2021 workspace in `system`
- Rust 2021 workspace in `atomize-hq/substrate`
- First-wave crates:
  - `crates/engine`
  - `crates/pipeline`
  - `crates/flow`
- Existing frozen-boundary authorities:
  - `docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `docs/specs/archive/phase-6-pipeline-boundary-cleanup/`
  - `docs/specs/handbook-substrate-import-adoption-plan.md` (current path/workspace-member plan; now input provenance, not final published-consumption authority)

## Commands

```bash
# system repo verification
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p handbook-engine
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-flow
cargo test -p handbook-compiler --test author

# manifest hardening / packageability signals in system
cargo package -p handbook-engine --allow-dirty
cargo package -p handbook-pipeline --allow-dirty
cargo package -p handbook-flow --allow-dirty

# staged release-session verification in system
# handbook-engine dry-run can run before any real publication
cargo publish --dry-run -p handbook-engine
# handbook-pipeline and handbook-flow dry-runs require a published/resolvable handbook-engine version
cargo publish --dry-run -p handbook-pipeline
cargo publish --dry-run -p handbook-flow

# actual publication (only in an explicitly authorized implementation session)
cargo publish -p handbook-engine
cargo publish -p handbook-pipeline
cargo publish -p handbook-flow

# downstream substrate verification after switching to published crates
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow
```

## Project Structure

```text
/Users/spensermcconnell/__Active_Code/system/
  crates/engine/Cargo.toml                 → publish metadata + version source for handbook-engine
  crates/pipeline/Cargo.toml               → publish metadata + versioned dependency on handbook-engine
  crates/flow/Cargo.toml                   → publish metadata + versioned dependency on handbook-engine
  crates/engine/src/lib.rs                 → published engine surface
  crates/pipeline/src/lib.rs               → published pipeline surface; must match frozen subset physically
  crates/flow/src/lib.rs                   → published flow surface; must keep caller-owned rendering out of boundary
  docs/specs/                              → active planning authorities for this follow-on

/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/
  Cargo.toml and affected member manifests → published dependency declarations for handbook crates
  downstream adapter / consumer call sites → only the files needed to consume the published crate surfaces
  verification artifacts / docs            → evidence that published-crate consumption works in live repo truth
```

## Code Style

Keep the published dependency contract explicit in manifests, and keep published public APIs physically aligned with the documented boundary.

```toml
# publishable intra-workspace dependency during local development
[dependencies]
handbook-engine = { version = "0.1.0", path = "../engine" }
```

```rust
// publish only the boundary we intend to support
mod declarative_roots;
mod setup;
pub mod pipeline;
pub mod pipeline_capture;
pub mod pipeline_compile;
pub mod pipeline_handoff;
pub mod pipeline_route;
pub mod route_state;

pub use pipeline::{load_pipeline_catalog, PipelineCatalog};
pub fn pipeline_contract_version() -> &'static str {
    handbook_engine::workspace_contract_version()
}
```

Conventions:
- published crates should expose only the reviewed surface, not merely document a narrower surface while exporting more in `lib.rs`
- manifest metadata should be complete enough for publication (`license` plus description/repository/doc metadata as chosen by the implementation session)
- internal dependency versioning should be explicit and coordinated across the first-wave release

## Testing Strategy

- **System repo manifest-hardening wall**
  - all existing workspace verification remains green
  - `handbook-engine` passes `cargo package`
  - `handbook-pipeline` and `handbook-flow` use explicit publishable `version + path` dependencies on `handbook-engine`
  - if `cargo package` still fails for `handbook-pipeline` or `handbook-flow` before release execution, the remaining failure must be registry resolution of unpublished `handbook-engine`, not a missing dependency version

- **Release-session dry-run wall**
  - `handbook-engine` passes `cargo publish --dry-run` before the first real publish
  - `handbook-pipeline` and `handbook-flow` pass `cargo publish --dry-run` only after the chosen `handbook-engine` version is published and resolvable from crates.io
  - source inspection confirms published APIs match the intended boundary contracts, especially for `handbook-pipeline`

- **Published boundary validation**
  - `handbook-pipeline` must physically narrow its public API to the documented frozen subset before publication, unless the boundary doc is explicitly widened by approved follow-on authority
  - `handbook-flow` must continue to expose only typed semantics on its published surface; final shell/operator wording stays out of boundary
  - `handbook-engine` either keeps its current surface as the accepted published API or gets an explicit optional freeze if publication review reveals a real need

- **Substrate published-consumption wall**
  - Substrate depends on published crate versions, not sibling path/workspace-member dependencies
  - Substrate builds and tests cleanly against the published versions
  - any needed adapter/rendering ownership remains in Substrate rather than leaking back into published handbook crate surfaces

## Boundaries

- **Always:**
  - Ground every claim in live repo truth (`cargo package`, staged `cargo publish --dry-run`, source inspection, downstream `cargo check`/`cargo test`) before recording it.
  - Keep the first-wave published set to `handbook-engine`, `handbook-pipeline`, and `handbook-flow` only.
  - Keep the published API physically aligned with the reviewed boundary contract; docs-only narrowing is not enough for publication.
  - Preserve unrelated local dirt, especially `AGENTS.md` and `CLAUDE.md`, while landing this follow-on.

- **Ask first:**
  - Publishing to crates.io for real.
  - Introducing release automation / CI changes instead of a minimal manual publish workflow.
  - Widening or breaking a documented public API boundary.
  - Renaming crates, changing package names, or changing the ownership decision that handbook remains the architectural owner.
  - Switching the downstream consumption target away from crates.io versions.

- **Never:**
  - Treat path/workspace-member consumption as proof of published-crate readiness.
  - Publish `handbook-cli` or `handbook-compiler` as part of this first-wave seam.
  - Fake publish-readiness based only on docs or `cargo check`; packaging and publish dry-runs are mandatory.
  - Widen into unrelated CLI redesign, compiler retirement, or broader Substrate product redesign.

## Success Criteria

1. The three manifests contain the publish-required metadata and coordinated internal dependency versioning needed for crates.io publication.
2. `handbook-engine` passes `cargo package`, and `handbook-pipeline` / `handbook-flow` no longer have any manifest-only packaging blocker beyond later crates.io resolution of `handbook-engine`.
3. `handbook-pipeline`'s actual public Rust surface matches the intended frozen first-wave boundary instead of exposing extra public modules/re-exports by accident.
4. `handbook-flow` keeps the cleaned consumer contract intact on the published surface: typed semantics in-boundary, final shell wording out-of-boundary.
5. The publication order, staged dry-run sequence, and publish verification wall are recorded as a durable authority that future sessions can execute without guessing.
6. The first-wave release session proves `cargo publish --dry-run` / `cargo publish` in an honest staged order: engine first, then dependent crates only after the published engine version is resolvable.
7. Substrate consumes the three crates via published crates.io versions and passes its verification wall without falling back to sibling path dependencies.
8. The resulting docs clearly separate:
   - architectural ownership (still handbook-owned)
   - publication readiness in `system`
   - downstream published-crate consumption in `substrate`

## Open Questions

1. Should Substrate pin exact versions (`=x.y.z`) for the first-wave adoption, or use compatible semver ranges (`^x.y.z`) with a lockfile update?
2. Which manifest metadata fields are required for the first publication wave beyond `license` — e.g. `repository`, `homepage`, `documentation`, `description`, `readme`, `keywords`, `categories`?
3. Is `handbook-engine`'s current public surface acceptable as the first published API, or should the optional engine boundary freeze activate before publication?
4. Should the first implementation wave include a minimal manual release checklist only, or also add repo-owned publish automation?
