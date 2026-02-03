# stage.10_feature_spec — Feature Specification → FEATURE_SPEC.md

## Purpose

Produces a single per-feature `FEATURE_SPEC.md` that acts as the contract for downstream phase decomposition and slicing.

The spec must be aligned with:
- `CHARTER.md` (posture and red lines)
- the Foundation Pack artifacts (if present)
- project reality (`PROJECT_CONTEXT.md` if present)

- **Work level:** L1 (Project/Planning)
- **Interaction style:** interview by default (up to ~10 clarifying questions)

## Inputs

### Library inputs (required)
- `core/library/feature_spec/feature_spec_architect_directive.md`
- `core/library/feature_spec/FEATURE_SPEC.md.tmpl`

### Artifact inputs
Required:
- `artifacts/base/BASE_CONTEXT.md`
- `artifacts/charter/CHARTER.md`

Optional (included if present):
- `artifacts/project_context/PROJECT_CONTEXT.md`
- `artifacts/foundation/FOUNDATION_STRATEGY.md`
- `artifacts/foundation/TECH_ARCH_BRIEF.md`
- `artifacts/foundation/TEST_STRATEGY_BRIEF.md`
- `artifacts/foundation/QUALITY_GATES_SPEC.md`
- `artifacts/foundation/quality_gates.yaml`
- `artifacts/foundation/ENVIRONMENT_INVENTORY.md`

## Outputs

### Artifacts
- `artifacts/feature_spec/FEATURE_SPEC.md`

## How to run

```bash
./tools/harness.sh compile --only stage.10_feature_spec
./tools/harness.sh run stage.10_feature_spec
```

## Expected model behavior

- Ask one question at a time **only if needed** to produce a complete, testable spec.
- When generating the final result, output **ONLY** the completed `FEATURE_SPEC.md` markdown.
- NFRs and rollout/testing sections should explicitly reference Charter posture and Foundation defaults when they exist.

## Common gotchas

- If you don’t provide a feature request/problem statement, the model will ask more questions (by design).
- Avoid hardcoding stack commands in the spec; refer to the selected profile commands conceptually.
