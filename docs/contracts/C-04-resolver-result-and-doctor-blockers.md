---
contract_id: C-04
seam_id: SEAM-4
owner_seam: SEAM-4
version: reduced-v1
currentness: current
status: published
revalidation_triggers:
  - Any change to determinism guarantees (ordering, tie-breaks, stable IDs).
  - Any change to the rule that `generate` and `doctor` share one typed resolver result (single-truth invariant).
  - Any change to the C-03 input confinement rule (canonical `.system/` + manifest truth only; no derived docs).
  - Any change to budget policy semantics or budget outcome variants (keep/summarize/exclude/refuse) or their required fields.
  - Any change to refusal categories, refusal required fields, or refusal ordering/priority rules.
  - Any change to blocker categories, blocker required fields, or blocker ordering/priority rules.
  - Any change to decision-log entry kinds, required fields, or ordering rules.
---

# C-04 Resolver Result and Doctor Blockers Contract

## Purpose

This contract defines the reduced-v1 typed resolver result produced by `SEAM-4` and the blocker/refusal semantics that both `generate` and `doctor` must share.

It exists so downstream seams (notably `SEAM-5` renderers and `SEAM-7` conformance) can treat packet identity, decision evidence, budget outcomes, refusals, and doctor-ready blockers as one explicit, versioned truth without parsing freeform strings or relying on incidental iteration order.

## Canonical Location

- Canonical artifact: `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
- Downstream seams that consume this contract: `SEAM-5`, `SEAM-7`
- Derived consumers (read-only): `SEAM-6`

## Owned Surface

`C-04` is authoritative about the **shape and semantics** of the resolver result and its ordered evidence:

- Packet identity (stable, deterministic) and selection reasons.
- An ordered decision log that explains resolver choices.
- Typed budget policy outcomes (including exact recovery action when refusing).
- Typed refusal structure for `generate` (compact, action-oriented).
- Typed blocker structure for `doctor` (ordered, action-oriented).
- Deterministic ordering rules and tie-breaks for all ordered lists.

`C-04` is **not** authoritative about renderer wording, formatting, or presentation order (owned by `SEAM-5`), and it is not authoritative about conformance rails (owned by `SEAM-7`).

## Normative Rules

### Single-truth invariant (generate + doctor)

- The system MUST produce one typed resolver result per request.
- `generate` and `doctor` MUST be views over the same typed resolver result.
- `doctor` MUST NOT compute blockers independently from the resolver truth used by `generate`.
- If the resolver produces a refusal outcome, `doctor` MUST surface the corresponding blockers and next actions from the same result (not by re-running divergent logic).

### Input confinement (consume C-03 only)

- Resolver inputs MUST be confined to:
  - a request (what packet is being requested and with what policy parameters), and
  - canonical manifest + freshness truth produced under `C-03`.
- Resolver logic MUST treat repo-local `.system/` as the only canonical project-truth input surface, as defined by `C-03`.
- Resolver logic MUST NOT treat derived views as canonical inputs, including (non-exhaustive):
  - `README.md`, `PLAN.md`, any doc under `docs/` other than referenced contracts
  - any renderer outputs (`inspect`, markdown, JSON)
  - `dist/` and `artifacts/`
- Resolver logic MUST NOT read wall-clock time or ambient environment state as a source of selection, refusal, budget, or blocker decisions.

### Determinism (same inputs -> same outputs)

For identical inputs (request + C-03 manifest truth):

- The resolver MUST produce identical:
  - packet identity selection (or refusal)
  - decision-log content and ordering
  - budget outcome and ordering of any budget-related lists
  - refusal category and refusal ordering rules (when multiple refusal candidates exist)
  - blocker set and blocker ordering
- Ordering MUST NOT depend on:
  - hash map iteration order
  - filesystem traversal order
  - non-deterministic set ordering
  - system time

### Versioning and compatibility

The resolver result MUST carry an explicit `c04_result_version` field.

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

Any refusal or blocker MUST reference a **subject** using contract-defined identifiers, not freeform text.

A subject reference MUST be one of:

- `CanonicalArtifact`:
  - `kind`: `{ charter, project_context, feature_spec }` (aligned with `C-03`)
  - `canonical_repo_relative_path`: the exact path from `C-03` (for example `.system/charter/CHARTER.md`)
- `InheritedDependency`:
  - `dependency_id`: stable string identifier
  - `version`: optional string
- `Policy`:
  - `policy_id`: stable string identifier (for example `budget`)

### Resolver result (top-level)

The resolver result MUST include:

- `c04_result_version`: string (see Versioning)
- `c03_schema_version`: string (copied from C-03 manifest truth)
- `c03_manifest_generation_version`: integer (copied from C-03 manifest truth)
- `c03_fingerprint_sha256`: string (copied from C-03 manifest truth)
- `packet_identity`:
  - `packet_id`: stable string identifier for the selected packet
  - `selection_reasons[]`: ordered list of selection-reason records (see Ordering)
- `decision_log`: a decision-log record (see below)
- `budget`: a budget outcome record (see below)
- exactly one of:
  - `refusal`: a refusal record (generate-facing failure path), or
  - `refusal` absent and the result is not refused
- `blockers[]`: ordered list of blockers (doctor-facing; may be empty)

The resolver result MAY include additional fields as long as:

- they do not broaden inputs beyond `C-03` + request
- they do not introduce non-deterministic ordering
- they preserve backward compatibility expectations for downstream consumers

### Decision log

The decision log MUST be an ordered list of typed entries.

Each entry MUST include:

- `kind`: a value from the Decision Entry Kind enum
- `summary`: short human-readable string
- `subject`: optional subject reference (when the entry applies to a specific artifact/dependency/policy)

The decision log MUST be stable and deterministic in ordering.

#### Decision entry kinds (initial)

The decision log `kind` MUST be one of (initial set; additions require revalidation):

- `Ingest`
- `Freshness`
- `Selection`
- `Budget`
- `Refusal`
- `Blocker`

### Budget outcome

Budget must be represented as typed data.

The budget outcome MUST include:

- `policy_id`: stable identifier (for example `budget`)
- `outcome`: one of `{ Keep, Summarize, Exclude, Refuse }`
- `reason`: short human-readable string
- `next_safe_action`: present when `outcome` is `Refuse`, absent otherwise

If budget produces any ordered lists (for example, ordered omissions), those lists MUST have contract-defined ordering and explicit tie-breaks.

### Refusal (generate)

The refusal record MUST be compact and action-oriented.

The refusal MUST include:

- `category`: one of the Refusal Category enum values
- `summary`: short human-readable string
- `broken_subject`: subject reference
- `next_safe_action`: exactly one explicit recovery action

### Blockers (doctor)

The blocker record MUST be compact and action-oriented.

Each blocker MUST include:

- `category`: one of the Blocker Category enum values
- `subject`: subject reference
- `summary`: short human-readable string
- `next_safe_action`: exactly one explicit recovery action
- Renderer-facing wording for missing-root, invalid-root, and required-artifact blockers SHOULD route the operator toward the setup family (`system setup`, `system setup init`, `system setup refresh`) while preserving this contract's single-truth blocker semantics.

## Categories and Ordering Rules

This section defines the initial category enums and their ordering priorities.

### Refusal categories (generate) (initial)

Allowed refusal categories include:

- `NonCanonicalInputAttempt`
- `SystemRootMissing`
- `SystemRootNotDir`
- `SystemRootSymlinkNotAllowed`
- `RequiredArtifactMissing`
- `RequiredArtifactEmpty`
- `RequiredArtifactStarterTemplate`
- `ArtifactReadError`
- `FreshnessInvalid`
- `BudgetRefused`
- `UnsupportedRequest`

### Blocker categories (doctor) (initial)

Allowed blocker categories include:

- `SystemRootMissing`
- `SystemRootNotDir`
- `SystemRootSymlinkNotAllowed`
- `RequiredArtifactMissing`
- `RequiredArtifactEmpty`
- `RequiredArtifactStarterTemplate`
- `ArtifactReadError`
- `FreshnessInvalid`
- `BudgetRefused`
- `UnsupportedRequest`

### Category priority tables

When multiple refusal candidates exist, refusal ordering MUST be deterministic and MUST use this priority order (lowest number = highest priority):

| Priority | Refusal category |
|---:|---|
| 0 | `NonCanonicalInputAttempt` |
| 1 | `SystemRootMissing` |
| 2 | `SystemRootSymlinkNotAllowed` |
| 3 | `SystemRootNotDir` |
| 4 | `RequiredArtifactMissing` |
| 5 | `RequiredArtifactEmpty` |
| 6 | `RequiredArtifactStarterTemplate` |
| 7 | `ArtifactReadError` |
| 8 | `FreshnessInvalid` |
| 9 | `BudgetRefused` |
| 10 | `UnsupportedRequest` |

When multiple blockers exist, blocker ordering MUST be deterministic and MUST use this priority order (lowest number = highest priority):

| Priority | Blocker category |
|---:|---|
| 0 | `SystemRootMissing` |
| 1 | `SystemRootSymlinkNotAllowed` |
| 2 | `SystemRootNotDir` |
| 3 | `RequiredArtifactMissing` |
| 4 | `RequiredArtifactEmpty` |
| 5 | `RequiredArtifactStarterTemplate` |
| 6 | `ArtifactReadError` |
| 7 | `FreshnessInvalid` |
| 8 | `BudgetRefused` |
| 9 | `UnsupportedRequest` |

### Tie-break rules (stable ordering within a category)

Within the same category, ordering MUST be deterministic.

Tie-break MUST be applied in this order:

1. Subject kind order:
   - For `CanonicalArtifact`: `{ charter, project_context, feature_spec }` (aligned with `C-03`)
   - For `InheritedDependency`: lexical by `dependency_id`, then by `version`
   - For `Policy`: lexical by `policy_id`
2. Subject path (for canonical artifacts): lexical by `canonical_repo_relative_path`
3. Summary: lexical by `summary`

## Compatibility and Downstream Revalidation

- Downstream seams MUST treat category enums and required fields as authoritative contract surface.
- Downstream seams MUST NOT parse semantics out of freeform strings to recover meaning that should be represented by categories or structured fields.
- Adding a new refusal category, blocker category, budget outcome, or decision entry kind MUST trigger downstream revalidation.
- Changing any priority table or tie-break rule MUST trigger downstream revalidation.

## Verification Checklist

- [ ] A resolver result carries `c04_result_version` and C-03 provenance (`schema_version`, `manifest_generation_version`, `fingerprint_sha256`).
- [ ] `generate` and `doctor` are views over one typed resolver result; `doctor` does not compute blockers separately.
- [ ] Inputs are confined to the request plus C-03 manifest/freshness truth; derived docs are never canonical inputs.
- [ ] All ordered lists (decision log, selection reasons, refusals, blockers) have explicit deterministic ordering and tie-break rules.
- [ ] Budget outcomes are typed (`Keep`, `Summarize`, `Exclude`, `Refuse`) and budget refusal includes exactly one next safe action.
- [ ] Refusals are compact and always include: category, summary, broken subject, and exactly one next safe action.
- [ ] Blockers are compact and always include: category, subject, summary, and exactly one next safe action.
- [ ] Refusal categories and blocker categories match the initial enums in this contract and are ordered by the priority tables.
- [ ] Any change to budget/refusal/blocker semantics is captured as a contract revision and triggers downstream revalidation.
- [ ] `artifacts/planning/reduced-v1-seam-pack/threading.md` describes `C-04` in the same nouns as this contract.
