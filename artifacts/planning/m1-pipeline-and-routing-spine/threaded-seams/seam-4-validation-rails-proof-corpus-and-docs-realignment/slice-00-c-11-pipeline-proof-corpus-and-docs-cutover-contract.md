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
    - Any rename of proof-corpus requirements, docs/help parity rules, or safety-boundary wording after `C-11` is drafted requires revalidation before execution continues.
gates:
  pre_exec:
    review: pending
    contract: pending
    revalidation: pending
  post_exec:
    landing: pending
    closeout: pending
threads: []
contracts_produced:
  - C-11
contracts_consumed:
  - C-08
  - C-09
  - C-10
open_remediations: []
---

### S00 - Define `C-11` Pipeline Proof Corpus and Docs Cutover Contract

- **User/system value**: Downstream milestone packs get one explicit conformance contract for proof corpus, docs/help parity, malformed-state rails, and the M1 safety boundary instead of rediscovering those rules from future implementation details.
- **Scope (in/out)**:
  - In:
    - choose the canonical contract artifact path `docs/contracts/pipeline-proof-corpus-and-docs-cutover.md`
    - define the proof-corpus requirements for compiler and CLI conformance checks
    - define docs/help parity and command-hierarchy realignment rules for the shipped `pipeline` subset
    - define malformed-state refusal, lock/revision conflict, and performance/security boundary expectations
    - define compatibility and revalidation triggers for consumers of `THR-04`
  - Out:
    - actual test implementation or fixture writes
    - unsupported public release/distribution work
    - M2 compile implementation or compile proof outputs
- **Acceptance criteria**:
  - `C-11` has one canonical descriptive home at `docs/contracts/pipeline-proof-corpus-and-docs-cutover.md`.
  - The contract makes proof-corpus ownership concrete enough to implement: what shared fixtures and goldens are required, what refusal classes must be exercised, and what docs/help claims are allowed.
  - The contract names the docs/help parity boundaries concretely enough for downstream conformance work.
  - The verification checklist names the concrete tests, docs, and help surfaces needed for this seam to later pass `gates.pre_exec.contract`.
- **Dependencies**:
  - Inputs: `../../threading.md`, `../../scope_brief.md`, `../../seam-4-validation-rails-proof-corpus-and-docs-realignment.md`, and the landed `SEAM-1`, `SEAM-2`, and `SEAM-3` closeouts
  - External contract constraints: `docs/contracts/pipeline-route-and-state-core.md`, `docs/contracts/pipeline-operator-surface-and-id-resolution.md`, `docs/contracts/stage-compile-boundary-and-route-freshness.md`
- **Verification**:
  - Document-level: a downstream planner can answer "what proof corpus is shared", "what docs/help parity is required", and "what safety rails must stay explicit" without reading future implementation code.
  - Planned verification should name the future compiler and CLI tests, docs/help snapshots, and any drift checks that pin malformed-state refusal behavior.
