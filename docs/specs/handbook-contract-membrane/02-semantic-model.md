# Semantic Model

## Purpose

This file defines the conceptual model that profiles, artifacts, vocabulary, Context Resolution, projections, memory, contracts, and adapters must share.

The system must be customizable without reducing every concept to an arbitrary string.

## Stable semantics and configurable language

Handbook maintains stable internal semantic identifiers. A user profile controls display language, local work-unit names, intentional role conflation, and artifact presentation.

Example stable roles:

```text
constitutional_authority
project_context
environment_inventory
delivery_unit
coordination_horizon
implementation_unit
execution_envelope
atomic_action
contract
claim
evidence
verdict
gate
```

A user may render these as Charter, System Brief, Feature, Phase, Slice, Packet, Task, or any other terms.

Stable identifiers exist for validation and translation. They are not a language-policing mechanism.

## Instance profile

An instance profile defines the Handbook semantics for one repository or project.

It contains:

- profile identity and schema version;
- artifact registry;
- vocabulary profile;
- Resolution stack;
- projection definitions;
- validator/dock requirements;
- shipped defaults and explicit repository overrides;
- optional adapter overlays.

Recommended first-version precedence:

```text
shipped default profile
  -> selected named profile
  -> repository profile
  -> explicit invocation overrides
```

Ambient, silently discovered local overrides are out of scope initially.

## Artifact registry

The artifact registry replaces the fixed assumption that every repository has exactly the same canonical documents.

Each artifact descriptor should identify at least:

- stable artifact ID;
- semantic role;
- user-facing label;
- canonical path or path template;
- schema reference and schema version;
- requiredness rule;
- dependencies;
- lifecycle/lock requirements;
- authoring strategy;
- supported projections;
- validation profile;
- applicable repository/project conditions.

### Constitutional root

The shipped model retains one required constitutional-root semantic role. Its display name and path may be profile-defined; the default may remain Charter.

This is a semantic invariant, not a literal filename requirement.

### Custom artifacts

Profiles may define artifact kinds beyond the shipped defaults.

- First-party artifacts may receive specialized deterministic authoring and rendering.
- Custom artifacts receive generic schema validation, lifecycle, projection, and contract behavior first.
- A custom artifact does not require a new Rust enum variant merely to exist.

## Vocabulary model

Vocabulary is organized by semantic axes, not a flat synonym map.

Recommended axes:

- scope hierarchy;
- delivery units;
- execution units;
- cadence/timebox;
- lifecycle/governance;
- evidence/verification;
- organizational roles.

### Intentional lexical conflation

A user may assign the same displayed term to distinct semantic roles.

Example:

```yaml
vocabulary:
  labels:
    delivery_unit: feature
    coordination_horizon: feature
```

The internal distinction remains available to validators and adapters.

### Intentional structural conflation

A user may genuinely treat multiple roles as one workflow unit.

Example:

```yaml
work_units:
  implementation:
    id: task
    absorbs:
      - implementation_unit
      - execution_envelope
      - atomic_action
```

The profile makes the collapse explicit so adapters can report whether the target system can preserve it.

## Context Resolution

Use namespaced implementation terminology such as:

- `ContextResolution`;
- `ContextResolutionEnvelope`;
- `ResolutionProjection`;
- `ResolutionEscalation`;
- `ResolutionPromotion`.

Do not use an unqualified `Resolution` type in code because route, path, selector, and dependency resolution already have different meanings.

### Definition

Context Resolution is the declared operating granularity of an artifact view, agent run, or workflow stage, together with the context, authority, memory, tooling, and validation envelope appropriate to that granularity.

### Six dimensions

Every envelope can independently constrain:

1. **Scope horizon** — how much of the system is in view.
2. **Detail resolution** — how granular the visible information is.
3. **Temporal horizon** — immediate action versus long-range outcome.
4. **Authority horizon** — which truth may be modified.
5. **Memory horizon** — where observations may be persisted.
6. **Validation horizon** — what level of completion may be claimed.

These dimensions must not be collapsed into a token budget or one numeric level.

### Ordered stack

The first version uses a configurable ordered stack rather than an arbitrary graph.

The shipped default may resemble:

```text
strategic
  -> coordination
  -> execution
  -> operation
```

Profiles may rename, add, remove, or intentionally collapse levels. Work-unit terms and resolution levels are related but not identical.

The current `L0`–`L3` work-level mechanism is a useful precursor, not the final semantic model, because it mixes scope levels with workflow phases such as merge/quality gate.

## Resolution envelope

Conceptual shape:

```yaml
schema_id: handbook.context-resolution-envelope
schema_version: "1.0"
active_level: execution
goal_ref: work.task.capture-screenshot
dimensions:
  scope_horizon: assigned_task
  detail_resolution: normal
  temporal_horizon: immediate
  authority_horizon: task_local
  memory_horizon: execution_record
  validation_horizon: task_closeout
parent_refs:
  - feature.browser-evidence
inherited_constraint_refs:
  - charter.security
  - feature.browser-evidence.acceptance
allowed_mutations:
  - crates/browser/**
forbidden_mutations:
  - docs/canon/**
escalation_triggers:
  - parent_contract_conflict
  - architectural_decision_required
  - insufficient_acceptance_criteria
```

Higher-horizon constraints constrain lower-horizon work. Lower-horizon observations do not rewrite higher-authority truth without promotion.

## Projection semantics

A projection is a derived view from canonical structured truth for a declared requester, surface, vocabulary profile, and Resolution envelope.

### Reveal

Expose canonical fields or referenced child artifacts already present.

- deterministic;
- non-inventive;
- canonical-source preserving.

### Derive

Compute a deterministic aggregate, selection, normalization, or presentation from canonical fields.

- deterministic;
- provenance-bearing;
- may be lossy as a view;
- cannot destroy or replace source truth.

### Synthesize

Generate new interpretation, prose, decomposition, or candidate detail through an agent/model.

- not part of the initial projection engine;
- candidate-only;
- requires explicit provenance and a review/promotion gate;
- if Handbook owns the use case later, it must invoke `unified-agent-api` programmatically rather than through prompt-shaped shell behavior.

## Collapse and expansion

User-facing mental model:

- **collapse** projects broader canonical meaning into a smaller working set;
- **expand** requests broader horizon or finer canonical detail when the current envelope is insufficient.

Implementation-level operations should remain explicit:

- project;
- reveal;
- derive;
- escalate;
- aggregate;
- promote.

Expansion cannot invent detail absent from canonical truth. Missing detail either remains missing, triggers escalation, or enters a candidate synthesis workflow.

## Projection provenance

Every projection should record:

- source artifact references and fingerprints;
- profile and vocabulary versions;
- projection definition/version;
- requested Resolution envelope;
- fields or claims included;
- fields or claims omitted;
- lossiness classification;
- deterministic derivation identifiers;
- synthesis provenance when applicable;
- promotion eligibility.

## Memory semantics

Memory is Resolution-tagged rather than flat.

Recommended default behavior:

- strategic memory: rare, high-authority, explicitly reviewed;
- coordination memory: contracts, decisions, feature/seam truth;
- execution memory: task outcomes, proof, local tradeoffs;
- operation memory: transient observations and command output.

An observation is not durable higher-horizon memory merely because an agent emitted it.

Promotion requires:

- a target memory horizon;
- source evidence references;
- validation appropriate to the target horizon;
- explicit authority to update that truth;
- a durable promotion record.

## Resolution-aware validation

A projection can prove only what it exposes and observes.

- Included and observed required claims may pass.
- Omitted or unavailable claims remain `not_observed` unless another evidence source covers them.
- A local task can close without claiming that its parent feature or phase is complete.
- A gate must distinguish local completion from promotion readiness.
- An external dock declares the Resolution envelope under which it collected evidence.

## Adapter implications

Future workflow adapters map:

- stable semantic roles;
- vocabulary labels and aliases;
- structural conflations;
- artifact kinds;
- Resolution placement;
- projection expectations;
- lifecycle and gate semantics.

An adapter translates between systems. It does not mutate Handbook canonical truth merely to imitate another tool's terminology.
