# System Docs

## Start Here (Supported)

Reduced v1 is a **Rust-first context compiler CLI**.

- Start here: [`docs/START_HERE.md`](START_HERE.md)
- Command surface: [`docs/SUPPORTED_COMMANDS.md`](SUPPORTED_COMMANDS.md)
- Implementation plan: [`PLAN.md`](../PLAN.md)
- Contracts (source of truth): [`docs/contracts/`](contracts/)
- Vision (broader, non-binding): [`docs/VISION.md`](VISION.md)
- Glossary: [`docs/GLOSSARY.md`](GLOSSARY.md)

## Legacy (Reference Only)

The Python harness and its stage-based workflow remain in the repo as frozen reference material until cutover.

All legacy reference docs are under:

- [`docs/legacy/`](legacy/README.md)

## What this system is

A **language‑agnostic prompt pipeline** that generates structured project artifacts using:

- a selected **profile** (stack pack: commands + conventions)
- a selected **runner** (how an execution agent should behave)
- optional **overlays** (extra policy modules)
- **work levels** (L0–L3) to scope strict rules
- a **harness** that compiles stage prompts into `dist/` and writes captured outputs into `artifacts/`

## Directory map (high level)

- `pipeline.yaml` — default stage order, defaults, and a small amount of routing (`sets`, `activation`)
- `pipelines/` — additional pipeline entrypoints (select with `--pipeline`)
- `core/` — rules, stages, overlays, and library directives/templates
- `profiles/` — stack packs (`profile.yaml`, `commands.yaml`, `conventions.md`)
- `runners/` — runner guidance modules
- `tools/harness.py` — compile/capture engine
- `dist/` — compiled stage prompts (generated)
- `artifacts/` — captured outputs (generated)

## Stages implemented today

These stages have working front matter + library templates/directives and are runnable via the harness:

- [`stage.00_base`](stages/stage.00_base.md)
- [`stage.05_charter_interview`](stages/stage.05_charter_interview.md)
- [`stage.06_project_context_interview`](stages/stage.06_project_context_interview.md)
- [`stage.07_foundation_pack`](stages/stage.07_foundation_pack.md)
- [`stage.01_release_plan`](stages/stage.01_release_plan.md)
- [`stage.02_sprint_plan`](stages/stage.02_sprint_plan.md)
- [`stage.10_feature_spec`](stages/stage.10_feature_spec.md)

Other stage files may exist in `core/stages/` but are currently scaffolds (empty/placeholder) and are not documented here.

## Quick start

From the `system/` directory:

```bash
./tools/harness.sh list
./tools/harness.sh compile --only stage.00_base
./tools/harness.sh run stage.00_base
```

Then continue stage‑by‑stage (see the stage reference for the recommended order and expected model output format).

To run an alternate pipeline:

```bash
./tools/harness.sh list --pipeline pipelines/release.yaml
./tools/harness.sh run stage.01_release_plan --pipeline pipelines/release.yaml --release-id release-001 --release-type minor
```
