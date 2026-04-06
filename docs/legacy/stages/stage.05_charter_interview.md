# stage.05_charter_interview — Charter Interview → CHARTER.md

## Purpose

Creates the project’s **Engineering Charter** (`CHARTER.md`): a posture/standards source of truth
(“how we decide / what we optimize for / where we allow shortcuts”).

Downstream stages treat the Charter as a constraint: feature specs and foundation artifacts must align with it.

- **Work level:** L1 (Project/Planning)
- **Interaction style:** interview (one question at a time) by default

## Inputs

### Library inputs (required)
- `core/library/charter/charter_gen_directive.md`
- `core/library/charter/charter.md.tmpl`

### Run variables (commonly used)
- `project_name`, `owner`, `team`, `repo_or_project_ref`
- `profile` (tooling assumptions are profile-aware)
- `repo_root`

> **Test mode (optional):** if your harness has been patched to support `--test-mode true`, it can swap the directive to a
> test-mode variant that generates a realistic synthetic Greenfield charter without asking questions.

## Outputs

### Artifacts
- `artifacts/charter/CHARTER.md`

### Repo files
- `${repo_root}/CHARTER.md` (required)

## How to run (copy/paste workflow)

Compile:
```bash
./tools/harness.sh compile --only stage.05_charter_interview
```

Run interactively (compile + capture):
```bash
./tools/harness.sh run stage.05_charter_interview
```

Test mode (if supported in your harness):
```bash
./tools/harness.sh compile --only stage.05_charter_interview --test-mode true
# paste dist/stage.05_charter_interview.md into your LLM
./tools/harness.sh capture stage.05_charter_interview
```

## Expected model behavior

- Ask one question at a time.
- Use defaults aggressively; only drill into contradictions or high-risk uncertainty.
- When generating the final result, output **ONLY** the completed `CHARTER.md` markdown (no preamble).

## Harness behavior after capture

In `pipeline.yaml`, this stage declares `sets: [needs_project_context]`.
After you capture the Charter output, the harness will prompt:

- `Set needs_project_context? [Y/n]:`

Answer **Yes** if the charter still leaves planning-critical unknowns (prod/users/data/back-compat/migrations/integrations/environments).
Answer **No** if the Charter is sufficient and Stage 06 can be skipped.

## Common gotchas

- **Don’t turn the Charter into a feature spec.** It should define posture, not “what we’re building.”
- **Profile awareness:** the charter can mention tooling assumptions, but avoid hardcoding exact commands; profile owns commands.
