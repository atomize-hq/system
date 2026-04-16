# Project Charter
- Charter Ref: `CH-M4-FOUNDATION-2026-04`
- Project: System
- Owner: Platform Foundations
- Team: Build Systems

## Mission
Ship an M4 proof corpus that demonstrates the full `pipeline.foundation_inputs` journey with
truthful operator evidence. The route must show that `pipeline compile` produces stage-10 model
input only, while `pipeline capture` writes the final `FEATURE_SPEC.md` only after a completed
external model response is supplied.

## Baseline Posture
- Baseline level: 4 - High rigor with delivery pragmatism
- Speed vs Quality: prefer correctness over speed whenever proof surfaces and wording disagree
- Type safety / static analysis: strict for compiler-owned and CLI-owned route contracts
- Testing rigor: deterministic fixture corpus plus focused integration tests for happy and skip paths
- Scalability & performance: optimize for local CLI repeatability, not distributed orchestration
- Reliability & operability: refuse stale route basis, malformed inputs, and unproved stage-10 writes
- Security & privacy: fixtures and captured outputs must stay free of secrets and production data
- Observability: prove route status transitions, skip reasons, capture ids, and next safe actions
- DX & automation: keep the operator surface small, explicit, and aligned with docs/help

## Red Lines
- Do not teach or prove a raw `compile | capture` handoff at stage 10.
- Do not imply that stage 10 capture can reconstruct missing external model output.
- Do not blur manual operator decisions into automatic reroute behavior.
- Do not merge this journey corpus into the shared proof corpus or `execution_demo`.

## Success Criteria
- One realistic happy path reaches `stage.10_feature_spec` through stage 06 and stage 07.
- One realistic skip path proves stage 06 stays skipped because both route predicates remain false.
- Docs/help/test proof surfaces all state the same compile-to-external-output-to-capture boundary.
- Final stage-10 artifacts are deterministic across reruns with fixed `now_utc` and normalized capture ids.

## Required Follow-Up Before Final Foundation Pack
Project context follow-up is required before the final foundation pack because these decisions
still need a factual repo snapshot:
- Which operator-visible evidence belongs in the M4 journey scorecard.
- Which rerun expectations should be proved in tests versus described in docs.
- Which manual decisions remain required after the happy-path proof lands.
