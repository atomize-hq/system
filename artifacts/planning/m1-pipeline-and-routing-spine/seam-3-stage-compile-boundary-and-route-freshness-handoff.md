---
seam_id: SEAM-3
seam_slug: stage-compile-boundary-and-route-freshness-handoff
type: integration
status: exec-ready
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts:
    - SEAM-1
    - SEAM-2
  required_threads:
    - THR-01
    - THR-02
  stale_triggers:
    - If `SEAM-1` changes route-basis fields, state semantics, or activation ownership rules, this seam must revalidate its compile freshness contract.
    - If `SEAM-2` changes supported stage or pipeline id lookup rules, this seam must revalidate compile-target selection semantics.
gates:
  pre_exec:
    review: passed
    contract: passed
    revalidation: passed
  post_exec:
    landing: pending
    closeout: pending
seam_exit_gate:
  required: true
  planned_location: S99
  status: pending
open_remediations: []
---

# SEAM-3 - Stage Compile Boundary and Route Freshness Handoff

- **Goal / value**: Freeze the downstream `pipeline compile` contract so later compile work consumes M1 route truth cleanly instead of reopening routing semantics.
- **Scope**
  - In:
    - source-of-truth split between pipeline YAML orchestration fields and stage front-matter compile fields
    - route-basis freshness checks against pipeline, stage, runner, profile, and route-state inputs
    - inactive-stage refusal semantics
    - duplicated activation equivalence rules between pipeline YAML and stage front matter during the transition
    - the stage-payload contract expected once compile lands in `M2`
  - Out:
    - actual compile implementation
    - payload materialization or file writes
    - stage execution or runtime orchestration
- **Primary interfaces**
  - Inputs:
    - `C-08`
    - `C-09`
    - `profiles/`
    - `runners/`
    - `core/stages/*.md`
  - Outputs:
    - published compile-boundary contract `C-10`
    - downstream compile-handoff rules for later milestone packs
- **Key invariants / rules**:
  - `pipeline resolve` uses pipeline-entry activation, not stage-front-matter activation
  - if pipeline YAML and stage front matter both define activation for one stage, they must be semantically equivalent during the current wedge
  - `compile` consumes one resolved route result plus one stage definition; it does not recompute the whole route or write state
  - if compile basis drifts, the operator must be told to re-run `pipeline resolve`
- **Dependencies**
  - Direct blockers:
    - `SEAM-1`
    - `SEAM-2`
  - Transitive blockers:
    - current stage front matter and profile/runner metadata shapes
  - Direct consumers:
    - `SEAM-4`
  - Derived consumers:
    - future `M2` compile implementation
    - later planning-generation milestones that depend on stable stage payload semantics
- **Touch surface**:
  - future compile contract docs under `docs/contracts/`
  - `crates/compiler` compile boundary
  - `core/stages/*.md`
  - `profiles/`
  - `runners/`
- **Verification**:
  - This seam now has active seam-local planning and a concrete owned-contract boundary for `C-10`.
  - Verification at seam-brief depth should prove the compile boundary is concrete enough for implementation, especially around freshness refusal, inactive-stage refusal, and pipeline-vs-stage ownership, without requiring compile to land in `M1`.
- **Canonical contract refs**:
  - `docs/contracts/stage-compile-boundary-and-route-freshness.md`
- **Risks / unknowns**:
  - Risk: compile sneaks back into M1 through partial hidden support or help text.
  - De-risk plan: keep this seam future-only and make the defer boundary explicit in docs/help cutover work.
  - Risk: duplicated activation metadata drifts silently between pipeline YAML and stage front matter.
  - De-risk plan: make activation-drift refusal a first-class contract rule and conformance target.
- **Rollout / safety**:
  - publish only the contract and freshness handoff in this seam; do not expose compile as supported surface until M2 lands
  - keep refusal classes explicit for inactive stages, stale route basis, and missing compile inputs
- **Downstream decomposition context**:
  - This seam is `active` because `SEAM-1` and `SEAM-2` have now published the route/state and operator-surface truth it depends on.
  - `THR-03` is the dominant outbound thread; `SEAM-4` is now the next seam and will consume it to keep docs/help and proof surfaces honest about what compile does not yet do.
  - The active seam-local review should focus on source-of-truth boundaries and whether compile freshness can be proven without reintroducing hidden caches or side effects.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-10`
  - Threads likely to advance:
    - `THR-03`
  - Review-surface areas likely to shift after landing:
    - `R1`
    - `R2`
  - Downstream seams most likely to require revalidation:
    - `SEAM-4`
    - future `M2` compile execution work
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.
