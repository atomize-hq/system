---
dispatch_id: 20260714T004730Z--HCM-0-1--independent-control-pack-review-corrected
source_handoff_id: 20260713T164052Z--HCM-0-1--documentation--artifact-intake-posture-layering
orchestration_decision: proof_or_review_followup
session_kind: review
phase_id: HCM-0
slice_id: HCM-0.1
packet_id: null
snapshot_ref: null
delta_ref: null
snapshot_projection_ref: null
status: ready
---

# Dispatch: Fresh Independent HCM-0.1 Control-Pack Review

## Mission

Independently review the complete HCM-0.1 Handbook Contract Membrane control pack and decide whether its docs-only proof gate is satisfied. Review from live files rather than trusting the source handoff's conclusions. Do not implement Rust, author an implementation packet, select shipped artifact defaults, or widen into HCM-0.2+ work.

This is a review-only session. Apart from the mandatory immutable review handoff record and ledger append, do not edit the control pack. If any actionable finding exists, do not close HCM-0.1; record the finding precisely and route it to a separate docs-only remediation session so a later fresh reviewer can remain independent.

The earlier dispatch `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260714T004544Z--HCM-0-1--independent-control-pack-review.md` is superseded and must not be used because shell interpolation removed required literal file and gate references.

## Active Context Resolution

- Level: coordination
- Scope horizon: program_control_pack
- Detail resolution: architecture_and_protocol
- Temporal horizon: phase_0
- Authority horizon: docs_and_context_infrastructure
- Memory horizon: program_record
- Validation horizon: independent_control_pack_freeze_review

## Snapshot Grounding

- Prior/session-end snapshot ref: null
- Current/session-start snapshot ref: null
- Deterministic delta ref: null
- Resolution-aware grounding projection ref: null
- Included state families: manual live verification of current branch/HEAD/cleanliness; selected handoff and ledger consistency; HCM-0.1 pack sections; bounded Rust/assets evidence named below; open proof obligations
- Explicitly omitted state families: secrets; unrestricted environment/command output; full unrelated diffs; unrelated program queues; implementation details outside the named live files
- Drift signals and durable justification refs: the source record's branch/head/dirty fields are stale but its semantic and routing conclusions remain valid; its previously dirty docs were committed in `a3babd20329027afacdcee9d8b7b9d638d15af5b`. Source record: `docs/specs/handbook-contract-membrane/handoffs/records/20260713T164052Z--HCM-0-1--documentation--artifact-intake-posture-layering.json`
- Capture consistency: not_available

Snapshot Memory has not landed. Do not invent snapshot evidence or treat this manual projection as a captured snapshot.

## Authority Order

1. No slice-local packet exists for HCM-0.1; this dispatch and the selected immutable source handoff bound the review.
2. Exact control-pack sections named below.
3. Live code/tests/assets for current implementation truth.
4. The active architecture memo only for concept lineage.
5. Archived docs only if an explicit pack claim requires historical verification.

## Must Read

- `AGENTS.md`
- `docs/START_HERE.md`
- `docs/specs/handbook-contract-membrane/00-README.md`: Authority stack; Control-pack map; Per-slice context assembly protocol; Session closeout rule; Initial program conclusion
- `docs/specs/handbook-contract-membrane/01-target-architecture.md`: Authority split; Canonical and projected artifacts; Charter intake and project-posture kernel; Snapshot Memory posture; Non-negotiable invariants; Explicit non-goals
- `docs/specs/handbook-contract-membrane/02-semantic-model.md`: Artifact-kind and instance registries; Artifact intake semantics; Project posture kernel; Snapshot Memory
- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`: Current-to-target crosswalk; High-risk coupling zones; Crosswalk update rule
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`: Sequencing rule; Phases 0 through 3; Finding-driven decomposition protocol
- `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`: Artifact kind definition contract; Artifact instance descriptor contract; Artifact intake definition contract; Intake record and artifact candidate contracts; Charter intake and canonical contract; Artifact validation layers; Snapshot capture policy through Snapshot redaction and retention; Project posture kernel and recommendation contracts
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`: PR-009; Open program proof gates; Regression rules; Slice closeout evidence record; Control-pack proof gate
- `docs/specs/handbook-contract-membrane/07-orchestration-onboarding-prompt.md`: Ready-to-use prompt; Orchestration boundary
- `docs/specs/handbook-contract-membrane/08-handoff-ledger-and-escalation-protocol.md`: Snapshot, handoff, and dispatch roles; Orchestration protocol; Dispatch requirements; Failure rules
- `docs/specs/handbook-contract-membrane/handoffs/dispatch-template.md`
- `docs/specs/handbook-contract-membrane/handoffs/handoff-record.schema.json`
- `docs/specs/handbook-contract-membrane/handoffs/handoff-template.json`
- `docs/specs/handbook-contract-membrane/handoffs/ledger-entry.schema.json`
- `docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl`
- `docs/specs/handbook-contract-membrane/handoffs/records/20260713T164052Z--HCM-0-1--documentation--artifact-intake-posture-layering.json`
- `docs/ideas/handbook-contract-membrane-architecture-memo.md`: Executive summary; Resolution and durable orchestration
- Live repo-truth files: `crates/engine/src/canonical_artifacts.rs`, `crates/engine/src/canonical_paths.rs`, `crates/engine/src/author/charter_core.rs`, `core/library/charter/CHARTER_INPUTS.yaml.tmpl`, `core/library/charter/charter_inputs_directive.md`, `core/library/authoring/charter_authoring_method.md`, `core/stages/04_charter_inputs.md`, `core/stages/05_charter_interview.md`, and `core/stages/05_charter_synthesize.md`

## Current Repo-Truth Statement

At dispatch creation the repository is on `feat/handbook-contract-membrane` at `a3babd20329027afacdcee9d8b7b9d638d15af5b` and was clean before this orchestration's dispatch/handoff/ledger artifacts. The live implementation still has a fixed four-variant `CanonicalArtifactKind`, a fixed-field `CanonicalLayoutContract`, Markdown canonical paths, structured Charter-input/rendering precursors, broad posture content, and inconsistent retained interview/synthesis stage guidance. No Rust definitions exist for the target artifact-kind/instance/intake/candidate/posture/snapshot types. The control-pack semantics are target authority only; the implementation and program proof gates remain open.

## Artifact / Intake / Posture Boundary

- Artifact-kind definition refs: none; review the target contract, no runtime semantic record exists
- Artifact-instance/profile refs: none; review the target contract, no repository instance record exists
- Intake definition/candidate/canonical refs, if applicable: none; current templates/assets are precursors, not target intake/candidate/canonical records
- Posture kernel/recommendation/transition refs, if applicable: none; current Charter posture content is a precursor, not a resolved kernel or enacted transition
- Shipped-default decision status: unresolved; reserved exclusively for HCM-0.6 research plus user brainstorming/decision

Do not infer shipped defaults from examples/current code, treat intake candidates as canonical, enact posture recommendations without approval, or introduce dynamic CLI commands for custom kinds/vocabulary.

## Allowed Scope

- Read the exact control-pack, handoff, idea-memo, Rust, and asset files named above.
- Run read-only validation and consistency checks.
- Write exactly one immutable review handoff under `docs/specs/handbook-contract-membrane/handoffs/records/`.
- Append exactly one matching ledger entry after duplicate-ID and schema checks.

## Explicit Non-Goals

- Any Rust, test, core asset, installed-skill, or control-pack modification.
- Selecting or implying the shipped default artifact set.
- Starting HCM-0.2, HCM-0.3, HCM-0.6, HCM-0.7, or an implementation slice.
- Restoring a nested CLI interview or adding hidden model synthesis.
- Dynamic/generated CLI commands for profiles, vocabulary, or artifact kinds.
- User migration tooling or implicit legacy compatibility.
- Unrelated cleanup.

## Tasks and Deliverables

1. Revalidate the source handoff and its relevant live-repo claims independently.
2. Review findings-first across: kind/instance/schema ownership; custom-kind safety; one-schema guided-adaptive/express/agent-assisted intake; provenance/candidate/canonical separation; Charter authority and projections; posture thresholds, hysteresis, notification, recommendation, and transition authority; Snapshot integration; default-set reservation; phase sequencing; schema/semantic refs; orchestration; and proof gates.
3. Check cross-document terminology, ownership, defaulting, validation, authority, non-goals, and phase dependencies for contradictions, ambiguity, untestable obligations, or accidental implementation authorization.
4. Validate Markdown links, JSON/schema/template/ledger consistency, portability, docs-only scope, and `git diff --check`. Do not modify the reviewed pack to make checks pass.
5. If any actionable issue exists, record severity, exact path/line evidence, classification, and bounded remediation scope; leave HCM-0.1 open.
6. If no actionable issue exists, explicitly record a clean independent review, satisfaction of the HCM-0.1 control-pack proof gate, and permission for HCM-0.1 to close. This earns no runtime seam promotion and authorizes no Rust by itself.
7. Write the mandatory durable review handoff and append its ledger entry.

## Contracts and Proof Gates

- Review the exact artifact kind, instance, intake, candidate/promotion, Charter, validation-layer, snapshot, and posture contracts named under Must Read.
- Evaluate the complete `Control-pack proof gate` in `06-proof-and-regression-ledger.md`.
- The single proof change this review may earn is: HCM-0.1 control-pack proof gate pending independent review -> satisfied, allowing HCM-0.1 to close.
- All implementation gates (`PG-DEFAULT-01`, `PG-KIND-*`, `PG-ARTIFACT-01`, `PG-INTAKE-*`, `PG-CHARTER-01`, `PG-SNAP-*`, and `PG-POSTURE-*`) remain open.

## Stop and Escalate When

- Target docs and live truth conflict in a behavior-changing way.
- Review exposes an unresolved authority boundary among artifact kinds/instances, intake/candidates/canonical artifacts, posture, snapshots, or contracts.
- A finding requires cross-document repair or child-packet decomposition.
- Shipped defaults would need to be selected; route that to HCM-0.6.
- Required proof cannot be produced inside this review-only boundary.

## Mandatory Closeout

Before responding in chat:

1. write one immutable handoff record under `docs/specs/handbook-contract-membrane/handoffs/records/`;
2. set `snapshot_refs.capture_status` to `not_available` with null refs because Snapshot Memory has not landed;
3. set the review handoff's `supersedes` to `20260714T004730Z--HCM-0-1--orchestration--correct-independent-review-dispatch`;
4. update `handoffs/ledger.jsonl` according to `08-handoff-ledger-and-escalation-protocol.md`;
5. validate the handoff and matching ledger entry against their Draft 2020-12 schemas;
6. keep chat to status, handoff path, short summary, next action, and one `jq` read command.
