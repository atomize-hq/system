---
slice_id: S3
seam_id: SEAM-2
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any introduction of real resolver semantics into these types must be deferred to `SEAM-4` and treated as a contract-affecting change.
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
  - C-02
open_remediations: []
---

### S3 - Add Minimal Shared Types Skeleton for Packet Results and Decision Logs

- **User/system value**: Downstream seams have a stable compile-time home for shared types without forcing early semantic commitments.
- **Scope (in/out)**:
  - In:
    - define placeholder types and module locations for:
      - “packet result” (shape only)
      - “decision log” (shape only)
      - shared error and “refusal placeholder” scaffolding (shape only)
    - keep these types in the compiler crate (shared ownership surface)
  - Out:
    - budget/refusal semantics, blocker taxonomy, and real decision log fields (owned by `SEAM-4`)
- **Acceptance criteria**:
  - Types compile and are importable by the CLI crate without circular dependencies.
  - Type and module names align with `C-02` crate ownership rules and do not pre-commit downstream schemas.
- **Dependencies**:
  - Requires `S1` crate skeletons and boundary layout.
  - Requires `S00` contract naming guidance to avoid drift.
- **Verification**:
  - `crates/cli` can reference the placeholder types while keeping behavior stubs.
- **Rollout/safety**:
  - Prefer intentionally minimal fields; only add real semantics when `SEAM-4` owns the resolver result contract (`C-04`).
- **Review surface refs**: `../../review_surfaces.md` (R3 touch surface map)

#### S3.T1 - Define placeholder types and their module locations

- **Outcome**: stable import paths exist for downstream seams to extend under the correct owner seam.
- **Thread/contract refs**: `THR-02`, `C-02`
