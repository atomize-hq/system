---
slice_id: S1
seam_id: SEAM-7
slice_kind: conformance
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to the workspace member list, CLI verb surface, or compiler refusal/rendering semantics requires revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: pending
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-07
contracts_produced: []
contracts_consumed:
  - C-01
  - C-02
  - C-03
  - C-04
  - C-05
  - C-06
open_remediations: []
---

### S1 - Conformance Rails and CI/Smoke

- **User/system value**: The repo fails fast when runtime, ordering, or refusal semantics drift away from the published contracts.
- **Scope (in/out)**:
  - In:
    - tests that pin trust header ordering and refusal categories/next-safe-actions
    - smoke coverage that exercises fixture-backed demo request plumbing
    - CI rails that run the required checks deterministically
  - Out:
    - redefining upstream contract semantics

