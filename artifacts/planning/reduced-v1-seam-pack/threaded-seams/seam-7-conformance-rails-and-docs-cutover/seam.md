---
seam_id: SEAM-7
seam_slug: conformance-rails-and-docs-cutover
status: decomposed
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-7-conformance-rails-and-docs-cutover.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts:
    - SEAM-1
    - SEAM-2
    - SEAM-3
    - SEAM-4
    - SEAM-5
    - SEAM-6
  required_threads:
    - THR-01
    - THR-02
    - THR-03
    - THR-04
    - THR-05
    - THR-06
  stale_triggers:
    - Any upstream seam closeout changes repo boundary, command vocabulary, manifest truth, resolver results, output ordering, or demo/refusal semantics.
    - Any supported target or CI/install-smoke requirement changes.
gates:
  pre_exec:
    review: pending
    contract: pending
    revalidation: pending
  post_exec:
    landing: pending
    closeout: pending
seam_exit_gate:
  required: true
  planned_location: S99
  status: pending
open_remediations: []
---

# SEAM-7 - Conformance Rails and Docs Cutover

## Seam Brief (Restated)

- **Goal / value**: Lock in the reduced v1 truth with tests, CI, install smoke, drift guards, help/docs parity, and final cutover messaging so the shipped repo tells one coherent story.
- **Type**: conformance
- **Scope**
  - In:
    - unit/integration/CLI-e2e tests and drift guards bound to published contracts `C-01..C-06`
    - CI rails for fmt/test/smoke (and any required install checks)
    - README/help/docs alignment with the supported Rust-first story
  - Out:
    - inventing new supported runtime behavior
    - redefining upstream contracts (those are inputs here)
- **Touch surface**:
  - `tests/`
  - CI workflow config
  - `README.md` and docs indexes/help examples
- **Verification**:
  - Every conformance surface must map to a published upstream contract or thread.
  - No free-floating cleanup bucket: changes must be evidence-driven and contract-scoped.

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Expected contracts to publish**: `C-07`
- **Expected threads to publish / advance**: `THR-07`
- **Likely downstream stale triggers**:
  - Any change to `C-01..C-06` semantics, wording, or verification expectations requires revalidation.
- **Expected closeout evidence**:
  - passing `cargo test` surfaces and any CI equivalence
  - install smoke evidence for supported targets
  - docs/help alignment evidence (no drift against runtime behavior)

## Slice index

- `S00` -> `slice-00-c-07-conformance-rails-and-docs-cutover-contract.md`
- `S1` -> `slice-1-conformance-rails-and-ci-smoke.md`
- `S2` -> `slice-2-docs-help-cutover-and-drift-guards.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-7-closeout.md`

