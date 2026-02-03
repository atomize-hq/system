---
kind: stage
id: stage.02_sprint_plan
version: 0.1.0
title: "Sprint Planning (Opening Gate)"
work_level: L1
description: >
  Uses release intent + previous sprint reality to produce a concrete sprint plan and task list.
  Tasks must reference release-selected work items and be machine-checkable.

includes:
  - core/rules/p0_absolute.md
  - core/rules/p1_pragmatic.md
  - core/rules/traceability_policy.md
  - core/rules/evidence_policy.md
  - runners/${runner}.md
  - profiles/${profile}/conventions.md

inputs:
  library:
    - path: core/library/sprint/sprint_planning_directive.md
      required: true
    - path: core/library/sprint/SPRINT_PLAN.md.tmpl
      required: true
    - path: core/library/sprint/sprint.yaml.tmpl
      required: true
    - path: core/library/sprint/tasks.yaml.tmpl
      required: true
  artifacts:
    - path: artifacts/base/BASE_CONTEXT.md
      required: false
    - path: artifacts/charter/CHARTER.md
      required: true
    - path: artifacts/releases/${release_id}/RELEASE_PLAN.md
      required: true
    - path: artifacts/releases/${release_id}/release.yaml
      required: true
    - path: ${repo_root}/backlog/WORK_CATALOG.yaml
      required: false
    - path: artifacts/sprints/${prev_sprint_id}/SPRINT_REPORT.md
      required: false
  variables:
    - runner
    - profile
    - repo_root
    - release_id
    - sprint_id?
    - sprint_slot?
    - prev_sprint_id?

outputs:
  artifacts:
    - path: artifacts/sprints/${sprint_id}/SPRINT_PLAN.md
    - path: artifacts/sprints/${sprint_id}/sprint.yaml
    - path: artifacts/sprints/${sprint_id}/tasks.yaml
  repo_files:
    - path: ${repo_root}/sprints/${sprint_id}/SPRINT_PLAN.md
      required: false
    - path: ${repo_root}/sprints/${sprint_id}/sprint.yaml
      required: false
    - path: ${repo_root}/sprints/${sprint_id}/tasks.yaml
      required: false

gating:
  mode: strict
  fail_on:
    - missing_required_inputs
    - output_missing
    - invented_work_items
    - tasks_not_machine_checkable

tags:
  - sprint
  - planning
  - tasks
---

# core/stages/02_sprint_plan.md

<!--
Stage body intentionally minimal.
This stage should turn release slot intent into a concrete task plan.
If a previous sprint report is present, incorporate carry-over work and adjust scope.
-->
