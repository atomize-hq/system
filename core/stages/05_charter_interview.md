---
kind: stage
id: stage.05_charter_interview
version: 0.1.0
title: "Charter Interview \u2192 CHARTER.md"
work_level: L1
description:
  "Runs a structured one-question-at-a-time interview to produce the project
  Engineering Charter (CHARTER.md), which becomes a source-of-truth posture/standards
  input for all later stages.

  "
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
    - path: core/library/charter/charter_gen_directive.md
      required: true
    - path: core/library/charter/charter.md.tmpl
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
    - test_mode?
outputs:
  artifacts:
    - path: artifacts/charter/CHARTER.md
  repo_files:
    - path: ${repo_root}/CHARTER.md
      required: true
gating:
  mode: strict
  fail_on:
    - missing_required_inputs
    - output_missing
  notes:
    - Final response must be ONLY the completed CHARTER.md markdown.
tags:
  - charter
  - posture
  - interview
  - upstream-constraint
---
