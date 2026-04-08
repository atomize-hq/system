---
slice_id: S99
seam_id: SEAM-3
slice_kind: seam_exit_gate
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any post-landing change that invalidates `C-03` without recording a new contract version and downstream revalidation decision.
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
  - C-03
open_remediations: []
---

### S99 - Seam-Exit Gate (SEAM-3)

- **User/system value**: Provide a deterministic handoff signal to downstream seams that the canonical manifest + freshness truth (`C-03`) is now publishable and promotable.
- **Scope (in/out)**:
  - In:
    - closeout evidence capture for the `C-03` contract and `THR-03` thread publication
    - review-surface delta capture (what changed versus the plan)
    - downstream stale-trigger emission and revalidation instructions
    - remediation disposition statement (including “none”)
    - promotion readiness statement: `ready | blocked`
  - Out:
    - net-new resolver, renderer, demo, or conformance work that belongs in downstream seams
- **Acceptance criteria**:
  - Closeout (`../../governance/seam-3-closeout.md`) records:
    - landed evidence links
    - `seam_exit_gate.status: passed|failed`
    - `seam_exit_gate.promotion_readiness: ready|blocked`
    - contracts + threads published (at least `C-03`, `THR-03`)
    - downstream stale triggers and revalidation requirements for `SEAM-4` through `SEAM-7`
- **Dependencies**:
  - Landing must include a concrete `C-03` contract artifact and a landed manifest ingest/freshness implementation in `crates/compiler`.
- **Verification**:
  - A downstream planner for `SEAM-4` can cite `C-03` as the stable contract for canonical `.system/` inputs and freshness semantics without hedging.
- **Rollout/safety**:
  - If promotion readiness is blocked, closeout must state the exact blocker (contract ambiguity, drift, missing evidence, or open remediation).
- **Review surface refs**: `../../review_surfaces.md` (R1-R3)

#### S99.T1 - Record landed evidence and contract/thread publication

- **Outcome**: `C-03` and `THR-03` are recorded as published with explicit evidence in closeout.
- **Thread/contract refs**: `THR-03`, `C-03`

#### S99.T2 - Emit downstream stale triggers and revalidation instructions

- **Outcome**: `SEAM-4` through `SEAM-7` have an explicit “revalidate against C-03” requirement where applicable.
- **Thread/contract refs**: `THR-03`, `C-03`
