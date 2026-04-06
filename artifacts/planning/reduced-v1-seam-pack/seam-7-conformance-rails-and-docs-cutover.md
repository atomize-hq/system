---
seam_id: SEAM-7
seam_slug: conformance-rails-and-docs-cutover
type: conformance
status: decomposed
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts:
    - SEAM-1
    - SEAM-2
    - SEAM-3
    - SEAM-4
    - SEAM-5
    - SEAM-6
  required_threads:
    - THR-01
    - THR-02
    - THR-03
    - THR-04
    - THR-05
    - THR-06
  stale_triggers:
    - Any upstream seam closeout changes repo boundary, command vocabulary, manifest truth, resolver results, output ordering, or demo/refusal semantics.
    - Any supported target or CI/install-smoke requirement changes.
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

# SEAM-7 - Conformance Rails and Docs Cutover

- **Goal / value**: Lock in the reduced v1 truth with tests, CI, install smoke, drift guards, help/docs parity, and final cutover messaging so the shipped repo tells one coherent story.
- **Scope**
  - In:
    - unit, integration, golden, fixture, CLI E2E, and drift tests
    - CI workflow for format, lint, test, and install smoke
    - README/help/docs alignment
    - cutover regression proving Python is not advertised as supported
    - final archive move only after upstream parity criteria are satisfied
  - Out:
    - defining upstream contracts that belong to earlier seams
    - adding public package-manager or release publishing
    - inventing new supported runtime behavior
- **Primary interfaces**
  - Inputs:
    - `C-01`
    - `C-02`
    - `C-03`
    - `C-04`
    - `C-05`
    - `C-06`
  - Outputs:
    - published conformance contract `C-07`
    - pack-level evidence that reduced v1 shipped coherently
- **Key invariants / rules**:
  - Tests and docs consume published upstream truth; they do not invent it.
  - Cutover messaging must match runtime behavior and help output.
  - Install smoke must cover `macOS arm64` and `Linux x86_64`.
  - Python is not presented as the supported runtime anywhere top-level.
- **Dependencies**
  - Direct blockers:
    - `SEAM-1`
    - `SEAM-2`
    - `SEAM-3`
    - `SEAM-4`
    - `SEAM-5`
    - `SEAM-6`
  - Transitive blockers:
    - none
  - Direct consumers:
    - none
  - Derived consumers:
    - future maintenance and release work
- **Touch surface**:
  - `tests/`
  - fixture and golden outputs
  - CI workflow config
  - `README.md`
  - docs index and command help examples
  - cutover/archive validation surfaces
- **Verification**:
  - Verification depends on accepted upstream contract evidence from every producing seam.
  - As a conformance seam, verification focuses on published contract alignment, drift prevention, and closeout readiness rather than producing a new upstream contract for other feature seams to consume.
- **Risks / unknowns**:
  - Risk: docs/help/tests encode different truths or let Python support language leak back in.
  - De-risk plan: bind conformance work directly to the published upstream contracts and stale triggers, with no free-floating cleanup bucket.
- **Rollout / safety**:
  - Defer the physical archive move until parity and cutover validation pass.
  - Prefer failing tests/docs checks over silently shipping inconsistent support messaging.
- **Downstream decomposition context**:
  - This seam is now `active` and decomposed; pre-exec review should revalidate that each conformance surface maps directly to published upstream contracts.
  - `THR-07` is the dominant closure thread.
  - First seam-local review should focus on whether each test/doc/help surface maps to a specific published contract and whether any stale trigger from upstream remains unresolved.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-07`
  - Threads likely to advance:
    - `THR-07`
  - Review-surface areas likely to shift after landing:
    - all pack-level review surfaces, especially `R1` and `R2`
  - Downstream seams most likely to require revalidation:
    - none inside this pack; future maintenance work must consume this closeout
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.
