# HCM-1.2 Post-Review-15 Remediation Proof Wall

**Captured:** 2026-07-17T09:18:27Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T090501Z--HCM-1-2--fresh-final-profile-review-15`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 15 returned `CHANGES_REQUIRED` with one Required
finding. The parent accepted it and remediated it without widening the slice.
Minimum-property witness selection now returns the exact dependency-closed set
of declared property names used to prove `minProperties`. The registry-aware
satisfiability traversal recursively proves every selected property's schema
through the existing bounded local-reference traversal before accepting the
object witness. An optional declared property can no longer hide an impossible
transitively required `$ref` behind fragment-only inspection.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **163** unit/integration tests plus
doc tests across all 26 engine test binaries. New proof covers:

- a direct required object whose optional `minProperties` witness contains a
  transitively required local reference resolving to boolean false;
- the same impossible witness reached through an additional required local
  reference; and
- a positive control whose selected optional witness contains a transitively
  required satisfiable local reference.

All required commands passed in one successful fail-fast replay. The raw
2,308-line log is `/tmp/hcm-1.2-final-proof-round-16.log` with SHA-256
`c00b5de8b3bba10d2df75e31b0bcb57a4e9e31b50ff960024a6bd2ba1265a70b`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 163 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 925.0 KiB uncompressed, 156.3 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 130 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 92 paths, no sibling slice or fixed product path |

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

GitNexus impact analysis preceded both existing-symbol edits and reported
`LOW`: the minimum-property witness helper has one direct and two indirect
dependents, while the registry-aware satisfiability method has no indexed
upstream dependents or execution flows. No `HIGH` or `CRITICAL` symbol blast
radius was present. Final staged change detection remains required before the
primary implementation commit.

## Review-16 gate

This wall records no Review-16 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, every baseline-to-subject test byte, all prior
dispatches, package manifest, and all sixteen proof walls. A different fresh
isolated built-in `default` reviewer must return findings first. Any valid
Critical or Required finding requires another bounded parent remediation, full
proof replay, a new dispatch, and another different fresh reviewer. No
exact-subject byte may change after final `CLEAN`.
