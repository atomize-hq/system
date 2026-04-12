# Remediation Log - M1 Pipeline And Routing Spine

## Open remediations

None.

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
