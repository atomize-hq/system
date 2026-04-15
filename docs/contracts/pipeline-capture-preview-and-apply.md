---
contract_id: C-12
seam_id: SEAM-4
owner_seam: SEAM-4
version: m3-v1
currentness: current
status: drafted
revalidation_triggers:
  - Any change to the supported `pipeline capture` targets, parser rules, or the rule that capture is the only writer surface.
  - Any change to preview-cache identity, cache location, or cache-integrity validation.
  - Any change to repo-file mirror derivation, transactional apply behavior, rollback guarantees, or post-capture state mutation semantics.
  - Any change to the refusal taxonomy or recovery guidance for stale route basis, inactive stages, malformed capture input, or tampered capture cache.
---

# C-12 Pipeline Capture Preview And Apply Contract

## Purpose

`C-12` defines the bounded M3 writer surface for stage-output capture.

It exists so downstream CLI, proof, and docs work can treat one compiler-owned contract as authoritative for:

- the supported `pipeline capture` targets
- how preview and apply share one typed capture plan
- how stdin is parsed for single-file and multi-file stages
- how artifact outputs and repo-file mirrors are materialized
- how cached previews are fingerprinted, loaded, and validated
- which route-state updates capture may apply automatically

## Canonical location

- Canonical artifact: `docs/contracts/pipeline-capture-preview-and-apply.md`
- Direct consumer seam: `SEAM-4`

## Consumed contracts

`C-12` consumes:

- [`pipeline-route-and-state-core`](pipeline-route-and-state-core.md) for route-state locking, revision checks, and route-basis freshness
- [`stage-compile-boundary-and-route-freshness`](stage-compile-boundary-and-route-freshness.md) for compile-adjacent freshness and inactive-stage refusal posture

## Owned surface

`C-12` is authoritative for:

- the reduced-v1 `pipeline capture` preview/apply boundary
- the preview cache shape under `.system/state/pipeline/capture/`
- capture-input parsing rules
- repo-file mirror derivation
- transactional apply and rollback requirements
- post-capture automatic state updates and the required next-safe-action hint

`C-12` is not authoritative for:

- CLI wording or help snapshots
- proof renderer formatting
- any future `pipeline run` surface
- compile payload generation

## Normative rules

### Supported capture wedge

- `pipeline capture` is the only supported writer surface in M3.
- The supported bounded targets are:
  - `pipeline.foundation_inputs` + `stage.05_charter_synthesize`
  - `pipeline.foundation_inputs` + `stage.07_foundation_pack`
- Capture MUST reuse the same route-basis freshness and inactive-stage refusal posture used by the compiler-owned route/compile flow.
- Capture MUST refuse rather than silently re-running `pipeline resolve`.

### Shared preview/apply plan

- Preview and apply MUST share one typed compiler-owned capture plan.
- The plan MUST include:
  - the canonical capture target
  - the persisted `route_basis` snapshot used to build the plan
  - ordered artifact writes
  - ordered repo-file mirror writes
  - any automatic post-capture state updates
  - a deterministic `capture_id`
- Direct apply MAY build the plan in memory, but it MUST use the same plan shape and write/apply semantics as cached preview apply.

### Capture-input parsing

- Single-file stages MUST accept plain stdin body content only.
- Single-file stages MUST refuse `--- FILE: <path> ---` wrappers.
- Multi-file stages MUST require declared artifact `--- FILE: <path> ---` blocks exactly once each.
- Multi-file stages MUST refuse:
  - undeclared artifact blocks
  - duplicate declared blocks
  - missing declared blocks
  - non-empty content outside declared FILE blocks
- Capture normalization MUST use LF newlines and end persisted text outputs with exactly one trailing newline when content is non-empty.

### Output materialization

- Capture plans MUST be built from the declared `outputs.artifacts` and `outputs.repo_files` in stage front matter after variable substitution.
- Artifact outputs MUST remain repo-relative.
- Repo-file outputs MUST remain `${repo_root}`-anchored in the stage definition and MUST be materialized as repo-root-relative writes.
- Repo files are mirrors only in M3; stdin does not author repo files directly.
- For multi-artifact stages, repo-file mirror content MUST be derived from exactly one declared artifact output matched by basename.
- If zero or multiple artifact outputs match one repo-file basename, capture MUST refuse.

### Preview cache

- Preview cache entries MUST live under `.system/state/pipeline/capture/<capture-id>.yaml`.
- Cached previews MUST include an explicit schema version and the typed capture plan.
- `capture_id` MUST be deterministic from the plan contents.
- Cached apply MUST validate:
  - requested capture id format
  - cache file existence
  - cache schema version
  - cache-plan deterministic identity
- A malformed or mismatched cache entry MUST be treated as tampered cache, not best-effort input.

### Apply and rollback

- Apply MUST acquire the compiler-owned advisory lock for the pipeline route-state file before re-checking current state and before any write begins.
- Apply MUST re-check:
  - persisted route-state revision
  - persisted `route_basis`
  - route-basis freshness against current canonical truth
  - selected-stage active status
- Apply MUST write artifact outputs first, then repo-file mirrors, then persist automatic route-state updates last.
- Apply MUST be transactional at the acceptance boundary:
  - if a file write fails, earlier writes MUST roll back
  - if state persistence fails, file writes from this apply MUST roll back
- Cached preview entries MUST remain on refusal or failure.
- Cached preview entries SHOULD be deleted after successful cached apply.

### Automatic post-capture state updates

- Capture MAY automatically update only compiler-owned route-state fields required by the bounded M3 wedge.
- The reduced-v1 automatic updates are:
  - `refs.charter_ref` when `artifacts/charter/CHARTER.md` is written
  - `refs.project_context_ref` when `artifacts/project_context/PROJECT_CONTEXT.md` is written
  - `routing.charter_gaps_detected` derived from captured charter content using the existing marker heuristic
- Capture MUST NOT auto-set `needs_project_context`.
- When the selected stage declares `sets:` values that still require human judgment, capture MUST return the exact `pipeline state set` next-safe-action command the operator can run after apply.

## Verification checklist

- [ ] The compiler exposes preview, direct apply, cached apply, and cache-load APIs for `pipeline capture`.
- [ ] The preview cache path is `.system/state/pipeline/capture/<capture-id>.yaml`.
- [ ] Single-file capture refuses FILE wrappers.
- [ ] Multi-file capture refuses undeclared, duplicate, or missing declared blocks.
- [ ] Repo-file writes are mirrors only and derive from declared artifact outputs.
- [ ] Apply re-checks route-basis freshness under lock before writing.
- [ ] Apply rolls back file writes if later persistence fails.
- [ ] Automatic post-capture state updates are limited to the bounded M3 fields listed above.
