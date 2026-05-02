---
kind: overlay
id: overlay.complexity_assessment
version: 0.1.0
title: "Complexity Assessment"
description: >
  Adds complexity constraints and verification steps using the active profile's complexity command.

activation:
  when:
    any:
      - variables.enable_complexity == true
      - profile.gates.required contains "complexity"

inputs:
  variables:
    - enable_complexity

effects:
  adds_requirements:
    - "Complexity evidence must be present for PASS."
  adds_sections:
    - "Complexity Verification"

tags:
  - optional
  - metrics
---

> Example specimen only. This file documents overlay shape and is not a runtime-loaded source of truth.
