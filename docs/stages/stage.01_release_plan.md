# stage.01_release_plan — Release Planning → RELEASE_PLAN.md + release.yaml

## Purpose

Creates a **Release Plan** that functions as:
- a **queue** (which work items are in scope)
- a **multi-sprint intent** (how the work is sequenced into sprint slots)

The release MUST reference real work items so downstream sprint planning cannot invent scope.

- **Work level:** L0 (Program / release sequencing)
- **Interaction style:** usually minimal questions; grounded in inputs

## Inputs

### Required artifacts
- `artifacts/charter/CHARTER.md`

### Optional artifacts (high value)
- `${repo_root}/backlog/WORK_CATALOG.yaml` (recommended; canonical work item list)
- `artifacts/project_context/PROJECT_CONTEXT.md`
- `artifacts/foundation/FOUNDATION_STRATEGY.md`
- `artifacts/foundation/QUALITY_GATES_SPEC.md`

## Outputs

### Artifacts (declared)
- `artifacts/releases/${release_id}/RELEASE_PLAN.md`
- `artifacts/releases/${release_id}/release.yaml`

### Repo files (optional)
- `${repo_root}/releases/${release_id}/RELEASE_PLAN.md`
- `${repo_root}/releases/${release_id}/release.yaml`

## Key rules

- **No invented work items:** release selection must come from the Work Catalog.
- Release defines **slot intent**, not detailed tasks.
- Slot intent MUST list:
  - slot goal
  - focus work item IDs
  - required task types (e.g., `planning_gate`, `research_discovery`, `execution_slice`, `integration_gate`)

## How to run

Compile:
```bash
./tools/harness.sh compile --only stage.01_release_plan --release-id release-001 --release-type minor
```

Run interactively:
```bash
./tools/harness.sh run stage.01_release_plan --release-id release-001 --release-type minor
```

## Expected model behavior

- If `backlog/WORK_CATALOG.yaml` is present, select work from it.
- If it’s missing or incomplete, ask **at most one** question: a short list of work item IDs + titles.
- Emit **only** multi-file `--- FILE: ... ---` blocks matching declared outputs.
