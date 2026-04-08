# Mechanism: Test Mode (synthetic Charter)

## What it is

Test Mode lets you run the Charter stage without answering interview questions.
The model generates a lifelike synthetic scenario (typically Greenfield) and outputs a complete `CHARTER.md`.

## Why it exists

- Iterate quickly on the pipeline and downstream stages
- Remove human bottlenecks during prompt development

## How it works today

Two common patterns:

### Pattern A: `--test-mode true` variable

The harness injects `test_mode: True` into run variables, and the Charter stage selects:

- `charter_gen_directive_TEST_MODE.md`

### Pattern B: Separate test-stage file

A dedicated test stage points directly to the test directive.

## How to use it

```bash
python3 tools/harness.py run stage.05_charter_interview --test-mode true
```

## What to expect

- No questions asked
- Output includes a subtle marker:
  - `<!-- TEST MODE: synthetic charter for pipeline iteration -->`
- Output is only the final `CHARTER.md` content (no commentary)

## Do / Don’t

✅ Do:
- use Test Mode to validate the pipeline wiring and output parsing
- reset state when switching between test and real runs

❌ Don’t:
- treat the synthetic charter as a real project record
