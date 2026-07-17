# HCM-1.2 Post-Review-6 Remediation Proof Wall

**Captured:** 2026-07-17T06:05:01Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T054102Z--HCM-1-2--fresh-final-profile-review-6`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 6 returned `CHANGES_REQUIRED`. The parent
accepted both Required findings and remediated them without widening the
slice:

1. binding-shape satisfiability now covers every selected root, referenced,
   intermediate, and terminal schema location plus every transitively required
   child. It follows exact admitted local references under the 32-edge ceiling,
   detects false and contradictory required children, applies transitive
   `dependentRequired`, validates required names against `propertyNames`, and
   applies Draft 2020-12's default `minContains = 1` whenever `contains` is
   present; and
2. descriptor record identity failures now return `UnsupportedRecord` at the
   exact `schema_id` or `schema_version` field, while each non-empty later-owned
   lifecycle, intake, renderer, Projection, or overlay selection remains an
   `UnsupportedDependency` at its exact field. Non-empty extensions remain an
   unsupported record rather than a dependency selection.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed 140 unit/integration tests plus doc tests.
New focused cases prove:

- required false and contradictory children refuse at root, referenced-root,
  intermediate, and referenced-child locations while the complete shipped
  constitutional schema and its bounded patterns/unique arrays remain valid;
- impossible transitive `dependentRequired` and `propertyNames` constraints
  refuse on closed binding parents;
- an array with `contains`, omitted `minContains`, and `maxContains: 0` refuses
  under the Draft 2020-12 default rather than receiving a vacuous shape; and
- mutated descriptor `schema_id` and `schema_version` have exact
  `UnsupportedRecord` locations distinct from an exact-field
  `UnsupportedDependency` for a later-owned selection.

All required commands passed in one successful fail-fast replay. The raw
2,265-line log is `/tmp/hcm-1.2-final-proof-round-7.log` with SHA-256
`7e2201911940b3438998db452912c7190ea1d06565fdeb031ffc223370f3e6b7`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 140 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 866.6 KiB uncompressed, 150.2 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 121 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 73 paths, no sibling slice or fixed product path |

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
reported `LOW` for `binding_shape` and descriptor error categorization; the new
satisfiability helper was not yet indexed and therefore remained `UNKNOWN`
rather than falsely safe. No `HIGH` or `CRITICAL` blast radius was present.
Final staged change detection remains required before the primary
implementation commit.

## Review-7 gate

This wall records no Review-7 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, all prior dispatches, package manifest, and
all seven proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
