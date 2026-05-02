---
slice_id: S2
seam_id: SEAM-4
slice_kind: documentation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to README, command hierarchy, product vocabulary, or help snapshots that would imply unsupported `pipeline compile` behavior requires revalidation before execution continues.
gates:
  pre_exec:
    review: pending
    contract: pending
    revalidation: pending
  post_exec:
    landing: pending
    closeout: pending
threads: []
contracts_produced: []
contracts_consumed:
  - C-09
  - C-10
open_remediations: []
---

### S2 - Docs, Help, and Command-Hierarchy Realignment

- **User/system value**: The shipped `pipeline` story stays coherent across README, docs, help snapshots, and vocabulary pages instead of drifting back toward packet-only wording.
- **Scope (in/out)**:
  - In:
    - align root docs with the shipped `pipeline` subset
    - keep command-hierarchy wording consistent with supported behavior
    - keep help snapshots honest about the deferred compile boundary
  - Out:
    - test fixture work
    - compile implementation
    - new product claims
- **Acceptance criteria**:
  - Docs and help do not imply unsupported command exposure.
  - The deferred compile boundary remains explicit in user-facing copy.
  - Product vocabulary and hierarchy docs agree with the operator-surface contract.
- **Dependencies**:
  - Inputs: `../../threading.md`, `../../review.md`, and the landed `SEAM-2` and `SEAM-3` closeouts
  - External contract constraints: `docs/contracts/pipeline-operator-surface-and-id-resolution.md`, `docs/contracts/stage-compile-boundary-and-route-freshness.md`
- **Verification**:
  - Docs/help parity checks fail if user-facing wording widens the shipped surface.
  - The deferred compile posture remains explicit in every updated docs/help surface.
