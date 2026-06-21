# Tasks: Handbook Published-Crate Readiness + Substrate Consumption

Plan reference: [handbook-published-crates-and-substrate-consumption-plan.md](./handbook-published-crates-and-substrate-consumption-plan.md)

Spec reference: [handbook-published-crates-and-substrate-consumption-spec.md](./handbook-published-crates-and-substrate-consumption-spec.md)

---

## Lane 1: Manifest + Packaging Hardening

### Packet 1.1: Publish Metadata Baseline

- [ ] Task: Add the first-wave publication metadata to `handbook-engine`, `handbook-pipeline`, and `handbook-flow`
  - Acceptance: Each manifest contains the agreed publish metadata set for the first wave (`license` already present; remaining fields such as `description`, `repository`, `documentation`, `homepage`, `readme`, `keywords`, `categories` are either added or explicitly deferred by documented decision).
  - Verify: `cargo package -p handbook-engine --allow-dirty`; `cargo package -p handbook-pipeline --allow-dirty`; `cargo package -p handbook-flow --allow-dirty`
  - Files: `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, `crates/flow/Cargo.toml`, optional shared README/doc files if required

### Packet 1.2: Versioned Intra-Workspace Dependencies

- [ ] Task: Convert publishable internal dependencies from path-only declarations to versioned publishable declarations
  - Acceptance: `handbook-pipeline` and `handbook-flow` depend on `handbook-engine` using a publishable dependency form (for example `version + path` during local development), and `cargo package` no longer fails on missing dependency versions.
  - Verify: `cargo package -p handbook-pipeline --allow-dirty`; `cargo package -p handbook-flow --allow-dirty`
  - Files: `crates/pipeline/Cargo.toml`, `crates/flow/Cargo.toml`

### Packet 1.3: Publish Dry-Run Wall

- [ ] Task: Pass the publish dry-run wall for all three crates
  - Acceptance: `handbook-engine`, `handbook-pipeline`, and `handbook-flow` all pass `cargo publish --dry-run` from live repo truth.
  - Verify: `cargo publish --dry-run -p handbook-engine`; `cargo publish --dry-run -p handbook-pipeline`; `cargo publish --dry-run -p handbook-flow`
  - Files: manifests and any package-content / metadata files required by the dry-runs

---

## Lane 2: Physical Public-Boundary Hardening

### Packet 2.1: Pipeline Published API Freeze

- [ ] Task: Narrow `handbook-pipeline`'s public Rust surface to the documented frozen first-wave boundary
  - Acceptance: `crates/pipeline/src/lib.rs` publicly exposes only the approved first-wave boundary (`pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, `pipeline_contract_version()`), unless a consciously approved follow-on updates the contract docs to widen that surface.
  - Verify: Source inspection of `crates/pipeline/src/lib.rs`; `cargo check --workspace`; `cargo test -p handbook-pipeline --test pipeline_catalog`; `cargo test -p handbook-pipeline --test pipeline_compile`; `cargo test -p handbook-pipeline --test pipeline_capture`; `cargo test -p handbook-pipeline --test pipeline_handoff`
  - Files: `crates/pipeline/src/lib.rs`, any newly needed internal-only modules/tests, and any authority docs that must be refreshed for honesty

### Packet 2.2: Engine Published Surface Decision

- [ ] Task: Decide whether `handbook-engine`'s current public surface is the accepted first published API
  - Acceptance: Either (a) the current engine public surface is explicitly confirmed as the first published API with no code change, or (b) a narrow engine-freeze packet lands and the authority docs are updated accordingly.
  - Verify: Source inspection of `crates/engine/src/lib.rs`; `cargo test -p handbook-engine`; any consumer-contract doc written for the chosen decision
  - Files: `crates/engine/src/lib.rs` only if narrowing is required; otherwise the relevant authority docs

### Packet 2.3: Flow Published-Surface Revalidation

- [ ] Task: Revalidate `handbook-flow` as a publishable API after manifest/versioning changes
  - Acceptance: The cleaned flow consumer contract still matches the live published surface, and no publish-specific blocker remains beyond the already-documented typed boundary.
  - Verify: Source inspection of `crates/flow/src/lib.rs`; `cargo test -p handbook-flow`; `cargo publish --dry-run -p handbook-flow`
  - Files: `crates/flow/src/lib.rs` only if required, plus any consumer-contract doc refresh needed for honesty

---

## Lane 3: Release / Publish Choreography

### Packet 3.1: Release Contract + Checklist

- [ ] Task: Record the first-wave release contract for engine → pipeline → flow
  - Acceptance: A durable doc/checklist records the release order, chosen versioning policy, dependency pin semantics, dry-run prerequisites, and the evidence required before the first real publish.
  - Verify: Human review of the checklist against the live manifests and boundary docs.
  - Files: `docs/specs/handbook-published-crates-and-substrate-consumption-*.md` or a dedicated release-checklist doc if needed

### Packet 3.2: Real crates.io Publication

- [ ] Task: Publish `handbook-engine`, `handbook-pipeline`, and `handbook-flow` to crates.io in the approved order
  - Acceptance: All three crates are published at the approved coordinated versions, and the published versions match the release contract.
  - Verify: Successful `cargo publish` results recorded for each crate; published versions visible in crates.io metadata / cargo index resolution.
  - Files: manifests/version files and any release notes/checklist artifacts needed to record the publish event

---

## Lane 4: Published-Crate Consumption in Substrate

### Packet 4.1: Downstream Dependency Wiring

- [ ] Task: Replace the path/workspace-member adoption assumption with published-crate dependency wiring in Substrate
  - Acceptance: The relevant Substrate manifests depend on crates.io versions of `handbook-engine`, `handbook-pipeline`, and `handbook-flow` rather than sibling path dependencies for this first-wave seam.
  - Verify: Source inspection of `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/Cargo.toml` and affected member manifests; `cargo tree -p handbook-engine`; `cargo tree -p handbook-pipeline`; `cargo tree -p handbook-flow`
  - Files: `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/Cargo.toml`, affected member `Cargo.toml` files

### Packet 4.2: Downstream Consumer Adaptation

- [ ] Task: Update only the Substrate call sites/adapters needed to consume the published crate boundaries
  - Acceptance: Substrate uses the published handbook crate APIs without relying on sibling path behavior, leaked shell wording, or out-of-boundary pipeline internals.
  - Verify: `cargo check --workspace`; targeted source inspection of the touched adapter/call sites
  - Files: only the affected downstream adapter / consumer files discovered during implementation

### Packet 4.3: Substrate Verification Wall

- [ ] Task: Pass the full downstream published-consumption verification wall
  - Acceptance: Substrate builds, lints, and tests successfully against the published crate versions, and no fallback path dependency remains in the first-wave consumption path.
  - Verify: `cargo check --workspace`; `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo tree -p handbook-engine`; `cargo tree -p handbook-pipeline`; `cargo tree -p handbook-flow`
  - Files: downstream manifests, adapter/call-site files, and any substrate-side evidence docs needed to record the landing honestly

---

## Wider-Seam Guardrail

Stop after the three crates are honestly publish-ready, published, and consumed from Substrate via published crates. Do not:
- widen into publishing `handbook-cli` or `handbook-compiler`
- change architectural ownership away from handbook without new authority
- quietly preserve a broader published API than the documented contract
- treat path/workspace-member integration as proof of published-consumption readiness
- widen into unrelated Substrate redesign work

---

## Lane Status Summary

| Lane | Status | Blocks published consumption? |
|------|--------|-------------------------------|
| 1 | Not started | Yes |
| 2 | Not started | Yes |
| 3 | Not started | Yes |
| 4 | Not started | — |
