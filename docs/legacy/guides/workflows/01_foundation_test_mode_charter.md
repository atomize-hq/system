# Workflow: Foundation run (TEST MODE Charter)

Use this when you want to quickly iterate on prompts and downstream stages without answering interview questions.
The Charter stage will generate a **synthetic but lifelike** Greenfield project scenario.

## What you get

- `artifacts/base/BASE_CONTEXT.md` (+ optional `BASE_CONTEXT.md` at repo root)
- `artifacts/charter/CHARTER.md` (+ optional `CHARTER.md` at repo root)
- optional `artifacts/project_context/PROJECT_CONTEXT.md`
- `artifacts/foundation/*` (foundation pack, quality gates spec, environment inventory)

## 0) Start clean (recommended)

```bash
rm -f artifacts/_harness_state.yaml
```

## 1) Run Base

### Single-pipeline (pipeline.yaml)

```bash
python3 tools/harness.py run stage.00_base \
  --enable-complexity false
```

### Multi-pipeline (pipelines/foundation.yaml)

If your harness supports `--pipeline`:

```bash
python3 tools/harness.py run --pipeline pipelines/foundation.yaml stage.00_base \
  --enable-complexity false
```

## 2) Run Charter in TEST MODE

```bash
python3 tools/harness.py run stage.05_charter_interview \
  --test-mode true \
  --enable-complexity false
```

Multi-pipeline variant:

```bash
python3 tools/harness.py run --pipeline pipelines/foundation.yaml stage.05_charter_interview \
  --test-mode true \
  --enable-complexity false
```

### If you get prompted: `Set needs_project_context? [Y/n]:`

This is normal if the stage declares `sets: [needs_project_context]`.

- Answer **Y** if anything planning-critical is unknown (prod? contracts? back-compat? migration? integrations? environments?)
- Answer **n** if the Charter is complete enough for planning.

## 3) Optional: Run Project Context

Only if `needs_project_context` is true (or if you want more concrete facts on repo reality):

```bash
python3 tools/harness.py run stage.06_project_context_interview \
  --enable-complexity false
```

Multi-pipeline variant:

```bash
python3 tools/harness.py run --pipeline pipelines/foundation.yaml stage.06_project_context_interview \
  --enable-complexity false
```

## 4) Run Foundation Pack Synthesis

```bash
python3 tools/harness.py run stage.07_foundation_pack \
  --enable-complexity false
```

Multi-pipeline variant:

```bash
python3 tools/harness.py run --pipeline pipelines/foundation.yaml stage.07_foundation_pack \
  --enable-complexity false
```

## Outputs to verify

```bash
ls -1 artifacts/base/BASE_CONTEXT.md
ls -1 artifacts/charter/CHARTER.md
ls -1 artifacts/foundation/
```

If your stages declare repo-root outputs too, you may also have:

```bash
ls -1 BASE_CONTEXT.md CHARTER.md
```
