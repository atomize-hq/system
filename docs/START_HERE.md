# Start Here (Supported, Rust-first)

Reduced v1 is a **Rust-first context compiler CLI**.

The legacy Python harness still exists in this repo as **frozen reference material**. It is useful for understanding the old scaffold, but it is not the supported v1 product path.

## What is supported in reduced v1

- **Canonical inputs live in repo-local `.system/`**.
  - Required:
    - `.system/charter/CHARTER.md`
    - `.system/feature_spec/FEATURE_SPEC.md`
  - Optional:
    - `.system/project_context/PROJECT_CONTEXT.md`
  - Non-canonical runtime state may also live under `.system/`, but it is not part of the canonical input set.
- **Planning packet generation** is supported from canonical repo-local `.system/`.
- **The reviewed command surface adds `pipeline`** for `list`, `show`, `resolve`, `compile`, `capture`, and `state set` over route truth, one explicit stage compilation wedge, one explicit writer wedge, and narrow route-state mutation.
  - The operator-surface contract baseline is [`C-09`](contracts/pipeline-operator-surface-and-id-resolution.md).
- **`pipeline compile --id <pipeline-id> --stage <stage-id>`** is the supported M2 compile entrypoint for the first bounded target.
  - Plain `pipeline compile` success is payload-only stdout.
  - `pipeline compile --explain` is proof-only stdout.
  - If compile refuses because route basis is missing, stale, or inactive, re-run `pipeline resolve` and retry.
- **`pipeline capture --id <pipeline-id> --stage <stage-id>`** is the supported M3 / M3.5 writer entrypoint for the bounded capture wedge.
  - `pipeline capture --preview` validates stdin, caches one typed materialization plan, and returns `CAPTURE ID`.
  - `pipeline capture apply --capture-id <capture-id>` revalidates freshness and applies the cached plan transactionally.
  - For `pipeline.foundation_inputs`, the supported capture stages are `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, and `stage.10_feature_spec`.
  - `pipeline compile` does not write files. For stage `10`, compile emits model input payload, an external operator or model runner produces the completed `FEATURE_SPEC.md`, and `pipeline capture` materializes that body.
  - If capture refuses because route basis is missing, stale, or inactive, re-run `pipeline resolve` and retry.
- **Execution packet generation** is fixture-backed demo only via `execution.demo.packet`; live execution is explicitly refused.
- **`inspect`** is the packet proof surface.
- **`doctor`** is the recovery surface, it explains blockers and safe next actions.
- **`setup`** is still a placeholder entrypoint and is not yet a real Rust setup flow.

## Documented `foundation_inputs` operator path

The first complete supported `pipeline.foundation_inputs` path is:

```bash
system pipeline resolve --id pipeline.foundation_inputs

cat /tmp/CHARTER_INPUTS.yaml \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.04_charter_inputs

cat /tmp/CHARTER.md \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.05_charter_synthesize

system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>
system pipeline resolve --id pipeline.foundation_inputs

# Only when resolve marks stage.06_project_context_interview active:
cat /tmp/PROJECT_CONTEXT.md \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.06_project_context_interview

system pipeline resolve --id pipeline.foundation_inputs

cat /tmp/FOUNDATION_PACK.blocks.txt \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.07_foundation_pack

system pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec

# External step outside `system`: use the compile payload to produce /tmp/FEATURE_SPEC.md
cat /tmp/FEATURE_SPEC.md \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.10_feature_spec
```

Important boundaries:

- `needs_project_context` stays manual and exact. Capture does not auto-set it.
- `pipeline capture` remains the only stage-output writer surface.
- Stage `10` capture writes `artifacts/feature_spec/FEATURE_SPEC.md`; it does not promote into canonical `.system/feature_spec/FEATURE_SPEC.md`.
- Transactional apply remains scoped to `system`-coordinated single-writer flows.

## How to navigate this repo

- Supported architecture + cutover plan: [`PLAN.md`](../PLAN.md)
- CLI interaction contract: [`DESIGN.md`](../DESIGN.md)
- Operator-facing vocabulary: [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md)
- Front door and command hierarchy: [`docs/CLI_COMMAND_HIERARCHY.md`](CLI_COMMAND_HIERARCHY.md)
- Tone rules for docs/help/runtime: [`docs/CLI_TONE_RULES.md`](CLI_TONE_RULES.md)
- Output anatomy for success, refusal, proof, and recovery: [`docs/CLI_OUTPUT_ANATOMY.md`](CLI_OUTPUT_ANATOMY.md)
- Operator journey and conformance review: [`docs/CLI_OPERATOR_JOURNEY.md`](CLI_OPERATOR_JOURNEY.md)
- Contracts (the authoritative truth): [`docs/contracts/`](contracts/)
- CLI command surface and wording: [`C-02`](contracts/C-02-rust-workspace-and-cli-command-surface.md)
- Pipeline operator surface and ID resolution: [`C-09`](contracts/pipeline-operator-surface-and-id-resolution.md)
- Canonical `.system/` manifest + freshness: [`C-03`](contracts/C-03-canonical-artifact-manifest-contract.md)
- Refusal + doctor blockers taxonomy: [`C-04`](contracts/C-04-resolver-result-and-doctor-blockers.md)
- Proof surfaces (markdown/json/inspect ordering): [`C-05`](contracts/C-05-renderer-and-proof-surfaces.md)
- Fixture-backed execution demo boundary: [`C-06`](contracts/C-06-fixture-execution-demo-boundary.md)
- Docs/help parity and conformance rails for the reviewed `pipeline` subset: [`C-11`](contracts/pipeline-proof-corpus-and-docs-cutover.md)
- Capture preview/apply, cache, mirror, and rollback rules: [`C-12`](contracts/pipeline-capture-preview-and-apply.md)

Legacy reference docs (Python harness, stage reference, old workflow guides) live under [`docs/legacy/`](legacy/README.md).
