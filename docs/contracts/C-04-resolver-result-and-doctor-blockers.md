---
contract_id: C-04
seam_id: SEAM-4
owner_seam: SEAM-4
version: reduced-v1-m8
currentness: current
status: published
revalidation_triggers:
  - Any change to determinism guarantees (ordering, tie-breaks, stable IDs).
  - Any change to the baseline readiness state set or its semantics.
  - Any change to the set of baseline artifacts inspected by `doctor`.
  - Any change to checklist line requirements (artifact label, canonical path, status, exact author command).
  - Any change to blocker categories, blocker required fields, or blocker ordering/priority rules.
---

# C-04 Doctor Baseline-Readiness and Blockers Contract

## Purpose

This contract defines the reduced-v1 `M8` baseline-readiness model produced for `doctor`, plus the blocker and checklist semantics that downstream docs and renderers rely on.

It exists so downstream seams can treat baseline states, checklist ordering, blocker structure, and next-safe-action requirements as explicit contract truth without parsing freeform strings.

## Canonical Location

- Canonical artifact: `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
- Downstream seams that consume this contract: `SEAM-5`, `SEAM-7`
- Derived consumers (read-only): `SEAM-6`

## Owned Surface

`C-04` is authoritative about the **shape and semantics** of the doctor baseline-readiness result:

- baseline readiness states
- the baseline artifact set inspected by `doctor`
- ordered checklist items for baseline artifacts
- typed blocker structure for `doctor` (ordered, action-oriented)
- deterministic ordering rules and tie-breaks for all ordered lists

`C-04` is **not** authoritative about packet renderer wording, packet proof ordering, or conformance rails.

## Normative Rules

### Doctor scope

- `doctor` MUST compute baseline readiness from canonical `.system/` truth rather than from packet readiness.
- `doctor` MUST inspect exactly these baseline artifacts:
  - `.system/charter/CHARTER.md`
  - `.system/project_context/PROJECT_CONTEXT.md`
  - `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
- `doctor` MUST NOT treat `.system/feature_spec/FEATURE_SPEC.md` as part of baseline readiness.

### Input confinement (consume C-03 only)

- Doctor logic MUST treat repo-local `.system/` as the only canonical project-truth input surface, as defined by `C-03`.
- Doctor logic MUST NOT treat derived views as canonical inputs, including (non-exhaustive):
  - `README.md`, `PLAN.md`, any doc under `docs/` other than referenced contracts
  - any renderer outputs (`inspect`, markdown, JSON)
  - `dist/` and `artifacts/`
- Doctor logic MUST NOT read wall-clock time or ambient environment state as a source of readiness, checklist, or blocker decisions.

### Determinism (same inputs -> same outputs)

For identical canonical baseline inputs:

- `doctor` MUST produce identical:
  - baseline state
  - checklist content and ordering
  - blocker set and blocker ordering
- Ordering MUST NOT depend on:
  - hash map iteration order
  - filesystem traversal order
  - non-deterministic set ordering
  - system time

### Versioning and compatibility

The doctor result MUST carry an explicit `c04_result_version` field.

- The version MUST be a stable identifier for the meaning of the result fields and ordering rules.
- Any change to:
  - the meaning of a field,
  - required field presence,
  - enum variants (adding, removing, renaming),
  - or ordering/priority rules
  MUST be treated as a contract revision requiring downstream revalidation.

## Data Model (Contract Shape)

This contract specifies field **semantics**. Concrete Rust type names are implementation detail, but the result MUST faithfully represent the following records and fields.

### Subject references (shared)

Any checklist item or blocker MUST reference a baseline artifact subject using contract-defined identifiers, not freeform text.

A subject reference MUST be:

- `CanonicalArtifact`:
  - `kind`: `{ charter, project_context, environment_inventory }`
  - `canonical_repo_relative_path`: the exact path from `C-03`

### Doctor result (top-level)

The doctor result MUST include:

- `c04_result_version`: string
- `c03_schema_version`: string
- `c03_manifest_generation_version`: integer
- `baseline_state`: one of:
  - `SCAFFOLDED`
  - `PARTIAL_BASELINE`
  - `INVALID_BASELINE`
  - `BASELINE_COMPLETE`
- `checklist[]`: ordered list of checklist items
- `blockers[]`: ordered list of blockers (may be empty)

### Checklist items

Each checklist item MUST include:

- `artifact_label`
- `subject`
- `status`
- `author_command`

Checklist item statuses MUST distinguish at least:

- `missing`
- `starter_owned`
- `invalid`
- `complete`

### Blockers (doctor)

The blocker record MUST be compact and action-oriented.

Each blocker MUST include:

- `category`: one of the Blocker Category enum values
- `subject`: subject reference
- `summary`: short human-readable string
- `next_safe_action`: exactly one explicit recovery action
- Renderer-facing wording for missing-root and invalid-root blockers SHOULD route the operator toward the setup family.
- Renderer-facing wording for artifact-specific blockers SHOULD route the operator toward the exact `system author ...` command for that artifact.

## Categories and Ordering Rules

This section defines the initial category enums and their ordering priorities.

### Blocker categories (doctor) (initial)

Allowed blocker categories include:

- `SystemRootMissing`
- `SystemRootNotDir`
- `SystemRootSymlinkNotAllowed`
- `BaselineArtifactMissing`
- `BaselineArtifactEmpty`
- `BaselineArtifactStarterTemplate`
- `BaselineArtifactInvalid`
- `ArtifactReadError`

### Category priority tables

When multiple blockers exist, blocker ordering MUST be deterministic and MUST use this priority order (lowest number = highest priority):

| Priority | Blocker category |
|---:|---|
| 0 | `SystemRootMissing` |
| 1 | `SystemRootSymlinkNotAllowed` |
| 2 | `SystemRootNotDir` |
| 3 | `BaselineArtifactMissing` |
| 4 | `BaselineArtifactEmpty` |
| 5 | `BaselineArtifactStarterTemplate` |
| 6 | `BaselineArtifactInvalid` |
| 7 | `ArtifactReadError` |

### Tie-break rules (stable ordering within a category)

Within the same category, ordering MUST be deterministic.

Tie-break MUST be applied in this order:

1. Subject kind order:
   - For `CanonicalArtifact`: `{ charter, project_context, environment_inventory }` (aligned with `C-03`)
2. Subject path (for canonical artifacts): lexical by `canonical_repo_relative_path`
3. Summary: lexical by `summary`

## Compatibility and Downstream Revalidation

- Downstream seams MUST treat baseline states, checklist-item fields, and blocker categories as authoritative contract surface.
- Downstream seams MUST NOT parse semantics out of freeform strings to recover meaning that should be represented by categories or structured fields.
- Adding a new blocker category, checklist field, or baseline state MUST trigger downstream revalidation.
- Changing any priority table or tie-break rule MUST trigger downstream revalidation.

## Verification Checklist

- [ ] A doctor result carries `c04_result_version` and C-03 provenance.
- [ ] `doctor` baseline states are exactly `SCAFFOLDED`, `PARTIAL_BASELINE`, `INVALID_BASELINE`, and `BASELINE_COMPLETE`.
- [ ] Inputs are confined to C-03 baseline artifact truth; derived docs are never canonical inputs.
- [ ] Checklist items always include artifact label, canonical path, status, and exact author command.
- [ ] Blockers are compact and always include: category, subject, summary, and exactly one next safe action.
- [ ] Blocker categories match the initial enum in this contract and are ordered by the priority table.
- [ ] Any change to baseline-state, checklist, or blocker semantics is captured as a contract revision and triggers downstream revalidation.
- [ ] `artifacts/planning/reduced-v1-seam-pack/threading.md` describes `C-04` in the same nouns as this contract.
