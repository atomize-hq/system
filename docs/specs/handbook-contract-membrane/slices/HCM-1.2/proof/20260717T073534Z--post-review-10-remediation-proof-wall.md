# HCM-1.2 Post-Review-10 Remediation Proof Wall

**Captured:** 2026-07-17T07:35:34Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T071337Z--HCM-1-2--fresh-final-profile-review-10`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 10 returned `CHANGES_REQUIRED`. The parent
accepted its two Required findings and remediated them without widening the
slice. The conservative schema-location satisfiability proof now:

- proves an actual Draft 2020-12-valid joint array witness when
  `uniqueItems`, `contains`, `prefixItems`, and item/contains cardinalities can
  interact, rather than certifying those constraints independently;
- bounds joint array witness construction to 64 items, 32 candidates per item,
  and 4,096 search states and refuses when those proof bounds are exhausted;
- counts an optional closed-object property toward `minProperties` only after
  its complete `dependentRequired` closure remains within `maxProperties`, all
  selected names and schemas are provably admissible, and every triggered
  `dependentSchemas` branch is trivially satisfiable; and
- applies both proofs to direct terminals and transitively required referenced
  siblings.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **151** unit/integration tests plus
doc tests across all 26 engine test binaries. New direct-terminal and
transitively required referenced-sibling negatives prove refusal of:

- a two-element unique array whose `minContains: 2` can only be met by two
  copies of the same value;
- a fixed-capacity two-element prefixed array whose required positions leave
  no capacity for a required contained value;
- a closed object's sole optional `minProperties` witness when selecting it
  triggers boolean-false `dependentSchemas`; and
- either of two optional `minProperties` witnesses when mutual
  `dependentRequired` expands beyond `maxProperties`.

Positive controls preserve two distinct matching witnesses, a contained item
with sufficient remaining array capacity, a safe optional property beside an
unsafe dependent-schema trigger, and dependency expansion within the exact
maximum property boundary.

All required commands passed in one successful fail-fast replay. The raw
2,284-line log is `/tmp/hcm-1.2-final-proof-round-11.log` with SHA-256
`932840efe0339a59cd58ce8954f9160ae9ed9f522d9dc92aa1ff9adc5c7011a6`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 151 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 901.5 KiB uncompressed, 154.2 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 125 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 82 paths, no sibling slice or fixed product path |

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

GitNexus impact analysis preceded the existing-symbol edit and reported `LOW`
for `schema_fragment_is_conservatively_satisfiable`; no `HIGH` or `CRITICAL`
symbol blast radius was present. Final staged change detection remains required
before the primary implementation commit.

## Review-11 gate

This wall records no Review-11 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, all prior dispatches, package manifest, and
all eleven proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
