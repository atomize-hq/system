---
slice_id: S00
seam_id: SEAM-3
slice_kind: contract_definition
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to direct packet inputs, inherited posture dependencies, override-with-rationale rules, or manifest versioning/freshness fields requires downstream revalidation.
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
  - THR-03
contracts_produced:
  - C-03
contracts_consumed:
  - C-01
  - C-02
open_remediations: []
---

### S00 - Define `C-03` Canonical Artifact Manifest Contract

- **User/system value**: Downstream seams can implement deterministic packet resolution against one explicit truth for canonical inputs, freshness semantics, and override-with-rationale rules.
- **Scope (in/out)**:
  - In:
    - choose the canonical `C-03` contract artifact location and format (aligned with `docs/contracts/C-01-*.md` and `docs/contracts/C-02-*.md`)
    - write normative `C-03` rules for:
      - canonical `.system/` paths and allowed direct inputs
      - optional vs required artifacts
      - inherited posture dependencies that affect freshness
      - override-with-rationale requirements and forbidden override patterns
      - deterministic freshness fields (including versioning policy)
    - define revalidation triggers + compat expectations for consumers (`SEAM-4` through `SEAM-7`)
  - Out:
    - resolver refusal taxonomy and budget/decision-log behavior (`SEAM-4`)
    - renderer ordering/copy (`SEAM-5`)
    - execution-demo boundary semantics (`SEAM-6`)
- **Acceptance criteria**:
  - `C-03` exists as a concrete artifact (doc + stable location) referenced by `threading.md` and consumable by `SEAM-4` without guessing.
  - Contract includes a verification checklist that can later mark `gates.pre_exec.contract = passed` for `SEAM-3`.
  - Contract explicitly forbids treating derived views (README, PLAN excerpts, rendered outputs) as canonical runtime inputs.
- **Dependencies**:
  - Inputs: `C-01` repo-surface/runtime boundary posture, `C-02` crate ownership boundary + “setup-first” CLI posture, and `threading.md` contract registry entry for `C-03`.
  - Upstream revalidation: satisfied by `SEAM-2` closeout and published `C-02` / `THR-02` (recorded at the seam level).
- **Verification**:
  - Document-level: a downstream seam planner can answer “what are the canonical inputs”, “what is optional”, “what fields define freshness”, and “what overrides are allowed” without reading implementation.
  - Spot-check: `threading.md` `C-03` definition matches the contract’s nouns/paths and versioning semantics.
- **Rollout/safety**:
  - Treat new live inputs and new refusal sources as contract bumps (schema/versioning).
  - Keep the manifest request-scoped and in-memory by default unless a future seam explicitly owns persistence semantics.
- **Review surface refs**: `../../review_surfaces.md` (R1 operator workflow, R2 runtime boundary, R3 touch surface map)

For `C-03`, the contract must be concrete enough that the producer seam can later satisfy `gates.pre_exec.contract` without requiring the final accepted contract artifact or downstream resolver behavior to have already landed.

#### S00.T1 - Choose the canonical contract artifact location and format

- **Outcome**: `C-03` has one repo-relative home (example: `docs/contracts/C-03-canonical-artifact-manifest.md`) and is referenced where drift would otherwise occur (threading contract registry, docs index, and/or CLI/help text pointers as appropriate).
- **Thread/contract refs**: `THR-03`, `C-03` (and boundary constraints from `C-01`, `C-02`)

#### S00.T2 - Write the contract rules (normative)

- **Outcome**: A rules section with MUST/MUST NOT/SHOULD language covering:
  - canonical paths:
    - `.system/charter/CHARTER.md` (required)
    - `.system/project_context/PROJECT_CONTEXT.md` (optional)
    - `.system/feature_spec/FEATURE_SPEC.md` (required)
  - canonical-truth rule: repo-local `.system/` is authoritative; derived docs are not runtime inputs
  - inherited posture dependencies:
    - what counts as an inherited dependency (and what does not)
    - how dependencies influence freshness deterministically without being “packet body” inputs
  - override-with-rationale:
    - what can be overridden, what cannot, and how rationale is recorded
    - forbidden patterns (silent override, override that hides freshness truth, override that introduces a new input surface)
  - freshness fields:
    - required fields (e.g. presence state, content identity, last-modified, derived version stamps) and deterministic computation rules
    - explicit distinction between schema version and manifest generation version
  - refusal sources:
    - what conditions are treated as “missing”, “stale”, “contradictory”, or “unsupported” inputs (to be consumed by `SEAM-4`)
  - compatibility and revalidation triggers for consumers
- **Thread/contract refs**: `THR-03`, `C-03`

#### S00.T3 - Add a verification checklist for the contract gate

- **Outcome**: A checklist that can later be used to mark `gates.pre_exec.contract = passed`, including:
  - contract paths and optional/required rules match `threading.md`
  - freshness computation rules are deterministic and testable
  - override-with-rationale rules are precise and include forbidden-case examples
  - explicit “no derived docs as runtime input” constraint is stated and linked to `C-01`
- **Thread/contract refs**: `THR-03`, `C-03`
