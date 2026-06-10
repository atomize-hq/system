# Tasks: Handbook Engine Extraction Phase 2 Slice 1 - Supported Target Contract

Plan reference: [handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md](./handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md)

## Packet 2.1.1 Prerequisite: Typed Target And Consumer Contract

Packet 2.1.1 already froze the supported target vocabulary for Slice 2.1: define `SupportedPipelineTarget`, `SupportedStageTarget`, and `SupportedConsumerTarget`; freeze the currently allowed compile/capture/handoff/provenance pairings; keep pipeline/stage truth declarative catalog truth; keep consumers as code-owned validated defaults; and allow **no runtime adoption** in Slice 2.1.

- Packet 2.1.2 must preserve this vocabulary and these pairings exactly as already frozen.

## Packet 2.1.2: Target Registry Lookup And Validation Owner

Packet 2.1.2 freezes `SupportedTargetRegistry` as the lookup-and-validation owner for Slice 2.1. It preserves declarative pipeline/stage truth, preserves consumers as code-owned validated defaults, records one hardcoded-target evidence ledger for the current supported wedge, marks `route_state.rs` and CLI help as explicit non-owner surfaces, and defers runtime adoption to Slice 2.2 while keeping Slice 2.3 template/library-only.

- [ ] Task: Freeze the `SupportedTargetRegistry` ownership boundary across the authority set
  - Acceptance: The spec, plan, and tasks docs all state that `SupportedTargetRegistry` owns supported-target lookup and validation, that pipeline/stage support stays declarative pipeline/stage truth, that consumers stay code-owned validated defaults, that Slice 2.2 owns runtime adoption, and that Slice 2.3 stays template/library-only.
  - Verify: `rg -n "SupportedTargetRegistry|declarative pipeline/stage|code-owned validated defaults|Slice 2\.2|Slice 2\.3" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`, `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md`, `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`

- [ ] Task: Freeze one hardcoded-target evidence ledger for the current supported wedge
  - Acceptance: The authority set records compile, capture, stage-10 provenance, handoff, CLI help, and adjacent runtime-state evidence, and it states that `crates/compiler/src/route_state.rs` and CLI help are explicit non-owner surfaces rather than the long-term owner.
  - Verify: `rg -n "pipeline\.foundation_inputs|stage\.10_feature_spec|feature-slice-decomposer" crates/compiler/src crates/cli/src` and `rg -n "route_state\.rs|CLI help|hardcoded-target evidence" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`, `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md`, `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`

## Deferred Follow-on (Not Part Of Packet 2.1.2)

- Slice 2.2 runtime adoption is deferred; it is where compile, capture, provenance, handoff, and adjacent runtime flows may later adopt `SupportedTargetRegistry`.
- Slice 2.3 template/library resolver work is deferred; it stays template/library-only.

## Human Review Gate

Stop after Packet 2.1.2 acceptance. Do not start Slice 2.2 or Slice 2.3 implementation work until the human has reviewed and approved this Packet 2.1.2 authority set.
