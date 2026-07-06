# Plan: Handbook Substrate Parameterization — Set 1: Pipeline Import Layout

Spec reference: [handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md)

## Overview

Set 1 is the first active implementation set from the three-set map. Its purpose is to make `handbook-pipeline` support honest downstream placement under `.substrate/handbook/**` without requiring Substrate to mirror handbook's repo-level `core/**` and `.handbook/state/**` defaults.

This set is one triplet with four sequential packets:

```text
Packet 1.1 (declarative root contract)
  -> Packet 1.2 (stage-root adoption)
  -> Packet 1.3 (public storage-layout injection)
  -> Packet 1.4 (final set proof)
```

No packet in this set is parallel-safe by default. Each packet builds on the contract decisions from the previous packet.

## Current State (live repo truth)

- `crates/pipeline/src/declarative_roots.rs` now exposes a supported declarative-root contract plus an explicit handbook-product default helper.
- `crates/pipeline/src/layout.rs` now exposes a supported storage-layout contract plus an explicit handbook-product default helper.
- `crates/pipeline/src/pipeline.rs` now has import-facing `*_with_roots` discovery and validation entry points for the Packet 1.2 seam, while handbook-product helpers remain as explicit default behavior.
- `crates/pipeline/tests/pipeline_loader.rs` now proves positive non-default-stage-root acceptance through explicit declarative roots instead of codifying rejection semantics.
- Packets 1.1, 1.2, and 1.3 now appear landed; Packet 1.4 can run as the final proof seam.

## Components

### 1. Declarative layout owner

Create the import-facing owner for these roots:

- pipeline catalog root
- profile-pack root
- runner root
- stage catalog root

This component should:

- preserve handbook-product defaults through an explicit default helper
- give downstream importers a typed way to supply non-default roots
- keep root derivation in one place instead of scattered literals

### 2. Stage-root adoption inside pipeline catalog / supported-target flows

Move the remaining `core/stages/**` ownership onto the active declarative contract.

This component includes:

- supported target/source path derivation
- stage catalog discovery
- stage-file validation rules that must know the active stage root
- pipeline-path checks that must know the active pipeline root

This component does **not** include broad product-shell wording cleanup beyond what is inseparable from the structural root change.

### 3. Public pipeline storage-layout contract

Promote the existing typed storage contract into a supported public/import-facing seam.

This component must cover:

- runtime state root
- pipeline state directory
- stage-capture provenance directory
- capture-cache directory
- handoff feature-slice bundle root

This component must preserve:

- handbook-product default behavior through an explicit default helper
- containment validation for runtime-state-owned paths
- the separation between runtime-state roots and handoff artifact roots

### 4. Runtime caller adoption for route state / capture / handoff

Adopt the public storage contract in the runtime callers that actually use those paths:

- `route_state.rs`
- `pipeline_capture.rs`
- `pipeline_handoff.rs`

This is where the set becomes honest for downstream importers: they should no longer need crate-private access just to select non-default storage roots.

### 5. Final proof + bounded residual-default inventory

After the structural changes land, run a full verification wall and record what handbook-product defaults still remain intentionally bounded.

The final proof must distinguish:

- **acceptable retained defaults**: explicit handbook-product default helpers or product-default behavior that does not block the import-facing contract
- **not acceptable**: hidden structural dependence on repo-level `core/**` or `.handbook/state/**` for the supported import-facing path

## Packet Plan

## Packet 1.1 — Declarative Root Contract And Owner Boundary

### Goal

Introduce the public/import-facing declarative root contract and adopt it in the root-owner helpers.

### Work

- define the typed owner for pipeline/profile/runner/stage roots
- preserve handbook-product defaults through an explicit helper
- wire the contract into the root helper surface instead of raw `core/**` constants being the only model
- re-export the supported contract from `lib.rs` if needed for downstream importers

### Verification checkpoint

```bash
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
```

### Exit condition

The crate has a coherent declarative-layout contract story before stage discovery/validation adoption begins.

## Packet 1.2 — Stage-Root Discovery And Validation Adoption

### Goal

Move stage-source assumptions, discovery, and inseparable validation logic onto the active declarative contract.

### Closeout status (live repo truth, 2026-06-20)

Packet 1.2 was reopened briefly to clear a false-complete state, then re-landed. The import-facing discovery / validation paths now accept explicit active roots, and the old rejection-only loader proof is gone. Packet 1.4 may proceed without reopening Packet 1.2 again unless the final proof wall finds new contradictory evidence.

### Work

- remove raw structural ownership of `core/stages/**` and handbook-product default-root truth from supported-target derivation and catalog discovery
- drive stage discovery from the active stage root instead of handbook-product default discovery behavior
- update stage/pipeline root validation where the root must derive from the active contract
- replace the current loader proof that codifies rejection of non-default stage roots with positive contract-driven acceptance proof
- keep broader wording-only cleanup deferred to Set 3

### Verification checkpoint

```bash
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_route_resolution
```

### Exit condition

The crate's stage-root behavior is structurally contract-driven rather than handbook-product-default-driven, and the old non-default-stage-root rejection semantics are gone from the Packet 1.2 proof surface.

## Packet 1.3 — Public Pipeline Storage Layout Injection

### Goal

Promote the storage layout contract to the public/import-facing boundary and adopt it across runtime callers.

### Work

- make the storage layout contract publicly usable through the supported boundary
- preserve handbook-product defaults with an explicit helper
- adopt the contract in route-state persistence
- adopt the contract in capture provenance/cache behavior
- adopt the contract in handoff bundle behavior

### Verification checkpoint

```bash
cargo test -p handbook-pipeline --test pipeline_state_store
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
```

### Exit condition

A downstream importer can choose non-default state/capture/handoff roots through the supported crate boundary.

## Packet 1.4 — Final Set Proof

### Goal

Run the final wall and record bounded residual defaults honestly.

### Work

- run the verification wall
- inspect remaining fixed-root literals
- confirm any remaining handbook-product defaults are explicit and bounded
- record completion notes in the tasks doc

### Verification checkpoint

```bash
cargo test -p handbook-pipeline
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
rg -n "core/pipelines|core/profiles|core/runners|core/stages|\.handbook/state|artifacts/handoff/feature_slice" crates/pipeline/src crates/pipeline/tests
```

### Exit condition

The set is reviewable as a complete structural import-layout story and the tasks doc records any retained defaults honestly.

## Implementation Order

1. **Packet 1.1 first** because every later packet needs a declared contract owner to target.
2. **Packet 1.2 second** because stage discovery/validation must follow the new declarative root owner before final proofs can be trusted.
3. **Packet 1.3 third** because storage-layout injection is a distinct runtime seam and should not be mixed into the declarative-root packets.
4. **Packet 1.4 last** because the final proof depends on all structural packets already landing.

## Risks And Mitigations

### Risk 1: accidental widening into CLI/compiler product-shell cleanup

- **Why it matters:** many visible literals still exist outside the import-target seam.
- **Mitigation:** keep this set limited to `crates/pipeline/**` and only the validation/refusal changes that are inseparable from the structural root contract.

### Risk 2: exposing internal layout types too broadly or too opaquely

- **Why it matters:** a public seam that still requires crate-private knowledge is not a real import-facing contract.
- **Mitigation:** explicitly define the supported public helpers/types in `lib.rs` and keep handbook-product defaults visible as defaults, not hidden assumptions.

### Risk 3: brittle tests tied to old repo-level roots

- **Why it matters:** pipeline tests currently assert many `core/**` and `.handbook/state/**` paths directly.
- **Mitigation:** update only the tests whose assertions are inseparable from the new contract story; preserve product-default assertions where they are still valid for the default helper path.

### Risk 4: Set 1 accidentally absorbs consumer-ownership or broader honesty-cleanup work

- **Why it matters:** the three-set map depends on clean seams.
- **Mitigation:** keep `feature-slice-decomposer` generalization out of scope and defer broad literal/wording cleanup to Set 3.

## Verification Wall

Use this as the set-level proof wall for Packet 1.4:

```bash
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_route_resolution
cargo test -p handbook-pipeline --test pipeline_state_store
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
rg -n "core/pipelines|core/profiles|core/runners|core/stages|\.handbook/state|artifacts/handoff/feature_slice" crates/pipeline/src crates/pipeline/tests
```

## Packet Dependency Summary

| Packet | Depends on | Main seam | Proof focus |
| --- | --- | --- | --- |
| 1.1 | — | declarative root contract | catalog + loader baseline |
| 1.2 | 1.1 | stage-root discovery + validation | catalog + loader + compile + route resolution |
| 1.3 | 1.2 | public storage layout injection | state store + capture + handoff |
| 1.4 | 1.1–1.3 | final set proof | full verification wall + bounded residual-default inventory |
