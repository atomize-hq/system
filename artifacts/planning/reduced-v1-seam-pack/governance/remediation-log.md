# Remediation Log - Reduced V1 Rust-First CLI Cutover

## Open remediations

```yaml
remediation_id: REM-002
origin_phase: pre_exec
source_gate: contract
related_seam: SEAM-6
related_slice: S1
related_thread: THR-06
related_contract: C-06
related_artifact: artifacts/planning/reduced-v1-seam-pack/seam-6-fixture-execution-demo-boundary.md
severity: blocking
status: open
owner_seam: SEAM-6
blocked_targets:
  - seam: SEAM-6
    field: status
    value: exec-ready
summary: Demo invocation surface is not yet concrete enough for conformance.
required_fix: Choose and document the execution-demo request surface (CLI flag vs tooling vs test-only), then pin deterministic fixture lineage rules and explicit live-refusal wording in `C-06` and seam-local slices.
resolution_evidence: []
```

New remediation entries must use this schema:

```yaml
remediation_id: REM-001
origin_phase: pre_exec
source_gate: review
related_seam: SEAM-1
related_slice: null
related_thread: THR-01
related_contract: C-01
related_artifact: README.md
severity: blocking
status: open
owner_seam: SEAM-1
blocked_targets:
  - seam: SEAM-1
    field: status
    value: exec-ready
summary: Approved repo-surface contract still leaves Python support messaging ambiguous.
required_fix: Publish one Rust-first support story and remove conflicting top-level Python runtime claims.
resolution_evidence: []
```

Rules:

- Use canonical YAML blocks for remediation entries.
- Use seam ownership only. Do not emit `WS-*` owners.
- For `severity: blocking`, `blocked_targets` must not be empty.
- For `severity: material` or `follow_up`, use `blocked_targets: []` unless a concrete blocked transition also applies.

## Resolved remediations

None yet.
