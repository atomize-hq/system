---
slice_id: S2
seam_id: SEAM-7
slice_kind: documentation
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to supported-vs-legacy wording or help examples requires revalidation.
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
  - C-05
  - C-06
open_remediations: []
---

### S2 - Docs/Help Cutover and Drift Guards

- **User/system value**: A cold reader sees one supported Rust-first story, and docs/help cannot drift away from runtime behavior without failing checks.
- **Scope (in/out)**:
  - In:
    - update docs/help examples to match the CLI surface and refusal semantics
    - add drift checks (where appropriate) so help output and docs remain aligned
  - Out:
    - adding new runtime capabilities

