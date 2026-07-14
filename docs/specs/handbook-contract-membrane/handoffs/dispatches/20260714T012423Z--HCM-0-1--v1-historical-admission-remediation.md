---
dispatch_id: 20260714T012423Z--HCM-0-1--v1-historical-admission-remediation
source_handoff_id: 20260714T011730Z--HCM-0-1--review--v1-routing-finding
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

# Dispatch: Enforce Historical-Only Handoff v1.0 Admission

## Mission

Resolve only `HCM-R2-F-001`: make the repository-owned validator preserve the exact immutable historical v1.0 handoffs while rejecting every newly introduced or modified v1.0 record. Add deterministic negative proof and align only the tightly coupled protocol/proof wording. Do not alter schemas/templates, rewrite records/dispatches, modify Rust/tests/core assets/skills, select defaults, or widen into other HCM work.

This patch session cannot close HCM-0.1 and must leave the result to another different fresh reviewer.

## Active Context Resolution

- Level: coordination
- Scope horizon: HCM-0.1 handoff control infrastructure
- Detail resolution: validator admission enforcement
- Temporal horizon: phase_0
- Authority horizon: docs_and_context_infrastructure
- Memory horizon: program_record
- Validation horizon: historical_v1_admission_negative_proof

## Snapshot Grounding

- All snapshot/delta/projection refs: null
- Capture consistency: not_available
- Included state: source finding, current canonical record history, validator, v1.0/v1.1 routing protocol, HCM-0.1 proof gate
- Omitted state: runtime code, unrelated docs/diffs, secrets, shipped-default decisions, later slices

## Orchestration Decision

- Classification: `cross_document_repair`
- Source handoff: `20260714T011730Z--HCM-0-1--review--v1-routing-finding`
- Handoff status: valid/current; five earlier findings remain clean and one admission finding is actionable
- Active/target Resolution: unchanged HCM-0.1 coordination-level docs/control infrastructure
- Exact decision: use an explicit immutable historical v1.0 admission set keyed by canonical record filename/handoff ID and exact SHA-256 file bytes (or an equally strict non-spoofable content fingerprint). Unknown v1.0 IDs and byte-modified admitted records must fail. New v1.1 records continue normal schema routing.
- Single proof change: HCM-R2-F-001 resolved and ready for another different fresh review; HCM-0.1 remains open in this session

## Authority Order

1. This dispatch and `20260714T011730Z--HCM-0-1--review--v1-routing-finding`.
2. `08` version-routing/closeout rules and `06` HCM-0.1 proof gate.
3. Current immutable canonical record bytes.
4. GitNexus impact evidence for the existing validator `main` symbol.

## Must Read

- `AGENTS.md`
- `docs/specs/handbook-contract-membrane/handoffs/records/20260714T011730Z--HCM-0-1--review--v1-routing-finding.json`
- `docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py`
- `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`: Handoff schema-version routing; closeout; failure rules
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`: Control-pack proof gate
- `docs/specs/handbook-contract-membrane/handoffs/handoff-record.schema.json`
- `docs/specs/handbook-contract-membrane/handoffs/handoff-record.v1.1.schema.json`
- Every current canonical record only to compute/verify the historical v1.0 admission fingerprints

## Current Repo-Truth Statement

The current validator chooses v1.0 or v1.1 solely from `schema_version` and therefore admits any new schema-valid v1.0 record. Exactly seven current canonical v1.0 records predate the v1.1 cutover. The three current v1.1 records and all later records must remain v1.1. GitNexus reports LOW upstream impact for validator `main`: one file-level caller, zero processes, zero modules.

## Allowed Scope

- `docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py`
- `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
- One immutable v1.1 remediation handoff and one ledger append

## Explicit Non-Goals

- No schema/template changes.
- No mutation/deletion of any existing record, dispatch, or historical ledger entry.
- No Rust/tests/core assets/skills/runtime changes.
- No unrelated validator refactor.
- No shipped-default decision or later slice work.
- No HCM-0.1 self-review/closure.

## Tasks and Deliverables

1. Before modifying validator `main`, use the already-recorded GitNexus upstream impact: LOW risk, one direct file caller, zero processes/modules. Re-run only if the indexed target changed.
2. Add an explicit immutable historical v1.0 admission mapping covering exactly the seven current v1.0 records and exact SHA-256 record bytes. Reject unknown v1.0 filenames/IDs, mismatched filename/handoff ID, and changed bytes before/alongside schema validation. Continue to reject unsupported versions.
3. Add deterministic negative proof, preferably a validator self-test or bounded temp-copy test, demonstrating that a new otherwise-schema-valid v1.0 record plus exact ledger rebuild fails. Also prove a byte-modified admitted v1.0 record fails.
4. Keep normal full validation green for current history/template/ledger and all current v1.1 records.
5. Align `08` to describe the exact ID/content-fingerprint admission boundary and fail-closed behavior. Align the HCM-0.1 proof gate in `06` to require refusal of unknown or modified v1.0 records.
6. Run the validator, its negative admission proof, `git diff --check`, docs-only path assertion, relative links/portability/archive checks applicable to changed docs, and `npx gitnexus detect-changes` (no commit is requested).
7. Write one immutable v1.1 remediation handoff superseding `20260714T012423Z--HCM-0-1--orchestration--dispatch-v1-admission-remediation`; record HCM-R2-F-001 resolved, HCM-0.1 still review_required, and request another different fresh reviewer. Append one ledger entry and rerun full validation.

## Stop and Escalate When

- Any existing immutable record would need modification.
- Enforcement cannot distinguish the exact historical set from a new v1.0 record without a mutable/forgeable date-only rule.
- Scope would broaden beyond validator admission plus coupled protocol/proof wording.
- Runtime or broader product authority is required.

## Mandatory Closeout

Write one v1.1 remediation handoff, append one unique ledger entry, run the repository validator and negative admission proof, set `supersedes` to `20260714T012423Z--HCM-0-1--orchestration--dispatch-v1-admission-remediation`, and return only durable status/path/next/jq lines.
