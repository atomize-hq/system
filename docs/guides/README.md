# Guides

## Legacy Status

These guides document the human-in-the-loop Python harness workflow that still exists in the repo.

- They are legacy reference guides.
- They do not define the reviewed Rust-first reduced-v1 product path.
- Do not treat the Python workflow described here as the long-term supported implementation direction.

This folder is meant to be dropped into your system `docs/` directory:

- `docs/guides/` — workflow how-tos and “how the system works” explanations.
- `docs/guides/workflows/` — step-by-step command recipes.
- `docs/guides/mechanisms/` — one page per mechanism: what it is, why it exists, how to use it, and how to create your own.

> **Scope note (important):** These guides describe what is intended to work **today** in the human-in-the-loop workflow:
> - compile prompts to `dist/`
> - copy/paste into an LLM
> - paste output back into the harness to write artifacts
>
> If your repo has both `pipeline.yaml` (single-pipeline) **and** `pipelines/*.yaml` (multi-pipeline), these guides cover both.
> Where commands differ, you’ll see “single-pipeline” vs “multi-pipeline” sections.

## Start here

- **Workflow quickstarts:** see `workflows/README.md`
- **Mechanism references:** see `mechanisms/README.md`

## Minimal prerequisites

- Python 3.x available
- Run from the system root (the folder that contains `tools/`, `core/`, `artifacts/`, `dist/`)

## Quick health checks

```bash
python3 tools/harness.py --help
python3 tools/harness.py list
python3 tools/harness.py overlays
```

If your harness supports multiple pipeline files, you’ll typically see a `--pipeline` flag in `--help`.

_Last updated: 2026-01-31T21:00:45Z_
