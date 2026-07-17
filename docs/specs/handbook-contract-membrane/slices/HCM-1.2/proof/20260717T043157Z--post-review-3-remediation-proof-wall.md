# HCM-1.2 Post-Review-3 Remediation Proof Wall

**Captured:** 2026-07-17T04:31:57Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T040949Z--HCM-1-2--fresh-final-profile-review-3`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 3 returned `CHANGES_REQUIRED`. The parent
accepted all three Required findings and remediated them without widening the
slice:

1. admitted selected-ancestry schema and kind refs remain a union only for
   source accounting and shadowed fingerprint replay; root-to-leaf effective
   schema/kind registries now honor complete-field replacement, authored
   descriptors validate against their effective source-time registries, and
   the public final registry exposes only the winning literal sets;
2. capability dependencies now resolve the exact contract producer and require
   its declared capability ID to equal `target_ref` before provider counting,
   so absent, version-mismatched, and wrong-capability contracts report
   `InvalidDependencyContract` at `target_contract_ref`; and
3. `binding_shape` now refuses any traversed parent or object terminal with a
   non-empty `patternProperties` map even when `additionalProperties` is
   `false`, preserving a conservative provably-closed result.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed 131 unit/integration tests plus doc tests.
New focused cases prove:

- a child profile can replace the six ancestor schema/kind entries with the
  literal Project Authority subset while all shadowed ancestor sources remain
  admitted and fingerprinted; the returned schema/kind registry contains only
  that subset and the returned descriptor set contains only
  `project_authority`;
- an inherited descriptor that references a kind removed by the winning
  replacement refuses instead of being backfilled by the ancestor union;
- unknown contracts, exact-version mismatches, and known contracts whose
  capability ID differs from `target_ref` report
  `InvalidDependencyContract` at the exact contract field; and
- both a traversed parent with a narrower pattern and an object terminal with
  a catch-all pattern refuse binding-shape compatibility.

All required commands passed in one successful fail-fast replay. The raw
2,238-line log is `/tmp/hcm-1.2-final-proof-round-4.log` with SHA-256
`b44ff2341893a54411f1ef73755d0e2e560e408e98a3a847bdbfd395de887240`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 131 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 801.7 KiB uncompressed, 141.7 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 118 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 67 paths, no sibling slice or fixed product path |

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

GitNexus impact analysis preceded the existing-symbol edits. The index remains
stale and returned `UNKNOWN`, rather than a false-safe zero, for the new
HCM-1.2 selection resolver, descriptor resolver, and binding-shape method.
Final staged change detection remains required before the primary
implementation commit.

## Review-4 gate

This wall records no Review-4 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, all prior dispatches, package manifest, and
all four proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
