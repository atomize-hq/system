# HCM-1.2 Post-Review-24 Remediation Proof Wall

**Captured:** 2026-07-17T12:27:35Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T115604Z--HCM-1-2--fresh-final-profile-review-24`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 24 returned `CHANGES_REQUIRED` with two Required
schema-registry findings. Registry-aware object minimum proof was blocked by a
fragment-only direct-`$ref` refusal before the registry could resolve the
selected witness, and fully prefix-covered array terminals incorrectly treated
`items: false` as applying to a required prefix item.

The parent accepted both findings and performed the smallest bounded fixes:

- registry-aware satisfiability may defer direct property references only long
  enough to resolve the exact selected property through the registry traversal;
  false targets and local reference cycles remain fail-closed;
- object minimum witnesses remain dependency-closure, property-name, maximum,
  and resolved-child checked;
- array terminal and registry-aware branch checks now share one post-prefix
  requirement predicate, including `minItems` and provable prefix satisfaction
  of `minContains`; and
- the binding-schema test helper exposes a fallible variant solely so the local
  cycle refusal can be asserted without weakening the HCM-1.1 load boundary.

RED/GREEN proof covers direct and transitively referenced optional witnesses,
false and cyclic negative controls, direct and referenced fully prefix-covered
array terminals, and a genuine post-prefix requirement that remains refused.

Before editing, GitNexus reported the registry-aware location traversal as
`LOW` upstream risk with zero indexed upstream callers or processes. It labeled
the test-helper refactor `HIGH` because 17 binding-shape tests directly call the
helper; that entire 30-test binding-shape module and the complete engine and
workspace surfaces passed below.

## Full proof

The remediated engine suite passed exactly **186** unit/integration tests plus
doc tests across the library unit target and all 27 engine integration-test
binaries. All required commands passed in one successful fail-fast replay. The
raw 2,360-line log is `/tmp/hcm-1.2-final-proof-round-25.log` with SHA-256
`7c14cfc78feff98cee517dd8ce08ab30c928a78389fc8c7cf402aea68e742a98`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 186 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 92 files, 986.5 KiB uncompressed, 163.5 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 139 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 111 paths, no sibling slice or fixed product path |

The exact package manifest remains byte-current at SHA-256
`a7678b8f3761e681e98f2b5e94459f4568c4d2ff9851e329bc5f776856829a23`.
`Cargo.toml`, `Cargo.lock`, and `crates/engine/Cargo.toml` remain byte-identical
to the entry baseline. No network, TLS, async, resolver, remote fetch, or
executable-hook dependency was added. All product-adoption behavior and
`PG-PROFILE-01`, `PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Review-25 gate

This wall records no Review-25 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, authorized fixtures,
task status, crosswalk, proof ledger, every baseline-to-subject test byte, all
prior dispatches, package manifest, and all twenty-five proof walls. A
different fresh isolated built-in `default` reviewer must return findings
first. Any valid Critical or Required finding requires another bounded parent
remediation, full proof replay, a new dispatch, and another different fresh
reviewer. No exact-subject byte may change after final `CLEAN`.
