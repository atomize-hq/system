---
slice_id: S2
seam_id: SEAM-4
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to refusal copy requirements or refusal-category boundaries requires downstream revalidation.
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

### S2 - Refusal Taxonomy and Compact Refusal Structure (Generate)

- **User/system value**: `generate` refusal behavior is compact, typed, and action-oriented, without leaking renderer copy decisions into core semantics.
- **Scope (in/out)**:
  - In:
    - define refusal categories aligned to `C-03` inputs and freshness semantics (missing, stale, contradictory, unsupported, budget, etc.)
    - define a compact refusal structure required by `C-04`:
      - blocker summary
      - broken artifact/dependency
      - exact next safe action
    - ensure refusal ordering is stable and deterministic
    - ensure refusal truth is produced by the resolver core (not renderer or CLI-only branching)
  - Out:
    - renderer phrasing and output ordering (`SEAM-5`)
    - conformance tests and docs cutover (`SEAM-7`)
- **Acceptance criteria**:
  - Refusals always include an exact next safe action.
  - Refusal categories are typed and sufficient for downstream renderers and conformance to avoid parsing strings.
  - Refusal ordering is deterministic and based on explicit rules (not incidental iteration order).
- **Dependencies**:
  - `C-03` defines canonical inputs and freshness semantics that refusal must reference.
  - `C-02` defines `generate` CLI posture and refusal UX constraints.
- **Verification**:
  - Contract-level: refusal structure matches the `C-04` contract.
  - Behavior-level: refusal ordering + action guidance is stable across repeated runs.
- **Rollout/safety**:
  - Prefer “refuse with recovery steps” over silent fallback.

#### S2.T1 - Define refusal categories + mapping rules (inputs -> refusal)

- **Outcome**: A typed refusal taxonomy and explicit mapping rules from `C-03` states to `C-04` refusal categories.
- **Thread/contract refs**: `THR-03`, `C-03`, `THR-04`, `C-04`

#### S2.T2 - Ensure compact refusal structure with exact next action

- **Outcome**: Every refusal includes a compact summary, the broken artifact/dependency, and exactly one next safe action.
- **Thread/contract refs**: `THR-04`, `C-04`
