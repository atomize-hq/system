# HCM-1.2 Post-Review-2 Remediation Proof Wall

**Captured:** 2026-07-17T04:08:46Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T033828Z--HCM-1-2--fresh-final-profile-review-2`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 2 returned `CHANGES_REQUIRED`. The parent
accepted all five Required findings and remediated them without widening the
slice:

1. ancestry now counts parent transitions, accepts exactly 32 edges (33
   profiles), and refuses the 33rd edge;
2. descriptor requiredness, dependency namespace/target/contract/cardinality,
   provider-count, cycle, and constitutional-root failures now preserve
   distinct engine-owned error kinds and bounded field locations through the
   public profile error boundary;
3. duplicate schema and kind refs within each authored replace-whole field now
   refuse before ancestry unioning, for root and child sources in either
   ordering;
4. condition producers are derived only from the exact condition-ref union of
   every selected-ancestry descriptor replacement, including shadowed
   replacements, so condition-free descriptors need no unrelated producer and
   unrelated condition sources refuse; and
5. vocabulary definitions retain their stable-role producer pair, that
   producer is admitted and resolved, and its exact ref/fingerprint must match
   the effective profile/kind stable-role registry.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed 129 unit/integration tests plus doc tests.
New focused cases prove:

- the literal 33-profile/32-edge ancestry succeeds and the 33rd transition
  reports `ProfileAncestryDepthExceeded`;
- exact descriptor error kinds and bounded locations for invalid requiredness,
  dependency namespace/target/contract/cardinality/provider-count/cycle, and
  zero or multiple constitutional roots;
- root and child duplicate schema and kind refs refuse in forward and reverse
  request order with `DuplicateProfileDependency`;
- an all-unconditional descriptor selection resolves without a condition
  producer, while adding the shipped condition as an unrelated source reports
  `UnreferencedSource`; and
- a vocabulary missing its embedded stable-role producer reports
  `MissingSource`, a selected/vocabulary stable-role mismatch reports the
  registry mismatch, and changed producer bytes invalidate the producer
  fingerprint.

All required commands passed in one fail-fast replay. The raw 2,239-line log
is `/tmp/hcm-1.2-final-proof-round-3.log` with SHA-256
`351ef83e6db39b4adde9f373e660c1d26193af9c2f1fe16ecd4a8a48308f3466`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 129 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 789.0 KiB uncompressed, 140.5 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 117 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 65 paths, no sibling slice or fixed product path |

## Exact package and scope evidence

`package-definition-manifest.json` remains byte-current with SHA-256
`60eb15eb1651b124829982cbebd16159988d4e26feeceda18a35930eba47e7e7`.
It lists all 29 definition members literally. The proof replay asserted exact
set equality across that manifest, the live `definitions/` tree, and the
packaged `.crate`, then compared every member's size, SHA-256, and bytes.

`Cargo.toml`, `Cargo.lock`, and `crates/engine/Cargo.toml` remain byte-identical
to the entry baseline. The dependency tree adds no network, TLS, async,
resolver, remote fetch, or executable-hook dependency. The baseline-to-subject
scope contains only packet-authorized engine definition/profile code, tests,
fixtures, HCM-1.2 evidence, review dispatches, and bounded parent crosswalk and
proof-ledger updates.

GitNexus impact analysis preceded existing-symbol edits. For this remediation,
the indexed existing public `RegistryLoadErrorKind` surface was `LOW`; the
stale index could not resolve the new HCM-1.2 resolver/layering/vocabulary
symbols and therefore reported `UNKNOWN`, not a false-safe zero. The parent
did not modify the separately reported `HIGH` shared normalized-path parser.
Final staged change detection remains required before the primary
implementation commit.

## Review-3 gate

This wall records no Review-3 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, both prior dispatches, package manifest, and
all three proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
