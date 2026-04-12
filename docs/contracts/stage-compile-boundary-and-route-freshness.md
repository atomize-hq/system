---
contract_id: C-10
seam_id: SEAM-3
owner_seam: SEAM-3
version: m1-v1
currentness: current
status: drafted
revalidation_triggers:
  - Any change to compile-target selection semantics, canonical id resolution, or the rule that compile consumes published operator-surface truth rather than inventing compile-only targeting behavior.
  - Any change to the source-of-truth split between pipeline YAML orchestration data and stage front-matter compile-facing metadata.
  - Any change to route-basis freshness inputs, inactive-stage refusal posture, or the operator recovery guidance that tells the caller to re-run `pipeline resolve`.
  - Any change to the activation-equivalence rule between pipeline YAML and stage front matter during the transition.
  - Any change to the stage-payload handoff boundary expected by later compile work.
---

# C-10 Stage Compile Boundary and Route Freshness Contract

## Purpose

This contract defines the compile boundary between published `pipeline` route truth and the later compile surface that will consume it.

`C-10` exists so downstream compile work can treat one canonical contract as the source of truth for:

- which pipeline and stage identifiers are valid compile targets
- which input family owns orchestration versus compile-facing stage metadata
- when compile must refuse stale or inactive basis instead of guessing
- how activation duplication between pipeline YAML and stage front matter is handled during the transition
- what kind of stage-payload handoff later compile work may expect, without pinning implementation details too early

`C-10` is intentionally downstream-facing. It freezes the handoff rules without widening the shipped M1 `pipeline` help surface.

## Canonical location

- Canonical artifact: `docs/contracts/stage-compile-boundary-and-route-freshness.md`
- Direct consumer seam: `SEAM-4`

## Consumed contracts

`C-10` consumes:

- [`C-08`](pipeline-route-and-state-core.md) compiler-owned route/state truth
- [`C-09`](pipeline-operator-surface-and-id-resolution.md) reviewed `pipeline` operator surface and id-resolution rules

## Owned surface

`C-10` is authoritative about:

- compile-target selection over the published `pipeline` operator surface
- the boundary between pipeline YAML orchestration data and stage front-matter compile-facing metadata
- route-basis freshness refusal semantics
- inactive-stage refusal semantics
- activation-equivalence or activation-drift refusal semantics during the transition
- the stage-payload handoff boundary expected by later compile work

`C-10` is not authoritative about:

- compile implementation details
- payload materialization or file writes
- CLI parsing internals
- shipped help/docs cutover work
- future M2 payload field names that are not yet evidenced in repo surfaces

## Normative rules

### Compile-target selection

- Compile-target selection MUST start from one resolved pipeline selection produced through the published `pipeline` operator surface.
- The only accepted target inputs are the canonical pipeline id and a canonical stage id within that pipeline context.
- Shorthand ids MAY be accepted only when they resolve through the same canonical-id and shorthand rules published in `C-09`.
- Raw file paths, hidden aliases, and compile-only lookup forms MUST be refused.
- Compile MUST treat route truth as an upstream input and MUST NOT rediscover targets from filesystem traversal or stage-front-matter parsing alone.
- The selected stage id MUST exist in the selected pipeline's declared stage list.

### Source-of-truth split

| Surface | Owns | Concrete fields |
| --- | --- | --- |
| Pipeline YAML | orchestration and route target declarations | `kind`, `id`, `version`, `title`, `description`, `defaults.runner`, `defaults.profile`, `defaults.enable_complexity`, `stages[].id`, `stages[].file`, `stages[].sets`, `stages[].activation` |
| Stage front matter | stage identity and stage-local compile metadata | `kind`, `id`, `version`, `title`, `description`, `work_level`, plus any other stage-local front matter already present in `core/stages/*.md` |

- Compile MUST read pipeline YAML for orchestration order and stage-to-file binding.
- Compile MUST read stage front matter for the selected stage's canonical identity and stage-local metadata.
- Compile MUST NOT infer a stage from front matter alone or from file path alone.
- If the selected stage metadata and the pipeline-declared stage entry do not line up, later compile work MUST treat that as contract drift rather than inventing a second source of truth.

### Freshness and refusal posture

- Compile MUST consume already-resolved route truth as the route basis for downstream compile work.
- Compile MUST refuse stale route basis instead of silently re-running `pipeline resolve`.
- Compile MUST refuse inactive stages explicitly when the selected stage is not active in the resolved route truth.
- Compile MUST surface a freshness refusal when the selected route basis is missing, stale, malformed, or otherwise outside the contract required for compile.
- Compile MUST give the operator one clear recovery direction when freshness is the problem: re-run `pipeline resolve` before retrying compile.
- Compile MUST NOT downgrade stale or inactive basis into best-effort behavior.

### Activation equivalence

- When activation appears in both pipeline YAML and stage front matter during the transition, the two representations MUST remain semantically equivalent.
- If activation values drift, compile MUST refuse rather than guessing which representation is authoritative.
- Compile MUST NOT silently normalize activation drift away.
- This contract does not define a new activation grammar; it only defines the transition rule that keeps the duplicated activation surfaces aligned.

### Stage-payload handoff

- Later compile work MUST be able to consume one explicit stage-payload handoff boundary from this contract.
- The handoff MUST describe the expected boundary between route truth, stage metadata, and compile output without over-specifying future payload field names.
- The handoff MUST remain compatible with future M2 implementation detail changes as long as the source-of-truth split and refusal posture stay intact.

### M1 help/docs posture

- `pipeline compile` MUST remain out of the shipped M1 help/docs surface.
- This contract MUST NOT be read as a release claim that compile is already supported in M1.
- This contract MAY define the compile boundary now, but it MUST keep publication of the executable surface deferred to later work.

## Compatibility and downstream revalidation

- Any change to compile-target selection or canonical id expectations requires downstream revalidation.
- Any change to route-basis freshness inputs, inactive-stage refusal wording, or recovery guidance requires downstream revalidation.
- Any change to the source-of-truth split between pipeline YAML and stage front matter requires downstream revalidation.
- Any change to activation-equivalence wording or refusal posture requires downstream revalidation.
- Any change to the stage-payload handoff boundary requires downstream revalidation.

## Verification checklist

- [ ] The canonical contract exists at `docs/contracts/stage-compile-boundary-and-route-freshness.md`.
- [ ] The contract names `pipeline` target selection, source-of-truth split, freshness refusal, inactive-stage refusal, activation-equivalence posture, and stage-payload handoff as separate normative concerns.
- [ ] The contract makes pipeline-YAML-owned fields and stage-front-matter-owned fields explicit enough for downstream implementation.
- [ ] The contract ties compile-target selection to the published canonical-id and shorthand rules without introducing raw-path targeting.
- [ ] The contract keeps `pipeline compile` out of the shipped M1 help/docs posture.
- [ ] The contract references only the published upstream truth at `docs/contracts/pipeline-route-and-state-core.md` and `docs/contracts/pipeline-operator-surface-and-id-resolution.md`.
- [ ] The contract does not over-specify future payload field names or implementation details.
