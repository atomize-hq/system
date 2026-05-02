---
seam_id: SEAM-1
seam_slug: compiler-pipeline-core-and-routing-state
status: landed
execution_horizon: future
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-1-compiler-pipeline-core-and-routing-state.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts: []
  required_threads: []
  stale_triggers:
    - If the approved parser base changes from `serde_yaml_bw` or the two-document pipeline shape changes, this seam must revalidate before execution.
    - If the canonical-vs-runtime `.system/` boundary changes in upstream docs/contracts, this seam must revalidate route-state storage assumptions.
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

# SEAM-1 - Compiler Pipeline Core and Routing State

## Seam Brief (Restated)

- **Goal / value**: Make `crates/compiler` the single source of truth for declared pipeline loading, deterministic route evaluation, and narrow route-state mutation so every downstream `pipeline` surface consumes one typed model.
- **Type**: capability
- **Scope**
  - In:
    - compiler-owned loading of the approved two-document pipeline YAML shape
    - repo-safe path validation for `pipelines/*.yaml` and `core/stages/*.md`
    - deterministic per-stage route status computation with explicit reasons
    - supported activation subset validation and refusal semantics
    - `.system/state/pipeline/<pipeline-id>.yaml` schema, bounded audit history, and typed mutation outcomes
  - Out:
    - CLI command wiring, help posture, and id lookup (`SEAM-2`)
    - compile payload generation or stage materialization (`SEAM-3`)
    - docs/help cutover and proof-corpus ownership (`SEAM-4`)
- **Touch surface**:
  - `crates/compiler/src/pipeline.rs`
  - `crates/compiler/src/lib.rs`
  - compiler-owned route/state modules that land under `crates/compiler/src/`
  - `pipelines/*.yaml`
  - `core/stages/*.md`
  - `.system/state/pipeline/`
  - `crates/compiler/tests/pipeline_loader.rs`
  - new compiler route/state tests under `crates/compiler/tests/`
- **Verification**:
  - A downstream seam can cite one typed route result vocabulary (`active`, `skipped`, `blocked`, `next`) and one narrow state-mutation outcome surface without reconstructing behavior from CLI code.
  - For the owned contract (`C-08`), pre-exec readiness now rests on the concrete baseline in `docs/contracts/pipeline-route-and-state-core.md` plus the owner execution checklist in `S00`.
  - Publication or acceptance of the canonical `C-08` artifact remains seam-exit evidence, not a pre-exec prerequisite for the producing seam.
- **Canonical contract refs**:
  - `docs/contracts/pipeline-route-and-state-core.md`
  - `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- **Basis posture**:
  - Currentness: current (checked against the existing pipeline loader, pipeline YAML fixtures, and current `.system/` runtime-zone contract wording)
  - Upstream closeouts assumed: none
  - Required threads: none
  - Stale triggers:
    - If the approved parser base changes from `serde_yaml_bw` or the two-document pipeline shape changes, this seam must revalidate before execution.
    - If the canonical-vs-runtime `.system/` boundary changes in upstream docs/contracts, this seam must revalidate route-state storage assumptions.
- **Threading constraints**
  - Upstream blockers: `M0.5` parser gate completion and continued validity of the external `.system/` runtime-zone rule from `C-03`
  - Downstream blocked seams: `SEAM-2`, `SEAM-3`, `SEAM-4` (consume `C-08` via `THR-01`)
  - Contracts produced: `C-08`
  - Contracts consumed: none inside this pack; external contract constraints come from `C-02` and `C-03`
  - Canonical contract refs:
    - `docs/contracts/pipeline-route-and-state-core.md`
    - `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
    - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Why this seam needs an explicit exit gate**: `C-08` and `THR-01` are the critical-path handoff for every downstream seam in the pack. Promotion must consume one closeout-backed signal that route truth, state semantics, and downstream revalidation triggers are actually landed.
- **Expected contracts to publish**: `C-08`
- **Expected threads to publish / advance**: `THR-01` (publish); `SEAM-2`, `SEAM-3`, and `SEAM-4` revalidate against the published route/state contract
- **Likely downstream stale triggers**:
  - any change to route status names or reason semantics
  - any change to supported activation syntax or evaluation rules
  - any change to the `.system/state/pipeline/` schema or mutation concurrency protocol
  - any change to repo-safe pipeline or stage path rules
- **Expected closeout evidence**:
  - landed `C-08` contract text and evidence links
  - compiler route result and state-mutation implementation surfaces
  - tests proving deterministic route ordering, refusal classes, and state-mutation protocol
  - downstream revalidation triggers recorded for `SEAM-2`, `SEAM-3`, and `SEAM-4`

## Slice index

- `S00` -> `slice-00-c-08-pipeline-route-and-state-core-contract.md`
- `S1` -> `slice-1-pipeline-definition-ingest-and-declared-route-shape.md`
- `S2` -> `slice-2-route-evaluation-and-status-reasons.md`
- `S3` -> `slice-3-route-state-persistence-and-mutation-refusals.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-1-closeout.md`
