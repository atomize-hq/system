# Spec: Handbook Engine Extraction Phase 2 Slice 1 - Supported Target Contract

## Assumptions

1. Phase 1 layout and storage parameterization is complete, so Packet 2.1.1 may tighten the supported-target authority without reopening prior path or layout decisions.
2. Slice 2.1 is **contract-only** at this stage: there is **no runtime adoption** in Packet 2.1.1, and there is no production Rust behavior change in this packet.
3. Pipeline and stage identities already come from declarative catalog truth under `core/pipelines/**` and `core/stages/**`.
4. Consumer identities do **not** yet have a declarative catalog, so Packet 2.1.1 freezes consumers as code-owned validated defaults.
5. Packet 2.1.2 registry-owner and evidence-ledger work is deferred until after this Packet 2.1.1 contract is approved.
6. Runtime adoption is deferred to Slice 2.2, and template/library resolver work is deferred to Slice 2.3.

## Objective

Freeze the typed supported-target contract for Phase 2 Slice 1 so later work can adopt one already-approved vocabulary instead of re-deciding supported targets inside compile, capture, provenance, or handoff flows.

Packet 2.1.1 must answer, in docs only:

- which supported target types exist right now
- which pairings are allowed right now
- which truth model governs those targets right now
- which follow-on work is explicitly deferred out of this packet

## Slice Scope

In scope:

- explicitly freeze `SupportedPipelineTarget`, `SupportedStageTarget`, and `SupportedConsumerTarget`
- explicitly freeze the allowed pairings for `pipeline compile`, `pipeline capture`, stage-10 provenance, and `pipeline handoff emit`
- explicitly state that pipeline and stage support comes from declarative catalog truth
- explicitly state that consumers remain code-owned validated defaults
- explicitly freeze the Slice 2.1 no-runtime-adoption boundary
- explicitly defer Packet 2.1.2 registry-owner/evidence-ledger work, Slice 2.2 runtime adoption, and Slice 2.3 template/library resolver work

Out of scope:

- choosing or documenting the registry owner in detail
- building the hardcoded-target evidence ledger
- migrating compile, capture, provenance, or handoff runtime callers
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
- Current runtime surfaces whose future adoption is deferred to Slice 2.2:
  - `crates/compiler/src/pipeline_compile.rs`
  - `crates/compiler/src/pipeline_capture.rs`
  - `crates/compiler/src/pipeline_handoff.rs`
  - `crates/compiler/src/stage_10_feature_spec_provenance.rs`

## Commands

Contract-freeze verification:

```bash
rg -n "SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget|Allowed pairings|pipeline compile|pipeline capture|pipeline handoff emit" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
```

Boundary verification:

```bash
rg -n "no runtime adoption|Slice 2\.2|Slice 2\.3|Out of scope" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
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

## Boundaries

- Always:
  - keep Slice 2.1 Packet 2.1.1 docs-only
  - keep pipeline/stage support anchored to declarative catalog truth
  - keep consumers as code-owned validated defaults
  - preserve current runtime behavior by not editing production Rust behavior in this packet
  - defer runtime adoption to Slice 2.2 and template/library resolver work to Slice 2.3
- Ask first:
  - any declarative consumer schema or new `core/consumers/**` tree
  - any new supported pipeline, stage, or consumer identity
  - any attempt to pull registry-owner or evidence-ledger design forward into Packet 2.1.1
- Never:
  - widen into Slice 2.2 runtime adoption
  - widen into Slice 2.3 template/library resolver work
  - redefine consumer support as user-defined or schema-driven in this packet
  - touch production Rust behavior for this packet

## Success Criteria

- Slice 2.1 explicitly freezes `SupportedPipelineTarget`, `SupportedStageTarget`, and `SupportedConsumerTarget`.
- Slice 2.1 explicitly freezes the allowed pairings for `pipeline compile`, `pipeline capture`, stage-10 provenance, and `pipeline handoff emit`.
- Slice 2.1 explicitly states pipeline/stage truth as declarative catalog truth.
- Slice 2.1 explicitly states consumers as code-owned validated defaults.
- Slice 2.1 explicitly states there is **no runtime adoption** in Packet 2.1.1.
- Slice 2.1 explicitly defers registry-owner/evidence-ledger work to Packet 2.1.2, runtime adoption to Slice 2.2, and template/library resolver work to Slice 2.3.

## Deferred Follow-on

- Packet 2.1.2 may choose the registry owner and assemble the evidence ledger, but that work is not part of this packet.
- Slice 2.2 may adopt this frozen contract in runtime compile/capture/provenance/handoff flows, but that work is not part of this packet.
- Slice 2.3 may own template/library resolver work, but that work is not part of this packet.
