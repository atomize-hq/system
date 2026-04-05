---
slice_id: S3
seam_id: SEAM-1
slice_kind: implementation
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any addition of supported runtime code paths that import, execute, or wrap content under `archived/`.
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
  - C-01
open_remediations: []
---

### S3 - Add Guardrails for the Archive/Runtime Boundary

- **User/system value**: Prevent future accidental violations of the "archived is reference-only" rule, especially once Rust workspace code exists.
- **Scope (in/out)**:
  - In:
    - document the boundary rule in one canonical location (the contract)
    - add at least one low-friction guardrail to catch violations early (lint, script, or CI check), scoped so it does not become SEAM-7's conformance job
  - Out:
    - full conformance rails, golden tests, and CI enforcement program (owned by `SEAM-7`)
- **Acceptance criteria**:
  - Guardrail exists and is easy to run locally.
  - It fails when a supported-path file tries to import/execute/wrap anything under `archived/`.
  - It is documented in the contract verification checklist.
- **Dependencies**:
  - `SEAM-2` will later introduce Rust workspace paths; guardrail should either be forward-compatible or explicitly scoped to "once crates/ exists".
- **Verification**:
  - Add at least one synthetic fixture case that the guardrail would reject.
- **Rollout/safety**:
  - Keep guardrail strict but narrow: focus on the archive/runtime boundary only.
- **Review surface refs**: `../../review_surfaces.md` (R2)

#### S3.T1 - Define what "supported runtime path" means for enforcement purposes

- **Outcome**: The guardrail has an unambiguous target set (example: future `crates/` + any supported entrypoints).
- **Inputs/outputs**:
  - Inputs: `C-01` rules
  - Outputs: enforcement definition section in contract + checklist step
- **Thread/contract refs**: `THR-01`, `C-01`

#### S3.T2 - Implement a minimal archive-boundary check

- **Outcome**: A script or lint rule that fails on forbidden references from supported-path code into `archived/`.
- **Inputs/outputs**:
  - Inputs: repo layout + enforcement definition
  - Outputs: check runnable locally, optionally wired into existing validation hooks
- **Thread/contract refs**: `THR-01`, `C-01`
