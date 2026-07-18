# HCM-1.4 Planning Remediation 9 Proof Wall

## Review lineage

- finding review agent: `/root/hcm_1_4_planning_review_10`
- immutable dispatch:
  `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260717T222942Z--HCM-1-4--fresh-planning-review-10.json`
- reviewed subject fingerprint:
  `sha256:56a70b115aec4e6e6952e3eee1b06e8f44d9683bdfa937dba3b91a17435a93ce`
- review result: `CHANGES_REQUIRED`
- findings: one Required, zero Critical/Optional/Nit
- remediation owner: parent orchestrator
- implementation performed: none

Review 10 independently accepted every earlier live remedy, including the
four-field capability-identity ceiling, and found one flaw in the first
Windows-portability prescription: the shared helper also owns explicit Unix
`LOCK_UN` during `Drop`.

## Finding disposition

### `HCM-1.4-PR10-R001` — one-argument helper prescription broke Unix unlock

**Accepted.** The portability remedy is now smaller and total:

- retain both existing cfg-specific two-argument `lock_authoring_file`
  signatures and bodies byte-for-byte;
- retain the Unix `Drop` call with `libc::LOCK_UN` byte-for-byte;
- change only `acquire_authoring_lock` by binding local `lock_operation` to
  `libc::LOCK_EX` under `#[cfg(unix)]` and ignored integer `0` under
  `#[cfg(not(unix))]` immediately before the unchanged helper call;
- preserve Unix exclusive acquisition, interruption retry, and explicit unlock;
- preserve the existing non-Unix no-op helper behavior; and
- require exact-hunk proof, compiler MSVC GREEN, fresh HIGH-risk impact/warning,
  and all 47 compiler/22 CLI author regressions.

The previous Remediation 8 proof wall remains immutable historical evidence;
its one-argument helper/signature prescription is explicitly superseded here.
The capability-identity remedy from that wall remains unchanged and accepted.

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
remains pending an eleventh different fresh isolated read-only reviewer over
the new complete manifest and aggregate fingerprint.
