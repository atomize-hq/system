# Legacy stage.07_foundation_pack — Foundation Pack Synthesis

## Purpose

Synthesizes a project-specific **Foundation Pack** from:
- `CHARTER.md` (posture/guardrails)
- optional `PROJECT_CONTEXT.md` (reality snapshot)

The Foundation Pack turns posture + facts into concrete, reusable planning defaults for all later work.

- **Work level:** L1 (Project/Planning)
- **Interaction style:** synthesis (zero-interview default)

## Inputs

### Library inputs (required)
- `core/library/foundation_pack/foundation_pack_directive.md`
- `core/library/foundation_pack/FOUNDATION_STRATEGY.md.tmpl`
- `core/library/foundation_pack/TECH_ARCH_BRIEF.md.tmpl`
- `core/library/foundation_pack/TEST_STRATEGY_BRIEF.md.tmpl`
- `core/library/foundation_pack/QUALITY_GATES_SPEC.md.tmpl`
- `core/library/foundation_pack/quality_gates.yaml.tmpl`
- `core/library/environment_inventory/environment_inventory_directive.md`
- `core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl`

### Artifact inputs
- `artifacts/charter/CHARTER.md` (required)
- `artifacts/project_context/PROJECT_CONTEXT.md` (optional)

## Outputs

### Artifacts (all written)
- `artifacts/foundation/FOUNDATION_STRATEGY.md`
- `artifacts/foundation/TECH_ARCH_BRIEF.md`
- `artifacts/foundation/TEST_STRATEGY_BRIEF.md`
- `artifacts/foundation/QUALITY_GATES_SPEC.md`
- `artifacts/foundation/quality_gates.yaml`
- `artifacts/foundation/ENVIRONMENT_INVENTORY.md`

### Repo files (legacy harness repo-root outputs)
- `${repo_root}/ENVIRONMENT_INVENTORY.md` (required)

> Legacy note: this stage treated `${repo_root}/ENVIRONMENT_INVENTORY.md` as the repo-root store-of-record.
> Current reduced-v1 product docs do not treat the repo root as the shipped canonical home; the baseline canonical path is `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`.

## Required model output format (multi-file FILE blocks)

This stage declares multiple outputs. The model must output **only** file blocks like:

```md
--- FILE: artifacts/foundation/FOUNDATION_STRATEGY.md ---
<contents>

--- FILE: artifacts/foundation/TECH_ARCH_BRIEF.md ---
<contents>

--- FILE: artifacts/foundation/TEST_STRATEGY_BRIEF.md ---
<contents>

--- FILE: artifacts/foundation/QUALITY_GATES_SPEC.md ---
<contents>

--- FILE: artifacts/foundation/quality_gates.yaml ---
<contents>

--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---
<contents>
```

No preamble, no epilogue, and no code fences.

## How to run

```bash
./tools/harness.sh compile --only stage.07_foundation_pack
./tools/harness.sh run stage.07_foundation_pack
```

## Expected model behavior

- Default mode: do not ask questions.
- It may ask up to a small number of clarifying questions only if a missing fact would materially change:
  - required quality gates
  - architecture direction
  - whether back-compat/migrations apply

## What is “machine testable” today?

- This stage produces a **machine-readable** `quality_gates.yaml` artifact.
- The harness **does not execute gates**; it only compiles prompts and writes artifacts.

## Harness behavior for legacy repo-root ENVIRONMENT_INVENTORY

If the model does not emit a separate repo-file block for `${repo_root}/ENVIRONMENT_INVENTORY.md`, the harness can copy
the artifact output with the same filename into the repo-root location.

## Common gotchas

- If the model forgets the `--- FILE:` wrapper format, capture will fail.
- Keep quality gates **explicit** (exit code / deterministic criteria), even though execution automation is not wired yet.
