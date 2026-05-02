---
slice_id: S3
seam_id: SEAM-4
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to `doctor` behavior or blocker taxonomy requires downstream revalidation.
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
  - THR-03
  - THR-04
contracts_produced: []
contracts_consumed:
  - C-02
  - C-03
  - C-04
open_remediations: []
---

### S3 - Doctor: Blocker Aggregation, Readiness, and Next Actions

- **User/system value**: `doctor` produces a single, ordered view of readiness blockers and next safe actions without inventing a separate truth from `generate`.
- **Scope (in/out)**:
  - In:
    - ensure `doctor` calls into the same resolver core and consumes the same typed `C-04` result
    - aggregate blockers across required inputs and freshness checks (from `C-03`) and budget/refusal outcomes
    - define stable ordering and “exact next action” semantics for blocker reporting
    - ensure `doctor` output can be rendered by downstream proof surfaces without changing semantics (`SEAM-5`)
  - Out:
    - renderer formatting decisions (`SEAM-5`)
    - fixture execution demo boundaries (`SEAM-6`)
- **Acceptance criteria**:
  - `doctor` does not compute blockers independently from the resolver core; it is a view over typed resolver truth.
  - Blocker list ordering is deterministic and stable.
  - Each blocker includes one exact next safe action.
- **Dependencies**:
  - `C-02` defines `doctor` CLI posture and verb shape.
  - `C-03` defines canonical input states and freshness semantics that can produce blockers.
  - `C-04` defines blocker taxonomy and ordered reporting fields.
- **Verification**:
  - Contract-level: blocker reporting fields match the `C-04` contract.
  - Behavior-level: repeated runs yield identical ordered blocker reports for identical inputs.
- **Rollout/safety**:
  - Prefer explicit “blocked until X” signals over ambiguous success/failure codes.

#### S3.T1 - Wire doctor to the shared resolver entrypoint

- **Outcome**: `doctor` consumes the same typed resolver result used by `generate`.
- **Thread/contract refs**: `THR-04`, `C-04`

#### S3.T2 - Define blocker aggregation and ordering rules

- **Outcome**: A deterministic ordered blocker list with exact next actions (no freeform inference).
- **Thread/contract refs**: `THR-04`, `C-04`
