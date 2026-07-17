# HCM-1.2 Post-Review-9 Remediation Proof Wall

**Captured:** 2026-07-17T07:12:45Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T064814Z--HCM-1-2--fresh-final-profile-review-9`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 9 returned `CHANGES_REQUIRED`. The parent
accepted its one Required finding and remediated it without widening the
slice. The conservative schema-location satisfiability proof now:

- refuses a `uniqueItems` minimum that exceeds a provable finite shared item
  domain and fails closed when two-or-more distinct witnesses are not proven;
- checks every `prefixItems` schema required by `minItems`, including local
  references resolved through the existing 32-edge traversal bound;
- counts required array slots that necessarily match `contains` and refuses
  them when they exceed `maxContains`;
- refuses a nontrivial `dependentSchemas` branch triggered by an effective
  required property unless that branch is the trivially satisfiable boolean
  true or empty schema; and
- applies the same proof to selected terminals, parents, and transitively
  required referenced siblings.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **149** unit/integration tests plus
doc tests. The 145-test count in the immutable Review-8 wall was stale for its
then-current subject; the raw Review-9-remediation replay was recounted from
all 26 engine test binaries and establishes the exact current total. New
focused direct-terminal and transitively required referenced-sibling cases
prove refusal of all four Review-9 counterexamples:

- `minItems: 2`, `items: {const: "only"}`, and `uniqueItems: true`;
- `minItems: 2`, `contains: true`, and `maxContains: 1`;
- `minItems: 1` with `prefixItems: [false]`; and
- required property `a` with `dependentSchemas: {a: false}`.

Positive controls preserve a two-member finite unique-item domain, an exact
two-match `maxContains` boundary, a satisfiable required prefix item, and a
triggered boolean-true dependent schema.

All required commands passed in one successful fail-fast replay. The raw
2,279-line log is `/tmp/hcm-1.2-final-proof-round-10.log` with SHA-256
`e57aebf9e48020872c76efd6cc33ce1ede25cbab2d2060b9e6a494fe069ce453`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 149 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 889.2 KiB uncompressed, 152.9 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 124 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 80 paths, no sibling slice or fixed product path |

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
for `binding_shape`; no `HIGH` or `CRITICAL` symbol blast radius was present.
Final staged change detection remains required before the primary
implementation commit.

## Review-10 gate

This wall records no Review-10 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, all prior dispatches, package manifest, and
all ten proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
