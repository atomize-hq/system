# HCM-2.1 final review 1 remediation proof wall

- entry HEAD: `fd6242fbab911eebb09a4d541a6e5e1e32dc5468`
- blocked dispatch:
  `20260719T063451Z--HCM-2-1--fresh-final-implementation-review`
- reviewer: `/root/hcm_2_1_final_review`
- verdict: `CHANGES_REQUIRED`
- evidence ceiling: unchanged from the HCM-2.1 implementation proof wall

## Required finding

The reviewer admitted the exact 73-entry canonical-LF subject and returned one
Required proof finding. Production Project Context authoring acquires the
descriptor-family lock and repeats preflight under that lock, but the active
Project Context author-test block did not prove contention or the under-lock
recheck required by the SPEC.

No production behavior was changed. The remediation adds one Unix-only public-
seam regression,
`author_project_context_waits_for_lock_and_rechecks_existing_truth`, to
`crates/compiler/tests/author.rs`. The test:

1. holds `.handbook/state/authoring/project_context.lock` exclusively;
2. starts public Project Context authoring and proves it cannot complete or
   create the selected target while the lock remains held;
3. installs valid canonical truth during contention;
4. releases the lock; and
5. proves the repeated preflight refuses with `ExistingCanonicalTruth` and
   preserves the installed bytes exactly.

The row uses real filesystem state, the public compiler API, and the platform's
real advisory lock. It introduces no mock, production hook, alternate API, or
new timing/lock contract.

## Impact and red/green proof

GitNexus upstream analysis classified the public
`author_project_context_from_input` seam as `CRITICAL`: 81 impacts, 32 direct
dependents, and 18 processes. The lock helper analysis was also `CRITICAL`: 120
impacts, 60 direct, and 18 processes. Both graphs contain the previously noted
unrelated-edge pollution, so the warning was retained and reported. The durable
repository edit changes only the integration test.

A disposable mutation checkout bypassed only
`with_project_context_authoring_lock`. The first 100 ms probe was rejected as
insufficiently sensitive because outer preflight could still be running. The
final row therefore uses a two-second completion deadline. With the lock bypass,
the test failed as required:

```text
authoring completed while its lock was held: Ok(".handbook/project/context.yaml")
test result: FAILED; 0 passed; 1 failed
```

Against the untouched production implementation, the exact row passes and
takes the timeout branch while locked, then receives the expected refusal after
unlock.

## Remediation verification

| Command or suite | Result |
|---|---|
| deliberate lock-bypass mutation test | RED, expected completion/mutation failure |
| focused production lock/recheck row | GREEN, 1 / 1 |
| `cargo test -p handbook-compiler --test author` | PASS, 49 / 49 |
| `cargo clippy -p handbook-compiler --test author -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS after formatting |
| `git diff --check` | PASS |

The earlier workspace, docs, native-Windows, smoke, archive, package, validator,
scope, and definition-manifest proof remains applicable because the remediation
adds only a Unix-gated test row. The native-Windows implementation is unchanged,
and the engine package membership and archive bytes are unchanged.

## Resumption gate

The blocked dispatch and its `CHANGES_REQUIRED` result remain immutable. The
parent must rebuild the exact complete-subject manifest, create a new immutable
dispatch, and use a different fresh isolated read-only reviewer. No primary
commit is allowed until that reviewer returns `CLEAN`; HCM-2.2 remains unstarted.
