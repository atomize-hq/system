# Mechanism: Artifacts vs Repo Files

## What it is

Stages can write outputs to two places:

1) **Artifacts** (pipeline-managed)
- stored under `artifacts/`
- used as inputs to later stages
- easy for harness to locate and version per run

2) **Repo files** (project-facing canonical docs)
- written into the repo/project root (or other repo paths)
- intended as durable records for humans

## Why it exists

- `artifacts/` makes the pipeline deterministic and repeatable
- repo files make outputs useful outside the pipeline

## How it works today

Stage front matter may declare:

- `outputs.artifacts: [ {path: artifacts/...} ]`
- `outputs.repo_files: [ {path: ./SOME_DOC.md} ]`

During capture:
- artifacts are written from model output
- repo files are either written directly or copied from artifacts (depending on harness behavior)

## Recommended practice

- Use `artifacts/` as the pipeline truth source
- Write repo copies for “canonical docs” like:
  - `CHARTER.md`
  - `QUALITY_GATES_SPEC.md`
  - `ENVIRONMENT_INVENTORY.md` (see dedicated guide)

## Do / Don’t

✅ Do:
- keep artifact and repo copies in sync where required
- prefer stable paths so downstream stages can reference them

❌ Don’t:
- treat repo files as pipeline inputs unless you also guarantee they exist
