# Tasks: Handbook Published-Crate Readiness + Substrate Consumption

Plan reference: [handbook-published-crates-and-substrate-consumption-plan.md](./handbook-published-crates-and-substrate-consumption-plan.md)

Spec reference: [handbook-published-crates-and-substrate-consumption-spec.md](./handbook-published-crates-and-substrate-consumption-spec.md)

---

## Lane 1: Manifest Hardening

### Packet 1.1: Publish Metadata Baseline

- [x] Task: Add the first-wave publication metadata to `handbook-engine`, `handbook-pipeline`, and `handbook-flow`
  - Acceptance: Each manifest contains the agreed publish metadata set for the first wave (`license` already present; first-wave baseline now includes `description`, `repository`, `documentation`, and `homepage`, while `readme`, `keywords`, and `categories` are intentionally deferred to later publication polish).
  - Verify: Source inspection of `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, and `crates/flow/Cargo.toml`; `cargo package -p handbook-engine --allow-dirty`
  - Files: `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, `crates/flow/Cargo.toml`, optional shared README/doc files if required

### Packet 1.2: Versioned Intra-Workspace Dependencies

- [x] Task: Convert publishable internal dependencies from path-only declarations to versioned publishable declarations
  - Acceptance: `handbook-pipeline` and `handbook-flow` depend on `handbook-engine` using a publishable dependency form (`version + path` during local development), and pre-release `cargo package` no longer fails because `handbook-engine` lacks a dependency version. Any remaining dependent-crate failure at this stage must be the later release-sequencing condition that the chosen `handbook-engine` version is not yet resolvable from crates.io.
  - Verify: Source inspection of `crates/pipeline/Cargo.toml` and `crates/flow/Cargo.toml`; `cargo package -p handbook-pipeline --allow-dirty`; `cargo package -p handbook-flow --allow-dirty`
  - Files: `crates/pipeline/Cargo.toml`, `crates/flow/Cargo.toml`

Lane 1 stop: after Packets 1.1 and 1.2 land, move any remaining registry-resolved package / dry-run proof into Lane 3 rather than treating it as a manifest-only blocker.

---

## Lane 2: Physical Public-Boundary Hardening

### Packet 2.1: Pipeline Published API Freeze

- [x] Task: Narrow `handbook-pipeline`'s public Rust surface to the documented frozen first-wave boundary
  - Acceptance: `crates/pipeline/src/lib.rs` publicly exposes only the approved first-wave boundary (`pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, `pipeline_contract_version()`), unless a consciously approved follow-on updates the contract docs to widen that surface.
  - Verify: Source inspection of `crates/pipeline/src/lib.rs`; `cargo check --workspace`; `cargo test -p handbook-pipeline --test pipeline_catalog`; `cargo test -p handbook-pipeline --test pipeline_compile`; `cargo test -p handbook-pipeline --test pipeline_capture`; `cargo test -p handbook-pipeline --test pipeline_handoff`
  - Files: `crates/pipeline/src/lib.rs`, any newly needed internal-only modules/tests, and any authority docs that must be refreshed for honesty

### Packet 2.2: Engine Published Surface Decision

- [x] Task: Decide whether `handbook-engine`'s current public surface is the accepted first published API
  - Decision (2026-06-22): accept the current `crates/engine/src/lib.rs` surface as the first published API; no narrower engine-freeze follow-on is required for this packet.
  - Acceptance: Option (a) is chosen: the current engine public surface is explicitly confirmed as the first published API with no code change. If release review or a real downstream consumer later proves a concrete need for a narrower engine boundary, open that as a separate follow-on instead of widening Packet 2.2.
  - Verify: Source inspection of `crates/engine/src/lib.rs`; `cargo test -p handbook-engine`; `cargo package -p handbook-engine --allow-dirty`; refreshed Packet 2.2 authority docs
  - Files: `docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`, `docs/specs/handbook-published-crates-and-substrate-consumption-plan.md`, `docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md`

### Packet 2.3: Flow Published-Surface Revalidation

- [x] Task: Revalidate `handbook-flow` as a publishable API after manifest/versioning changes
  - Decision (2026-06-22): the current `crates/flow/src/lib.rs` surface still matches the cleaned consumer contract after the publish-metadata / versioned-dependency hardening in `crates/flow/Cargo.toml`, so Packet 2.3 lands as docs-only.
  - Acceptance: The cleaned flow consumer contract still matches the live published surface, and no publish-surface blocker remains beyond the already-documented typed boundary plus the later release-sequencing requirement that `handbook-engine` be published/resolvable for dependent dry-runs.
  - Verify: Source inspection of `crates/flow/src/lib.rs`; `cargo test -p handbook-flow`
  - Files: `crates/flow/src/lib.rs` only if required, plus any consumer-contract doc refresh needed for honesty

---

## Lane 3: Release / Publish Choreography

### Packet 3.1: Release Contract + Checklist

- [x] Task: Record the first-wave release contract for engine → pipeline → flow
  - Decision (2026-06-22): Packet 3.1 lands as docs-only via `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md`, which fixes the first-wave train at coordinated `0.1.0` unless a real pre-publish crate/manifest/boundary fix forces a full-train bump before any real publish, approves the `handbook-engine` → `handbook-pipeline` → `handbook-flow` order, keeps the Packet 1.2 `version + path` manifest contract in `system`, requires exact downstream `=` pins for the first Substrate published-consumption proof, and names the checklist as the Packet 3.2 execution-evidence ledger.
  - Acceptance: A durable doc/checklist records the release order, chosen versioning policy, dependency pin semantics, the staged dry-run sequence (`engine` prepublish dry-run, then dependent dry-runs only after the published engine version is resolvable), and the evidence required before each real publish step.
  - Verify: Human review of the checklist against the live manifests, current packageability truth, and boundary docs; source inspection of `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, and `crates/flow/Cargo.toml`; `cargo package -p handbook-engine --allow-dirty`; `cargo package -p handbook-pipeline --allow-dirty`; `cargo package -p handbook-flow --allow-dirty`
  - Files: `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md`, `docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`, `docs/specs/handbook-published-crates-and-substrate-consumption-plan.md`, `docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md`

### Packet 3.2: Staged Dry-Run + Real crates.io Publication

- [x] Task: Execute the staged first-wave release for `handbook-engine`, `handbook-pipeline`, and `handbook-flow` in the approved order
  - Decision (2026-06-22): initial Packet 3.2 execution on crate-source commit `5c5bf437168b47c9ab749aee5307a190841502d8` proved the engine dry-run but exposed a failing `cargo fmt --all -- --check` gate before any real publish. Removing that blocker required changing first-wave crate source, so the coordinated train was bumped to `0.1.1` per the release contract and the blocker-removal crate-source commit became `b88086ae58a66c8d9c6adf71e98a9555ee5c6e9a`.
  - Status: published on `0.1.1`; `cargo publish -p handbook-engine`, `cargo publish --dry-run -p handbook-pipeline`, `cargo publish --dry-run -p handbook-flow`, `cargo publish -p handbook-pipeline`, and `cargo publish -p handbook-flow` all succeeded, and `cargo search` now shows `handbook-engine`, `handbook-pipeline`, and `handbook-flow` at `0.1.1`.
  - Acceptance: `handbook-engine` passes `cargo publish --dry-run` and is published first; once that published version is resolvable from crates.io, `handbook-pipeline` and `handbook-flow` both pass `cargo publish --dry-run` and are then published in the approved order; the published versions match the release contract; the release-candidate commit hash, dry-run outputs, real publish outputs, final published versions, and any partial-wave stop state are recorded in `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md` before plan/tasks status are refreshed.
  - Verify: `cargo publish --dry-run -p handbook-engine`; successful `cargo publish -p handbook-engine`; successful dependent dry-runs for `handbook-pipeline` and `handbook-flow` after engine resolution; successful `cargo publish -p handbook-pipeline`; successful `cargo publish -p handbook-flow`; published versions visible in crates.io metadata / cargo index resolution.
  - Files: release-candidate manifests/version fields as needed for the chosen train; `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md` for execution evidence; `docs/specs/handbook-published-crates-and-substrate-consumption-plan.md` and `...-tasks.md` for post-evidence status/handoff refresh only

---

## Lane 4: Published-Crate Consumption in Substrate

### Packet 4.1: Downstream Dependency Wiring

- [x] Task: Replace the path/workspace-member adoption assumption with published-crate dependency wiring in Substrate
  - Status (2026-06-22): landed in Substrate commit `017aaec75` (`feat: wire substrate to published handbook crate pins`). The root workspace now pins `handbook-engine`, `handbook-pipeline`, and `handbook-flow` to exact `=0.1.1` versions, `crates/shell/Cargo.toml` consumes them via `workspace = true`, and `Cargo.lock` resolves all three from crates.io.
  - Acceptance: The relevant Substrate manifests depend on the exact published `=` versions recorded by Packet 3.2 for `handbook-engine`, `handbook-pipeline`, and `handbook-flow`, rather than sibling path dependencies for this first-wave seam.
  - Verify: Source inspection of `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/Cargo.toml`, `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/crates/shell/Cargo.toml`, and `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/Cargo.lock`; `cargo tree -p handbook-engine`; `cargo tree -p handbook-pipeline`; `cargo tree -p handbook-flow`
  - Files: `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/Cargo.toml`, `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/crates/shell/Cargo.toml`, `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate/Cargo.lock`

### Packet 4.2: Downstream Consumer Adaptation

- [ ] Task: Update only the Substrate call sites/adapters needed to consume the published crate boundaries
  - Execution note: create a fresh dedicated Substrate worktree under `/Users/spensermcconnell/.codex/worktrees/` from the current Substrate tip before any Packet 4.2 source edits; do not edit the main `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate` checkout directly.
  - Acceptance: Substrate uses the published handbook crate APIs without relying on sibling path behavior, leaked shell wording, or out-of-boundary pipeline internals, and the Packet 4.2 edits remain confined to the dedicated worktree plus the discovered adapter/call-site files.
  - Verify: `cargo check --workspace` from the dedicated Packet 4.2 worktree; targeted source inspection of the touched adapter/call sites
  - Files: only the affected downstream adapter / consumer files discovered during implementation

### Packet 4.3: Substrate Verification Wall

- [ ] Task: Pass the full downstream published-consumption verification wall
  - Execution note: reuse the dedicated Packet 4.2 Substrate worktree if it exists; otherwise create a fresh dedicated worktree under `/Users/spensermcconnell/.codex/worktrees/` before running the verification/fix loop.
  - Acceptance: Substrate builds, lints, and tests successfully against the published crate versions from the dedicated downstream worktree, and no fallback path dependency remains in the first-wave consumption path.
  - Verify: `cargo check --workspace`; `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `cargo test --workspace`; `cargo tree -p handbook-engine`; `cargo tree -p handbook-pipeline`; `cargo tree -p handbook-flow` — all from the dedicated downstream worktree
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
| 1 | Packet 1.1 + 1.2 landed; remaining proof moved to Lane 3 | Yes |
| 2 | Packets 2.1-2.3 landed | Yes |
| 3 | Packets 3.1-3.2 complete; first-wave crates published at `0.1.1` | No |
| 4 | Packet 4.1 landed; Packet 4.2 next in dedicated worktree | — |
