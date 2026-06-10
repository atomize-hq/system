# Plan: Handbook Engine Extraction Phase 2 Slice 1 - Supported Target Contract

## Objective

Land Packet 2.1.1 as a thin docs-only slice that freezes the supported-target contract without starting registry-owner work, runtime adoption, or template/library resolver work.

Spec reference: [handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md](./handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md)

## Major Artifacts

1. Supported target contract
   - freezes `SupportedPipelineTarget`, `SupportedStageTarget`, and `SupportedConsumerTarget`
   - freezes the allowed pairings for compile, capture, provenance, and handoff

2. Truth-model statement
   - keeps pipeline/stage truth declarative catalog truth
   - keeps consumer truth as code-owned validated defaults

3. Boundary statement
   - repeats that there is **no runtime adoption** in Packet 2.1.1
   - repeats that Slice 2.2 owns runtime adoption
   - repeats that Slice 2.3 owns template/library resolver work

## Order

### Packet 2.1.1 now: Freeze the typed target and consumer contract

Why now:

- later packets need stable supported-target vocabulary before they can safely add ownership or adoption details
- the currently supported compile/capture/provenance/handoff wedge must be frozen once, in one authority set, before any follow-on packet uses it
- the declarative catalog already provides the right pipeline/stage truth source for this packet

Output:

- one approved contract family: `SupportedPipelineTarget`, `SupportedStageTarget`, `SupportedConsumerTarget`
- one approved allowed-pairings table
- one approved truth model: declarative pipeline/stage truth plus code-owned validated default consumers
- one approved no-runtime-adoption boundary for Slice 2.1 Packet 2.1.1

### Deferred after review: Packet 2.1.2, Slice 2.2, and Slice 2.3

This plan intentionally does **not** perform those follow-on steps. It only keeps their boundaries explicit:

- Packet 2.1.2 is where registry-owner and evidence-ledger wording may be added later
- Slice 2.2 is where runtime adoption may happen later
- Slice 2.3 is where template/library resolver work may happen later

## Risks And Mitigations

### Risk: the contract is too weak to guide later adoption

Mitigation:

- freeze the concrete live pairings already in scope today
- freeze the three target types explicitly instead of describing them informally

### Risk: Slice 2.1 leaks into registry-owner or evidence-ledger work

Mitigation:

- keep Packet 2.1.1 limited to contract language only
- treat Packet 2.1.2 as explicitly deferred follow-on work

### Risk: Slice 2.1 leaks into Slice 2.2 runtime adoption

Mitigation:

- repeat that there is **no runtime adoption** in this packet
- keep the packet docs-only and behavior-neutral

### Risk: Slice 2.1 leaks into Slice 2.3 template/library work

Mitigation:

- keep template/library resolver work explicitly reserved for Slice 2.3
- reject contract wording that exists only to solve resolver behavior

### Risk: the packet invents a declarative consumer schema too early

Mitigation:

- keep consumers as code-owned validated defaults
- treat any `core/consumers/**` design as future ask-first work

## Verification Checkpoints

### Checkpoint 1: Contract freeze complete

Confirm the spec contains:

- `SupportedPipelineTarget`
- `SupportedStageTarget`
- `SupportedConsumerTarget`
- `Allowed pairings`
- `pipeline compile`
- `pipeline capture`
- `pipeline handoff emit`

Suggested verification:

```bash
rg -n "SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget|Allowed pairings|pipeline compile|pipeline capture|pipeline handoff emit" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md
```

### Checkpoint 2: Boundary freeze complete

Confirm the authority set contains:

- `no runtime adoption`
- `Slice 2.2`
- `Slice 2.3`
- `Out of scope`

Suggested verification:

```bash
rg -n "no runtime adoption|Slice 2\.2|Slice 2\.3|Out of scope" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md
```

## Exit Conditions

Packet 2.1.1 is ready for human review when:

- the supported-target contract is explicit enough that later work does not need to rename or rediscover the target vocabulary
- the allowed pairings are frozen clearly enough for later runtime adoption
- the truth model clearly separates declarative pipeline/stage truth from code-owned validated default consumers
- the docs clearly state there is **no runtime adoption** in this packet
- any Packet 2.1.2, Slice 2.2, and Slice 2.3 work is clearly deferred rather than partially started
