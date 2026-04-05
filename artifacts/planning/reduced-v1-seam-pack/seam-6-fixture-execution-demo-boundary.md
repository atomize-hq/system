---
seam_id: SEAM-6
seam_slug: fixture-execution-demo-boundary
type: risk
status: decomposed
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts:
    - SEAM-4
  required_threads:
    - THR-04
  stale_triggers:
    - Any change to fixture lineage assumptions, unsupported live execution scope, or refusal wording.
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

# SEAM-6 - Fixture Execution Demo Boundary

- **Goal / value**: Prove the v1 execution demo can exist without misrepresenting the product as supporting live slice execution.
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
  - Inputs:
    - `C-04`
  - Outputs:
    - published demo-boundary contract `C-06`
    - refusal examples and demo evidence for downstream docs/tests
- **Key invariants / rules**:
  - Fixture execution packet demos are allowed.
  - Unsupported live slice execution requests must refuse explicitly.
  - The demo must not quietly introduce live slice lineage or runtime capabilities deferred by plan.
  - Help text and docs must call the demo “fixture-backed” everywhere.
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
  - fixture lineage definitions
  - execution demo path
  - refusal messaging and examples
- **Verification**:
  - Verification may depend on accepted upstream resolver evidence from `SEAM-4`.
  - This seam’s owned contract becomes concrete when the fixture-only boundary, refusal semantics, and operator-facing wording are precise enough for seam-local planning and implementation rather than requiring all docs and tests to be complete first.
- **Risks / unknowns**:
  - Risk: the demo is interpreted as a live capability.
  - De-risk plan: reserve review focus on refusal copy, scope labeling, and proof artifacts showing fixture lineage.
- **Rollout / safety**:
  - Default to refusal for anything that smells like live slice execution.
  - Keep the demo path narrow and easy to audit.
- **Downstream decomposition context**:
  - This seam is `future` because it depends on the landed resolver result and refusal model.
  - `THR-06` is the dominant downstream thread.
  - First seam-local review should focus on product-trust risk, unsupported-path wording, and evidence that the demo cannot be confused with live execution.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-06`
  - Threads likely to advance:
    - `THR-06`
  - Review-surface areas likely to shift after landing:
    - `R1` operator workflow and `R3` touch-surface map
  - Downstream seams most likely to require revalidation:
    - `SEAM-7`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.
