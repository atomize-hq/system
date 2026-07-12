# Phase and Slice Map

## Status

This is the provisional program decomposition. Phase 0 documentation/design slices may proceed after control-pack review. No Rust implementation slice is authorized until its own packet is created and approved.

## Sequencing rule

The program proceeds from semantic authority to representation, projection, transport, verification, and consumers:

```text
control-pack/design authority
  -> profile and artifact semantics
  -> canonical YAML truth
  -> vocabulary + Context Resolution + projections
  -> SDK + complete machine transports
  -> contract membrane + docks
  -> Substrate/Tauri/adapters
```

Do not begin with the CLI, Tauri UI, external docks, or Substrate integration before their underlying typed semantics exist.

## Phase 0 — Architecture and contract freeze

**Purpose:** turn the current idea lineage into implementation-grade target authority without changing Rust.

| Slice | Objective | Primary outputs | Exit gate |
|---|---|---|---|
| `HCM-0.1` | Establish the context-engineering control pack | `00`–`08`, handoff schema/template, active-doc pointers | pack is cohesive, repo-truth grounded, JSON artifacts valid, and reviewed |
| `HCM-0.2` | Freeze instance-profile, artifact-registry, vocabulary, and constitutional-root semantics | approved `02` and exact schema sections in `05`; decision records for remaining profile questions | every semantic field has an owner, default, validation rule, and explicit non-goal |
| `HCM-0.3` | Freeze Context Resolution and deterministic projection contracts | exact envelope, projection, omission, provenance, and promotion contracts | reveal/derive/synthesize boundaries are unambiguous and testable |
| `HCM-0.4` | Freeze crate ownership, `handbook-sdk`, CLI JSON, Tauri, and Substrate integration ladder | owner matrix, SDK use-case inventory, transport DTO contract, published proof plan | no use case depends on CLI prose; bridge and permanent boundary are distinct |
| `HCM-0.5` | Freeze contract-membrane and dock protocol boundaries | lifecycle, claim/evidence/verdict/gate contracts; process-dock capability/request/result contract | validators remain witnesses; Resolution limits proof; first proof dock is selected |
| `HCM-0.6` | Approve the implementation program and first slice packet | reviewed phase map plus first `slices/<id>/SPEC.md`, plan, and todo | first slice is independently implementable and has a complete proof wall |

### Phase 0 non-goals

- Rust changes;
- public API publication;
- legacy migration tooling;
- new CLI commands;
- Tauri scaffolding;
- actual dock execution;
- speculative third-party workflow adapters.

## Phase 1 — Profile and artifact semantic kernel

**Purpose:** replace the fixed artifact universe with a versioned profile-selected semantic model.

### `HCM-1.1` — Profile schema and shipped default

- define typed profile identity/version;
- define shipped opinionated default semantics directly;
- validate explicit profile selection and repository profile input;
- do not add a legacy profile merely to preserve old behavior.

### `HCM-1.2` — Descriptor-driven artifact registry

- replace enum-owned universe with profile-resolved descriptors;
- support first-party stable roles plus custom artifact IDs;
- make requiredness, dependencies, paths, and validators data-driven;
- preserve trusted repo-relative path enforcement.

### `HCM-1.3` — Profile-aware setup and doctor decisions

- make setup/doctor use typed profile decisions;
- keep CLI wording outside engine decisions;
- expose machine-readable profile/capability truth.

### Phase 1 exit gate

- one selected profile determines the complete artifact registry;
- custom artifacts do not require new enum variants;
- shipped defaults are intentional target semantics;
- no permanent compatibility dispatch remains;
- setup and doctor consume the same resolved profile truth.

## Phase 2 — Canonical YAML artifact authority

**Purpose:** make structured artifact data authoritative and human-readable documents derived.

### `HCM-2.1` — Vertical pilot artifact

Select one lower-risk artifact family after Phase 0 review. Project Context is the current leading candidate because a structured input model and deterministic renderer already exist.

- canonical YAML load/validate/write;
- derived Markdown projection;
- source/projection fingerprints;
- setup, authoring, doctor, and flow integration for the pilot;
- direct cutover of tests and fixtures.

### `HCM-2.2` — Constitutional-root artifact

- cut the constitutional-root artifact to canonical structured truth;
- preserve semantic root authority without requiring a literal filename;
- prove derived human review output and lifecycle behavior.

### `HCM-2.3` — Remaining shipped artifact families

- convert remaining first-party artifacts that need structured manipulation;
- decide explicitly which outputs are views only;
- remove superseded Markdown-authority helpers.

### Phase 2 exit gate

- each targeted artifact has exactly one editable canonical truth;
- Markdown is reproducibly derived;
- no user migration tooling or dual-read promise exists;
- every temporary internal cutover bridge named in `06` is deleted.

## Phase 3 — Vocabulary, Context Resolution, and projections

**Purpose:** make views and agent context profile-aware and resolution-aware.

### `HCM-3.1` — Vocabulary resolution

- axis-based labels;
- lexical and structural conflation;
- deterministic renderer consumption;
- stable machine semantics beneath local terminology.

### `HCM-3.2` — Context Resolution kernel

- configurable ordered stack;
- six explicit dimensions;
- inheritance, mutation, memory, validation, and escalation semantics;
- migration of useful work-level behavior without freezing L0-L3.

### `HCM-3.3` — Deterministic projection engine

- reveal and derive;
- collapse/expand request handling;
- omission and lossiness accounting;
- source/profile/projection fingerprints;
- no synthesis in the core implementation.

### `HCM-3.4` — Resolution-aware packet and pipeline adoption

- flow consumes Context Resolution rather than only byte budgets;
- pipeline scoped inclusion consumes namespaced shared semantics;
- gates distinguish local completion from parent promotion readiness.

### Phase 3 exit gate

- the same canonical truth produces multiple deterministic Resolution views;
- custom vocabulary appears consistently in generated surfaces;
- omitted claims cannot be misreported as passed;
- current work-level behavior is either intentionally represented or removed.

## Phase 4 — SDK and machine transports

**Purpose:** establish one ordinary-consumer facade and make every product transport thin.

### `HCM-4.1` — SDK owner and use-case inventory

- create or approve `handbook-sdk`;
- move composition out of CLI/compiler compatibility seams where appropriate;
- keep advanced owner-crate APIs public;
- expose capabilities, not internal module topology.

### `HCM-4.2` — Shared DTO and JSON Schema contract

- request/result/error/refusal envelopes;
- schema IDs and versioning;
- generated JSON Schema;
- deterministic serialization and compatibility tests.

### `HCM-4.3` — Complete CLI `--json` parity

- every nontrivial command emits exactly one response document on stdout;
- human rendering derives from typed results;
- progress/logging uses stderr;
- exit-code semantics are stable and documented.

### `HCM-4.4` — Tauri-ready command facade

- prove SDK DTOs can back thin Serde/Tauri commands;
- no normal-operation CLI subprocess dependency;
- no Tauri UI implementation required yet.

### `HCM-4.5` — Capability-driven Handbook skill

- update the installed Handbook skill to discover supported profiles, artifacts, schemas, vocabulary, and Resolution capabilities through machine interfaces;
- keep the agent workflow as structured-input gathering plus supported CLI/SDK invocation;
- prohibit prompt-owned reimplementation and untracked nested synthesis;
- preserve deterministic refusal and doctor/contract closeout.

### Phase 4 exit gate

- CLI, SDK, and Tauri adapter tests exercise the same typed use cases;
- `handbook-compiler` has an explicit retained or retirement posture;
- JSON Schema covers every supported machine response;
- no transport owns domain truth.

## Phase 5 — Contract membrane and docks

**Purpose:** implement executable contract authority and external witness integration.

### `HCM-5.1` — Contract lifecycle and claim model

- identity, version, draft/review/lock/active/close lifecycle;
- claims, invariants, severity, and required evidence;
- canonical structured contract records.

### `HCM-5.2` — Evidence, verdict, and gate engine

- normalized evidence;
- pass/fail/blocked/warning/not-observed/not-applicable/flaky;
- hard-fail semantics independent of weighted score;
- Resolution-qualified proof and promotion.

### `HCM-5.3` — Process dock protocol

- capability manifest;
- JSON request/result;
- isolation, timeout, cancellation, and refusal semantics;
- evidence and artifact references.

### `HCM-5.4` — First real validator dock proof

- choose one existing validator ecosystem;
- execute it through the process protocol;
- normalize evidence;
- prove a real contract gate without making the validator authoritative.

### Phase 5 exit gate

- a locked contract drives a real dock/evidence/verdict/gate path;
- missing or out-of-resolution evidence cannot produce false green;
- process protocol is stable enough for later Rust-native binding;
- `handbook contract ...` use cases exist behind SDK types before CLI polish claims completion.

## Phase 6 — Consumer adoption

### `HCM-6.1` — Bundled CLI Substrate bridge

- exact binary and schema versions;
- JSON-only consumption;
- isolated replaceable adapter;
- real Substrate product seam.

### `HCM-6.2` — Published SDK/owner APIs

- publish exact affected crate versions;
- registry-only external consumer proof;
- no path fallback.

### `HCM-6.3` — Direct Substrate crates.io adoption

- dedicated worktree from current Substrate tip;
- exact published versions;
- real seam using the new API;
- proof wall and bridge-replacement decision.

### `HCM-6.4` — Tauri product implementation

- GUI over the same SDK capabilities;
- artifact browsing, projection, contract execution, and evidence display;
- no new semantic authority in the frontend.

### `HCM-6.5` — Workflow adapter foundation

- adapter manifests and semantic mappings;
- profile/vocabulary/Resolution translation;
- no adapter marketplace or broad third-party inventory yet.

## Finding-driven decomposition protocol

Implementation and review findings may reveal that a task needs further decomposition. The active session must not silently widen.

Classify each finding as one of:

1. `local_remediation` — inside current packet authority;
2. `child_packet_required` — same slice, but independently reviewable work;
3. `cross_document_repair` — pack/spec/contract inconsistency must be fixed before code continues;
4. `resolution_escalation` — a broader design, authority, or validation decision is required;
5. `external_blocker` — dependency/environment/human action outside the repo;
6. `proof_gap` — implementation may exist, but required evidence is missing;
7. `future_program` — valuable but outside this program's approved target.

The session writes a handoff record and stops at its packet boundary. An orchestration session then:

- revalidates the finding against pack and live truth;
- decides whether to resume, repair docs, decompose, escalate, or defer;
- updates this map and affected contracts when authority changes;
- creates a durable dispatch prompt for the next session.

Only the orchestration/design authority may promote a discovered child packet into the active slice plan. Implementation output alone does not change program scope.

## Slice packet layout

When authorized, each implementation slice uses:

```text
slices/<slice-id>/
├── SPEC.md
└── tasks/
    ├── plan.md
    └── todo.md
```

Optional prompt or evidence artifacts may be added only when the slice requires them. Do not copy the whole control pack into each packet.
