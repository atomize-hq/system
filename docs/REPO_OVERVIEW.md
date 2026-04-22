# Repository Overview

## What This Repo Is

`system` is a Rust-first planning compiler for agent-assisted software delivery.

Its job is to take canonical project inputs from a repo-local `.system/` directory, resolve the right planning path, compile stage-specific model payloads, capture completed outputs back into declared artifacts, and emit downstream handoff bundles that other planning tools can trust.

In practical terms, this repo exists to remove a specific class of operator pain:

- repeated repo archaeology before every planning step
- manual copy/paste of context between planning stages
- inconsistent planning outputs across runs
- fragile human babysitting of multi-step planning flows

The current shipped wedge is the baseline + foundation-family planning flow. Setup scaffolds the baseline truth set under `.system/`, authoring replaces starter content with canonical baseline truth, `doctor` classifies baseline readiness, and the packet path continues into foundation-flow artifacts and feature-spec handoff material.

## The Problem It Solves

Without a compiler-owned planning spine, an operator has to keep rebuilding context manually:

- figure out which planning stage should run next
- decide whether conditional stages should activate
- assemble the right prompt inputs for that stage
- manage state between stages
- validate whether the output is complete and fresh
- write the result into the correct artifact locations
- prepare a clean bundle for downstream consumers

This repo centralizes those responsibilities inside a deterministic CLI. Instead of treating planning as an ad hoc chat loop, it treats planning as a controlled pipeline with explicit inputs, routing, capture rules, freshness checks, and proof surfaces.

## Current Product Shape

The operator-facing surface is the `system` CLI. The main commands are:

- `setup` to initialize or refresh canonical `.system/` inputs
- `author` to replace setup-owned baseline scaffolding with canonical truth
- `pipeline` to list, inspect, resolve, compile, capture, hand off, and mutate narrow route state
- `generate` to produce supported packets from canonical inputs
- `inspect` to render proof and decision evidence
- `doctor` to explain blockers, baseline readiness, and next safe actions

The most important workflow in the repo today is the `pipeline.foundation_inputs` route:

1. Resolve the active route.
2. Capture completed stage outputs into declared artifacts.
3. Compile the feature-spec stage payload for an external model step.
4. Capture the finished feature spec.
5. Emit a downstream handoff bundle for the next planning consumer.

That makes the repo a compiler/orchestration layer for planning work, not a general execution runtime.

## Short Tech Overview

The implementation is a small Rust workspace with a deliberate split:

- `crates/cli`: thin command-line entrypoint, argument parsing, help text, and command dispatch
- `crates/compiler`: core setup, pipeline loading, route resolution, compile/capture logic, handoff emission, rendering, state handling, and refusal modeling

The compiler is driven by declarative repo content:

- `pipelines/`: pipeline definitions that declare stages, activation rules, and defaults
- `core/stages/`: stage source documents used to assemble planning inputs
- `core/library/`: reusable templates and directive content
- `core/schemas/`: structured YAML contracts for generated artifacts
- `docs/contracts/`: the authoritative behavioral contracts for the shipped CLI surface

At runtime, the system works roughly like this:

1. Load pipeline definitions and canonical `.system/` inputs.
2. Resolve the active route and persist route basis/state.
3. Compile one stage into a typed payload or proof surface.
4. Validate and capture completed stage output into declared artifacts.
5. Emit derived handoff bundles for downstream planning consumers.

The repo also includes strong verification rails:

- CLI surface tests for help and command behavior
- compiler tests for routing, freshness, capture, handoff, manifests, and refusals
- fixture-backed proof corpora and golden outputs for supported planning flows

## Repo Landmarks

- `crates/cli/` for the binary entrypoint
- `crates/compiler/` for the core planning compiler
- `pipelines/` for declared planning routes
- `core/` for stages, templates, rules, overlays, and schemas
- `docs/contracts/` for product and behavior contracts
- `tests/fixtures/` for proof fixtures and golden-path validation

## Bottom Line

This repo provides the deterministic planning spine for turning canonical project inputs into reusable planning artifacts and trusted downstream handoffs, with Rust owning the command surface, routing logic, capture rules, and proof story end to end.
