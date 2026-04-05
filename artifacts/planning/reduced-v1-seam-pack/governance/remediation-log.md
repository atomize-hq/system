# Remediation Log - Reduced V1 Rust-First CLI Cutover

## Open remediations

None at extraction time.

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

