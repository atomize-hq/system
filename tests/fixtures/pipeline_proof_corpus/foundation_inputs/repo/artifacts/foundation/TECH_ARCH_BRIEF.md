# Technical Architecture Brief

## Compile Path
1. CLI accepts `pipeline compile --id <pipeline> --stage <stage-id> [--explain]`.
2. Compiler loads the canonical pipeline definition and requested stage metadata from repo truth.
3. Compiler reads the persisted route-basis snapshot produced by `pipeline resolve`.
4. Compiler validates freshness, active-stage status, and required input presence.
5. Success returns either:
   - payload-only compiled stage content, or
   - proof-only explain output

## Key Contracts
- Pipeline ids and stage ids remain canonical identifiers.
- Route basis is compile input, not mutable execution state.
- Required library and artifact inputs fail closed on missing or empty content.
- Optional artifacts participate in explain output without blocking payload generation.

## Integration Touchpoints
- `crates/compiler`
- `crates/cli`
- `core/pipelines/foundation_inputs.yaml`
- `core/stages/10_feature_spec.md`
- `tests/fixtures/pipeline_proof_corpus/foundation_inputs`
