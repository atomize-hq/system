---
slice_id: S2
seam_id: SEAM-1
slice_kind: implementation
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any root-facing help/output that claims the legacy harness is supported, or that implies Rust owns setup flows that still rely on legacy behavior.
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

### S2 - Freeze and Label the Legacy Harness Entry Points

- **User/system value**: People running `./tools/harness.sh` cannot miss that it is legacy-only, transitional, and not the supported product path.
- **Scope (in/out)**:
  - In:
    - add explicit legacy-only messaging to `tools/harness.sh` and `tools/harness.py` help/output
    - add a stable pointer to the supported story and to `C-01`
    - define "allowed changes" policy for legacy harness (bugfixes only, no feature growth)
  - Out:
    - moving legacy code under `archived/` (planned later at cutover)
- **Acceptance criteria**:
  - Running `./tools/harness.sh --help` (or equivalent) prints a legacy-only banner and contract pointer.
  - README and tools output do not contradict each other.
  - A reviewer can point to an explicit policy boundary for what legacy harness changes are permitted.
- **Dependencies**:
  - Contract language from `C-01` (S00) to ensure identical wording across docs and tool output.
- **Verification**:
  - Manual run: help banner appears and is unambiguous on narrow terminals.
  - Grep check: legacy-only wording present in both README and tools output surfaces.
- **Rollout/safety**:
  - Do not rename commands or break existing legacy behavior in this seam; keep it runnable as reference until Rust parity.
- **Review surface refs**: `../../review_surfaces.md` (R2 runtime boundary)

#### S2.T1 - Add legacy-only banner + contract pointer to `tools/harness.sh`

- **Outcome**: Every entrypoint execution surfaces the legacy-only truth.
- **Inputs/outputs**:
  - Inputs: `tools/harness.sh`, `C-01` nouns/verbs
  - Outputs: banner + link/pointer to contract + plan
- **Thread/contract refs**: `THR-01`, `C-01`

#### S2.T2 - Add legacy-only banner + doc pointer to `tools/harness.py`

- **Outcome**: Python harness output cannot be mistaken for supported runtime.
- **Inputs/outputs**:
  - Inputs: `tools/harness.py`, `C-01` nouns/verbs
  - Outputs: help/output messaging changes only
- **Thread/contract refs**: `THR-01`, `C-01`

