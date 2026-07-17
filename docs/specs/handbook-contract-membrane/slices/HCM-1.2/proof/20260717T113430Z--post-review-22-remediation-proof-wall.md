# HCM-1.2 Post-Review-22 Remediation Proof Wall

**Captured:** 2026-07-17T11:34:30Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T111616Z--HCM-1-2--fresh-final-profile-review-22`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 22 returned `CHANGES_REQUIRED` with one Required
proof finding. The production artifact-instance resolver distinguished an
equal-fingerprint duplicate from a same-identity conflicting fingerprint, but
the integration suite reached only the equal-fingerprint branch.

The parent accepted the finding and performed the smallest bounded
remediation. One private `insert_condition_fingerprint` helper now owns the
production insertion/classification branch. A direct unit regression invokes
that exact helper and proves:

- first fingerprint followed by itself returns `DuplicateIdentity`;
- second fingerprint followed by itself returns `DuplicateIdentity`;
- first followed by second returns `ConflictingIdentity`; and
- second followed by first returns `ConflictingIdentity`.

The existing public resolver regression still proves that two equal typed
condition definitions refuse before descriptor resolution. The existing typed
condition registry regressions still prove both-order duplicate source refusal
and both-order typed-binding conflicts. No public API, definition byte,
identity, fingerprint, shipped set, package path, fixture path, or product
behavior changed in this remediation.

## Impact and full proof

Before editing the existing resolver again, GitNexus reconfirmed `MEDIUM`
upstream risk: 13 direct test callers, 28 upstream symbols, and the profile-
selection process. The complete caller and workspace surface is covered below.

The remediated engine suite passed exactly **182** unit/integration tests plus
doc tests across the library unit target and all 27 engine integration-test
binaries. All required commands passed in one successful fail-fast replay. The
raw 2,358-line log is `/tmp/hcm-1.2-final-proof-round-23.log` with SHA-256
`6d39603286bcbdd2dba716ce82fb1fd7558e28d3f755dbd031a28571ec98d66b`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 182 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 92 files, 978.3 KiB uncompressed, 162.7 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 137 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 107 paths, no sibling slice or fixed product path |

The path-corrected `package-definition-manifest.json` remains byte-current at
SHA-256 `a7678b8f3761e681e98f2b5e94459f4568c4d2ff9851e329bc5f776856829a23`.
`Cargo.toml`, `Cargo.lock`, and `crates/engine/Cargo.toml` remain byte-identical
to the entry baseline. No network, TLS, async, resolver, remote fetch, or
executable-hook dependency was added. All product-adoption behavior and
`PG-PROFILE-01`, `PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Review-23 gate

This wall records no Review-23 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, authorized fixtures,
task status, crosswalk, proof ledger, every baseline-to-subject test byte, all
prior dispatches, package manifest, and all twenty-three proof walls. A
different fresh isolated built-in `default` reviewer must return findings
first. Any valid Critical or Required finding requires another bounded parent
remediation, full proof replay, a new dispatch, and another different fresh
reviewer. No exact-subject byte may change after final `CLEAN`.
