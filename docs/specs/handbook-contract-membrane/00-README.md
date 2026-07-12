# Handbook Contract Membrane Control Pack

**Status:** draft active control pack; docs and architecture only  
**Scope:** target architecture, semantic contracts, sequencing, context assembly, handoff, escalation, and proof gates  
**Implementation authorization:** none; implementation begins only from an approved slice-local packet  
**Repo-truth snapshot:** 2026-07-12; re-check live code before every slice

## Purpose

This directory is the durable context-engineering control surface for the Handbook contract-membrane program.

It exists because the work spans canonical artifact representation, configurable artifact sets, user vocabulary, Context Resolution, projections, a public SDK facade, CLI/Tauri transports, Substrate consumption, contract lifecycle, and external validator docks. Loading all source discussions, archived plans, and implementation files into every session would create context drift rather than context quality.

The pack separates:

1. **target truth** — what the architecture must become;
2. **repo truth** — what the live implementation does now;
3. **slice truth** — what one bounded packet is allowed to change;
4. **proof truth** — what evidence is required before a seam may be called landed.

An artifact, type, command, test, or published crate is not by itself proof that the target seam has landed.

## Authority stack

Use this order and keep the kinds of authority distinct:

1. **Approved slice-local packet** under `slices/<slice-id>/`
   - exact implementation authority for that slice;
   - `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` define its boundary.
2. **This control pack**
   - target architecture, semantic contracts, program sequencing, escalation rules, and proof gates.
3. **Live code and tests**
   - current implementation truth; they do not override the pack's approved target architecture merely because old behavior exists.
4. **Active idea memos** under `docs/ideas/`
   - concept lineage and design input.
5. **Archived docs** under `archived/`
   - provenance only unless a pack section explicitly revives one bounded rule.
6. **Conversation history, prior summaries, and handoff prose**
   - discovery hints until revalidated against the pack and live tree.

If target docs and live code conflict, record both truths explicitly. Do not silently select whichever makes the current task easiest.

## Greenfield rule

Handbook is greenfield for this architecture:

- do not build user-facing legacy migration tooling;
- do not promise compatibility with previous Markdown-first artifact layouts;
- do not keep permanent dual-readable or dual-writable truth;
- do not preserve a hard-coded behavior merely because it exists today;
- define the desired shipped default profile directly.

A temporary internal bridge is allowed only when it has a concrete architectural purpose in a bounded cutover, is not presented as a public compatibility promise, and has an explicit deletion gate in `04-phase-slice-map.md` and `06-proof-and-regression-ledger.md`.

## Control-pack map

| File | Load when | Canonical content |
|---|---|---|
| [`01-target-architecture.md`](01-target-architecture.md) | deciding ownership, crate boundaries, or transport posture | target layers, owner map, CLI/SDK/Tauri/Substrate/dock boundaries, non-negotiable invariants |
| [`02-semantic-model.md`](02-semantic-model.md) | changing artifacts, profiles, vocabulary, Resolution, memory, or projections | stable semantic roles, configurable terminology, resolution envelope, projection and promotion semantics |
| [`03-seam-crosswalk.md`](03-seam-crosswalk.md) | scoping a slice or checking whether a named capability really exists | current artifacts, current classification, target owner, required action, sibling dependencies |
| [`04-phase-slice-map.md`](04-phase-slice-map.md) | planning, decomposing, executing, or escalating work | phase order, slice boundaries, child-packet rules, non-goals, exit gates |
| [`05-contracts-schemas-and-gates.md`](05-contracts-schemas-and-gates.md) | defining public types, YAML/JSON schemas, CLI output, SDK surfaces, or docks | preliminary contract shapes, schema policy, lifecycle, evidence/verdict/gate rules |
| [`06-proof-and-regression-ledger.md`](06-proof-and-regression-ledger.md) | reviewing, validating, closing, or preserving known behavior | current proof tiers, open proof gaps, permanent guard rails, cutover-deletion gates |
| [`07-orchestration-onboarding-prompt.md`](07-orchestration-onboarding-prompt.md) | starting or resuming a program-level orchestration session | ready-to-use orchestration prompt, handoff selection, dispatch output contract |
| [`08-handoff-ledger-and-escalation-protocol.md`](08-handoff-ledger-and-escalation-protocol.md) | stopping any session, reporting a blocker, or expanding scope/resolution | durable record rules, status model, escalation choreography, `jq` queries, short chat closeout |

## Semantic landing labels

Use these labels for seams, not for isolated files or functions:

| Label | Meaning |
|---|---|
| `TargetOnly` | Approved target semantics exist, but no meaningful implementation seam exists. |
| `UsefulPrecursor` | Reusable implementation exists but does not yet own or enforce the target semantics. |
| `BoundaryLanded` | The correct owner and typed boundary exist, but real-path adoption or runtime proof is incomplete. |
| `RealPathAdopted` | A real product path uses the correct boundary, but the complete proof wall is not closed. |
| `ContractCorrectAndProven` | Correct owner, semantics, real path, enforcement point, and required proof all exist. |
| `Superseded` | A prior model remains only as bounded provenance or temporary cutover scaffolding. |

Do not promote a seam because similarly named code exists. A seam is complete only at `ContractCorrectAndProven`.

## Per-slice context assembly protocol

Context assembly is part of every implementation, review, documentation-repair, and proof slice.

Assemble three bounded packets:

1. **Authority packet — what must be true**
   - exact `04` slice row;
   - affected `03` seam rows;
   - applicable `01` invariants and `02` semantics;
   - exact `05` contract sections;
   - current slice-local `SPEC.md` and task ledgers.
2. **Repo-truth packet — what is true now**
   - current production path;
   - allowed source/test files;
   - related public types;
   - one relevant precedent;
   - fresh impact/call-path evidence when symbols will change.
3. **Proof packet — how completion is judged**
   - exact `06` gate rows;
   - targeted tests;
   - negative/fail-closed cases;
   - required CLI, downstream, dock, or runtime evidence;
   - the single classification change the evidence may support.

Target fewer than 2,000 focused lines per implementation or review task. Load sections, not entire archives.

Use this capsule at slice start:

```text
SLICE / OBJECTIVE:
ACTIVE RESOLUTION ENVELOPE:
TARGET AUTHORITY BOUNDARY:
CURRENT REPO-TRUTH STATUS:
MUST-READ PACK SECTIONS:
LIVE SOURCE / TESTS / PRECEDENT:
SIBLING SEAMS IN CONTEXT:
ALLOWED AREAS / EXPLICIT NON-GOALS:
APPLICABLE CONTRACTS / PROOF GATES:
KNOWN CORRECTIONS OR CONFLICTS:
HANDOFF RECORD TO RESUME, IF ANY:
EXIT PROOF / STOP CONDITIONS:
```

## Session closeout rule

Every design, planning, implementation, documentation, review, proof, or orchestration session must finish by writing one durable handoff record under `handoffs/records/`, even when the work completed cleanly.

The record must distinguish:

- completed work;
- partial work safe to resume;
- a local blocker;
- an escalation requiring broader resolution or authority;
- documentation/control-pack repair;
- further decomposition discovered during execution;
- required review or proof follow-up.

The chat response should then be short: status, handoff ID/path, and one `jq` command. Do not paste the full handoff report into chat when the durable record exists.

## Orchestration loop

```text
slice session
  -> durable handoff record
  -> orchestration session selects latest/specific handoff
  -> orchestration revalidates pack + live truth
  -> orchestration updates docs/decomposition when required
  -> orchestration writes a durable dispatch prompt
  -> next bounded session runs from that dispatch
```

Escalation is a normal resolution transition, not a failure. Silent scope widening is a failure.

## Initial program conclusion

The live repository contains published, reusable owner crates and several valuable precursors, but the configurable profile, canonical YAML authority, Context Resolution model, projection engine, SDK facade, full JSON protocol, contract membrane, and dock protocol are not yet one landed system.

The next authorized work after this pack is reviewed is documentation/design work inside Phase 0. No Rust implementation is authorized by the existence of this pack alone.
