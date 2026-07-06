# Archive Index — Handbook Engine Extraction Specs

## Purpose

This directory holds all landed/historical planning artifacts from the handbook engine extraction work. These files are retained as provenance but are no longer active planning authorities.

For current work, start from the active authority set listed below.

---

## Active Authority Set (not archived)

| File | Role |
|------|------|
| `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` (repo root) | Root plan and phase-order authority |
| `docs/specs/handbook-engine-extraction-slice-map.md` | Phase → Slice → Packet decomposition map |
| `docs/specs/handbook-engine-extraction-closeout-four-set-map.md` | Closeout seam map for Phases 1–5 |

---

## Archive Contents

### `phase-1-layout-contract-and-inventory/` (20 files)

Phases 1 Slices 1–5: layout contract, canonical/setup layout, stateful storage layout, authoring layout, layout parameterization closeout.

**Status:** All landed. Phases 1 complete.

Triplets: `phase-1-slice-1` through `phase-1-slice-5` (spec/plan/tasks/packet-prompts each).

### `phase-2-supported-target-contract/` (16 files)

Phase 2 Slices 1–4: supported target contract, runtime target adoption, template/library resolver, orchestration target parameterization closeout.

**Status:** All landed. Phase 2 complete.

Triplets: `phase-2-slice-1` through `phase-2-slice-4` (spec/plan/tasks/packet-prompts each).

### `phase-3-core-splits/` (16 files)

Phase 3 Slices 1–4: charter core split, project context core split, environment inventory core split, shell wording split.

**Status:** All landed. Phase 3 complete.

Triplets: `phase-3-slice-1` through `phase-3-slice-4` (spec/plan/tasks/packet-prompts each).

### `phase-4-crate-migration/` (21 files)

Phase 4 Slices 1–5: crate scaffold, engine migration, pipeline migration, flow migration, caller rewire and compiler narrowing. Includes 1 packet note (`packet-4-5-1-residual-caller-inventory`).

**Status:** All landed. Phase 4 complete.

Triplets: `phase-4-slice-1` through `phase-4-slice-5` (spec/plan/tasks/packet-prompts each), plus `phase-4-slice-5-packet-4-5-1-residual-caller-inventory.md`.

### `phase-5-cli-shell/` (11 files)

Phase 5 Slices 1–3: CLI skeleton and author setup, CLI runtime command shell, CLI shell closeout. (Slice 2 has no packet-prompts artifact.)

**Status:** All landed. Phase 5 complete.

Triplets: `phase-5-slice-1` through `phase-5-slice-3` (spec/plan/tasks, plus packet-prompts for slices 1 and 3).

### `phase-6-slice-1-migration-readiness/` (6 files)

Phase 6 Slice 1: migration readiness reassessment. Includes triplet, packet-prompts, and two packet notes (6.1.1 live truth freeze, 6.1.2 ownership matrix).

**Status:** Complete. Produced READY verdict and named the ownership/integration planning family as the next step.

Key conclusions lifted into active context:
- All five extracted crates reassessed; no substrate-domain center of gravity found in any crate surface.
- `handbook-engine` boundary blocker cleared; export is now `default_canonical_layout_contract`.
- `handbook-pipeline` has remaining compiler-backed test coupling (since resolved — see phase-6-pipeline-boundary-cleanup).
- `handbook-flow` is the least-settled seam; keep handbook-owned longer.
- `handbook-cli` is product shell, not an import target.
- Retained `handbook-compiler` is transition glue, not an import target.

### `phase-6-ownership-and-integration-planning/` (4 files)

Phase 6 Ownership and Integration Planning: 4 planning packets (authority freeze, engine/pipeline ownership, flow/cli/compiler deferred boundaries, downstream seam map + human review gate).

**Status:** Complete. Planning-only; no execution authorized from this family.

Key conclusions lifted into active context:
- Target Substrate import set: `handbook-engine` + `handbook-pipeline` + `handbook-flow`.
- `handbook-cli` is out. Retained `handbook-compiler` is not an import target.
- `handbook-engine` is import-ready.
- `handbook-pipeline` needs boundary freeze (since completed — see phase-6-pipeline-boundary-cleanup).
- `handbook-flow` needs import-boundary proof (deferred to Lane B).
- Optional `handbook-engine` boundary freeze (Lane C) stays closed unless later review says current surface is insufficient.

### `phase-6-pipeline-boundary-cleanup/` (4 files)

Phase 6 Pipeline Boundary Cleanup: triplet + packet-prompts. Planning packets 1–3 plus Implementation Packet 1.

**Status:** Complete and review-clean (closed 2026-06-17, commit `794ed6e`).

Key conclusions lifted into active context:
- Compiler-backed dev-dependency removed (commit `2dfb9b7`).
- Durable boundary decision: documented frozen subset of the current public surface.
- In-boundary: `pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, `pipeline_contract_version()`.
- Out-of-boundary: `setup` (CLI/compiler), `declarative_roots` (internal).
- No narrower facade needed at this time; deferred to Lane D if consumer evidence warrants.

### `candidate-work/` (8 files)

Earlier candidate sessions: candidate-2 (trusted pipeline session) and candidate-3 (workspace access). Pre-Phase-6 work.

**Status:** Historical. Conclusions superseded by the Phase 1–6 extraction work.

---

## Archive Date

2026-06-17
