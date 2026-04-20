# CLI Command Hierarchy And Front Door (Reduced v1)

## Purpose

This document defines the canonical command hierarchy and startup-routing model for the reduced-v1 CLI product.

It exists so README/docs, CLI help, tests, and future interaction-design work all describe the same front door, the same steady-state path, and the same recovery path.

This document depends on [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md) for the product language it uses.

## Core Model

The reviewed reduced-v1 CLI product has six operator-facing surfaces, in this order:

1. `setup` (`setup`, `setup init`, `setup refresh`)
2. `author` (`author charter`, `author charter --from-inputs <path|->`)
3. `pipeline`
4. `generate`
5. `inspect`
6. `doctor`

That ordering is not arbitrary.

- The `setup` family establishes or refreshes trusted project truth.
- `author` replaces setup-owned charter scaffolding with completed canonical truth.
- `pipeline` owns orchestration truth, route resolution, explicit stage selection, explicit stage compilation, explicit stage-output capture, and narrow route-state mutation.
- `generate` is the default ready-path command once trusted truth exists.
- `inspect` is the packet proof surface when the operator needs to verify why a packet looks the way it does.
- `doctor` is the recovery and readiness surface when the operator needs blocker aggregation, repair guidance, or a readiness check.

Current implementation note:

- the currently shipped binary exposes `setup`, `author`, `pipeline`, `generate`, `inspect`, and `doctor`
- the reviewed product surface includes `pipeline` as the orchestration surface for `list`, `show`, `resolve`, `compile`, `capture`, and `state set`

## Front Door Rule

The front door is the `setup` family.

The stable operation name remains `setup`.

That distinction matters:

- `setup` is the durable product operation name for establishing or repairing trusted project truth.
- `setup init` is only the concrete first-run subcommand name.
- Bare `system setup` routes to `setup init` when canonical `.system/` truth is absent or invalid; otherwise it routes to `setup refresh`.

Do not rename the core operation based on the interaction style. The interaction style may evolve. The operation should stay stable.

Historical reference only:

- Earlier docs said: `The front door is a guided setup experience.`

## Reduced-v1 Truth About Setup

Reduced v1 now has one explicit setup-family story:

- The product front door is setup-first.
- `setup` is the durable family term; `setup init` is only the concrete first-run subcommand.
- `setup refresh` is the explicit refresh path once canonical `.system/` truth already exists.
- `setup refresh` preserves canonical files by default.
- `setup refresh --rewrite` rewrites only the setup-owned starter files:
  - `.system/charter/CHARTER.md`
  - `.system/feature_spec/FEATURE_SPEC.md`
  - `.system/project_context/PROJECT_CONTEXT.md`
- `setup refresh --reset-state` resets only `.system/state/**`.
- `PROJECT_CONTEXT.md` is optional semantically for planning packets, but setup still creates it as a starter file.
- The shipped starter templates are scaffolding only. `generate` and `doctor` stay blocked until the required starter files are replaced with completed canonical truth.
- Scaffolded setup flows end with `run \`system author charter\`` as the next safe action; ready setup flows end with `system doctor`.

## Hierarchy Rules

### `setup`

Use `setup` when:

- the operator wants the setup family to choose the right path
- canonical `.system/` truth is absent, invalid, or needs refresh
- the operator wants one stable front-door command name

### `setup init`

Use `setup init` when:

- the repo is new to the system
- canonical `.system/` truth does not exist yet
- the operator is naming the first-run path explicitly

### `setup refresh`

Use `setup refresh` when:

- trusted project truth exists but posture is stale
- architecture, standards, or project context changed materially
- the operator is intentionally refreshing canonical artifacts, not generating a packet
- the operator needs one of the explicit refresh modifiers:
  - default preserve behavior
  - `--rewrite` for setup-owned starter files only
  - `--reset-state` for `.system/state/**` only

### `author`

Use `author` when:

- setup has already established canonical `.system/` truth
- a setup-owned starter file must be replaced with completed canonical truth
- the operator needs a supported human-guided or automation-safe authoring surface

The first shipped authoring wedge is `system author charter`.

`system author charter` is the human-guided surface.

`system author charter --from-inputs <path|->` is the agent and automation surface.

### `generate`

Use `generate` when:

- canonical artifacts already exist
- the operator wants the minimum correct planning packet quickly
- the repo is in a ready state or close enough that a compact refusal is the right first response

`generate` is the default ready-path command.

### `pipeline`

Use `pipeline` when:

- the operator needs the authoritative route for a pipeline
- the operator wants to compile one explicitly selected stage payload with `pipeline compile --id <pipeline-id> --stage <stage-id>`
- the operator wants to capture one explicitly selected stage output with `pipeline capture --id <pipeline-id> --stage <stage-id>`
- the operator needs to set narrow route-state routing, refs, or run fields inside the declared schema

`pipeline` is not the front door. It is the orchestration surface after trusted project truth already exists.

### `inspect`

Use `inspect` when:

- the operator wants proof rather than the packet body alone
- the operator needs inclusion, exclusion, freshness, or budget reasoning
- success output from `generate` needs verification
- the operator is reviewing packet proof, not compile proof

`inspect` is not the front door. It is the proof path after or alongside generation.

### `doctor`

Use `doctor` when:

- the operator needs blocker aggregation in one place
- the next safe action is not obvious from one compact refusal
- the operator wants a readiness check before retrying `generate`

`doctor` is the recovery and readiness command, not a competing front door.

## Repo-State Routing

### Routing matrix

| Repo state | Operator intent | Correct first surface | Why |
|------------|-----------------|-----------------------|-----|
| No canonical `.system/` truth yet, or the root is invalid | Establish trusted project truth | `setup` -> `setup init` | The repo is not ready for packet generation |
| Canonical `.system/` truth exists but posture is stale or needs deliberate refresh | Re-establish trusted project truth | `setup` -> `setup refresh` or direct `setup refresh` | The job is posture refresh, not packet generation |
| Canonical root exists but setup-owned starter files must be regenerated | Recover setup-owned starter files without inventing new truth surfaces | `setup refresh --rewrite` | Recovery belongs to the setup family, not raw file creation |
| Canonical artifacts exist and the operator needs route truth or stage selection | Work the planning compiler control plane | `pipeline` | The operator is resolving or preparing explicit pipeline stages |
| Canonical artifacts are ready | Get the packet | `generate` | This is the default steady-state path |
| Canonical artifacts are ready | Prove or audit the packet basis | `inspect` | The operator wants proof |
| Canonical artifacts are contradictory, malformed beyond setup-family recovery, or otherwise unclear | Diagnose and recover cleanly | `doctor` | The operator needs the full blocker report |
| Unsupported live execution request | Return to supported reduced-v1 path | `generate` refusal with exact next safe action | Reduced v1 supports planning packets and fixture-backed execution demos only |

### Practical routing rules

- If the operator is trying to establish truth, route to `setup`, `setup init`, or `setup refresh`.
- If the operator is trying to resolve a route or select an explicit stage, route to `pipeline`.
- If the operator is trying to get work done from an already-prepared repo, route to `generate`.
- If the operator is asking “why did this happen?” route to `inspect`.
- If the operator is asking “what is wrong and what do I fix first?” route to `doctor`.

## Next-Safe-Action Rules

The next safe action should reinforce the hierarchy rather than compete with it.

- Use setup-family next actions when canonical truth is missing, invalid, or must be re-established.
- Use `system setup refresh --rewrite` when setup-owned starter files must be regenerated without inventing new artifact types.
- Use `system setup refresh --reset-state` only when the repair is limited to `.system/state/**`.
- Use `pipeline resolve` when the operator needs route truth before future stage-specific compile work.
- Use `pipeline compile --id <pipeline-id> --stage <stage-id>` only after `pipeline resolve` established the current route basis.
- Use `pipeline compile --explain` when the operator needs compile proof; keep `inspect` for packet proof.
- Use `pipeline capture --preview` or `pipeline capture` only after `pipeline resolve` established the current route basis for the selected stage.
- Use `doctor` when deeper diagnosis or blocker aggregation is needed.
- Use `inspect` after a ready `generate` result when the operator needs proof.
- Use `system generate --packet planning.packet` as the fallback from unsupported live execution requests.

Do not send the operator to `inspect` when there is no trusted packet basis yet.

Do not send the operator to `doctor` as the default happy path when the repo is already ready.

## Repo Discovery Rules

The routing story also depends on which repo root the command is acting against.

- Commands anchor to the enclosing git root when one exists.
- If there is no enclosing git root, the nearest ancestor with `.system/` may act as the managed root.
- A nested git repo boundary wins over a parent managed repo. The CLI must not cross that boundary and silently use the parent `.system/`.
- For normal planning flows, invocation from a nested directory inside a ready repo should still resolve against the repo root.
- Fixture-backed execution demos are selected explicitly via `execution.demo.packet` with `--fixture-set`; they do not redefine the normal planning front door.

## Experience Layer Versus Command Layer

This product has two layers that should be named differently:

- Experience layer: setup-init experience, setup-refresh experience, future assisted or automated setup
- Command layer: `setup`, `author`, `pipeline`, `generate`, `inspect`, `doctor`

Do not try to make the command names carry the full UX story.

The command names should describe the durable operation.

The experience layer can evolve from LLM-guided to partially automated to fully CLI-owned without forcing a command rename.

Historical reference only:

- `guided setup`
- `guided setup refresh`

## Consequences For Other Docs

Downstream docs and help text should follow these rules:

- Present `setup` first, but do not imply that every steady-state user starts there every time.
- Present `setup init` as the concrete first-run subcommand without turning `init` into the durable product term.
- Present `setup refresh` as preserve-by-default refresh plus the narrow `--rewrite` and `--reset-state` modifiers.
- Present `pipeline` as the orchestration surface once canonical truth exists.
- Present `generate` as the normal repeat-use path for ready repos.
- Present `inspect` as proof, not a second packet generator.
- Present `doctor` as recovery/readiness, not a second setup command and not a rival to `generate`.
- Keep historical guided-setup wording fenced as historical reference only.

## Downstream Dependencies

This document should be treated as an input to:

- `D3` tone rules for docs/help/runtime
- `D4` output anatomy for success, refusal, proof, and recovery
- `D5` `DESIGN.md` as the CLI interaction contract
- `D6` operator-journey conformance review
