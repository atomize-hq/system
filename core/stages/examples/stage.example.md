---
kind: stage
id: stage.60_slice_execute
version: 0.1.0
title: "Slice Execute"
description: >
  Executes a single slice against a repo using the active runner adapter and active project profile.
  Produces an execution report + evidence log suitable for quality gating.

# Composition
includes:
  - core/rules/p0_absolute.md
  - core/rules/p1_pragmatic.md
  - core/rules/evidence_policy.md
  - core/rules/traceability_policy.md
  - core/rules/integration_policy.md
  - core/runners/${runner}.md
  - core/profiles/${profile}/conventions.md

optional_overlays:
  - core/overlays/complexity_assessment.md
  - core/overlays/fix_errors.md

# Inputs / Outputs (artifact contract)
inputs:
  artifacts:
    - path: artifacts/slices/${slice_id}.yaml
      required: true
    - path: artifacts/feature_spec.yaml
      required: true
  variables:
    - runner
    - profile
    - slice_id
    - repo_root

outputs:
  artifacts:
    - path: artifacts/execution/${slice_id}/execution_report.yaml
      schema: core/schemas/execution_report.yaml
    - path: artifacts/execution/${slice_id}/evidence.jsonl
      schema: core/schemas/evidence_log.yaml
    - path: artifacts/execution/${slice_id}/patch.diff
      optional: true

# Runtime behavior
gating:
  mode: strict
  fail_on:
    - missing_required_inputs
    - missing_required_evidence
  retry_policy:
    enabled: true
    max_attempts: 2
    retry_on:
      - test_failures
      - lint_failures

# Prompt assembly knobs
tokens:
  priority: high
  budget_hint: medium
  omit_sections_if_missing_vars: true

tags:
  - execution
  - repo_mutating
  - evidence_required
---

> Example specimen only. This file documents stage shape and is not a runtime-loaded source of truth.
