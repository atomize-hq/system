# System Docs

## Status

These docs primarily describe the **legacy Python harness path that works today** in this repo.

- They are useful reference material for the current scaffold.
- They are **not** the reviewed reduced-v1 target architecture.
- The reviewed v1 direction is a Rust-first context compiler CLI over live project + feature artifacts.
- The next artifact is the implementation plan at [PLAN.md](/Users/spensermcconnell/__Active_Code/system/PLAN.md).
- The authoritative repo-surface contract lives at [docs/contracts/C-01-approved-repo-surface.md](/Users/spensermcconnell/__Active_Code/system/docs/contracts/C-01-approved-repo-surface.md).

These docs describe **what is working today** in this repository’s human‑in‑the‑loop prompt pipeline.

- The harness **does not call any LLM APIs**.
- You **copy/paste** compiled prompts into your LLM of choice.
- You paste model output back into the harness; it writes files under `artifacts/` (and some repo‑root docs).

## Start here

If you want the reviewed v1 decisions first, read the design doc at [spensermcconnell-main-design-20260403-110234.md](/Users/spensermcconnell/.gstack/projects/system/spensermcconnell-main-design-20260403-110234.md) and the [C-01 contract](docs/contracts/C-01-approved-repo-surface.md) before using the legacy harness references below.

- [Harness](HARNESS.md)
- [System model](SYSTEM_MODEL.md)
- [Glossary](GLOSSARY.md)
- [Stage reference](stages/README.md)

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
