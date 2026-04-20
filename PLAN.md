# PLAN

## Status

This is the active execution source of truth for the next `system` wedge.

The prior foundation-spine plan was archived to the gstack project directory at:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-plan-archive-20260419-120831.md`

That archived plan remains important evidence for why the current Rust CLI looks the way it does. It is no longer the active objective.

## Active Objective

Ship the first real canonical authoring wedge after `M6`.

That means:

- add a top-level `author` command family
- start with one real surface, `system author charter`
- support both visible charter paths in the first shipped slice:
  - interview-style authoring
  - deterministic inputs authoring
- write canonical truth to `.system/charter/CHARTER.md`
- cut `setup`, `doctor`, and `generate` over to the new authoring surface instead of generic blank-file instructions
- publish one repo-owned authoring method artifact so humans and agents follow the same charter-production method

This repo remains the generator/compiler layer for a larger workflow stack. The job here is not to become a chat runtime. The job is to own the boring, exact, reproducible authoring path.

## Problem Statement

`M6` fixed the front door honestly.

`system setup` now establishes canonical `.system/` truth, distinguishes scaffolded from ready states, and keeps recovery inside the supported Rust CLI surface.

But the very next operator job is still outside the product.

Right after setup, the operator still lands in a blank-file gap:

- `.system/charter/CHARTER.md` exists as scaffolding
- the system has no first-class command for turning that scaffold into usable canonical truth
- the best current charter-generation assets still live behind pipeline-stage and directive machinery shaped for legacy or dev/test usage
- agents do not yet have one compiler-owned, repo-owned authoring method artifact that says exactly how to produce a charter well

So the repo can become truthful, but not yet quickly become usable.

That is the next bottleneck.

## Scope

### In scope

- one top-level `author` family as a supported CLI surface
- one shipped command in that family:
  - `system author charter`
- one visible deterministic-input variant for the same wedge:
  - `system author charter --from-inputs <path|->`
- compiler-owned charter authoring execution that writes canonical `.system/charter/CHARTER.md`
- reuse of the existing charter interview, synthesize, directive, and template assets where they still fit
- one repo-owned charter authoring method artifact for humans and agents
- cutover of `setup`, `doctor`, and `generate` next-safe-action/refusal guidance to the new authoring surface
- docs, contracts, help text, snapshots, and tests for the new command family

### Out of scope for this plan

- project-context authoring as a shipped product surface
- feature-spec authoring as a shipped product surface
- generic “author any canonical artifact” work
- UI, MCP, or web companion work
- public distribution changes
- chat-style onboarding automation beyond the bounded charter authoring wedge
- widening canonical truth beyond `.system/*`

Those can come back only after the charter wedge is real, trusted, and boring.

## Operator And Wedge

Primary operator:

- the planning/orchestration operator standing up canonical planning truth in a real repo, initially you

What they need right after `setup`:

- a first-class command that replaces the scaffolded charter with usable canonical truth
- a deterministic path for automation and agents
- an interview path for humans who need guidance
- one repo-owned method that keeps outputs consistent

Current failure mode:

- setup succeeds
- doctor and generate correctly stay blocked on scaffolded truth
- the next instruction is still “fill canonical artifact ...”
- the operator leaves the product surface and improvises

That is the blank-file moment this wedge has to remove.

## Current Product Truth After M6

The current product model is already locked in a few important ways:

- `setup` is the durable front door
- `doctor` is the recovery and readiness surface
- `generate` is the ready-path packet surface
- canonical project truth lives under `.system/*`
- derived `artifacts/*` outputs are not canonical truth
- required starter templates must not satisfy readiness until replaced with real content

The new authoring wedge must fit that model cleanly. It cannot quietly reintroduce a second truth system.

## Guardrails

These rules are active for every later session that touches this plan:

1. Do not move canonical truth out of `.system/*`.
2. Do not make `author` a hidden alias for `setup` or `doctor`.
3. Do not ship a public authoring surface that still writes only to `artifacts/*` or repo-root mirrors.
4. Do not build a second authoring engine if the current compiler-owned directive/template machinery can be reused.
5. Do not expose internal pipeline stage ids as the public authoring product surface.
6. Do not widen the wedge to project context or feature spec just because the namespace can support it later.
7. Do not let docs/help/contracts ship split-brain command vocabulary.
8. Do not call the wedge done if the happy path still requires manual blank-file editing.
9. Do not hide later-surface ambition inside `setup` just to avoid one more top-level verb.

## Reference Inputs And Current Sources Of Truth

Reference material for this wedge lives in:

- [README.md](README.md)
- [docs/START_HERE.md](docs/START_HERE.md)
- [docs/CLI_PRODUCT_VOCABULARY.md](docs/CLI_PRODUCT_VOCABULARY.md)
- [docs/CLI_COMMAND_HIERARCHY.md](docs/CLI_COMMAND_HIERARCHY.md)
- [docs/contracts/C-02-rust-workspace-and-cli-command-surface.md](docs/contracts/C-02-rust-workspace-and-cli-command-surface.md)
- [docs/contracts/C-03-canonical-artifact-manifest-contract.md](docs/contracts/C-03-canonical-artifact-manifest-contract.md)
- [crates/compiler/src/setup.rs](crates/compiler/src/setup.rs)
- [crates/compiler/src/resolver.rs](crates/compiler/src/resolver.rs)
- [crates/cli/src/main.rs](crates/cli/src/main.rs)

Existing charter-generation assets live in:

- [core/stages/04_charter_inputs.md](core/stages/04_charter_inputs.md)
- [core/stages/05_charter_interview.md](core/stages/05_charter_interview.md)
- [core/stages/05_charter_synthesize.md](core/stages/05_charter_synthesize.md)
- [core/library/charter/charter_inputs_directive.md](core/library/charter/charter_inputs_directive.md)
- [core/library/charter/charter_gen_directive.md](core/library/charter/charter_gen_directive.md)
- [core/library/charter/charter_synthesize_directive.md](core/library/charter/charter_synthesize_directive.md)
- [core/library/charter/charter.md.tmpl](core/library/charter/charter.md.tmpl)
- [core/library/charter/CHARTER_INPUTS.yaml.tmpl](core/library/charter/CHARTER_INPUTS.yaml.tmpl)

Current writer and route-state machinery lives in:

- [crates/compiler/src/pipeline_capture.rs](crates/compiler/src/pipeline_capture.rs)
- [docs/contracts/pipeline-capture-preview-and-apply.md](docs/contracts/pipeline-capture-preview-and-apply.md)
- [pipelines/foundation_inputs.yaml](pipelines/foundation_inputs.yaml)

If an existing asset is still useful, reuse it. If it encodes the wrong authority boundary, wrap or rewrite it. Do not cargo-cult the old shape.

## Capability Ledger

| ID | Capability to land | Existing evidence | Current pain when missing | Priority | Target milestone |
| --- | --- | --- | --- | --- | --- |
| AU-01 | Top-level `author` command family | Approved design doc, CLI vocabulary/hierarchy docs | The product has no first-class authoring lifecycle after setup | P0 | M7 |
| AU-02 | Interview-style charter authoring surface | `stage.05_charter_interview`, charter directives/templates | Humans still fall into blank-file editing | P0 | M7 |
| AU-03 | Deterministic-input charter authoring surface | `stage.04_charter_inputs`, `stage.05_charter_synthesize` | Agents/automation lack a stable no-interview path | P0 | M7 |
| AU-04 | Canonical `.system/charter/CHARTER.md` write path | `M6` canonical truth contract, setup/doctor readiness model | Existing charter machinery writes to the wrong authority boundary | P0 | M7 |
| AU-05 | Repo-owned charter authoring method artifact | existing charter directives plus approved design intent | Humans and agents do not share one exact method | P0 | M7 |
| AU-06 | Setup/doctor/generate cutover to `author` | `setup.rs`, resolver blockers, CLI help/docs contracts | Post-setup next actions still point at manual editing | P0 | M7 |
| AU-07 | Docs/help/contracts/snapshots parity for sixth surface | existing drift guards and contract rails | The product story will split immediately if `author` lands partially | P0 | M7 |
| AU-08 | Future namespace path for project-context and later authoring flows | approved `3A` decision | Later authoring surfaces will incur taxonomy debt if charter lands in the wrong family | P1 | M7 |

## Ordered Milestones

### M7. Canonical Authoring Wedge

Status:

- next active milestone after shipped `M6` on `main`
- this is the active implementation plan for making authoring a first-class product surface

Goal:

- introduce a top-level `author` family
- ship one real slice, `system author charter`
- support both visible charter paths in the same milestone:
  - interview-style
  - deterministic inputs
- write canonical truth into `.system/charter/CHARTER.md`
- cut the readiness story over so setup/doctor/generate point at `author`, not manual blank-file editing
- publish one repo-owned method artifact that explains how charter authoring works

Why this exists now:

- `M6` honestly established canonical truth and readiness semantics
- the next operator job is still outside the product
- the repo already has strong charter-generation components, but they terminate in the wrong public shape and the wrong authority boundary
- if this wedge lands under `setup`, later `author` commands will inherit taxonomy debt immediately

### Step 0. Scope Challenge

Exact user outcomes:

- on a scaffolded repo, `system setup` succeeds and ends with:
  - `NEXT SAFE ACTION: run \`system author charter\``
- the operator can run `system author charter` and complete charter authoring without opening a blank markdown file
  - `system author charter` is an in-process terminal interview
  - it asks one question at a time
  - it runs only on an interactive TTY
  - if the command is run without an interactive TTY, it refuses and points to `system author charter --from-inputs <path|->`
  - raw human answers are not written directly into the charter
  - each answer is normalized into structured charter-input fields
  - the interview loop validates completeness and asks follow-up questions only when required fields remain vague or missing
- the operator can run `system author charter --from-inputs <path|->` when they already have structured charter inputs
- both flows write canonical `.system/charter/CHARTER.md`
- both flows converge on one final LLM synthesis step after a complete structured input set exists
- `M7` uses one LLM synthesis pass at the end, not one LLM call after every answer
- `M7` only replaces the shipped charter starter template
  - if `.system/charter/CHARTER.md` already contains non-starter canonical truth, both authoring paths refuse
  - there is no explicit update or revise mode in `M7`
- after successful authoring:
  - `system doctor` is no longer blocked by the charter starter template
  - `system generate` no longer refuses on the charter starter template
- if authoring still cannot proceed because `.system/` is missing or invalid, the refusal points back to the `setup` family

Premise lock:

- `3A` is settled, `author` is a top-level family
- both visible charter paths ship in the first slice
- readiness/refusal/blocker cutover happens in the same milestone
- `.system/*` remains the only canonical project-truth surface
- `artifacts/*` remain derived
- `author` reuses existing compiler-owned charter assets where possible
- public product copy says `author`, not `stage.05_charter_synthesize`
- `system author charter` is a single-command, line-oriented TTY interview loop in `M7`, not a multi-command wizard and not an external capture flow
- `system author charter --from-inputs <path|->` is the only non-interactive authoring entrypoint in `M7`
- both paths share one synthesis engine and one final LLM generation pass
- no per-answer LLM synthesis calls are allowed in the interview loop
- the LLM integration layer for `M7` should use the published [`unified-agent-api`](https://crates.io/crates/unified-agent-api) crate, currently `0.2.2` as of 2026-04-19
- `M7` has no update mode for non-scaffolded existing charter truth; refusal is the correct bounded behavior
- `M7` persists no derived mirrors for charter authoring; it writes only `.system/charter/CHARTER.md`
- later authoring surfaces are explicitly enabled by the namespace, but not shipped here

What existing code already solves each sub-problem:

- canonical setup and readiness posture already exist in:
  - [crates/compiler/src/setup.rs](crates/compiler/src/setup.rs)
  - [crates/compiler/src/resolver.rs](crates/compiler/src/resolver.rs)
- charter-generation directives and templates already exist in:
  - [core/library/charter/](core/library/charter/)
- deterministic input and synthesize stages already exist in:
  - [core/stages/04_charter_inputs.md](core/stages/04_charter_inputs.md)
  - [core/stages/05_charter_synthesize.md](core/stages/05_charter_synthesize.md)
- interview-stage content already exists in:
  - [core/stages/05_charter_interview.md](core/stages/05_charter_interview.md)
- safe writer and follow-up sequencing patterns already exist in:
  - [crates/compiler/src/pipeline_capture.rs](crates/compiler/src/pipeline_capture.rs)
- docs/help/contract drift rails already exist in:
  - [crates/cli/tests/help_drift_guard.rs](crates/cli/tests/help_drift_guard.rs)
  - [crates/cli/tests/cli_surface.rs](crates/cli/tests/cli_surface.rs)

Minimum change set:

1. add an `author` family to the supported CLI surface
2. add a compiler-owned authoring module that executes the charter paths
3. make canonical `.system/charter/CHARTER.md` the primary write target
4. add one repo-owned charter authoring method artifact
5. cut `setup`, `doctor`, and `generate` over to the new next-safe-action/refusal surface
6. land docs, contracts, help snapshots, and tests together

Frozen implementation decisions for `M7`:

1. `system author charter` is an interactive TTY-only interview command that completes in one invocation.
2. `system author charter --from-inputs <path|->` is the deterministic, non-interactive path.
3. The interview loop accumulates and validates structured charter-input fields; it does not write freeform answers directly into the charter.
4. Both paths converge on one final LLM synthesis pass over the completed structured inputs.
5. The synthesis engine should be wired through `unified-agent-api`, not a one-off bespoke provider binding in the first wedge.
6. Both paths only replace scaffolded starter-template charter truth.
7. Both paths refuse when non-scaffolded canonical charter truth already exists.
8. The command persists only `.system/charter/CHARTER.md`; no `artifacts/charter/CHARTER.md` or repo-root `CHARTER.md` mirror is written in `M7`.

Complexity check:

- this milestone will touch more than 8 files because command surface, docs, contracts, and tests all need parity work
- that file count is acceptable only if the implementation stays structurally small:
  - one compiler-owned authoring module
  - thin CLI wiring
  - no second authoring engine
  - no new persistent state machine
- if implementation grows into multiple new service layers or a second generic writer framework, stop and reduce scope

TODOS cross-reference:

- the existing TODO for post-setup onboarding upgrade is absorbed by this milestone at the charter wedge only
- richer follow-on authoring for project context and later docs remains deferred
- no new TODO should be added unless this milestone intentionally punts a concrete boundary that is discovered during implementation

Completeness check:

- the complete version includes:
  - top-level `author`
  - both visible charter paths
  - canonical `.system` write path
  - method artifact
  - setup/doctor/generate cutover
  - docs/help/contracts/tests parity
- the shortcut version would land only a helper command, or only one path, or only docs cutover
- reject the shortcut; this is a boilable lake

Distribution check:

- this milestone does not introduce a new artifact type
- no new publish pipeline is required
- existing workspace/test rails are sufficient

Implementation alternatives:

| Approach | Effort | Risk | Why it is or is not the `M7` choice |
| --- | --- | --- | --- |
| Put charter authoring under `setup` | S | High | Rejected. It bakes later namespace debt into the first slice. |
| Make `doctor` own authoring | S | High | Rejected. It overloads diagnosis with content generation. |
| Ship top-level `author`, charter first | M | Medium | Recommended. It matches the lifecycle cleanly and leaves room for later authoring surfaces. |
| Ship deterministic-input only and defer interview path | S | High | Rejected. The approved wedge explicitly ships both visible charter paths together. |

### What Already Exists

- the current CLI already has a thin dispatch boundary in [crates/cli/src/main.rs](crates/cli/src/main.rs)
- the current canonical truth contract already says only `.system/*` is authoritative in [docs/contracts/C-03-canonical-artifact-manifest-contract.md](docs/contracts/C-03-canonical-artifact-manifest-contract.md)
- `setup` already routes scaffolded repos to filling `.system/charter/CHARTER.md` in [crates/compiler/src/setup.rs](crates/compiler/src/setup.rs)
- existing charter-generation stage assets already provide:
  - one-question-at-a-time interview discipline
  - deterministic structured inputs generation
  - synthesize-from-inputs generation
  - reusable charter template
- existing capture logic already knows how to:
  - validate bounded inputs
  - materialize files safely
  - compute post-write next actions
- what does not exist yet is:
  - a public `author` family
  - a canonical `.system`-first charter write path for the public authoring wedge
  - one repo-owned method artifact for charter authoring
  - readiness cutover from `fill canonical artifact ...` to `author`

### Architecture Review

Architecture ASCII diagram:

```text
operator
  │
  ├── system setup
  │     └── scaffolded repo
  │            └── NEXT SAFE ACTION: system author charter
  │
  └── system author charter
         │
         ├── interview path (TTY-only)
         │     ├── collect 1 answer at a time
         │     ├── normalize into structured charter inputs
         │     └── validate / ask follow-up only when needed
         │
         ├── deterministic path
         │     └── load complete structured charter inputs
         │
         ├── shared charter method artifact
         │
         ├── one shared synthesis engine
         │     └── one final LLM synthesis pass via `unified-agent-api`
         │
         ├── scaffold check
         │     ├── starter template -> continue
         │     └── existing real charter -> REFUSED
         │
         └── only persisted write
               └── .system/charter/CHARTER.md   [canonical truth]

steady-state commands
  doctor / generate
         │
         └── read canonical .system/* truth only
```

Opinionated architecture decisions:

- add `author` as a top-level family, not as a `setup` subcommand
- keep author execution compiler-owned, not CLI-owned
- treat current stage/directive assets as implementation ingredients, not public UX
- make `.system/charter/CHARTER.md` the primary and required output of the public authoring wedge
- make the interview path TTY-only in `M7`; refuse non-interactive use instead of inventing a second UX shape
- make the interview path a structured-input collector, not a direct document-writing flow
- run exactly one final LLM synthesis pass after structured input collection completes
- share that synthesis engine with `system author charter --from-inputs`
- use `unified-agent-api` as the LLM integration layer in the first wedge
- refuse on non-scaffolded existing canonical charter truth in `M7`; do not widen scope with revise/update semantics yet
- persist no derived charter mirrors in `M7`; this milestone should have one output authority and one persisted write target
- publish one shared charter method artifact rather than scattering the method across docs and hidden prompts

Concrete module boundaries:

- compiler ownership:
  - authoring execution
  - input validation
  - structured interview-state accumulation
  - final synthesis request construction
  - canonical write plan
  - readiness-cutover helper semantics
- CLI ownership:
  - argument parsing
  - command routing
  - refusal rendering
  - help text
- LLM provider ownership:
  - `unified-agent-api` facade wiring
  - one bounded charter-synthesis call path shared by interview and deterministic modes
- docs/contracts ownership:
  - surface vocabulary
  - canonical-vs-derived truth story
  - examples and operator journey

Recommended method artifact shape:

- add one scoped artifact under `core/library/authoring/`:
  - `core/library/authoring/charter_authoring_method.md`
- that file becomes the discoverable repo-owned method source for:
  - interview flow expectations
  - deterministic-input flow expectations
  - output quality bar
  - assumptions and red lines
- docs may point to it, but should not duplicate its content as a competing authority

ASCII comments that should land with implementation:

- the new compiler authoring module should carry a short command-path diagram
- the synthesis entrypoint should carry a short comment stating that both authoring modes converge on one final LLM call
- the scaffold-check boundary should carry a short comment explaining why `M7` refuses non-starter existing charters

### Code Quality Review

Minimum-diff posture:

- reuse current charter directives/templates instead of re-authoring the whole method from scratch
- reuse current safe write patterns instead of inventing a second mutation framework
- keep the CLI thin
- keep authoring-specific logic out of generic setup/recovery modules unless it is truly shared
- do not force the old `pipeline.foundation_inputs` dev/test wording to become the public product copy

Expected touch surfaces:

| Area | Expected modules |
| --- | --- |
| CLI surface | `crates/cli/src/`, help snapshots, CLI tests |
| Compiler authoring | `crates/compiler/src/` |
| Charter method asset | `core/library/authoring/`, possibly `core/library/charter/` |
| Docs/contracts | `README.md`, `docs/`, `docs/contracts/`, `PLAN.md` |

Overbuild traps to reject:

- a generic multi-artifact authoring framework in the first wedge
- a second stateful authoring engine parallel to the current compiler assets
- a public UX that exposes pipeline-stage ids or dev/test terminology directly
- auto-healing or silent overwrites of non-scaffolded canonical charter truth
- any persisted `artifacts/*` or repo-root charter mirror in `M7`
- one LLM call per interview answer instead of one final synthesis pass
- a bespoke direct provider integration when `unified-agent-api` already solves the facade layer

### Test Review

```text
AUTHORING WEDGE COVERAGE
===========================
[+] system author charter
    │
    ├── [GAP] happy path from scaffolded repo writes `.system/charter/CHARTER.md`
    ├── [GAP] TTY interview path refuses cleanly when `.system/` root is missing or invalid
    ├── [GAP] TTY interview path refuses when run non-interactively
    ├── [GAP] interview answers are normalized into structured inputs before synthesis
    ├── [GAP] non-scaffolded existing charter is refused, not overwritten
    └── [GAP] public help surfaces show `author` consistently

[+] deterministic path
    │
    ├── [GAP] `system author charter --from-inputs <path|->` writes the same canonical output
    ├── [GAP] malformed or empty structured inputs refuse
    ├── [GAP] deterministic path does not leak dev/test stage wording into UX
    └── [GAP] deterministic and interview modes share one synthesis engine

[+] readiness cutover
    │
    ├── [GAP] scaffolded `setup` next-safe-action points to `system author charter`
    ├── [GAP] blocked `doctor` points to `system author charter`
    ├── [GAP] blocked `generate` points to `system author charter`
    └── [GAP] successful authoring clears the charter starter-template blocker

[+] authority boundary
    │
    ├── [GAP] canonical `.system/charter/CHARTER.md` is the readiness-driving output
    └── [GAP] no persisted derived mirror is written in `M7`

─────────────────────────────────
COVERAGE: 0/13 gaps closed in plan text alone
  Public authoring surface: 0/6
  Deterministic path: 0/4
  Readiness cutover: 0/4
  Authority boundary: 0/2
─────────────────────────────────
```

Required test artifacts:

1. one scaffolded-repo CLI integration for `system author charter`
2. one deterministic-input CLI integration for `system author charter --from-inputs`
3. one non-interactive refusal test for `system author charter`
4. one interview normalization test proving freeform answers become structured charter-input state before synthesis
5. one shared-synthesis-engine test proving interview and deterministic modes call the same final synthesis path
6. one malformed-input refusal test
7. one invalid-root refusal test
8. one refusal test for existing non-scaffolded canonical charter truth
9. one regression test proving setup/doctor/generate cut over to `author`
10. one regression test proving canonical `.system/charter/CHARTER.md` drives readiness and no persisted mirror is used
11. help snapshot and help-drift updates for the sixth top-level surface

Required assertions:

- canonical `.system/charter/CHARTER.md` is written on success
- starter-template blockers clear only when canonical truth is replaced
- deterministic and interview paths both land on the same canonical authority boundary
- `system author charter` refuses when not running on an interactive TTY
- raw interview answers are not written directly into the final charter
- only one final LLM synthesis call is made after structured input collection completes
- non-scaffolded existing canonical charter truth is refused, not overwritten
- no public docs/help output still teaches manual “fill canonical artifact ...” as the normal next step after setup

### Performance Review

Performance and determinism rules:

- authoring execution must stay bounded to the charter wedge and declared assets, not full-repo rescans
- deterministic-input mode must remain stable across reruns for the same input file
- help/docs drift rails must remain exact because this milestone changes the supported command surface
- no network access or live model dependency belongs in automated proof for this wedge
- the interview loop must remain line-oriented and terminal-local; do not invent background sessions or persisted prompt state
- exactly one synthesis request should be issued per completed authoring run
- the LLM facade layer should stay inside `unified-agent-api`; do not duplicate provider plumbing in `system`

Performance smells that fail review:

- duplicate compile/hash/render work across interview and deterministic paths
- separate canonical and derived write pipelines with divergent validation rules
- product copy parity relying on manual review instead of existing help-drift rails

### Failure Modes Registry

| Failure mode | Severity | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- | --- |
| `system author charter` writes anything other than `.system/charter/CHARTER.md` as persisted output in `M7` | Critical | yes | hard failure in tests | split authority boundary |
| setup/doctor/generate still point to manual file editing after `author` ships | Critical | yes | hard failure in tests | split product story |
| `system author charter` tries to run interview mode non-interactively | High | yes | explicit refusal | clear direction to `--from-inputs` |
| interview answers are streamed directly into the charter body without structured normalization | Critical | yes | hard failure in tests | invalid charter content path |
| interview mode and deterministic mode drift into separate synthesis implementations | High | yes | shared-engine regression test | two inconsistent authoring systems |
| deterministic-input mode accepts empty or malformed structured inputs | High | yes | explicit refusal | clear next safe action |
| public UX leaks `stage.04` / `stage.05` or dev/test terminology | High | yes | docs/help drift failure | exact product copy stays stable |
| authoring overwrites non-scaffolded charter truth silently | Critical | yes | explicit refusal | no silent data loss |
| future contributors reintroduce a persisted derived mirror because tests only check canonical output | High | yes | authority-boundary regression tests | canonical truth remains exact |

Critical gaps:

- any path that has no test, no refusal, and no operator-visible error is a release blocker
- any implementation that leaves `.system/charter/CHARTER.md` out of the primary happy path is a release blocker

### Required Deliverables

1. one supported top-level `author` family in the CLI
2. one shipped `system author charter` surface
3. one shipped deterministic-input charter path under that same family
4. one compiler-owned canonical charter write path to `.system/charter/CHARTER.md`
5. one explicit TTY-only interview contract for `system author charter`
6. one explicit structured-input interview pipeline that ends in one final LLM synthesis pass
7. one `unified-agent-api`-backed synthesis integration for the charter wedge
8. one repo-owned charter authoring method artifact
9. setup/doctor/generate cutover to the new authoring next-safe-action/refusal path
10. docs/help/contracts/snapshots/test parity for the new command family

### Deferred Work

- `system author project-context`
- `system author feature-spec`
- generic authoring orchestration or batching
- richer onboarding/chat-style guided flows
- UI/MCP companion work
- public distribution changes

### NOT In Scope

- changing the canonical `.system/*` contract
- widening the shipped wedge beyond charter authoring
- exposing pipeline internals as the public authoring surface
- building a generic multi-document authoring framework
- making `doctor` or `setup` absorb authoring as a hidden sub-lifecycle
- revise/update semantics for non-scaffolded existing charter truth
- persisted `artifacts/*` or repo-root charter mirrors

### Worktree Parallelization Strategy

Dependency table:

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Lock CLI/docs/contracts surface for `author` and canonical boundary | `docs/`, `docs/contracts/`, `PLAN.md` | — |
| B. Add compiler-owned authoring execution and canonical charter write path | `crates/compiler/` | A |
| C. Add CLI `author` family and help/snapshot updates | `crates/cli/`, CLI snapshots | A, B |
| D. Add method artifact and deterministic-input path integration | `core/library/`, `crates/compiler/`, optional CLI docs | A, B |
| E. Add setup/doctor/generate cutover tests and authority-boundary regressions | `crates/cli/tests/`, optional compiler tests | B, C, D |

Parallel lanes:

- Lane A: A
- Lane B: B
- Lane C: C
- Lane D: D
- Lane E: E

Execution order:

- launch Lane A first and merge it before code starts
- launch Lane B next
- once Lane B freezes the canonical write path, launch Lane C and Lane D in parallel if ownership is split cleanly
- finish with Lane E once the CLI surface and method artifacts are stable

Conflict flags:

- Lanes B and D both touch compiler authoring assets, so parallel work is safe only if the ownership split is explicit
- Lane E touches CLI tests that depend on final wording, so do not start it before C and D settle

### Exit Criteria

- scaffolded repos route cleanly from `setup` into `system author charter`
- both visible charter paths exist and land on the same canonical output
- `system author charter` has one exact interaction shape: TTY-only interview or explicit `--from-inputs`
- interview mode normalizes answers into structured inputs and then runs one final LLM synthesis pass
- deterministic and interview modes share the same synthesis engine
- non-scaffolded existing charter truth refuses rather than silently broadening scope
- canonical `.system/charter/CHARTER.md` is the readiness-driving output
- no persisted derived charter mirror exists in `M7`
- setup/doctor/generate no longer teach manual blank-file editing as the normal next step
- one repo-owned authoring method artifact exists and is referenced consistently
- docs, contracts, help text, snapshots, and tests all agree on the same command family and authority boundary

### Completion Summary

- Step 0: Scope Challenge, scope accepted as-is and locked to the charter authoring wedge
- Architecture Review: 0 open architecture questions, 13 boundary decisions locked
- Code Quality Review: 4 structural guardrails locked, no second authoring engine admitted
- Test Review: diagram produced, 13 required coverage gaps enumerated
- Performance Review: 5 determinism and bounded-work rules locked
- NOT in scope: written
- What already exists: written
- TODOS.md updates: none required in the plan rewrite
- Failure modes: 5 release-blocking critical boundary classes identified
- Outside voice: approved design doc and eng-review decisions already incorporated
- Parallelization: 5 lanes total, 2 lanes parallelizable after boundary lock, 3 sequential gates
- Lake Score: complete option preserved across surface, authority, and cutover work
