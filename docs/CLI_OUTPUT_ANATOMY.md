# CLI Output Anatomy (Reduced v1)

## Purpose

This document defines the operator-facing output anatomy for reduced-v1 success, refusal, proof, and recovery surfaces.

It exists so docs, fixtures, tests, and future output work all agree on section order, compactness rules, and what belongs in the first lines versus deeper sections.

This document depends on:

- [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md)
- [`docs/CLI_COMMAND_HIERARCHY.md`](CLI_COMMAND_HIERARCHY.md)
- [`docs/CLI_TONE_RULES.md`](CLI_TONE_RULES.md)
- [`docs/contracts/C-04-resolver-result-and-doctor-blockers.md`](contracts/C-04-resolver-result-and-doctor-blockers.md)
- [`docs/contracts/C-05-renderer-and-proof-surfaces.md`](contracts/C-05-renderer-and-proof-surfaces.md)

## Global Rules

- `generate` and `inspect` start with the same three-line trust header:
  1. `OUTCOME`
  2. `OBJECT`
  3. `NEXT SAFE ACTION`
- The trust header comes before any other section.
- Ready-path output keeps the useful result as the main event.
- Refusal-path output stays compact.
- Proof output is allowed to be denser, but section ordering stays stable.
- `pipeline compile` is a special M2 case: plain success is payload-only stdout, and `pipeline compile --explain` is proof-only stdout.
- `pipeline capture` is a special M3 case: preview and apply use stable capture-specific section ordering instead of the packet trust-header shape.
- `doctor` is a special case in current reduced v1: its shipped anatomy is still transitional and does not yet use the same trust-header shape as `generate` and `inspect`.
- `setup` is also a special case in current reduced v1: it is placeholder-only and therefore has placeholder anatomy, not full reduced-v1 runtime anatomy.

## `generate` Anatomy

### Ready

First three lines:

1. `OUTCOME: READY`
2. `OBJECT: <packet_id>`
3. Current shipped behavior: `NEXT SAFE ACTION: run \`system inspect ...\` for proof`

Section order after the trust header:

1. `## PACKET OVERVIEW`
2. `## INCLUDED SOURCES`
3. `## OMISSIONS AND BUDGET`
4. `## DECISION SUMMARY`
5. `## PACKET BODY`

Notes:

- The packet body is the product, not a receipt.
- Fixture-backed execution demo output may include fixture context in the body path.
- Budget summarize/exclude behavior must preserve the same section order without leaking omitted content.

### Refused

First three lines:

1. `OUTCOME: REFUSED`
2. `OBJECT: <packet_id>`
3. `NEXT SAFE ACTION: <exact repair action>`

Section order after the trust header:

1. `## REFUSAL`
2. `CATEGORY`
3. `SUMMARY`
4. `BROKEN SUBJECT`
5. `NEXT SAFE ACTION`

Rules:

- Do not print packet body sections on refusal.
- Do not expand into a full diagnostic dump.
- One refusal section, one broken subject, one exact next safe action.

## `inspect` Anatomy

### Ready

First three lines:

1. `OUTCOME: READY`
2. `OBJECT: <packet_id>`
3. `NEXT SAFE ACTION: run \`system inspect ...\` for proof`

Section order after the trust header:

1. `## DECISION LOG`
2. `## BUDGET OUTCOME`
3. `## REFUSAL`
4. `## BLOCKERS`
5. `## PACKET OVERVIEW`
6. `## PACKET BODY`
7. `## JSON FALLBACK`

Rules:

- `## REFUSAL` and `## BLOCKERS` still appear on the ready path and may contain `NONE`.
- `## JSON FALLBACK` always appears.
- Proof order privileges evidence review over narrative prose.
- The ready-path `NEXT SAFE ACTION` must hand off to the packet surface, not back into `inspect`.
- `inspect` remains the packet proof surface; compile-specific proof belongs to `pipeline compile --explain`.

### Blocked or refused

First three lines:

1. `OUTCOME: REFUSED` or `OUTCOME: BLOCKED`
2. `OBJECT: <packet_id>`
3. `NEXT SAFE ACTION: <exact repair action>`

Section order after the trust header:

1. `## DECISION LOG`
2. `## BUDGET OUTCOME`
3. `## REFUSAL`
4. `## BLOCKERS`
5. `## JSON FALLBACK`

Conditional rule:

- For non-ready fixture-backed execution demo requests, fixture context may be injected immediately after the trust header so the operator still sees the demo basis before the deeper proof sections.

## `doctor` Anatomy

### Current shipped reduced-v1 anatomy

`doctor` is still transitional.

Current ready shape:

1. `READY`

Current blocked shape:

1. `BLOCKED`
2. repeated blocker groups with:
   - `CATEGORY`
   - `SUMMARY`
   - `SUBJECT`
   - `NEXT ACTION`

Important honesty rule:

- Until `doctor` is upgraded, docs must not claim that it already shares the full trust-header anatomy used by `generate` and `inspect`.

### Required future alignment

The long-term anatomy should converge on:

1. `OUTCOME`
2. `OBJECT`
3. `NEXT SAFE ACTION`
4. `## BLOCKERS`
5. readiness or retry guidance

But that is not the full shipped shape today.

## `pipeline compile` Anatomy

### Ready

plain `pipeline compile` success is payload-only stdout.

Rules:

- plain `pipeline compile` success is payload-only stdout with no trust header
- plain success starts directly with the compiled stage payload
- plain success must not append route-basis recap, refusal framing, or a trailing next safe action

### Proof mode

`pipeline compile --explain` success is proof-only stdout.

Section order:

1. `OUTCOME: COMPILED`
2. `TARGET`
3. `ROUTE BASIS`
4. `ROUTE SNAPSHOT`
5. `VARIABLES`
6. `DOCUMENTS`
7. `OUTPUTS`
8. `GATING`
9. payload summary only

Rules:

- `pipeline compile --explain` success is proof-only stdout
- explain mode must not include the payload body in addition to proof
- freshness recovery remains explicit: re-run `pipeline resolve` before retrying compile when route basis is missing, stale, or inactive

### Refused

Compile refusal uses a compact refusal block:

1. `OUTCOME: REFUSED`
2. `PIPELINE`
3. `STAGE`
4. `REASON`
5. `BROKEN SUBJECT`
6. `NEXT SAFE ACTION`

## `pipeline capture` Anatomy

### Preview

`pipeline capture --preview` success is proof-like preview output.

Section order:

1. `OUTCOME: PREVIEW`
2. `PIPELINE`
3. `STAGE`
4. `CAPTURE ID`
5. `ROUTE BASIS REVISION`
6. `WRITE PLAN`
7. `POST-CAPTURE STATE UPDATES`
8. `NEXT SAFE ACTION`

Rules:

- preview does not write declared outputs
- preview writes only the runtime cache entry under `.system/state/pipeline/capture/`
- preview and apply must render from one shared compiler-owned capture plan

### Apply

`pipeline capture` direct apply and `pipeline capture apply --capture-id <capture-id>` success share one stable captured order.

Section order:

1. `OUTCOME: CAPTURED`
2. `PIPELINE`
3. `STAGE`
4. `WRITTEN FILES`
5. `STATE UPDATES`
6. `NEXT SAFE ACTION`

Rules:

- apply success reports written files only after every write and state update has succeeded
- apply remains transactional at the acceptance boundary
- cached apply revalidates freshness before writing anything
- when apply persisted automatic route-state updates, `NEXT SAFE ACTION` must tell the operator to run `pipeline resolve` before the next compile or capture
- when apply also leaves manual `sets:` decisions unresolved, `NEXT SAFE ACTION` may be a single exact multi-step line: one `pipeline state set` command per unresolved variable in declared `stage.sets` order, then `pipeline resolve`

### Refused

Capture refusal uses the same compact refusal posture as compile:

1. `OUTCOME: REFUSED`
2. `PIPELINE`
3. `STAGE`
4. `REASON`
5. `NEXT SAFE ACTION`

## `setup` Anatomy

### Current shipped reduced-v1 anatomy

`setup` is placeholder-only in current reduced v1.

Current shape:

1. one placeholder line naming the contract version
2. the fact that `setup` is a placeholder-only entrypoint
3. the fact that planning packet generation, `inspect`, and `doctor` are implemented in reduced v1

Important honesty rule:

- Do not document `setup` as if it already has the full guided runtime anatomy. Today it is a placeholder entrypoint plus an external guided setup story.

## Presentation Failure And Parse-Validation Output

These are narrow exception paths, not primary product surfaces.

Rules:

- Keep them terse.
- Name the failure type directly.
- Do not pretend they are packet or proof output.
- Do not let them redefine the normal anatomy for `generate`, `inspect`, `doctor`, or `setup`.

## Stable First-Impression Rules

For any surface that has the full reduced-v1 anatomy:

- the first lines answer what happened
- what object that applies to
- what to do next

For surfaces that are still transitional (`doctor`, `setup`), docs must say so explicitly rather than silently implying parity.

## Downstream Dependencies

This document should be treated as an input to:

- `D5` `DESIGN.md` as the CLI interaction contract
- `D6` operator-journey conformance review
