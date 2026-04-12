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
    - Any rename of route statuses, reason classes, supported activation syntax, or state mutation outcomes after `C-08` is drafted requires revalidation before execution continues.
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
  - C-08
contracts_consumed: []
open_remediations: []
---

### S00 - Define `C-08` Pipeline Route and State Core Contract

- **User/system value**: Downstream seams get one compiler-owned contract for pipeline loading, resolved-route truth, and narrow route-state mutation, instead of inferring semantics from CLI wiring or scattered tests.
- **Scope (in/out)**:
  - In:
    - choose the canonical contract artifact path `docs/contracts/pipeline-route-and-state-core.md`
    - define normative rules for pipeline loading, repo-safe path rules, supported activation subset, route statuses and reasons, state schema, audit-history bounds, and mutation concurrency protocol
    - define compatibility and revalidation triggers for consumers of `THR-01`
    - define the contract verification checklist and target test locations
  - Out:
    - CLI command-family behavior and id lookup semantics (`SEAM-2`)
    - compile handoff payload semantics (`SEAM-3`)
    - docs/help cutover and proof-corpus ownership (`SEAM-4`)
- **Acceptance criteria**:
  - `C-08` has one canonical descriptive home at `docs/contracts/pipeline-route-and-state-core.md`.
  - The contract names the only supported route statuses for `M1`: `active`, `skipped`, `blocked`, and `next`, plus the reason/mutation surfaces that explain them.
  - The contract makes `.system/state/pipeline/<pipeline-id>.yaml` concrete enough to implement: allowed keys, revision expectations, audit-history bounds, lock/write semantics, and malformed-state refusal posture.
  - The verification checklist names the concrete test surfaces needed for `SEAM-1` to later pass `gates.pre_exec.contract`.
- **Dependencies**:
  - Inputs: `../../threading.md`, `../../scope_brief.md`, `../../seam-1-compiler-pipeline-core-and-routing-state.md`, `crates/compiler/src/pipeline.rs`, `crates/compiler/tests/pipeline_loader.rs`
  - External contract constraints: `docs/contracts/C-03-canonical-artifact-manifest-contract.md`, `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- **Verification**:
  - Document-level: a downstream planner can answer "what statuses exist", "what makes a route blocked", and "how route-state mutation fails safely" without reading code.
  - Planned tests live in `crates/compiler/tests/pipeline_loader.rs`, plus new route/state suites such as `crates/compiler/tests/pipeline_route_resolution.rs` and `crates/compiler/tests/pipeline_state_store.rs`.
- **Rollout/safety**:
  - Keep planning IDs out of the canonical contract doc; the durable artifact stays descriptive-only.
  - Do not let `C-08` broaden `.system/state/**` into canonical project truth; it remains runtime-only under `C-03`.
- **Review surface refs**: `../../review_surfaces.md` (`R2`, `R3`)

#### Contract baseline

- The canonical contract now lives at `docs/contracts/pipeline-route-and-state-core.md`.
- Loader semantics are anchored in the current compiler evidence at `crates/compiler/src/pipeline.rs` and `crates/compiler/tests/pipeline_loader.rs`:
  - repo-relative pipeline paths only
  - exactly two YAML documents
  - stage files confined to existing regular markdown files under `core/stages/`
  - `activation.when.any|all` with boolean `variables.<name> == true|false` clauses only
- The contract fixes one reduced-v1 route vocabulary: `active`, `skipped`, `blocked`, `next`.
- The contract fixes one closed runtime-state shape under `.system/state/pipeline/<pipeline-id>.yaml` with revisioned boolean variables, bounded audit history, advisory locking, and atomic replace semantics.

#### Owner execution checklist

- `S1` must keep the declared-pipeline ingest surface aligned with the canonical contract and preserve the loader evidence already present in `crates/compiler/tests/pipeline_loader.rs`.
- `S2` must land a compiler-owned route-result model plus `crates/compiler/tests/pipeline_route_resolution.rs` covering declared-order preservation, `skipped` activation semantics, one `next` stage, and refusal of out-of-contract route inputs.
- `S3` must land the state store plus `crates/compiler/tests/pipeline_state_store.rs` covering round-trip persistence, malformed-state refusal, revision-conflict refusal, bounded audit trimming, and atomic replace behavior.
- Publication or acceptance of the canonical contract remains post-exec evidence for seam exit and thread publication; it is not a pre-exec dependency for this producer seam.

#### Unblock outcome

- The contract-definition artifact is now concrete enough for `gates.pre_exec.contract`.
- `REM-001` no longer blocks `status: exec-ready`; it now tracks the remaining landing and seam-exit evidence that must exist before publication is considered complete.

#### S00.T1 - Fix the canonical artifact location and section outline

- **Outcome**: `docs/contracts/pipeline-route-and-state-core.md` is the one durable home for `C-08`, with sections for declared pipeline shape, route result vocabulary, state schema, concurrency/refusal rules, and consumer revalidation triggers.
- **Thread/contract refs**: `THR-01`, `C-08`

#### S00.T2 - Write the normative route/state rules

- **Outcome**: The contract uses explicit MUST/MUST NOT language for:
  - repo-safe pipeline and stage path resolution inside the approved repo surface
  - supported activation clauses and refusal of unsupported shapes
  - deterministic stage-order evaluation and the meaning of `active`, `skipped`, `blocked`, and `next`
  - state schema keys, revision handling, audit-history bounds, advisory locking, and atomic commit behavior
- **Thread/contract refs**: `THR-01`, `C-08`

#### S00.T3 - Add the verification checklist

- **Outcome**: `C-08` names the exact evidence required to pass the contract gate later, including route-resolution tests, malformed-state refusal tests, revision-conflict tests, and thread publication requirements for downstream seams.
- **Thread/contract refs**: `THR-01`, `C-08`
