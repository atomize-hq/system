---
slice_id: S00
seam_id: SEAM-6
slice_kind: contract_definition
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to fixture lineage assumptions, demo invocation surface, or refusal wording requires downstream revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-06
contracts_produced:
  - C-06
contracts_consumed:
  - C-04
open_remediations:
  - REM-002
---

### S00 - Define `C-06` Fixture Execution Demo Boundary Contract

- **User/system value**: Downstream conformance (`SEAM-7`) can pin the fixture-only execution demo boundary and refusal wording without reverse-engineering implementation details.
- **Scope (in/out)**:
  - In:
    - choose canonical `C-06` artifact location under `docs/contracts/`
    - define fixture lineage nouns, ordering rules, and determinism requirements
    - define the demo invocation surface and its explicit “fixture-backed” labeling rules
    - define explicit refusal behavior for unsupported live execution requests (wording + required fields)
  - Out:
    - implementing the demo itself (S1/S2)
    - conformance rails and golden tests (owned by `SEAM-7`)
- **Acceptance criteria**:
  - `docs/contracts/C-06-fixture-execution-demo-boundary.md` exists with concrete MUST/MUST NOT rules.
  - Contract includes a verification checklist usable by `SEAM-7` conformance rails.
