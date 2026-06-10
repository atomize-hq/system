# Spec: Handbook Engine Extraction Phase 2 Slice 1 - Supported Target Contract

## Assumptions

1. Phase 1 layout and storage parameterization is complete, so Slice 2.1 may tighten the supported-target authority without reopening prior path or layout decisions.
2. Packet 2.1.1 already froze the typed supported-target contract in docs only: there was **no runtime adoption** and no production Rust behavior change in that packet.
3. Pipeline and stage identities already come from declarative catalog truth under `core/pipelines/**` and `core/stages/**`.
4. Consumer identities do **not** yet have a declarative catalog, so Slice 2.1 keeps consumers as code-owned validated defaults.
5. Packet 2.1.2 freezes the lookup-and-validation owner plus one hardcoded-target evidence ledger for that already-approved wedge, still without runtime adoption.
6. Slice 2.2 owns runtime adoption of the registry in runtime flows, and Slice 2.3 stays template/library-only.

## Objective

Freeze the lookup-and-validation ownership boundary for the already-approved Slice 2.1 supported-target contract so later work can adopt one approved owner instead of rediscovering supported targets across compile, capture, provenance, handoff, CLI help, or adjacent runtime-state surfaces.

Packet 2.1.2 must answer, in docs only:

- which surface owns supported-target lookup and validation right now
- which live hardcoded sites make up the current supported wedge evidence ledger
- which truth model continues to govern those targets right now
- which follow-on work is explicitly deferred out of this packet

## Slice Scope

In scope:

- preserve the Packet 2.1.1 contract vocabulary and allowed pairings without renaming them
- explicitly freeze `SupportedTargetRegistry` as the lookup-and-validation owner for the supported-target contract
- explicitly freeze one hardcoded-target evidence ledger covering compile, capture, stage-10 provenance, handoff, CLI help, and adjacent runtime-state evidence
- explicitly state that pipeline and stage support comes from declarative catalog truth
- explicitly state that consumers remain code-owned validated defaults
- explicitly state that `crates/compiler/src/route_state.rs` and CLI help text are observed/non-owner surfaces
- explicitly freeze that Slice 2.2 owns runtime adoption and Slice 2.3 stays template/library-only

Out of scope:

- redefining or renaming the Packet 2.1.1 supported-target vocabulary
- migrating compile, capture, provenance, or handoff runtime callers
- migrating `route_state.rs` or CLI help to read from the registry
- changing production Rust behavior
- introducing a declarative consumer schema or new `core/consumers/**` tree
- widening into Slice 2.2 runtime adoption or Slice 2.3 template/library resolver work
- adding new pipelines, stages, or downstream consumers beyond the currently supported set

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Declarative target sources:
  - `core/pipelines/foundation_inputs.yaml`
  - `core/stages/04_charter_inputs.md`
  - `core/stages/05_charter_synthesize.md`
  - `core/stages/06_project_context_interview.md`
  - `core/stages/07_foundation_pack.md`
  - `core/stages/10_feature_spec.md`
- Current hardcoded-target evidence surfaces for Packet 2.1.2:
  - `crates/compiler/src/pipeline_compile.rs`
  - `crates/compiler/src/pipeline_capture.rs`
  - `crates/compiler/src/pipeline_handoff.rs`
  - `crates/compiler/src/stage_10_feature_spec_provenance.rs`
  - `crates/cli/src/main.rs`
  - `crates/compiler/src/route_state.rs`

## Commands

Live evidence verification:

```bash
rg -n "pipeline\.foundation_inputs|stage\.10_feature_spec|feature-slice-decomposer" crates/compiler/src crates/cli/src
```

Owner-boundary verification:

```bash
rg -n "SupportedTargetRegistry|declarative pipeline/stage|code-owned validated defaults|Slice 2\.2|Slice 2\.3" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
```

Non-owner-surface verification:

```bash
rg -n "route_state\.rs|CLI help|hardcoded-target evidence" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
```

## Supported Target Contract Frozen By Packet 2.1.1

### Contract types

| Contract type | Frozen meaning in Packet 2.1.1 | Source of truth |
| --- | --- | --- |
| `SupportedPipelineTarget` | One supported pipeline identity plus its declared stage membership | declarative catalog truth |
| `SupportedStageTarget` | One supported stage identity plus its declared source and participating pipelines | declarative catalog truth |
| `SupportedConsumerTarget` | One supported downstream consumer plus its allowed producer pairing notes | code-owned validated defaults |

### Allowed pairings frozen for later runtime adoption

| Runtime surface | Allowed pairings |
| --- | --- |
| `pipeline compile` | `pipeline.foundation_inputs` -> `stage.10_feature_spec` |
| `pipeline capture` | `pipeline.foundation_inputs` -> `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, `stage.10_feature_spec` |
| stage-10 provenance | `pipeline.foundation_inputs` -> `stage.10_feature_spec` |
| `pipeline handoff emit` | `pipeline.foundation_inputs` -> `stage.10_feature_spec` -> `feature-slice-decomposer` |

### Truth model frozen for Slice 2.1

- Pipeline truth is declarative catalog truth.
- Stage truth is declarative catalog truth.
- Consumer truth is code-owned validated defaults.
- Packet 2.1.1 freezes contract vocabulary and allowed pairings only; there is **no runtime adoption** in this packet.

## Packet 2.1.2 Lookup And Validation Owner Frozen

### Ownership boundary frozen by Packet 2.1.2

- `SupportedTargetRegistry` is the owner for supported-target lookup and validation across `SupportedPipelineTarget`, `SupportedStageTarget`, `SupportedConsumerTarget`, and the frozen allowed pairings.
- `SupportedTargetRegistry` keeps declarative pipeline/stage truth as the authority for pipeline and stage identities.
- `SupportedTargetRegistry` keeps consumers as code-owned validated defaults until a later consumer-catalog ask exists.
- Compile, capture, provenance, and handoff runtime files remain current hardcoded adopters/evidence, not competing owners, until Slice 2.2 adopts the registry in runtime flows.
- `crates/compiler/src/route_state.rs` and CLI help text are explicit non-owner surfaces: they expose adjacent runtime-state or help wording, but they do not own supported-target lookup or validation.
- Slice 2.2 owns runtime adoption of `SupportedTargetRegistry`. Slice 2.3 stays template/library-only and does not own runtime lookup or validation.

### Hardcoded-target evidence ledger frozen by Packet 2.1.2

| Surface | Current live evidence | Packet 2.1.2 ownership posture |
| --- | --- | --- |
| `crates/compiler/src/pipeline_compile.rs` | hardcodes `pipeline.foundation_inputs` -> `stage.10_feature_spec` compile support | evidence-only adopter now; Slice 2.2 later reads from `SupportedTargetRegistry` |
| `crates/compiler/src/pipeline_capture.rs` | hardcodes the supported capture stage set for `pipeline.foundation_inputs` | evidence-only adopter now; Slice 2.2 later reads from `SupportedTargetRegistry` |
| `crates/compiler/src/stage_10_feature_spec_provenance.rs` | hardcodes `pipeline.foundation_inputs` -> `stage.10_feature_spec` provenance support | evidence-only adopter now; Slice 2.2 later reads from `SupportedTargetRegistry` |
| `crates/compiler/src/pipeline_handoff.rs` | hardcodes `pipeline.foundation_inputs` -> `stage.10_feature_spec` -> `feature-slice-decomposer` handoff support | evidence-only adopter now; Slice 2.2 later reads from `SupportedTargetRegistry` |
| `crates/cli/src/main.rs` | CLI help text and producer-command wording describe the supported capture and handoff wedge | observed help surface only; explicit non-owner |
| `crates/compiler/src/route_state.rs` | adjacent runtime-state evidence references `pipeline.foundation_inputs` state paths and capture-cache reset paths | observed runtime-state surface only; explicit non-owner |

## Boundaries

- Always:
  - preserve the Packet 2.1.1 contract vocabulary and allowed pairings exactly as already frozen
  - keep Packet 2.1.2 docs-only
  - keep pipeline/stage support anchored to declarative pipeline/stage truth
  - keep consumers as code-owned validated defaults
  - treat `SupportedTargetRegistry` as the lookup-and-validation owner
  - keep `crates/compiler/src/route_state.rs` and CLI help text as explicit non-owner surfaces
  - preserve current runtime behavior by not editing production Rust behavior in this packet
  - defer runtime adoption to Slice 2.2 and keep Slice 2.3 template/library-only
- Ask first:
  - any declarative consumer schema or new `core/consumers/**` tree
  - any new supported pipeline, stage, or consumer identity
  - any attempt to pull runtime adoption or template/library resolver behavior forward into Packet 2.1.2
- Never:
  - widen into Slice 2.2 runtime adoption
  - widen into Slice 2.3 template/library resolver work
  - redefine consumer support as user-defined or schema-driven in this packet
  - promote `crates/compiler/src/route_state.rs` or CLI help text into ownership surfaces
  - touch production Rust behavior for this packet

## Success Criteria

- Slice 2.1 preserves the Packet 2.1.1 contract vocabulary and allowed pairings unchanged.
- Slice 2.1 explicitly names `SupportedTargetRegistry` as the lookup-and-validation owner.
- Slice 2.1 explicitly keeps pipeline/stage truth as declarative pipeline/stage truth.
- Slice 2.1 explicitly keeps consumers as code-owned validated defaults.
- Slice 2.1 includes one hardcoded-target evidence ledger covering compile, capture, stage-10 provenance, handoff, CLI help, and adjacent runtime-state evidence.
- Slice 2.1 explicitly marks `crates/compiler/src/route_state.rs` and CLI help text as non-owner surfaces.
- Slice 2.1 explicitly states Slice 2.2 owns runtime adoption and Slice 2.3 stays template/library-only.

## Deferred Follow-on

- Slice 2.2 may adopt `SupportedTargetRegistry` in runtime compile/capture/provenance/handoff flows and adjacent runtime surfaces, but that work is not part of this packet.
- Slice 2.3 may own template/library resolver work, but that work is not part of this packet.
