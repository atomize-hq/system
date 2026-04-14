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
cargo run -p system-cli -- pipeline state set --id pipeline.foundation_inputs --var needs_project_context=true
```

For the reviewed operator-surface contract baseline, see [`C-09`](contracts/pipeline-operator-surface-and-id-resolution.md).

## Current command meanings

- `setup` is the reserved setup-first entrypoint for the reduced-v1 trust flow. It is still a placeholder and not yet a real Rust setup flow.
- `pipeline` owns `list`, `show`, `resolve`, `compile`, and `state set` for the reviewed wedge.
- `pipeline compile --id <pipeline-id> --stage <stage-id>` is the supported M2 compile surface for the first bounded target: `pipeline.foundation_inputs` + `stage.10_feature_spec`.
- Plain `pipeline compile` success is payload-only stdout. `pipeline compile --explain` is proof-only stdout.
- Compile freshness is explicit. If route basis is missing, stale, or inactive, re-run `pipeline resolve` before retrying compile.
- `generate` produces planning packets from canonical repo-local `.system/` inputs and supports the fixture-backed execution demo via `execution.demo.packet`.
- `inspect` is the packet proof surface for packet composition and decision evidence.
- `doctor` is the recovery surface for blockers and safe next actions.

## What to expect right now

- The currently shipped binary exposes `pipeline` as the reviewed operator surface alongside `setup`, `generate`, `inspect`, and `doctor`.
- `pipeline` now includes one explicit stage compilation wedge in M2 without widening into generic multi-stage compile support.
- `setup` is still a placeholder, but it is part of the supported command surface and help ordering.
- For `generate`, `inspect`, and `doctor` on planning/live packet flows, you may invoke from repo root or a nested directory inside the target git repo. Before `.system/` exists, routing anchors to the enclosing git root.
- For `pipeline`, list/show/resolve/compile/state-set stay inside the approved repo surface and use one shared resolved-route truth.
- `pipeline compile --id <pipeline-id> --stage <stage-id>` consumes the persisted route basis written by `pipeline resolve`; `pipeline compile --explain` is the compile proof surface for that same result.
- If `.system/` is missing, `generate`, `inspect`, and `doctor` refuse or block with a deterministic next safe action.
- Once `.system/` canonical artifacts exist, planning packet generation is supported.
- Execution packets are only supported as fixture-backed demos via `execution.demo.packet`, and live execution is explicitly refused.
