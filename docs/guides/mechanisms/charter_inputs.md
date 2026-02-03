# Mechanism: Charter Inputs (Dev/Test Path)

## What it is

The **Charter Inputs** mechanism is a development/testing-friendly alternative to the multi-turn Charter interview.

Instead of “ask one question at a time,” you generate a single deterministic input file:

- `artifacts/charter/CHARTER_INPUTS.yaml`

…and then synthesize the final:

- `artifacts/charter/CHARTER.md`

This keeps the **production UX** (interactive interview) available, while enabling a fast, repeatable, testable loop during system development.

## Why it exists

The interview-based Charter is great for humans, but it’s hard to:

- run in CI or automated checks
- regression-test prompt/template changes
- iterate quickly without copy/paste or multi-turn transcripts

The inputs path turns the Charter into a **single-shot, fixture-driven** step that can be validated without any model.

## How it works today

There are two Charter flows:

### A) Interview (dynamic UX)

- Stage: `stage.05_charter_interview`
- Output: `artifacts/charter/CHARTER.md` (+ `${repo_root}/CHARTER.md`)

This is the “real” path for interactive use.

### B) Dev/Test inputs → synthesize (fast iteration)

- Stage: `stage.04_charter_inputs` → writes `artifacts/charter/CHARTER_INPUTS.yaml`
- Stage: `stage.05_charter_synthesize` → reads inputs + writes `artifacts/charter/CHARTER.md`
- Pipeline: `pipelines/foundation_inputs.yaml`

This is the path you use while developing the system, because it supports deterministic fixtures.

## Validation (no model required)

Use the validator to check structure and catch typos before synthesis:

```bash
python3 tools/check_charter_inputs.py --strict tools/fixtures/charter_inputs
python3 tools/check_charter_inputs.py --strict artifacts/charter/CHARTER_INPUTS.yaml
```

## How to use it (recommended dev loop)

1) Pick a fixture from `tools/fixtures/charter_inputs/` (or create a new one).
2) Copy it to `artifacts/charter/CHARTER_INPUTS.yaml` (or generate via `stage.04_charter_inputs`).
3) Validate with `tools/check_charter_inputs.py`.
4) Run `stage.05_charter_synthesize` to generate `CHARTER.md`.
5) Repeat as you evolve directives/templates.

## Do / Don’t

✅ Do:
- treat `CHARTER_INPUTS.yaml` as a regression-test fixture format
- keep fixtures small and representative (2–3 is usually enough)
- validate inputs before synthesis

❌ Don’t:
- remove the interview path (it’s the best UX for real users)
- let fixtures drift without running the validator

