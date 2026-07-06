# Plan: Handbook Engine Extraction Phase 2 Slice 2 - Runtime Target Adoption

## Objective

Adopt the approved supported-target contract into live compile, capture, stage-10 provenance, and handoff runtime flows so the current hardcoded wedge stops being owned independently by each runtime module.

Spec reference: [handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md](./handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md)

## Major Modules

1. Runtime supported-target owner
   - likely centered in `crates/compiler/src/pipeline.rs` or a tightly adjacent internal module
   - owns runtime lookup and validation for `SupportedPipelineTarget`, `SupportedStageTarget`, `SupportedConsumerTarget`, and approved pairings

2. Compile adopter
   - `crates/compiler/src/pipeline_compile.rs`
   - consumes the runtime target owner for compile target validation while keeping compile refusal posture local

3. Capture and provenance adopters
   - `crates/compiler/src/pipeline_capture.rs`
   - `crates/compiler/src/stage_10_feature_spec_provenance.rs`
   - consume the runtime target owner for supported capture stages and stage-10 provenance target validation while keeping capture/provenance rules local

4. Handoff adopter
   - `crates/compiler/src/pipeline_handoff.rs`
   - consumes the runtime target owner for pipeline/consumer/stage validation while keeping handoff manifest, provenance, and trust posture local

5. Regression layer
   - `crates/compiler/tests/pipeline_compile.rs`
   - `crates/compiler/tests/pipeline_capture.rs`
   - `crates/compiler/tests/pipeline_handoff.rs`
   - `crates/cli/tests/cli_surface.rs`
   - optionally `crates/cli/tests/help_drift_guard.rs`

## Dependencies And Order

### Prerequisite: Slice 2.1 authority stays frozen

Why first:

- Slice 2.2 is an adoption slice, not a contract-redesign slice
- runtime code must implement the already approved vocabulary, truth model, and allowed pairings rather than inventing new ones mid-flight

Output:

- one frozen contract vocabulary
- one frozen supported wedge
- one frozen truth model: declarative pipeline/stage truth plus code-owned validated default consumers

### Packet 2.2.1: Runtime registry implementation and compile adoption

Why first:

- compile is the simplest single-stage adopter and proves the registry can power a real runtime flow
- handoff and stage-10 provenance both depend on stage-10 compile semantics, so compile should adopt the contract before they do

Output:

- one runtime `SupportedTargetRegistry` implementation or equivalent owner consistent with Slice 2.1
- compile target validation migrated off direct local constants and onto the runtime owner
- compile refusal classifications and recovery posture preserved

### Packet 2.2.2: Capture and provenance adoption

Why second:

- capture owns the widest stage list in the approved wedge and therefore proves the registry can express more than the single compile target
- stage-10 provenance should consume the same target contract once compile and capture both rely on it

Output:

- capture supported-stage validation and supported-stage rendering move onto the runtime owner
- stage-10 provenance target validation moves onto the runtime owner
- capture/provenance still own their local state, schema, input-shape, and persistence rules

### Packet 2.2.3: Handoff adoption and closeout

Why third:

- handoff depends on compile semantics and stage-10 provenance remaining correct
- handoff is the last runtime owner still hardcoding the approved consumer and manifest-target checks

Output:

- handoff consumer and manifest target validation move onto the runtime owner
- handoff keeps local ownership of provenance matching, trust-class behavior, bundle layout, and write failures
- final slice regression evidence proves the supported wedge still behaves the same after runtime adoption

## Risks And Mitigations

### Risk: Slice 2.2 quietly redesigns Slice 2.1

Mitigation:

- treat Slice 2.1 docs as the runtime authority contract
- reject any packet that adds new pipeline, stage, or consumer support without explicit approval

### Risk: the runtime owner is implemented, but adopters still keep duplicate hardcoded guards

Mitigation:

- require each adopter packet to remove or demote its local supported-target owner logic
- use targeted tests plus code review to confirm the registry is the only supported-target owner

### Risk: refusal wording regresses while target ownership improves

Mitigation:

- keep refusal classifications and message construction local to each subsystem
- verify compiler and CLI tests after every packet, not just at the end

### Risk: capture and handoff adoption leak into Slice 2.3 template/library work

Mitigation:

- keep the registry focused on supported targets and approved pairings only
- defer any template/library selection rules or shipped-default posture changes to Slice 2.3

### Risk: CLI help or `route_state.rs` gets pulled into ownership by convenience

Mitigation:

- treat those files as regression observers only in this slice
- use CLI tests to detect fallout without moving ownership there

## Parallel Vs Sequential

Sequential:

- runtime target owner before any adopter migration
- compile adoption before provenance and handoff
- capture/provenance before handoff closeout

Parallel opportunities after Packet 2.2.1:

- capture adoption and its targeted tests can proceed in parallel with provenance helper updates if both consume the same landed runtime owner
- CLI regression updates can land alongside the packet that actually changes the observed public behavior

## Verification Checkpoints

### Checkpoint 1: Runtime owner and compile adoption

```bash
cargo test -p handbook-compiler --test pipeline_compile
cargo test -p handbook-cli --test cli_surface
```

### Checkpoint 2: Capture and provenance adoption

```bash
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-cli --test cli_surface
```

### Checkpoint 3: Handoff adoption

```bash
cargo test -p handbook-compiler --test pipeline_handoff
cargo test -p handbook-cli --test cli_surface
```

### Contract-adoption spot-check

```bash
rg -n "SupportedTargetRegistry|SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget" crates/compiler/src/pipeline.rs crates/compiler/src/pipeline_compile.rs crates/compiler/src/pipeline_capture.rs crates/compiler/src/pipeline_handoff.rs crates/compiler/src/stage_10_feature_spec_provenance.rs
```

### Final checkpoint

```bash
cargo check --workspace
cargo test --workspace
```

Optional final public-help guard:

```bash
cargo test -p handbook-cli --test help_drift_guard
```

## Exit Conditions

The slice is ready for human review when:

- one runtime supported-target owner exists and matches the Slice 2.1 authority set
- compile, capture, stage-10 provenance, and handoff all consume that owner for supported-target lookup and validation
- compile/capture/provenance/handoff still keep their local refusal and behavior posture
- the approved supported wedge remains unchanged
- no Slice 2.3 template/library work leaked in
