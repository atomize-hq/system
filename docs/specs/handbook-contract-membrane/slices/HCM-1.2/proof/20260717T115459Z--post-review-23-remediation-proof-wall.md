# HCM-1.2 Post-Review-23 Remediation Proof Wall

**Captured:** 2026-07-17T11:54:59Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T113536Z--HCM-1-2--fresh-final-profile-review-23`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 23 returned `CHANGES_REQUIRED` with one Required
typed-error finding. A present, parseable non-object profile document was
incorrectly classified as `MissingSource` without a location.

The parent accepted the finding and performed the smallest bounded fix.
`parse_profile_source` now classifies YAML nulls, scalars, and sequences as
`InvalidProfileRecord` at the stable bounded location `profile_source`.
Actual repository source absence remains `MissingSource`; no source-admission,
profile schema, selection, layering, or fingerprint behavior changed.

RED/GREEN proof covers both public and resolver boundaries:

- public `parse_profile_source` rejects null, scalar, and sequence YAML with
  the exact kind and location;
- profile-selection Stage 5 rejects the same three record shapes in forward
  and reverse source orders;
- the Stage-5 record error precedes the later unreferenced-source check; and
- all existing missing-source cases retain their prior typed category.

Before editing the parser, GitNexus reported `MEDIUM` upstream risk: six direct
callers, 24 upstream symbols, and the profile-selection process. The complete
caller and workspace surface is covered below.

## Full proof

The remediated engine suite passed exactly **184** unit/integration tests plus
doc tests across the library unit target and all 27 engine integration-test
binaries. All required commands passed in one successful fail-fast replay. The
raw 2,363-line log is `/tmp/hcm-1.2-final-proof-round-24.log` with SHA-256
`78cd20954ac647c8c08ea0730eb74456e5e8ddc5c516f0086122aef3d5ab78cb`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 184 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 92 files, 980.2 KiB uncompressed, 163.0 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 138 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 109 paths, no sibling slice or fixed product path |

The exact package manifest remains byte-current at SHA-256
`a7678b8f3761e681e98f2b5e94459f4568c4d2ff9851e329bc5f776856829a23`.
`Cargo.toml`, `Cargo.lock`, and `crates/engine/Cargo.toml` remain byte-identical
to the entry baseline. No network, TLS, async, resolver, remote fetch, or
executable-hook dependency was added. All product-adoption behavior and
`PG-PROFILE-01`, `PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Review-24 gate

This wall records no Review-24 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, authorized fixtures,
task status, crosswalk, proof ledger, every baseline-to-subject test byte, all
prior dispatches, package manifest, and all twenty-four proof walls. A
different fresh isolated built-in `default` reviewer must return findings
first. Any valid Critical or Required finding requires another bounded parent
remediation, full proof replay, a new dispatch, and another different fresh
reviewer. No exact-subject byte may change after final `CLEAN`.
