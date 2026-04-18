# CLI Product Vocabulary (Reduced v1)

## Purpose

This document locks the operator-facing language for the reduced-v1 CLI product.

It exists so docs, help text, examples, runtime output, tests, and future interaction-design work all use the same nouns and verbs. If a user-facing surface needs new wording, update this document first and then revalidate the dependent surfaces.

This is not the place for internal type names or implementation jargon. It owns the product language the operator sees.

## Scope

This vocabulary applies to:

- `README.md`
- `docs/START_HERE.md`
- `docs/SUPPORTED_COMMANDS.md`
- CLI help text
- default markdown and inspect output
- examples, onboarding snippets, and fixtures that explain the supported product path
- future CLI interaction guidance in `DESIGN.md`

This vocabulary does not rewrite legacy docs. Legacy reference material may preserve older terms when clearly scoped as legacy.

## Canonical Terms

### Core verbs

- `setup`
  - The durable front-door operation name for canonical `.system/` truth establishment and repair.
  - Use when naming the setup family or the routed front door.
  - Bare `system setup` routes to `setup init` when canonical `.system/` truth is absent or invalid; otherwise it routes to `setup refresh`.
- `setup init`
  - The concrete first-run subcommand name.
  - Use when naming the explicit first-run or recovery path for an absent or invalid canonical `.system/` root.
  - Do not promote `init` into the durable product term for the setup family.
- `setup refresh`
  - Use when existing canonical artifacts must be refreshed because posture is stale or has changed.
  - Preserves canonical files by default.
  - `setup refresh --rewrite` rewrites only the setup-owned starter files.
  - `setup refresh --reset-state` resets only `.system/state/**`.
  - Do not rename this as bootstrap refresh, re-init, or health repair.
- `generate`
  - The packet-generation surface.
  - Use when the operator wants the minimum correct planning packet from trusted inputs.
- `pipeline`
  - The orchestration surface.
  - Use when the operator needs route truth, explicit stage selection, explicit stage compilation, explicit stage-output capture, or narrow route-state mutation.
  - Do not describe this as a generic workflow framework unless the product contract actually expands that far.
- `pipeline capture --preview`
  - The capture preview surface for the supported M3 writer wedge.
  - Use when the operator wants one validated materialization plan and a deterministic `capture_id` before writing.
- `pipeline capture apply --capture-id`
  - The cached capture-apply surface for the supported M3 writer wedge.
  - Use when the operator wants to apply one previously previewed capture plan without re-pasting stdin.
- `pipeline handoff emit`
  - The downstream handoff-emission surface for the supported M5 adoption wedge.
  - Use when the operator wants one derived bundle that a named downstream consumer can trust without rereading the repo on the happy path.
- `inspect`
  - The packet proof surface.
  - Use when the operator wants to verify why a packet looks the way it does.
- `pipeline compile --explain`
  - The compile proof surface for the supported M2 stage-compile wedge.
  - Use when the operator wants route-basis, input-decision, and output-contract proof for one compiled stage payload.
- `doctor`
  - The recovery surface.
  - Use when the operator needs blocker aggregation and the next safe recovery path.

### Core nouns

- `canonical artifacts`
  - The preferred term for the trusted repo-local `.system/` files the system reads as project truth.
  - In reduced v1, this is the primary noun for the files themselves.
- `setup-owned starter files`
  - The canonical files created by setup:
    - `.system/charter/CHARTER.md`
    - `.system/feature_spec/FEATURE_SPEC.md`
    - `.system/project_context/PROJECT_CONTEXT.md`
  - `PROJECT_CONTEXT.md` is optional semantically, but it remains one of the starter files created by setup.
  - The shipped starter templates are scaffolding only until the required files are replaced with completed canonical truth.
- `runtime zone`
  - The preferred term for non-canonical derived state kept under `.system/`.
  - Runtime zones are not canonical inputs and must never be described as project truth.
- `canonical inputs`
  - Not a separate concept from `canonical artifacts`.
  - Use only when the sentence is specifically about packet generation reading those artifacts as inputs.
  - Avoid defining `canonical artifacts` and `canonical inputs` side by side in the same explanation unless the sentence needs the input framing.
- `derived views`
  - Human-facing or renderer-facing outputs that are derived from canonical truth and are not runtime inputs.
- `handoff bundle`
  - The derived downstream bundle emitted by `pipeline handoff emit`.
  - It is a trust surface for one named consumer, not canonical project truth.
- `trust class`
  - The per-file trust label inside the emitted handoff bundle.
  - In M5 the supported trust classes are `canonical`, `compiler_derived`, and `external_manual_derived`.
- `bundle-only read set`
  - The allowlisted emitted-bundle files a downstream consumer may read on the happy path.
- `planning packet`
  - The default live reduced-v1 packet product.
- `execution demo packet`
  - The fixture-backed execution-only demo surface via `execution.demo.packet`.
- `proof surface`
  - The operator-facing explanation layer for inclusion, exclusion, freshness, and budget decisions.
- `recovery surface`
  - The operator-facing blocker and repair layer.
- `refusal`
  - The explicit blocked outcome when the system stops unsafe work.
- `next safe action`
  - The exact follow-up command or repair action the operator should take next.
- `legacy reference material`
  - Old Python harness behavior and docs that remain readable but are not the supported runtime path.

## Phrase Rules

These phrases should remain stable across top-level docs and help text unless the product boundary changes:

- `system setup`, `system setup init`, `system setup refresh`
- "`setup` is the durable setup term"
- "`setup init` is the concrete first-run subcommand"
- "`setup refresh` preserves canonical files by default"
- "`setup refresh --rewrite` rewrites only setup-owned starter files"
- "`setup refresh --reset-state` resets only `.system/state/**`"
- "success path ends with `system doctor`"
- `planning packet generation`
- `canonical repo-local `.system/` inputs`
- `fixture-backed execution demo`
- `live execution is explicitly refused`
- "`inspect` is the packet proof surface"
- "`pipeline compile --explain` is the compile proof surface"
- "`pipeline capture` is the explicit writer surface"
- "`pipeline handoff emit` is the downstream handoff-emission surface"
- "`doctor` is the recovery surface"
- `payload-only stdout`
- `proof-only stdout`
- `bundle-only reads`
- `external_manual_derived`
- `next safe action`

Use backticks around command verbs when they refer to command names.

Use the full phrase `next safe action` in output. Do not shorten it to `next step` or `fix`.

## Allowed And Banned Language

### Prefer

- `setup`, not generic startup wording
- `setup init` only when naming the concrete first-run subcommand
- `pipeline`, not generic framework wording, when naming route/control-plane work
- `pipeline capture`, not generic save/apply wording, when naming stage-output materialization
- `pipeline handoff emit`, not generic export/package wording, when naming the M5 downstream bundle surface
- `generate`, not compile/build/render when describing packet creation
- `inspect`, not explain/debug when naming the proof command
- `pipeline compile --explain`, not `inspect`, when naming compile proof
- `doctor`, not health repair or troubleshoot when naming the recovery command
- `refusal`, not warning or issue, when a command is blocked
- `canonical artifacts` or `canonical inputs`, not config files or metadata, when referring to trusted product inputs
- `derived views`, not source of truth, for human-facing copies or render outputs
- `handoff bundle`, not canonical bundle or source of truth, for the emitted downstream trust surface
- `trust class`, not confidence score or quality tier, for emitted handoff trust labels
- `next safe action`, not recommendation or suggestion, for the operator handoff line

### Ban On Operator-Facing Supported Surfaces

Do not use these terms as the primary operator-facing language for the supported reduced-v1 path:

- `bootstrap`
- `init` as the generic product label for the setup family
- `initialize` as the generic product label for the setup family
- `hydrate`
- `workflow engine` as the primary product label for the supported wedge
- `health repair`
- `warning` when the command is actually blocked
- `issue` when the command is actually blocked and the product means `refusal` or `blocker`
- `problem` or `something went wrong` when the exact blocker is known

These words may still appear in:

- legacy reference docs
- internal type or contract names such as `issue categories`
- low-level implementation details that are not presented as product copy

## Surface-Specific Rules

### Docs and help text

- Use the same surface ordering everywhere: `setup`, `pipeline`, `generate`, `inspect`, `doctor` once `pipeline` lands.
- Present reduced v1 as a trust product, not a generic scaffold.
- Keep the supported path and the legacy reference path clearly separated.

### Runtime output

- When a command is blocked, call it a `REFUSED` outcome and provide the `NEXT SAFE ACTION`.
- When a command succeeds, keep the language operator-focused and concrete.
- Do not celebrate. Summarize.
- Do not hide a blocked state behind vague copy.

### Contracts and tests

- Contracts may use stricter internal language, but any operator-facing examples inside contracts should still follow this vocabulary.
- Tests that guard help or rendering copy should treat the phrases in this document as stable unless the product boundary itself changes.

## Examples

Use:

- `REFUSED: missing required canonical artifact`
- `NEXT SAFE ACTION: run \`system setup\``
- `NEXT SAFE ACTION: run \`system setup refresh --rewrite\``
- "`inspect` is the packet proof surface"
- "`pipeline compile --explain` is the compile proof surface"
- "`doctor` is the recovery surface"

Do not use:

- `WARNING: setup is incomplete`
- `Issue detected, maybe run health repair`
- `Something went wrong during generation`
- `Initialize the repo metadata before debugging packet output`

## Historical Reference Only

- Superseded M4/M5 wording retained only so older drift checks and review notes stay searchable:
  - "`setup` is still a placeholder"

## Downstream Dependencies

This document should be treated as an input to:

- `D2` command hierarchy and front door
- `D3` tone rules for docs/help/runtime
- `D4` output anatomy for success, refusal, proof, and recovery
- `D5` `DESIGN.md` as the CLI interaction contract
- `D6` operator-journey conformance review
