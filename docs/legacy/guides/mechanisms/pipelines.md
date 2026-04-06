# Mechanism: Pipelines

## What it is

A **pipeline** is an ordered list of stages with defaults (runner, profile, flags). It defines *what to run* and in what sequence.

Pipelines are typically YAML files (e.g., `pipeline.yaml` or `pipelines/foundation.yaml`).

## Why it exists

- Separate “what to do” (pipeline) from “how to do it” (stages, profiles, runners).
- Support multiple flows without cramming everything into one mega-pipeline:
  - foundation
  - release planning
  - sprint planning
  - feature delivery / slice execution

## How it works today

- The harness reads one pipeline file and uses the `stages:` list to:
  - compile prompts into `dist/`
  - capture outputs back into `artifacts/`

Two common modes:

### Single-pipeline
- `pipeline.yaml` at the system root is the default.

### Multi-pipeline
- multiple pipeline files under `pipelines/`
- harness supports selecting a pipeline file (often `--pipeline <path>`)

## How to create a new pipeline

1) Copy an existing pipeline file
2) Set:
   - `id`, `version`, `title`, `description`
   - `defaults.runner`
   - `defaults.profile`
3) Add `stages:` entries:
   - `id`
   - `file` (path to stage markdown)

## Do / Don’t

✅ Do:
- keep pipelines small and purpose-specific
- reuse the same stages across pipelines when possible

❌ Don’t:
- put stage logic directly into pipelines
- hardcode stack commands in pipelines (profiles own commands)
