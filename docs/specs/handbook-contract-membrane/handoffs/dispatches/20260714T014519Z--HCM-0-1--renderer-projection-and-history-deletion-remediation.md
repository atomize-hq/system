---
dispatch_id: 20260714T014519Z--HCM-0-1--renderer-projection-and-history-deletion-remediation
source_handoff_id: 20260714T013942Z--HCM-0-1--review--final-control-pack-findings
orchestration_decision: cross_document_repair
session_kind: documentation
phase_id: HCM-0
slice_id: HCM-0.1
packet_id: null
snapshot_ref: null
delta_ref: null
snapshot_projection_ref: null
status: ready
---

# Dispatch: Simplify Renderer/Projection Boundary and Reject History Deletion

## Mission

Resolve exactly `HCM-R3-F-001` and `HCM-R3-F-002`. Use the smallest coherent docs/control-infrastructure repair. Do not add a precursor projection engine, alter schemas/templates/immutable records, implement Rust, select defaults, or widen into later slices. This remediation cannot close HCM-0.1; another different fresh reviewer is mandatory.

## Active Context Resolution

- Level: coordination
- Scope horizon: HCM-0.1 exact cross-document and validator repair
- Detail resolution: renderer_projection_term_boundary_and_history_integrity
- Authority horizon: docs_and_context_infrastructure
- Validation horizon: two_finding_remediation_then_fresh_review
- Snapshot grounding: not_available; all refs null

## Orchestration Decision

- Classification: `cross_document_repair`
- Source: `20260714T013942Z--HCM-0-1--review--final-control-pack-findings`
- Both findings are valid/current.
- Simplification decision for F-001: define **renderer-derived view** as a fixed deterministic pre-Phase-3 human-review output with no Context Resolution input and outside the capitalized Phase-3 `Projection` request/result/provenance contract. Reserve **Projection** for the Phase-3 generic Resolution-aware engine. Align 01/02/04/05 to those exact terms; do not create a second provenance subset or precursor engine.
- Enforcement decision for F-002: the validator must require every exact admitted historical v1.0 filename/ID/SHA-256 entry to be present, and the self-test must prove deletion plus exact ledger rebuild fails.
- Single proof change: both findings remediated and ready for a different fresh review; no HCM-0.1 closure here.

## Must Read

- `AGENTS.md`
- `docs/specs/handbook-contract-membrane/handoffs/records/20260714T013942Z--HCM-0-1--review--final-control-pack-findings.json`
- `01-target-architecture.md`: Canonical and projected artifacts; Charter intake and project-posture kernel
- `02-semantic-model.md`: Custom artifacts; intake/canonical view language; Projection semantics; Projection provenance
- `04-phase-slice-map.md`: Phases 2 and 3
- `05-contracts-schemas-and-gates.md`: Charter intake and canonical contract; Artifact validation layers; Projection request/result
- `06-proof-and-regression-ledger.md`: Open gates; Control-pack proof gate
- `08-handoff-ledger-and-escalation-protocol.md`: Canonical truth; schema routing; closeout; failure rules
- `handoffs/validate_handoffs.py`: admission mapping, main record-set validation, self-test

## Allowed Scope

- `docs/specs/handbook-contract-membrane/01-target-architecture.md`
- `docs/specs/handbook-contract-membrane/02-semantic-model.md`
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`
- `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
- `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`
- `docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py`
- One v1.1 remediation handoff and ledger append

## Non-Goals

No schemas/templates/immutable records/dispatch changes. No Rust/tests/core assets/skills. No new runtime/Projection mechanism. No defaults/later slices/unrelated cleanup. No self-review or closure.

## Tasks

1. Use the fresh GitNexus impact for validator `main`: LOW, one file caller, zero processes/modules. Re-run only if target changes before edit.
2. Introduce one explicit terminology boundary in 01/02: lowercase renderer-derived views are fixed deterministic pre-Phase-3 human-review outputs; capitalized `Projection` is the Phase-3 generic Resolution-aware contract. Remove universal statements that accidentally require Resolution provenance for renderer-derived views.
3. Align Phase 2 rows/gates in 04 to say renderer-derived Markdown/human-review view, not Projection. Keep all generic configured custom-kind and Resolution-aware Projection execution/proof in HCM-3.3.
4. Align 05 Charter/validation wording and mark Projection request/result as Phase-3-only, excluding fixed renderer-derived views.
5. In validator normal mode, compare the exact set of present v1.0 canonical filenames against the seven-entry admission mapping and fail on missing admitted history as well as extra/modified records.
6. Extend `--self-test-v1-admission` so unknown, byte-modified, and deleted admitted v1.0 history all fail after exact ledger rebuild. Update output text.
7. Align 06/08 only as needed to require deletion refusal and the renderer/Projection boundary proof.
8. Run normal validator, extended self-test, bounded terminology assertions, links/portability/archive, docs-only paths, `git diff --check`, and `npx gitnexus detect-changes -r handbook`.
9. Write one v1.1 remediation handoff superseding `20260714T014519Z--HCM-0-1--orchestration--dispatch-final-findings-remediation`, append one ledger entry, rerun proof, and leave HCM-0.1 `review_required` for another different fresh reviewer.

## Stop Conditions

Stop rather than broaden if either repair would require immutable-history mutation, schemas/templates, a precursor engine, Rust/runtime, defaults, or later-slice authority.
