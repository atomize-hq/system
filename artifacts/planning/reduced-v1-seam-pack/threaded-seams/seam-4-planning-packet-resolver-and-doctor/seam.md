---
seam_id: SEAM-4
seam_slug: planning-packet-resolver-and-doctor
status: exec-ready
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-4-planning-packet-resolver-and-doctor.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts:
    - SEAM-2
    - SEAM-3
  required_threads:
    - THR-02
    - THR-03
  stale_triggers:
    - Any change to command hierarchy, direct packet inputs, freshness semantics, or budget policy.
    - Any change to refusal copy requirements or `doctor` as the canonical recovery verb.
gates:
  pre_exec:
    review: passed
    contract: passed
    revalidation: passed
  post_exec:
    landing: pending
    closeout: pending
seam_exit_gate:
  required: true
  planned_location: S99
  status: pending
open_remediations: []
---

# SEAM-4 - Planning Packet Resolver and Doctor

## Seam Brief (Restated)

- **Goal / value**: Deliver deterministic planning packet selection, budget handling, compact refusal behavior, and `doctor` blocker diagnosis from one typed resolver truth.
- **Type**: capability
- **Scope**
  - In:
    - consume canonical manifest + freshness truth (`C-03`)
    - deterministic packet selection (same inputs -> same packet)
    - typed budget policy and outcomes
    - typed decision log (stable ordering, inspectable)
    - refusal taxonomy + compact refusal structure for `generate`
    - shared blocker model surfaced by `doctor`
  - Out:
    - renderer-specific formatting and ordering (`SEAM-5`)
    - fixture execution demo behavior (`SEAM-6`)
    - conformance rails, golden tests, and docs cutover (`SEAM-7`)
- **Touch surface**:
  - resolver core in `crates/compiler`
  - decision-log and resolver-result types (`C-04`)
  - refusal + blocker policy logic
  - `doctor` command behavior in `crates/cli` (consuming compiler truth)
- **Verification**:
  - For the owned contract (`C-04`), the resolver result shape, refusal semantics, and `doctor` blocker outputs are concrete enough (rules + verification checklist) that downstream seams can implement without guessing.
  - Reserve “accepted/published contract artifact exists” for seam-exit planning and closeout evidence.
- **Basis posture**:
  - Currentness: provisional (depends on `C-03` / `THR-03` publication from `SEAM-3`)
  - Upstream closeouts assumed: `SEAM-2`, `SEAM-3`
  - Required threads: `THR-02` (published), `THR-03` (defined; must publish before revalidation can pass)
  - Stale triggers:
    - Any change to command hierarchy, direct packet inputs, freshness semantics, or budget policy.
    - Any change to refusal copy requirements or `doctor` as the canonical recovery verb.
- **Threading constraints**
  - Upstream blockers: `SEAM-2` (`C-02` via `THR-02`), `SEAM-3` (`C-03` via `THR-03`)
  - Downstream blocked seams: `SEAM-5`, `SEAM-7` (direct); derived: `SEAM-6`
  - Contracts produced: `C-04`
  - Contracts consumed: `C-01`, `C-02`, `C-03`

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Why this seam needs an explicit exit gate**: `C-04` is the shared truth that downstream proof surfaces (`SEAM-5`) and conformance (`SEAM-7`) must trust; promotion must key off closeout-backed evidence that `generate` and `doctor` are aligned on one typed resolver result.
- **Expected contracts to publish**: `C-04`
- **Expected threads to publish / advance**: `THR-04` (publish)
- **Likely downstream stale triggers**:
  - any change to budget policy or budget outcome classification
  - any change to refusal ordering, refusal categories, or required refusal fields (“exact next action”)
  - any change to blocker taxonomy or stable ordering guarantees
  - any change to packet identity / selection-reason fields consumed by renderers and conformance
- **Expected closeout evidence**:
  - a concrete `C-04` contract artifact (rules + verification checklist) aligned with `threading.md`
  - landed resolver-result + decision-log types in `crates/compiler`
  - landed parity evidence: `generate` and `doctor` consume the same typed resolver truth and cannot drift
  - recorded downstream revalidation triggers for `SEAM-5` through `SEAM-7` if contract deltas occurred

## Slice index

- `S00` -> `slice-00-c-04-resolver-result-and-doctor-blockers-contract.md`
- `S1` -> `slice-1-resolver-core-packet-selection-and-budget.md`
- `S2` -> `slice-2-refusal-taxonomy-and-compact-refusal-structure.md`
- `S3` -> `slice-3-doctor-blocker-aggregation-and-next-actions.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-4-closeout.md`
