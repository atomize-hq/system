# Tasks: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

Plan reference: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md)

## Status

- Packet 1 only is landed in this task ledger.
- This ledger remains planning-only and docs-only.
- Packets 2, 3, and 4 are not started here.
- Packet 1 verification-time repo-truth freeze (pre-landing baseline, not the later landed HEAD):
  - branch: `feat/seam-extraction`
  - pre-landing baseline HEAD: `01b50868599bc55e7680784a9b5b2dace5ab6042`
  - working tree posture at verification time: dirty only in unrelated local files `AGENTS.md` and `CLAUDE.md`
  - `aa882af42792a250cc02a6740bd1e2123178caff..01b50868599bc55e7680784a9b5b2dace5ab6042` is the verification-time docs-only baseline range
  - later Packet 1 docs-only commits may advance HEAD without changing the prerequisite authority truth frozen here

## Implementation Authority Used

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-spec.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-plan.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-tasks.md`
- `docs/specs/handbook-engine-extraction-phase-6-slice-1-packet-6-1-2-ownership-matrix.md` as background only, not as the final verdict

## Packet 1: Freeze Current Authority And Scope Guard

- [x] Task: Record the current repo-truth starting gate for this planning family
  - Acceptance: The triplet states the verification-time branch/pre-landing-baseline posture, the docs-only baseline delta from `aa882af...` to `01b5086`, and that Phase 6 Slice 1 is already READY before this family begins.
  - Verify: `git status --short --branch && git rev-parse HEAD && git log --oneline --decorate -20 && git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..HEAD && git diff --name-only aa882af42792a250cc02a6740bd1e2123178caff..HEAD && rg -n "READY|Packet 6\.1\.4|ownership-and-integration-planning|default_canonical_layout_contract" docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md`
  - Completion note: Recorded branch `feat/seam-extraction`, pre-landing verification baseline `01b50868599bc55e7680784a9b5b2dace5ab6042`, unrelated local dirt in `AGENTS.md` and `CLAUDE.md`, and a docs-only `aa882af... .. 01b5086` baseline delta across nine `docs/specs/` files. Later Packet 1 docs-only commits do not change that prerequisite authority truth. The prerequisite reassessment triplet remains **READY** and Packet 6.1.4 names this family as the next planning boundary.

- [x] Task: Re-state the root ownership rule and hard planning boundaries inside the triplet
  - Acceptance: The triplet makes explicit that handbook stays owner when the center of gravity is handbook-domain, Substrate only becomes owner if the crate becomes truly substrate-specific, and implementation stays out of scope.
  - Verify: `rg -n "handbook should own it and Substrate should import it|substrate-specific|planning-only|out of scope|GitNexus impact analysis" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`
  - Completion note: Packet 1 now freezes the Phase 6 root rule, keeps this landing docs-only/planning-only, and explicitly defers Packets 2 through 4.

- [x] Task: Check the triplet against the prerequisite authority set and list contradictions explicitly
  - Acceptance: Any contradiction is listed explicitly; if none exist, the triplet says so plainly.
  - Verify: `rg -n "Contradictions Vs\. Prerequisite Authority Set|No contradiction found|outside Packet 1 authority" docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-{spec,plan,tasks}.md`
  - Completion note: No contradiction was found in the prerequisite authority set. Existing prompt/draft artifacts remain outside Packet 1 authority and are untouched by this landing.

## Packet 2: Decide Handbook-Owned Imported-Core Boundaries

- [ ] Not started in this landing.
- [ ] Out of scope until Packet 1 is reviewed and the human explicitly continues the family.

## Packet 3: Decide Handbook-Side Deferred Boundaries And Non-Targets

- [ ] Not started in this landing.
- [ ] Out of scope until a later explicit packet request.

## Packet 4: Define Downstream Execution Seams And Review Gate

- [ ] Not started in this landing.
- [ ] Out of scope until a later explicit packet request.

## Human Review Gate

Do not begin Packet 2, Packet 3, Packet 4, packet-prompt authoring, implementation, production code edits, crate moves, runtime behavior changes, CLI redesign, or retained-compiler retirement work from this ledger. Stop at Packet 1 and route the result to orchestration review.
