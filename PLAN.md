<!-- previous reduced-v1 baseline archived at .implemented/PLAN-20260409-144209-reduced-v1-baseline.md -->
# PLAN

## Status

This is the active execution source of truth for `system`.

The old reduced-v1 packet-baseline plan is preserved as historical evidence at:

- [.implemented/PLAN-20260409-144209-reduced-v1-baseline.md](.implemented/PLAN-20260409-144209-reduced-v1-baseline.md)

That archived plan explains why the current Rust CLI looks the way it does. It is no longer the active objective.

## Active Objective

Make the Rust version recover the useful planning-generation behavior the legacy Python harness already had.

This repo remains exclusively the generator/compiler layer for a larger agent workflow stack.

The first job is not:

- build the whole agent platform
- widen into runtime orchestration
- invent a prettier future architecture before the useful behavior exists

The first job is:

- stop repeated repo research
- stop manual context shuttling
- stop inconsistent planning outputs
- stop operator babysitting loops

by making Rust do the useful planning-generation work that Python currently still does.

## Problem Statement

Today the operator can get useful planning behavior only by chaining together:

- legacy harness mechanics
- staged planning docs
- pre-planning packs
- seam extraction, decomposition, promotion, execution, and unblock skills

That works, but it forces repeated repo grounding and repeated copy/paste context movement across multiple loops.

The current Rust CLI proves part of the right long-term posture:

- trust-heavy CLI surface
- planning packet generation from canonical repo-local inputs
- `inspect` proof output
- `doctor` recovery posture

But it does not yet recover the planning-generation capabilities that actually remove the operator tax.

## Scope

### In scope

- Rust parity for the useful legacy planning-generation behavior
- Rust planning-generation from already-populated canonical project documents
- parity inventory tied to concrete proof files and commands
- the minimum end-to-end planning-generation flow that removes the most manual babysitting
- stable outputs that downstream planning and execution consumers can reuse
- preserving the trust-heavy CLI posture already defined in [DESIGN.md](DESIGN.md)

### Out of scope for this plan

- rebuilding the entire downstream seam-skill ecosystem inside this repo
- rebuilding the front-door onboarding or intake chat flow first
- new public release/distribution work
- thin MCP/UI companion work
- review/fix packet family
- live slice lineage and live execution packet generation
- broad architecture cleanup that does not reduce operator pain

Those can come back only after Rust parity proves it can replace the legacy planning path for real work.

## Operator And Wedge

Primary operator:

- the planning/orchestration operator in a complex repo, initially you

What they need:

- turn repo truth plus feature inputs into useful planning artifacts without redoing repo archaeology
- keep planning just in time so it does not rot before execution
- feed downstream seam/execution skills with smaller, more consistent grounding
- manage multiple projects without babysitting long-running loops

Current failure mode:

- inconsistent output
- brain-dead copy/paste and wait loops
- reduced parallel project capacity because the human must keep stitching context back together

## Assumed Inputs For Parity

Parity starts after the front-door onboarding work has already happened.

For this plan, assume the canonical project documents already exist and are rich enough to be useful:

- charter
- project context
- foundation/posture docs
- feature-level planning inputs

The onboarding flow that chats with an AI agent and populates those documents is a real part of the eventual product, but it is not the first implementation target in this plan.

The parity question for this phase is narrower:

- given realistic canonical inputs that already exist, can Rust generate useful planning artifacts and reusable grounding without manual context shuttling?

## Guardrails

These rules are active for every later session that touches this plan:

1. Do not widen the repo objective beyond the generator/compiler layer.
2. Do not treat the archived reduced-v1 plan as the active north star.
3. Do not delete useful historical evidence just to simplify the story.
4. Do not call parity "done" by feature count alone. Measure it by removed operator pain.
5. Do not rebuild all legacy behavior blindly. Recover only the behavior that materially removes repeated grounding and manual shuttling.
6. Do not block parity on final canonical storage debates if a compatibility path can prove the workflow sooner.
7. Keep the existing downstream planning skills as consumers of compiler outputs unless and until a separate plan changes that.
8. Do not treat the onboarding/intake flow as a prerequisite for proving planning-generation parity.
9. Do not use toy one-paragraph fixtures as parity evidence. Use realistic demo project artifacts that look like actual project truth.

## Sources Of Truth For Parity

Legacy behavior proof lives in:

- [tools/harness.py](tools/harness.py)
- [docs/legacy/HARNESS.md](docs/legacy/HARNESS.md)
- [docs/legacy/SYSTEM_MODEL.md](docs/legacy/SYSTEM_MODEL.md)
- [docs/legacy/stages/README.md](docs/legacy/stages/README.md)
- [profiles/](profiles/)
- [pipelines/](pipelines/)
- [core/stages/](core/stages/)
- [core/rules/](core/rules/)
- [core/overlays/](core/overlays/)

Current Rust baseline proof lives in:

- [README.md](README.md)
- [docs/START_HERE.md](docs/START_HERE.md)
- [DESIGN.md](DESIGN.md)
- [docs/contracts/](docs/contracts/)
- [crates/cli/src/main.rs](crates/cli/src/main.rs)
- [crates/compiler/](crates/compiler/)

Fixture and demo-corpus evidence should come from:

- realistic archived planning artifacts under [archived/legacy-generated-artifacts/](archived/legacy-generated-artifacts/)
- real-world-looking demo packs and charters that use the established templates and document shapes

If an archived artifact is too thin to prove usefulness, treat it as shape reference only, not as parity proof.

## Parity Ledger

The table below is the active parity inventory. It is intentionally behavior-first, not architecture-first.

| ID | Legacy behavior to recover | Proof | Current pain when missing in Rust | Priority | Target milestone |
| --- | --- | --- | --- | --- | --- |
| PL-01 | Pipeline selection and deterministic stage order | `tools/harness.py`, `pipelines/*.yaml`, `docs/legacy/HARNESS.md` | The operator has to remember and reconstruct the planning path manually instead of invoking one compiler-owned flow | P0 | M1 |
| PL-02 | Stage activation and post-capture routing variables such as `needs_project_context` | `tools/harness.py`, `pipelines/foundation.yaml`, `pipelines/foundation_inputs.yaml`, `docs/legacy/SYSTEM_MODEL.md` | Conditional planning paths are recreated by hand, which causes repeated repo research and inconsistent branching | P0 | M1 |
| PL-03 | Prompt compilation from stage front matter, includes, profile docs, runner docs, library inputs, and upstream artifacts | `tools/harness.py`, `core/stages/`, `core/rules/`, `runners/`, `profiles/`, `docs/legacy/HARNESS.md` | The same repo truth is reassembled over and over in manual planning sessions | P0 | M2 |
| PL-04 | Profile-aware command and convention injection | `profiles/*/profile.yaml`, `profiles/*/commands.yaml`, `profiles/*/conventions.md`, `docs/legacy/SYSTEM_MODEL.md` | Planning outputs drift because stack-specific commands and conventions are not compiler-owned | P0 | M2 |
| PL-05 | Scoped-rule filtering by work level | `tools/harness.py`, `core/rules/p0_absolute.md`, `docs/legacy/SYSTEM_MODEL.md` | Early planning gets bloated or later execution lacks the strict guidance it needs | P1 | M2 |
| PL-06 | State persistence for planning variables and re-entry | `artifacts/_harness_state.yaml` behavior in `tools/harness.py`, `docs/legacy/HARNESS.md` | Multi-step planning loops lose routing state and force the operator to restitch context and choices manually | P0 | M3 |
| PL-07 | Single-file and multi-file output capture, including exact `--- FILE:` block handling | `tools/harness.py`, `docs/legacy/HARNESS.md` | Even when prompts are correct, outputs still need manual sorting and writing | P0 | M3 |
| PL-08 | Writing artifact outputs plus canonical repo-file outputs when declared | `tools/harness.py`, `docs/legacy/SYSTEM_MODEL.md` | The compiler cannot become the real planning authority if it cannot materialize the planning artifacts and canonical docs | P0 | M3 |
| PL-09 | Minimum useful end-to-end foundation planning flow, not just isolated packet generation | `pipelines/foundation_inputs.yaml`, `pipelines/foundation.yaml`, `core/stages/00_base.md`, `core/stages/04_charter_inputs.md`, `core/stages/05_charter_interview.md`, `core/stages/05_charter_synthesize.md`, `core/stages/06_project_context_interview.md`, `core/stages/07_foundation_pack.md` | The operator still falls back to manual planning chains because Rust only covers one narrow packet wedge | P0 | M4 |
| PL-10 | Reusable compiler-owned grounding for downstream planning consumers | current downstream skill workflow plus compiled outputs from PL-01 through PL-09 | Seam extraction and execution still repeat repo grounding because Rust outputs are not yet the thing downstream workflows trust | P0 | M5 |
| PL-11 | Legacy compatibility cleanup and cutover boundaries | `README.md`, `docs/START_HERE.md`, archive plan, legacy docs | Later sessions drift back into "Python is still fine for now" instead of finishing replacement | P1 | M6 |
| PL-12 | Realistic demo corpus quality for parity proof | archived artifacts, established templates, demo project docs with substantial content | Thin one- or two-sentence fixtures can make compilation appear to work while proving nothing about usefulness | P0 | M1-M4 |

## Ordered Milestones

### M0. Control-plane reset

Status: complete

- archive the old reduced-v1 baseline plan
- make this file the only active root plan
- stop later sessions from following the wrong source of truth

### M1. Pipeline And Routing Parity

Goal:

- recover compiler ownership of planning path selection and conditional routing

Command-shape decision:

- keep the existing top-level verb surface for packet work and recovery
- add a first-class `pipeline` noun family for legacy-parity orchestration work
- `pipeline list` discovers available pipelines
- `pipeline show --id <pipeline>` shows the declared pipeline config
- `pipeline resolve --id <pipeline>` is the authoritative compute step for route selection
- `pipeline compile --id <pipeline> --stage <stage-id>` compiles one resolved stage payload
- `generate packet --id <packet>` remains the downstream packet-generation surface
- `setup` remains first-time setup and repo-wide refresh, but does **not** absorb incremental pipeline rebuild behavior

In plain English:

- `pipeline` owns orchestration
- `resolve` decides which stages are active, skipped, blocked, and next
- `compile` builds one stage payload from pipeline context plus stage schema
- `generate packet` stays separate from pipeline work
- `doctor` stays the recovery surface

This avoids two bad outcomes:

- overloading `generate` so it means both canonical rebuild work and packet generation
- overloading `setup` so it means both first-time truth establishment and everyday incremental pipeline work
- hiding route-computation logic inside `compile` instead of making it explicit and inspectable

Legacy proof surface:

- [tools/harness.py](tools/harness.py)
  - `resolve_pipeline_path(...)`
  - `load_pipeline(...)`
  - `eval_activation(...)`
  - `_update_state_after_capture(...)`
  - `cmd_compile(...)`
  - `cmd_capture(...)`
  - `cmd_run(...)`
- [pipelines/foundation.yaml](pipelines/foundation.yaml)
- [pipelines/foundation_inputs.yaml](pipelines/foundation_inputs.yaml)
- [docs/legacy/HARNESS.md](docs/legacy/HARNESS.md)
- [docs/legacy/SYSTEM_MODEL.md](docs/legacy/SYSTEM_MODEL.md)
- [core/stages/00_base.md](core/stages/00_base.md)
- [core/stages/04_charter_inputs.md](core/stages/04_charter_inputs.md)
- [core/stages/05_charter_interview.md](core/stages/05_charter_interview.md)
- [core/stages/05_charter_synthesize.md](core/stages/05_charter_synthesize.md)
- [core/stages/06_project_context_interview.md](core/stages/06_project_context_interview.md)
- [core/stages/07_foundation_pack.md](core/stages/07_foundation_pack.md)
- [docs/legacy/guides/workflows/01_foundation_test_mode_charter.md](docs/legacy/guides/workflows/01_foundation_test_mode_charter.md)
- [docs/legacy/guides/workflows/02_foundation_real_project.md](docs/legacy/guides/workflows/02_foundation_real_project.md)
- [docs/legacy/guides/workflows/09_foundation_inputs_charter.md](docs/legacy/guides/workflows/09_foundation_inputs_charter.md)

Must prove:

- Rust can load a planning pipeline definition
- Rust can select a deterministic stage order
- Rust can evaluate stage activation and compute one authoritative resolved route
- Rust can persist and reuse the small routing state needed for multi-step planning flows
- Rust can update routing state explicitly in M1 without pulling full output materialization forward
- the chosen test corpus uses realistic canonical docs, not toy placeholder text

Minimum acceptable wedge:

- one foundation-style flow with `needs_project_context`-style branching
- canonical docs are assumed to be pre-populated before the flow starts
- the command family and schema behavior must remain pipeline-generic even though the first proof corpus is intentionally foundation-family only

Implementation checklist:

1. Define the Rust ownership boundary for M1.
   - `crates/compiler` owns pipeline models, route resolution, activation evaluation, and persisted pipeline state.
   - CLI command handlers stay thin adapters for argument parsing and rendering.
2. Define the Rust data model for pipeline loading.
   - Support the current two-document YAML shape used by the pipeline files.
   - Preserve defaults plus ordered `stages`.
   - Preserve stage-local `sets` and `activation`.
3. Reproduce pipeline path resolution.
   - Default to the root pipeline when no override is provided.
   - Allow relative pipeline paths rooted at the repo root.
   - Allow absolute pipeline paths.
4. Define the M1 `pipeline` command family.
   - Declarative inspection commands:
     - `pipeline list` reads pipeline metadata only.
     - `pipeline show --id <pipeline>` shows declared pipeline configuration only.
   - Route computation command:
     - `pipeline resolve --id <pipeline>` computes and prints the authoritative resolved route.
   - Stage payload compilation command:
     - `pipeline compile --id <pipeline> --stage <stage-id>` compiles one resolved stage payload.
   - Mutation commands:
     - `pipeline state set --id <pipeline> --var key=value` updates derived pipeline-run state.
   - Do not make `compile` implicitly choose a stage in the canonical form.
   - Canonical file ids remain the source of truth (for example `pipeline.foundation_inputs` and `stage.07_foundation_pack`).
   - CLI shorthand may strip the `pipeline.` or `stage.` prefix only when the shorthand is unambiguous.
   - If shorthand lookup is ambiguous, output must say that overlapping ids were found, list the conflicting canonical ids, and tell the operator to use the full canonical id or rename the conflicting ids.
5. Reproduce deterministic stage selection and route computation.
   - `resolve` decides, not just displays.
   - Declared stage order stays stable.
   - Active, skipped, blocked, and next stage status must be explicit in the resolved result.
6. Reproduce activation evaluation.
   - Support `activation.when.any` and `activation.when.all`.
   - Support the current literal comparisons the Python harness uses:
     - booleans
     - quoted strings
     - numbers
   - Preserve the foundation and foundation-inputs branching behavior for `needs_project_context` and `charter_gaps_detected`.
7. Reproduce planning state persistence under the Rust storage model.
   - Persist derived pipeline-run state under `.system/state/pipeline/<pipeline-id>.yaml`.
   - Treat this as derived orchestration state, not canonical project truth.
   - Keep the file narrow with an explicit top-level shape:
     - `routing`: branch variables such as `needs_project_context` and `charter_gaps_detected`
     - `refs`: convenience refs to prior outputs such as `charter_ref` and `project_context_ref`
     - `run`: run parameters such as `runner`, `profile`, `repo_root`, and any stage-family identifiers required for path substitution
   - Do not store compiled payloads, resolved-route snapshots, copied canonical artifact contents, or other duplicated derived views in this file.
   - Do not widen this into a new general state machine.
8. Define the explicit M1 state-update surface.
   - M1 cannot rely on full output materialization yet.
   - Add a narrow explicit command such as `pipeline state set --id <pipeline> --var key=value`.
   - This command exists so persisted routing-state proofs are real product behavior, not test-only fixture setup.
9. Define the stage compilation contract.
   - Pipeline YAML remains the source of truth for orchestration commands.
   - Pipeline-entry truth is limited to orchestration fields such as `id`, `file`, `sets`, `activation`, and control-flow.
   - Stage front matter remains the source of truth for stage compilation fields like `includes`, `inputs`, `outputs`, `gating`, `work_level`, and `tags`.
   - Any duplicated `inputs` or `outputs` in pipeline YAML are treated as legacy hints or removed over time. They must not outrank stage front matter.
   - Pipeline-entry `activation` is the source of truth for orchestration and route selection.
   - Stage-front-matter `activation` is legacy metadata unless and until a later plan removes it entirely.
   - `pipeline resolve` and any downstream route computation must use pipeline-entry `activation`, not stage-front-matter `activation`.
   - If both pipeline YAML and stage front matter define `activation` for the same stage, they must match exactly during the parity period.
   - Conformance must fail if duplicated `activation` blocks drift.
   - New stages added for the Rust-first path should define `activation` in pipeline YAML only unless there is a documented legacy-compatibility reason to duplicate it.
   - `compile` consumes the resolved pipeline result plus one stage definition. It does not compile the entire pipeline in one shot.
   - `compile` outputs only the stage-executable payload:
     - resolved `includes`
     - resolved library, artifact, and variable inputs
     - the declared outputs contract
     - gating metadata needed by the stage
   - `compile` must not persist writes, update pipeline state, cache resolved-route snapshots, or copy canonical artifact contents into a second source of truth.
10. Record the validation rail that parity work must add after M1 command shape is stable.
   - Validation must reject duplicate or conflicting canonical ids within the same namespace.
   - Validation must reject duplicate stage ids within a single pipeline.
   - Validation must fail if shorthand id normalization would create ambiguous command lookup without an explicit conflict report.
   - Validation must fail if duplicated `activation` blocks drift between pipeline YAML and stage front matter during the parity period.
   - The repo does not have this validator yet. Record it now so the requirement is not lost.
11. Record the required documentation realignment for the new command family.
   - The current CLI vocabulary and hierarchy docs were written for reduced-v1 packet-only behavior.
   - They must be revised before or alongside landing the `pipeline` family so docs do not become a competing product contract.
   - At minimum, parity work must update:
     - `docs/CLI_PRODUCT_VOCABULARY.md`
     - `docs/CLI_COMMAND_HIERARCHY.md`
     - `DESIGN.md`
     - `README.md`
     - `docs/START_HERE.md`
   - Until those files are revised, treat their packet-only `generate` language as historical reduced-v1 guidance rather than the final CLI contract for parity work.
12. Record the required test rail for the new command family.
   - Add a dedicated CLI integration suite for the `pipeline` family instead of folding these cases into the existing packet-only CLI tests.
   - Add compiler-core tests for resolved-route truth, activation branching, shorthand ambiguity handling, explicit `--stage`, and state persistence semantics.
   - Add explicit negative-path tests for:
     - unique shorthand success
     - ambiguous shorthand refusal that lists the conflicting canonical ids
     - duplicate stage ids within one pipeline
     - activation-drift validation failure once the validator lands
   - The `pipeline` family is public product contract. Manual verification alone is not sufficient for M1 completion.
13. Record the M1 performance boundary for pipeline parsing.
   - Reuse parsed pipeline config, parsed stage front matter, and loaded pipeline state in memory within a single command invocation.
   - This is tightly scoped per-invocation reuse only.
   - Do not add persisted caches, cross-command caches, or cached resolved-route/compiled-stage artifacts in M1.
   - If later profiling shows repeated process-start parsing is a real bottleneck, address that in a later milestone rather than smuggling broader caching into M1.

Proof commands for M1 completion:

- list available pipelines
- show the declared config for the default root pipeline
- show the declared config for `pipelines/foundation_inputs.yaml`
- resolve `foundation` with `needs_project_context=false` and confirm stage 06 is skipped
- resolve `foundation` with `needs_project_context=true` and confirm stage 06 is included
- resolve `foundation_inputs` with `charter_gaps_detected=true` and confirm stage 06 is included
- persist routing state under `.system/state/pipeline/<pipeline-id>.yaml`, re-run resolve, and confirm the same route is chosen without manual re-entry
- compile `stage.07_foundation_pack` from `foundation_inputs` with an explicit `--stage` and confirm the compiled payload reflects resolved pipeline context plus stage-front-matter inputs
- prove ambiguous shorthand id handling by showing a conflict message that lists the overlapping canonical ids and instructs the operator to use the full canonical id
- run dedicated `pipeline` CLI and compiler tests that cover route resolution, activation branching, shorthand ambiguity, explicit `--stage`, and pipeline-state persistence

Exit criteria:

- the two foundation-family pipeline files can be parsed by Rust
- stage order is byte-for-byte deterministic for the chosen foundation proof outputs
- the two foundation-family branches behave the same way they do in Python
- routing state survives enough to continue a multi-step planning flow
- the resolved-route result is one shared truth consumed by both `resolve` output and `compile`
- `compile` is stage-explicit in the canonical path and does not silently choose a stage
- the proof corpus uses realistic pre-populated canonical docs rather than toy fixtures
- dedicated `pipeline` CLI and compiler tests cover route resolution, activation branching, shorthand ambiguity, explicit `--stage`, and pipeline-state persistence

Non-goals inside M1:

- no full pipeline-wide compilation in one shot
- no artifact writing yet
- no onboarding chat flow
- no downstream seam-skill integration yet
- no release or sprint pipeline parity work yet
- no collapsing derived pipeline-run state into canonical `.system` artifact truth

### M2. Compilation Parity

Goal:

- recover the actual prompt-compilation behavior that keeps repo research from being repeated

Must prove:

- Rust can compile one stage from front matter, includes, profiles, runner guidance, and upstream artifacts
- scoped rules still filter correctly by work level
- stack-specific commands and conventions come from profiles, not hardcoded ad hoc prompts
- compiled output remains useful when source artifacts contain real-world-looking detail rather than toy fixture text

Minimum acceptable wedge:

- one compiled stage that matches the useful content classes the Python harness currently assembles

### M3. Output Materialization Parity

Goal:

- recover writing behavior so the compiler owns both prompt generation and artifact emission

Must prove:

- Rust can capture single-file outputs
- Rust can capture multi-file `--- FILE:` outputs exactly
- Rust can write both artifact outputs and canonical repo-file outputs
- re-entry state survives enough to continue a multi-stage planning flow without manual bookkeeping

Minimum acceptable wedge:

- one multi-file planning stage plus one canonical repo-file mirror

### M4. End-to-End Planning Flow Parity

Goal:

- replace one real planning-generation path that the operator actually uses

Must prove:

- the operator can complete one useful planning-generation flow through Rust without copy/paste context shuttling between phases
- the result is stable enough to trust as the generated planning basis
- the source corpus looks like a believable real project, not a synthetic two-line demo

Preferred first flow:

- foundation-inputs through feature-spec-grade planning, because it captures branching, compilation, and output materialization together

Important clarification:

- this does **not** mean Rust must own the initial onboarding chat first
- it means Rust must prove it can take already-populated canonical docs and generate the useful planning outputs from them

### M5. Downstream Consumer Handoff

Goal:

- make Rust outputs useful to the downstream seam and execution workflow, not just locally correct

Must prove:

- at least one downstream planning consumer can use Rust-generated artifacts without redoing the same repo research
- output size and structure reduce token bloat instead of amplifying it
- provenance and freshness are explicit enough that downstream consumers know what they can trust

This is where parity starts paying back the operator tax for real.

### M6. Cutover And Cleanup

Goal:

- make Rust the real planning-generation authority and reduce Python to historical reference

Must prove:

- the chosen parity flow is fully replaced in practice
- docs and entrypoints no longer push users back to the legacy harness for that flow
- the remaining legacy surface is explicitly historical or still-needed-only, not vaguely half-supported

## What Already Exists And Must Be Preserved

The Rust baseline already bought some useful product decisions. Do not throw these away while chasing parity:

- trust-heavy CLI posture
- small stable verb surface
- provenance-aware packet thinking
- `inspect` as proof surface
- `doctor` as recovery surface
- progressive disclosure as a product principle

Parity work should absorb and extend these, not bulldoze them.

## Deferred Work

These items are explicitly deferred behind parity:

- thin MCP/UI companion from [TODOS.md](TODOS.md)
- review/fix packet family from [TODOS.md](TODOS.md)
- live slice lineage and live execution packets from [TODOS.md](TODOS.md)
- public CLI distribution from [TODOS.md](TODOS.md)
- CLI release workflow from [TODOS.md](TODOS.md)

If a session proposes one of these before parity proves replacement value, the answer should usually be "not yet."

## Success Criteria

Parity is only real when all of the following are true for the chosen first flow:

1. the operator does not repeat the same repo research at multiple planning stages
2. the operator does not manually shuttle context between compiler-owned steps
3. outputs are consistent across repeated runs
4. downstream planning/execution consumers receive smaller, more trustworthy grounding
5. the operator can handle more concurrent work because babysitting is reduced
6. the proof corpus is rich enough that success actually demonstrates usefulness, not just parser correctness

## Immediate Next Work

1. Turn the parity ledger above into a concrete implementation checklist with proof commands and target test coverage.
2. Pick the first end-to-end planning flow to replace.
3. Define the acceptance test for "Rust now does the useful thing Python did" in operator terms, not internal architecture terms.
4. Start M1 only after the chosen first flow and acceptance checks are written down.

## Explicit Non-Goals For The Next Session

- do not redesign the whole compiler architecture
- do not reopen the archived reduced-v1 baseline as the active plan
- do not start with public release packaging
- do not add UI wrappers
- do not widen into all downstream seam skills

Stay on the parity path until the operator pain is materially reduced.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 0 | — | — |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR | 31 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**UNRESOLVED:** 0

**VERDICT:** ENG CLEARED — ready to implement.
