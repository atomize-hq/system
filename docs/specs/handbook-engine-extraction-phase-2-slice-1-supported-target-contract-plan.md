# Plan: Handbook Engine Extraction Phase 2 Slice 1 - Supported Target Contract

## Objective

Land Packet 2.1.2 as a thin docs-only slice that freezes `SupportedTargetRegistry` as the lookup-and-validation owner and records one hardcoded-target evidence ledger for the already-approved Packet 2.1.1 contract, without starting runtime adoption or template/library resolver work.

Spec reference: [handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md](./handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md)

## Major Artifacts

1. Packet 2.1.1 contract baseline
   - preserves `SupportedPipelineTarget`, `SupportedStageTarget`, and `SupportedConsumerTarget`
   - preserves the allowed pairings for compile, capture, provenance, and handoff

2. SupportedTargetRegistry ownership boundary
   - freezes `SupportedTargetRegistry` as the lookup-and-validation owner
   - keeps declarative pipeline/stage truth and consumer truth as code-owned validated defaults

3. Hardcoded-target evidence ledger
   - covers compile, capture, stage-10 provenance, handoff, CLI help, and adjacent runtime-state evidence
   - marks `route_state.rs` and CLI help as explicit non-owner surfaces

4. Boundary statement
   - repeats that Slice 2.2 owns runtime adoption
   - repeats that Slice 2.3 stays template/library-only
   - keeps Packet 2.1.2 docs-only and behavior-neutral

## Order

### Packet 2.1.1 already frozen: prerequisite contract baseline

Why this remains the prerequisite:

- later packets need stable supported-target vocabulary before they can safely add ownership or adoption details
- the currently supported compile/capture/provenance/handoff wedge must be frozen once, in one authority set, before any follow-on packet uses it
- the declarative catalog already provides the right pipeline/stage truth source for this packet

Output:

- one approved contract family: `SupportedPipelineTarget`, `SupportedStageTarget`, `SupportedConsumerTarget`
- one approved allowed-pairings table
- one approved truth model: declarative pipeline/stage truth plus code-owned validated default consumers
- one approved no-runtime-adoption boundary for Slice 2.1 Packet 2.1.1

### Packet 2.1.2 now: Freeze registry owner and hardcoded-target evidence

Why now:

- the current supported wedge already exists in hardcoded runtime/help/state sites, but those sites do not yet have one frozen owner boundary
- later runtime adoption needs one approved owner before code changes begin
- `route_state.rs` and CLI help must be called explicit non-owner surfaces before Slice 2.2 starts using the registry

Output:

- one approved owner: `SupportedTargetRegistry`
- one approved hardcoded-target evidence ledger covering compile, capture, stage-10 provenance, handoff, CLI help, and adjacent runtime-state evidence
- one approved truth model: declarative pipeline/stage truth plus code-owned validated default consumers
- one approved boundary: Slice 2.2 adopts the registry in runtime flows and Slice 2.3 stays template/library-only

### Deferred after Packet 2.1.2: Slice 2.2 and Slice 2.3

This plan intentionally does **not** perform those follow-on steps. It only keeps their boundaries explicit:

- Slice 2.2 is where runtime adoption may happen later
- Slice 2.3 is where template/library resolver work may happen later

## Risks And Mitigations

### Risk: the contract is too weak to guide later adoption

Mitigation:

- freeze the concrete live pairings already in scope today
- freeze the three target types explicitly instead of describing them informally

### Risk: ownership stays ambiguous after Packet 2.1.2

Mitigation:

- name `SupportedTargetRegistry` explicitly as the owner
- keep one hardcoded-target evidence ledger for the supported wedge instead of leaving ownership distributed

### Risk: Slice 2.1 leaks into Slice 2.2 runtime adoption

Mitigation:

- repeat that Slice 2.2 owns runtime adoption
- keep the packet docs-only and behavior-neutral

### Risk: Slice 2.1 leaks into Slice 2.3 template/library work

Mitigation:

- keep Slice 2.3 explicitly template/library-only
- reject ownership wording that exists only to solve resolver behavior

### Risk: route_state.rs or CLI help is mistaken for the long-term owner

Mitigation:

- record both surfaces inside the hardcoded-target evidence ledger
- mark `route_state.rs` and CLI help as explicit non-owner surfaces

### Risk: the packet invents a declarative consumer schema too early

Mitigation:

- keep consumers as code-owned validated defaults
- treat any `core/consumers/**` design as future ask-first work

## Verification Checkpoints

### Checkpoint 1: Live hardcoded-target evidence ledger is complete

Confirm live repo evidence covers:

- `pipeline.foundation_inputs`
- `stage.10_feature_spec`
- `feature-slice-decomposer`

Suggested verification:

```bash
rg -n "pipeline\.foundation_inputs|stage\.10_feature_spec|feature-slice-decomposer" crates/compiler/src crates/cli/src
```

### Checkpoint 2: Owner boundary freeze complete

Confirm the authority set contains:

- `SupportedTargetRegistry`
- `declarative pipeline/stage`
- `code-owned validated defaults`
- `Slice 2.2`
- `Slice 2.3`

Suggested verification:

```bash
rg -n "SupportedTargetRegistry|declarative pipeline/stage|code-owned validated defaults|Slice 2\.2|Slice 2\.3" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
```

### Checkpoint 3: Non-owner surface freeze complete

Confirm the authority set contains:

- `route_state.rs`
- `CLI help`
- `hardcoded-target evidence`

Suggested verification:

```bash
rg -n "route_state\.rs|CLI help|hardcoded-target evidence" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
```

## Exit Conditions

Packet 2.1.2 is ready for human review when:

- the Packet 2.1.1 supported-target contract stays explicit enough that later work does not need to rename or rediscover the target vocabulary
- `SupportedTargetRegistry` is frozen clearly enough that later runtime adoption does not need to rediscover ownership
- the truth model clearly separates declarative pipeline/stage truth from code-owned validated default consumers
- the docs clearly state that `route_state.rs` and CLI help are non-owner surfaces recorded only as hardcoded-target evidence
- any Slice 2.2 and Slice 2.3 work is clearly deferred rather than partially started
