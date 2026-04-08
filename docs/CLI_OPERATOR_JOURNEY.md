# CLI Operator Journey And Conformance Review (Reduced v1)

## Purpose

This document pressure-tests the reduced-v1 CLI experience from first run to trust recovery.

It is not a new design source of truth. It is the conformance review against:

- [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md)
- [`docs/CLI_COMMAND_HIERARCHY.md`](CLI_COMMAND_HIERARCHY.md)
- [`docs/CLI_TONE_RULES.md`](CLI_TONE_RULES.md)
- [`docs/CLI_OUTPUT_ANATOMY.md`](CLI_OUTPUT_ANATOMY.md)
- [`DESIGN.md`](../DESIGN.md)

Its job is to answer one question:

Does the shipped reduced-v1 product actually produce the confidence -> momentum -> controlled caution arc the interaction contract describes?

## Audit Basis

This review used:

- `system --help`
- `system setup`
- `system generate` with no `.system/`
- `system generate` with ready canonical artifacts
- `system inspect` with ready canonical artifacts
- `system doctor` with no `.system/`
- `system doctor` with ready canonical artifacts

Date of review: 2026-04-08

## Journey Map

### Step 0, orient in the product

Observed surface:

- `system --help` is concise, setup-first, and consistent with the command hierarchy.
- The top-level story matches the supported reduced-v1 boundary.

Conformance:

- D1 vocabulary: aligned
- D2 hierarchy: aligned
- D3 tone: aligned

Verdict:

- Good first impression.
- The product sounds like one CLI, not a bag of seams.

### Step 1, try the advertised front door

Observed surface:

- `system setup` prints a placeholder line and exits nonzero.
- It is honest that `setup` is placeholder-only.
- It does not hand the operator to an exact guided setup entry path.

Conformance:

- D1 vocabulary: aligned
- D2 hierarchy: partially aligned
- D3 tone: aligned
- D4 anatomy: intentionally transitional
- D5 honesty rules: aligned

Verdict:

- Honest, but incomplete.
- The front door is named correctly, but the shipped command still stops one step before usefulness.

### Step 2, try `generate` in a new repo with no canonical artifacts

Observed surface:

- `generate` refuses cleanly.
- The first three lines are excellent: outcome, object, next safe action.
- The refusal section is compact and concrete.

Conformance:

- D1 vocabulary: aligned
- D2 routing: mostly aligned
- D3 tone: aligned
- D4 anatomy: aligned

Verdict:

- Strong refusal design.
- The one weakness is the recovery handoff. The CLI says exactly what file system state is missing, but it still does not bridge the operator into the guided setup experience the product story promises.

### Step 3, try `generate` in a ready repo

Observed surface:

- `generate` is the strongest shipped surface.
- The trust header is clear.
- The packet body arrives quickly.
- Included sources, omissions, budget, and decision summary appear in stable order.

Conformance:

- D1 vocabulary: aligned
- D2 steady-state path: aligned
- D3 tone: aligned
- D4 anatomy: aligned
- D5 interaction contract: aligned

Verdict:

- This is the best expression of the product today.
- It produces the intended momentum.

### Step 4, use `inspect` on a ready repo

Observed surface:

- `inspect` is evidence-rich and stable.
- Section ordering feels like proof, not chat.
- The ready-path next action is self-referential: it tells the operator to run `inspect` for proof while they are already in `inspect`.

Conformance:

- D1 vocabulary: aligned
- D2 proof role: aligned
- D3 tone: aligned
- D4 anatomy: partially aligned because the shipped next action is semantically wrong
- D5 honesty rules: aligned because the contract now names this as a known quirk

Verdict:

- Good proof surface, flawed ready-path handoff.
- The output is useful despite the next-action bug, not because of it.

### Step 5, use `doctor` when the repo is blocked

Observed surface:

- `doctor` prints `BLOCKED`.
- It then prints raw debug-shaped blocker groups:
  - `CATEGORY`
  - `SUMMARY`
  - `SUBJECT: Policy { ... }`
  - `NEXT ACTION: CreateSystemRoot { ... }`
- It does not use the trust header.
- It does not use the human-facing `NEXT SAFE ACTION` phrasing.
- It does not name an object.

Conformance:

- D1 vocabulary: not aligned
- D2 recovery role: aligned in concept, weak in presentation
- D3 tone: not aligned
- D4 anatomy: not aligned
- D5 honesty rules: aligned because the interaction contract explicitly calls `doctor` transitional

Verdict:

- This is the clearest shipped mismatch against the interaction contract.
- The command is functionally correct and productically wrong.

### Step 6, use `doctor` when the repo is ready

Observed surface:

- `doctor` prints `READY` and exits zero.
- That is mechanically fine, but too thin relative to the intended readiness role.

Conformance:

- D2 role: partially aligned
- D3 tone: acceptable but underspecified
- D4 anatomy: still transitional

Verdict:

- Not broken.
- Still not the finished recovery/readiness surface the product contract describes.

## Conformance Summary

| Area | Status | Notes |
|------|--------|-------|
| D1 Vocabulary | Mostly aligned | Main drift is `doctor` using `NEXT ACTION` and raw debug-shaped subject rendering |
| D2 Hierarchy and front door | Partially aligned | Help and steady-state path are good; guided setup handoff is still unresolved in the shipped CLI |
| D3 Tone | Mostly aligned | `generate` and `inspect` are good; `doctor` still feels implementation-shaped |
| D4 Output anatomy | Partially aligned | `generate` is strong, `inspect` is strong with one next-action bug, `doctor` is still transitional, `setup` is still placeholder-only |
| D5 Interaction contract | Aligned | The contract is honest about the remaining gaps instead of hiding them |

## Verdict

The reduced-v1 journey is credible but uneven.

It does achieve the intended arc on the strongest path:

- `generate` in a ready repo creates momentum
- `inspect` provides real proof

It does not yet fully achieve the intended arc on the boundary paths:

- the front door still stops at a placeholder instead of a guided handoff
- `doctor` still reads like implementation output instead of a finished recovery product

So the system is directionally right and operationally useful, but the trust experience is not fully closed.

## Revision Backlog

### R1, Align `doctor` to the interaction contract

Problem:

- `doctor` is functionally correct but still ships raw debug-shaped blocker output.

Required revision:

- adopt the trust-header model
- switch from `NEXT ACTION` to `NEXT SAFE ACTION`
- render subjects and next actions in human-facing shared language
- make ready-state output more informative than a bare `READY`

Why this matters:

- this is the biggest product gap between the contract and the shipped CLI

### R2, Fix `inspect` ready-path next-action semantics

Problem:

- `inspect` tells the operator to run `inspect` for proof while already showing proof

Required revision:

- replace the self-referential ready-path next action with a non-self-referential handoff

Why this matters:

- the current line weakens trust because it looks templated instead of intentional

### R3, Make the setup placeholder hand off to a real guided entry path

Problem:

- `setup` is honest but dead-ends

Required revision:

- keep `setup` as the stable operation name
- add an exact handoff to the current guided setup experience until Rust setup exists

Why this matters:

- the front door should be incomplete only once, not ambiguous every time a new operator tries it

## Relationship To Existing Backlog

- R1 is a new revision item created by this review.
- R2 is a new revision item created by this review.
- R3 overlaps with the existing setup-ownership and entry-routing work and should be treated as the concrete interaction-design expression of that existing backlog item.
