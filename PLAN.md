<!-- previous reduced-v1 baseline archived at .implemented/PLAN-20260409-144209-reduced-v1-baseline.md -->
# PLAN

## Status

This is the active execution source of truth for `system`.

The old reduced-v1 packet-baseline plan is preserved as historical evidence at:

- [.implemented/PLAN-20260409-144209-reduced-v1-baseline.md](.implemented/PLAN-20260409-144209-reduced-v1-baseline.md)

That archived plan explains why the current Rust CLI looks the way it does. It is no longer the active objective.

## Active Objective

Build the Rust-first planning compiler spine for foundation-family workflows, using legacy material as reference evidence where it is useful, without carrying migration baggage into the active product story.

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

by making Rust own the useful planning-generation work directly.

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

- a Rust-first `pipeline` command family for route resolution, explicit stage compilation, and narrow pipeline-run state management
- Rust planning-generation from already-populated canonical project documents
- a capability ledger tied to concrete proof files and commands
- the minimum end-to-end foundation-family flow that removes the most manual babysitting
- a locked proof corpus with named golden outputs for `pipeline resolve` and `pipeline compile`
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

Those can come back only after the Rust pipeline spine proves it materially reduces operator work on real foundation-family flows.

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

## Assumed Inputs For The First Wedge

The first wedge starts after the front-door onboarding work has already happened.

For this plan, assume the canonical project documents already exist and are rich enough to be useful:

- charter
- project context
- foundation/posture docs
- feature-level planning inputs

The onboarding flow that chats with an AI agent and populates those documents is a real part of the eventual product, but it is not the first implementation target in this plan.

The question for this phase is narrower:

- given realistic canonical inputs that already exist, can Rust generate useful planning artifacts and reusable grounding without manual context shuttling?

## Guardrails

These rules are active for every later session that touches this plan:

1. Do not widen the repo objective beyond the generator/compiler layer.
2. Do not treat the archived reduced-v1 plan as the active north star.
3. Do not delete useful historical evidence just to simplify the story.
4. Do not call the wedge "done" by feature count alone. Measure it by removed operator pain.
5. Do not rebuild all legacy behavior blindly. Borrow only the behavior that materially removes repeated grounding and manual shuttling.
6. Do not block the wedge on final canonical storage debates if the canonical versus runtime split is explicit in contracts and docs.
7. Keep the existing downstream planning skills as consumers of compiler outputs unless and until a separate plan changes that.
8. Do not treat the onboarding/intake flow as a prerequisite for proving the first planning-generation wedge.
9. Do not use toy one-paragraph fixtures as proof. Use realistic demo project artifacts that look like actual project truth.

## Reference Inputs And Current Sources Of Truth

Reference material that informs the wedge lives in:

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

If an archived artifact is too thin to prove usefulness, treat it as shape reference only, not as proof.

## Capability Ledger

The table below is the active capability inventory. It is intentionally behavior-first, not architecture-first.

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
| PL-11 | Product-contract cleanup and historical-reference boundaries | `README.md`, `docs/START_HERE.md`, archive plan, legacy docs | Later sessions drift into split product stories or treat historical reference files like active product contracts | P1 | M6 |
| PL-12 | Realistic demo corpus quality for proof | archived artifacts, established templates, demo project docs with substantial content | Thin one- or two-sentence fixtures can make compilation appear to work while proving nothing about usefulness | P0 | M1-M4 |

## Ordered Milestones

### M0. Control-plane reset

Status: complete

- archive the old reduced-v1 baseline plan
- make this file the only active root plan
- stop later sessions from following the wrong source of truth

### M0.5. YAML Parser Foundation Gate

Goal:

- lock the Rust YAML parsing choice before M1 so route work starts on a boring, supported parser base instead of parser churn

Must prove:

- Rust can parse the current two-document pipeline YAML shape with `serde_yaml_bw`
- the chosen parser supports the multi-document deserializer flow the pipeline files require
- the supported schema is explicit and narrow, rejecting unsupported YAML shapes instead of silently accepting them
- parser adoption lands with a small compiler-owned loader seam, not as a repo-wide parsing abstraction project

Gate:

- do not start M1 route/state implementation until `serde_yaml_bw` parsing is wired, tested, and proven against the current foundation-family pipeline files

### M1. Pipeline And Routing Spine

Goal:

- establish compiler ownership of planning path selection and conditional routing

Command-shape decision:

- keep the existing top-level verb surface for packet work and recovery
- add a first-class `pipeline` noun family as a supported product surface
- `pipeline list` discovers available pipelines
- `pipeline show --id <pipeline>` shows a normalized typed view of the declared pipeline config, including backing-file provenance
- `pipeline resolve --id <pipeline>` is the authoritative compute step for route selection and renders a compact ordered route report by default
- the shipped `M1` help/docs surface exposes only `pipeline list`, `pipeline show`, `pipeline resolve`, and `pipeline state set`
- `pipeline compile --id <pipeline> --stage <stage-id>` stays out of the shipped `M1` help/docs surface and lands as a supported command in `M2`
- `generate packet --id <packet>` remains the downstream packet-generation surface
- `setup` remains first-time setup and repo-wide refresh, but does **not** absorb incremental pipeline rebuild behavior

In plain English:

- `pipeline` owns orchestration
- `resolve` decides which stages are active, skipped, blocked, and next
- `M1` locks route truth and state truth
- `M2` makes `compile` build one stage payload from pipeline context plus stage schema
- `generate packet` stays separate from pipeline work
- `doctor` stays the recovery surface

This avoids two bad outcomes:

- overloading `generate` so it means both canonical rebuild work and packet generation
- overloading `setup` so it means both first-time truth establishment and everyday incremental pipeline work
- hiding route-computation logic inside `compile` instead of making it explicit and inspectable
- shipping `compile` before the compiler can actually honor stage-payload and refusal semantics

Reference surface:

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
   - pipeline route truth lives in a dedicated compiler-owned module and typed route result, separate from the existing packet resolver.
   - the current packet resolver stays packet-oriented; do not widen it to own pipeline routing semantics.
   - share only boring low-level helpers across packet and pipeline codepaths, such as path guards, hashing, and expression-evaluation helpers when justified.
   - CLI command handlers stay thin adapters for argument parsing and rendering.
2. Define the Rust data model for pipeline loading.
   - Use `serde_yaml_bw` as the Rust YAML parser for this wedge.
   - Support the current two-document YAML shape used by the pipeline files.
   - Preserve defaults plus ordered `stages`.
   - Preserve stage-local `sets` and `activation`.
3. Reproduce pipeline path resolution inside the approved repo surface.
   - Default to the root pipeline when no override is provided.
   - Allow relative pipeline paths rooted at the repo root.
   - Refuse out-of-root absolute paths, symlink escapes, or stage-file resolution outside the approved repo surface.
4. Define the M1 `pipeline` command family.
   - Declarative inspection commands:
     - `pipeline list` reads pipeline metadata only and prints canonical pipeline ids plus backing repo-relative file paths.
     - `pipeline show --id <pipeline>` shows a normalized typed view of declared pipeline configuration only.
     - the default `show` output includes canonical id, defaults, ordered stages, `sets`, `activation`, and backing repo-relative file path.
     - raw YAML remains repo evidence, not the default CLI render contract.
   - Route computation command:
     - `pipeline resolve --id <pipeline>` computes and prints the authoritative resolved route.
     - the default `resolve` output is a compact ordered route report:
       - pipeline id
       - route-basis summary
       - one ordered stage list with explicit per-stage status
     - per-stage status must be one of `active`, `skipped`, `blocked`, or `next`.
     - any stage that is not `active` must include the key condition or refusal reason that explains the status.
   - Mutation commands:
     - `pipeline state set --id <pipeline> --var key=value` updates derived pipeline-run state.
   - Canonical file ids remain the source of truth (for example `pipeline.foundation_inputs` and `stage.07_foundation_pack`).
   - `show` accepts canonical ids first, with shorthand only when unambiguous.
   - Raw file-path targeting is evidence in `list` output, not a first-class operator input in M1.
   - CLI shorthand may strip the `pipeline.` or `stage.` prefix only when the shorthand is unambiguous.
   - If shorthand lookup is ambiguous, output must say that overlapping ids were found, list the conflicting canonical ids, and tell the operator to use the full canonical id or rename the conflicting ids.
   - the shipped `M1` subset of `pipeline` is a supported surface once the contract, docs, help, tests, and proof-corpus gates pass.
   - do not expose `pipeline compile` in shipped help text or supported-command docs until `M2` lands.
   - do not ship `pipeline` as a vague or partially documented shadow command family.
5. Reproduce deterministic stage selection and route computation.
   - `resolve` decides, not just displays.
   - Declared stage order stays stable.
   - Active, skipped, blocked, and next stage status must be explicit in the resolved result.
6. Reproduce activation evaluation.
   - Support `activation.when.any` and `activation.when.all`.
   - Support only the narrow typed subset needed for the first wedge:
     - booleans
     - quoted strings
     - numbers
   - Limit comparisons to variable-path equality checks inside that subset.
   - Treat any activation shape outside the supported subset as a pipeline load/validation failure, not a late resolve-time surprise.
   - Preserve the foundation and foundation-inputs branching behavior for `needs_project_context` and `charter_gaps_detected`.
7. Reproduce planning state persistence under the Rust storage model.
   - Persist derived pipeline-run state under `.system/state/pipeline/<pipeline-id>.yaml`.
   - Treat this as derived orchestration state, not canonical project truth.
   - Update contracts and product vocabulary so `.system/` is governed as two explicit zones:
     - canonical artifact zones
     - non-canonical runtime zones
   - Keep the file narrow with an explicit top-level shape:
     - `routing`: branch variables such as `needs_project_context` and `charter_gaps_detected`
     - `refs`: exactly two convenience refs in M1, `charter_ref` and `project_context_ref`
     - `run`: a closed M1 set of run parameters, `runner`, `profile`, and derived `repo_root`
   - Do not add a generic open-ended refs bucket in M1. Any ref beyond `charter_ref` and `project_context_ref` is deferred until a later milestone justifies it.
   - Do not add a generic extension point under `run` in M1. Any new run key requires an explicit plan/contract update.
   - Do not store compiled payloads, resolved-route snapshots, copied canonical artifact contents, or other duplicated derived views in this file.
   - Do not widen this into a new general state machine.
8. Define the explicit M1 state-update surface.
   - M1 cannot rely on full output materialization yet.
   - Add a narrow explicit command such as `pipeline state set --id <pipeline> --var key=value`.
   - Restrict writes to a declared schema of allowed keys and typed values. Unknown keys must refuse.
   - The writable M1 surface is only:
     - `routing.*`
     - `run.runner`
     - `run.profile`
     - `refs.charter_ref`
     - `refs.project_context_ref`
   - `run.repo_root` may be stored for audit or basis freshness, but it is derived from invocation/root discovery and is not writable through `state set`.
   - Any other state key is read-only or out of scope for M1 and must refuse through `state set`.
   - Append an inspectable audit record for each route-relevant mutation.
   - Keep audit history bounded inside the state file in M1, for example the most recent 50 mutation records, instead of turning the state file into a long-term provenance log.
   - `resolve` uses only the current state needed for route truth and freshness; audit history remains for inspection/debugging and is not part of the normal resolve hot path.
   - Use one concrete mutation protocol:
     - acquire an advisory file lock on the pipeline-state file before read/modify/write
     - require a monotonic stored revision field in the state payload
     - refuse the write if the on-disk revision changed between read and commit
     - commit with atomic write-then-rename only after the lock and revision checks pass
   - Do not allow silent last-write-wins ambiguity.
   - This command exists so persisted routing-state proofs are real product behavior, not test-only fixture setup.
9. Define the stage compilation contract for M2.
   - Pipeline YAML remains the source of truth for orchestration commands.
   - Pipeline-entry truth is limited to orchestration fields such as `id`, `file`, `sets`, `activation`, and control-flow.
   - Stage front matter remains the source of truth for stage compilation fields like `includes`, `inputs`, `outputs`, `gating`, `work_level`, and `tags`.
   - Any duplicated `inputs` or `outputs` in pipeline YAML are treated as legacy hints or removed over time. They must not outrank stage front matter.
   - Pipeline-entry `activation` is the source of truth for orchestration and route selection.
   - Stage-front-matter `activation` is legacy metadata unless and until a later plan removes it entirely.
   - `pipeline resolve` and any downstream route computation must use pipeline-entry `activation`, not stage-front-matter `activation`.
   - If both pipeline YAML and stage front matter define `activation` for the same stage, they must be semantically equivalent after normalization during the current wedge.
   - Conformance must fail immediately in M1 if duplicated `activation` blocks drift.
   - New stages added for the Rust-first path should define `activation` in pipeline YAML only unless there is a documented legacy-compatibility reason to duplicate it.
   - `compile` consumes the resolved pipeline result plus one stage definition. It does not compile the entire pipeline in one shot.
   - `compile` must verify that the resolved-route basis is still fresh against pipeline, stage, profile, runner, and route-state inputs. If the basis drifted, refuse and tell the operator to re-run `pipeline resolve`.
   - If the requested stage exists but is inactive in the current resolved route, `compile` must refuse explicitly, show that the stage is real but inactive, and identify the current routing condition blocking it.
   - `compile` outputs only the stage-executable payload:
     - resolved `includes`
     - resolved library, artifact, and variable inputs
     - the declared outputs contract
     - gating metadata needed by the stage
   - Required compile inputs are all-or-nothing. Missing, empty, malformed, or out-of-scope required inputs must refuse instead of producing partial payloads.
   - `compile` must not persist writes, update pipeline state, cache resolved-route snapshots, or copy canonical artifact contents into a second source of truth.
10. Record the validation rail that this wedge must add after M1 command shape is stable.
   - Validation must reject duplicate or conflicting canonical ids within the same namespace.
   - Validation must reject duplicate stage ids within a single pipeline.
   - Validation must fail if shorthand id normalization would create ambiguous command lookup without an explicit conflict report.
   - Validation must fail if duplicated `activation` blocks drift between pipeline YAML and stage front matter during the current wedge.
   - Activation-drift enforcement is required in M1. A first-class `pipeline validate` operator surface is deferred to [TODOS.md](TODOS.md).
11. Record the required documentation realignment for the new command family.
   - The current CLI vocabulary and hierarchy docs were written for reduced-v1 packet-only behavior.
   - They must be revised before or alongside landing the `pipeline` family so docs do not become a competing product contract.
   - At minimum, this wedge must update:
     - `docs/CLI_PRODUCT_VOCABULARY.md`
     - `docs/CLI_COMMAND_HIERARCHY.md`
     - `DESIGN.md`
     - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
     - `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
      - `README.md`
      - `docs/START_HERE.md`
   - Until those files are revised, treat their packet-only `generate` language as historical reduced-v1 guidance rather than the final CLI contract for the `pipeline` spine.
12. Record the required test rail for the new command family.
   - Add explicit M0.5 compiler tests for `serde_yaml_bw` adoption:
     - multi-document YAML parsing for the current pipeline files
     - unsupported activation syntax failing at load/validation time
     - malformed pipeline definition refusal
     - duplicate stage-id or duplicate/conflicting canonical-id refusal
   - Add one shared malformed-pipeline proof case that `pipeline list`, `pipeline show`, and `pipeline resolve` must all fail on with the same refusal classification and next-action posture.
   - Add an M1 command-by-command test matrix for `pipeline list`, `show`, `resolve`, and `state set`, covering happy path, refusal path, edge cases, and stateful rerun behavior.
   - Add a dedicated CLI integration suite for the `pipeline` family instead of folding these cases into the existing packet-only CLI tests.
   - Add explicit M1 help snapshots and docs/help parity checks for the shipped `pipeline` subset, reusing the existing `help_drift_guard` pattern for `system --help`, `system pipeline --help`, and relevant pipeline subcommands.
   - Add M1 compiler-core tests for resolved-route truth, activation branching, shorthand ambiguity handling, schema handling, and state persistence semantics.
   - Add explicit negative-path tests for:
     - unknown canonical ids
     - unique shorthand success
     - ambiguous shorthand refusal that lists the conflicting canonical ids
     - duplicate stage ids within one pipeline
     - activation-drift validation failure
     - malformed persisted state refusal
   - Add dedicated M1 state-mutation tests for advisory lock acquisition, revision-conflict refusal, atomic write behavior, audit-record append semantics, and read-after-write route consistency.
   - Require at least one real competing-write integration test that exercises lock acquisition on the actual state-file path, not just helper-level unit tests.
   - Require at least one real revision-conflict integration test against the actual state-file path, not just mocked revision helpers.
   - Lock an M1 proof corpus with golden outputs for `pipeline resolve` and `pipeline state set`.
   - Store that M1 proof corpus in one repo-owned foundation-family directory adjacent to the CLI/compiler test suites, not inside `.system/` or another product-shaped runtime zone.
   - CLI and compiler tests must share that one foundation-family proof corpus rather than maintaining duplicate golden case sets.
   - Compiler tests assert typed route/state semantics over the shared cases; CLI tests assert rendered output over those same cases.
   - Every golden update must include an explicit reason and the affected contract surface.
   - Add M2 compile-specific CLI and compiler tests for explicit `--stage`, route-basis freshness, inactive-stage refusal, and compiled payload contents.
   - Lock M2 golden outputs for `pipeline compile`.
   - The `pipeline` family is public product contract. Manual verification alone is not sufficient for either milestone.
13. Record the M1 performance boundary for pipeline parsing.
   - Keep command cost split explicit:
     - `pipeline list` loads pipeline YAML plus minimal validation only
     - `pipeline show` loads pipeline YAML plus the metadata needed for the normalized typed view
     - `pipeline resolve` is the first command allowed to load activation-bearing stage metadata and pipeline state
   - Reuse parsed pipeline config, parsed stage metadata, and loaded pipeline state in memory within a single command invocation when that command actually needs them.
   - This is tightly scoped per-invocation reuse only.
   - Do not add persisted caches, cross-command caches, or cached resolved-route/compiled-stage artifacts in M1.
   - Add simple latency expectations for the foundation-family proof corpus and treat repeated growth in parse steps or corpus costs as a regression to investigate.
   - Rough local budgets for the proof corpus:
     - `pipeline list`: low tens of milliseconds
     - `pipeline show`: low tens of milliseconds
     - `pipeline resolve`: comfortably under a human-perceptible pause on the proof corpus
   - Any material regression against those rough budgets must be explained in the golden-change note or captured as explicit follow-on work.
   - Keep default `resolve` proof surfaces compact and auditable in M1. `compile` proof-surface rules land with M2.
   - If later profiling shows repeated process-start parsing is a real bottleneck, address that in a later milestone rather than smuggling broader caching into M1.
14. Record the security and operability rules for the wedge.
   - Profile and runner selection must resolve from declared allowlisted ids inside the approved repo surface.
   - Proof output must surface the exact profile and runner inputs used to shape compilation.
   - Unknown canonical ids and ambiguous shorthand ids must remain distinct refusal classes with distinct user-facing recovery guidance.
   - Malformed or semantically invalid persisted state must refuse explicitly. Do not auto-delete, auto-heal, or silently ignore it.
   - Structured run provenance for `resolve`, `compile`, and `state set` is deferred to [TODOS.md](TODOS.md), but the plan should preserve the requirement shape.
15. Keep the product-story boundary honest.
   - The command family and typed core are generic.
   - The first supported proof corpus and schema coverage remain foundation-family narrow until later milestones expand them.
   - Future compiler work is justified by reduced operator work and more trustworthy downstream artifacts, not by generic workflow-engine flexibility.

Proof commands for M1 completion:

- list available pipelines
- show the declared config for the default root pipeline
- show the declared config for `pipelines/foundation_inputs.yaml`
- resolve `foundation` with `needs_project_context=false` and confirm stage 06 is skipped
- resolve `foundation` with `needs_project_context=true` and confirm stage 06 is included
- resolve `foundation_inputs` with `charter_gaps_detected=true` and confirm stage 06 is included
- persist routing state under `.system/state/pipeline/<pipeline-id>.yaml`, re-run resolve, and confirm the same route is chosen without manual re-entry
- prove ambiguous shorthand id handling by showing a conflict message that lists the overlapping canonical ids and instructs the operator to use the full canonical id
- prove malformed persisted state refusal and revision-conflict refusal
- run dedicated `pipeline` CLI and compiler tests that cover route resolution, activation branching, shorthand ambiguity, pipeline-state persistence, advisory locking, and state mutation semantics

Exit criteria:

- the two foundation-family pipeline files can be parsed by Rust
- stage order is byte-for-byte deterministic for the chosen foundation proof outputs
- the two foundation-family branches behave the way the locked proof corpus says they should
- routing state survives enough to continue a multi-step planning flow
- the resolved-route result is the one shared truth consumed by `resolve` output and persisted routing-state behavior
- `.system/` canonical versus runtime zones are documented explicitly
- the proof corpus uses realistic pre-populated canonical docs rather than toy fixtures
- dedicated `pipeline` CLI and compiler tests cover route resolution, activation branching, shorthand ambiguity, malformed-state refusal, revision-conflict refusal, advisory locking, and pipeline-state persistence
- docs, contracts, help, tests, and proof-corpus gates are all green in the same release, and the M1 subset of `pipeline` is shipped as a supported surface rather than an undocumented experiment

Non-goals inside M1:

- no full pipeline-wide compilation in one shot
- no artifact writing yet
- no onboarding chat flow
- no downstream seam-skill integration yet
- no release or sprint pipeline coverage yet
- no collapsing derived pipeline-run state into canonical `.system` artifact truth
- no real `pipeline compile` stage-payload generation yet

### M2. Compilation Capability

Goal:

- recover the actual prompt-compilation behavior that keeps repo research from being repeated, after M1 has already locked route truth and state truth

Must prove:

- Rust can compile one stage from front matter, includes, profiles, runner guidance, and upstream artifacts
- `pipeline compile --id <pipeline> --stage <stage-id>` is the supported product surface for that behavior
- scoped rules still filter correctly by work level
- stack-specific commands and conventions come from profiles, not hardcoded ad hoc prompts
- compiled output remains useful when source artifacts contain real-world-looking detail rather than toy fixture text

Minimum acceptable wedge:

- one compiled stage that matches the useful content classes the Python harness currently assembles

### M3. Output Materialization Capability

Goal:

- recover writing behavior so the compiler owns both prompt generation and artifact emission

Must prove:

- Rust can capture single-file outputs
- Rust can capture multi-file `--- FILE:` outputs exactly
- Rust can write both artifact outputs and canonical repo-file outputs
- re-entry state survives enough to continue a multi-stage planning flow without manual bookkeeping

Minimum acceptable wedge:

- one multi-file planning stage plus one canonical repo-file mirror

### M4. End-to-End Foundation Flow

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

This is where the wedge starts paying back the operator tax for real.

### M6. Historical Reference Cleanup And Coverage Expansion

Goal:

- keep the Rust product story coherent while expanding coverage and keeping historical reference material clearly non-authoritative

Must prove:

- the chosen foundation-family flow is fully supported through the Rust surface
- docs and entrypoints describe one coherent Rust-first product story
- the remaining historical reference surface is explicitly non-authoritative, not vaguely half-supported

## What Already Exists And Must Be Preserved

The Rust baseline already bought some useful product decisions. Do not throw these away while chasing the pipeline spine:

- trust-heavy CLI posture
- small stable verb surface
- provenance-aware packet thinking
- `inspect` as proof surface
- `doctor` as recovery surface
- progressive disclosure as a product principle

This work should absorb and extend these, not bulldoze them.

## CEO Review Addendum (2026-04-11)

These decisions were locked during `/plan-ceo-review` and are now part of the active plan:

- `pipeline` is a supported surface once the code, contracts, docs, help, tests, and proof-corpus gates pass. Do not ship it as an experimental or shadow surface.
- `resolve` and `compile` stay separate jobs, but they must share one typed resolved-route truth.
- `.system/` must be governed as canonical artifact zones plus explicit non-canonical runtime zones.
- `pipeline state set` is schema-bound, audited, atomic, and conflict-aware.
- Unknown ids, ambiguous shorthand, malformed state, stale route basis, and inactive stages are all distinct refusal classes.
- Required compile inputs are all-or-nothing.
- The implementation should stay in a small number of boring modules with compiler-owned semantics and one small shared typed identity layer.
- In M1, that identity layer means only concrete pipeline-id and stage-id wrappers plus normalization rules, not a generic identity framework.
- The proof corpus is mandatory and golden changes require an explicit reason and affected contract surface.
- Latency and output-size discipline are part of the wedge, not post-ship cleanup.
- Future work is justified by reduced operator work and more trustworthy downstream artifacts, not by generic YAML workflow-engine flexibility.

## Deferred Work

These items are explicitly deferred behind the first wedge:

- thin MCP/UI companion from [TODOS.md](TODOS.md)
- review/fix packet family from [TODOS.md](TODOS.md)
- live slice lineage and live execution packets from [TODOS.md](TODOS.md)
- public CLI distribution from [TODOS.md](TODOS.md)
- CLI release workflow from [TODOS.md](TODOS.md)
- operator-outcome scoreboard from [TODOS.md](TODOS.md)
- `pipeline validate` preflight surface from [TODOS.md](TODOS.md)
- structured run provenance for `resolve`, `compile`, and `state set` from [TODOS.md](TODOS.md)

If a session proposes one of these before the first wedge proves replacement value, the answer should usually be "not yet."

## Success Criteria

The first wedge is only real when all of the following are true for the chosen flow:

1. the operator does not repeat the same repo research at multiple planning stages
2. the operator does not manually shuttle context between compiler-owned steps
3. outputs are consistent across repeated runs
4. downstream planning/execution consumers receive smaller, more trustworthy grounding
5. the operator can handle more concurrent work because babysitting is reduced
6. the proof corpus is rich enough that success actually demonstrates usefulness, not just parser correctness

## Immediate Next Work

1. Turn the capability ledger above into a concrete implementation checklist with proof commands and target test coverage.
2. Write the full error/rescue registry, data-flow diagrams, and test matrix into the active plan artifact.
3. Pick the first end-to-end foundation-family flow to support.
4. Define the acceptance test in operator terms, not internal architecture terms.
5. Start M1 only after the chosen first flow and acceptance checks are written down.

## Explicit Non-Goals For The Next Session

- do not redesign the whole compiler architecture
- do not reopen the archived reduced-v1 baseline as the active plan
- do not start with public release packaging
- do not add UI wrappers
- do not widen into all downstream seam skills

Stay on the wedge until the operator pain is materially reduced.

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR | 4 proposals, 1 accepted, 2 deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | CLEAR | 31 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**UNRESOLVED:** 0

**VERDICT:** CEO + ENG CLEARED — ready to implement.
