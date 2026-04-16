# Supported Commands (Reduced v1, Rust-first)

The supported reduced-v1 entrypoint is the Rust CLI (crate `system-cli`, binary `system`).

For the authoritative command surface and help ordering, see [`C-02`](contracts/C-02-rust-workspace-and-cli-command-surface.md).
For the canonical operator-facing product language, see [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md).
For the front door and repo-state routing model, see [`docs/CLI_COMMAND_HIERARCHY.md`](CLI_COMMAND_HIERARCHY.md).
For operator-facing tone rules, see [`docs/CLI_TONE_RULES.md`](CLI_TONE_RULES.md).
For section ordering and output-shape rules, see [`docs/CLI_OUTPUT_ANATOMY.md`](CLI_OUTPUT_ANATOMY.md).

## Local invocation (current shipped binary)

From repo root:

```bash
cargo run -p system-cli -- --help
cargo run -p system-cli -- setup
cargo run -p system-cli -- pipeline --help
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec --explain
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.04_charter_inputs
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.05_charter_synthesize
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.06_project_context_interview
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.07_foundation_pack --preview
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.10_feature_spec < /tmp/FEATURE_SPEC.md
cargo run -p system-cli -- pipeline capture apply --capture-id <capture-id>
cargo run -p system-cli -- generate
cargo run -p system-cli -- inspect
cargo run -p system-cli -- doctor
```

## Reviewed `pipeline` surface

```bash
cargo run -p system-cli -- pipeline list
cargo run -p system-cli -- pipeline show --id pipeline.foundation
cargo run -p system-cli -- pipeline resolve --id pipeline.foundation_inputs
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec --explain
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.04_charter_inputs
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.05_charter_synthesize
cargo run -p system-cli -- pipeline state set --id pipeline.foundation_inputs --var needs_project_context=true
cargo run -p system-cli -- pipeline resolve --id pipeline.foundation_inputs
# Only if resolve marks stage.06_project_context_interview active:
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.06_project_context_interview
cargo run -p system-cli -- pipeline resolve --id pipeline.foundation_inputs
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.07_foundation_pack --preview
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec
# External step outside `system`: use the compile payload to produce /tmp/FEATURE_SPEC.md
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.10_feature_spec < /tmp/FEATURE_SPEC.md
cargo run -p system-cli -- pipeline capture apply --capture-id <capture-id>
```

For the reviewed operator-surface contract baseline, see [`C-09`](contracts/pipeline-operator-surface-and-id-resolution.md).

## Current command meanings

- `setup` is the reserved setup-first entrypoint for the reduced-v1 trust flow. It is still a placeholder and not yet a real Rust setup flow.
- `pipeline` owns `list`, `show`, `resolve`, `compile`, `capture`, and `state set` for the reviewed wedge.
- `pipeline compile --id <pipeline-id> --stage <stage-id>` is the supported M2 compile surface for the first bounded target: `pipeline.foundation_inputs` + `stage.10_feature_spec`.
- Plain `pipeline compile` success is payload-only stdout. `pipeline compile --explain` is proof-only stdout.
- Compile freshness is explicit. If route basis is missing, stale, or inactive, re-run `pipeline resolve` before retrying compile.
- `pipeline capture --id <pipeline-id> --stage <stage-id>` is the supported M3 / M3.5 writer surface for the bounded capture targets:
  - `pipeline.foundation_inputs` + `stage.04_charter_inputs`
  - `pipeline.foundation_inputs` + `stage.05_charter_synthesize`
  - `pipeline.foundation_inputs` + `stage.06_project_context_interview`
  - `pipeline.foundation_inputs` + `stage.07_foundation_pack`
  - `pipeline.foundation_inputs` + `stage.10_feature_spec`
- `pipeline capture --preview` validates stdin, caches one typed materialization plan, and prints a deterministic `capture_id`.
- `pipeline capture apply --capture-id <capture-id>` revalidates freshness and applies the cached plan transactionally.
- `pipeline capture` remains the only supported stage-output writer surface. For stage `10`, `pipeline compile` emits model input payload, an external operator or model runner produces the completed `FEATURE_SPEC.md`, and `pipeline capture` materializes that body.
- `needs_project_context` remains an explicit operator-owned handoff:
  - `pipeline capture --stage stage.05_charter_synthesize`
  - `pipeline state set --var needs_project_context=<true|false>`
  - `pipeline resolve`
- Capture freshness is explicit. If route basis is missing, stale, or inactive, re-run `pipeline resolve` before retrying preview or apply.
- `generate` produces planning packets from canonical repo-local `.system/` inputs and supports the fixture-backed execution demo via `execution.demo.packet`.
- `inspect` is the packet proof surface for packet composition and decision evidence.
- `doctor` is the recovery surface for blockers and safe next actions.

## What to expect right now

- The currently shipped binary exposes `pipeline` as the reviewed operator surface alongside `setup`, `generate`, `inspect`, and `doctor`.
- `pipeline` now includes one explicit stage compilation wedge in M2 and one explicit writer wedge in M3 without widening into generic multi-stage compile or run support.
- The documented `pipeline.foundation_inputs` path is `04` capture -> `05` capture -> manual `state set` -> `resolve` -> conditional `06` capture -> `resolve` -> `07` capture -> `10` compile -> external model output -> capture.
- For `stage.10_feature_spec`, raw `pipeline compile` payload is refused as `invalid_capture_input`; capture only accepts a completed `FEATURE_SPEC.md` body.
- `setup` is still a placeholder, but it is part of the supported command surface and help ordering.
- For `generate`, `inspect`, and `doctor` on planning/live packet flows, you may invoke from repo root or a nested directory inside the target git repo. Before `.system/` exists, routing anchors to the enclosing git root.
- For `pipeline`, list/show/resolve/compile/capture/state-set stay inside the approved repo surface and use one shared resolved-route truth.
- `pipeline compile --id <pipeline-id> --stage <stage-id>` consumes the persisted route basis written by `pipeline resolve`; `pipeline compile --explain` is the compile proof surface for that same result.
- `pipeline capture` and `pipeline capture apply` consume the same persisted fresh `route_basis`; they do not silently re-run `pipeline resolve`.
- `pipeline capture` applies transactionally only for `system`-coordinated single-writer flows; arbitrary concurrent external writers are outside the shipped claim.
- If `.system/` is missing, `generate`, `inspect`, and `doctor` refuse or block with a deterministic next safe action.
- Once `.system/` canonical artifacts exist, planning packet generation is supported.
- Execution packets are only supported as fixture-backed demos via `execution.demo.packet`, and live execution is explicitly refused.

## Exact `foundation_inputs` path

```bash
cargo run -p system-cli -- pipeline resolve --id pipeline.foundation_inputs

cat /tmp/CHARTER_INPUTS.yaml \
  | cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.04_charter_inputs

cat /tmp/CHARTER.md \
  | cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.05_charter_synthesize

cargo run -p system-cli -- pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>
cargo run -p system-cli -- pipeline resolve --id pipeline.foundation_inputs

# Only when resolve marks stage.06_project_context_interview active:
cat /tmp/PROJECT_CONTEXT.md \
  | cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.06_project_context_interview

cargo run -p system-cli -- pipeline resolve --id pipeline.foundation_inputs

cat /tmp/FOUNDATION_PACK.blocks.txt \
  | cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.07_foundation_pack

cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec

# External step outside `system`: use the compile payload to produce /tmp/FEATURE_SPEC.md
cat /tmp/FEATURE_SPEC.md \
  | cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.10_feature_spec
```
