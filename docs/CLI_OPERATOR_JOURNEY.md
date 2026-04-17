# CLI Operator Journey: M4 Proof And M5 Downstream Adoption

## Purpose

This document records the shipped M4 operator proof artifact for `pipeline.foundation_inputs`
and the bounded M5 downstream adoption proof layered on top of it.

It records the journey that is now proved in the repo. It does not widen the product beyond the
bounded reduced-v1 wedge, and it does not promote derived bundle outputs into canonical truth.

M4 stops at:

- one believable happy path
- one believable skip path
- one explicit `stage.10_feature_spec` handoff contract:
  `compile -> external model output -> capture`
- one deterministic rerun story for proof surfaces

M5 adds:

- one explicit downstream handoff emission step:
  `pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer`
- one bundle-only consumer proof that writes a deterministic `SLICE_PLAN.md`
- one before / after scorecard proving the same planning job completes with zero repo rereads

## Setup-Family Context

This M4/M5 journey now sits downstream of the M6 setup family.

- `system setup` is the durable front door.
- Bare `system setup` routes to `setup init` when canonical `.system/` truth is absent or invalid; otherwise it routes to `setup refresh`.
- `setup refresh` preserves canonical files by default, `--rewrite` rewrites only setup-owned starter files, and `--reset-state` resets only `.system/state/**`.
- The canonical setup-created starter files are exactly `.system/charter/CHARTER.md`, `.system/feature_spec/FEATURE_SPEC.md`, and `.system/project_context/PROJECT_CONTEXT.md`.
- Successful setup-family flows end with `system doctor`.

This journey starts only after that setup-family work has established or repaired canonical `.system/` truth. Missing-root, invalid-root, and missing-artifact recovery belongs to the setup family rather than to raw file-creation instructions.

## Evidence Basis

The current M4 proof is grounded in these repo surfaces:

- Fixture corpus: [`tests/fixtures/foundation_flow_demo/`](../tests/fixtures/foundation_flow_demo/)
- Deterministic evidence bundle: [`tests/fixtures/foundation_flow_demo/evidence/`](../tests/fixtures/foundation_flow_demo/evidence/)
- CLI journey proofs:
  - [`pipeline_foundation_inputs_m4_happy_path_proves_real_stage_10_handoff`](../crates/cli/tests/cli_surface.rs)
  - [`pipeline_foundation_inputs_m4_skip_path_skips_stage_06_when_both_route_predicates_are_false`](../crates/cli/tests/cli_surface.rs)
- Stage-10 capture regression coverage:
  - [`pipeline_capture_preview_stage_10_matches_shared_golden`](../crates/cli/tests/cli_surface.rs)
  - [`pipeline_capture_apply_stage_10_matches_shared_golden`](../crates/cli/tests/cli_surface.rs)
  - [`capture_apply_stage_10_matches_shared_golden_from_completed_external_output`](../crates/compiler/tests/pipeline_capture.rs)
- Structural `FEATURE_SPEC.md` contract coverage:
  - [`foundation_flow_demo_feature_specs_match_directive_and_template_contract`](../crates/cli/tests/feature_spec_contract.rs)
- M5 downstream-adoption proof:
  - [`pipeline_foundation_inputs_m5_happy_path_emits_valid_bundle_and_produces_slice_plan`](../crates/cli/tests/cli_surface.rs)
  - [`tests/fixtures/foundation_flow_demo/expected/happy_path/SLICE_PLAN.md`](../tests/fixtures/foundation_flow_demo/expected/happy_path/SLICE_PLAN.md)
  - [`tests/fixtures/foundation_flow_demo/evidence/m5_handoff_scorecard.md`](../tests/fixtures/foundation_flow_demo/evidence/m5_handoff_scorecard.md)

## Happy Path

The happy path proves the route where project context is genuinely required.

1. `pipeline resolve` establishes the initial route basis.
2. Stage `04` capture writes the charter-input artifact.
3. Stage `05` capture writes the charter artifact.
4. The operator makes the explicit branch decision:
   `system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=true`
5. `pipeline resolve` runs again and shows `stage.06_project_context_interview | active`.
6. Stage `06` capture writes `PROJECT_CONTEXT.md`.
7. Stage `07` capture writes the foundation pack.
8. Stage `10` compile runs with a fixed clock and emits payload-only model input.
9. If that raw compile payload is passed directly to `pipeline capture`, raw `pipeline compile` payload is refused as `invalid_capture_input`.
10. An external model or operator produces a completed `FEATURE_SPEC.md`.
11. Stage `10` capture consumes that completed external markdown and writes
    `artifacts/feature_spec/FEATURE_SPEC.md`.

The proof asserts that the stage-10 compile payload is not the final feature spec body, that raw
compile payload is refused by `pipeline capture`, and that the written artifact matches
[`tests/fixtures/foundation_flow_demo/expected/happy_path/final_feature_spec.md`](../tests/fixtures/foundation_flow_demo/expected/happy_path/final_feature_spec.md).

## Skip Path

The skip path proves the route where stage `06` remains skipped for an explicit, content-backed
reason.

1. `pipeline resolve` establishes the initial route basis.
2. Stage `04` capture writes the charter-input artifact.
3. Stage `05` capture writes a charter whose content keeps `charter_gaps_detected=false`.
4. The operator makes the explicit branch decision:
   `system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=false`
5. `pipeline resolve` runs again and shows:
   - `needs_project_context = false`
   - `charter_gaps_detected = false`
   - `stage.06_project_context_interview | skipped`
   - `REASON: activation evaluated false for variables: charter_gaps_detected, needs_project_context`
6. Stage `07` remains active and capture writes the foundation pack.
7. Stage `10` remains active. Compile still emits payload-only model input, capture refuses that
   raw compile payload as `invalid_capture_input`, an external model produces the completed
   `FEATURE_SPEC.md`, and capture materializes that completed output.

The skip path is valid only because both route predicates are explicit and false. It is not a
placeholder path and it is not inferred from missing text.

## Manual Branch Decision

`needs_project_context` remains an operator-owned handoff after stage `05`.

- Capture does not auto-set it.
- The operator must set it explicitly with `pipeline state set`.
- `pipeline resolve` must run again before the route is trusted for the next step.
- M4 proves both branches:
  - `needs_project_context=true` activates stage `06`
  - `needs_project_context=false` keeps stage `06` skipped only when
    `charter_gaps_detected=false` is also true

## Stage-10 Boundary

M4 locks the real stage-10 contract:

- `pipeline compile --stage stage.10_feature_spec` remains payload-only.
- The compile payload is model input, not a materialized `FEATURE_SPEC.md`.
- Passing raw `pipeline compile` payload directly into `pipeline capture` means raw `pipeline compile` payload is refused as `invalid_capture_input`.
- The completed feature spec comes from an external model response or operator-supplied completed
  markdown.
- `pipeline capture --stage stage.10_feature_spec` writes only after that completed external body
  exists.

M4 does not claim:

- downstream feature-to-slice adoption
- canonical promotion
- a new `pipeline run` surface
- a compile write mode

## M5 Downstream Adoption

M5 starts only after the truthful M4 boundary is complete.

1. The operator or proof harness finishes the M4 path through stage-10 capture.
2. `pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer`
   emits one derived bundle under
   `artifacts/handoff/feature_slice/fs-m4-foundation-journey-2026-04/`.
3. The emitted bundle contains the handoff manifest, trust matrix, read allowlist, scorecard
   metadata, and copied bundle-local inputs needed by the named consumer.
4. The repo-local proof harness validates that emitted bundle before reading it.
5. The repo-local proof harness reads only the allowlisted bundle files and writes
   `artifacts/planning/feature_slice/fs-m4-foundation-journey-2026-04/SLICE_PLAN.md`.

M5 proves:

- one named downstream consumer, `feature-slice-decomposer`
- one bundle-only happy path with repo reread fallback disabled
- one derived trust model where stage-10 `artifacts/feature_spec/FEATURE_SPEC.md` remains
  `external_manual_derived`

M5 still does not claim:

- canonical promotion of `artifacts/*`
- a generic multi-consumer framework
- a public consumer command surface inside this repo

## Deterministic Reruns

The proof surfaces keep reruns stable in two ways:

- CLI stage-10 compile tests set
  `SYSTEM_PIPELINE_COMPILE_NOW_UTC=2026-01-28T18:35:10Z`, so the compile payload uses a fixed
  `now_utc` value.
- CLI and compiler capture-preview assertions normalize the generated `capture_id` to
  `{{CAPTURE_ID}}`, so deterministic preview/apply evidence does not drift on rerun.
- The committed transcripts under
  [`tests/fixtures/foundation_flow_demo/evidence/`](../tests/fixtures/foundation_flow_demo/evidence/)
  are the operator-visible proof bundle for the happy and skip paths.

The dedicated corpus under `tests/fixtures/foundation_flow_demo/` keeps the journey local,
committed, and independent of any network call during proof execution.

The M5 extension keeps the same posture:

- the feature id is deterministic from the emitted feature spec
- the happy-path transcript now includes handoff emit, bundle validation, and consumer output
- the downstream scorecard is committed and reviewable

## Scorecard

| Area | Status | Evidence-backed conclusion |
|------|--------|----------------------------|
| Manual decisions still required | Yes | The operator still chooses and sets `needs_project_context` after stage `05`, then reruns `pipeline resolve`. |
| Model-output boundaries | Locked for M4 | Stage `10` is proved only as `compile -> external model output -> capture`; success-path tests assert compile payload and completed feature-spec output are distinct, and raw compile payload capture is refused as `invalid_capture_input`. |
| Repo rereads avoided | Bounded in M4 | The journey uses route state plus committed fixture outputs from `tests/fixtures/foundation_flow_demo/`; it does not reread the repo to reconstruct missing route truth after resolve. |
| What remains manual for M5 | Still manual / out of scope here | M4 ends at journey proof plus handoff contract. It does not prove downstream consumers or later workflow adoption beyond the captured `FEATURE_SPEC.md`. |

## M5 Scorecard

The downstream adoption scorecard is now committed at:

- [`tests/fixtures/foundation_flow_demo/evidence/m5_handoff_scorecard.md`](../tests/fixtures/foundation_flow_demo/evidence/m5_handoff_scorecard.md)

That scorecard proves the same planning job completes with:

- identical `SLICE_PLAN.md` output
- zero repo rereads on the bundle-only path
- reduced total grounding reads compared with the repo-reread baseline

## Boundaries Of The M4 Claim

This document is intentionally narrow.

It proves that the CLI, compiler, fixtures, and docs agree on one happy path, one skip path, one
manual branch decision, one truthful stage-10 external-model boundary, and one deterministic rerun
story.

It now claims exactly one bounded downstream adoption proof and no more:

- the M4 journey stops truthfully at captured stage-10 output
- the M5 extension proves one bundle-only downstream planning job from that emitted trust surface
- no other downstream consumer or canonical promotion claim is implied

## Historical Note

This artifact replaces the earlier reduced-v1 conformance review. The historical phrases below are
preserved only so the existing journey-doc drift guard can keep locating the superseded review
language until that guard is updated:

- Does the shipped reduced-v1 product actually produce the confidence -> momentum -> controlled caution arc
- The command is functionally correct and productically wrong.
- The front door is named correctly, but the shipped command still stops one step before usefulness.

## Revision Backlog

Historical backlog preserved for the same drift-guard reason:

- R1, Align `doctor` to the interaction contract
- R2, Fix `inspect` ready-path next-action semantics
- R3, Retire the old setup placeholder/help wording so runtime matches the M6 setup family
