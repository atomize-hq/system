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
cargo run -p system-cli -- setup init
cargo run -p system-cli -- setup refresh
cargo run -p system-cli -- author charter
cargo run -p system-cli -- author charter --validate --from-inputs /tmp/CHARTER_INPUTS.yaml
cargo run -p system-cli -- author charter --from-inputs /tmp/CHARTER_INPUTS.yaml
cargo run -p system-cli -- author project-context --from-inputs /tmp/PROJECT_CONTEXT_INPUTS.yaml
cargo run -p system-cli -- author project-context
cargo run -p system-cli -- author environment-inventory
cargo run -p system-cli -- setup refresh --rewrite
cargo run -p system-cli -- setup refresh --reset-state
cargo run -p system-cli -- pipeline --help
cargo run -p system-cli -- pipeline handoff --help
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec --explain
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.04_charter_inputs
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.05_charter_synthesize
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.06_project_context_interview
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.07_foundation_pack --preview
cargo run -p system-cli -- pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec
cargo run -p system-cli -- pipeline capture --id pipeline.foundation_inputs --stage stage.10_feature_spec < /tmp/FEATURE_SPEC.md
cargo run -p system-cli -- pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer
cargo run -p system-cli -- pipeline capture apply --capture-id <capture-id>
cargo run -p system-cli -- generate
cargo run -p system-cli -- inspect
cargo run -p system-cli -- doctor
cargo run -p system-cli -- doctor --json
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
cargo run -p system-cli -- pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer
cargo run -p system-cli -- pipeline capture apply --capture-id <capture-id>
```

For the reviewed operator-surface contract baseline, see [`C-09`](contracts/pipeline-operator-surface-and-id-resolution.md).

## Current command meanings

- `setup` is the durable setup-family term for the reduced-v1 trust flow.
- Bare `system setup` routes to `setup init` when canonical `.system/` truth is absent or invalid; otherwise it routes to `setup refresh`.
- `setup init` is the concrete first-run subcommand name. Use it when you need to name the first-run path explicitly.
- `setup refresh` preserves canonical files by default while refreshing setup-owned posture.
- `setup refresh --rewrite` rewrites only the setup-owned starter files:
  - `.system/charter/CHARTER.md`
  - `.system/project_context/PROJECT_CONTEXT.md`
  - `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
- `setup refresh --reset-state` resets only `.system/state/**`.
- The shipped starter templates are scaffolding only. `setup` establishes the baseline file set; `doctor` owns baseline readiness and reports `SCAFFOLDED`, `PARTIAL_BASELINE`, `INVALID_BASELINE`, or `BASELINE_COMPLETE`.
- `doctor` checklist lines include the artifact label, canonical path, status, and exact author command.
- `FEATURE_SPEC.md` stays off the setup/bootstrap path and off the baseline-readiness path. It remains a packet-stage artifact.
- `author` owns canonical content authoring for setup-created starter truth.
- `author` is the baseline authoring surface.
- `system author charter` is the human-guided surface.
- `system author charter --from-inputs <path|->` is the agent and automation surface.
- `system author charter --validate --from-inputs <path|->` is the mutation-free charter preflight surface.
- `system author charter --validate` is invalid without `--from-inputs <path|->`.
- `system author project-context` is the guided project-context authoring surface.
- `system author project-context --from-inputs <path|->` is the agent and automation surface for project-context authoring.
- `system author environment-inventory` authors `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`.
- The repo-owned charter authoring method artifact is `core/library/authoring/charter_authoring_method.md`.
- `doctor --json` is the only machine-readable readiness surface for the installed charter-intake skill.
- `pipeline` owns `list`, `show`, `resolve`, `compile`, `capture`, `handoff emit`, and `state set` for the reviewed wedge.
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
- `pipeline handoff emit --id <pipeline-id> --consumer <consumer-id>` is the supported M5 downstream handoff-emission surface for the first bounded adoption proof.
  - Today it supports only `pipeline.foundation_inputs` + `feature-slice-decomposer`.
  - It emits one derived bundle under `artifacts/handoff/feature_slice/<feature-id>/`.
  - The emitted bundle contains `handoff_manifest.json`, `trust_matrix.md`, `read_allowlist.json`, `scorecard/metadata.json`, and copied bundle-local inputs for the named consumer.
  - The emitted bundle is derived, not canonical; the happy-path consumer is expected to read only the emitted bundle.
- `needs_project_context` remains an explicit operator-owned handoff:
  - `pipeline capture --stage stage.05_charter_synthesize`
  - `pipeline state set --var needs_project_context=<true|false>`
  - `pipeline resolve`
- Capture freshness is explicit. If route basis is missing, stale, or inactive, re-run `pipeline resolve` before retrying preview or apply.
- `generate` produces planning packets from canonical repo-local `.system/` inputs and supports the fixture-backed execution demo via `execution.demo.packet`.
- `inspect` is the packet proof surface for packet composition and decision evidence.
- `doctor` is the recovery surface for blockers and safe next actions.
- `doctor` is also the baseline-readiness surface for `CHARTER`, `PROJECT_CONTEXT`, and `ENVIRONMENT_INVENTORY`.

## Codex packaging and install surfaces

```bash
bash tools/codex/generate.sh
bash tools/codex/install.sh
bash tools/codex/dev-setup.sh
bash tools/codex/relink.sh
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh
```

- `tools/codex/generate.sh` is the handwritten source-to-generated-assets boundary for `.agents/skills/**`.
- `tools/codex/install.sh` installs or reinstalls packaging assets under `~/.codex/skills/`. It does not build or reinstall the Rust binary.
- `tools/codex/dev-setup.sh` is the dev-only symlink flow.
- `tools/codex/relink.sh` is a convenience wrapper around the dev symlink flow.
- `tools/ci/install-smoke.sh` proves install, reinstall, stale-runtime refusal, and install-mode crossover.
- `tools/ci/codex-skill-live-smoke.sh` proves the installed happy path, existing-charter refusal, repo-local runtime override, and outside-git-repo refusal.

## What to expect right now

- The currently shipped binary exposes `author` between `setup` and `pipeline`, alongside `generate`, `inspect`, and `doctor`.
- `pipeline` now includes one explicit stage compilation wedge in M2 and one explicit writer wedge in M3 without widening into generic multi-stage compile or run support.
- The documented `pipeline.foundation_inputs` path is `04` capture -> `05` capture -> manual `state set` -> `resolve` -> conditional `06` capture -> `resolve` -> `07` capture -> `10` compile -> external model output -> capture.
- The downstream adoption extension to that path is explicit: after stage-10 capture, `pipeline handoff emit` writes the derived handoff bundle for `feature-slice-decomposer`.
- For `stage.10_feature_spec`, raw `pipeline compile` payload is refused as `invalid_capture_input`; capture only accepts a completed `FEATURE_SPEC.md` body.
- Stage-10 `artifacts/feature_spec/FEATURE_SPEC.md` remains `external_manual_derived` inside the emitted handoff trust model.
- The public setup family is `setup`, `setup init`, and `setup refresh`; `setup` remains the durable term, while `setup init` names only the concrete first-run subcommand.
- Missing-root, invalid-root, and missing-artifact recovery should point to `system setup` or `system setup refresh`, not to raw file-creation instructions.
- For `generate`, `inspect`, and `doctor` on planning/live packet flows, you may invoke from repo root or a nested directory inside the target git repo. Before `.system/` exists, routing anchors to the enclosing git root.
- For `pipeline`, list/show/resolve/compile/capture/state-set stay inside the approved repo surface and use one shared resolved-route truth.
- `pipeline compile --id <pipeline-id> --stage <stage-id>` consumes the persisted route basis written by `pipeline resolve`; `pipeline compile --explain` is the compile proof surface for that same result.
- `pipeline capture` and `pipeline capture apply` consume the same persisted fresh `route_basis`; they do not silently re-run `pipeline resolve`.
- `pipeline capture` applies transactionally only for `system`-coordinated single-writer flows; arbitrary concurrent external writers are outside the shipped claim.
- If `.system/` is missing, invalid, or missing setup-owned starter files, `generate`, `inspect`, and `doctor` refuse or block with a deterministic next safe action that routes to the setup family.
- Once `.system/` canonical artifacts exist, planning packet generation is supported.
- Fixture guidance for manual QA and proof runs is exact:
  - `tests/fixtures/foundation_flow_demo/` is only for the `pipeline.foundation_inputs` journey-proof flow.
  - Planning packet success examples require a canonical repo fixture with top-level `.system/` inputs; the journey-proof corpus is not that fixture family.
  - If you need to invoke from a nested directory during manual QA, use `tools/qa/prepare_fixture_checkout.sh --fixture-root <path> [--nested-cwd <relative-path>]` instead of ad hoc `cp -R`.
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

cargo run -p system-cli -- pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer
```
