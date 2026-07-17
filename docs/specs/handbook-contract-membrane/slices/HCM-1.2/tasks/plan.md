# Implementation Plan: HCM-1.2 Profile Schema and Shipped Default

## Overview

Build the additive `handbook-engine` definition/profile boundary on the
completed HCM-1.1 registry foundation. Land bounded source/schema primitives,
then exact definition families, descriptors, profile layering/resolution, and
the HCM-0.6 shipped root. Current product paths remain fixed until HCM-1.3/
HCM-1.4. Canonical `07`/`08` own orchestration and closeout mechanics.

## Authority and sequencing

- Consume HCM-1.1 only as reviewed dependency evidence.
- Preserve its exact-ref, safe-source, closure, and fingerprint rules.
- Enable a non-empty ref only after its exact typed producer is green.
- Publish only the literal definitions and HCM-0.6 set frozen in SPEC.
- Keep kind, instance, role, capability, label, path, condition, and profile
  namespaces separate.
- Require explicit typed sources and schema roots; no ambient discovery.
- Keep enum/layout/setup/doctor/flow behavior untouched.
- Leave PG-PROFILE-01, PG-ARTIFACT-01, PG-KIND-01, and PG-KIND-02 open.

## Dependency graph

```text
1 source/request envelope -> 2 request-wide schema shape/budget surface
2 -> 3 Project Authority schema
2 -> 4 Project + Environment Context schemas
2 -> 5 Work + Decision schemas
2 -> 6 Risk schema
3 -> 7 constitutional capability + validator metadata
3 + 4 + 7 -> 8 selected-root kind definitions
5 + 6 -> 9 unselected kind definitions
1 -> 10 condition producer
1 -> 11 vocabulary producer
1 -> 12 Context Resolution policy producers -> 13 shipped stack
8 + 10 -> 14 artifact descriptors
3-14 -> 15 authored profile/layering -> 16 exact resolution
16 -> 17 shipped root/repository fixture -> 18 full proof/closeout
```

Every task is an S/M green increment with approximately five touched files or
fewer. New definition assets count as files. Do not collapse adjacent tasks.

## Task 1: Closed request and typed source envelope

**Work:** Add the exact `DefinitionSource`, two-field binding, structural
`ProfileSelectionRequest`, stable errors, `SymbolicId`, path/root grammar,
count/byte ceilings, and stages 1-4 of fail-fast admission. Do not decode/load
new semantic definitions or decide source usage.

**Acceptance:** Exact request fields; declared identities unique; a built-in
variant's repeated exact ref equals its binding ref structurally; 64 profiles/
512 bindings/32 roots; 1024-byte/64-component paths; 1-MiB source/8-MiB
aggregate sentinel reads; complete `SymbolicId` boundary/rejection matrix;
errors are bounded and never expose absolute paths or bytes. Loaded record
class/ref validation belongs to its producer task; unused-source accounting
belongs only to Task 16.

**Dependencies:** HCM-1.1. **Likely files:** `instance_profile.rs`, optional
`profile_selection.rs`, `lib.rs`, one focused request/source test. **Size:** M.

## Task 2: Request-wide schema budget and shape query

**Work:** Extend the HCM-1.1 schema owner with one request-wide distinct-
document visited set/budget and the normalized read-only `binding_shape` query
in SPEC. Retain parsed values privately; expose no authored value.

**Acceptance:** Shared 128-document and 32-reference-depth limits; cross-entry
identity dedup rules; exact 128/129 and shared-document cases; query implements
the exact RFC 6901/`$ref`/closed-property/determinate-type algorithm; raw values
and source paths cannot escape. Every schema-entry document ref and normalized
transitive `$ref` target enforces 1024/1025-byte and 64/65-component cases
before filesystem open.

**Dependencies:** Task 1. **Likely files:** `schema_registry.rs`,
`instance_profile.rs`, one focused schema-budget/shape test. **Size:** M.

## Task 3: Project Authority schema

**Work:** Add the exact Project Authority schema entry/document only.

**Acceptance:** Complete closed shape and all nested required fields/bounds;
valid minimum; per-field missing/extra/type/bound negatives; exact entry/
closure fingerprint and two package members.

**Dependencies:** Task 2. **Likely files:** one `.entry.yaml`, one
`.schema.json`, one focused Project Authority schema test. **Size:** S.

## Task 4: Project and Environment Context schemas

**Work:** Add the two exact context schema entry/document pairs.

**Acceptance:** Complete literal shapes, structural duplicate rejection,
forward/reverse unique-order acceptance without content normalization, valid
minimums, every boundary/unknown-field negative, and Environment Context
secret-surface impossibility; exact four-asset package proof.

**Dependencies:** Task 2. **Likely files:** four definition assets and one
focused context-schema test. **Size:** M.

## Task 5: Work Specification and Decision Record schemas

**Work:** Add the two exact unselected schema entry/document pairs.

**Acceptance:** Exact fields/status enums/set rules/bounds, positive minimums,
per-field negatives, no root materialization authority, exact package proof.

**Dependencies:** Task 2. **Likely files:** four definition assets and one
focused work/decision schema test. **Size:** M.

## Task 6: Risk Record schema

**Work:** Add the exact Risk Record schema entry/document.

**Acceptance:** Exact fields/status enum/non-empty review basis, full boundary
matrix, no root materialization authority, exact two-asset package proof.

**Dependencies:** Task 2. **Likely files:** two definition assets and one
focused risk-schema test. **Size:** S.

## Task 7: Constitutional capability and validator metadata

**Work:** Add the complete capability record, complete nine-rule validator
profile, and schema-aware binding compatibility.

**Acceptance:** Literal producer equality; exact nine pointers/types/
cardinalities/empty policies; missing/extra/duplicate/ambiguous/boolean/union/
wrong-shape/stale/wrong-class/changed-byte refusal; no executable validation.
Validator binds only the non-definition capability ID; validator -> capability
-> kind fingerprint order is acyclic and a back-edge refuses.

**Dependencies:** Tasks 2 and 3. **Likely files:** two definition assets,
`semantic_capability_registry.rs`, `artifact_kind_registry.rs`, one focused
capability/binding test. **Size:** M.

## Task 8: Selected-root kind definitions

**Work:** Add Project Authority, Project Context, and Environment Context kind
definitions after all their producers exist.

**Acceptance:** Exact three-member set; roles core 1.1.0; exact schema refs;
only Project Authority advertises constitutional capability; every later-owned
field remains empty/null; source permutation and package hashes reproduce.

**Dependencies:** Tasks 3, 4, and 7. **Likely files:** three kind assets,
`artifact_kind_registry.rs`, one focused selected-kind test. **Size:** M.

## Task 9: Unselected kind definitions

**Work:** Add Work Specification, Decision Record, and Risk Record kinds.

**Acceptance:** Exact three-member set; exact schemas/role allowlists; no
capability or instance selection; later-owned fields empty/null; deterministic
fingerprints and package proof.

**Dependencies:** Tasks 5 and 6. **Likely files:** three kind assets,
`artifact_kind_registry.rs`, one focused unselected-kind test. **Size:** M.

## Task 10: Managed-operational-surface condition producer

**Work:** Add the complete literal condition record and typed registry only.

**Acceptance:** Exact identity, ordered outcomes/inputs/precedence/effects,
freshness and independent-basis metadata, self-reference exclusion, empty
extensions, fingerprint; every structural/stale/wrong-class negative; no
evaluator/setup/doctor/scaffold behavior.

**Dependencies:** Task 1. **Likely files:** one condition asset,
`project_condition_registry.rs`, `lib.rs`, one focused condition test. **Size:** S.

## Task 11: Shipped vocabulary producer

**Work:** Add the complete empty-mapping vocabulary and small typed registry.

**Acceptance:** Literal record equality, roles core pair, exact fingerprint,
wrong/stale/changed-byte refusal; no label/alias/absorption product behavior.

**Dependencies:** Task 1. **Likely files:** one vocabulary asset,
`vocabulary_registry.rs`, `lib.rs`, one focused vocabulary test. **Size:** S.

## Task 12: Context Resolution policy producers

**Work:** Add the three complete matcher/escalation/promotion metadata records
and typed registry before a stack references them.

**Acceptance:** Every literal field/type/order/bound and matcher grammar;
exact identities/fingerprints; structural/wrong-class/stale/changed refusal;
no selector, escalation, or promotion execution.

**Dependencies:** Task 1. **Likely files:** three policy assets,
`context_resolution_registry.rs`, `lib.rs` with co-located unit tests. **Size:** M.

## Task 13: Shipped Context Resolution stack

**Work:** Add the literal stack after Task 12.

**Acceptance:** Exact four levels/six domains/ranks/defaults/policy refs;
broad-to-narrow validation and closure fingerprint; complete drift negatives;
no envelope/Projection/work-level behavior.

**Dependencies:** Task 12. **Likely files:** one stack asset,
`context_resolution_registry.rs`, one focused stack integration test. **Size:** S.

## Task 14: Artifact instance descriptors

**Work:** Add the closed descriptor/dependency types and exact three shipped
descriptor records independently from kinds.

**Acceptance:** Exact whole-record equality; unique IDs/paths; typed kind/role/
capability/condition refs; requiredness truth table; provider cardinality/
contract/order/cycle matrix; later-owned fields empty; invalid constitutional
root sets refuse.

**Dependencies:** Tasks 8 and 10. **Likely files:** `artifact_instance.rs`,
`lib.rs`, one focused descriptor test. **Size:** M.

## Task 15: Authored profile parsing and layering

**Work:** Implement the closed profile source and uniform single-parent
replace-whole layering over all eleven fields.

**Acceptance:** Exact identity/version/scope; root materializes eleven fields;
omit/inherit and present/replace including empty/null; no merge/tombstone;
exactly eleven decisions; depth 32/source 64 accepted, 33/65/deep cycle refuse.

**Dependencies:** Tasks 3-14. **Likely files:** `instance_profile.rs`, `lib.rs`,
one focused profile parser/layer test. **Size:** M.

## Task 16: Exact selection and resolved fingerprint

**Work:** Resolve one explicit leaf using only matching typed sources and roots,
then derive the full resolved profile/fingerprint.

**Acceptance:** Exact leaf/ancestry/transitive source accounting; compile-time
built-in allowlist; no ambient/invocation/range/latest behavior; all definition
fingerprints recomputed; deterministic permutations; stale closure invalidates.
Dependencies needed to recompute any selected-ancestry source-profile
fingerprint count as used even when their parent field is later shadowed.
Missing shadowed-parent dependencies refuse; only genuinely unrelated sources
are unused. Duplicate declared identities remain the earlier Task-1 error.

**Dependencies:** Task 15. **Likely files:** `instance_profile.rs`, optional
`profile_selection.rs`, `lib.rs`, one focused resolver test. **Size:** M.

## Task 17: Shipped root and repository replacement fixture

**Work:** Package the exact root profile and one explicit repository child.

**Acceptance:** Complete eleven-field root; literal six-kind/three-instance set;
unique root; no work/decision/risk instance or legacy/materialization; valid
whole-field replacements and per-field invalid cases; fixed product bytes and
behavior unchanged.

**Dependencies:** Task 16. **Likely files:** one root profile asset, one
repository profile fixture, up to two fixture source files, one end-to-end
profile test. **Size:** M.

## Task 18: Full proof, review, promotion, and closeout

**Work:** Run the complete SPEC wall, promote only earned bounded evidence,
obtain fresh exact-subject CLEAN review, commit implementation, and close
mechanically under `07`/`08`.

**Acceptance:** All positive/boundary/N+1/negative/security/package/platform/
workspace/archive/handoff/scope proof passes; literal package members/sizes/
hashes match; named gates remain open; staged GitNexus/manifest replay passes;
implementation and closeout commits are separate; HCM-1.3 does not start.

**Dependencies:** Task 17. **Likely files:** bounded crosswalk/proof-ledger and
HCM-1.2 proof/review/parent-closeout evidence only. **Size:** M.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| Product cutover leaks into HCM-1.2 | additive engine-only scope and hard product-path stop |
| Opaque later dependency | exact typed producer before non-empty ref |
| Binding pointer exists but shape is wrong | exact nine-rule table and determinate schema query |
| Source graph is unbounded or ambiguous | explicit collections, roots, budgets, fail-fast order |
| Kind/instance authority collapses | separate records, assets, owners, and validation tasks |
| Shipped set drifts | literal value/set and package-tree equality |
| Resolution metadata activates behavior | metadata-only registries; HCM-3.2 remains owner |
| Proof is overclaimed | `BoundaryLanded` ceiling; named gates remain open |

## Checkpoint for every task

- [ ] RED evidence precedes GREEN.
- [ ] Required GitNexus upstream impact precedes existing symbol edits.
- [ ] Focused tests, format, and affected engine tests pass.
- [ ] Diff remains within the task's named files and one logical increment.
- [ ] Staged change detection passes before commit.

Only the complete proof wall and fresh independent review establish final
confidence.
