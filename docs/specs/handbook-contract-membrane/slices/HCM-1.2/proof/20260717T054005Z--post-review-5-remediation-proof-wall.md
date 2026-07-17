# HCM-1.2 Post-Review-5 Remediation Proof Wall

**Captured:** 2026-07-17T05:40:05Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T051416Z--HCM-1-2--fresh-final-profile-review-5`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 5 returned `CHANGES_REQUIRED`. The parent
accepted all three Required findings and remediated them without widening the
slice:

1. descriptor decoding now requires every explicitly nullable closed-record
   member to be present before typed decoding: `role_ref`,
   `lifecycle_policy_ref`, `intake_definition_ref`,
   `requiredness.condition_ref`, and dependency `target_contract_ref`;
2. binding-shape traversal now proves each root, referenced, and intermediate
   parent satisfiable before descending, so contradictory parent cardinality
   or required-property constraints cannot admit an unreachable terminal; and
3. Stage 5 validates the closed syntactic shape of all eleven authored profile
   fields before retaining them for later producers, so malformed nested field
   values win before the Stage 6 inheritance-cycle boundary.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed 137 unit/integration tests plus doc tests.
New focused cases prove:

- omission of each explicitly nullable descriptor member refuses with the
  exact typed syntax error before defaults can collapse omission into `null`;
- unsatisfiable root, referenced, and intermediate binding parents refuse even
  when the selected terminal is independently satisfiable; and
- malformed values in each of all eleven authored profile fields refuse at
  Stage 5 before a simultaneous Stage 6 self-cycle, with exact field location.

All required commands passed in one successful fail-fast replay. The raw
2,259-line log is `/tmp/hcm-1.2-final-proof-round-6.log` with SHA-256
`de856dff2f18a252d40e7bad38082c4068880e4d686b4885de9506d266c9b429`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 137 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 849.2 KiB uncompressed, 148.2 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 120 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 71 paths, no sibling slice or fixed product path |

## Exact package and scope evidence

`package-definition-manifest.json` remains byte-current with SHA-256
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

GitNexus impact analysis preceded the existing-symbol edits. The current index
reported `MEDIUM` for `ArtifactInstanceRegistry::resolve`, `LOW` for
`binding_shape`, and `MEDIUM` for `parse_profile_source`; no `HIGH` or
`CRITICAL` blast radius was present. Final staged change detection remains
required before the primary implementation commit.

## Review-6 gate

This wall records no Review-6 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, all prior dispatches, package manifest, and
all six proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
