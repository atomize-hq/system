---
slice_id: S1
seam_id: SEAM-3
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to canonical pipeline/stage selection rules, stage-front-matter ownership, or the rule that compile reuses published operator-surface targeting semantics requires this slice to revalidate.
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

### S1 - Compile Target Selection and Source-of-Truth Boundary

- **User/system value**: Downstream compile work can target one stage against one resolved pipeline route without re-opening canonical-id semantics or blurring what belongs to pipeline YAML versus stage front matter.
- **Scope (in/out)**:
  - In:
    - canonical compile-target selection rules over the published `pipeline` operator surface
    - stage selection posture within one pipeline context
    - source-of-truth split between pipeline YAML orchestration fields and stage front-matter compile fields
  - Out:
    - route-basis freshness refusal semantics
    - actual compile implementation or payload materialization
    - docs/help cutover outside compile-boundary evidence
- **Acceptance criteria**:
  - Compile-target selection reuses canonical-id and shorthand semantics from the published operator surface rather than inventing compile-only targeting rules.
  - The source-of-truth split is concrete enough that a downstream implementer can name which fields come from pipeline YAML and which come from stage front matter.
  - Raw file-path targeting and compile-only hidden aliases remain out of scope.
- **Verification**:
  - Add or update compile-boundary planning/tests that pin canonical target-selection rules, source-of-truth ownership, and the rule that compile consumes resolved route truth rather than re-running route discovery.
