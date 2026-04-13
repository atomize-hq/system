# Base Context

## Project
- Name: System
- Repo / Project Ref: `system`
- Owner: Platform Foundations
- Team: Build Systems

## Problem Statement
The repository is migrating planning and pipeline behavior from the legacy Python harness
to a Rust-owned CLI/compiler surface. `M2` introduces bounded stage compilation with a
proof-first refusal model and a single supported compile target for the
`pipeline.foundation_inputs` route.

## Delivery Snapshot
- Active milestone: `M2` Compilation Capability
- Locked compile target: `stage.10_feature_spec`
- Current delivery concern: keep compile truthful by refusing stale or incomplete inputs
  instead of reconstructing missing route state or synthesizing success output.

## Repository Shape
- `crates/compiler`: pipeline loading, routing, compile contracts, refusal surfaces
- `crates/cli`: operator-facing `pipeline` commands and rendered proof surfaces
- `core/stages`: stage metadata and compile-time include/input declarations
- `core/library`: shared directives and templates used during stage compilation
- `tests/fixtures/pipeline_proof_corpus`: shared committed proof corpus for compiler and CLI tests

## Constraints
- Canonical pipeline and stage ids are the source of truth.
- `pipeline resolve` owns persisted route basis state.
- `pipeline compile` must not rerun resolve implicitly.
- Help/docs only advertise compile once the full `M2` slice is coherent and green.
