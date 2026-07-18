# HCM-1.4 Planning Remediation 3 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_3`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T200810Z--HCM-1-4--fresh-planning-review-3.json`
- reviewed subject fingerprint:
  `sha256:6b37609552f50d14da062fa8782057d37b6b3679ceeb8c10135db7fd51375f79`
- review result: `CHANGES_REQUIRED`
- findings: two Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

Review 3 independently accepted all Review 1/2 remedies, replayed the exact
manifest/validators/29-member package proof/47+22 author baselines, and found
two remaining setup-contract contradictions. The parent accepted both without
widening implementation scope.

## Finding dispositions

### `HCM-1.4-PR3-R001` — unreachable invalid-root replacement

**Accepted with the narrow fail-closed option.** The profile inspection gate
classifies selected reads through a non-directory/symlink `.handbook` root as
invalid/unsafe and prohibits mutation. Therefore a planned pre-inspection
invalid-root deletion cannot coexist with the frozen safety order. The packet
now:

- removes `SetupRootAction::ReplaceInvalid` and
  `CanonicalRootRepairFailed`;
- maps init/auto against a non-directory or symlink root directly to the exact
  `InvalidCanonicalRoot` reason;
- permits only `preserve` for a directory and `create` for a missing root;
- states that current missing/directory mode routing remains while the old
  pre-membrane invalid-root deletion is intentionally retired; and
- requires tests and control proof for the fail-closed behavior.

No repair-before-inspection exception, production `route_state` change, or
hidden mutation is authorized.

### `HCM-1.4-PR3-R002` — SetupError optional fields not total

**Accepted.** No retained typed setup source can bind an instance ID, so the
ungrounded `instance_id` field was removed. A closed table now maps every
constructible error kind/reason pair to exactly `None`, `Some(".handbook")`, or
`Some(".handbook/state")`. Runtime plan/apply errors always use the constant
state-owner path, never a child path or parsed legacy human string. No other
kind/reason pair is constructible, and CLI prose cannot include the underlying
absolute path/I/O string.

## Cross-document reconciliation

- SPEC freezes the two-value root action enum, fail-closed root table, total
  error payload table, precedence, and CLI disclosure boundary.
- Plan and unchecked todo require test-first invalid-root and error-payload
  coverage.
- `06` records fail-closed invalid-root behavior and exact optional error-path
  proof with no instance/string inference.
- Earlier review dispatches/proof walls remain immutable history; this proof
  supersedes only their invalidated setup-root/error claims.

## Replayed checks

```text
git diff --check
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS: 3 record schemas, 2 internal-dispatch schemas, 2 templates, 39 records,
151 current internal dispatches, 8 admitted legacy dispatches, 39 ledger entries

checked-todo scan
PASS: zero checked implementation items
```

No Rust, Cargo, test, production authoring, reset owner, condition evidence,
canonical content, staging, commit, or HCM-2 work occurred.

## Remediation verdict

Both Required findings have bounded documentation-only remedies. Approval
remains pending a fourth different fresh isolated read-only reviewer over the
new complete manifest/fingerprint.
