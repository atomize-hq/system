# stage.06_project_context_interview — Project Context Interview → PROJECT_CONTEXT.md

## Purpose

Produces `PROJECT_CONTEXT.md`: a short, factual “reality snapshot” that prevents agents from inventing
brownfield work (migrations, backward-compat, rollout constraints, integrations) when it isn’t real.

It is **optional** and only runs when the Charter still leaves planning-critical unknowns.

- **Work level:** L1 (Project/Planning)
- **Interaction style:** interview (one question at a time)

## Activation

This stage is gated by front-matter activation:
- `variables.needs_project_context == true`

If `needs_project_context` is false and you don’t pass `--force`, the harness will write a “SKIPPED” compiled prompt.

## Inputs

### Library inputs (required)
- `core/library/project_context/project_context_gen_directive.md`
- `core/library/project_context/PROJECT_CONTEXT.md.tmpl`

### Artifact inputs (required)
- `artifacts/charter/CHARTER.md`

## Outputs

### Artifacts
- `artifacts/project_context/PROJECT_CONTEXT.md`

### Repo files
- `${repo_root}/PROJECT_CONTEXT.md` (optional)

## How to run

If you answered “Yes” to `needs_project_context` after the Charter stage:

```bash
./tools/harness.sh compile --only stage.06_project_context_interview
./tools/harness.sh run stage.06_project_context_interview
```

Force-run even if activation is false:

```bash
./tools/harness.sh compile --only stage.06_project_context_interview --force
./tools/harness.sh run stage.06_project_context_interview --force
```

## Expected model behavior

- Read `CHARTER.md` first.
- Ask only questions that fill **missing facts** that affect planning (prod/users/data/back-compat/migrations/integrations/environments).
- When generating the final result, output **ONLY** the completed `PROJECT_CONTEXT.md` markdown.

## Common gotchas

- Project Context is **facts**, not posture (Charter) and not a feature plan (Feature Spec).
- If a fact is unknown, the document should record “Unknown” rather than guessing.
