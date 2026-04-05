---
slice_id: S00
seam_id: SEAM-1
slice_kind: contract_definition
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to root layout rules, archive timing, supported-path messaging, or runtime-boundary policy in PLAN.md, README, or root docs.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-01
contracts_produced:
  - C-01
contracts_consumed: []
open_remediations: []
---

### S00 - Define `C-01` Approved Repo-Surface Contract

- **User/system value**: Downstream seams can plan and implement against one explicit root truth: what is supported, what is legacy-only, and what must never become a runtime dependency.
- **Scope (in/out)**:
  - In:
    - write the concrete `C-01` rules (approved surface, supported workflow language, legacy freeze posture, archive/runtime boundary)
    - define versioning/currentness and revalidation triggers as part of the contract doc
  - Out:
    - implementing `SEAM-2` Rust workspace/CLI details (`C-02`)
    - any physical move of legacy harness under `archived/`
- **Acceptance criteria**:
  - `C-01` exists as a concrete artifact (doc + stable location) referenced by root docs.
  - The contract includes a verification checklist with pass/fail conditions.
  - The contract explicitly states what counts as "supported runtime path" today vs transitional legacy behavior.
- **Dependencies**:
  - Inputs: `PLAN.md`, `README.md`, `../../threading.md` contract registry for `C-01`
  - No upstream contracts required.
- **Verification**:
  - Document-level verification: a reviewer can answer "what is supported vs legacy vs archived" without reading code.
  - Spot-check: `README.md` and `PLAN.md` use the contract's exact nouns/verbs for supported vs legacy.
- **Rollout/safety**:
  - Contract is the truth surface; do not land partial copy changes that contradict it.
- **Review surface refs**: `../../review_surfaces.md` (R2 runtime boundary, R3 touch surface map)

For `C-01`, the contract must be concrete enough that the producer seam can later satisfy `gates.pre_exec.contract` without requiring the final archive move or any downstream seam to have landed.

#### S00.T1 - Choose the canonical contract artifact location and format

- **Outcome**: `C-01` has one repo-relative home (example: `docs/contracts/C-01-approved-repo-surface.md`) and is referenced from root docs.
- **Inputs/outputs**:
  - Inputs: existing root docs + plan
  - Outputs: contract doc path + references added to `README.md` / `PLAN.md` / docs index
- **Thread/contract refs**: `THR-01`, `C-01`

#### S00.T2 - Write the contract rules (normative)

- **Outcome**: A rules section with MUST/MUST NOT/SHOULD language covering:
  - approved root surface definition (which top-level paths are approved surface vs legacy vs archived)
  - supported workflow language (Rust-first direction, what is and is not supported during transition)
  - legacy freeze posture (what changes are allowed in legacy harness; what is forbidden)
  - archive/runtime boundary (explicit "no import/execute/wrap archived" rule)
  - versioning + compatibility expectations for consumers (`SEAM-2`, `SEAM-7`)
- **Inputs/outputs**:
  - Input: `../../threading.md` contract registry
  - Output: contract doc content
- **Thread/contract refs**: `THR-01`, `C-01`

#### S00.T3 - Add a verification checklist for the contract gate

- **Outcome**: A checklist that can be used to mark `gates.pre_exec.contract` as passed later.
- **Inputs/outputs**:
  - Inputs: contract rules
  - Outputs: verification steps and pass/fail conditions (including "30 second supported-path discovery" check)
- **Thread/contract refs**: `THR-01`, `C-01`

