# HCM-1.2 Post-Review-13 Remediation Proof Wall

**Captured:** 2026-07-17T08:45:07Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T082449Z--HCM-1-2--fresh-final-profile-review-13`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 13 returned `CHANGES_REQUIRED` with one Required
finding. The parent accepted it and remediated it without widening the slice.
The central conservative satisfiability proof now validates array-valued JSON
Schema `type` declarations as a non-empty, duplicate-free list of known JSON
Schema primitive type names, normalizes each listed member into its own typed
branch, and accepts the union only when at least one normalized branch is
conservatively proven satisfiable. It no longer allows an array-valued `type`
to bypass the established per-type contradiction checks.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **159** unit/integration tests plus
doc tests across all 26 engine test binaries. New proof covers:

- a direct singleton `type: ["object"]` branch whose positive
  `minProperties` cannot be satisfied under `unevaluatedProperties: false`;
- the same impossible singleton array-valued type reached through a
  transitively required local reference; and
- a positive `type: ["object"]` control whose declared property witnesses the
  required evaluated property.

All required commands passed in one successful fail-fast replay. The raw
2,299-line log is `/tmp/hcm-1.2-final-proof-round-14.log` with SHA-256
`7746316ce80f783c4ed47177bad885807f898077ec7e21e01459cdc3877e0d3a`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 159 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 918.3 KiB uncompressed, 155.9 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 128 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 88 paths, no sibling slice or fixed product path |

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

GitNexus impact analysis preceded the existing-symbol edit and reported `LOW`
for `schema_fragment_is_conservatively_satisfiable`; no `HIGH` or `CRITICAL`
symbol blast radius was present. Final staged change detection remains required
before the primary implementation commit.

## Review-14 gate

This wall records no Review-14 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, every baseline-to-subject test byte, all prior
dispatches, package manifest, and all fourteen proof walls. A different fresh
isolated built-in `default` reviewer must return findings first. Any valid
Critical or Required finding requires another bounded parent remediation, full
proof replay, a new dispatch, and another different fresh reviewer. No
exact-subject byte may change after final `CLEAN`.
