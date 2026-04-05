---
slice_id: S1
seam_id: SEAM-1
slice_kind: implementation
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to supported-vs-legacy wording or archive/runtime boundary policy after landing.
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

### S1 - Land the Approved-Surface Docs and One Supported Story

- **User/system value**: The root surface communicates one stable truth: Rust-first is the supported product path; the legacy harness is explicitly transitional and reference-only; `archived/` is never a runtime dependency.
- **Scope (in/out)**:
  - In:
    - restructure README and root docs so the supported-vs-legacy split is unmissable
    - ensure links and terminology consistently match `C-01`
    - ensure no docs imply Rust CLI exists before `SEAM-2` lands
  - Out:
    - changing actual runtime behavior (owned by later seams)
- **Acceptance criteria**:
  - README first screen contains: supported story, what is legacy-only, and link to the contract doc (`C-01`).
  - Legacy quick start remains available but is explicitly labeled legacy-only and framed as transitional.
  - Root docs index (if present) points to the same supported-vs-legacy language and contract reference.
- **Dependencies**:
  - `S00` contract rules for `C-01` (must exist to avoid copy drift).
- **Verification**:
  - "30 second rule": cold reader identifies supported path, legacy-only status, and archive boundary without scrolling past unrelated content.
  - Link check: README links to the contract doc and plan, and the wording matches contract nouns/verbs.
- **Rollout/safety**:
  - Avoid half-landing: do not split changes across multiple merges where intermediate state is ambiguous.
- **Review surface refs**: `../../review_surfaces.md` (R2, R3)

#### S1.T1 - Rewrite README top-level "Current Status" to align to `C-01`

- **Outcome**: Supported-vs-legacy copy matches the contract exactly, with explicit "available today vs supported direction" phrasing.
- **Inputs/outputs**:
  - Inputs: README, `C-01` rules
  - Outputs: README updates + contract links
- **Thread/contract refs**: `THR-01`, `C-01`

#### S1.T2 - Normalize root doc entrypoints to prevent drift

- **Outcome**: `PLAN.md` and any root docs index use the same canonical terms and point at `C-01` as the governing truth.
- **Inputs/outputs**:
  - Inputs: PLAN + docs index
  - Outputs: consistent wording and references
- **Thread/contract refs**: `THR-01`, `C-01`

