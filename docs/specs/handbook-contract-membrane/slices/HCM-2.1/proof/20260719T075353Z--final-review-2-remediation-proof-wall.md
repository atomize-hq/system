# HCM-2.1 final review 2 remediation proof wall

- entry HEAD: `fd6242fbab911eebb09a4d541a6e5e1e32dc5468`
- blocked dispatch:
  `20260719T073002Z--HCM-2-1--fresh-final-implementation-review-2`
- reviewer: `/root/hcm_2_1_final_review_2`
- verdict: `CHANGES_REQUIRED`
- evidence ceiling: unchanged from the HCM-2.1 implementation proof wall

## Required finding

The second final reviewer admitted the exact 75-entry canonical-LF subject and
confirmed that the lock regression uses the real public compiler seam. It found
one remaining mutation-sensitivity gap: the candidate author call and the truth
installed during lock contention serialized the same record. A faulty
write-before-recheck implementation could therefore overwrite with identical
bytes, return `ExistingCanonicalTruth`, and satisfy the byte-preservation
assertion.

The remediation remains test-only. The installed valid canonical record now has
the distinct literal summary
`Existing canonical truth must survive lock contention.` The test independently
serializes the authoring candidate and asserts the two byte sequences differ
before installing the existing truth. The blocked author call continues to use
the original valid candidate.

## Ordering mutation proof

GitNexus could not yet resolve the newly added test symbol, so it reported
`UNKNOWN` with zero indexed callers. The public author seam retains the prior
`CRITICAL` impact warning: 81 impacts, 32 direct dependents, and 18 processes.
No durable production function was edited.

In the disposable mutation checkout, the real lock acquisition was restored and
only the two operations inside the production lock closure were reversed:
candidate bytes were written before the repeated preflight. The strengthened
test failed after unlock because the actual file contained candidate bytes while
the expected file contained the byte-distinct installed truth. The production
implementation remains correctly ordered as preflight, then write.

```text
assertion `left == right` failed
left: candidate canonical YAML bytes
right: distinct installed canonical YAML bytes
test result: FAILED; 0 passed; 1 failed
```

This mutation demonstrates that refusal alone is insufficient and that the
exact-byte assertion now proves recheck-before-write plus preservation.

## Remediation verification

| Command or suite | Result |
|---|---|
| deliberate write-before-recheck mutation | RED, expected distinct-byte preservation failure |
| strengthened production lock/recheck row | GREEN, 1 / 1 |
| `cargo test -p handbook-compiler --test author` | PASS, 49 / 49 |
| `cargo clippy -p handbook-compiler --test author -- -D warnings` | PASS |
| `cargo fmt --all -- --check` | PASS |
| `git diff --check` | PASS |

The first final-review remediation's real-lock/no-completion/no-mutation proof
remains active. This wall adds the missing byte-distinct ordering sensitivity;
it changes no production, native-Windows, fixture, package, definition, schema,
pipeline/stage, or installed-skill byte. All earlier broad proof remains
applicable.

## Resumption gate

Both blocked final-review dispatches and results remain immutable. The parent
must rebuild the complete-subject manifest, create a third immutable dispatch,
and use another different fresh isolated read-only reviewer. No primary commit
is allowed until that exact subject returns `CLEAN`; HCM-2.2 remains unstarted.
