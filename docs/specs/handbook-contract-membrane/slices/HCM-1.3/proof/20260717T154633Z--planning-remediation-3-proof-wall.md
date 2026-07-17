# HCM-1.3 Planning Remediation 3 Proof Wall

## Classification and lineage

- phase/slice: `HCM-1` / `HCM-1.3`
- work class: planning-only remediation
- baseline: `c5733785fbd60b7d7a19318cb86058395a02e1c3`
- predecessor subject:
  `sha256:1c514a65bd31ea0095776ba52084a7a35e71b329158f789c0bf6a82774ab9110`
- predecessor dispatch:
  `20260717T153116Z--HCM-1-3--fresh-planning-review-4`
- predecessor reviewer: `/root/hcm_1_3_planning_review_4`
- predecessor verdict: `CHANGES_REQUIRED`
- remediation timestamp: `20260717T154633Z`

Review 4 returned zero Critical, one Required, and no other findings. It
replayed the complete eleven-file subject, exact 29-file fixed-surface/facade
set, public API, TDD ordering, HCM-1.2 focused source tests, package evidence,
security boundary, unchecked todo, evidence classification, and all handoff
validator modes. This wall accepts and remediates its sole scope contradiction.
It does not claim final `CLEAN`.

## Finding accepted

`HCM-1.3-R4-001` identified that the future implementation SPEC allowed only
`03`, `04`, and `06` among existing documentation files while Task 11 required
a deterministic rebuild of the existing
`docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl`. A future parent
could not satisfy both the exact allowlist and mandatory two-commit closeout.

## Remediation

The SPEC now adds exactly one existing handoff-control file:

```text
docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl
```

Its use is restricted to the deterministic rebuild after adding the new parent
v1.2 implementation closeout record. The ledger and new record must be staged
and committed together in the separate second closeout commit. No other ledger
mutation is allowed.

Task 11 now repeats that exact path and makes the deterministic rebuild the
sole allowed existing handoff-control-file change. Any historical-record
rewrite, other record rewrite, dispatch rewrite, template/schema edit, or other
control-plane mutation is a scope stop. The todo requires an explicit check
that `handoffs/ledger.jsonl` is the only existing handoff-control file changed
before committing only the new closeout record plus ledger.

Mechanical synchronization proof passed:

```text
PASS: exact ledger allowlist and closeout-only restriction are synchronized
PASS: whitespace and unchecked todo
```

## Replayed scope and non-goal gates

- future implementation runtime existing-file allowlist remains limited to
  `crates/engine/src/lib.rs` plus the conditional read-only iterator addition
  in `artifact_instance.rs`;
- future implementation post-proof control-pack edits remain limited to exact
  `03`, `04`, and `06` classifications;
- future closeout control-plane mutation is exactly one new v1.2 record plus
  deterministic `ledger.jsonl` rebuild;
- the first commit remains exact reviewed implementation subject plus review/
  proof evidence;
- the second commit remains only the new closeout record plus ledger;
- all three validator modes, staged change detection, cached diff check, and
  cached diff inspection remain mandatory;
- no implementation occurs in this planning session;
- setup/doctor adoption, canonical content, behavior execution, compatibility
  dispatch, and HCM-1.4 remain prohibited; and
- the future implementation todo remains entirely unchecked.

The API, TDD, 29-file inventory, package, security, classification, and review
lineage bytes were not changed by this remediation.

## Remediation verdict before Review 5

| Gate | Result |
|---|---|
| R4-001 exact ledger allowlist | PASS |
| deterministic closeout-only restriction | PASS |
| plan/todo synchronization | PASS |
| two-commit implementability | PASS |
| docs-only/non-goal preservation | PASS |
| different fresh exact-subject review | PENDING |

A new aggregate fingerprint and immutable dispatch follow. Any valid Critical
or Required finding requires another proof wall/fingerprint and another
different fresh reviewer.
