# Tasks: Handbook Engine Extraction Phase 2 Slice 1 - Supported Target Contract

Plan reference: [handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md](./handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md)

## Packet 2.1.1: Typed Target And Consumer Contract

Packet 2.1.1 freezes the supported target vocabulary for Slice 2.1: define `SupportedPipelineTarget`, `SupportedStageTarget`, and `SupportedConsumerTarget`; freeze the currently allowed compile/capture/handoff/provenance pairings; keep pipeline/stage truth declarative catalog truth; keep consumers as code-owned validated defaults; and allow **no runtime adoption** in Slice 2.1.

- [ ] Task: Freeze the supported target contract in the slice authority docs
  - Acceptance: The spec defines `SupportedPipelineTarget`, `SupportedStageTarget`, and `SupportedConsumerTarget`, and it records the allowed pairings for compile, capture, stage-10 provenance, and handoff.
  - Verify: `rg -n "SupportedPipelineTarget|SupportedStageTarget|SupportedConsumerTarget|Allowed pairings|pipeline compile|pipeline capture|pipeline handoff emit" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`

- [ ] Task: Freeze the Slice 2.1 boundary across the authority set
  - Acceptance: The spec, plan, and tasks docs all state there is **no runtime adoption** in Packet 2.1.1, that Slice 2.2 owns runtime adoption, that Slice 2.3 owns template/library resolver work, and that registry-owner/evidence-ledger work remains deferred to Packet 2.1.2.
  - Verify: `rg -n "no runtime adoption|Slice 2\.2|Slice 2\.3|Out of scope" docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-spec.md`, `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-plan.md`, `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-tasks.md`

## Deferred Follow-on (Not Part Of Packet 2.1.1)

- Packet 2.1.2 may later define the registry-owner and evidence-ledger details, but that work is not part of this packet.
- Slice 2.2 runtime adoption is deferred.
- Slice 2.3 template/library resolver work is deferred.

## Human Review Gate

Do not start Packet 2.1.2 or Slice 2.2 implementation work until the human has reviewed and approved this Packet 2.1.1 authority set.
