---
seam_id: SEAM-3
seam_slug: stage-compile-boundary-and-route-freshness-handoff
status: landed
execution_horizon: future
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-3-stage-compile-boundary-and-route-freshness-handoff.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts:
    - ../../governance/seam-1-closeout.md
    - ../../governance/seam-2-closeout.md
  required_threads:
    - THR-01
    - THR-02
  stale_triggers:
    - If `SEAM-1` changes route-basis fields, route-state semantics, or activation ownership rules, this seam must revalidate compile freshness and inactive-stage refusal behavior before execution.
    - If `SEAM-2` changes canonical-id or shorthand lookup rules, raw-path posture, or the shipped `pipeline` help surface, this seam must revalidate compile-target selection and operator recovery guidance before execution.
    - If the published operator surface changes normalized `resolve` wording or refusal classes, this seam must revalidate the compile-boundary defer posture before execution.
gates:
  pre_exec:
    review: passed
    contract: passed
    revalidation: passed
  post_exec:
    landing: passed
    closeout: passed
seam_exit_gate:
  required: true
  planned_location: S99
  status: passed
open_remediations: []
---

# SEAM-3 - Stage Compile Boundary and Route Freshness Handoff

## Seam Brief (Restated)

- **Goal / value**: Freeze the downstream `pipeline compile` handoff so later compile work consumes M1 route truth cleanly instead of reopening routing semantics or smuggling compile into the shipped M1 CLI surface.
- **Type**: integration
- **Scope**
  - In:
    - source-of-truth split between pipeline YAML orchestration fields and stage front-matter compile fields
    - compile-target selection rules over the published pipeline/stage operator surface
    - route-basis freshness checks against pipeline, stage, runner, profile, and route-state inputs
    - inactive-stage refusal semantics
    - duplicated activation equivalence rules between pipeline YAML and stage front matter during the transition
    - the stage-payload contract expected once compile lands in M2
  - Out:
    - actual compile implementation
    - payload materialization or file writes
    - stage execution or runtime orchestration
    - exposing `pipeline compile` as shipped M1 help/docs surface
- **Touch surface**:
  - `docs/contracts/stage-compile-boundary-and-route-freshness.md`
  - future compile boundary logic under `crates/compiler`
  - `core/stages/*.md`
  - `profiles/`
  - `runners/`
- **Verification**:
  - The compile handoff must consume published `C-08` and `C-09` truth without recomputing route selection or reintroducing hidden operator semantics.
  - Compile freshness and inactive-stage refusal categories must be concrete enough for downstream implementation without adding compile support to the M1 shipped CLI surface.
  - Stage-front-matter activation equivalence rules must be concrete enough to detect drift rather than silently preferring one source of truth.
- **Canonical contract refs**:
  - `docs/contracts/stage-compile-boundary-and-route-freshness.md`
  - `docs/contracts/pipeline-route-and-state-core.md`
  - `docs/contracts/pipeline-operator-surface-and-id-resolution.md`
- **Basis posture**:
  - Currentness: current (revalidated against the landed `SEAM-1` and `SEAM-2` closeouts and the published `THR-01` / `THR-02` handoff)
  - Upstream closeouts assumed:
    - `../../governance/seam-1-closeout.md`
    - `../../governance/seam-2-closeout.md`
  - Required threads:
    - `THR-01`
    - `THR-02`
  - Stale triggers:
    - If `SEAM-1` changes route-basis fields, route-state semantics, or activation ownership rules, this seam must revalidate compile freshness and inactive-stage refusal behavior before execution.
    - If `SEAM-2` changes canonical-id or shorthand lookup rules, raw-path posture, or shipped help exposure, this seam must revalidate compile-target selection and operator recovery guidance before execution.
    - If the published operator surface changes normalized `resolve` wording or refusal classes, this seam must revalidate the compile-boundary defer posture before execution.
- **Threading constraints**
  - Upstream blockers: none beyond the published `SEAM-1` and `SEAM-2` closeouts and the published `THR-01` / `THR-02` threads
  - Downstream blocked seams: `SEAM-4`
  - Contracts produced: `C-10`
  - Contracts consumed:
    - `C-08`
    - `C-09`
  - Canonical contract refs:
    - `docs/contracts/stage-compile-boundary-and-route-freshness.md`
    - `docs/contracts/pipeline-route-and-state-core.md`
    - `docs/contracts/pipeline-operator-surface-and-id-resolution.md`

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Why this seam needs an explicit exit gate**: `C-10` and `THR-03` are the compile-boundary handoff for the conformance seam and later M2 compile work. Promotion must consume one closeout-backed signal that compile freshness, inactive-stage refusal, and source-of-truth boundaries are aligned with published upstream route and operator truth.
- **Expected contracts to publish**: `C-10`
- **Expected threads to publish / advance**: `THR-03`
- **Likely downstream stale triggers**:
  - any change to compile-target selection semantics or canonical-id expectations
  - any change to route-basis freshness inputs or inactive-stage refusal wording
  - any change to the activation-equivalence rule between pipeline YAML and stage front matter
- **Expected closeout evidence**:
  - landed `C-10` contract text and evidence links
  - compile-boundary verification surfaces and refusal evidence under `crates/compiler`
  - tests or proof rails that pin freshness refusal and inactive-stage behavior
  - downstream revalidation triggers recorded for `SEAM-4`

## Slice index

- `S00` -> `slice-00-c-10-stage-compile-boundary-and-route-freshness-contract.md`
- `S1` -> `slice-1-compile-target-selection-and-source-of-truth-boundary.md`
- `S2` -> `slice-2-route-basis-freshness-and-inactive-stage-refusals.md`
- `S3` -> `slice-3-activation-equivalence-and-stage-payload-handoff.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-3-closeout.md`
