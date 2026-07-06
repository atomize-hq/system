# Plan: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

Spec reference: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md)

## Status

- Packets 1, 2, and 3 are landed in this plan.
- Packet 1 froze the authority chain and hard planning-only scope guard.
- Packet 2 decides the handbook-owned imported-core boundaries for `handbook-engine` and `handbook-pipeline`.
- Packet 3 decides the handbook-side deferred boundaries and non-targets for `handbook-flow`, `handbook-cli`, and retained `handbook-compiler`.
- Packet 4 lands only the bounded downstream execution seam map plus the final human review gate.
- This is still a docs-only, planning-only family; no implementation work is authorized.

## Objective

Apply the Phase 6 root ownership rule across the imported-core and handbook-side deferred-boundary calls that Phase 6 needs first: `handbook-engine`, `handbook-pipeline`, `handbook-flow`, `handbook-cli`, and retained `handbook-compiler`.

For this landing, success means:

- the engine and pipeline calls stay separate
- the flow, CLI, and retained compiler calls also stay separate
- each crate has an explicit architectural owner
- each crate has an explicit Substrate posture and import boundary
- any residual cleanup seam is named honestly instead of being smuggled into the ownership decision
- Packet 3 names which support surfaces remain deferred rather than pretending every handbook-side seam is already settled
- Packet 4 names the bounded downstream execution seams without starting them
- the planning family ends at an explicit human review gate before any execution work

## Packet Order

### Packet 1: Freeze current authority and scope guard

Status: **already landed before this packet**

Packet 1 recorded the verification-time branch / baseline / dirty-tree posture, the docs-only baseline delta, the READY prerequisite gate, the root ownership rule, and the planning-only hard boundaries.

### Packet 2: Decide handbook-owned imported-core boundaries

Status: **already landed before this packet**

Packet 2 makes all of the following explicit:

- `handbook-engine` architectural ownership stays handbook-side
- whether Substrate can import `handbook-engine` through the current public surface
- the exact repo-level boundary text for `handbook-engine`
- `handbook-pipeline` architectural ownership stays handbook-side
- whether Substrate should import `handbook-pipeline` only through a thinner reviewed boundary
- the exact repo-level boundary text for `handbook-pipeline`
- the pipeline-specific deferred cleanup seam that remains separate from the ownership call

### Packet 3: Decide handbook-side deferred boundaries and non-targets

Status: **already landed before this packet**

Packet 3 makes all of the following explicit:

- `handbook-flow` remains handbook-owned longer-term as the current resolver / packet-result / budget seam
- a future narrower `handbook-flow` import slice would need explicit proof before later planning could bless it
- `handbook-cli` is the handbook product shell and not an import target
- retained `handbook-compiler` is transition glue rather than a future ownership target
- the remaining support surfaces that stay deferred to later seams instead of being falsely “resolved” here

### Packet 4: Define downstream execution seams and review gate

Status: **landed in this change**

Packet 4 makes all of the following explicit:

- whether any later `handbook-engine` adapter / boundary-freeze seam is still needed
- the bounded `handbook-pipeline` boundary cleanup seam
- the bounded `handbook-flow` ownership clarification seam
- the bounded retained `handbook-compiler` narrowing seam
- the bounded CLI shell/support clarification seam
- the final human review gate that stops the planning family before execution

## Packet 2 Execution Approach

1. verify the root ownership rule in `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
2. verify the live `handbook-engine` public surface and the post-`aa882af` generic layout-contract export truth
3. verify the live `handbook-pipeline` public surface, dependency graph, and bounded compiler-backed fixture/support coupling
4. record separate engine and pipeline ownership/import decisions without collapsing them into one generic verdict
5. call out the residual pipeline cleanup as its own bounded deferred seam
6. leave `handbook-flow`, `handbook-cli`, retained `handbook-compiler` final posture work, and all implementation work out of scope

## Packet 3 Execution Approach

1. verify the live `handbook-flow` surface, dependency posture, and `resolver_core` proof without claiming a move target
2. verify the live `handbook-cli` shell boundary through command wiring, help posture, prompting/rendering ownership, and `help_drift_guard`
3. verify retained `handbook-compiler` still sits above `handbook-engine`, `handbook-flow`, and `handbook-pipeline` as compatibility/support glue rather than the implementation center
4. record separate Packet 3 boundary text for flow, CLI, and retained compiler without reopening Packet 2 decisions
5. name the handbook-side support surfaces that remain deferred to later seams
6. leave Packet 4 downstream execution seams and review-gate design out of scope

## Packet 4 Execution Approach

1. preserve all Packet 1 through 3 ownership calls exactly as already landed
2. translate the landed Packet 2 and Packet 3 ownership decisions into a bounded downstream seam map only
3. record the decision focus for each follow-on seam without authoring prompts or approving execution
4. make explicit that none of those seams start here
5. make explicit that publication, crates.io, and Substrate consumption remain later human-reviewed decisions
6. end the planning family at an explicit human review gate before any execution work

## Packet 2 Verification Outputs Used

- `rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs` -> `handbook-engine` exposes a narrow reusable crate root, while `handbook-pipeline` exposes the broader catalog/runtime wedge directly from `lib.rs`
- `rg -n "default_canonical_layout_contract|workspace_contract_version" crates/engine/src/lib.rs crates/engine/src/canonical_paths.rs` -> confirms the generic `default_canonical_layout_contract` export and shared workspace contract version are the current engine layout boundary
- `rg -n "PipelineCapture|PipelineHandoff|RouteState|template_library|stage_10_feature_spec" crates/pipeline/src crates/pipeline/tests` -> confirms the pipeline crate exports compile/capture/handoff/route-state surfaces and that the catalog test still reaches into compiler-owned `template_library`
- `cargo tree -p handbook-engine` -> shows only foundational runtime dependencies
- `cargo tree -p handbook-pipeline` -> shows the runtime dependency on `handbook-engine` and a remaining `handbook-compiler` **dev-dependency**, not a runtime ownership inversion
- `cargo test -p handbook-engine --test canonical_artifacts_ingest` -> passes, including the non-default layout-contract path coverage
- `cargo test -p handbook-pipeline --test pipeline_catalog` -> passes, including supported-target wedge checks and the template-library-backed declarative-source assertions

## Packet 3 Verification Outputs Used

- `rg -n "pub mod|pub use|resolve|PacketResult|BudgetOutcome" crates/flow/src/lib.rs crates/flow/src` -> confirms `handbook-flow` is still just the `budget`, `packet_result`, and `resolver` family plus reviewed re-exports
- `cargo tree -p handbook-flow` -> shows `handbook-flow` depending directly on `handbook-engine`, not on CLI or retained compiler ownership layers
- `cargo test -p handbook-flow --test resolver_core` -> passes, so the current middle-layer resolver seam is stable as handbook-owned live truth
- `rg -n "CommandFactory|ExitCode|pipeline_help|doctor|rendering|prompt" crates/cli/src/main.rs crates/cli/src` -> confirms the CLI crate still owns command wiring, dynamic help, prompting, rendering, doctor/setup shell flow, and exit-code handling
- `cargo test -p handbook-cli --test help_drift_guard` -> passes, so the CLI help/product-shell posture is current repo truth
- `rg -n "rendering|refusal|doctor|setup|template_library|pub use" crates/compiler/src/lib.rs crates/compiler/src` -> confirms retained compiler glue still re-exports CLI-facing support adapters rather than acting as the true owner layer
- `cargo tree -p handbook-compiler` -> shows retained `handbook-compiler` sitting above `handbook-engine`, `handbook-flow`, and `handbook-pipeline`
- `cargo check --workspace` -> passes, so the Packet 3 boundary call does not depend on a hidden code regression

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

## Packet 3 Decision Summary

### `handbook-flow`

- handbook remains the architectural owner longer-term
- the current crate stays a handbook-owned middle layer rather than a current move target
- any future narrower import slice must first prove a stable reviewed contract around the resolver / packet-result / budget family
- Packet 3 does not claim that proof already exists

### `handbook-cli`

- handbook remains the owner
- the CLI crate is the handbook product shell, not an import target
- command wiring, dynamic help, prompting, rendering, doctor/setup wording, and exit-code policy stay CLI-owned
- later CLI redesign or product splitting is explicitly deferred

### retained `handbook-compiler`

- retained compiler stays handbook-side as transition glue
- it is not the future ownership target and not the implementation center of gravity
- later work may narrow or retire the glue, but Packet 3 does not resolve that timing
- downstream callers should not be told to import through compiler glue as the durable boundary

## Packet 4 Decision Summary

### Optional `handbook-engine` adapter / boundary-freeze seam

- only a later human-reviewed seam if future consumers still need a narrower reviewed adapter or importer-boundary freeze beyond the current public `handbook-engine` surface
- bounded to deciding whether the current public engine surface is sufficient or whether a narrower adapter / boundary freeze is required
- not started here

### `handbook-pipeline` boundary cleanup seam

- bounded to defining the reviewed supported-target importer boundary and removing or relocating the remaining compiler-backed fixture/support coupling
- owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge, rather than the CLI-shell or retained-compiler ownership split
- explicitly separate from the Packet 2 ownership call
- not started here

### `handbook-flow` ownership clarification seam

- bounded to proving or rejecting a reviewed importer contract around the `resolver` / `packet_result` / `budget` family
- owns only the possible flow-side importer/error-boundary proof around that family; it does not settle CLI rendering, `doctor`, or `setup` shell ownership, and it does not own generic non-shell compatibility helpers outside that bounded proof
- must stay separate from CLI shell concerns and retained compiler support glue
- not started here

### Retained `handbook-compiler` narrowing seam

- bounded to later narrowing or retirement of retained compiler glue only after support surfaces have explicit homes and downstream callers no longer need compiler-routed transition helpers
- owns the later reassignment/retirement of any remaining compiler-routed compatibility adapters, including template-library authoring glue and any non-shell `refusal` / `rendering` / `error` compatibility helpers that still survive only as transition glue outside the bounded flow importer/error-boundary proof
- explicitly separate from ownership and from current execution approval
- not started here

### CLI shell/support clarification seam

- bounded to clarifying which surfaces stay shell-owned in `handbook-cli` and which support helpers, if any, later work may relocate below the shell
- owns the shell-side split for rendering/output formatting, shell-facing refusal/error presentation, `doctor`, `setup`, prompting, operator wording, and exit-code policy
- does not make the CLI crate an import target
- not started here

## Final Human Review Gate

**Review gate wording:** Packet 4 ends this planning family at a human review gate. None of the downstream execution seams above start here, this packet is not execution approval, and no packet-prompt authoring, production edits, crate publication or crates.io work, Substrate consumption, or integration implementation may begin until a human separately reviews this triplet and explicitly approves a later execution packet.

Publication, crates.io, and Substrate consumption remain later human-reviewed decisions rather than automatic consequences of this planning landing.

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

### Risk: later work overclaims `handbook-flow` as already import-ready

Mitigation:

- Packet 3 records handbook ownership for the current live seam without claiming a move target
- Packet 3 names the proof a future narrower import boundary would need before later planning can bless it

### Risk: later work treats `handbook-cli` as reusable owner layer instead of product shell

Mitigation:

- Packet 3 ties the CLI posture to live command/help/prompt/rendering/exit ownership evidence
- Packet 3 makes CLI redesign and product-shell splitting explicit non-targets

### Risk: later work treats retained `handbook-compiler` as the durable owner boundary

Mitigation:

- Packet 3 records retained compiler as transition glue that sits above the extracted owner crates
- Packet 3 leaves narrowing/retirement timing deferred instead of hiding that glue inside a fake “future owner” story

### Risk: later readers mistake Packet 4 planning for execution approval

Mitigation:

- Packet 4 names only bounded downstream seams and says none of them start here
- Packet 4 ends with an explicit human review gate covering packet prompts, publication, crates.io, Substrate consumption, and integration implementation

## Exit Condition For This Landing

This landing is complete when:

- Packets 2 and 3 are explicit in the spec/plan/tasks triplet
- `handbook-engine` and `handbook-pipeline` each have separate ownership/import boundary text
- `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` each have separate Packet 3 boundary text
- the engine decision reflects the generic default-layout contract truth
- the pipeline decision reflects the bounded compiler-backed fixture/support coupling truth
- the pipeline-specific deferred cleanup seam is named explicitly
- Packet 3 names the support surfaces still deferred to later seams
- Packet 4 lands only the bounded downstream seam map and the final human review gate
- the result is ready for orchestration review and still not execution-approved
