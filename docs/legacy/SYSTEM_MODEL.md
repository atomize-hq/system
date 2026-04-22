# Legacy System Model

## Legacy Scope

This document explains the mechanisms that work **today** in the legacy Python harness path.

- It is accurate as a description of the current scaffold.
- It is **not** the reviewed Rust-first target architecture for reduced v1.
- The target v1 is a Rust CLI that resolves planning packets over live project + feature artifacts, with fixture-backed execution demos only.

This doc explains the **mechanisms that work today**: pipeline config, stages, includes, profiles, overlays, work levels, and outputs.

If you want stage-by-stage behavior, see [`docs/legacy/stages/`](stages/README.md).

## Pipeline (`pipeline.yaml`)

- Defines pipeline defaults (`runner`, `profile`, and a few booleans)
- Defines stage order
- Supports limited routing:
  - `sets:` (values the harness prompts you to set after capture)
  - optional `activation:` (conditions to skip stages)

The harness reads stage ordering from `pipeline.yaml` and uses each stage’s markdown file for detailed behavior.

### `sets:` (post-capture prompts)

A pipeline stage may declare:

```yaml
sets:
  - needs_project_context
```

After you capture that stage’s output, the harness will prompt you to set the value and store it in `artifacts/_harness_state.yaml`.

### Activation conditions

A stage can be gated by `activation.when.any` / `activation.when.all` expressions like:
- `variables.needs_project_context == true`

The legacy harness reference supported simple equality checks against booleans, numbers, and quoted strings.
The reduced-v1 Rust pipeline contract is intentionally narrower: only boolean equality in the form `variables.<name> == true|false` is supported, and string or numeric activation literals are refused at load time.

## Stage files (`core/stages/*.md`)

A stage file contains YAML front matter and an optional body.
The harness uses the front matter to build the compiled prompt.

Key front matter fields used today:

- `id`, `title`, `description`
- `work_level`: `L0` | `L1` | `L2` | `L3`
- `includes`: list of markdown/yaml files to inline into the compiled prompt
- `inputs`:
  - `library`: directives/templates the model must follow
  - `artifacts`: upstream generated artifacts to include (if present)
  - `variables`: variables exposed in the “Run Variables” section
- `outputs`:
  - `artifacts`: files written under `system/artifacts/...`
  - `repo_files`: canonical files written into `${repo_root}/...`
- `gating`: notes/fail_on (used as guidance in the compiled prompt)

## Includes

Includes are markdown/yaml files inlined into the compiled prompt, such as:
- `core/rules/*.md`
- `runners/<runner>.md`
- `profiles/<profile>/{profile.yaml,commands.yaml,conventions.md}`

Includes support simple `${var}` substitution (e.g., `${profile}` and `${runner}`).

## Profiles (`profiles/<id>/`)

Profiles keep the core system language/tooling agnostic.

A profile directory contains:
- `profile.yaml`: metadata about the stack and default project layout
- `commands.yaml`: canonical command strings for gate names (lint/test/typecheck/etc.)
- `conventions.md`: naming/style/path conventions for that stack

Stages include profile files so the model can refer to tool commands **without** hardcoding commands into core rules.

## Runners (`runners/<id>.md`)

A runner is guidance for how an execution-capable agent should behave (e.g., Codex CLI).

Runners are informational today: the harness does not execute commands; it only compiles prompts and writes files.

## Overlays (`core/overlays/*.md`)

Overlays are optional policy modules that can be injected into compiled prompts.

How overlays are included today:
- `--overlays overlay_a,overlay_b` on `compile`/`run`
- Automatic: if `enable_complexity` is true, the harness includes `complexity_assessment` when present

## Work levels + scoped rules

Stages declare a `work_level`:
- L0 Program
- L1 Project/Planning
- L2 Slice Execution
- L3 Quality Gate & Merge

Included markdown can contain scoped blocks:

```md
<!-- SCOPE: L2,L3 -->
This content only appears for L2/L3 stages.
<!-- END_SCOPE -->
```

The harness filters scoped blocks during compile, so early stages stay concise while L2/L3 stages receive stricter execution/merge discipline.

## Outputs: artifacts vs repo files

- **Artifacts** are written under `system/artifacts/…` and represent pipeline outputs.
- **Repo files** are written to `${repo_root}/…` as part of the legacy harness behavior.

Legacy repo-file behavior:
- `CHARTER.md` is written to `${repo_root}/CHARTER.md` and `artifacts/charter/CHARTER.md`
- `ENVIRONMENT_INVENTORY.md` is written to `${repo_root}/ENVIRONMENT_INVENTORY.md` and `artifacts/foundation/ENVIRONMENT_INVENTORY.md`

Current reduced-v1 product note:
- the shipped baseline canonical environment-inventory path is `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`, not the repo root

## Model output contracts (what the harness can parse)

- **Single-file stages**: output only the final document markdown.
- **Multi-file stages** (Foundation Pack): output `--- FILE: <path> ---` blocks exactly.

See [Harness](HARNESS.md) for the exact format.

## Test mode (optional)

Some setups support a harness flag (e.g., `--test-mode true`) to swap the Charter directive to a test-mode variant that generates a realistic synthetic charter without questions. If enabled, it is visible in “Run Variables” as `test_mode: True`.
