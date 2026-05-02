---
slice_id: S00
seam_id: SEAM-3
slice_kind: contract_definition
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any rename of compile-target selection rules, freshness inputs, inactive-stage refusal posture, or activation-equivalence wording after `C-10` is drafted requires revalidation before execution continues.
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
contracts_produced:
  - C-10
contracts_consumed:
  - C-08
  - C-09
open_remediations: []
---

### S00 - Define `C-10` Stage Compile Boundary and Route Freshness Contract

- **User/system value**: Downstream seams get one explicit compile-boundary contract for target selection, source-of-truth ownership, freshness refusal, and stage-payload handoff instead of rediscovering those rules from future implementation details.
- **Scope (in/out)**:
  - In:
    - choose the canonical contract artifact path `docs/contracts/stage-compile-boundary-and-route-freshness.md`
    - define normative rules for compile-target selection over the published `pipeline` operator surface
    - define the source-of-truth split between pipeline YAML orchestration data and stage front-matter compile data
    - define route-basis freshness, inactive-stage refusal, and activation-equivalence posture
    - define the stage-payload handoff expected once compile lands in M2
    - define compatibility and revalidation triggers for consumers of `THR-03`
  - Out:
    - actual compile implementation or file writes
    - exposing `pipeline compile` as shipped M1 surface
    - proof-corpus and docs/help cutover work owned by `SEAM-4`
- **Acceptance criteria**:
  - `C-10` has one canonical descriptive home at `docs/contracts/stage-compile-boundary-and-route-freshness.md`.
  - The contract makes source-of-truth ownership concrete enough to implement: what compile reads from route truth, what it reads from stage front matter, and what it must refuse when those inputs drift.
  - The contract names inactive-stage refusal and freshness refusal posture concretely enough for downstream implementation.
  - The verification checklist names the concrete compiler and docs surfaces needed for this seam to later pass `gates.pre_exec.contract`.
- **Dependencies**:
  - Inputs: `../../threading.md`, `../../scope_brief.md`, `../../seam-3-stage-compile-boundary-and-route-freshness-handoff.md`, the landed `SEAM-1` and `SEAM-2` closeouts, and current stage front matter under `core/stages/*.md`
  - External contract constraints: `docs/contracts/pipeline-route-and-state-core.md`, `docs/contracts/pipeline-operator-surface-and-id-resolution.md`
- **Verification**:
  - Document-level: a downstream planner can answer "what compile consumes", "when compile must refuse stale or inactive basis", and "how pipeline-vs-stage ownership is split" without reading future compile implementation code.
  - Planned verification should name the future compile-boundary surfaces under `crates/compiler`, stage metadata under `core/stages/*.md`, and any tests that pin freshness or inactive-stage refusal.

#### Contract baseline

- The canonical contract lives at `docs/contracts/stage-compile-boundary-and-route-freshness.md`.
- Compile-target selection rules:
  - compile consumes a canonical pipeline id, a canonical stage id, and one previously resolved route basis
  - compile reuses the published operator-surface selection rules rather than inventing compile-only target syntax
  - stage ids are meaningful only within the context of the selected pipeline and resolved route
- Source-of-truth split:
  - pipeline YAML owns orchestration order and route-entry activation
  - stage front matter owns compile-facing stage payload metadata
  - compile must refuse if the upstream route basis and stage-front-matter expectations drift out of contract
- Freshness and refusal rules:
  - compile must refuse stale route basis instead of silently re-running `pipeline resolve`
  - inactive stages must be refused explicitly
  - activation duplicated in pipeline YAML and stage front matter must remain semantically equivalent during the transition or compile must refuse
- M2 handoff rules:
  - this seam defines the stage-payload handoff contract expected once compile lands
  - `pipeline compile` remains out of shipped M1 help/docs surface until later milestone work lands

#### Owner execution checklist

- `S1` must land canonical compile-target selection and the source-of-truth split between pipeline YAML and stage front matter.
- `S2` must land route-basis freshness and inactive-stage refusal rules without reinterpreting published route truth.
- `S3` must land activation-equivalence and stage-payload handoff rules concrete enough for downstream implementation and conformance planning.
- Publication or acceptance of the canonical contract remains post-exec evidence for seam exit and thread publication; it is not a pre-exec dependency for this producer seam.
