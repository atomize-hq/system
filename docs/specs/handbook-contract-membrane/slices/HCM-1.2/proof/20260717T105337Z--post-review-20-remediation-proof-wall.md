# HCM-1.2 Post-Review-20 Remediation Proof Wall

**Captured:** 2026-07-17T10:53:37Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T103124Z--HCM-1-2--fresh-final-profile-review-20`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 20 returned `CHANGES_REQUIRED` with one Required
public-owner finding. The reviewed implementation exposed the underlying
profile and capability types but did not expose the exact SPEC-owned names
`InstanceProfileDefinition` and `SemanticCapabilityContract`; it also lacked
the required typed `ProjectConditionRegistry` owner.

The parent accepted the finding and remediated it without widening the slice.
Compatible public aliases now expose the two exact contract names without
creating a second schema or semantic identity. A typed, deterministic
`ProjectConditionRegistry` owns project-condition loading, exact-ref lookup,
source-budget enforcement, duplicate identity refusal, and typed admitted-
source binding checks. Profile selection now obtains condition definitions
through that registry instead of assembling a private map directly.

The remediation did not add a condition evaluator or activate setup, doctor,
flow, compiler, CLI, SDK, Tauri, Substrate, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## TDD and impact proof

RED first proved that the three exact public names were absent. GREEN then
proved:

- public compilation of `SemanticCapabilityContract`,
  `ProjectConditionRegistry`, and `InstanceProfileDefinition`;
- exact-ref registry lookup and sorted ref enumeration;
- duplicate identities refuse identically under both source orders;
- typed source-binding conflicts refuse identically under both source orders;
- wrong-record and stale-fingerprint sources refuse with typed error kinds;
- the condition registry shares the repository source-byte budget; and
- all 18 profile-selection tests remain green through the registry-owned path.

Before editing `resolve_profile_selection`, GitNexus upstream impact reported
`HIGH`: 17 direct and two indirect callers, all in the test module, with no
affected product execution flow. That complete caller surface is covered by
the focused selection suite and the full engine/workspace proof below.

## Focused and full proof

The remediated engine suite passed exactly **180** unit/integration tests plus
doc tests across the library unit target and all 27 engine integration-test
binaries. All required commands passed in one successful fail-fast replay. The
raw 2,354-line log is `/tmp/hcm-1.2-final-proof-round-21.log` with SHA-256
`a1cf5b0da9904afd02b85ef2e309b80f2264d1705c4a51af95f0235c18e6cf48`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 180 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 92 files, 975.3 KiB uncompressed, 162.1 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 135 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 103 paths, no sibling slice or fixed product path |

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

## Review-21 gate

This wall records no Review-21 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, every baseline-to-subject test byte, all prior
dispatches, package manifest, and all twenty-one proof walls. A different fresh
isolated built-in `default` reviewer must return findings first. Any valid
Critical or Required finding requires another bounded parent remediation, full
proof replay, a new dispatch, and another different fresh reviewer. No exact-
subject byte may change after final `CLEAN`.
