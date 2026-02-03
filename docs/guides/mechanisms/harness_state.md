# Mechanism: Harness state (`artifacts/_harness_state.yaml`)

## What it is

The harness maintains a small state file so you don’t have to keep re-entering variables:

- runner/profile
- repo_root
- refs to previously produced artifacts (charter_ref, etc.)
- flags such as `enable_complexity` and `needs_project_context`

## Why it exists

- Avoid repetitive CLI flags
- Support conditional activation (e.g., skip project context)
- Keep downstream stages grounded via stable refs

## How it works today

- After compile/run, the harness persists variables into `artifacts/_harness_state.yaml`.
- After capture, the harness may update:
  - convenience refs (`charter_ref`, `project_context_ref`, etc.)
  - settable variables declared by the stage (`sets:`)

## Resetting state

```bash
rm -f artifacts/_harness_state.yaml
```

## Do / Don’t

✅ Do:
- reset state when debugging a surprising compile/run behavior
- treat state as local run metadata, not a “project truth” source

❌ Don’t:
- rely on state for long-term project records (that’s what repo files are for)
