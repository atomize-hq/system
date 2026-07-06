# Handbook Published-Import Decoupling Audit — 2026-06-23

## Purpose

This document records the current audit truth for whether `handbook-engine`, `handbook-flow`, and `handbook-pipeline` are sufficiently decoupled to support continued crates.io-based downstream consumption from Substrate or other projects.

It is written so a fresh session can pick up planning immediately without needing the full prior chat context.

## Executive Summary

### Bottom line

As of **2026-06-23**, the strong claim that **all seams are complete** is **not yet validated**.

The honest state is:

- **Validated**
  - `handbook-engine` is published and consumable from crates.io.
  - `handbook-flow` is published and consumable from crates.io.
  - Substrate has a **real downstream proof** that consumes published handbook crates through a narrow production seam.
  - The parameterization / honesty verification walls in `system` currently pass.

- **Not yet validated**
  - `handbook-pipeline` is **not yet proven as a genuinely reusable downstream import surface** for non-default layout consumption.
  - The docs currently overstate this point relative to live published-crate behavior.

### Most important finding

`handbook-pipeline`'s supposed import-facing parameterization seam is still not actually usable by an external crates.io consumer, because the relevant modules/types remain private in the published crate.

That means:

- the published-crate story is **real** for `engine` + `flow`
- the broader “flow/engine/pipeline are all fully decoupled” story is **not yet honest**

## Scope Audited

Primary repo:

- `/Users/spensermcconnell/__Active_Code/system`

Downstream proof worktree:

- `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054`

Authorities reviewed:

- `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-published-crates-and-substrate-consumption-spec.md`
- `/Users/spensermcconnell/__Active_Code/system/docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`
- `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md`
- `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-parameterization-set-2-flow-canonical-layout-injection-tasks.md`
- `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md`

## Docs-specs archive note

During this session, the prior active markdown planning/spec files under:

- `/Users/spensermcconnell/__Active_Code/system/docs/specs/`

were moved into:

- `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/`

so future active planning can start from a cleaner surface.

Important caveat for a fresh session:

- the repo currently has a broad `.gitignore` rule for `archive`, so the moved archive copies may be ignored by Git unless that ignore behavior is intentionally adjusted or the files are force-added during a later commit workflow

Key code inspected:

- `/Users/spensermcconnell/__Active_Code/system/crates/engine/src/lib.rs`
- `/Users/spensermcconnell/__Active_Code/system/crates/engine/src/canonical_paths.rs`
- `/Users/spensermcconnell/__Active_Code/system/crates/flow/src/lib.rs`
- `/Users/spensermcconnell/__Active_Code/system/crates/flow/src/resolver.rs`
- `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/lib.rs`
- `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/declarative_roots.rs`
- `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/layout.rs`
- `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054/crates/shell/src/execution/prompt_fulfillment.rs`

## Commands Re-run During Audit

### Publication / packageability checks in `system`

Re-run in `/Users/spensermcconnell/__Active_Code/system`:

```bash
cargo package -p handbook-engine --allow-dirty
cargo package -p handbook-pipeline --allow-dirty
cargo package -p handbook-flow --allow-dirty

cargo publish --dry-run -p handbook-engine
cargo publish --dry-run -p handbook-pipeline
cargo publish --dry-run -p handbook-flow
```

Result:

- all passed
- dry-runs warned that `handbook-engine`, `handbook-pipeline`, and `handbook-flow` version `0.1.1` already exist on crates.io

### Parameterization / honesty verification checks in `system`

Re-run in `/Users/spensermcconnell/__Active_Code/system`:

```bash
cargo test -p handbook-flow --test resolver_core
cargo test -p handbook-flow
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_loader
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_route_resolution
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-engine --test baseline_validation
cargo test -p handbook-engine --test author_core
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

Result:

- all passed on 2026-06-23

### Downstream proof revalidation in the Substrate worktree

Re-run in `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054`:

```bash
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow
cargo check --workspace
cargo test -p shell compose_prompt_with_host_toolbox_contract -- --nocapture
```

Result:

- all passed
- downstream proof still resolves published crates at exact `=0.1.1`

## What Is Successfully Decoupled Today

## 1. Publication readiness and crates.io resolution are real

Current live manifests in `system` show:

- `handbook-engine` version `0.1.1`
- `handbook-pipeline` version `0.1.1`
- `handbook-flow` version `0.1.1`

The package/dry-run wall is no longer hypothetical.

## 2. `handbook-flow` is a real downstream surface

Live `flow` exports include:

- `resolve(...)`
- `resolve_with_contract(...)`
- typed resolver request/result/refusal/blocker surfaces
- `flow_contract_version()`

This is important because the proofed Substrate seam uses:

- `handbook_flow::resolve_with_contract(...)`
- typed resolver semantics
- Substrate-owned rendering of those semantics

## 3. `handbook-engine` is a real downstream surface

The critical downstream engine surface currently validated is:

- `CanonicalLayoutContract`
- `default_canonical_layout_contract()`
- the canonical artifact / baseline model needed by flow

An external scratch consumer compiled successfully against crates.io `=0.1.1` using:

- `handbook_engine::CanonicalLayoutContract::from_paths(...)`
- `handbook_flow::resolve_with_contract(...)`

That is positive proof that `engine` + `flow` are not just internally coherent; they are usable by an external downstream crate.

## 4. Substrate has a real published-crate downstream proof

The dedicated proof worktree documents a real, narrow consumer seam:

- proof worktree:
  - `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054`
- proof commit:
  - `7f1db2eab613d103d91b4194492c56917ce7563b`
- changed production file:
  - `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054/crates/shell/src/execution/prompt_fulfillment.rs`

The proof consumes published handbook crates without sibling-path fallback and keeps final wording in Substrate.

## What Is Not Actually Finished

## 1. `handbook-pipeline` is not yet a verified public import seam

The most important audit finding is that the published pipeline crate does **not** currently expose the parameterization seam that the Set 1 docs describe as public/import-facing.

### Live code truth

In:

- `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/lib.rs`

the crate root keeps these modules private:

- `mod declarative_roots;`
- `mod layout;`

In:

- `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/declarative_roots.rs`

the core contract type is still:

- `pub(crate) struct PipelineDeclarativeRootsContract`

In:

- `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/layout.rs`

the storage contract type is still:

- `pub(crate) struct PipelineStorageLayoutContract`

and its helpers remain crate-private.

### External consumer proof failure

I tested this directly with a temporary standalone crate depending on:

```toml
handbook-pipeline = "=0.1.1"
```

and attempted to import:

```rust
use handbook_pipeline::layout::PipelineStorageLayoutContract;
```

That failed with:

```text
error[E0603]: module `layout` is private
```

This is the cleanest, highest-signal invalidation in the audit.

## 2. The docs currently overstate Set 1 completion

The Set 1 tasks file says downstream importers can access a supported public/import-facing storage layout contract and declarative contract, but the published crate surface does not actually make those seams available to external crates.

This means the planning docs and the live published API have drifted apart.

## 3. The Packet 4.2 proof never proved `pipeline` adoption

The Packet 4.2 proof findings are directionally correct, but they are narrower than the broader decoupling claim:

- they validate real published consumption
- they specifically validate `handbook_flow` + `handbook_engine`
- they explicitly warn that `handbook-pipeline` is not yet a drop-in Substrate foundation

That warning should be treated as still active.

## Why This Matters For Planning

If new handbook work begins immediately without closing this gap, the team risks:

1. assuming `pipeline` is already a stable public downstream contract when it is not
2. adding new features that accidentally deepen internal coupling before the public boundary is made honest
3. shipping more docs that describe a reusable pipeline import seam that external consumers still cannot actually use

## Honest State Label

The best concise state label right now is:

> **Published-crate downstream consumption is proven for `handbook-engine` and `handbook-flow`; `handbook-pipeline` still needs public-boundary remediation before the full decoupling story is honest.**

## Recommended Immediate Planning Track

The next planning work should focus on one narrow remediation seam before broader feature work assumes full decoupling.

## Planning Goal A — Make the pipeline import seam actually public

Need to decide the intended supported external surface for:

- declarative root ownership
- storage layout ownership
- any explicit default helper that remains handbook-product-owned

At minimum, planning should answer:

1. Should `handbook_pipeline::declarative_roots` become public?
2. Should `handbook_pipeline::layout` become public?
3. Should the public contract be exposed as modules, re-exported types, or a narrower façade?
4. Which default-helper surfaces remain intentionally product-default versus reusable-import API?

## Planning Goal B — Add guard rails so this cannot silently regress

After the public-boundary fix, add guard rails such as:

1. **external consumer smoke tests**
   - one scratch consumer for `engine + flow`
   - one scratch consumer for `pipeline` non-default layout use

2. **published API boundary checks**
   - focused inspection or automated API-surface diffing for:
     - `/Users/spensermcconnell/__Active_Code/system/crates/engine/src/lib.rs`
     - `/Users/spensermcconnell/__Active_Code/system/crates/flow/src/lib.rs`
     - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/lib.rs`

3. **mandatory publish-wall reruns for import-surface changes**
   - `cargo package`
   - `cargo publish --dry-run`
   - external consumer smoke
   - downstream Substrate smoke

4. **boundary-ownership discipline**
   - handbook owns typed planning/readiness semantics
   - Substrate owns operator/runtime wording

## Recommended Next Planning Packet

If a fresh session is continuing planning rather than implementing immediately, the next document/work packet should probably do this:

### Packet: Pipeline public-boundary honesty remediation plan

Objective:

- reconcile the Set 1 docs with live published-crate truth
- define the smallest honest public API needed for downstream consumers
- define proof requirements for closing the gap

Suggested deliverables:

1. one new active planning doc for pipeline-boundary remediation
2. one acceptance checklist for external consumer proof
3. one guard-rail checklist for future publish/import changes

## Fresh-Session Pickup Instructions

If a new session resumes from this document, start here:

1. Re-read this file fully.
2. Reconfirm the live code truth in:
   - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/lib.rs`
   - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/declarative_roots.rs`
   - `/Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/layout.rs`
3. Re-run the external consumer proof:
   - positive: `engine + flow`
   - negative or newly fixed: `pipeline`
4. Decide whether the next session is:
   - **planning-only** (author a remediation spec/plan/tasks triplet), or
   - **implementation** (make the pipeline seam public and add smoke-proof guard rails)
5. Do **not** claim full decoupling unless the external crates.io consumer can actually use the pipeline contract surfaces.

## Critical Evidence Summary

### Positive evidence

- publication/packageability wall passes
- parameterization/honesty verification wall passes
- Substrate proof worktree consumes published crates at `=0.1.1`
- external scratch consumer for `engine + flow` compiles successfully

### Negative evidence

- external scratch consumer for `pipeline` fails because the module is private
- live `pipeline` contract types remain `pub(crate)`
- Set 1 docs currently overclaim public downstream accessibility

## Open Questions

1. Is the intended long-term public downstream surface for `pipeline` the current contract types, or should there be a narrower façade?
2. Should downstream consumers directly construct pipeline layout/declarative-root contracts, or should handbook expose a smaller validated builder API?
3. Which parts of pipeline layout/default ownership are product-default-only and should intentionally remain outside the reusable downstream contract?
4. Should the next work be docs-first planning, or should the repo go straight to a bounded implementation/remediation packet?

## Suggested One-Sentence Resume Prompt

> Re-read `HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`, verify that `handbook-pipeline`'s supposed public import seam is still private in the published crate, and then author the next bounded planning artifact for pipeline public-boundary remediation plus future guard rails.
