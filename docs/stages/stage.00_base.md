# stage.00_base — Base Initialization → BASE_CONTEXT.md

## Purpose

Establishes a stable baseline for a pipeline run (runner/profile/repo_root/identifiers) and writes a small
`BASE_CONTEXT.md` artifact. Downstream stages use it to avoid guessing foundational metadata.

- **Work level:** L0 (Program)
- **Interaction style:** interview (one question at a time if required fields are missing)

## Inputs

### Library inputs (required)
- `core/library/base/base_init_directive.md`
- `core/library/base/BASE_CONTEXT.md.tmpl`

### Run variables (commonly used)
Provided via `pipeline.yaml` defaults, CLI flags, or `artifacts/_harness_state.yaml`:
- `runner`, `profile`, `repo_root`
- `project_name`, `owner`, `team`, `repo_or_project_ref` (may be blank initially)
- `now_utc`
- `run_mode` (bootstrap | onboard | hotfix | unknown)
- `enable_complexity` (affects overlay injection; does not change this stage’s content)

## Outputs

### Artifacts
- `artifacts/base/BASE_CONTEXT.md`

### Repo files
- `${repo_root}/BASE_CONTEXT.md` (optional)

## How to run (copy/paste workflow)

Compile only this stage:
```bash
./tools/harness.sh compile --only stage.00_base
```

Run interactively (compile + then capture output):
```bash
./tools/harness.sh run stage.00_base
```

Or, compile then capture later:
```bash
./tools/harness.sh compile --only stage.00_base
# paste dist/stage.00_base.md into your LLM
./tools/harness.sh capture stage.00_base
```

## Expected model behavior

- If required fields are missing, the model should ask **one question at a time** (e.g., project name, repo ref).
- When you indicate “go ahead / generate,” it should output **ONLY** the completed `BASE_CONTEXT.md` markdown.

## Common gotchas

- **State precedence:** CLI flags override `artifacts/_harness_state.yaml`, which overrides pipeline defaults.
- **Keep it tooling-agnostic:** this stage should not include stack/tool commands (profiles handle that).
