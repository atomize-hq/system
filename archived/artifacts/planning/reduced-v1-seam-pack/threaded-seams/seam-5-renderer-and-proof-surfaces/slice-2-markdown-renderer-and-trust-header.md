---
slice_id: S2
seam_id: SEAM-5
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to trust header field set or ordering requires downstream revalidation.
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

### S2 - Markdown Renderer and Trust Header

- **User/system value**: Operators get a stable, narrow-terminal-friendly packet view that starts with trust facts (outcome, object, next action) and then the packet body.
- **Scope (in/out)**:
  - In:
    - implement markdown rendering as a pure view over `C-04`
    - implement trust header layout and stable ordering
    - ensure refusal rendering stays compact and action-oriented (without redefining refusal semantics)
  - Out:
    - JSON and inspect proof surfaces (S3)
- **Acceptance criteria**:
  - The first three lines of markdown output provide: outcome, object, next safe action.
  - Refusal output includes category, summary, broken subject, and exact next action.

