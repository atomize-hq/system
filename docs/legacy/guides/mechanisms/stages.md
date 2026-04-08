# Mechanism: Stages

## What it is

A **stage** is a Markdown file with YAML front matter that describes:

- its purpose
- included rules/runner/profile content
- required inputs (library templates, artifacts)
- outputs it must produce
- gating/failure behavior
- optional activation conditions

Stages compile into prompts placed in `dist/`.

## Why it exists

- Makes each step independently understandable and testable
- Enables modular pipelines
- Keeps project-specific stack behavior in profiles, not in core

## How it works today (compile → LLM → capture)

1) The harness reads stage front matter
2) It resolves `includes:` (rules, runner, profile conventions)
3) It inlines `inputs:` content (directives, templates, prior artifacts)
4) It writes a compiled prompt to `dist/<stage>.md`
5) You paste `dist/<stage>.md` into an LLM
6) You paste the LLM output back into the harness (capture)
7) The harness writes declared outputs under `artifacts/` (and optional repo-root files)

## Common stage front matter fields

- `id`: stable stage identifier
- `title`, `description`
- `work_level`: L0/L1/L2/L3 (used for scoped rule blocks)
- `includes`: markdown files to inline (rules, runner, profile)
- `inputs.library`: directive + templates used to generate output
- `inputs.artifacts`: upstream artifacts required (Charter, Context, etc.)
- `outputs.artifacts`: files written under `artifacts/`
- `outputs.repo_files`: files written to the repo/project root (optional)
- `activation`: a small “when to run” rule based on variables
- `sets`: variables to prompt for after capture (e.g., `needs_project_context`)
- `gating`: notes/requirements (e.g., “outputs must be machine-testable”)

## How to create a new stage

1) Copy an existing stage file from `core/stages/`
2) Decide the output contract:
   - single doc, OR
   - multi-file wrapper blocks (`--- FILE: ... ---`)
3) Add templates + directive under `core/library/<topic>/`
4) Declare them in `inputs.library`
5) Declare outputs in `outputs.artifacts` (and optional `outputs.repo_files`)
6) Add `gating.notes` that enforce output discipline

## Do / Don’t

✅ Do:
- keep stage bodies minimal; put the “how” in directives + templates
- require upstream artifacts if they materially affect correctness
- define outputs so the harness can write them deterministically

❌ Don’t:
- embed stack commands in stage logic (profiles own commands)
- allow vague “free-form” outputs when artifacts must be parsed
