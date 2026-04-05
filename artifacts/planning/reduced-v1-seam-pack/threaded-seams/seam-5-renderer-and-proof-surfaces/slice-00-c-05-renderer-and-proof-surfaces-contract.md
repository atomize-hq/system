---
slice_id: S00
seam_id: SEAM-5
slice_kind: contract_definition
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to trust-header ordering, proof ordering, refusal layout, or JSON fallback behavior requires downstream revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-05
contracts_produced:
  - C-05
contracts_consumed:
  - C-04
open_remediations: []
---

### S00 - Define `C-05` Renderer and Proof-Surfaces Contract

- **User/system value**: Downstream conformance (`SEAM-7`) can pin stable output ordering and semantics for markdown, JSON, and inspect views without parsing implementation details.
- **Scope (in/out)**:
  - In:
    - choose canonical `C-05` artifact location under `docs/contracts/`
    - define output surface nouns and ordering rules
    - define deterministic trust header fields and ordering
    - define inspect proof ordering rules
    - define JSON fallback requirements for dense evidence views
    - define how renderer failures are isolated from successful resolver results
  - Out:
    - packet selection / refusal semantics (owned by `C-04`)
    - fixture demo boundary semantics (`SEAM-6`)
- **Acceptance criteria**:
  - `docs/contracts/C-05-renderer-and-proof-surfaces.md` exists with concrete MUST/MUST NOT rules.
  - Contract includes a verification checklist usable by `SEAM-7` conformance rails.

