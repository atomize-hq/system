---
slice_id: S2
seam_id: SEAM-3
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to freshness fields, inherited dependency rules, or override-with-rationale semantics requires a `C-03` update and downstream revalidation.
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

### S2 - Implement Freshness Computation and Override-with-Rationale Semantics

- **User/system value**: The manifest can produce deterministic freshness truth and explicit override-with-rationale records that downstream seams can consume for refusal and proof surfaces.
- **Scope (in/out)**:
  - In:
    - implement the deterministic freshness fields defined by `C-03`
    - implement inherited posture dependency accounting that influences freshness without becoming a packet-body input
    - implement override-with-rationale records and forbidden override enforcement
  - Out:
    - “what to do next” recovery UX and refusal copy (`SEAM-4`, `SEAM-5`)
    - budget policy decisions and packet selection (`SEAM-4`)
- **Acceptance criteria**:
  - Freshness truth is deterministic given the same canonical artifact set and the same declared dependency inputs.
  - Overrides cannot silently expand the input surface; they require explicit rationale and remain inspectable.
  - The output shape is stable enough for `SEAM-4` to consume as contract input.
- **Dependencies**:
  - Requires `S1` ingest output for canonical artifact identities.
  - Requires concrete rules from `C-03` (`S00`) for freshness fields and override semantics.
- **Verification**:
  - Unit tests for:
    - stable freshness output ordering and field determinism
    - inherited dependency affects freshness as expected
    - forbidden overrides are rejected deterministically
- **Rollout/safety**:
  - Avoid “best effort” merges: ambiguous freshness should surface as explicit “unknown/invalid” states that downstream seams can refuse.
- **Review surface refs**: `../../review_surfaces.md` (R1, R3)

#### S2.T1 - Define the freshness field model and computation function

- **Outcome**: A single function (e.g., `compute_freshness(artifacts, dependencies) -> FreshnessTruth`) with stable ordering and explicit field semantics.
- **Thread/contract refs**: `THR-03`, `C-03`

#### S2.T2 - Implement inherited dependency handling (manifest-local)

- **Outcome**: The manifest can accept declared inherited dependencies (from `setup` posture or repo configuration) and incorporate them into freshness deterministically.
- **Thread/contract refs**: `THR-03`, `C-03`

#### S2.T3 - Implement override-with-rationale records and enforcement

- **Outcome**: Overrides produce explicit rationale records and cannot hide freshness truth or introduce new canonical inputs without a contract bump.
- **Thread/contract refs**: `THR-03`, `C-03`
