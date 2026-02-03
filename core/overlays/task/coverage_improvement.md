# Overlay: Coverage Improvement

Use when coverage gates are failing or the project posture requires high coverage.

## Strategy
- Start with the most valuable behavior:
  - pure functions and core business rules
  - boundary conditions and error handling
- Prefer adding tests over lowering thresholds.
- If legacy code is low coverage, use a ratchet:
  - no regressions
  - raise threshold gradually per area

## What not to test
- stdlib behavior
- trivial getters/setters without logic
- implementation details that will churn (unless contract-level)

## Evidence
When claiming coverage improvement, include:
- before/after coverage numbers
- which modules were targeted
