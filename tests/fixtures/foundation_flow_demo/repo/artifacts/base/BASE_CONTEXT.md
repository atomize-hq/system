# Base Context

## Project
- Name: System
- Repo / Project Ref: `system`
- Owner: Platform Foundations
- Team: Build Systems

## Problem Statement
The repository is proving the `M4` foundation journey: a realistic route progression from
charter inputs through foundation-pack synthesis and a truthful stage-10 handoff where
`pipeline compile` produces model input while `pipeline capture` consumes a completed
external `FEATURE_SPEC.md`.

## Delivery Snapshot
- Active milestone: `M4` Foundation Journey Proof And Handoff Contract
- Locked compile target: `stage.10_feature_spec`
- Journey requirement: prove one happy path, one skip path, and one deterministic rerun story
- Current delivery concern: remove the invalid `compile | capture` shortcut from tests and docs

## Repository Shape
- `crates/compiler`: pipeline loading, route-state validation, compile/capture contracts
- `crates/cli`: operator-facing `pipeline` commands and rendered proof surfaces
- `core/stages`: stage metadata and activation rules for the foundation route
- `core/library`: directives and templates for charter, context, foundation, and feature-spec work
- `tests/fixtures/foundation_flow_demo`: dedicated M4 demo corpus for journey-proof tests

## Constraints
- Canonical pipeline and stage ids are the source of truth.
- `pipeline resolve` owns persisted route basis state.
- `pipeline compile` must remain payload-only and must not materialize `FEATURE_SPEC.md`.
- Stage 10 capture is valid only after an external model response exists.
- The demo corpus stays separate from `pipeline_proof_corpus` and `execution_demo`.
