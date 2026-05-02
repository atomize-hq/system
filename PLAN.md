# PLAN

## Status

Fresh plan for consolidating declarative compiler inputs under one canonical repo root on branch `main`.

This replaces the previous packaging-focused `PLAN.md`. The current milestone is a namespace and contract cleanup:

- make `core/` the only declarative compiler-input root
- move `pipelines/`, `profiles/`, and `runners/` under `core/`
- keep `core/stages/`, `core/rules/`, `core/overlays/`, `core/library/`, and `core/schemas/` as-is
- retire the split-brain contract where some declarative inputs live at repo root and others already live under `core/`

## Objective

Land one coherent declarative namespace:

```text
core/
  pipelines/
  profiles/
  runners/
  stages/
  rules/
  overlays/
  library/
  schemas/
```

Success means all of the following are true at once:

- compiler discovery loads pipelines only from `core/pipelines/`
- profile packs resolve only from `core/profiles/<id>/`
- runner guidance resolves only from `core/runners/<id>.md`
- route basis, compile proof, handoff manifests, and refusal text all emit `core/**` paths
- repo docs and contracts describe `core/**` as the full declarative root
- top-level `pipelines/`, `profiles/`, `runners/`, and the current top-level `pipeline.yaml` no longer define supported compiler input truth

## Problem Statement

The repo already acts like `core/` is the center of gravity, but the namespace is still split:

- catalog discovery in [crates/compiler/src/pipeline.rs](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/pipeline.rs) still scans `pipelines/`
- route-basis snapshots in [crates/compiler/src/route_state.rs](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/route_state.rs) still fingerprint `runners/` and `profiles/`
- compile include classification in [crates/compiler/src/pipeline_compile.rs](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/pipeline_compile.rs) still special-cases `profiles/<id>/...`
- handoff trust classification in [crates/compiler/src/pipeline_handoff.rs](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/pipeline_handoff.rs) still treats `runners/` and `profiles/` as separate top-level canonical roots
- repo docs such as [docs/REPO_OVERVIEW.md](/Users/spensermcconnell/__Active_Code/system/docs/REPO_OVERVIEW.md) and [docs/GLOSSARY.md](/Users/spensermcconnell/__Active_Code/system/docs/GLOSSARY.md) still teach the old split layout
- [docs/contracts/C-01-approved-repo-surface.md](/Users/spensermcconnell/__Active_Code/system/docs/contracts/C-01-approved-repo-surface.md) still names `profiles/`, `runners/`, `pipelines/`, and `pipeline.yaml` as approved top-level surfaces

That is messy for humans and worse for the compiler contract. One product surface should not need two mental models for where declarative truth lives.

## Premises

1. Declarative compiler inputs should live under one repo-owned root, `core/`, not a mix of repo root and `core/`.
2. Runtime state, generated artifacts, install surfaces, and `.system/**` remain outside this change. This is a repo-input namespace migration, not a runtime redesign.
3. The repo is still early enough that an atomic cutover is better than a long-lived compatibility shim.
4. A compatibility layer that keeps accepting `pipelines/`, `profiles/`, or `runners/` would preserve the exact ambiguity this milestone is trying to remove.

## Step 0: Scope Challenge

### What already exists

| Surface | Current state | Keep / change |
| --- | --- | --- |
| `core/stages/**` | already canonical for stage docs | keep path, no semantic change |
| `core/rules/**` | already canonical for shared policy includes | keep path, no semantic change |
| `core/overlays/**` | already canonical for optional policy modules | keep path, no semantic change |
| `core/library/**` | already canonical for reusable directives/templates | keep path, no semantic change |
| `core/schemas/**` | already canonical for structured contracts | keep path, no semantic change |
| `pipelines/*.yaml` | still discovered by catalog/loader/tests/docs | move to `core/pipelines/*.yaml` |
| `profiles/<id>/*` | still treated as top-level profile packs | move to `core/profiles/<id>/*` |
| `runners/*.md` | still treated as top-level runner guidance | move to `core/runners/*.md` |
| top-level `pipeline.yaml` | repo-root legacy/superset pipeline shape; tests already refuse it as supported loader input | retire as supported declarative input; preserve content only by moving its useful definition under `core/pipelines/` |
| `crates/compiler/src/pipeline.rs` | owns catalog discovery and stage boundary validation | update to the new canonical roots |
| `crates/compiler/src/route_state.rs` | owns route-basis runner/profile path snapshots | update path builders |
| `crates/compiler/src/pipeline_compile.rs` | classifies runner/profile/include compile documents | update profile path recognition |
| `crates/compiler/src/pipeline_handoff.rs` | classifies canonical trust for source paths | collapse canonical trust to `core/**` plus existing artifact rules |
| compiler + CLI tests + proof fixtures | lock old path strings aggressively | rewrite as regression proof for the new root |

### Existing-code leverage map

| Sub-problem | Existing code to reuse | This plan's action |
| --- | --- | --- |
| Pipeline file discovery | `discover_repo_relative_files(...)` in `pipeline.rs` | keep discovery mechanism, change root from `pipelines` to `core/pipelines` |
| Stage boundary enforcement | `core/stages/` validation in `pipeline.rs` | keep exactly as-is |
| Route-basis fingerprinting | runner/profile SHA logic in `route_state.rs` | keep SHA behavior, change path builders only |
| Compile include typing | document-kind routing in `pipeline_compile.rs` | keep behavior, change runner/profile path matching |
| Handoff trust model | canonical/artifact/manual-derived split in `pipeline_handoff.rs` | keep trust classes, collapse canonical declarative roots under `core/` |
| Catalog / show proof | existing `SOURCE:` rendering in tests and catalog output | keep output shape, change emitted paths |
| Fixture-backed proof | proof corpus and CLI surface tests | keep rails, refresh golden strings and fixture layout |

### Dream State

```text
CURRENT
  split declarative truth:
  pipelines/ + profiles/ + runners/ + core/**
        |
        v
THIS PLAN
  one canonical declarative root:
  core/pipelines + core/profiles + core/runners + core/**
        |
        v
12-MONTH IDEAL
  every repo-authored compiler input under core/**
  runtime/generated/install state outside core/**
  docs/contracts/proof all teaching one map
```

### Implementation alternatives

| Option | Shape | Pros | Cons | Recommendation |
| --- | --- | --- | --- | --- |
| A. Atomic canonical cutover | move files and code in one PR; old roots stop being supported | honest contract, no dual truth, no extra IO, simplest long-term model | bigger one-time diff | **Recommended** |
| B. Canonical `core/**` plus legacy aliases | move files but keep loader accepting old roots | lower short-term breakage | preserves ambiguity, doubles test surface, invites permanent drift | reject |
| C. Virtual namespace only | keep files where they are, just rewrite docs around them | smallest immediate diff | fake improvement, contract stays incoherent | reject |

### Scope ruling

This touches far more than 8 files. That is still the minimum complete change.

Why:

- the split namespace is one bug expressed in code, declarative files, fixtures, docs, and contracts
- partial migration would create mixed proofs like `core/stages/...` next to `profiles/...` and `pipelines/...`
- tests already prove path strings directly, so any honest cutover has to update the proof corpus too

Recommendation: do the full sweep in one milestone. Do not stage this behind a compatibility shim.

### Search and boring-tech check

**[Layer 1]** This plan uses boring tools only:

- file moves
- constant/path helper cleanup
- existing loader and fingerprint logic
- existing test rails
- docs/contract refresh

No new infrastructure. No new runtime mode. No innovation tokens spent.

### TODOS cross-reference

[TODOS.md](/Users/spensermcconnell/__Active_Code/system/TODOS.md) has no blocker for this work. The deferred items are larger product follow-ons, not namespace prerequisites.

No new TODO should be added for “legacy alias support.” That would be a backdoor way to keep the split contract alive.

### Completeness check

Shortcut version:

- move `pipelines/` only
- leave `profiles/` and `runners/` at repo root
- keep docs/contracts half-updated
- let trust/route/proof surfaces drift

Complete version:

- move `pipelines/`, `profiles/`, and `runners/` into `core/`
- retire the top-level `pipeline.yaml` support story
- update compiler code, proof output, fixtures, docs, and contracts in the same PR
- refuse old-root support instead of silently carrying it forward

Recommendation: do the complete version. The extra work is small compared with the future confusion cost.

### Distribution check

This change introduces no new artifact type.

Out of scope:

- release/distribution changes
- installer behavior changes
- package-manager or GitHub Release changes

The only user-visible effect is path vocabulary in docs, proof output, and refusals.

## Canonical Target Contract

### Repo shape after landing

```text
core/
  pipelines/
    default.yaml
    foundation.yaml
    foundation_inputs.yaml
    release.yaml
    sprint.yaml
  profiles/
    _template/
    dotnet/
    go-mod/
    node-pnpm/
    python-poetry/
    python-uv/
    rust-cargo/
  runners/
    claude-tools.md
    codex-cli.md
    cursor.md
    plain-chat.md
    examples/
      runner.example.md
  stages/
  rules/
  overlays/
  library/
  schemas/
```

Rules:

- `core/**` is the only canonical repo-authored declarative compiler-input root.
- top-level `pipelines/`, `profiles/`, and `runners/` no longer exist as supported roots.
- top-level `pipeline.yaml` is removed as supported input. If its definition still matters, it lives as `core/pipelines/default.yaml`.
- runtime state stays under `.system/**`.
- generated outputs stay under `artifacts/**`, `dist/**`, or other generated/runtime roots, never under `core/`.

### Code contract after landing

- [crates/compiler/src/pipeline.rs](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/pipeline.rs) discovers pipelines only from `core/pipelines/`
- [crates/compiler/src/route_state.rs](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/route_state.rs) snapshots runner/profile files only from `core/runners/` and `core/profiles/`
- [crates/compiler/src/pipeline_compile.rs](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/pipeline_compile.rs) classifies profile documents only from `core/profiles/<id>/...`
- [crates/compiler/src/pipeline_handoff.rs](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/pipeline_handoff.rs) treats repo-authored canonical declarative inputs as `core/**`
- CLI/catalog/proof output prints `SOURCE: core/...` everywhere

### Docs contract after landing

- [README.md](/Users/spensermcconnell/__Active_Code/system/README.md), [docs/REPO_OVERVIEW.md](/Users/spensermcconnell/__Active_Code/system/docs/REPO_OVERVIEW.md), [docs/GLOSSARY.md](/Users/spensermcconnell/__Active_Code/system/docs/GLOSSARY.md), and [docs/contracts/C-01-approved-repo-surface.md](/Users/spensermcconnell/__Active_Code/system/docs/contracts/C-01-approved-repo-surface.md) all teach the same `core/**` map
- legacy docs may mention old roots only as historical reference, never as supported runtime truth
- refusal/help text should point operators at `core/pipelines/...`, `core/profiles/...`, and `core/runners/...`

## Architecture Review

### Architecture recommendation

Use one small shared compiler helper for declarative roots, not a new service layer.

Add a narrow module such as:

- `crates/compiler/src/declarative_roots.rs`

It should expose only explicit helpers/constants:

- `PIPELINES_ROOT = "core/pipelines"`
- `PROFILES_ROOT = "core/profiles"`
- `RUNNERS_ROOT = "core/runners"`
- `STAGES_ROOT = "core/stages"`
- `profile_pack_paths(id) -> [String; 3]`
- `runner_path(id) -> String`
- `is_canonical_core_source(path) -> bool`

Why this is the right level:

- it kills the current string-literal drift
- it keeps the diff explicit and readable
- it avoids inventing a generic path registry or config system

### Dependency graph

```text
system CLI / tests
      |
      v
crates/compiler/src/pipeline.rs
  - discovers core/pipelines/*.yaml
  - validates core/stages/*.md
      |
      +--> crates/compiler/src/route_state.rs
      |      - fingerprints core/runners/* + core/profiles/*
      |
      +--> crates/compiler/src/pipeline_compile.rs
      |      - classifies includes from core/runners/* + core/profiles/* + core/rules/* + core/library/*
      |
      +--> crates/compiler/src/pipeline_handoff.rs
             - assigns trust classes for core/** and artifacts/**
```

### Implementation steps

1. **Move declarative files into `core/`.**
   - `pipelines/*.yaml` -> `core/pipelines/*.yaml`
   - `profiles/**` -> `core/profiles/**`
   - `runners/**` -> `core/runners/**`
   - move or retire top-level `pipeline.yaml` by landing the kept definition under `core/pipelines/default.yaml`

2. **Normalize compiler path ownership.**
   - update discovery in `pipeline.rs`
   - update route-basis path builders in `route_state.rs`
   - update compile include classification in `pipeline_compile.rs`
   - update canonical trust recognition in `pipeline_handoff.rs`

3. **Normalize proof surfaces.**
   - catalog/list/show output
   - route-basis snapshots
   - explain/full-context proof output
   - refusal strings that mention allowed roots

4. **Refresh repo contracts and docs.**
   - approved repo surface contract
   - route/state contract
   - repo overview, glossary, README
   - legacy inventory wording

5. **Refresh tests and fixtures as hard regression proof.**
   - compiler tests
   - CLI tests
   - proof corpus fixtures/goldens
   - demo repo fixtures

## Code Quality Review

### Findings and decisions

1. **String-literal drift is already real.**
   - Evidence: `pipeline.rs`, `route_state.rs`, `pipeline_compile.rs`, `pipeline_handoff.rs`, tests, and docs each spell roots separately.
   - Decision: add one tiny helper/constants module. Do not keep repeating raw `"profiles/"`, `"runners/"`, or `"pipelines/"` strings.

2. **A compatibility shim would be under-engineered for the user-facing contract and over-engineered for maintenance.**
   - If the loader accepts both `pipelines/` and `core/pipelines/`, the compiler has to explain ambiguity forever.
   - Decision: no dual-root support layer. Refuse old roots after the cutover.

3. **Docs cannot trail code here.**
   - This repo locks vocabulary in tests and contracts. Shipping compiler path changes without doc/contract updates would create a false proof story.
   - Decision: docs/contracts are part of the same milestone, not a follow-up.

### Files that should stay simple

- `pipeline.rs`: explicit root constants and discovery calls
- `route_state.rs`: explicit path-builder helpers
- `pipeline_compile.rs`: explicit profile-pack matching against the new canonical helpers
- `pipeline_handoff.rs`: explicit `core/**` canonical check

Do not add:

- a dynamic config file for declarative roots
- environment-variable overrides for root discovery
- a migration subsystem

## Test Review

### Test framework detection

This is a Rust workspace. The primary proof rails are:

- `cargo test`
- compiler tests under `crates/compiler/tests/`
- CLI tests under `crates/cli/tests/`
- fixture-backed proof corpora under `tests/fixtures/`

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] Pipeline catalog / loader
    │
    ├── [GAP] load catalog from core/pipelines/*.yaml only
    ├── [GAP] refuse selectors and direct loads rooted at pipelines/*.yaml
    ├── [GAP] list/show output prints SOURCE: core/pipelines/...
    └── [GAP] top-level pipeline.yaml is no longer treated as approved declarative input

[+] Stage route basis
    │
    ├── [GAP] runner fingerprint path uses core/runners/<id>.md
    ├── [GAP] profile fingerprint paths use core/profiles/<id>/*
    └── [GAP] route-basis mismatch/refusal text prints core/** paths consistently

[+] Compile include classification
    │
    ├── [GAP] runner includes still classify correctly from core/runners/*
    ├── [GAP] profile includes still classify correctly from core/profiles/*
    └── [GAP] explain/full-context proof renders core/** include paths

[+] Handoff trust classification
    │
    ├── [GAP] canonical trust accepts repo-authored core/** inputs
    └── [GAP] old-root inputs are refused instead of silently treated as canonical
```

### User flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Operator runs `system pipeline list`
    │
    ├── [GAP] list output shows only core/pipelines/... sources
    └── [GAP] no stale docs/help still suggest pipelines/... at repo root

[+] Operator runs `system pipeline show --id ...`
    │
    ├── [GAP] show output points at core/pipelines/... and core/stages/...
    └── [GAP] shorthand / selector handling stays deterministic after the move

[+] Operator runs resolve/compile flows
    │
    ├── [GAP] route-basis snapshot stores core/runners/... and core/profiles/...
    ├── [GAP] compile proof includes core/** references only
    └── [GAP] refusal output tells the operator the new canonical root

[+] Maintainer updates docs or fixtures
    │
    ├── [GAP] repo docs teach the same root as compiler behavior
    └── [GAP] proof corpus goldens fail on any reintroduction of old roots

─────────────────────────────────
COVERAGE: 0/14 namespace paths verified for the new root yet
  Code paths: 0/9
  User flows: 0/5
QUALITY NEEDED: all gaps are regression-critical for this milestone
─────────────────────────────────
```

### Required test changes

| Area | Files | What must be asserted |
| --- | --- | --- |
| Pipeline load/catalog | `crates/compiler/tests/pipeline_loader.rs`, `pipeline_catalog.rs`, `pipeline_route_resolution.rs` | discovery and direct loads use `core/pipelines/`; old-root selectors refuse cleanly |
| Route basis | `crates/compiler/tests/pipeline_state_store.rs` | runner/profile fingerprints and mismatch reasons use `core/runners/` and `core/profiles/` |
| Compile proof | `crates/compiler/tests/pipeline_compile.rs`, `crates/cli/tests/cli_surface.rs` | explain/full-context and CLI proof output show `core/**` paths only |
| Handoff trust | `crates/cli/tests/pipeline_handoff_refusals.rs`, compiler handoff tests | canonical trust admits `core/**` declarative inputs and rejects stale old-root assumptions |
| Fixtures / goldens | `tests/fixtures/pipeline_proof_corpus/**`, `tests/fixtures/foundation_flow_demo/**` | golden text and repo fixture layout match the new root |
| Docs drift | existing help/docs drift rails | old root strings do not reappear in approved docs |

### Regression rule

This change modifies existing behavior and proof output. Every changed path string is a regression surface.

Mandatory regression tests:

- loader/catalog tests for `core/pipelines/`
- route-basis tests for `core/runners/` and `core/profiles/`
- CLI surface/proof tests for `SOURCE: core/...`
- fixture/golden tests proving no old-root strings survive in supported surfaces

### Test plan artifact

Test plan artifact written to:

- [spensermcconnell-main-eng-review-test-plan-20260502-155030.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-eng-review-test-plan-20260502-155030.md)

### LLM / eval scope

None. This change does not modify prompt templates, tool definitions, or model-facing behavior.

## Failure Modes Registry

| Codepath | Production failure mode | Test cover required | Error handling required | User-visible result if missed | Critical gap |
| --- | --- | --- | --- | --- | --- |
| Catalog discovery | compiler still scans `pipelines/` and misses moved files | yes | load refusal should name `core/pipelines/` | operator sees “missing pipeline” for a valid repo | yes |
| Route basis | runner/profile SHA snapshots still point at old roots | yes | mismatch/refusal must be explicit | resolve succeeds once, later compile/refusal becomes confusing | yes |
| Compile include typing | moved profile docs are treated as generic includes instead of profile inputs | yes | compile refusal/proof should stay typed | prompts/proof lose clear runner/profile provenance | no |
| Handoff trust | `core/pipelines/...` or `core/profiles/...` is treated as unsupported trust source | yes | refusal must name allowed canonical roots | downstream handoff breaks even though repo is correct | yes |
| Docs drift | README/contracts still point at old roots | yes | doc drift tests should fail | cold reader learns the wrong repo map | no |
| Top-level `pipeline.yaml` limbo | file stays in repo and looks supported while loader still refuses it | yes | contract/docs must say it is retired or moved | maintainers keep editing the wrong file | yes |

## Performance Review

### Findings and decisions

1. **Do not support both old and new roots in discovery.**
   - Scanning both `pipelines/` and `core/pipelines/` would add avoidable filesystem traversal and, worse, ambiguity.
   - Decision: single-root discovery only.

2. **This migration should be performance-neutral otherwise.**
   - Path-string changes and directory moves do not alter algorithmic complexity.
   - No caching or new indexing layer is justified.

## DX / Operator Review

This repo is a developer tool, so namespace cleanup is also a DX change.

Required DX acceptance:

- examples, contracts, and glossary use the same root as the compiler
- any refusal mentioning allowed paths points to `core/...`
- a maintainer can find every declarative compiler input by starting at `core/`
- there is no “except pipelines/profiles/runners live somewhere else” footnote

TTHW impact should be neutral to positive. The repo becomes easier to navigate, not harder.

## NOT In Scope

- changing pipeline schema shape
- broadening the activation evaluator
- changing runtime `.system/state/**` storage
- changing install or release behavior
- adding a migration command for external repos
- adding dynamic config for declarative roots
- archiving every historical legacy doc that mentions old roots, beyond updating supported-path authority docs and any tests/fixtures needed to keep proof honest

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Move declarative files into `core/` | `core/pipelines/`, `core/profiles/`, `core/runners/`, fixture repos | — |
| Normalize compiler path constants and loaders | `crates/compiler/` | move plan agreed |
| Refresh docs/contracts | `README.md`, `docs/`, contracts | move plan agreed |
| Refresh proof/golden/test assertions | `crates/compiler/tests/`, `crates/cli/tests/`, `tests/fixtures/` | compiler path changes + moved files |

### Parallel lanes

- `Lane A`: move declarative files into `core/` → refresh fixture repo layouts
- `Lane B`: refresh docs/contracts (independent once final canonical names are agreed)
- `Lane C`: compiler path normalization → proof/golden/test assertion refresh

### Execution order

Launch `Lane A` and `Lane B` in parallel.

After `Lane A` lands or is ready to merge, run `Lane C`.

Why: compiler/tests need the moved files to exist, but docs/contracts can update as soon as the target namespace is fixed.

### Conflict flags

- `Lane A` and `Lane C` both affect fixture-backed tests. Coordinate carefully.
- `Lane B` should avoid touching proof fixtures or compiler tests to stay truly parallel.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | CEO | Make `core/` the only declarative root | mechanical | completeness | one root is the whole point of the milestone | leave split roots |
| 2 | CEO | Do an atomic cutover, not a compatibility shim | mechanical | explicit over clever | dual-root support preserves ambiguity | alias old roots |
| 3 | Eng | Retire top-level `pipeline.yaml` as supported input | mechanical | boring by default | current loader/tests already refuse it; keeping it supported would contradict the compiler | silent limbo |
| 4 | Eng | Add one tiny path-helper module | mechanical | DRY | repeated root strings already drift across modules | copy strings everywhere |
| 5 | Eng | Update docs/contracts in the same PR | mechanical | completeness | proof and docs are part of the contract here | docs later |
| 6 | Eng | Treat every changed path string as regression-test required | mechanical | tests non-negotiable | this milestone is mostly path-contract churn | spot-check only |
| 7 | DX | Keep command surface unchanged; only fix namespace wording | mechanical | minimal diff | the user asked for stronger namespacing, not new verbs | add migration commands |

## Cross-Phase Themes

1. **One canonical path contract.** This showed up in strategy, architecture, testing, and docs. The milestone fails if any supported surface still teaches multiple declarative roots.
2. **No silent compatibility.** A soft alias sounds pragmatic but creates worse long-term proof and support burden.
3. **Proof matters as much as code.** This repo uses tests, goldens, and contracts as product truth. Path cleanup without proof cleanup is incomplete.

## Completion Summary

- Step 0: Scope Challenge — scope accepted as full atomic cutover
- Architecture Review: 3 issues found, all resolved by plan decisions
- Code Quality Review: 3 issues found, all resolved by plan decisions
- Test Review: diagram produced, 14 regression gaps identified
- Performance Review: 1 issue found, resolved by rejecting dual-root discovery
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 0 items proposed
- Failure modes: 4 critical gaps flagged if migration is partial
- Outside voice: skipped
- Parallelization: 3 lanes, 2 parallel / 1 dependent follow-up
- Lake Score: 7/7 recommendations chose the complete option

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR | 1 strategy decision, 0 deferred scope expansions, 0 critical gaps |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR | 7 decisions locked, 14 regression gaps identified, 4 partial-migration critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | no UI scope detected |

**UNRESOLVED:** 0
**VERDICT:** CEO + ENG CLEARED — ready to implement this namespace migration plan.
