# Plan: Agent-Driven Deterministic Baseline Authoring

## Status

- Source: [`SPEC.md`](./SPEC.md)
- Phase: Plan
- Implementation: not started
- Planning baseline: `da6e6ed`
- Review gate: implementation must not begin until this plan and [`TASKS.md`](./TASKS.md) are approved

## Objective

Remove the CLI-owned human-guided authoring path and embedded Codex transport while retaining deterministic CLI authoring for charter, project context, and environment inventory.

The implementation must preserve the Phase 5 CLI ownership split, the Phase 6 crate dependency direction, the frozen `handbook-pipeline` supported-target wedge, and the runway for a later first-class Handbook contract membrane.

## Planning Inputs

### Current authority

- `SPEC.md`
- `archived/specs/phase-5-cli-shell/**`
- `archived/specs/phase-6-ownership-and-integration-planning/**`
- `archived/specs/phase-6-pipeline-boundary-cleanup/**`
- `archived/specs/handbook-engine-extraction-phase-6-remaining-work-{spec,plan,tasks}.md`
- `docs/ideas/handbook-contract-membrane-architecture-memo.md`
- `docs/ideas/substrate_executable_contracts_architecture.md`

### Live implementation surfaces

- `crates/cli/src/{main,author,author_prompting,doctor,doctor_rendering,shell_shared}.rs`
- `crates/compiler/src/author/**`
- `crates/compiler/src/{blocker,doctor,doctor_shell}.rs`
- `crates/engine/src/author/**`
- `crates/{engine,compiler,cli}/tests/**`
- `crates/pipeline/tests/**` as a no-regression wall only
- `core/library/{authoring,charter,project_context,environment_inventory}/**`
- `install/handbook-home/**`
- `tools/codex/**`
- `tools/ci/{author-live-smoke,codex-skill-live-smoke,install-smoke}.sh`
- `.github/workflows/ci.yml`
- `justfile`

## Current Repo Truth

### Authoring behavior

- Charter already has deterministic YAML file/stdin authoring and validation, plus a separate TTY interview that invokes `codex exec` for synthesis.
- Project context already has deterministic YAML file/stdin authoring, plus a separate TTY interview; it lacks `--validate` parity.
- Environment inventory validates canonical Markdown in `handbook-engine`, but its only CLI mutation path invokes `codex exec`; it lacks typed structured inputs, deterministic rendering, file/stdin input, and validation-only mode.
- `crates/cli/src/author_prompting.rs` is a 501-line guided-input leaf used only by the CLI interview path.
- `portable-pty`, stub Codex scripts, model environment variables, and the live inference CI job exist to prove behavior that this change removes.

### Ownership and dependency posture to preserve

```text
handbook-engine                    # no Handbook crate dependency
├── handbook-pipeline -> engine    # no compiler/CLI dependency
├── handbook-flow -> engine        # no pipeline/compiler/CLI dependency
├── handbook-compiler -> engine + pipeline + flow  # transition/support glue
└── handbook-cli -> owner crates + retained compiler adapters  # product shell
```

Hard constraints:

- Do not add `handbook-compiler` or `handbook-cli` to `handbook-pipeline`.
- Do not add `handbook-pipeline`, `handbook-compiler`, or `handbook-cli` to `handbook-flow`.
- Do not add any Handbook crate dependency to `handbook-engine`.
- Do not move final help, refusal presentation, or exit-code policy out of `handbook-cli`.
- Do not alter declarative pipeline IDs, stage IDs, capture pairings, consumer IDs, or `SupportedTargetRegistry` ownership.
- Do not add contract-membrane types or commands in this cleanup.

### Planning-time baseline proof

The following passed on the planning baseline:

- `cargo test -p handbook-engine --test author_core` — 4 passed
- `cargo test -p handbook-compiler --test author` — 59 passed
- `cargo test -p handbook-cli --test author_cli` — 25 passed
- `cargo test -p handbook-pipeline --test pipeline_catalog` — 15 passed
- `cargo test -p handbook-pipeline --test pipeline_compile` — 21 passed
- `cargo test -p handbook-pipeline --test pipeline_capture` — 46 passed
- `cargo test -p handbook-pipeline --test pipeline_handoff` — 9 passed
- Phase 6 dependency/source-coupling checks — passed

GitNexus is stale at indexed commit `8fbb9ac`; `npx gitnexus analyze` currently aborts with a native N-API error. Existing stale-index impact results classify `execute_author_charter_command` as CRITICAL and `author_environment_inventory` as HIGH. Implementation may not edit those symbols until the index is refreshed or the GitNexus failure is repaired and fresh impact results are reported.

## Dependency Graph

```text
P0  Fresh impact + baseline freeze
 │
 ├── P1.1 Environment input model/template
 │      └── P1.2 Compiler deterministic environment adapter
 │             └── P1.3 Environment CLI validation/authoring cutover
 │
 ├── P2.1 Project-context validation parity + CLI interview removal
 │
 └── P2.2 Charter deterministic-only CLI cutover

P1.3 + P2.1 + P2.2
 ├── P3.1 Remove charter Codex synthesis
 ├── P3.2 Remove environment Codex synthesis
 └── P3.3 Delete CLI prompting/PTY leaves
          └── P3.4 Remove live inference CI/config

P1.3 + P2.1 + P2.2
 ├── P4.1 Unify help/refusal/doctor recovery contract
 └── P4.2 Package all deterministic input resources for the existing skills
          └── P4.3 Prove installed skill/runtime behavior

P3.4 + P4.1 + P4.3
 └── P5 Full functional, coupling, pipeline, and scope proof
```

`P1.1`, `P2.1`, and `P2.2` may be implemented independently after `P0`, but each packet must keep its own targeted tests green. Deletion packets must wait until every caller has moved to deterministic paths.

## Phase 0: Fresh Impact And Baseline Freeze

### P0.1 Repair or refresh GitNexus before symbol edits

Purpose: satisfy the repository's mandatory impact-analysis gate using current graph truth.

Steps:

1. Reproduce `npx gitnexus analyze` failure.
2. Repair the local GitNexus runtime/index issue without changing product code.
3. Run `npx gitnexus analyze` until `npx gitnexus status` reports current `HEAD`.
4. Run upstream impact analysis for every existing symbol named in `TASKS.md`, at minimum:
   - `execute_author_charter_command`
   - `execute_author_project_context_command`
   - `author_environment_inventory`
   - `author_charter_guided`
   - `synthesize_charter_markdown`
   - `synthesize_environment_inventory_markdown`
   - `validate_environment_inventory_markdown`
   - `doctor_shell::author_command`
5. Report HIGH/CRITICAL results before implementation begins.

Checkpoint: no production or test symbol edit occurs before this packet passes.

### P0.2 Re-run the baseline wall

Re-run the planning-time author and pipeline commands before the first implementation commit. Record failures as pre-existing blockers rather than weakening later acceptance criteria.

## Phase 1: Deterministic Environment-Inventory Path

### P1.1 Add the pure environment-inventory input and render contract

Implement the missing engine-owned authoring core without touching the CLI or compiler transport yet.

Deliverables:

- `EnvironmentInventoryStructuredInput` and typed nested entries for the existing canonical sections.
- YAML parsing, normalization, semantic validation, and deterministic Markdown rendering.
- Explicit unknown representation and secret-safe field rules.
- `ENVIRONMENT_INVENTORY_INPUTS.yaml.tmpl` matching the typed contract.
- Engine tests for valid, malformed, incomplete, placeholder, unknown, secret-safe, deterministic, and heading-order cases.

Ownership rule: this packet may extend `handbook-engine` authoring core but may not add a Handbook crate dependency or contract-membrane semantics.

Checkpoint:

```bash
cargo test -p handbook-engine --test author_core
cargo test -p handbook-engine
cargo tree -p handbook-engine -e normal,dev
```

### P1.2 Add the compiler deterministic environment adapter

Add a second, deterministic environment path before deleting the old synthesis path.

Deliverables:

- Parse/validate/render re-exports mapped into existing environment refusal categories.
- `preflight_author_environment_inventory_from_input`.
- `author_environment_inventory_from_input` using existing preflight, lock, canonical target validation, and guarded write behavior.
- Required-charter and optional-project-context validation preserved.
- Compiler tests showing validation without mutation, safe replacement/repair, existing-truth refusal, path/symlink refusal, and no partial write.

Temporary rule: the old nested-Codex function may remain during this packet only so intermediate commits stay green; no new caller may be added.

Checkpoint:

```bash
cargo test -p handbook-compiler --test author
cargo check -p handbook-compiler
```

### P1.3 Cut the environment CLI to file/stdin inputs

Make environment inventory match the other author surfaces end-to-end.

Deliverables:

- `--from-inputs <path|->` and `--validate` arguments.
- File and stdin parsing through the shared CLI input-source boundary.
- Validation-only output with zero mutation.
- Deterministic canonical authoring output for file and stdin sources.
- Immediate missing-input refusal with no model launch.
- Updated environment-inventory help snapshot and CLI tests.

Checkpoint:

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli
```

## Phase 2: Deterministic-Only Existing Author Surfaces

### P2.1 Cut project context to deterministic-only authoring

Deliverables:

- Add `--validate` parity.
- Require `--from-inputs <path|->` for mutation and validation.
- Remove TTY detection, guided defaults, question collection, and guided project-context success/refusal modes from the active command path.
- Preserve YAML parsing, timestamp behavior, canonical render, guarded write, overwrite/repair policy, and file/stdin behavior.
- Update project-context help snapshot and tests.

Checkpoint:

```bash
cargo test -p handbook-cli --test author_cli project_context
cargo test -p handbook-compiler --test author project_context
```

### P2.2 Cut charter to deterministic-only authoring

Deliverables:

- Require `--from-inputs <path|->` for mutation.
- Preserve the current `--validate` contract.
- Simplify `execute_author_charter_command` to the deterministic branch only.
- Remove TTY detection, guided collection, guided success mode, and guided preflight/callback injection from the active CLI path.
- Preserve existing deterministic parser/renderer, canonical validation, write locks, repair policy, file/stdin behavior, and output ownership in CLI.
- Update charter help snapshot and CLI/unit tests.

Checkpoint:

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli
```

## Phase 3: Delete Transport And Prompting Leaves

Deletion happens only after `P1.3`, `P2.1`, and `P2.2` prove there are no live callers.

### P3.1 Remove charter nested-Codex synthesis

Delete:

- `author_charter_guided` and its public re-export.
- `synthesize_charter_markdown`, prompt construction, temporary output handling, model/binary environment variables, and synthesized-output transport validation used only by the deleted path.
- Charter stub-Codex/model/auth/transport tests.

Preserve:

- deterministic charter render validation;
- preflight, authoring lock, canonical write, and refusal mapping;
- any template-library surface still used by supported pipeline/catalog behavior.

Checkpoint:

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-cli --test author_cli
```

### P3.2 Remove environment nested-Codex synthesis

Delete:

- the old no-input `author_environment_inventory` entry point once all callers use `_from_input`;
- `EnvironmentInventorySynthesisInputs` and preparation/prompt/process/temp-output helpers used only by nested Codex;
- environment Codex binary/model environment variables and stub transport tests.

Preserve:

- required charter and optional project-context preflight semantics;
- canonical validation, lock, write, and refusal behavior;
- reusable environment template assets needed by the deterministic renderer or installed skill.

Checkpoint:

```bash
cargo test -p handbook-engine --test author_core
cargo test -p handbook-compiler --test author
cargo test -p handbook-cli --test author_cli
```

### P3.3 Delete CLI prompting and PTY leaves

Delete:

- `crates/cli/src/author_prompting.rs` and its module registration;
- remaining guided collectors, prompt parsers/default builders, TTY/EOF tests, PTY harnesses, and guided fixtures;
- `portable-pty` when no retained test uses it;
- misleading tests that claim the deterministic `--from-inputs` path uses live Codex.

Do not delete declarative pipeline stages or prompt assets merely because they contain “interview” or “Codex”; those are outside this CLI cleanup unless a separate approved contract change exists.

Checkpoint:

```bash
cargo test -p handbook-cli
! rg -n 'author_prompting|interactive_authoring_is_allowed|guided_interview' crates/cli crates/compiler crates/engine
```

### P3.4 Remove live inference CI and configuration

Delete the live author inference job and `tools/ci/author-live-smoke.sh`, remove author-model/API-key variables from CI and `justfile`, and retain deterministic install/skill smoke coverage.

Checkpoint:

```bash
! rg -n 'HANDBOOK_AUTHOR_(CHARTER|ENVIRONMENT_INVENTORY)_CODEX|HANDBOOK_RUN_LIVE_AUTHOR_CHARTER_SMOKE|HANDBOOK_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL' \
  crates tools .github justfile
```

## Phase 4: Shell Contract, Recovery, And Installed Skill

### P4.1 Unify help, output, and recovery guidance

Deliverables:

- Root/author help describes deterministic agent-facing authoring only.
- All three author help surfaces expose the same file/stdin and validation grammar.
- Missing-input refusals are immediate and consistent.
- Doctor/blocker next actions no longer recommend a bare command that would fail; they point to the installed skill and/or the exact `--from-inputs <path|->` form.
- CLI remains the final presentation/exit-code owner; compiler changes remain narrow typed transition support.
- Existing valid canonical artifacts and baseline verdict semantics stay unchanged.

Checkpoint:

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-compiler --test doctor
```

### P4.2 Package deterministic resources without adding skill leaves

Use the existing `handbook` root skill as the consolidated baseline-authoring workflow and retain `handbook-charter-intake` as the charter-specific discovery surface. Do not rename or add skill identities.

Deliverables:

- Root skill instructions cover setup, deterministic authoring of all three artifacts, and `doctor --json` proof.
- Charter leaf remains deterministic and contains no human-interview guidance.
- Installed resources include the normalized input templates needed for charter, project context, and environment inventory.
- `core/library/authoring/charter_authoring_method.md` no longer describes a human-guided surface.
- `tools/codex/install.sh` packages the added resources without changing the Phase 5/6 crate graph.

Checkpoint:

```bash
bash tools/codex/generate.sh --output-root "$(mktemp -d)"
bash tools/ci/install-smoke.sh
```

### P4.3 Extend deterministic skill/runtime smoke proof

Retain `tools/ci/codex-skill-live-smoke.sh` as a skill/install proof—it does not represent nested model execution. Extend or complement it so a clean installed runtime can:

1. scaffold a repository;
2. validate and author charter from installed resources;
3. validate and author project context from installed resources;
4. validate and author environment inventory from installed resources;
5. reach `baseline_complete` through `handbook doctor --json`;
6. prove no Codex binary, API key, model selection, or network inference is required.

Checkpoint:

```bash
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh
```

## Phase 5: Full Verification And Closeout

### P5.1 Functional wall

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace
python3 tools/check_archive_boundary.py --self-test
python3 tools/check_archive_boundary.py
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh
git diff --check
```

### P5.2 Phase 6 coupling and pipeline wall

```bash
! cargo tree -p handbook-pipeline -e normal,dev | rg 'handbook-compiler|handbook-cli'
! cargo tree -p handbook-flow -e normal,dev | rg 'handbook-pipeline|handbook-compiler|handbook-cli'
! cargo tree -p handbook-engine -e normal,dev | rg 'handbook-pipeline|handbook-flow|handbook-compiler|handbook-cli'
! rg -n 'handbook_compiler|handbook_cli' crates/pipeline/src crates/pipeline/tests
! rg -n 'handbook_pipeline|handbook_compiler|handbook_cli' crates/flow/src crates/flow/tests

cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
```

Acceptance:

- No pipeline target, stage, pairing, or public importer boundary changed.
- No lower crate gained a compiler/CLI back-edge.
- No future contract-membrane behavior was implemented under `author`, `pipeline`, `inspect`, or `doctor`.

### P5.3 GitNexus detect-changes and review gate

Run `npx gitnexus detect-changes` against the complete implementation diff. Expected affected scope is limited to authoring core/adapters, CLI authoring/presentation, authoring tests, installed skill resources, and obsolete live-inference CI.

Stop and investigate if GitNexus reports:

- pipeline compile/capture/handoff/route execution-flow changes;
- flow resolver/result/budget changes;
- setup/generate/inspect behavior changes beyond updated author recovery copy;
- new downstream contract-membrane API surfaces.

After the full wall is green, request human review. Do not begin contract-membrane implementation from this plan.

## Commit And Review Checkpoints

Recommended commit boundaries:

1. Environment-inventory pure input/render core.
2. Environment compiler/CLI deterministic path.
3. Project-context deterministic-only cutover.
4. Charter deterministic-only cutover.
5. Dead Codex transport/prompt/PTY cleanup.
6. Help, doctor, skill resources, and deterministic install proof.
7. Final verification-only fixes, if needed.

Each commit must be independently reviewable and keep the relevant targeted tests green. Run GitNexus impact before symbol edits and `detect-changes` before every commit, per repository instructions.

## Explicit Stop Conditions

Stop and ask before proceeding if implementation would require:

- a new Handbook dependency edge not allowed by the spec;
- a change to `handbook-pipeline` supported targets or declarative stage truth;
- a canonical Markdown heading/schema migration for existing artifacts;
- a new skill identity;
- a contract/evidence/verdict/dock/gate model or `handbook contract` command;
- a downstream-intended public API without the memo's crates.io and real Substrate-consumer proof;
- weakening or deleting tests solely to make the cleanup pass.
