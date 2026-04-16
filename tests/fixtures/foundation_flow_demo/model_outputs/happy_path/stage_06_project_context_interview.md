# Project Context

## Users
- Primary: engineers maintaining the Rust compiler and CLI route proofs
- Secondary: operators validating that stage-10 handoff semantics stay truthful
- Reviewers: docs owners checking that help text and narrative docs match shipped behavior

## Current State
`M4` is replacing the old stage-10 shortcut with a real external-model boundary. The live route
already resolves and captures upstream stages, but existing proof surfaces still need one
realistic journey that separates compile payload generation from final feature-spec capture.

## Operational Reality To Preserve
- The happy path must show stage 06 as manually activated after stage 05 establishes the need for
  project context.
- The skip path must remain truthful because both `needs_project_context=false` and
  `charter_gaps_detected=false` are evidenced, not inferred from missing text.
- The operator journey artifact must call out where manual decisions still exist after `M4`.
- The rerun story must show deterministic output when `now_utc` is fixed and capture ids are normalized.

## Repo Constraints
- The dedicated M4 fixture tree must remain under `tests/fixtures/foundation_flow_demo`.
- No product commands, flags, or writer modes are added for this proof.
- The same canonical stage metadata must drive both the proof corpus and the M4 demo corpus.

## Integration Touchpoints
- `crates/cli/tests/cli_surface.rs`
- `crates/cli/tests/help_drift_guard.rs`
- `crates/compiler/tests/pipeline_capture.rs`
- `docs/CLI_OPERATOR_JOURNEY.md`

## Decisions Confirmed Here
- The M4 journey scorecard must cover manual decisions, model-output boundaries, repo rereads
  avoided, and what remains manual for `M5`.
- The rerun story belongs in the proof tests and is summarized in the journey doc.
- External model output for stage 10 is a completed `FEATURE_SPEC.md`, not compile payload.
