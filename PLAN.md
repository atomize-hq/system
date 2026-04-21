<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-system/feat-m8-autoplan-restore-20260421-112647.md -->
# PLAN

## Status

This is the active execution source of truth for `M8`.

It supersedes the charter-only `M7` authoring plan and replaces it with the approved baseline-canonical-truth milestone.

Archived predecessor:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-plan-archive-20260421-100326.md`

Primary design basis:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-design-20260421-082110.md`

Current shipped baseline on `main`:

- `system setup` is the front door
- `system author charter` is shipped
- `system doctor` is the readiness/recovery surface
- canonical truth is moving under `.system/*`
- readiness/product copy still reflects the charter-only `M7` story

The shipped reduced-v1 pipeline-routing contract remains intentionally narrow: activation clauses use only `variables.<name> == true|false`, and string or numeric activation literals are not part of shipped `M1`.

## Landing Notes

`M7` landed the first real authoring wedge on 2026-04-20.

That was the right first move, but it left the repo baseline incomplete:

- `CHARTER` is first-class
- `PROJECT_CONTEXT` is still setup/legacy-shaped, not baseline-authored
- `ENVIRONMENT_INVENTORY` still has split authority in docs, rules, and stage material
- `doctor` does not yet expose a truthful `baseline complete` tier

This milestone fixes that without widening into generic authoring orchestration.

## Active Objective

Ship the `M8` baseline canonical truth tier.

That means:

- keep `system author charter` as the completed first slice
- add `system author project-context`
- add `system author environment-inventory`
- define the baseline canonical set explicitly as:
  - `.system/charter/CHARTER.md`
  - `.system/project_context/PROJECT_CONTEXT.md`
  - `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
- make `doctor` compute readiness from that baseline set
- separate `baseline complete` from later feature-specific readiness
- remove split-authority claims for `ENVIRONMENT_INVENTORY`

This repo remains a compiler/generator product, not a chat runtime and not a generic document-authoring framework.

## Exact Shipped Behavior

The milestone is only done when all of the following are true:

1. `system setup` scaffolds the baseline canonical paths needed for `M8`, including `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`.
2. `system author charter` remains the shipped charter surface with the existing `M7` refusal boundary.
3. `system author project-context` exists as a first-class public command.
4. `system author environment-inventory` exists as a first-class public command.
5. The baseline canonical set is exactly `CHARTER`, `PROJECT_CONTEXT`, and `ENVIRONMENT_INVENTORY`.
6. `BASE_CONTEXT` is not treated as baseline canonical truth anywhere in `M8`.
7. `doctor` exposes distinct baseline states:
   - `SCAFFOLDED`
   - `PARTIAL_BASELINE`
   - `INVALID_BASELINE`
   - `BASELINE_COMPLETE`
8. `doctor` computes `BASELINE_COMPLETE` from canonical `.system` truth only.
9. `FEATURE_SPEC` does not block `BASELINE_COMPLETE`.
10. `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` is the only canonical environment inventory path in product contracts/docs/runtime behavior.
11. Repo-root `ENVIRONMENT_INVENTORY.md` is never described as canonical after `M8`.
12. This milestone ships no end-user migration workflow, no dual-write system, and no compatibility product surface.
13. Any internal cleanup of old repo-root or artifact-path files is handled as repo maintenance, not product behavior.
14. Setup/help/docs/contracts/tests all describe the same baseline-vs-feature readiness boundary.

## Scope

### In scope

- extend the top-level `author` family with:
  - `system author project-context`
  - `system author environment-inventory`
- scaffold the new canonical environment inventory path under `.system/`
- baseline readiness-state model in `doctor`
- canonical-artifact contract updates for baseline truth
- authority cleanup for `ENVIRONMENT_INVENTORY`
- docs/help/contracts/snapshots/tests cutover for the new baseline story
- repo-owned authoring-method artifacts for the new baseline surfaces if needed

### NOT in scope

- bundled `system author baseline`
- `system author feature-spec`
- generic multi-artifact authoring engine
- productized migration/import flow for old repos
- dual-write or long-lived compatibility shims
- redesigning `BASE_CONTEXT`
- changing live execution/demo boundaries
- public distribution changes
- MCP/UI companion work

## Frozen Decisions

These are already settled for `M8`:

1. The next milestone is the formal baseline canonical truth tier, not “charter done, now generate.”
2. `CHARTER`, `PROJECT_CONTEXT`, and `ENVIRONMENT_INVENTORY` make up the baseline set.
3. `BASE_CONTEXT` is out because it is still run metadata, not durable repo truth.
4. `doctor` needs a first-class `baseline complete` state.
5. `ENVIRONMENT_INVENTORY` must have one authoritative home.
6. The milestone is greenfield from a user-adoption perspective, so no shipped migration support is required.
7. No bundled `system author baseline` surface ships in this milestone.
8. No generic authoring abstraction is allowed.
9. `M8` authoring extends a shared compiler core, but artifact-specific logic must be split into small modules under `crates/compiler/src/author/`, not piled into one giant file.
10. `crates/compiler/src/canonical_artifacts.rs` is the single source of truth for the `M8` baseline artifact set and its required/optional semantics.
11. CLI integration tests are the primary proof for `M8` authoring and doctor flows, with unit tests underneath for shared validation and artifact-specific internals.

## Review-Locked Decisions

These decisions were locked during `/plan-eng-review`:

1. `setup` stops scaffolding `.system/feature_spec/FEATURE_SPEC.md` in `M8`. Bootstrap creates only the true baseline set.
2. Repo-root `ENVIRONMENT_INVENTORY.md` is fully unsupported after `M8`. There is no mirror in the live product contract.
3. `doctor` shows a compact ordered checklist when multiple baseline artifacts are incomplete, with item 1 marked as the next safe action.

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing code or asset | Reuse decision |
| --- | --- | --- |
| top-level `author` family and charter wiring | `crates/cli/src/main.rs`, `crates/compiler/src/author.rs` | extend, do not replace |
| canonical artifact starter templates and setup refresh behavior | `crates/compiler/src/setup.rs`, `crates/compiler/src/canonical_artifacts.rs` | reuse |
| readiness rendering and next-safe-action infrastructure | `crates/compiler/src/resolver.rs`, `crates/compiler/src/rendering/shared.rs`, `system doctor` surfaces | reuse and extend with checklist rendering |
| project-context content/template assets | `core/library/project_context/*`, `core/stages/06_project_context_interview.md` | reuse content, do not expose stage ids |
| environment inventory content/template assets | `core/library/environment_inventory/*`, `core/stages/07_foundation_pack.md` | reuse content, replace root-canonical claims |
| docs/help drift rails | `crates/cli/tests/help_drift_guard.rs`, `crates/cli/tests/cli_surface.rs` | reuse and extend |
| existing canonical path for project context | `.system/project_context/PROJECT_CONTEXT.md` in `crates/compiler/src/canonical_artifacts.rs` | keep |

### Scope reduction verdict

This milestone is large in file count, but structurally still small enough to be the right lake:

- no new top-level product noun
- no new persistence model
- no new external transport
- no generic authoring framework
- no user-migration subsystem

Anything bigger than that is overbuilt. Anything smaller leaves the repo baseline half-finished again.

### Search check

- **[Layer 1]** Reuse the existing CLI subcommand and compiler-boundary pattern already proven by `system author charter`.
- **[Layer 1]** Reuse existing template/directive assets for `PROJECT_CONTEXT` and `ENVIRONMENT_INVENTORY` instead of inventing new authoring formats.
- **[Layer 3]** The eureka here is that baseline truth in this repo is not passive documentation. It is machine-usable canonical truth for downstream planning. That is why `PROJECT_CONTEXT` and `ENVIRONMENT_INVENTORY` belong in baseline readiness, while `BASE_CONTEXT` does not.

### TODO cross-reference

Existing TODOs for UI companion, release automation, live execution packets, and richer onboarding remain deferred.

No existing TODO blocks `M8`. The main risk is plan drift, not missing backlog capture.

### Completeness and distribution check

- This is a boilable lake.
- The complete version is baseline authoring + readiness + authority cleanup, not a shortcut.
- No new distributable artifact is introduced, so no new release pipeline is required.

## Architecture Review

### Primary architecture decision

Extend the compiler-owned authoring pattern from `M7` into a bounded baseline tier.

That means:

- CLI owns command parsing, help text, and refusal rendering
- compiler owns authoring semantics, validation, canonical writes, and readiness classification
- baseline readiness is computed from canonical `.system` artifacts only
- no stage-id UX leaks into the public surface

### Baseline truth model

```text
baseline canonical truth
  ├── .system/charter/CHARTER.md
  ├── .system/project_context/PROJECT_CONTEXT.md
  └── .system/environment_inventory/ENVIRONMENT_INVENTORY.md

later feature-phase truth
  └── .system/feature_spec/FEATURE_SPEC.md (later phase, not scaffolded by setup in M8)

run metadata / legacy stage artifacts
  ├── artifacts/base/BASE_CONTEXT.md
  ├── artifacts/project_context/PROJECT_CONTEXT.md
  └── artifacts/foundation/ENVIRONMENT_INVENTORY.md
```

The milestone succeeds only if operators and downstream code no longer have to guess which layer they are looking at.

### Command/data-flow diagram

```text
operator
  │
  ├── system setup
  │     └── writes starter baseline files under `.system/`
  │
  ├── system author charter
  │     └── canonical `.system/charter/CHARTER.md`
  │
  ├── system author project-context
  │     └── canonical `.system/project_context/PROJECT_CONTEXT.md`
  │
  ├── system author environment-inventory
  │     └── canonical `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
  │
  └── system doctor
        ├── checks baseline canonical paths
        ├── classifies readiness state
        └── renders a compact ordered checklist with item 1 as next safe action
```

### Readiness-state diagram

```text
                 missing / starter-owned baseline file
       ┌────────────────────────────────────────────────────┐
       │                                                    ▼
  [SCAFFOLDED] ── author one artifact ──> [PARTIAL_BASELINE]
       ▲                                      │
       │                                      │ another missing/starter file
       │                                      │
       │ invalid file repaired                ▼
       └────────────── [INVALID_BASELINE] <───┘
                               │
                               │ all baseline files valid
                               ▼
                      [BASELINE_COMPLETE]
                               │
                               │ later milestone work
                               ▼
                         [FEATURE_READY]
```

### Module boundaries

| Area | Ownership | Planned work |
| --- | --- | --- |
| `crates/cli/src/` | CLI | add project-context and environment-inventory subcommands, help text, refusal mapping |
| `crates/compiler/src/author.rs` or adjacent compiler authoring modules | compiler | shared baseline-authoring execution and validation |
| `crates/compiler/src/canonical_artifacts.rs` | compiler | canonical path/starter-template updates, including environment inventory |
| `crates/compiler/src/resolver.rs` + renderers | compiler | baseline readiness-state computation and messaging |
| `core/library/project_context/` | authoring assets | reuse/update content for public authoring |
| `core/library/environment_inventory/` | authoring assets | reuse/update content, remove root-canonical claims |
| `core/rules/` + `docs/contracts/` + `docs/` | product truth | cut authority/readiness wording over to the M8 model |

### Security and production-failure review

| New codepath | Realistic failure | Planned response |
| --- | --- | --- |
| `system author project-context` | writes invalid or incomplete structure | structural validation refusal, no canonical write |
| `system author environment-inventory` | old root-canonical assumptions leak into generated content | explicit canonical-path contract, root claims removed from directives and docs |
| setup scaffold update | environment inventory starter missing or wrong path | setup regression tests |
| readiness computation | feature-spec or legacy artifact accidentally blocks baseline complete | baseline-state tests against canonical `.system` set only |
| docs/contracts cutover | README/help/contracts disagree on authority boundary | help-drift and docs parity regression coverage |

## Code Quality Review

### Minimal-diff rules

- extend the existing `author` family, do not add a new top-level command
- reuse compiler-side authoring boundary, do not scatter artifact-specific logic through CLI renderers
- keep baseline classification explicit, not clever
- update existing contracts/docs instead of introducing shadow design docs in code comments everywhere
- avoid creating a separate migration subsystem for a greenfield milestone
- keep one shared compiler authoring core, but split artifact-specific logic into small modules rather than growing `crates/compiler/src/author.rs` into a blob
- make `canonical_artifacts` the baseline source of truth; setup/doctor/rendering should consume it instead of redefining baseline rules

### DRY guardrails

1. Do not build three independent authoring engines. Reuse the `M7` authoring pattern.
2. Do not let readiness logic live in multiple places with different artifact sets.
3. Do not maintain two canonical stories for `ENVIRONMENT_INVENTORY`.
4. Do not let legacy `artifacts/*` outputs silently continue to drive baseline truth.
5. Do not duplicate baseline required/optional semantics across setup, resolver, and rendering.

### Overbuild traps that fail review

- bundled `system author baseline`
- auto-import/migration machinery for nonexistent users
- generic “canonical document” framework
- reclassifying `BASE_CONTEXT` without redesigning it
- long-lived root-file compatibility shims

### Files that deserve inline ASCII comments

- compiler-side readiness classification code
- any shared baseline-authoring entrypoint that multiple commands converge on
- the module boundary comment in `crates/compiler/src/author/` explaining shared core vs artifact-specific modules
- canonical-artifact ownership code if both baseline and later feature-phase artifacts are handled nearby

## Test Review

### Coverage diagram

```text
M8 BASELINE TIER COVERAGE
=========================
[+] setup baseline scaffolding
    │
    ├── [GAP] creates `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
    ├── [GAP] preserves existing baseline canonical truth on refresh
    └── [GAP] no longer scaffolds `.system/feature_spec/FEATURE_SPEC.md`

[+] system author project-context
    │
    ├── [GAP] scaffolded happy path writes canonical `.system/project_context/PROJECT_CONTEXT.md`
    ├── [GAP] refuses overwrite of existing valid truth
    ├── [GAP] validates structure before write
    └── [GAP] public help/docs describe it as baseline authoring, not legacy stage capture

[+] system author environment-inventory
    │
    ├── [GAP] scaffolded happy path writes canonical `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
    ├── [GAP] refuses overwrite of existing valid truth
    ├── [GAP] strips root-canonical language from generated/runtime contract
    └── [GAP] does not write repo-root canonical truth

[+] doctor readiness states and checklist
    │
    ├── [GAP] SCAFFOLDED when all three baseline files are starter-owned/missing
    ├── [GAP] PARTIAL_BASELINE when some but not all baseline files are valid
    ├── [GAP] INVALID_BASELINE when a non-starter file fails structural validation
    ├── [GAP] BASELINE_COMPLETE when all baseline files are valid
    ├── [GAP] FEATURE_SPEC absence does not block BASELINE_COMPLETE
    └── [GAP] multiple missing artifacts render a compact ordered checklist with item 1 as next safe action

[+] authority boundary
    │
    ├── [GAP] `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` is the only canonical env-inventory path
    ├── [GAP] repo-root ENVIRONMENT_INVENTORY is not described as canonical or emitted as a mirror in docs/help/contracts/runtime
    └── [GAP] BASE_CONTEXT remains outside baseline readiness

─────────────────────────────────
COVERAGE: 0/17 M8 paths implemented yet
  Setup: 0/3
  Project Context: 0/4
  Environment Inventory: 0/4
  Doctor readiness: 0/6
  Authority boundary: 0/3
─────────────────────────────────
```

### Required tests and expected homes

Primary testing posture for `M8`:

- command-surface behavior is proved first through CLI integration tests
- shared validation and artifact-specific normalization rules get unit coverage underneath
- no new baseline authoring surface lands with unit-only proof

| Test requirement | Expected home | Type |
| --- | --- | --- |
| setup scaffolds env inventory starter path and stops scaffolding feature spec | `crates/compiler/tests/setup.rs` | regression |
| setup refresh preserves valid baseline truth | `crates/compiler/tests/setup.rs` | regression |
| project-context author happy path | `crates/cli/tests/author_cli.rs`, `crates/compiler/tests/author.rs` | integration/unit |
| environment-inventory author happy path | `crates/cli/tests/author_cli.rs`, `crates/compiler/tests/author.rs` | integration/unit |
| refusal on existing valid truth for both new commands | `crates/compiler/tests/author.rs` | regression |
| structural validation failure maps to INVALID_BASELINE | `crates/compiler/tests/author.rs`, `crates/compiler/tests/resolver_core.rs` | regression |
| doctor readiness-state classification for all four baseline states | `crates/compiler/tests/resolver_core.rs` or matching doctor/rendering tests | regression |
| doctor checklist rendering marks item 1 as next safe action | `crates/compiler/tests/rendering_surface.rs`, CLI regression tests | regression |
| feature-spec absence and non-scaffolding do not block baseline complete | `crates/compiler/tests/resolver_core.rs`, CLI regression tests | regression |
| docs/help/contracts no longer call root env inventory canonical | `crates/cli/tests/help_drift_guard.rs`, snapshots, docs parity | regression |
| BASE_CONTEXT remains excluded from baseline readiness | `crates/compiler/tests/resolver_core.rs` | regression |

### Mandatory regression rules

- any readiness regression that still makes feature-spec block baseline completion is critical
- any authority regression that still treats repo-root env inventory as canonical or emits it as a live mirror is critical
- any authoring regression that silently overwrites valid baseline truth is critical

## Failure Modes Registry

| Failure mode | Severity | Test cover | Error handling | User-visible outcome |
| --- | --- | --- | --- | --- |
| environment inventory still has two canonical homes | Critical | required | hard failure in docs/contracts/tests | prevents split authority |
| project context authoring writes invalid structure | High | required | explicit refusal | clear repair path |
| invalid baseline file is reported as complete | Critical | required | explicit invalid state | prevents false readiness |
| setup still scaffolds feature spec in M8 | High | required | regression failure | prevents baseline boundary drift |
| doctor shows only one opaque next action when multiple baseline files are incomplete | High | required | checklist rendering | prevents hidden recovery work |
| BASE_CONTEXT accidentally enters baseline checks | High | required | explicit exclusion tests | prevents boundary drift |
| docs/help keep old root-canonical env language | High | required | drift-guard/docs parity | avoids product-story split |

Critical-gap rule:

- any failure mode with no test, no refusal, and silent wrong readiness is a release blocker

## Performance Review

### Determinism and bounded-work rules

- baseline readiness must read only the bounded canonical baseline set, not scan arbitrary repo files
- authoring paths must perform one write to one canonical path per successful run
- no productized migration/import workflow means no background reconciliation logic
- help/docs parity remains machine-checked because wording drift is part of the risk surface here

### Performance smells that fail review

- readiness computed from broad filesystem heuristics instead of explicit artifact list
- duplicated validation logic across CLI and compiler layers
- legacy root/artifact-path reads kept in the hot path after cleanup

## Deterministic Implementation Contract

The plan is not implementation-ready unless two independent workers would choose the same structural diff.

For `M8`, that means these decisions are no longer discretionary:

1. `crates/compiler/src/canonical_artifacts.rs` becomes the single registry for all canonical artifact semantics used by setup, doctor, and downstream rendering.
   It must explicitly model four artifacts:
   - `CHARTER`
   - `PROJECT_CONTEXT`
   - `ENVIRONMENT_INVENTORY`
   - `FEATURE_SPEC`
2. The registry must separate these concerns instead of collapsing them into one `required` boolean:
   - baseline-readiness participation
   - setup scaffolding participation
   - later feature-phase participation
   - canonical path ownership
3. `system doctor` must stop using raw planning-packet `READY` versus `BLOCKED` as its repo-baseline model.
   `doctor` needs a compiler-owned baseline-readiness result with named states:
   - `SCAFFOLDED`
   - `PARTIAL_BASELINE`
   - `INVALID_BASELINE`
   - `BASELINE_COMPLETE`
4. `system author` grows exactly two new public commands in `M8`:
   - `system author project-context`
   - `system author environment-inventory`
5. Compiler-side authoring stops growing as one file.
   `M8` must leave authoring split under `crates/compiler/src/author/` with these module targets:
   - `mod.rs`
   - `charter.rs`
   - `project_context.rs`
   - `environment_inventory.rs`
6. Repo-root `ENVIRONMENT_INVENTORY.md` is not a supported canonical output after `M8`.
   The live product contract is the `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` path only.
   Any legacy capture/mirror cleanup is internal repo maintenance, not shipped compatibility behavior.
7. `setup` may keep its own mutation result vocabulary, but it must stop implying that repo readiness is equivalent to packet readiness or charter-only completion.
   `setup` hands off to `doctor` for baseline truth.
8. `doctor` next actions must name the command when a command exists.
   Do not leave `project-context` or `environment-inventory` recovery as a generic “fill canonical artifact” file-path hint.

## Implementation Plan

### Slice 1: Shared baseline model and setup cutover

**Owned files**

- `crates/compiler/src/canonical_artifacts.rs`
- `crates/compiler/src/setup.rs`
- `crates/compiler/src/refusal.rs`
- `crates/compiler/src/rendering/shared.rs`
- `crates/cli/src/main.rs`
- `crates/compiler/tests/setup.rs`
- `crates/cli/tests/cli_surface.rs`
- `crates/cli/tests/snapshots/system-setup-help.txt`
- `crates/cli/tests/snapshots/system-setup-init-help.txt`
- `crates/cli/tests/snapshots/system-setup-refresh-help.txt`

**Required code changes**

1. Extend the canonical artifact registry with `EnvironmentInventory`.
2. Replace the current single-axis required/optional model with explicit baseline semantics so:
   - `CHARTER`, `PROJECT_CONTEXT`, and `ENVIRONMENT_INVENTORY` are baseline artifacts
   - `FEATURE_SPEC` remains canonical but is later-phase only
3. Make setup scaffold exactly the baseline starter set for `M8`.
   That means:
   - create `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`
   - keep `.system/charter/CHARTER.md`
   - keep `.system/project_context/PROJECT_CONTEXT.md`
   - stop creating `.system/feature_spec/FEATURE_SPEC.md`
4. Remove the current setup note that says project context is optional for the setup-owned story.
5. Expand next-safe-action vocabulary so later doctor/checklist rendering can point to the concrete author commands, not only file paths.

**Acceptance**

- setup creates only the baseline starter set
- setup refresh preserves valid baseline truth
- no shared compiler or CLI file still assumes `FEATURE_SPEC` is part of setup bootstrap
- setup snapshots and CLI surface tests are updated to the new baseline language

### Slice 2: Restructure compiler authoring into explicit modules

**Owned files**

- `crates/compiler/src/author.rs` replaced by `crates/compiler/src/author/mod.rs`
- `crates/compiler/src/author/charter.rs`
- `crates/compiler/src/author/project_context.rs`
- `crates/compiler/src/author/environment_inventory.rs`
- `crates/compiler/src/lib.rs`

**Required code changes**

1. Move the existing charter implementation into `author/charter.rs` without changing its shipped refusal boundary.
2. Make `author/mod.rs` the shared entrypoint for all authoring surfaces.
3. Keep only shared utilities in the module root:
   - canonical-write guardrails
   - starter-template detection
   - shared validation/refusal helpers
   - codex-process plumbing only where truly shared
4. Do not leave `project-context` or `environment-inventory` as giant copy-paste branches inside one file.

**Acceptance**

- `charter` behavior remains unchanged except for mechanical module movement
- the new authoring surfaces plug into the same compiler-owned write boundary
- the post-`M8` authoring tree has obvious ownership by artifact

### Slice 3: Add `system author project-context`

**Owned files**

- `crates/cli/src/main.rs`
- `crates/compiler/src/author/project_context.rs`
- `core/library/project_context/PROJECT_CONTEXT.md.tmpl`
- `core/library/project_context/project_context_gen_directive.md`
- `crates/compiler/tests/author.rs`
- `crates/cli/tests/author_cli.rs`
- `crates/cli/tests/snapshots/system-author-help.txt`
- new help snapshot for `system author project-context`

**Required code changes**

1. Add the public CLI subcommand and help text.
2. Add compiler-owned project-context authoring that writes only `.system/project_context/PROJECT_CONTEXT.md`.
3. Match `M7` write-safety posture:
   - missing or starter-owned file can be authored
   - existing valid non-starter truth refuses overwrite by default
4. Add project-context structural validation strong enough for doctor to classify invalid versus starter versus valid.
5. Keep public UX free of stage ids and artifact-copy jargon.

**Acceptance**

- `system author project-context` works end to end from scaffolded repo to canonical write
- overwrite refusal matches the charter precedent
- invalid project-context content can be surfaced later by doctor as `INVALID_BASELINE`

### Slice 4: Add `system author environment-inventory` and cut authority

**Owned files**

- `crates/cli/src/main.rs`
- `crates/compiler/src/author/environment_inventory.rs`
- `core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl`
- `core/library/environment_inventory/environment_inventory_directive.md`
- `core/rules/p0_absolute.md`
- `crates/compiler/tests/author.rs`
- `crates/cli/tests/author_cli.rs`
- new help snapshot for `system author environment-inventory`

**Required code changes**

1. Add the public CLI subcommand and help text.
2. Add compiler-owned environment-inventory authoring that writes only `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`.
3. Rewrite environment-inventory prompt/template/rules text so the `.system` path is the sole canonical home.
4. Remove product-surface language that still claims:
   - repo-root `ENVIRONMENT_INVENTORY.md` is canonical
   - `artifacts/foundation/ENVIRONMENT_INVENTORY.md` is the canonical store of record
5. Keep pipeline-capture legacy artifact behavior out of the `M8` product contract.
   If older fixture or capture tests still reference repo-root or artifact copies, they remain legacy implementation surfaces and must not be described as live canonical truth in docs/help/rules.

**Acceptance**

- `system author environment-inventory` writes only the `.system` canonical path
- no live authoring/rules/help copy still names the repo-root file as canonical
- the plan leaves no ambiguity about which file downstream operators should edit

### Slice 5: Introduce a compiler-owned doctor model for baseline readiness

**Owned files**

- new `crates/compiler/src/doctor.rs`
- `crates/compiler/src/lib.rs`
- `crates/cli/src/main.rs`
- `crates/compiler/tests/resolver_core.rs`
- `crates/compiler/tests/rendering_surface.rs`
- `crates/cli/tests/cli_surface.rs`

**Required code changes**

1. Stop making `system doctor` a thin wrapper around packet `resolve()`.
2. Add a compiler-owned baseline-readiness result that classifies exactly:
   - `SCAFFOLDED`
   - `PARTIAL_BASELINE`
   - `INVALID_BASELINE`
   - `BASELINE_COMPLETE`
3. Make the classifier inspect only the three baseline artifacts.
   `FEATURE_SPEC` must not participate in this state machine.
4. Render a compact ordered checklist when more than one baseline artifact still needs work.
   Item 1 is the next safe action.
5. Use concrete command guidance:
   - `run \`system author charter\``
   - `run \`system author project-context\``
   - `run \`system author environment-inventory\``
6. Preserve packet resolver behavior as a separate concern.
   `doctor` reports repo-baseline truth, not planning-packet selection.

**Acceptance**

- doctor no longer prints a misleading bare `READY` when only packet prerequisites happen to pass
- invalid authored files are separated from missing/starter-owned files
- feature-spec absence does not affect baseline completion

### Slice 6: Product-story cutover and proof

**Owned files**

- `README.md`
- `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
- `docs/CLI_OUTPUT_ANATOMY.md`
- `docs/VISION.md`
- `docs/GLOSSARY.md`
- `docs/legacy/SYSTEM_MODEL.md`
- `docs/legacy/stages/stage.07_foundation_pack.md`
- `docs/legacy/guides/mechanisms/environment_inventory.md`
- `core/library/foundation_pack/foundation_pack_directive.md`
- `crates/cli/tests/help_drift_guard.rs`

**Required code and doc changes**

1. Cut every live product description from charter-only or root-canonical wording to the `M8` baseline tier.
2. Update the artifact contract docs so `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` is the canonical environment-inventory path.
3. Where legacy docs are kept for historical mechanism coverage, mark them as legacy and remove statements that would contradict the shipped product contract.
4. Make help-drift coverage fail if author/setup/doctor help slips back to the old story.

**Acceptance**

- docs, help, contracts, and rules all tell the same baseline story
- no user-facing source instructs the operator to treat repo-root `ENVIRONMENT_INVENTORY.md` as canonical
- proof is test-backed, not trust-me prose

## Worktree Parallelization Strategy

### Required lane ownership

Parallel work is safe only if ownership is explicit.

| Lane | Ownership | Write set | Must not edit |
| --- | --- | --- | --- |
| Lane A: shared baseline foundation | baseline registry, setup, shared next-safe-action vocabulary | `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/setup.rs`, `crates/compiler/src/refusal.rs`, `crates/compiler/src/rendering/shared.rs`, setup-related CLI/help snapshots | author modules, doctor module, docs cutover |
| Lane B: author module restructure + project-context | author module split and project-context authoring | `crates/compiler/src/author/*`, project-context assets, project-context author tests | authority docs, doctor module |
| Lane C: environment-inventory authority cutover | environment-inventory authoring plus authority docs/rules | `crates/compiler/src/author/environment_inventory.rs`, environment-inventory assets, `core/rules/p0_absolute.md`, authority docs listed in Slice 6 | setup registry internals, doctor module |
| Lane D: doctor closeout | baseline readiness classification and doctor rendering | `crates/compiler/src/doctor.rs`, `crates/cli/src/main.rs`, doctor-facing tests and snapshots | author asset files, authority docs except wording needed for doctor snapshots |

### Handoff rules

1. Lane A lands first.
   It establishes the shared artifact model and removes the current “feature spec is part of setup bootstrap” assumption.
2. Lane B and Lane C branch only after Lane A lands.
   They depend on the same canonical registry and next-safe-action vocabulary.
3. Lane D starts only after both Lane B and Lane C land.
   Doctor cannot finalize checklist text or state classification until both new authoring surfaces and the environment-inventory authority boundary are real.

### Why this is the only safe parallel split

- `crates/cli/src/main.rs` is a conflict magnet.
  Do not have multiple lanes racing there.
  Lane D owns the final CLI closeout after the compiler-side authoring work is real.
- `crates/compiler/src/author/*` is shared infrastructure.
  Project-context and environment-inventory authoring may share helpers, but only Lane B owns the structural module split.
- authority-doc cleanup is not a side quest.
  Lane C owns it because environment-inventory canonical-path truth is part of the shipped feature, not post-merge polish.

### Parallelization verdict

The safe plan is one sequential foundation lane, two middle implementation lanes, then one sequential doctor closeout lane.
That is the highest parallelism available without inviting merge-conflict roulette or divergent product wording.

## Verification Commands

Run these as the minimum acceptance proof for `M8`:

```bash
cargo test -p system-compiler --test setup --test author --test resolver_core --test rendering_surface
cargo test -p system-cli --test author_cli --test cli_surface --test help_drift_guard
```

If help snapshots or new command snapshots are split into additional test files, they join this minimum bar. `M8` is not done on unit coverage alone.

## Exit Criteria

The milestone is complete only when:

- the active plan and approved design agree on the `M8` baseline tier
- setup scaffolds the baseline set required for `M8`
- setup does not scaffold `FEATURE_SPEC`
- `system author project-context` and `system author environment-inventory` are first-class surfaces
- baseline readiness is computed from `CHARTER`, `PROJECT_CONTEXT`, and `ENVIRONMENT_INVENTORY`
- `doctor` exposes `SCAFFOLDED`, `PARTIAL_BASELINE`, `INVALID_BASELINE`, and `BASELINE_COMPLETE`
- `FEATURE_SPEC` does not block `BASELINE_COMPLETE`
- `.system/environment_inventory/ENVIRONMENT_INVENTORY.md` is the only canonical environment inventory path in the live product story
- repo-root `ENVIRONMENT_INVENTORY.md` is not described as canonical anywhere in live docs/contracts/help and is not emitted as a live mirror
- `BASE_CONTEXT` remains outside the baseline tier
- doctor shows a compact ordered checklist when more than one baseline action remains
- doctor next actions name the concrete `author` command for each missing or invalid baseline artifact
- compiler authoring is split under `crates/compiler/src/author/` rather than left as a monolith
- regression coverage proves the authority boundary and readiness model

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 2 | CLEAR | 16 proposals, 16 accepted, 0 deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 7 | CLEAR | 6 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

- **UNRESOLVED:** 0
- **VERDICT:** CEO + ENG CLEARED — ready to implement
