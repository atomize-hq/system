---
seam_id: SEAM-3
seam_slug: canonical-artifact-manifest-contract
type: integration
status: proposed
execution_horizon: future
plan_version: v1
basis:
  currentness: provisional
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts:
    - SEAM-2
  required_threads:
    - THR-02
  stale_triggers:
    - Any change to supported direct packet inputs, inherited posture dependencies, or manifest-version rules.
    - Any change to `.system/` as the canonical project-truth location.
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

# SEAM-3 - Canonical Artifact Manifest Contract

- **Goal / value**: Define the typed manifest and freshness contract for canonical `.system/` artifacts so the resolver can trust what is in scope, what is inherited, and what makes a packet stale.
- **Scope**
  - In:
    - typed ingest for `CHARTER`, optional `PROJECT_CONTEXT`, and `FEATURE_SPEC`
    - inherited posture dependency handling
    - override-with-rationale rules for lower-level artifacts
    - request-scoped derived manifest shape
    - deterministic freshness fields and schema versioning
  - Out:
    - packet selection logic itself
    - markdown/JSON/inspect rendering
    - execution-demo behavior
- **Primary interfaces**
  - Inputs:
    - `C-02`
    - managed-project `.system/` artifact rules from `PLAN.md`
  - Outputs:
    - published manifest/freshness contract `C-03`
    - explicit thread inputs for resolver and conformance seams
- **Key invariants / rules**:
  - Canonical project truth lives in repo-local `.system/`, not user-home state.
  - Direct packet inputs are limited to the approved v1 artifacts.
  - Inherited posture dependencies can affect freshness without becoming mandatory packet body inputs.
  - Metadata/schema expansion requires explicit triggers.
- **Dependencies**
  - Direct blockers:
    - `SEAM-2`
  - Transitive blockers:
    - `SEAM-1`
  - Direct consumers:
    - `SEAM-4`
    - `SEAM-7`
  - Derived consumers:
    - `SEAM-5`
    - `SEAM-6`
- **Touch surface**:
  - manifest and ingest types
  - `.system/` path rules
  - schema/versioning definitions
  - freshness/override rationale logic
- **Verification**:
  - Verification may depend on accepted upstream command and ownership evidence from `SEAM-2`.
  - As an owned contract seam, verification should prove the manifest, freshness, and override rules are concrete enough for seam-local planning and implementation rather than requiring a final accepted runtime artifact before the seam can proceed.
- **Risks / unknowns**:
  - Risk: metadata scope grows by vibes and erodes the reduced v1 wedge.
  - De-risk plan: enforce expansion triggers and keep live inputs, refusal sources, and provenance dependencies explicit.
- **Rollout / safety**:
  - Ignore unsupported artifacts explicitly rather than implicitly.
  - Keep manifest persistence request-scoped and in-memory by default.
- **Downstream decomposition context**:
  - This seam is `future` because its contract depends on the landed workspace and CLI shape before deeper planning is safe.
  - `THR-03` is the dominant thread and likely needs `S00` for contract definition when seam-local planning begins.
  - First seam-local review should focus on canonical-vs-derived truth, freshness semantics, and override-with-rationale precision.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-03`
  - Threads likely to advance:
    - `THR-03`
  - Review-surface areas likely to shift after landing:
    - `R1` operator workflow and `R2` runtime boundary
  - Downstream seams most likely to require revalidation:
    - `SEAM-4`
    - `SEAM-7`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.

