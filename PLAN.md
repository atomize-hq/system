# PLAN

## Status

This is the active execution source of truth for the next `system` wedge.

It supersedes the earlier scaffold-to-ready planning pass and folds the approved design work plus the current eng-review conclusions into one implementation-ready plan.

Archived predecessor:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-plan-archive-20260419-120831.md`

Primary design basis:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-design-20260419-103432.md`

This milestone starts from the current shipped state on `main`:

- `system setup` is the front door
- `system doctor` is the recovery/readiness surface
- `system generate` is the ready-path packet surface
- canonical project truth lives under `.system/*`
- setup-owned starter templates do not count as ready truth

## Active Objective

Ship the first real canonical authoring wedge after `M6`.

That means:

- add a top-level `author` family to the public CLI
- ship one real surface under it, `system author charter`
- ship both visible charter paths in the first slice:
  - interactive interview path
  - deterministic input path via `--from-inputs <path|->`
- write canonical truth only to `.system/charter/CHARTER.md`
- cut `setup`, `doctor`, and `generate` over to the new authoring surface
- publish one repo-owned charter authoring method artifact for humans and agents

This repo remains a compiler/generator product, not a chat runtime. The point of the wedge is to remove the blank-file moment after setup with one boring, exact, reproducible authoring path.

## Exact Shipped Behavior

The milestone is only done when all of the following are true:

1. On a scaffolded repo, `system setup` succeeds and ends with `NEXT SAFE ACTION: run \`system author charter\`` instead of `fill canonical artifact ...`.
2. `system author charter` runs as a line-oriented, TTY-only interview in one invocation.
3. If `system author charter` is invoked without an interactive TTY, it refuses and points to `system author charter --from-inputs <path|->`.
4. Interview answers are normalized into structured charter-input state before synthesis. Raw answers are not copied directly into the final charter body.
5. The interview loop asks follow-up questions only when a required field is still empty or unusably vague after normalization.
6. `system author charter --from-inputs <path|->` accepts a complete structured charter-input document from a file or stdin (`-`).
7. There are no per-answer LLM synthesis calls during the interview loop.
8. Both paths converge on one shared final synthesis engine and one final LLM synthesis pass.
9. The shared LLM integration layer for `M7` uses the published [`unified-agent-api`](https://crates.io/crates/unified-agent-api) crate, version `0.2.2` as of 2026-04-19.
10. The human-guided surface is `system author charter`; the agent and automation surface is `system author charter --from-inputs <path|->`.
11. The only persisted authored output in `M7` is `.system/charter/CHARTER.md`.
12. If `.system/charter/CHARTER.md` already contains non-starter canonical truth, both authoring paths refuse. `M7` does not ship update/revise mode.
13. Successful charter authoring clears the starter-template blocker for `doctor` and `generate`.
14. Public product copy says `author`, not legacy stage ids or dev/test mechanism names.

## Scope

### In scope

- one top-level `author` family in the CLI
- one shipped command family member: `system author charter`
- one shipped deterministic variant: `system author charter --from-inputs <path|->`
- compiler-owned charter authoring execution
- canonical `.system/charter/CHARTER.md` write path
- reuse of the existing charter interview, input, synthesize, directive, and template assets where they still fit
- one discoverable repo-owned charter authoring method artifact
- setup/doctor/generate cutover to `author`
- docs, contracts, help text, snapshots, and tests for the new surface

### NOT in scope

- `system author project-context`
- `system author feature-spec`
- generic multi-artifact authoring orchestration
- chat-style onboarding automation
- MCP or web/UI companion work
- public distribution changes
- moving canonical truth out of `.system/*`
- persisted `artifacts/*` or repo-root charter mirrors for the authoring surface
- update/revise semantics for non-scaffolded existing charter truth

## Frozen Decisions

These decisions are already settled for `M7`:

1. `author` is a top-level family, not a hidden `setup` or `doctor` extension.
2. `system author charter` is the first shipped slice.
3. The first slice includes both visible charter paths together.
4. The interview path is TTY-only in `M7`.
5. The deterministic path is `--from-inputs <path|->` and is the only non-interactive entrypoint.
6. Both paths normalize to one structured input model and one shared synthesis engine.
7. The interview loop may ask follow-up questions only when a required field remains empty or unusably vague after normalization.
8. There are no per-answer LLM synthesis calls in the interview loop.
9. The final charter is produced by one final LLM synthesis pass after the structured input set is complete.
10. The LLM integration layer for `M7` is the published [`unified-agent-api`](https://crates.io/crates/unified-agent-api) crate, version `0.2.2` as of 2026-04-19, not a bespoke one-off provider binding.
11. `system author charter` is the human-guided surface; `system author charter --from-inputs <path|->` is the agent and automation surface.
12. `M7` reuses existing compiler-owned charter assets where possible, but does not expose stage ids as the public UX.
13. The public authoring wedge writes only `.system/charter/CHARTER.md`.
14. If real charter truth already exists, `M7` refuses instead of widening scope.
15. Setup/doctor/generate cutover happens in the same milestone as the new authoring surface.
16. One repo-owned authoring method artifact ships in the same milestone as the CLI surface.

## Step 0: Scope Challenge

### What already exists

The repo already solves large parts of the problem. The plan must reuse those pieces instead of building parallel systems.

| Sub-problem | Existing code or asset | Reuse decision |
| --- | --- | --- |
| CLI dispatch and top-level command wiring | [crates/cli/src/main.rs](crates/cli/src/main.rs) | Extend, do not replace |
| canonical truth model and starter-template handling | [crates/compiler/src/setup.rs](crates/compiler/src/setup.rs), [crates/compiler/src/resolver.rs](crates/compiler/src/resolver.rs) | Reuse |
| charter interview content | [core/stages/05_charter_interview.md](core/stages/05_charter_interview.md) | Reuse as implementation evidence, not public UX |
| deterministic charter inputs flow | [core/stages/04_charter_inputs.md](core/stages/04_charter_inputs.md), [core/library/charter/charter_inputs_directive.md](core/library/charter/charter_inputs_directive.md), [core/library/charter/CHARTER_INPUTS.yaml.tmpl](core/library/charter/CHARTER_INPUTS.yaml.tmpl) | Reuse schema and prompt ingredients |
| final charter synthesis ingredients | [core/stages/05_charter_synthesize.md](core/stages/05_charter_synthesize.md), [core/library/charter/charter_gen_directive.md](core/library/charter/charter_gen_directive.md), [core/library/charter/charter.md.tmpl](core/library/charter/charter.md.tmpl) | Reuse |
| safe file-write and mutation patterns | [crates/compiler/src/pipeline_capture.rs](crates/compiler/src/pipeline_capture.rs), repo-file access helpers | Reuse patterns, not artifact authority |
| help/doc drift guards | [crates/cli/tests/help_drift_guard.rs](crates/cli/tests/help_drift_guard.rs), [crates/cli/tests/cli_surface.rs](crates/cli/tests/cli_surface.rs) | Reuse and extend |

### Existing pain points the milestone must resolve

- `setup` still ends the scaffolded path with `fill canonical artifact ...` in [crates/compiler/src/setup.rs](crates/compiler/src/setup.rs).
- the public CLI still describes a five-surface product in [crates/cli/src/main.rs](crates/cli/src/main.rs).
- legacy charter stages still target `artifacts/charter/CHARTER.md` and `${repo_root}/CHARTER.md`, which is the wrong authority boundary for the public authoring surface.
- tests and docs still encode the current manual handoff and five-surface vocabulary.

### Minimum change set

The minimum complete version of `M7` is:

1. add `author` to the public CLI surface
2. add one compiler-owned authoring module for charter execution
3. add one shared structured-input model plus one shared synthesis entrypoint
4. add one canonical charter write path to `.system/charter/CHARTER.md`
5. add one repo-owned charter authoring method artifact
6. cut setup/doctor/generate/docs/help/tests over to the new surface

Anything smaller is a shortcut. Examples of rejected shortcuts:

- add only an interview wrapper command
- add only `--from-inputs` and defer interview
- keep the command under `setup`
- keep manual `fill canonical artifact ...` as the normal post-setup instruction
- keep writing derived mirrors in the public authoring flow

### Complexity check

This milestone will touch more than eight files because the surface change spans CLI wiring, compiler behavior, docs, contracts, and tests. That is acceptable only if the implementation stays structurally small:

- one new authoring module in the compiler
- thin CLI wiring
- one shared synthesis engine
- no new persistent state machine
- no generic authoring framework

If implementation starts growing into multiple new services, generic workflow abstractions, or a second writer engine, stop and reduce scope back to the charter wedge.

### TODO cross-reference

No new TODO is required to make this milestone coherent. Existing TODOs for wider onboarding, UI, public distribution, and later authoring surfaces stay deferred. `M7` absorbs the already-identified post-setup authoring gap at the charter wedge only.

### Completeness and distribution check

- This is a boilable lake. The complete version is the right one.
- The milestone does not introduce a new artifact type, so no new build/publish pipeline is required.

## Architecture Review

### Primary architecture decision

Ship a first-class public authoring family while keeping authoring execution compiler-owned.

That means:

- CLI owns argument parsing, help text, and refusal rendering
- compiler owns authoring semantics, validation, synthesis request construction, and canonical writes
- legacy prompt/template assets are inputs to the compiler-owned authoring flow, not the public interface

### Architecture diagram

```text
operator
  │
  ├── system setup
  │     └── scaffolded repo
  │            └── NEXT SAFE ACTION: run `system author charter`
  │
  └── system author charter
         │
         ├── interactive path (TTY only)
         │     ├── ask one question at a time
         │     ├── normalize answers into structured charter inputs
         │     └── loop until required fields are complete
         │
         ├── deterministic path
         │     └── load full structured charter inputs from file or stdin
         │
         ├── shared charter authoring method + directives + template
         │
         ├── shared synthesis engine
         │     └── one final LLM call
         │
         ├── scaffold/authority check
         │     ├── starter template => continue
         │     └── real charter truth => refuse
         │
         └── canonical write
               └── .system/charter/CHARTER.md

steady-state consumers
  doctor / generate
         │
         └── read canonical `.system/*` truth only
```

### Module boundaries

| Area | Ownership | Planned work |
| --- | --- | --- |
| `crates/cli/src/` | CLI | add `author` family, route `author charter`, render help/refusals |
| `crates/compiler/src/` | compiler | add charter authoring module and shared synthesis path |
| `core/library/charter/` | authoring inputs | reuse directives/templates and input schema ingredients |
| `core/library/authoring/` | method artifact | add one discoverable charter authoring method file |
| `docs/` + `docs/contracts/` | product surface | cut vocabulary and operator journey over to `author` |
| `crates/cli/tests/` + `crates/compiler/tests/` | proof | add happy-path, refusal, cutover, and authority-boundary regression tests |

### Concrete implementation shape

Planned new or changed seams:

- add `Command::Author(AuthorArgs)` beside the current top-level command enum in [crates/cli/src/main.rs](crates/cli/src/main.rs)
- add a compiler-owned authoring entrypoint, expected as `crates/compiler/src/author.rs`
- keep the structured input model charter-specific in `M7`; do not invent a generic document-authoring schema
- wire the shared synthesis engine through the published [`unified-agent-api`](https://crates.io/crates/unified-agent-api) crate, version `0.2.2` as of 2026-04-19
- use the existing canonical-artifact and repo-root safety helpers for validation and writes
- add a new method artifact at `core/library/authoring/charter_authoring_method.md`

### Data-flow contract

#### Interactive path

```text
TTY interview
  -> answer normalization
  -> follow-up only if a required field remains empty or unusably vague
  -> structured charter inputs
  -> completeness validation
  -> shared synthesis request
  -> canonical `.system/charter/CHARTER.md` write
```

#### Deterministic path

```text
file/stdin structured inputs
  -> parse + validate
  -> shared synthesis request
  -> canonical `.system/charter/CHARTER.md` write
```

The two paths must diverge only at input capture. They must converge before synthesis.
The human path is the TTY interview. The agent and automation path is `--from-inputs`. Agents should not drive the TTY loop as the primary integration surface.

### Security and production-failure review

| New codepath | Realistic production failure | Plan response |
| --- | --- | --- |
| interactive interview | command invoked in CI or piped shell without a TTY | explicit refusal to `--from-inputs` |
| deterministic input load | malformed or incomplete YAML from agent automation | parse/validation refusal with exact next step |
| interview normalization | required field stays vague and the loop does not ask a clarifying follow-up | normalize, detect insufficiency, ask one more bounded question |
| synthesis step | provider call fails or returns empty/invalid content | hard failure, no partial write |
| canonical write | repo path invalid or blocked | mutation refusal, no fallback mirror |
| cutover in setup/doctor/generate | help/docs/tests drift and product story splits | drift-guard and CLI regression coverage |

## Code Quality Review

### Minimal-diff rules

- keep the CLI thin
- keep authoring semantics in the compiler, not scattered across CLI renderers
- reuse current charter directives/templates instead of re-authoring the method in multiple places
- keep one structured input model and one synthesis path
- keep one canonical persisted output
- keep the product-language split explicit: human-guided `author charter`, agent/automation `author charter --from-inputs`

### DRY guardrails

Aggressive no-duplication rules for `M7`:

1. do not build separate interview-synthesis and deterministic-synthesis implementations
2. do not duplicate the charter quality bar across docs, prompts, and method artifacts
3. do not create a second write pipeline for authoring when the compiler already has path-safety primitives
4. do not copy legacy stage copy into public help text

### Overbuild traps that fail review

- a generic authoring engine for future documents
- a second persistence model or saved authoring session state
- a public UX that mentions `stage.04_*` or `stage.05_*`
- per-answer LLM calls during the interview loop
- silent overwrite of non-scaffolded charter truth
- public authoring writes to `artifacts/charter/CHARTER.md` or repo-root `CHARTER.md`

### Files that need diagram or boundary comments during implementation

- the new compiler authoring module should include a short command-path diagram comment
- the shared synthesis entrypoint should explicitly document that both modes converge here
- the scaffold/refusal boundary should carry a short comment explaining why `M7` refuses existing real charters

## Test Review

### Coverage diagram

```text
AUTHORING WEDGE COVERAGE
===========================
[+] system author charter
    │
    ├── [GAP] interactive happy path from scaffolded repo writes `.system/charter/CHARTER.md`
    ├── [GAP] refuses cleanly when `.system/` root is missing or invalid
    ├── [GAP] refuses cleanly when invoked without an interactive TTY
    ├── [GAP] interview answers normalize into structured inputs before synthesis
    ├── [GAP] follow-up questions only happen when a required field remains empty or unusably vague
    ├── [GAP] existing non-starter charter truth is refused, not overwritten
    └── [GAP] public help surfaces show `author` consistently

[+] system author charter --from-inputs <path|->
    │
    ├── [GAP] valid structured inputs from file write the same canonical output
    ├── [GAP] valid structured inputs from stdin write the same canonical output
    ├── [GAP] malformed or incomplete inputs refuse with exact next action
    └── [GAP] deterministic path shares the same synthesis engine as interview mode

[+] setup / doctor / generate cutover
    │
    ├── [GAP] scaffolded setup points to `system author charter`
    ├── [GAP] blocked doctor points to `system author charter`
    ├── [GAP] blocked generate points to `system author charter`
    └── [GAP] successful authoring clears the charter starter-template blocker

[+] authority boundary
    │
    ├── [GAP] `.system/charter/CHARTER.md` is the only readiness-driving authored output
    └── [GAP] no persisted derived mirror is written by the authoring surface

─────────────────────────────────
COVERAGE: 0/16 paths implemented yet
  Author surface: 0/6
  Deterministic path: 0/4
  Cutover: 0/4
  Authority boundary: 0/2
─────────────────────────────────
```

### Required tests and expected homes

| Test requirement | Expected file home | Type |
| --- | --- | --- |
| interactive happy path writes canonical charter | `crates/cli/tests/author_cli.rs` | CLI integration |
| non-interactive refusal points to `--from-inputs` | `crates/cli/tests/author_cli.rs` | CLI integration |
| invalid root refusal for authoring path | `crates/cli/tests/author_cli.rs` or `crates/compiler/tests/author.rs` | integration/unit |
| interview normalization happens before synthesis | `crates/compiler/tests/author.rs` | unit/integration |
| follow-up is asked only for empty or unusably vague required fields | `crates/compiler/tests/author.rs` | unit/integration |
| deterministic file input path | `crates/cli/tests/author_cli.rs` | CLI integration |
| deterministic stdin path | `crates/cli/tests/author_cli.rs` | CLI integration |
| malformed/incomplete input refusal | `crates/compiler/tests/author.rs` | unit |
| shared synthesis engine across both paths | `crates/compiler/tests/author.rs` | unit |
| refusal on non-starter existing charter | `crates/compiler/tests/author.rs` | unit/integration |
| setup next-safe-action cutover | `crates/compiler/tests/setup.rs` | regression |
| doctor/generate blocker cutover | `crates/compiler/tests/refusal_mapping.rs`, `crates/compiler/tests/resolver_core.rs`, or matching CLI regression tests | regression |
| help text and docs parity for sixth surface | `crates/cli/tests/help_drift_guard.rs`, CLI help snapshots, docs parity | regression |
| no persisted derived mirror written by public authoring | `crates/compiler/tests/author.rs` | regression |

### Mandatory regression rules

- changing the post-setup handoff is a regression-sensitive behavior, so setup cutover must ship with regression coverage
- changing the authority boundary from legacy artifact/mirror outputs to canonical `.system` truth must ship with regression coverage
- any path that risks silent overwrite of existing real charter truth is a release-blocking regression

### Test assertions that are non-negotiable

- `.system/charter/CHARTER.md` exists and contains the authored charter on success
- no public authoring command persists `artifacts/charter/CHARTER.md` or repo-root `CHARTER.md`
- interactive mode refuses without a TTY
- `--from-inputs -` reads stdin, validates, and writes the same canonical output shape
- interview mode uses structured inputs as an intermediate representation
- interview mode asks clarifying follow-ups only when required fields remain empty or unusably vague
- both modes call the same final synthesis path
- existing non-scaffolded charter truth is refused
- setup/doctor/generate no longer teach manual blank-file editing as the normal charter path

## Failure Modes Registry

| Failure mode | Severity | Test cover | Error handling | User-visible outcome |
| --- | --- | --- | --- | --- |
| authoring writes anywhere other than `.system/charter/CHARTER.md` | Critical | required | hard failure | prevents split authority |
| setup/doctor/generate still point at manual editing after `author` ships | Critical | required | hard failure | prevents split product story |
| interactive mode runs in non-interactive contexts | High | required | explicit refusal | clear direction to `--from-inputs` |
| agent integrations try to drive the TTY interview as the primary machine path | High | required | explicit product/docs boundary to `--from-inputs` | prevents brittle automation |
| interview answers are copied straight into the final charter | Critical | required | hard failure | blocks invalid synthesis path |
| interview loop keeps asking or skips clarification without a required-field trigger | High | required | bounded follow-up rule | keeps interview deterministic enough |
| interview and deterministic paths drift into separate synthesis engines | High | required | shared-engine regression test | prevents duplicate systems |
| malformed structured inputs are accepted | High | required | explicit refusal | clear recovery path |
| authoring silently overwrites real charter truth | Critical | required | explicit refusal | prevents data loss |
| docs/help leak stage ids or dev/test terminology | High | required | drift-guard failure | keeps product copy exact |

Critical-gap rule:

- any path with no test, no refusal, and a silent failure mode is a release blocker

## Performance Review

### Performance and determinism requirements

- the authoring path must stay bounded to declared authoring assets and canonical repo state, not full-repo scans
- there is exactly one synthesis request per completed authoring run
- tests must not require live network access or a real provider
- deterministic-input mode must be stable across reruns for identical inputs
- docs/help parity must stay machine-checked because this milestone changes the public command surface

### Performance smells that fail review

- duplicate normalization or synthesis work across the two paths
- separate canonical and derived write pipelines with different validation rules
- hidden provider wiring outside the chosen shared LLM facade
- public docs/help updates that rely on manual review instead of drift guards

## Implementation Plan

### Slice 1: Lock the public surface and contract

Deliverables:

- add `author` to CLI vocabulary/hierarchy/docs/contracts
- define `system author charter` and `--from-inputs <path|->`
- define the new canonical handoff language for setup/doctor/generate

Acceptance:

- all product-facing docs agree on the six-surface story
- no docs still present manual charter editing as the normal next step after setup

### Slice 2: Add compiler-owned charter authoring core

Deliverables:

- add the new compiler authoring module
- implement the structured charter-input model
- add scaffold/refusal gating for starter-template replacement only

Acceptance:

- compiler can validate inputs, refuse existing real truth, and produce one write plan for the canonical charter path

### Slice 3: Ship the interactive interview path

Deliverables:

- TTY detection
- line-oriented question loop
- answer normalization and completeness checks
- follow-up questions only for empty or unusably vague required fields
- handoff into shared synthesis

Acceptance:

- interactive path works in one invocation and refuses cleanly without a TTY
- interactive path stays a thin human-guided wrapper around the shared deterministic/synthesis engine, not a separate authoring engine

### Slice 4: Ship the deterministic input path

Deliverables:

- file/stdin loader for structured inputs
- parse + validation layer
- handoff into shared synthesis

Acceptance:

- file and stdin variants land on the same canonical output and same synthesis path
- docs and help text make it explicit that this is the agent and automation surface

### Slice 5: Add method artifact and cut over the product story

Deliverables:

- add `core/library/authoring/charter_authoring_method.md`
- update setup/doctor/generate handoffs
- update docs/help/snapshots

Acceptance:

- humans and agents have one discoverable repo-owned method artifact
- public product copy is consistent everywhere

### Slice 6: Land proof and regressions

Deliverables:

- CLI and compiler tests from the test plan
- drift-guard updates
- authority-boundary regressions

Acceptance:

- all required authoring, cutover, and authority-boundary tests pass

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Lock product vocabulary, command hierarchy, and contracts for `author` | `docs/`, `docs/contracts/`, `PLAN.md` | — |
| B. Add compiler-owned charter authoring core and canonical write path | `crates/compiler/` | A |
| C. Add CLI `author` family, renderers, and help snapshots | `crates/cli/`, CLI snapshots | A, B |
| D. Add method artifact plus prompt/template integration cleanup | `core/library/`, `crates/compiler/` | A, B |
| E. Add setup/doctor/generate cutover regressions and authority-boundary tests | `crates/compiler/tests/`, `crates/cli/tests/` | B, C, D |

### Parallel lanes

- Lane A: A
- Lane B: B
- Lane C: C
- Lane D: D
- Lane E: E

### Execution order

1. Launch Lane A first and merge it before code starts.
2. Launch Lane B once the public surface and vocabulary are locked.
3. After Lane B stabilizes the compiler-owned boundary, launch Lane C and Lane D in parallel if ownership is explicit.
4. Launch Lane E after C and D settle, because cutover tests depend on final wording and final write semantics.

### Conflict flags

- Lanes B and D both touch compiler-side authoring assets. They can run in parallel only if one owns execution code and the other owns prompt/method assets.
- Lane E depends on the final command wording and write semantics. Starting it early guarantees churn.

### Parallelization verdict

Parallel implementation is worth it after the boundary lock. The first gate is sequential. The middle phase supports two safe parallel lanes. The final proof phase is sequential.

## Exit Criteria

The milestone is complete only when:

- scaffolded repos route from `setup` to `system author charter`
- both visible charter paths exist and converge on one synthesis engine
- `system author charter` has one exact interaction contract: TTY interview or explicit `--from-inputs`
- interview mode normalizes answers before synthesis
- interview mode asks follow-ups only when required fields remain empty or unusably vague
- deterministic mode accepts file and stdin inputs
- the product language explicitly distinguishes the human-guided surface from the agent/automation surface
- non-scaffolded existing charter truth is refused
- canonical `.system/charter/CHARTER.md` is the readiness-driving output
- no persisted derived mirror is written by the public authoring surface
- setup/doctor/generate no longer teach manual blank-file editing as the normal charter path
- one repo-owned charter authoring method artifact is discoverable and referenced consistently
- docs, contracts, help text, snapshots, and tests all agree on the same six-surface product story

## Completion Summary

- Step 0: Scope Challenge, scope accepted as-is and locked to the charter authoring wedge
- Architecture Review: 0 unresolved architecture questions, public boundary locked
- Code Quality Review: 4 DRY/overbuild guardrails locked
- Test Review: coverage diagram produced, 16 required paths enumerated
- Performance Review: determinism and bounded-work rules locked
- NOT in scope: written
- What already exists: written
- TODOS.md updates: none required for this rewrite
- Failure modes: critical authority-boundary and cutover gaps explicitly tracked
- Parallelization: 5 steps, 2 safe middle parallel lanes after the initial boundary lock
- Lake Score: complete option preserved across command surface, authority boundary, and cutover scope
