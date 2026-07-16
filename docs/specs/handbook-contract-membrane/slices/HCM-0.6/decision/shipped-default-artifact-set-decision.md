# HCM-0.6 Shipped Default Artifact Set Decision

## Status and authority

**Status:** approved  
**Approved:** 2026-07-16  
**Decision authority:** explicit user/product decision after the reviewed HCM-0.6
research and candidate comparison  
**Implementation effect:** target documentation and future shipped-profile data
only; no runtime, schema publication, setup, renderer, intake, CLI, SDK, Tauri,
Substrate, or HCM-0.7 implementation is authorized

This record is the authoritative HCM-0.6 shipped-default decision. It replaces
the candidate recommendation as the decision surface while retaining the
research and comparison as rationale/provenance. Exact identities below are
machine semantics; labels and repository vocabulary remain presentation.

## Decision summary

Handbook v1 ships a curated first-party catalog of exactly six artifact kinds.
The shipped root profile selects exactly three long-lived project-level
instances. Two are always required. The Environment Context instance is
conditionally required through one evidence-gated exact condition. The root
profile selects no Work Specification, Decision Record, or Risk Record
instance.

The catalog is broader than the root-profile instance set. That distinction is
intentional: first-party support does not imply universal selection,
materialization, or requiredness.

## Exact stable-role registry decision

`handbook.roles.core@1.0.0` remains immutable. The additive registry is
`handbook.roles.core@1.1.0`; it preserves every 1.0.0 entry byte-for-semantic-
byte and adds one distinct artifact role:

| Role ID | Canonical fallback label | Category |
|---|---|---|
| `environment_context` | Environment Context | `artifact` |

The exact registry pins are:

| Registry ref | Registry fingerprint | Disposition |
|---|---|---|
| `handbook.roles.core@1.0.0` | `sha256:7d9407b43ebdda9ac73206bdfcb0e60e3906bdba980820ed12717d63c28e5c3f` | immutable and still available |
| `handbook.roles.core@1.1.0` | `sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029` | additive shipped-default registry |

These fingerprints are the frozen uniform-definition derivation over the
authored registry order and normalized content, excluding only
`registry_fingerprint`. Every shipped profile, kind, and vocabulary definition
using 1.1.0 must pin the exact 1.1.0 ref/fingerprint pair. A minor version never
substitutes automatically for 1.0.0, and changing an existing role's meaning is
not an additive minor-version change.

`environment_context` is a machine identity, not required visible terminology.
It is neither a subtype nor an alias of `project_context`; the shared suffix
does not merge their semantics. Its ID and category grant no authority,
applicability, requiredness, schema shape, materialization, or exhaustive
operational ownership.

## Exact first-party kind catalog

Every kind below begins at its own independent semantic version `1.0.0` and
pins `handbook.roles.core@1.1.0` plus its exact registry fingerprint. Shared
initial versions create no lockstep evolution or cross-kind compatibility.

| Exact kind ref | Durable semantic responsibility | Supported stable roles | Semantic capability contracts |
|---|---|---|---|
| `handbook.artifact-kind.project-authority@1.0.0` | durable normative project authority | `constitutional_authority` | `constitutional_root` via `handbook.capabilities.constitutional-root@1.0.0` |
| `handbook.artifact-kind.project-context@1.0.0` | maintained factual project context | `project_context` | none |
| `handbook.artifact-kind.environment-context@1.0.0` | maintained environment/runtime operational facts | `environment_context` | none |
| `handbook.artifact-kind.work-specification@1.0.0` | bounded intended work or change | `coordination_horizon`, `delivery_unit`, `implementation_unit`, `execution_envelope`, `atomic_action` | none |
| `handbook.artifact-kind.decision-record@1.0.0` | one discrete project- or work-related decision | none; v1 instances use explicit `role_ref: null` | none |
| `handbook.artifact-kind.risk-record@1.0.0` | maintained risk and evidence-qualified uncertainty | none; v1 instances use explicit `role_ref: null` | none |

Kind identity, role support, and capability conformance are separate. A kind ID
does not grant a role, capability, authority, currentness, lifecycle state, or
instance identity. Each kind has its own definition, fingerprint, canonical
content-schema binding, compatibility history, and future version sequence.
Kind versions are distinct from record-schema, content-schema, registry,
package, and product versions; exact refs admit no implicit latest or version
range.

The Work Specification allowlist permits an individual instance to select at
most one supported role explicitly, or explicit null when permitted. Support
never auto-selects a workflow level, creates an artifact at every level,
establishes hierarchy/cardinality, or turns a canonical work specification into
a task tracker. Adapters preserve the selected role or apply an explicit
vocabulary absorption with declared loss behavior; labels and filenames cannot
select it.

A Decision Record or Risk Record describes a subject; it does not occupy that
subject's project, environment, or workflow role. Their v1 subject, scope,
evidence, and posture relationships require separately approved typed
references. `role_ref: null` permits no inference from kind ID, label, path,
scope, methodology, or external terminology.

## Exact shipped root-profile instance set

All three canonical YAML paths are beneath `.handbook/project/`. Directory and
filename placement is profile configuration, never semantic discovery.

| Instance ID | Exact kind ref | `role_ref` | Selected capability IDs and exact contract refs | Default label | Canonical path | Requiredness | Exact condition ref |
|---|---|---|---|---|---|---|---|
| `project_authority` | `handbook.artifact-kind.project-authority@1.0.0` | `constitutional_authority` | `constitutional_root` / `handbook.capabilities.constitutional-root@1.0.0` | Charter | `.handbook/project/charter.yaml` | `always` | `null` |
| `project_context` | `handbook.artifact-kind.project-context@1.0.0` | `project_context` | none | Project Context | `.handbook/project/context.yaml` | `always` | `null` |
| `environment_context` | `handbook.artifact-kind.environment-context@1.0.0` | `environment_context` | none | Environment Context | `.handbook/project/environment.yaml` | `conditional` | `handbook.condition.project.managed-operational-surface@1.0.0` |

The root profile selects no instance of:

- `handbook.artifact-kind.work-specification@1.0.0`;
- `handbook.artifact-kind.decision-record@1.0.0`; or
- `handbook.artifact-kind.risk-record@1.0.0`.

Absence of those instances asserts neither that no work, decisions, nor risks
exist nor that their obligations are optional. Setup and baseline doctor must
not require or scaffold empty general-purpose artifacts for them. Work and
decision records remain separately identified canonical records, not rolling
singletons or mutable omnibus logs; future aggregate views reference source
records rather than becoming peer authority.

The lack of a root Decision Record does not prevent constitutional policy from
requiring separately identified records for qualifying decisions. The lack of
a root Risk Record removes no constitutional posture, contract, evidence, or
gate obligation and does not assert that the repository has no risks. Selected
risk records may later supply typed posture-evaluation inputs, but posture
evaluation cannot universally depend on their presence.

Paths, labels, kind refs, role refs, capability IDs, and instance IDs occupy
separate typed namespaces even when spelling matches. Consumers resolve exact
descriptors and must not infer semantics through a directory or filename.
Co-location does not merge responsibilities. A later path change requires
explicit profile/migration handling and cannot create two authorities or rely
on fallback filename discovery.

## Unique constitutional root

`project_authority` is the unique root-profile instance that selects the stable
capability ID `constitutional_root`; the kind binds that capability to the
exact contract `handbook.capabilities.constitutional-root@1.0.0`. The same
instance separately selects the stable role `constitutional_authority` and is
always required.

Requiredness, role, label, path, kind name, or profile presence alone grants no
constitutional authority. Capability conformance must cover the frozen policy,
decision, exception/waiver, engineering-posture, red-line, review-trigger, and
reassessment bindings and semantic validators. Missing or invalid identity,
bindings, content, approval, or validation fail closed. Handbook and its agents
cannot fabricate constitutional content from defaults, templates, Project
Context, external systems, or inference.

Project Context, Environment Context, work specifications, decision records,
risk records, external systems, and derived views may inform or reference the
root but never become peer constitutional roots. Vocabulary or adapters cannot
erase or reinterpret the stable role or capability. An adapter that cannot
preserve them reports loss or refuses.

## Project Context boundary

The single always-required `project_context` instance is a maintained
project-level orientation and reference surface. Stable means its identity and
responsibility persist; its content must change as authoritative current facts
change.

It records or references authoritative current facts. It cannot restate
constitutional rules, manufacture facts by inference, absorb Environment
Context or another separately owned role, or become an exhaustive duplicate of
specialized inventories or external authorities. Requiredness grants no
constitutional authority, accuracy, completeness, or freshness. Absence or
structural/semantic invalidity fails closed; exact minimum content, factual
boundary, freshness thresholds, stale-fact consequences, and external-reference
rules remain subordinate decisions.

## Environment Context boundary and applicability

The conditional `environment_context` instance owns maintained environment,
deployment, runtime-dependency, and safe configuration metadata. It may store
durable operational facts and authoritative references. It must not become a
secret store, copy volatile live values unnecessarily, represent inferred
deployment state as fact, duplicate topology owned elsewhere, own runbooks,
mirror exhaustive software catalogs, or claim all operational information.

One selected artifact instance does not imply one deployment/runtime
environment. Internal environment modeling and cardinality remain subordinate
schema decisions. The descriptor and canonical path retain stable identity when
the condition is false and no artifact file is required.

### Exact condition

The instance uses
`handbook.condition.project.managed-operational-surface@1.0.0`.

The condition is applicable when an independently authoritative verified
project fact or admitted evidence establishes continuing project responsibility
for a runtime, deployment, operated automation, or operational integration
surface whose durable operational facts must be maintained for safe operation,
support, or change. Production deployment is not required. A non-production
surface may qualify. A language runtime, package manager, local-development or
test environment, container file, or CI configuration alone is insufficient.
Profile declaration may contribute an assertion or evidence reference, but
profile opt-in alone cannot replace the verified fact.

| Outcome | Required meaning | Requiredness effect |
|---|---|---|
| `true` | current independent authoritative fact or admitted evidence proves continuing responsibility for at least one qualifying surface | the artifact is applicable and missing/invalid content fails closed |
| `false` | affirmative current authoritative fact or admitted evidence proves no qualifying responsibility exists | the artifact is inapplicable; absence cannot make setup, doctor, or profile resolution incomplete |
| `unknown` | the condition is resolvable, but available evidence proves neither true nor false | indeterminate; never coerced to false or true |
| `unresolved` | the exact condition definition, required input, reference, or evidence cannot resolve | indeterminate; never coerced |
| `stale` | otherwise admissible evidence lies outside the explicit freshness basis | indeterminate; never coerced |
| `refused` | evaluation is circular, malformed, untrusted, disallowed, or materially contradictory without an approved precedence contract | indeterminate; never coerced |

Evaluation is deterministic over exact authoritative inputs/evidence and an
explicit freshness basis. It excludes Environment Context as its own
applicability input and records the complete fingerprinted input/evidence
closure. Identical definitions, facts, evidence, and freshness basis reproduce
the same outcome and closure fingerprint. Bare silence and unchecked profile
flags never produce `false`. Contradictory evidence produces `refused` unless a
separately approved authority/precedence contract resolves it.

The condition's record schema, exact input bindings, admitted evidence types,
outcome precedence, freshness thresholds, evaluator implementation, transport,
and migration behavior remain undecided. Conditional requiredness authorizes no
inferred facts, empty scaffolding, or automatic creation. Applicability becoming
false neither deletes a retained artifact nor establishes that retained content
is current or authoritative.

## Lifecycle and review posture

| Kind | Approved responsibility and reassessment posture |
|---|---|
| Project Authority | long-lived normative identity; explicit reviewed amendment; reassess on governance, policy, exception, red-line, engineering-posture, or posture-trigger changes; preserve provenance and prior history |
| Project Context | maintained current-state identity; reassess owned facts, references, boundaries, ownership, and material bounded-topology changes; never infer missing facts or expand into an exhaustive catalog |
| Environment Context | maintained while applicable; reassess responsibility, durable dependencies, authoritative references, operational-fact validity, and applicability evidence when they change |
| Work Specification | one stable bounded intended-change identity; explicit lifecycle/status changes; never a rolling singleton for unrelated work; existence alone asserts no approval, activity, implementation, verification, or current fact |
| Decision Record | one stable discrete-decision identity; preserve history and distinguish record status from external effects; supersession never rewrites prior identity |
| Risk Record | evidence-qualified uncertainty; reassess relevant evidence, assumptions, ownership, treatment, validity basis, and referenced constitutional-posture changes; never overstate certainty or auto-enact posture |

A review trigger creates an obligation to reassess. It does not automatically
mutate, invalidate, supersede, delete, or recreate an artifact. Stable identity
does not mean immutable content; revision, state change, supersession, and
history must not silently replace one semantic identity with another. Exact
lifecycle-policy IDs, states/transitions, timestamps, freshness thresholds,
retention/archival rules, mutation operations, automation, notifications, and
renderer behavior remain subordinate decisions.

## First-party intake, renderers, and Projection posture

Each of the six kinds must publish before Phase 3:

1. at least one compatible schema-backed first-party intake definition; and
2. at least one fixed deterministic human-review renderer.

Canonical YAML is authoritative. Intake produces typed records or candidates
subject to schema, semantic validation, coverage, approval, and promotion; its
existence selects no instance and cannot manufacture missing truth or authority.
Project Authority intake must satisfy the complete frozen constitutional-root
coverage, approval, and reassessment requirements.

Fixed renderer outputs are derived, deterministic, non-authoritative, and
outside the capitalized Phase-3 `Projection` contract. They are not editable
peer truth and cannot silently feed canonical mutations. The shipped root
profile initially selects no capitalized Projection definitions. Future
Projections require separately published exact definitions and explicit
compatible profile selection.

This support posture includes the unselected Work Specification, Decision
Record, and Risk Record kinds. It does not approve exact intake/renderer IDs,
schemas, prompts, modes, templates, output paths, audiences, adapters, or
implementations.

## Deferred catalog responsibilities

Handbook v1 initially ships no distinct first-party kind for:

- operational procedures or runbooks;
- project-wide quality strategy;
- software, service, component, API, or resource catalogs; or
- any other semantic responsibility not in the exact six-kind table.

Profiles, schema-backed custom kinds, external authoritative systems, and future
adapters remain available. Deferral is not permanent rejection, but a deferred
responsibility cannot be introduced indirectly by expanding an approved kind
until it substantially owns the deferred role.

Environment Context may reference procedures but cannot own or embed them as
environment truth. Quality remains with durable constitutional posture, bounded
work expectations, and executable contracts/evidence/gates; constitutional
authority must not absorb mutable implementation verification plans. Project
Context may summarize major topology and reference authoritative catalogs;
Environment Context may reference operational dependencies; neither may become
a stale exhaustive mirror. JSON compatibility or JSON Schema conformance proves
structure, not catalog meaning, identity, authority, or currentness.

A deferred kind may be reconsidered only through a separate evidence-backed
catalog amendment showing a stable Handbook-owned semantic core and concrete
cross-system consumer that Handbook must validate, resolve, render, or project
without duplicating an authoritative external system. Closing this artifact
catalog does not close Handbook's non-artifact semantic records or executable
contract/evidence/gate system.

## Rationale and rejected alternatives

The approved posture derives from the reviewed Candidate B direction but is not
identical to its initial candidate values. It preserves a small universal core,
supplies common first-party schemas without requiring empty files, separates
project authority from facts and intended change, and bounds operational facts
through evidence-gated applicability.

Rejected for v1:

- **Minimal catalog:** too much common environment/risk/decision capability
  would fragment immediately into repository-local kinds.
- **Governance-heavy default:** universal environment, risk, quality, catalog,
  and procedure files create disproportionate onboarding, overlap, and drift.
- **Optional root risk instance:** selection still creates discoverability and
  empty-file pressure; the kind remains first-party but unselected.
- **Root work or decision singleton:** either becomes an empty placeholder or a
  mutable omnibus record that collapses separately identified work/decisions.
- **Role inference from names:** kind, role, capability, instance, label, path,
  and adapter vocabulary require explicit typed identities.

## Explicitly unresolved subordinate contracts

This decision does not approve any canonical content-schema identity or fields,
exact content boundary beyond the invariants above, kind/definition fingerprint
other than the exact stable-role registry fingerprints, lifecycle-policy ID,
intake/renderer definition ID, initial content, setup/materialization behavior,
migration/alias policy, permissions, downstream-profile cardinality, adapter or
external source-of-truth mapping, Projection, or implementation.

HCM-1.1 through HCM-2.x may implement only the exact catalog and root-profile
selection above, with each remaining subordinate contract reviewed through its
own authorized slice. HCM-0.7 remains separate and is not started by this
decision.
