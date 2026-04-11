---
contract_id: C-03
seam_id: SEAM-3
owner_seam: SEAM-3
version: reduced-v1
currentness: current
status: published
revalidation_triggers:
  - Any change to the canonical `.system/` artifact set, paths, or required/optional semantics.
  - Any change to the definition of `missing` vs `present_empty` vs `present_non_empty`.
  - Any change to artifact identity rules (including the content hashing algorithm or inputs).
  - Any change to freshness fields, field meanings, or deterministic ordering rules.
  - Any change to schema vs manifest-generation versioning policy.
  - Any change to inherited posture dependency identity or how it influences freshness truth.
  - Any change to override-with-rationale rules or any new override capability that could expand inputs.
  - Any change to the rule that repo-local `.system/` is the only canonical project-truth input surface.
---

# C-03 Canonical Artifact Manifest Contract

## Purpose

This contract defines the reduced-v1 canonical artifact inventory and the manifest/freshness truth model produced from repo-local `.system/` artifacts.

It exists so downstream seams (notably `SEAM-4` and `SEAM-7`) can treat canonical inputs and freshness semantics as one explicit, versioned truth without guessing or reading non-canonical derived sources.

## Canonical Location

- Canonical artifact: `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
- Downstream seams that consume this contract: `SEAM-4`, `SEAM-7`
- Derived consumers (read-only): `SEAM-5`, `SEAM-6`

## Owned Surface

### Canonical inputs (repo-local)

The canonical project-truth inputs for reduced-v1 are repo-local files under `.system/` only.

The canonical input set MUST be exactly:

- Required:
  - `.system/charter/CHARTER.md`
  - `.system/feature_spec/FEATURE_SPEC.md`
- Optional:
  - `.system/project_context/PROJECT_CONTEXT.md`

No other file path is a direct canonical input under this contract.

### Runtime zones under `.system/`

This contract allows explicitly non-canonical runtime zones under `.system/`, provided they are documented as runtime-only and never treated as canonical inputs.

Reduced-v1 runtime-zone rule:

- `.system/state/**` is a runtime zone, not a canonical artifact zone.
- Runtime-zone contents MAY support orchestration or proof, but MUST NOT be treated as project-truth inputs under this contract.
- Future additions under `.system/` MUST declare whether they are canonical artifact zones or runtime zones.

### Canonical-truth rule (no derived inputs)

- Repo-local `.system/` inputs are authoritative.
- Runtime-zone contents under `.system/` are not authoritative unless a later contract revision explicitly moves them into the canonical input set.
- Derived views MUST NOT be treated as runtime inputs, including (non-exhaustive):
  - `README.md`, `PLAN.md`, `docs/README.md`
  - any renderer output (`inspect`, markdown, JSON) or generated artifacts under `dist/` or `artifacts/`
  - runtime-zone files such as `.system/state/**`
  - any prompt packs, stage outputs, or other derived documentation

Downstream seams MAY reference derived views for operator guidance, but MUST NOT read them as canonical packet inputs.

## Normative Rules

### Presence semantics: `missing` vs `present_empty` vs `present_non_empty`

The manifest model MUST represent presence for each canonical input as exactly one of:

- `missing`: the canonical path does not exist as a file.
- `present_empty`: the canonical path exists as a file with **exactly zero bytes** of content.
- `present_non_empty`: the canonical path exists as a file with one or more bytes of content.

Important constraints:

- `present_empty` MUST be defined by **raw byte length == 0**; it MUST NOT be computed by trimming whitespace or normalizing text.
- Missing optional inputs MUST remain distinguishable from empty optional inputs.
- Missing required inputs MUST remain distinguishable from empty required inputs.

This contract intentionally does not define refusal copy or recovery UX; it defines the truth states that downstream seams consume.

### Canonical artifact identity (stable, deterministic)

For each canonical input, the manifest MUST expose an artifact identity record with:

- `kind`: one of `{ charter, project_context, feature_spec }`
- `canonical_repo_relative_path`: the exact path listed in this contract
- `requirement`: `{ required, optional }`
- `presence`: `{ missing, present_empty, present_non_empty }`
- `content_sha256`:
  - MUST be present when `presence` is `present_empty` or `present_non_empty`
  - MUST be computed as SHA-256 over the **raw file bytes**
  - MUST be absent when `presence` is `missing`

Ordering:

- Artifact identity records MUST be emitted in a deterministic, contract-defined order independent of filesystem iteration.
- The required ordering for reduced-v1 MUST be:
  1. `charter` (`.system/charter/CHARTER.md`)
  2. `project_context` (`.system/project_context/PROJECT_CONTEXT.md`)
  3. `feature_spec` (`.system/feature_spec/FEATURE_SPEC.md`)

### Freshness truth (contract-level shape)

Freshness truth MUST be a deterministic function of:

- the canonical artifact identity records defined above
- the declared inherited posture dependencies defined below
- any declared override-with-rationale records defined below

The manifest MUST expose freshness truth fields that are stable enough for downstream seams to consume as contract input. At minimum, the model MUST carry:

- `schema_version`:
  - MUST identify the C-03 schema contract version (this document’s `version`, and any successor revisions).
  - MUST change only when the schema meaning changes (fields, semantics, canonical inputs).
- `manifest_generation_version`:
  - MUST be a separate version signal from `schema_version`.
  - MUST change when generation behavior/output meaning changes within the same schema version.
  - MUST be monotonic within a given implementation lineage (never decreases).
- `artifacts[]`: the ordered canonical artifact identity list (per this contract)
- `inherited_dependencies[]`: the ordered inherited posture dependency list (per this contract)
- `overrides[]`: the ordered override-with-rationale record list (per this contract)
- `issues[]`: a deterministically ordered list of issue categories discovered while producing the manifest (see Issue Categories).

This contract does not require persistence. The manifest MAY be request-scoped and in-memory by default.

### Inherited posture dependencies (affect freshness, not canonical inputs)

The system MAY accept a declared set of inherited posture dependencies that influence freshness deterministically without becoming additional canonical file inputs.

Rules:

- Inherited dependencies MUST be explicitly declared (e.g., by setup posture or repo configuration); they MUST NOT be inferred from ambient environment state.
- Declared dependencies MUST NOT expand the canonical input surface beyond the `.system/` paths listed above.
- Each dependency MUST have a stable identity, at minimum:
  - `dependency_id` (stable string identifier)
  - `version` (stable string or numeric)
  - optional `content_sha256` (SHA-256 over raw bytes) when a dependency is content-addressable
- The dependency list MUST be emitted in deterministic order (e.g., lexical by `dependency_id`, then `version`).

Reduced-v1 known inherited posture dependency IDs include (non-exhaustive, but stable once adopted by downstream seams):

- `FOUNDATION_STRATEGY`
- `TECH_ARCH_BRIEF`
- `TEST_STRATEGY_BRIEF`
- `QUALITY_GATES_SPEC`
- `ENVIRONMENT_INVENTORY`

### Override-with-rationale (recorded, inspectable, non-expanding)

Override-with-rationale exists to prevent silent drift and to keep any future override behavior explicit and inspectable.

Rules:

- Any override attempt MUST produce an explicit override record with:
  - `override_id` (stable identifier)
  - `target` (what is being overridden)
  - `rationale` (human-readable, non-empty)
  - `scope` (what the override affects)
- Overrides MUST NOT introduce new canonical input paths under `.system/`.
- Overrides MUST NOT hide or falsify freshness truth fields or artifact identity.
- Overrides MUST NOT change the definition of presence semantics (`missing`, `present_empty`, `present_non_empty`).

Forbidden patterns (MUST be rejected deterministically and/or surfaced as issues):

- Silent overrides (no explicit record or empty rationale).
- Overrides that add a new input surface or canonical path.
- Overrides that suppress, reorder, or rewrite artifact identity or freshness truth.

## Issue Categories (names only)

The manifest MAY surface issue categories as data. This contract names issue categories but does not define refusal policy, severity, or recovery UX.

Allowed issue categories include (non-exhaustive):

- `NonCanonicalInputAttempt`
- `RequiredArtifactMissing`
- `ArtifactReadError`
- `InvalidOverride`
- `InheritedDependencyInvalid`

## Compatibility and Downstream Revalidation

- Any addition of a new direct input, new canonical path, or new refusal source that depends on additional runtime inputs MUST be treated as a contract change and requires downstream revalidation.
- Any change to freshness fields, ordering, hashing rules, or versioning policy MUST be treated as a contract change and requires downstream revalidation.
- Any change to override-with-rationale capabilities MUST be treated as a contract change and requires downstream revalidation.

## Verification Checklist

- [ ] This contract is the only source of truth for canonical `.system/` inputs in reduced-v1.
- [ ] The canonical input set and repo-relative paths are explicit and exhaustive.
- [ ] `present_empty` is defined as exactly zero bytes (no whitespace trimming).
- [ ] Missing vs empty semantics are unambiguous for both required and optional inputs.
- [ ] Artifact identity includes `sha256` of raw bytes and is absent only when missing.
- [ ] Manifest ordering is deterministic and contract-defined (not filesystem order).
- [ ] Schema version is distinct from manifest generation version, and both are defined.
- [ ] Inherited posture dependencies influence freshness without expanding canonical inputs and have stable identities.
- [ ] Override-with-rationale rules require explicit rationale and forbid input-surface expansion or freshness falsification.
- [ ] `README.md`, `PLAN.md`, and `docs/README.md` link to this contract alongside `C-01` and `C-02`.
- [ ] `artifacts/planning/reduced-v1-seam-pack/threading.md` points `C-03` at this canonical artifact path.
