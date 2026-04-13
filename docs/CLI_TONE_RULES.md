# CLI Tone Rules (Reduced v1)

## Purpose

This document defines the operator-facing tone rules for reduced-v1 docs, help text, and runtime output.

It exists so the product sounds like one system instead of three different authors. A trust product should feel precise, calm, and concrete under both success and failure.

This document depends on:

- [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md)
- [`docs/CLI_COMMAND_HIERARCHY.md`](CLI_COMMAND_HIERARCHY.md)

## Core Tone

The default tone is **strict but guided**.

That means:

- strict about trust boundaries
- guided about what failed and what to do next
- concise without becoming cryptic
- direct without becoming hostile

The operator should never wonder whether the system knows what is wrong.

The operator should also never feel like the product is scolding them for hitting an expected boundary.

## Global Rules

- Write like a deterministic operator-facing tool for both humans and agents.
- Assume the audience may be:
  - a human in a terminal
  - a CLI agent consuming stdout
  - a CLI agent running commands to gather the information it needs to complete a task
  - a cloud or API workflow calling the same surface
- Prefer concrete facts over filler.
- Name the exact artifact, dependency, packet, or policy rule when known.
- End blocked states with an exact next action.
- Use stable state labels such as `READY`, `REFUSED`, `BLOCKED`, `STALE`, and `NEXT SAFE ACTION`.
- Keep sentences short when the state is urgent or blocked.
- Do not celebrate normal success.
- Do not soften real refusals into vague language.

## Banned Patterns

Do not use:

- `something went wrong`
- `unable to process request` when the exact blocker is known
- `please try again later`
- apologetic filler such as `sorry` or `unfortunately`
- celebratory filler such as `great news`, `success!`, or `all set!`
- blamey phrasing such as `you forgot` or `you failed to`
- mystery phrasing such as `an issue occurred`

## Surface Rules

### Docs

Docs should:

- sound calm, exact, and slightly explanatory
- distinguish the supported path from the legacy reference path without drama
- explain why the command exists before explaining edge cases
- preserve the product hierarchy: `setup`, `generate`, `inspect`, `doctor`

Docs should not:

- oversell partially shipped surfaces
- use marketing language for basic CLI behavior
- bury important trust boundaries in footnotes

### Help text

Help text should:

- be terse, factual, and stable
- explain the supported boundary in one pass
- avoid examples unless the example adds real disambiguation
- read cleanly in a narrow terminal
- keep `pipeline compile` terse about the bounded M2 wedge: one stage payload, one optional `--explain` proof mode

Help text should not:

- sound conversational
- duplicate full recovery guidance that belongs in runtime output
- invent alternative wording for the same command roles

### Runtime success output

Success output should:

- read like an operator summary
- start with outcome, object, and next safe action
- keep the packet or proof result as the main event
- stay calm and matter-of-fact
- keep `pipeline compile --explain` in ordered proof form when the operator explicitly asks for compile proof

Success output should not:

- congratulate the operator
- sound like a status email
- bury the useful result behind framing text

Plain `pipeline compile` success should stay payload-only.

`pipeline compile --explain` should stay proof-only.

### Runtime refusal output

Refusal output should:

- be compact
- name the blocker first
- name the broken artifact, dependency, or policy second
- end with the exact next safe action
- feel procedural rather than dramatic
- keep compile refusal framing stable: `OUTCOME`, `PIPELINE`, `STAGE`, `REASON`, `BROKEN SUBJECT`, `NEXT SAFE ACTION`

Refusal output should not:

- apologize
- moralize
- dump every possible diagnostic detail into the first response
- hide the refusal behind weaker labels like warning or issue

### Runtime proof output

Proof output should:

- read like evidence review
- privilege stable ordering over prose flourish
- explain inclusion, exclusion, freshness, and budget decisions clearly
- feel auditable

Proof output should not:

- sound like internal debug logs
- narrate implementation details before evidence
- drift into a second default packet view

### Runtime recovery output

Recovery output should:

- aggregate blockers once
- make repair feel finite and procedural
- highlight the safest path back to `generate`
- stay concrete even when multiple blockers exist

Recovery output should not:

- make the operator play command whack-a-mole
- repeat the same blocker in different phrasings
- force the operator to infer ordering or severity from prose alone

## Style Rules

- Prefer short declarative sentences.
- Prefer active voice.
- Use imperative phrasing for next steps: `create`, `fill`, `reduce`, `run`.
- Avoid rhetorical questions.
- Avoid emotional hedging.
- Avoid jargon when the product noun already exists.

## Examples

Use:

- `OUTCOME: REFUSED`
- `OBJECT: planning.packet`
- `NEXT SAFE ACTION: create canonical artifact at .system/feature_spec/FEATURE_SPEC.md`
- `SUMMARY: required canonical artifact is missing`
- `BROKEN SUBJECT: canonical artifact FeatureSpec at .system/feature_spec/FEATURE_SPEC.md`

Do not use:

- `Something went wrong while generating your packet`
- `Unfortunately we could not continue`
- `Good news: your packet is ready`
- `There may be an issue with your setup`

## Calibration By Surface

- Docs: most explanatory
- Help text: most compressed
- Success output: concise summary
- Refusal output: shortest and sharpest
- Proof output: densest, but still ordered for humans
- Recovery output: procedural and complete

## Downstream Dependencies

This document should be treated as an input to:

- `D4` output anatomy for success, refusal, proof, and recovery
- `D5` `DESIGN.md` as the CLI interaction contract
- `D6` operator-journey conformance review
