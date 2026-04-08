---
slice_id: S2
seam_id: SEAM-2
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any rename of supported verbs or change to help ordering requires downstream revalidation of `C-02`.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-02
contracts_produced: []
contracts_consumed:
  - C-01
  - C-02
open_remediations: []
---

### S2 - Implement the CLI Verb Skeleton and Setup-First Help Posture

- **User/system value**: The CLI exposes the reduced v1 verb surface (`setup`, `generate`, `inspect`, `doctor`) with a help posture that matches the trust pipeline and prevents downstream seams from inventing their own command UX.
- **Scope (in/out)**:
  - In:
    - implement command parsing + verb dispatch skeleton (no downstream behavior semantics)
    - help text structure and ordering (setup-first)
    - stable error shape for “not yet implemented” that does not misrepresent capability
  - Out:
    - implementing manifest ingest, resolver, rendering, or execution demo behavior
- **Acceptance criteria**:
  - `--help` (or equivalent) lists the supported verbs and descriptions without implying downstream seams have landed.
  - `setup`, `generate`, `inspect`, `doctor` exist and route to placeholders with honest copy about what is missing.
  - CLI does not shell out to or wrap the legacy harness as a supported runtime path (must satisfy `C-01` constraints).
- **Dependencies**:
  - `S00` contract rules for verb names and help posture (`C-02`).
  - Future `SEAM-4` will consume this verb surface for real behavior; keep interfaces stable.
- **Verification**:
  - Smoke: running each verb prints consistent placeholder output and exit codes.
  - Help posture: ordering and “setup-first” framing matches the contract and review surfaces.
- **Rollout/safety**:
  - Prefer “explicitly unimplemented” over accidental partial behavior.
- **Review surface refs**: `../../review_surfaces.md` (R1 operator workflow)

#### S2.T1 - Implement verb parsing + dispatch with stable command names

- **Outcome**: commands exist with stable names and help entries; downstream seams can rely on the dispatch structure.
- **Thread/contract refs**: `THR-02`, `C-02`

#### S2.T2 - Write help copy that avoids capability overclaim

- **Outcome**: help text states what exists vs what is planned without claiming resolver/render/demo behavior exists early.
- **Thread/contract refs**: `THR-02`, `C-02`, `C-01`
