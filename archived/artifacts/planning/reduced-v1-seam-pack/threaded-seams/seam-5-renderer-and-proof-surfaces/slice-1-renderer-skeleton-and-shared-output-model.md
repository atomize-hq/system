---
slice_id: S1
seam_id: SEAM-5
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to resolver result fields (`C-04`) that affects renderer inputs requires revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-04
  - THR-05
contracts_produced: []
contracts_consumed:
  - C-04
  - C-05
open_remediations: []
---

### S1 - Renderer Skeleton and Shared Output Model

- **User/system value**: All renderers share a single output-model boundary so semantics cannot drift across markdown/JSON/inspect.
- **Scope (in/out)**:
  - In:
    - define a renderer module layout (markdown/json/inspect) as pure views over `C-04`
    - define the shared ordering helpers and stable sort keys for proof surfaces
    - define error-isolation behavior (renderer failure does not erase typed resolver success)
  - Out:
    - any changes to packet selection, refusal, or blocker computation (owned by `SEAM-4`)
- **Acceptance criteria**:
  - `system generate` can render at least one proof surface once S2/S3 land in this seam.
  - Renderer code has no access to canonical ingest or resolver logic beyond `C-04` typed result input.

