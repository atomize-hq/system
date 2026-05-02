---
kind: stage
id: stage.06_project_context_interview
version: 0.1.0
title: "Project Context Interview \u2192 PROJECT_CONTEXT.md"
work_level: L1
description: 'Optional stage that produces PROJECT_CONTEXT.md as a factual reality
  snapshot. Runs only when the Charter still contains unknowns that would impact planning
  (prod/users/data/back-compat/migrations/integrations/environments).

  '
includes:
- core/rules/p0_absolute.md
- core/rules/p1_pragmatic.md
- core/rules/traceability_policy.md
- core/runners/${runner}.md
- core/profiles/${profile}/conventions.md
- core/profiles/${profile}/profile.yaml
- core/profiles/${profile}/commands.yaml
inputs:
  library:
  - path: core/library/project_context/project_context_gen_directive.md
    required: true
  - path: core/library/project_context/PROJECT_CONTEXT.md.tmpl
    required: true
  artifacts:
  - path: artifacts/charter/CHARTER.md
    required: true
  variables:
  - runner
  - profile
  - repo_root
  - charter_ref?
  - needs_project_context?
outputs:
  artifacts:
  - path: artifacts/project_context/PROJECT_CONTEXT.md
  repo_files:
  - path: ${repo_root}/PROJECT_CONTEXT.md
    required: false
gating:
  mode: strict
  fail_on:
  - missing_required_inputs
  - output_missing
  notes:
  - Final response must be ONLY the completed PROJECT_CONTEXT.md markdown.
tags:
- project_context
- foundation
- optional
- interview
---
