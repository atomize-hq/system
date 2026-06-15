# Plan: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

Spec reference: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md)

## Status

- This plan currently lands **Packet 1 only**.
- Packet 1 starts from the already-READY Phase 6 Slice 1 reassessment and freezes the authority chain plus hard scope guard for the family.
- Packets 2, 3, and 4 are intentionally **not started** here.
- This is still a docs-only, planning-only family; no implementation work is authorized.

## Objective

Establish the verification-time authoritative starting point for the ownership/integration planning family before any crate-by-crate decision work begins.

For this landing, success means later packets cannot honestly claim stale pre-READY authority, hidden scope expansion, or ambiguous ownership rules.

## Packet Order

### Packet 1: Freeze current authority and scope guard

Status: **landed in this change**

Packet 1 must make all of the following explicit:

- verification-time branch / pre-landing baseline HEAD / working-tree posture
- whether the verification-time `aa882af... .. 01b5086` baseline range is docs-only
- that Phase 6 Slice 1 is already **READY** and is the prerequisite gate
- the root ownership decision rule from `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- the planning-only hard boundaries for this family
- any contradiction between this triplet and the prerequisite authority set

### Packet 2: Decide handbook-owned imported-core boundaries

Status: **pending, out of scope here**

Future packet only. Do not start until Packet 1 is reviewed and the family is explicitly continued.

### Packet 3: Decide handbook-side deferred boundaries and non-targets

Status: **pending, out of scope here**

Future packet only. Do not start from this landing.

### Packet 4: Define downstream execution seams and review gate

Status: **pending, out of scope here**

Future packet only. Do not start from this landing.

## Packet 1 Execution Approach

1. verify live branch / pre-landing baseline HEAD / dirty-tree posture
2. verify the `aa882af... .. 01b5086` baseline delta and whether it is docs-only
3. verify the prerequisite authority chain across the root plan, slice map, closeout map, and READY reassessment triplet
4. restate the root ownership rule verbatim enough that later packets cannot miss it
5. freeze the hard planning-only boundaries and explicitly defer later packets
6. list contradictions if any; otherwise state that none were found

## Packet 1 Verification Outputs Used

- `git status --short --branch` -> branch `feat/seam-extraction`; unrelated local dirt in `AGENTS.md` and `CLAUDE.md`
- `git rev-parse HEAD` -> `01b50868599bc55e7680784a9b5b2dace5ab6042` as the Packet 1 pre-landing verification baseline
- `git log --oneline --decorate -20` -> the verification-time HEAD was the docs commit `01b5086 Add Phase 6 ownership planning docs and packet prompts`
- `git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..HEAD` -> the verification-time `aa882af... .. 01b5086` baseline covered nine changed files, all under `docs/specs/`
- `git diff --name-only aa882af42792a250cc02a6740bd1e2123178caff..HEAD` -> confirms that verification-time baseline delta is docs-only
- authority `rg` checks -> Phase 6 is the next authoritative step; the reassessment triplet records **READY** and Packet 6.1.4 naming of this family

## Risks And Mitigations

### Risk: later packets inherit stale pre-READY wording

Mitigation:

- Packet 1 freezes the READY prerequisite explicitly
- Packet 1 treats Packet 6.1.2 as background input only, not as the final verdict

### Risk: this family silently widens past planning-only work

Mitigation:

- Packet 1 says Packets 2 through 4 are out of scope here
- Packet 1 forbids production code edits from this landing
- any unavoidable production-symbol change would require GitNexus impact analysis plus explicit human approval to widen

### Risk: prompt artifacts or existing drafts get mistaken for landed authority

Mitigation:

- Packet 1 limits landed scope to this triplet
- existing prompt/draft artifacts are explicitly treated as outside Packet 1 authority

## Exit Condition For This Landing

This landing is complete when:

- Packet 1 is explicit in the spec/plan/tasks triplet
- the current authority chain and scope guard are frozen
- the docs-only delta verdict is recorded
- no contradiction with the prerequisite authority set is hidden
- Packets 2, 3, and 4 remain pending and out of scope
- the result is ready for orchestration review and still not execution-approved
