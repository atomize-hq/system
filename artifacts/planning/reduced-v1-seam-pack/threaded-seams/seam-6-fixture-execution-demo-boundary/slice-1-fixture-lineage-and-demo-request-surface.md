---
slice_id: S1
seam_id: SEAM-6
slice_kind: implementation
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to fixture discovery, fixture ordering, or demo invocation parameters requires downstream revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-06
contracts_produced: []
contracts_consumed:
  - C-04
  - C-06
open_remediations:
  - REM-002
---

### S1 - Fixture Lineage and Demo Request Surface

- **User/system value**: The demo can be invoked deterministically and is auditable as fixture-backed (not live capability).
- **Scope (in/out)**:
  - In:
    - define the canonical fixture root and fixture lineage layout under the approved repo surface
    - define deterministic fixture selection and ordering rules (no incidental FS traversal ordering)
    - define the demo request surface (CLI flag vs tooling vs test-only) and keep it labeled as fixture-backed
  - Out:
    - live slice discovery or any live execution packet support
- **Acceptance criteria**:
  - A deterministic fixture lineage can be enumerated and explained in inspect output (S2).
  - The demo invocation surface is explicit enough that `SEAM-7` can write conformance tests without guessing.
