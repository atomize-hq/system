# HCM-1.2 Specification: Profile Schema, Artifact Instances, and Shipped Default

## Status and authority

This is the planning-only implementation packet for HCM-1.2. Packet approval
does not begin implementation. Execute it only in a later, separately selected
top-level orchestration run with:

```text
PHASE_ID: HCM-1
SLICE_ID: HCM-1.2
ACTIVE_PACKET: docs/specs/handbook-contract-membrane/slices/HCM-1.2
HANDOFF_SELECTOR: <exact completed HCM-1.2 planning closeout, when selected>
```

The completed HCM-1.1 handoff
`20260716T222906Z--HCM-1-1--orchestration--registry-boundary-landed` is
reviewed transition and dependency evidence only. It does not select HCM-1.2,
reopen HCM-1.1, or authorize implementation by itself.

Canonical authority for this packet is:

1. this `SPEC.md`, [`tasks/plan.md`](tasks/plan.md), and
   [`tasks/todo.md`](tasks/todo.md);
2. the HCM-1.2 row and Phase 1 ordering in
   [`../../04-phase-slice-map.md`](../../04-phase-slice-map.md);
3. the Instance profile, Artifact instance descriptor, semantic-capability,
   exact shipped-data, vocabulary, and Context Resolution definition contracts
   in
   [`../../05-contracts-schemas-and-gates.md`](../../05-contracts-schemas-and-gates.md);
4. the approved HCM-0.6 decision in
   [`../HCM-0.6/decision/shipped-default-artifact-set-decision.md`](../HCM-0.6/decision/shipped-default-artifact-set-decision.md);
5. the target owner/invariants and profile/kind/instance semantics in
   [`../../01-target-architecture.md`](../../01-target-architecture.md) and
   [`../../02-semantic-model.md`](../../02-semantic-model.md);
6. the Artifact kind/schema registry, Canonical artifact identities, Shipped
   default artifact set, Canonical layout, Setup scaffolding, and Doctor rows
   in [`../../03-seam-crosswalk.md`](../../03-seam-crosswalk.md); and
7. `PG-PROFILE-01`, `PG-DEFAULT-01`, `PG-KIND-01`, `PG-KIND-02`,
   `PG-ARTIFACT-01`, and regression rules 1-4, 13-17, and 29-32 in
   [`../../06-proof-and-regression-ledger.md`](../../06-proof-and-regression-ledger.md).

## Objective

Land an additive `handbook-engine` profile-definition boundary that:

1. extends the HCM-1.1 exact-ref/fingerprint and safe source machinery without
   weakening it;
2. loads closed, versioned profile source records with deterministic
   single-parent replace-whole layering;
3. defines `ArtifactInstanceDescriptor` independently from reusable kind
   definitions;
4. resolves every exact definition dependency needed by the shipped root
   profile through a typed source and recomputable fingerprint producer;
5. validates exact profile selection and an explicitly supplied repository
   profile source without ambient discovery or invocation-time field mutation;
6. publishes only the six-kind catalog and three-instance root selection
   approved by HCM-0.6; and
7. proves the unique constitutional root and evidence-gated Environment
   Context descriptor without adopting setup, doctor, flow, canonical YAML, or
   any current product path.

This is a definition/profile owner boundary. HCM-1.3 owns descriptor-driven
product registry adoption and HCM-1.4 owns setup/doctor decisions.

## HCM-1.2 subordinate-definition authority

The HCM-0.6 decision intentionally left content schemas and other subordinate
definitions to later reviewed slices. This packet is that reviewed subordinate
decision for the definition data required to construct the first exact profile.
It freezes the exact schemas, descriptor bytes, vocabulary data, Context
Resolution data, and non-executing dependency metadata below. These values are
not inferred from the illustrative examples in canonical `05`: the complete
values enumerated here are the HCM-1.2 selection, and any byte not derivable
from this packet is unauthorized.

This authority is limited to definition bytes, typed loading, fingerprint
closure, and meta-validation. It does not authorize vocabulary rendering,
Context Resolution envelope application, condition evaluation, lifecycle
execution, or product-path adoption. HCM-3.1/HCM-3.2 retain those behavior
boundaries.

## Live baseline and blast-radius boundary

Live repository truth at packet approval:

- HCM-1.1 landed `ExactDefinitionRef`, `DefinitionFingerprint`, packaged core
  stable-role registries, a safe repository-local Draft 2020-12 schema
  registry, and capability-free `ArtifactKindDefinition` loading in
  `handbook-engine`;
- the selected stable-role registry `handbook.roles.core@1.1.0` and frozen
  fingerprint already exist as package-owned bytes;
- HCM-1.1 deliberately refuses every non-empty semantic-capability,
  semantic-validator, renderer, lifecycle, review-trigger, Projection,
  required-capability, extension, or opaque dependency input;
- `CanonicalArtifactKind`, fixed descriptor/layout tables, setup, doctor,
  baseline validation, flow, compiler, and CLI still bypass the new registry;
  and
- no first-party content schema, first-party kind, profile, artifact instance,
  condition definition, vocabulary definition, or Context Resolution stack is
  published yet.

Before editing an existing function, type, or method, run repository-required
GitNexus upstream impact analysis on that symbol and warn before HIGH or
CRITICAL changes. New symbols still require context inspection of their owner
module and integration point. HCM-1.2 must not turn this additive boundary into
the HCM-1.3 product-path cutover.

## Required skill chain

The implementation runner applies, in order as phases begin:

1. `using-agent-skills`;
2. `context-engineering`;
3. `source-driven-development`;
4. `spec-driven-development` and `planning-and-task-breakdown` for packet
   revalidation only;
5. `api-and-interface-design`;
6. `security-and-hardening`;
7. `incremental-implementation`;
8. `test-driven-development`;
9. `debugging-and-error-recovery` for failures;
10. `code-review-and-quality`; and
11. `git-workflow-and-versioning`.

## Exact allowed scope

### Existing files

- `crates/engine/src/lib.rs`;
- `crates/engine/src/artifact_kind_registry.rs`, only to replace the HCM-1.1
  blanket refusal with the exact typed HCM-1.2 dependency closures below;
- `crates/engine/src/schema_registry.rs`, only for a shared typed built-in
  source path, request-wide closure budget, and retained normalized read-only
  schema-shape query that preserve the existing repository-source security
  rules; raw authored bytes must not escape this owner;
- `crates/engine/src/definition_identity.rs`, only for a reusable internal
  typed-fingerprint helper after impact analysis;
- `crates/engine/Cargo.toml` and `Cargo.lock` only when implementation proves a
  dependency is unavoidable and the packet is reviewed before adding it;
- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` and
  `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`
  only for evidence-backed HCM-1.2 closeout truth; and
- this packet's todo/proof status.

### New source, asset, and proof areas

- `crates/engine/src/semantic_capability_registry.rs`;
- `crates/engine/src/project_condition_registry.rs`;
- `crates/engine/src/vocabulary_registry.rs`;
- `crates/engine/src/context_resolution_registry.rs`;
- `crates/engine/src/artifact_instance.rs`;
- `crates/engine/src/instance_profile.rs`;
- `crates/engine/src/profile_selection.rs` when a separate small module is
  clearer than embedding selection in `instance_profile.rs`;
- package-owned exact definitions under
  `crates/engine/definitions/{semantic-capabilities,semantic-validators,project-conditions,vocabularies,context-resolution,context-resolution-policies,schemas,artifact-kinds,profiles}/`;
- focused `crates/engine/tests/*capability*.rs`, `*condition*.rs`,
  `*instance*.rs`, and `*profile*.rs` tests;
- `crates/engine/tests/fixtures/hcm_1_2_repository_profile/`;
- HCM-1.2 proof/review dispatch evidence; and
- the parent closeout artifacts allowed by canonical `07`/`08`.

No existing canonical artifact, path/layout, setup, doctor, flow, compiler,
CLI, pipeline, SDK, Tauri, Substrate, authoring, intake, renderer, Projection,
posture, contract, dock, or adapter module is in scope. If correct
implementation requires one, stop with `authority_boundary`.

## Dependency and source posture

Prefer the existing dependency graph. HCM-1.2 should require no new runtime
crate: exact identity, SemVer, closed YAML/JSON decoding, RFC 8785, SHA-256,
Draft 2020-12 validation, safe repo-relative reads, and package-owned
`include_bytes!` assets already exist.

All definition sources use one closed typed source enum:

```rust,ignore
pub enum DefinitionSource {
    BuiltIn(ExactDefinitionRef),
    RepositoryPath(String),
}

pub struct DefinitionSourceBinding {
    pub definition_ref: ExactDefinitionRef,
    pub source: DefinitionSource,
}
```

Exact Rust layout may differ, but the semantics may not:

- built-in selection maps an allowlisted exact ref to package-owned immutable
  bytes; no directory scan, path construction from user text, or implicit
  minor/latest substitution exists;
- repository selection names one explicit normalized repo-relative source and
  uses the HCM-1.1 descriptor-relative no-follow/bounded reader;
- every source is parsed as its exact closed record type before fingerprinting;
- every referenced definition is loaded from a typed source registry and its
  fingerprint is recomputed; caller-supplied opaque dependency digests remain
  forbidden; and
- package proof enumerates exact members rather than checking counts alone.

### Exact symbolic-ID grammar

Definition identities/refs continue to use HCM-1.1's namespaced-ID and exact-
ref grammar. Non-definition machine symbols use a separate `SymbolicId`:

- 1-64 ASCII bytes;
- exact regular language `[a-z][a-z0-9]*(?:_[a-z0-9]+)*`;
- lowercase letters, digits, and single internal underscores only; and
- no trimming, case folding, Unicode normalization, or other repair.

Thus internal single underscores are accepted, while empty, 65-byte,
uppercase, hyphen, dot, Unicode, control, leading/trailing underscore, and
adjacent-underscore inputs refuse. This grammar applies to capability IDs,
validator `rule_id`/`binding_key`, Context Resolution level/dimension/value IDs,
artifact-instance IDs, and instance/capability dependency `target_ref` values.
Stable role refs remain validated by the selected stable-role registry, though
the shipped role values also satisfy `SymbolicId`. Exact definition IDs,
versions, refs, repository paths, display labels, record IDs, and free text do
not use `SymbolicId`.

### Exact request and source graph

`ProfileSelectionRequest` is a closed typed record with exactly:

```rust,ignore
pub struct ProfileSelectionRequest {
    pub selected_profile_ref: ExactDefinitionRef,
    pub profile_sources: Vec<DefinitionSourceBinding>,
    pub stable_role_registry_sources: Vec<DefinitionSourceBinding>,
    pub schema_entry_sources: Vec<DefinitionSourceBinding>,
    pub artifact_kind_sources: Vec<DefinitionSourceBinding>,
    pub semantic_capability_sources: Vec<DefinitionSourceBinding>,
    pub semantic_validator_sources: Vec<DefinitionSourceBinding>,
    pub project_condition_sources: Vec<DefinitionSourceBinding>,
    pub vocabulary_sources: Vec<DefinitionSourceBinding>,
    pub context_resolution_sources: Vec<DefinitionSourceBinding>,
    pub context_resolution_policy_sources: Vec<DefinitionSourceBinding>,
    pub allowed_schema_roots: Vec<String>,
}
```

Each `DefinitionSourceBinding` has exactly the two fields shown. The loaded
record's derived exact ref must equal the
declared ref. Bindings are unique across and within their type class; a source
in the wrong collection, duplicate/conflicting exact ref, mismatched record
type, unreferenced supplied source, missing transitive source, or source-order
winner refuses. Built-in refs map only through a compile-time allowlist.
Repository paths never derive from refs.

The selected leaf must appear exactly once in `profile_sources`. Its complete
parent chain is resolved only from that collection. A definition source is
used when it is traversed either (a) to recompute any authored source-profile
fingerprint in the selected ancestry, including a dependency of a parent field
later shadowed by a child replacement, or (b) to resolve the final winning
profile closure. Every such source must appear exactly once in its matching
typed collection. A source outside both closures is unreferenced and refuses;
a shadowed-parent dependency is not unused. Repository schema documents remain
selected through their exact schema-entry records and `allowed_schema_roots`;
no profile field or source binding bypasses HCM-1.1 root containment.

One request admits at most 64 profile sources, 512 total definition-source
bindings across all collections, 32 allowed schema roots, 128 distinct schema
closure documents request-wide, 32 profile ancestry edges, 32 schema-reference
edges on one traversal path, 1 MiB per source, and 8 MiB total bytes across
definitions plus schema documents. Every repository source path and allowed
schema-root string is 1-1024 UTF-8 bytes after exact-trim equality and has 1-64
normalized components. Root/source paths must already be normalized, roots are
unique, and duplicate roots refuse rather than being silently deduplicated.

The same 1-1024-byte/1-64-component ceiling applies to every schema-entry
`document_ref` and every normalized repository-local target produced while
resolving a transitive `$ref`. Task 1 owns explicit binding/root admission.
Task 2's schema-registry surface owns document-ref and transitive-target checks
after URI/reference resolution and before any filesystem open. Both the
declared path string and final normalized target must fit; percent encoding,
dot removal, or shared-document reuse cannot evade the ceiling. Built-in
package paths are compile-time allowlisted and separately package-proven.

The 128-document ceiling is shared by every schema entry resolved for the
complete profile request, not reset per entry. A document is counted once by
its typed source identity: normalized repo-relative path for repository data or
compile-time package path for built-in data. Reuse of that same identity by
multiple entries counts once; equal bytes at distinct identities count
separately. The profile resolver owns one request-wide visited set and remaining
budget and supplies it to the schema-registry closure loader/shape-query
surface. Per-entry HCM-1.1 closure safety remains additionally enforced.

Fail-fast order is exact: (1) closed request decode and scalar grammar;
(2) collection/root/path counts and duplicate declared identities; (3) path
normalization/component/byte ceilings; (4) sentinel-byte per-source and
aggregate reads; (5) typed record decode, record-class/schema identity, and
derived exact-ref equality without closure-dependent fingerprint acceptance;
(6) selected profile ancestry/scope/cycle validation and replace-whole layer
decisions; (7) request-wide schema closure; (8) typed dependency closure and
fingerprint recomputation in the topological order below; (9) complete
source-usage accounting; then (10) final descriptor/profile invariants and
resolved-profile fingerprint. No later error replaces an earlier class.

Step 8 is stable-role registry -> schema entry/document closure fingerprint ->
validator profile -> capability contract -> kind -> condition -> vocabulary ->
Context Resolution policy -> Context Resolution stack -> descriptor closure -> authored
profile fingerprints in root-to-leaf ancestry order. Within one class, exact
refs sort lexicographically. Each fingerprint is compared only after all of its
closure inputs have been recomputed; the first stale pair in that order wins.
Mixed-invalid tests preserve precedence across all ten stages.

The implementation uses iterative ancestry
traversal (or proves an equivalent bounded stack) and stops after one sentinel
byte at every byte boundary. Depth 32/source 64/root 32/path-component 64 are
admitted; 33/65/33/65 refuse with typed stable locations.

## Public owner boundary

`handbook-engine` exposes typed local-library APIs for:

- `SemanticCapabilityContract` and `SemanticCapabilityRegistry`;
- `SemanticValidationProfileDefinition` for binding-shape meta-validation,
  not executable repository code;
- `ProjectConditionDefinition` and `ProjectConditionRegistry`;
- `ArtifactInstanceDescriptor`, `ArtifactRequiredness`, and typed instance/
  capability dependency records;
- `InstanceProfileDefinition`, `ResolvedInstanceProfile`, and deterministic
  layer decisions;
- `ProfileSelectionRequest` with one exact selected profile ref and explicit
  typed per-definition-class sources plus explicit allowed schema roots; and
- typed profile/instance errors carried through `RegistryLoadErrorKind` or one
  equally specific engine-owned error enum without string parsing.

The primary entry point is conceptually:

```rust,ignore
pub fn resolve_instance_profile(
    repo_root: impl AsRef<std::path::Path>,
    request: ProfileSelectionRequest,
) -> Result<ResolvedInstanceProfile, ProfileLoadError>;
```

The request selects one exact profile and supplies all repository source paths
explicitly. It cannot mutate profile fields, select a version range, or infer a
profile from cwd, filenames, current enums, templates, or a legacy mode.

This API is additive and not downstream-published by HCM-1.2.

## Exact package-owned definition set

HCM-1.2 publishes these exact definition identities as package-owned data:

| Definition class | Exact refs |
|---|---|
| semantic capability | `handbook.capabilities.constitutional-root@1.0.0` |
| semantic validator metadata | `handbook.semantic-validation.constitutional-root@1.0.0` |
| project condition | `handbook.condition.project.managed-operational-surface@1.0.0` |
| vocabulary | `handbook.vocabulary.shipped-root@1.0.0` |
| Context Resolution stack | `handbook.context-resolution.shipped-root@1.0.0` |
| Context Resolution dependency policies | `handbook.mutation-matcher.core@1.0.0`, `handbook.resolution-escalation.core@1.0.0`, `handbook.memory-promotion.core@1.0.0` |
| content schemas | `handbook.schemas.artifacts.project-authority@1.0.0`, `project-context@1.0.0`, `environment-context@1.0.0`, `work-specification@1.0.0`, `decision-record@1.0.0`, and `risk-record@1.0.0` under the same prefix |
| artifact kinds | the exact six HCM-0.6 kind refs, each at `1.0.0` |
| profile | `handbook.profile.shipped-root@1.0.0` |

Every source has an explicit record-schema identity/version, exact ref,
recomputed fingerprint, and package path. No identity is derived from its
filename. Derived hashes are implementation outputs captured in immutable
proof and replayed from exact reviewed bytes; no caller or test fixture chooses
them and no post-review packet mutation is required.

New assets use this exact package layout, where `<id>` is the identity before
`@` and `<version>` is its canonical full SemVer:

- ordinary typed definitions:
  `crates/engine/definitions/<class>/<id>/<version>.yaml`;
- each content-schema identity:
  `crates/engine/definitions/schemas/<id>/<version>.entry.yaml` plus
  `crates/engine/definitions/schemas/<id>/<version>.schema.json`.

`<class>` is exactly `semantic-capabilities`, `semantic-validators`,
`project-conditions`, `vocabularies`, `context-resolution`,
`context-resolution-policies`, `artifact-kinds`, or `profiles` as applicable.
Package proof expands this rule to a literal sorted member list and compares
that list, file sizes, and SHA-256 values against the built-in allowlist; a
count or recursive directory scan is not admission authority.

## First-party content-schema boundary

HCM-1.2 publishes structural schemas only so the six exact kinds have real,
non-vacuous canonical-schema closures. It does not adopt those schemas as the
current product path or claim canonical YAML.

All six roots are closed objects (`additionalProperties: false` and
`unevaluatedProperties: false`) requiring exactly `schema_id`,
`schema_version`, `record_id`, plus the kind fields below. `schema_id` is the
kind ref identity with `handbook.artifact-kind` replaced by
`handbook.artifact`; `schema_version` is constant `"1.0"`; `record_id` uses the
HCM-1.1 3-255-byte namespaced-ID grammar. No optional top-level field,
nullable field, `body`, `data`, or extension map exists.

Shared value types are exact:

- `ShortText`: UTF-8 string, 1-256 Unicode scalar values, no NUL;
- `LongText`: UTF-8 string, 1-8192 Unicode scalar values, no NUL;
- `StableRef`: HCM-1.1 namespaced ID or exact definition ref, 3-320 ASCII
  bytes, no absolute path/URI/whitespace;
- `ShortTextList`: semantic-set array of `ShortText`, 1-64 items;
- `OptionalShortTextList`: semantic-set array of `ShortText`, 0-64 items; and
- `ReferenceList`: semantic-set array of `StableRef`, 0-128 items.

All three list aliases structurally reject duplicates with `uniqueItems`.
HCM-1.2 accepts any authored order of otherwise valid unique values and does
not load, reorder, or fingerprint canonical content. This packet freezes
Unicode-scalar lexicographic ordering as the future semantic-set normalization
rule for the HCM-2 canonical-content owner; HCM-2 must apply it before content
fingerprinting or version this contract. HCM-1.2 proof covers duplicate
rejection and acceptance of forward/reverse unique orders only, never equal
content-fingerprint output.

Every nested object is closed. The complete remaining shapes are:

| Schema | Exact required fields and types |
|---|---|
| Project Authority | `policy` object with `revision: ShortText`, `authority_statement: LongText`; `governance` object with `decision_authority: ShortTextList`, `required_approvals: ShortTextList`, `exception_policy: LongText`, `review_triggers: ShortTextList`, `reassessment_triggers: ShortTextList`; `engineering_posture` object with `dimensions: ShortTextList`, `red_lines: ShortTextList` |
| Project Context | `summary: LongText`, `system_boundaries: ShortTextList`, `ownership: ShortTextList`, `authoritative_references: ReferenceList`, `known_unknowns: OptionalShortTextList` |
| Environment Context | `applicability_basis: ReferenceList` with at least one item, `operational_surfaces: ShortTextList`, `runtime_dependencies: ShortTextList`, `safe_configuration_references: ReferenceList`, `authoritative_references: ReferenceList`, `known_unknowns: OptionalShortTextList` |
| Work Specification | `objective: LongText`, `scope: ShortTextList`, `non_goals: OptionalShortTextList`, `acceptance_criteria: ShortTextList`, `status` enum `draft|review_ready|approved|active|completed|cancelled` |
| Decision Record | `context: LongText`, `decision: LongText`, `status` enum `proposed|accepted|superseded|withdrawn`, `consequences: ShortTextList`, `supersedes: ReferenceList` |
| Risk Record | `uncertainty: LongText`, `evidence_refs: ReferenceList`, `owner: ShortText`, `treatment: LongText`, `status` enum `open|monitoring|mitigated|accepted|closed`, `review_basis: ReferenceList` with at least one item |

The three named list aliases above are sets; no other array exists in these six
schemas. Environment Context contains reference/configuration names only: keys
or nested shapes named `secret`, `secret_value`, `token`, `password`,
`credential`, `private_key`, or `environment_values` are impossible under the
closed schema and covered by negatives. The unselected work/decision/risk
schemas grant no root-profile selection or materialization. Phase 2 owns
product-path adoption, authoring, intake, renderer behavior, and richer semantic
validation; it must version any incompatible schema change instead of mutating
these exact bytes.

## Capability and binding-shape compatibility

HCM-1.2 may enable only the exact constitutional capability needed by the
approved shipped root.

The package capability source is this complete closed record (with the derived
placeholder convention defined below):

```yaml
schema_id: handbook.semantic-capability-contract
schema_version: "1.0"
contract_id: handbook.capabilities.constitutional-root
contract_version: "1.0.0"
capability_id: constitutional_root
required_bindings: [policy_root, policy_revision, decision_authority, required_approvals, exception_policy, engineering_posture_dimensions, red_lines, review_triggers, reassessment_triggers]
semantic_validation_profile_refs: [handbook.semantic-validation.constitutional-root@1.0.0]
allowed_instance_cardinality: exactly_one
extensions: {}
capability_fingerprint: sha256:derived
```

The one semantic-validator producer is a closed
`handbook.semantic-validation-profile-definition / 1.0` record with exactly
`schema_id`, `schema_version`, `profile_id`, `profile_version`,
`capability_id`, `binding_rules`, `extensions`, and
`profile_fingerprint`. Its identity is
`handbook.semantic-validation.constitutional-root@1.0.0`, its exact capability
ID is the non-definition `SymbolicId` `constitutional_root`, extensions are
`{}`, and `profile_fingerprint` is derived. `binding_rules` is an ordered array of
exactly nine closed records with fields `rule_id`, `binding_key`, `json_type`,
`cardinality`, and `empty_policy`. IDs/keys use `SymbolicId`; all other values
are closed enums. The
array order equals the capability's `required_bindings` order and is semantic.

```yaml
schema_id: handbook.semantic-validation-profile-definition
schema_version: "1.0"
profile_id: handbook.semantic-validation.constitutional-root
profile_version: "1.0.0"
capability_id: constitutional_root
binding_rules:
  - { rule_id: policy_root, binding_key: policy_root, json_type: object, cardinality: singular, empty_policy: forbidden }
  - { rule_id: policy_revision, binding_key: policy_revision, json_type: string, cardinality: singular, empty_policy: forbidden }
  - { rule_id: decision_authority, binding_key: decision_authority, json_type: array, cardinality: plural, empty_policy: forbidden }
  - { rule_id: required_approvals, binding_key: required_approvals, json_type: array, cardinality: plural, empty_policy: forbidden }
  - { rule_id: exception_policy, binding_key: exception_policy, json_type: string, cardinality: singular, empty_policy: forbidden }
  - { rule_id: engineering_posture_dimensions, binding_key: engineering_posture_dimensions, json_type: array, cardinality: plural, empty_policy: forbidden }
  - { rule_id: red_lines, binding_key: red_lines, json_type: array, cardinality: plural, empty_policy: forbidden }
  - { rule_id: review_triggers, binding_key: review_triggers, json_type: array, cardinality: plural, empty_policy: forbidden }
  - { rule_id: reassessment_triggers, binding_key: reassessment_triggers, json_type: array, cardinality: plural, empty_policy: forbidden }
extensions: {}
profile_fingerprint: sha256:derived
```

The exact kind bindings and validator rules are:

| Binding key / rule ID | Exact Project Authority JSON Pointer | JSON type | Cardinality | Empty policy | Validator profile ref |
|---|---|---|---|---|---|
| `policy_root` | `/policy` | `object` | `singular` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |
| `policy_revision` | `/policy/revision` | `string` | `singular` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |
| `decision_authority` | `/governance/decision_authority` | `array` | `plural` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |
| `required_approvals` | `/governance/required_approvals` | `array` | `plural` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |
| `exception_policy` | `/governance/exception_policy` | `string` | `singular` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |
| `engineering_posture_dimensions` | `/engineering_posture/dimensions` | `array` | `plural` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |
| `red_lines` | `/engineering_posture/red_lines` | `array` | `plural` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |
| `review_triggers` | `/governance/review_triggers` | `array` | `plural` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |
| `reassessment_triggers` | `/governance/reassessment_triggers` | `array` | `plural` | `forbidden` | `handbook.semantic-validation.constitutional-root@1.0.0` |

Each `rule_id` equals its binding key. The validator-profile-ref column states
the sole exact producer selected by the capability; it is not an additional
self-referential field or fingerprint edge inside each rule. Runtime semantic
execution is later-owned; HCM-1.2 admits and fingerprints only this non-vacuous
shape metadata.

The validator profile has no exact-definition edge back to the capability.
Its non-definition `capability_id` must equal the selecting capability record's
ID. The acyclic fingerprint order is validator profile -> capability contract
-> kind. A validator-to-capability exact ref or any graph back-edge refuses.

`schema_registry.rs` may retain normalized parsed schema documents privately
inside the resolved closure and expose one engine-internal read-only query:

```rust,ignore
fn binding_shape(
    &self,
    instance_pointer: &str,
) -> Result<ResolvedBindingShape, RegistryLoadError>;
```

`ResolvedBindingShape` contains only closed enums `json_type`
(`object|array|string`), `cardinality` (`singular|plural`), and `empty_policy`
(`forbidden|allowed`); it contains no `serde_json::Value`, authored key/value,
path, or source bytes. The query operates only on an already safely admitted
closure. It parses RFC 6901 tokens without normalization, follows only exact
local HCM-1.1 `$ref` edges, and consumes the same 32-edge depth/cycle budget.
For each token it requires a closed `type: object`, an exact `properties`
member, and membership in `required`. A `$ref` node has no semantic sibling.
At the terminal it requires exactly one explicit `object`, `array`, or `string`
type with no boolean schema, union, `allOf`, `anyOf`, `oneOf`, conditional, or
type array on the traversed path. Cardinality is plural only for `array`.
Empty is forbidden only when the terminal proves a non-empty `required` set or
`minProperties >= 1` for an object, `minItems >= 1` for an array, or
`minLength >= 1` for a string. Anything less determinate
refuses rather than guessing. Retained normalized documents and this query are
internal implementation detail, not a new downstream-published schema API.

Kind binding compatibility is mechanical and schema-aware:

1. parse the pointer as RFC 6901 without normalization;
2. resolve it against the exact admitted schema closure, following only the
   HCM-1.1-safe local `$ref` graph;
3. require a statically determinate target shape compatible with the exact
   expected binding shape;
4. reject missing, ambiguous (`anyOf`/`oneOf` without one provable common
   shape), contradictory, boolean-`true`, or incompatible targets;
5. require exactly the contract's binding-key set with no unknown or duplicate
   keys; and
6. include the capability contract, validator metadata, schema-entry, schema-
   closure, and normalized bindings in the kind fingerprint closure.

Positive proof uses all nine constitutional bindings. Negative proof mutates
each binding independently to missing, wrong-shape, ambiguous, duplicate,
unknown, and changed-fingerprint cases. Pointer existence alone is not proof.

No other non-empty semantic capability, semantic validator, lifecycle policy,
review trigger, fixed renderer, Projection, required capability, or extension
is enabled. Those fields retain HCM-1.1 fail-closed behavior until their owning
slice supplies the same typed-producer and non-vacuous compatibility proof.

## Project-condition definition boundary

The exact managed-operational-surface definition is this complete closed
declarative record, not an evaluator or profile flag:

```yaml
schema_id: handbook.project-condition-definition
schema_version: "1.0"
condition_id: handbook.condition.project.managed-operational-surface
condition_version: "1.0.0"
outcomes: ["true", "false", unknown, unresolved, stale, refused]
accepted_input_classes: [authoritative_fact_ref, admitted_evidence_ref]
freshness_requirement: explicit_current_basis_required
minimum_independent_current_bases: 1
self_reference_exclusions: [environment_context]
outcome_precedence: [refused_on_contradiction_or_disallowed_input, unresolved_on_missing_definition_or_required_input, stale_on_expired_basis, unknown_on_insufficient_proof, false_on_current_affirmative_no_responsibility, true_on_current_affirmative_qualifying_responsibility]
effects: [metadata_only, no_boolean_coercion, no_create, no_scaffold, no_delete]
extensions: {}
definition_fingerprint: sha256:derived
```

The record has exactly the fields shown. `condition_id` uses the HCM-1.1
namespaced-ID grammar (3-255 ASCII bytes), `condition_version` is canonical full
SemVer, and the exact ref derives from them. `outcomes`,
`accepted_input_classes`, `self_reference_exclusions`, `outcome_precedence`,
and `effects` are ordered arrays with exactly the literal members and order
shown; their members are closed enums, not authored strings.
`freshness_requirement` is the one closed enum literal shown and deliberately
contains no HCM-1.4 evaluator threshold. `minimum_independent_current_bases` is
the integer constant `1`. `extensions` is the exact empty map. The derived
fingerprint covers every field except itself and the record has no optional or
nullable field.

Those literal outcomes retain the exact HCM-0.6 meanings. In particular,
Environment Context cannot supply its own applicability basis, silence cannot
be false, contradiction refuses, missing definition/input is unresolved,
expired evidence is stale, and insufficient proof is unknown.

HCM-1.2 validates the exact definition and descriptor reference only. HCM-1.4
owns evaluation in setup/doctor. No HCM-1.2 API returns condition truth.

## Exact shipped vocabulary and Context Resolution closure

The HCM-0.2/HCM-0.3 examples do not select shipped data. This reviewed
HCM-1.2 subordinate decision does. The package-owned vocabulary is this
complete closed record:

```yaml
schema_id: handbook.vocabulary-profile
schema_version: "1.0"
vocabulary_id: handbook.vocabulary.shipped-root
vocabulary_version: "1.0.0"
stable_role_registry:
  ref: handbook.roles.core@1.1.0
  fingerprint: sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029
labels: {}
aliases: {}
absorptions: []
extensions: {}
vocabulary_fingerprint: sha256:derived
```

It has exactly those fields; the two identity fields follow the HCM-1.1
namespaced-ID/full-SemVer rules, the stable-role pair is a closed nested
object, both maps are exact empty maps, both arrays/maps preserve the canonical
`05` semantics, and the fingerprint is derived over every field except itself.
Empty labels means registry fallback labels; it is not an ambient vocabulary.

The package-owned `handbook.context-resolution.shipped-root@1.0.0` record is
the following literal stack definition. These labels, values, ranks, defaults,
policy refs, ordering, and empty extension map are normative HCM-1.2 data, not
an inference that the canonical example was already a shipped selection:

```yaml
schema_id: handbook.context-resolution-stack-definition
schema_version: "1.0"
stack_id: handbook.context-resolution.shipped-root
stack_version: "1.0.0"
levels:
  - level_id: strategic
    display_label: Strategic
    defaults: { scope_horizon: program, detail_resolution: full, temporal_horizon: long_range, authority_horizon: program_policy, memory_horizon: strategic, validation_horizon: program_gate }
  - level_id: coordination
    display_label: Coordination
    defaults: { scope_horizon: slice, detail_resolution: normal, temporal_horizon: current_slice, authority_horizon: slice_write, memory_horizon: coordination, validation_horizon: slice_closeout }
  - level_id: execution
    display_label: Execution
    defaults: { scope_horizon: assigned_unit, detail_resolution: normal, temporal_horizon: immediate, authority_horizon: local_write, memory_horizon: execution, validation_horizon: unit_closeout }
  - level_id: operation
    display_label: Operation
    defaults: { scope_horizon: local_observation, detail_resolution: identifier_only, temporal_horizon: current_operation, authority_horizon: read_only, memory_horizon: operation, validation_horizon: observation_only }
dimension_domains:
  scope_horizon: [{ value_id: local_observation, rank: 0 }, { value_id: assigned_unit, rank: 1 }, { value_id: slice, rank: 2 }, { value_id: program, rank: 3 }]
  detail_resolution: [{ value_id: identifier_only, rank: 0 }, { value_id: summary, rank: 1 }, { value_id: normal, rank: 2 }, { value_id: full, rank: 3 }]
  temporal_horizon: [{ value_id: current_operation, rank: 0 }, { value_id: immediate, rank: 1 }, { value_id: current_slice, rank: 2 }, { value_id: long_range, rank: 3 }]
  authority_horizon: [{ value_id: read_only, rank: 0 }, { value_id: local_write, rank: 1 }, { value_id: slice_write, rank: 2 }, { value_id: program_policy, rank: 3 }]
  memory_horizon: [{ value_id: operation, rank: 0 }, { value_id: execution, rank: 1 }, { value_id: coordination, rank: 2 }, { value_id: strategic, rank: 3 }]
  validation_horizon: [{ value_id: observation_only, rank: 0 }, { value_id: unit_closeout, rank: 1 }, { value_id: slice_closeout, rank: 2 }, { value_id: program_gate, rank: 3 }]
mutation_matcher: { ref: handbook.mutation-matcher.core@1.0.0, fingerprint: sha256:derived }
escalation_policy: { ref: handbook.resolution-escalation.core@1.0.0, fingerprint: sha256:derived }
memory_promotion_policy: { ref: handbook.memory-promotion.core@1.0.0, fingerprint: sha256:derived }
extensions: {}
definition_fingerprint: sha256:derived
```

The stack record has exactly the shown fields. Level, dimension, and value IDs
use `SymbolicId`; display labels are 1-64 Unicode scalar values with no NUL.
`levels` is an ordered four-item array and every domain is
an ordered four-item array in the literal order shown. The domain object is
closed to the six literal keys, ranks are integers exactly `0..3`, every
defaults object is closed to those same six keys, the three dependency pairs
are closed ref/fingerprint objects, and extensions are exact empty. No field is
optional or nullable; the derived fingerprint covers every field except itself
and all three recomputed producer fingerprints.

`sha256:derived` is notation in this packet only: the source assets carry the
real lowercase SHA-256 values recomputed under canonical `05`; the literal is
not an accepted runtime fingerprint.

The three non-empty dependencies above have these complete closed
package-owned typed producer records:

```yaml
schema_id: handbook.mutation-matcher-definition
schema_version: "1.0"
matcher_id: handbook.mutation-matcher.core
matcher_version: "1.0.0"
target_kinds: [repository_path]
selector_grammar:
  grammar_id: normalized_repo_relative_glob_v1
  encoding: ascii
  min_bytes: 1
  max_bytes: 1024
  min_segments: 1
  max_segments: 64
  separator: "/"
  normal_segment_character_class: "[A-Za-z0-9._*-]"
  single_segment_wildcard: "*"
  recursive_wildcard: "**"
  recursive_position: terminal_segment_only
  disallowed: [leading_slash, trailing_slash, empty_segment, dot_segment, dotdot_segment, backslash, nul, uri_prefix, adjacent_double_star_in_normal_segment]
case_mode: sensitive
deny_precedence: true
extensions: {}
definition_fingerprint: sha256:derived
```

```yaml
schema_id: handbook.resolution-escalation-policy-definition
schema_version: "1.0"
policy_id: handbook.resolution-escalation.core
policy_version: "1.0.0"
trigger_classes: [dimension_rank_increase, mutation_allow_expansion, missing_context, missing_authority]
proposal_relation: same_profile_stack_strict_widening
required_request_bindings: [current_envelope_ref_fingerprint, proposed_envelope_ref_fingerprint, trigger_ref_fingerprint, missing_condition, requested_authority_ref, evidence_refs]
terminal_outcomes: [approved, refused, superseded]
terminal_cardinality: exactly_one
preapproval_effect: request_only_no_authority_change
extensions: {}
policy_fingerprint: sha256:derived
```

```yaml
schema_id: handbook.memory-promotion-policy-definition
schema_version: "1.0"
policy_id: handbook.memory-promotion.core
policy_version: "1.0.0"
source_requirement: nonempty_exact_ref_fingerprint_pairs
target_authority: semantic_memory
horizon_relation: strictly_higher_memory_rank
write_precondition: expected_target_fingerprint_compare_and_write
required_request_bindings: [source_inputs, source_envelope_ref_fingerprint, target_memory_horizon, target_record_ref, expected_target_fingerprint, requested_authority_ref]
terminal_outcomes: [applied, refused, stale]
terminal_cardinality: exactly_one
forbidden_authorities: [canonical_artifact, contract, posture]
extensions: {}
policy_fingerprint: sha256:derived
```

Each record has exactly the shown fields, with no optional/null field.
`matcher_id`/`policy_id` use the HCM-1.1 namespaced-ID grammar (3-255 ASCII
bytes), versions are canonical full SemVer, and exact refs derive from the two.
All arrays are semantic ordered arrays with exactly the literal members and
order shown; each member is a closed enum. Matcher integer values are the exact
positive constants shown, `deny_precedence` is Boolean `true`, the nested
`selector_grammar` object is closed, and each extension map is exactly empty.
Every scalar not otherwise typed is a closed enum literal, not an authored free
string. The named fingerprint is derived over every field except itself under
the common canonical `05` rule.

`normalized_repo_relative_glob_v1` is exact: a selector is 1-1024 ASCII bytes
and 1-64 non-empty `/`-separated segments, with no leading/trailing slash,
backslash, NUL, URI prefix, `.` segment, or `..` segment. A normal segment
contains one or more `[A-Za-z0-9._*-]` bytes, may use isolated `*` tokens to
match zero or more `[A-Za-z0-9._-]` bytes within that segment, and may not
contain adjacent `**`. An exact terminal segment `**` is the only recursive
form and matches zero or more complete normal segments. Matching is bytewise
case-sensitive over an already normalized safe repo-relative target. Unknown
target kinds or indeterminate patterns refuse; allow/deny overlap selects deny.

These producer records are definition metadata only. HCM-3.2 still owns
envelope comparison, selector application, escalation request/disposition, and
memory-promotion behavior. HCM-1.2 proves exact parsing, recomputation,
cross-reference compatibility, changed-byte invalidation, and refusal of an
opaque/missing/wrong-class producer; it does not execute any policy.

## Artifact instance descriptor contract

Implement the exact closed descriptor record from canonical `05`. In
particular:

- `id` is a unique `SymbolicId` and never derived from path or label;
- `kind_ref` resolves one exact admitted kind and definition fingerprint;
- `role_ref` is explicit registered value or explicit null and is supported by
  the selected kind;
- `capability_refs` is explicit and resolves through exact kind capability
  contracts;
- `canonical_path` is one normalized unique repo-relative path with the
  existing no-follow path grammar;
- `requiredness` is exactly `always`, `conditional`, or `optional`, and only
  conditional mode carries one exact project-condition ref;
- dependencies use the exact instance/capability namespace, `SymbolicId`
  `target_ref`, contract ref, and cardinality rules from `05` with
  deterministic provider ordering;
- intake is exact ref or null, fixed renderer and future Projection selections
  are explicit lists, and every non-empty later-owned selection still refuses;
- lifecycle policy and overlays remain null/empty in this slice; and
- extensions are exactly empty.

One kind may back multiple instances. A descriptor never modifies or becomes a
kind definition.

The shipped root contains these three complete literal descriptor records; no
field is left to implementation inference:

```yaml
- schema_id: handbook.artifact-instance-descriptor
  schema_version: "1.0"
  id: project_authority
  kind_ref: handbook.artifact-kind.project-authority@1.0.0
  role_ref: constitutional_authority
  capability_refs: [constitutional_root]
  label: Charter
  canonical_path: .handbook/project/charter.yaml
  requiredness: { mode: always, condition_ref: null }
  depends_on: []
  lifecycle_policy_ref: null
  intake_definition_ref: null
  renderer_definition_refs: []
  projection_definition_refs: []
  validation_overlay_refs: []
  extensions: {}
- schema_id: handbook.artifact-instance-descriptor
  schema_version: "1.0"
  id: project_context
  kind_ref: handbook.artifact-kind.project-context@1.0.0
  role_ref: project_context
  capability_refs: []
  label: Project Context
  canonical_path: .handbook/project/context.yaml
  requiredness: { mode: always, condition_ref: null }
  depends_on: []
  lifecycle_policy_ref: null
  intake_definition_ref: null
  renderer_definition_refs: []
  projection_definition_refs: []
  validation_overlay_refs: []
  extensions: {}
- schema_id: handbook.artifact-instance-descriptor
  schema_version: "1.0"
  id: environment_context
  kind_ref: handbook.artifact-kind.environment-context@1.0.0
  role_ref: environment_context
  capability_refs: []
  label: Environment Context
  canonical_path: .handbook/project/environment.yaml
  requiredness:
    mode: conditional
    condition_ref: handbook.condition.project.managed-operational-surface@1.0.0
  depends_on: []
  lifecycle_policy_ref: null
  intake_definition_ref: null
  renderer_definition_refs: []
  projection_definition_refs: []
  validation_overlay_refs: []
  extensions: {}
```

For `project_authority`, the selected `constitutional_root` capability resolves
through its kind to exact contract
`handbook.capabilities.constitutional-root@1.0.0`; a descriptor does not carry a
second caller-chosen contract value. All empty/null later-owned selections are
semantic bytes in the descriptor and any non-empty substitution refuses.

## Profile source, layering, and selection contract

Profile sources implement the complete canonical `handbook.instance-profile /
1.0` record and uniform fingerprint contract.

- identity is `profile_id` plus canonical full SemVer;
- scope is exactly `shipped`, `named`, or `repository`;
- at most one exact parent exists and ancestry is acyclic with scope order
  shipped -> named -> repository;
- root materializes every field; child omission inherits the complete field,
  presence replaces the complete field, and explicit empty/null clears then
  independently validates;
- there is no append, keyed merge, tombstone, multi-parent merge, source-order
  conflict repair, or invocation-time field override;
- every layer decision records the exact winning source and `inherited` or
  `replaced`;
- the resolved fingerprint binds ordered ancestry plus every recomputed stable-
  role, schema, kind, capability, condition, vocabulary, Context Resolution,
  and descriptor closure; and
- diagnostics, timestamps, absolute paths, and invocation request fields are
  excluded from semantic fingerprints.

Every authored field after `extends_profile_ref` is layerable, exactly as
canonical `05` requires. The complete field set is:

1. `stable_role_registry`;
2. `schema_registry_sources`;
3. `artifact_kind_sources`;
4. `artifact_instances`;
5. `vocabulary_ref`;
6. `context_resolution_ref`;
7. `projection_catalog_refs`;
8. `posture_evaluation_policy`;
9. `dock_requirement_refs`;
10. `adapter_overlay_refs`; and
11. `extensions`.

A root source must materialize all eleven. A child may omit any field to inherit
the complete parent value or present the field to replace it whole. Present
empty lists/maps and present `null` are replacements, not omission, and must
then pass final validation. No field has keyed merge, append, tombstone, or a
special repository-only exception. The resolver emits exactly one ordered
layer decision for each of the eleven fields, identifying the winning exact
source profile and `inherited` or `replaced`.

The exact shipped-root authored values are:

| Field | Complete root value |
|---|---|
| `stable_role_registry` | exact `handbook.roles.core@1.1.0` plus recomputed `sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029` |
| `schema_registry_sources` | the exact six content-schema refs in this SPEC, in Project Authority, Project Context, Environment Context, Work Specification, Decision Record, Risk Record order |
| `artifact_kind_sources` | the matching exact six HCM-0.6 kind refs in the same semantic order |
| `artifact_instances` | the three complete descriptor records above, in `project_authority`, `project_context`, `environment_context` order |
| `vocabulary_ref` | `handbook.vocabulary.shipped-root@1.0.0` |
| `context_resolution_ref` | `handbook.context-resolution.shipped-root@1.0.0` |
| `projection_catalog_refs` | `[]` |
| `posture_evaluation_policy` | `null` |
| `dock_requirement_refs` | `[]` |
| `adapter_overlay_refs` | `[]` |
| `extensions` | `{}` |

The root identity is `handbook.profile.shipped-root@1.0.0`, scope `shipped`,
and `extends_profile_ref: null`. Its vocabulary and Context Resolution refs
resolve only through the exact typed sources and producers frozen above.
HCM-1.2 loads/fingerprints those definitions but does not apply envelopes,
selectors, escalation, promotion, or current work-level behavior.

A named or repository source may replace **any** of the eleven complete fields,
not merely descriptor paths/labels/requiredness, but every winning value must
resolve from the request's matching typed source collection and the final
profile must pass all schema, kind, descriptor, dependency, role, capability,
condition, Context Resolution, and unique-constitutional-root gates. The one
explicit repository-scoped source has one exact named-or-shipped parent and is
resolved from `profile_sources`; there is no second ambient repository overlay.
HCM-1.2's admitted values for `projection_catalog_refs`,
`posture_evaluation_policy`, `dock_requirement_refs`, `adapter_overlay_refs`,
and `extensions` remain respectively empty, null, empty, empty, and empty;
layering a non-empty later-owned value works syntactically and then refuses
final validation because this slice has no producer collection for it. A
repository source cannot define a legacy profile, inherit ambient local files,
bypass explicit schema roots, or mutate the selected profile at invocation.

## Exact HCM-0.6 shipped closure

The package-owned shipped root profile contains exactly:

- the six exact HCM-0.6 kind refs and no seventh first-party kind;
- `project_authority`, `project_context`, and `environment_context` descriptors
  with byte-exact kind, role, capability, label, path, requiredness, and
  condition values from the approved decision;
- exactly one always-required `constitutional_root` provider:
  `project_authority`;
- no Work Specification, Decision Record, or Risk Record instance;
- no implicit materialization or setup/doctor decision; and
- no legacy/compatibility profile.

Literal set equality is required for catalog and instance proof. Counts alone
are insufficient. Both forward and reverse source permutations of at least six
kinds and three instances must yield identical lookup sets, dependency
resolution, layer decisions, and fingerprints.

## Typed failure and threat contract

Typed errors distinguish at least:

- invalid profile/descriptor/capability/condition record schema or version;
- unknown top-level/nested/wrong-record field or non-empty extension;
- invalid identity, SemVer, exact ref, fingerprint, or profile scope;
- missing, duplicate, conflicting, cyclic, illegal-scope, or over-depth
  profile ancestry;
- missing/duplicate/conflicting schema, kind, instance, path, role,
  capability, condition, vocabulary, or Context Resolution identity;
- source-order conflict, ambient source, invocation mutation, implicit latest,
  and legacy-profile refusal;
- unsupported role/capability, missing/extra/ambiguous/wrong-shape binding, or
  stale dependency fingerprint;
- invalid requiredness/condition combination;
- dependency namespace, contract, cardinality, provider-count, or cycle error;
- zero or multiple constitutional roots, or a root that is not always
  required;
- shipped catalog/instance set mismatch; and
- unsafe/unbounded repository source or path.

Errors expose stable bounded field locations and no absolute paths, source
contents, secret values, or unbounded authored identifiers. No unsupported or
indeterminate input falls back to success.

## TDD and implementation increments

Every task follows RED -> GREEN -> REFACTOR:

1. capture a focused failing test for the next exact contract;
2. implement the smallest engine-owned behavior;
3. run focused tests and `cargo fmt --all -- --check`;
4. run affected engine tests before expanding; and
5. stage/commit only a green independently reviewable increment after required
   GitNexus change detection.

Do not batch profile parsing, dependency producers, packaged data, repository
layering, and proof into one unreviewable change.

## Full proof wall

Before closeout, preserve exact evidence for:

### Focused positive proof

- exact profile/descriptor/capability/condition identities and fingerprints;
- positive replay of every frozen `SymbolicId`, including internal
  underscores, plus the exact 64-byte boundary;
- literal six-kind and three-instance shipped-set equality;
- all nine constitutional bindings with compatible schema shapes;
- acyclic validator -> capability -> kind fingerprint propagation, including a
  changed-validator change through capability/kind/profile fingerprints;
- unique constitutional root and exact condition reference;
- root, named, and repository replace-whole layering with deterministic
  decisions;
- root-to-leaf source-profile fingerprint replay where a child replaces
  vocabulary, Context Resolution, schema, and kind fields while every
  shadowed-parent dependency remains supplied and counted as used;
- explicit exact profile selection and repository source input;
- exact typed source-class/ref matching for every winning transitive
  definition and explicit admitted schema root;
- literal shipped vocabulary, four-level/six-domain stack, and all three exact
  policy/matcher producer records with recomputed closure fingerprints;
- ancestry depth 32, profile-source count 64, total-binding count 512,
  allowed-root count 32, request-wide cross-entry distinct schema-document
  count 128, schema-reference path depth 32, repository path bytes 1024/path
  components 64, exact 1-MiB per-source bytes, and exact 8-MiB aggregate bytes
  accepted;
- deterministic repeated and forward/reverse source-order fingerprints; and
- structural semantic-set duplicate refusal plus forward/reverse unique content
  order acceptance, with explicit proof that HCM-1.2 emits no content
  fingerprint or normalization;
- exact package member list, sizes, and hashes for every definition asset.

### Negative and fail-closed proof

- duplicate/unknown/wrong-record fields at every nesting level;
- empty/65-byte/uppercase/hyphen/dot/Unicode/control/leading-underscore/
  trailing-underscore/double-underscore `SymbolicId` inputs and any attempted
  trim/case-fold/Unicode-normalization repair;
- forged/stale definition fingerprints and changed bytes behind the same ref;
- missing/extra/wrong-shape/ambiguous constitutional bindings;
- unknown capability/condition/role/kind/schema/profile refs;
- wrong typed source collection, loaded-ref mismatch, duplicate/conflicting or
  unused supplied source, missing transitive source, unadmitted schema root,
  and built-in ref not present in the compile-time allowlist;
- missing dependency needed only for a shadowed selected-ancestor field and a
  genuinely unrelated source, distinguished from each other deterministically;
- ancestry depth 33, profile-source count 65, total-binding count 513,
  allowed-root count 33, request-wide multi-entry schema-document count 129,
  schema-reference path depth 33, repository path byte/component counts
  1025/65, one byte over 1 MiB, and one byte over 8 MiB, plus duplicate roots
  and a depth-32 back-edge cycle;
- schema-entry root-document and normalized transitive `$ref` targets at
  1024/1025 bytes and 64/65 components before filesystem access;
- exact capability, nine-rule validator profile, condition, matcher,
  escalation-policy, and promotion-policy record equality plus missing/extra/
  wrong-type/over-bound field mutations at every nested level;
- compound-invalid fixtures proving the exact ten-stage fail-fast precedence,
  topological within-stage ordering, and closure-before-stale-fingerprint
  behavior;
- missing/wrong-class/stale Context Resolution matcher or policy producer,
  changed producer bytes, invalid stack rank/domain/default, and invalid
  repository-path selector grammar;
- validator-to-capability exact-definition back-edge and every other
  definition-closure cycle;
- duplicate catalog/instance/path identities in both source orders;
- zero/two constitutional roots and non-always root;
- invalid requiredness/condition pairings;
- zero/one/two providers for both capability cardinalities, exact contract
  match/mismatch, duplicate capability declarations, and dependency cycles;
- child omission versus complete replacement, explicit empty clearing,
  illegal scope ancestry, multi-parent/cycle, and source-order fallback;
- invocation field mutation, ambient discovery, ranges/latest, legacy profile,
  and filename/enum/template inference;
- shipped catalog/instance additions, omissions, substitutions, and path/label/
  requiredness drift;
- Environment Context self-condition, profile-flag-only applicability, secret
  fields, and outcome coercion; and
- unsafe/symlinked/absolute/traversing/oversize repository profile sources.

### Regression and repository proof

Run at minimum:

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-engine --all-targets --all-features -- -D warnings
cargo test -p handbook-engine --all-features
cargo test --workspace --all-features
cargo check -p handbook-engine --target x86_64-pc-windows-gnu --all-features
cargo tree -p handbook-engine -e features
cargo package -p handbook-engine --allow-dirty --no-verify
python3 tools/check_archive_boundary.py --self-test
python3 tools/check_archive_boundary.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
git diff --check
```

Additionally prove exact package/tree equality, no new network/TLS/async/
executable-hook dependency, no current product-path change, and staged
GitNexus change detection limited to the additive engine definition/profile
boundary.

## Permitted classification and gate change

HCM-1.2 may, only after complete proof and CLEAN final review:

- promote the shipped-default runtime-data row from `TargetOnly` to
  `BoundaryLanded` for exact package-owned definitions/profile data only;
- record an additive profile-schema/profile-selection owner boundary and the
  descriptor-definition subset of `PG-PROFILE-01` and `PG-ARTIFACT-01`;
- record exact runtime publication of the HCM-0.6 data while leaving
  `PG-DEFAULT-01`'s documentation decision unchanged; and
- extend the bounded `PG-KIND-01` evidence only for exact constitutional
  capability-contract and binding-shape meta-validation.

`PG-PROFILE-01`, `PG-ARTIFACT-01`, `PG-KIND-01`, and `PG-KIND-02` remain open.
HCM-1.2 cannot claim product registry adoption, setup/doctor/flow use,
condition evaluation, lifecycle, intake, renderer, Projection, canonical YAML,
publication, or downstream consumption.

## Exit gate

HCM-1.2 completes only when:

- all exact public types, record schemas, source paths, package assets,
  fingerprints, and HCM-0.6 set values match this packet;
- every non-empty dependency has a typed source/fingerprint producer and
  non-vacuous compatibility proof;
- the complete positive, negative, security, package, cross-platform,
  regression, and workspace proof wall passes;
- fixed product paths and sibling slices remain unchanged;
- only the permitted bounded crosswalk/gate evidence is recorded;
- a fresh isolated built-in `default` reviewer returns CLEAN over the exact
  complete implementation and proof subject;
- every valid finding receives bounded remediation, complete proof replay, and
  a different fresh reviewer;
- staged GitNexus detection and exact byte replay pass;
- the reviewed implementation is committed first; and
- one parent-owned v1.2 handoff plus deterministic ledger rebuild validates and
  is committed separately without beginning HCM-1.3.

## Stop conditions

Stop with an exact parent-owned handoff rather than improvising when:

- the selected HCM-1.1 completion evidence or live owner boundary does not
  validate;
- canonical authority proves an exact shipped content-schema, vocabulary, or
  Context Resolution choice requires new human/product authority rather than
  bounded implementation detail;
- a non-empty kind/profile dependency lacks a typed source/fingerprint producer
  or machine-readable compatibility proof;
- correct implementation reaches a prohibited current product path or HCM-1.3+;
- the full profile cannot be resolved without an ambient/opaque dependency or
  weakening HCM-1.1 security;
- required package/workspace/platform proof is unavailable;
- a public downstream API must be published; or
- mandatory fresh built-in review is unavailable.

## Explicit non-goals

- modifying or deleting `CanonicalArtifactKind` or fixed product descriptors;
- descriptor-driven canonical artifact registry adoption;
- setup, doctor, flow, compiler, CLI, SDK, Tauri, Substrate, or adapter use;
- canonical YAML read/write, migration, materialization, authoring, intake,
  renderer, lifecycle execution, or capitalized Projection behavior;
- evaluating managed-operational-surface truth;
- applying vocabulary labels or Context Resolution envelopes at runtime;
- a legacy/compatibility profile, dual read/write, filename fallback, or
  ambient local override;
- remote registry/network fetch, executable schemas/validators, generated CLI
  commands, or opaque dependency injection;
- closing PG-PROFILE-01, PG-ARTIFACT-01, PG-KIND-01, or PG-KIND-02;
- crate version/publish/downstream proof; and
- HCM-1.3, HCM-1.4, HCM-2.x, HCM-3.x, or unrelated cleanup.
