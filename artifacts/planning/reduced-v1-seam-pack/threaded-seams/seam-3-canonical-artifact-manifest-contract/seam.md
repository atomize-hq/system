---
seam_id: SEAM-3
seam_slug: canonical-artifact-manifest-contract
status: landed
execution_horizon: future
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-3-canonical-artifact-manifest-contract.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts:
    - SEAM-2
  required_threads:
    - THR-02
  stale_triggers:
    - Any change to supported direct packet inputs, inherited posture dependencies, or manifest-version rules.
    - Any change to `.system/` as the canonical project-truth location.
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

# SEAM-3 - Canonical Artifact Manifest Contract

## Seam Brief (Restated)

- **Goal / value**: Define a concrete, typed manifest and freshness contract for canonical `.system/` artifacts so the resolver (`SEAM-4`) can trust what is in scope, what is inherited, and what makes a request stale.
- **Type**: integration
- **Scope**
  - In:
    - typed ingest for `.system/charter/CHARTER.md`, optional `.system/project_context/PROJECT_CONTEXT.md`, `.system/feature_spec/FEATURE_SPEC.md`
    - inherited posture dependency handling (without turning dependencies into mandatory packet-body inputs)
    - override-with-rationale rules for lower-level artifacts
    - request-scoped derived manifest shape (in-memory by default)
    - deterministic freshness fields and schema/manifest versioning
  - Out:
    - planning packet selection logic and refusal taxonomy (`SEAM-4`)
    - markdown/JSON/inspect rendering (`SEAM-5`)
    - execution-demo behavior (`SEAM-6`)
- **Touch surface**:
  - manifest and ingest types (Rust)
  - `.system/` path and canonical-truth rules
  - schema/versioning definitions
  - freshness + override-with-rationale semantics
- **Verification**:
  - For the owned contract (`C-03`), the manifest, versioning, freshness, and override rules are concrete enough (rules + verification checklist) that downstream seams can implement without guessing.
  - Do not require the final accepted contract artifact or downstream resolver behavior to exist as a pre-exec input; reserve publication/acceptance evidence for `S99` and closeout.
- **Basis posture**:
  - Currentness: current (revalidated against `SEAM-2` closeout and published `C-02` / `THR-02`)
  - Upstream closeouts assumed: `SEAM-2`
  - Required threads: `THR-02` (consume the `C-02` ownership + CLI contract)
  - Stale triggers:
    - Any change to supported direct packet inputs, inherited posture dependencies, or manifest-version rules.
    - Any change to `.system/` as the canonical project-truth location.
- **Threading constraints**
  - Upstream blockers: `SEAM-2` (consume `C-02` via `THR-02`); derived boundary constraints from `C-01` must remain consistent with “canonical truth lives in repo-local `.system/`”.
  - Downstream blocked seams: `SEAM-4`, `SEAM-7` (consume `C-03` via `THR-03`); derived consumers: `SEAM-5`, `SEAM-6`
  - Contracts produced: `C-03`
  - Contracts consumed: `C-01`, `C-02`

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Why this seam needs an explicit exit gate**: `C-03` is the trust boundary between canonical project truth and resolver behavior; downstream promotion must have a closeout-backed signal that the manifest and freshness semantics are now publishable truth.
- **Expected contracts to publish**: `C-03`
- **Expected threads to publish / advance**: `THR-03` (publish); downstream `THR-07` (conformance) consumes the published `C-03` alongside other contracts later.
- **Likely downstream stale triggers**:
  - any new direct packet input or refusal source (expands manifest scope)
  - any change to `.system/` canonical path rules
  - any change to inherited posture dependency semantics
  - any change to override-with-rationale requirements
  - any change to schema/manifest versioning or freshness-field computation
- **Expected closeout evidence**:
  - concrete `C-03` contract artifact (stable repo-relative location + normative rules + verification checklist)
  - landed Rust types/APIs for canonical artifact ingest and manifest generation (in `crates/compiler`)
  - recorded downstream revalidation requirements for `SEAM-4` through `SEAM-7` if contract deltas occurred

## Slice index

- `S00` -> `slice-00-c-03-canonical-artifact-manifest-contract.md`
- `S1` -> `slice-1-canonical-artifact-ingest-and-discovery.md`
- `S2` -> `slice-2-freshness-and-override-with-rationale.md`
- `S3` -> `slice-3-manifest-generation-interface-for-resolver.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-3-closeout.md`
