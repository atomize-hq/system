# PLAN

## Status

This is the implementation plan for the reviewed reduced v1 wedge.

It is derived from:

- [reviewed design doc](/Users/spensermcconnell/.gstack/projects/system/spensermcconnell-main-design-20260403-110234.md)
- CEO review decisions embedded in that design
- engineering review decisions recorded in that design's `## GSTACK REVIEW REPORT`

This plan is the current execution source of truth for repo shape, migration order, and milestone sequencing.

## Locked Decisions

- Rust is the only supported packet-resolution authority for v1.
- The current Python harness is legacy reference material only.
- Legacy Python stays frozen and clearly labeled in place until the Rust planning packet path is proven. The physical move under `archived/` happens during cutover, not before.
- The repo root becomes approved surface only.
- Nothing under `archived/` is imported, executed, or wrapped by the supported runtime path.
- Live v1 packet resolution is scoped to existing `project + feature` artifacts.
- V1 execution packets are fixture-backed demos only.
- Live slice lineage and live execution packets are deferred.
- Canonical live packet inputs live under `artifact_inputs/`.
- V1 direct packet inputs are `CHARTER`, optional `PROJECT_CONTEXT`, and `FEATURE_SPEC`.
- `FOUNDATION_STRATEGY`, `TECH_ARCH_BRIEF`, `TEST_STRATEGY_BRIEF`, `QUALITY_GATES_SPEC`, and `ENVIRONMENT_INVENTORY` are inherited posture dependencies. Lower-level artifacts may override them only with explicit rationale captured in artifact content and the decision log.
- Repo-facing copies may exist for humans, but they are derived views, not runtime inputs.
- V1 metadata/schema work is limited to those direct packet inputs plus inherited posture dependencies and one request-scoped derived manifest.
- V1 freshness is deterministic: file presence, file hash, schema version, manifest generation version, and declared dependency checks.
- V1 manifest state is request-scoped and in-memory by default. Persist detailed diagnostics only on request or on failure.
- Renderers are pure views over one typed resolver result plus typed decision log.
- `doctor` or `health` is a required v1 command surface, not a post-v1 nicety.
- Packet budgets are a first-class typed policy contract with deterministic keep, summarize, exclude, and refuse behavior.
- V1 performance stays simple until measurement proves otherwise.
- V1 distribution is a Rust CLI with explicit local install support for `macOS arm64` and `Linux x86_64`. Public package-manager and release publishing are deferred.

## Goal

Ship a reduced v1 that proves the product honestly:

- live planning packet generation over existing project + feature artifacts
- fixture-backed execution packet demo only
- explicit refusal for unsupported live slice execution requests
- explicit `doctor` guidance for stale, missing, or contradictory packet inputs
- Rust CLI as the only supported product path

## What Already Exists

- `pipeline.yaml` already declares the live artifact graph for `CHARTER`, optional `PROJECT_CONTEXT`, and `FEATURE_SPEC`.
- `tools/harness.py` already implements include resolution, artifact input loading, output routing, and stage assembly as legacy reference behavior.
- `core/stages/10_feature_spec.md` already declares a concrete feature-spec output plus optional inherited posture inputs from foundation artifacts.
- The repo already documents that pipeline artifacts are the deterministic truth source and repo-facing copies are for human-facing durability.
- The current docs already distinguish implemented stages from placeholder slice/execution scaffolding.

## NOT in scope

- Do not preserve Python as a supported runtime path.
- Do not build live `project -> feature -> slice` lineage in v1.
- Do not build review/fix packets in v1.
- Do not build MCP UI in v1.
- Do not normalize every existing artifact into the metadata system in v1.
- Do not add an on-disk derived-state cache or semantic freshness layer in v1.
- Do not do public package-manager or release publishing in v1.

## Repo Migration Contract

### Root Rule

The repository root is the approved product surface only.

Anything in the root must satisfy one of these:

- part of the supported Rust CLI/compiler path
- a canonical artifact intentionally kept at root
- repo infrastructure required to build, test, validate, or document the supported path

### Archive Rule

Legacy Python scaffold material moves under `archived/`.

That includes:

- Python harness code
- legacy harness shell wrappers
- legacy harness docs that describe the supported runtime as Python
- legacy generated prompt scaffolding that is retained only for reference

### Promotion Rule

Files or ideas may move from `archived/` back into the approved surface only when they meet all of these:

- they are needed by the reviewed reduced v1 scope
- they are rewritten or re-approved intentionally
- they do not pull Python runtime coupling back into the supported path
- their role is documented in this plan or the reviewed design

### Runtime Boundary

- The supported runtime path must not import, shell out to, or wrap anything in `archived/`.
- `archived/` is evidence and reference material, not an execution dependency.

## Target Repo Shape

```text
system/
├── artifact_inputs/
│   ├── charter/
│   ├── project_context/
│   └── feature_spec/
├── archived/
│   └── python-harness/
├── crates/
│   ├── compiler/
│   └── cli/
├── tests/
│   ├── fixtures/
│   └── golden/
├── docs/
├── Cargo.toml
├── Cargo.lock
├── PLAN.md
├── README.md
└── canonical artifacts retained at root only if explicitly approved
```

## Milestones

### M1. Freeze The Legacy Scaffold

Outcome:

- legacy Python scaffold clearly reads as frozen reference material
- root clearly communicates Rust-first direction without losing the executable reference surface too early
- no ambiguity about supported vs legacy paths

Work:

- relabel legacy docs so the root docs do not present Python as the active product path
- freeze Python harness mechanics in place as reference-only behavior
- leave only approved root docs and canonical artifacts in place
- update references so `PLAN.md` and the reviewed design are easy to find

Exit criteria:

- a new contributor can tell in under 30 seconds that Python is legacy
- nothing at the root implies Python is the supported runtime
- the legacy harness remains runnable as a reference surface until Rust planning packet parity exists

### M2. Scaffold The Rust Workspace

Outcome:

- Rust workspace exists at the root with library + CLI split

Work:

- add root `Cargo.toml`
- add `crates/compiler`
- add `crates/cli`
- add initial shared types for packet result and decision log
- define CLI command surface skeleton

Exit criteria:

- `cargo check` passes
- CLI help exists
- there is one obvious place to add compiler logic

### M3. Define Minimal Packet Inputs And Manifest

Outcome:

- minimal live packet-input contract exists

Work:

- define typed ingest for `artifact_inputs/charter/CHARTER.md`, optional `artifact_inputs/project_context/PROJECT_CONTEXT.md`, and `artifact_inputs/feature_spec/FEATURE_SPEC.md`
- define source-of-truth rules for canonical `artifact_inputs/` versus derived repo-facing copies
- define inherited posture dependency handling for `FOUNDATION_STRATEGY`, `TECH_ARCH_BRIEF`, `TEST_STRATEGY_BRIEF`, `QUALITY_GATES_SPEC`, and `ENVIRONMENT_INVENTORY`
- define explicit override-with-rationale rules for lower-level artifacts that diverge from inherited posture
- define request-scoped derived manifest shape
- define deterministic freshness fields
- define supported target matrix for local installation: `macOS arm64` and `Linux x86_64`
- document explicit triggers for expanding metadata/schema to more artifacts

Expansion triggers:

- an artifact becomes a required live packet input
- an artifact becomes a refusal source
- an artifact becomes a provenance dependency shown to the user
- an artifact becomes necessary to explain inclusion or exclusion decisions

Exit criteria:

- manifest can be built deterministically from approved live inputs
- unsupported artifacts are ignored explicitly, not implicitly
- inherited posture dependencies can mark packets stale without becoming mandatory packet body inputs

### M4. Implement Planning Packet Resolution

Outcome:

- live planning packets work over project + feature artifacts

Work:

- implement ingest
- implement manifest build
- implement deterministic freshness checks
- implement planning packet selection
- implement typed budget policy with deterministic keep, summarize, exclude, and refuse behavior
- implement typed decision log
- implement explicit refusal behavior
- implement `doctor` or `health` for blockers, stale reasons, safe next actions, and packet-readiness status

Exit criteria:

- same inputs yield same packet and same decision log
- stale or missing required inputs refuse clearly
- `doctor` reports the same blocker and freshness truth that packet generation uses
- budget behavior is deterministic and inspectable

### M5. Implement Renderers

Outcome:

- markdown, JSON, and inspect views all render from the same typed result

Work:

- add markdown renderer
- add JSON renderer
- add inspect renderer
- prove no renderer changes packet selection logic
- prove inspect output reflects the same decision log and budget policy as markdown and JSON

Exit criteria:

- inspect explains the same decision log used by markdown and JSON
- renderer failure does not destroy a successful resolver result

### M6. Add Fixture-Backed Execution Demo

Outcome:

- execution packet capability is demonstrated honestly without pretending live slice support

Work:

- define fixture lineage for execution packet demos
- implement execution demo path
- implement explicit refusal for unsupported live slice requests

Exit criteria:

- fixture execution packet demo works
- live slice execution requests refuse with clear wording

### M7. Add Test And CI Rails

Outcome:

- the Rust path is validated, not hoped into existence

Work:

- unit tests for ingest, metadata validation, manifest build, freshness checks, and refusal logic
- unit tests for inherited posture dependency freshness and override-with-rationale rules
- unit tests for budget policy: keep, summarize, exclude, and refuse
- unit tests for renderer failure isolation
- integration tests for planning packet resolution
- golden tests for markdown, JSON, and inspect outputs
- fixture-backed execution packet tests
- CLI E2E tests for install, help, non-repo-root invocation, `doctor`, and refusal flows
- drift tests for canonical `artifact_inputs/` versus derived published docs
- cutover regression tests proving Python is not advertised as supported
- CI workflow for format, lint, test, and install smoke on `macOS arm64` and `Linux x86_64`

Exit criteria:

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- install smoke passes in CI on both supported targets

### M8. Docs And Cutover

Outcome:

- repo tells one story

Work:

- update top-level README for Rust-first product path
- keep legacy docs under clearly marked legacy/archive locations
- document how to use the Rust CLI for reduced v1
- document what is deferred
- move the frozen Python harness under `archived/python-harness/` once Rust planning packet parity and cutover validation are complete

Exit criteria:

- help text, README, and docs index all agree
- no top-level doc presents Python as the supported product path

## Workstreams

### Lane A: Repo Reshape

Scope:

- archive move
- root cleanup
- doc relabeling

Depends on:

- none

### Lane B: Rust Workspace

Scope:

- workspace scaffold
- compiler and CLI crate setup

Depends on:

- M1 root decisions locked

### Lane C: Resolver Core

Scope:

- ingest
- manifest
- freshness
- planning packet selection
- renderers

Depends on:

- M2
- M3

### Lane D: Validation Rail

Scope:

- tests
- golden fixtures
- CLI E2E
- CI

Depends on:

- M2 for workspace
- M4 for real behavior

## Execution Order

1. Do M1 first. This is the repo contract.
2. Start M2 immediately after M1.
3. Run M3 and the early part of M4 after M2.
4. Run M5 after the first typed resolver result exists.
5. Run M6 after planning packet resolution is stable.
6. Run M7 in parallel with late M4 to M6 once the command surface is real.
7. Finish with M8 so the docs match what actually shipped.

## Risks

### Risk: Legacy Freeze Leaves Support Messaging Ambiguous

Mitigation:

- relabel aggressively now
- keep one obvious Rust-first story in root docs
- do the physical archive move only after the Rust path is proven

### Risk: Python Patterns Leak Back Into Runtime Design

Mitigation:

- reference-only rule for `archived/`
- no runtime imports or wrappers
- promotion requires explicit approval

### Risk: Execution Demo Gets Mistaken For Live Capability

Mitigation:

- call it fixture-backed everywhere
- add explicit refusal for live slice requests
- test help text and docs for this wording

### Risk: Metadata Scope Grows Unbounded

Mitigation:

- expansion only through the trigger list in M3
- no artifact enters the schema by vibes

## Deliverables

- `PLAN.md`
- frozen legacy scaffold clearly labeled as reference-only, then archived under `archived/python-harness/` at cutover
- Rust workspace at root
- planning packet resolver
- `doctor` or `health` command
- fixture-backed execution packet demo
- tests and CI
- updated docs

## Definition Of Done For Reduced V1

- root repo shape reflects the approved Rust-first direction
- legacy Python is clearly labeled as frozen during implementation, then lives under `archived/` at cutover
- Rust CLI is the only supported product path
- live planning packets work over approved project + feature inputs
- inherited posture dependency freshness and override rationale are enforced
- packet budgets behave deterministically and are explained by inspect output
- `doctor` reports blockers and safe next actions
- execution packet demo works from fixtures only
- unsupported live slice requests refuse clearly
- docs and help text match reality
- CI validates build, lint, test, and install smoke on both supported targets

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR | 5 proposals, 4 accepted, 1 deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 5 | CLEAR | 15 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**UNRESOLVED:** 0
**VERDICT:** CEO + ENG CLEARED — ready to implement.
