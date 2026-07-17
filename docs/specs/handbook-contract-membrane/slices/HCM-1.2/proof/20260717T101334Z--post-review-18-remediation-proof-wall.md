# HCM-1.2 Post-Review-18 Remediation Proof Wall

**Captured:** 2026-07-17T10:13:34Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T095132Z--HCM-1-2--fresh-final-profile-review-18`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 18 returned `CHANGES_REQUIRED` with one Required
finding. The parent accepted it and remediated it without widening the slice.
Semantic capability and validator Stage-5 admission and Stage-8 resolution now
classify wrong record schema IDs, versions, and non-empty extensions as
`UnsupportedRecord` at exact bounded field locations. Closed typed decoding
prevalidates top-level and nested field shapes, distinguishes `UnknownField`
from `SyntaxError`, and carries stable slash-delimited locations without source
contents or filesystem paths. Actual dependency failures remain
`UnsupportedDependency`.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **171** unit/integration tests plus
doc tests across all 26 engine test binaries. New proof covers capability and
validator records for:

- wrong schema ID and schema version;
- non-empty extensions;
- top-level and nested unknown fields;
- nested wrong-type fields and array members;
- stale-fingerprint compound inputs proving Stage-5 record failures win; and
- valid-first and mutant-first source orders at Stage 8 with exact kind and
  location equality.

All required commands passed in one successful fail-fast replay. The raw
2,324-line log is `/tmp/hcm-1.2-final-proof-round-19.log` with SHA-256
`96bcbfed74df6dd28baf9e8987b9c46cdc7ead27a06760c525d0c68f06564de0`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 171 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 951.3 KiB uncompressed, 159.0 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 133 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 98 paths, no sibling slice or fixed product path |

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

GitNexus impact analysis preceded every existing-symbol edit and reported
`LOW`. The shared decode path has two direct dependents and reaches the
profile-selection flow; each Stage-5 exact-ref producer reaches nineteen test
symbols; both Stage-8 resolvers have no indexed upstream dependents. No `HIGH`
or `CRITICAL` symbol blast radius was present. Final staged change detection
remains required before the primary implementation commit.

## Review-19 gate

This wall records no Review-19 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, every baseline-to-subject test byte, all prior
dispatches, package manifest, and all nineteen proof walls. A different fresh
isolated built-in `default` reviewer must return findings first. Any valid
Critical or Required finding requires another bounded parent remediation, full
proof replay, a new dispatch, and another different fresh reviewer. No
exact-subject byte may change after final `CLEAN`.
