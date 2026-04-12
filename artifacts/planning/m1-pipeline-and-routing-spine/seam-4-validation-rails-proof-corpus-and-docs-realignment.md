---
seam_id: SEAM-4
seam_slug: validation-rails-proof-corpus-and-docs-realignment
type: conformance
status: proposed
execution_horizon: next
plan_version: v1
basis:
  currentness: provisional
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts: []
  required_threads:
    - THR-01
    - THR-02
    - THR-03
  stale_triggers:
    - Any change to `SEAM-1` route/state semantics requires revalidation of proof corpus, malformed-state rails, and shared golden outputs.
    - Any change to `SEAM-2` supported command surface or help posture requires revalidation of docs/help parity and help snapshots.
    - Any change to `SEAM-3` compile defer boundary requires revalidation of docs and proof surfaces that describe compile as future work.
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

# SEAM-4 - Validation Rails, Proof Corpus, and Docs Realignment

- **Goal / value**: Lock the shipped `M1` surface with realistic proof corpus, goldens, malformed-state and concurrency tests, help/docs parity, and explicit performance/security boundaries so the repo tells one coherent product story.
- **Scope**
  - In:
    - compiler and CLI tests for pipeline loading, activation, route truth, shorthand ambiguity, malformed state, lock/revision conflicts, and state mutation semantics
    - one shared foundation-family proof corpus and golden outputs for `pipeline resolve` and `pipeline state set`
    - help snapshots and help/docs parity checks for the shipped `pipeline` subset
    - docs realignment across root docs, CLI vocabulary/hierarchy docs, design docs, and contracts
    - explicit M1 performance boundary and security/operability rules
  - Out:
    - actual M2 compile implementation or compile proof outputs
    - broad architecture cleanup unrelated to the `pipeline` story
    - unsupported public release/distribution work
- **Primary interfaces**
  - Inputs:
    - `C-08`
    - `C-09`
    - `C-10`
    - existing root docs and current contract surfaces
  - Outputs:
    - published conformance contract `C-11`
    - shared proof corpus, goldens, and docs/help parity evidence for the supported M1 surface
- **Key invariants / rules**:
  - manual verification alone is insufficient; the `pipeline` family is public product contract once shipped
  - CLI and compiler tests must share one realistic proof corpus rather than duplicate fixture sets
  - malformed pipeline and malformed state refusals must stay explicit; auto-heal or silent ignore is invalid
  - docs and help must not present packet-only reduced-v1 wording as the active final contract once `pipeline` ships
  - M1 command-cost boundaries stay explicit: `list` and `show` are metadata-first; `resolve` is the first command allowed to load route-bearing state and stage metadata
- **Dependencies**
  - Direct blockers:
    - `SEAM-1`
    - `SEAM-2`
    - `SEAM-3`
  - Transitive blockers:
    - existing reduced-v1 docs and contract surfaces that currently describe a packet-first world
  - Direct consumers:
    - none inside this pack
  - Derived consumers:
    - future milestone packs
    - release validation and operator trust surfaces
- **Touch surface**:
  - `tests/`
  - shared proof fixtures and goldens
  - `README.md`
  - `DESIGN.md`
  - `docs/START_HERE.md`
  - `docs/CLI_PRODUCT_VOCABULARY.md`
  - `docs/CLI_COMMAND_HIERARCHY.md`
  - `docs/contracts/`
  - help snapshot tests and CI/conformance checks
- **Verification**:
  - Verification depends on accepted upstream contract evidence from `SEAM-1`, `SEAM-2`, and `SEAM-3`.
  - As a conformance seam, verification focuses on published contract alignment, drift prevention, proof-corpus realism, and closeout readiness rather than inventing new runtime behavior.
- **Canonical contract refs**:
  - `docs/contracts/pipeline-proof-corpus-and-docs-cutover.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
- **Risks / unknowns**:
  - Risk: proof fixtures look realistic enough to pass tests while still being too thin to prove operator value.
  - De-risk plan: treat archived realistic planning artifacts and substantial demo docs as the minimum acceptable proof basis.
  - Risk: docs/help parity lags the shipped command surface and creates a split product story.
  - De-risk plan: tie doc updates directly to the same conformance seam that owns help snapshots and goldens.
- **Rollout / safety**:
  - fail fast on drift in help text, proof outputs, refusal classes, or malformed-state behavior
  - keep performance expectations explicit and narrow rather than smuggling caches into M1
- **Downstream decomposition context**:
  - This seam is `next` because it can now plan against the published operator-surface truth while waiting on the active compile-boundary seam to publish `C-10`.
  - `THR-04` is the closeout thread that carries the shared proof and docs/help posture into later milestone packs.
  - The first seam-local review should focus on proof-corpus realism, whether every check maps to a published contract, and whether the docs/help cutover removes competing packet-only product claims.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-11`
  - Threads likely to advance:
    - `THR-04`
  - Review-surface areas likely to shift after landing:
    - `R1`
    - `R2`
    - `R3`
  - Downstream seams most likely to require revalidation:
    - future `M2`
    - future `M3`
    - future `M4`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.
