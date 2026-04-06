# Mechanism: Overlays

## What it is

An **overlay** is an optional markdown module that adds guidance or constraints.
Overlays are “layers” that can be included per run without changing core stages.

Overlays commonly live under `core/overlays/`. Some systems organize them in subdirectories:

- `core/overlays/task/`
- `core/overlays/sprint/`
- `core/overlays/release/`
- `core/overlays/quality/`

## Why it exists

- Keep the core system small, stable, and language-agnostic
- Add “policy lenses” only when needed:
  - bounded sprints and lanes
  - research/discovery task requirements
  - stricter gates in regulated contexts
  - complexity guidance

## How it works today

- The harness may include overlays via a flag (e.g., `--overlays`)
- Some overlays may be auto-included based on variables (e.g., `enable_complexity`)

## Creating an overlay

1) Choose a scope and directory (e.g., `task/`, `sprint/`)
2) Create the file, e.g.:

```bash
mkdir -p core/overlays/task
touch core/overlays/task/research_discovery.md
```

3) Write:
- goal
- when it applies
- constraints and expectations
- how it affects outputs or gating (if at all)

4) Include it when running:

```bash
python3 tools/harness.py run stage.07_foundation_pack --overlays task/research_discovery
```

## Do / Don’t

✅ Do:
- keep overlays orthogonal (one major concern each)
- use overlays to add heuristics and defaults
- keep overlays stack-agnostic

❌ Don’t:
- override core output contracts in an overlay
- create “must always include” overlays (if it’s mandatory, put it in core rules or stage includes)
