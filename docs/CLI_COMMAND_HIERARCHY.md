# CLI Command Hierarchy And Front Door (Reduced v1)

## Purpose

This document defines the canonical command hierarchy and startup-routing model for the reduced-v1 CLI product.

It exists so README/docs, CLI help, tests, and future interaction-design work all describe the same front door, the same steady-state path, and the same recovery path.

This document depends on [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md) for the product language it uses.

## Core Model

The reviewed reduced-v1 CLI product has five operator-facing surfaces, in this order:

1. `setup` / `setup refresh`
2. `pipeline`
3. `generate`
4. `inspect`
5. `doctor`

That ordering is not arbitrary.

- `setup` / `setup refresh` establishes or refreshes trusted project truth.
- `pipeline` owns orchestration truth, route resolution, explicit stage selection, and narrow route-state mutation.
- `generate` is the default ready-path command once trusted truth exists.
- `inspect` is the proof surface when the operator needs to verify why a packet looks the way it does.
- `doctor` is the recovery and readiness surface when the operator needs blocker aggregation, repair guidance, or a readiness check.

Current implementation note:

- the currently shipped binary exposes `setup`, `pipeline`, `generate`, `inspect`, and `doctor`
- the reviewed product surface includes `pipeline` as the orchestration surface for `list`, `show`, `resolve`, and `state set`

## Front Door Rule

The front door is a guided setup experience.

The stable operation name remains `setup`.

That distinction matters:

- `guided setup` is the experience layer. It may be LLM-guided, partially automated, or fully CLI-owned later.
- `setup` is the durable product operation name for establishing trusted project truth.

Do not rename the core operation based on the interaction style. The interaction style may evolve. The operation should stay stable.

## Reduced-v1 Truth About Setup

Reduced v1 has a split but intentional setup story:

- The product front door is still setup-first.
- The Rust CLI `setup` command is still placeholder-only.
- Until Rust setup exists, the supported docs path may route the operator through the existing guided setup flow to establish canonical artifacts.
- Once those artifacts exist, the Rust CLI becomes the supported packet-resolution authority.

This means the product should talk about setup as the front door without pretending the Rust CLI already owns the entire guided setup flow.

## Hierarchy Rules

### `setup` / `setup refresh`

Use `setup` when:

- the repo is new to the system
- canonical artifacts do not exist yet
- the operator is establishing trusted project truth for the first time

Use `setup refresh` when:

- trusted project truth exists but posture is stale
- architecture, standards, or project context changed materially
- the operator is intentionally refreshing canonical artifacts, not generating a packet

### `generate`

Use `generate` when:

- canonical artifacts already exist
- the operator wants the minimum correct planning packet quickly
- the repo is in a ready state or close enough that a compact refusal is the right first response

`generate` is the default ready-path command.

### `pipeline`

Use `pipeline` when:

- the operator needs the authoritative route for a pipeline
- the operator wants to select one explicitly selected stage payload for future compile work
- the operator needs to set narrow route-state routing, refs, or run fields inside the declared schema

`pipeline` is not the front door. It is the orchestration surface after trusted project truth already exists.

### `inspect`

Use `inspect` when:

- the operator wants proof rather than the packet body alone
- the operator needs inclusion, exclusion, freshness, or budget reasoning
- success output from `generate` needs verification

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
| No canonical `.system/` artifacts yet | Establish trusted project truth | Guided setup / `setup` | The repo is not ready for packet generation |
| Canonical artifacts exist but posture is stale or needs deliberate refresh | Re-establish trusted project truth | Guided setup refresh / `setup refresh` | The job is posture refresh, not packet generation |
| Canonical artifacts exist and the operator needs route truth or stage selection | Work the planning compiler control plane | `pipeline` | The operator is resolving or preparing explicit pipeline stages |
| Canonical artifacts are ready | Get the packet | `generate` | This is the default steady-state path |
| Canonical artifacts are ready | Prove or audit the packet basis | `inspect` | The operator wants proof |
| Canonical artifacts are missing, malformed, contradictory, or otherwise unclear | Diagnose and recover cleanly | `doctor` | The operator needs the full blocker report |
| Unsupported live execution request | Return to supported reduced-v1 path | `generate` refusal with exact next safe action | Reduced v1 supports planning packets and fixture-backed execution demos only |

### Practical routing rules

- If the operator is trying to establish truth, route to `setup` or `setup refresh`.
- If the operator is trying to resolve a route or select an explicit stage, route to `pipeline`.
- If the operator is trying to get work done from an already-prepared repo, route to `generate`.
- If the operator is asking “why did this happen?” route to `inspect`.
- If the operator is asking “what is wrong and what do I fix first?” route to `doctor`.

## Next-Safe-Action Rules

The next safe action should reinforce the hierarchy rather than compete with it.

- Use setup-oriented next actions when canonical truth is missing or must be re-established.
- Use `pipeline resolve` when the operator needs route truth before future stage-specific compile work.
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

- Experience layer: guided setup, guided setup refresh, future assisted or automated setup
- Command layer: `setup`, `generate`, `inspect`, `doctor`

Do not try to make the command names carry the full UX story.

The command names should describe the durable operation.

The experience layer can evolve from LLM-guided to partially automated to fully CLI-owned without forcing a command rename.

## Consequences For Other Docs

Downstream docs and help text should follow these rules:

- Present `setup` first, but do not imply that every steady-state user starts there every time.
- Present `pipeline` as the orchestration surface once canonical truth exists.
- Present `generate` as the normal repeat-use path for ready repos.
- Present `inspect` as proof, not a second packet generator.
- Present `doctor` as recovery/readiness, not a second setup command and not a rival to `generate`.
- Keep the guided setup story explicit while Rust setup remains placeholder-only.

## Downstream Dependencies

This document should be treated as an input to:

- `D3` tone rules for docs/help/runtime
- `D4` output anatomy for success, refusal, proof, and recovery
- `D5` `DESIGN.md` as the CLI interaction contract
- `D6` operator-journey conformance review
