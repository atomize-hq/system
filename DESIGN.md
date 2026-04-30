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

The deliberate product choice for this repo is to keep the command surface small and make the handoffs between those commands feel intentional.

Do not expand the command set just to paper over weak transitions.

## Safe Choices And Deliberate Risks

Safe choices, these are category-baseline expectations:

- keep the top-level trust roles stable: `setup`, `author`, `pipeline`, `generate`, `inspect`, `doctor`
- keep the trust-header model as the primary orientation pattern
- keep narrow-terminal readability and explicit labels ahead of decorative output

Deliberate risks, these are where the product gets its own face:

- `doctor` should become a finished recovery product, not a raw diagnostic dump
- `inspect` should stay audit-dense, even if that is less immediately friendly than mainstream CLI proof views
- `setup` should stay one durable family name even when it routes between `setup init` and `setup refresh`

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
- `author`
- `pipeline`
- `generate`
- `inspect`
- `doctor`

### 5. Be honest about transitional surfaces and conformance gaps

If a surface still ships transitional output or a local runtime surface lags the contract, say so plainly.

Do not document target-state behavior as if it already exists.

## Command Roles

### `setup`

Role:

- establish trusted project truth

Current reduced-v1 reality:

- the public setup family is `system setup`, `system setup init`, and `system setup refresh`
- bare `system setup` routes to `setup init` when canonical `.system/` truth is absent or invalid; otherwise it routes to `setup refresh`
- `setup refresh` preserves canonical files by default
- `setup refresh --rewrite` rewrites only the setup-owned starter files
- `setup refresh --reset-state` resets only `.system/state/**`
- setup hands off to `system doctor`, which renders baseline readiness and the next exact authoring action
- the canonical setup-owned starter files are exactly:
  - `.system/charter/CHARTER.md`
  - `.system/project_context/PROJECT_CONTEXT.md`
  - `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
- `FEATURE_SPEC.md` stays off setup bootstrap and baseline doctor readiness; it remains on the packet path
- this repository does not ship completed canonical `.system/` truth at repo root; a fresh clone starts with `system setup`

Design rule:

- keep `setup` as the stable operation name even while the routed subcommand changes
- keep `init` scoped to the concrete first-run subcommand name

Finished interaction target:

- expose the routed subcommand when bare `system setup` selects one
- keep setup-owned file semantics explicit: preserve by default, rewrite only starter files, reset only `.system/state/**`
- end with `system doctor`, which orders the baseline checklist and names the exact next authoring command when action is still required

### `author`

Role:

- canonical content-authoring surface for setup-created starter truth
- the baseline authoring family is `system author charter`, `system author project-context`, and `system author environment-inventory`

Shipped M9.5 reality:

- the first shipped slice is `system author charter`
- `system author charter` is a TTY-only guided interview
- `system author charter --from-inputs <path|->` is the deterministic non-interactive path
- `system author charter --validate --from-inputs <path|->` is the mutation-free preflight path
- `--validate` is legal only with `--from-inputs <path|->`
- the deterministic `--from-inputs` write path is compiler-owned and does not shell out to `codex exec`
- the guided `system author charter` path remains the Codex-backed interview surface
- the public authoring surface writes canonical truth to `.system/charter/CHARTER.md`
- if real charter truth already exists, `author` refuses rather than silently broadening scope

Design rule:

- `author` owns converting scaffolded canonical truth into usable canonical truth
- `author` must not become a generic chat surface or hide legacy stage ids in the public UX

Finished interaction target:

- `author` is the only normal path from scaffolded setup to a usable charter
- `author` reuses compiler-owned charter assets internally while keeping the public story simple
- `author` keeps one authority boundary: canonical `.system/*` truth, no public derived mirrors

### Installed Codex Skill

Role:

- installed Codex discovery surface for charter intake
- thin runtime wrapper over `system`, not a second source of truth

Shipped M9.5 reality:

- one discoverable skill name: `system-charter-intake`
- repo-local `.agents/skills/*` trees are thin generated projections only
- the installed home is `~/system/`
- installed thin projections live under `~/system/.agents/skills/*`
- `~/.codex/skills/system*` is discovery glue only and points into `~/system/.agents/skills/*`
- `tools/codex/install.sh` owns the installed `~/system/` home and refreshes the Codex discovery glue
- `tools/codex/dev-setup.sh` is the only symlink-based dev flow
- the installed runtime may machine-parse only `system doctor --json`
- validate/write steps use exit code plus persisted stdout/stderr transcripts; there is no second machine-readable authoring contract
- the runtime resolves the target repo from the current working directory or enclosing git root and refuses before asking questions when outside a real git repo

Finished interaction target:

- fixed happy path:
  - `system doctor --json`
  - optional bare `system setup`
  - `system doctor --json` again if setup ran
  - `system author charter --validate --from-inputs`
  - `system author charter --from-inputs`
  - final `system doctor --json`
- run artifacts persist under `~/.local/state/system/intake/runs/`
- normal install after dev setup replaces symlinks with copied directories cleanly

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

### `pipeline`

Role:

- orchestration surface
- own route resolution, one bounded explicit stage-compilation surface, one bounded explicit writer surface, and narrow pipeline-run state mutation

Design rule:

- `pipeline` is not a generic workflow engine brand
- it exists to make route truth and compile boundaries explicit and auditable

Finished interaction target:

- `pipeline resolve` and `pipeline compile` stay separate jobs with one shared typed route truth
- the shipped compile surface stays intentionally narrow:
  - `pipeline compile --id <pipeline-id> --stage <stage-id>`
  - `pipeline compile --id <pipeline-id> --stage <stage-id> --explain`
- the shipped capture surface stays intentionally narrow:
  - `pipeline capture --id <pipeline-id> --stage <stage-id>`
  - `pipeline capture --id <pipeline-id> --stage <stage-id> --preview`
  - `pipeline capture apply --capture-id <capture-id>`
- plain `pipeline compile` success is payload-only stdout
- `pipeline compile --explain` is the compile proof surface for that same typed result
- `pipeline capture` is the only supported writer surface for declared stage outputs in M3
- `pipeline state set` stays schema-bound, auditable, and narrow
- `pipeline` should feel like compiler control-plane tooling, not a second front door

### `inspect`

Role:

- packet proof surface
- explain why the packet looks the way it does

Design rule:

- evidence order matters more than prose
- `inspect` should feel auditable, not chatty
- compile-specific proof belongs to `pipeline compile --explain`, not `inspect`

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

- `doctor` is the recovery and baseline-readiness surface
- it reports `SCAFFOLDED`, `PARTIAL_BASELINE`, `INVALID_BASELINE`, or `BASELINE_COMPLETE`
- checklist rows include the artifact label, canonical path, per-artifact status, and exact author command when action is needed

Design rule:

- `doctor` is the only canonical recovery verb

Finished interaction target:

- `doctor` should translate blocker taxonomy into human-facing recovery language
- `doctor` must not print Rust debug shapes in operator output
- `doctor` should keep packet readiness separate from baseline readiness
- baseline checklist order should be stable and actionable

## Experience Layer Versus Command Layer

These are not the same thing.

Experience layer:

- setup-init experience
- setup-refresh experience
- future assisted setup
- future automated setup

Command layer:

- `setup`
- `author`
- `pipeline`
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
- use `runtime zone` for non-canonical derived state under `.system/`
- use `canonical inputs` only when the sentence is explicitly about packet generation reading those artifacts as inputs
- use `refusal` for blocked command outcomes
- use `next safe action` for the repair handoff line
- do not rename `setup` to `bootstrap`, `init`, `hydrate`, or `onboard`
- do not describe `pipeline` as a generic framework when the product meaning is route truth plus a bounded explicit stage-compilation surface

## Hierarchy And Routing Contract

The canonical front-door and routing rules live in [`docs/CLI_COMMAND_HIERARCHY.md`](docs/CLI_COMMAND_HIERARCHY.md).

The highest-value hierarchy rules are:

- the front door is the `setup` family
- the stable operation name remains `setup`
- `setup init` is the concrete first-run subcommand
- bare `system setup` routes to `setup init` when canonical `.system/` truth is absent or invalid; otherwise to `setup refresh`
- `generate` is the default ready-path command
- `inspect` is for packet proof
- `pipeline compile --explain` is for compile proof
- `doctor` is for recovery and readiness

Repo-state routing:

- missing or invalid canonical `.system/` truth -> `setup` / `setup init`
- stale canonical artifacts -> `setup` / `setup refresh`
- regenerate starter files only -> `setup refresh --rewrite`
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
- `pipeline compile` is now a shipped special case:
  - plain success is payload-only stdout
  - `--explain` is proof-only stdout
- `doctor` is a distinct recovery and baseline-readiness surface rather than a packet renderer
- `setup` uses the setup-family anatomy rather than packet anatomy

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

- say `setup` is the durable front door, and say `setup init` is only the concrete first-run subcommand
- say `doctor` is the canonical recovery surface and the baseline-readiness surface
- say `inspect` is the packet proof surface, and say compile proof lives on `pipeline compile --explain`
- say `pipeline compile` is shipped for one bounded M2 target, but do not describe it as generic multi-stage compile support
- do not pretend the current self-referential `inspect` next action is ideal

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
2. `pipeline resolve` establishes route truth for stage-specific pipeline work when needed
3. `pipeline compile` compiles the bounded shipped stage payload when that route is ready
4. `generate` produces the packet quickly
5. `inspect` proves why the packet looks the way it does
6. `doctor` recovers the operator from broken or ambiguous state

That loop breaks when a surface feels semantically wrong even if the code is technically correct.

Current examples of semantically wrong behavior:

- `inspect` gives a self-referential handoff

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

- `inspect` currently emits a self-referential ready-path next action

These gaps should be treated as the highest-value remaining CLI interaction work because they are the moments where the product still feels unfinished at the exact point the operator most needs confidence.

These should be treated as future implementation and conformance work, not silently normalized.

## Source Documents

This file is downstream of:

- [`docs/CLI_PRODUCT_VOCABULARY.md`](docs/CLI_PRODUCT_VOCABULARY.md)
- [`docs/CLI_COMMAND_HIERARCHY.md`](docs/CLI_COMMAND_HIERARCHY.md)
- [`docs/CLI_TONE_RULES.md`](docs/CLI_TONE_RULES.md)
- [`docs/CLI_OUTPUT_ANATOMY.md`](docs/CLI_OUTPUT_ANATOMY.md)

If those documents and this file disagree, fix the disagreement instead of picking one ad hoc.
