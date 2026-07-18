# HCM-1.4 Planning Remediation 8 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_9`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T220933Z--HCM-1-4--fresh-planning-review-9.json`
- reviewed subject fingerprint:
  `sha256:34ee576a071d889faebff6ffe79c51c38ec7c004eecf0579509712108fca7693`
- review result: `CHANGES_REQUIRED`
- findings: two Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

Review 9 independently accepted the Review 1-7 remedies and the non-
authoritative Review 8 stop. It found one pre-existing compiler Windows build
failure outside the then-frozen file scope and one overclaim in the capability
report boundary.

## Finding dispositions

### `HCM-1.4-PR9-R001` — mandatory Windows proof was impossible in frozen scope

**Accepted.** The future implementation packet now:

- records the current compiler MSVC `libc::LOCK_EX` failure as RED;
- admits exactly one production-authoring file and hunk:
  `crates/compiler/src/author/mod.rs` may remove the caller-supplied operation
  parameter and move the same `LOCK_EX` constant into the Unix-only helper;
- preserves the existing Unix exclusive-lock/retry behavior and existing non-
  Unix no-op behavior;
- requires fresh impact and an explicit warning before editing the HIGH-risk
  `acquire_authoring_lock` surface; the planning impact found 24 upstream
  impacts, three direct callers, and one affected author CLI process;
- requires the compiler Windows target to pass after the hunk; and
- retains complete 47 compiler/22 CLI author regressions and forbids every
  other production authoring change.

The portability exception makes the mandatory Windows setup/inspection proof
executable without weakening the platform gate or turning HCM-1.4 into an
authoring migration.

### `HCM-1.4-PR9-R002` — report overclaimed a complete capability closure

**Accepted by narrowing the claim.** The packet now defines
`ProfileCapabilityTruth` and `ProfileCapabilityRow` as exactly the four-field
identity projection: owning instance ID, capability ID, exact contract ref, and
contract fingerprint. Setup/doctor equality covers only that projection.
Required bindings, allowed instance cardinality, instance binding maps,
semantic validators, and binding rules remain owned and accessible through the
selected registry; HCM-1.4 neither copies them into reports nor claims their
closure. The phase map, SPEC, plan, todo, tests, and `06` gate now use that same
ceiling.

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

Both Required findings have bounded documentation-only remedies. Approval
remains pending a tenth different fresh isolated read-only reviewer over the
new complete manifest and aggregate fingerprint.
