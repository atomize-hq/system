---
slice_id: S2
seam_id: SEAM-1
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to route status vocabulary, supported activation semantics, or stage-order evaluation rules requires this slice to revalidate.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-01
contracts_produced: []
contracts_consumed:
  - C-08
open_remediations: []
---

### S2 - Route Evaluation and Status Reasons

- **User/system value**: The compiler computes one deterministic route result with explicit per-stage statuses and reasons, so downstream seams can present or consume route truth without reinterpretation.
- **Scope (in/out)**:
  - In:
    - compiler-owned route evaluation over the declared pipeline model from `S1`
    - explicit per-stage status assignment for `active`, `skipped`, `blocked`, and `next`
    - supported activation-subset evaluation and refusal posture when inputs fall outside the approved wedge
    - typed route-result API surfaces exported from `crates/compiler`
  - Out:
    - persisted state mutation mechanics
    - CLI render wording, table shape, or shorthand-id ergonomics
    - compile handoff payload generation
- **Acceptance criteria**:
  - One typed resolved-route model exists in `crates/compiler` and is exported for downstream use.
  - Route evaluation is deterministic for identical pipeline definitions and state inputs.
  - Non-`active` stages carry explicit reasons rather than forcing downstream seams to infer why they were skipped, blocked, or parked as next.
  - Unsupported activation shapes are refused at the compiler boundary, not hidden behind later CLI behavior.
- **Dependencies**:
  - Inputs: `S1` declared pipeline ingest surface, `pipelines/foundation.yaml`, `pipelines/foundation_inputs.yaml`
  - Future consumers: `SEAM-2`, `SEAM-3`, `SEAM-4`
- **Verification**:
  - Add a route-resolution suite such as `crates/compiler/tests/pipeline_route_resolution.rs`.
  - Cover deterministic ordering, false activation leading to `skipped`, first actionable blocked stage leading to `next` or `blocked` as defined by `C-08`, and refusal of unsupported activation semantics.
  - Verify the exported route-result types remain compiler-owned and do not depend on CLI rendering models.
- **Rollout/safety**:
  - Keep the route-result model separate from packet resolver types so this seam does not blur into unrelated compiler/resolver behavior.
  - Prefer explicit refusal over fallback when route semantics or activation inputs are out of scope for `M1`.
- **Review surface refs**: `../../review_surfaces.md` (`R2`)
