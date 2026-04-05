---
slice_id: S2
seam_id: SEAM-6
slice_kind: implementation
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to refusal wording or how the system distinguishes fixture demo requests from unsupported live requests requires downstream revalidation.
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
  - C-05
  - C-06
open_remediations:
  - REM-002
---

### S2 - Live Refusal and Demo Proof Surfaces

- **User/system value**: Operators see explicit refusal for unsupported live execution requests, and the fixture-backed demo (when invoked) cannot be confused with live capability.
- **Scope (in/out)**:
  - In:
    - implement explicit refusal for unsupported live execution requests, using `C-04` refusal structure and `C-06` wording rules
    - ensure proof surfaces (`generate` markdown + `inspect`) keep the “fixture-backed” label visible and first-class
    - ensure the demo path is auditable: fixture lineage is reported deterministically and ties back to fixture artifacts
  - Out:
    - any new supported verb surface, live execution packets, or live slice lineage
- **Acceptance criteria**:
  - Unsupported live execution requests refuse with clear, non-generic wording and a stable “next safe action”.
  - The demo proof surface shows fixture lineage in stable order and labels it as fixture-backed.
