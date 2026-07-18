# HCM-1.4 Planning Remediation 7 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_7`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T211659Z--HCM-1-4--fresh-planning-review-7.json`
- reviewed subject fingerprint:
  `sha256:d68328abf2435e4e35d5bd87d8635227ee4ebafb6f0764c7e1b1b680e6bfebd5`
- review result: `CHANGES_REQUIRED`
- findings: one Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

Review 7 independently accepted every Review 1-6 remedy and found one remaining
representation leak: independently public doctor error kind/reason fields could
construct two invalid pairings outside the intended two-row error contract.

## Finding disposition

### `HCM-1.4-PR7-R001` — DoctorError kind/reason combinations were not closed

**Accepted.** The compiler contract now:

- defines `DoctorError` as exactly two public enum variants,
  `ShippedProfileUnavailable` and `SelectedProfileDecisionInvalid`;
- exposes an ordered `DoctorError::ALL` containing exactly those variants;
- derives read-only `kind()` and `reason_code()` values from the variant;
- freezes the exact two-row projection to
  `ProfileResolution`/`ShippedProfileUnavailable` and
  `ProfileDecision`/`SelectedProfileDecisionInvalid`;
- exposes no independently constructible kind/reason fields or unbounded error
  payload; and
- requires exhaustive compiler replay of `ALL` plus proof that all default and
  injected doctor error exits use one of the two variants.

The SPEC, implementation plan, unchecked todo, and `06` proof gate all require
the same closed representation and exhaustive mapping.

## Replayed checks

```text
git diff --check
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
PASS

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
PASS

checked-todo scan
PASS: zero checked implementation items
```

No implementation, Cargo, production test, staging, commit, handoff, ledger,
or HCM-2 work occurred.

## Remediation verdict

The sole Required finding has a bounded documentation-only remedy. Approval
remains pending an eighth different fresh isolated read-only reviewer over the
new complete manifest and aggregate fingerprint.
