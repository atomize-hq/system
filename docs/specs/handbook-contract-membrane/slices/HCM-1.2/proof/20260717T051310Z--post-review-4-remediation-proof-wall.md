# HCM-1.2 Post-Review-4 Remediation Proof Wall

**Captured:** 2026-07-17T05:13:10Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T043237Z--HCM-1-2--fresh-final-profile-review-4`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 4 returned `CHANGES_REQUIRED`. The parent
accepted all three Required findings and remediated them without widening the
slice:

1. every compile-time package-owned exact ref is now source-reserved to its
   immutable built-in bytes before any repository read; all ten typed source
   classes are covered, and the exact shipped-root catalog/instance/path/
   label/requiredness drift matrix refuses at `definition_source`;
2. retained schema binding-shape queries now refuse contradictory supported
   terminals, including reversed cardinality bounds, impossible required
   closed-object members, impossible required array members/contains clauses,
   and terminal semantic negation/constant/enumeration constraints whose
   satisfiability cannot be proven; and
3. the resolver now preserves the literal ten-stage fail-fast boundary and
   frozen Stage 8 topology. Structural schema closure completes before
   deferred schema fingerprint comparison, while the comparisons occur after
   stable-role validation and before validator, capability, kind, condition,
   vocabulary, policy, stack, descriptor, and root-to-leaf authored-profile
   producers. Compound-invalid fixtures assert each adjacent stage and
   producer boundary, exact error kind/location, reversed source order where
   relevant, and closure-before-stale-fingerprint behavior.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed 134 unit/integration tests plus doc tests.
New focused cases prove:

- repository sources cannot bind any package-owned profile, stable-role,
  schema, kind, validator, capability, condition, vocabulary, Context
  Resolution policy, or Context Resolution stack exact ref;
- all nine shipped catalog/instance drift classes refuse before repository
  bytes are opened;
- object, array, and string binding terminals with contradictory supported
  constraints refuse deterministically without rejecting the bounded regex or
  uniqueness constraints used by the shipped schemas;
- stages 1 through 10 preserve exact first-error precedence in compound
  fixtures, including forward/reverse source order; and
- Stage 8 preserves stable -> schema -> validator -> capability -> kind ->
  condition -> vocabulary -> policy -> stack -> descriptor -> authored profile
  order, with structural closure winning before any stale schema/profile
  fingerprint and authored profiles validating root-to-leaf.

All required commands passed in one successful fail-fast replay. The raw
2,253-line log is `/tmp/hcm-1.2-final-proof-round-5.log` with SHA-256
`8887bbe04d2bec97a99f809d3468a313207619d69799cb0f41851022b2552b66`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; 134 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 838.5 KiB uncompressed, 146.8 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 119 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 69 paths, no sibling slice or fixed product path |

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

GitNexus impact analysis preceded the existing-symbol edits. The stale index
returned `LOW` for `AuthoredSchemaRegistryEntry::resolve` and `UNKNOWN`, rather
than a false-safe zero, for the new HCM-1.2 resolver/admission surfaces. Final
staged change detection remains required before the primary implementation
commit.

## Review-5 gate

This wall records no Review-5 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, all prior dispatches, package manifest, and
all five proof walls. A different fresh isolated built-in `default` reviewer
must return findings first. Any valid Critical or Required finding requires
another bounded parent remediation, full proof replay, a new dispatch, and
another different fresh reviewer. No exact-subject byte may change after final
`CLEAN`.
