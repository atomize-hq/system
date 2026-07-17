# HCM-1.2 Post-Review-19 Remediation Proof Wall

**Captured:** 2026-07-17T10:30:48Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T101412Z--HCM-1-2--fresh-final-profile-review-19`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 19 returned `CHANGES_REQUIRED` with one Required
proof finding. The parent accepted it and remediated it without widening the
slice or changing shipped producer semantics. Complete table-driven structural
mutation matrices now exercise the condition, vocabulary, matcher, escalation,
promotion, and stack records at every applicable top-level and nested record
boundary. The matrices distinguish structural error kinds before stale
fingerprint fallback and cover each frozen producer rather than only the first
policy source.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **175** unit/integration tests plus
doc tests across all 26 engine test binaries. The new matrices prove:

- missing, extra, wrong-type, unsupported-version, and forged-fingerprint
  mutations for all six producer records;
- condition numeric overflow, array-member type, self-reference, and exact
  literal-set refusal;
- vocabulary nested stable-role selection closure and every required-empty
  mapping/list/extension boundary;
- matcher nested selector grammar, exact segment bound, and target-kind shape;
- escalation and promotion binding/outcome/authority arrays and exact bounds;
- stack level/default/domain/ranked-value/policy-selection nesting, cardinality
  bounds, reference/fingerprint closure, and extension refusal; and
- exact expected typed error kinds for every case.

All required commands passed in one successful fail-fast replay. The raw
2,328-line log is `/tmp/hcm-1.2-final-proof-round-20.log` with SHA-256
`a7b6b7c5f5355368ad48310d05b0fcda676627274d57626c1aab85b0f19cd60e`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 175 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 969.2 KiB uncompressed, 161.0 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 134 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 100 paths, no sibling slice or fixed product path |

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

This remediation added tests and proof evidence only; it edited no existing
function, class, or method and therefore required no additional symbol-level
GitNexus impact analysis. Final staged change detection remains required before
the primary implementation commit.

## Review-20 gate

This wall records no Review-20 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, every baseline-to-subject test byte, all prior
dispatches, package manifest, and all twenty proof walls. A different fresh
isolated built-in `default` reviewer must return findings first. Any valid
Critical or Required finding requires another bounded parent remediation, full
proof replay, a new dispatch, and another different fresh reviewer. No
exact-subject byte may change after final `CLEAN`.
