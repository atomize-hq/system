# HCM-1.3 Planning Remediation 1 Proof Wall

## Classification and lineage

- phase: `HCM-1`
- slice: `HCM-1.3`
- work class: planning-only remediation
- baseline: `c5733785fbd60b7d7a19318cb86058395a02e1c3`
- predecessor subject:
  `sha256:9d21fca4d103cd5e29fa3a6aa2e39128a655c1d819f55478b5502d4c0f664bc8`
- predecessor dispatch:
  `20260717T144006Z--HCM-1-3--fresh-planning-review-1`
- predecessor reviewer: `/root/hcm_1_3_planning_review_1`
- predecessor verdict: `CHANGES_REQUIRED`
- remediation timestamp: `20260717T145533Z`

The first planning proof wall and first review dispatch remain immutable
historical evidence. This wall supersedes only the invalidated PASS claims in
that candidate wall. It does not rewrite the prior review or claim final
approval. The next immutable dispatch owns the new exact subject manifest and
aggregate fingerprint; a different fresh isolated built-in `default` reviewer
must independently return `CLEAN`.

## Review 1 findings accepted

Review 1 returned zero Critical and three Required findings. The parent accepts
all three:

1. `HCM-1.3-R1-001`: the 23-file fixed-consumer scan was too narrow and the
   proof-wall rendering of that command was not byte-reproducible;
2. `HCM-1.3-R1-002`: Task 2 implemented closure and dependency behavior before
   the Task 3/4 red tests; and
3. `HCM-1.3-R1-003`: four public value types had narrative data lists but no
   frozen accessor signatures.

Each remediation is documentation-only and remains within the planning
authority. No Rust, Cargo, product, definition, handoff record, or ledger byte
changed.

## Remediation R1-001 — Complete reproducible consumer inventory

The SPEC now freezes the broader direct fixed-surface scan, not only enum/table
names. The exact Python script uses this one-line raw pattern:

```python
pattern = r"CanonicalArtifactKind|CANONICAL_ARTIFACT_ORDER|canonical_artifact_descriptors|CanonicalLayoutContract|baseline_artifact_validations|from_canonical_artifacts|CanonicalArtifactIdentity|CanonicalArtifact\b|ArtifactPresence|ArtifactManifest"
```

It executes `rg -l` over production `crates/**/*.rs`, excluding test
directories, and normalizes the resulting set lexically. The literal result is
28 files:

```text
crates/cli/src/doctor_rendering.rs
crates/cli/src/rendering.rs
crates/compiler/src/author/charter_shell.rs
crates/compiler/src/author/environment_inventory_shell.rs
crates/compiler/src/author/mod.rs
crates/compiler/src/author/project_context_shell.rs
crates/compiler/src/baseline_validation.rs
crates/compiler/src/blocker.rs
crates/compiler/src/doctor.rs
crates/compiler/src/doctor_shell.rs
crates/compiler/src/layout.rs
crates/compiler/src/lib.rs
crates/compiler/src/refusal.rs
crates/compiler/src/rendering/json.rs
crates/compiler/src/rendering/markdown.rs
crates/compiler/src/rendering/shared.rs
crates/compiler/src/resolver.rs
crates/compiler/src/setup.rs
crates/engine/src/artifact_manifest.rs
crates/engine/src/baseline_validation.rs
crates/engine/src/canonical_artifacts.rs
crates/engine/src/canonical_paths.rs
crates/engine/src/freshness.rs
crates/engine/src/lib.rs
crates/flow/src/budget.rs
crates/flow/src/packet_result.rs
crates/flow/src/resolver.rs
crates/pipeline/src/pipeline_handoff.rs
```

The parent compared this result against a separately literalized expected set,
not a count:

```text
PASS: literal 28-file set equality
```

The five previously omitted production consumers now have exact dispositions:

- `crates/cli/src/doctor_rendering.rs`: HCM-1.4 profile-aware doctor boundary,
  with CLI wording outside engine decisions;
- `crates/compiler/src/rendering/json.rs`: HCM-2 deterministic artifact-view
  migration; no dynamic renderer dispatch;
- `crates/compiler/src/resolver.rs`: HCM-2.1/HCM-3.5 flow adoption;
- `crates/flow/src/budget.rs`: HCM-2.1 content/flow pilot and HCM-3.5
  Resolution-aware flow adoption; and
- `crates/pipeline/src/pipeline_handoff.rs`: exact HCM-3.5 snapshot/packet/
  pipeline adoption boundary.

The SPEC, plan entry gate, and todo now require the reproducible 28-file set
equality. Future HCM-1.3 product edits remain prohibited.

## Remediation R1-002 — True red-before-production order

Task 2 is now limited to behavior covered by Task 1 red tests:

- public registry/type declarations and registry method signatures;
- selected profile and stable-role registry identities/fingerprints;
- literal lexical kind-ref and instance-ID membership;
- lookup containers sufficient for identity-only values; and
- exact kind/instance set equality.

Task 2 explicitly forbids schema, role-definition, capability-contract,
semantic-validator, descriptor-field, dependency-provider, dependency-order,
and structural-validation implementation.

Task 3 first adds compile/API red assertions for every public value-type
accessor and runtime red assertions for data-driven kind/instance closure. Only
then may non-dependency closure fields/accessors be implemented. Task 3
explicitly excludes dependency provider expansion/order and structural
validation.

Task 4 first adds red provider/cardinality/order cases. Only after those fail
may production code populate provider IDs and compute the providers-before-
consumers order. Task 5 separately owns red structural-validation behavior.
The todo repeats the same fences.

Source anchors were verified:

```text
plan: Task 2 "must not bind ..."
plan: Task 3 "does not expand dependency ..."
plan: Task 4 "Only after this red evidence ..."
todo: "Only after red evidence, implement provider expansion ..."
```

No later test can be made accidentally green by following an earlier task.

## Remediation R1-003 — Exact public value APIs

The SPEC now freezes conceptual `impl` blocks for every public value type.
Accessor counts are:

| Type | Frozen accessors | Key completeness |
|---|---:|---|
| `ResolvedArtifactKind` | 8 | identity/fingerprint, structural schema refs/fingerprints, roles, capabilities |
| `ResolvedArtifactInstance` | 15 | identity, kind/role/capabilities, label/path/requiredness/condition, dependencies, all later refs/extensions |
| `ResolvedArtifactCapability` | 7 | ID, contract/ref fingerprint, required bindings/cardinality, kind bindings, validator definitions |
| `ResolvedArtifactDependency` | 5 | target namespace/ID/contract/cardinality plus resolved providers |

`ResolvedArtifactRegistry` additionally exposes exact stable-role registry
identity/fingerprint. Every method name, return type, identity type, lookup
shape, slice/map borrowing rule, and allocation rule is frozen. Specifically:

- slices borrow immutable engine-owned `Vec` values in normative order;
- maps borrow immutable engine-owned `BTreeMap` values;
- registry lookups borrow the registry;
- key-list APIs allocate only borrowed-key vectors;
- capability semantic validators reuse the exact public
  `SemanticValidationProfileDefinition` value type and its existing accessors;
  and
- dependencies expose authored data plus lexically sorted provider IDs while
  their owning instance preserves authored dependency order.

A mechanical signature scan passed:

```text
PASS: kind 8 accessors
PASS: instance 15 accessors
PASS: capability 7 accessors
PASS: dependency 5 accessors
```

Task 1 imports all four value types and proves registry API shape. Task 3 owns
the red compile/API assertions for every value accessor immediately before
their implementation, preserving both interface freeze and TDD order.

## Replayed planning gates

After remediation:

- all candidate changes remain under
  `docs/specs/handbook-contract-membrane/`;
- `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` remain present;
- the future implementation todo remains entirely unchecked;
- the phase map still records HCM-1.1/HCM-1.2 landed dependency evidence,
  HCM-1.3 approval as the next boundary, and HCM-1.4 unauthorized;
- no Rust, Cargo, runtime, product, shipped definition, setup/doctor adoption,
  canonical YAML, behavior execution, or later-phase work changed;
- the HCM-1.2 owner API and HIGH-risk surface quarantines remain unchanged;
- exact error, deterministic ordering, path/security, N/N+1, regression,
  Windows, package set/hash/size, scope, review, and two-commit closeout
  contracts remain present; and
- final newlines and no trailing whitespace were verified across all current
  subject/evidence files.

All three handoff validator modes passed before Review 1. The next dispatch is
validated and replayed separately after this wall is hashed.

## Remediation verdict before re-review

| Gate | Result |
|---|---|
| R1-001 complete fixed-consumer set | PASS |
| R1-001 command reproducibility | PASS |
| R1-002 red-before-production order | PASS |
| R1-003 exact public value API | PASS |
| packet presence/internal consistency | PASS |
| cross-document status consistency | PASS |
| docs-only/allowed scope | PASS |
| non-goal preservation | PASS |
| independent re-review | PENDING new immutable dispatch |

The parent does not claim `CLEAN`. Any valid new or residual Required finding
requires another docs-only remediation, a new proof wall/fingerprint, and a
different fresh isolated reviewer.
