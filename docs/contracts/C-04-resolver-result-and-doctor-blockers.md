---
contract_id: C-04
seam_id: SEAM-4
owner_seam: SEAM-4
version: reduced-v1
currentness: current
status: published
revalidation_triggers:
  - Any change to the resolver result top-level fields, field meanings, or ordering guarantees.
  - Any change to the refusal categories, refusal required fields, or refusal selection priority rules.
  - Any change to the blocker categories, blocker required fields, or blocker ordering rules.
  - Any change to the decision-log entry kinds, required fields, or deterministic ordering guarantees.
  - Any change to budget outcome kinds, budget policy fields, or “exact next safe action” rules.
  - Any change to packet identity fields or selection-reason semantics.
---

# C-04 Resolver Result and Doctor Blockers Contract

## Purpose

This contract defines the typed resolver result produced by the Rust compiler core and consumed by both `generate` and `doctor`.

It exists to prevent drift:

- `generate` and `doctor` MUST be views over one shared typed resolver truth.
- Downstream seams (`SEAM-5`, `SEAM-7`) MUST NOT need to infer semantics by parsing renderer strings.

## Canonical Location

- Canonical artifact: `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
- Upstream contracts consumed by this seam: `C-01`, `C-02`, `C-03`
- Downstream seams that consume this contract: `SEAM-5`, `SEAM-7`

## Owned Surface

`C-04` is authoritative about:

- The *typed* resolver result shape.
- Deterministic ordering rules for decision logs, refusals, and blockers.
- Refusal taxonomy and compact refusal structure for `generate`.
- Blocker taxonomy and “exact next safe action” semantics for `doctor`.
- Budget outcome shape and how budget refusal is represented.
- The rule that callers do not bypass resolver guardrails (no parallel blocker computation in the CLI).

`C-04` is *not* authoritative about:

- Renderer formatting, phrasing, or output ordering for human-readable proof surfaces (`SEAM-5`).
- Fixture execution demo boundaries (`SEAM-6`).
- Conformance rails, golden tests, and docs cutover (`SEAM-7`).

## Normative Rules

### Single truth: `generate` and `doctor` must not drift

- `generate` and `doctor` MUST both call the same resolver entrypoint and consume the same typed resolver result.
- `doctor` MUST NOT compute blockers independently from `generate` (no separate “doctor-only” blocker logic branches).
- Any blocker/refusal semantics MUST be represented as typed data in the resolver result, not in CLI-only branching.

### Determinism (same inputs → same outputs)

For identical canonical inputs (as defined by `C-03`) and identical request parameters:

- The resolver MUST yield identical outcomes:
  - selected packet identity (or refusal)
  - decision-log entry ordering
  - refusal selection (when refusing)
  - blocker list ordering (when blocking)
  - budget outcome classification
- Ordering MUST NOT depend on:
  - hash map / set iteration order
  - filesystem traversal order
  - wall-clock time
  - locale-specific formatting

### Input confinement

- The resolver MUST treat the `C-03` manifest + freshness truth as its canonical input truth.
- The resolver MUST NOT treat derived views as canonical inputs (examples: `README.md`, `PLAN.md`, any `dist/` output, any `artifacts/` planning docs).

### Action-oriented failure

- Every refusal and every blocker MUST contain exactly one “next safe action”.
- `next_safe_action` MUST be safe to follow and MUST NOT require guessing (no “do something” wording).
- Refusal and blocker categories MUST be typed enums; semantics MUST NOT be encoded only in freeform strings.

## Resolver Result Data Model (contract-level)

This section defines the required fields and their meaning. It is not a Rust signature; it is the contract shape.

### Top-level: `ResolverResult`

Required fields:

- `c04_version`: string (MUST equal this contract’s `version` value, e.g. `reduced-v1`)
- `manifest_ref`:
  - `c03_schema_version`: string
  - `manifest_generation_version`: integer
  - `fingerprint_sha256`: string
- `status`: enum `{ ready, refused }`
- `packet`:
  - When `status == ready`: MUST be present.
  - When `status == refused`: MUST be absent.
- `decision_log`: ordered list of decision-log entries (see below)
- `budget`: typed budget outcome (see below)
- `refusal`:
  - When `status == refused`: MUST be present.
  - When `status == ready`: MUST be absent.

Optional (reserved for downstream seams):

- `blockers[]`: ordered list of blockers (doctor uses these; if present, it is authoritative and deterministic)

### Packet selection

Packet identity fields MUST be stable and renderer-independent:

- `packet.packet_id`: stable identifier string (example: `planning.v1`)
- `packet.selection_reason`: typed reason kind (enum) plus stable detail fields (no freeform-only semantics)

### Decision log

- `decision_log.entries[]` MUST be deterministic and stable for identical inputs.
- Each entry MUST contain:
  - `kind`: enum
  - `summary`: short renderer-neutral string
  - `subject`: stable subject reference (canonical artifact, freshness issue, or budget domain)

### Budget outcome

Budget is a typed phase of resolution:

- `budget.kind`: enum `{ within_budget, summarize, exclude, refuse, not_evaluated }`
- `budget.summary`: short renderer-neutral string
- `budget.next_safe_action`: exactly one action (same rule as refusals/blockers)

### Refusal (generate)

Refusals MUST be compact and typed:

- `refusal.category`: one of the refusal categories defined below
- `refusal.summary`: short renderer-neutral summary
- `refusal.broken_subject`: subject reference (canonical artifact, `.system` root, freshness, or budget)
- `refusal.next_safe_action`: exactly one safe next action

## Refusal Categories (generate)

The refusal categories are:

- `SystemRootMissing`
- `SystemRootNotDir`
- `SystemRootSymlinkNotAllowed`
- `RequiredArtifactMissing`
- `RequiredArtifactEmpty`
- `FreshnessInvalid`
- `BudgetRefused`
- `UnsupportedRequest`

### Refusal selection priority (deterministic)

When multiple refusal causes apply, the resolver MUST pick exactly one refusal using this deterministic priority order:

1. `.system` root problems (`SystemRootMissing`, `SystemRootNotDir`, `SystemRootSymlinkNotAllowed`)
2. required artifact presence problems (`RequiredArtifactMissing`, then `RequiredArtifactEmpty`) in canonical artifact order:
   1. `CHARTER`
   2. `PROJECT_CONTEXT` (optional; MUST NOT trigger required-artifact refusal)
   3. `FEATURE_SPEC`
3. `FreshnessInvalid`
4. `BudgetRefused`
5. `UnsupportedRequest`

Tie-breakers MUST be deterministic and MUST be defined by contract order and stable fields (no freeform string comparisons).

## Blocker Categories (doctor)

Blockers use the same action-oriented structure as refusals, but are an ordered list.

Initial blocker categories (v1):

- `SystemRootMissing`
- `SystemRootNotDir`
- `SystemRootSymlinkNotAllowed`
- `RequiredArtifactMissing`
- `RequiredArtifactEmpty`
- `FreshnessInvalid`
- `BudgetRefused`
- `UnsupportedRequest`

### Blocker ordering (deterministic)

Blockers MUST be ordered deterministically using:

1. blocker category priority (same order as refusal selection priority above)
2. canonical subject order for artifact subjects (CHARTER, PROJECT_CONTEXT, FEATURE_SPEC)
3. stable, contract-defined tie-break fields (no hash order; no wall-clock)

## Compatibility and Downstream Revalidation

- Any change to refusal categories, blocker categories, decision-log entry kinds, ordering rules, budget kinds, or required fields is a contract change and requires downstream revalidation (`SEAM-5` through `SEAM-7`).
- Adding a new refusal category, budget kind, or blocker category MUST be treated as a contract bump.

## Verification Checklist

- [ ] `docs/contracts/C-04-resolver-result-and-doctor-blockers.md` exists and is cited as canonical by downstream seams.
- [ ] `generate` and `doctor` both consume the same typed resolver result and do not compute blockers in parallel.
- [ ] Resolver determinism is defined: same inputs → same outcomes including ordering.
- [ ] Refusal structure is compact and typed, includes exactly one next safe action.
- [ ] Refusal selection priority is explicit and deterministic.
- [ ] Blocker structure is typed, ordered deterministically, and includes exactly one next safe action per blocker.
- [ ] Budget outcome is typed and carries exactly one next safe action.
- [ ] Derived views are explicitly non-canonical inputs.
