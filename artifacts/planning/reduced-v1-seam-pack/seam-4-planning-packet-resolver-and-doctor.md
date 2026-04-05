---
seam_id: SEAM-4
seam_slug: planning-packet-resolver-and-doctor
type: capability
status: proposed
execution_horizon: future
plan_version: v1
basis:
  currentness: provisional
  source_scope_ref: scope_brief.md
  source_scope_version: v1
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

# SEAM-4 - Planning Packet Resolver and Doctor

- **Goal / value**: Deliver deterministic packet selection, budget handling, refusal behavior, and `doctor` blocker diagnosis from one typed resolver truth.
- **Scope**
  - In:
    - manifest consumption
    - deterministic freshness checks
    - planning packet selection
    - typed budget policy
    - typed decision log
    - compact refusal behavior
    - `doctor` packet-readiness and blocker reporting
  - Out:
    - renderer-specific formatting details
    - fixture execution demo behavior
    - final cross-platform CI and docs parity work
- **Primary interfaces**
  - Inputs:
    - `C-02`
    - `C-03`
  - Outputs:
    - published resolver result contract `C-04`
    - blocker truth consumed by renderers and conformance rails
- **Key invariants / rules**:
  - Same inputs yield the same packet and the same decision log.
  - `generate` refusals stay compact: blocker summary, broken artifact/dependency, exact next action.
  - `doctor` aggregates blockers and safe next actions in one view.
  - Budget behavior is deterministic and inspectable.
- **Dependencies**
  - Direct blockers:
    - `SEAM-2`
    - `SEAM-3`
  - Transitive blockers:
    - `SEAM-1`
  - Direct consumers:
    - `SEAM-5`
    - `SEAM-7`
  - Derived consumers:
    - `SEAM-6`
- **Touch surface**:
  - compiler resolver core
  - decision-log types
  - freshness and refusal policy logic
  - `doctor` command behavior
- **Verification**:
  - Verification may depend on accepted upstream manifest and command-surface evidence from `SEAM-2` and `SEAM-3`.
  - For this seam’s owned contract, verification should prove the resolver result, refusal semantics, and `doctor` outputs are concrete enough for seam-local planning and implementation, not that every renderer or test fixture is already finalized.
- **Risks / unknowns**:
  - Risk: `generate` and `doctor` drift into separate truths.
  - De-risk plan: keep one typed result and one blocker model with proof surfaces layered on top.
- **Rollout / safety**:
  - Refuse unsafe or unsupported requests explicitly.
  - Avoid hidden negative caching so retry-after-repair is clean.
- **Downstream decomposition context**:
  - This seam is `future` because it requires landed command/workspace and manifest contracts.
  - `THR-04` is the dominant downstream thread; `THR-02` and `THR-03` are the preconditions.
  - First seam-local review should focus on decision-log completeness, refusal ordering, and parity between `generate` and `doctor`.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-04`
  - Threads likely to advance:
    - `THR-04`
  - Review-surface areas likely to shift after landing:
    - `R1` operator workflow and `R3` touch-surface map
  - Downstream seams most likely to require revalidation:
    - `SEAM-5`
    - `SEAM-6`
    - `SEAM-7`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.

