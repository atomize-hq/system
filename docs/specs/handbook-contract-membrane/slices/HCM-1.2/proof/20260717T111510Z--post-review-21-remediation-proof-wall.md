# HCM-1.2 Post-Review-21 Remediation Proof Wall

**Captured:** 2026-07-17T11:15:10Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T105450Z--HCM-1-2--fresh-final-profile-review-21`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 21 returned `CHANGES_REQUIRED` with three
Required boundary findings. The parent accepted all three and remediated only
the exact cited surfaces:

1. The shipped Context Resolution stack asset moved from the unauthorized
   `context-resolution-stacks` package class to the exact authorized
   `context-resolution` class. Built-in, focused-test, package-manifest, tree,
   and `.crate` paths now agree byte-for-byte.
2. The two repository-profile fixtures moved from `hcm_1_2_profile` to the
   packet's exact `hcm_1_2_repository_profile` area, and every test source
   binding now names that area.
3. The public `ArtifactInstanceRegistry::resolve` boundary now rejects a
   duplicate condition exact ref instead of silently overwriting it. Equal
   fingerprints return `DuplicateIdentity`; a conflicting fingerprint for one
   identity returns `ConflictingIdentity`. The typed condition registry's
   both-order duplicate and typed-binding conflict regressions remain green.

The remediation did not change any definition bytes, identity, fingerprint,
shipped set, or product behavior. It did not add a condition evaluator or
activate setup, doctor, flow, compiler, CLI, SDK, Tauri, Substrate, vocabulary
application, Context Resolution execution, renderer, intake, lifecycle,
Projection, publication, or HCM-1.3 behavior. The classification ceiling
remains `BoundaryLanded`; `PG-PROFILE-01`, `PG-ARTIFACT-01`, `PG-KIND-01`, and
`PG-KIND-02` remain open.

## TDD, impact, and exact-boundary proof

RED first reproduced Review 21's duplicate-condition probe: the public
artifact resolver returned `Ok` for the same typed condition twice. GREEN now
returns the exact typed duplicate error before descriptor resolution. Focused
proof also verifies:

- no live engine or current HCM-1.2 package evidence names
  `context-resolution-stacks`;
- no live engine test or fixture path names `hcm_1_2_profile`;
- the literal package member set contains exactly
  `context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml`;
- the authorized fixture directory contains exactly the root and repository
  profile fixtures; and
- all 18 profile-selection, three stack, and nine instance tests pass.

Before editing the existing symbols, GitNexus reported `HIGH` upstream risk
for built-in `definition` (two direct callers, 23 upstream symbols, four
affected processes), `MEDIUM` for `ArtifactInstanceRegistry::resolve` (13
direct test callers and the profile-selection process), and LOW to MEDIUM for
the affected fixture-only helpers. The full caller and workspace surface is
covered below.

## Full proof

The remediated engine suite passed exactly **181** unit/integration tests plus
doc tests across the library unit target and all 27 engine integration-test
binaries. All required commands passed in one successful fail-fast replay. The
raw 2,357-line log is `/tmp/hcm-1.2-final-proof-round-22.log` with SHA-256
`1e5d9b6d6668a926c395bce9bdbfd75892daa82a68478f542a5fd1b2f25f8662`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 181 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 92 files, 976.8 KiB uncompressed, 162.2 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 136 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 105 paths, no sibling slice or fixed product path |

## Exact package, manifest, and scope evidence

The path-corrected `package-definition-manifest.json` has SHA-256
`a7678b8f3761e681e98f2b5e94459f4568c4d2ff9851e329bc5f776856829a23`.
It lists all 29 definition members literally. The proof replay asserted exact
set equality across that manifest, the live `definitions/` tree, and the
packaged `.crate`, then compared every member's size, SHA-256, and bytes. The
Context Resolution stack's bytes and member hash did not change; only its
authority-correct member path changed.

`Cargo.toml`, `Cargo.lock`, and `crates/engine/Cargo.toml` remain byte-identical
to the entry baseline. The dependency tree adds no network, TLS, async,
resolver, remote fetch, or executable-hook dependency. The next manifest must
again be rebuilt against the complete live baseline-to-subject path set and
exclude only its own transport envelope.

## Review-22 gate

This wall records no Review-22 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, authorized fixtures,
task status, crosswalk, proof ledger, every baseline-to-subject test byte, all
prior dispatches, package manifest, and all twenty-two proof walls. A different
fresh isolated built-in `default` reviewer must return findings first. Any
valid Critical or Required finding requires another bounded parent
remediation, full proof replay, a new dispatch, and another different fresh
reviewer. No exact-subject byte may change after final `CLEAN`.
