---
slice_id: S3
seam_id: SEAM-2
slice_kind: conformance
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to shipped help exposure, normalized render wording, or ambiguity/unknown-id recovery guidance requires this slice to revalidate.
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
  - C-09
open_remediations: []
---

### S3 - Help, Ambiguity Refusals, and Proof Rails

- **User/system value**: The shipped `pipeline` subset stays explicit and auditable because help posture, refusal copy, and CLI proof surfaces all tell the same supported story.
- **Scope (in/out)**:
  - In:
    - shipped CLI help posture for `pipeline`
    - ambiguity and unknown-id refusal wording with explicit operator recovery guidance
    - CLI proof rails and integration tests that pin the supported surface
  - Out:
    - broad docs/help cutover beyond the command-family evidence needed to ship the CLI surface
    - proof-corpus ownership outside CLI-facing operator evidence
- **Acceptance criteria**:
  - Help exposes only `list`, `show`, `resolve`, and `state set` as the supported M1 `pipeline` subset.
  - Ambiguous shorthand and unknown canonical ids remain distinct refusal classes with distinct recovery guidance.
  - CLI proof rails make help and refusal drift visible before downstream seams consume the operator surface as stable truth.
- **Verification**:
  - Add or update help snapshots and CLI integration tests covering shipped command exposure, ambiguity refusal, unknown-id refusal, and normalized output posture.
