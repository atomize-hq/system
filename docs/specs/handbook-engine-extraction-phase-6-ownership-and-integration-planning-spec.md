# Spec: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

## Status

- Packets 1 and 2 are now landed in this triplet.
- Packet 1 froze the authority chain, root decision rule, verification-time baseline, and hard planning-only scope guard.
- Packet 2 lands the handbook-owned imported-core boundary decisions for `handbook-engine` and `handbook-pipeline` only.
- This family remains intentionally **docs-only** and **planning-only**.
- Packets 3 and 4 are **not started** here and remain out of scope.
- Packet 1 verification-time repo-truth freeze (pre-landing baseline, not the later landed HEAD):
  - branch: `feat/seam-extraction`
  - pre-landing baseline HEAD: `01b50868599bc55e7680784a9b5b2dace5ab6042`
  - working tree posture at verification time: dirty only in unrelated local files `AGENTS.md` and `CLAUDE.md`
- `aa882af42792a250cc02a6740bd1e2123178caff..01b50868599bc55e7680784a9b5b2dace5ab6042` is the Packet 1 verification-time **docs-only** baseline range. Later docs-only commits may advance HEAD without changing the prerequisite authority truth frozen there.

## Packet 1 Objective

Freeze the authority chain, root decision rule, verification-time branch/pre-landing-baseline truth, and hard scope guard for the new Phase 6 ownership family so later planning packets cannot silently drift back to stale pre-READY assumptions.

Packet 1 does **not** make per-crate ownership decisions. It only establishes the authoritative starting point for that later work.

## Packet 2 Objective

Decide the handbook-owned imported-core posture for `handbook-engine` and `handbook-pipeline` separately.

Packet 2 must:

- keep the engine and pipeline calls distinct
- state the architectural owner for each crate
- state Substrate's posture for each crate
- state the intended import boundary for each crate
- name any residual cleanup seam explicitly instead of hiding it inside the ownership call
- keep `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` final posture work out of scope except for tiny consistency cross-references

## Authority Chain Frozen By Packet 1

1. `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` is the root authority.
2. `docs/specs/handbook-engine-extraction-slice-map.md` confirms Phases 1 through 5 are fully landed through Slice 5.3 and that Phase 6 is the next authoritative step.
3. `docs/specs/handbook-engine-extraction-closeout-four-set-map.md` records the honest Phase 1 through 5 closeout posture that Phase 6 inherits.
4. `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-{spec,plan,tasks}.md` are the prerequisite READY gate for this family.
5. `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md` remains useful background input, but it is **not** the final verdict because it predates the later boundary-fix and READY reassessment closeout.

## Root Ownership Decision Rule

Per `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` Phase 6:

- if a crate is still fundamentally handbook-domain, **handbook should own it and Substrate should import it**
- only move a crate into Substrate if its real center of gravity becomes **substrate-specific**

Packet 2 applies that rule crate-by-crate for the two imported-core candidates in scope here.

## Packet 2 Scope Guard

In scope for Packet 2:

- `handbook-engine` ownership/import verdict
- `handbook-pipeline` ownership/import verdict
- exact repo-level boundary text for both crates
- the post-`aa882af` generic default layout contract truth for the engine call
- the bounded compiler-backed fixture/support coupling truth for the pipeline call
- explicit naming of any later bounded cleanup seam required by the pipeline evidence

Out of scope for Packet 2:

- `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` final posture work beyond tiny cross-references strictly required for consistency
- any production code edits
- any crate publication or crates.io planning
- any Substrate integration implementation
- any claim that `handbook-pipeline` is fully decoupled while the live evidence still shows bounded compiler-backed fixture/support coupling
- packet-prompt authoring or approval

If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.

## Packet 2 Repo-Truth Evidence

Packet 2 is grounded in the required command wall plus the authority set:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` Phase 6 still says the default rule is handbook-owned/imported unless a crate becomes truly substrate-specific.
- `crates/engine/src/lib.rs` exports a narrow crate root around canonical artifacts, authored-truth parsing/rendering/validation, baseline validation, freshness, `CanonicalLayoutContract`, and `default_canonical_layout_contract`.
- `crates/engine/src/canonical_paths.rs` now exposes the generic `default_canonical_layout_contract()` over `CanonicalLayoutContract`; the prior handbook-product-specific default-layout naming blocker is gone.
- `crates/pipeline/src/lib.rs` exports the catalog/runtime wedge directly: declarative loading, compile, capture, handoff, route resolution, route-state, and setup surfaces.
- `cargo tree -p handbook-pipeline` shows a runtime dependency on `handbook-engine`, while the remaining `handbook-compiler` edge is a **dev-dependency** rather than the runtime center of gravity.
- `crates/pipeline/tests/pipeline_catalog.rs` still imports `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`, which proves bounded compiler-backed fixture/support coupling still exists.
- `cargo test -p handbook-engine --test canonical_artifacts_ingest` passes, including the non-default layout-contract coverage.
- `cargo test -p handbook-pipeline --test pipeline_catalog` passes, including the catalog/supported-target wedge checks and the template-library-backed declarative-source assertions.

## Packet 2 Decisions

### `handbook-engine`

- **Architectural owner:** handbook remains the architectural owner.
- **Substrate posture:** Substrate should import `handbook-engine` through the current public crate surface; Packet 2 does **not** require a thinner adapter before making the ownership call.
- **Exact repo-level boundary text to record:** `handbook-engine` remains a handbook-owned imported core. Substrate may depend on the existing public `handbook-engine` surface as the reusable canonical-artifact, validation, freshness, and layout-contract boundary. The generic layout seam is the exported `CanonicalLayoutContract` / `default_canonical_layout_contract` pair plus the shared workspace contract version, not a handbook-product-specific default path contract.
- **Residual cleanup deferred out of this packet:** if Substrate later wants a narrower consumer-specific façade for ergonomics, that is a later downstream integration-seam choice, not an ownership blocker and not Packet 2 work.

### `handbook-pipeline`

- **Architectural owner:** handbook remains the architectural owner.
- **Substrate posture:** Substrate should treat `handbook-pipeline` as a handbook-owned external core and should import only through a thinner reviewed integration boundary, not by blessing the entire current crate re-export surface as the durable importer contract.
- **Exact repo-level boundary text to record:** `handbook-pipeline` remains handbook-owned. Substrate may integrate through the catalog-backed supported-target wedge only: the reviewed loading/selection, compile, capture, handoff, and route-state surfaces needed for supported pipeline execution. Do **not** record the full public re-export surface as fully decoupled or move ownership into Substrate while bounded compiler-backed fixture/support coupling still exists.
- **Residual cleanup deferred out of this packet:** the pipeline-specific deferred seam is compiler-backed fixture/support decoupling for the catalog/runtime wedge, starting with `crates/pipeline/tests/pipeline_catalog.rs` importing `handbook_compiler::author::template_library`. That cleanup should become its own later bounded seam instead of being hidden inside the ownership verdict.

## Contradictions Vs. Prerequisite Authority Set

- **No contradiction found** in the prerequisite authority set used for Packet 2.
- Packet 2 resolves the per-crate imported-core call for `handbook-engine` and `handbook-pipeline` without reopening the Packet 1 READY gate or the root ownership rule.
- Packet 2 also preserves the root-plan caution that `handbook-pipeline` must not be overstated as fully generic while bounded compiler-backed fixture/support coupling remains.

## Deferred Later Packets (Not Started Here)

- **Packet 3:** decide handbook-side deferred boundaries and non-targets
- **Packet 4:** define downstream execution seams and the review gate

Those packets are framed here only so the family boundary stays honest; they remain out of scope for this landing.

## Required Verification For Packet 2

```bash
rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs
rg -n "default_canonical_layout_contract|workspace_contract_version" crates/engine/src/lib.rs crates/engine/src/canonical_paths.rs
rg -n "PipelineCapture|PipelineHandoff|RouteState|template_library|stage_10_feature_spec" crates/pipeline/src crates/pipeline/tests
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo test -p handbook-engine --test canonical_artifacts_ingest
cargo test -p handbook-pipeline --test pipeline_catalog
```

## Success Criteria

- the triplet keeps Packet 1's authority freeze intact
- `handbook-engine` and `handbook-pipeline` receive separate ownership/import calls
- the engine boundary explicitly reflects the generic `default_canonical_layout_contract` truth
- the pipeline boundary explicitly records the bounded compiler-backed fixture/support coupling instead of ignoring it or overstating it as a blocker
- the repo-level boundary text for each imported-core crate is explicit enough to quote in later planning work
- the pipeline-specific deferred cleanup seam is named explicitly rather than hidden inside the ownership verdict
- Packets 3 and 4 remain pending and out of scope
- the result is ready for orchestration review, but not execution-approved
