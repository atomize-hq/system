---
kind: stage
id: stage.01_release_plan
version: 0.1.0
title: "Release Planning"
work_level: L0
description: >
  Defines a release as a queue + multi-sprint intent. The release must reference real
  work items (features/bugs/chores) so sprint planning can stay grounded.

includes:
  - core/rules/p0_absolute.md
  - core/rules/p1_pragmatic.md
  - core/rules/traceability_policy.md
  - core/rules/evidence_policy.md
  - runners/${runner}.md
  - profiles/${profile}/conventions.md

inputs:
  library:
    - path: core/library/release/release_planning_directive.md
      required: true
    - path: core/library/release/RELEASE_PLAN.md.tmpl
      required: true
    - path: core/library/release/release.yaml.tmpl
      required: true
    - path: core/library/work_catalog/WORK_CATALOG.yaml.tmpl
      required: true
  artifacts:
    - path: artifacts/base/BASE_CONTEXT.md
      required: false
    - path: artifacts/charter/CHARTER.md
      required: true
    - path: artifacts/project_context/PROJECT_CONTEXT.md
      required: false
    - path: artifacts/foundation/FOUNDATION_STRATEGY.md
      required: false
    - path: artifacts/foundation/QUALITY_GATES_SPEC.md
      required: false
    - path: ${repo_root}/backlog/WORK_CATALOG.yaml
      required: false
  variables:
    - runner
    - profile
    - repo_root
    - release_id?
    - release_type?
    - charter_ref?
    - project_context_ref?

outputs:
  artifacts:
    - path: artifacts/releases/${release_id}/RELEASE_PLAN.md
    - path: artifacts/releases/${release_id}/release.yaml
  repo_files:
    - path: ${repo_root}/releases/${release_id}/RELEASE_PLAN.md
      required: false
    - path: ${repo_root}/releases/${release_id}/release.yaml
      required: false

gating:
  mode: strict
  fail_on:
    - missing_required_inputs
    - output_missing
    - invented_work_items

tags:
  - release
  - planning
  - queue
---

# core/stages/01_release_plan.md

<!--
Stage body intentionally minimal.
The directive + templates define the content.
Inputs are expected to include a Work Catalog at ${repo_root}/backlog/WORK_CATALOG.yaml when available.
-->
