# Workflow: Release planning

This workflow creates a Release plan as a **queue + multi-sprint intent**.

## Key idea

- Release planning chooses *what* is in focus (features, bugs, work items) and the multi-sprint shape.
- Sprint planning turns that intent into **typed tasks** and sprint gates.

## Prerequisites

You should have already run Foundation:
- Charter exists
- Foundation pack exists (esp. quality gates posture)

## Identify your release pipeline + stage id

List stages:

```bash
python3 tools/harness.py list
```

If you have multiple pipelines:

```bash
ls -1 pipelines/
python3 tools/harness.py list --pipeline pipelines/release.yaml
```

## Run the release planning stage

Use the stage id shown by `list` (examples below).

### Example A: stage.01_release_plan

```bash
python3 tools/harness.py run --pipeline pipelines/release.yaml stage.01_release_plan \
  --enable-complexity false
```

### Example B: stage.11_release_plan

```bash
python3 tools/harness.py run --pipeline pipelines/release.yaml stage.11_release_plan \
  --enable-complexity false
```

## Outputs to expect

Typical outputs (exact paths depend on your stage front matter):

- `artifacts/release/RELEASE_PLAN.md`
- `artifacts/release/release.yaml` (machine-readable)
- optionally repo-root copies like `./RELEASE_PLAN.md`

Verify:

```bash
find artifacts -maxdepth 3 -type f -iname "*release*"
```

## Common gotcha: “don’t make up features”

If your release stage references a Work Catalog / backlog file, ensure it exists and contains IDs the model can point to.
If it doesn’t exist, the release stage should either:
- ask minimal questions, or
- require you to add a minimal backlog list first.
