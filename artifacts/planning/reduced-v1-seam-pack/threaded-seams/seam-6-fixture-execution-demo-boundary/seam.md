---
seam_id: SEAM-6
seam_slug: fixture-execution-demo-boundary
status: exec-ready
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-6-fixture-execution-demo-boundary.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts:
    - SEAM-4
  required_threads:
    - THR-04
  stale_triggers:
    - Any change to fixture lineage assumptions, unsupported live execution scope, or refusal wording.
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
open_remediations:
  - REM-002
---

# SEAM-6 - Fixture Execution Demo Boundary

## Seam Brief (Restated)

- **Goal / value**: Prove the reduced-v1 execution demo can exist without misrepresenting the product as supporting live slice execution.
- **Type**: risk
- **Scope**
  - In:
    - fixture-backed execution lineage for demos
    - execution demo path
    - explicit refusal for unsupported live slice requests
    - wording and evidence that keeps the demo honest
  - Out:
    - live slice lineage
    - live execution packets
    - review/fix packet support
- **Primary interfaces**
  - Inputs: `C-04`
  - Outputs: `C-06`
  - Threads: `THR-06`

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Expected contracts to publish**: `C-06`
- **Expected threads to publish / advance**: `THR-06`
- **Likely downstream stale triggers**:
  - Any change to fixture lineage definitions or what counts as a “fixture-backed” demo.
  - Any change to refusal wording that could be misread as live capability.
  - Any change to how the demo path is invoked (CLI flag, fixture tooling, or test harness entrypoint).

## Slice index

- `S00` -> `slice-00-c-06-fixture-execution-demo-boundary-contract.md`
- `S1` -> `slice-1-fixture-lineage-and-demo-request-surface.md`
- `S2` -> `slice-2-live-refusal-and-demo-proof-surfaces.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-6-closeout.md`
