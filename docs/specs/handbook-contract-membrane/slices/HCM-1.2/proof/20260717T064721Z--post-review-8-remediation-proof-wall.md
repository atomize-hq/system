# HCM-1.2 Post-Review-8 Remediation Proof Wall

**Captured:** 2026-07-17T06:47:21Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T062916Z--HCM-1-2--fresh-final-profile-review-8`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 8 returned `CHANGES_REQUIRED`. The parent
accepted its one Required finding and remediated it without widening the
slice. The conservative schema-location satisfiability proof now:

- proves a common candidate for simultaneous `items` and effective non-zero
  `contains` constraints or refuses when no witness is proven;
- counts a closed object's optional literal property toward `minProperties`
  only when both its name and its schema are provably satisfiable, while every
  required/ref property is still checked transitively;
- validates bounded string candidates against the actual local Draft 2020-12
  schema, including its pattern and length constraints;
- validates bounded numeric candidates against minimum, maximum, exclusive,
  and multiple constraints rather than treating numeric siblings as
  unconstrained; and
- conservatively refuses required/minimum object cardinality that depends on
  unproven patterned names.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed 145 unit/integration tests plus doc tests.
New focused cases prove all three Review-8 counterexamples refuse at the
selected terminal and/or through a transitively required referenced sibling:

- `integer` with `minimum: 1` and `maximum: 0`;
- `items: {type: string}` combined with effective non-zero
  `contains: {type: object}`; and
- a closed object whose only nameable property has the boolean-false schema
  while `minProperties: 1`.

The same matrix keeps the exact shipped constitutional closure accepted and
also covers false items forced by `contains`, property-name refusal, root,
intermediate, referenced-parent, referenced-child, and required-sibling
locations.

All required commands passed in one successful fail-fast replay. The raw
2,275-line log is `/tmp/hcm-1.2-final-proof-round-9.log` with SHA-256
`37c5f7296d223d3e6749836e3313b2264f163d33caa8a723099b3681c58fd09a`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 145 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 879.5 KiB uncompressed, 151.8 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 123 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 78 paths, no sibling slice or fixed product path |

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

## Review-9 gate

This wall records no Review-9 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, all prior dispatches, package manifest, and
all nine proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
