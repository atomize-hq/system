# HCM-1.3 Specification: Descriptor-Driven Artifact-Instance Registry

## Status and authority

This is the complete planning-only implementation packet for `HCM-1.3`. It is
authority for a future, separately selected implementation session only after
the exact planning subject receives a fresh `CLEAN` review and the parent
planning closeout records that result. It does not authorize implementation in
the session that creates or closes this packet.

The packet consumes, without reopening, the reviewed HCM-1.1 kind/schema
registry boundary and the reviewed HCM-1.2 profile/descriptor boundary. The
entry evidence is:

- branch `feat/handbook-contract-membrane`;
- planning entry HEAD `c5733785fbd60b7d7a19318cb86058395a02e1c3`;
- HCM-1.2 reviewed implementation commit
  `832716a66241bdcf86e2a82ffb3ae72680a7c2cd`;
- HCM-1.2 reviewed subject fingerprint
  `sha256:d120149e21831c2009d9708c71c335482a97a10d8d975b8246742ed07fe64c71`;
- selected HCM-1.2 closeout
  `20260717T125103Z--HCM-1-2--orchestration--profile-boundary-landed`; and
- completed HCM-1.1 implementation ancestry at
  `0bc51a9cc282581143a5b21f50162456aa32154c`.

The HCM-1.2 closeout is dependency and transition evidence only. It neither
changes this slice's scope nor selects an implementation session. Live source,
tests, package proof, control-pack authority, and Git history at the entry HEAD
override stale narrative wording.

## Objective

Make the artifact universe selected by one `ResolvedInstanceProfile` owned by
one descriptor-driven engine registry rather than by
`CanonicalArtifactKind`, `CANONICAL_ARTIFACT_ORDER`, fixed descriptor arrays,
or fixed path structs. The registry must bind every selected artifact instance
to its exact kind, stable role, capabilities, structural schema, semantic
validator metadata, requiredness, dependency providers, and already-admitted
trusted repository-relative path.

The result is an additive engine owner API. A repository-defined custom kind
and custom artifact-instance ID must enter the selected registry without a new
Rust enum variant, generated command, filename switch, ambient discovery, or
product adoption. Current pre-membrane Markdown setup, doctor, authoring,
manifest, freshness, flow, and rendering paths remain unchanged and
non-authoritative for the selected-profile universe until their owning later
slices cut them over.

## Boundary decisions

### What "replace the enum-owned universe" means here

After HCM-1.3, the only engine owner API for the complete artifact universe of
a selected profile is `ResolvedArtifactRegistry`. Its membership comes solely
from `ResolvedInstanceProfile::artifact_instances()`; its kinds come solely
from `ResolvedInstanceProfile::artifact_kind_registry()`. No HCM-1.3 API may
derive membership, order, identity, path, requiredness, dependency, role,
capability, or validator selection from the fixed enum or its tables.

The old fixed APIs remain temporarily because they describe the existing
pre-membrane Markdown product projection, including `FeatureSpec`, while the
approved HCM-0.6 root profile contains three different canonical YAML artifact
instances. HCM-1.3 must not invent a mapping between those universes. Their
coexistence is not a legacy profile, compatibility dispatch, fallback, or
conversion API: no call path chooses between them, and neither can be converted
to the other. New selected-profile work must use the new registry.

### What HCM-1.3 does not evaluate

HCM-1.3 carries requiredness and condition references as data but does not
evaluate project conditions. It carries semantic validator profiles as typed,
fingerprinted metadata but executes no semantic validator. It performs only
the structural JSON Schema validation already owned by HCM-1.1. It does not
read, write, materialize, render, intake, resolve, project, or mutate artifact
content. Those prohibitions prevent accidental HCM-1.4 setup/doctor adoption,
HCM-2 canonical-YAML migration, and HCM-3 Context Resolution execution.

## Required skill chain

The future implementation parent must apply, in order and with durable proof:

1. `using-agent-skills`;
2. `context-engineering`;
3. `source-driven-development`;
4. `spec-driven-development`;
5. `planning-and-task-breakdown`;
6. `api-and-interface-design`;
7. `security-and-hardening`;
8. `test-driven-development`;
9. `documentation-and-adrs`;
10. `code-review-and-quality`; and
11. `git-workflow-and-versioning`.

The parent must re-read the live skill files rather than rely on this summary.

## Live baseline and blast radius

GitNexus was refreshed at planning entry with
`npx gitnexus analyze --index-only`; the native analyzer terminated with a
`Napi::Error`. Known-symbol context and impact inspection still returned graph
results, while free-form queries reported a missing FTS index. The future
implementation must retry one supported index refresh, record the exact result,
and use live source plus compiler evidence if the index remains unavailable.
It must not weaken or guess the graph.

Planning-time upstream impact results, from the available index, were:

| Surface | Risk | Observed blast radius | HCM-1.3 disposition |
|---|---:|---:|---|
| `CanonicalArtifactKind` | LOW | no indexed upstream nodes | do not edit; source inventory below governs |
| `CanonicalArtifacts::load_with_contract` | LOW | 8 impacted, 3 direct | do not edit or call from the new registry |
| `baseline_artifact_validations` | LOW | 3 direct | do not edit; structural validation uses kind registry |
| `ArtifactManifest::from_canonical_artifacts` | **HIGH** | 19 impacted, 4 direct; doctor/CLI flows | prohibited; HCM-1.4/product migration owns adoption |
| `compute_freshness` | MEDIUM | 10 impacted | prohibited; later structured-authority work owns migration |
| flow `resolve_with_contract` | MEDIUM | 5 impacted | prohibited; no flow adoption |
| `resolve_profile_selection` | **HIGH** | 20 impacted, 18 direct tests | consume unchanged; do not reimplement or modify |
| `ArtifactInstanceRegistry::resolve` | MEDIUM | 30 impacted, 14 direct | consume unchanged; do not duplicate descriptor admission |

The two HIGH surfaces are explicit stop boundaries. If implementation appears
to require modifying either one, stop and escalate rather than widening this
packet. Before changing any existing function, class, method, or public symbol,
the implementation parent must run a fresh upstream impact analysis and record
HIGH or CRITICAL results before proceeding.

## Exact allowed scope

### Existing files

Only these existing runtime files may change:

- `crates/engine/src/lib.rs`, solely to declare and re-export the new owner
  module and public types; and
- `crates/engine/src/artifact_instance.rs`, only if a read-only deterministic
  iterator accessor is required by the implementation. Prefer the existing
  `ids()` plus `instance()` API. No decode, validation, fingerprint, dependency,
  or admission behavior may change.

Only these existing documentation files may change during implementation:

- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`, solely to mark
  the exact registry owner boundary `BoundaryLanded` after all proof passes;
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`, solely for an
  accurate HCM-1.3 implementation closeout status; and
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`,
  solely to append the exact HCM-1.3 proof classification without closing
  downstream adoption gates; and
- `docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl`, solely for the
  deterministic rebuild after adding the new parent v1.2 implementation
  closeout record, staged and committed with that record in the separate
  second closeout commit. No other ledger mutation is allowed.

No other existing file is allowed. A discovered need outside this list is a
scope stop requiring a separately reviewed packet amendment.

### New-file areas

The future implementation may create only:

- `crates/engine/src/artifact_registry.rs`;
- `crates/engine/tests/hcm_1_3_artifact_registry.rs`;
- `crates/engine/tests/fixtures/hcm_1_3_registry/**` for explicit custom
  profile/kind/instance source fixtures;
- `docs/specs/handbook-contract-membrane/slices/HCM-1.3/proof/**` for immutable
  implementation proof; and
- HCM-1.3 internal review dispatches and one parent implementation closeout
  under the existing handoff control-plane paths.

No Cargo manifest, lockfile, build script, package definition, shipped
first-party definition asset, compiler, flow, CLI, Tauri, SDK, adapter,
Substrate, setup, doctor, authoring, renderer, contract, or dock file may
change.

## HCM-1.2 owner APIs consumed unchanged

The new registry is a projection of these reviewed public owners:

| Owner | Exact consumed API/data |
|---|---|
| selected profile | `resolve_profile_selection`, `ResolvedInstanceProfile::{exact_ref, artifact_kind_registry, artifact_instances, stable_role_registry, resolved_profile_fingerprint}` |
| kinds | `ArtifactKindRegistry::{kind_refs, kind, validate_json, schema_registry, semantic_capability_registry}` |
| kind definitions | `ArtifactKindDefinition::{exact_ref, canonical_schema_ref, supported_role_refs, semantic_capabilities, definition_fingerprint}` |
| instances | `ArtifactInstanceRegistry::{ids, instance, fingerprint}` and every read-only `ArtifactInstanceDescriptor` accessor |
| roles | `StableRoleRegistry::{exact_ref, fingerprint, roles, role}` |
| capabilities | kind capability bindings plus `SemanticCapabilityRegistry::{capability, validator}` and their exact-ref/fingerprint accessors |
| condition references | `ArtifactRequiredness::{mode, condition_ref}`; definitions were already admitted by profile resolution and are not evaluated here |
| vocabulary and Context Resolution | retained only in the resolved-profile fingerprint; HCM-1.3 neither exposes new behavior nor executes them |

`resolve_profile_selection` remains the only source/profile/path/identity/
fingerprint admission entry point. `ResolvedArtifactRegistry` accepts only an
already successful `&ResolvedInstanceProfile`; it accepts no repository root,
source paths, bytes, profile request, enum kind, filename, environment state,
or CLI option.

## Frozen public API

The new `handbook-engine` public surface is conceptually exact:

```rust,ignore
pub struct ResolvedArtifactRegistry { /* engine-owned immutable closure */ }
pub struct ResolvedArtifactKind { /* exact kind + structural owner data */ }
pub struct ResolvedArtifactInstance { /* descriptor-driven instance view */ }
pub struct ResolvedArtifactCapability { /* contract + validator metadata */ }
pub struct ResolvedArtifactDependency { /* authored target + providers */ }

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArtifactRegistryValidationError {
    UnknownArtifactInstance,
    Structural(Vec<StructuralValidationError>),
}

impl ResolvedArtifactRegistry {
    pub fn from_profile(
        profile: &ResolvedInstanceProfile,
    ) -> Result<Self, RegistryLoadError>;

    pub fn profile_ref(&self) -> &ExactDefinitionRef;
    pub fn profile_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn stable_role_registry_ref(&self) -> &ExactDefinitionRef;
    pub fn stable_role_registry_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn kind_refs(&self) -> Vec<&ExactDefinitionRef>;
    pub fn instance_ids(&self) -> Vec<&SymbolicId>;
    pub fn kind(&self, exact_ref: &ExactDefinitionRef)
        -> Option<&ResolvedArtifactKind>;
    pub fn instance(&self, id: &SymbolicId)
        -> Option<&ResolvedArtifactInstance>;
    pub fn dependency_order(&self) -> &[SymbolicId];
    pub fn validate_json(
        &self,
        id: &SymbolicId,
        value: &serde_json::Value,
    ) -> Result<(), ArtifactRegistryValidationError>;
}

impl ResolvedArtifactKind {
    pub fn exact_ref(&self) -> &ExactDefinitionRef;
    pub fn definition_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn canonical_schema_ref(&self) -> &ExactDefinitionRef;
    pub fn schema_entry_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn schema_document_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn schema_closure_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn supported_role_ids(&self) -> &[String];
    pub fn capabilities(&self) -> &[ResolvedArtifactCapability];
}

impl ResolvedArtifactInstance {
    pub fn id(&self) -> &SymbolicId;
    pub fn kind_ref(&self) -> &ExactDefinitionRef;
    pub fn role(&self) -> Option<&StableRoleDefinition>;
    pub fn capabilities(&self) -> &[ResolvedArtifactCapability];
    pub fn label(&self) -> &str;
    pub fn canonical_path(&self) -> &str;
    pub fn requiredness_mode(&self) -> RequirednessMode;
    pub fn condition_ref(&self) -> Option<&ExactDefinitionRef>;
    pub fn dependencies(&self) -> &[ResolvedArtifactDependency];
    pub fn lifecycle_policy_ref(&self) -> Option<&ExactDefinitionRef>;
    pub fn intake_definition_ref(&self) -> Option<&ExactDefinitionRef>;
    pub fn renderer_definition_refs(&self) -> &[ExactDefinitionRef];
    pub fn projection_definition_refs(&self) -> &[ExactDefinitionRef];
    pub fn validation_overlay_refs(&self) -> &[ExactDefinitionRef];
    pub fn extensions(&self)
        -> &std::collections::BTreeMap<String, serde_json::Value>;
}

impl ResolvedArtifactCapability {
    pub fn capability_id(&self) -> &SymbolicId;
    pub fn contract_ref(&self) -> &ExactDefinitionRef;
    pub fn contract_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn required_bindings(&self) -> &[SymbolicId];
    pub fn allowed_instance_cardinality(&self) -> AllowedInstanceCardinality;
    pub fn bindings(&self)
        -> &std::collections::BTreeMap<SymbolicId, String>;
    pub fn semantic_validators(&self)
        -> &[SemanticValidationProfileDefinition];
}

impl ResolvedArtifactDependency {
    pub fn target_kind(&self) -> DependencyTargetKind;
    pub fn target_ref(&self) -> &SymbolicId;
    pub fn target_contract_ref(&self) -> Option<&ExactDefinitionRef>;
    pub fn cardinality(&self) -> DependencyCardinality;
    pub fn provider_ids(&self) -> &[SymbolicId];
}
```

Public value accessors must expose, without mutation:

- `ResolvedArtifactKind`: exact kind ref, definition fingerprint, canonical
  schema ref, schema entry/document/closure fingerprints already supplied by
  the schema registry, lexically sorted stable supported-role IDs, and
  lexically sorted capability bindings;
- `ResolvedArtifactInstance`: symbolic instance ID, exact kind ref, optional
  stable-role definition, sorted capability closures, label, trusted normalized
  repository-relative canonical path as a string, requiredness mode, optional
  exact condition ref, authored dependencies, resolved dependency-provider
  IDs, and all still-null/empty later-owned descriptor refs; and
- `ResolvedArtifactCapability`: symbolic capability ID, exact capability
  contract ref, contract fingerprint, kind binding map, and semantic validation
  profile definitions in exact-ref lexical order. The existing validator value
  type supplies exact ref, capability ID, binding rules, and profile
  fingerprint. These are metadata only; and
- `ResolvedArtifactDependency`: the authored target namespace, target ID,
  optional exact capability-contract ref, cardinality, and lexically sorted
  resolved provider instance IDs. Dependency values remain in original
  descriptor order on their owning instance.

The actual implementation may use private helper structs, but it may not alter
the names, result types, identity types, error variants, or behavioral
contracts above without first amending and re-reviewing this packet. Every
slice-returning accessor borrows an engine-owned immutable `Vec` in the stated
order; each map accessor borrows an engine-owned `BTreeMap`; lookup results
borrow the registry; the two registry key-list accessors allocate only the
listed borrowed-key vector. Returned collections are immutable snapshots; no
invocation-time mutation API exists.

The registry does not mint a second fingerprint. `profile_fingerprint()` is
the exact HCM-1.2 resolved-profile fingerprint and is the registry closure
identity. The kind, instance, role, schema, capability, validator, condition,
vocabulary, and Context Resolution fingerprints already included by HCM-1.2
remain the sole fingerprint owners.

## Construction and binding algorithm

`from_profile` must execute these steps in order:

1. clone the selected exact profile ref and resolved-profile fingerprint;
2. iterate `artifact_kind_registry().kind_refs()` in exact-ref lexical order;
3. for each kind, bind its schema entry/resolved schema, supported stable roles,
   capability contracts, binding maps, and validator metadata by exact ref;
4. iterate `artifact_instances().ids()` in symbolic-ID lexical order;
5. bind each descriptor to exactly one admitted kind and, when present,
   exactly one stable-role definition;
6. bind each selected capability ID through that instance's kind mapping to
   exactly one capability contract, then bind every semantic validation
   profile exact ref in lexical order;
7. expand each authored instance dependency to its deterministic provider set
   using the HCM-1.2 namespace, contract-ref, and cardinality rules;
8. compute one providers-before-consumers topological order: dependency
   providers are ready before their consumers, and the lexical `SymbolicId`
   is the tie-break among simultaneously ready nodes; and
9. compare the constructed membership against the exact selected instance and
   kind sets before returning.

An instance dependency resolves to its one exact target instance. A capability
dependency resolves to all instances whose selected kind binding names the
same capability ID and exact contract ref, sorted by provider instance ID.
HCM-1.2 has already checked `exactly_one`/`at_least_one`, missing providers,
duplicates, and cycles; HCM-1.3 repeats the closure binding fail-closed so an
internal inconsistency cannot become a partial registry. It does not reinterpret
or default authored dependency semantics.

Kinds that have no selected artifact instances remain in `kind_refs()` because
the selected profile's kind registry is complete. `instance_ids()` defines
artifact-universe membership. No kind is synthesized from an instance and no
instance is synthesized from a kind.

## Deterministic ordering

The following order is normative:

- kind refs: canonical exact-ref byte order;
- instance IDs: canonical `SymbolicId` byte order;
- role definitions: the admitted stable-role registry order when exposed as a
  registry, but instance lookup is by exact role ID;
- capability closures: capability ID, then exact contract ref;
- semantic validator profiles: exact-ref byte order;
- binding keys: `SymbolicId` byte order;
- authored dependencies: original descriptor order when exposed as authored
  data;
- resolved provider IDs per dependency: `SymbolicId` byte order; and
- dependency execution order: providers before consumers with
  `SymbolicId` lexical tie-break.

Repository source order, map insertion order, filesystem enumeration order,
enum discriminants, fixed descriptor order, and hash-map iteration are never
observable authority. Reordered equivalent explicit request collections must
produce byte-equivalent debug/test projections and the same profile
fingerprint.

## Trusted path and security contract

HCM-1.3 never accepts or opens a path. Each exposed canonical path is copied
only from an HCM-1.2-admitted `ArtifactInstanceDescriptor`. Therefore all
existing fail-closed boundaries remain mandatory:

- exact-trim equality;
- 1 through 1024 UTF-8 bytes;
- 1 through 64 normalized components;
- no empty, `.` or `..` component;
- no absolute, UNC, drive-prefixed, URI-scheme, backslash, NUL, or control-path
  escape;
- no ambient cwd, home, environment, PATH, filename, or directory discovery;
- explicit source bindings, allowed roots, no-follow source opens, regular-file
  checks, per-source 1-MiB and aggregate 8-MiB sentinel-byte limits;
- source count 64, total definition bindings 512, allowed schema roots 32,
  schema documents 128, profile ancestry edges 32, and schema-reference depth
  32; and
- all HCM-1.2 exact identity, closed-record, duplicate/conflict, fingerprint,
  compatibility, source-usage, provider-cardinality, and cycle refusals.

There is no new artifact-count, capability-count, dependency-count, or label
limit in HCM-1.3 because the new API accepts no new input and the admitted
HCM-1.2 source byte budgets already bound the closure. Adding a new numeric
limit here would silently reject a profile that HCM-1.2 admitted and is not
authorized. The implementation must remain linear in admitted kinds,
instances, capability bindings, validators, and dependency edges, apart from
ordered-map logarithmic factors.

The public API returns repository-relative strings only. It must not expose an
absolute joined path, OS handle, canonicalized host path, source bytes, or
unbounded error location. Content opens and time-of-check/time-of-use handling
remain with the existing trusted repository file owner in the slice that
actually reads content.

## Error taxonomy and fail-fast order

`from_profile` uses only existing `RegistryLoadError` categories because it
rebinds already admitted definitions:

1. `ConflictingIdentity` at `artifact_registry/kinds` or
   `artifact_registry/instances` for set mismatch or duplicate projected
   identity;
2. `UnsupportedDependency` at the exact bounded registry member location for
   a missing kind, role, schema, capability contract, or semantic validator;
3. `InvalidDependencyContract` for a capability/contract mismatch;
4. `InvalidDependencyTarget` for a missing exact instance target;
5. `InvalidDependencyProviderCount` for provider cardinality mismatch; and
6. `DependencyCycle` at `artifact_registry/dependencies` if the repeated graph
   check cannot produce a complete order.

The first error in that order wins. Locations must use only bounded symbolic
IDs/exact refs or the existing safe generic fallback; no host path or source
content enters error text.

`validate_json` first checks instance membership. Absence returns
`ArtifactRegistryValidationError::UnknownArtifactInstance`. Presence delegates
exactly once to `ArtifactKindRegistry::validate_json` for the instance's bound
kind. Structural failure returns the existing ordered
`Vec<StructuralValidationError>` inside `Structural`. It performs no semantic,
requiredness, condition, lifecycle, intake, renderer, projection, overlay, or
repository validation and must not collapse an empty schema-setup error into
success.

## Compatibility posture

- exact refs require full canonical SemVer; no range, latest, alias, or
  unversioned lookup exists;
- no old enum-to-ID or ID-to-enum conversion is added;
- no legacy profile, implicit shipped-profile fallback, compatibility flag,
  dual registry, or dispatch is added;
- one explicit `ResolvedInstanceProfile` always yields one registry;
- custom IDs remain symbolic IDs and never become generated Rust variants,
  CLI subcommands, filenames, labels, or renderer selectors;
- unsupported later-owned descriptor fields continue to refuse in HCM-1.2;
  HCM-1.3 does not reinterpret them; and
- HCM-1.3 publishes no downstream compatibility promise outside the local
  engine owner boundary. Any downstream API adoption requires its owning
  slice and regression proof.

## Current fixed-consumer inventory and cutover ledger

The live production inventory at planning entry contains these exact 29 files.
The disposition is normative; no omitted file may be opportunistically edited.

| Current consumer group | Exact files | HCM-1.3 cutover/disposition |
|---|---|---|
| fixed universe definition and load | `crates/engine/src/canonical_artifacts.rs`, `crates/engine/src/canonical_paths.rs` | no edit; cease being owner of the **selected-profile** universe when the new registry API lands; remain pre-membrane Markdown projection pending HCM-1.4/HCM-2 |
| compiler fixed-universe facade | `crates/compiler/src/canonical_artifacts.rs` | no edit; wildcard re-export remains a pre-membrane compiler facade and moves only with its HCM-1.4 setup/doctor and HCM-2 content consumers; no new registry bridge or wildcard export is added |
| fixed validation/manifest/freshness | `crates/engine/src/baseline_validation.rs`, `crates/engine/src/artifact_manifest.rs`, `crates/engine/src/freshness.rs` | no edit; HCM-1.3 registry validation is a separate structural API; manifest/freshness cutover is later and HIGH/MEDIUM risk |
| engine exports | `crates/engine/src/lib.rs` | add and export the new registry; retain existing exports without bridge or fallback |
| flow and fixed byte-budget projection | `crates/flow/src/resolver.rs`, `crates/flow/src/packet_result.rs`, `crates/flow/src/budget.rs` | no edit; HCM-2.1 owns pilot content/flow integration and HCM-3.5 owns Resolution-aware flow adoption |
| setup and doctor | `crates/compiler/src/setup.rs`, `crates/compiler/src/doctor.rs`, `crates/compiler/src/doctor_shell.rs`, `crates/cli/src/doctor_rendering.rs` | no edit; HCM-1.4 owns profile-aware setup/doctor decisions while CLI wording stays outside engine decisions |
| compiler fixed layout/validation/blocking | `crates/compiler/src/layout.rs`, `crates/compiler/src/baseline_validation.rs`, `crates/compiler/src/blocker.rs`, `crates/compiler/src/refusal.rs`, `crates/compiler/src/lib.rs` | no edit; cut only with owning product adoption and machine-decision contract |
| compiler flow-result mapping | `crates/compiler/src/resolver.rs` | no edit; move only with its owning HCM-2.1/HCM-3.5 flow adoption |
| authoring | `crates/compiler/src/author/mod.rs`, `crates/compiler/src/author/charter_shell.rs`, `crates/compiler/src/author/project_context_shell.rs`, `crates/compiler/src/author/environment_inventory_shell.rs` | no edit; HCM-2 canonical content/intake boundary |
| rendering | `crates/compiler/src/rendering/json.rs`, `crates/compiler/src/rendering/markdown.rs`, `crates/compiler/src/rendering/shared.rs`, `crates/cli/src/rendering.rs` | no edit; HCM-2 owns deterministic artifact-view migration and no generated command or dynamic renderer dispatch is allowed |
| pipeline manifest/presence adoption | `crates/pipeline/src/pipeline_handoff.rs` | no edit; exact HCM-3.5 Resolution-aware snapshot/packet/pipeline adoption boundary |

The inventory is reproduced with this exact direct-surface/facade scan; the
resulting sorted set must equal the literal table above rather than merely
equal 29:

```python
import subprocess

pattern = r"CanonicalArtifactKind|CANONICAL_ARTIFACT_ORDER|canonical_artifact_descriptors|canonical_artifacts::\*|CanonicalLayoutContract|baseline_artifact_validations|from_canonical_artifacts|CanonicalArtifactIdentity|CanonicalArtifact\b|ArtifactPresence|ArtifactManifest"
paths = subprocess.check_output(
    ["rg", "-l", pattern, "crates", "--glob", "*.rs", "--glob", "!**/tests/**"],
    text=True,
).splitlines()
print("\n".join(sorted(set(paths))))
```

The precise HCM-1.3 cutover order is:

1. resolve one profile through the unchanged HCM-1.2 entry point;
2. construct the new registry from that resolved profile;
3. prove selected membership, data-driven fields, structural validator routing,
   and custom kind/custom instance operation entirely in engine tests;
4. export the new registry as the sole selected-profile universe owner; and
5. prove by scope and reference scans that no new code uses the enum or fixed
   tables and no existing product consumer changed.

There is deliberately no sixth product-adoption step in HCM-1.3. HCM-1.4 later
adopts the same resolved-profile truth for setup/doctor. HCM-2 later owns
canonical YAML content and current author/render/freshness/manifest migration.
HCM-3 owns Context Resolution behavior. If a later planning packet assigns a
remaining file differently, that exact packet must be independently reviewed.

## TDD increments

Implementation is test-first and must use these atomic increments:

1. **API compile refusal/shape.** Add a failing integration test that imports
   the exact new types and cannot compile before the owner module exists.
2. **Shipped membership.** Resolve the shipped profile, build the registry,
   and assert literal kind refs and literal instance IDs; make only this pass.
3. **Data-driven fields.** Assert all three shipped instance paths,
   requiredness, condition refs, roles, capabilities, structural schema refs,
   and validator metadata from descriptors/kinds; make only this pass.
4. **Custom IDs.** Add an explicit repository-source fixture with one custom
   kind and one custom artifact instance, then prove registration and lookup
   without enum/CLI changes.
5. **Dependencies/order.** Add instance and capability dependencies with
   multiple lexical tie cases; assert literal provider sets and exact
   providers-before-consumers order.
6. **Validation routing.** Prove valid/invalid JSON routes by bound kind schema,
   unknown instance is typed, and semantic validator metadata is not executed.
7. **Fail-closed replay.** Exercise HCM-1.2 source/path/identity/fingerprint/
   duplicate/compatibility/dependency refusals through the selected-profile to
   registry entry sequence and assert construction never observes a partial
   profile.
8. **Determinism and regression.** Reorder all explicitly unordered request
   collections; compare literal registry projections, then run the full proof
   wall and documentation classification.

Each increment records red command/output, minimal green diff, focused green
command/output, and refactor proof before the next increment. Production code
must not precede its failing test.

## Positive, boundary, negative, and security tests

### Positive tests

- literal selected profile ref and resolved-profile fingerprint are preserved;
- literal six shipped kind refs are present in exact order;
- literal instance set is exactly `environment_context`,
  `project_authority`, `project_context` in lexical API order;
- `project_authority` binds stable role `constitutional_authority`, capability
  `constitutional_root`, its exact capability contract and semantic validator
  metadata, `always` requiredness, and trusted path
  `.handbook/project/charter.yaml`;
- `project_context` and `environment_context` bind their literal kind, role,
  path, and requiredness values, including the exact managed-operational-surface
  condition ref for `environment_context`;
- all six kinds bind their exact structural schemas even where no root instance
  selects that kind;
- a custom kind and custom artifact-instance ID work without touching any enum,
  command, template, renderer, or filename switch;
- equivalent request-source permutations produce identical registry views; and
- a multi-level dependency fixture produces the literal expected provider
  sets and providers-before-consumers order.

### N/N+1 boundary tests

No new count ceiling is introduced. The integration path must replay the
existing exact HCM-1.2 boundaries and show HCM-1.3 neither bypasses nor lowers
them: 64/65 profile sources, 512/513 total bindings, 32/33 roots, 128/129 schema
documents, 32/33 ancestry edges, 32/33 schema-reference edges, 1024/1025 path
bytes, 64/65 path components, 1 MiB plus sentinel per source, and 8 MiB plus
sentinel aggregate. The N case must reach registry construction where the
fixture remains otherwise valid; the N+1 case must refuse before construction.
If an existing HCM-1.2 test cannot be reused without creating an enormous new
fixture, exact existing focused tests plus one integration sentinel prove the
same boundary; the proof wall must name both commands and test names.

### Negative and security tests

- unknown instance validation returns the exact new typed variant;
- malformed JSON content fails the bound kind's structural schema and cannot
  be accepted by a different kind's schema;
- missing internal kind, role, capability contract, validator, schema, or
  provider is not constructible through public APIs; a private unit test may
  use a test-only builder to prove the exact fail-closed category without
  exposing mutation in production;
- duplicate instance IDs, duplicate paths, duplicate dependencies,
  conflicting refs/fingerprints, unsupported compatibility, invalid
  requiredness, missing/extra capability providers, and cycles continue to
  refuse through HCM-1.2 before registry construction;
- absolute, parent-traversal, dot-segment, URI, drive, UNC, backslash, NUL,
  untrimmed, over-byte, and over-component paths refuse before any HCM-1.3
  registry exists;
- symlink/non-regular source races and source byte limits remain covered by the
  trusted-source regression suite;
- semantic validator profiles are observable metadata only and no repository
  code, command name, dynamic library, script, or function pointer is loaded;
- condition, vocabulary, Context Resolution, lifecycle, intake, renderer,
  Projection, and overlay behavior is never invoked; and
- error text contains no absolute repository root, source bytes, credentials,
  environment data, or unbounded attacker-controlled location.

## Regression matrix

The future implementation must pass, at minimum:

| Layer | Required command/proof |
|---|---|
| format | `cargo fmt --all -- --check` |
| focused owner | `cargo test -p handbook-engine --test hcm_1_3_artifact_registry` |
| HCM-1.1 | kind/schema registry focused tests named from the live test inventory |
| HCM-1.2 | `cargo test -p handbook-engine --test profile_selection` plus all `hcm_1_2_*` integration targets found live |
| engine | `cargo test -p handbook-engine` |
| full workspace | `cargo test --workspace --all-targets` |
| lints | `cargo clippy --workspace --all-targets -- -D warnings` |
| Windows | workspace check/test using the repository's established Windows target proof or a documented environment stop; no Unix-only success claim |
| docs | link/path/status/unchecked-todo and forbidden-scope scans |
| scope | exact allowed-file set equality plus GitNexus change detection |

Test target names must be discovered from the live repository at execution
time; the parent may not silently omit a renamed HCM-1.1/HCM-1.2 regression.

## Package boundary and proof

No new package-owned definition asset is authorized. The literal HCM-1.2
29-member definition manifest remains exact and must be reproduced from both
the filesystem allowlist and `cargo package` archive. HCM-1.3 adds only Rust
source/test/fixture files; the implementation proof must:

1. run `cargo package -p handbook-engine --allow-dirty --no-verify` or the live
   equivalent established by repository policy;
2. list the archive member names deterministically;
3. compare the literal definition member set for set equality with the
   reviewed HCM-1.2 package-definition manifest;
4. compare SHA-256 and byte size for every definition member;
5. prove no HCM-1.3 repository fixture appears as a package-owned definition;
6. prove package build/check succeeds from the archive or extracted package;
   and
7. retain the exact commands, environment, archive hash, lists, diffs, and
   outputs in the proof wall.

Counts alone are insufficient. Missing, extra, renamed, reordered-without-
normalization, byte-changed, or size-changed definition members fail the gate.

## Scope proof and proof wall

The implementation proof wall must include:

- entry branch/HEAD/clean-state and selected handoff/ledger parity;
- HCM-1.1/HCM-1.2 ancestry, commits, fingerprints, and required files;
- skill-chain evidence;
- GitNexus refresh, queries/context, per-edited-symbol upstream impacts, HIGH/
  CRITICAL warnings, and final `detect_changes` output;
- exact allowed-files manifest and set-equality comparison against `git diff`;
- a literal changed-symbol inventory;
- TDD red/green/refactor evidence for every increment;
- positive, boundary, negative, security, regression, Windows, and package
  proof from the matrix above;
- `rg` proof that no new enum match, fixed table, generated command, dynamic
  filename/renderer dispatch, legacy profile, fallback, range/latest selector,
  setup/doctor adoption, content migration, or later-phase behavior landed;
- explicit evidence that all current fixed consumers in this packet remained
  byte-unchanged except the permitted `lib.rs` export;
- control-pack consistency and gate-classification proof;
- the exact sorted review-subject manifest and aggregate SHA-256 fingerprint;
- every immutable fresh-review dispatch/result and remediation mapping; and
- final fresh `CLEAN`, no mutation of reviewed bytes, reviewed-subject commit,
  then separate deterministic implementation closeout/ledger commit.

Before either commit, run `git diff --check`, inspect the exact cached diff,
run GitNexus change detection for staged scope, and refuse unexpected symbols
or execution flows.

## Permitted classification and open gates

After all implementation proof passes, HCM-1.3 may mark only the selected-
profile artifact registry owner boundary `BoundaryLanded`. It may record a
narrower HCM-1.3 planning/implementation subset for `PG-PROFILE-01`,
`PG-KIND-01`, `PG-KIND-02`, and `PG-ARTIFACT-01`: selected registry membership,
data-driven descriptor bindings, structural validator routing, custom kind/
instance registration, deterministic dependencies, and trusted-path carry-
through.

Those gates remain open. HCM-1.3 does not prove setup/doctor adoption,
real-path content reads, semantic validator execution, canonical YAML
authority, intake/lifecycle coverage, renderer/Projection behavior, or
downstream release. No seam becomes `RealPathAdopted`, `Verified`, or
`Released`.

## Exit gate

HCM-1.3 is complete only when all of the following are true:

- one explicitly selected HCM-1.2 profile produces one complete immutable
  `ResolvedArtifactRegistry`;
- literal shipped and custom membership proves no enum-owned selected-profile
  universe remains;
- kinds and instances remain distinct and all fields are data-driven;
- roles, capabilities, structural schemas, semantic validator metadata,
  requiredness, condition refs, dependencies, and paths bind exactly;
- trusted path, source, identity, fingerprint, compatibility, duplicate, and
  dependency failures remain fail-closed;
- deterministic ordering and all TDD/regression/package/scope proof pass;
- setup, doctor, product content, and later phases are untouched;
- a fresh isolated built-in `default` reviewer returns `CLEAN` over the exact
  complete implementation subject;
- reviewed bytes are committed first and remain unchanged; and
- one separate parent implementation closeout and deterministic ledger commit
  names HCM-1.4 planning as next, without starting it.

## Stop conditions

Stop and escalate without broadening if:

- the selected-profile universe cannot be built exclusively from reviewed
  HCM-1.2 owner APIs;
- correctness requires changing `resolve_profile_selection`,
  `ArtifactInstanceRegistry::resolve`, manifest, freshness, flow, compiler,
  setup, doctor, CLI, Cargo, or definition assets;
- a custom kind/instance requires an enum variant, filename, command, ambient
  lookup, generated dispatch, or new compatibility behavior;
- implementation would need to evaluate a condition, semantic validator,
  lifecycle, intake, renderer, Projection, vocabulary, or Context Resolution;
- a new fingerprint, default, schema, identity, limit, fallback, migration, or
  product behavior appears necessary but is not frozen here;
- trusted repo-relative path provenance cannot be preserved without exposing
  or opening a host path;
- a HIGH/CRITICAL impact surface lacks explicit parent warning and proof;
- the allowed-file set, package member set, or reviewed subject is not exact;
- mandatory Windows/package/full-workspace proof cannot be run or honestly
  classified; or
- fresh review is unavailable or does not return `CLEAN` after valid findings
  are remediated.

## Explicit non-goals

- Rust, Cargo, runtime, or product changes in this planning session;
- HCM-1.3 implementation in the planning closeout session;
- setup or doctor profile adoption (HCM-1.4);
- canonical YAML/content authority, current Markdown migration, authoring,
  intake, renderers, or freshness/manifest cutover (HCM-2);
- condition execution, vocabulary behavior, Context Resolution, Snapshot
  Memory, lifecycle, Projection, or validation-overlay execution (HCM-3+);
- new or changed definition schemas, identifiers, fingerprints, shipped
  defaults, compatibility, migration, fallback, or selection semantics;
- a legacy profile, compatibility dispatch, ambient discovery, range/latest
  selection, invocation-time profile mutation, dynamic filename/command/
  renderer dispatch, or generated CLI commands;
- SDK, CLI, Tauri, Substrate, adapter, contract, dock, or downstream release;
- closing open downstream proof gates beyond the narrow permitted evidence; or
- beginning HCM-1.4 after HCM-1.3 planning or implementation closeout.
