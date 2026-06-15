# Spec: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

## Status

- Packet 1 only: **Freeze Current Authority And Scope Guard**.
- This family starts only after the Phase 6 Slice 1 migration-readiness reassessment reached **READY** and Packet 6.1.4 named this triplet as the next planning family.
- This landing is intentionally **docs-only** and **planning-only**.
- Packets 2, 3, and 4 are **not started** here and remain out of scope.
- Packet 1 verification-time repo-truth freeze (pre-landing baseline, not the later landed HEAD):
  - branch: `feat/seam-extraction`
  - pre-landing baseline HEAD: `01b50868599bc55e7680784a9b5b2dace5ab6042`
  - working tree posture at verification time: dirty only in unrelated local files `AGENTS.md` and `CLAUDE.md`
- `aa882af42792a250cc02a6740bd1e2123178caff..01b50868599bc55e7680784a9b5b2dace5ab6042` is the Packet 1 verification-time **docs-only** baseline range. Later Packet 1 docs-only commits may advance HEAD without changing the prerequisite authority truth frozen here. The changed paths in that baseline range are all under `docs/specs/`:
  - `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-packet-prompts.md`
  - `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-1-live-truth-freeze.md`
  - `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md`

## Packet 1 Objective

Freeze the authority chain, root decision rule, verification-time branch/pre-landing-baseline truth, and hard scope guard for the new Phase 6 ownership family so later planning packets cannot silently drift back to stale pre-READY assumptions.

This packet does **not** make per-crate ownership decisions yet. It only establishes the current authoritative starting point for that later work.

## Authority Chain Frozen By Packet 1

1. `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` is the root authority.
2. `docs/specs/handbook-engine-extraction-slice-map.md` confirms Phases 1 through 5 are fully landed through Slice 5.3 and that Phase 6 is the next authoritative step.
3. `docs/specs/handbook-engine-extraction-closeout-four-set-map.md` records the honest Phase 1 through 5 closeout posture that Phase 6 inherits.
4. `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-{spec,plan,tasks}.md` are the prerequisite READY gate for this family.
5. `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md` remains useful background input, but it is **not** the final verdict because it predates the later boundary-fix and READY reassessment closeout.

## Root Ownership Decision Rule

Per `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` Phase 6:

- if a crate is still fundamentally handbook-domain, **handbook should own it and Substrate should import it**
- only move a crate into Substrate if its real center of gravity becomes **substrate-specific**

Packet 1 freezes that rule as the required baseline for this family. Later packets may apply the rule crate by crate, but they must not rewrite it implicitly.

## Hard Scope Guard For This Landing

In scope for Packet 1 only:

- record the verification-time branch, pre-landing baseline HEAD, and working-tree posture
- record whether the verification-time `aa882af... .. 01b5086` baseline range is docs-only
- restate the READY prerequisite gate from the Phase 6 reassessment
- restate the root ownership decision rule
- state the family's hard planning-only boundaries
- list any contradiction between this triplet and the prerequisite authority set

Out of scope for Packet 1:

- Packets 2, 3, and 4
- per-crate ownership/import verdicts
- downstream execution seam definitions beyond brief future framing
- packet-prompt authoring or approval
- production code edits, crate moves, runtime rewrites, CLI redesign, or compiler-retirement work

If any production symbol edit somehow becomes unavoidable, run GitNexus impact analysis first, report the blast radius, and stop unless the human explicitly approves widening.

## Current Repo-Truth Starting Gate

The prerequisite authority set is aligned on the same starting statement:

- Phase 6 is the next authoritative step after the Phases 1 through 5 closeout.
- Phase 6 Slice 1 finished with a **READY** verdict for a separate ownership/integration planning family.
- Packet 6.1.4 named this exact triplet as that family.
- Therefore this family starts **after** the READY reassessment and remains **planning-only** until the human explicitly approves later work.

## Contradictions Vs. Prerequisite Authority Set

- **No contradiction found** in the prerequisite authority set used for Packet 1.
- Explicit non-contradiction note: the existing docs-only `handbook-engine-extraction-phase-6-ownership-and-integration-planning-packet-prompts.md` path is present in the `aa882af... .. HEAD` delta, but Packet 1 does not treat that artifact as landed authority and does not touch it here.

## Deferred Later Packets (Not Started Here)

- **Packet 2:** decide handbook-owned imported-core boundaries
- **Packet 3:** decide handbook-side deferred boundaries and non-targets
- **Packet 4:** define downstream execution seams and the review gate

Those packets are framed here only so the family boundary is honest; they remain out of scope for this landing.

## Required Verification For Packet 1

```bash
git status --short --branch
git rev-parse HEAD
git log --oneline --decorate -20
git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..HEAD
git diff --name-only aa882af42792a250cc02a6740bd1e2123178caff..HEAD
rg -n "Phase 6|Final ownership decision rule|Exit criteria|Open Questions" HANDBOOK_ENGINE_EXTRACTION_PLAN.md
rg -n "Phase 6|fully landed through Slice 5.3|next authoritative step" docs/specs/handbook-engine-extraction-slice-map.md
rg -n "Phase 6|next authoritative step|Set 3|Set 4" docs/specs/handbook-engine-extraction-closeout-four-set-map.md
rg -n "READY|Packet 6\.1\.4|ownership-and-integration-planning|default_canonical_layout_contract" docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md
```

## Success Criteria

- the triplet explicitly records the verification-time branch/pre-landing-baseline truth and working-tree posture
- the triplet explicitly records that the verification-time `aa882af... .. 01b5086` baseline range is docs-only
- the READY Phase 6 Slice 1 reassessment is frozen as the prerequisite gate
- the root ownership decision rule is restated explicitly
- Packets 2, 3, and 4 remain clearly out of scope
- any contradiction with prerequisite authority is listed explicitly rather than papered over
- the result is ready for orchestration review, but not execution-approved
