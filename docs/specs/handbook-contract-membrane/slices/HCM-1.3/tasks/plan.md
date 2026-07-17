# HCM-1.3 Future Implementation Plan

## Authority and session fence

This plan executes only in a separately selected top-level HCM-1.3
implementation session after the planning subject and planning closeout are
review-clean and committed. The planning session must stop after closeout. The
future implementation consumes `../SPEC.md` as controlling authority; a plan
summary never overrides the specification.

## Entry gate

1. Confirm branch `feat/handbook-contract-membrane`, the exact implementation
   entry HEAD named by the selected planning closeout, and a clean worktree.
2. Validate the selected handoff against both the deterministic ledger and its
   immutable record. Run base, v1-admission self-test, and orchestration-contract
   self-test modes.
3. Verify HCM-1.1/HCM-1.2 primary and closeout commits are ancestors, and verify
   their reviewed subject fingerprints and package proof.
4. Re-read live control-pack authority, HCM-1.2 source/tests/definitions/public
   APIs/package manifest, this packet, repo instructions, and all required
   skills.
5. Refresh GitNexus once with a supported command. Query the selected-profile
   artifact registry flow, inspect context for every planned existing symbol,
   and run upstream impact before any existing symbol edit. Record and warn on
   HIGH/CRITICAL surfaces.
6. Run the SPEC's exact direct-surface/facade scan, enumerate the live 29-file
   fixed-consumer inventory, and compare it for literal set equality with the SPEC
   ledger. Stop on drift rather than silently extending scope.

## Task 1 — Red API and shipped-membership tests

Allowed writes:

- new `crates/engine/tests/hcm_1_3_artifact_registry.rs`; and
- new `crates/engine/tests/fixtures/hcm_1_3_registry/**` only as needed.

Write a compile/API test for the frozen owner types and entry point. Resolve
the exact shipped profile through `resolve_profile_selection`, call
`ResolvedArtifactRegistry::from_profile`, and assert literal six-kind and
three-instance sets plus the exact profile identity/fingerprint and stable-role
registry identity/fingerprint. Import the four public value types and type-check
the registry method signatures; the value-type accessor signatures join the
Task 3 red compile/API test so no stub accessor implementation precedes its
behavioral test. Capture the expected red failure before adding production
code.

Stop if the test requires a repository-root argument, fixed enum, default
profile, implicit source, current layout, or product crate.

## Task 2 — Minimal registry construction

Allowed writes:

- new `crates/engine/src/artifact_registry.rs`;
- `crates/engine/src/lib.rs` export only; and
- `crates/engine/src/artifact_instance.rs` only for a read-only deterministic
  iterator if the existing `ids()`/`instance()` surface is demonstrably
  insufficient.

Implement only the surface exercised by Task 1: public type/method signatures,
selected profile and stable-role registry identities/fingerprints, literal
lexical kind refs, literal lexical instance IDs, lookup containers sufficient
to represent those identities, and final kind/instance set equality. Value
objects may carry only the identity fields needed to compile the frozen
registry lookup signatures; their accessor impl blocks are not added yet. Task
2 must not bind schema, role definitions, capability contracts, validator
profiles, instance data fields, dependency providers, dependency order, or
structural validation. Use existing HCM-1.2 fingerprints and errors.
Do not add input, content IO, discovery, defaults, conversion, mutation, or a
second fingerprint.

Run only the focused test until green. Then refactor without changing behavior
and rerun it.

## Task 3 — Red data-driven closure tests

Extend the focused compile/API test to type-check every frozen accessor on all
four value types, and add runtime tests that assert literal shipped data:

- every kind/schema exact ref and fingerprint;
- each instance ID, kind, role, label, path, requiredness, condition ref, and
  authored dependency;
- constitutional capability contract, binding map, validator exact ref, and
  fingerprints; and
- explicit null/empty later-owned fields.

Add the custom fixture through explicit `ProfileSelectionRequest` sources. It
must add one custom kind and one custom instance ID without changing enum,
command, filename, renderer, or package-owned definitions. Capture red before
implementing the exact schema/role/capability/validator and descriptor-field
binding views frozen in the SPEC. This increment fills kind and instance value
objects and their non-dependency accessors; it does not expand dependency
providers, compute dependency order, or implement structural validation. Then
make only this increment green.

## Task 4 — Red dependency-order tests

Add fixture cases for:

- exact instance dependency;
- capability dependency with `exactly_one`;
- capability dependency with `at_least_one` and multiple providers;
- independent lexical tie-breaks; and
- a multi-level dependency chain.

Assert authored order separately from literal sorted provider sets and literal
providers-before-consumers topological order. Add private fail-closed unit cases
for missing internal targets/contracts/providers and cycles only if public
HCM-1.2 APIs cannot create those inconsistent states. Test-only builders must
not escape the module. Only after this red evidence may production code expand
providers, populate `ResolvedArtifactDependency::provider_ids`, or compute
`dependency_order()`.

## Task 5 — Red structural-validation tests

Assert `validate_json`:

1. returns `UnknownArtifactInstance` before schema routing for an unknown ID;
2. accepts valid JSON only through the selected instance's exact bound kind;
3. returns existing ordered structural errors inside `Structural` for invalid
   JSON; and
4. exposes semantic validator profiles as metadata without executing them.

Make the smallest implementation change. Do not add YAML content parsing,
repository reads, semantic callbacks, commands, dynamic libraries, condition
evaluation, renderer lookup, or overlays.

## Task 6 — Security and N/N+1 replay

Run the live HCM-1.2 positive and negative suites, naming exact tests that prove
the existing 64/65 sources, 512/513 bindings, 32/33 roots, 128/129 schema
documents, 32/33 ancestry, 32/33 schema-reference depth, 1024/1025 path bytes,
64/65 path components, 1-MiB sentinel, and 8-MiB sentinel boundaries.

Add one integration sentinel showing an admitted N case reaches registry
construction and the corresponding N+1 case refuses before it. Do not add a
new descriptor/count limit. Prove path escapes, symlink/non-regular sources,
duplicates/conflicts, identity/fingerprint mismatch, unsupported compatibility,
requiredness errors, provider cardinality, and cycles remain fail-closed.

## Task 7 — Determinism and regression

Permute every explicitly unordered request collection and fixture source order.
Serialize a test-only normalized registry projection and compare literal bytes,
membership, provider order, validation routing, and resolved-profile
fingerprint. Then run:

1. `cargo fmt --all -- --check`;
2. focused HCM-1.3 test;
3. all live HCM-1.1 focused integration targets;
4. `cargo test -p handbook-engine --test profile_selection` and every live
   `hcm_1_2_*` target;
5. `cargo test -p handbook-engine`;
6. `cargo test --workspace --all-targets`;
7. `cargo clippy --workspace --all-targets -- -D warnings`; and
8. the established Windows target proof.

Record exact commands, versions, target, exit status, and output artifacts.
No skipped or unavailable required proof may be rewritten as pass.

## Task 8 — Package and scope proof

1. Build/package `handbook-engine` using the established repository command.
2. Generate literal sorted archive and filesystem definition-member manifests.
3. Compare each for exact set equality to the reviewed HCM-1.2 29-member
   manifest; compare SHA-256 and byte size per member.
4. Prove no HCM-1.3 test fixture is admitted as a package definition.
5. Build/check the packaged artifact.
6. Compare the literal changed-file set with the SPEC allowlist.
7. Run forbidden-scope scans across the diff for enum matches, fixed tables,
   product crates, setup/doctor, YAML content, execution policies, dynamic
   dispatch, compatibility/fallback/range/latest/mutation behavior, Cargo, and
   definition assets.
8. Run GitNexus change detection, `git diff --check`, and inspect the exact
   staged diff. Stop on any unexpected symbol/flow/file/member.

## Task 9 — Narrow control-pack update

Only after runtime proof is green, update the permitted lines in `03`, `04`,
and `06`:

- mark the selected-profile artifact registry owner `BoundaryLanded`;
- record only the narrow registry/data-driven/custom-instance/trusted-path
  evidence;
- keep setup/doctor, real-path content, semantic execution, canonical YAML,
  intake/lifecycle/render/Projection, downstream publication, and all dependent
  gates open; and
- name HCM-1.4 planning as next without creating or starting it.

Run cross-document and forbidden-status scans. No unrelated cleanup.

## Task 10 — Proof wall and fresh review loop

Create an immutable proof wall containing the exact evidence required by the
SPEC. Fingerprint the complete implementation subject using a sorted
path/NUL/SHA-256/newline manifest. Create one immutable internal dispatch to a
fresh isolated built-in `default` reviewer with read-only scope, exact subject,
authority refs, required skills, gates, and structured return contract.

For each valid actionable finding:

1. record the finding-to-change mapping;
2. remediate only within allowed scope;
3. rerun every invalidated proof;
4. create a new proof wall and new aggregate subject fingerprint; and
5. use a different fresh isolated `default` reviewer.

Continue until a fresh reviewer returns `CLEAN`. Do not mutate any reviewed
subject byte after final CLEAN. If a mandatory reviewer cannot be created,
stop honestly; do not self-approve or reuse reviewer context.

## Task 11 — Two-commit closeout

First commit only the exact final reviewed implementation subject and immutable
review/proof evidence. Before committing, run staged GitNexus change detection,
`git diff --cached --check`, exact cached diff inspection, and subject
fingerprint verification.

Then create one parent handoff v1.2 implementation closeout and rebuild the
deterministic ledger in a separate commit. The closeout must:

- bind the exact primary commit and final reviewed fingerprint;
- identify every review dispatch/result;
- classify HCM-1.3 complete at `BoundaryLanded` only;
- name the future HCM-1.4 planning packet path/selector;
- set `continue` or its schema-equivalent without activating work; and
- explicitly prohibit beginning HCM-1.4 in this session.

Validate all three handoff-validator modes, stage only the closeout record and
the exact existing
`docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl`, inspect the
cached diff, commit, report exact terminal state, and stop. This deterministic
rebuild is the sole allowed existing handoff-control-file change; any other
record rewrite, template/schema edit, or control-plane mutation is a scope
stop.

## Dependency order

```text
entry and authority proof
  -> GitNexus context/impact and fixed-consumer set equality
  -> red API/membership test
  -> minimal registry construction
  -> red data-driven/custom test
  -> red dependency-order test
  -> red structural-validation test
  -> HCM-1.2 security boundary replay
  -> determinism/full regression
  -> package/scope proof
  -> narrow control-pack classification
  -> exact proof wall
  -> fresh review/remediation loop to CLEAN
  -> reviewed-subject commit
  -> separate closeout/ledger commit
  -> stop
```

No task may begin before its predecessor's exit condition is proven.
