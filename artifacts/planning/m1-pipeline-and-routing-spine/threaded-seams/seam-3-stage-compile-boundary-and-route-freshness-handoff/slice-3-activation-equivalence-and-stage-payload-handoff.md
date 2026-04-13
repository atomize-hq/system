---
slice_id: S3
seam_id: SEAM-3
slice_kind: conformance
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to stage-front-matter activation posture, compile payload field ownership, or the rule that activation drift is an explicit refusal requires this slice to revalidate.
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
  - C-08
  - C-09
  - C-10
open_remediations: []
---

### S3 - Activation Equivalence and Stage-Payload Handoff

- **User/system value**: Later compile work gets one explicit rule for how pipeline-entry activation, stage-front-matter activation, and stage payload metadata relate, so downstream seams do not guess when those sources drift.
- **Scope (in/out)**:
  - In:
    - duplicated activation equivalence rules between pipeline YAML and stage front matter during the transition
    - explicit refusal posture when activation drift exists
    - the stage-payload contract expected once compile lands in M2
  - Out:
    - actual payload materialization or compile execution
    - proof corpus or docs/help cutover work beyond what the compile-boundary contract needs
- **Acceptance criteria**:
  - Activation drift between pipeline YAML and stage front matter is treated as a first-class refusal category rather than an implementation detail.
  - The stage-payload handoff is concrete enough that downstream compile implementation work can consume it without reopening source-of-truth rules.
  - The seam keeps `pipeline compile` deferred from shipped M1 help/docs surface while still defining the contract needed for later work.
- **Verification**:
  - Add or update compile-boundary planning/tests that pin activation-equivalence rules, activation-drift refusal posture, and the stage-payload handoff contract.
