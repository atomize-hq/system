# HCM-1.2 Post-Review-11 Remediation Proof Wall

**Captured:** 2026-07-17T08:03:39Z  
**Phase / slice:** `HCM-1` / `HCM-1.2`  
**Entry handoff:** `20260717T012715Z--HCM-1-2--orchestration--implementation-packet-approved`  
**Baseline HEAD:** `83ccaabcaa2481b33bc58a69736a285b9c714e00`  
**Prior review dispatch:** `20260717T073653Z--HCM-1-2--fresh-final-profile-review-11`  
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.2`

## Review result and bounded remediation

Fresh isolated default Review 11 returned `BLOCKED` because its otherwise
byte-reproducible 89-entry manifest omitted the modified
`crates/engine/tests/artifact_kind_registry.rs` proof byte. It also reported
two valid Required findings. The parent accepted the manifest-integrity stop
and both findings, remediated them without widening the slice, and will rebuild
the next manifest from live baseline-to-subject truth. The implementation now:

- includes `unevaluatedItems` in joint array witness proof whenever a positive
  minimum item cardinality requires actual item admission;
- refuses `minItems: 1` with `unevaluatedItems: false` when no applicator can
  evaluate/admit the required item, while retaining the exact empty-array
  boundary;
- validates all five later-owned fields on every selected ancestry source, so
  a child cannot hide a forbidden parent projection, posture, dock, adapter,
  or extension value;
- performs that ancestry-wide validation at the start of Stage 10, after
  authored-profile fingerprint closure and Stage-9 literal source-use equality,
  preserving the frozen ten-stage fail-fast order; and
- continues to include every source's empty/null later-owned field bytes in its
  authored profile fingerprint definition.

The remediation did not activate setup, doctor, flow, compiler, CLI, SDK,
Tauri, Substrate, condition evaluation, vocabulary application, Context
Resolution execution, renderer, intake, lifecycle, Projection, publication,
or HCM-1.3 behavior. The classification ceiling remains `BoundaryLanded` for
the additive definition/profile boundary only. `PG-PROFILE-01`,
`PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.

## Focused and full proof

The remediated engine suite passed exactly **155** unit/integration tests plus
doc tests across all 26 engine test binaries. New proof covers:

- required `unevaluatedItems: false` arrays at a direct terminal and through a
  transitively required reference, plus the zero-item positive boundary;
- a nonempty parent and explicit empty/null child for each of
  `projection_catalog_refs`, `posture_evaluation_policy`,
  `dock_requirement_refs`, `adapter_overlay_refs`, and `extensions`, in both
  source orders;
- explicit empty values at both ancestry levels and inherited empty values in
  both source orders; and
- the existing compound Stage-9-unreferenced-plus-Stage-10-later-owned fixture,
  proving `UnreferencedSource` still wins before the later-owned refusal.

All required commands passed in one successful fail-fast replay after an
initial replay caught and drove correction of the Stage-9/Stage-10 ordering
regression. The final raw 2,292-line log is
`/tmp/hcm-1.2-final-proof-round-12.log` with SHA-256
`b734d3689ea8e6e8087df264302b8840882dcd8bc265885d6e32c2d15b68b401`.

| Command | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings` | PASS |
| `cargo test -p handbook-engine --all-features` | PASS; exactly 155 tests plus doc tests |
| `cargo test --workspace --all-features` | PASS; all workspace and doc tests |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| `cargo tree -p handbook-engine -e features` | PASS; dependency/feature tree inspected |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 91 files, 912.9 KiB uncompressed, 155.4 KiB compressed |
| exact package/tree/manifest equality | PASS; literal 29-member path, size, hash, and byte equality |
| `python3 tools/check_archive_boundary.py --self-test` | PASS |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS; 36 records, 126 current dispatches, 36 ledger entries |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| dependency-manifest baseline equality | PASS |
| bounded baseline-to-subject scope assertion | PASS; 84 paths, no sibling slice or fixed product path |

## Exact package, manifest, and scope evidence

`package-definition-manifest.json` remains byte-current with SHA-256
`60eb15eb1651b124829982cbebd16159988d4e26feeceda18a35930eba47e7e7`.
It lists all 29 definition members literally. The proof replay asserted exact
set equality across that manifest, the live `definitions/` tree, and the
packaged `.crate`, then compared every member's size, SHA-256, and bytes.

`Cargo.toml`, `Cargo.lock`, and `crates/engine/Cargo.toml` remain byte-identical
to the entry baseline. The dependency tree adds no network, TLS, async,
resolver, remote fetch, or executable-hook dependency. The next review
manifest must include every baseline-to-subject modified or added path that is
part of the HCM-1.2 implementation/proof subject, including the previously
omitted artifact-kind registry test.

GitNexus impact analysis preceded the existing-symbol edits and reported `LOW`
for `schema_fragment_is_conservatively_satisfiable`, `MEDIUM` for
`resolve_profile_selection`, and `LOW` for
`validate_authored_profile_fingerprints`; no `HIGH` or `CRITICAL` symbol blast
radius was present. Final staged change detection remains required before the
primary implementation commit.

## Review-12 gate

This wall records no Review-12 conclusion. A new immutable complete-subject
dispatch must bind the remediated implementation, assets, fixtures, task
status, crosswalk, proof ledger, every baseline-to-subject test byte, all prior
dispatches, package manifest, and all twelve proof walls. A different fresh
isolated built-in `default` reviewer must return findings first. Any valid
Critical or Required finding requires another bounded parent remediation, full
proof replay, a new dispatch, and another different fresh reviewer. No
exact-subject byte may change after final `CLEAN`.
