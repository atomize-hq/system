---
seam_id: SEAM-1
seam_slug: approved-surface-and-legacy-freeze
type: platform
status: closed
execution_horizon: future
plan_version: v1
basis:
  currentness: current
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts: []
  required_threads: []
  stale_triggers:
    - Any change to the approved root surface, archive timing, or supported-runtime wording in PLAN.md or root docs.
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

# SEAM-1 - Approved Surface and Legacy Freeze

- **Goal / value**: Make the repo boundary trustworthy by clearly publishing that Rust is the supported product path, Python is frozen reference material only, and `archived/` is not a runtime dependency.
- **Scope**
  - In:
    - root README/help/doc relabeling
    - approved-surface and archive/runtime boundary rules
    - freeze posture for Python harness and wrappers
    - cutover prerequisites for later physical archive move
  - Out:
    - Rust compiler or CLI implementation logic
    - manifest, resolver, renderer, or execution-demo behavior
    - final physical archive move before Rust planning packet parity exists
- **Primary interfaces**
  - Inputs:
    - `PLAN.md`
    - current root docs and legacy harness references
  - Outputs:
    - published repo-surface contract `C-01`
    - unambiguous support messaging and archive rules consumed by downstream seams
- **Key invariants / rules**:
  - The repo root is approved surface only.
  - Nothing in `archived/` is imported, executed, or wrapped by the supported runtime path.
  - Python remains runnable only as reference behavior until Rust parity exists.
  - Docs must not blur “supported workflow” with “already reimplemented in Rust.”
- **Dependencies**
  - Direct blockers:
    - none
  - Transitive blockers:
    - none
  - Direct consumers:
    - `SEAM-2`
    - `SEAM-7`
  - Derived consumers:
    - `SEAM-3`
    - `SEAM-4`
    - `SEAM-5`
    - `SEAM-6`
- **Touch surface**:
  - `README.md`
  - `PLAN.md`
  - root docs index and legacy harness docs
  - `tools/harness.py`
  - `tools/harness.sh`
  - future `archived/python-harness/`
- **Verification**:
  - Review should prove that a new contributor can identify the supported Rust-first path in under 30 seconds.
  - Verification depends on the repo-surface contract becoming concrete enough that seam-local planning and later cutover work can trust it without needing the final archive move to have landed yet.
- **Risks / unknowns**:
  - Risk: support messaging stays ambiguous even after partial relabeling.
  - De-risk plan: use one canonical Rust-first story across README, help text, plan links, and archive/runtime boundary copy.
- **Rollout / safety**:
  - Keep Python runnable as reference material until Rust planning packet parity exists.
  - Do not move legacy runtime files into `archived/` prematurely if it would remove reference behavior needed during migration.
- **Downstream decomposition context**:
- This seam was `active` because every later seam inherits its support and runtime-boundary truth.
  - `THR-01` is the primary control thread.
  - First seam-local review should focus on wording drift, root-surface enforcement, and whether any supported-path examples still imply Python ownership.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-01`
  - Threads likely to advance:
    - `THR-01`
  - Review-surface areas likely to shift after landing:
    - `R2` runtime boundary and `R3` touch-surface map
  - Downstream seams most likely to require revalidation:
    - `SEAM-2`
    - `SEAM-7`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.
