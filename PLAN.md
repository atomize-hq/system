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
- Legacy Python moves under `archived/` before new Rust implementation work begins in earnest.
- The repo root becomes approved surface only.
- Nothing under `archived/` is imported, executed, or wrapped by the supported runtime path.
- Live v1 packet resolution is scoped to existing `project + feature` artifacts.
- V1 execution packets are fixture-backed demos only.
- Live slice lineage and live execution packets are deferred.
- V1 metadata/schema work is limited to `CHARTER`, optional `PROJECT_CONTEXT`, `FEATURE_SPEC`, and one derived manifest.
- V1 freshness is deterministic: file presence, file hash, schema version, manifest generation version, and declared dependency checks.
- Renderers are pure views over one typed resolver result plus typed decision log.
- V1 performance stays simple until measurement proves otherwise.
- V1 distribution is a Rust CLI with CI validation for build, lint, test, and install smoke.

## Goal

Ship a reduced v1 that proves the product honestly:

- live planning packet generation over existing project + feature artifacts
- fixture-backed execution packet demo only
- explicit refusal for unsupported live slice execution requests
- Rust CLI as the only supported product path

## Non-Goals

- Do not preserve Python as a supported runtime path.
- Do not build live `project -> feature -> slice` lineage in v1.
- Do not build review/fix packets in v1.
- Do not build MCP UI in v1.
- Do not normalize every existing artifact into the metadata system in v1.
- Do not introduce incremental caching or semantic freshness in v1.

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

### M1. Archive The Legacy Scaffold

Outcome:

- legacy Python scaffold moved under `archived/python-harness/`
- root clearly communicates Rust-first direction
- no ambiguity about supported vs legacy paths

Work:

- move Python harness code and wrappers under `archived/python-harness/`
- move or relabel legacy docs so the root docs do not present Python as the active product path
- leave only approved root docs and canonical artifacts in place
- update references so `PLAN.md` and the reviewed design are easy to find

Exit criteria:

- a new contributor can tell in under 30 seconds that Python is legacy
- nothing at the root implies Python is the supported runtime

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

- define typed ingest for `CHARTER`, optional `PROJECT_CONTEXT`, and `FEATURE_SPEC`
- define source-of-truth rules for each
- define derived manifest shape
- define deterministic freshness fields
- document explicit triggers for expanding metadata/schema to more artifacts

Expansion triggers:

- an artifact becomes a required live packet input
- an artifact becomes a refusal source
- an artifact becomes a provenance dependency shown to the user
- an artifact becomes necessary to explain inclusion or exclusion decisions

Exit criteria:

- manifest can be built deterministically from approved live inputs
- unsupported artifacts are ignored explicitly, not implicitly

### M4. Implement Planning Packet Resolution

Outcome:

- live planning packets work over project + feature artifacts

Work:

- implement ingest
- implement manifest build
- implement deterministic freshness checks
- implement planning packet selection
- implement typed decision log
- implement explicit refusal behavior

Exit criteria:

- same inputs yield same packet and same decision log
- stale or missing required inputs refuse clearly

### M5. Implement Renderers

Outcome:

- markdown, JSON, and inspect views all render from the same typed result

Work:

- add markdown renderer
- add JSON renderer
- add inspect renderer
- prove no renderer changes packet selection logic

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
- integration tests for planning packet resolution
- golden tests for markdown, JSON, and inspect outputs
- fixture-backed execution packet tests
- CLI E2E tests for install, help, non-repo-root invocation, and refusal flows
- cutover regression tests proving Python is not advertised as supported
- CI workflow for format, lint, test, and install smoke

Exit criteria:

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- install smoke passes in CI on at least one target

### M8. Docs And Cutover

Outcome:

- repo tells one story

Work:

- update top-level README for Rust-first product path
- keep legacy docs under clearly marked legacy/archive locations
- document how to use the Rust CLI for reduced v1
- document what is deferred

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

### Risk: Archive Move Leaves Too Much At Root

Mitigation:

- be aggressive
- root should contain approved surface only
- if unsure, move to `archived/`

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
- archived legacy scaffold under `archived/python-harness/`
- Rust workspace at root
- planning packet resolver
- fixture-backed execution packet demo
- tests and CI
- updated docs

## Definition Of Done For Reduced V1

- root repo shape reflects the approved Rust-first direction
- legacy Python lives under `archived/`
- Rust CLI is the only supported product path
- live planning packets work over approved project + feature inputs
- execution packet demo works from fixtures only
- unsupported live slice requests refuse clearly
- docs and help text match reality
- CI validates build, lint, test, and install smoke
