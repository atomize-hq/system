---
slice_id: S2
seam_id: SEAM-3
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to route-basis fields, route-state semantics, inactive-stage status meaning, or the operator recovery path that says to re-run `pipeline resolve` requires this slice to revalidate.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-03
contracts_produced: []
contracts_consumed:
  - C-08
  - C-09
  - C-10
open_remediations: []
---

### S2 - Route-Basis Freshness and Inactive-Stage Refusals

- **User/system value**: Compile-boundary work can fail safely and explicitly when the selected stage is inactive or the route basis is stale, instead of silently compiling against invalid planning truth.
- **Scope (in/out)**:
  - In:
    - route-basis freshness checks against pipeline, stage, runner, profile, and route-state inputs
    - inactive-stage refusal posture
    - operator recovery path that tells the caller when to re-run `pipeline resolve`
  - Out:
    - stage-payload field ownership or activation-drift equivalence rules
    - actual compile implementation or output writes
    - docs/help parity work owned by `SEAM-4`
- **Acceptance criteria**:
  - Freshness inputs and stale-basis refusal semantics are concrete enough for downstream implementation.
  - Inactive-stage refusal remains explicit and does not degrade into silent skip or best-effort compile behavior.
  - The contract makes clear that compile does not silently refresh route truth on behalf of the operator.
- **Verification**:
  - Add or update compile-boundary planning/tests that cover stale route-basis refusal, inactive-stage refusal, and the operator recovery guidance to re-run `pipeline resolve`.
