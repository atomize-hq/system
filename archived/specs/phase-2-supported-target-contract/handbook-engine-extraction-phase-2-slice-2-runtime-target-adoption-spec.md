# Spec: Handbook Engine Extraction Phase 2 Slice 2 - Runtime Target Adoption

## Assumptions

1. Phase 2 Slice 1 is complete and approved, so its authority set now governs the runtime adoption boundary for `SupportedPipelineTarget`, `SupportedStageTarget`, `SupportedConsumerTarget`, and `SupportedTargetRegistry`.
2. The current runtime still hardcodes the approved supported wedge inside compile, capture, stage-10 provenance, and handoff modules, so Slice 2.2 must migrate those callers onto one runtime registry implementation instead of merely rewording docs.
3. The only approved runtime wedge for this slice remains:
   - compile: `pipeline.foundation_inputs` -> `stage.10_feature_spec`
   - capture: `pipeline.foundation_inputs` -> `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, `stage.10_feature_spec`
   - handoff: `pipeline.foundation_inputs` -> `stage.10_feature_spec` -> `feature-slice-decomposer`
4. `crates/compiler/src/route_state.rs` and CLI help text stay non-owner surfaces in this slice. They may observe fallout through tests, but they do not become supported-target owners here.
5. Slice 2.3 still owns template/library resolver work, so Slice 2.2 must not widen into template selection, shipped-default policy changes, or new override rules.
6. The work stays inside the existing Rust workspace and current crates; no new crate boundary or external dependency is required for this slice.

## Objective

Adopt the approved Phase 2 Slice 1 supported-target contract into live runtime compile, capture, stage-10 provenance, and handoff flows so those modules stop owning scattered hardcoded target validation.

The maintainer needs one runtime owner for target lookup and validation before later extraction work can move these flows behind cleaner crate boundaries. Success means:

- runtime compile, capture, provenance, and handoff all resolve supported targets through one approved contract
- behavior-specific refusal classifications, summaries, and recovery guidance remain local to each subsystem
- the currently supported wedge stays unchanged while moving ownership out of ad hoc constants and validators
- Slice 2.3 template/library work remains deferred

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Declarative pipeline and stage truth under `core/pipelines/**` and `core/stages/**`
- Existing authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`

## Commands

Targeted runtime adoption verification:

```bash
cargo test -p handbook-compiler --test pipeline_compile
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
cargo test -p handbook-cli --test cli_surface
```

Registry adoption spot-check:

```bash
rg -n "SupportedTargetRegistry|SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget" crates/compiler/src/pipeline.rs crates/compiler/src/pipeline_compile.rs crates/compiler/src/pipeline_capture.rs crates/compiler/src/pipeline_handoff.rs crates/compiler/src/stage_10_feature_spec_provenance.rs
```

Optional CLI/help fallout guard after handoff adoption:

```bash
cargo test -p handbook-cli --test help_drift_guard
```

Final workspace verification wall:

```bash
cargo check --workspace
cargo test --workspace
```

## Project Structure

```text
crates/compiler/src/pipeline.rs                           -> Declarative pipeline/stage catalog logic and the natural home for the runtime supported-target contract
crates/compiler/src/pipeline_compile.rs                   -> Compile target validation and compile refusal posture
crates/compiler/src/pipeline_capture.rs                   -> Capture target validation, capture stage support, and apply/preview flow
crates/compiler/src/stage_10_feature_spec_provenance.rs  -> Stage-10 provenance validation and persistence rules
crates/compiler/src/pipeline_handoff.rs                  -> Handoff emit/validate logic and consumer-target posture
crates/compiler/tests/pipeline_compile.rs                -> Compiler compile regression coverage
crates/compiler/tests/pipeline_capture.rs                -> Compiler capture and provenance regression coverage
crates/compiler/tests/pipeline_handoff.rs                -> Compiler handoff regression coverage
crates/cli/tests/cli_surface.rs                          -> Public CLI regression coverage for compile/capture/handoff flows
crates/cli/tests/help_drift_guard.rs                     -> Optional public help wording guard
core/pipelines/foundation_inputs.yaml                    -> Declarative pipeline truth for the current wedge
core/stages/*.md                                         -> Declarative stage truth for supported stage ids
docs/specs/                                              -> Slice 2.2 authority documents
```

## Code Style

Prefer one runtime registry plus thin behavior-specific adapters over duplicate hardcoded target guards in each flow.

```rust
let registry = SupportedTargetRegistry::load(repo_root)?;
let target = registry
    .resolve_compile_target(&pipeline.header.id, &stage_id)
    .map_err(|reason| PipelineCompileRefusal {
        classification: PipelineCompileRefusalClassification::UnsupportedTarget,
        summary: reason.to_string(),
        pipeline_id: Some(pipeline.header.id.clone()),
        stage_id: Some(stage_id.clone()),
        recovery: render_compile_recovery(&registry),
    })?;
```

Conventions:

- keep target lookup and validation centralized
- keep compile/capture/provenance/handoff refusal wording local to the adopting module
- preserve declarative pipeline/stage truth and code-owned validated default consumers
- prefer typed target records and helper methods over raw string comparisons spread across runtime modules
- allow runtime modules to format local supported-target lists or recovery strings from registry-backed data rather than owning separate constants

## Testing Strategy

- Framework: existing Rust integration tests in `crates/compiler/tests/` and `crates/cli/tests/`
- Primary test levels:
  - compiler integration tests for compile, capture, provenance, and handoff behavior
  - CLI regression tests for public compile/capture/handoff fallout
  - optional help drift regression once handoff adoption is complete
- Coverage focus:
  - unsupported target refusal posture remains correct after registry adoption
  - approved compile/capture/handoff wedge still succeeds unchanged
  - stage-10 provenance validation still enforces the same schema/path/hash guarantees
  - handoff manifest validation still preserves consumer, provenance, trust-class, and write-failure behavior
- Coverage expectation:
  - every packet adds or updates targeted tests for the adopter it migrates
  - no packet relies on docs-only proof once runtime adoption begins

## Slice Scope

In scope:

- implement the runtime `SupportedTargetRegistry` contract required by the Slice 2.1 authority set if live code does not already expose it
- migrate compile target validation off local hardcoded pipeline/stage constants and onto the registry-backed contract
- migrate capture supported-stage validation and any supported-stage rendering off local hardcoded owner logic and onto the registry-backed contract
- migrate stage-10 provenance supported-target validation onto the same registry-backed contract
- migrate handoff consumer/pipeline validation and manifest target validation onto the registry-backed contract
- preserve the currently approved supported wedge exactly while moving ownership
- keep behavior-specific refusal classifications, summaries, and recovery posture local to compile, capture, provenance, and handoff

Out of scope:

- expanding the supported wedge beyond the currently approved pipeline, stages, and consumer
- changing `route_state.rs` or CLI help into ownership surfaces
- defining template/library resolver behavior or override rules from Slice 2.3
- changing the declarative pipeline/stage source-of-truth model
- introducing a user-defined consumer catalog or `core/consumers/**`
- widening into Phase 4 crate extraction or broader compiler/CLI cleanup

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 2.1 authority set:
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`
- Declarative truth:
  - `core/pipelines/foundation_inputs.yaml`
  - `core/stages/04_charter_inputs.md`
  - `core/stages/05_charter_synthesize.md`
  - `core/stages/06_project_context_interview.md`
  - `core/stages/07_foundation_pack.md`
  - `core/stages/10_feature_spec.md`
- Live runtime adopters to migrate:
  - `crates/compiler/src/pipeline_compile.rs`
  - `crates/compiler/src/pipeline_capture.rs`
  - `crates/compiler/src/stage_10_feature_spec_provenance.rs`
  - `crates/compiler/src/pipeline_handoff.rs`
- Live regression evidence:
  - `crates/compiler/tests/pipeline_compile.rs`
  - `crates/compiler/tests/pipeline_capture.rs`
  - `crates/compiler/tests/pipeline_handoff.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`

## Runtime Adoption Targets For Slice 2.2

| Runtime surface | Current live ownership problem | Slice 2.2 adoption requirement |
| --- | --- | --- |
| `pipeline compile` | owns direct supported pipeline/stage constants and unsupported-target checks | resolve the compile target through `SupportedTargetRegistry` while keeping compile-specific refusal classification and recovery text local |
| `pipeline capture` | owns direct supported pipeline id, supported capture stage list, and supported-target rendering | resolve supported capture targets through `SupportedTargetRegistry` while keeping capture-specific stage/input/state posture local |
| stage-10 provenance | owns a dedicated supported-target validator tied directly to stage-10 constants | validate provenance targets through the same runtime contract used by compile/capture |
| `pipeline handoff emit` and validation | owns direct supported consumer/pipeline/stage checks and manifest validation rules | resolve the approved handoff target through `SupportedTargetRegistry` while keeping handoff-specific provenance, trust, and write posture local |

## Boundaries

- Always:
  - preserve the Slice 2.1 contract vocabulary and approved allowed pairings
  - keep declarative pipeline/stage truth as the authority for pipeline and stage identities
  - keep consumers as code-owned validated defaults in this slice
  - centralize supported-target lookup and validation in one runtime owner
  - keep refusal classifications and subsystem-specific recovery/help wording local to each adopter
  - update or add targeted regression tests with each adoption packet
- Ask first:
  - expanding the supported target set or adding a new consumer
  - introducing a new crate, external dependency, or public library API surface just to host the registry
  - moving CLI help or `route_state.rs` into the ownership boundary
  - pulling template/library resolver work forward from Slice 2.3
- Never:
  - widen beyond the approved Slice 2.1 supported wedge
  - let compile/capture/handoff/provenance each keep competing supported-target owner logic after adoption lands
  - convert declarative pipeline/stage truth into hardcoded runtime truth again
  - treat help text alone as proof that runtime adoption is complete
  - start Slice 2.3 template/library work inside Slice 2.2 packets

## Success Criteria

- Live runtime code exposes one approved supported-target owner consistent with the Slice 2.1 authority set.
- `pipeline_compile.rs`, `pipeline_capture.rs`, `stage_10_feature_spec_provenance.rs`, and `pipeline_handoff.rs` adopt that owner for supported-target lookup and validation.
- Compile, capture, provenance, and handoff keep their current behavior-specific refusal posture after adoption.
- The currently approved compile/capture/handoff wedge stays unchanged.
- CLI regression coverage still passes without making CLI help or `route_state.rs` an ownership surface.
- Slice 2.3 template/library resolver work remains fully deferred.

## Open Questions

- Should the runtime registry live inside `pipeline.rs`, or does live code truth justify a narrowly scoped sibling module while still staying inside Slice 2.2 boundaries?
- Should supported-target recovery text be rendered from registry-backed helper methods, or should each adopter keep fully local string construction after consuming typed registry results?
