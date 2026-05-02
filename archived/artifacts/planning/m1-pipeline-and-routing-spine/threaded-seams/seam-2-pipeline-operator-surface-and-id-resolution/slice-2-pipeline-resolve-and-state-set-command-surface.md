---
slice_id: S2
seam_id: SEAM-2
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to published route statuses, reason payloads, state mutation outcomes, or `.system/state/pipeline/` schema requires this slice to revalidate.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-02
contracts_produced: []
contracts_consumed:
  - C-08
  - C-09
open_remediations: []
---

### S2 - Pipeline Resolve and State Set Command Surface

- **User/system value**: Operators can compute one authoritative route and apply narrow route-state mutations through the shipped CLI surface without hiding compiler truth inside wrapper code.
- **Scope (in/out)**:
  - In:
    - `pipeline resolve` and `pipeline state set`
    - normalized default render contracts for declared route and mutation outcomes
    - CLI-to-compiler handoff for route evaluation and state mutation
    - refusal mapping that preserves compiler-owned distinctions
  - Out:
    - docs/help realignment beyond shipped CLI help
    - compile payload generation or output materialization
- **Acceptance criteria**:
  - `pipeline resolve` consumes the published compiler route model and default render contract without redefining statuses or reasons in CLI code.
  - `pipeline state set` consumes the published compiler state-mutation surface and preserves malformed-state, unsupported-variable, and revision-conflict refusals distinctly.
  - The CLI layer remains a thin adapter over compiler-owned route/state semantics.
- **Verification**:
  - Add CLI coverage for resolve happy paths, branching behavior, state-set success, unsupported-variable refusal, malformed-state refusal, and revision-conflict refusal.
