# Overlay: Gates as Tasks

Use this overlay when you want quality/release/sprint gates to be represented as **typed tasks**.

## Goal
Make gates machine-checkable and unavoidable by:
- putting them in the same queue as other work
- making their completion criteria explicit
- blocking sprint/release closure until they are done

## Gate task types
Recommended gate types (extend as needed):
- `gate.planning_open`: sprint opening planning gate
- `gate.quality`: quality gates and CI validation
- `gate.integration`: end-to-end and cross-lane integration validation
- `gate.release`: release readiness checks

## Task fields (recommended)
In `tasks.yaml`:
- `type:` one of the gate types above
- `blocks_close: true`
- `acceptance:` list of objective checks (commands, expected outputs)
- `evidence_required:` list of evidence items the executor must attach

## Closure rules
- Sprint closure MUST fail if any `blocks_close: true` task is incomplete.
- Release closure MUST fail if any release gate task is incomplete.

## Output expectation
When planning a release and its sprints:
- express gates as tasks
- put them in the appropriate lane (often `lane.integration`)
- tie each gate back to Charter + Quality Gates Spec (no invented thresholds)
