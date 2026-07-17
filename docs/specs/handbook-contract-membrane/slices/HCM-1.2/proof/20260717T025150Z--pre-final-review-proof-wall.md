# HCM-1.2 Pre-Final-Review Proof Wall

**Captured:** 2026-07-17T02:51:50Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Implementation lineage HEAD before final remediation:** `c964a85134f798737849d2fb29ccde6b18553489`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Result and classification ceiling

The implementation supplies the additive `handbook-engine` HCM-1.2 profile,
artifact-instance, exact HCM-0.6 shipped-default, typed definition producer,
profile layering, and explicit profile-selection boundary. It packages the
literal six-kind catalog and three selected instances and recomputes the exact
typed source/fingerprint closure. It does not adopt the boundary in canonical
layout, setup, doctor, flow, compiler, CLI, SDK, Tauri, Substrate, authoring,
materialization, condition evaluation, vocabulary application, Context
Resolution execution, intake, lifecycle, renderer, Projection, publication, or
downstream product paths.

The classification ceiling is `BoundaryLanded` for exact additive definition,
profile/descriptor selection, shipped-profile data, vocabulary metadata, and
Context Resolution metadata only. `PG-PROFILE-01`, `PG-ARTIFACT-01`,
`PG-KIND-01`, and `PG-KIND-02` remain open. Product adoption remains
`TargetOnly`.

## Incremental implementation lineage

| Commit | Green increment |
|---|---|
| `988de49` | request/source envelope and exact `SymbolicId` admission |
| `59c0fe9` | request-wide schema closure budget and closed binding-shape query |
| `bc87f7f` | Project Authority schema closure |
| `af830d3` | Project and Environment Context schema closures |
| `790a435` | Work Specification and Decision Record schema closures |
| `51e4fbc` | Risk Record schema closure |
| `49e93b8` | constitutional capability and nine-rule validator metadata |
| `cac1697` | selected shipped-root kind definitions |
| `1a1c57a` | unselected Work/Decision/Risk kind definitions |
| `d7e158e` | managed-operational-surface condition metadata |
| `14d533c` | shipped vocabulary metadata |
| `5cd34e4` | Context Resolution policy metadata |
| `106965a` | shipped Context Resolution stack metadata |
| `f5d1e8e` | artifact-instance descriptors and registry |
| `8d5f239` | authored profile parsing and eleven-field layering |
| `69bb6f2` | exact typed-source profile selection and closure fingerprint |
| `c964a85` | exact shipped-root profile and repository replacement fixture |

Every increment used focused RED/green tests, affected verification,
formatting, scoped diff inspection, staged GitNexus change detection, and an
atomic commit. The final pre-review hardening remains in the complete reviewed
subject rather than an extra unreviewed increment.

## Focused positive, boundary, negative, and security proof

`cargo test -p handbook-engine --all-features` passed 116 unit/integration
tests plus doc tests. The exact HCM-1.2 suites prove:

- all request, profile-source, binding, root, path, source-byte, aggregate-byte,
  schema-document, reference-depth, ancestry, and source-count N/N+1 ceilings;
- exact `SymbolicId` and full canonical SemVer grammars without trim, case,
  Unicode-normalization, range, or latest repair;
- all six closed Draft 2020-12 schemas, stable-ref grammar, exact set semantics,
  secret-surface refusal, and no content fingerprint/normalization output;
- schema-aware compatibility for every one of the nine constitutional bindings
  and acyclic validator -> capability -> kind -> profile fingerprint
  propagation;
- literal exact condition, vocabulary, three policy, stack, kind, descriptor,
  and shipped-profile records with forged/stale/changed/wrong-class refusal;
- dependency namespace, contract, provider-cardinality, duplicate, cycle,
  requiredness/condition, unique-path, and exactly-one-constitutional-root
  checks;
- one bounded acyclic shipped -> named -> repository ancestry with all eleven
  fields using omission/inheritance and present/replace-whole semantics;
- exact typed source accounting for winning and shadowed selected-ancestor
  dependencies, with unrelated, missing, duplicate, conflicting, ambient, and
  unallowlisted sources refused deterministically;
- deterministic repeated and source-permuted fingerprints and the exact
  six-kind/three-instance HCM-0.6 set; and
- safe retained-handle repository reads plus absolute, traversal, symlink,
  non-regular, URI-like, non-normalized, and oversize source/path refusal.

The final hardening additionally binds dependency and later-owned descriptor
bytes into the descriptor-registry fingerprint, validates the complete
dependency graph, validates every selected-ancestor replacement rather than
only the winning field, requires literal constitutional metadata, admits only
canonical StableRef SemVer, and reuses the existing exact normalized
repo-relative path grammar for instance paths.

## Required replay commands and raw results

All commands below exited `0` in one fail-fast run. The raw 2,281-line stream is
`/tmp/hcm-1.2-final-proof-round-1.log`, with SHA-256
`9bc86aa284e742d178ce3584485bbd9b4cfc58ef6269e757d0948fd0b8d7cf23`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 116 tests plus doc tests, 0 failed |
| `cargo test --workspace --all-features` | PASS; every workspace suite and doc test, 0 failed |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; exact feature graph inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 90 files, 704.9 KiB uncompressed, 128.0 KiB compressed |
| exact package/tree definition equality | PASS; 29 literal members, paths, sizes, SHA-256 values, and bytes equal |
| `python3 tools/check_archive_boundary.py --self-test` | PASS; injected runtime archive reference rejected |
| `python3 tools/check_archive_boundary.py` | PASS; no supported-runtime archive references |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 115 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |

The immutable review dispatch increments the current-dispatch count after this
capture. All validator modes must rerun after dispatch creation, after final
clean byte replay, and after mechanical closeout.

## Exact package and dependency proof

`package-definition-manifest.json` enumerates all 29 files beneath the engine
package `definitions/` tree with literal repo-relative path, byte size, and
SHA-256. Its SHA-256 is
`feb124668a706fedd494bc2dd1211f1d1870bef8aaad0bcaa7cebb0a37e75a0b`.
The manifest generator compared the tree map to the `.crate` archive map and
asserted literal key-set, size, hash, and byte equality; both sides contained
exactly 29 members.

`Cargo.toml` and `Cargo.lock` are byte-identical to the entry baseline. The
feature-tree and manifest diff add no network, TLS, async resolver, executable
hook, or other dependency. No source scan, ambient directory discovery, remote
fetch, executable validator, condition evaluator, policy matcher, escalation,
promotion, renderer, lifecycle, intake, or Projection behavior is present.

## Scope and preserved product boundary

The baseline-to-subject diff contains only packet-authorized additive engine
definition/profile modules, the smallest HCM-1.1 schema/kind integration,
package assets, focused tests/fixtures, HCM-1.2 task/proof evidence, and bounded
crosswalk/proof-ledger evidence. No sibling slice byte changed.

The scope assertion found no diff in current fixed product modules, canonical
artifact/layout code, CLI, SDK, Tauri, Substrate, setup, doctor, flow, compiler,
authoring, baseline-validation, or dependency manifests. The product still
uses its fixed current baseline; this slice does not activate setup/doctor or
begin HCM-1.3.

GitNexus impact analysis preceded edits to indexed existing symbols. The
initial shared loader surface was `MEDIUM` because eight test callers depended
on it; audited struct/validation surfaces were `LOW`. The stale index could not
resolve newly added symbols and therefore reported `UNKNOWN`, not false-safe
zero, for their final hardening. Every staged increment reported only the
expected additive boundary. Final staged detection is required before the
primary implementation commit.

## Final review gate

This proof records no reviewer conclusion. The immutable dispatch must bind
every implementation, asset, fixture, task-status, crosswalk, gate-evidence,
package-manifest, and proof-wall byte using
`repo-path-null-sha256-newline-v1`. A fresh isolated read-only built-in
`default` reviewer must report findings first. Any valid Critical or Required
finding requires bounded parent remediation, complete proof replay, a new
complete manifest, and a different fresh reviewer. No reviewed subject byte
may change after final `CLEAN`.
