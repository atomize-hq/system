# HCM-1.2 Post-Review-17 Remediation Proof Wall

**Captured:** 2026-07-17T09:50:56Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T093459Z--HCM-1-2--fresh-final-profile-review-17`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 17 returned `CHANGES_REQUIRED` with one Required
finding. The parent accepted it and remediated it without widening the slice.
A successful bounded joint array witness now settles the fragment-level
combination without reapplying a broader `items` guard. Registry-aware `items`
traversal is required only when the minimum cardinality or unsatisfied
`contains` minimum requires a post-`prefixItems` position. Fully covered prefix
positions therefore retain their Draft 2020-12 meaning while participating
child references remain recursively proven.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **167** unit/integration tests plus
doc tests across all 26 engine test binaries. New direct and transitively
referenced positive controls combine:

- singleton `type: ["array"]`;
- `minItems: 1` and `maxItems: 1`;
- one fully covering `prefixItems` constant;
- `items: false`;
- matching `contains`, `minContains`, and `maxContains`;
- `uniqueItems: true`; and
- `unevaluatedItems: false`.

All required commands passed in one successful fail-fast replay. The raw
2,316-line log is `/tmp/hcm-1.2-final-proof-round-18.log` with SHA-256
`03bec7a7325fbce929176cadebee0dc8ff8e7355df4685b0020cfabc5f1b6fac`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 167 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 933.0 KiB uncompressed, 156.9 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 132 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 96 paths, no sibling slice or fixed product path |

## Exact package, manifest, and scope evidence

`package-definition-manifest.json` remains byte-current with SHA-256
`60eb15eb1651b124829982cbebd16159988d4e26feeceda18a35930eba47e7e7`.
It lists all 29 definition members literally. The proof replay asserted exact
set equality across that manifest, the live `definitions/` tree, and the
packaged `.crate`, then compared every member's size, SHA-256, and bytes.

`Cargo.toml`, `Cargo.lock`, and `crates/engine/Cargo.toml` remain byte-identical
to the entry baseline. The dependency tree adds no network, TLS, async,
resolver, remote fetch, or executable-hook dependency. The next manifest must
again be rebuilt against the complete live baseline-to-subject path set and
exclude only its own transport envelope.

GitNexus impact analysis preceded the indexed helper edit and reported `LOW`
for `schema_fragment_is_conservatively_satisfiable`: three direct dependents,
one module, and no indexed execution flow. The new branch helper was not yet
present in the index. No `HIGH` or `CRITICAL` symbol blast radius was present.
Final staged change detection remains required before the primary commit.

## Review-18 gate

This wall records no Review-18 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, every baseline-to-subject test byte, all prior
dispatches, package manifest, and all eighteen proof walls. A different fresh
isolated built-in `default` reviewer must return findings first. Any valid
Critical or Required finding requires another bounded parent remediation, full
proof replay, a new dispatch, and another different fresh reviewer. No
exact-subject byte may change after final `CLEAN`.
