---
kind: stage
id: stage.10_feature_spec
version: 0.1.0
title: "Feature Specification"
work_level: L1
description: >
  Produces a single FEATURE_SPEC.md for one feature, aligned with the project Charter and Foundation Pack.

includes:
  - core/rules/p0_absolute.md
  - core/rules/p1_pragmatic.md
  - core/rules/traceability_policy.md
  - core/rules/evidence_policy.md
  - runners/${runner}.md
  - profiles/${profile}/profile.yaml
  - profiles/${profile}/commands.yaml
  - profiles/${profile}/conventions.md

inputs:
  library:
    - path: core/library/feature_spec/feature_spec_architect_directive.md
      required: true
    - path: core/library/feature_spec/FEATURE_SPEC.md.tmpl
      required: true
  artifacts:
    - path: artifacts/base/BASE_CONTEXT.md
      required: true
    - path: artifacts/charter/CHARTER.md
      required: true
    - path: artifacts/project_context/PROJECT_CONTEXT.md
      required: false
    - path: artifacts/foundation/FOUNDATION_STRATEGY.md
      required: false
    - path: artifacts/foundation/TECH_ARCH_BRIEF.md
      required: false
    - path: artifacts/foundation/TEST_STRATEGY_BRIEF.md
      required: false
    - path: artifacts/foundation/QUALITY_GATES_SPEC.md
      required: false
    - path: artifacts/foundation/quality_gates.yaml
      required: false
    - path: artifacts/foundation/ENVIRONMENT_INVENTORY.md
      required: false
  variables:
    - runner
    - profile
    - repo_root
    - now_utc
    - project_name?
    - owner?
    - team?
    - repo_or_project_ref?
    - charter_ref?

outputs:
  artifacts:
    - path: artifacts/feature_spec/FEATURE_SPEC.md

gating:
  mode: strict
  fail_on:
    - missing_required_inputs
    - output_missing
  notes:
    - Output must be ONLY the completed FEATURE_SPEC.md (no extra commentary).
    - NFRs and rollout/testing sections must explicitly reference Charter posture and Foundation Pack defaults (if present).

tags:
  - feature-spec
  - planning
---

# core/stages/10_feature_spec.md

<!--
Stage body intentionally minimal.
The directive + template define the content.
-->
