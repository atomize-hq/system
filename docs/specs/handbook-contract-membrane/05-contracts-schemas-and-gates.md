# Contracts, Schemas, and Gates

## Status

The HCM-0.2 sections are frozen design contracts: schema policy, instance-profile composition for stable-role/schema/kind/instance/vocabulary truth, stable-role and schema registries, artifact kinds, artifact instances, intake records/candidates/promotion, Charter/constitutional-root semantics, validation layers, vocabulary, and the project-posture owner/transition boundary. They are implementation authority for later slice packets, not published API guarantees or evidence that the runtime types exist.

Context Resolution, Projection, Snapshot Memory, SDK/transport, contract/dock, and public-API sections remain preliminary until their named Phase 0 slices close. The shipped artifact-kind/default-instance/requiredness set remains unresolved until HCM-0.6.

## Schema policy

- Canonical human-authored structured records use YAML where appropriate.
- YAML canonical artifacts are parsed into the JSON data model and validated by versioned JSON Schema plus separate semantic validators.
- Every structured record has a stable `schema_id` and version.
- Transport DTOs serialize to JSON and publish JSON Schema.
- OpenAPI is an optional HTTP adapter output, not the CLI/Tauri/SDK authority.
- Schema versions and semantic contract versions are explicit and independently reviewable.
- Unknown required semantics fail closed; optional extension fields require a declared extension mechanism.
- Repository-local schema references are trusted only after repo-relative/no-follow resolution and meta-schema validation; remote fetching, executable schema hooks, and ambient unversioned overrides are initially refused.
- First-version schema, kind, stable-role registry, intake, renderer, capability-contract, posture-trigger/policy, and profile-parent references are exact ID/version references with their required semantic fingerprints. Version ranges, implicit latest selection, and undeclared fallback are refused.
- Canonical normalization and fingerprints exclude absolute machine paths, timestamps unless time is itself semantic input, and presentation-only ordering. They include every semantic value and exact referenced definition fingerprint.

### Uniform exact-definition identity

Every HCM-0.2 versioned definition uses a namespaced stable identity field and SemVer field. Its exact ref is mechanically `identity + "@" + version`; for example, `profile_id: handbook.profile.example` plus `profile_version: "1.0.0"` yields `handbook.profile.example@1.0.0`. A ref that cannot be derived this way, resolves to different identity fields, or resolves more than one record is invalid.

Every such definition also exposes one derived fingerprint field named by its contract (`profile_fingerprint`, `entry_fingerprint`, `registry_fingerprint`, `definition_fingerprint`, `capability_fingerprint`, `intake_definition_fingerprint`, `vocabulary_fingerprint`, `trigger_fingerprint`, `policy_fingerprint`, or another explicitly declared `*_fingerprint`). The common derivation is:

1. parse YAML into the JSON data model;
2. remove only the definition's own derived fingerprint field and fields explicitly classified as presentation-only by its matrix;
3. resolve every exact referenced definition and include its recomputed fingerprint in a typed closure entry, failing on a missing producer, mismatch, cycle, or duplicate identity;
4. sort object keys; preserve semantically ordered arrays; sort arrays declared unordered by their stable identity key;
5. serialize as UTF-8 RFC 8785 JSON Canonicalization Scheme bytes and emit lowercase `sha256:<64-hex>`.

Type-specific matrices may add closure inputs but may not weaken this base. A referenced renderer, lifecycle policy, semantic validator, approval policy, freshness policy, signal, or evidence requirement is admissible even when its later behavioral contract is not yet frozen only if its exact versioned definition already satisfies this identity/fingerprint envelope. Changed bytes behind the same exact ref must change the recomputed fingerprint and invalidate every stale consumer. Bare refs, caller-trusted fingerprints, and versioned records with no fingerprint producer fail closed.

## Instance profile contract

Conceptual minimum:

```yaml
schema_id: handbook.instance-profile
schema_version: "1.0"
profile_id: handbook.profile.example
profile_version: "1.0.0"
profile_scope: named
extends_profile_ref: null
stable_role_registry:
  ref: handbook.roles.core@1.0.0
  fingerprint: sha256:...
schema_registry_sources:
  - handbook.schemas.artifacts.charter@1.0.0
artifact_kind_sources:
  - handbook.artifact-kind.charter@1.0.0
artifact_instances:
  - schema_id: handbook.artifact-instance-descriptor
    schema_version: "1.0"
    id: example_constitutional_root
    kind_ref: handbook.artifact-kind.charter@1.0.0
    role_ref: constitutional_authority
    capability_refs:
      - constitutional_root
    label: Example Constitutional Root
    canonical_path: .handbook/example/root.yaml
    requiredness:
      mode: always
      condition_ref: null
    depends_on: []
    lifecycle_policy_ref: handbook.lifecycle.constitutional-review-lock@1.0.0
    intake_definition_ref: handbook.intake.charter@1.0.0
    renderer_definition_refs: []
    projection_definition_refs: []
    validation_overlay_refs: []
    extensions: {}
vocabulary_ref: handbook.vocabulary.example@1.0.0
context_resolution_ref: null
projection_catalog_refs: []
posture_evaluation_policy:
  ref: handbook.posture-policy.example@1.0.0
  fingerprint: sha256:...
dock_requirement_refs: []
adapter_overlay_refs: []
extensions: {}
profile_fingerprint: sha256:...
```

This is one internally consistent illustrative authored profile, not a proposed shipped profile. Its artifact kind, instance, label, path, renderer selection, and requiredness are examples only and do not freeze the shipped set; that decision remains reserved until `HCM-0.6` completes research and a user brainstorming/decision session.

Required gates:

- unique stable IDs;
- exact parent/source refs with an acyclic profile inheritance chain;
- one exact stable-role registry ref/fingerprint pair shared by every resolved kind and vocabulary record;
- one of `shipped`, `named`, or `repository` scope, with parent scope ordered no narrower than child scope;
- valid repo-relative paths;
- exactly one `always`-required instance selecting the `constitutional_root` capability;
- dependency graph has no invalid references or cycles;
- custom artifacts reference valid schemas;
- artifact instance IDs resolve valid exact kind definitions, stable roles, and compatible semantic capabilities;
- vocabulary conflations are explicit;
- Resolution stack is ordered and connected;
- projection definitions reference valid source roles and target levels.

Field authority/defaulting matrix:

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies the profile record shape | none | exact supported pair | not the profile's semantic version |
| `profile_id`, `profile_version` | profile author identifies one versioned semantic configuration | none | stable ID plus SemVer; unique tuple | no implicit `default` or latest profile |
| `profile_scope` | profile author declares `shipped`, `named`, or `repository` layer scope | none | allowlisted value; ancestry order is shipped -> named -> repository | not runtime authority horizon |
| `extends_profile_ref` | profile author selects one exact parent | `null` | exact resolvable ref; acyclic | no ambient or multi-parent merge |
| `stable_role_registry.ref`/fingerprint | stable-role registry owns role IDs/default labels; profile author selects exact semantics | root: required; child omission inherits; present pair replaces whole | exact versioned ref; supplied fingerprint matches; every kind/vocabulary pair is identical | no ambient registry or label drift |
| `schema_registry_sources` | profile author supplies a complete ordered source list when present | root: required; child omission inherits; present list replaces whole; explicit empty clears | exact refs; unique identities; conflicts within replacement fail | no append merge or remote discovery |
| `artifact_kind_sources` | profile author supplies a complete ordered kind-source list when present | root: required; child omission inherits; present list replaces whole; explicit empty clears | exact refs; unique identities; conflicts within replacement fail | no append merge or enum/template inference |
| `artifact_instances` | profile author supplies a complete instance registry when present | root: required; child omission inherits; present list replaces whole; explicit empty clears and then fails final constitutional validation | unique IDs/paths; valid kinds/roles/capabilities/requiredness/dependencies | no keyed merge/tombstone or reusable kind authority |
| `vocabulary_ref` | profile author selects one exact vocabulary profile | root: required; child omission inherits; present value replaces | resolves and validates | does not rename machine identifiers or commands |
| `context_resolution_ref` | later HCM-0.3 authority | root: explicit `null`; child omission inherits; present value replaces | must remain null until later contract freezes | HCM-0.2 does not freeze Resolution semantics |
| `projection_catalog_refs` | later HCM-0.3 authority | root: explicit empty; child omission inherits; present list replaces whole | must remain empty until Projection contract freezes | fixed renderer refs do not belong here |
| `posture_evaluation_policy` | profile author selects an approved policy ref/fingerprint pair | root: exact pair or `null`; child omission inherits; present pair/null replaces | exact compatible current policy fingerprint | null disables recommendations; no automatic Charter mutation |
| `dock_requirement_refs` | later HCM-0.5 authority | root: explicit empty; child omission inherits; present list replaces whole | exact refs after later contract lands | no executable validator in a profile |
| `adapter_overlay_refs` | adapter/profile integration authority | root: explicit empty; child omission inherits; present list replaces whole | exact declared refs; cannot change domain truth | no transport-owned semantics |
| `extensions` | declaring schema owns namespaced optional additions | root: explicit map; child omission inherits; present map replaces whole | registered namespace and optional-field rules | no key merge or unknown required semantics |
| `profile_fingerprint` | Handbook derives immutable source-layer identity | none; derived for every source profile | uniform exact-definition normalization over authored fields except this fingerprint; referenced definitions are pinned by their exact refs/fingerprints | not the fully resolved profile fingerprint |

All authored fields after `extends_profile_ref` through `extensions` are optional in a child source record specifically so omission and explicit empty/null remain distinguishable. Root profiles materialize every authored field, and every source profile carries a derived `profile_fingerprint`. No v1 invocation request may override these fields; invocation selects an exact profile ref only.

### Resolved instance profile

```yaml
schema_id: handbook.resolved-instance-profile
schema_version: "1.0"
profile_ref: handbook.profile.example@1.0.0
ancestry:
  - profile_ref: handbook.profile.example@1.0.0
    profile_fingerprint: sha256:...
stable_role_registry:
  ref: handbook.roles.core@1.0.0
  fingerprint: sha256:...
schema_registry_entries:
  - entry_ref: handbook.schemas.artifacts.charter@1.0.0
    entry_fingerprint: sha256:...
    closure_fingerprint: sha256:...
artifact_kind_definitions:
  - kind_ref: handbook.artifact-kind.charter@1.0.0
    definition_fingerprint: sha256:...
artifact_instances:
  - schema_id: handbook.artifact-instance-descriptor
    schema_version: "1.0"
    id: example_constitutional_root
    kind_ref: handbook.artifact-kind.charter@1.0.0
    role_ref: constitutional_authority
    capability_refs:
      - constitutional_root
    label: Example Constitutional Root
    canonical_path: .handbook/example/root.yaml
    requiredness:
      mode: always
      condition_ref: null
    depends_on: []
    lifecycle_policy_ref: handbook.lifecycle.constitutional-review-lock@1.0.0
    intake_definition_ref: handbook.intake.charter@1.0.0
    renderer_definition_refs: []
    projection_definition_refs: []
    validation_overlay_refs: []
    extensions: {}
vocabulary:
  vocabulary_ref: handbook.vocabulary.example@1.0.0
  vocabulary_fingerprint: sha256:...
context_resolution_ref: null
projection_catalog_refs: []
posture_evaluation_policy:
  ref: handbook.posture-policy.example@1.0.0
  fingerprint: sha256:...
dock_requirement_refs: []
adapter_overlay_refs: []
extensions: {}
layer_decisions:
  - field: stable_role_registry
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: schema_registry_sources
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: artifact_kind_sources
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: artifact_instances
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: vocabulary_ref
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: context_resolution_ref
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: projection_catalog_refs
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: posture_evaluation_policy
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: dock_requirement_refs
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: adapter_overlay_refs
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
  - field: extensions
    source_profile_ref: handbook.profile.example@1.0.0
    disposition: replaced
diagnostics: []
resolved_profile_fingerprint: sha256:...
```

This is a generic conformance example, not a shipped profile/default decision.

| Resolved field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| schema identity and `profile_ref` | Handbook resolver identifies the derived record and selected exact source profile | none | exact supported schema; source profile exists | resolved view is not editable profile authority |
| `ancestry` | resolver records ordered source identity/fingerprint chain | none; at least selected profile | exact refs/fingerprints; acyclic; legal scope order | no hidden/ambient layer |
| `stable_role_registry` | winning profile layer selects exact role semantics; registry remains authority | none | exact ref/fingerprint; every kind/vocabulary pair matches; all role refs resolve | no ambient registry/default label |
| `schema_registry_entries` | resolver materializes schema definitions and document closures from the winning complete source field | empty only if no kind needs a schema | unique exact refs; current entry fingerprints plus safe document closure fingerprints | a document-only closure fingerprint cannot stand in for definition identity; no source-order conflict repair |
| `artifact_kind_definitions` | resolver materializes exact kinds and fingerprints | empty only if no instance exists | unique exact kinds; all instance refs resolve | no inferred enum kinds |
| `artifact_instances` | winning profile layer remains configuration authority; resolver materializes full descriptors | none; final list must be explicit | all descriptor gates plus exactly one constitutional capability | no keyed merge/tombstone |
| `vocabulary` | winning vocabulary record remains display/workflow mapping authority | none | exact ref/fingerprint and vocabulary gates | no capability/command renaming |
| later-phase refs/lists | their named profile field/later contract owns them | explicit null/empty as shown until later freezes | later contract compatibility | no premature Resolution/Projection/dock behavior |
| `posture_evaluation_policy` | winning profile layer selects policy ref/fingerprint pair | explicit pair or null | compatible exact current policy | null disables recommendations, not policy authority |
| `adapter_overlay_refs` | winning profile layer selects declared overlays | explicit list | exact refs cannot change domain truth | no transport-owned semantics |
| `extensions` | winning profile layer supplies complete namespaced map | explicit map | declared optional namespaces | no key merge/unknown required semantics |
| `layer_decisions` | resolver explains the winning source for every layerable field | none; one decision per field | exact ancestry source and `inherited` or `replaced` disposition | diagnostics do not change authority |
| `diagnostics` | resolver records deterministic warnings/refusals | empty on clean resolution | typed, stable ordering | no silent conflict repair |
| `resolved_profile_fingerprint` | Handbook derives complete semantic identity | none | normalized record plus stable-role registry and every exact definition fingerprint; excludes diagnostics, absolute paths, timestamps, invocation requests | no caller-supplied trust |

Profile conformance scenarios required by implementation packets:

| Scenario | Required result |
|---|---|
| child omits a field | inherits the complete parent value and records `inherited` |
| child supplies a field | replaces the complete parent value and records `replaced` |
| child supplies empty instances or removes constitutional capability | final resolution refuses; no tombstone semantics |
| replacement list contains duplicate/conflicting schema, kind, instance, or path identities | refuses without source-order fallback |
| repository layer replaces paths/requiredness | allowed only as a complete versioned replacement that passes every descriptor/constitutional gate |
| invocation attempts field mutation | refuses; invocation may select only an exact profile plus operation DTO fields |
| same layer bytes/definition fingerprints resolve twice | identical resolved-profile fingerprint and ordered decisions |
| selected role-registry bytes change behind the same ref | stale fingerprint refuses; a new fingerprint changes resolved-profile and vocabulary-default identity |

## Schema registry entry contract

```yaml
schema_id: handbook.schema-registry-entry
schema_version: "1.0"
content_schema_id: handbook.schemas.artifacts.charter
content_schema_version: "1.0.0"
document_ref: handbook.schemas/artifacts/charter/1.0.0.schema.json
document_fingerprint: sha256:...
closure_fingerprint: sha256:...
meta_schema_ref: https://json-schema.org/draft/2020-12/schema
media_type: application/schema+json
compatibility: exact
extensions: {}
entry_fingerprint: sha256:...
```

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies the entry shape | none | exact supported pair | not the content schema identity |
| `content_schema_id`, `content_schema_version` | schema author owns structural schema identity | none | stable ID plus SemVer; unique tuple | no filename-derived identity |
| `document_ref` | schema entry owns local resolution | none | normalized repository-relative regular file; no-follow; inside allowed roots | no URL/network fetch |
| `document_fingerprint` | Handbook derives content identity from exact bytes | none; recomputed on load | lowercase SHA-256 matches resolved document | no trust in a caller-supplied digest alone |
| `closure_fingerprint` | Handbook derives structural-schema closure identity | none | uniform hash over root document and every transitively resolved local `$ref` document fingerprint in deterministic ref order | no omitted or remote dependency |
| `meta_schema_ref` | Handbook schema policy selects the validator dialect | Draft 2020-12 only in v1 | exact allowlisted meta-schema; document passes it | no executable custom meta-schema hook |
| `media_type` | schema entry declares parser expectations | `application/schema+json` | exact allowlisted value | canonical artifact YAML is not the schema document format |
| `compatibility` | Handbook version policy owns resolution posture | `exact` | only `exact` is valid in v1 | no range/latest fallback |
| `extensions` | declaring schema owns namespaced optional additions | empty | declared namespace | no unknown required behavior |
| `entry_fingerprint` | Handbook derives exact versioned registry-entry identity | none | uniform exact-definition fingerprint including document/closure fingerprints | no caller-supplied or document-only identity |

Registry gates:

- `(content_schema_id, content_schema_version)` resolves to exactly one document and fingerprint;
- conflicting sources for the same tuple fail closed rather than using source order to mask drift;
- every local `$ref` follows the same normalized repo-relative/no-follow policy and participates in the resolved fingerprint closure;
- remote `$ref`, executable formats/hooks, unresolved refs, symlink escapes, and byte/fingerprint mismatch are refused;
- passing structural schema validation does not satisfy semantic validation, intake coverage, approval, or a contract gate.

## Stable-role registry contract

```yaml
schema_id: handbook.stable-role-registry
schema_version: "1.0"
registry_id: handbook.roles.core
registry_version: "1.0.0"
roles:
  - role_id: constitutional_authority
    canonical_display_label: Constitutional Authority
    category: governance
  - role_id: project_context
    canonical_display_label: Project Context
    category: artifact
  - role_id: coordination_horizon
    canonical_display_label: Coordination Horizon
    category: workflow
  - role_id: delivery_unit
    canonical_display_label: Delivery Unit
    category: workflow
  - role_id: implementation_unit
    canonical_display_label: Implementation Unit
    category: workflow
  - role_id: execution_envelope
    canonical_display_label: Execution Envelope
    category: workflow
  - role_id: atomic_action
    canonical_display_label: Atomic Action
    category: workflow
registry_fingerprint: sha256:...
```

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies registry shape | none | exact supported pair | not the role-registry semantic version |
| `registry_id`, `registry_version` | Handbook stable-role authority identifies one immutable version | none | stable ID plus SemVer; unique tuple | no implicit core/latest registry |
| `roles[].role_id` | stable-role authority owns machine identity | none; at least one | stable and unique within registry | vocabulary cannot add/rename a role |
| `roles[].canonical_display_label` | stable-role authority supplies deterministic fallback display text | none | non-empty normalized display string | not a profile-selected label |
| `roles[].category` | stable-role authority classifies legal uses/absorptions | none | `artifact`, `workflow`, `governance`, `evidence`, or `organizational` | category does not grant a capability |
| `registry_fingerprint` | Handbook derives exact registry identity | none | SHA-256 over normalized registry fields except this fingerprint | no mutable registry behind a stable ref |

Profiles select exactly one registry ref/fingerprint pair. Every resolved kind and vocabulary record must cite that same pair. Unknown roles, mismatched registry versions/fingerprints, category-invalid absorptions, and changed bytes behind an unchanged ref fail closed. Exact selection, not process-global code, determines canonical fallback labels and role validation.

## Artifact kind definition contract

```yaml
schema_id: handbook.artifact-kind-definition
schema_version: "1.0"
kind_id: handbook.artifact-kind.charter
kind_version: "1.0.0"
compatibility: exact
stable_role_registry:
  ref: handbook.roles.core@1.0.0
  fingerprint: sha256:...
canonical_schema_ref: handbook.schemas.artifacts.charter@1.0.0
supported_role_refs:
  - constitutional_authority
semantic_capabilities:
  - capability_id: constitutional_root
    contract_ref: handbook.capabilities.constitutional-root@1.0.0
    bindings:
      policy_root: /policy
      policy_revision: /policy/revision
      decision_authority: /governance/decision_authority
      required_approvals: /governance/required_approvals
      exception_policy: /governance/exceptions
      engineering_posture_dimensions: /engineering_posture/dimensions
      red_lines: /engineering_posture/red_lines
      review_triggers: /governance/review_triggers
      reassessment_triggers: /governance/reassessment_triggers
structural_validation_profile_ref: json-schema.draft-2020-12
semantic_validation_profile_refs:
  - handbook.semantic-validation.constitutional-root@1.0.0
renderer_definition_refs:
  - handbook.renderer.charter-review-markdown@1.0.0
projection_definition_refs: []
lifecycle_policy_ref: handbook.lifecycle.constitutional-review-lock@1.0.0
review_triggers:
  - handbook.intake-trigger.production-posture-changed@1.0.0
  - handbook.intake-trigger.trust-boundary-changed@1.0.0
required_capabilities: []
extensions: {}
definition_fingerprint: sha256:...
```

Kind gates:

- stable kind/schema IDs and versions are unique;
- canonical schema resolves safely and passes its declared meta-schema;
- semantic capabilities cite exact contracts and valid schema bindings rather than being inferred from filenames;
- fixed renderer and future Projection refs are distinct and resolve compatible exact versions; intake definitions target kinds in the opposite direction and are not kind fields;
- repository-defined kinds pass the same definition validation as shipped kinds;
- no new Rust enum variant or CLI command is required;
- schemas contain no executable hooks or undeclared remote references.

The actual shipped kind/default-instance set is not defined by this illustrative Charter example. It is frozen only by the research and user decision in `HCM-0.6`.

`definition_fingerprint` is derived over the normalized kind record plus the stable-role registry fingerprint and the resolved fingerprints of its schema, capability contracts, semantic validators, renderer definitions, lifecycle policy, and declared extension schemas. It does not include or reference intake definitions; intake definitions point to kinds. It is emitted on the validated definition and excluded from its own hash; authors do not choose it.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies the kind record shape | none | exact supported pair | not kind semantic identity |
| `kind_id`, `kind_version` | kind author owns reusable kind identity | none | stable ID plus SemVer; unique tuple | no path/label/requiredness state |
| `compatibility` | Handbook version policy | `exact` | only `exact` in v1 | no range/latest selection |
| `stable_role_registry.ref`/fingerprint | stable-role registry owns supported-role meaning; kind author pins it | none | exact pair matches resolved profile | no ambient role validation |
| `canonical_schema_ref` | kind author selects structural content authority | none | exact schema-registry ref and fingerprint closure | schema alone does not grant semantic authority |
| `supported_role_refs` | Handbook stable-role registry owns role meaning; kind author declares compatible roles | empty | every ref is a registered stable role compatible with the kind schema | roles do not grant behavioral capabilities |
| `semantic_capabilities[].capability_id` | Handbook semantic registry owns stable capability identity | empty capability list; no per-entry default | registered capability ID | no label/filename-derived capability |
| `semantic_capabilities[].contract_ref` | Handbook capability contract owns meaning and conformance rules | none per entry | exact ref matching the capability ID | no implicit latest contract |
| `semantic_capabilities[].bindings` | kind author maps capability slots into its content schema | none per required slot | every required key maps to a valid compatible JSON Pointer | bindings do not change capability meaning |
| `structural_validation_profile_ref` | Handbook schema policy | Draft 2020-12 | exact allowlisted ref compatible with schema entry | no arbitrary executable validator |
| `semantic_validation_profile_refs` | Handbook or declared built-in semantic validators own cross-field rules | empty | exact refs; all compatible with kind/capabilities | no repository executable hook |
| `renderer_definition_refs` | renderer-definition authors own fixed deterministic pre-Phase-3 renderers | empty | exact refs; deterministic inputs declared | not a capitalized Projection |
| `projection_definition_refs` | later Projection authority | empty and must remain empty until HCM-0.3/Phase 3 authorization | exact refs after later freeze | no precursor generic Projection engine |
| `lifecycle_policy_ref` | kind author selects a Handbook lifecycle policy | `null` only when lifecycle is unrestricted by the kind | exact compatible ref or explicit null | not instance lock state |
| `review_triggers` | kind author declares semantic reassessment triggers | empty | exact versioned trigger refs with fingerprint producers; no duplicates | no direct transport notification behavior |
| `required_capabilities[].capability_id` | kind author declares a cross-artifact semantic prerequisite | empty list; no per-entry default | registered capability ID | no filename dependency |
| `required_capabilities[].contract_ref` | kind author binds the prerequisite to one exact capability contract | none per entry | exact ref whose capability ID matches; one exact version | no range/latest or provider-specific fallback |
| `required_capabilities[].cardinality` | capability contract/kind author declares how many distinct resolved instances must satisfy the prerequisite | none per entry | `exactly_one` or `at_least_one`; count is satisfiable | not shipped-default selection |
| `extensions` | kind schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required semantics |
| derived `definition_fingerprint` | Handbook resolver identifies the complete resolved definition | no authored default | deterministic SHA-256 over semantic closure | no timestamp or absolute-path input |

### Semantic capability contract

```yaml
schema_id: handbook.semantic-capability-contract
schema_version: "1.0"
contract_id: handbook.capabilities.constitutional-root
contract_version: "1.0.0"
capability_id: constitutional_root
required_bindings:
  - policy_root
  - policy_revision
  - decision_authority
  - required_approvals
  - exception_policy
  - engineering_posture_dimensions
  - red_lines
  - review_triggers
  - reassessment_triggers
semantic_validation_profile_refs:
  - handbook.semantic-validation.constitutional-root@1.0.0
allowed_instance_cardinality: exactly_one
extensions: {}
capability_fingerprint: sha256:...
```

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies capability-contract shape | none | exact supported pair | not capability semantic identity |
| `contract_id`, `contract_version` | Handbook semantic registry identifies one exact versioned contract definition | none | namespaced ID plus SemVer; exact ref derives as `contract_id@contract_version`; unique tuple | no implicit latest or ID reconstruction |
| `capability_id` | Handbook semantic registry owns stable capability meaning selected by instances | none | registered stable capability ID; contract ref resolves this same ID | no user label/filename authority |
| `required_bindings` | capability contract owns required semantic slots | none | unique registered binding keys | does not prescribe one artifact schema layout |
| `semantic_validation_profile_refs` | capability contract owns conformance invariants | none for constitutional authority | exact built-in/declarative refs | no repository executable hook |
| `allowed_instance_cardinality` | capability contract owns resolved-profile cardinality | explicit | `exactly_one` or `at_least_one`; `constitutional_root` is `exactly_one` | not artifact-kind or shipped-default cardinality |
| `extensions` | capability schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required behavior |
| `capability_fingerprint` | Handbook derives exact capability-contract identity | none | uniform exact-definition fingerprint including every semantic-validator fingerprint | no stable ref with mutable contract meaning |

A kind's `semantic_capabilities[].bindings` must provide every required binding as a valid JSON Pointer into the exact canonical content schema. Required slots may map to differently named/schema-shaped fields, but the declared semantic validators must prove the capability invariants. This is how custom kinds can conform without copying Charter's filename or schema layout.

V1 cardinality has exactly two values. `exactly_one` requires one distinct resolved provider instance. `at_least_one` requires one or more and resolves the complete provider set in stable instance-ID order; it never chooses one provider implicitly. A kind may contain at most one `semantic_capabilities` entry for a capability ID. Multiple registry versions may exist, but a kind selection or dependency always cites one exact contract version, and the capability ID in the contract must match. Duplicate declarations, zero providers, excess providers for `exactly_one`, mismatched versions, and any definition-closure cycle fail closed.

## Artifact instance descriptor contract

```yaml
schema_id: handbook.artifact-instance-descriptor
schema_version: "1.0"
id: project_context
kind_ref: handbook.artifact-kind.project-context@1.0.0
role_ref: project_context
capability_refs: []
label: Project Context
canonical_path: .handbook/project_context/project-context.yaml
requiredness:
  mode: always
  condition_ref: null
depends_on:
  - target_kind: capability
    target_ref: constitutional_root
    target_contract_ref: handbook.capabilities.constitutional-root@1.0.0
    cardinality: exactly_one
lifecycle_policy_ref: null
intake_definition_ref: null
renderer_definition_refs:
  - handbook.renderer.project-context-review-markdown@1.0.0
projection_definition_refs: []
validation_overlay_refs: []
extensions: {}
```

Instance gates:

- `schema_id` and `schema_version` identify the descriptor record contract independently from the referenced artifact-kind version;
- `kind_ref` resolves exactly;
- path, label, requiredness, and dependencies are repository-instance concerns rather than kind-definition fields;
- selected `role_ref` is supported by the kind and every selected `capability_ref` has a kind capability contract;
- dependencies resolve exact instance IDs or exact semantic-capability contracts without cycles, source-order fallback, or ambiguous provider selection;
- overlays cannot weaken kind schema, constitutional floors, or red lines.

Custom instances use generic operations. Specialized first-party behavior must key off stable capability/role semantics, not filename matching.

The `project_context` descriptor above is a shape example only. It does not select a shipped kind, instance, path, label, renderer, or requiredness decision.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies descriptor shape | none | exact supported pair | not kind schema identity |
| `id` | profile author owns repository instance identity | none | unique stable ID | no filename-derived identity |
| `kind_ref` | profile author selects reusable kind | none | exact resolvable kind and definition fingerprint | no version range/latest fallback |
| `role_ref` | profile author selects one stable semantic role | explicit registered value or explicit `null` | referenced kind lists the role in `supported_role_refs` | role selection does not grant a capability |
| `capability_refs` | profile author selects behavioral/conformance capabilities exposed by the kind | explicit list, possibly empty | unique registered IDs; kind supplies each exact capability contract/binding | capability IDs are not vocabulary roles |
| `label` | profile vocabulary/presentation authority owns instance display text | none | non-empty display string | does not rename ID, path, schema, command, or capability |
| `canonical_path` | profile author owns concrete repository placement | none | normalized repo-relative no-follow path; unique among instances; no template remains after resolution | no absolute path or filename dispatch |
| `requiredness.mode` | profile author owns `always`, `conditional`, or `optional` | none | allowlisted enum; constitutional root is `always` | no inference from current setup/templates |
| `requiredness.condition_ref` | profile author selects deterministic project-condition truth | `null` unless mode is `conditional` | exact condition ref; required iff conditional | no arbitrary expression/script |
| `depends_on[].target_kind` | profile author selects dependency namespace | empty dependency list; no per-entry default | `instance` or `capability` | no ambiguous bare string |
| `depends_on[].target_ref` | profile author selects exact instance ID or registered capability ID | none per entry | matches target namespace and resolves | no filename dependency |
| `depends_on[].target_contract_ref` | profile author binds a capability dependency to one exact contract | exact ref when `target_kind=capability`; explicit `null` when `instance` | contract capability ID matches `target_ref`; every provider exposes that exact contract | no compatible-range or source-order provider selection |
| `depends_on[].cardinality` | profile author declares required target count | none per entry | `exactly_one` or `at_least_one` for capability; only `exactly_one` for unique instance ID; acyclic and satisfiable | not artifact requiredness or an implicit provider choice |
| `lifecycle_policy_ref` | profile author selects instance policy override | explicit exact ref or `null` | compatible with kind; cannot weaken kind/constitutional policy | not current mutable lock state |
| `intake_definition_ref` | profile author explicitly selects intake | exact ref or explicit `null`; no implicit inheritance | definition's exact `artifact_kind_ref` equals selected kind; capability-required constitutional coverage cannot be disabled | kind does not list/fingerprint intakes; no prompt-owned behavior |
| `renderer_definition_refs` | profile author explicitly selects fixed renderers | explicit list, possibly empty | subset of compatible kind refs | not Projection selection |
| `projection_definition_refs` | later Projection authority | explicit empty list until later authorization | compatible exact refs after later freeze | no Resolution-aware view in Phase 2 |
| `validation_overlay_refs` | profile author selects declarative repository constraints | empty | exact schema/declarative refs; only equal/stricter rules | no executable hook or weakened floor |
| `extensions` | descriptor schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required behavior |

The constitutional-root instance adds these gates: it is the only instance whose `capability_refs` contains `constitutional_root`, its requiredness is `always`, its kind capability bindings satisfy the versioned constitutional contract, and no overlay, condition, role/vocabulary mapping, or adapter can remove or weaken that authority. Its `role_ref` remains a separate registered role, normally `constitutional_authority`.

Dependency conformance scenarios required by implementation packets include: zero/one/two providers against both cardinalities; an exact contract-version match and mismatch; duplicate same-capability declarations in one kind; deterministic ordering of multiple `at_least_one` providers; and refusal when an instance dependency uses any cardinality other than `exactly_one`.

## Artifact intake definition contract

```yaml
schema_id: handbook.artifact-intake-definition
schema_version: "1.0"
intake_id: handbook.intake.charter
intake_version: "1.0.0"
artifact_kind_ref: handbook.artifact-kind.charter@1.0.0
candidate_schema_ref: handbook.schemas.artifacts.charter@1.0.0
supported_modes:
  - guided_adaptive
  - express
  - agent_assisted
coverage:
- coverage_id: project_shape.definition
  target_paths:
  - /project_shape
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: delivery.constraints
  target_paths:
  - /delivery_constraints
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: delivery.default_implications
  target_paths:
  - /default_implications/backward_compatibility
  - /default_implications/migration_planning
  - /default_implications/rollout_controls
  - /default_implications/deprecation_policy
  - /default_implications/observability_threshold
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: operational_reality.production_state
  target_paths:
  - /project_conditions/production_state
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: risk.domains
  target_paths:
  - /risk_domains
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: policy.authority_and_revision
  target_paths:
  - /policy
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.decision_authority
  target_paths:
  - /governance/decision_authority
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: named_role_or_owner
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.required_approvals
  target_paths:
  - /governance/required_approvals
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.exception_policy
  target_paths:
  - /governance/exceptions
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: engineering_posture.dimensions
  target_paths:
  - /engineering_posture/dimensions
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_levels
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: engineering_posture.red_lines
  target_paths:
  - /engineering_posture/red_lines
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.review_triggers
  target_paths:
  - /governance/review_triggers
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: governance.reassessment_triggers
  target_paths:
  - /governance/reassessment_triggers
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: explicit_policy
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
- coverage_id: debt.register
  target_paths:
  - /debt
  applicability: always
  authority_class: observational
  acquisition:
    inferable: true
    user_declaration_required: false
    evidence_kinds:
    - repository_configuration
    freshness: session
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: null
  prompt_guidance_refs: []
- coverage_id: decisions.records
  target_paths:
  - /decision_records
  applicability: always
  authority_class: normative
  acquisition:
    inferable: false
    user_declaration_required: true
    evidence_kinds: []
    freshness: null
    sensitivity: internal
    deterministic_default: null
  evaluation:
    required: true
    minimum_specificity: concrete
    minimum_confidence: high
    unknown_policy: block
    contradiction_policy: block
    waiver_policy_ref: handbook.waiver.constitutional-intake@1.0.0
  prompt_guidance_refs: []
approval_policy_ref: handbook.approval.constitutional-candidate@1.0.0
reassessment_triggers:
  - trigger_ref: handbook.intake-trigger.production-posture-changed@1.0.0
    affected_coverage_ids:
      - operational_reality.production_state
  - trigger_ref: handbook.intake-trigger.trust-boundary-changed@1.0.0
    affected_coverage_ids:
      - governance.exception_policy
extensions: {}
intake_definition_fingerprint: sha256:...
```

The exact illustrative Charter intake is coverage-complete for the frozen constitutional capability and retained Charter domains:

| Required constitutional binding | Required coverage ID |
|---|---|
| `policy_root`, `policy_revision` | `policy.authority_and_revision` |
| `decision_authority` | `governance.decision_authority` |
| `required_approvals` | `governance.required_approvals` |
| `exception_policy` | `governance.exception_policy` |
| `engineering_posture_dimensions` | `engineering_posture.dimensions` |
| `red_lines` | `engineering_posture.red_lines` |
| `review_triggers` | `governance.review_triggers` |
| `reassessment_triggers` | `governance.reassessment_triggers` |

| Retained Charter domain | Required coverage ID |
|---|---|
| project shape | `project_shape.definition` |
| delivery constraints | `delivery.constraints` |
| default delivery implications (backward compatibility, migration planning, rollout controls, deprecation policy, observability threshold) | `delivery.default_implications` |
| operational reality | `operational_reality.production_state` |
| risk domains | `risk.domains` |
| posture and engineering dimensions | `engineering_posture.dimensions`, `engineering_posture.red_lines` |
| exceptions and governance | `governance.decision_authority`, `governance.required_approvals`, `governance.exception_policy`, `governance.review_triggers`, `governance.reassessment_triggers` |
| debt | `debt.register` |
| decision records | `decisions.records` |

Meta-validation requires every constitutional binding and every retained domain in these matrices to resolve at least one required coverage item whose target path equals or is an ancestor of the bound schema pointer. Every populated candidate field is source-mapped. Missing, optionalized, unmapped, or conflicting coverage refuses initial promotion.

Intake gates:

- every v1 coverage item maps to one or more valid candidate-schema paths;
- constitutional intake meta-validation proves complete required-binding and retained-domain coverage before the definition fingerprint is admitted;
- shorter modes cannot weaken required coverage or hide unknowns;
- inferable observations record evidence, confidence, freshness, and sensitivity;
- normative/constitutional decisions identify the authority required to approve them;
- question wording may vary by skill/projection, but stable coverage IDs and evaluation semantics do not;
- no intake definition embeds a model/provider call.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies definition shape | none | exact supported pair | not intake semantic identity |
| `intake_id`, `intake_version` | intake author owns coverage-contract identity | none | stable ID plus SemVer; unique tuple | no prompt/version inference |
| `artifact_kind_ref` | intake author selects target kind | none | exact kind ref | no cross-kind candidate mutation |
| `candidate_schema_ref` | intake author selects candidate structural shape | none | exact schema ref matching the kind's canonical schema unless a reviewed normalization contract says otherwise | no weaker mode-specific schema |
| `supported_modes` | intake author permits acquisition paths | none; at least one | subset of `guided_adaptive`, `express`, `agent_assisted`; unique | mode does not change quality bar |
| `coverage` | intake author owns stable coverage semantics | none; at least one for rich intake | unique IDs; deterministic dependency/applicability graph | not terminal question order |
| `coverage[].coverage_id` | intake author owns semantic coverage identity | none | stable unique ID | wording is not identity |
| `coverage[].target_paths` | intake author maps coverage to candidate fields | none; non-empty in v1 | unique valid JSON Pointers in candidate schema | rationale-only/evidence-only non-field coverage awaits a later typed disposition contract; no implicit free-text placement |
| `coverage[].applicability` | intake author owns deterministic applicability | none | `always` or exact declarative condition ref | no script/model judgment |
| `coverage[].authority_class` | Handbook authority taxonomy classifies the value | none | `observational`, `rationale`, or `normative` | evidence does not become normative authority |
| `coverage[].acquisition.inferable` | intake author allows evidence-backed inference | `false` | boolean consistent with authority class | normative policy is not silently inferable |
| `coverage[].acquisition.user_declaration_required` | intake author marks declaration authority | `false` only when omission is explicit | boolean; normative constitutional fields require it unless the approval policy supplies stronger authority | no mode-specific bypass |
| `coverage[].acquisition.evidence_kinds` | intake author allows evidence classes | empty | registered evidence kinds | no embedded validator execution |
| `coverage[].acquisition.freshness` | intake author sets evidence freshness | `null` means no time-based claim, not indefinitely fresh | registered freshness rule | no wall-clock-only authority |
| `coverage[].acquisition.sensitivity` | intake author sets handling class | `internal` | registered sensitivity class and redaction policy | no unrestricted secret capture |
| `coverage[].acquisition.deterministic_default` | intake author may declare a non-inferred default with authority effect | `null` | value validates; source is recorded; forbidden where declaration/approval is required | no silent constitutional default |
| `coverage[].evaluation.required` | intake author sets promotion coverage requirement | `false` | boolean; shorter modes cannot change it | not artifact-instance requiredness |
| `coverage[].evaluation.minimum_specificity` | intake author sets deterministic quality floor | none when required | registered evaluator | no subjective prompt-only judgment |
| `coverage[].evaluation.minimum_confidence` | intake author sets the evidence/declaration confidence floor | none when required | registered ordered confidence value; source kind may impose a higher floor | no agent confidence as authority by itself |
| `coverage[].evaluation.unknown_policy` | intake author sets `block`, `allow_explicit`, or `require_waiver` | `block` for required coverage; otherwise omission is invalid | allowlisted value compatible with authority class and approval policy | no hidden unknown |
| `coverage[].evaluation.contradiction_policy` | intake author sets `block`, `require_resolution`, or `record_warning` | `block` for required normative coverage; otherwise omission is invalid | allowlisted value compatible with authority class | no silent last-write-wins |
| `coverage[].evaluation.waiver_policy_ref` | intake author selects waiver authority when waivers are permitted | `null` means not waivable | exact compatible policy ref | waiver does not equal satisfied truth |
| `coverage[].prompt_guidance_refs` | skill/presentation contract owns optional wording guidance | empty | safe exact refs; guidance cannot change coverage/evaluation | no prompt text as semantic identity |
| `approval_policy_ref` | intake author selects promotion authority policy | exact ref or `null` for non-governed kinds | compatible with kind/capabilities; constitutional intake requires non-null | no agent self-approval |
| `reassessment_triggers[].trigger_ref` | intake author declares a targeted reopen condition | empty trigger list; no per-entry default | exact versioned trigger definition with fingerprint producer; unique | no implicit whole-artifact reassessment |
| `reassessment_triggers[].affected_coverage_ids` | intake author owns the exact coverage subset reopened by the trigger | none per entry; non-empty | every ID exists in this definition; unique and deterministically ordered | an unrelated coverage item does not reopen |
| `extensions` | intake schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required behavior |
| `intake_definition_fingerprint` | Handbook derives exact intake-definition identity | none | uniform exact-definition fingerprint including target kind/schema, approval policy, and trigger-definition fingerprints | no stable intake ref with mutable coverage/authority semantics |

Typed coverage results are `satisfied`, `unknown`, `contradicted`, `waived`, `not_applicable`, or `blocked`. `not_applicable` includes applicability evidence; `waived` includes exact waiver authority and expiry/review rules; neither is equivalent to `satisfied`. Required `unknown`, `contradicted`, or `blocked` coverage prevents promotion.

## Intake record and artifact candidate contracts

```yaml
schema_id: handbook.artifact-intake-record
schema_version: "1.0"
intake_record_id: intake_...
intake_definition_ref: handbook.intake.charter@1.0.0
acquisition_mode: guided_adaptive
target_kind_ref: handbook.artifact-kind.charter@1.0.0
target_instance_id: example_constitutional_root
profile_ref: handbook.profile.example@1.0.0
resolved_profile_fingerprint: sha256:...
consumer:
  kind: handbook_skill
  id: handbook
  version: "..."
coverage_results:
- coverage_id: project_shape.definition
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/project_shape.definition.json
  evidence_refs:
  - evidence.project_shape.definition
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: delivery.constraints
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/delivery.constraints.json
  evidence_refs:
  - evidence.delivery.constraints
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: delivery.default_implications
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/delivery.default_implications.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: operational_reality.production_state
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/operational_reality.production_state.json
  evidence_refs:
  - evidence.operational_reality.production_state
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: risk.domains
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/risk.domains.json
  evidence_refs:
  - evidence.risk.domains
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: policy.authority_and_revision
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/policy.authority_and_revision.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.decision_authority
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.decision_authority.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.required_approvals
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.required_approvals.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.exception_policy
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.exception_policy.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: engineering_posture.dimensions
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/engineering_posture.dimensions.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: engineering_posture.red_lines
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/engineering_posture.red_lines.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.review_triggers
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.review_triggers.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: governance.reassessment_triggers
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/governance.reassessment_triggers.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: debt.register
  applicability: applicable
  source_kind: evidenced_inference
  value_ref: intake-values/debt.register.json
  evidence_refs:
  - evidence.debt.register
  confidence: high
  freshness: session
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
- coverage_id: decisions.records
  applicability: applicable
  source_kind: user_declaration
  value_ref: intake-values/decisions.records.json
  evidence_refs: []
  confidence: high
  freshness: null
  sensitivity: internal
  evaluation: satisfied
  contradiction_refs: []
  waiver_ref: null
prompt_event_refs: []
finalized_at_utc: "..."
record_fingerprint: sha256:...
```

`source_kind` is exactly one of `user_declaration`, `evidenced_inference`, `deterministic_default`, `known_unknown`, `contradiction`, or `waiver`. Approval is a separate authority record, never a value source. Prompt events are descriptive audit references and do not become coverage identity or canonical truth.

```yaml
schema_id: handbook.artifact-candidate
schema_version: "1.0"
candidate_id: candidate_...
intake_record_ref: intake_...
target_kind_ref: handbook.artifact-kind.charter@1.0.0
target_instance_id: example_constitutional_root
target_schema_ref: handbook.schemas.artifacts.charter@1.0.0
profile_ref: handbook.profile.example@1.0.0
resolved_profile_fingerprint: sha256:...
normalized_content_ref: candidates/...yaml
field_sources:
- target_path: /project_shape
  coverage_id: project_shape.definition
  source_kind: evidenced_inference
- target_path: /delivery_constraints
  coverage_id: delivery.constraints
  source_kind: evidenced_inference
- target_path: /default_implications/backward_compatibility
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /default_implications/migration_planning
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /default_implications/rollout_controls
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /default_implications/deprecation_policy
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /default_implications/observability_threshold
  coverage_id: delivery.default_implications
  source_kind: user_declaration
- target_path: /project_conditions/production_state
  coverage_id: operational_reality.production_state
  source_kind: evidenced_inference
- target_path: /risk_domains
  coverage_id: risk.domains
  source_kind: evidenced_inference
- target_path: /policy
  coverage_id: policy.authority_and_revision
  source_kind: user_declaration
- target_path: /governance/decision_authority
  coverage_id: governance.decision_authority
  source_kind: user_declaration
- target_path: /governance/required_approvals
  coverage_id: governance.required_approvals
  source_kind: user_declaration
- target_path: /governance/exceptions
  coverage_id: governance.exception_policy
  source_kind: user_declaration
- target_path: /engineering_posture/dimensions
  coverage_id: engineering_posture.dimensions
  source_kind: user_declaration
- target_path: /engineering_posture/red_lines
  coverage_id: engineering_posture.red_lines
  source_kind: user_declaration
- target_path: /governance/review_triggers
  coverage_id: governance.review_triggers
  source_kind: user_declaration
- target_path: /governance/reassessment_triggers
  coverage_id: governance.reassessment_triggers
  source_kind: user_declaration
- target_path: /debt
  coverage_id: debt.register
  source_kind: evidenced_inference
- target_path: /decision_records
  coverage_id: decisions.records
  source_kind: user_declaration
validation_result_refs:
  - validation_...
unresolved_coverage_ids: []
promotion_eligibility: requires_approval
required_approval_policy_ref: handbook.approval.constitutional-candidate@1.0.0
candidate_fingerprint: sha256:...
```

```yaml
schema_id: handbook.artifact-approval-record
schema_version: "1.0"
approval_id: approval_...
candidate_ref: candidate_...
candidate_fingerprint: sha256:...
approval_policy_ref: handbook.approval.constitutional-candidate@1.0.0
decision: approved
actor_ref: user.project_owner
authority_ref: charter.governance.project_owner
conditions: []
decided_at_utc: "..."
approval_fingerprint: sha256:...
```

```yaml
schema_id: handbook.artifact-promotion-record
schema_version: "1.0"
promotion_id: promotion_...
candidate_ref: candidate_...
candidate_fingerprint: sha256:...
target_instance_id: example_constitutional_root
expected_current_artifact_fingerprint: null
profile_ref: handbook.profile.example@1.0.0
resolved_profile_fingerprint: sha256:...
resolved_definitions:
  - definition_ref: handbook.artifact-kind.charter@1.0.0
    definition_fingerprint: sha256:...
approval_refs:
  - approval_...
validation_result_refs:
  - validation_...
decision: approved
authorized_by_ref: charter.governance.project_owner
canonical_artifact_ref: .handbook/example/root.yaml
canonical_artifact_fingerprint: sha256:...
promotion_fingerprint: sha256:...
```

| Record/fields | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| intake record identity/definition/mode/target, `profile_ref`, `resolved_profile_fingerprint` | Handbook intake engine records the exact acquisition envelope | none | exact refs and supported mode; current fully resolved profile fingerprint | no source-layer fingerprint or mutable session scratchpad as authority |
| `consumer` and `prompt_event_refs` | adapter supplies provenance; Handbook records it | prompt refs empty | typed identity/version; safe refs | agent wording does not define coverage |
| `coverage_results` | Handbook evaluator owns typed per-item result; source records own declared/observed values | one result per applicable/considered item | definition coverage ID; allowed source; evidence/confidence/freshness/sensitivity; typed evaluation | no collapse of declaration, inference, default, unknown, contradiction, or waiver |
| intake `finalized_at_utc` and `record_fingerprint` | Handbook closes the record after acquisition/evaluation and derives immutable provenance identity | none | valid UTC plus deterministic normalized record | no later candidate/approval/promotion forward link or mutation |
| candidate identity/target/schema, `profile_ref`, `resolved_profile_fingerprint` | Handbook candidate builder owns normalized proposal identity | none | exact refs/fingerprints match intake and current fully resolved profile | candidate is not canonical truth |
| `normalized_content_ref` | candidate record owns content-addressed proposal ref | none | safe repo/content-store ref; bytes validate | no untracked inline model output |
| `field_sources` | Handbook maps candidate fields to coverage and value sources | none for populated fields | every populated field has a valid mapping or declared deterministic derivation | no source-free policy value |
| candidate validation/gaps/eligibility/approval policy | Handbook validators and intake policy own readiness | empty gaps only when actually complete; eligibility is explicit | current structural, semantic, coverage, and authority checks | no shorter-mode quality exemption |
| `candidate_fingerprint` | Handbook derives proposal identity | none | normalized content plus exact semantic refs | no timestamp/presentation-only input |
| approval identity/candidate/policy/decision | declared approval policy and authorized actor own a decision over one immutable candidate | none | candidate fingerprint matches; policy current; decision `approved` or `rejected` | approval does not rewrite candidate/intake |
| approval actor/authority/conditions/time | authorized human/role supplies authority; Handbook records event | conditions empty; other fields explicit | actor satisfies authority/policy; conditions validate; UTC valid | agent/validator cannot self-authorize constitutional truth |
| `approval_fingerprint` | Handbook derives immutable approval identity | none | deterministic normalized approval record | no mutable approval state |
| promotion candidate/current, `profile_ref`, `resolved_profile_fingerprint`, and definition fields | Handbook transition engine binds compare-and-write preconditions | none; expected current fingerprint may be explicit null only for create | candidate unchanged; target state/fully resolved profile/definitions current | no stale-candidate overwrite or source-layer fingerprint |
| promotion approvals/authorization/decision | declared approval policy and authorized human/role own authority | no approvals; decision cannot be approved without required refs | approver satisfies current policy; decision allowlisted | agent/validator cannot self-authorize constitutional truth |
| promotion validation refs | Handbook validators own proof of current checks | none | structural, semantic, coverage, authority, and path/write preconditions pass | no reuse of stale validation after drift |
| canonical artifact ref/fingerprint | canonical artifact store owns approved truth after atomic write | absent unless approved write succeeds | safe concrete path; bytes validate; fingerprint matches | promotion record is not a second editable artifact |
| `promotion_fingerprint` | Handbook derives immutable transition identity | none | deterministic normalized record | no wall-clock identity |

Promotion re-resolves the exact profile, kind, schema, intake, capability, renderer, lifecycle, and approval-policy refs; compares the candidate and current target fingerprints; reruns every current validation layer; writes canonical YAML atomically; then emits the immutable promotion record. Any drift, missing approval, unresolved required coverage, unknown required semantic, or failed write leaves canonical truth unchanged and returns a typed refusal.

Record finalization and lineage are strictly append-only:

1. finalize and fingerprint `ArtifactIntakeRecord` after coverage evaluation;
2. build/finalize `ArtifactCandidate` with `intake_record_ref`;
3. write one or more immutable approval records with candidate ref/fingerprint;
4. write promotion only after validating the candidate and required approvals;
5. derive any forward index from downstream refs without rewriting prior records.

Required conformance scenarios include: an empty `target_paths` item refuses in v1; one trigger reopens only its non-empty mapped coverage set; unknown/unmapped triggers refuse; creating a candidate/approval/promotion leaves prior record bytes and fingerprints unchanged; reordered or stale lineage refuses; rejected/wrong-authority approval cannot promote; changing a parent/repository profile layer while retaining the selected `profile_ref` invalidates the old `resolved_profile_fingerprint`; and current candidate/target/fully-resolved-profile/definition fingerprints are required at compare-and-write.

## Charter intake and canonical contract

`CharterIntakeDefinition` is the first rich first-party intake definition. Its coverage must account for the historical domains of project shape, delivery constraints, operational reality, posture and delivery implications, risk domains, engineering dimensions, exceptions/governance, debt, and decision records. Research/design may revise questions and branches, but omissions are explicit decisions. This contract does not decide the shipped default kind/instance set reserved for HCM-0.6.

Guided-adaptive, express, and agent-assisted modes all produce the same canonical Charter candidate schema. The skill-directed LLM agent conducts the conversation and invokes Handbook CLI/SDK operations; Handbook owns coverage/evaluation/promotion and performs no hidden nested synthesis.

Canonical Charter YAML owns approved constitutional truth. Intake provenance explains how it was reached. Before Phase 3, Markdown and other fixed human-review outputs are renderer-derived views; Phase-3 Resolution-aware GUI, packet, and agent-context outputs are Projections. Neither output class is an independently editable authority.

The versioned `constitutional_root` capability contract requires schema bindings for:

- policy root and policy revision;
- decision authority and required approval classes;
- exception/waiver authority and record requirements;
- effective engineering-posture dimensions, floors, and red lines;
- review and targeted reassessment triggers.

Exactly one resolved artifact instance selects this capability and is `always` required. A custom kind may expose the capability only when its exact capability contract, bindings, semantic validators, intake/approval posture, and lifecycle policy pass. A role ref, label, filename, current enum variant, or requiredness flag alone never grants constitutional authority.

Charter-specific defaults are intentionally narrow: absent optional rationale/evidence collections are empty only when the Charter content schema allows it; policy, authority, posture floors/red lines, required approvals, and required coverage have no implicit default. The HCM-0.6 decision may choose a shipped binding/path/label but cannot weaken these semantics.

Explicit non-goals are a rigid CLI questionnaire, prompt-owned policy, agent self-approval, Markdown authority, a second editable posture document, automatic whole-Charter regeneration on one changed condition, or default-set selection by example.

## Artifact validation layers

Do not ask one schema mechanism to own every kind of correctness:

1. **Structural schema** — fields, types, enumerations, requiredness, and local shape after YAML parsing.
2. **Semantic validation** — cross-field, cross-artifact, lifecycle, capability, and authority invariants owned by Handbook.
3. **Intake coverage evaluation** — whether required information was established with appropriate evidence, specificity, confidence, and approval.
4. **External validator docks** — domain-specific witnesses that emit normalized evidence without becoming artifact or contract authority.

A custom kind may need only structural and semantic validation. Human-authored constitutional/governance kinds commonly add intake coverage. Docks remain optional unless the kind/profile/contract requires them.

In Phase 2, a fixed deterministic first-party renderer may produce a renderer-derived human-review view only from validated canonical truth. That view accepts no Context Resolution envelope and is outside the capitalized Phase-3 `Projection` request/result/provenance contract.

Validation order is structural schema -> Handbook semantic/capability validation -> intake coverage/approval when applicable -> required external evidence gates when a later contract demands them. Passing a later layer cannot waive an earlier failure. Unknown required semantics fail closed; warnings never promote blocked truth.

## Vocabulary contract

The vocabulary schema distinguishes labels, aliases, and structural absorption:

```yaml
schema_id: handbook.vocabulary-profile
schema_version: "1.0"
vocabulary_id: handbook.vocabulary.example
vocabulary_version: "1.0.0"
stable_role_registry:
  ref: handbook.roles.core@1.0.0
  fingerprint: sha256:...
labels:
  coordination_horizon: phase
  delivery_unit: feature
  implementation_unit: slice
  execution_envelope: packet
  atomic_action: task
aliases:
  implementation_unit:
    - task
absorptions:
  - unit_id: implementation
    absorbs:
      - implementation_unit
      - execution_envelope
      - atomic_action
extensions: {}
vocabulary_fingerprint: sha256:...
```

Duplicate displayed labels are legal. Ambiguity matters only when a machine operation cannot resolve a stable role from the surrounding typed context.

| Field | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| `schema_id`, `schema_version` | Handbook record-schema registry identifies vocabulary record shape | none | exact supported pair | not vocabulary semantic identity |
| `vocabulary_id`, `vocabulary_version` | vocabulary author owns one versioned mapping | none | stable ID plus SemVer; unique tuple | no implicit local vocabulary |
| `stable_role_registry.ref`/fingerprint | stable-role registry owns keys/default labels/categories; vocabulary author pins it | none | exact pair matches resolved profile and consuming kinds | no ambient registry or mutable fallback label |
| `labels` | vocabulary author owns display labels for registered stable roles | missing role uses the stable-role registry's canonical display label | keys are registered roles; values non-empty; duplicate values allowed | no change to stable role meaning or machine IDs |
| `aliases` | vocabulary author owns accepted display/search aliases | empty map/list | keys registered; aliases normalized; ambiguity surfaced in untyped input | aliases do not select typed operations by themselves |
| `absorptions[].unit_id` | vocabulary author names one local workflow unit | none | stable local ID; unique | not a new Handbook stable role |
| `absorptions[].absorbs` | vocabulary author explicitly combines registered workflow roles for presentation/execution mapping | empty absorptions | registered roles; directed acyclic mapping; one owning absorption per role; adapter preservation checked | cannot absorb constitutional authority, erase approval/evidence boundaries, or change schemas/commands |
| `extensions` | vocabulary schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required behavior |
| `vocabulary_fingerprint` | Handbook derives exact vocabulary identity | none | normalized SHA-256 over all semantic fields plus stable-role registry fingerprint except this field | no unchanged fingerprint after registry/label/absorption drift |

Stable role resolution always uses typed context first. Untyped ambiguous input returns candidates or a typed refusal. Profile vocabulary never renames schema IDs, profile/kind/instance IDs, capability IDs, SDK/JSON operation IDs, or CLI commands. An adapter that cannot preserve structural absorption reports typed loss/refusal rather than silently flattening it.

## Context Resolution envelope

```yaml
schema_id: handbook.context-resolution-envelope
schema_version: "1.0"
active_level: execution
dimensions:
  scope_horizon: assigned_unit
  detail_resolution: normal
  temporal_horizon: immediate
  authority_horizon: local_write
  memory_horizon: execution
  validation_horizon: unit_closeout
parent_refs: []
inherited_constraint_refs: []
allowed_mutations: []
forbidden_mutations: []
escalation_triggers: []
```

Envelope gates:

- active level exists in the selected profile;
- narrower authority cannot override inherited broader constraints;
- allowed and forbidden mutation sets cannot resolve ambiguously;
- validation claims cannot exceed the validation horizon;
- memory writes above the allowed horizon require promotion evidence;
- expansion requests identify the missing context/authority condition.

## Projection request

This capitalized `Projection` request/result pair belongs only to the Phase 3 generic Resolution-aware Projection engine. A Phase 2 fixed deterministic first-party renderer produces a renderer-derived human-review view, not a Projection; it accepts no Resolution envelope and is excluded from this request/result/provenance contract. Generic configured custom-kind Projections and every Resolution-aware view are deferred until the `HCM-3.2` Context Resolution kernel and `HCM-3.3` deterministic Projection engine land.

```yaml
schema_id: handbook.projection-request
schema_version: "1.0"
artifact_refs: []
projection_id: agent.execution-packet
vocabulary_profile_ref: default
resolution_envelope: {}
surface: agent_packet
```

## Projection result

This result is Phase-3-only and is never required for a fixed pre-Phase-3 renderer-derived view.

```yaml
schema_id: handbook.projection-result
schema_version: "1.0"
projection_id: agent.execution-packet
source_refs: []
source_fingerprints: []
resolved_profile_fingerprint: ""
projection_definition_version: "1"
resolution_envelope: {}
operation: reveal
lossiness: lossless
included_claim_refs: []
omitted_claim_refs: []
derived_artifact_refs: []
promotion_eligibility: local_only
```

Allowed `operation` values initially:

- `reveal`;
- `derive`.

Reserve `synthesize_candidate` in the conceptual model but do not implement it in the first projection slice.

## Snapshot capture policy

```yaml
schema_id: handbook.snapshot-capture-policy
schema_version: "1.0"
policy_id: session-boundary.default
triggers:
  - session_start
  - session_end
state_families:
  git:
    include_paths: true
    include_diff_stats: true
    full_diff: artifact_ref_only
  handbook:
    include_profile: true
    include_artifact_fingerprints: true
    include_contract_state: true
  work:
    recent_completed:
      count: 10
      source_ref: work-ledger
      cursor: null
      ordering: completed_at_then_id
    queued_next:
      count: 10
      source_ref: active-plan
      cursor: null
      ordering: canonical_queue_order
  evidence:
    include_latest_gate_refs: true
redaction_policy_ref: snapshot-redaction.default
consistency:
  retries: 2
  unstable_action: persist_non_promotable
retention_policy_ref: snapshot-retention.session
```

For every bounded history window, `cursor: null` means start at the policy-defined initial boundary for the declared ordering. A non-null cursor is an opaque source-issued continuation position, applied exclusively so the item named by the cursor is not repeated. Reusing the same source revision, cursor, count, and ordering must select the same window; changing any of them changes capture input and therefore the policy/capture fingerprint as applicable.

Capture policy gates:

- every selected state family names its authority/source;
- bounded history windows name count, source, cursor, and ordering;
- sensitive content defaults to excluded or artifact-ref-only;
- capture hooks and retention are explicit;
- unstable captures cannot support promotion or closeout;
- changing capture policy changes its fingerprint/version.

## Context Memory Snapshot

Conceptual minimum:

```yaml
schema_id: handbook.context-memory-snapshot
schema_version: "1.0"
snapshot_id: snap_...
capture:
  trigger: session_end
  policy_ref: session-boundary.default
  started_at: "..."
  completed_at: "..."
  producer_version: "..."
  consistency: stable
  pre_revisions: {}
  post_revisions: {}
context_resolution: {}
repository:
  repository_id: "..."
  worktree_id: "..."
  branch: main
  head: "..."
  upstream: "..."
  operation_state: clean
  dirty_paths: []
  untracked_paths: []
  diff_summary: {}
  diff_artifact_refs: []
handbook:
  profile_ref: "..."
  resolved_profile_fingerprint: "..."
  artifact_kind_registry_fingerprint: "..."
  vocabulary_fingerprint: "..."
  resolution_stack_fingerprint: "..."
  artifacts: []
  intake_refs: []
  unresolved_intake_coverage: []
  posture_kernel_ref: null
  posture_recommendation_refs: []
  contracts: []
  verdict_refs: []
  gate_refs: []
work:
  active_refs: []
  recent_completed: []
  queued_next: []
  blocked_refs: []
  deferred_refs: []
  escalation_refs: []
session:
  session_ref: "..."
  handoff_ref: "..."
  dispatch_ref: "..."
evidence:
  validation_refs: []
  unresolved_proof_refs: []
redaction:
  policy_ref: snapshot-redaction.default
  excluded_surfaces: []
previous_snapshot_ref: null
state_fingerprint: sha256:...
record_fingerprint: sha256:...
promotion_eligibility: grounding_only
```

### Snapshot consistency

Supported consistency values:

- `stable` — selected authorities/revisions did not change during capture;
- `bounded` — separately captured surfaces are revision-bound and all remained within declared bounds;
- `unstable` — one or more authorities changed and the retry policy could not obtain a stable/bounded record.

An unstable snapshot remains useful for diagnostics but cannot ground a closeout, promotion, or hard gate.

### Snapshot fingerprints

- `state_fingerprint` covers normalized observed state and excludes volatile capture timestamp/trigger metadata.
- `record_fingerprint` covers the complete immutable record.
- Map keys, paths, work-item windows, and evidence refs use canonical deterministic ordering.
- Two records captured at different times may have equal state fingerprints.

### Snapshot authority

Snapshot Memory is descriptive evidence. It cannot:

- lock or mutate a contract;
- replace canonical artifacts;
- rewrite a queue or handoff;
- infer why a divergence occurred;
- pass claims beyond captured/observed state.

## Snapshot delta

```yaml
schema_id: handbook.snapshot-delta
schema_version: "1.0"
delta_id: delta_...
from_snapshot_ref: snap_previous
to_snapshot_ref: snap_current
compatibility:
  capture_policies_compatible: true
  compared_state_families: []
changes:
  git: {}
  artifacts: []
  contracts: []
  work_completed: []
  work_not_completed: []
  unplanned_work: []
  queue_changes: []
  blockers_added: []
  blockers_cleared: []
  proof_gates_gained: []
  proof_gates_lost: []
signals:
  - kind: expected_progress
    evidence_refs: []
    justification_refs: []
delta_fingerprint: sha256:...
```

Deterministic signal values:

- `expected_progress`;
- `justified_divergence`;
- `unexplained_drift`;
- `scope_expansion`;
- `execution_inefficiency_signal`;
- `planning_inaccuracy_signal`;
- `proof_drift`;
- `semantic_drift`;
- `stale_handoff`.

Signals identify evidence and durable justification refs. They do not make an unreviewed causal claim.

## Snapshot projection request/result

```yaml
schema_id: handbook.snapshot-projection-request
schema_version: "1.0"
snapshot_ref: snap_current
delta_ref: delta_previous_to_current
target_resolution_envelope: {}
purpose: session_grounding
include_families:
  - active_work
  - changed_paths
  - unresolved_blockers
  - queued_next
  - applicable_contracts
  - proof_obligations
```

```yaml
schema_id: handbook.snapshot-projection-result
schema_version: "1.0"
snapshot_ref: snap_current
snapshot_state_fingerprint: sha256:...
delta_ref: delta_previous_to_current
target_resolution_envelope: {}
included_paths: []
omitted_paths: []
grounding_data: {}
lossiness: collapsed
projection_fingerprint: sha256:...
promotion_eligibility: grounding_only
```

Projection gates:

- included fields fit the target Resolution authority and detail horizons;
- omitted sensitive or out-of-scope fields remain enumerated;
- comprehensive capture does not imply comprehensive disclosure;
- grounding projection never mutates the source snapshot;
- snapshot/delta fingerprints remain traceable;
- a new live capture or revision check detects staleness before acting.

## Snapshot redaction and retention

By default snapshots exclude:

- secret values and credential material;
- unrestricted environment variables;
- `.env` and secret-file contents;
- raw command arguments/output that may carry secrets;
- full diffs when normalized statistics/fingerprints and evidence refs suffice.

Snapshots record the redaction policy and excluded surfaces. Retention is profile/policy-driven by horizon and trigger. Immutable retained records may be content-addressed and deduplicated; compaction writes a new reviewed aggregate and never rewrites retained source snapshots.

## Project posture kernel and recommendation contracts

V1 posture levels are global per registered engineering dimension. A `causal_scope_ref` records where trigger evidence applies, but it is not a second posture-state coordinate, does not select a scoped level, and never changes the constitutional authority path. If scoped posture state is later required, it needs a separate contract and migration.

### Posture trigger definition

```yaml
schema_id: handbook.posture-trigger-definition
schema_version: "1.0"
trigger_id: handbook.posture-trigger.public-api-critical
trigger_version: "1.0.0"
trigger_kind: hard
signal_input:
  ref: handbook.signal.public-api-risk@1.0.0
  fingerprint: sha256:...
evidence_requirement_input:
  ref: handbook.evidence-requirement.public-api-risk@1.0.0
  fingerprint: sha256:...
freshness_policy_input:
  ref: handbook.freshness.release-risk@1.0.0
  fingerprint: sha256:...
dimension_id: testing_rigor
causal_scope_ref: artifact.public_api
proposed_transition:
  from: 4
  to: 5
default_confidence: high
default_urgency: before_next_release
required_approval_ref: charter.governance.project_owner
suggested_actions: []
trigger_fingerprint: sha256:...
```

A hard trigger is an immutable versioned definition. `trigger_fingerprint` is SHA-256 over every normalized field except itself and includes the exact signal, evidence-requirement, and freshness-policy fingerprints. One trigger binds one global dimension transition and one causal scope. It cannot authorize mutation, notification delivery, a second dimension, or a scoped authority path.

### Freshness evaluation basis

```yaml
schema_id: handbook.freshness-evaluation-basis
schema_version: "1.0"
basis_id: freshness_basis_...
evaluated_at_utc: "..."
source_states:
  - source_ref: evidence.public-api-risk
    source_fingerprint: sha256:...
    freshness_policy_ref: handbook.freshness.release-risk@1.0.0
    freshness_policy_fingerprint: sha256:...
    valid_from_utc: "..."
    expires_at_utc: "..."
    classification: fresh
basis_fingerprint: sha256:...
```

The freshness basis is an immutable deterministic semantic input, not Snapshot Memory. `classification` is `fresh`, `stale`, or `unknown` as of the explicit evaluation instant under the exact policy. Its fingerprint covers the evaluation instant, every source ref/fingerprint, freshness-policy ref/fingerprint, validity bound, and classification; it excludes `basis_id` and its own fingerprint. Reusing the same basis is stable. Evaluating before versus after expiry produces a different basis/fingerprint.

### Project posture kernel

```yaml
schema_id: handbook.project-posture-kernel
schema_version: "1.0"
kernel_id: posture_kernel_...
constitutional_artifact_ref: .handbook/example/root.yaml
constitutional_artifact_fingerprint: sha256:...
profile_ref: handbook.profile.example@1.0.0
resolved_profile_fingerprint: sha256:...
override_inputs: []
condition_inputs: []
contract_inputs: []
evidence_inputs:
  - ref: evidence.public-api-risk
    fingerprint: sha256:...
snapshot_inputs: []
freshness_basis_ref: freshness_basis_...
freshness_basis_fingerprint: sha256:...
dimensions:
  testing_rigor:
    level: 4
    floor: 3
    red_line_refs: []
    trigger_refs: []
    allowed_shortcut_refs: []
    proof_obligation_refs: []
applicable_scope_refs: []
omitted_condition_refs: []
unresolved_condition_refs: []
recommendation_refs: []
resolved_at_utc: "..."
input_fingerprint: sha256:...
kernel_fingerprint: sha256:...
```

`ProjectPostureKernel` is derived, not independently editable canonical truth. Every `*_inputs` entry is a typed `{ref, fingerprint}` pair. `input_fingerprint` covers schema identity, constitutional ref/fingerprint, exact profile ref plus `resolved_profile_fingerprint`, every normalized input pair, and the freshness-basis ref/fingerprint whenever any input is freshness-qualified. It excludes `kernel_id`, resolved output, recommendation refs, `resolved_at_utc`, and both fingerprint fields. `kernel_fingerprint` covers `input_fingerprint` plus global dimension state, applicability/explanation refs, and other resolved semantic output; it excludes `kernel_id`, recommendation refs, `resolved_at_utc`, and its own field. Lists whose order has no meaning are sorted by stable keys before hashing. Changed bytes behind a stable ref require a changed fingerprint or refusal.

### Posture evaluation policy

```yaml
schema_id: handbook.posture-evaluation-policy
schema_version: "1.0"
policy_id: handbook.posture-policy.example
policy_version: "1.0.0"
hard_trigger_inputs:
  - ref: handbook.posture-trigger.public-api-critical@1.0.0
    fingerprint: sha256:...
accumulated_signal_rules:
  - rule_id: repeated_release_regressions
    signal_input:
      ref: handbook.signal.release-regression@1.0.0
      fingerprint: sha256:...
    evidence_requirement_input:
      ref: handbook.evidence-requirement.release-regression@1.0.0
      fingerprint: sha256:...
    freshness_policy_input:
      ref: handbook.freshness.release-risk@1.0.0
      fingerprint: sha256:...
    dimension_id: testing_rigor
    causal_scope_ref: artifact.public_api
    comparator: at_least
    threshold: 3
    window: P30D
    minimum_observations: 3
    proposed_transition:
      from: 4
      to: 5
    default_confidence: high
    default_urgency: before_next_release
    required_approval_ref: charter.governance.project_owner
    suggested_actions: []
    rule_fingerprint: sha256:...
lowering_evidence_window: P90D
cooldown: P30D
recipient_refs:
  - charter.governance.project_owner
acknowledgment_required: true
escalate_after: P7D
extensions: {}
policy_fingerprint: sha256:...
```

Each accumulated rule is local to one exact policy version. `rule_fingerprint` covers every normalized rule field and cited input fingerprint except itself. A recommendation cites the policy pair, `rule_id`, and `rule_fingerprint`; it does not invent a synthetic external trigger ref. `policy_fingerprint` covers normalized policy identity/configuration, ordered hard-trigger pairs, accumulated rules including their fingerprints, notification semantics, hysteresis, extensions, and referenced definition fingerprints, excluding only itself. Hard triggers and accumulated rules each bind one global dimension transition and one causal scope.

### Posture recommendation

```yaml
schema_id: handbook.posture-recommendation
schema_version: "1.0"
recommendation_id: posture_rec_...
kernel_ref: posture_kernel_...
kernel_fingerprint: sha256:...
evaluation_policy_ref: handbook.posture-policy.example@1.0.0
evaluation_policy_fingerprint: sha256:...
proposed_transition:
  dimension_id: testing_rigor
  from: 4
  to: 5
causal_scope_ref: artifact.public_api
trigger_source:
  kind: hard_trigger
  ref: handbook.posture-trigger.public-api-critical@1.0.0
  fingerprint: sha256:...
  rule_id: null
  rule_fingerprint: null
triggering_observations:
  - public API blast radius is critical
evidence_inputs:
  - ref: evidence.public-api-risk
    fingerprint: sha256:...
snapshot_delta_inputs: []
confidence: high
urgency: before_next_release
required_approval_ref: charter.governance.project_owner
notification:
  recipient_refs:
    - charter.governance.project_owner
  acknowledgment_required: true
  escalate_after: P7D
suggested_actions: []
promotion_eligibility: recommendation_only
recommendation_fingerprint: sha256:...
```

V1 permits exactly one `proposed_transition` per recommendation. For `trigger_source.kind: hard_trigger`, `ref` and `fingerprint` are required and both rule fields are null. For `accumulated_rule`, the trigger ref/fingerprint fields are null while `rule_id` and `rule_fingerprint` identify a rule inside the cited exact policy. The transition, causal scope, confidence, urgency, approval, actions, evidence/freshness requirements, and notification must match the selected trigger/rule and policy. Two dimension changes produce two recommendations; recommendation grouping is presentation-only.

### Posture transition

```yaml
schema_id: handbook.posture-transition
schema_version: "1.0"
transition_id: posture_transition_...
recommendation_ref: posture_rec_...
recommendation_fingerprint: sha256:...
source_kernel_ref: posture_kernel_...
source_kernel_fingerprint: sha256:...
target_authority_ref: .handbook/example/root.yaml
target_authority_fingerprint: sha256:...
target_authority_class: constitutional_root
change:
  authority_path: /engineering_posture/dimensions/testing_rigor/level
  dimension_id: testing_rigor
  operation: replace
  expected_value: 4
  proposed_value: 5
approval_inputs:
  - ref: approval_...
    fingerprint: sha256:...
authorized_by_ref: charter.governance.project_owner
reassessment:
  intake_definition_ref: handbook.intake.charter@1.0.0
  affected_coverage_ids:
    - engineering_posture.dimensions
  validation_result_inputs:
    - ref: validation_...
      fingerprint: sha256:...
effective_at_utc: "..."
resulting_authority_fingerprint: sha256:...
resulting_kernel_ref: posture_kernel_...
resulting_kernel_fingerprint: sha256:...
transition_fingerprint: sha256:...
```

| Record/fields | Owner and authority | Default/omission | Required validation | Explicit non-goal |
|---|---|---|---|---|
| trigger identity/kind | trigger author owns one immutable hard-trigger definition | none | exact schema; SemVer identity; `hard` is the only external trigger kind in v1 | accumulated rules are policy-local, not fake external triggers |
| trigger signal/evidence/freshness input pairs | source definition owners retain authority; trigger pins them | none | exact ref/fingerprint pairs; compatible definitions | no bare mutable ref or model-only evidence rule |
| trigger dimension/causal scope/transition | trigger author binds one deterministic proposal | none | registered global dimension; typed causal scope; `from`/`to` obey floors/red lines/direction | scope is not a posture-state coordinate or authority path |
| trigger guidance and fingerprint | trigger/policy author owns defaults; Handbook derives identity | actions empty; other guidance explicit | confidence/urgency/approval allowlisted; normalized fingerprint covers full semantic closure | trigger cannot authorize mutation or notification delivery |
| freshness-basis identity/evaluation instant | Handbook freshness evaluator records one immutable evaluation boundary | none when freshness-qualified input exists | exact schema; valid UTC; unique ID | presentation time is not a substitute |
| freshness source/policy pairs, validity, classification | source/policy remain authority; evaluator records state | bounds only as policy permits; classification required | exact current fingerprints; interval/order/classification agree at instant | stale does not silently count as fresh |
| `basis_fingerprint` | Handbook derives semantic time/applicability input | none | normalization stated above | no wall-clock lookup outside the record |
| kernel identity/constitutional/profile inputs | resolver records derived identity and exact constitutional plus fully resolved-profile inputs | none | exact ref/fingerprint pairs; `resolved_profile_fingerprint` current | no source-layer `profile_fingerprint` or second authority |
| kernel override/condition/contract/evidence/snapshot inputs | each source remains its own authority; resolver records exact pairs | lists empty except as applicable | safe refs; current fingerprints; changed bytes change/refuse | no observations rewriting Charter policy |
| kernel freshness pair | freshness evaluator supplies applicability truth | explicit null pair only when no input is freshness-qualified | basis covers every qualified source | `resolved_at_utc` cannot drive freshness |
| kernel global `dimensions` | resolver derives effective global posture | none for required dimensions | registered dimension; levels/floors/red lines/triggers/shortcuts/proofs obey authority | no per-scope posture level in v1 |
| kernel applicability/condition/recommendation refs | resolver explains applicability and advisory outputs | lists empty | typed refs; unresolved required condition prevents overclaim | scope refs do not create state coordinates; recommendation refs excluded from fingerprint |
| kernel timestamp and fingerprints | resolver records presentation time; Handbook derives semantic identity | none | explicit normalization above; same inputs/basis replay exactly | no ambient clock or presentation timestamp input |
| policy identity/hard-trigger pairs | policy author owns mechanics and pins exact hard triggers | hard-trigger list empty | exact schema/SemVer/ref/fingerprint; unique trigger refs | no ambient trigger registry |
| accumulated rule identity and input pairs | policy author owns one local repeated-signal rule | rule list empty; no per-rule defaults | unique ID; exact signal/evidence/freshness pairs | local rule is not an external trigger ref |
| accumulated dimension/scope/comparator/threshold/window/count | policy author owns deterministic bounded semantics | none per rule | registered global dimension; causal scope; allowlisted comparator; bounded numeric threshold/window/count | no unscoped, unbounded, or model-only inference |
| accumulated transition/guidance/rule fingerprint | policy author binds one proposal; Handbook derives rule identity | actions empty; other fields explicit | `from` matches kernel; `to` respects authority; full normalized fingerprint | no multi-dimension or per-scope state mutation |
| policy hysteresis/notification/extensions | policy owns lowering and delivery obligations | lowering disabled unless both window/cooldown set; recipients empty; acknowledgment false; escalation null; extensions empty | durations/refs valid; lowering cannot cross floors/red lines | adapters deliver but cannot change semantics |
| `policy_fingerprint` | Handbook derives exact policy identity | none | normalization stated above includes trigger/rule/notification closure | no changed policy behind same fingerprint |
| recommendation kernel/policy pairs | evaluator owns advisory proposal; sources remain authority | none | exact current pairs | recommendation is not enacted policy |
| recommendation `proposed_transition` and causal scope | evaluator copies exactly one trigger/rule-bound proposal | none | one global dimension; `from` matches kernel; `to` valid; causal scope matches source | no second transition, scoped level, or scoped authority path |
| recommendation trigger source | evaluator selects hard trigger or policy-local accumulated rule | mutually exclusive null fields as stated above | exact current trigger pair or exact policy rule fingerprint | no ambiguous trigger namespace |
| recommendation evidence/snapshot inputs | source records remain authority | evidence required unless bound definition supplies sufficient basis; snapshot list empty | exact fingerprints; freshness sufficient; unexplained drift not causal alone | no evidence-free/bare-ref authority |
| recommendation guidance/notification | selected trigger/rule and policy remain authority | actions empty; other fields explicit | values match source and policy exactly | no per-transition ambiguity because v1 has exactly one transition |
| recommendation eligibility/fingerprint | Handbook fixes advisory-only status and derives immutable identity | `recommendation_only` | normalized SHA-256 over full semantic closure except ID/fingerprint | no direct mutation or omitted mutable input |
| transition recommendation/kernel/target pairs and class | transition engine binds compare-and-write state | none | current fingerprints; class exactly `constitutional_root`; target satisfies capability bindings | no approved-override target or bare ref |
| transition `change` | canonical policy owner identifies one bound global-dimension mutation | none | exactly one `replace`; path matches constitutional dimension binding; expected/proposed match recommendation/kernel | no patch script, scoped path, or unrelated field |
| transition approval inputs/authorized actor | constitutional authority owns approval | none | exact approval pairs and required authority; hysteresis/floors/red lines hold | agent/evaluator cannot self-approve |
| transition reassessment | intake definition owns affected coverage; transition records proof | none | exact intake ref; non-empty mapped coverage IDs; current validation pairs pass | no whole-Charter or unrelated reopen |
| transition effective/result pairs/fingerprint | transition engine records atomic write and re-resolution | none after success | authority/kernel results replay; fingerprint covers full record/input fingerprints except itself | no mutable history or success without resulting-kernel proof |
| all `extensions` | declaring schema owns namespaced optional additions | empty | declared namespace/schema | no unknown required semantics |

Recommendation and transition gates:

- every recommendation cites one current kernel/policy pair, one exact hard trigger or accumulated rule, and sufficient exact evidence inputs;
- hard triggers and accumulated rules remain distinct; unexplained snapshot drift cannot silently become causal evidence;
- a causal event requiring multiple dimensions produces separate recommendations with independent approvals and fingerprints;
- recommendations are advisory and never modify canonical Charter or override state;
- lowering requires configured sustained evidence and cooldown and cannot cross floors or red lines;
- notification delivery is adapter-owned and cannot change recipient, acknowledgment, approval, or escalation semantics;
- only an authorized compare-and-write `PostureTransition` revalidates mapped intake coverage, updates the canonical constitutional-root authority atomically, and re-resolves the kernel; override mutation is refused in v1;
- a failed, stale, unapproved, fingerprint-mismatched, scoped-path, or invalid transition leaves canonical authority unchanged and returns a typed refusal.

Required conformance scenarios for later implementation packets:

| Scenario | Required result |
|---|---|
| same semantic sources and same freshness basis resolve twice | identical input/kernel fingerprints and global dimensions |
| parent/repository profile changes behind same selected profile ref | old `resolved_profile_fingerprint` refuses; new one changes kernel input fingerprint |
| source/policy/trigger bytes change behind same ref | stale fingerprint refuses; new fingerprint changes dependent identities |
| same sources evaluated immediately before and after expiry | different basis/input fingerprints; classification/output changes honestly |
| hard trigger and normalized policy resolve twice | identical trigger/policy fingerprints |
| accumulated rule misses count/window/freshness threshold | no recommendation |
| accumulated rule meets exact bounded semantics | recommendation cites policy/rule fingerprints and exact evidence |
| one event warrants two dimension changes | two recommendations, never one multi-transition record |
| causal scope differs while global dimension state is the same | scope is retained as evidence metadata; `from` still reads the global dimension |
| recommendation notification differs from policy or guidance differs from trigger/rule | refusal |
| lowering lacks sustained window/cooldown or crosses floor/red line | refusal |
| transition path is scoped/outside binding, operation is not `replace`, or target is an override | refusal without canonical mutation |
| transition expected value/kernel/target/recommendation fingerprint is stale | compare-and-write refusal |
| actor/approval, mapped reassessment proof, or resulting-kernel fingerprint is missing/mismatched | invalid transition proof and no success claim |

## Optional synthesis-candidate contract

If later approved:

```yaml
operation: synthesize_candidate
producer:
  adapter: unified-agent-api
  provider_family: codex
  model: "..."
input_projection_refs: []
prompt_contract_ref: "..."
candidate_artifact_ref: "..."
promotion_eligibility: requires_review_and_lock
```

The exact UAA model/provider contract is loaded from the live `unified-agent-api` repository only for that future slice.

## Development orchestration contracts

These contracts govern development of Handbook through this control pack. They do not define a Handbook product-runtime agent engine.

### Dispatch execution envelope

The only normative current machine example is
`handoffs/internal-dispatch-template.json`. It is validated against
`handoffs/internal-dispatch.v1.1.schema.json` by `validate_handoffs.py`; do not
maintain a second partial YAML shape that can drift from the schema. Every
current internal dispatch includes the complete identity and timing fields,
parent and source lineage, execution contract, ordered skill chain, explicit
phase/slice/packet authority, replayable subject manifest, active Resolution,
authority and repo-truth statements, bounded scope/tasks/gates/stops, and the
complete structured return contract.

`execution_target` is one of:

- `internal_subagent` — default for delegable implementation, documentation, proof, remediation, and review work; the active parent executes it immediately through built-in subagent tools;
- `top_level_resume` — used only when a genuine context/runtime or authorization boundary requires another top-level task;
- `human_interactive` — used only when the next action requires user judgment, interactive observation, approval, or another non-delegable human action.

Rules:

1. `internal_subagent` uses the built-in fresh `default` agent capability with isolated context; shell-launched Codex processes, background jobs, temporary-file reviewer transport, and filesystem polling do not satisfy it.
2. Every dispatch declares an ordered `required_skills` chain beginning with `using-agent-skills`; the resolved skill workflows are mandatory, not advisory labels.
3. Review agents are read-only and receive authority, diff/evidence, gates, and non-goals without implementation reasoning or prior reviewer conclusions.
4. Internal agents return structured results to the parent and never append the global handoff ledger.
5. The parent validates findings against live truth, remediates valid findings directly or through a fresh fix agent, reruns verification, and sends the resulting state to a different fresh reviewer.
6. The parent owns integration, control-pack truth, final proof, commit, and top-level closeout.

### Parent-owned delegated-run evidence

New top-level handoffs record each proof-relevant built-in delegation:

```json
{
  "run_id": "review-round-1",
  "dispatch_id": "20260714T120000Z--HCM-0-1--independent-review",
  "dispatch_ref": "docs/specs/handbook-contract-membrane/handoffs/dispatches/20260714T120000Z--HCM-0-1--independent-review.json",
  "dispatch_fingerprint": "sha256:...",
  "role": "review",
  "agent_id": "independent_review",
  "agent_type": "default",
  "fresh_context": true,
  "required_skills": ["using-agent-skills", "code-review-and-quality"],
  "subject_fingerprint": "sha256:...",
  "result_subject_fingerprint": "sha256:...",
  "review_round": 1,
  "predecessor_run_id": null,
  "remediation_for_run_ids": [],
  "final_status": "completed",
  "verdict": "clean",
  "finding_refs": [],
  "evidence_refs": ["docs/specs/handbook-contract-membrane/handoffs/dispatches/...json"]
}
```

Handoff schema v1.2 adds required `orchestration_id`, `source_handoff_ids`, `delegation_capability`, `delegated_runs`, typed `remediations`, `reviewed_state`, `stop_reason`, and `resume`. `stop_reason` is one of `completed`, `human_input`, `external_blocker`, `authority_boundary`, `context_boundary`, or `capability_unavailable`.

A v1.2 handoff with `status: completed` must use `stop_reason: completed`, contain a completed clean fresh review run bound to `reviewed_state.subject_fingerprint` and its replayable manifest, and require no top-level resume. Human, external, authority, context, and capability stop reasons map to explicit permitted status/resume targets; v1.2 cannot emit historical `review_required`. A `capability_unavailable` stop is `blocked`. Dispatch/result parent, slice, packet, role, skills, and subject fingerprints must match. Earlier records and schemas remain immutable historical evidence.

The subject manifest sorts repository-relative paths and encodes each entry as `path + NUL + lowercase SHA-256 + newline`; the aggregate is SHA-256 over the concatenated encoded entries. Dispatch validation always recomputes the aggregate from the stored entries. At execution, the parent and reviewer verify every entry against the live subject. A completed v1.2 closeout verifies the final clean manifest against `reviewed_state.baseline_head`, the primary reviewed commit, while ledger parity is validated separately against the post-closeout record set. This preserves exact review binding even when the mechanical second commit adds the parent record and rebuilds `ledger.jsonl`. Earlier review manifests remain immutable identities of superseded pre-remediation subjects and are not incorrectly compared with the later repaired tree.

`delegation_capability.status: unavailable` and `stop_reason: capability_unavailable` are bidirectionally coupled. Unavailable mandatory delegation cannot be hidden under a human, external, authority, or context stop reason.

Every findings review in a completed closeout has a typed remediation record. Its owner is either `parent_orchestrator` with durable evidence or a completed delegated `remediation` run. The remediation names the findings run, result fingerprint, and different-fresh completed re-review run. That re-review may itself report findings; if it does, it receives its own typed remediation and another different fresh review. The last completed review must be clean. Failed or wrong-role work cannot satisfy remediation lineage.

### Review choreography

```text
implement or document
  -> verify
  -> fresh read-only reviewer
      -> clean: proof wall and closeout
      -> actionable findings:
           validate findings
           -> parent repair or fresh remediation agent
           -> verify
           -> different fresh reviewer
           -> repeat until clean or genuinely blocked
```

The orchestrator may not self-approve. A dispatch artifact proves a bounded job was specified; only captured built-in agent identity/status plus reconciled results prove that the job was executed.

## SDK command contract

Every ordinary use case has:

- a typed request;
- a typed result;
- structured expected blocked/refused states;
- a stable operation identifier;
- schema and capability versions;
- deterministic serialization.

Snapshot use cases should include capture, compare/delta, project, inspect, and verify-current operations without forcing callers to parse git or handoff prose themselves.

Artifact/intake use cases should include kind/instance discovery, structural/semantic validation, coverage inspection, evidence/declaration submission, candidate validation, promotion, projection, posture inspection, and recommendation acknowledgment. Operation IDs remain generic and select kind/instance IDs; custom kinds and vocabulary never create or rename CLI commands.

Proposed common response envelope:

```json
{
  "schema_id": "handbook.command-response",
  "schema_version": "1.0",
  "operation": "contract.verify",
  "status": "ok",
  "data": {},
  "diagnostics": [],
  "next_actions": [],
  "artifact_refs": [],
  "provenance": {}
}
```

Status values:

- `ok`;
- `blocked`;
- `refused`;
- `error`.

Domain verdicts such as contract `fail` remain inside `data`; they are not collapsed into process/transport failure.

## CLI JSON contract

For every nontrivial command in JSON mode:

1. stdout contains exactly one complete JSON response document;
2. human progress and logs use stderr;
3. expected blocked/refused outcomes still serialize the envelope;
4. exit codes are stable and separately documented;
5. no ANSI styling appears in JSON;
6. large/binary outputs use artifact references;
7. the response reports capability/schema versions;
8. human output renders from the same typed result.

The initial Substrate CLI bridge consumes this contract only. It never parses human text.

## Contract record

```yaml
schema_id: handbook.contract
schema_version: "1.0"
contract_id: example.operation.v1
kind: operation
status: locked
authority_resolution: coordination
subject: {}
claims: []
required_evidence: []
scoring: {}
```

Lifecycle target:

```text
draft -> review_ready -> locked -> active -> passed|blocked -> closed
                                  \-> deprecated
```

Lifecycle transitions require typed evidence and authority. A rendered document or passing test cannot lock or close a contract by itself.

## Evidence record

Evidence identifies:

- contract and claim refs;
- producer/dock identity and version;
- subject and case;
- observed facts;
- artifact/trace refs;
- source fingerprints;
- Context Resolution envelope;
- included and unobserved claims;
- execution provenance;
- confidence/flakiness where applicable.
- snapshot and delta refs when point-in-time state supports the observation.

## Verdict contract

Supported verdicts:

- `pass`;
- `fail`;
- `blocked`;
- `warning`;
- `not_observed`;
- `not_applicable`;
- `flaky`.

Rules:

- hard-fail severity blocks regardless of aggregate score;
- required `not_observed` cannot become pass through weighting;
- evidence outside the required Resolution/subject boundary cannot satisfy a claim;
- flaky evidence remains visible and cannot silently average into green;
- verdicts cite evidence records.

## Gate contract

A gate identifies:

- target contract and lifecycle transition;
- claim verdicts;
- hard blockers;
- weighted score if used;
- local closeout eligibility;
- parent promotion eligibility;
- remediation/next actions;
- evidence and projection refs.
- snapshot/delta refs used for grounding, staleness, or drift decisions.

Local completion and broader promotion are separate decisions.

## Dock capability manifest

```json
{
  "schema_id": "handbook.dock-capability",
  "schema_version": "1.0",
  "dock_id": "example.validator",
  "dock_protocol_versions": ["1.0"],
  "contract_kinds": ["operation"],
  "evidence_kinds": ["schema_validation"],
  "input_media_types": ["application/json"],
  "resolution_support": {
    "minimum_level": "execution",
    "dimensions": ["scope_horizon", "detail_resolution", "validation_horizon"]
  },
  "execution": {
    "mode": "process",
    "supports_timeout": true,
    "supports_cancellation": true
  }
}
```

## Dock request/result

Request includes:

- protocol and schema versions;
- contract and selected claim refs;
- Resolution envelope;
- canonical/projection/evidence artifact refs;
- snapshot/delta refs when the validator needs a point-in-time state boundary;
- workspace capability grant;
- timeout/cancellation policy;
- requested evidence types.

Result includes:

- dock identity/version;
- structured status/refusal;
- normalized evidence refs;
- observed/unobserved claim refs;
- diagnostics;
- produced artifact refs;
- exact Resolution/provenance;
- execution timing and completion state.

## Public API proof gates

### CLI bridge gate

- exact Handbook binary version;
- exact JSON schema/capability versions;
- real Substrate seam;
- no human-output parsing;
- explicit replacement boundary.

### Published Rust API gate

A downstream-intended API is complete only when:

1. relevant crates are published to crates.io;
2. a registry-only external consumer resolves exact versions;
3. a dedicated Substrate worktree from current tip imports those exact versions;
4. a real seam uses the API;
5. the proof wall passes;
6. no sibling-path or unpublished fallback exists.

The CLI bridge may ship earlier, but it cannot satisfy this Rust API gate.

## Schema compatibility posture

This greenfield program does not promise compatibility with legacy Handbook artifact formats.

Compatibility rules apply only after a new schema/API is deliberately published as supported. Temporary internal cutover types must name their deletion gate and cannot become implicit public contracts.
