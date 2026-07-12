# Contracts, Schemas, and Gates

## Status

These are preliminary target contracts to be frozen by Phase 0 design slices. They are architecture authority for direction, not yet published API guarantees.

## Schema policy

- Canonical human-authored structured records use YAML where appropriate.
- Every structured record has a stable `schema_id` and version.
- Transport DTOs serialize to JSON and publish JSON Schema.
- OpenAPI is an optional HTTP adapter output, not the CLI/Tauri/SDK authority.
- Schema versions and semantic contract versions are explicit and independently reviewable.
- Unknown required semantics fail closed; optional extension fields require a declared extension mechanism.

## Instance profile contract

Conceptual minimum:

```yaml
schema_id: handbook.instance-profile
schema_version: "1.0"
profile_id: default
artifacts: []
vocabulary: {}
context_resolution: {}
projections: []
docks: []
```

Required gates:

- unique stable IDs;
- valid repo-relative paths;
- exactly one constitutional-root role unless Phase 0 explicitly changes this invariant;
- dependency graph has no invalid references or cycles;
- custom artifacts reference valid schemas;
- vocabulary conflations are explicit;
- Resolution stack is ordered and connected;
- projection definitions reference valid source roles and target levels.

## Artifact descriptor contract

```yaml
id: project_context
role: project_context
label: Project Context
canonical_path: .handbook/project_context/project-context.yaml
schema_ref: handbook.artifact.project-context/v1
required_when: always
depends_on:
  - constitutional_root
authoring:
  strategy: first_party
projections:
  - project_context.review_markdown
validation_profile: project_context.v1
```

Custom descriptors may use generic authoring/validation. Specialized first-party behavior must key off stable capability/role semantics, not filename matching.

## Vocabulary contract

The vocabulary schema distinguishes labels, aliases, and structural absorption:

```yaml
labels:
  coordination_horizon: phase
  delivery_unit: feature
  implementation_unit: slice
  execution_envelope: packet
  atomic_action: task
aliases:
  implementation_unit:
    - task
absorptions: []
```

Duplicate displayed labels are legal. Ambiguity matters only when a machine operation cannot resolve a stable role from the surrounding typed context.

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

```yaml
schema_id: handbook.projection-result
schema_version: "1.0"
projection_id: agent.execution-packet
source_refs: []
source_fingerprints: []
profile_fingerprint: ""
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

## SDK command contract

Every ordinary use case has:

- a typed request;
- a typed result;
- structured expected blocked/refused states;
- a stable operation identifier;
- schema and capability versions;
- deterministic serialization.

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
