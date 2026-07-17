# HCM-1.2 Post-Review-1 Remediation Proof Wall

**Captured:** 2026-07-17T03:36:45Z
**Phase / slice:** `HCM-1` / `HCM-1.2`
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`
**Prior review dispatch:** `20260717T025352Z--HCM-1-2--fresh-final-profile-review-1`
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 1 returned `CHANGES_REQUIRED`. The parent
accepted all ten required findings and remediated them without widening the
slice:

1. built-in definition and schema-document selection now uses only exact
   compile-time package-byte allowlists and ignores repository shadows;
2. one admission retains every definition byte and shares one 8 MiB budget
   with request-wide schema-document closure reads;
3. every retained typed source is closed-record decoded and its derived exact
   ref is compared with the declared binding before ancestry or closure work;
4. authored profile fingerprints are deferred until root-to-leaf typed
   dependency closure recomputation;
5. `binding_shape` shares one 32-edge traversal across the complete query,
   requires closed object parents and terminals, and refuses semantic `$ref`
   siblings;
6. profile failures preserve public typed identity, fingerprint, scope,
   ancestry, source-usage, and underlying registry categories;
7. the public artifact-instance descriptor exposes typed requiredness,
   dependency, lifecycle, intake, renderer, Projection, validation-overlay,
   and extension metadata without inventing an artifact-label length limit;
8. repository scope is one unique selected leaf over shipped or named
   authority, never a root or repository-on-repository chain;
9. source, schema, and descriptor paths reject URI-scheme and drive-like
   prefixes before filesystem access; and
10. focused proof now covers immutable built-ins, post-admission substitution,
    swapped schema/kind paths, combined definition/schema byte N/N+1,
    open/semantic-sibling binding shapes, public descriptor metadata, typed
    errors, and illegal repository layering.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed 125 unit/integration tests plus doc tests.
New focused cases prove:

- an empty repository resolves the exact shipped profile from immutable
  package bytes, and invalid same-path repository shadows cannot substitute;
- repository source bytes remain the originally admitted bytes after the
  source path is overwritten;
- schema-entry and artifact-kind binding/path swaps fail at typed stage 5;
- definition admission plus built-in schema-document closure accepts exactly
  8 MiB and rejects the sentinel byte;
- open object parents and semantic `$ref` siblings refuse while annotation-only
  siblings remain determinate;
- repository roots and repository-on-repository ancestry refuse with
  `IllegalProfileScope`, and cycles remain typed;
- invalid profile scope, identity, fingerprint syntax, stale fingerprint, and
  unreferenced source report distinct public error categories;
- descriptor public getters preserve typed dependency and later-owned empty
  values, a 65-scalar artifact label remains valid under canonical `05`, and
  URI/drive-like instance paths refuse.

All required commands passed in one fail-fast replay. The raw 2,232-line log
is `/tmp/hcm-1.2-final-proof-round-2.log` with SHA-256
`67da84770a55b67bd9537120dc79e4b0824703ef89be48c42d5f03f9c4e0579d`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 125 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 763.6 KiB uncompressed, 136.4 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 116 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 62 paths, no sibling slice or fixed product path |

## Exact package and scope evidence

`package-definition-manifest.json` has SHA-256
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

GitNexus impact analysis preceded existing-symbol edits. The stale index could
not resolve the new HCM-1.2 resolver symbols and reported `UNKNOWN`, not a
false-safe zero. The shared HCM-1.1 kind loader remained `MEDIUM`; the parent
did not modify the separately reported `HIGH` shared normalized-path parser,
using bounded HCM-1.2 lexical validation instead. Final staged change detection
remains required before the primary implementation commit.

## Review-2 gate

This wall records no Review-2 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, prior dispatch, package manifest, and both
proof walls. A different fresh isolated built-in `default` reviewer must
return findings first. Any valid Critical or Required finding requires another
bounded parent remediation, full proof replay, a new dispatch, and another
different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
