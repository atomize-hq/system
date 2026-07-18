# HCM-1.4 Specification: Profile-Aware Setup and Doctor Decisions

## Status and authority

This is the complete planning-only implementation packet for `HCM-1.4`. It is
authority for a future, separately selected implementation session only after
the exact planning subject receives a fresh `CLEAN` review and the parent
planning closeout records that result. It does not authorize Rust or product
implementation in the session that creates or closes this packet.

The packet consumes, without reopening, the reviewed HCM-1.1 kind/schema
registry, HCM-1.2 profile/descriptor closure, and HCM-1.3 selected-profile
artifact registry. Entry evidence is:

- branch `feat/handbook-contract-membrane`;
- planning entry HEAD `8e3af4694aa0ae30c033c54cd4d13628ba0662e8`;
- HCM-1.3 reviewed implementation commit
  `8194f9f4534b2d27e1077ffab2c89d12da5ff456`;
- HCM-1.3 reviewed subject fingerprint
  `sha256:56d8559ffcecf91935a2ce1140f582949ee87408f0291b75e56c17b55a6f8fee`;
- selected HCM-1.3 closeout
  `20260717T183202Z--HCM-1-3--orchestration--artifact-registry-landed`;
- completed HCM-1.2 implementation ancestry at
  `832716a66241bdcf86e2a82ffb3ae72680a7c2cd`; and
- completed HCM-1.1 implementation ancestry at
  `0bc51a9cc282581143a5b21f50162456aa32154c`.

The selected HCM-1.3 handoff is dependency and transition evidence only. It
does not widen this slice or select a future implementation session. Live
source, tests, package proof, control-pack authority, and Git history at the
entry HEAD override stale narrative wording.

## Objective

Make setup and doctor derive their artifact membership, paths, requiredness,
condition applicability, roles, and capability identity projection from one exact
`ResolvedInstanceProfile` through one `ResolvedArtifactRegistry` and one typed
profile-decision closure. Remove the fixed `CanonicalArtifactKind`/
`CANONICAL_ARTIFACT_ORDER`/fixed-descriptor decision path from setup and doctor
without converting between the fixed Markdown projection and the selected
profile universe.

The future implementation must:

1. bind every selected conditional descriptor to its exact condition definition
   while refusing to invent the separately unowned evidence/evaluator contract;
2. produce one immutable engine-owned `ResolvedProfileDecisions` value whose
   conditional result is explicitly `unresolved`/`evidence_contract_unavailable`;
3. inspect selected repository paths through bounded no-follow reads and route
   structural validation through `ResolvedArtifactRegistry::validate_json`;
4. make both setup planning/execution and doctor reporting consume that same
   value rather than rebuilding or reinterpreting profile truth;
5. expose exact profile, condition-definition, role, capability identity, path,
   applicability, and inspection truth as machine-readable data; and
6. keep human wording, command names, argument parsing, repository discovery,
   and exit-code mapping outside engine decisions.

The result completes Phase 1 decision/readiness adoption. It does not make YAML
content semantically authoritative, write or render canonical artifacts,
implement intake, invent a temporary legacy profile, or start HCM-2.

## Boundary decisions

### One closure, not two setup/doctor interpretations

`ResolvedProfileDecisions::from_profile` is the only HCM-1.4 constructor for
operation-time profile decisions. It accepts only an already resolved
`&ResolvedInstanceProfile`, constructs the HCM-1.3 `ResolvedArtifactRegistry`
internally, binds exact selected condition definitions, and emits one ordered
closure. It accepts no fact/evidence input because the admitted evidence record,
producer verification, freshness, evaluator, transport, and migration contracts
are explicitly unowned at this boundary.
Setup and doctor accept that closure. Neither may accept a fixed artifact enum,
filename list, template table, independent condition boolean, profile ref
without its fingerprint, or separately rebuilt registry.

Setup and doctor may render or classify the same decision differently for
their use cases, but the following bytes of meaning are identical:

- selected exact profile ref and resolved-profile fingerprint;
- selected kind and instance membership;
- stable-role registry ref/fingerprint;
- instance role and the four-field capability identity projection;
- trusted canonical path;
- requiredness mode and exact condition ref;
- exact condition-definition ref/fingerprint and the explicit absence of an
  admitted evidence/evaluation closure;
- `unresolved` plus `evidence_contract_unavailable` for every conditional instance;
- derived applicability; and
- structural inspection result for the same observed repository state.

No compatibility dispatcher chooses between the fixed and selected-profile
universes. The old fixed Markdown projection may remain for sibling authoring,
flow, rendering, or historical tests until their owning slices cut over, but
setup and doctor do not call it, translate it, compare it, or expose it as a
fallback after HCM-1.4.

### Decision/readiness adoption is not Phase 2 content authority

HCM-1.4 may read one selected artifact path as bounded YAML, parse it into the
JSON data model with duplicate-key refusal, and perform the already-owned
HCM-1.1 structural schema validation. This proves generic profile-aware
readiness and routing; it does not create a typed canonical artifact model,
approve semantic content, write canonical YAML, select intake or renderers,
produce a renderer-derived view, or claim that a document is canonical
authority.

The inspection vocabulary therefore stops at `structurally_valid`. It must not
emit `valid_canonical_truth`, `approved`, `current`, `locked`, `authoritative`,
or another stronger state. Phase 2 owns content-specific load/write,
first-party semantic validation, intake/promotion, renderer-derived views,
source/rendered fingerprints, and direct authority cutover.

Setup is no longer permitted to write fixed Markdown starter templates. It may
preserve an already present structurally valid selected artifact, report that
authoring is required, or stop on condition-evidence-unavailable/unsafe/invalid
truth. `--rewrite` has no profile-owned
materializer in this slice and must return a typed refusal without changing
artifact bytes. Runtime-state reset remains a separate existing mutation and
may execute only after the entire artifact decision/inspection plan is safe.

### Condition-definition binding without invented evidence authority

The control pack freezes the six condition outcome names and the high-level
managed-operational-surface policy, but it also states that the condition record
schema, exact input bindings, evidence types, freshness thresholds, evaluator,
transport, and migration are separate contracts. No landed owner currently
verifies an `authoritative_fact_ref` or `admitted_evidence_ref`. HCM-1.4 must not
turn caller-provided strings, booleans, fingerprints, environment state, file
presence, or profile flags into purported authoritative evidence.

HCM-1.4 therefore implements only the decision that is currently justified by
the definition's exact precedence: missing required evidence/input resolves to
`unresolved`, while `unknown` is reserved for an admitted evaluable basis that
proves neither truth value.

- `always` becomes `required`;
- `optional` becomes `optional`;
- every `conditional` instance binds its exact condition ref and definition
  fingerprint, reports outcome `unresolved`, reason
  `evidence_contract_unavailable`, and applicability `indeterminate`; and
- the evidence-closure fingerprint is `null` because no admitted evidence
  closure exists. It is never replaced by a hash of absence or caller claims.

The six outcome enum is preserved as machine vocabulary because the selected
definition owns those exact names. HCM-1.4 produces only `unresolved`; it does
not evaluate or synthesize `true`, `false`, `unknown`, `stale`, or `refused`.
Repository silence never yields false, Environment Context never
proves its own applicability, and present/valid Environment Context bytes do not
change the outcome. No public or private HCM-1.4 API accepts an observation,
evidence ref, freshness basis, source class, assertion, bare condition boolean,
or condition override.

A later separately reviewed condition-evaluation packet must freeze the admitted
basis record/reference, producer-verification boundary, complete fingerprinted
evidence/input closure, freshness semantics, semantic identity/equivalence,
contradiction/precedence behavior, transport, and migration before another
outcome can be produced. HCM-1.4 tests enforce the input-surface absence and the
exact unresolved result rather than mocking unverifiable authority.

## Required skill chain

The future implementation parent must apply, in order and with durable proof:

1. `using-agent-skills`;
2. `context-engineering`;
3. `source-driven-development`;
4. `spec-driven-development`;
5. `planning-and-task-breakdown`;
6. `api-and-interface-design`;
7. `security-and-hardening`;
8. `incremental-implementation`;
9. `test-driven-development`;
10. `debugging-and-error-recovery` when any check fails;
11. `documentation-and-adrs`;
12. `code-review-and-quality`; and
13. `git-workflow-and-versioning`.

The parent must re-read the live skill files rather than rely on this summary.

## Live baseline and blast radius

Planning refreshed GitNexus with `npx gitnexus analyze --index-only`. The
incremental index completed successfully at the planning entry. Free-form
query remained degraded because the FTS index was unavailable, so planning
used graph context/Cypher, live source, tests, and Git history without claiming
semantic-search completeness.

Planning-time upstream impact results were:

| Surface | Risk | Observed blast radius | HCM-1.4 disposition |
|---|---:|---:|---|
| `run_setup` in `crates/compiler/src/setup.rs` | **HIGH** | 39 impacted, 15 direct, setup CLI flow | change only after fresh impact warning; preserve all non-artifact mutation safety |
| `build_setup_execution_plan` | MEDIUM | 41 impacted, 2 direct, setup CLI flow | replace fixed artifact planning test-first; retain request/reset ordering |
| `doctor` in `crates/compiler/src/doctor.rs` | MEDIUM | 12 impacted, 9 direct, doctor CLI flow | replace fixed baseline decision path; preserve typed process outcome |
| `doctor_from_artifacts` | LOW | 13 impacted, 1 direct | remove from setup/doctor public path; no fixed-artifact adapter |
| `acquire_authoring_lock` in `crates/compiler/src/author/mod.rs` | **HIGH** | 24 impacted, 3 direct, author CLI flow | permit only the behavior-preserving cfg-selected local acquisition-operation hunk frozen below; preserve helper signatures plus Unix acquire/unlock and non-Unix behavior; run complete author suites |
| CLI setup `run` | LOW | no indexed upstream nodes | keep shell-only responsibility |
| CLI doctor `run` | LOW | no indexed upstream nodes | keep shell-only responsibility |

The HIGH surface requires an explicit warning before a future code edit. The
implementation parent must rerun upstream impact for every existing function,
class, method, or public symbol it changes, record HIGH/CRITICAL results before
editing, and stop if the required remediation would widen beyond this packet.

## Exact allowed scope

### Existing runtime files

Only these existing engine files may change:

- `crates/engine/src/lib.rs`, solely to declare/re-export HCM-1.4 decision and
  inspection types/functions;
- `crates/engine/src/profile_selection.rs`, solely to retain and expose the
  already-admitted selected `ProjectConditionRegistry` in
  `ResolvedInstanceProfile`;
- `crates/engine/src/project_condition_registry.rs`, solely for bounded
  read-only accessors needed to bind an exact definition/fingerprint; and
- `crates/engine/src/canonical_repo_support.rs`, only if the new inspection
  module cannot use existing crate-private normalized no-follow read helpers
  unchanged. Prefer no edit.

Only these existing compiler files may change:

- `crates/compiler/src/setup.rs`, to replace fixed artifact planning with the
  HCM-1.4 decision/inspection closure while retaining setup mode, root safety,
  complete profile preflight, and the unchanged legacy reset owner/behavior;
- `crates/compiler/src/doctor.rs`, to replace fixed baseline decisions with the
  same closure and return the frozen machine-readable report;
- `crates/compiler/src/lib.rs`, only to expose the revised setup/doctor public
  types/functions; and
- `crates/compiler/src/author/mod.rs`, solely for the exact Windows-build
  portability repair inside `acquire_authoring_lock`: immediately before the
  unchanged two-argument acquisition call, bind local `lock_operation` to
  `libc::LOCK_EX` under `#[cfg(unix)]` and to integer `0` under
  `#[cfg(not(unix))]`, then pass `lock_operation` to the existing helper. Both
  cfg-specific helper signatures/bodies and the `Drop` call using
  `libc::LOCK_UN` remain byte-unchanged. No other author symbol, branch, error,
  lock semantics, command behavior, or file may change; and
- `crates/compiler/src/setup_shell.rs` and
  `crates/compiler/src/doctor_shell.rs`, only to delete or narrow obsolete
  fixed-artifact wording helpers after the CLI owns their presentation. They
  may not receive new domain decisions.

Only these existing CLI files may change:

- `crates/cli/src/setup.rs`, for shell invocation and human rendering of typed
  setup results;
- `crates/cli/src/doctor.rs`, for shell invocation, JSON selection, human
  rendering, and exit mapping;
- `crates/cli/src/doctor_rendering.rs`, for profile-aware text/JSON rendering;
- `crates/cli/src/exit_policy.rs`, only if exact new typed doctor status mapping
  cannot be expressed unchanged; and
- `crates/cli/src/main.rs`, only for type/API rewiring. Command names and
  arguments are frozen; no new profile, condition, or generated command is
  authorized.

Only these existing test files may change:

- `crates/compiler/tests/setup.rs`;
- `crates/compiler/tests/doctor.rs`;
- `crates/compiler/tests/author.rs`, only to replace its `scaffold_repo` call to
  public setup with a test-local legacy authoring fixture writer;
- `crates/cli/tests/cli_surface.rs`; and
- `crates/cli/tests/author_cli.rs`, only to replace its setup subprocess-based
  `scaffold_repo` with the same test-owned legacy fixture shape; and
- exact engine HCM-1.2/HCM-1.3 tests only when a public accessor addition
  requires compile-only expectation updates without weakening prior proof.

Only these existing documentation/control files may change during
implementation:

- `docs/specs/handbook-contract-membrane/00-README.md`;
- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`;
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`;
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`;
  and
- `docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl`, solely for the
  deterministic second-commit closeout rebuild.

No Cargo manifest, lockfile, definition asset, content schema, fixed
`canonical_artifacts`/`canonical_paths`/layout module, authoring file other than
the exact `author/mod.rs` portability hunk above, flow,
pipeline, SDK, Tauri, Substrate, adapter, contract, dock, intake, renderer, or
Projection file may change. A discovered need outside the list is a scope stop
requiring a separately reviewed packet amendment.

### New-file areas

The future implementation may create only:

- `crates/engine/src/profile_decision.rs`;
- `crates/engine/src/profile_inspection.rs`;
- `crates/engine/tests/hcm_1_4_profile_decisions.rs`;
- `crates/engine/tests/hcm_1_4_profile_inspection.rs`;
- `crates/compiler/src/profile_readiness.rs`, as the sole owner of the exact
  shared compiler rows/status plus pure setup/doctor projection/classification;
- `docs/specs/handbook-contract-membrane/slices/HCM-1.4/proof/**` for immutable
  implementation proof; and
- HCM-1.4 internal dispatches plus one parent implementation closeout under the
  existing handoff control-plane paths.

No committed fixture is required: focused tests generate bounded YAML and
repository trees in temporary directories so HCM-1.4 adds no package-owned
definition member or accidental product default. The two existing authoring
tests may write their current legacy Markdown baseline into each temporary
directory through test-local helpers; they may not call setup, change
production authoring behavior beyond the exact lock-call portability hunk,
turn those bytes into selected-profile truth, or
assert that HCM-1.4 setup owns the legacy files.

## Reviewed owner APIs consumed unchanged

| Owner | Exact consumed API/data |
|---|---|
| profile resolution | `resolve_profile_selection`, `ResolvedInstanceProfile::{exact_ref, artifact_kind_registry, artifact_instances, stable_role_registry, resolved_profile_fingerprint}` plus the narrow new project-condition-registry accessor |
| selected registry | `ResolvedArtifactRegistry::{from_profile, profile_ref, profile_fingerprint, stable_role_registry_ref, stable_role_registry_fingerprint, kind_refs, instance_ids, kind, instance, dependency_order, validate_json}` |
| instance decision data | every HCM-1.3 `ResolvedArtifactInstance` accessor for identity, kind, role, capability, path, requiredness, condition, dependency, and later-owned refs |
| capability identity projection | only `ResolvedArtifactCapability::{capability_id, contract_ref, contract_fingerprint}` plus owning instance ID; required bindings, allowed cardinality, instance bindings, and semantic validators remain registry-owned and are intentionally not copied into HCM-1.4 reports |
| condition definition | `ProjectConditionRegistry::definition` and `ProjectConditionDefinition::{exact_ref, definition_fingerprint}` |
| safe repository read | engine normalized repo-relative path plus descriptor-relative no-follow bounded read helpers |
| structural parse/validation | `parse_definition_yaml`'s duplicate-safe JSON-data-model parse and `ResolvedArtifactRegistry::validate_json` |

HCM-1.4 must not modify profile/kind/instance identity, fingerprints, source
admission, selected membership, dependency order, schema closure behavior,
validator metadata, or definition assets merely to simplify consumer adoption.
`ProfileCapabilityTruth` and `ProfileCapabilityRow` are therefore explicitly a
four-field identity projection, not the complete semantic-capability closure.
They expose instance ID, capability ID, exact contract ref, and contract
fingerprint only. HCM-1.4 does not reinterpret, duplicate, serialize, equality-
check, or claim report ownership over required bindings, allowed instance
cardinality, instance binding maps, semantic-validator definitions, or binding
rules. Those remain accessible through the selected registry owner and are
outside setup/doctor decision needs in this slice.

## Frozen engine public API

The engine public surface is exact. Private helper names may differ; public
names, variants, owned field types, ordering, and return contracts may not
change without packet amendment and re-review. Engine decision/inspection
values are typed domain values, not transport DTOs; they do not derive
`Serialize` merely to force serialization of HCM-1.2 identity owners.

```rust,ignore
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectConditionOutcome {
    True,
    False,
    Unknown,
    Unresolved,
    Stale,
    Refused,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProjectConditionDecisionReason {
    EvidenceContractUnavailable,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectConditionEvaluation {
    condition_ref: ExactDefinitionRef,
    condition_definition_fingerprint: DefinitionFingerprint,
    outcome: ProjectConditionOutcome,
    reason: ProjectConditionDecisionReason,
    evidence_closure_fingerprint: Option<DefinitionFingerprint>,
}

impl ProjectConditionEvaluation {
    pub fn condition_ref(&self) -> &ExactDefinitionRef;
    pub fn condition_definition_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn outcome(&self) -> ProjectConditionOutcome;
    pub fn reason(&self) -> ProjectConditionDecisionReason;
    pub fn evidence_closure_fingerprint(&self) -> Option<&DefinitionFingerprint>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactApplicability {
    Required,
    Optional,
    Indeterminate,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProfileCapabilityTruth {
    instance_id: SymbolicId,
    capability_id: SymbolicId,
    contract_ref: ExactDefinitionRef,
    contract_fingerprint: DefinitionFingerprint,
}

impl ProfileCapabilityTruth {
    pub fn instance_id(&self) -> &SymbolicId;
    pub fn capability_id(&self) -> &SymbolicId;
    pub fn contract_ref(&self) -> &ExactDefinitionRef;
    pub fn contract_fingerprint(&self) -> &DefinitionFingerprint;
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactProfileDecision {
    instance_id: SymbolicId,
    kind_ref: ExactDefinitionRef,
    role_id: Option<String>,
    canonical_path: String,
    requiredness_mode: RequirednessMode,
    condition_ref: Option<ExactDefinitionRef>,
    condition_outcome: Option<ProjectConditionOutcome>,
    condition_reason: Option<ProjectConditionDecisionReason>,
    evidence_closure_fingerprint: Option<DefinitionFingerprint>,
    applicability: ArtifactApplicability,
    capabilities: Vec<ProfileCapabilityTruth>,
}

impl ArtifactProfileDecision {
    pub fn instance_id(&self) -> &SymbolicId;
    pub fn kind_ref(&self) -> &ExactDefinitionRef;
    pub fn role_id(&self) -> Option<&str>;
    pub fn canonical_path(&self) -> &str;
    pub fn requiredness_mode(&self) -> RequirednessMode;
    pub fn condition_ref(&self) -> Option<&ExactDefinitionRef>;
    pub fn condition_outcome(&self) -> Option<ProjectConditionOutcome>;
    pub fn condition_reason(&self) -> Option<ProjectConditionDecisionReason>;
    pub fn evidence_closure_fingerprint(&self) -> Option<&DefinitionFingerprint>;
    pub fn applicability(&self) -> ArtifactApplicability;
    pub fn capabilities(&self) -> &[ProfileCapabilityTruth];
}

#[derive(Clone, Debug)]
pub struct ResolvedProfileDecisions {
    registry: ResolvedArtifactRegistry,
    condition_evaluations: Vec<ProjectConditionEvaluation>,
    artifact_decisions: Vec<ArtifactProfileDecision>,
    capability_truth: Vec<ProfileCapabilityTruth>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProfileDecisionError {
    Registry(RegistryLoadError),
    MissingConditionDefinition { condition_ref: ExactDefinitionRef },
}

#[derive(Debug)]
pub enum ShippedProfileDecisionError {
    Profile(ProfileLoadError),
    Decision(ProfileDecisionError),
}

pub fn resolve_shipped_profile_decisions(
    repo_root: impl AsRef<Path>,
) -> Result<ResolvedProfileDecisions, ShippedProfileDecisionError>;

impl ResolvedProfileDecisions {
    pub fn from_profile(
        profile: &ResolvedInstanceProfile,
    ) -> Result<Self, ProfileDecisionError>;

    pub fn profile_ref(&self) -> &ExactDefinitionRef;
    pub fn profile_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn stable_role_registry_ref(&self) -> &ExactDefinitionRef;
    pub fn stable_role_registry_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn condition_evaluations(&self) -> &[ProjectConditionEvaluation];
    pub fn artifact_decisions(&self) -> &[ArtifactProfileDecision];
    pub fn capability_truth(&self) -> &[ProfileCapabilityTruth];
    pub fn registry(&self) -> &ResolvedArtifactRegistry;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactInspectionStatus {
    Missing,
    StructurallyValid,
    StructurallyInvalid,
    UnsafePath,
    Unreadable,
    NotInspected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactInspectionReason {
    PresentAndStructurallyValid,
    RequiredPathMissing,
    OptionalPathMissing,
    ConditionalEvidenceUnavailablePathMissing,
    ConditionalEvidenceUnavailablePathPresent,
    YamlSyntaxInvalid,
    DuplicateYamlKey,
    DocumentNotObject,
    StructuralValidationFailed,
    DocumentLimitExceeded,
    AggregateReadLimitExceeded,
    SymlinkRefused,
    NonRegularFileRefused,
    UnsafeRepositoryPath,
    UnsupportedPlatformStrictRead,
    RepositoryReadFailed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactInspection {
    instance_id: SymbolicId,
    canonical_path: String,
    applicability: ArtifactApplicability,
    status: ArtifactInspectionStatus,
    reason: ArtifactInspectionReason,
}

impl ArtifactInspection {
    pub fn instance_id(&self) -> &SymbolicId;
    pub fn canonical_path(&self) -> &str;
    pub fn applicability(&self) -> ArtifactApplicability;
    pub fn status(&self) -> ArtifactInspectionStatus;
    pub fn reason(&self) -> ArtifactInspectionReason;
}

#[derive(Clone, Debug)]
pub struct ProfileInspectionReport {
    profile_ref: ExactDefinitionRef,
    profile_fingerprint: DefinitionFingerprint,
    artifacts: Vec<ArtifactInspection>,
}

impl ProfileInspectionReport {
    pub fn profile_ref(&self) -> &ExactDefinitionRef;
    pub fn profile_fingerprint(&self) -> &DefinitionFingerprint;
    pub fn artifacts(&self) -> &[ArtifactInspection];
}

pub fn inspect_profile_repository(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
) -> ProfileInspectionReport;
```

`ResolvedInstanceProfile` gains exactly
`project_condition_registry(&self) -> &ProjectConditionRegistry`; the registry
and definition APIs otherwise remain unchanged.
`resolve_shipped_profile_decisions(repo_root)` constructs the exact package-owned
HCM-0.6 `ProfileSelectionRequest` internally from explicit built-in bindings,
then calls `resolve_profile_selection` and `from_profile`; it performs no cwd,
environment, latest/range, filename, or repository-profile discovery.
`ProfileDecisionError::Registry` wraps only failure to construct the
already-reviewed selected artifact registry.
A selected conditional descriptor whose exact condition is absent returns
`MissingConditionDefinition` before any partial closure exists.

Every conditional evaluation contains the exact definition ref/fingerprint,
`Unresolved`, `EvidenceContractUnavailable`, and `None` for the evidence-closure
fingerprint. There is no request type, observation constructor, source ref,
evidence ref, freshness basis, assertion, override, or evaluator entry point.
Every artifact decision exposes the exact owned fields above and no command,
sentence, translated label, exit code, mutable action, or fixed artifact value.

Ordering is exact-ref lexical order for conditions, symbolic-ID lexical order
for artifact decisions, then instance ID/capability ID/contract ref for
capability identity truth. Filesystem order, map order, enum discriminants, and fixed
artifact order are never observable authority.

## Condition and applicability algorithm

For every selected descriptor, HCM-1.4 performs only this closed derivation:

1. construct one `ResolvedArtifactRegistry` from the selected profile;
2. for `always`, reject any condition ref and emit `required`;
3. for `optional`, reject any condition ref and emit `optional`;
4. for `conditional`, require the exact condition ref, resolve that definition
   from the profile-retained condition registry, bind its fingerprint, emit
   `unresolved`/`evidence_contract_unavailable`/`null` evidence fingerprint, and
   derive `indeterminate`; and
5. sort the complete immutable result as frozen above.

The only applicability table implemented by this slice is:

| Requiredness | HCM-1.4 condition truth | Applicability |
|---|---|---|
| `always` | no condition allowed | `required` |
| `optional` | no condition allowed | `optional` |
| `conditional` | exact definition bound; required evidence/evaluator input unavailable, so `unresolved` | `indeterminate` |

There is deliberately no `not_applicable` result in HCM-1.4 because no approved
input can prove false. The enum and report must not imply otherwise. Tests prove
that condition/file/environment/profile/CLI inputs cannot be injected and that
identical selected profiles produce identical decisions.

## Repository inspection contract

Inspection iterates `artifact_decisions()` in symbolic-ID order. It never
discovers files, follows a symlink, joins an absolute path, consults cwd/home/
environment/PATH, or infers an instance from a filename.

For `optional`, absence is `missing` without a blocker and presence is
structurally checked. For `required`, absence or invalidity blocks readiness.
For `indeterminate`, absence is `not_inspected` with
`conditional_evidence_unavailable_path_missing`; a present file is structurally
checked and reports either its structural status or
`conditional_evidence_unavailable_path_present`, but applicability remains
indeterminate. Presence never becomes true and absence never becomes false.

Each read:

1. uses the already-admitted descriptor path;
2. opens every component no-follow from the repository directory descriptor on
   Unix and uses the existing fail-closed strict posture on unsupported
   platforms;
3. reads at most the existing 1 MiB document ceiling and participates in the
   existing 8 MiB aggregate source ceiling;
4. parses one YAML document into the JSON data model with duplicate-key,
   non-finite-number, syntax, and size refusal;
5. routes validation by instance ID through the selected registry; and
6. returns typed bounded categories without absolute paths, raw bytes,
   credentials, or attacker-controlled diagnostic text.

The public status/reason projection is total and first-match. Applicability
does not change an unsafe/invalid/read failure pair; it changes only missing or
successful presence rows:

| Low-level result | Applicability | `(ArtifactInspectionStatus, ArtifactInspectionReason)` |
|---|---|---|
| strict open returns `Missing` | required | `(missing, required_path_missing)` |
| strict open returns `Missing` | optional | `(missing, optional_path_missing)` |
| strict open returns `Missing` | indeterminate | `(not_inspected, conditional_evidence_unavailable_path_missing)` |
| bounded parse plus instance validation succeeds | required or optional | `(structurally_valid, present_and_structurally_valid)` |
| bounded parse plus instance validation succeeds | indeterminate | `(structurally_valid, conditional_evidence_unavailable_path_present)` |
| YAML syntax or non-finite-number refusal | any | `(structurally_invalid, yaml_syntax_invalid)` |
| duplicate YAML key refusal | any | `(structurally_invalid, duplicate_yaml_key)` |
| parsed top-level value is not an object | any | `(structurally_invalid, document_not_object)` |
| instance-routed schema/structural validation fails, including wrong kind | any | `(structurally_invalid, structural_validation_failed)` |
| document exceeds 1 MiB, detected with a 1 MiB + 1 bounded read | any | `(unreadable, document_limit_exceeded)` |
| next bounded document would exceed the 8 MiB report budget | any | `(unreadable, aggregate_read_limit_exceeded)` |
| any component/final strict open returns `SymlinkNotAllowed` | any | `(unsafe_path, symlink_refused)` |
| strict open returns `NotRegularFile` | any | `(unsafe_path, non_regular_file_refused)` |
| normalized-path/escape/other Unix `InvalidPath` | any | `(unsafe_path, unsafe_repository_path)` |
| `cfg(not(unix))` strict-read-unavailable `InvalidPath` | any | `(unsafe_path, unsupported_platform_strict_read)` |
| other metadata/open/read I/O failure | any | `(unreadable, repository_read_failed)` |

No catch-all may collapse one row into another. A present invalid conditional
artifact uses its syntax/duplicate/object/structural/size/path/read pair; the
condition-unavailable-present reason applies only after successful structural
validation. The focused inspection suite has one named test per table row plus
permutations proving the same pair is returned independent of selected-instance
and filesystem creation order. After aggregate exhaustion, the current and
every later symbolic-ID row is `(unreadable,
aggregate_read_limit_exceeded)` without another open; earlier completed rows
remain unchanged.

Inspection returns all selected instances even when one fails, so doctor can
report complete machine truth. Setup mutation preflight is fail-closed: no
root/reset mutation begins unless the entire decision/inspection closure was
produced successfully. Invalid or indeterminate readiness permits no mutation;
required-missing action-required truth may still permit the separately
requested unchanged reset after complete preflight.

## Compiler setup/doctor and CLI contract

The compiler remains bounded composition scaffolding until HCM-4.1. Its default
entry points call engine `resolve_shipped_profile_decisions(repo_root)`; its
injected entry points accept `&ResolvedProfileDecisions` so shipped/custom
profile proof can pass the exact same closure to setup and doctor. Every path
then calls engine inspection once. No compiler entry point accepts a profile
ref, condition/evidence input, fixed artifact value, filename list, or prose.

The revised public compiler surface is exact:

```rust,ignore
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupMode {
    Auto,
    Init,
    Refresh,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RepositoryReadinessStatus {
    Ready,
    ActionRequired,
    Indeterminate,
    Invalid,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileConditionRow {
    pub condition_ref: String,
    pub condition_definition_fingerprint: String,
    pub outcome: ProjectConditionOutcome,
    pub reason: ProjectConditionDecisionReason,
    pub evidence_closure_fingerprint: Option<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileCapabilityRow {
    pub instance_id: String,
    pub capability_id: String,
    pub contract_ref: String,
    pub contract_fingerprint: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ProfileArtifactRow {
    pub instance_id: String,
    pub kind_ref: String,
    pub role_id: Option<String>,
    pub capability_ids: Vec<String>,
    pub canonical_path: String,
    pub requiredness: RequirednessMode,
    pub condition_ref: Option<String>,
    pub condition_outcome: Option<ProjectConditionOutcome>,
    pub condition_reason: Option<ProjectConditionDecisionReason>,
    pub evidence_closure_fingerprint: Option<String>,
    pub applicability: ArtifactApplicability,
    pub inspection_status: ArtifactInspectionStatus,
    pub inspection_reason: ArtifactInspectionReason,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupArtifactActionKind {
    Preserve,
    AuthorRequired,
    OptionalAbsent,
    ConditionIndeterminate,
    Invalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupRootAction {
    Preserve,
    Create,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SetupArtifactAction {
    pub artifact: ProfileArtifactRow,
    pub action: SetupArtifactActionKind,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SetupPlan {
    pub requested_mode: SetupMode,
    pub resolved_mode: SetupMode,
    pub root_action: SetupRootAction,
    pub profile_ref: String,
    pub profile_fingerprint: String,
    pub stable_role_registry_ref: String,
    pub stable_role_registry_fingerprint: String,
    pub conditions: Vec<ProfileConditionRow>,
    pub capabilities: Vec<ProfileCapabilityRow>,
    pub artifacts: Vec<SetupArtifactAction>,
    pub reset_paths: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct SetupOutcome {
    pub plan: SetupPlan,
    pub status: RepositoryReadinessStatus,
    pub reset_applied: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupErrorKind {
    ProfileResolution,
    ProfileDecision,
    AlreadyInitialized,
    MissingCanonicalRoot,
    InvalidCanonicalRoot,
    InvalidRequest,
    MaterializerUnavailable,
    RuntimeStatePlan,
    RuntimeStateApply,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupErrorReasonCode {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateMutationFailed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SetupErrorCode {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
    UnresolvedMode,
    InitRejectsRefreshFlags,
    RootAlreadyInitialized,
    RefreshRootMissing,
    RootNotDirectory,
    RootSymlinkRefused,
    CanonicalRootInspectFailed,
    CanonicalRootCreateFailed,
    RewriteHasNoMaterializer,
    RuntimeStateTargetUnsafe,
    RuntimeStateMutationFailed,
}

impl SetupErrorCode {
    pub const ALL: [Self; 13];
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetupError {
    code: SetupErrorCode,
}

impl SetupError {
    pub fn code(&self) -> SetupErrorCode;
    pub fn kind(&self) -> SetupErrorKind;
    pub fn reason_code(&self) -> SetupErrorReasonCode;
    pub fn repo_relative_path(&self) -> Option<&'static str>;

    pub(crate) fn from_code(code: SetupErrorCode) -> Self;
}

pub fn plan_setup(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
) -> Result<SetupPlan, SetupError>;

pub fn plan_setup_with_decisions(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
    decisions: &ResolvedProfileDecisions,
) -> Result<SetupPlan, SetupError>;

pub fn run_setup(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
) -> Result<SetupOutcome, SetupError>;

pub fn run_setup_with_decisions(
    repo_root: impl AsRef<Path>,
    request: &SetupRequest,
    decisions: &ResolvedProfileDecisions,
) -> Result<SetupOutcome, SetupError>;

pub const DOCTOR_REPORT_SCHEMA_ID: &str = "handbook.repository-doctor-report";
pub const DOCTOR_REPORT_SCHEMA_VERSION: &str = "1.0.0";

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct DoctorReport {
    pub schema_id: String,
    pub schema_version: String,
    pub profile_ref: String,
    pub profile_fingerprint: String,
    pub stable_role_registry_ref: String,
    pub stable_role_registry_fingerprint: String,
    pub conditions: Vec<ProfileConditionRow>,
    pub capabilities: Vec<ProfileCapabilityRow>,
    pub artifacts: Vec<ProfileArtifactRow>,
    pub status: RepositoryReadinessStatus,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoctorErrorKind {
    ProfileResolution,
    ProfileDecision,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoctorErrorReasonCode {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DoctorError {
    ShippedProfileUnavailable,
    SelectedProfileDecisionInvalid,
}

impl DoctorError {
    pub const ALL: [Self; 2];
    pub fn kind(&self) -> DoctorErrorKind;
    pub fn reason_code(&self) -> DoctorErrorReasonCode;
}

pub fn doctor(repo_root: impl AsRef<Path>) -> Result<DoctorReport, DoctorError>;

pub fn doctor_with_decisions(
    repo_root: impl AsRef<Path>,
    decisions: &ResolvedProfileDecisions,
) -> Result<DoctorReport, DoctorError>;
```

`SetupMode` retains its three current variants and adds the exact lowercase
snake-case wire values `"auto"`, `"init"`, and `"refresh"` required by the
serialized `SetupPlan`. `SetupRequest { mode, rewrite, reset_state }` retains
its exact current public shape and is not a transport DTO. The old
`SetupActionLabel`, `SetupAction`,
`SetupDisposition`, prose-bearing `SetupRefusal`, `DoctorBaselineStatus`,
`DoctorArtifactStatus`, checklist/blocker/C-03/C-04 report fields, and
`doctor_from_artifacts` leave the setup/doctor public path; no adapter preserves
the historical report as a target guarantee.

`DoctorError` is the closed two-variant representation. Its projection is
total and exact:

| `DoctorError` variant | `kind()` | `reason_code()` |
|---|---|---|
| `ShippedProfileUnavailable` | `ProfileResolution` | `ShippedProfileUnavailable` |
| `SelectedProfileDecisionInvalid` | `ProfileDecision` | `SelectedProfileDecisionInvalid` |

`DoctorError::ALL` contains those variants in that order. Only the two public
enum variants construct values; no independent public fields, arbitrary
kind/reason pair, path, instance ID, or unbounded string exists. Compiler tests
must exhaustively replay `ALL` against the table and assert that every default/
injected doctor error uses one of the two variants.

### Setup derivation and mutation

Setup evaluates inputs in this exact order:

1. inspect `.handbook` with `symlink_metadata`; an I/O failure returns
   `InvalidCanonicalRoot`/`CanonicalRootInspectFailed` with
   `repo_relative_path = Some(".handbook")`;
2. resolve `auto` from root state and reject an impossible remaining auto as
   `InvalidRequest`/`UnresolvedMode`;
3. if the resolved mode is init and either flag is true, return
   `InvalidRequest`/`InitRejectsRefreshFlags` before rewrite handling;
4. apply the root/mode table below;
5. for a valid refresh request with `rewrite=true`, return
   `MaterializerUnavailable`/`RewriteHasNoMaterializer`;
6. construct/inject the profile decisions, inspect all selected paths, and
   classify the complete readiness plan;
7. plan the unchanged reset when requested; and
8. only for `ready` or `action_required`, apply root action then reset action.

The root/mode table is total:

| Requested/resolved mode | Root state | Result/root action |
|---|---|---|
| `auto` -> `refresh`, or explicit `refresh` | directory | `preserve` |
| `auto` -> `init`, or explicit `init` | missing | `create` |
| explicit `init` | directory | `AlreadyInitialized`/`RootAlreadyInitialized` |
| `auto` -> `init`, or explicit `init` | non-directory | `InvalidCanonicalRoot`/`RootNotDirectory` |
| `auto` -> `init`, or explicit `init` | symlink | `InvalidCanonicalRoot`/`RootSymlinkRefused` |
| explicit `refresh` | missing | `MissingCanonicalRoot`/`RefreshRootMissing` |
| explicit `refresh` | non-directory | `InvalidCanonicalRoot`/`RootNotDirectory` |
| explicit `refresh` | symlink | `InvalidCanonicalRoot`/`RootSymlinkRefused` |

HCM-1.4 deliberately retires the current pre-membrane invalid-root deletion:
repairing a non-directory/symlink root before profile inspection would be the
only way to make `replace_invalid` reachable, but it would mutate before the
frozen safe preflight. Failure to create a planned missing root maps to
`InvalidCanonicalRoot`/`CanonicalRootCreateFailed`. No invalid-root removal or
repair-failure branch remains. The CLI owns every summary/next-action sentence
and exhaustively maps these enums without exposing an underlying absolute path
or I/O string.

The error projection table is total. `SetupError` stores only one private
`SetupErrorCode`; its kind, reason, and safe path are exhaustively derived from
that code. It intentionally has no `instance_id`: no retained typed setup
failure can bind one without parsing a legacy string or inventing a source.

| `SetupErrorCode` | Derived kind | Derived reason | `repo_relative_path()` |
|---|---|---|---|
| `ShippedProfileUnavailable` | `ProfileResolution` | `ShippedProfileUnavailable` | `None` |
| `SelectedProfileDecisionInvalid` | `ProfileDecision` | `SelectedProfileDecisionInvalid` | `None` |
| `UnresolvedMode` | `InvalidRequest` | `UnresolvedMode` | `None` |
| `InitRejectsRefreshFlags` | `InvalidRequest` | `InitRejectsRefreshFlags` | `None` |
| `RootAlreadyInitialized` | `AlreadyInitialized` | `RootAlreadyInitialized` | `Some(".handbook")` |
| `RefreshRootMissing` | `MissingCanonicalRoot` | `RefreshRootMissing` | `Some(".handbook")` |
| `RootNotDirectory` | `InvalidCanonicalRoot` | `RootNotDirectory` | `Some(".handbook")` |
| `RootSymlinkRefused` | `InvalidCanonicalRoot` | `RootSymlinkRefused` | `Some(".handbook")` |
| `CanonicalRootInspectFailed` | `InvalidCanonicalRoot` | `CanonicalRootInspectFailed` | `Some(".handbook")` |
| `CanonicalRootCreateFailed` | `InvalidCanonicalRoot` | `CanonicalRootCreateFailed` | `Some(".handbook")` |
| `RewriteHasNoMaterializer` | `MaterializerUnavailable` | `RewriteHasNoMaterializer` | `None` |
| `RuntimeStateTargetUnsafe` | `RuntimeStatePlan` | `RuntimeStateTargetUnsafe` | `Some(".handbook/state")` |
| `RuntimeStateMutationFailed` | `RuntimeStateApply` | `RuntimeStateMutationFailed` | `Some(".handbook/state")` |

Only compiler code can call `from_code`; downstream crates cannot use a struct
literal, public constructor, or mutable field. `SetupErrorCode::ALL` contains
the thirteen variants in table order. Unit tests exhaustively replay `ALL`
against the table. After the API exists, two independent rustdoc boundary tests
prove privacy: one `compile_fail,E0451` example contains only an external
`SetupError { code: ... }` struct literal; a separate `compile_fail,E0624`
example contains only an external `SetupError::from_code(...)` call. Each sample
imports already-public types and has no other expected error, so either privacy
regression makes its own doctest fail rather than being masked by the other
boundary. No other kind/reason/path combination is representable. Runtime
errors never parse the legacy human string or disclose the failed child; both
bind the constant `.handbook/state` owner path. CLI prose may say that reset
failed but must not include the underlying string.

Setup artifact actions derive totally from one `ProfileArtifactRow`:

| Applicability/inspection | Action | Readiness contribution |
|---|---|---|
| required + structurally valid | `preserve` | ready |
| required + missing | `author_required` | action required |
| optional + structurally valid | `preserve` | ready |
| optional + missing | `optional_absent` | ready |
| indeterminate + missing/valid | `condition_indeterminate` | indeterminate |
| any + structurally invalid/unsafe/unreadable | `invalid` | invalid |

Status precedence is `invalid`, `indeterminate`, `action_required`, `ready`.
`SetupPlan` carries no prose. `run_setup` writes no selected artifact.
`rewrite=true` returns `MaterializerUnavailable`/
`RewriteHasNoMaterializer` before root or reset mutation.

Existing auto/init/refresh routing for missing/directory roots remains; the old
invalid-root deletion is intentionally retired by the fail-closed table above.
The existing runtime-state reset planner/applier remains in its current owner
and keeps its current behavior; HCM-1.4 neither claims nor implements
transactional rollback for the sequential reset. The complete profile/
inspection plan is constructed before any root/reset mutation. Reset is
attempted only when readiness is `ready` or
`action_required`, never for `invalid` or `indeterminate`; `reset_applied` is
true only after the unchanged applier returns success. A reset apply error may
follow partial legacy deletion and is reported honestly as
`RuntimeStateApply`/`RuntimeStateMutationFailed`; repairing that independent
mutation contract requires a separate packet. `route_state.rs` is unchanged.

### Doctor report and CLI mapping

Doctor emits the exact closed report above. Setup and doctor projections for
profile identity, conditions, four-field capability identity, and every
`ProfileArtifactRow` are
byte-equal for the same repository snapshot and injected decision reference.
The report contains no fixed kind, label, author command, Markdown validation,
blocker, current C-03/C-04 field, product sentence, or absolute path.

The existing CLI grammar remains exactly `handbook setup [init|refresh]` and
`handbook doctor [--json]`; no profile, condition, evidence, or generated
command is added. CLI setup and doctor own all human wording. Doctor JSON is
`serde_json::to_string_pretty(&report)` plus exactly one trailing LF. Setup and
doctor use the same exit table: `ready` exits 0; `action_required`,
`indeterminate`, `invalid`, and every typed error exit 1. Text labels and JSON
snake-case values must map exhaustively to the same enum/status; no wildcard or
stringly default is permitted.

These compiler structs are the HCM-1.4 transitional machine-readable domain
projection, not the final HCM-4 operation DTO/schema envelope. HCM-4 later owns
shared request/result/problem/receipt/version negotiation and generated JSON
Schema without changing the profile-decision meaning proved here.

## Compatibility and cutover posture

This is a greenfield direct cutover for setup/doctor decisions:

- no legacy profile;
- no selected-profile-to-enum mapping;
- no filename alias or fallback search;
- no Markdown/YAML dual-read decision;
- no old/new report mode or compatibility flag;
- no current-template inference;
- no generated command for a kind or instance; and
- no preservation of the historical doctor JSON shape as a public target
  guarantee.

The command grammar `handbook setup [init|refresh]` and `handbook doctor
[--json]` remains stable. The compatibility/support compiler crate may retain
unrelated fixed artifact modules for sibling seams, but setup/doctor imports and
tests must prove their path no longer references the fixed universe. HCM-4.1
still owns compiler retirement; HCM-1.4 adds no new permanent public owner
there.

The two authoring integration suites are an explicit test-only exception to the
setup/doctor fixed-symbol absence scan. Their existing `scaffold_repo` helpers
are renamed `legacy_authoring_fixture_repo` and must stop calling public setup
or the setup CLI. Instead, each helper creates a temporary `.handbook` tree by
iterating the retained sibling `canonical_artifact_descriptors()` entries whose
`setup_scaffolded` flag is true and writing
`setup_starter_template_bytes(descriptor.kind)` to the descriptor's legacy
repo-relative path. This is fixture construction only. No other assertion,
fixed descriptor/template byte, or author command changes; outside the
separately frozen behavior-preserving `author/mod.rs` portability hunk, the two
test diffs must contain only helper/import replacement. The
complete author suites then remain mandatory regression proof without implying
that HCM-1.4 setup still scaffolds those files.

The mandatory compiler Windows target already fails before HCM-1.4 at the
unconditional `libc::LOCK_EX` argument in `acquire_authoring_lock`. This packet
admits exactly one production-authoring portability exception to make its own
platform proof executable: cfg-select one local acquisition operation as
`libc::LOCK_EX` on Unix and ignored integer `0` on non-Unix, then pass it to the
unchanged two-argument helper. The helper signatures/bodies and the `Drop`
`LOCK_UN` call stay byte-unchanged. Unix still acquires the same exclusive lock,
retries interruption, and explicitly unlocks; non-Unix remains the existing
no-op. The implementation
must record the failing compiler MSVC check before this edit, rerun fresh HIGH-
risk impact and warn, prove the exact hunk, pass the compiler MSVC check, and run
the complete 47/22 author suites. No authoring migration or additional repair is
authorized.

## TDD increments

The future implementation is sequential and test-first:

1. **Decision API RED** — compile tests require every exact engine type,
   accessor, shipped resolver, retained condition registry, and the deliberate
   absence of any condition/evidence request API.
2. **Definition binding GREEN** — minimally retain the selected condition
   registry and implement always/optional plus exact conditional
   `unresolved`/`evidence_contract_unavailable`/`indeterminate` derivation.
3. **Profile closure RED/GREEN** — shipped/custom profile tests require exact
   instance, role, four-field capability identity, path, requiredness,
   definition binding, and
   applicability truth without fixed enums or caller evidence.
4. **Inspection RED/GREEN** — generated temp repositories prove no-follow
   bounded YAML parsing, instance-routed structural validation, every frozen
   status/reason, aggregation, and order determinism.
5. **Compiler contract RED** — positive compile tests require every frozen
   shared row, setup/doctor status/error/code/reason type, `ALL`, read-only
   accessor, default/injected signature, serialization value, and total status/
   action mapping before compiler edits. They fail because those APIs do not yet
   exist; privacy doctests are not misclassified as pre-API RED.
6. **Setup GREEN/boundary** — implement the minimal exact API and mapping, then
   run the exhaustive 13-row unit replay plus the independent
   `compile_fail,E0451` and `compile_fail,E0624` privacy doctests; replace fixed
   decisions; prove no artifact write, typed rewrite refusal, full profile
   preflight before unchanged reset behavior, and equality with injected
   decisions.
7. **Doctor GREEN** — replace fixed baseline report; prove schema identity,
   shared rows, status precedence, equality with setup decisions, and exhaustive
   two-variant `DoctorError::ALL` projection.
8. **CLI RED/GREEN** — preserve grammar/root discovery, render human text only
   in CLI, serialize exact JSON, and prove the frozen exit table.
9. **Compiler Windows portability RED/GREEN** — record the existing MSVC
   `libc::LOCK_EX` failure, warn on the fresh HIGH impact, apply only the frozen
   behavior-preserving cfg-selected local acquisition-operation hunk, and prove the
   compiler Windows check plus complete author suites.
10. **Regression/refactor** — remove obsolete setup/doctor fixed imports and
   helpers, keep sibling seams green, and simplify without widening.

Every RED result is recorded before its corresponding production edit. A
production implementation followed by retroactive tests does not satisfy this
packet.

## Positive, boundary, negative, and security tests

### Positive tests

- shipped profile produces exactly six kinds and three selected artifact
  decisions with exact HCM-0.6 paths/roles/capability identities/requiredness;
- every capability truth/report row contains exactly instance ID, capability
  ID, exact contract ref, and contract fingerprint in deterministic order;
- its conditional Environment Context binds the exact condition definition and
  returns `unresolved`/`evidence_contract_unavailable`/`indeterminate` with null
  evidence fingerprint whether its path is absent or structurally valid;
- a custom profile/custom kind/custom instance produces decisions and
  inspection without enum or command changes;
- the same injected closure makes setup and doctor expose byte-equivalent
  profile/condition/applicability/capability-identity/artifact rows;
- valid bounded YAML at required paths is structurally valid through the bound
  kind schema; and
- setup/doctor text/JSON status and the exit table agree.

### Boundary tests

- 0/1/1 MiB/1 MiB+1 artifact bytes and aggregate 8 MiB/8 MiB+1;
- optional, conditional-indeterminate, and required path presence/absence;
- zero/one/multiple selected conditional instances sharing one exact definition;
- Unix strict no-follow and actual Windows-host fail-closed refusal;
- all setup modes, rewrite, reset-state, and status-precedence combinations; and
- empty/minimum/maximum selected profile/instance/capability sets already
  admitted by HCM-1.2/HCM-1.3 boundaries.

### Negative and security tests

- a missing selected condition definition refuses before a partial decision set;
- public/source absence scans prove no observation/evidence/freshness/assertion/
  boolean/condition-override input enters engine, compiler, or CLI HCM-1.4 APIs;
- environment, cwd, file presence, profile flag, and Environment Context bytes
  cannot change `unresolved` or populate an evidence-closure fingerprint;
- capability reports do not copy or claim required bindings, allowed instance
  cardinality, instance binding maps, semantic validators, or binding rules;
- missing/unsafe/unreadable/oversized/duplicate-key/malformed/non-object/wrong-
  kind YAML returns bounded typed inspection truth;
- symlink in every intermediate or final component refuses, including
  substitution-race proof on Unix;
- an unselected Work Specification, Decision Record, or Risk Record path is
  never inspected or reported missing;
- indeterminate conditional truth never creates, scaffolds, rewrites, or
  deletes Environment Context;
- `rewrite` changes no artifact/root/reset byte; profile invalid/indeterminate
  preflight starts no root/reset mutation; existing route-state tests remain
  unchanged and green without a false rollback claim;
- errors contain no absolute repo path, raw document bytes, credential-shaped
  values, environment values, or unbounded attacker text;
- profile source, filesystem creation, and map insertion permutations produce
  identical decisions/projections; and
- setup/doctor production source contains no `CanonicalArtifactKind`,
  `CANONICAL_ARTIFACT_ORDER`, `canonical_artifact_descriptors`, fixed starter
  template, filename switch, or old/new dispatch; the diff scanner admits
  `canonical_artifact_descriptors`/`setup_starter_template_bytes` only inside
  the exact helper hunks of the two allowlisted author test files.

## Regression matrix

Before the portability hunk, the future proof wall records the current
`cargo check -p handbook-compiler --target x86_64-pc-windows-msvc
--all-features` `libc::LOCK_EX` failure as RED. After the exact hunk, the future
proof wall must run at least:

```text
cargo fmt --all -- --check
cargo test -p handbook-engine --test hcm_1_4_profile_decisions
cargo test -p handbook-engine --test hcm_1_4_profile_inspection
cargo test -p handbook-engine --test hcm_1_3_artifact_registry
cargo test -p handbook-engine --test hcm_1_2_public_owner_api
cargo test -p handbook-engine --test profile_selection
cargo test -p handbook-engine
cargo test -p handbook-compiler --test setup
cargo test -p handbook-compiler --test doctor
cargo test -p handbook-compiler --test author
cargo test -p handbook-compiler
cargo test -p handbook-compiler --doc
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test author_cli
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p handbook-engine --target x86_64-pc-windows-msvc --all-features
cargo check -p handbook-compiler --target x86_64-pc-windows-msvc --all-features
cargo package -p handbook-engine --allow-dirty --no-verify
cargo metadata --no-deps --format-version 1
git diff --exit-code "$ENTRY_HEAD" -- Cargo.toml Cargo.lock crates/engine/Cargo.toml crates/compiler/Cargo.toml crates/cli/Cargo.toml
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
git diff --check
```

The following commands must also execute on an actual Windows MSVC host, not a
cross-compiling Linux/macOS host and not as `cargo check` only:

```text
cargo test -p handbook-engine --test hcm_1_4_profile_inspection \
  --target x86_64-pc-windows-msvc -- \
  non_unix_repository_inspection_refuses_before_read
cargo test -p handbook-compiler --test setup \
  --target x86_64-pc-windows-msvc -- \
  windows_profile_inspection_refusal_prevents_setup_mutation
```

The proof wall records host OS/architecture, Rust target, command, exit status,
and test name. Existing CI need not change in this slice; the parent must use an
available Windows runner/host or stop. Compilation is supplementary and cannot
substitute for those runtime results.

Any intentionally retired fixed setup/doctor test is replaced by a named
profile-aware assertion in the same commit. The two author suites may change
only the exact fixture helpers/imports frozen above; their substantive
assertions remain unchanged and all 47/22 tests pass. Manifest, freshness,
flow, rendering, route-state mutation, and fixed-projection tests remain green
and are not rewritten to pretend that HCM-1.4 migrated those seams.

## Package, source-tree scope, and proof wall

The HCM-1.2 literal 29-member package-definition manifest remains exact for the
repository definition tree and the engine `.crate` archive by member set, size,
SHA-256, and byte equality. HCM-1.4 adds no definition asset. Engine package
proof extracts and checks the normalized package and proves no absolute
workspace path or fixture leak.

`handbook-compiler` is not packaged in HCM-1.4: its manifest has path-only local
workspace dependencies and Cargo publication requires registry versions for
published dependencies. The no-Cargo slice therefore substitutes exact
compiler source-tree proof: unchanged `crates/compiler/Cargo.toml`, focused and
complete compiler/workspace tests, metadata resolution from the workspace,
exact changed-path set equality, forbidden fixture/absolute-path scans, and no
untracked compiler source. Publication/versioning is a separate packet and
`cargo package -p handbook-compiler --no-verify` is not claimed as proof.

The final proof wall records:

- entry and final HEAD/status;
- every fresh GitNexus impact result and warning;
- every RED/GREEN/refactor command/result;
- exact changed-path set equality;
- exact `author/mod.rs` portability-hunk equality, fresh HIGH-risk impact/
  warning, unchanged Unix/non-Unix behavior, compiler MSVC GREEN, and complete
  47/22 author regression results;
- fixed setup/doctor symbol/import absence;
- the literal setup/doctor affected test/file inventory and dispositions;
- engine package member equality, bytes, sizes, hashes, extraction/check, and
  the exact compiler source-tree boundary checks above;
- format, focused, workspace, clippy, actual Windows runtime, handoff, diff,
  secret, and untracked-aware whitespace proof;
- raw review dispatch/result refs and remediation lineage; and
- the maximum classification/gate promotion supported by real CLI setup and
  doctor execution.

Before either commit, run `npx gitnexus analyze --index-only` and repository-
required `gitnexus_detect_changes` for the staged scope. A failed refresh is
recorded honestly; it is not converted into a clean graph claim. The primary
commit stages only the reviewed implementation/control-pack/proof subject. The
second commit stages only the parent handoff, deterministic ledger rebuild, and
directly required mechanical closeout artifacts.

## Permitted classification and open gates

The maximum promotion is:

- Setup decision path: `BoundaryLanded` only for profile-decision/readiness
  adoption with no artifact materialization;
- Doctor baseline: `BoundaryLanded` only for profile-aware structurally checked
  machine truth;
- Instance profile and selected artifact registry: evidence of real setup/
  doctor adoption, without changing their existing `BoundaryLanded` owner
  status; and
- the setup/doctor adoption subset of `PG-PROFILE-01`, `PG-KIND-01`,
  `PG-KIND-02`, and `PG-ARTIFACT-01` may be recorded as passed evidence, while
  those gates remain open for content authority, semantic validation, intake,
  condition evidence/evaluation, renderers/Projections, publication, and
  downstream use.

HCM-1.4 does not promote canonical YAML authority, content semantics,
materialization, authoring, intake, renderer-derived views, capitalized
Projection, full SDK/DTO/JSON Schema, Tauri, Substrate, contract, dock, or Phase
2+ proof. Structural validity is not canonical authority.

## Exit gate

HCM-1.4 implementation is complete only when:

1. one selected profile creates one registry and one typed decision closure;
2. every conditional descriptor binds its exact definition/fingerprint and
   produces only `unresolved`/`evidence_contract_unavailable`/`indeterminate`
   with null evidence fingerprint, while all condition/evidence input surfaces
   are absent;
3. setup and doctor consume that same closure and no fixed artifact decision
   path or compatibility dispatch remains in either;
4. repository inspection is descriptor-driven, bounded, no-follow, duplicate-
   safe, structurally routed, and honest about applicability;
5. setup writes no canonical artifact, refuses rewrite before mutation, and
   completes profile preflight before invoking the unchanged legacy reset
   owner without claiming transactional rollback;
6. doctor exposes the frozen closed machine-readable profile/capability-identity
   report
   and CLI text/JSON/exit semantics agree;
7. shipped and custom profile positive/negative/security/determinism tests pass;
8. the complete regression, engine-package, compiler-source-tree, actual
   Windows-runtime, scope, handoff, and GitNexus proof wall passes;
9. control-pack updates promote only supported setup/doctor adoption truth;
10. a fresh isolated read-only built-in `default` reviewer returns `CLEAN` over
    the exact complete final subject; every valid finding receives bounded
    remediation, full proof replay, and a different fresh reviewer; and
11. the reviewed implementation is committed first, followed only by a
    separate parent-owned completed v1.2 handoff/ledger closeout commit. HCM-2
    is named as future work but not started.

## Stop conditions

Stop and require packet amendment or broader authority when:

- any condition outcome other than `unresolved` would be required before a
  separately reviewed admitted-evidence/evaluator contract exists;
- condition truth would require a bare boolean, caller source/fingerprint,
  environment inference, Environment Context self-evidence, or an otherwise
  unverified input;
- setup/doctor adoption requires a fixed-enum/profile bridge, legacy profile,
  filename fallback, permanent dual path, or generated command;
- canonical artifact content must be written, semantically approved, rendered,
  promoted, or treated as authority;
- a Cargo/dependency/definition-asset/schema change is required;
- current CLI needs a new profile/condition input grammar or full operation DTO
  envelope;
- a required edit falls outside the exact allowed files;
- HIGH/CRITICAL impact cannot be contained by the frozen regression wall;
- safe no-follow behavior would be weakened on any platform;
- profile invalid/indeterminate truth cannot prevent root/reset mutation from
  starting before the current legacy reset owner is invoked;
- production authoring/flow/rendering/manifest behavior or any author-test byte
  outside the exact behavior-preserving `author/mod.rs` portability hunk and
  two fixture helper/import replacements must change to make setup/doctor pass;
  or
- required platform/runtime/review proof is unavailable.

## Explicit non-goals

- implementation in the packet-planning session;
- profile/kind/instance/condition definition changes;
- admitted condition fact/evidence records, freshness/evaluator/transport,
  outcomes other than explicit unresolved, or evidence-closure fingerprinting;
- a legacy, fallback, compatibility, inferred, ambient, latest, or range-
  selected profile;
- canonical YAML writes, semantic authority, intake, approval/promotion,
  renderer-derived views, or Projections;
- restoring Markdown starter templates or mapping selected instances to the
  fixed Markdown product projection;
- semantic-capability required-binding, cardinality, instance-binding,
  validator, or binding-rule reporting beyond the frozen four-field identity
  projection;
- authoring-command migration or generating one command per kind/instance;
- transactional runtime-state reset repair or a compiler `.crate` publication
  proof that would require Cargo dependency-version changes;
- full repository.setup plan/apply SDK operation DTOs, generated JSON Schema,
  complete CLI JSON parity, Tauri, Substrate, publication, or downstream proof;
- flow, pipeline, resolver, freshness, manifest, Snapshot Memory, posture,
  contract, dock, or adapter work;
- closing the broad profile/kind/artifact gates beyond the exact adoption
  evidence; and
- HCM-2 or unrelated cleanup.
