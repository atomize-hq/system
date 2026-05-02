---
kind: stage
id: stage.04_charter_inputs
version: 0.1.0
title: "Charter Inputs (Dev/Test) → CHARTER_INPUTS.yaml"
work_level: L1
description: >
  Generates a stable CHARTER_INPUTS.yaml fixture for testable charter generation.
  This is a development/testing path that avoids multi-turn interviewing.

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
    - path: core/library/charter/charter_inputs_directive.md
      required: true
    - path: core/library/charter/CHARTER_INPUTS.yaml.tmpl
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
    - path: artifacts/charter/CHARTER_INPUTS.yaml

gating:
  mode: strict
  fail_on:
    - missing_required_inputs
    - output_missing
  notes:
    - Final response must be ONLY the YAML content for CHARTER_INPUTS.yaml.

tags:
  - charter
  - dev
  - fixtures
---

<!-- Stage body intentionally minimal. -->
