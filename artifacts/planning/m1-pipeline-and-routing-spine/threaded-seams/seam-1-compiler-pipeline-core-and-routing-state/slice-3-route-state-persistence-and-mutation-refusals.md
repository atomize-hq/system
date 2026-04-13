---
slice_id: S3
seam_id: SEAM-1
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to the `.system/state/pipeline/` schema, revision protocol, audit-history bounds, or atomic-write requirements requires this slice to revalidate.
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
contracts_produced: []
contracts_consumed:
  - C-08
open_remediations: []
---

### S3 - Route-State Persistence and Mutation Refusals

- **User/system value**: Route-relevant state becomes safely persistable across multi-step planning flows without turning `.system/state/` into hidden canonical truth or accepting unsafe concurrent mutation.
- **Scope (in/out)**:
  - In:
    - `.system/state/pipeline/<pipeline-id>.yaml` schema and typed read/write surface
    - bounded audit-history behavior for state changes
    - advisory locking, revision checks, and atomic write-then-rename semantics
    - malformed-state refusal, revision-conflict refusal, and typed mutation outcomes
  - Out:
    - generalized workflow state machines or provenance logs
    - CLI command syntax for `pipeline state set`
    - docs/help cutover for the supported surface
- **Acceptance criteria**:
  - The persisted state shape is narrow, explicit, and limited to route-relevant keys named in `C-08`.
  - State writes refuse unsafe conditions instead of silently overwriting newer data.
  - Malformed or out-of-contract state files fail cleanly at the compiler boundary.
  - Mutation outcomes are typed and reusable by downstream CLI/reporting layers.
- **Dependencies**:
  - Inputs: `S2` route-result model, `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
  - Expected code surfaces: compiler-owned state module(s) under `crates/compiler/src/`
- **Verification**:
  - Add a persistence suite such as `crates/compiler/tests/pipeline_state_store.rs`.
  - Cover state-file round-trip, malformed-state refusal, revision-conflict refusal, bounded audit-history trimming, and atomic commit behavior using tempdir-backed tests.
  - Verify `.system/state/**` remains runtime-only by contract and is not treated as canonical artifact input by the surrounding compiler surfaces.
- **Rollout/safety**:
  - Keep the schema closed for `M1`; avoid open-ended maps or generalized orchestration state.
  - Refuse silent last-write-wins behavior.
- **Review surface refs**: `../../review_surfaces.md` (`R2`, `R3`)
