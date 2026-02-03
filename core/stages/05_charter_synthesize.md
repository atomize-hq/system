---
kind: stage
id: stage.05_charter_synthesize
version: 0.1.0
title: "Charter Synthesis (from inputs) → CHARTER.md"
work_level: L1
description: >
  Produces CHARTER.md from CHARTER_INPUTS.yaml in a single shot.
  This is designed for development/testing and CI-like iteration.

includes:
  - core/rules/p0_absolute.md
  - core/rules/p1_pragmatic.md
  - core/rules/traceability_policy.md
  - runners/${runner}.md
  - profiles/${profile}/conventions.md
  - profiles/${profile}/profile.yaml
  - profiles/${profile}/commands.yaml

inputs:
  library:
    - path: core/library/charter/charter_synthesize_directive.md
      required: true
    - path: core/library/charter/charter.md.tmpl
      required: true
  artifacts:
    - path: artifacts/charter/CHARTER_INPUTS.yaml
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
  - synthesis
  - dev
---

<!-- Stage body intentionally minimal. -->

