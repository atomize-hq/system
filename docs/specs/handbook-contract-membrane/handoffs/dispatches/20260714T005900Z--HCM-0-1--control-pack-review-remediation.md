---
dispatch_id: 20260714T005900Z--HCM-0-1--control-pack-review-remediation
source_handoff_id: 20260714T005400Z--HCM-0-1--review--control-pack-actionable-findings
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

# Dispatch: Remediate HCM-0.1 Independent-Review Findings

## Mission

Resolve exactly `HCM-R-F-001` through `HCM-R-F-006` from the source review handoff through the smallest cohesive docs/control-infrastructure repair. Do not implement Rust, modify tests or core assets, select shipped artifact defaults, start a later Phase 0 slice, or broaden into unrelated control-pack cleanup.

This documentation session does not close HCM-0.1. It must leave a durable remediation handoff for a different fresh independent reviewer.

## Active Context Resolution

- Level: coordination
- Scope horizon: HCM-0.1 control pack
- Detail resolution: architecture_and_protocol_repair
- Temporal horizon: phase_0
- Authority horizon: docs_and_context_infrastructure
- Memory horizon: program_record
- Validation horizon: independent_review_finding_remediation

## Snapshot Grounding

- Prior/session-end snapshot ref: null
- Current/session-start snapshot ref: null
- Deterministic delta ref: null
- Resolution-aware grounding projection ref: null
- Included state families: source review findings; exact affected control-pack sections; handoff schemas/templates/protocol; current git diff and immutable orchestration/review records
- Explicitly omitted state families: secrets; unrestricted environment/command output; unrelated source/tests/assets; shipped-default candidates; later program queues
- Drift signals and durable justification refs: the current dirty state consists only of HCM-0.1 dispatch/handoff/ledger artifacts created by this orchestration/review loop; semantic runtime truth remains unchanged at `a3babd20329027afacdcee9d8b7b9d638d15af5b`
- Capture consistency: not_available

Snapshot Memory is not implemented. Keep all snapshot refs null and do not synthesize evidence.

## Orchestration Decision

- Classification: `cross_document_repair`
- Source handoff: `20260714T005400Z--HCM-0-1--review--control-pack-actionable-findings`
- Handoff validity: valid and current for the six review findings
- Active and target Resolution: coordination-level HCM-0.1 control-pack architecture/protocol; no broader product authority is required
- Phase 2/3 escalation decision: adopt the review's recommended first option. Phase 2 may use fixed deterministic, non-Resolution, first-party review renderers only. Generic configured custom-kind projections and all Resolution-aware views remain Phase 3 work after the Context Resolution kernel and deterministic projection engine. Do not invent an earlier precursor engine.
- Pack/spec/task changes required before re-review: the exact files below; no slice-local packet exists or is needed
- Single proof change this remediation may earn: resolve the six review findings and become ready for a different fresh independent HCM-0.1 review. It cannot itself satisfy the control-pack proof gate.

## Authority Order

1. This dispatch and the source review handoff for remediation scope.
2. The exact HCM-0.1 control-pack sections below.
3. Existing immutable handoff records for compatibility constraints.
4. Live runtime code only as read-only repo truth; no source changes are authorized.

## Must Read

- `AGENTS.md`
- `docs/specs/handbook-contract-membrane/handoffs/records/20260714T005400Z--HCM-0-1--review--control-pack-actionable-findings.json`: findings, escalation, pack_updates, next_session
- `docs/specs/handbook-contract-membrane/02-semantic-model.md`: Custom artifacts; Projection semantics
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`: Phases 2 and 3
- `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`: Schema policy; Artifact instance descriptor contract; Projection request/result; Snapshot capture policy
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`: Open program proof gates; Control-pack proof gate
- `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`: Storage model; Snapshot, handoff, and dispatch roles; Session closeout procedure; Ledger entry creation; Failure rules
- `docs/specs/handbook-contract-membrane/handoffs/handoff-record.schema.json`
- `docs/specs/handbook-contract-membrane/handoffs/ledger-entry.schema.json`
- `docs/specs/handbook-contract-membrane/handoffs/handoff-template.json`
- `docs/specs/handbook-contract-membrane/handoffs/dispatch-template.md`
- All immutable records in `docs/specs/handbook-contract-membrane/handoffs/records/` only for schema compatibility validation

## Current Repo-Truth Statement

The pack correctly treats artifact-kind/intake/posture/Snapshot semantics as target-only, keeps shipped defaults unresolved for HCM-0.6, and authorizes no Rust. The fresh review found six documentation/protocol inconsistencies: review-request-only closure wording; Phase 2/3 projection sequencing ambiguity; syntax-only handoff validation; unenforced new-record refs; missing snapshot window cursors; and missing ArtifactInstanceDescriptor schema identity/version.

## Artifact / Intake / Posture Boundary

- Artifact-kind definition refs: no runtime records; target contract only
- Artifact-instance/profile refs: no runtime records; add only descriptor schema identity/version to the target example
- Intake definition/candidate/canonical refs: no runtime records; do not change authority semantics
- Posture kernel/recommendation/transition refs: no runtime records; do not change authority semantics
- Shipped-default decision status: unresolved and reserved for HCM-0.6

## Allowed Scope

- `docs/specs/handbook-contract-membrane/02-semantic-model.md`
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`
- `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
- `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`
- `docs/specs/handbook-contract-membrane/handoffs/handoff-record.schema.json` only as retained v1.0 compatibility truth; do not invalidate existing immutable records
- One new versioned handoff-record schema if required to enforce new-record refs
- `docs/specs/handbook-contract-membrane/handoffs/handoff-template.json`
- `docs/specs/handbook-contract-membrane/handoffs/dispatch-template.md`
- One small docs-scoped Draft 2020-12 validation utility or an equivalently executable repository-owned validation command documented in `08`
- One immutable remediation handoff record and one matching ledger append

## Explicit Non-Goals

- Rust, tests, core assets, installed skills, or runtime behavior.
- Selecting or implying shipped artifact defaults.
- General schema redesign beyond the new-handoff compatibility strategy.
- Any projection runtime or precursor projection engine.
- HCM-0.2, HCM-0.3, HCM-0.6, HCM-0.7, or implementation packet work.
- Rewriting immutable records or prior dispatches.
- Unrelated cleanup.

## Tasks and Deliverables

1. Repair the HCM-0.1 close gate so it requires a completed independent review with no unresolved actionable findings; require separate remediation and a different later fresh review after findings.
2. Resolve Phase 2/3 projection sequencing consistently: limit Phase 2 to fixed deterministic non-Resolution first-party review rendering; defer generic configured custom-kind projections and Resolution-aware views/proof to Phase 3 after HCM-3.2/HCM-3.3. Align semantic, phase, contract, and proof wording without weakening custom-kind registration/intake proof.
3. Add stable `schema_id` and `schema_version` fields to the ArtifactInstanceDescriptor contract example and gates as needed.
4. Add explicit cursor fields and semantics to every bounded SnapshotCapturePolicy history window so the example satisfies its gate.
5. Replace syntax-only closeout validation with actual Draft 2020-12 validation of both handoff records and ledger entries. Keep `jq empty` only as an optional syntax precheck. Provide a deterministic repository-owned command/utility with clear nonzero failure behavior.
6. Preserve the existing v1.0 handoff schema for immutable historical records and introduce an explicit versioned new-record strategy that requires both `snapshot_refs` and `semantic_refs`. Update the new-record template, protocol, dispatch instructions, and validator routing consistently. Do not rewrite older records.
7. Run schema validation across every historical record, the updated template, and all ledger entries; rebuild the ledger in-memory and compare it byte-for-byte; run bounded terminology/phase assertions, link checks, portability scan, archive-boundary check, docs-only changed-path assertion, and `git diff --check`.
8. Write the mandatory immutable remediation handoff. It must supersede `20260714T005900Z--HCM-0-1--orchestration--dispatch-review-remediation`, name all six findings as resolved or explicitly remaining, and request orchestration of a different fresh reviewer. Append exactly one matching ledger entry.

## Contracts and Proof Gates

- Existing immutable v1.0 records must remain schema-valid.
- Every newly created handoff must validate against the new versioned schema and require `snapshot_refs` plus `semantic_refs`.
- Ledger entries must validate against `ledger-entry.schema.json` and resolve uniquely to their records.
- HCM-0.1 stays open until a different fresh reviewer returns no actionable findings.
- All runtime/open program gates remain open; no seam classification changes.

## Stop and Escalate When

- A repair would require modifying an immutable historical record.
- A new schema strategy would invalidate existing records instead of validating versions explicitly.
- Projection sequencing would require implementing or authorizing a precursor engine.
- Any change would select shipped defaults or require Rust/runtime work.
- A finding cannot be resolved inside the exact docs/control-infrastructure paths above.

## Mandatory Closeout

Before responding in chat:

1. write one immutable remediation handoff under `docs/specs/handbook-contract-membrane/handoffs/records/` using the new schema version if introduced;
2. set Snapshot Memory refs honestly to `not_available` and semantic runtime refs to empty/null;
3. set `supersedes` to `20260714T005900Z--HCM-0-1--orchestration--dispatch-review-remediation`;
4. append one unique matching ledger entry;
5. run the new actual Draft 2020-12 validation command across the new handoff and full ledger/history;
6. return only status, handoff path, short summary, next action, and one `jq` read command.
