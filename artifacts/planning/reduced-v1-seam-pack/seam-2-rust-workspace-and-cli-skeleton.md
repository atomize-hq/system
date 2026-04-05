---
seam_id: SEAM-2
seam_slug: rust-workspace-and-cli-skeleton
type: platform
status: proposed
execution_horizon: next
plan_version: v1
basis:
  currentness: provisional
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts:
    - SEAM-1
  required_threads:
    - THR-01
  stale_triggers:
    - Any change to the approved root surface or supported verb vocabulary after SEAM-1 lands.
    - Any change to the desired crate split or local-install target matrix in PLAN.md.
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

# SEAM-2 - Rust Workspace and CLI Skeleton

- **Goal / value**: Establish the Rust workspace, library/CLI crate split, and supported command hierarchy so downstream resolver work has one authoritative ownership map.
- **Scope**
  - In:
    - root `Cargo.toml`
    - `crates/compiler`
    - `crates/cli`
    - initial shared packet-result and decision-log types
    - CLI help and supported verb skeleton
  - Out:
    - canonical artifact manifest behavior
    - packet selection logic
    - renderer logic
    - execution demo implementation
- **Primary interfaces**
  - Inputs:
    - `C-01` from `SEAM-1`
    - reduced v1 command vocabulary from `PLAN.md`
  - Outputs:
    - published command/workspace contract `C-02`
    - stable crate ownership for downstream seams
- **Key invariants / rules**:
  - Rust CLI is the only supported packet-resolution authority.
  - Help text must present setup first, packet generation second, and repair third.
  - Workspace split must make compiler ownership obvious and avoid runtime coupling to Python.
  - This seam may need `S00` later if command-surface ownership or crate boundaries are still ambiguous at seam-local planning time.
- **Dependencies**
  - Direct blockers:
    - `SEAM-1`
  - Transitive blockers:
    - none
  - Direct consumers:
    - `SEAM-4`
    - `SEAM-5`
    - `SEAM-7`
  - Derived consumers:
    - `SEAM-6`
- **Touch surface**:
  - `Cargo.toml`
  - `Cargo.lock`
  - `crates/compiler/`
  - `crates/cli/`
  - CLI help and command parsing surfaces
- **Verification**:
  - Verification may depend on accepted upstream evidence that the repo boundary and command vocabulary from `SEAM-1` are trustworthy.
  - For this seam’s owned contract, the command and crate split must become concrete enough for downstream seam-local planning and implementation rather than requiring the final published help/docs package to exist already.
- **Risks / unknowns**:
  - Risk: the workspace skeleton publishes a command hierarchy that later seams cannot honor cleanly.
  - De-risk plan: review verb ownership, crate boundaries, and help posture before deeper implementation seams are decomposed.
- **Rollout / safety**:
  - Introduce the Rust workspace without advertising unsupported behavior.
  - Preserve narrow install targets and avoid premature packaging/distribution commitments.
- **Downstream decomposition context**:
  - This seam is `next` because it is the first seam after the repo-boundary contract lands and because every later implementation seam consumes its ownership map.
  - `THR-02` is the dominant thread, with `THR-01` as an upstream dependency.
  - First seam-local review should focus on crate ownership, command vocabulary drift, and whether setup/generate/inspect/doctor hierarchy remains explicit.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-02`
  - Threads likely to advance:
    - `THR-02`
  - Review-surface areas likely to shift after landing:
    - `R1` operator workflow and `R3` touch-surface map
  - Downstream seams most likely to require revalidation:
    - `SEAM-3`
    - `SEAM-4`
    - `SEAM-5`
    - `SEAM-7`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.

