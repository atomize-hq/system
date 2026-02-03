# Workflow: Sprint planning

This workflow creates or updates a Sprint plan that is grounded in:

- the Release plan intent (what the sprint must contribute to)
- the previous sprint reality (what actually happened)

## Key idea

Sprints should not invent scope.
They should **derive tasks** by referencing:
- Release-level focus items (feature IDs / backlog item IDs)
- Prior sprint outcome (completed/rolled/blocked)

## Identify your sprint pipeline + stage id

```bash
python3 tools/harness.py list --pipeline pipelines/sprint.yaml
```

## Run sprint planning

Use the stage id shown by `list`.

Example:

```bash
python3 tools/harness.py run --pipeline pipelines/sprint.yaml stage.02_sprint_plan \
  --enable-complexity false
```

If your harness/stage supports explicit vars, pass them:

```bash
python3 tools/harness.py run --pipeline pipelines/sprint.yaml stage.02_sprint_plan \
  --release-id "REL-0001" \
  --sprint-id "SPR-0001" \
  --sprint-slot "S1" \
  --enable-complexity false
```

### Include previous sprint reality (recommended for Sprint 2+)

If you have a previous sprint report available, pass its sprint id so the stage can include:
`artifacts/sprints/<prev_sprint_id>/SPRINT_REPORT.md` (if present).

```bash
python3 tools/harness.py run --pipeline pipelines/sprint.yaml stage.02_sprint_plan \
  --release-id "REL-0001" \
  --sprint-id "SPR-0002" \
  --sprint-slot "S2" \
  --prev-sprint-id "SPR-0001" \
  --enable-complexity false
```

If you don’t have a formal `SPRINT_REPORT.md` yet, you can create a simple one manually and iterate later.

## Outputs to expect

Typically:

- `artifacts/sprints/<id>/SPRINT_PLAN.md`
- `artifacts/sprints/<id>/tasks.yaml` (typed tasks, including gates-as-tasks)

Verify:

```bash
find artifacts -maxdepth 4 -type f -iname "*sprint*"
find artifacts -maxdepth 4 -type f -iname "*tasks*"
```

## Closing note

Sprint planning often benefits from an “opening planning gate” task.
That gate is itself a task in `tasks.yaml`, and it blocks sprint closure if incomplete.
