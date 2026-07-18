# HCM-1.4 Planning Remediation 5 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_5`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T204012Z--HCM-1-4--fresh-planning-review-5.json`
- reviewed subject fingerprint:
  `sha256:e2d0c60e12b0c5f2ba099b95833ce7e0f381bdb6a247d3a6992cfccc153782ad`
- review result: `CHANGES_REQUIRED`
- findings: one Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

Review 5 independently accepted every earlier remedy and found one remaining
proof-design defect: one combined `compile_fail` doctest could pass when only
one of the two privacy boundaries remained closed, and it could not serve as a
pre-API RED because the missing type alone would make it pass.

## Finding disposition

### `HCM-1.4-PR5-R001` — combined privacy doctest was not independently discriminating

**Accepted.** The packet now:

- requires one independent `compile_fail,E0451` doctest containing only an
  external `SetupError` struct literal;
- requires a second independent `compile_fail,E0624` doctest containing only
  an external call to crate-private `SetupError::from_code`;
- retains exhaustive unit replay of all thirteen `SetupErrorCode` rows;
- classifies both privacy doctests as post-API GREEN/boundary proof rather than
  as pre-production RED proof; and
- limits the preceding compiler RED step to positive compile-contract tests
  that fail while the public types and accessors are absent.

The current `SPEC.md`, implementation plan, unchecked todo, and `06` proof gate
all encode the same two-example, error-constrained boundary proof and corrected
RED-before-GREEN order. The earlier Remediation 4 proof wall remains immutable
historical evidence; its singular-doctest claim is explicitly superseded by
this remediation.

## Replayed checks

```text
git diff --check
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS

checked-todo scan
PASS: zero checked implementation items
```

No implementation, Cargo, production test, staging, commit, handoff, ledger,
or HCM-2 work occurred.

## Remediation verdict

The sole Required finding has a bounded documentation-only remedy. Approval
remains pending a sixth different fresh isolated read-only reviewer over the
new complete manifest and aggregate fingerprint.
