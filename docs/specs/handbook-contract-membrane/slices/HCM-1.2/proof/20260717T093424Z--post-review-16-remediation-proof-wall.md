# HCM-1.2 Post-Review-16 Remediation Proof Wall

**Captured:** 2026-07-17T09:34:24Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T091903Z--HCM-1-2--fresh-final-profile-review-16`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 16 returned `CHANGES_REQUIRED` with one Required
finding. The parent accepted it and remediated it without widening the slice.
Registry-aware satisfiability now normalizes every member of an array-valued
JSON Schema `type`, applies the existing fragment constraints to that effective
type, and recursively proves the complete object or array child/reference
closure under the existing depth and cycle budget. Failed branches restore the
traversal state; the union is accepted only when one complete normalized branch
is proven.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **165** unit/integration tests plus
doc tests across all 26 engine test binaries. New proof covers:

- direct and transitively referenced singleton `type: ["object"]` branches
  whose required child resolves to boolean false;
- direct and transitively referenced singleton `type: ["array"]` branches
  whose required item resolves to boolean false; and
- positive controls for both normalized object-child and array-item branches
  whose local references are satisfiable.

All required commands passed in one successful fail-fast replay. The raw
2,312-line log is `/tmp/hcm-1.2-final-proof-round-17.log` with SHA-256
`aa95d5b7039cb8b07291d3bffaa94446d771088ce76ed214bb16da0d82d0f01a`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 165 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 930.0 KiB uncompressed, 156.7 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 131 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 94 paths, no sibling slice or fixed product path |

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

GitNexus impact analysis preceded the existing method edit and reported `LOW`
for `schema_location_is_conservatively_satisfiable`, with no indexed upstream
dependents or execution flows. No `HIGH` or `CRITICAL` symbol blast radius was
present. Final staged change detection remains required before the primary
implementation commit.

## Review-17 gate

This wall records no Review-17 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, every baseline-to-subject test byte, all prior
dispatches, package manifest, and all seventeen proof walls. A different fresh
isolated built-in `default` reviewer must return findings first. Any valid
Critical or Required finding requires another bounded parent remediation, full
proof replay, a new dispatch, and another different fresh reviewer. No
exact-subject byte may change after final `CLEAN`.
