# stage.02_sprint_plan — Sprint Planning (Opening Gate) → SPRINT_PLAN.md + sprint.yaml + tasks.yaml

## Purpose

Turns release intent into an **executable sprint** by producing:
- a sprint plan (`SPRINT_PLAN.md`)
- a machine sprint file (`sprint.yaml`)
- a machine task list (`tasks.yaml`)

This stage is an **opening planning gate**:
- the release provides rough intent (slot goal + focus items)
- the sprint plan fleshes out the concrete tasks required to deliver that intent
- the sprint plan must incorporate what the **previous sprint actually accomplished** when that input is present

- **Work level:** L1 (Project/Planning)

## Inputs

### Required artifacts
- `artifacts/charter/CHARTER.md`
- `artifacts/releases/${release_id}/RELEASE_PLAN.md`
- `artifacts/releases/${release_id}/release.yaml`

### Optional artifacts
- `${repo_root}/backlog/WORK_CATALOG.yaml` (recommended)
- `artifacts/sprints/${prev_sprint_id}/SPRINT_REPORT.md` (if not first sprint)
- Feature specs referenced by the release items (if available)

## Outputs

### Artifacts (declared)
- `artifacts/sprints/${sprint_id}/SPRINT_PLAN.md`
- `artifacts/sprints/${sprint_id}/sprint.yaml`
- `artifacts/sprints/${sprint_id}/tasks.yaml`

### Repo files (optional)
- `${repo_root}/sprints/${sprint_id}/SPRINT_PLAN.md`
- `${repo_root}/sprints/${sprint_id}/sprint.yaml`
- `${repo_root}/sprints/${sprint_id}/tasks.yaml`

## Key rules

- **No invented work items:** every task must reference at least one ID from the release selection.
- Tasks must be **machine-checkable**: each task must include a clear completion condition.
- Include at least one `planning_gate` task.
- Add `integration_gate` if the sprint crosses boundaries or uses multiple lanes.

## How to run

Compile:
```bash
./tools/harness.sh compile --only stage.02_sprint_plan --release-id release-001 --sprint-id SPRINT-SEQ-0001 --sprint-slot slot-1
```

Run interactively:
```bash
./tools/harness.sh run stage.02_sprint_plan --release-id release-001 --sprint-id SPRINT-SEQ-0001 --sprint-slot slot-1
```

## Expected model behavior

- Derive the sprint goal from the release slot and adjust it based on previous sprint reality (if provided).
- Create a small, typed task list (research/decisions/execution/review/gates) that directly references the focus work items.
- Emit **only** multi-file `--- FILE: ... ---` blocks matching declared outputs.
