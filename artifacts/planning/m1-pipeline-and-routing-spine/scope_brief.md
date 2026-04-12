---
pack_id: m1-pipeline-and-routing-spine
pack_version: v1
pack_status: extracted
source_ref: PLAN.md#M1-pipeline-and-routing-spine
execution_horizon:
  active_seam: SEAM-1
  next_seam: SEAM-2
---

# Scope Brief - M1 Pipeline And Routing Spine

- **Goal**: Establish compiler ownership of planning-path selection, conditional routing, and narrow route-relevant state for foundation-family pipeline flows.
- **Why now**: The current Rust CLI proves the long-term posture, but operators still reconstruct pipeline selection, branching, and state by hand. `M1` is the first milestone that can remove repeated repo research and manual context shuttling.
- **Primary user(s) + JTBD**: The planning/orchestration operator needs to inspect a pipeline, resolve one authoritative route, persist the small routing state needed for multi-step planning, and hand that truth to downstream planning-generation work without manual restitching.
- **In-scope**:
  - compiler-owned loading of the current pipeline YAML shape with deterministic stage ordering
  - activation evaluation and one authoritative resolved-route result with explicit `active`, `skipped`, `blocked`, and `next` statuses
  - narrow persisted pipeline-run state under `.system/state/pipeline/`
  - the shipped `pipeline` operator subset: `list`, `show`, `resolve`, and `state set`
  - validation rails, proof corpus, and docs/help realignment required to ship `pipeline` as a supported surface
  - defining the downstream `pipeline compile` handoff contract without shipping compile itself in `M1`
- **Out-of-scope**:
  - full stage payload generation or output materialization
  - pipeline-wide compilation in one shot
  - onboarding chat flows, review/fix packets, or downstream seam-skill reimplementation
  - live execution lineage or broader runtime orchestration
  - caching schemes or generalized state machines beyond the narrow M1 route-state wedge
- **Success criteria**:
  - Rust can load the two foundation-family pipeline definitions and preserve declared order
  - `pipeline resolve` computes deterministic route truth and explains every non-`active` stage status
  - route-relevant state persists narrowly enough to continue multi-step planning flows without becoming canonical project truth
  - the shipped `pipeline` subset has aligned code, help, docs, tests, and proof outputs
  - one realistic foundation-family proof corpus demonstrates route selection, branching, ambiguity refusal, malformed-state refusal, and state-mutation semantics
- **Constraints**:
  - keep the repo objective scoped to the generator/compiler layer
  - preserve the existing trust-heavy CLI posture from `DESIGN.md`
  - use realistic canonical docs and proof artifacts, not toy fixtures
  - treat `.system/` as split into canonical artifact zones and non-canonical runtime zones
  - keep `pipeline compile` out of shipped `M1` help/docs until the M2 compile contract is real and honored
- **External systems / dependencies**:
  - `pipelines/foundation.yaml`
  - `pipelines/foundation_inputs.yaml`
  - `core/stages/*.md`
  - `profiles/` and `runners/`
  - existing CLI/runtime contracts in `docs/contracts/`
  - legacy harness behavior in `tools/harness.py` and legacy docs as reference evidence only
- **Known unknowns / risks**:
  - exact compiler module shape for route truth without leaking packet resolver semantics into pipeline routing
  - how much stage metadata `resolve` must load while preserving the performance boundary for `list` and `show`
  - how to keep shorthand id ergonomics without creating ambiguous lookup debt
  - whether route-state mutation concurrency semantics remain simple enough for one seam-local implementation slice
  - whether existing reduced-v1 packet-first docs create misleading product contract overlap until the docs cutover seam lands
- **Assumptions**:
  - the `M0.5` parser gate is complete and `serde_yaml_bw` is the approved parser base
  - canonical project documents already exist before the first foundation-family planning flow starts
  - the first supported proof corpus remains intentionally foundation-family narrow even though the command family stays pipeline-generic
  - legacy materials remain reference evidence, not an active supported runtime path
