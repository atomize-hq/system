# HCM-1.4 Planning Remediation 6 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_6`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T205757Z--HCM-1-4--fresh-planning-review-6.json`
- reviewed subject fingerprint:
  `sha256:44b1c445a64e157844cceece48efbcdb23cf88e57783707e4210de0ffdb7641f`
- review result: `CHANGES_REQUIRED`
- findings: one Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

Review 6 independently accepted every Review 1-5 remedy and found one remaining
authority contradiction: the selected condition definition reserves `unknown`
for an admitted evaluable basis that proves neither truth value, while missing
required evidence/input must resolve first to `unresolved`.

## Finding disposition

### `HCM-1.4-PR6-R001` — unavailable required evidence/input must be unresolved

**Accepted.** The control pack, SPEC, plan, and unchecked todo now:

- bind the exact selected condition definition and its fingerprint;
- apply the definition's frozen missing-required-input precedence;
- emit only `unresolved` with `evidence_contract_unavailable`,
  `indeterminate` applicability, and a null evidence-closure fingerprint;
- reserve `unknown` in the six-value vocabulary for a future admitted evaluable
  basis that proves neither truth value;
- keep all observation/evidence/freshness/assertion/boolean/override input
  surfaces absent; and
- update the exact derivation, tests, exit gate, and stop conditions without
  widening into the separate evidence/evaluator contract.

The earlier Remediation 1 proof wall remains immutable historical evidence; its
`unknown` condition claim is explicitly superseded by this remediation.

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
remains pending a seventh different fresh isolated read-only reviewer over the
new complete manifest and aggregate fingerprint.
