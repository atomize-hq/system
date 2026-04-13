---
slice_id: S1
seam_id: SEAM-2
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to canonical pipeline ids, shorthand lookup rules, or the rule that raw file paths remain evidence-only requires this slice to revalidate.
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
  - C-08
  - C-09
open_remediations: []
---

### S1 - Pipeline List, Show, and Canonical ID Discovery

- **User/system value**: Operators can discover supported pipelines and inspect declared config through one stable command family without learning raw repo paths.
- **Scope (in/out)**:
  - In:
    - `pipeline list` and `pipeline show`
    - canonical pipeline and stage id discovery
    - unique shorthand lookup and distinct ambiguity/unknown-id refusals
    - normalized declared-config render for `show`
  - Out:
    - resolved-route output
    - persisted state mutation behavior
    - compile exposure or docs-help cutover outside the shipped subset
- **Acceptance criteria**:
  - `pipeline list` and `pipeline show` consume compiler-owned declared pipeline data rather than reparsing YAML in CLI glue.
  - Canonical ids are unique, shorthand succeeds only when unambiguous, and ambiguity remains a distinct refusal class.
  - Raw pipeline file paths remain repo evidence rather than a first-class operator input.
- **Verification**:
  - Add CLI coverage for canonical-id discovery, list/show happy paths, ambiguity refusal, and unknown-id refusal.
