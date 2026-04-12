# Remediation Log - M1 Pipeline And Routing Spine

## Open remediations

```yaml
remediation_id: REM-001
origin_phase: pre_exec
source_gate: contract
related_seam: SEAM-1
related_slice: S00
related_thread: THR-01
related_contract: C-08
related_artifact: artifacts/planning/m1-pipeline-and-routing-spine/threaded-seams/seam-1-compiler-pipeline-core-and-routing-state/slice-00-c-08-pipeline-route-and-state-core-contract.md
severity: blocking
status: open
owner_seam: SEAM-1
blocked_targets:
  - seam: SEAM-1
    field: status
    value: landed
summary: The route/state baseline is now explicit; remaining work is to land the compiler route/state surfaces and publish the landing evidence promised by the owner slices.
required_fix: |
  Land the owner implementation and verification checklist anchored by the new canonical contract at `docs/contracts/pipeline-route-and-state-core.md`.

  Required implementation surfaces:
  - keep the declared-pipeline ingest behavior aligned with `crates/compiler/src/pipeline.rs`
  - land the compiler-owned resolved-route model and status semantics from `S2`
  - land the runtime-only state store, revision protocol, and mutation refusals from `S3`

  Required verification:
  - preserve loader coverage in `crates/compiler/tests/pipeline_loader.rs`
  - add `crates/compiler/tests/pipeline_route_resolution.rs`
  - add `crates/compiler/tests/pipeline_state_store.rs`

  Publication remains blocked until seam closeout can point at landed code, tests, and thread publication evidence.
resolution_evidence:
  - docs/contracts/pipeline-route-and-state-core.md
  - artifacts/planning/m1-pipeline-and-routing-spine/threaded-seams/seam-1-compiler-pipeline-core-and-routing-state/slice-00-c-08-pipeline-route-and-state-core-contract.md
  - artifacts/planning/m1-pipeline-and-routing-spine/threaded-seams/seam-1-compiler-pipeline-core-and-routing-state/slice-2-route-evaluation-and-status-reasons.md
  - artifacts/planning/m1-pipeline-and-routing-spine/threaded-seams/seam-1-compiler-pipeline-core-and-routing-state/slice-3-route-state-persistence-and-mutation-refusals.md
```

New remediation entries must use this schema:

```yaml
remediation_id: REM-001
origin_phase: pre_exec
source_gate: review
related_seam: SEAM-1
related_slice: null
related_thread: THR-01
related_contract: C-08
related_artifact: artifacts/planning/m1-pipeline-and-routing-spine/seam-1-compiler-pipeline-core-and-routing-state.md
severity: blocking
status: open
owner_seam: SEAM-1
blocked_targets:
  - seam: SEAM-1
    field: status
    value: exec-ready
summary: Pipeline route-state contract still leaves concurrency semantics ambiguous.
required_fix: Publish one explicit mutation protocol covering lock acquisition, revision conflict refusal, and atomic commit behavior.
resolution_evidence: []
```

Rules:

- Use canonical YAML blocks for remediation entries.
- Use seam ownership only. Do not emit `WS-*` owners.
- For `severity: blocking`, `blocked_targets` must not be empty.
- For `severity: material` or `follow_up`, use `blocked_targets: []` unless a concrete blocked transition also applies.

## Resolved remediations

- Move resolved items here using the same schema, set `status: resolved`, and populate `resolution_evidence`.
