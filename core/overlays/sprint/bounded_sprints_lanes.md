# Overlay: Bounded Sprints & Lanes

Use this overlay when sprint planning should be organized into **bounded lanes** rather than a flat task list.

## Intent
Lanes make parallel work explicit without allowing execution chaos. They create a small number of "workstreams" that
can proceed in parallel (often in separate worktrees), while keeping one sprint goal and one sprint close-out discipline.

## Required sprint structure (when this overlay is active)
- Sprint plan MUST define **2–5 lanes**.
- Each lane MUST include:
  - `lane_id` (stable id)
  - a one-sentence goal
  - a list of `focus_items` (feature/bug/work-item IDs)
- Each task MUST:
  - belong to exactly one lane (`lane_id`)
  - reference at least one `work_item_id` that exists in the Release Plan selection

## Recommended default lane set
Choose the smallest set that fits. Defaults:
- `discovery`: research/discovery + decision work (spikes, validation, prototypes)
- `delivery`: implementation slices + tests
- `integration`: integration, docs, rollout readiness, cross-cutting fixes
Optional:
- `hardening`: perf/reliability/security hardening triggered by Charter posture or release type
- `ops`: environment/inventory changes, automation, CI improvements

## Gate tasks (still "just tasks")
When lanes are used, prefer making gates explicit tasks:
- `planning_gate`: confirms the sprint task list is complete and linked to release + feature refs
- `integration_gate`: ensures lane outputs are wired together and build/test gates pass
- `release_gate`: (only if sprint is last sprint of a release) confirms release DoD is satisfied

## Parallelism note
Parallel work is allowed across lanes and across worktrees.
- Within a single lane/worktree, keep slice execution serialized to avoid evidence drift.
- Sprint close-out should be centralized (integration lane), with clear evidence.
