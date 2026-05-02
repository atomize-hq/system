---
slice_id: S00
seam_id: SEAM-4
slice_kind: contract_definition
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to budget policy, refusal shape, blocker taxonomy, or decision-log fields requires downstream revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-04
contracts_produced:
  - C-04
contracts_consumed:
  - C-01
  - C-02
  - C-03
open_remediations: []
---

### S00 - Define `C-04` Resolver Result and Doctor Blockers Contract

- **User/system value**: Downstream seams (`SEAM-5`, `SEAM-7`) consume one typed resolver truth for packet identity, selection decisions, freshness/budget outcomes, refusal structure, and `doctor` blocker reporting.
- **Scope (in/out)**:
  - In:
    - choose the canonical `C-04` contract artifact location and format (aligned with `docs/contracts/C-01-*.md` and `docs/contracts/C-02-*.md`)
    - write normative `C-04` rules for:
      - resolver inputs (manifest/freshness truth consumed from `C-03`; no derived-doc inputs)
      - packet identity + selection-reason fields (stable ordering)
      - decision log schema (typed, ordered, inspectable)
      - budget policy outcomes (typed; inspectable; deterministic)
      - refusal taxonomy + compact refusal structure (including “exact next action”)
      - blocker taxonomy and stable ordering guarantees for `doctor`
      - invariants tying `generate` and `doctor` to the same resolver truth
    - define revalidation triggers + compat expectations for consumers (`SEAM-5` through `SEAM-7`)
  - Out:
    - renderer ordering/copy (`SEAM-5`)
    - fixture execution demo boundary semantics (`SEAM-6`)
    - conformance rails and docs cutover (`SEAM-7`)
- **Acceptance criteria**:
  - `C-04` exists as a concrete artifact: `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`.
  - Contract includes a verification checklist that can later be used to mark `gates.pre_exec.contract = passed` for `SEAM-4`.
  - Contract explicitly forbids:
    - `generate` and `doctor` computing blockers independently
    - non-deterministic ordering of decision log entries or blocker lists
    - refusal behavior that omits an exact next safe action
- **Dependencies**:
  - Inputs: `C-02` (command surface + `doctor` posture) and `C-03` (manifest + freshness truth).
  - Boundary constraints: `C-01` (repo surface and “Rust-first supported path” messaging).
- **Verification**:
  - Document-level: a downstream seam planner can answer “what fields exist”, “what is ordered”, “what is refused”, “what is blocked”, and “what is the next action” without reading implementation.
  - Spot-check: `threading.md` `C-04` definition matches the contract’s nouns and versioning semantics.
- **Rollout/safety**:
  - Treat any new refusal category, new budget outcome, or new blocker category as a contract bump with downstream revalidation.

For `C-04`, the contract must be concrete enough that the producer seam can later satisfy `gates.pre_exec.contract` without requiring the final accepted contract artifact or downstream renderer/conformance work to have already landed.

#### S00.T1 - Choose the canonical contract artifact location and format

- **Outcome**: `C-04` has one repo-relative home: `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`.
- **Thread/contract refs**: `THR-04`, `C-04`

#### S00.T2 - Write the contract rules (normative)

- **Outcome**: A rules section with MUST/MUST NOT/SHOULD language covering:
  - resolver inputs:
    - consumes `C-03` manifest + freshness truth only (no derived docs)
  - resolver outputs:
    - selected packet identity (stable identifier) and selection-reason fields (stable ordering)
    - typed decision log: entry types, required fields, and ordering guarantees
    - typed budget outcome: outcome enum + reason + “exact next action”
    - typed refusal: category + compact summary + broken dependency/artifact + exact next action
    - typed blockers: category + subject + summary + exact next action + stable ordering guarantees
  - invariants:
    - `doctor` is a view over resolver truth; it must not compute a different blocker set than `generate`
    - “same inputs -> same outputs” includes decision-log ordering and blocker ordering
  - versioning and revalidation triggers:
    - define a `C-04` version field and explicit compat expectations for consumers
- **Thread/contract refs**: `THR-04`, `C-04`

#### S00.T3 - Add a verification checklist for the contract gate

- **Outcome**: A checklist that can later be used to mark `gates.pre_exec.contract = passed`, including:
  - `generate` and `doctor` share one typed resolver result
  - decision-log and blocker ordering are deterministic
  - refusal structure is compact and includes an exact next action
  - budget outcomes are typed and inspectable
- **Thread/contract refs**: `THR-04`, `C-04`
