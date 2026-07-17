# HCM-1.3 Future Implementation Checklist

This checklist is intentionally unchecked. Planning creation, review, proof,
and closeout are recorded in planning proof/handoff evidence, not here. A
future separately selected HCM-1.3 implementation session owns every item.

## Entry and authority

- [ ] Verify branch, exact planning-closeout entry HEAD, and clean worktree.
- [ ] Validate handoff/ledger parity and all three validator modes.
- [ ] Verify HCM-1.1/HCM-1.2 commits, fingerprints, ancestry, and package proof.
- [ ] Re-read live control pack, prior source/tests/assets/APIs, repo rules, and required skills.
- [ ] Refresh GitNexus once; record query/context/impact evidence and any index stop.
- [ ] Warn on every HIGH/CRITICAL surface before proceeding.
- [ ] Reproduce and compare the live 29-file fixed-consumer/facade set exactly with the SPEC ledger.

## TDD increment 1 — API and shipped membership

- [ ] Add the failing registry import/API-shape test and import all four public value types.
- [ ] Add the failing literal six-kind/three-instance shipped-membership test.
- [ ] Capture red output before production code.
- [ ] Implement only identity/membership containers, registry signatures, and `lib.rs` export; do not add value accessors or bind closure/dependencies/validation yet.
- [ ] Capture focused green and refactor-green output.

## TDD increment 2 — Data-driven closure and custom IDs

- [ ] Add the failing compile/API-shape assertions for every frozen value-type accessor signature.
- [ ] Add failing literal shipped kind/schema/fingerprint assertions.
- [ ] Add failing literal role/capability/validator metadata assertions.
- [ ] Add failing literal path/requiredness/condition/dependency assertions.
- [ ] Add an explicit custom kind plus custom artifact-instance fixture.
- [ ] Prove custom IDs require no enum, CLI, template, filename, or renderer change.
- [ ] Implement only non-dependency kind/instance closure fields and accessors, then rerun focused proof.

## TDD increment 3 — Dependencies and ordering

- [ ] Add instance-dependency provider test.
- [ ] Add `exactly_one` capability-provider test.
- [ ] Add multi-provider `at_least_one` test.
- [ ] Add multi-level and lexical tie-break test.
- [ ] Assert authored dependency order separately from sorted provider order.
- [ ] Assert exact providers-before-consumers topological order.
- [ ] Prove missing target/contract/provider, cardinality mismatch, and cycle refusals.
- [ ] Only after red evidence, implement provider expansion and dependency ordering.

## TDD increment 4 — Structural validation

- [ ] Add failing unknown-instance typed-error test.
- [ ] Add valid bound-kind structural-validation test.
- [ ] Add invalid and wrong-kind structural-validation tests.
- [ ] Preserve exact ordered `StructuralValidationError` values.
- [ ] Prove semantic validator profiles remain metadata and are not executed.

## Security and boundaries

- [ ] Prove 64/65 profile-source boundary.
- [ ] Prove 512/513 total-binding boundary.
- [ ] Prove 32/33 allowed-root boundary.
- [ ] Prove 128/129 schema-document boundary.
- [ ] Prove 32/33 ancestry and schema-reference-depth boundaries.
- [ ] Prove 1024/1025-byte and 64/65-component path boundaries.
- [ ] Prove 1-MiB and 8-MiB sentinel-byte boundaries.
- [ ] Prove all path escape classes refuse before registry construction.
- [ ] Prove symlink, non-regular source, race, and unbounded-error regressions.
- [ ] Prove source, identity, fingerprint, duplicate, conflict, compatibility, requiredness, provider, and cycle refusals.
- [ ] Confirm no new artifact/capability/dependency/count limit was introduced.

## Determinism and regressions

- [ ] Prove equivalent request/source permutations yield identical registry projections.
- [ ] Run `cargo fmt --all -- --check`.
- [ ] Run focused HCM-1.3 test.
- [ ] Run every live HCM-1.1 focused integration target.
- [ ] Run profile-selection and every live HCM-1.2 focused target.
- [ ] Run `cargo test -p handbook-engine`.
- [ ] Run `cargo test --workspace --all-targets`.
- [ ] Run `cargo clippy --workspace --all-targets -- -D warnings`.
- [ ] Run and record established Windows target proof.

## Package and scope proof

- [ ] Build the `handbook-engine` package archive.
- [ ] List literal sorted filesystem and archive definition members.
- [ ] Compare both member sets exactly with the HCM-1.2 29-member manifest.
- [ ] Compare SHA-256 and byte size for every definition member.
- [ ] Prove HCM-1.3 fixtures are not package-owned definitions.
- [ ] Build/check from the packaged artifact.
- [ ] Compare changed files for exact equality with the allowed-file set.
- [ ] Scan for forbidden enum/fixed-table/product/adoption/later-phase changes.
- [ ] Run GitNexus change detection and inspect affected symbols/flows.
- [ ] Run `git diff --check` and exact staged-diff inspection.

## Control pack, review, and closeout

- [ ] Apply only the permitted narrow `03`/`04`/`06` classification updates.
- [ ] Keep all downstream adoption/execution/release gates open.
- [ ] Build the complete immutable implementation proof wall.
- [ ] Fingerprint the exact complete subject including every coupled control-pack byte.
- [ ] Dispatch a fresh isolated built-in `default` reviewer.
- [ ] Remediate every valid actionable finding and rerun invalidated proof.
- [ ] Use a different fresh reviewer and new fingerprint after each remediation.
- [ ] Obtain final fresh `CLEAN`.
- [ ] Freeze reviewed bytes and verify the final subject fingerprint.
- [ ] Commit the exact reviewed subject and review/proof evidence first.
- [ ] Run staged GitNexus detection, cached diff check, and cached diff inspection.
- [ ] Create the parent v1.2 implementation closeout separately.
- [ ] Rebuild the deterministic ledger and pass all three validator modes.
- [ ] Confirm `handoffs/ledger.jsonl` is the only existing handoff-control file changed.
- [ ] Commit only closeout record plus ledger in the second commit.
- [ ] Name HCM-1.4 planning as next without creating, selecting, or beginning it.
- [ ] Stop after HCM-1.3 implementation closeout.

## Permanent stop checks

- [ ] Stop if any required file or symbol falls outside the SPEC allowlist.
- [ ] Stop if `resolve_profile_selection` or `ArtifactInstanceRegistry::resolve` would change.
- [ ] Stop if setup, doctor, compiler, flow, CLI, Cargo, or definitions would change.
- [ ] Stop if content IO, semantic execution, conditions, lifecycle, intake, renderers, Projection, vocabulary, or Context Resolution would execute.
- [ ] Stop if a new schema, ID, fingerprint, default, compatibility, migration, fallback, selector, or limit must be invented.
- [ ] Stop if package/scope/full-workspace/Windows proof is not honestly complete.
- [ ] Stop if an independent fresh reviewer cannot return `CLEAN`.
