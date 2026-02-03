# Workflow: Foundation run (DEV/TEST Charter Inputs → synthesize Charter)

Use this during system development when you want a **testable** Charter path without a multi-turn interview.

The flow is:

1) Generate `CHARTER_INPUTS.yaml` (a stable fixture)  
2) Synthesize `CHARTER.md` from those inputs  
3) Continue with optional Project Context + Foundation Pack

## 0) Pick the dev/test pipeline

This repo includes a pipeline that inserts the “inputs + synthesize” stages:

```bash
python3 tools/harness.py list --pipeline pipelines/foundation_inputs.yaml
```

## 1) Run Base

```bash
python3 tools/harness.py run --pipeline pipelines/foundation_inputs.yaml stage.00_base
```

## 2) Generate Charter inputs (fixture)

```bash
python3 tools/harness.py run --pipeline pipelines/foundation_inputs.yaml stage.04_charter_inputs
```

This writes:
- `artifacts/charter/CHARTER_INPUTS.yaml`

You can also hand-edit `artifacts/charter/CHARTER_INPUTS.yaml` to make it deterministic for repeated testing.

### Validate inputs (no model required)

Validate the YAML structure (fixtures live under `tools/fixtures/charter_inputs/`):

```bash
python3 tools/check_charter_inputs.py --strict tools/fixtures/charter_inputs
python3 tools/check_charter_inputs.py --strict artifacts/charter/CHARTER_INPUTS.yaml
```

## 3) Synthesize Charter from inputs

```bash
python3 tools/harness.py run --pipeline pipelines/foundation_inputs.yaml stage.05_charter_synthesize
```

This writes:
- `artifacts/charter/CHARTER.md`
- `${repo_root}/CHARTER.md` (canonical copy)

If you’re testing, you can keep `repo_root` pointed at a sandbox folder via `--repo-root`.

## 4) Optional: Run Project Context

Project Context is activated when:
- `needs_project_context` is true, or
- the captured Charter contains unknown markers (tracked as `charter_gaps_detected`)

Run it explicitly:

```bash
python3 tools/harness.py run --pipeline pipelines/foundation_inputs.yaml stage.06_project_context_interview
```

## 5) Run Foundation Pack Synthesis

```bash
python3 tools/harness.py run --pipeline pipelines/foundation_inputs.yaml stage.07_foundation_pack
```

## Outputs to verify

```bash
ls -1 artifacts/charter/CHARTER_INPUTS.yaml
ls -1 artifacts/charter/CHARTER.md
ls -1 artifacts/foundation/
```
