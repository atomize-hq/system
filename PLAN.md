# PLAN

## Status

This pass rewrites the namespace-migration plan into one implementation-ready contract for branch `main`.

The milestone stays the same. The plan is now explicit about:

- the exact declarative roots that move under `core/`
- the live compiler inputs that must be rewritten, not just the files that move
- the exact compiler, docs, fixture, and proof surfaces that must change together
- the exact validation rails and worktree-parallel execution order

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
- live stage and library references point at `core/runners/...` and `core/profiles/...`
- route basis, compile proof, handoff manifests, and refusal text emit `core/**` paths
- approved docs and contracts teach `core/**` as the full declarative compiler-input root
- top-level `pipelines/`, `profiles/`, `runners/`, and top-level `pipeline.yaml` no longer define supported compiler input truth

## Milestone Decision

Do an atomic canonical cutover. Do not ship compatibility aliases for the old roots.

That means:

- move the declarative trees into `core/`
- rewrite live declarative references that still point at top-level `profiles/` or `runners/`
- update compiler path ownership in one sweep
- update proof, fixtures, docs, and contracts in the same PR
- refuse the old-root story instead of silently supporting both worlds

## Problem Statement

The repo already behaves as if `core/` is the center of gravity, but the declarative namespace is still split.

Current contradictions:

- `crates/compiler/src/pipeline.rs` still discovers pipelines under `pipelines/`
- `crates/compiler/src/route_state.rs` still fingerprints runner and profile files under top-level `runners/` and `profiles/`
- `crates/compiler/src/pipeline_compile.rs` still special-cases `profiles/<id>/...`
- `crates/compiler/src/pipeline_handoff.rs` still treats `runners/` and `profiles/` as separate top-level canonical roots
- live stage documents under `core/stages/**` still include `runners/${runner}.md` and `profiles/${profile}/...`
- live library content and proof fixtures still cite old-root examples
- approved docs and contracts still teach top-level `pipelines/`, `profiles/`, `runners/`, and `pipeline.yaml` as supported surfaces

That leaves the operator with two mental models for one compiler surface. That is the whole bug.

## Premises

1. Declarative compiler inputs should live under one repo-owned root, `core/`, not a mix of repo root and `core/`.
2. Runtime state, generated artifacts, install surfaces, and `.system/**` are outside this change.
3. The repo is still early enough that an atomic cutover is cheaper and clearer than a long-lived compatibility shim.
4. Keeping the loader, stage includes, or proof surfaces compatible with old roots would preserve the exact ambiguity this milestone exists to remove.

## Step 0: Scope Challenge

### What already exists

| Surface | Current state | Keep / change |
| --- | --- | --- |
| `core/stages/**` | already canonical for stage docs | keep path, but rewrite live include strings that still point at old roots |
| `core/rules/**` | already canonical for shared policy includes | keep path, no semantic change |
| `core/overlays/**` | already canonical for optional policy modules | keep path, no semantic change |
| `core/library/**` | already canonical for reusable directives/templates | keep path, but rewrite example references that still point at old roots |
| `core/schemas/**` | already canonical for structured contracts | keep path, no semantic change |
| `pipelines/*.yaml` | still discovered by catalog, loader, tests, and docs | move to `core/pipelines/*.yaml` |
| `profiles/<id>/*` | still treated as top-level profile packs | move to `core/profiles/<id>/*` |
| `runners/*.md` | still treated as top-level runner guidance | move to `core/runners/*.md` |
| top-level `pipeline.yaml` | still documented in some legacy and contract surfaces; loader already treats it as out-of-scope for the current shape | retire as supported declarative input; preserve any still-needed definition only under `core/pipelines/` |
| `crates/compiler/src/pipeline.rs` | owns catalog discovery and stage boundary validation | update to the new canonical roots |
| `crates/compiler/src/route_state.rs` | owns route-basis runner/profile path snapshots | update path builders and mismatch text |
| `crates/compiler/src/pipeline_compile.rs` | classifies runner/profile/include compile documents | update path recognition and any rendered path strings |
| `crates/compiler/src/pipeline_handoff.rs` | classifies canonical trust for source paths | collapse canonical declarative trust to `core/**` plus existing artifact rules |
| compiler tests, CLI tests, proof fixtures, and demo repos | lock old path strings aggressively | rewrite as regression proof for the new root |

### Existing-code leverage map

| Sub-problem | Existing code to reuse | This plan's action |
| --- | --- | --- |
| Pipeline file discovery | `discover_repo_relative_files(...)` in `pipeline.rs` | keep the discovery mechanism, change the root from `pipelines/` to `core/pipelines/` |
| Stage boundary enforcement | `core/stages/` validation in `pipeline.rs` | keep exactly as-is |
| Route-basis fingerprinting | runner/profile SHA logic in `route_state.rs` | keep SHA behavior, change path builders and emitted text only |
| Compile include typing | document-kind routing in `pipeline_compile.rs` | keep behavior, change runner/profile path matching only |
| Handoff trust model | canonical/artifact/manual-derived split in `pipeline_handoff.rs` | keep trust classes, collapse canonical declarative roots under `core/` |
| Catalog and proof output | existing `SOURCE:` rendering in tests and CLI output | keep output shape, change emitted paths |
| Fixture-backed proof | proof corpus and CLI surface tests | keep the rails, refresh file layout and golden strings |

### Complexity ruling

This change touches far more than 8 files and more than 2 logical surfaces. That is still the minimum complete version.

Why:

- the split namespace is expressed in live declarative files, compiler code, tests, fixtures, docs, and contracts
- partial migration would create mixed proof like `core/stages/...` next to `profiles/...` and `pipelines/...`
- stage markdown files under `core/stages/**` are live compiler inputs, so leaving their include strings untouched would create a fake migration

Recommendation: do the full sweep in one milestone. Do not stage this behind a compatibility shim.

### Search and boring-tech check

**[Layer 1]** This plan uses boring tools only:

- file moves
- explicit path helper cleanup
- existing loader and fingerprint logic
- existing test rails
- docs and contract refresh

No new infrastructure. No new runtime mode. No innovation token spent.

### TODOS cross-reference

`TODOS.md` has no blocker for this migration. The deferred items are broader product follow-ons, not prerequisites for namespace cleanup.

No new TODO should be added for legacy alias support. That would be a backdoor way to keep the split contract alive.

### Completeness check

Shortcut version:

- move only `pipelines/`
- leave `profiles/` and `runners/` at repo root
- leave live stage/library references untouched
- update only some tests and only some docs

Complete version:

- move `pipelines/`, `profiles/`, and `runners/` into `core/`
- rewrite live declarative references in `core/stages/**`, `core/library/**`, and fixture repos
- retire the top-level `pipeline.yaml` support story
- update compiler code, proofs, fixtures, docs, and contracts in the same PR
- refuse old-root support instead of silently carrying it forward

Recommendation: do the complete version. The extra work is small compared with the future confusion cost.

### Distribution check

This change introduces no new artifact type.

Out of scope:

- release or installer behavior changes
- package-manager changes
- new CI publish workflows

The user-visible effect is path vocabulary and canonical repo structure, not distribution.

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

### Compiler-owned rules after landing

- `core/**` is the only canonical repo-authored declarative compiler-input root.
- pipeline discovery reads only `core/pipelines/*.yaml`.
- runner/profile allowlists resolve only from `core/runners/` and `core/profiles/`.
- route basis snapshots record `core/**` paths only.
- compile proof and handoff proof render `core/**` paths only.
- top-level `pipelines/`, `profiles/`, `runners/`, and top-level `pipeline.yaml` are not supported inputs.

### Live declarative content rules after landing

- every live include or example path in `core/stages/**` that currently says `runners/${runner}.md` becomes `core/runners/${runner}.md`
- every live include or example path in `core/stages/**` that currently says `profiles/${profile}/...` becomes `core/profiles/${profile}/...`
- any live example text under `core/library/**` that teaches runner/profile paths uses `core/runners/...` and `core/profiles/...`
- fixture repos under `tests/fixtures/**/repo/**` mirror the same canonical paths and include strings

### Docs contract after landing

- `README.md`, `docs/REPO_OVERVIEW.md`, `docs/GLOSSARY.md`, and `docs/contracts/C-01-approved-repo-surface.md` teach the same `core/**` map
- `docs/contracts/pipeline-route-and-state-core.md` names the new allowlist discovery roots
- supported-path docs may mention old roots only as historical reference, never as active truth
- refusal and help text point operators at `core/pipelines/...`, `core/profiles/...`, and `core/runners/...`

### Explicit removals

- do not leave empty supported top-level `pipelines/`, `profiles/`, or `runners/` directories behind
- do not preserve top-level `pipeline.yaml` as an apparently editable supported surface
- do not introduce a migration command, root override, or dual-root fallback

## Scope And Blast Radius

### Declarative tree moves

| Old location | New location | Notes |
| --- | --- | --- |
| `pipelines/*.yaml` | `core/pipelines/*.yaml` | update all direct path references and proof strings |
| `profiles/**` | `core/profiles/**` | includes `profile.yaml`, `commands.yaml`, `conventions.md`, and `_template/` |
| `runners/**` | `core/runners/**` | includes `examples/` |
| top-level `pipeline.yaml` | removed as supported input | preserve only if still needed as `core/pipelines/default.yaml` |

### Live declarative inputs that must be rewritten

| Surface | Why it matters |
| --- | --- |
| `core/stages/**/*.md` | these are live compiler inputs; stale include paths would break or mislead compile |
| `tests/fixtures/pipeline_proof_corpus/**/repo/core/stages/**/*.md` | fixture repos must mirror the new canonical layout |
| `tests/fixtures/foundation_flow_demo/repo/core/stages/**/*.md` | demo proof repo must mirror the new canonical layout |
| `core/library/**/*.md` where examples cite profile or runner paths | operators and proofs read these directly |
| fixture library/example content under `tests/fixtures/**` | proof corpus must stop teaching old roots |

### Compiler and runtime code blast radius

| File | Responsibility in this milestone |
| --- | --- |
| `crates/compiler/src/pipeline.rs` | pipeline discovery root, selector handling, stage-boundary proof strings |
| `crates/compiler/src/route_state.rs` | runner/profile snapshot paths, mismatch reasons, route-basis rendering |
| `crates/compiler/src/pipeline_compile.rs` | include classification for runner/profile files and rendered proof text |
| `crates/compiler/src/pipeline_handoff.rs` | canonical trust classification for declarative inputs |
| `crates/compiler/src/lib.rs` or equivalent module exports | export any new helper module if needed |

### Docs and contracts blast radius

| Surface | Required change |
| --- | --- |
| `README.md` | repo-layout and command examples stop teaching top-level declarative roots |
| `docs/REPO_OVERVIEW.md` | repo landmarks and short tech overview point at `core/pipelines/`, `core/profiles/`, `core/runners/` |
| `docs/GLOSSARY.md` | legacy glossary stays historical, but any active path examples are clearly old-world or updated |
| `docs/LEGACY_INVENTORY.md` | status notes reflect the canonical move and no longer call top-level declarative trees current |
| `docs/contracts/C-01-approved-repo-surface.md` | approved repo surface lists `core/` as the declarative root, not top-level `pipelines/`, `profiles/`, `runners/`, `pipeline.yaml` |
| `docs/contracts/pipeline-route-and-state-core.md` | allowlist discovery and route-state contract point at `core/runners/` and `core/profiles/` |

### Test and proof rails blast radius

| Surface | Required change |
| --- | --- |
| `crates/compiler/tests/pipeline_loader.rs` | discovery and direct-load path assertions |
| `crates/compiler/tests/pipeline_catalog.rs` | selector behavior and `SOURCE:` rendering |
| `crates/compiler/tests/pipeline_route_resolution.rs` | fixture definitions and any repo-relative pipeline source paths |
| `crates/compiler/tests/pipeline_state_store.rs` | runner/profile file layout, mismatch reasons, route-basis proof |
| `crates/cli/tests/cli_surface.rs` | list/show output, fixture repo setup, proof strings |
| `crates/cli/tests/pipeline_handoff_refusals.rs` | canonical trust and refusal wording |
| `crates/compiler/tests/support/pipeline_proof_corpus_support.rs` | proof corpus load roots |
| `tests/fixtures/pipeline_proof_corpus/**` | repo layout, state seeds, and goldens |
| `tests/fixtures/foundation_flow_demo/**` | repo layout, evidence transcripts, and model-output references |

## Architecture Review

### Architecture recommendation

Use one tiny shared helper module for declarative roots. Do not invent a config system.

Recommended shape:

- `crates/compiler/src/declarative_roots.rs`

Recommended contents:

- `PIPELINES_ROOT: &str = "core/pipelines"`
- `PROFILES_ROOT: &str = "core/profiles"`
- `RUNNERS_ROOT: &str = "core/runners"`
- `STAGES_ROOT: &str = "core/stages"`
- `pipeline_path(id: &str) -> String`
- `profile_pack_paths(id: &str) -> [String; 3]`
- `runner_path(id: &str) -> String`
- `is_canonical_core_source(path: &str) -> bool`

Why this is the right level:

- it kills string-literal drift
- it keeps the diff explicit and readable
- it avoids a generic registry, env override, or runtime-config abstraction

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
      |      - fingerprints core/runners/* and core/profiles/*
      |
      +--> crates/compiler/src/pipeline_compile.rs
      |      - classifies includes from core/runners/* + core/profiles/* + core/rules/* + core/library/*
      |
      +--> crates/compiler/src/pipeline_handoff.rs
             - assigns trust classes for core/** and artifacts/**
```

### Exact implementation rules

- no environment-variable overrides for declarative roots
- no compatibility aliases for top-level `pipelines/`, `profiles/`, or `runners/`
- no new migration subsystem
- no pipeline schema change
- no route-state schema expansion beyond path normalization already required by this move
- every live stage include that currently points at `runners/...` or `profiles/...` must move to `core/runners/...` or `core/profiles/...`
- every supported docs surface must land in the same PR as the code move

### Implementation phases

#### Phase 1: Move declarative trees and rewrite live declarative references

Do this first.

Scope:

- move `pipelines/*.yaml` to `core/pipelines/*.yaml`
- move `profiles/**` to `core/profiles/**`
- move `runners/**` to `core/runners/**`
- remove or retire top-level `pipeline.yaml`
- rewrite all live include strings in:
  - `core/stages/**/*.md`
  - `tests/fixtures/pipeline_proof_corpus/**/repo/core/stages/**/*.md`
  - `tests/fixtures/foundation_flow_demo/repo/core/stages/**/*.md`
- rewrite live example references in `core/library/**` and matching fixture content

Exit criteria:

- the repo tree itself reflects the new canonical layout
- no live declarative compiler input under `core/**` still points at old runner/profile roots

#### Phase 2: Normalize compiler path ownership

Scope:

- update discovery in `pipeline.rs`
- update route-basis path builders and reason text in `route_state.rs`
- update compile include classification and rendered path strings in `pipeline_compile.rs`
- update canonical trust recognition in `pipeline_handoff.rs`
- add or wire the tiny helper module if it reduces duplicated root strings

Exit criteria:

- compiler-owned path generation is centralized and explicit
- every compiler-emitted canonical declarative path uses `core/**`

#### Phase 3: Refresh approved docs and contracts

Scope:

- `README.md`
- `docs/REPO_OVERVIEW.md`
- `docs/GLOSSARY.md`
- `docs/LEGACY_INVENTORY.md`
- `docs/contracts/C-01-approved-repo-surface.md`
- `docs/contracts/pipeline-route-and-state-core.md`

Exit criteria:

- approved docs and contracts teach one namespace
- historical mentions of old roots are clearly labeled historical only

#### Phase 4: Refresh tests, proof fixtures, and goldens

Scope:

- compiler test fixtures and assertions
- CLI surface tests
- proof corpus repo layouts, seeds, and goldens
- demo repo layouts and transcripts

Exit criteria:

- tests fail on any reintroduction of old-root supported-path strings
- proof corpus renders `core/**` paths only for supported surfaces

#### Phase 5: Validation and stale-root sweep

Scope:

- run the focused test rails
- run a repo grep for stale supported-path strings
- confirm docs, contracts, proofs, and compiler behavior all agree

Exit criteria:

- focused test rails pass
- supported-path stale-root grep is clean
- any remaining old-root mentions are clearly legacy-only or example-history-only

## Code Quality Review

### Findings and decisions

1. **String-literal drift is already real.**
   - Evidence: `pipeline.rs`, `route_state.rs`, `pipeline_compile.rs`, `pipeline_handoff.rs`, tests, fixtures, and docs all spell roots independently.
   - Decision: centralize the declarative-root strings in one tiny helper or one explicit shared constant surface.

2. **Live stage includes are part of the compiler contract.**
   - Evidence: `core/stages/**` and fixture stage docs still point at `runners/${runner}.md` and `profiles/${profile}/...`.
   - Decision: rewrite them in the same milestone. Leaving them untouched would create a fake migration.

3. **A compatibility shim would be under-engineered for the product and over-engineered for maintenance.**
   - If the loader accepts both `pipelines/` and `core/pipelines/`, the compiler has to explain ambiguity forever.
   - Decision: no dual-root support layer.

4. **Docs cannot trail code here.**
   - This repo treats docs, contracts, goldens, and tests as product truth.
   - Decision: docs and contracts are part of the implementation, not a follow-up.

### Files that must stay simple

- `pipeline.rs`: explicit root constants and discovery calls
- `route_state.rs`: explicit path-builder helpers
- `pipeline_compile.rs`: explicit runner/profile path matching against the new roots
- `pipeline_handoff.rs`: explicit `core/**` canonical check

Do not add:

- a dynamic config file for declarative roots
- environment-variable overrides for repo-root discovery
- a migration subsystem
- a generic path registry abstraction with indirection the compiler does not need

## Test Review

### Test framework detection

This is a Rust workspace.

Primary proof rails:

- `cargo test`
- compiler integration tests under `crates/compiler/tests/`
- CLI integration tests under `crates/cli/tests/`
- fixture-backed proof corpora under `tests/fixtures/`

### Validation commands

Run these exact rails before calling the milestone done:

```bash
cargo test -p system-compiler --test pipeline_loader
cargo test -p system-compiler --test pipeline_catalog
cargo test -p system-compiler --test pipeline_route_resolution
cargo test -p system-compiler --test pipeline_state_store
cargo test -p system-cli --test cli_surface
cargo test -p system-cli --test pipeline_handoff_refusals
rg -n '(^|[^a-zA-Z])pipelines/|(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/|pipeline\\.yaml' README.md docs crates tests core
```

The grep is not a blanket ban on historical mentions. It is a sweep for stale supported-path language that must then be triaged as:

- supported-path bug, fix now
- legacy-only reference, keep but label clearly

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] Pipeline catalog and direct loading
    │
    ├── [GAP] discover catalog entries from core/pipelines/*.yaml only
    ├── [GAP] refuse selectors and direct loads rooted at pipelines/*.yaml
    ├── [GAP] list/show output prints SOURCE: core/pipelines/...
    └── [GAP] top-level pipeline.yaml is no longer treated as supported declarative input

[+] Live declarative include resolution
    │
    ├── [GAP] stage includes resolve runner files from core/runners/${runner}.md
    ├── [GAP] stage includes resolve profile files from core/profiles/${profile}/...
    └── [GAP] library/example references used in proof surfaces no longer teach old roots

[+] Route basis and compile proof
    │
    ├── [GAP] runner fingerprint path uses core/runners/<id>.md
    ├── [GAP] profile fingerprint paths use core/profiles/<id>/*
    └── [GAP] explain/full-context proof renders core/** paths consistently

[+] Handoff trust classification
    │
    ├── [GAP] canonical trust accepts repo-authored core/** inputs
    ├── [GAP] old-root inputs are refused instead of silently treated as canonical
    └── [GAP] handoff manifest canonical input paths render core/** only
```

### User flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Operator runs `system pipeline list`
    │
    ├── [GAP] list output shows only core/pipelines/... sources
    └── [GAP] no supported docs/help still suggest top-level pipelines/ at repo root

[+] Operator runs `system pipeline show --id ...`
    │
    ├── [GAP] show output points at core/pipelines/... and core/stages/...
    └── [GAP] shorthand and selector handling stays deterministic after the move

[+] Operator runs resolve, compile, or handoff flows
    │
    ├── [GAP] route-basis snapshot stores core/runners/... and core/profiles/...
    ├── [GAP] compile proof and handoff proof include core/** references only
    └── [GAP] refusal output names the new canonical roots

[+] Maintainer edits the repo
    │
    ├── [GAP] repo layout makes every declarative compiler input discoverable from core/
    └── [GAP] proof corpus goldens fail on any reintroduction of old-root supported surfaces

─────────────────────────────────
COVERAGE: 0/16 namespace migration paths verified for the new root yet
  Code paths: 0/10
  User flows: 0/6
QUALITY NEEDED: all gaps are regression-critical for this milestone
─────────────────────────────────
```

### Required test changes

| Area | Files | What must be asserted |
| --- | --- | --- |
| Pipeline load and catalog | `crates/compiler/tests/pipeline_loader.rs`, `crates/compiler/tests/pipeline_catalog.rs`, `crates/compiler/tests/pipeline_route_resolution.rs` | discovery and direct loads use `core/pipelines/`; old-root selectors refuse cleanly |
| Route basis | `crates/compiler/tests/pipeline_state_store.rs` | runner/profile fingerprints and mismatch reasons use `core/runners/` and `core/profiles/` |
| Compile proof | `crates/compiler/tests/pipeline_compile.rs` if present, proof-corpus support, and `crates/cli/tests/cli_surface.rs` | explain/full-context and CLI proof output show `core/**` paths only |
| Handoff trust | `crates/cli/tests/pipeline_handoff_refusals.rs` and compiler handoff tests | canonical trust admits `core/**` declarative inputs and rejects stale old-root assumptions |
| Live declarative inputs | fixture repo stage docs and any compile-driven goldens | moved stage include paths still resolve and render correctly |
| Fixtures and goldens | `tests/fixtures/pipeline_proof_corpus/**`, `tests/fixtures/foundation_flow_demo/**` | repo layout and golden text match the new root |
| Docs drift | existing help/docs drift rails plus grep sweep | old-root supported-path strings do not survive in approved docs |

### Regression rule

This milestone is almost entirely regression surface. Every changed path string is a regression requirement.

Mandatory regression proof:

- loader and catalog tests for `core/pipelines/`
- route-basis tests for `core/runners/` and `core/profiles/`
- stage-include and proof tests for `core/runners/...` and `core/profiles/...`
- CLI surface and handoff tests for `SOURCE: core/...`
- fixture and golden tests proving no supported surface still emits old-root paths

### Test plan artifact

Test plan artifact already exists at:

- `~/.gstack/projects/atomize-hq-system/spensermcconnell-main-eng-review-test-plan-20260502-155030.md`

Use it as the QA handoff for this milestone. The path-level assertions in this plan are the authoritative additions for this consolidation pass.

### LLM / eval scope

None. This change does not modify prompt templates, tool definitions, or model-eval surfaces.

## Failure Modes Registry

| Codepath | Production failure mode | Test cover required | Error handling required | User-visible result if missed | Critical gap |
| --- | --- | --- | --- | --- | --- |
| Catalog discovery | compiler still scans `pipelines/` and misses moved files | yes | load refusal must name `core/pipelines/` | operator sees “missing pipeline” for a valid repo | yes |
| Stage include resolution | stage markdown still points at `runners/...` or `profiles/...` and compile can no longer load those files | yes | compile refusal must point at the moved path | operator sees a broken compile after the repo tree already looks migrated | yes |
| Route basis | runner/profile SHA snapshots still point at old roots | yes | mismatch/refusal must be explicit | resolve succeeds once, later compile or capture becomes confusing | yes |
| Compile include typing | moved profile docs are treated as generic includes instead of profile inputs | yes | compile refusal and proof should stay typed | proof loses runner/profile provenance | no |
| Handoff trust | `core/pipelines/...` or `core/profiles/...` is treated as unsupported trust source | yes | refusal must name allowed canonical roots | downstream handoff breaks even though repo is correct | yes |
| Docs drift | README or contracts still point at old roots | yes | doc drift or grep sweep must fail | cold reader learns the wrong repo map | no |
| Top-level `pipeline.yaml` limbo | file stays in repo and looks editable while the loader still refuses it | yes | docs and contracts must say it is retired or moved | maintainers keep editing the wrong file | yes |

## Performance Review

### Findings and decisions

1. **Do not support both old and new roots in discovery.**
   - Scanning both `pipelines/` and `core/pipelines/` adds ambiguity first and filesystem work second.
   - Decision: single-root discovery only.

2. **This migration should otherwise be performance-neutral.**
   - File moves and path-string updates do not change algorithmic complexity.
   - Decision: no new caching or indexing layer.

## DX / Operator Review

This repo is a developer tool, so namespace cleanup is also a DX change.

Required DX acceptance:

- examples, contracts, and glossary use the same root as the compiler
- any refusal mentioning allowed declarative paths points to `core/...`
- a maintainer can find every declarative compiler input by starting at `core/`
- there is no “except pipelines, profiles, or runners live somewhere else” footnote
- a maintainer cannot reasonably mistake top-level `pipeline.yaml` for supported truth

TTHW impact should be neutral to positive. The repo becomes easier to navigate, not harder.

## NOT In Scope

- changing pipeline schema shape
- broadening the activation evaluator
- changing runtime `.system/state/**` storage rules beyond path normalization already implied by the move
- changing install or release behavior
- adding a migration command for external repos
- adding dynamic config for declarative roots
- rewriting every historical legacy doc that mentions old roots beyond the supported-path and proof-honesty updates required here

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Declarative tree migration and live reference rewrite | `core/pipelines/`, `core/profiles/`, `core/runners/`, `core/stages/`, `core/library/`, fixture repos under `tests/fixtures/**` | — |
| Docs and contract refresh | `README.md`, `docs/`, `docs/contracts/` | canonical target contract agreed |
| Compiler path normalization | `crates/compiler/` | declarative tree migration agreed |
| Proof and test refresh | `crates/compiler/tests/`, `crates/cli/tests/`, `tests/fixtures/**` | declarative tree migration + compiler path normalization |

### Parallel lanes

- `Lane A`: declarative tree migration and live reference rewrite
- `Lane B`: docs and contract refresh
- `Lane C`: compiler path normalization -> proof and test refresh

### Execution order

Launch `Lane A` and `Lane B` in parallel.

After `Lane A` is ready, launch `Lane C` compiler work.

After compiler path ownership is stable, finish `Lane C` with proof and test refresh.

Final step: run the focused validation rails and stale-root sweep on the integrated tree.

### Conflict flags

- `Lane A` and `Lane C` both touch fixture-backed stage content and proof assumptions. Keep those sequential.
- `Lane B` must avoid touching compiler tests or fixture goldens if it is going to stay truly parallel.
- `Lane C` test refresh must be last inside its lane because it depends on both the moved file tree and the compiler-emitted path strings.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | CEO | Make `core/` the only declarative compiler-input root | mechanical | completeness | one root is the whole point of the milestone | leave split roots |
| 2 | CEO | Do an atomic cutover, not a compatibility shim | mechanical | explicit over clever | dual-root support preserves ambiguity | alias old roots |
| 3 | Eng | Treat live stage and library references as in-scope migration work | mechanical | completeness | live declarative inputs are part of the compiler contract | move files only |
| 4 | Eng | Retire top-level `pipeline.yaml` as supported input | mechanical | boring by default | current loader already refuses it; keeping it visible as truth would contradict the compiler | silent limbo |
| 5 | Eng | Add one tiny root-helper surface, not a config system | mechanical | DRY | repeated root strings already drift across modules | copy strings everywhere |
| 6 | Eng | Update docs and contracts in the same PR | mechanical | completeness | proof and docs are part of the contract here | docs later |
| 7 | Eng | Treat every changed path string as regression-test required | mechanical | tests non-negotiable | this milestone is mostly path-contract churn | spot-check only |
| 8 | DX | Keep the command surface unchanged; only fix namespace wording | mechanical | minimal diff | the milestone is repo-root truth cleanup, not product-surface expansion | add migration commands |
| 9 | DX | Preserve legacy mentions only when clearly labeled historical | mechanical | explicit over clever | silent half-legacy wording is how split truth survives | blanket search-and-replace without triage |

## Cross-Phase Themes

1. **One canonical path contract.**
   This showed up in strategy, architecture, testing, and docs. The milestone fails if any supported surface still teaches multiple declarative roots.

2. **No silent compatibility.**
   A soft alias sounds pragmatic, but it creates worse proof, support, and maintenance burden.

3. **Proof matters as much as code.**
   This repo uses tests, goldens, and contracts as product truth. Namespace cleanup without proof cleanup is incomplete.

## Completion Summary

- Step 0: Scope Challenge — scope accepted as full atomic cutover
- Architecture Review: 4 issues found, all resolved by plan decisions
- Code Quality Review: 4 issues found, all resolved by plan decisions
- Test Review: diagram produced, 16 regression gaps identified
- Performance Review: 2 issues found, both resolved by rejecting dual-root discovery and extra indexing
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 0 items proposed
- Failure modes: 5 critical gaps flagged if migration is partial
- Outside voice: not run in this pass, this was a plan-solidification rewrite
- Parallelization: 3 lanes, 2 launchable in parallel / 1 dependent execution lane
- Lake Score: 9/9 recommendations chose the complete option

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 prior pass | CLEAR | atomic cutover accepted, no compatibility shim, no deferred scope reduction |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 prior pass + this structure refresh | CLEAR | plan now names live include rewrites, exact blast radius, 16 regression gaps, and worktree-parallel execution order |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | SKIPPED | no UI scope detected |

**NOTE:** This section records plan state, not a freshly logged multi-model review run. Re-run `/autoplan` or `/plan-eng-review` if implementation scope changes materially.

**UNRESOLVED:** 0
**VERDICT:** CEO direction held, ENG structure solidified, ready to implement this namespace migration plan.
