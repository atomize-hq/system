---
slice_id: S3
seam_id: SEAM-5
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to inspect proof ordering or JSON fallback behavior requires downstream revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-05
contracts_produced: []
contracts_consumed:
  - C-04
  - C-05
open_remediations: []
---

### S3 - JSON and Inspect Proof Surfaces

- **User/system value**: Machines can consume a stable JSON output and humans can audit resolver evidence via inspect output without re-running logic.
- **Scope (in/out)**:
  - In:
    - implement JSON renderer as a pure view over `C-04`
    - implement inspect proof ordering as a pure view over `C-04` decision evidence and stable ordering rules
    - ensure dense views have machine-readable fallback surfaces
  - Out:
    - conformance rails (SEAM-7)
- **Acceptance criteria**:
  - JSON output is stable and deterministic for identical resolver inputs.
  - Inspect output explains inclusion, exclusion, freshness, and budget decisions in a stable order.

