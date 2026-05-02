# Orchestration Plan: Atomic Declarative Namespace Cutover to `core/**`

## Summary
- This session exists to execute [`PLAN.md`](/Users/spensermcconnell/__Active_Code/system/PLAN.md) end to end on `main`, not to reinterpret it.
- All worktrees branch from the current checked-out baseline `main`, because `main` is the live branch in this workspace for this run.
- The milestone is one atomic canonical cutover:
  - move `pipelines/*.yaml` to `core/pipelines/*.yaml`
  - move `profiles/**` to `core/profiles/**`
  - move `runners/**` to `core/runners/**`
  - retire top-level `pipeline.yaml` as supported input
  - rewrite live include and reference strings in `core/stages/**`, `core/library/**`, and fixture repos under `tests/fixtures/**`
  - normalize compiler-owned path generation to `core/**`
  - refresh approved docs, contracts, tests, proof fixtures, and goldens in the same integrated pass
- Parent is the only integrator, the only scope interpreter, and the only cross-lane conflict resolver.
- Every worker brief uses `GPT-5.4` with `high` reasoning effort.
- Max worker concurrency is `2` during `Lane A` + `Lane B`, then `1` worker at a time for all later gates.
- Recommended orchestration roots in this repo:
  - worktrees: `/Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/`
  - run artifacts: `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/`
- Parent branch and worktree:
  - `codex/namespace-cutover-parent`
  - `/Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent`
- Worker branches and worktrees:
  - `codex/namespace-cutover-lane-a-tree` at `/Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/lane-a-tree`
  - `codex/namespace-cutover-lane-b-docs` at `/Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/lane-b-docs`
  - `codex/namespace-cutover-lane-c1-compiler` at `/Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/lane-c1-compiler`
  - `codex/namespace-cutover-lane-c2-proof` at `/Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/lane-c2-proof`
- Parent-owned run artifacts:
  - queue authority: `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/task-queue.md`
  - session log: `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/session-log.md`
  - worker briefs: `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/worker-briefs/`
  - worker returns: `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/worker-returns/`
  - gate records: `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/gate-records/`
  - sentinels: `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/sentinels/`
  - final validation record: `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/final-validation.md`
- `task-queue.md` is the only orchestration state authority. `session-log.md` is append-only narrative. Sentinels are convenience markers only.

## Hard Guards
- [`PLAN.md`](/Users/spensermcconnell/__Active_Code/system/PLAN.md) is authoritative for scope, phases, validation commands, and failure modes.
- Parent alone may:
  - decide scope
  - widen a lane's ownership
  - resolve cross-lane conflicts
  - merge branches
  - declare a gate passed
  - land back to `main`
- No worker may preserve or introduce compatibility aliases for top-level `pipelines/`, `profiles/`, `runners/`, or top-level `pipeline.yaml`.
- No worker may invent dynamic root config, env overrides, or a migration subsystem.
- Lane ownership is strict. A worker that needs files outside its brief stops and returns control to the parent.
- `pipeline.yaml` is a parent-owned pre-lane decision at `G0`: either delete it as obsolete or preserve unique supported truth under `core/pipelines/default.yaml`.
- `Lane A` does not get to improvise the `pipeline.yaml` decision. Parent locks that choice in `G0-preflight.md` before `Lane A` starts.
- The integrated tree must finish with one canonical declarative compiler-input namespace under `core/`.
- Workers do not merge, rebase other lanes, or consume other worker transcripts as authority.
- All queue state changes are parent-written only.
- Parent must capture `git status --short` before opening worktrees and record the existing modified `PLAN.md` state as baseline evidence at `G0`.

## Workstream Plan

| Gate | Parent action | Parallel state | Gate passes when |
| --- | --- | --- | --- |
| `G0` Preflight | capture honest baseline, lock `pipeline.yaml` decision, create worktrees, seed run artifacts | no workers yet | parent has baseline evidence, locked the `pipeline.yaml` handling, and written worker briefs |
| `G1` Tree/reference move freeze | merge `Lane A` | `Lane B` may already be running | moved tree exists under `core/**`, old top-level roots are retired, live reference rewrites are complete |
| `G2` Compiler-path freeze | merge `Lane C1` | `Lane B` may still be running | compiler source owns `core/**` paths explicitly and builds cleanly |
| `G3` Docs/contracts merge | merge `Lane B` | no new worker starts before this merge | approved docs and contracts teach the same `core/**` contract as code |
| `G4` Proof/test refresh | merge `Lane C2`, run focused rails, run stale-root sweep | no parallel work left | focused validation commands from `PLAN.md` pass on the integrated tree |
| `G5` Final validation and landing | write final record, land to `main`, close the run | parent only | parent branch is validated, merged back to `main`, and closure artifacts are written |

## Parent-Only Critical Path
1. Run `G0` preflight from the current checked-out `main`.
2. Create parent and worker worktrees.
3. Launch `Lane A` and `Lane B` in parallel from `main`.
4. Merge `Lane A` at `G1`. This freezes the physical repo layout and live declarative reference rewrite.
5. Launch `Lane C1` from the updated parent branch.
6. Merge `Lane C1` at `G2`. This freezes compiler-owned path generation and trust classification.
7. Merge `Lane B` at `G3`. Docs must describe the frozen code contract, not an in-between state.
8. Launch `Lane C2` from the updated parent branch only after `G3`.
9. Merge `Lane C2` at `G4`, then run the focused validation rails and stale-root sweep from `PLAN.md`.
10. Land the validated parent branch back to `main` at `G5`.

## Setup Commands / Worktree Topology
```bash
mkdir -p /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover
mkdir -p /Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/{worker-briefs,worker-returns,gate-records,sentinels}

printf '# Namespace Cutover Queue\n' > /Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/task-queue.md
printf '# Namespace Cutover Session Log\n' > /Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/session-log.md

git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent -b codex/namespace-cutover-parent main
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/lane-a-tree -b codex/namespace-cutover-lane-a-tree main
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/lane-b-docs -b codex/namespace-cutover-lane-b-docs main
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/lane-c1-compiler -b codex/namespace-cutover-lane-c1-compiler main
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/lane-c2-proof -b codex/namespace-cutover-lane-c2-proof main
```

Baseline rule before any `git worktree add`:

- parent runs `git status --short` on `/Users/spensermcconnell/__Active_Code/system`
- parent records the baseline, including the current modified `PLAN.md` state, in `gate-records/G0-preflight.md`
- parent confirms that all new branches fork from the currently checked-out `main`, not from a stale side branch or prior parent branch

Queue rows to seed in `task-queue.md`:

| Item | State | Owner | Depends on | Evidence |
| --- | --- | --- | --- | --- |
| `G0-preflight` | `READY` | parent | — | `gate-records/G0-preflight.md` |
| `Lane-A-tree-reference-move` | `READY` | worker | `G0` | `worker-returns/lane-a.md` |
| `Lane-B-docs-contracts` | `READY` | worker | `G0` | `worker-returns/lane-b.md` |
| `G1-tree-freeze` | `READY` | parent | `Lane-A` | `gate-records/G1-tree-freeze.md` |
| `Lane-C1-compiler-paths` | `READY` | worker | `G1` | `worker-returns/lane-c1.md` |
| `G2-compiler-freeze` | `READY` | parent | `Lane-C1` | `gate-records/G2-compiler-freeze.md` |
| `G3-docs-merge` | `READY` | parent | `Lane-B`, `G2` | `gate-records/G3-docs-merge.md` |
| `Lane-C2-proof-refresh` | `READY` | worker | `G3` | `worker-returns/lane-c2.md` |
| `G4-proof-test-refresh` | `READY` | parent | `Lane-C2` | `gate-records/G4-proof-test-refresh.md` |
| `G5-land-main` | `READY` | parent | `G4` | `final-validation.md` |

## Queue And Gate Mechanics

Normal lane lifecycle:

- `READY`: parent seeds the queue row during setup and the lane has not started.
- `IN_PROGRESS`: parent flips the row when the worker brief is issued and the worker begins.
- `AWAIT_PARENT_GATE`: parent flips the row after the worker return is received and the lane is waiting on its merge gate.
- `MERGED_PARENT`: parent flips the row immediately after merging that lane branch into `codex/namespace-cutover-parent`.
- `VERIFIED`: parent flips the row after the corresponding gate checks pass on the merged parent worktree.
- `LANDED`: parent flips the row during `G5` after the validated parent branch lands on `main`.

Gate ownership rules:

- only the parent changes queue states
- a worker return never changes queue state by itself
- a gate is not passed by worker claim; it is passed only by parent merge plus parent verification commands

`Lane B` waiting rule:

- `Lane B` may finish before `G3`
- parent records the worker return, sets `Lane-B-docs-contracts` to `AWAIT_PARENT_GATE`, and leaves it there while `Lane C1` and `G2` finish
- parent flips `Lane-B-docs-contracts` to `MERGED_PARENT` only when `Lane B` is actually merged at `G3`
- parent flips it to `VERIFIED` only after the `G3` doc/contract grep checks pass

## Worker Lanes With Owned Files And Required Commands

| Lane | Branch / worktree | Start gate | Model | Owned files and directories | Required commands before return | Exit conditions |
| --- | --- | --- | --- | --- | --- | --- |
| `Lane A` Tree migration + live reference rewrite | `codex/namespace-cutover-lane-a-tree` / `.worktrees/namespace-cutover/lane-a-tree` | `G0` | `GPT-5.4/high` | `pipelines/**`, `profiles/**`, `runners/**`, `pipeline.yaml`, `core/pipelines/**`, `core/profiles/**`, `core/runners/**`, `core/stages/**`, `core/library/**`, `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/**`, `tests/fixtures/foundation_flow_demo/repo/**` | `find core/pipelines core/profiles core/runners -maxdepth 3 -print | sort`; `test ! -d pipelines`; `test ! -d profiles`; `test ! -d runners`; `test ! -e pipeline.yaml`; if `G0` chose preservation, `test -e core/pipelines/default.yaml`; `rg -n '(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/' core/stages core/library tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo tests/fixtures/foundation_flow_demo/repo` | repo tree reflects the new canonical layout; live declarative content no longer points at old runner/profile roots |
| `Lane B` Docs + contracts | `codex/namespace-cutover-lane-b-docs` / `.worktrees/namespace-cutover/lane-b-docs` | `G0` | `GPT-5.4/high` | `README.md`, `docs/REPO_OVERVIEW.md`, `docs/GLOSSARY.md`, `docs/LEGACY_INVENTORY.md`, `docs/contracts/C-01-approved-repo-surface.md`, `docs/contracts/pipeline-route-and-state-core.md` | `rg -n 'core/pipelines|core/profiles|core/runners' README.md docs/REPO_OVERVIEW.md docs/GLOSSARY.md docs/LEGACY_INVENTORY.md docs/contracts/C-01-approved-repo-surface.md docs/contracts/pipeline-route-and-state-core.md`; `rg -n '(^|[^a-zA-Z])pipelines/|(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/|pipeline\.yaml' README.md docs/REPO_OVERVIEW.md docs/GLOSSARY.md docs/LEGACY_INVENTORY.md docs/contracts/C-01-approved-repo-surface.md docs/contracts/pipeline-route-and-state-core.md` | approved docs and contracts teach one `core/**` namespace; any retained old-root mentions are explicitly historical-only |
| `Lane C1` Compiler path normalization | `codex/namespace-cutover-lane-c1-compiler` / `.worktrees/namespace-cutover/lane-c1-compiler` | `G1` | `GPT-5.4/high` | `crates/compiler/src/pipeline.rs`, `crates/compiler/src/route_state.rs`, `crates/compiler/src/pipeline_compile.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/lib.rs`, optional new `crates/compiler/src/declarative_roots.rs` | `cargo test -p system-compiler --no-run`; `rg -n 'core/pipelines|core/profiles|core/runners' crates/compiler/src`; `rg -n '(^|[^a-zA-Z])pipelines/|(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/|pipeline\.yaml' crates/compiler/src` | compiler-generated canonical paths resolve to `core/**`; trust and refusal text point operators at `core/**` |
| `Lane C2` Proof, tests, and goldens | `codex/namespace-cutover-lane-c2-proof` / `.worktrees/namespace-cutover/lane-c2-proof` | `G3` | `GPT-5.4/high` | `crates/compiler/tests/pipeline_loader.rs`, `crates/compiler/tests/pipeline_catalog.rs`, `crates/compiler/tests/pipeline_route_resolution.rs`, `crates/compiler/tests/pipeline_state_store.rs`, `crates/compiler/tests/pipeline_compile.rs`, `crates/compiler/tests/pipeline_handoff.rs`, `crates/compiler/tests/support/pipeline_proof_corpus_support.rs`, `crates/cli/tests/cli_surface.rs`, `crates/cli/tests/pipeline_handoff_refusals.rs`, `tests/fixtures/pipeline_proof_corpus/**`, `tests/fixtures/foundation_flow_demo/**` | `cargo test -p system-compiler --test pipeline_loader`; `cargo test -p system-compiler --test pipeline_catalog`; `cargo test -p system-compiler --test pipeline_route_resolution`; `cargo test -p system-compiler --test pipeline_state_store`; `cargo test -p system-cli --test cli_surface`; `cargo test -p system-cli --test pipeline_handoff_refusals`; `rg -n '(^|[^a-zA-Z])pipelines/|(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/|pipeline\.yaml' README.md docs crates tests core` | focused rails pass locally in the lane worktree; proof fixtures and goldens emit `core/**` paths only for supported surfaces |

Parent keeps exclusive ownership of:

- `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/**`
- merges and rebases
- `task-queue.md` state changes
- final validation records
- any file not named in an active lane brief

## Lane Subtasks

### Lane A
- `A1` Move top-level declarative roots into `core/pipelines/`, `core/profiles/`, and `core/runners/`.
- `A2` Retire top-level `pipeline.yaml` as a supported surface.
- `A3` Rewrite live include strings in `core/stages/**` from `profiles/...` and `runners/...` to `core/profiles/...` and `core/runners/...`.
- `A4` Rewrite live example and guidance references in `core/library/**`.
- `A5` Mirror the same path rewrite and physical moves in fixture repos under:
  - `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/**`
  - `tests/fixtures/foundation_flow_demo/repo/**`

### Lane B
- `B1` Update repo-layout and command-path wording in `README.md`.
- `B2` Update [`docs/REPO_OVERVIEW.md`](/Users/spensermcconnell/__Active_Code/system/docs/REPO_OVERVIEW.md) to describe `core/pipelines/`, `core/profiles/`, and `core/runners/`.
- `B3` Update `docs/GLOSSARY.md` and `docs/LEGACY_INVENTORY.md` so any old-root wording is clearly historical only.
- `B4` Update `docs/contracts/C-01-approved-repo-surface.md` to make `core/**` the declarative root.
- `B5` Update `docs/contracts/pipeline-route-and-state-core.md` so allowlists and route-state paths point at `core/runners/` and `core/profiles/`.

### Lane C1
- `C1.1` Add a tiny shared declarative-root helper surface if needed to kill string drift.
- `C1.2` Update pipeline discovery and source rendering in `crates/compiler/src/pipeline.rs`.
- `C1.3` Update route-basis path builders and mismatch text in `crates/compiler/src/route_state.rs`.
- `C1.4` Update include classification and rendered path strings in `crates/compiler/src/pipeline_compile.rs`.
- `C1.5` Update canonical trust recognition in `crates/compiler/src/pipeline_handoff.rs`.
- `C1.6` Export any new helper module in `crates/compiler/src/lib.rs` if required.

### Lane C2
- `C2.1` Refresh loader, catalog, route-resolution, and route-state tests to assert `core/**` paths.
- `C2.2` Refresh CLI and handoff refusal tests for `SOURCE: core/...` and `core/**` refusal/help text.
- `C2.3` Refresh proof corpus support code, state seeds, fixture repo layouts, goldens, expected outputs, and evidence transcripts.
- `C2.4` Run the focused validation rails from `PLAN.md` inside the lane worktree before returning to parent.

## Parent Merge Order
1. Merge `Lane A` into `codex/namespace-cutover-parent`.
2. Launch and then merge `Lane C1` from the updated parent branch.
3. Merge `Lane B` into the updated parent branch.
4. Launch and then merge `Lane C2` from the updated parent branch.
5. Run final integrated validation.
6. Land `codex/namespace-cutover-parent` back to `main`.

`Lane B` may finish early, but parent holds that merge until `G3`. That keeps docs aligned to the frozen compiler contract instead of a provisional tree move.

## Worker Return Contract
- Every worker return includes only:
  - changed files
  - commands run
  - exit codes
  - blockers or unresolved assumptions
  - explicit statement that no edits escaped lane ownership
- Parent records each return in:
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/worker-returns/lane-a.md`
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/worker-returns/lane-b.md`
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/worker-returns/lane-c1.md`
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/worker-returns/lane-c2.md`
- Every worker brief records:
  - branch
  - worktree
  - current gate
  - model = `GPT-5.4`
  - reasoning = `high`
  - owned files
  - required commands
  - exit conditions
  - known blockers

## Parent Review Discipline
- Parent reviews worker returns plus narrow diffs only.
- Parent does not treat full worker transcripts as the review artifact.
- Recommended review commands:
```bash
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent diff --stat codex/namespace-cutover-parent...codex/namespace-cutover-lane-a-tree
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent diff --stat codex/namespace-cutover-parent...codex/namespace-cutover-lane-b-docs
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent diff --stat codex/namespace-cutover-parent...codex/namespace-cutover-lane-c1-compiler
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent diff --stat codex/namespace-cutover-parent...codex/namespace-cutover-lane-c2-proof
```
- Parent writes one gate record per merge:
  - `gate-records/G0-preflight.md`
  - `gate-records/G1-tree-freeze.md`
  - `gate-records/G2-compiler-freeze.md`
  - `gate-records/G3-docs-merge.md`
  - `gate-records/G4-proof-test-refresh.md`

## Serialization Boundaries
- `Lane A` must merge before `Lane C1`. Compiler path normalization depends on the final physical root names and moved fixture-repo tree.
- `Lane C1` must merge before `Lane B`. Docs can draft in parallel, but merge waits until code path ownership is frozen.
- `Lane C2` must wait for `G3`. Proof and golden refresh depends on:
  - moved repo trees from `Lane A`
  - frozen compiler-emitted path strings from `Lane C1`
  - the integrated near-final tree that final validation will actually test
- No lane after `Lane A` may re-open physical tree moves unless the parent explicitly updates ownership and queue state.

## Context-Control Rules
- Parent-only artifacts:
  - `task-queue.md`
  - `session-log.md`
  - `gate-records/**`
  - `sentinels/**`
  - `final-validation.md`
- Queue states are parent-written only:
  - `READY`
  - `IN_PROGRESS`
  - `AWAIT_PARENT_GATE`
  - `MERGED_PARENT`
  - `VERIFIED`
  - `BLOCKED`
  - `LANDED`
- Sentinel names are fixed:
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/sentinels/lane-a.done`
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/sentinels/lane-b.done`
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/sentinels/lane-c1.done`
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/sentinels/lane-c2.done`
  - `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/sentinels/parent-final.done`
- Workers read only:
  - the current brief
  - the current gate
  - the queue row the parent marked active
- Blocked-lane procedure:
  - worker stops immediately
  - worker returns changed files, commands run, exit codes, and blocker
  - parent marks the queue row `BLOCKED`
  - parent appends the blocker to `session-log.md`
  - parent writes `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/sentinels/<lane>.blocked`
  - parent chooses exactly one next action:
    - resolve locally in parent
    - relaunch the same lane with a narrowed brief
    - open a replacement lane from the current parent branch

## Tests And Acceptance

### `G0` Preflight
```bash
cd /Users/spensermcconnell/__Active_Code/system
test "$(git branch --show-current)" = "main"
git status --short
git diff -- PLAN.md
find pipelines profiles runners -maxdepth 2 -print | sort
test -e pipeline.yaml
rg -n '(^|[^a-zA-Z])pipelines/|(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/|pipeline\.yaml' README.md docs crates tests core
```
Parent `G0` decisions and evidence are mandatory:

- record `git status --short` in `gate-records/G0-preflight.md`
- record the existing modified `PLAN.md` state in `gate-records/G0-preflight.md`
- lock the `pipeline.yaml` path before `Lane A` starts:
  - `Decision A`: delete `pipeline.yaml` as obsolete
  - `Decision B`: preserve unique supported truth under `core/pipelines/default.yaml`
- write the chosen `pipeline.yaml` decision into `gate-records/G0-preflight.md` and the `Lane A` brief
- only after those steps may parent create the worker briefs and flip `Lane A` / `Lane B` from `READY` to `IN_PROGRESS`

### `G1` Tree/Reference Move Freeze
```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent
git merge --no-ff codex/namespace-cutover-lane-a-tree
test -d core/pipelines
test -d core/profiles
test -d core/runners
test ! -d pipelines
test ! -d profiles
test ! -d runners
test ! -e pipeline.yaml
rg -n '(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/' core/stages core/library tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo tests/fixtures/foundation_flow_demo/repo
```
`G1` passes only if:
- the final `rg` returns no hits
- `pipeline.yaml` is gone
- if `G0` chose preservation, the parent also confirms `core/pipelines/default.yaml` exists before marking `G1` verified

### `G2` Compiler-Path Freeze
```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent
git merge --no-ff codex/namespace-cutover-lane-c1-compiler
cargo test -p system-compiler --no-run
rg -n 'core/pipelines|core/profiles|core/runners' crates/compiler/src
rg -n '(^|[^a-zA-Z])pipelines/|(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/|pipeline\.yaml' crates/compiler/src
```
`G2` passes only if:
- compiler tests build with `--no-run`
- canonical path hits exist for the new roots
- any remaining old-root hits are clearly refusal or historical text, not active canonical path generation

### `G3` Docs/Contracts Merge
```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent
git merge --no-ff codex/namespace-cutover-lane-b-docs
rg -n 'core/pipelines|core/profiles|core/runners' README.md docs/REPO_OVERVIEW.md docs/GLOSSARY.md docs/LEGACY_INVENTORY.md docs/contracts/C-01-approved-repo-surface.md docs/contracts/pipeline-route-and-state-core.md
rg -n '(^|[^a-zA-Z])pipelines/|(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/|pipeline\.yaml' README.md docs/REPO_OVERVIEW.md docs/GLOSSARY.md docs/LEGACY_INVENTORY.md docs/contracts/C-01-approved-repo-surface.md docs/contracts/pipeline-route-and-state-core.md
```
`G3` passes only if the second grep leaves no unlabeled supported-path teaching. Historical references are allowed only when they are explicit historical notes.

### `G4` Proof/Test Refresh
Run the exact focused rails from `PLAN.md` on the integrated parent branch:

```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent
git merge --no-ff codex/namespace-cutover-lane-c2-proof
cargo test -p system-compiler --test pipeline_loader
cargo test -p system-compiler --test pipeline_catalog
cargo test -p system-compiler --test pipeline_route_resolution
cargo test -p system-compiler --test pipeline_state_store
cargo test -p system-cli --test cli_surface
cargo test -p system-cli --test pipeline_handoff_refusals
rg -n '(^|[^a-zA-Z])pipelines/|(^|[^a-zA-Z])profiles/|(^|[^a-zA-Z])runners/|pipeline\.yaml' README.md docs crates tests core
```

`G4` passes only if:
- all six cargo rails pass
- the stale-root grep has no supported-surface regressions
- any intentionally retained historical hits are reviewed and recorded in `gate-records/G4-proof-test-refresh.md`

### `G5` Final Validation And Landing
```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/namespace-cutover/parent
git status --short

cd /Users/spensermcconnell/__Active_Code/system
git checkout main
git merge --ff-only codex/namespace-cutover-parent
```

If `main` moved during the session, parent updates `codex/namespace-cutover-parent`, reruns `G4`, and only then lands.

Closure steps after landing are required:

- write `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/final-validation.md` with:
  - merged branch name
  - final commands run
  - exit codes
  - final grep disposition
  - final acceptance verdict
- mark all queue rows `LANDED` in `task-queue.md`
- write `/Users/spensermcconnell/__Active_Code/system/.runs/namespace-cutover/sentinels/parent-final.done`
- append the landing result and branch state to `session-log.md`

Branch and worktree cleanup policy:

- keep `codex/namespace-cutover-parent` until the landed `main` state is confirmed acceptable
- worker branches may be deleted after landing unless a blocker record or follow-up requires retention
- worker worktrees may be removed after `final-validation.md` and `parent-final.done` exist
- parent worktree may be removed last, after queue rows are `LANDED` and session log closure is written

## Assumptions
- `main` is the correct landing branch for this milestone.
- Parent can create local worktrees and `.runs/namespace-cutover/**` artifacts in this repo.
- `Lane B` owns only the approved docs and contracts named above. If the final stale-root sweep reveals additional approved docs that still teach old roots, parent either patches them locally or reopens `Lane B` with expanded ownership.
- Historical references to old roots may still exist in clearly labeled legacy surfaces, but not in active supported-path teaching, proofs, or compiler output.
