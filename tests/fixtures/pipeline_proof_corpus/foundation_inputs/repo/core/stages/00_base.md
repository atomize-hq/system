---
kind: stage
id: stage.00_base
version: 0.1.0
title: "Base Initialization \u2192 BASE_CONTEXT.md"
work_level: L0
description: 'Establishes the pipeline run baseline (runner/profile/repo_root/identifiers)
  and emits a small BASE_CONTEXT.md artifact so downstream stages can reference stable
  project metadata and defaults. This stage is language-agnostic and must not hardcode
  stack commands (profile owns tooling).

  '
includes:
- core/rules/p0_absolute.md
- core/rules/p1_pragmatic.md
- core/rules/traceability_policy.md
- core/rules/evidence_policy.md
- runners/${runner}.md
- profiles/${profile}/conventions.md
- profiles/${profile}/profile.yaml
- profiles/${profile}/commands.yaml
inputs:
  library:
  - path: core/library/base/base_init_directive.md
    required: true
  - path: core/library/base/BASE_CONTEXT.md.tmpl
    required: true
  variables:
  - runner
  - profile
  - repo_root
  - project_name?
  - owner?
  - team?
  - repo_or_project_ref?
  - now_utc?
  - run_mode?
  - enable_complexity?
outputs:
  artifacts:
  - path: artifacts/base/BASE_CONTEXT.md
  repo_files:
  - path: ${repo_root}/BASE_CONTEXT.md
    required: false
gating:
  mode: strict
  fail_on:
  - missing_required_inputs
  - output_missing
  notes:
  - This stage must remain language/tooling agnostic. No Poetry/uv/cargo/pnpm commands
    here.
  - Final response must be ONLY the completed BASE_CONTEXT.md markdown.
tags:
- base
- bootstrap
- pipeline-metadata
- upstream
---
# core/stages/00_base.md

<!--
Stage body intentionally minimal.
The directive + template define the content.
Orchestrator injects:
- runner/profile/repo_root
- any known metadata (project_name, repo ref, owner/team)
This stage emits a stable BASE_CONTEXT.md used as an anchor for downstream artifacts.
-->
