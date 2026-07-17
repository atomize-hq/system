# HCM-1.3 Implementation Proof Wall

**Captured:** 2026-07-17T18:06:28Z
**Phase / slice:** `HCM-1` / `HCM-1.3`
**Entry handoff:** `20260717T163627Z--HCM-1-3--orchestration--implementation-packet-approved`
**Entry HEAD:** `0c711d7dfbd0e27659128717cdcf1050482dfe45`
**Active packet:** `docs/specs/handbook-contract-membrane/slices/HCM-1.3/SPEC.md`

## Scope and implementation summary

The implementation is additive and engine-owned only:

- added `crates/engine/src/artifact_registry.rs` with immutable
  `ResolvedArtifactRegistry` construction from one already resolved
  `ResolvedInstanceProfile`;
- updated `crates/engine/src/lib.rs` only to declare/re-export the new owner
  module and public value types;
- added `crates/engine/tests/hcm_1_3_artifact_registry.rs` for shipped
  membership, data-driven fields, custom kind/instance registration,
  dependency providers/order, validation routing, and deterministic source
  permutation proof; and
- updated only the allowed HCM control-pack classification/proof files.

No Cargo manifest, lockfile, package-owned definition asset, compiler, flow,
setup, doctor, CLI, SDK, Tauri, Substrate, adapter, contract, dock, manifest,
freshness, renderer, or product-adoption file changed.

## Entry and dependency proof

- Selected v1.2 handoff:
  `docs/specs/handbook-contract-membrane/handoffs/records/20260717T163627Z--HCM-1-3--orchestration--implementation-packet-approved.json`.
- Handoff validators passed before implementation:
  - `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py`
    -> `validation passed: 38 records, 146 current dispatches, 8 legacy dispatches`;
  - `python3 ... --self-test-v1-admission` -> PASS;
  - `python3 ... --self-test-orchestration-contract` -> PASS.
- Git ancestry was verified for HCM-1.1
  `0bc51a9cc282581143a5b21f50162456aa32154c`, HCM-1.2
  `832716a66241bdcf86e2a82ffb3ae72680a7c2cd`, HCM-1.3 planning
  `ea2ee1eae2560e73885cb9365f06c90d61df342f`, and HCM-1.2 entry/planning
  `c5733785fbd60b7d7a19318cb86058395a02e1c3`.
- HCM-1.2 reviewed manifest replayed clean: 119 entries, aggregate
  `sha256:d120149e21831c2009d9708c71c335482a97a10d8d975b8246742ed07fe64c71`.
- HCM-1.3 reviewed planning manifest replayed clean: 15 entries, aggregate
  `sha256:3f08530f778febbeadcab5e59ef61db355290589d004af300f06608e8ca9d8d3`.

## GitNexus proof

`npx gitnexus analyze --index-only` was retried and passed before editing, then
passed again after implementation with `changed=1, added=2, deleted=0` and
`12,921 nodes | 23,639 edges | 346 clusters | 300 flows`.

Fresh upstream impact checks recorded the following blast-radius boundaries:

| Surface | Result |
|---|---|
| `resolve_profile_selection` | LOW; consumed unchanged |
| `CanonicalArtifactKind` enum | LOW; not edited/called by the registry |
| `ArtifactManifest::from_canonical_artifacts#2` | HIGH; explicitly warned and prohibited, not modified/called |
| `ArtifactInstanceRegistry::resolve#3` | LOW; consumed unchanged |
| `compute_freshness` | LOW; prohibited, not modified/called |
| `resolve_with_contract` | LOW; prohibited, not modified/called |
| `crates/engine/src/lib.rs` export-only target | UNKNOWN by symbol lookup; limited to module declaration/re-exports and checked by diff/detect |

Final staged GitNexus change detection over the seven-file subject reported
`changed_count: 143`, `changed_files: 7`, `affected_count: 0`, `risk_level:
low`, and no affected processes. The broad changed-symbol list is limited to
the new registry module, its focused integration test, the `lib.rs` re-export
touch, and the allowed HCM control-pack sections.

## TDD and behavior proof

The TDD sequence captured real red states before green implementation:

1. API shape test initially failed to compile because the new public registry
   types and module did not exist; adding the module/export and minimal owner
   constructor made the one shape/membership test pass.
2. Data-driven accessor assertions then failed on missing kind/instance/
   capability/dependency accessors; implementing immutable value projections
   made shipped field and metadata proof pass.
3. Custom/dependency/validation/determinism tests initially failed before the
   custom repository-source fixture was admitted (`LocalReferenceOutsideRoot`
   for the copied project-authority schema root); adding the explicit allowed
   schema root made all six focused HCM-1.3 tests pass.
4. Clippy then failed on a cloned slice helper; replacing it with
   `std::slice::from_ref` made the lint proof pass.

Focused HCM-1.3 assertions prove:

- exact shipped profile ref/fingerprint, six kind refs, and three instance IDs;
- Project Authority role, constitutional capability contract, binding map, and
  non-executing validator metadata;
- Project Context and Environment Context role/path/requiredness/condition
  data, with all unselected kinds still present and capability-free;
- a repository-source custom kind and `registry_brief` custom instance enter
  the registry without any enum variant, command, filename, or product bridge;
- authored instance and capability dependencies resolve to provider IDs and one
  providers-before-consumers lexical order;
- `validate_json` routes through the instance-bound kind schema, returns typed
  `UnknownArtifactInstance`, and wraps structural errors; and
- reversed explicit source collections produce an identical registry projection.

## Regression matrix

| Command / proof | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p handbook-engine --test hcm_1_3_artifact_registry` | PASS; 6 tests |
| `cargo test -p handbook-engine --test hcm_1_1_custom_kind` | PASS |
| `cargo test -p handbook-engine --test profile_selection` | PASS; 19 tests |
| `cargo test -p handbook-engine --test hcm_1_2_public_owner_api` | PASS |
| `cargo test -p handbook-engine --test hcm_1_2_selected_kinds` | PASS |
| `cargo test -p handbook-engine --test hcm_1_2_unselected_kinds` | PASS |
| `cargo test -p handbook-engine --test artifact_kind_registry` | PASS; 8 tests |
| `cargo test -p handbook-engine --test artifact_instances` | PASS; 9 tests |
| `cargo test -p handbook-engine` | PASS; engine unit, integration, and doc tests |
| `cargo test --workspace --all-targets` | PASS; workspace all-targets |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS after clippy red/green fix |
| `cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features` | PASS |
| handoff validators and self-tests | PASS; 38 records, 146 current dispatches, 8 legacy dispatches |
| exact allowed changed-file set | PASS; 7 expected, 7 actual, no missing/extra |
| fixed-consumer/facade inventory scan | PASS; exact 29-file set equality, no missing/extra files |
| forbidden new-code fixed-product scan | PASS; no new enum/table/product adoption; only descriptor `renderer_definition_refs` accessors and pre-existing `lib.rs` exports match broad terms |
| `cargo package -p handbook-engine --allow-dirty --no-verify` | PASS; 94 files, 1.0 MiB uncompressed, 172.0 KiB compressed |
| package archive SHA-256 | `71b59334e5ddc2ba2dce6900766718718385e759b5519f460a78493f7de4f66b` |
| repository definition tree vs HCM-1.2 package manifest | PASS; 29 expected, 29 actual, no missing/extra/changed path-size-SHA entries |
| package archive definitions vs HCM-1.2 package manifest | PASS; 29 expected, 29 actual, no missing/extra/changed path-size-SHA entries |
| HCM-1.3 fixture/package definition leakage | PASS; none |
| extracted `handbook-engine-0.1.1.crate` `cargo check` | PASS |

The existing HCM-1.2 boundary suites replay the exact source-count, binding,
root, schema-document, ancestry, schema-reference, path byte/component,
per-source, aggregate-byte, duplicate, compatibility, dependency, and cycle
refusals before registry construction. HCM-1.3 adds no input source and no new
numeric ceiling.

## Package definition members

The package and repository proof compared this literal 29-member set by path,
size, SHA-256, and bytes against
`docs/specs/handbook-contract-membrane/slices/HCM-1.2/proof/package-definition-manifest.json`:

- `artifact-kinds/handbook.artifact-kind.decision-record/1.0.0.yaml`
- `artifact-kinds/handbook.artifact-kind.environment-context/1.0.0.yaml`
- `artifact-kinds/handbook.artifact-kind.project-authority/1.0.0.yaml`
- `artifact-kinds/handbook.artifact-kind.project-context/1.0.0.yaml`
- `artifact-kinds/handbook.artifact-kind.risk-record/1.0.0.yaml`
- `artifact-kinds/handbook.artifact-kind.work-specification/1.0.0.yaml`
- `context-resolution-policies/handbook.memory-promotion.core/1.0.0.yaml`
- `context-resolution-policies/handbook.mutation-matcher.core/1.0.0.yaml`
- `context-resolution-policies/handbook.resolution-escalation.core/1.0.0.yaml`
- `context-resolution/handbook.context-resolution.shipped-root/1.0.0.yaml`
- `profiles/handbook.profile.shipped-root/1.0.0.yaml`
- `project-conditions/handbook.condition.project.managed-operational-surface/1.0.0.yaml`
- `schemas/handbook.schemas.artifacts.decision-record/1.0.0.entry.yaml`
- `schemas/handbook.schemas.artifacts.decision-record/1.0.0.schema.json`
- `schemas/handbook.schemas.artifacts.environment-context/1.0.0.entry.yaml`
- `schemas/handbook.schemas.artifacts.environment-context/1.0.0.schema.json`
- `schemas/handbook.schemas.artifacts.project-authority/1.0.0.entry.yaml`
- `schemas/handbook.schemas.artifacts.project-authority/1.0.0.schema.json`
- `schemas/handbook.schemas.artifacts.project-context/1.0.0.entry.yaml`
- `schemas/handbook.schemas.artifacts.project-context/1.0.0.schema.json`
- `schemas/handbook.schemas.artifacts.risk-record/1.0.0.entry.yaml`
- `schemas/handbook.schemas.artifacts.risk-record/1.0.0.schema.json`
- `schemas/handbook.schemas.artifacts.work-specification/1.0.0.entry.yaml`
- `schemas/handbook.schemas.artifacts.work-specification/1.0.0.schema.json`
- `semantic-capabilities/handbook.capabilities.constitutional-root/1.0.0.yaml`
- `semantic-validators/handbook.semantic-validation.constitutional-root/1.0.0.yaml`
- `stable-roles/handbook.roles.core/1.0.0.yaml`
- `stable-roles/handbook.roles.core/1.1.0.yaml`
- `vocabularies/handbook.vocabulary.shipped-root/1.0.0.yaml`

## Classification ceiling and review gate

The only requested classification change is `BoundaryLanded` for the additive
selected-profile registry owner boundary. Setup/doctor adoption, content
materialization, real-path reads, semantic validator execution, condition
execution, vocabulary application, Context Resolution behavior, lifecycle,
intake, renderer, Projection, overlay, downstream release, HCM-1.4, HCM-2,
HCM-3, contract, and dock gates remain open.

This wall records no fresh-review conclusion. A new immutable dispatch must bind
the complete subject, a fresh isolated built-in `default` reviewer must return
findings first, and any valid finding requires bounded remediation, full proof
replay, a new dispatch, and a different fresh reviewer before the implementation
can be committed.
