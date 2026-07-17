# HCM-1.3 Planning Proof Wall

## Classification

- phase: `HCM-1`
- slice: `HCM-1.3`
- packet: none
- work class: documentation/specification/planning/proof/review/closeout only
- implementation authority: not active
- product adoption: not active
- proof timestamp: `20260717T143610Z`
- planning baseline: `c5733785fbd60b7d7a19318cb86058395a02e1c3`

This proof wall binds the candidate planning subject before independent review.
The immutable review dispatch carries the sorted path/hash manifest and exact
aggregate subject fingerprint. Final `CLEAN`, the primary reviewed-subject
commit, and the separate parent closeout/ledger commit are recorded in the
review transport and v1.2 closeout because adding those later facts here would
mutate the bytes under review.

## Entry-state proof

At entry, before any edit:

- branch was exactly `feat/handbook-contract-membrane`;
- HEAD was exactly `c5733785fbd60b7d7a19318cb86058395a02e1c3`;
- `git status --short` was empty;
- `docs/specs/handbook-contract-membrane/slices/HCM-1.3/` did not exist; and
- the selected handoff ID was exactly
  `20260717T125103Z--HCM-1-2--orchestration--profile-boundary-landed`.

The selected record and one ledger entry agreed. The record binds:

- HCM-1.2 primary commit
  `832716a66241bdcf86e2a82ffb3ae72680a7c2cd`; and
- reviewed subject fingerprint
  `sha256:d120149e21831c2009d9708c71c335482a97a10d8d975b8246742ed07fe64c71`.

Git ancestry checks passed for:

```text
0bc51a9cc282581143a5b21f50162456aa32154c  HCM-1.1 primary
  -> 832716a66241bdcf86e2a82ffb3ae72680a7c2cd  HCM-1.2 primary
  -> c5733785fbd60b7d7a19318cb86058395a02e1c3  planning entry
```

The HCM-1.1 selected closeout also binds its reviewed subject fingerprint
`sha256:2c93f55377e8fbbce082c2f189c5a9476a4c5fb292ab7eed2e01aa723999db55`.

## Handoff and ledger validation

All required validator modes passed against 37 records and 37 ledger entries:

```text
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS: 3 record schemas, 2 internal-dispatch schemas, 2 templates, 37 records,
140 current internal dispatches, 8 admitted legacy dispatches, 37 ledger entries

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py \
  --self-test-v1-admission
PASS: unknown/modified/deleted historical records and dispatches refuse;
exact ledger rebuild passes every scenario

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py \
  --self-test-orchestration-contract
PASS: invalid child/agent/status/resume/fingerprint/result/remediation and
two-commit contract cases fail closed; valid chained/direct remediation passes
```

The selected HCM-1.2 record is reviewed transition and dependency evidence
only. No record was interpreted as HCM-1.3 implementation authority.

## Authority and skill proof

The parent re-read the repo-owned runner `07`, protocol `08`, active control
pack `00` through `06`, HCM-1.1/HCM-1.2 packets and closeouts, HCM-1.2 live
source/tests/definitions/package proof/public APIs, and the exact HCM-1.3 phase
row before drafting.

The required local skill files were read in this order:

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

The planning subject uses live repository evidence as primary authority,
separates owner API from product adoption, freezes an exact interface/error/
security/test/package contract, keeps the future todo unchecked, and reserves
implementation plus next-slice activation for separate sessions.

## Live-source grounding

The parent re-read these owner surfaces, not only prior narrative:

- `profile_selection.rs`: `ResolvedInstanceProfile` and the HIGH-risk
  `resolve_profile_selection` entry point;
- `artifact_instance.rs`: descriptor/accessor shapes,
  `ArtifactInstanceRegistry::resolve`, trusted canonical-path admission,
  dependency/provider/cardinality/cycle validation, and registry fingerprint;
- `artifact_kind_registry.rs`: kind lookup, schema/capability owners, and
  structural `validate_json` delegation;
- `stable_role_registry.rs`, `semantic_capability_registry.rs`,
  `project_condition_registry.rs`, `schema_registry.rs`,
  `definition_identity.rs`, and `canonical_repo_support.rs`;
- the HCM-1.2 shipped profile/kind/schema/capability/validator/condition/
  vocabulary/Context Resolution definition assets and literal package proof;
- the HCM-1.2 public-owner API test and focused profile-selection tests; and
- current fixed enum, layout, validation, manifest, freshness, flow, setup,
  doctor, authoring, blocking, and rendering consumers.

The packet therefore consumes the exact selected-profile, kind, descriptor,
role, capability, condition, vocabulary, validator, schema, and fingerprint
owners already live. It does not infer a new source schema, identity,
fingerprint, default, selector, compatibility rule, or content behavior.

## GitNexus proof and risk wall

The configured index was named `handbook`; the stale repo instruction named
`system`. A supported refresh attempt was made once:

```text
npx gitnexus analyze --index-only
STOP RESULT: native analyzer terminated with libc++abi / Napi::Error
```

Free-form concept queries then reported missing FTS results. Known-symbol
context and upstream impact still returned evidence. The packet records this
degraded state rather than pretending the refresh succeeded, requires a fresh
implementation-entry attempt, and binds live source/compiler proof as the
fallback.

Planning impact results:

| Symbol | Risk | Impact | Planning response |
|---|---:|---:|---|
| `CanonicalArtifactKind` | LOW | 0 indexed upstream | source inventory overrides suspicious zero |
| `CanonicalArtifacts::load_with_contract` | LOW | 8 / 3 direct | prohibited from HCM-1.3 registry |
| `baseline_artifact_validations` | LOW | 3 direct | unchanged |
| `ArtifactManifest::from_canonical_artifacts` | **HIGH** | 19 / 4 direct | explicit warning; prohibited product cutover |
| `compute_freshness` | MEDIUM | 10 | unchanged |
| flow `resolve_with_contract` | MEDIUM | 5 | unchanged |
| `resolve_profile_selection` | **HIGH** | 20 / 18 direct tests | explicit warning; consumed unchanged |
| `ArtifactInstanceRegistry::resolve` | MEDIUM | 30 / 14 direct | consumed unchanged |

The packet makes both HIGH surfaces implementation stop conditions and requires
fresh upstream impact on every existing symbol before editing.

## Fixed-consumer set equality

This search was executed over production Rust:

```text
rg -l 'CanonicalArtifactKind|CANONICAL_ARTIFACT_ORDER|\
canonical_artifact_descriptors|CanonicalLayoutContract|\
baseline_artifact_validations|from_canonical_artifacts' \
  crates --glob '*.rs' --glob '!**/tests/**'
```

Its literal set equaled the 23-file set frozen in `SPEC.md`:

```text
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
crates/compiler/src/rendering/markdown.rs
crates/compiler/src/rendering/shared.rs
crates/compiler/src/setup.rs
crates/engine/src/artifact_manifest.rs
crates/engine/src/baseline_validation.rs
crates/engine/src/canonical_artifacts.rs
crates/engine/src/canonical_paths.rs
crates/engine/src/freshness.rs
crates/engine/src/lib.rs
crates/flow/src/packet_result.rs
crates/flow/src/resolver.rs
```

Result: `PASS: literal production fixed-consumer set equality (23 files)`.
Each group has an explicit cutover owner/disposition. The selected-profile
universe owner changes in HCM-1.3; no current product path changes.

## Packet presence and implementation separability

Required authoritative files exist:

- `slices/HCM-1.3/SPEC.md`;
- `slices/HCM-1.3/tasks/plan.md`; and
- `slices/HCM-1.3/tasks/todo.md`.

The specification freezes:

- exact objective, entry authority, and additive owner boundary;
- HCM-1.2 APIs consumed unchanged;
- exact public types, entry point, accessors, validation error variants, and
  absence of a second fingerprint;
- construction order, dependency provider resolution, and deterministic order;
- trusted path, source, identity, fingerprint, duplicate, compatibility,
  dependency, and bounded error protections;
- exact allowed existing files and new-file areas;
- literal current fixed-consumer inventory and per-group disposition;
- TDD increments, positive tests, N/N+1 replay, negative/security tests, full
  regression matrix, Windows proof, package set/hash/size proof, and scope proof;
- narrow allowed classification without downstream gate closure;
- exit gate, stop conditions, and explicit non-goals; and
- final fresh review plus two-commit implementation closeout choreography.

The plan orders every increment and names its red/green and stop boundary. The
todo is a future implementation checklist with zero checked boxes. A future
parent can execute the packet without inventing owner, API, compatibility,
ordering, error, security, test, package, or closeout semantics.

## Internal consistency checks

The following checks passed:

- `git diff --check` for the tracked phase-map edit;
- all three authoritative packet files exist;
- todo contains no checked implementation item;
- the stale sentence claiming only HCM-1.1 is authorized is absent;
- the phase-map HCM-1.3 packet link resolves;
- required owner, error, dependency-order, security-boundary, package-proof,
  setup/doctor reservation, HCM-2/HCM-3 reservation, and fresh-review phrases
  are present across the packet;
- every candidate path is under
  `docs/specs/handbook-contract-membrane/`; and
- no Rust, Cargo, runtime, product, or definition path is modified.

Before review dispatch, the parent must rerun whitespace/path/unchecked-todo/
docs-only checks across tracked and untracked files and hash the exact subject.

## Cross-document consistency

The only coupled existing control-pack edit is
`04-phase-slice-map.md`. It now accurately states:

- HCM-1.1 and HCM-1.2 landed and are dependency evidence;
- HCM-1.3 packet approval is the next authorization boundary;
- approval is not execution and requires a later separately selected session;
  and
- HCM-1.4 and later slices remain unauthorized.

The HCM-1.2 row names its exact primary and selected closeout. The HCM-1.3 row
links the packet and preserves its descriptor-driven objective. Existing `03`
and `06` correctly describe HCM-1.2 as additive `BoundaryLanded` data and keep
HCM-1.3/product gates open, so changing them during planning would create a
false implementation claim. No unrelated documentation was changed.

## Dependency-order proof

The packet's session dependency order is exact:

```text
HCM-1.1 reviewed kind/schema owners
  -> HCM-1.2 reviewed selected-profile/descriptor owners
  -> HCM-1.3 reviewed planning packet and closeout
  -> future separately selected HCM-1.3 implementation
  -> future HCM-1.4 planning and implementation
```

The future code dependency order is exact:

```text
explicit ProfileSelectionRequest
  -> unchanged resolve_profile_selection
  -> immutable ResolvedInstanceProfile
  -> ResolvedArtifactRegistry::from_profile
  -> kind/instance/role/capability/validator metadata binding
  -> provider expansion and deterministic providers-before-consumers order
  -> structural validation by already-bound kind schema
```

No product or later-phase node is inserted.

## Allowed-scope and non-goal proof

Planning mutations are documentation-only. Future implementation has two
possible existing Rust changes (`lib.rs` export and, only if necessary, a
read-only descriptor iterator), one new owner module, one integration target,
one fixture subtree, proof/review evidence, and three narrow post-proof control-
pack classifications. Everything else is an explicit stop.

Forbidden scans and packet text preserve:

- no Rust/Cargo/product implementation during planning;
- no setup/doctor adoption;
- no YAML/content-authority migration;
- no condition, vocabulary, Context Resolution, lifecycle, intake, renderer,
  Projection, or overlay execution;
- no SDK/CLI/Tauri/Substrate/adapter/contract/dock work;
- no legacy profile, compatibility dispatch, fallback, ambient discovery,
  range/latest selection, invocation-time mutation, or dynamic filename/
  command/renderer dispatch; and
- no unsupported downstream gate closure or HCM-1.4 activation.

## Planning proof verdict before review

| Gate | Result |
|---|---|
| packet presence | PASS |
| internal consistency | PASS |
| cross-document consistency | PASS |
| live-source grounding | PASS |
| dependency ordering | PASS |
| allowed-scope enforcement | PASS |
| non-goal preservation | PASS |
| implementation separability | PASS |
| handoff validation | PASS |
| independent exact-subject review | PENDING immutable dispatch |

The parent must not claim planning approval until a fresh isolated built-in
`default` reviewer returns `CLEAN` over the exact manifested subject. Any valid
actionable finding invalidates this candidate; remediation requires a new proof
wall, a new subject fingerprint, and a different fresh reviewer.
