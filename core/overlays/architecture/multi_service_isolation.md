# Overlay: Multi-Service Isolation

Use when a feature touches multiple services or repos.

## Principles
- Prefer single-service changes per slice.
- If multiple services are required, define an explicit contract boundary:
  - API schema / message format
  - versioning / compatibility plan
  - rollback strategy

## Guidance
- Avoid “drive-by refactors” across services.
- Add contract tests or schema validation at boundaries.
- Make cross-service changes reversible (feature flags, dual-write, compatibility shims).

## Evidence
For any cross-service change, capture:
- which services were touched
- contract change summary
- how compatibility is preserved or intentionally broken (with exception approval)
