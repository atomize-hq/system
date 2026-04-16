# M4 Evidence Bundle

This directory is the committed deterministic proof bundle for the `pipeline.foundation_inputs`
M4 happy and skip journeys.

## Files

- `happy_path.transcript.txt`: normalized operator-visible transcript for the happy-path journey.
- `skip_path.transcript.txt`: normalized operator-visible transcript for the skip-path journey.

## Normalization Contract

- Stage-10 compile evidence is pinned with
  `SYSTEM_PIPELINE_COMPILE_NOW_UTC=2026-01-28T18:35:10Z`.
- Any preview or apply output that contains a generated capture id must normalize that value to
  `{{CAPTURE_ID}}`.
- Temp repo roots must normalize to `{{REPO_ROOT}}`.
- Persisted route-state paths must normalize to `{{STATE_PATH}}`.

## Source Separation

Shared stage-10 contract regressions read their completed external output from `tests/fixtures/pipeline_proof_corpus/foundation_inputs/model_outputs/stage_10_feature_spec.md`.

The demo fixture under `tests/fixtures/foundation_flow_demo/` remains the source for the happy and
skip journey proofs plus the structural `FEATURE_SPEC.md` contract checker.
