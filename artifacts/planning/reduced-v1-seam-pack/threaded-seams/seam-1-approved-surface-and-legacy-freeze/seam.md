---
seam_id: SEAM-1
seam_slug: approved-surface-and-legacy-freeze
status: decomposed
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-1-approved-surface-and-legacy-freeze.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts: []
  required_threads: []
  stale_triggers:
    - Any change to the approved root surface, archive timing, or supported-runtime wording in PLAN.md or root docs.
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

# SEAM-1 - Approved Surface and Legacy Freeze

## Seam Brief (Restated)

- **Goal / value**: Make the repo boundary trustworthy by publishing one Rust-first support story, freezing the legacy Python harness as reference-only, and enforcing an archive/runtime boundary so downstream seams never treat `archived/` as a dependency.
- **Type**: platform
- **Scope**
  - In:
    - root README/help/doc relabeling
    - approved-surface and archive/runtime boundary rules
    - freeze posture for Python harness and wrappers
    - cutover prerequisites for later physical archive move
  - Out:
    - Rust compiler or CLI implementation logic
    - manifest, resolver, renderer, or execution-demo behavior
    - physical archive move before Rust planning packet parity exists
- **Touch surface**:
  - `README.md`, `PLAN.md`, root docs
  - `tools/harness.py`, `tools/harness.sh`
  - future `archived/python-harness/`
- **Verification**:
  - A new contributor can identify the supported Rust-first path (and what is legacy-only) in under 30 seconds.
  - For the owned contract, `C-01` must be concrete enough (rules + verification checklist) that `SEAM-2` and `SEAM-7` can plan without waiting on the physical archive move.
- **Basis posture**:
  - Currentness: current (sanity-checked against repo `README.md` and `PLAN.md` as of this decomposition)
  - Upstream closeouts assumed: none
  - Required threads: none
  - Stale triggers:
    - Any change to the approved root surface, archive timing, or supported-runtime wording in `PLAN.md` or root docs.
- **Threading constraints**
  - Upstream blockers: none
  - Downstream blocked seams: `SEAM-2`, `SEAM-7` (consume `C-01` via `THR-01`)
  - Contracts produced: `C-01`
  - Contracts consumed: none

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Why this seam needs an explicit exit gate**: `C-01` is the root trust boundary for the whole pack; downstream seams must have an explicit, closeout-backed signal that the repo surface and support messaging are now publishable truth.
- **Expected contracts to publish**: `C-01`
- **Expected threads to publish / advance**: `THR-01` (publish); downstream `THR-07` (conformance) becomes unblocked only after `THR-01` is published.
- **Likely downstream stale triggers**:
  - Any change to supported-vs-legacy wording or archive/runtime boundary policy after landing.
- **Expected closeout evidence**:
  - root doc changes (README + plan anchors) demonstrating one supported Rust-first story
  - explicit legacy-freeze posture in harness entrypoints (messages + doc links)
  - recorded archive/runtime boundary rules and any automated guardrails added

## Slice index

- `S00` -> `slice-00-c-01-approved-repo-surface-contract.md`
- `S1` -> `slice-1-approved-surface-docs-and-support-story.md`
- `S2` -> `slice-2-legacy-harness-freeze-and-labeling.md`
- `S3` -> `slice-3-archive-runtime-boundary-guardrails.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-1-closeout.md`

