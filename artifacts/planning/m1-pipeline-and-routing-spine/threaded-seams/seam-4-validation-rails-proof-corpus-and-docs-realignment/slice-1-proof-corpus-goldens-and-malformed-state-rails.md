---
slice_id: S1
seam_id: SEAM-4
slice_kind: conformance
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to shared proof-fixture shape, golden output wording, or malformed-state refusal semantics requires revalidation before execution continues.
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
  - C-08
  - C-09
  - C-10
open_remediations: []
---

### S1 - Proof Corpus, Goldens, and Malformed-State Rails

- **User/system value**: Shared fixtures and goldens make the shipped `pipeline` surface testable with one realistic foundation-family corpus instead of ad hoc cases spread across compiler and CLI tests.
- **Scope (in/out)**:
  - In:
    - define shared foundation-family proof fixtures for `pipeline resolve` and `pipeline state set`
    - pin golden outputs for resolved-route truth and refusal behavior
    - exercise malformed-state refusal paths explicitly
  - Out:
    - docs/help cutover work
    - compile implementation
    - unsupported command exposure
- **Acceptance criteria**:
  - Compiler and CLI tests can consume the same realistic fixture set.
  - Golden outputs capture the supported route statuses and explicit refusal classes.
  - Malformed-state handling is explicit and never silently auto-healed.
- **Dependencies**:
  - Inputs: `../../threading.md`, `../../review.md`, and the landed `SEAM-1`, `SEAM-2`, and `SEAM-3` closeouts
  - External contract constraints: `docs/contracts/pipeline-route-and-state-core.md`, `docs/contracts/pipeline-operator-surface-and-id-resolution.md`, `docs/contracts/stage-compile-boundary-and-route-freshness.md`
- **Verification**:
  - Shared fixtures exist for compiler and CLI tests.
  - Refusal paths for malformed state are covered by goldens rather than prose-only assertions.
