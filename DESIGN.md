# Design System, CLI Interaction Contract

## Purpose

This file is the canonical interaction contract for the reduced-v1 CLI product.

This repo does not need a visual design system yet. It needs a product interaction system for a trust-heavy CLI that is used by:

- humans in terminals
- CLI agents consuming stdout
- CLI agents running commands to gather the information they need to complete tasks
- cloud or API workflows calling the same surfaces

If a change affects docs, help text, runtime wording, output order, or the meaning of a command surface, read this file first.

## What This Owns

This file owns the interaction-design layer for:

- command roles and hierarchy
- front-door and steady-state routing
- operator-facing vocabulary
- operator-facing tone
- output anatomy
- honesty rules about what is shipped versus what is still transitional

This file does not replace the lower-level contracts. It composes them into one product-facing source of truth.

## Product Posture

Reduced v1 is a trust product.

The operator should feel three things in sequence:

1. confidence during setup
2. momentum during packet generation
3. controlled caution during refusal and repair

The product earns trust by being exact, stable, and honest about its boundaries.

The product loses trust when it:

- overclaims what is shipped
- uses different words for the same thing across docs and runtime
- hides the next action
- makes repair feel mysterious

## Interaction Direction

The design direction is **audited utilitarian**.

This product should feel:

- exact
- narrow-terminal friendly
- calm under failure
- useful to both humans and agents
- boring in the good way

The model is not a chatty assistant and not a giant command maze.

It should feel closer to a trust-heavy developer tool with a small verb surface than to a conversational wrapper around compiler internals.

## Research Grounding

This interaction direction was pressure-tested against current trust-heavy CLI products and their official docs surfaces.

The common patterns worth keeping are:

- a small stable verb surface
- help text and runtime output that describe the same product
- machine-readable and human-readable output treated as first-class
- explicit boundaries around unsupported behavior
- recovery that feels finite rather than mysterious

The deliberate product choice for this repo is to keep the four-command surface small and make the handoffs between those commands feel intentional.

Do not expand the command set just to paper over weak transitions.

## Safe Choices And Deliberate Risks

Safe choices, these are category-baseline expectations:

- keep the four command roles stable: `setup`, `generate`, `inspect`, `doctor`
- keep the trust-header model as the primary orientation pattern
- keep narrow-terminal readability and explicit labels ahead of decorative output

Deliberate risks, these are where the product gets its own face:

- `doctor` should become a finished recovery product, not a raw diagnostic dump
- `inspect` should stay audit-dense, even if that is less immediately friendly than mainstream CLI proof views
- `setup` should be honest about being a handoff surface until Rust setup exists, instead of pretending the front door is already complete

These risks are worth taking because the product wins on trust, not on surface-level friendliness.

## Core Principles

### 1. Trust before fluency

The system should sound clear before it sounds polished.

If there is a tradeoff between clever copy and exact copy, choose exact copy.

### 2. Outcome first

The operator should know what happened before reading implementation detail.

For full reduced-v1 surfaces, the first facts are:

1. outcome
2. object
3. next safe action

### 3. Stable nouns and verbs

Do not invent alternative command stories in different surfaces.

The product language is locked in [`docs/CLI_PRODUCT_VOCABULARY.md`](docs/CLI_PRODUCT_VOCABULARY.md).

### 4. Interaction style can evolve, operation names should not

The guided experience may be LLM-assisted, partially automated, or fully CLI-owned later.

The durable command names stay:

- `setup`
- `generate`
- `inspect`
- `doctor`

### 5. Be honest about transitional surfaces

If a surface is still placeholder-only or still ships transitional output, say so plainly.

Do not document target-state behavior as if it already exists.

## Command Roles

### `setup`

Role:

- establish trusted project truth

Current reduced-v1 reality:

- still placeholder-only in the Rust CLI
- may still be delivered through an external guided setup experience

Design rule:

- keep `setup` as the stable operation name even while the experience layer evolves
- a placeholder front door is acceptable only if it hands off to one exact current guided path

Finished interaction target:

- acknowledge that Rust `setup` is still placeholder-only
- name the guided setup path that currently owns truth establishment
- end with one exact next safe action instead of a dead end

### `generate`

Role:

- default ready-path command
- produce the minimum correct planning packet from canonical artifacts

Design rule:

- the packet is the product
- success output should move quickly from trust header to packet body

Finished interaction target:

- `generate` remains the strongest shipped surface
- keep the trust header compact
- keep the packet body as the main event
- do not add decorative framing that delays useful output

### `inspect`

Role:

- proof surface
- explain why the packet looks the way it does

Design rule:

- evidence order matters more than prose
- `inspect` should feel auditable, not chatty

Current quirk:

- the shipped ready-path next action is self-referential
- treat that as implementation debt, not ideal product design

Finished interaction target:

- the proof ordering stays dense and audit-like
- the ready-path next safe action must hand the operator back to a productive surface
- `inspect` must never tell the operator to run `inspect` while they are already in `inspect`

### `doctor`

Role:

- recovery and readiness surface
- aggregate blockers and show the safest path back to `generate`

Current reduced-v1 reality:

- still ships a transitional anatomy
- not yet aligned with the full trust-header model used by `generate` and `inspect`

Design rule:

- `doctor` is the only canonical recovery verb

Finished interaction target:

- `doctor` should use the same trust-header posture as the finished product surfaces
- `doctor` should translate blocker taxonomy into human-facing recovery language
- `doctor` must not print Rust debug shapes in operator output
- `doctor` ready state should confirm readiness in a way that makes retrying `generate` feel safe and obvious

## Experience Layer Versus Command Layer

These are not the same thing.

Experience layer:

- guided setup
- guided setup refresh
- future assisted setup
- future automated setup

Command layer:

- `setup`
- `generate`
- `inspect`
- `doctor`

Design rule:

- describe the experience layer when you need to explain workflow
- keep the command layer stable when you name operations

## Vocabulary Contract

The canonical product vocabulary lives in [`docs/CLI_PRODUCT_VOCABULARY.md`](docs/CLI_PRODUCT_VOCABULARY.md).

The highest-value vocabulary rules are:

- use `canonical artifacts` as the primary noun for trusted repo-local `.system/` files
- use `canonical inputs` only when the sentence is explicitly about packet generation reading those artifacts as inputs
- use `refusal` for blocked command outcomes
- use `next safe action` for the repair handoff line
- do not rename `setup` to `bootstrap`, `init`, `hydrate`, or `onboard`

## Hierarchy And Routing Contract

The canonical front-door and routing rules live in [`docs/CLI_COMMAND_HIERARCHY.md`](docs/CLI_COMMAND_HIERARCHY.md).

The highest-value hierarchy rules are:

- the front door is a guided setup experience
- the stable operation name remains `setup`
- `generate` is the default ready-path command
- `inspect` is for proof
- `doctor` is for recovery and readiness

Repo-state routing:

- missing canonical artifacts -> guided setup / `setup`
- stale canonical artifacts -> guided setup refresh / `setup refresh`
- ready repo -> `generate`
- proof request -> `inspect`
- unclear, contradictory, or multi-blocker state -> `doctor`

## Tone Contract

The canonical tone rules live in [`docs/CLI_TONE_RULES.md`](docs/CLI_TONE_RULES.md).

The product voice is:

- strict but guided
- concise without becoming cryptic
- direct without becoming hostile
- exact for both humans and agents

Never use:

- vague failure filler
- apology filler
- celebratory filler
- blamey phrasing

Success should read like an operator summary.

Refusal should read like a procedural stop with a clear way forward.

## Output Anatomy Contract

The canonical output anatomy lives in [`docs/CLI_OUTPUT_ANATOMY.md`](docs/CLI_OUTPUT_ANATOMY.md).

Current reduced-v1 interaction shape:

- `generate` and `inspect` already have a strong trust-header model
- `doctor` is still transitional
- `setup` is still placeholder-only

Design rule:

- do not flatten those differences away in docs
- keep the contract honest about shipped versus target-state behavior

## Honesty Rules

When describing the product, always distinguish:

- supported reduced-v1 path
- transitional reduced-v1 behavior
- legacy reference material
- future intended alignment

Do not let future-state intent leak into present-tense support claims.

Examples:

- say `setup` is the front door, but also say the Rust CLI setup path is still placeholder-only
- say `doctor` is the canonical recovery surface, but also say its shipped anatomy is still transitional
- say `inspect` is the proof surface, but do not pretend the current self-referential next action is ideal

## Accessibility And Agent Readability

This product must remain easy to use in:

- narrow terminals
- keyboard-only workflows
- screen-reader order
- agentic command loops
- machine parsing of stable labels

Design rules:

- keep stable labels at the top
- do not rely on color for meaning
- do not bury next actions in prose
- prefer explicit labels over decorative formatting
- preserve predictable section order

## Finished Surface Expectations

The target product should create one coherent loop:

1. `setup` establishes or refreshes trusted project truth
2. `generate` produces the packet quickly
3. `inspect` proves why the packet looks the way it does
4. `doctor` recovers the operator from broken or ambiguous state

That loop breaks when a surface feels semantically wrong even if the code is technically correct.

Current examples of semantically wrong behavior:

- `setup` is honest but not useful enough
- `inspect` gives a self-referential handoff
- `doctor` exposes implementation-shaped output instead of a finished recovery report

When a future change touches one of these surfaces, prefer fixing the handoff quality over adding more explanatory prose.

## Change Workflow

If a proposed change affects any of the following:

- command naming
- command hierarchy
- front-door or repo-state routing
- operator-facing vocabulary
- tone
- section order
- trust-header fields
- shipped-versus-transitional claims

Do this in order:

1. update the relevant D1-D4 source document
2. update this `DESIGN.md` if the interaction contract changed
3. update docs/help/runtime/tests
4. re-run the drift guards

## Current Known Gaps

These are acknowledged interaction-design debts, not hidden contradictions:

- `setup` remains placeholder-only in the Rust CLI
- `doctor` still uses a transitional output anatomy
- `inspect` currently emits a self-referential ready-path next action

These gaps should be treated as the highest-value remaining CLI interaction work because they are the three moments where the product still feels unfinished at the exact point the operator most needs confidence.

These should be treated as future implementation and conformance work, not silently normalized.

## Source Documents

This file is downstream of:

- [`docs/CLI_PRODUCT_VOCABULARY.md`](docs/CLI_PRODUCT_VOCABULARY.md)
- [`docs/CLI_COMMAND_HIERARCHY.md`](docs/CLI_COMMAND_HIERARCHY.md)
- [`docs/CLI_TONE_RULES.md`](docs/CLI_TONE_RULES.md)
- [`docs/CLI_OUTPUT_ANATOMY.md`](docs/CLI_OUTPUT_ANATOMY.md)

If those documents and this file disagree, fix the disagreement instead of picking one ad hoc.
