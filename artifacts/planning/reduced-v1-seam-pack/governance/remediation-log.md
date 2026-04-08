# Remediation Log - Reduced V1 Rust-First CLI Cutover

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

```yaml
remediation_id: REM-002
origin_phase: pre_exec
source_gate: contract
related_seam: SEAM-6
related_slice: S1
related_thread: THR-06
related_contract: C-06
related_artifact: artifacts/planning/reduced-v1-seam-pack/seam-6-fixture-execution-demo-boundary.md
severity: material
status: resolved
owner_seam: SEAM-6
blocked_targets: []
summary: Contract baseline for the fixture-backed execution demo boundary is now explicit; remaining work was implementation and conformance evidence.
required_fix: |
  Implement the `C-06` contract baseline by wiring the selected request surface (packet id + CLI flags), enforcing deterministic fixture lineage ordering, and surfacing explicit refusal copy for unsupported live slice execution requests.

  Evidence and decision baseline:
  - `docs/contracts/C-06-fixture-execution-demo-boundary.md`
  - `artifacts/planning/reduced-v1-seam-pack/threaded-seams/seam-6-fixture-execution-demo-boundary/slice-1-fixture-lineage-and-demo-request-surface.md`
  - `artifacts/planning/reduced-v1-seam-pack/threaded-seams/seam-6-fixture-execution-demo-boundary/slice-2-live-refusal-and-demo-proof-surfaces.md`
resolution_evidence:
  - docs/contracts/C-06-fixture-execution-demo-boundary.md
  - crates/cli/src/main.rs
  - crates/cli/tests/cli_surface.rs
  - crates/compiler/src/resolver.rs
  - crates/compiler/src/refusal.rs
  - crates/compiler/src/rendering/markdown.rs
  - crates/compiler/src/rendering/shared.rs
  - crates/compiler/tests/refusal_mapping.rs
  - tests/fixtures/execution_demo/basic/.system/charter/CHARTER.md
  - tests/fixtures/execution_demo/basic/.system/feature_spec/FEATURE_SPEC.md
  - artifacts/planning/reduced-v1-seam-pack/governance/seam-6-closeout.md
```
