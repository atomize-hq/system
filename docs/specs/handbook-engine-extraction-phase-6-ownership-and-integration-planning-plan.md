# Plan: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

Spec reference: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md)

## Status

- Packets 1 and 2 are landed in this plan.
- Packet 1 froze the authority chain and hard planning-only scope guard.
- Packet 2 decides the handbook-owned imported-core boundaries for `handbook-engine` and `handbook-pipeline`.
- Packets 3 and 4 are intentionally **not started** here.
- This is still a docs-only, planning-only family; no implementation work is authorized.

## Objective

Apply the Phase 6 root ownership rule to the two imported-core candidates that matter first: `handbook-engine` and `handbook-pipeline`.

For this landing, success means:

- the engine and pipeline calls stay separate
- each crate has an explicit architectural owner
- each crate has an explicit Substrate posture and import boundary
- any residual cleanup seam is named honestly instead of being smuggled into the ownership decision

## Packet Order

### Packet 1: Freeze current authority and scope guard

Status: **already landed before this packet**

Packet 1 recorded the verification-time branch / baseline / dirty-tree posture, the docs-only baseline delta, the READY prerequisite gate, the root ownership rule, and the planning-only hard boundaries.

### Packet 2: Decide handbook-owned imported-core boundaries

Status: **landed in this change**

Packet 2 makes all of the following explicit:

- `handbook-engine` architectural ownership stays handbook-side
- whether Substrate can import `handbook-engine` through the current public surface
- the exact repo-level boundary text for `handbook-engine`
- `handbook-pipeline` architectural ownership stays handbook-side
- whether Substrate should import `handbook-pipeline` only through a thinner reviewed boundary
- the exact repo-level boundary text for `handbook-pipeline`
- the pipeline-specific deferred cleanup seam that remains separate from the ownership call

### Packet 3: Decide handbook-side deferred boundaries and non-targets

Status: **pending, out of scope here**

Future packet only. Do not start from this landing.

### Packet 4: Define downstream execution seams and review gate

Status: **pending, out of scope here**

Future packet only. Do not start from this landing.

## Packet 2 Execution Approach

1. verify the root ownership rule in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
2. verify the live `handbook-engine` public surface and the post-`aa882af` generic layout-contract export truth
3. verify the live `handbook-pipeline` public surface, dependency graph, and bounded compiler-backed fixture/support coupling
4. record separate engine and pipeline ownership/import decisions without collapsing them into one generic verdict
5. call out the residual pipeline cleanup as its own bounded deferred seam
6. leave `handbook-flow`, `handbook-cli`, retained `handbook-compiler` final posture work, and all implementation work out of scope

## Packet 2 Verification Outputs Used

- `rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs` -> `handbook-engine` exposes a narrow reusable crate root, while `handbook-pipeline` exposes the broader catalog/runtime wedge directly from `lib.rs`
- `rg -n "default_canonical_layout_contract|workspace_contract_version" crates/engine/src/lib.rs crates/engine/src/canonical_paths.rs` -> confirms the generic `default_canonical_layout_contract` export and shared workspace contract version are the current engine layout boundary
- `rg -n "PipelineCapture|PipelineHandoff|RouteState|template_library|stage_10_feature_spec" crates/pipeline/src crates/pipeline/tests` -> confirms the pipeline crate exports compile/capture/handoff/route-state surfaces and that the catalog test still reaches into compiler-owned `template_library`
- `cargo tree -p handbook-engine` -> shows only foundational runtime dependencies
- `cargo tree -p handbook-pipeline` -> shows the runtime dependency on `handbook-engine` and a remaining `handbook-compiler` **dev-dependency**, not a runtime ownership inversion
- `cargo test -p handbook-engine --test canonical_artifacts_ingest` -> passes, including the non-default layout-contract path coverage
- `cargo test -p handbook-pipeline --test pipeline_catalog` -> passes, including supported-target wedge checks and the template-library-backed declarative-source assertions

## Packet 2 Decision Summary

### `handbook-engine`

- handbook remains the architectural owner
- Substrate should import through the current public `handbook-engine` surface
- no thinner adapter is required to make the Packet 2 ownership call
- later consumer-specific ergonomic narrowing, if desired, belongs to a downstream integration seam rather than this packet

### `handbook-pipeline`

- handbook remains the architectural owner
- Substrate should treat the crate as handbook-owned external core and import only through a thinner reviewed boundary aligned to the supported-target wedge
- the full crate re-export surface is not yet the durable importer contract
- the remaining compiler-backed fixture/support coupling becomes a named later cleanup seam instead of an ownership blocker

## Risks And Mitigations

### Risk: the engine decision reintroduces the old handbook-product default-layout blocker

Mitigation:

- Packet 2 anchors the engine call to the live `CanonicalLayoutContract` / `default_canonical_layout_contract` export pair
- Packet 2 records that the generic default-layout naming blocker was already removed before this decision

### Risk: the pipeline decision overstates decoupling and hides residual compiler coupling

Mitigation:

- Packet 2 records the current `handbook_compiler` edge as bounded fixture/support coupling, not as the runtime center of gravity
- Packet 2 names that cleanup as a separate deferred seam instead of pretending the crate is fully decoupled today

### Risk: later work collapses engine and pipeline into one generic import verdict

Mitigation:

- Packet 2 records separate per-crate boundary text
- Packet 2 keeps different Substrate postures for the two crates: current public surface for engine, thinner reviewed boundary for pipeline

## Exit Condition For This Landing

This landing is complete when:

- Packet 2 is explicit in the spec/plan/tasks triplet
- `handbook-engine` and `handbook-pipeline` each have separate ownership/import boundary text
- the engine decision reflects the generic default-layout contract truth
- the pipeline decision reflects the bounded compiler-backed fixture/support coupling truth
- the pipeline-specific deferred cleanup seam is named explicitly
- Packets 3 and 4 remain pending and out of scope
- the result is ready for orchestration review and still not execution-approved
