---
seam_id: SEAM-5
seam_slug: renderer-and-proof-surfaces
type: capability
status: proposed
execution_horizon: future
plan_version: v1
basis:
  currentness: provisional
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts:
    - SEAM-4
  required_threads:
    - THR-04
  stale_triggers:
    - Any change to resolver result fields, trust-header ordering, or inspect proof ordering.
    - Any change to JSON fallback requirements for dense evidence views.
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

# SEAM-5 - Renderer and Proof Surfaces

- **Goal / value**: Render markdown, JSON, and inspect outputs from one typed resolver result so operators can trust the packet, inspect evidence, and understand refusals without divergent logic.
- **Scope**
  - In:
    - markdown renderer
    - JSON renderer
    - inspect renderer
    - trust header and proof ordering rules
    - renderer failure isolation from successful resolver results
  - Out:
    - packet selection logic
    - canonical artifact ingest
    - execution-demo semantics
- **Primary interfaces**
  - Inputs:
    - `C-04`
  - Outputs:
    - published output contract `C-05`
    - proof-surface ordering consumed by tests and docs
- **Key invariants / rules**:
  - Renderers are pure views over one typed resolver result plus typed decision log.
  - Inspect output privileges evidence order over internal module order.
  - Successful packet generation is not destroyed by renderer-specific failures.
  - Narrow terminals still receive the first three facts in stable order: outcome, object, next action.
- **Dependencies**
  - Direct blockers:
    - `SEAM-4`
  - Transitive blockers:
    - `SEAM-2`
    - `SEAM-3`
  - Direct consumers:
    - `SEAM-7`
  - Derived consumers:
    - none
- **Touch surface**:
  - renderer modules
  - output formatting rules
  - inspect proof copy and machine-readable fallbacks
- **Verification**:
  - Verification may depend on accepted upstream resolver-result evidence from `SEAM-4`.
  - This seam’s owned contract becomes concrete when output ordering, trust headers, refusal layout, and inspect proof semantics are stable enough for seam-local planning and implementation, not only when every golden test is already landed.
- **Risks / unknowns**:
  - Risk: output surfaces drift into separate semantics or become too dense for narrow terminals.
  - De-risk plan: keep one typed source of truth and review the output ordering against the explicit CLI design language.
- **Rollout / safety**:
  - Prefer stable labeling over color or formatting tricks.
  - Defer dense detail to inspect/JSON rather than letting primary facts wrap into noise.
- **Downstream decomposition context**:
  - This seam is `future` because it depends on the published resolver result contract.
  - `THR-05` is the dominant downstream thread.
  - First seam-local review should focus on proof readability, stable refusal structure, and whether help/docs examples can mirror the same vocabulary.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-05`
  - Threads likely to advance:
    - `THR-05`
  - Review-surface areas likely to shift after landing:
    - `R1` operator workflow and `R3` touch-surface map
  - Downstream seams most likely to require revalidation:
    - `SEAM-7`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.

