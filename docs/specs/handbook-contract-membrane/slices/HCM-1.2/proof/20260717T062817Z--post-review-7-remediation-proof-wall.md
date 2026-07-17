# HCM-1.2 Post-Review-7 Remediation Proof Wall

**Captured:** 2026-07-17T06:28:17Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T060601Z--HCM-1-2--fresh-final-profile-review-7`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 7 returned `CHANGES_REQUIRED`. The parent
accepted both Required findings and remediated them without widening the
slice:

1. the shared conservative schema-location satisfiability proof now rejects
   cross-key contradictions where effective `minContains` exceeds
   `maxItems`, where `contains` requires an item but the item schema is
   impossible, or where a closed object cannot name enough properties to meet
   `minProperties`. Literal properties are counted only when admitted by
   `propertyNames`; patterned or additional properties do not become invented
   satisfiability evidence; and
2. artifact-kind `semantic_capabilities` now decode as the closed typed
   `AuthoredKindCapability` record during the outer Stage 5 parse rather than
   remaining raw JSON until Stage 8. Nested unknown fields therefore return
   the typed `UnknownField` failure before Stage 6 ancestry, including under
   reversed source ordering.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed 144 unit/integration tests plus doc tests.
New focused cases prove:

- both cross-key array/object contradictions refuse as selected terminals and
  as transitively required referenced siblings;
- effective `minContains` also forces a possible item, and closed-object
  `minProperties` cannot count a literal name refused by `propertyNames`;
- the complete shipped constitutional schema and its bounded string patterns
  and unique arrays remain accepted;
- an active semantic-capability record with a nested unknown member refuses
  during outer artifact-kind decoding; and
- that Stage 5 nested failure wins over a simultaneous Stage 6 profile cycle
  in forward and reversed artifact-kind source order.

All required commands passed in one successful fail-fast replay. The raw
2,273-line log is `/tmp/hcm-1.2-final-proof-round-8.log` with SHA-256
`3797ad414ea55b07a4fbe28ed118b490ae1e68d69a0ea2ee8ba7c72e95724530`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 144 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 872.7 KiB uncompressed, 150.7 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 122 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 76 paths, no sibling slice or fixed product path |

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

GitNexus impact analysis preceded the existing-symbol edits. The current index
reported `LOW` for `binding_shape`, `MEDIUM` for artifact-kind outer parsing,
and `LOW` for exact artifact-kind identity admission. No `HIGH` or `CRITICAL`
symbol blast radius was present. Final staged change detection remains required
before the primary implementation commit.

## Review-8 gate

This wall records no Review-8 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, all prior dispatches, package manifest, and
all eight proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
