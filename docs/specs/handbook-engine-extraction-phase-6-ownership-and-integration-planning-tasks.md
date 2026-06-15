# Tasks: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

Plan reference: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md)

## Status

- Packets 1 and 2 are landed in this task ledger.
- This ledger remains planning-only and docs-only.
- Packets 3 and 4 are not started here.
- Packet 1 verification-time repo-truth freeze (pre-landing baseline, not the later landed HEAD):
  - branch: `feat/seam-extraction`
  - pre-landing baseline HEAD: `01b50868599bc55e7680784a9b5b2dace5ab6042`
  - working tree posture at verification time: dirty only in unrelated local files `AGENTS.md` and `CLAUDE.md`
  - `aa882af42792a250cc02a6740bd1e2123178caff..01b50868599bc55e7680784a9b5b2dace5ab6042` is the verification-time docs-only baseline range
  - later Packet 1 docs-only commits may advance HEAD without changing the prerequisite authority truth frozen there

## Implementation Authority Used

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md` as background only, not as the final verdict
- `crates/engine/src/lib.rs`
- `crates/engine/src/canonical_paths.rs`
- `crates/pipeline/src/lib.rs`
- `crates/pipeline/tests/pipeline_catalog.rs`

## Packet 1: Freeze Current Authority And Scope Guard

- [x] Task: Record the current repo-truth starting gate for this planning family
  - Acceptance: The triplet states the verification-time branch/pre-landing-baseline posture, the docs-only baseline delta from `aa882af...` to `01b5086`, and that Phase 6 Slice 1 is already READY before this family begins.
  - Verify: `git status --short --branch && git rev-parse HEAD && git log --oneline --decorate -20 && git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..HEAD && git diff --name-only aa882af42792a250cc02a6740bd1e2123178caff..HEAD && rg -n "READY|Packet 6\.1\.4|ownership-and-integration-planning|default_canonical_layout_contract" docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
  - Completion note: Recorded branch `feat/seam-extraction`, pre-landing verification baseline `01b50868599bc55e7680784a9b5b2dace5ab6042`, unrelated local dirt in `AGENTS.md` and `CLAUDE.md`, and a docs-only `aa882af... .. 01b5086` baseline delta across nine `docs/specs/` files. Later Packet 1 docs-only commits do not change that prerequisite authority truth. The prerequisite reassessment triplet remains **READY** and Packet 6.1.4 names this family as the next planning boundary.

- [x] Task: Re-state the root ownership rule and hard planning boundaries inside the triplet
  - Acceptance: The triplet makes explicit that handbook stays owner when the center of gravity is handbook-domain, Substrate only becomes owner if the crate becomes truly substrate-specific, and implementation stays out of scope.
  - Verify: `rg -n "handbook should own it and Substrate should import it|substrate-specific|planning-only|out of scope|GitNexus impact analysis" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`
  - Completion note: Packet 1 froze the Phase 6 root rule, kept the landing docs-only/planning-only, and explicitly deferred later packets.

- [x] Task: Check the triplet against the prerequisite authority set and list contradictions explicitly
  - Acceptance: Any contradiction is listed explicitly; if none exist, the triplet says so plainly.
  - Verify: `rg -n "Contradictions Vs\. Prerequisite Authority Set|No contradiction found|READY gate|outside Packet 1 authority" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`
  - Completion note: No contradiction was found in the prerequisite authority set. Packet 1 froze the READY gate without reopening it.

## Packet 2: Decide Handbook-Owned Imported-Core Boundaries

- [x] Task: Verify the `handbook-engine` public boundary and generic default-layout truth
  - Acceptance: The triplet records that `handbook-engine` remains handbook-owned, states whether Substrate should import through the current public surface or a thinner adapter, and explicitly reflects the generic `default_canonical_layout_contract` / `CanonicalLayoutContract` truth.
  - Verify: `rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs && rg -n "default_canonical_layout_contract|workspace_contract_version" crates/engine/src/lib.rs crates/engine/src/canonical_paths.rs && cargo tree -p handbook-engine && cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - Completion note: `crates/engine/src/lib.rs` exposes the reusable canonical-artifact, authored-truth, validation, freshness, and layout-contract surface; `crates/engine/src/canonical_paths.rs` exports the generic `default_canonical_layout_contract()` over `CanonicalLayoutContract`; `workspace_contract_version()` remains shared workspace truth; `cargo tree -p handbook-engine` stayed foundational; and `canonical_artifacts_ingest` passed, including non-default layout-contract coverage. Packet 2 therefore records `handbook-engine` as handbook-owned imported core that Substrate may consume through the current public crate surface.

- [x] Task: Verify the `handbook-pipeline` runtime wedge and bounded compiler-backed fixture/support coupling
  - Acceptance: The triplet records that `handbook-pipeline` remains handbook-owned, states whether Substrate should import through the full public surface or only a thinner reviewed boundary, and names the residual compiler-backed fixture/support coupling honestly.
  - Verify: `rg -n "pub use|pub mod|mod " crates/pipeline/src/lib.rs && rg -n "PipelineCapture|PipelineHandoff|RouteState|template_library|stage_10_feature_spec" crates/pipeline/src crates/pipeline/tests && cargo tree -p handbook-pipeline && cargo test -p handbook-pipeline --test pipeline_catalog`
  - Completion note: `crates/pipeline/src/lib.rs` re-exports the catalog/runtime wedge directly; `cargo tree -p handbook-pipeline` shows `handbook-engine` as the runtime dependency and `handbook-compiler` only as a dev-dependency; `crates/pipeline/tests/pipeline_catalog.rs` still imports `handbook_compiler::author::template_library`; and `pipeline_catalog` passed, including the supported-target wedge checks plus declarative-source assertions. Packet 2 therefore records `handbook-pipeline` as handbook-owned external core that Substrate should import only through a thinner reviewed boundary rather than by endorsing the full crate surface as fully decoupled.

- [x] Task: Record exact repo-level boundary text for engine and pipeline without collapsing the calls together
  - Acceptance: The triplet contains separate, quote-ready boundary text for `handbook-engine` and `handbook-pipeline` and keeps their Substrate postures distinct.
  - Verify: `rg -n "handbook-owned imported core|handbook-owned external core|thinner reviewed integration boundary|current public crate surface|CanonicalLayoutContract" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`
  - Completion note: The docs now state that `handbook-engine` stays handbook-owned and is importable through its current public crate surface, while `handbook-pipeline` stays handbook-owned but should be consumed only through the supported-target reviewed boundary rather than the entire re-export surface.

- [x] Task: Name the deferred pipeline-specific cleanup seam explicitly and keep later packets out of scope
  - Acceptance: The triplet names the residual pipeline seam instead of hiding it in the ownership verdict and leaves Packets 3 and 4 untouched.
  - Verify: `rg -n "compiler-backed fixture/support decoupling|template_library|Packet 3|Packet 4|out of scope" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`
  - Completion note: Packet 2 names compiler-backed fixture/support decoupling for the catalog/runtime wedge—starting with `crates/pipeline/tests/pipeline_catalog.rs` reaching into `handbook_compiler::author::template_library`—as its own later bounded seam. Packet 3 and Packet 4 remain pending and out of scope.

## Packet 3: Decide Handbook-Side Deferred Boundaries And Non-Targets

- [ ] Not started in this landing.
- [ ] Out of scope until a later explicit packet request.

## Packet 4: Define Downstream Execution Seams And Review Gate

- [ ] Not started in this landing.
- [ ] Out of scope until a later explicit packet request.

## Human Review Gate

Do not begin Packet 3, Packet 4, packet-prompt authoring, implementation, production code edits, crate moves, runtime behavior changes, CLI redesign, retained-compiler retirement work, or Substrate integration work from this ledger. Stop at Packet 2 and route the result to orchestration review.
