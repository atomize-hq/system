# Foundation Strategy

## Objective
Deliver `M2` as one narrow, credible compile slice:
- one canonical target stage: `stage.10_feature_spec`
- one persisted route-basis handoff from `pipeline resolve`
- one shared proof corpus used by both compiler and CLI tests

## Strategy Pillars
1. Preserve canonical repo truth in pipeline/stage metadata and fixture copies.
2. Refuse stale, inactive, malformed, or incomplete compile requests before any payload rendering.
3. Reuse shared library, rule, runner, and profile inputs so compiled payloads are explainable.
4. Keep proof output compact and terminal-friendly.

## Delivery Guardrails
- Add only the compile-time dependencies that the target stage actually references.
- Keep optional foundation artifacts visible to `--explain` but non-fatal when absent.
- Prefer stable fixture content over exhaustive scenario sprawl.
