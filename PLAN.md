<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-system/main-autoplan-restore-20260416-191254.md -->
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

- a Rust-first `pipeline` command family for route resolution, explicit stage compilation, explicit capture/materialization, and narrow pipeline-run state management
- Rust planning-generation from already-populated canonical project documents
- a capability ledger tied to concrete proof files and commands
- the minimum end-to-end foundation-family flow that removes the most manual babysitting
- a locked proof corpus with named golden outputs for `pipeline resolve`, `pipeline compile`, and the active `pipeline capture` wedge
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
   - Support only the narrow reduced-v1 subset needed for the first wedge:
     - boolean literals only
   - Limit comparisons to variable-path equality checks in the form `variables.<name> == true|false`.
   - Treat quoted strings and numbers as out of scope for shipped M1 activation evaluation.
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
   - Add split malformed-pipeline proof cases:
     - `pipeline list` and `pipeline show` must ignore unrelated malformed pipeline or stage files during metadata-only inventory inspection.
     - `pipeline show` must still fail with an explicit catalog refusal when the selected pipeline's declared metadata cannot be loaded.
     - `pipeline resolve` must still fail on malformed pipeline or stage data that invalidates the strict route-aware catalog.
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
   - Add M2 compile-specific CLI and compiler tests for:
     - explicit `--stage` selection within the chosen pipeline
     - missing persisted `route_basis` refusal
     - malformed persisted `route_basis` refusal
     - stale `route_basis` refusal after route-state mutation without a fresh `pipeline resolve`
     - inactive-stage refusal when the selected stage is not active in the persisted route basis
     - selected-stage-not-in-pipeline refusal
     - missing required artifact refusal for `stage.10_feature_spec`
     - optional-artifact absence success for `stage.10_feature_spec`
     - payload-only stdout success for plain `pipeline compile`
     - proof-only success for `pipeline compile --explain`
     - shared typed compile-result behavior so payload and explain modes cannot drift semantically
     - CLI help/docs parity for the new compile surface and `--explain`
   - Treat stale-route-basis refusal and inactive-stage refusal as regression-critical tests for `M2`; they are contract-preserving tests, not optional coverage.
   - Lock M2 golden outputs for both `pipeline compile` payload output and `pipeline compile --explain` proof output over the shared foundation-family proof corpus.
   - CLI and compiler compile tests must share those same M2 goldens instead of maintaining separate payload/proof fixtures.
   - The `pipeline` family is public product contract. Manual verification alone is not sufficient for either milestone.
13. Record the M1 performance boundary for pipeline parsing.
   - Keep command cost split explicit:
     - `pipeline list` loads pipeline YAML plus minimal validation only
     - `pipeline show` loads pipeline YAML plus the metadata needed for the normalized typed view
     - `pipeline resolve` is the first command allowed to load activation-bearing stage metadata and pipeline state
     - `pipeline compile` reuses the already-selected pipeline definition, selected stage metadata, persisted `route_basis`, and loaded artifact/profile content in memory within that single command invocation only
   - Reuse parsed pipeline config, parsed stage metadata, and loaded pipeline state in memory within a single command invocation when that command actually needs them.
   - This is tightly scoped per-invocation reuse only.
   - Do not add persisted caches, cross-command caches, cached compiled payloads, or stored explain payloads in `M2`.
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

Status: complete

Post-ship QA note, 2026-04-14:

- post-merge QA revalidated the shipped M2 surface end to end: `pipeline compile`, `pipeline compile --explain`, route-basis refusal flows, shared goldens, help/docs parity, and workspace quality gates
- post-merge QA also fixed the inactive-stage compile refusal wording so the next safe action now tells the operator to adjust route state if needed before retrying compile

Goal:

- recover the actual prompt-compilation behavior that keeps repo research from being repeated, after M1 has already locked route truth and state truth

Reviewed scope lock, 2026-04-13:

- `M2` is intentionally narrowed to one explicit single-stage compile wedge.
- The first supported compile target in `M2` is `pipeline.foundation_inputs` stage `stage.10_feature_spec`.
- `M2` does not attempt generic multi-stage compile coverage, output writing, or broad compile parity across the full legacy harness surface.
- `M2` must land as one coherent supported slice: compiler behavior, CLI surface, proof corpus, docs/help parity, and tests all green together.
- Stage selection for that first compile wedge must be explicit in the plan and justified by downstream usefulness rather than convenience alone.
- `pipeline compile` in `M2` prints the compiled stage payload to stdout; it does not recreate legacy `dist/` writes and does not grow an optional output-path flag.
- `pipeline resolve` in `M2` becomes the writer of one bounded persisted `route_basis` snapshot, and `pipeline compile` consumes that snapshot for freshness and active-stage checks instead of silently re-running resolve.
- successful `pipeline compile` output in `M2` is payload-only. Proof metadata, route-basis detail, and freshness evidence stay out of the payload stream and remain the job of refusal output plus `pipeline compile --explain`.
- `pipeline compile` in `M2` does not accept runner/profile override flags. Compile consumes the persisted `route_basis` exactly as resolved; changing runner or profile requires state mutation plus a fresh `pipeline resolve`.
- compile proof in `M2` is exposed as `pipeline compile --explain`. Plain `compile` stays payload-only, while `compile --explain` is a proof-only mode for route basis, selected stage metadata, include expansion, and required-input decisions.
- the implementation budget for `M2` stays intentionally small: thin CLI wiring in `crates/cli`, one new compiler-owned compile module in `crates/compiler`, and only minimal extensions to existing `pipeline` and `route_state` modules for shared contracts and persisted `route_basis`
- plain `pipeline compile` and `pipeline compile --explain` share one compiler-owned typed compile result. The stage payload is assembled once, then rendered either as payload-only stdout or proof-only explain output.
- `M2` preserves the useful content classes from the legacy harness, includes, runner/profile material, library inputs, required/optional artifact inputs, and scoped-rule filtering, but normalizes final output shape and refusal language to the reviewed Rust contracts instead of chasing byte-for-byte legacy parity.
- The first executable `M2` slice is prerequisite prep, not compile assembly: add `stage.10_feature_spec` to the real and proof-corpus `pipeline.foundation_inputs` declarations and seed the shared proof corpus with the compile-time library, rules, profile-pack, and upstream artifact inputs that stage needs.
- Until that prerequisite slice lands, truthful `M2` compile behavior for the locked target is refusal, not synthetic success.

Must prove:

- Rust can compile one stage from front matter, includes, profiles, runner guidance, and upstream artifacts
- `pipeline compile --id <pipeline> --stage <stage-id>` is the supported product surface for that behavior
- scoped rules still filter correctly by work level
- stack-specific commands and conventions come from profiles, not hardcoded ad hoc prompts
- compiled output remains useful when source artifacts contain real-world-looking detail rather than toy fixture text

Minimum acceptable wedge:

- one compiled stage that matches the useful content classes the Python harness currently assembles
- one explicit first-stage target, not a vague promise of "single-stage support"
- the explicit first-stage target is `stage.10_feature_spec`, because it proves real compile assembly over runner/profile/includes/library inputs and upstream artifacts without dragging multi-file output materialization into `M2`
- the compile payload is a terminal-facing product surface in `M2`; persisted output files stay deferred to `M3`
- freshness refusal in `M2` is grounded on a persisted `route_basis` snapshot written by `pipeline resolve`, not on implicit resolver reruns or caller-managed ad hoc tokens
- the copy-paste handoff matters more than decorative proof on success; payload-only stdout keeps `pipeline compile` usable while preserving `inspect` as the packet proof surface and `pipeline compile --explain` as the compile proof surface
- compile must not become a second route-selection surface; runner/profile changes remain explicit route-state changes upstream of compile
- compile proof stays local to compile semantics instead of widening `inspect` from packet proof into a generic catch-all proof surface
- payload and explain output must not drift; one typed compile result keeps success rendering and proof rendering on the same truth
- legacy Python remains behavioral reference, not formatting authority; Rust owns the supported output shape once `M2` lands
- compile success is gated on repo truth and proof-corpus truth matching the selected target; if the stage is absent from the pipeline or its required proof inputs are missing, refusal is the correct product behavior

Non-goals inside M2:

- no artifact or repo-file writes yet, those stay in `M3`
- no multi-stage compile orchestration or `--until` / `--only` style compile breadth from the legacy harness
- no hidden fallback to legacy Python for missing Rust behavior
- no generalized compile IR, cache layer, or new abstraction stack beyond what the single-stage wedge requires
- no multi-module compile mini-framework for one stage; if the design needs several new helper modules to explain itself, it is already overbuilt for `M2`
- no fake compile success against a target stage that the real pipeline and shared proof corpus do not yet support

Implementation-ready expansion, validated 2026-04-13:

Current repo blockers that must be cleared before `M2` can claim compile success:

- the chosen target stage already exists at `core/stages/10_feature_spec.md`, but neither `pipelines/foundation_inputs.yaml` nor the shared proof-corpus copy at `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/pipelines/foundation_inputs.yaml` currently declares `stage.10_feature_spec`
- the shared proof corpus also does not yet contain the compile-time inputs that `stage.10_feature_spec` needs, including the feature-spec library files, the full profile pack, the rules referenced by the stage, and realistic upstream artifacts under `artifacts/base/`, `artifacts/charter/`, and `artifacts/foundation/`
- until those repo-truth and proof-corpus prerequisites land, the correct behavior for the locked M2 target is refusal, not fake success

Parallel implementation lanes:

```text
M2 lane dependency map
======================
Lane A  compiler core + live route_basis contract
  touches: crates/compiler/, pipelines/foundation_inputs.yaml, route-state contract docs
  produces: compile API, typed compile result, persisted route_basis shape

Lane C  proof corpus + shared goldens
  touches: tests/fixtures/, crates/compiler/tests/, crates/cli/tests/
  produces: stage.10 proof corpus, refusal seeds, shared payload/explain goldens

Lane B  CLI + help/docs cutover
  touches: crates/cli/, help snapshots, README/docs
  consumes: Lane A compile API and Lane C committed examples/goldens

Launch order:
  1. Start Lane A + Lane C in parallel.
  2. Freeze compile API + shared proof corpus.
  3. Finish Lane B wiring, snapshots, and docs parity.
  4. Run the full shared green suite before calling M2 done.
```

| Lane | Scope | Primary touch surfaces | Depends on |
|------|-------|------------------------|------------|
| A | compiler core, live pipeline target wiring, `route_basis`, typed compile result, payload/proof renderers | `crates/compiler`, `pipelines/foundation_inputs.yaml`, stage metadata loaders, route-state contract docs | — |
| B | CLI command shape, help text, docs/help cutover, payload/proof posture | `crates/cli`, docs, help snapshots | Lane A compile interface, Lane C committed goldens/examples |
| C | proof corpus, shared goldens, regression coverage, help/doc drift gates | `tests/fixtures`, `crates/compiler/tests`, `crates/cli/tests`, docs parity checks | fixture prep may start early; final goldens wait on A renderer contract and B shipped CLI wording |

Lane A. Compiler Core + `route_basis` Persistence:

- exact file budget for Lane A:
  - `pipelines/foundation_inputs.yaml`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/src/pipeline.rs`
  - `crates/compiler/src/pipeline_route.rs`
  - `crates/compiler/src/route_state.rs`
  - `crates/compiler/src/pipeline_compile.rs`
  - `docs/contracts/pipeline-route-and-state-core.md`
- keep compiler ownership boring and explicit:
  - `crates/compiler/src/pipeline.rs` continues to own pipeline loading, selector resolution, declared stage order, and stage-file identity validation
  - `crates/compiler/src/pipeline_route.rs` continues to own route evaluation only
  - `crates/compiler/src/route_state.rs` continues to own persisted pipeline state, revisions, locking, and atomic replacement
  - add exactly one new compiler-owned module, `crates/compiler/src/pipeline_compile.rs`, for compile-target eligibility, stale-basis checks, input expansion, and render-from-one-result behavior
- Lane A owns the live repo target wiring by appending `stage.10_feature_spec` to `pipelines/foundation_inputs.yaml`; the shared proof-corpus pipeline copy stays with Lane C
- extend the persisted route-state schema with one bounded `route_basis` snapshot written only by `pipeline resolve`
- when `route_basis` lands, bump the route-state schema version and update `docs/contracts/pipeline-route-and-state-core.md` in the same change; do not introduce a new top-level field under the old exact-key contract
- `route_basis` must store only the information compile needs to trust a resolved route without rerunning resolve:
  - canonical `pipeline_id`
  - repo-relative pipeline file path plus a file fingerprint
  - the state revision captured at resolve time
  - snapshots of `routing`, `refs`, and `run` exactly as resolve used them
  - the ordered resolved route snapshot for all declared stages, including `stage_id`, repo-relative `file`, persisted route `status`, persisted route `reason`, and stage file fingerprint
  - the selected runner id and runner-doc fingerprint
  - the selected profile id and fingerprints for `profile.yaml`, `commands.yaml`, and `conventions.md`
  - one explicit `route_basis` schema/version field
- `route_basis` must not store compiled payload bytes, explain output bytes, copied artifact contents, copied include contents, duplicated audit history, or compile-only overrides
- freshness checks for `pipeline compile` must stay compiler-owned and must not rerun route evaluation:
  - require present, well-formed `route_basis`
  - require current pipeline-state `revision`, `routing`, `refs`, and `run` to match the persisted snapshot
  - require current pipeline, selected stage, selected runner, and selected profile-pack fingerprints to match the persisted snapshot
  - any mismatch is stale-route-basis refusal with the exact recovery path: rerun `pipeline resolve`
- selected-stage eligibility order:
  - resolve the selected pipeline through the published pipeline selector rules
  - resolve the selected stage through the same canonical-id / shorthand rules, still scoped to the selected pipeline
  - confirm the stage is declared in the pipeline’s ordered stage list
  - confirm the stage file exists and its front matter matches the canonical stage id
  - confirm the stage is present in the persisted resolved-route snapshot
  - confirm the stage is `active` in that snapshot
- preserve the useful legacy content classes from `tools/harness.py` while normalizing the output shape to Rust contracts:
  - ordered include expansion from stage front matter
  - selected runner docs and selected profile-pack files derived from `run.runner` and `run.profile`
  - library inputs
  - required and optional artifact inputs
  - scoped filtering by `work_level`, defaulting to `L1` when the stage leaves it empty
  - outputs contract, gating notes, and optional stage body
- normalize, do not clone, legacy behavior:
  - do not recreate `dist/` writes
  - do not keep the harness habit of silently rendering `(missing)` into a “successful” payload for compile-shaping inputs
  - treat missing compile-shaping includes, missing runner docs, missing selected profile-pack files, and missing required library/artifact inputs as explicit refusals
  - keep optional-artifact absence visible in `--explain` but non-fatal on the success path
  - render repo-relative paths, LF newlines, and one stable output order from typed data instead of ad hoc string concatenation
- one shared typed compile result must power both plain `pipeline compile` and `pipeline compile --explain`
- the typed result must carry:
  - `target`: pipeline id, stage id, stage file, title, description, and work level
  - `basis`: accepted `route_basis` summary
  - `variables`: the resolved variable map used for substitution and run-variable reporting
  - `documents`: one ordered list of expanded includes, library inputs, and artifact inputs, each with `kind`, `path`, `required`, `status`, and content when present
  - `outputs`: the declared output contract after substitution
  - `gating`: mode, `fail_on`, and notes
  - `stage_body`: optional stage body text when non-empty
- lock the public compiler-side names now so CLI and tests can plan against them:
  - `PipelineCompileRequest`
  - `PipelineCompileResult`
  - `RouteBasisSnapshot`
  - `PipelineCompileRefusal`
- Lane A sequencing:
  1. extend `pipeline.rs` with one compile-facing stage-front-matter loader
  2. extend `route_state.rs` with typed `route_basis` structs and the new schema version
  3. add fingerprint helpers for pipeline YAML, stage files, runner docs, and profile-pack files
  4. add `pipeline_compile.rs` with one public compile entrypoint that returns one typed compile result or one typed refusal
  5. implement eligibility checks before document expansion
  6. implement document expansion and renderer logic from the shared typed result

Lane B. CLI + Docs + Help Surface:

- exact file budget for Lane B:
  - `crates/cli/src/main.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
  - `crates/cli/tests/pipeline_proof_corpus_support.rs`
  - `crates/cli/tests/snapshots/system-help.txt`
  - `crates/cli/tests/snapshots/system-pipeline-help.txt`
  - `crates/cli/tests/snapshots/system-pipeline-compile-help.txt`
  - `README.md`
  - `docs/START_HERE.md`
  - `docs/README.md`
  - `docs/SUPPORTED_COMMANDS.md`
  - `docs/CLI_PRODUCT_VOCABULARY.md`
  - `docs/CLI_TONE_RULES.md`
  - `docs/CLI_OUTPUT_ANATOMY.md`
  - `docs/CLI_COMMAND_HIERARCHY.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `docs/contracts/pipeline-proof-corpus-and-docs-cutover.md`
- ship exactly two supported invocations in M2:
  - `system pipeline compile --id <pipeline-id> --stage <stage-id>`
  - `system pipeline compile --id <pipeline-id> --stage <stage-id> --explain`
- `--id` reuses the existing canonical-id and unambiguous shorthand rules from the `pipeline` family
- `--stage` accepts a canonical stage id or unambiguous shorthand stage id, still scoped to the selected pipeline
- `--explain` is a boolean proof-mode switch, not a second subcommand family
- do not add positional target syntax, raw path selectors, `--out`, `--format`, `--json`, runner/profile override flags, or compile-time route mutation flags in M2
- `crates/cli` stays thin:
  - add `PipelineCommand::Compile`
  - add one `PipelineCompileArgs` shape with exactly `--id`, `--stage`, and `--explain`
  - forward all compile targeting, payload rendering, explain rendering, and refusal classification to compiler-owned code
- help snapshot work is mandatory in the same slice:
  - update `system --help`
  - update `system pipeline --help`
  - add `system pipeline compile --help`
- success posture:
  - plain `pipeline compile` success is payload-only stdout, with no `OUTCOME`, no trust header, no route-basis recap, and no trailing next action
  - `pipeline compile --explain` success is proof-only stdout, not payload-plus-proof
  - explain output should use a stable proof ordering:
    - target
    - route basis used for freshness
    - selected stage metadata
    - include expansion
    - required vs optional input decisions
    - output contract
    - gating notes
    - payload summary only
- refusal posture:
  - `OUTCOME: REFUSED`
  - `PIPELINE: <pipeline-id>`
  - `STAGE: <stage-id>`
  - `REASON: <category and summary>`
  - `NEXT SAFE ACTION: <exact repair command>`
  - one refusal block naming category, summary, and broken subject in the repo's existing refusal language
- keep compile-specific proof out of `inspect`
- `inspect` remains the packet proof surface, not the compile proof surface
- update help/docs in one cut after the command is real:
  - `docs/SUPPORTED_COMMANDS.md`
  - `docs/CLI_PRODUCT_VOCABULARY.md`
  - `docs/CLI_TONE_RULES.md`
  - `docs/CLI_OUTPUT_ANATOMY.md`
  - `docs/CLI_COMMAND_HIERARCHY.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `docs/contracts/stage-compile-boundary-and-route-freshness.md`
- only after compiler behavior, proof corpus, snapshots, and docs land together should `pipeline compile` be described as supported

Lane C. Proof Corpus + Goldens + Tests:

- exact file budget for Lane C:
  - add `crates/compiler/tests/pipeline_compile.rs`
  - extend `crates/compiler/tests/pipeline_state_store.rs`
  - extend `crates/compiler/tests/pipeline_proof_corpus_support.rs`
  - extend `crates/cli/tests/cli_surface.rs`
  - extend `crates/cli/tests/help_drift_guard.rs`
  - extend `crates/cli/tests/pipeline_proof_corpus_support.rs`
  - extend `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/pipelines/foundation_inputs.yaml`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/stages/10_feature_spec.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/library/feature_spec/feature_spec_architect_directive.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/library/feature_spec/FEATURE_SPEC.md.tmpl`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/rules/p0_absolute.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/rules/p1_pragmatic.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/rules/traceability_policy.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/core/rules/evidence_policy.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/profiles/python-uv/commands.yaml`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/profiles/python-uv/conventions.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/base/BASE_CONTEXT.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/charter/CHARTER.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/project_context/PROJECT_CONTEXT.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/foundation/FOUNDATION_STRATEGY.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/foundation/TECH_ARCH_BRIEF.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/foundation/TEST_STRATEGY_BRIEF.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/foundation/QUALITY_GATES_SPEC.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/foundation/quality_gates.yaml`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/artifacts/foundation/ENVIRONMENT_INVENTORY.md`
  - add `tests/fixtures/pipeline_proof_corpus/foundation_inputs/state_seeds/malformed_route_basis.yaml`
  - add shared goldens under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/`
- keep one shared foundation-family proof corpus under `tests/fixtures/pipeline_proof_corpus/foundation_inputs`; do not create separate CLI-vs-compiler corpus trees
- extend the shared proof-corpus pipeline copy to append `stage.10_feature_spec` after `stage.07_foundation_pack`; the live repo pipeline declaration stays with Lane A
- add the stage file plus every compile-time dependency referenced by `stage.10_feature_spec` to the shared proof corpus with realistic committed content:
  - `core/stages/10_feature_spec.md`
  - `core/rules/p0_absolute.md`
  - `core/rules/p1_pragmatic.md`
  - `core/rules/traceability_policy.md`
  - `core/rules/evidence_policy.md`
  - `profiles/python-uv/profile.yaml`
  - `profiles/python-uv/commands.yaml`
  - `profiles/python-uv/conventions.md`
  - `runners/codex-cli.md`
  - `core/library/feature_spec/feature_spec_architect_directive.md`
  - `core/library/feature_spec/FEATURE_SPEC.md.tmpl`
- add realistic upstream artifact inputs to the shared corpus:
  - required baseline: `artifacts/base/BASE_CONTEXT.md` and `artifacts/charter/CHARTER.md`
  - richer optional baseline: `artifacts/project_context/PROJECT_CONTEXT.md` and the foundation-pack outputs referenced by `stage.10_feature_spec`
- keep optional-input coverage non-duplicative:
  - the committed corpus baseline should be the richer full-context case
  - optional-absence success cases should delete optional files from the temp installed copy during tests instead of creating a second nearly identical fixture tree
- route-basis fixture support:
  - create fresh happy-path `route_basis` snapshots by running `pipeline resolve` inside tests
  - keep only malformed-route-basis as a committed negative seed
  - extend path normalization helpers so compile goldens can replace temp repo paths and persisted route-basis paths with stable placeholders
- lock one shared success payload golden and one shared success explain golden under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens`:
  - `compile.stage_10_feature_spec.payload.full_context.txt`
  - `compile.stage_10_feature_spec.explain.full_context.txt`
- add shared refusal goldens in the same directory:
  - `compile.refused.missing_route_basis.txt`
  - `compile.refused.malformed_route_basis.txt`
  - `compile.refused.stale_route_basis.txt`
  - `compile.refused.inactive_stage.txt`
  - `compile.refused.stage_not_in_pipeline.txt`
  - `compile.refused.missing_required_artifact.txt`
- compiler and CLI tests must read those same golden files; do not create duplicate compiler-only or cli-only compile goldens
- mandatory regression coverage for M2:
  - missing persisted `route_basis` refusal
  - malformed persisted `route_basis` refusal
  - stale `route_basis` refusal after route-state mutation without a fresh resolve
  - inactive-stage refusal when `stage.10_feature_spec` is present in the pipeline but not active in the persisted route basis
  - selected-stage-not-in-pipeline refusal, rendered under the shipped `unsupported_target` contract
  - missing required artifact refusal for `stage.10_feature_spec`
  - optional-artifact absence success for `stage.10_feature_spec`
  - plain `pipeline compile` payload-only success
  - `pipeline compile --explain` proof-only success
  - one shared typed compile result proving payload and explain cannot drift semantically
- exact compiler tests to add in `crates/compiler/tests/pipeline_compile.rs`:
  - `compile_feature_spec_payload_matches_shared_golden`
  - `compile_feature_spec_explain_matches_shared_golden`
  - `compile_payload_and_explain_share_one_typed_result`
  - `compile_refuses_missing_route_basis`
  - `compile_refuses_malformed_route_basis`
  - `compile_refuses_stale_route_basis_after_route_state_mutation`
  - `compile_refuses_inactive_stage`
  - `compile_refuses_stage_not_declared_in_pipeline`
  - `compile_refuses_missing_required_artifact`
  - `compile_succeeds_when_optional_artifacts_are_absent`
- exact route-state tests to add in `crates/compiler/tests/pipeline_state_store.rs`:
  - `route_basis_round_trips_when_written_by_resolve`
  - `legacy_m1_state_without_route_basis_still_loads`
  - `malformed_route_basis_is_distinct_from_malformed_route_state`
- exact CLI tests to add in `crates/cli/tests/cli_surface.rs`:
  - `pipeline_compile_feature_spec_payload_matches_shared_golden`
  - `pipeline_compile_feature_spec_explain_matches_shared_golden`
  - `pipeline_compile_refuses_missing_route_basis`
  - `pipeline_compile_refuses_malformed_route_basis`
  - `pipeline_compile_refuses_stale_route_basis_after_state_set`
  - `pipeline_compile_refuses_inactive_stage`
  - `pipeline_compile_refuses_stage_not_in_pipeline` (asserting the existing `unsupported_target` surface)
  - `pipeline_compile_refuses_missing_required_artifact`
  - `pipeline_compile_allows_optional_artifacts_to_be_absent`
- split responsibilities cleanly:
  - compiler tests assert typed compile-result assembly, refusal classification, and shared-golden rendering from compiler-owned code
  - CLI tests assert selector parsing, exit codes, stdout/refusal rendering, help snapshots, and docs/help drift parity
- help/doc drift work:
  - update top-level help and pipeline help snapshots
  - add `system-pipeline-compile-help.txt`
  - extend help-drift parity checks so docs and snapshots both require `pipeline compile`, `pipeline compile --explain`, payload-only success wording, and the freshness recovery path
- determinism requirement:
  - fix `now_utc` to a stable injected value inside compile tests; do not normalize arbitrary wall-clock timestamps in goldens

Execution order and merge gates:

- Lane A starts first because Lane B needs a stable compile interface and Lane C needs the real refusal/output contract before final goldens can freeze
- Lane C fixture expansion may start in parallel with Lane A, but final compile goldens wait for the Lane A renderer contract and the Lane A live pipeline target wiring
- Lane B may prepare clap wiring early, but final snapshots and help/docs lock only after Lane A exposes the compile entrypoint and Lane C commits the shared examples/goldens
- final ship gate for M2 is one coherent green slice:
  - live pipeline declares `stage.10_feature_spec`
  - shared proof corpus declares `stage.10_feature_spec`
  - compiler tests pass
  - CLI tests pass
  - shared payload and explain goldens are committed
  - help snapshots match
  - docs/help parity checks match
  - help/docs describe `pipeline compile` as supported and no longer deferred

Lane completion checklists:

Lane A:

- [x] `crates/compiler` owns compile entrypoint, typed result, refusal logic, and both payload/explain renderers
- [x] `pipelines/foundation_inputs.yaml` declares `stage.10_feature_spec`
- [x] `route_state` persists one bounded `route_basis` snapshot written by `pipeline resolve`
- [x] `docs/contracts/pipeline-route-and-state-core.md` is updated with the new `route_basis` field set and schema version
- [x] compile can refuse missing, malformed, or stale `route_basis` without rerunning resolve
- [x] compile can refuse inactive stages using persisted route status and reason
- [x] required `stage.10_feature_spec` library/artifact inputs refuse on missing or empty content
- [x] optional foundation inputs remain non-fatal and visible in explain output

Lane B:

- [x] `pipeline compile` ships with exactly `--id`, `--stage`, and optional `--explain`
- [x] plain compile success is payload-only stdout
- [x] explain success is proof-only stdout
- [x] compile-specific proof stays out of `inspect`
- [x] help text, docs, and contracts all describe the same operator boundary

Lane C:

- [x] the shared proof corpus declares `stage.10_feature_spec`
- [x] the shared corpus contains every compile-time dependency for `stage.10_feature_spec`
- [x] shared payload and explain goldens are committed once and reused by both suites
- [x] stale-basis and inactive-stage regressions are locked as mandatory tests
- [x] help snapshots and docs/help drift checks cover the shipped compile surface

### M3. Output Materialization Capability

Status: complete

Post-ship note, 2026-04-15:

- `M3` shipped on `main` in commit `c5f3072` with compiler-owned capture planning, preview/apply cache flow, contract docs, shared goldens, and green compiler/CLI coverage.
- The remaining work is not “finish M3.” The remaining work is defining the next bounded milestone that makes a real end-to-end `foundation_inputs` flow possible.

Goal:

- recover writing behavior so the compiler owns both prompt generation and artifact emission

Reviewed scope lock, 2026-04-14:

- `M3` is intentionally narrowed to the writer surface, not the full end-to-end planning flow.
- The bounded proof targets are:
  - one single-file stage with a canonical repo-file mirror
  - one multi-file stage with exact `--- FILE:` parsing plus one canonical repo-file mirror
- The recommended proof stages for that boundary are:
  - `stage.05_charter_synthesize` for single-file artifact + required repo mirror (`CHARTER.md`)
  - `stage.07_foundation_pack` for multi-file artifact emission + repo mirror fallback (`ENVIRONMENT_INVENTORY.md`)
- `M3` must also cover the minimum post-capture state updates needed for re-entry:
  - convenience refs such as `charter_ref` and `project_context_ref`
  - deterministic route-relevant booleans such as `charter_gaps_detected`
- `M3` does not attempt to prove the whole `foundation_inputs` flow in one milestone. That remains `M4`.
- `M3` should reuse the existing boring writer rules from the legacy harness wherever they still fit:
  - exact `--- FILE: <path> ---` parsing
  - declared-output filtering
  - repo-file copy fallback by basename when the artifact copy exists and the repo mirror was not emitted directly
- `M3` must stay compatible with the current Rust compiler posture:
  - compiler-owned typed truth
  - CLI as a thin wrapper
  - explicit runtime state under `.system/state/`
  - no broadening of `generate` or `inspect` into stage-output writing surfaces
- reviewed command-shape decision:
  - `pipeline compile` stays the payload builder and proof boundary from `M2`
  - `pipeline capture` becomes the explicit stdin-driven writer surface for declared stage outputs
  - `generate` stays packet-only and does not absorb stage-output writing
  - `inspect` stays packet proof only and does not become a generic capture proof surface
- reviewed write-safety decision:
  - `pipeline capture` must be a two-phase operation, not legacy incremental writes
  - phase 1 parses stdin, validates exact declared outputs, resolves repo-mirror fallback candidates, and builds one compiler-owned materialization plan
  - phase 2 applies that plan with temp-file plus rename writes, then persists post-capture state only after every file write succeeds
  - partial write success is refusal, not an acceptable end state
  - the shipped rollback guarantee is scoped to `system`-coordinated single-writer flows; arbitrary concurrent external writers touching the same targets remain out of scope for `M3`
  - direct in-place overwrite is out of scope for the shipped M3 path except where a later contract explicitly justifies it
- reviewed freshness decision:
  - `pipeline capture` must consume the same persisted fresh `route_basis` contract as `pipeline compile`
  - capture must refuse when the selected stage is inactive, missing from the persisted route snapshot, or stale against current `routing` / `refs` / `run`
  - the recovery path for freshness failures remains explicit and boring: re-run `pipeline resolve`, then retry compile/capture
  - `pipeline capture` does not add a second compile-receipt or session-token mechanism in `M3`
- reviewed model-boundary decision:
  - `PipelineCompileResult` remains the compile payload/proof type
  - `pipeline capture` gets its own compiler-owned typed model for parsed stdin, validated write intents, repo-mirror fallback choices, and post-capture state mutations
  - CLI glue must not assemble capture semantics ad hoc from raw stdin plus compile metadata
  - capture reuses compile-owned declared outputs and freshness checks, but it does not collapse compile and write concerns into one overgrown result type
- reviewed post-capture-state decision:
  - `pipeline capture` auto-persists deterministic post-capture facts only:
    - convenience refs already in the reviewed runtime-state contract, such as `charter_ref` and `project_context_ref`
    - deterministic heuristics such as `charter_gaps_detected`
  - judgment variables such as `needs_project_context` stay explicit operator state and are updated through `pipeline state set`, not tty prompting inside capture
  - `M3` must remove the legacy hidden-tty prompt behavior from the supported Rust writer path
  - when a captured stage declares `sets:` values that still require human judgment, capture must surface the exact follow-up `pipeline state set` command as the next safe action instead of guessing

Must prove:

- Rust can capture single-file outputs
- Rust can capture multi-file `--- FILE:` outputs exactly
- Rust can write both artifact outputs and canonical repo-file outputs
- re-entry state survives enough to continue a multi-stage planning flow without manual bookkeeping
- preview and apply share one compiler-owned capture truth instead of reparsing or re-deciding writes in CLI glue
- the operator can preview a capture once, receive a stable `capture_id`, and apply that cached capture later without pasting the same model output twice
- stale or inactive route truth refuses before any writes happen
- write failure or state-persistence failure never leaves the repo in a silently partial-success state

Minimum acceptable wedge:

- one single-file planning stage plus required repo mirror:
  - `pipeline.foundation_inputs` + `stage.05_charter_synthesize`
- one multi-file planning stage plus repo mirror fallback:
  - `pipeline.foundation_inputs` + `stage.07_foundation_pack`
- one preview/apply cache flow:
  - `pipeline capture --preview`
  - `pipeline capture apply --capture-id <capture-id>`

What already exists and must be reused for M3:

- `crates/compiler/src/pipeline.rs` already loads stage front matter and owns ordered `outputs.artifacts` / `outputs.repo_files` metadata
- `crates/compiler/src/pipeline_compile.rs` already proves the selected-stage freshness and active-stage boundary against persisted `route_basis`
- `crates/compiler/src/repo_file_access.rs` already owns repo-relative path validation and symlink refusal for read-side file access
- `crates/compiler/src/repo_file_access.rs` should become the one writer-side path gate too, so M3 does not grow a second capture-local rule set for output-target validation
- `crates/compiler/src/route_state.rs` already owns advisory-lock, temp-file, rename, and runtime-state persistence patterns that M3 should reuse instead of inventing a second filesystem protocol
- the shared proof-corpus support under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/` already gives M3 one realistic repo fixture tree, one state-path convention, and one golden-normalization strategy

ASCII data-flow sketch:

```text
stdin
  |
  v
pipeline capture --id ... --stage ...
  |
  +--> freshness + stage-eligibility gate
  |      |
  |      +--> REFUSED: rerun `pipeline resolve`
  |
  +--> parse input contract
  |      |
  |      +--> single-file body
  |      +--> or exact FILE blocks
  |
  +--> build typed capture plan
         |
         +--> artifact write intents
         +--> repo mirror intents
         +--> deterministic state updates
         +--> preview renderer / cache entry
                    |
                    +--> --preview
                    |      |
                    |      +--> print preview + CAPTURE ID
                    |      +--> store runtime cache only
                    |
                    +--> direct apply or `capture apply --capture-id`
                           |
                           +--> stage temp files
                           +--> commit writes
                           +--> persist state updates
                           +--> clear cache on success
                           +--> rollback on failure
```

Command surface:

- direct apply:
  - `system pipeline capture --id <pipeline-id> --stage <stage-id>`
  - reads the model output from stdin
  - validates and applies the capture immediately when no preview flag is present
- preview:
  - `system pipeline capture --id <pipeline-id> --stage <stage-id> --preview`
  - reads stdin
  - validates and renders the capture preview
  - stores one runtime-only cached capture entry
  - prints `CAPTURE ID: <capture-id>`
  - prints `NEXT SAFE ACTION: run \`system pipeline capture apply --capture-id <capture-id>\``
- cached apply:
  - `system pipeline capture apply --capture-id <capture-id>`
  - consumes the cached preview entry instead of stdin
  - revalidates freshness and stage eligibility before writing anything
- flag posture:
  - `--preview` is the supported preview word; do not expose `--dry-run` in M3
  - do not add `--save` in M3; it is too easy to confuse “cache the preview” with “write the outputs”
  - `inspect` remains packet proof only, not capture proof

Preview/output anatomy:

- preview success must render in one stable order:
  - `OUTCOME: PREVIEW`
  - `PIPELINE`
  - `STAGE`
  - `CAPTURE ID`
  - `ROUTE BASIS REVISION`
  - `WRITE PLAN`
  - `POST-CAPTURE STATE UPDATES`
  - `NEXT SAFE ACTION`
- apply success must render in one stable order:
  - `OUTCOME: CAPTURED`
  - `PIPELINE`
  - `STAGE`
  - `WRITTEN FILES`
  - `STATE UPDATES`
  - `NEXT SAFE ACTION`
- refusal output must stay aligned with the reviewed CLI refusal posture:
  - `OUTCOME: REFUSED`
  - `PIPELINE`
  - `STAGE`
  - `REASON`
  - `NEXT SAFE ACTION`

Implementation checklist:

1. Lock the compiler-owned capture boundary.
   - Add one new compiler-owned module, `crates/compiler/src/pipeline_capture.rs`.
   - Keep CLI glue thin; `crates/cli` parses args and prints renderers, but does not parse FILE blocks or build write plans itself.
   - Reuse the compile-facing selected-stage freshness gate from `pipeline_compile.rs` rather than duplicating route-basis logic in CLI code.
2. Lock the typed capture model now.
   - Public compiler-owned names for M3:
     - `PipelineCaptureRequest`
     - `PipelineCapturePlan`
     - `PipelineCapturePreview`
     - `PipelineCaptureCacheEntry`
     - `PipelineCaptureApplyResult`
     - `PipelineCaptureRefusal`
   - `PipelineCapturePlan` must carry:
     - target pipeline/stage identity
     - accepted route-basis summary
     - normalized artifact contents in declared-output order
     - derived repo-mirror writes in declared-output order
     - deterministic post-capture state updates
     - the stable `capture_id`
   - `PipelineCaptureCacheEntry` must persist enough integrity material to prove that the cached bytes still match the advertised `capture_id`, either by storing the normalized content hash directly or by storing the exact fields required to recompute it losslessly.
   - `PipelineCapturePreview` and apply success output must both render from that same typed plan so preview/apply cannot drift semantically.
3. Define the input-parsing rules.
   - Single-artifact stages:
     - accept plain stdin body only
     - normalize LF newlines and one trailing newline
     - refuse `--- FILE:` wrappers for the supported single-file wedge
   - Multi-artifact stages:
     - require exact `--- FILE: <path> ---` wrappers
     - require every declared artifact output exactly once
     - refuse duplicate declared blocks
     - refuse undeclared blocks instead of silently ignoring them
   - Declared artifact order from stage front matter becomes the canonical render/apply order, regardless of the order in which blocks were pasted.
4. Define the repo-file rules for the M3 wedge.
   - In M3, repo files are compiler-owned mirrors, not independently authored stdin blocks.
   - Every artifact output target and repo-file mirror target must pass one compiler-owned write-side path validation step before preview succeeds or apply starts:
     - repo-relative targets only
     - no out-of-root absolute paths
     - no symlinked final targets
     - no symlinked parent-directory escapes
   - Reuse `repo_file_access` for this boundary instead of inventing a capture-only validator.
   - Single-file stage rule:
     - write the sole artifact output
     - mirror the same normalized content into any declared repo-file target
   - Multi-file stage rule:
     - repo mirror targets may be satisfied only by one exact basename match among the declared artifact outputs that were materialized in the same plan
     - if zero or multiple basename matches exist for a required repo-file target, refuse
   - This keeps `M3` narrow and covers the actual proof stages:
     - `CHARTER.md`
     - `ENVIRONMENT_INVENTORY.md`
   - Direct repo-file authorship from stdin is deferred until a later milestone proves a real need.
5. Define preview cache behavior.
   - Store runtime-only cached previews under `.system/state/pipeline/capture/<capture-id>.yaml`.
   - The cache entry must include:
     - pipeline id
     - stage id
     - accepted route-basis revision and freshness-critical fingerprints
     - normalized artifact contents
     - derived repo-mirror writes
     - deterministic state updates
   - `capture_id` should be deterministic from:
     - pipeline id
     - stage id
     - accepted route-basis revision / fingerprints
     - normalized capture contents
   - Repeated preview of the same content over the same fresh basis may reuse the same `capture_id`.
   - `capture apply --capture-id <capture-id>` must recompute the deterministic identity from the loaded cache entry and refuse if the on-disk cache contents no longer match the requested `capture_id`.
   - Successful `capture apply` deletes the cache entry.
   - Refused or failed `capture apply` keeps the cache entry so the operator can retry after repair.
   - M3 does not add cache GC, cache listing, or a `capture show` surface.
6. Define the write protocol.
   - Preview builds the full capture plan but performs no writes outside the runtime cache.
   - The shipped apply/rollback contract is for `system`-coordinated single-writer flows. Do not document or imply protection against arbitrary concurrent external writers modifying the same output paths during apply.
   - Apply must:
     - acquire the selected pipeline-state advisory lock before the final freshness check and keep it until either state persistence succeeds or rollback completes
     - validate that the current locked state revision and route-basis summary still match the previewed capture plan
     - snapshot the pre-write state of every target path (missing or prior bytes)
     - stage temp files for every target in the same parent directory as the final target
     - fsync staged temp files
     - commit writes in deterministic order:
       - artifact outputs first
       - repo mirrors second
     - persist state updates only after every file write succeeds
   - If any file commit fails after the first committed path:
     - rollback already-committed files to their snapped prior state
     - refuse loudly
     - do not persist post-capture state
   - If post-capture state persistence fails after file writes succeed:
     - rollback file writes to the snapped prior state
     - refuse loudly
     - keep the cached preview entry
   - If the locked state revision or route-basis summary no longer matches the cached plan at apply time:
     - refuse before any file write begins
     - keep the cached preview entry
   - Partial write success is not an acceptable steady state for the supported M3 path.
7. Define freshness/refusal behavior.
   - `pipeline capture` and `pipeline capture apply` must both require the same fresh persisted `route_basis` contract as `pipeline compile`.
   - They must refuse:
     - missing `route_basis`
     - malformed `route_basis`
     - stale `route_basis`
     - stage not present in the persisted route snapshot
     - inactive selected stage
   - The recovery path remains:
     - rerun `pipeline resolve`
     - rerun `pipeline compile` if the operator wants payload proof
     - rerun preview or direct capture after route truth is fresh again
8. Define deterministic post-capture state updates.
   - Auto-persist only deterministic facts:
     - `refs.charter_ref`
     - `refs.project_context_ref`
     - `routing.charter_gaps_detected` when the charter output contains the existing unknown-marker heuristic
   - Do not prompt on TTY inside the Rust writer flow.
   - Do not auto-persist `needs_project_context`.
   - When the captured stage declares `sets:` values that still require human judgment:
     - print a next-safe-action command in the form
       - `system pipeline state set --id <pipeline-id> --var needs_project_context=<true|false>`
9. Lock the CLI/help/docs surface for M3.
   - CLI file budget:
     - `crates/cli/src/main.rs`
     - `crates/cli/tests/cli_surface.rs`
     - `crates/cli/tests/help_drift_guard.rs`
     - `crates/cli/tests/pipeline_proof_corpus_support.rs`
     - snapshots:
       - `system-help.txt`
       - `system-pipeline-help.txt`
       - `system-pipeline-capture-help.txt`
       - `system-pipeline-capture-apply-help.txt`
   - Docs/help budget:
     - `README.md`
     - `docs/START_HERE.md`
     - `docs/README.md`
     - `docs/SUPPORTED_COMMANDS.md`
     - `docs/CLI_PRODUCT_VOCABULARY.md`
     - `docs/CLI_OUTPUT_ANATOMY.md`
     - `docs/CLI_COMMAND_HIERARCHY.md`
     - `DESIGN.md`
     - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
     - `docs/contracts/pipeline-route-and-state-core.md`
     - add one capture-specific contract that locks preview/apply semantics, cache posture, mirror rules, and rollback guarantees
10. Lock the proof corpus and tests before calling M3 done.
   - Add `crates/compiler/tests/pipeline_capture.rs`.
   - Extend `crates/compiler/tests/pipeline_state_store.rs` for capture-state interactions when needed.
   - Extend CLI surface/help tests rather than creating a second ad hoc CLI harness.
   - Reuse the shared proof corpus at `tests/fixtures/pipeline_proof_corpus/foundation_inputs/`; do not create a second corpus tree for capture.
   - Add committed goldens under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/`:
     - `capture.preview.stage_05_charter_synthesize.txt`
     - `capture.preview.stage_07_foundation_pack.txt`
     - `capture.apply.stage_05_charter_synthesize.txt`
     - `capture.apply.stage_07_foundation_pack.txt`
     - `capture.refused.single_file_with_file_wrapper.txt`
     - `capture.refused.missing_declared_block.txt`
     - `capture.refused.duplicate_declared_block.txt`
     - `capture.refused.undeclared_block.txt`
     - `capture.refused.stale_route_basis.txt`
     - `capture.refused.inactive_stage.txt`
     - `capture.refused.missing_capture_id.txt`
   - Mandatory compiler tests:
     - `capture_preview_charter_synthesize_builds_single_file_plan`
     - `capture_preview_foundation_pack_builds_multi_file_plan`
     - `capture_preview_and_apply_share_one_typed_plan`
     - `capture_preview_refuses_symlinked_or_out_of_root_write_target`
     - `capture_refuses_single_file_with_file_wrapper`
     - `capture_refuses_missing_declared_block`
     - `capture_refuses_duplicate_declared_block`
     - `capture_refuses_undeclared_block`
     - `capture_refuses_stale_route_basis`
     - `capture_refuses_inactive_stage`
     - `capture_apply_refuses_tampered_cached_preview`
     - `capture_apply_refuses_state_revision_conflict_before_writes`
     - `capture_apply_writes_charter_artifact_and_repo_mirror`
     - `capture_apply_writes_foundation_pack_and_repo_mirror_fallback`
     - `capture_apply_rolls_back_written_files_when_state_persistence_fails`
     - `capture_apply_keeps_cached_preview_on_refusal`
   - Mandatory CLI tests:
     - `pipeline_capture_preview_charter_matches_shared_golden`
     - `pipeline_capture_preview_foundation_pack_matches_shared_golden`
     - `pipeline_capture_apply_charter_matches_shared_golden`
     - `pipeline_capture_apply_foundation_pack_matches_shared_golden`
     - `pipeline_capture_apply_refuses_missing_capture_id`
     - `pipeline_capture_apply_refuses_tampered_cached_preview`
     - `pipeline_capture_preview_refuses_invalid_write_target`
     - `pipeline_capture_help_matches_snapshot`
     - `pipeline_capture_apply_help_matches_snapshot`

CODE PATH COVERAGE
===========================
[+] crates/compiler/src/pipeline_capture.rs
    │
    ├── parse single-file stdin
    │   ├── [GAP] exact body accepted
    │   └── [GAP] FILE wrapper refused
    │
    ├── parse multi-file stdin
    │   ├── [GAP] all declared blocks present
    │   ├── [GAP] missing declared block refused
    │   ├── [GAP] duplicate block refused
    │   └── [GAP] undeclared block refused
    │
    ├── build capture plan
    │   ├── [GAP] single-file repo mirror derived
    │   ├── [GAP] multi-file basename mirror derived
    │   ├── [GAP] ambiguous/missing repo mirror refused
    │   └── [GAP] invalid or symlinked write target refused
    │
    ├── preview cache
    │   ├── [GAP] preview writes runtime cache entry
    │   └── [GAP] apply reuses cache entry by capture_id
    │
    └── apply path
        ├── [GAP] fresh route basis required
        ├── [GAP] inactive stage refused
        ├── [GAP] tampered cached preview refused
        ├── [GAP] route-state revision conflict refused before writes
        ├── [GAP] writes committed in deterministic order
        ├── [GAP] rollback on file-write failure
        └── [GAP] rollback on state-persist failure

USER FLOW COVERAGE
===========================
[+] Operator preview flow
    │
    ├── [GAP] [→CLI] preview one charter capture and receive a capture_id
    └── [GAP] [→CLI] preview one foundation-pack capture and inspect the write plan

[+] Operator apply flow
    │
    ├── [GAP] [→CLI] direct-apply single-file charter capture from stdin
    ├── [GAP] [→CLI] preview once, then apply by capture_id without re-pasting stdin
    ├── [GAP] [→CLI] stale cached preview refuses after route-state mutation
    └── [GAP] [→CLI] tampered cached preview refuses before any writes

─────────────────────────────────
COVERAGE: 0/21 paths tested today for M3
  Code paths: 0/15
  User flows: 0/6
QUALITY: ★★★: 0  ★★: 0  ★: 0
GAPS: 21 paths need tests
─────────────────────────────────

Failure modes that M3 must name explicitly:

| Codepath | Real failure mode | Test required | Error handling required | User-visible posture |
| --- | --- | --- | --- | --- |
| single-file capture | model emits FILE wrapper for a single-file stage | yes | yes | explicit refusal |
| multi-file capture | one declared artifact block is missing | yes | yes | explicit refusal |
| multi-file capture | model emits an extra undeclared block | yes | yes | explicit refusal |
| repo mirror derivation | required repo mirror cannot be derived from artifacts | yes | yes | explicit refusal |
| preview/apply cache | cached preview exists but route basis is stale by apply time | yes | yes | explicit refusal with rerun resolve |
| preview/apply cache | cached preview yaml is edited or no longer matches the requested `capture_id` | yes | yes | explicit refusal before writes |
| write target validation | declared artifact or repo-mirror path escapes repo root or resolves through a symlink | yes | yes | explicit refusal |
| file commit | one target rename fails after another already committed | yes | yes | rollback + refusal |
| concurrent route-state mutation | another process changes pipeline state between preview and apply | yes | yes | explicit refusal before writes |
| state persistence | file writes succeed but runtime-state persistence fails | yes | yes | rollback + refusal |

Any failure mode above that silently succeeds is a release blocker for M3.

Lane split and merge gates:

| Lane | Scope | Modules / directories | Depends on |
| --- | --- | --- | --- |
| A | compiler capture core, cache entry, mirror rules, rollback semantics | `crates/compiler/src`, `docs/contracts/`, `.system/state/` runtime-shape docs | — |
| B | CLI surface, help, operator wording | `crates/cli/`, `README.md`, `docs/SUPPORTED_COMMANDS.md`, `docs/CLI_*`, `DESIGN.md` | A command/result shapes |
| C | proof corpus, goldens, compiler + CLI tests | `tests/fixtures/pipeline_proof_corpus/`, `crates/compiler/tests/`, `crates/cli/tests/` | A core semantics, B wording for final snapshots |

Execution order:

- Start Lane A first because preview/apply truth, cache posture, and rollback semantics must settle before goldens freeze.
- Lane C can prepare fixture updates and helper support in parallel with Lane A, but final goldens wait on the Lane A typed plan and Lane B wording.
- Lane B may wire Clap shapes early, but final help/docs lock only after Lane A and Lane C settle the shipped semantics.

NOT in scope for M3:

- whole-flow `foundation_inputs` replacement, which remains `M4`
- feature-spec output materialization
- generic repo-file authorship from stdin when no artifact mirror exists
- `pipeline run` or any compile-and-capture mega-command
- cache listing, cache GC, or a `pipeline capture show` surface
- packet generation or packet proof changes

### M3.5. Foundation Inputs Surface Completion

Status:

- complete on `main` in `8ac7aeb` (`feat: complete M3.5 foundation-inputs capture surface (#7)`)

Goal:

- make the shipped `pipeline` surfaces compose into one explicit `foundation_inputs` operator path from `stage.04_charter_inputs` through `stage.10_feature_spec`
- finish the missing writer and handoff boundaries so `M4` can prove a real end-to-end flow without inventing stage-output rules on the fly

Why this exists:

- `M3` proved the writer boundary for stages `05` and `07`, but the current shipped surface does not yet compose into a full `foundation_inputs` planning flow
- the missing boundary is concrete:
  - `stage.04_charter_inputs` produces `CHARTER_INPUTS.yaml` but has no shipped materialization path
  - `stage.06_project_context_interview` produces `PROJECT_CONTEXT.md` but has no shipped materialization path
  - `stage.10_feature_spec` compiles to stdout but does not yet materialize `FEATURE_SPEC.md`

Shipped product decision for `M3.5`:

- `pipeline capture` remains the only shipped writer surface
- `pipeline compile` stays payload-only / proof-only stdout, with no `--write` mode and no new `pipeline run` command
- `stage.04_charter_inputs`, `stage.06_project_context_interview`, and `stage.10_feature_spec` join the supported `pipeline capture` target set for `pipeline.foundation_inputs`
- `needs_project_context` remains an explicit operator-owned `pipeline state set` decision after `stage.05_charter_synthesize`
- the write-safety claim remains scoped to `system`-coordinated single-writer flows unless a stronger boundary actually ships in code and tests

In plain English:

- `M3` already proved that Rust can validate stage output, preview a write plan, apply it under the route-state lock, and roll back its own writes on failure
- `M3.5` does **not** invent a second writer model
- it extends the same capture boundary to the remaining output-producing stages that block the first real `foundation_inputs` flow
- the only human judgment handoff that stays manual is `needs_project_context`

Must prove:

- Rust can materialize or otherwise explicitly own the output boundary for every output-producing stage that blocks the first real `foundation_inputs` flow
- the plan names one boring operator path from `stage.04_charter_inputs` through `stage.10_feature_spec` without hidden manual jumps
- the `needs_project_context` decision boundary is explicit:
  - either still manual with one exact `pipeline state set` handoff
  - or promoted into a newly supported compiler-owned boundary with matching contracts and tests
- the write-safety claim stays exact and scoped to `system`-coordinated single-writer flows unless a stronger concurrency boundary actually ships

Minimum acceptable wedge:

- add the required stage-output surface for `stage.04_charter_inputs`
- add the required stage-output surface for `stage.06_project_context_interview`
- define and ship the supported materialization boundary for `stage.10_feature_spec`
- add shared proof-corpus coverage, CLI help/docs parity, and regression tests for the newly supported stage surfaces
- document the exact operator sequence for the first `foundation_inputs` path that `M4` will prove

Exact operator path `M3.5` must ship:

```text
pipeline resolve
  -> stage.04_charter_inputs capture
  -> stage.05_charter_synthesize capture
  -> operator sets needs_project_context exactly once
  -> pipeline resolve
  -> stage.06_project_context_interview capture (only if active)
  -> pipeline resolve
  -> stage.07_foundation_pack capture
  -> stage.10_feature_spec compile (optional explain proof)
  -> stage.10_feature_spec capture
```

Canonical command sequence:

```bash
system pipeline resolve --id pipeline.foundation_inputs

cat /tmp/CHARTER_INPUTS.yaml \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.04_charter_inputs

cat /tmp/CHARTER.md \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.05_charter_synthesize

system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>
system pipeline resolve --id pipeline.foundation_inputs

# Only when resolve marks stage.06_project_context_interview active:
cat /tmp/PROJECT_CONTEXT.md \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.06_project_context_interview

system pipeline resolve --id pipeline.foundation_inputs

cat /tmp/FOUNDATION_PACK.blocks.txt \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.07_foundation_pack

system pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec --explain
system pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec \
  | system pipeline capture --id pipeline.foundation_inputs --stage stage.10_feature_spec
```

`M3.5` architecture:

```text
fresh route truth
  -> route_state.rs persists route_basis

selected stage body on stdin
  -> pipeline_capture.rs validates supported stage + fresh route_basis
  -> single-file parser for stages 04 / 05 / 06 / 10
  -> multi-file parser for stage 07
  -> write plan from stage outputs.artifacts + outputs.repo_files
  -> repo-file mirrors derived by basename, same as M3
  -> deterministic state updates:
       stage 05 -> refs.charter_ref + routing.charter_gaps_detected
       stage 06 -> refs.project_context_ref
       all others -> no new automatic route-state mutation
  -> direct apply or preview/apply cache
  -> locked apply + rollback + next-safe-action rendering

stage.10 compile/capture handoff
  -> pipeline_compile.rs produces payload / explain to stdout
  -> payload body feeds pipeline_capture.rs
  -> capture materializes artifacts/feature_spec/FEATURE_SPEC.md
```

Implementation packet:

1. Compiler target expansion.
   - Extend the supported stage whitelist in [`crates/compiler/src/pipeline_capture.rs`](crates/compiler/src/pipeline_capture.rs) to include:
     - `stage.04_charter_inputs`
     - `stage.06_project_context_interview`
     - `stage.10_feature_spec`
   - Keep the current parser split:
     - single-file capture for `04`, `05`, `06`, and `10`
     - multi-file capture for `07`
   - Do **not** add a new writer abstraction, new command family, or stage-specific sidecar service.
   - Keep state-update derivation path-based and boring:
     - `artifacts/charter/CHARTER.md` drives `refs.charter_ref` plus `routing.charter_gaps_detected`
     - `artifacts/project_context/PROJECT_CONTEXT.md` drives `refs.project_context_ref`
     - `stage.04_charter_inputs` and `stage.10_feature_spec` add no automatic route-state updates

2. `needs_project_context` boundary cleanup.
   - Remove the current ambiguity by locking one exact posture:
     - capture does **not** auto-set `needs_project_context`
     - `stage.05_charter_synthesize` success returns the exact next-safe-action sequence:
       - `system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>`
       - `system pipeline resolve --id pipeline.foundation_inputs`
   - `stage.06_project_context_interview` remains conditional on the resolved route after that manual decision.

3. Stage-specific writer expectations.
   - `stage.04_charter_inputs`
     - accept plain YAML body only
     - write `artifacts/charter/CHARTER_INPUTS.yaml`
     - refuse FILE wrappers and empty body
   - `stage.06_project_context_interview`
     - accept plain markdown body only
     - write `artifacts/project_context/PROJECT_CONTEXT.md`
     - reuse the existing repo-mirror derivation rules for the declared repo-file output
     - persist `refs.project_context_ref`
   - `stage.10_feature_spec`
     - accept plain markdown body only
     - write `artifacts/feature_spec/FEATURE_SPEC.md`
     - remain compatible with the current `pipeline compile` payload-only stdout contract
     - do **not** imply canonical `.system/feature_spec/FEATURE_SPEC.md` promotion in this milestone

4. Proof corpus and regression coverage.
   - Extend the shared proof corpus under [`tests/fixtures/pipeline_proof_corpus/foundation_inputs/`](tests/fixtures/pipeline_proof_corpus/foundation_inputs/) rather than creating a second corpus tree.
   - Add capture success goldens for:
     - `capture.preview.stage_04_charter_inputs.txt`
     - `capture.apply.stage_04_charter_inputs.txt`
     - `capture.preview.stage_06_project_context_interview.txt`
     - `capture.apply.stage_06_project_context_interview.txt`
     - `capture.preview.stage_10_feature_spec.txt`
     - `capture.apply.stage_10_feature_spec.txt`
   - Add compiler / CLI regression tests proving:
     - single-file wrapper refusal for `04`, `06`, and `10`
     - empty-body refusal for `04`, `06`, and `10`
     - `stage.06_project_context_interview` writes the expected artifact path and project-context ref update
     - `stage.10_feature_spec` capture works with the real compile payload handoff, not only with a synthetic markdown body
     - route progression across `04 -> 05 -> state set -> resolve -> 06? -> 07 -> 10`

5. Docs, contracts, and wording parity.
   - Update:
     - [`docs/contracts/pipeline-capture-preview-and-apply.md`](docs/contracts/pipeline-capture-preview-and-apply.md)
     - [`docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`](docs/contracts/C-02-rust-workspace-and-cli-command-surface.md)
     - [`docs/contracts/pipeline-proof-corpus-and-docs-cutover.md`](docs/contracts/pipeline-proof-corpus-and-docs-cutover.md)
     - [`docs/START_HERE.md`](docs/START_HERE.md)
     - [`docs/SUPPORTED_COMMANDS.md`](docs/SUPPORTED_COMMANDS.md)
     - [`docs/CLI_PRODUCT_VOCABULARY.md`](docs/CLI_PRODUCT_VOCABULARY.md)
     - [`docs/CLI_OUTPUT_ANATOMY.md`](docs/CLI_OUTPUT_ANATOMY.md) if any example output or supported-target wording changes
   - Keep the transactionality wording exact:
     - `system`-coordinated single-writer apply is guaranteed
     - arbitrary external concurrent writers are not covered by `M3.5`

File touch expectation:

- compiler:
  - `crates/compiler/src/pipeline_capture.rs`
- compiler tests:
  - `crates/compiler/tests/pipeline_capture.rs`
  - `crates/compiler/tests/support/pipeline_proof_corpus_support.rs`
- CLI tests:
  - `crates/cli/tests/cli_surface.rs`
  - help snapshot files only if wording changes
- proof corpus:
  - `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/*`
- docs / contracts:
  - the exact list above

Test diagram:

| Flow | Command path | Coverage required |
| --- | --- | --- |
| Stage 04 capture | stdin -> `pipeline capture stage.04_charter_inputs` | preview/apply success, FILE-wrapper refusal, empty-body refusal |
| Stage 05 handoff | stdin -> `pipeline capture stage.05_charter_synthesize` -> `state set` -> `resolve` | existing goldens stay green, next-safe-action remains exact |
| Stage 06 capture | stdin -> `pipeline capture stage.06_project_context_interview` | preview/apply success, repo-mirror derivation, `refs.project_context_ref` persistence |
| Stage 07 capture | stdin -> `pipeline capture stage.07_foundation_pack` | existing success/refusal/rollback coverage stays green |
| Stage 10 handoff | `pipeline compile stage.10_feature_spec` -> stdout -> `pipeline capture stage.10_feature_spec` | compile-to-capture integration test, preview/apply success, wrapper refusal, empty-body refusal |
| Route freshness | stale or inactive route basis during any supported capture target | refusal before writes, same recovery path as M3 |

Biggest hidden risks to close during implementation:

- **Artifact-vs-canonical confusion.** `stage.10_feature_spec` capture writes `artifacts/feature_spec/FEATURE_SPEC.md`, not `.system/feature_spec/FEATURE_SPEC.md`. If docs blur those, later sessions will treat stage output like canonical input.
- **Manual-decision drift.** If any contract or help text implies capture “covers” `needs_project_context`, operators will skip the required `pipeline state set` step and route truth will drift.
- **Synthetic handoff proof.** If tests prove stage 10 capture only with hand-authored markdown, the documented compile-to-capture path will still be unproven.
- **Overclaimed transactionality.** The current apply lock protects the compiler-owned route-state boundary, not arbitrary external concurrent writers touching the same output paths.

NOT in scope for `M3.5`:

- expanding `pipeline compile` to new supported stages beyond the already-shipped `stage.10_feature_spec`
- auto-deciding `needs_project_context`
- adding `pipeline run`, `pipeline compile --write`, or any other combined orchestration command
- promoting captured `FEATURE_SPEC.md` or `PROJECT_CONTEXT.md` directly into canonical `.system/` artifacts
- strengthening apply safety beyond `system`-coordinated single-writer flows
- proving downstream consumer trust or operator-outcome reduction, which remains `M4` / `M5`

Exit criteria:

- one exact `foundation_inputs` stage sequence is documented and testable without inventing new writer rules mid-session
- `pipeline capture` support for `04`, `06`, and `10` is shipped, documented, and covered by shared proof-corpus goldens plus compiler / CLI regression tests
- `stage.10_feature_spec` has one proven compile-to-capture materialization path
- `needs_project_context` is documented exactly once as a manual operator decision followed by `pipeline resolve`
- stages `04`, `06`, and `10` no longer represent undocumented holes in the `foundation_inputs` path
- the plan, contracts, docs, and tests all agree on the supported `M3.5` boundary

Historical correction, 2026-04-15:

- the original M3.5 wording treated direct `pipeline compile ... | pipeline capture ...` as the stage-10 materialization path
- the shipped code actually supports `pipeline compile` payload generation plus `pipeline capture` of a completed single-file body
- `M4` owns the docs/tests/contract correction for that boundary; if any M3.5 wording below or above implies direct raw compile stdout capture is valid, treat that as superseded by the M4 section

### M4. Foundation Journey Proof And Handoff Contract

Status:

- complete on `main` in `efdaf42` (`feat: prove M4 foundation journey and stage-10 handoff (#8)`)

Post-ship note, 2026-04-16:

- `M4` shipped the realistic `foundation_flow_demo` corpus, happy-path and skip-path CLI journey proofs, the structural `FEATURE_SPEC.md` contract checker, deterministic evidence-bundle regression coverage, and the docs/contract cutover that removes the false direct `compile | capture` stage-10 story.
- the bounded stage-10 handoff contract is now the active repo truth: `pipeline compile` produces payload, an external model produces the completed `FEATURE_SPEC.md`, and `pipeline capture` materializes that completed single-file body.
- post-merge follow-up `6a43779` (`fix: avoid pipefail false positive in install smoke (#9)`) corrected the macOS install-smoke shell assertion so the shipped M4 surface stays green on `main`.

Goal:

- prove one realistic `pipeline.foundation_inputs` journey from `stage.04_charter_inputs` through `stage.10_feature_spec` using the shipped Rust CLI boundaries plus the explicit human-in-the-loop model step the code actually requires
- produce one contract-valid `artifacts/feature_spec/FEATURE_SPEC.md` handoff package with deterministic evidence, without claiming real downstream consumer adoption yet
- collapse the active M4 truth into this section so later sessions do not have to reconstruct it from review appendices

Why this exists now:

- `M3.5` already shipped the missing writer boundary for stages `04`, `06`, and `10`
- the current docs, tests, and milestone prose still overstate the stage-10 handoff by treating `pipeline compile` output like a completed `FEATURE_SPEC.md`
- the repo now needs one honest journey-proof milestone before `M5` can claim that downstream consumers actually trust and adopt the generated artifacts

Premise lock:

- `pipeline compile --stage stage.10_feature_spec` emits a compile payload, not a completed feature spec body
- `pipeline capture --stage stage.10_feature_spec` writes any non-empty single-file body verbatim to `artifacts/feature_spec/FEATURE_SPEC.md`
- therefore the valid M4 stage-10 path is:
  1. `pipeline compile` produces the stage payload
  2. an operator or model runner uses that payload and produces the completed `FEATURE_SPEC.md`
  3. `pipeline capture` materializes that completed markdown body
- direct `pipeline compile ... | pipeline capture ...` is not valid end-to-end proof and must be removed from plan/docs/contracts/tests during this milestone
- the `needs_project_context=false` branch is only a real skip-path proof when both `needs_project_context=false` and `charter_gaps_detected=false`; the demo fixture must make that route truth explicit instead of relying on accidental stage-05 content

Exact user outcome:

- an operator can run one believable `foundation_inputs` journey on a realistic demo corpus and finish with the full artifact chain:
  - `artifacts/charter/CHARTER_INPUTS.yaml`
  - `artifacts/charter/CHARTER.md`
  - `artifacts/project_context/PROJECT_CONTEXT.md` on the primary branch
  - `artifacts/foundation/*`
  - `artifacts/feature_spec/FEATURE_SPEC.md`
- the operator still owns exactly two manual boundaries:
  - deciding `needs_project_context`
  - supplying stage-local model output bodies
- `M4` proves journey correctness, handoff-contract correctness, and deterministic rerun evidence
- `M4` does **not** claim that operator tax is already paid back in real workflows, and it does **not** claim that a live downstream consumer already starts from artifact outputs; those claims remain `M5`

What already exists:

- route truth and persisted `route_basis` already live in [`crates/compiler/src/route_state.rs`](crates/compiler/src/route_state.rs)
- compile payload / explain proof for `stage.10_feature_spec` already ship in [`crates/compiler/src/pipeline_compile.rs`](crates/compiler/src/pipeline_compile.rs)
- stage-output materialization for `04`, `05`, `06`, `07`, and `10` already ships in [`crates/compiler/src/pipeline_capture.rs`](crates/compiler/src/pipeline_capture.rs)
- the CLI already exposes the bounded `pipeline` family in [`crates/cli/src/main.rs`](crates/cli/src/main.rs)
- shared proof-corpus and capture goldens already exist under [`tests/fixtures/pipeline_proof_corpus/foundation_inputs/`](tests/fixtures/pipeline_proof_corpus/foundation_inputs/)
- the CLI suite already proves most route progression in [`crates/cli/tests/cli_surface.rs`](crates/cli/tests/cli_surface.rs), but the stage-10 handoff is still modeled incorrectly as compile stdout flowing directly into capture

Must prove:

- the exact shipped CLI path works on one believable demo corpus, not only on the narrow contract proof corpus
- the primary happy path uses `needs_project_context=true` and exercises the optional stage-06 branch explicitly
- the secondary skip path uses `needs_project_context=false` **and** charter content that keeps `charter_gaps_detected=false`, so stage 06 is skipped for the right reason
- the stage-10 journey is exercised through the real boundary:
  - `system pipeline compile --id pipeline.foundation_inputs --stage stage.10_feature_spec`
  - external model execution against that payload
  - `system pipeline capture --id pipeline.foundation_inputs --stage stage.10_feature_spec`
- the generated `FEATURE_SPEC.md` passes one bounded contract checker tied to [`core/library/feature_spec/feature_spec_architect_directive.md`](core/library/feature_spec/feature_spec_architect_directive.md) and [`core/library/feature_spec/FEATURE_SPEC.md.tmpl`](core/library/feature_spec/FEATURE_SPEC.md.tmpl)
- docs, tests, contracts, and this plan all agree on the same stopping point:
  - `M4` proves one complete journey plus one contract-valid handoff artifact
  - `M5` proves actual downstream consumer adoption from that artifact set

### Architecture Review

Architecture ASCII diagram:

```text
tests/fixtures/foundation_flow_demo/
  -> realistic repo-local canonical inputs
  -> committed stage outputs for happy-path and skip-path model responses

happy path
  -> system pipeline resolve
  -> capture stage.04_charter_inputs
  -> capture stage.05_charter_synthesize
  -> system pipeline state set needs_project_context=true
  -> system pipeline resolve
  -> capture stage.06_project_context_interview
  -> system pipeline resolve
  -> capture stage.07_foundation_pack
  -> system pipeline compile stage.10_feature_spec
  -> external model response fixture (completed FEATURE_SPEC.md)
  -> system pipeline capture stage.10_feature_spec
  -> feature-spec contract checker
  -> evidence bundle + operator-journey doc

skip path
  -> same through stage.05
  -> state set needs_project_context=false
  -> charter_gaps_detected remains false by fixture design
  -> resolve skips stage.06
  -> capture stage.07
  -> compile stage.10
  -> external model response fixture
  -> capture stage.10
  -> same contract checker + evidence bundle
```

Opinionated architecture decisions:

- keep `pipeline compile` and `pipeline capture` as separate verbs
- do not add a new writer command, orchestration command, or live model-runner surface in `M4`
- keep the realistic demo corpus separate from the shared proof corpus so contract tests stay small and the journey proof can be more believable
- keep the feature-spec checker test-only; it is evidence for M4, not a new product surface
- docs/help/contract wording correction is part of the milestone, not optional cleanup, because the current stage-10 narrative is wrong

### Code Quality Review

Minimum-diff implementation posture:

- reuse existing compile and capture modules rather than introducing a new stage-10 handoff abstraction
- keep test support boring:
  - one installable demo fixture tree under `tests/fixtures/foundation_flow_demo/`
  - optional small helper support in existing CLI test support
  - no second route-state engine, no second capture parser, no ad hoc shell wrapper crate
- correct the docs/contracts/help wording in the same milestone that adds the proof, so the repo stops teaching a false compile-to-capture story
- anchor the contract checker to the existing feature-spec directive/template rather than inventing fuzzy “consumer-readiness” heuristics

File / module touch expectation:

| Area | Expected modules |
| --- | --- |
| Demo fixture | `tests/fixtures/foundation_flow_demo/` |
| CLI journey proof | `crates/cli/tests/cli_surface.rs`, optional CLI test support helpers |
| Docs / contract wording | `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, `docs/CLI_OPERATOR_JOURNEY.md`, `docs/contracts/pipeline-proof-corpus-and-docs-cutover.md`, `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md` |
| Plan / milestone truth | `PLAN.md` |

### Test Review

CODE PATH COVERAGE
===========================
[+] foundation journey proof
    │
    ├── [GAP] [→CLI] resolve -> 04 capture -> 05 capture -> state set(true) -> 06 -> 07
    │               -> compile stage.10 -> external response -> capture stage.10
    │               Needs one exact happy-path integration using the demo fixture.
    │
    ├── [GAP] [→CLI] resolve -> 04 capture -> 05 capture -> state set(false) -> resolve skips 06
    │               -> 07 -> compile stage.10 -> external response -> capture stage.10
    │               Must prove skip-path truth with `charter_gaps_detected=false`.
    │
    ├── [GAP] [→CLI] stage-10 boundary contract
    │               compile payload is not captured directly; only completed model output is.
    │
    └── [GAP] [→CLI] deterministic rerun evidence
                    fixed `now_utc`, normalized capture ids, stable fixture outputs.

HANDOFF CONTRACT COVERAGE
===========================
[+] generated FEATURE_SPEC.md
    │
    ├── [GAP] [→TEST] Required sections from directive/template are present
    ├── [GAP] [→TEST] Goals map to acceptance criteria
    ├── [GAP] [→TEST] At least one alternative is documented
    ├── [GAP] [→TEST] Security / performance / reliability NFR rows are present or explicit N/A
    └── [GAP] [→TEST] Integration touchpoints are named or explicitly scoped TBD

DOC / CONTRACT PARITY
===========================
[+] operator journey wording
    │
    ├── [GAP] [→TEST] Docs stop claiming direct `compile | capture` as valid stage-10 materialization
    └── [GAP] [→TEST] M4 stopping point is described consistently as journey proof + handoff contract, not downstream adoption

─────────────────────────────────
COVERAGE: 0/9 gaps closed in plan text alone
  Journey paths: 0/4
  Handoff contract checks: 0/5
QUALITY TARGET: every gap closed before M4 is marked complete
─────────────────────────────────

Required test artifacts:

1. One happy-path CLI integration in [`crates/cli/tests/cli_surface.rs`](crates/cli/tests/cli_surface.rs) that uses the demo fixture, fixed `now_utc`, explicit external stage-10 response input, and final artifact assertions.
2. One skip-path CLI integration in the same suite that proves stage 06 stays skipped because the fixture keeps `charter_gaps_detected=false`.
3. One contract-check test that validates the captured `FEATURE_SPEC.md` against the required sections and traceability rules from the directive/template.
4. One docs/help drift assertion that fails if the repo reintroduces direct `compile | capture` wording for stage 10.
5. One evidence normalization path for capture ids and clock-driven values so the rerun artifact stays deterministic.

### Performance Review

Performance / determinism requirements:

- pin `PIPELINE_COMPILE_NOW_UTC_ENV_VAR` in journey tests and evidence generation
- normalize volatile `capture_id` values anywhere they are compared or stored as proof
- keep the demo fixture repo-local and static; do not require live model execution, network access, or external services in automated tests
- keep the feature-spec contract checker structural and bounded; do not attempt semantic grading or expensive diffing against multiple ideal outputs
- do not duplicate the full proof corpus into the demo fixture; the new fixture should prove journey realism, not replace contract regression coverage

### Failure Modes Registry

| Failure mode | Severity | Test required | Why it matters |
| --- | --- | --- | --- |
| stage-10 compile payload is captured directly as `FEATURE_SPEC.md` | Critical | yes | This would prove a semantically invalid operator path and preserve false docs. |
| skip path accidentally activates stage 06 because charter output still contains `TBD` / `TODO` / `UNKNOWN` markers | Critical | yes | The plan would appear deterministic while actually depending on accidental content. |
| docs and contracts continue to teach direct `compile | capture` wording | High | yes | Later sessions will keep implementing against the wrong boundary. |
| evidence bundle flakes because `now_utc` or `capture_id` changes between runs | High | yes | The proof stops being rerunnable and loses value as milestone evidence. |
| feature-spec checker is vague prose instead of template/directive-backed assertions | High | yes | “Consumer-readiness” becomes unreviewable vibes instead of a contract. |

### Required Deliverables

1. A dedicated realistic demo corpus under `tests/fixtures/foundation_flow_demo/`.
2. One exact operator-journey document that names the happy path, the skip path, the manual branch decision, the stage-10 external-model boundary, and the stopping point.
3. One happy-path CLI integration test that proves the real M4 journey without piping raw compile payload directly into capture.
4. One skip-path CLI integration test that proves stage 06 stays skipped only because both route predicates say so.
5. One feature-spec contract checker in tests, anchored to the current directive/template.
6. One deterministic evidence bundle for the demo journey, including clock and capture-id normalization rules.
7. Docs / contract parity updates that remove direct `compile | capture` wording and replace it with the real stage-10 boundary.
8. One small journey scorecard capturing manual decisions, model-output boundaries, repo rereads avoided, and what still remains manual going into `M5`.

### NOT In Scope

- `pipeline run`, `pipeline compile --write`, or any orchestration mega-command
- auto-deciding `needs_project_context`
- live model execution inside the Rust CLI
- promoting captured outputs directly into canonical `.system/` artifacts
- real downstream consumer adoption, consumer rewiring, or seam execution integration
- public distribution, UI/MCP wrapper work, or onboarding-chat work

### Worktree Parallelization Strategy

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Correct stage-10 boundary wording in plan/docs/contracts | `PLAN.md`, `docs/`, `docs/contracts/` | — |
| B. Add demo corpus and fixture helpers | `tests/fixtures/`, CLI test support | — |
| C. Add happy-path and skip-path CLI journey tests | `crates/cli/tests/` | A, B |
| D. Add feature-spec contract checker + evidence normalization | `crates/cli/tests/`, optional shared test support | B |
| E. Publish operator-journey artifact and scorecard | `docs/`, `tests/fixtures/` | A, B, C, D |

Parallel lanes:

- Lane A: A
- Lane B: B
- Lane C: C after A + B
- Lane D: D after B
- Lane E: E after A + B + C + D

Execution order:

- Launch A and B in parallel.
- Launch D once B lands.
- Launch C after A and B land.
- Finish with E once the proof and checker outputs exist.

Conflict flags:

- C and D both touch CLI test surfaces, so they should merge sequentially or share one owner.

### Completion Summary

| Section | Status | Notes |
| --- | --- | --- |
| Scope challenge | complete | M4 is now pinned to journey proof + handoff contract, not downstream adoption. |
| Architecture review | complete | Stage-10 model boundary is explicit and docs correction is part of scope. |
| Code quality review | complete | Reuse current compile/capture modules, add only boring fixture/test support. |
| Test review | complete | Nine concrete gaps identified, all tied to exact journey or handoff assertions. |
| Performance review | complete | Determinism requirements for `now_utc` and `capture_id` are locked. |
| NOT in scope | written | six items |
| What already exists | written | current route, compile, capture, CLI, and proof-corpus surfaces mapped |

Prerequisite status:

- satisfied on `main` by `8ac7aeb`, which shipped the explicit `04` / `06` / `10` capture boundary
- satisfied on `main` by `efdaf42`, which shipped the stage-10 journey proof, truthful docs/tests/contracts, and the bounded handoff artifact evidence needed before `M5`

### M5. Downstream Consumer Adoption

Status:

- next active milestone after shipped `M4` on `main`
- this is the active implementation plan for proving real downstream adoption, not just handoff plausibility

Goal:

- prove one named downstream planning consumer, `feature-slice-decomposer`, can complete one real feature-to-slice planning job from compiler outputs without reopening repo truth outside an explicit fallback contract
- add one compiler-owned, versioned downstream handoff bundle plus manifest in a clearly non-canonical zone so downstream trust has an explicit authority model
- capture before / after adoption evidence that shows fewer repo rereads, smaller grounding, fewer manual patch-ups, and a bounded trust path

Why this exists now:

- `M4` ended honestly at a derived handoff package, not at canonical promotion:
  - `artifacts/charter/CHARTER.md`
  - optional `artifacts/project_context/PROJECT_CONTEXT.md`
  - `artifacts/foundation/*`
  - `artifacts/feature_spec/FEATURE_SPEC.md`
- the current planning packet and manifest contracts still treat only repo-local `.system/*` as canonical project truth
- raw artifact reuse is not proof of downstream adoption, because it can hide repo rereads or accidental canonical promotion

### Step 0. Scope Challenge

Exact user outcome:

- after the shipped `M4` flow, the operator runs one explicit downstream handoff emission step:
  - `system pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer`
- that step emits one non-canonical bundle under `artifacts/handoff/feature_slice/<feature-id>/`
- the bundle is the only admissible downstream basis for the named consumer and contains:
  - a versioned `handoff_manifest.json`
  - a `trust_matrix.md`
  - a `read_allowlist.json`
  - `scorecard/*`
  - fingerprints for the canonical `.system/*` inputs the bundle was derived from
  - fingerprints for the derived `artifacts/foundation/*` and `artifacts/feature_spec/FEATURE_SPEC.md` inputs the consumer may read
  - `route_basis` fingerprint and revision, compile payload hash, directive or template version, producer version, and explicit manual-boundary disclosures
  - the exact allowed read set plus explicit fallback conditions
- the named consumer or repo-local harness may read only that emitted bundle unless an explicit fallback condition is declared and logged, and produces one concrete downstream planning output:
  - `artifacts/planning/feature_slice/<feature-id>/SLICE_PLAN.md`
- the repo captures one before / after scorecard for that same job:
  - repo files reopened
  - grounding bytes and sections passed to the consumer
  - manual patch-ups required
  - elapsed operator steps

Premise lock:

- `.system/*` remains the only canonical project-truth input set under [`docs/contracts/C-03-canonical-artifact-manifest-contract.md`](docs/contracts/C-03-canonical-artifact-manifest-contract.md); `M5` does **not** broaden canonical inputs
- `artifacts/*` remain derived; `M5` does **not** silently promote captured stage outputs into canonical `.system/*`
- stage-10 `artifacts/feature_spec/FEATURE_SPEC.md` remains `external_manual_derived`, not compiler-derived, and the downstream consumer must see that trust class explicitly
- `M5` succeeds only when the named consumer finishes one real planning job from the emitted handoff bundle and explicit fallback rules; “consumer can start from artifacts” is not enough
- the named consumer may read only the emitted bundle unless an explicit fallback condition is declared and logged
- if the consumer reopens charter, project context, foundation, or feature-spec repo files outside the declared bundle, `M5` fails unless the fallback condition is explicitly allowed and recorded

Assumption ledger:

| Premise | Status | What disproves it | Handling |
| --- | --- | --- | --- |
| Current canonical docs are already good enough to support downstream adoption | assumed | the named consumer still falls back because the basis bundle is missing decisions or conflicts | stop and run a canonical-input quality audit before widening compiler surfaces |
| `feature-slice-decomposer` can consume compiler outputs with a handoff change rather than a repo-reread change | must prove | the consumer still needs charter/project context/foundation repo archaeology outside the bundle | `M5` fails; do not claim adoption |
| Downstream trust can be established without changing canonical `.system/*` authority | accepted with contract work | the only working path requires silent canonical promotion or raw artifact-as-truth behavior | treat that as a separate plan change, not hidden implementation detail |
| Operator-pain reduction is measurable on one real job | accepted | the team cannot show concrete before / after read-set and manual-step reduction | keep expansion blocked until the scorecard exists |

Implementation alternatives:

| Approach | Effort | Risk | Why it is or is not the `M5` choice |
| --- | --- | --- | --- |
| Let the consumer read raw `artifacts/*` directly and call that adoption | S | Critical | Rejected. It collides with the canonical-input contract and makes trust informal. |
| Promote `artifacts/*` into canonical `.system/*` as part of stage-10 capture | M | Critical | Rejected. It hides a new authority boundary inside the wrong milestone and breaks the explicit `M4` contract. |
| Emit one explicit downstream handoff bundle plus manifest, then run one named consumer from that bundle only | M | Medium | Recommended. It keeps `.system/*` canonical while giving the consumer one explicit derived trust surface. |
| Implement the full downstream seam and slice ecosystem inside this repo | L | High | Deferred. That widens the repo far beyond the next proof we need. |

Implementation slices:

1. Lock the derived handoff contract, trust classes, fallback rules, and success criteria in one contract doc and in this milestone.
2. Add the handoff emitter surface and compiler-owned manifest builder without changing canonical `.system/*` rules.
3. Add one named-consumer harness that can only read the emitted bundle and that writes one `SLICE_PLAN.md` proof output.
4. Add the refusal corpus for stale canonical inputs, tampered derived inputs, missing provenance, trust-class mismatch, and undeclared repo rereads.
5. Add the before / after scorecard plus docs and vocabulary parity so the trust story is exact everywhere the operator will read it.

### What Already Exists

- route truth, `route_basis`, and stage-output materialization already exist in:
  - [`crates/compiler/src/route_state.rs`](crates/compiler/src/route_state.rs)
  - [`crates/compiler/src/pipeline_capture.rs`](crates/compiler/src/pipeline_capture.rs)
  - [`crates/compiler/src/pipeline_compile.rs`](crates/compiler/src/pipeline_compile.rs)
- canonical manifest and freshness truth already exist for `.system/*` in:
  - [`crates/compiler/src/canonical_artifacts.rs`](crates/compiler/src/canonical_artifacts.rs)
  - [`crates/compiler/src/artifact_manifest.rs`](crates/compiler/src/artifact_manifest.rs)
  - [`docs/contracts/C-03-canonical-artifact-manifest-contract.md`](docs/contracts/C-03-canonical-artifact-manifest-contract.md)
- resolver decision logs already surface `C-03` provenance for canonical packet flows in [`crates/compiler/src/resolver.rs`](crates/compiler/src/resolver.rs)
- the shipped planning packet surfaces still read only canonical `.system/*`; this is correct today and must remain true unless a later contract changes it
- `M4` already ships the realistic journey proof corpus and evidence bundle under [`tests/fixtures/foundation_flow_demo/`](tests/fixtures/foundation_flow_demo/) and [`docs/CLI_OPERATOR_JOURNEY.md`](docs/CLI_OPERATOR_JOURNEY.md)
- what does **not** exist yet is:
  - a derived downstream handoff contract
  - a named consumer harness with bundle-only read rules
  - a trust-class model for external/manual stage-10 output
  - a concrete before / after adoption scorecard

### Architecture Review

Architecture ASCII diagram:

```text
canonical truth
  .system/charter/CHARTER.md
  .system/project_context/PROJECT_CONTEXT.md?
  .system/feature_spec/FEATURE_SPEC.md?   [canonical boundary unchanged]
          │
          ├── existing C-03 manifest + freshness
          └── existing resolver provenance
          │
          v
M4 derived outputs
  artifacts/charter/CHARTER.md
  artifacts/project_context/PROJECT_CONTEXT.md?
  artifacts/foundation/*
  artifacts/feature_spec/FEATURE_SPEC.md   [external_manual_derived]
          │
          v
new M5 handoff emitter
  system pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer
          │
          v
artifacts/handoff/feature_slice/<feature-id>/
  handoff_manifest.json
  trust_matrix.md
  read_allowlist.json
  scorecard/
          │
          v
feature-slice-decomposer harness
  - reads bundle only
  - refuses on stale, tampered, missing, or misclassified inputs
          │
          v
artifacts/planning/feature_slice/<feature-id>/SLICE_PLAN.md
```

Opinionated architecture decisions:

- add one explicit handoff emission surface under `pipeline`; do not overload `generate` or `inspect`
- the handoff bundle is derived, versioned, and consumer-specific; it is not a shadow canonical zone
- the bundle manifest, not raw `artifacts/*` paths, is the thing the consumer trusts
- trust must be file-by-file and explicit:
  - `canonical`
  - `compiler_derived`
  - `external_manual_derived`
- consumer fallback must refuse first and only allow repo rereads when the fallback condition is explicitly declared and logged
- do not build a multi-consumer framework in `M5`; one allowlisted consumer is enough to prove the thesis

Concrete module boundaries:

- compiler ownership:
  - manifest assembly
  - provenance and fingerprint checks
  - trust-class encoding
  - bundle directory layout
- CLI ownership:
  - argument parsing
  - refusal rendering
  - operator-facing proof output
- harness ownership:
  - bundle-only read enforcement
  - acceptance output generation
  - before / after scorecard capture

Implementation file budget:

- prefer one new compiler module such as `crates/compiler/src/pipeline_handoff.rs`
- keep CLI wiring thin in [`crates/cli/src/main.rs`](crates/cli/src/main.rs) or the current command-surface split if that file is already too dense
- add one contract doc for the derived handoff bundle rather than scattering the boundary across multiple prose files
- if a second compiler module is needed, use it only for manifest schema or renderer separation, not for a generic handoff framework

ASCII comments that should land with implementation:

- `crates/compiler/src/pipeline_handoff.rs`, include a short directory-layout and provenance-flow diagram
- consumer harness test support, include a short allowed-read versus refusal-path diagram if the fixture rules become non-obvious

### Code Quality Review

Minimum-diff posture:

- reuse existing manifest, provenance, route-basis, and capture metadata instead of inventing a second freshness engine
- keep the first consumer path explicit and allowlisted, not plugin-driven
- keep trust-class decisions data-driven in the manifest, not spread across hardcoded branch logic in multiple modules
- keep bundle writing one-way: emit derived output, do not mutate canonical `.system/*`

Expected touch surfaces:

| Area | Expected modules |
| --- | --- |
| Handoff bundle builder | `crates/compiler/src/`, `crates/cli/src/` |
| Handoff contract | `docs/contracts/` |
| Consumer harness + scorecard | `tests/fixtures/`, `crates/cli/tests/`, optional shared test support |
| Docs / vocabulary parity | `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, `docs/CLI_PRODUCT_VOCABULARY.md`, `docs/CLI_OPERATOR_JOURNEY.md`, `PLAN.md` |

Overbuild traps to reject:

- a generic consumer adapter registry
- a second manifest system that duplicates `C-03`
- background mutation or auto-heal behavior when provenance mismatches
- any implementation that requires bundle consumers to know repo internals to succeed

### Test Review

```text
CONSUMER ADOPTION COVERAGE
===========================
[+] emit handoff bundle
    │
    ├── [GAP] [→CLI] happy path emit from real `M4` fixture
    ├── [GAP] [→CLI/HARNESS] consumer reads bundle only and writes
    │                       `artifacts/planning/feature_slice/<feature-id>/SLICE_PLAN.md`
    └── [GAP] [→ARTIFACT] before/after scorecard captures read-set, grounding size,
                          manual patch-ups, and operator steps

TRUST / PROVENANCE COVERAGE
===========================
[+] bundle truth
    │
    ├── [GAP] [→TEST] stale canonical-basis refusal after `.system/*` mutation
    ├── [GAP] [→TEST] tampered derived-artifact refusal after `artifacts/*` mutation
    ├── [GAP] [→TEST] missing or corrupt provenance refusal
    ├── [GAP] [→TEST] trust-class mismatch refusal
    └── [GAP] [→TEST] consumer-assumption mismatch refusal instead of silent repo reread

CANONICAL BOUNDARY COVERAGE
===========================
[+] existing planning packet truth
    │
    ├── [GAP] [→TEST] `generate` and `inspect` still treat only `.system/*` as canonical
    └── [GAP] [→TEST] handoff emission does not silently create or mutate canonical `.system/*`

─────────────────────────────────
COVERAGE: 0/8 gaps closed in plan text alone
  Consumer adoption: 0/3
  Trust / provenance: 0/5
QUALITY TARGET: every gap closed before `M5` is marked complete
─────────────────────────────────
```

Required test artifacts:

1. One happy-path bundle-emission plus consumer-adoption proof using a real `M4` fixture.
2. One stale-canonical-input refusal test.
3. One tampered-derived-artifact refusal test.
4. One missing-provenance refusal test.
5. One trust-class mismatch refusal test.
6. One consumer-assumption mismatch refusal test.
7. One canonical-boundary regression test for existing planning packet flows.
8. One committed before / after scorecard artifact with exact read-set and grounding-size deltas.

Required assertions:

- the consumer reads only the allowlisted bundle paths on the happy path
- every refusal class is distinct and points to the next safe action
- `external_manual_derived` stays explicit all the way through bundle emission and consumer validation
- existing packet flows stay green without learning anything about downstream handoff bundles

### Performance Review

Performance and determinism rules:

- bundle emission must hash only the declared canonical and derived inputs, not the whole repo
- proof runs stay offline and fixture-backed; no live model execution or network access in automated tests
- handoff manifests and scorecards must be deterministic across reruns
- the consumer harness must log allowed reads and fallbacks in stable order so the before / after comparison stays reviewable
- refuse on stale or mismatched provenance before the consumer does expensive work

Expected hot paths:

- manifest assembly over a bounded read set
- fingerprint verification over already-declared files
- harness read logging over one allowlisted bundle tree

Performance smells that fail review:

- full-repo scans during emit
- recomputing unrelated compiler state to validate one bundle
- nondeterministic scorecard ordering that makes adoption evidence noisy

### Failure Modes Registry

| Failure mode | Severity | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- | --- |
| raw `artifacts/*` are treated like canonical runtime inputs | Critical | yes | hard refusal | explicit contract-boundary refusal |
| stage-10 `FEATURE_SPEC.md` is mislabeled as compiler-derived | Critical | yes | hard refusal | explicit trust-class mismatch |
| downstream consumer silently reopens repo files outside the bundle | Critical | yes | hard refusal plus read-log evidence | explicit undeclared-read refusal |
| bundle provenance is missing, stale, or tampered but consumer proceeds | Critical | yes | hard refusal before planning starts | explicit provenance refusal |
| scorecard cherry-picks a friendly baseline instead of the actual old flow | High | yes | deterministic baseline definition in fixtures | explicit scorecard provenance note |
| bundle emission mutates canonical `.system/*` | High | yes | hard refusal or test failure | explicit canonical-mutation regression |

Critical gaps:

- any path that has no test, no refusal, and no operator-visible error is a release blocker for `M5`
- silent fallback to repo rereads is a release blocker even if the happy path looks good

### Required Deliverables

1. One explicit derived handoff contract in `docs/contracts/`.
2. One explicit `pipeline handoff emit` surface or equivalent narrow compiler-owned emission step.
3. One versioned handoff bundle manifest with trust classes, fingerprints, provenance, and fallback rules.
4. One repo-local harness for `feature-slice-decomposer` if the real consumer stays off-repo.
5. One accepted `SLICE_PLAN.md` happy-path proof artifact.
6. One refusal corpus for stale, tampered, missing-provenance, and trust-class mismatch cases.
7. One before / after grounding scorecard and transcript bundle.
8. Docs and vocabulary parity updates that keep canonical truth, derived trust, and manual boundaries exact.

### NOT In Scope

- broadening the canonical `.system/*` input contract
- silently promoting `artifacts/*` into canonical `.system/*`
- building a multi-consumer plugin system
- wiring the full seam, slice, or execution ecosystem inside this repo
- live model execution inside the Rust CLI
- UI or MCP wrapper work
- broad generic command provenance beyond what the `M5` bundle needs

### Worktree Parallelization Strategy

Dependency table:

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Lock the derived handoff contract, trust classes, and success criteria | `docs/contracts/`, `PLAN.md` | — |
| B. Add bundle emission surface and compiler manifest builder | `crates/compiler/`, `crates/cli/` | A |
| C. Add named-consumer harness and happy-path proof | `tests/fixtures/`, `crates/cli/tests/` | A, B |
| D. Add stale, tampered, provenance, and canonical-boundary refusal corpus | `tests/fixtures/`, `crates/cli/tests/`, optional compiler tests | A, B |
| E. Publish scorecard plus docs and vocabulary cutover | `docs/`, `PLAN.md`, `tests/fixtures/` | A, B, C, D |

Parallel lanes:

- Lane A: A
- Lane B: B
- Lane C: C
- Lane D: D
- Lane E: E

Execution order:

- launch Lane A first and merge it before code starts
- launch Lane B next
- once Lane B freezes the manifest shape and CLI wording, launch Lane C and Lane D in parallel worktrees
- merge Lane C and Lane D, then finish Lane E as the final cutover pass

Conflict flags:

- Lanes C and D both touch `tests/fixtures/` and `crates/cli/tests/`, so parallel work is valid only if fixture directories or test files are pre-split; otherwise run them sequentially inside one lane
- Lane E touches `PLAN.md` and docs already touched by Lane A, so do not start Lane E early

### Exit Criteria

- one named consumer is proved, not merely suggested
- the derived handoff bundle has one explicit authority model and one explicit trust-class model
- the happy path produces one accepted downstream planning output from bundle-only inputs
- stale, tampered, missing-provenance, and undeclared-read cases refuse explicitly
- canonical `.system/*` authority stays unchanged for existing planning packet flows
- the before / after scorecard shows concrete repo-reread and grounding-size reduction for the same job
- docs, contracts, tests, and milestone prose all agree on the same boundary

### Completion Summary

- Step 0: Scope Challenge, scope accepted as-is and narrowed to one named consumer plus one derived handoff contract
- Architecture Review: 0 open architecture questions, 6 architecture decisions locked
- Code Quality Review: 4 code-structure guardrails locked, no generic framework work admitted
- Test Review: diagram produced, 8 required coverage gaps enumerated
- Performance Review: 5 determinism and hot-path rules locked
- NOT in scope: written
- What already exists: written
- TODOS.md updates: none required, existing deferred items remain valid
- Failure modes: 4 release-blocking critical gaps identified
- Outside voice: historical review inputs already incorporated, no new external review run in this rewrite
- Parallelization: 5 lanes total, 2 lanes parallelizable after compiler surface freeze, 3 sequential gates
- Lake Score: complete option preserved across boundary, trust, proof, and refusal design

### M6. Rust Front Door Completion And Historical Cleanup

Status:

- next active milestone after shipped `M5` on `main`
- this is the active implementation plan for replacing placeholder-only `setup` with one real Rust-owned front door and removing the split setup story from shipped docs, help, and contracts

Goal:

- ship one honest `setup` family:
  - `system setup`
  - `system setup init`
  - `system setup refresh`
- make canonical `.system/` bootstrap and later truth re-establishment boring, explicit, and safe
- make `generate`, `inspect`, `doctor`, help text, contracts, and docs all describe the same front-door story
- demote historical and guided-setup wording from active product contract into explicit historical reference only

Why this exists now:

- `M5` proved the downstream handoff story, but the top-level CLI still lies about the first thing an operator should do
- the current binary still hard-fails `setup` as a placeholder in [`crates/cli/src/main.rs`](crates/cli/src/main.rs)
- `generate`, `inspect`, and `doctor` already depend on canonical `.system/*` truth and already surface missing-root or missing-artifact refusals through the compiler
- the biggest remaining product gap is no longer pipeline depth, it is the fake front door

### Step 0. Scope Challenge

Exact user outcome:

- on a new repo, the operator runs `system setup`
  - the CLI auto-routes to `system setup init`
  - `init` creates the canonical `.system/` scaffold plus starter files for:
    - `.system/charter/CHARTER.md`
    - `.system/feature_spec/FEATURE_SPEC.md`
    - `.system/project_context/PROJECT_CONTEXT.md`
  - the success output lists created paths, marks `PROJECT_CONTEXT.md` optional, tells the operator to fill the starter files, and ends with one explicit next command: `system doctor`
- on a repo with an existing canonical `.system/` root, the operator runs `system setup`
  - the CLI auto-routes to `system setup refresh`
  - `refresh` preserves canonical files by default, repairs missing scaffold pieces, and only rewrites setup-owned files or resets runtime state when the operator explicitly asks for it
  - the accepted explicit flags are:
    - `--rewrite`
    - `--reset-state`
  - the success output ends with one explicit next command: `system doctor`
- on any repo state, `generate`, `inspect`, and `doctor` point back to the same `setup` family rather than a placeholder or implied external flow

Premise lock:

- `setup` stays the durable family name
- the `pipeline`, `generate`, `inspect`, and `doctor` surfaces stay intact
- `.system/*` remains split into canonical artifact zones plus explicit non-canonical runtime zones
- `setup init` is scaffold-first in `M6`; it does not import, infer, or migrate project truth from ambient repo evidence
- `setup refresh` is preserve-by-default; destructive behavior requires explicit flags
- runtime `.system/state/**` remains non-canonical and must not decide canonical truth
- cleanup work only counts if it supports the honest front door, the canonical `.system/` contract, or the historical-reference boundary

What existing code already solves each sub-problem:

- canonical artifact paths and required-input rules already exist in [`crates/compiler/src/canonical_artifacts.rs`](crates/compiler/src/canonical_artifacts.rs)
- missing-root and missing-artifact refusal categories, blocker summaries, and next-safe-action rendering already exist in:
  - [`crates/compiler/src/resolver.rs`](crates/compiler/src/resolver.rs)
  - [`docs/contracts/C-04-resolver-result-and-doctor-blockers.md`](docs/contracts/C-04-resolver-result-and-doctor-blockers.md)
- repo-safe path validation, symlink refusal, atomic file writes, and parent-directory creation already exist in [`crates/compiler/src/repo_file_access.rs`](crates/compiler/src/repo_file_access.rs)
- bounded state-directory creation patterns already exist in [`crates/compiler/src/route_state.rs`](crates/compiler/src/route_state.rs)
- help-order and command-surface drift guards already exist in:
  - [`crates/cli/tests/cli_surface.rs`](crates/cli/tests/cli_surface.rs)
  - [`crates/cli/tests/help_drift_guard.rs`](crates/cli/tests/help_drift_guard.rs)
- docs and contract parity discipline already exists; `M6` needs to update it, not invent it

Minimum change set:

1. replace the placeholder `Setup` branch in the CLI with real `setup` subcommands plus bare-command auto-routing
2. add one compiler-owned setup module that plans and executes init/refresh work using the existing repo-relative write helpers
3. add starter-template constants or fixture-backed template sources for the three canonical setup-created files
4. rewire refusal copy and `doctor` guidance so missing canonical truth routes to `setup`
5. update docs, contracts, help snapshots, and proof corpus together

Complexity check:

- this milestone will touch more than 8 files because docs, contracts, help snapshots, and tests all need parity updates
- that file count is acceptable only because the code path itself stays small:
  - one new compiler-owned setup module
  - thin CLI wiring
  - no new service layer
  - no new storage system
- if implementation grows beyond one new compiler module plus optional shared test support, stop and reduce scope

Search check:

- [Layer 1] reuse Clap nested subcommands for `setup init` and `setup refresh`; do not build a custom command parser
- [Layer 1] reuse `repo_file_access` for atomic repo-relative writes and symlink-safe directory traversal; do not add a second file-mutation framework
- [Layer 1] reuse existing refusal and blocker rendering for `doctor`, `generate`, and `inspect`; do not introduce parallel recovery wording logic
- [Layer 3] no new cache, daemon, migration engine, or setup manifest database is justified here

TODOS cross-reference:

- the existing TODO for canonical `.system/` bootstrap flow is absorbed by `M6`
- richer post-setup onboarding stays deferred to [TODOS.md](TODOS.md)
- public distribution, CLI release workflow, pipeline validation, and structured run provenance remain deferred and are not prerequisites for the front door

Completeness check:

- the complete version of `M6` includes init, refresh, bare-command auto-routing, starter templates, refusal cutover, `doctor` cutover, docs/help/contracts parity, and proof corpus coverage
- the shortcut version would ship only `setup init` or docs-only cleanup and leave the rest of the story split
- reject the shortcut; the complete version is still a boilable lake because the repo already has the refusal, write-safety, and parity infrastructure

Distribution check:

- `M6` does not introduce a new artifact type
- no new build or publish pipeline is required for this milestone beyond the existing Rust workspace and test rails
- distribution remains explicitly out of scope

Implementation alternatives:

| Approach | Effort | Risk | Why it is or is not the `M6` choice |
| --- | --- | --- | --- |
| Keep `setup` placeholder-only and clean up docs copy around it | S | Critical | Rejected. It preserves the main product lie. |
| Ship `setup init` only and defer `refresh` plus auto-routing | S | High | Rejected. It would keep the repo-state story split and force a second front-door revision soon after. |
| Ship one bounded `setup` family with `init`, `refresh`, and bare-command routing | M | Medium | Recommended. It solves the actual user-facing gap without widening into onboarding automation. |
| Add guided onboarding, repo inference, or migration logic in the same milestone | L | High | Rejected. It spends innovation tokens on the wrong thing. |

### What Already Exists

- the CLI already exposes the right top-level noun surface in [`crates/cli/src/main.rs`](crates/cli/src/main.rs), but `setup` still hard-fails as a placeholder
- canonical artifact truth already assumes the exact paths `M6` needs to bootstrap:
  - `.system/charter/CHARTER.md`
  - `.system/project_context/PROJECT_CONTEXT.md`
  - `.system/feature_spec/FEATURE_SPEC.md`
- resolver and blocker logic already distinguishes:
  - missing `.system` root
  - non-directory `.system` root
  - symlinked `.system` root
  - missing required canonical artifact
  - empty required canonical artifact
- the repo already has proven patterns for safe repo mutation in:
  - [`crates/compiler/src/repo_file_access.rs`](crates/compiler/src/repo_file_access.rs)
  - [`crates/compiler/src/pipeline_capture.rs`](crates/compiler/src/pipeline_capture.rs)
  - [`crates/compiler/src/pipeline_handoff.rs`](crates/compiler/src/pipeline_handoff.rs)
- the docs already encode the exact semantic debt `M6` must erase:
  - [`README.md`](README.md)
  - [`docs/START_HERE.md`](docs/START_HERE.md)
  - [`docs/SUPPORTED_COMMANDS.md`](docs/SUPPORTED_COMMANDS.md)
  - [`docs/CLI_PRODUCT_VOCABULARY.md`](docs/CLI_PRODUCT_VOCABULARY.md)
  - [`docs/CLI_COMMAND_HIERARCHY.md`](docs/CLI_COMMAND_HIERARCHY.md)
  - [`DESIGN.md`](DESIGN.md)
  - [`docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`](docs/contracts/C-02-rust-workspace-and-cli-command-surface.md)
  - [`docs/contracts/C-01-approved-repo-surface.md`](docs/contracts/C-01-approved-repo-surface.md)

### Architecture Review

Architecture ASCII diagram:

```text
operator
  │
  └── system setup
        │
        ├── repo discovery
        │     └── existing managed-root rules
        │
        ├── canonical setup-surface detector
        │     ├── .system missing / invalid -> route to init
        │     └── .system present as canonical root -> route to refresh
        │
        ├── setup init
        │     ├── create .system/
        │     ├── create canonical namespace dirs
        │     ├── write starter templates
        │     ├── create minimal runtime state scaffold if needed
        │     └── print created paths + next steps + `system doctor`
        │
        └── setup refresh
              ├── inspect existing scaffold
              ├── preserve canonical files by default
              ├── repair missing setup-owned scaffold pieces
              ├── rewrite setup-owned files only with --rewrite
              ├── prune/reset .system/state only with --reset-state
              └── print actions taken + next step `system doctor`

steady-state commands
  generate / inspect / doctor
        │
        └── shared resolver truth
              └── next-safe-action now routes to `setup`
```

Opinionated architecture decisions:

- keep setup execution compiler-owned, not CLI-owned
- keep the CLI layer thin:
  - parse args
  - choose subcommand
  - render operator output
- reuse the existing canonical-artifact contract instead of creating a second bootstrap contract source
- reuse the existing repo-relative write helpers instead of inventing setup-specific file I/O
- treat starter templates as static compiler-owned content, not runtime-generated prose blobs
- keep auto-routing boring:
  - if canonical `.system` root is missing or invalid, route to `init`
  - once the canonical `.system` root exists as a valid directory, route to `refresh`
  - detailed missing-file repair belongs to `refresh`, because that is the only path that can safely preserve partial existing truth
- keep `setup refresh` exact:
  - default path preserves existing canonical files
  - `--rewrite` only rewrites setup-owned starter files
  - `--reset-state` only targets runtime `.system/state/**`
- do not add a hidden bootstrap database, lockfile, or marker file just to prove setup happened

Concrete module boundaries:

- compiler ownership:
  - repo-state detection for setup routing
  - init/refresh planning
  - template bodies
  - write execution and safety checks
  - typed outcome model for created, preserved, rewritten, skipped, and refused paths
- CLI ownership:
  - `setup` command parsing
  - short routing line for bare `system setup`
  - success and refusal rendering
- resolver ownership:
  - continue to own missing-root and missing-artifact truth
  - update next-safe-action wording to point at real `setup`

Implementation file budget:

- prefer one new compiler module such as `crates/compiler/src/setup.rs`
- keep CLI changes bounded to [`crates/cli/src/main.rs`](crates/cli/src/main.rs) unless that file becomes unreadable enough to justify a later split
- update existing contracts rather than creating several new setup contract documents
- one setup-focused test fixture helper is acceptable if existing proof support becomes awkward

ASCII comments that should land with implementation:

- `crates/compiler/src/setup.rs`, include a short scaffold / refresh decision diagram
- if `doctor` ready or refusal rendering gets more complex, add one short next-safe-action flow comment near the renderer or setup outcome mapper

### Code Quality Review

Minimum-diff posture:

- add one compiler-owned setup seam, not a setup framework
- keep starter templates explicit and reviewable in source control
- avoid generic templating or content-composition abstractions for three files
- keep destructive behavior flag-bound and local
- keep setup-owned file lists explicit in code; do not infer them from directory scans

Expected touch surfaces:

| Area | Expected modules |
| --- | --- |
| Setup execution core | `crates/compiler/src/`, optionally one new `setup.rs` |
| CLI wiring and render surface | `crates/cli/src/`, help snapshots |
| Resolver and next-safe-action wording | `crates/compiler/src/resolver.rs`, `crates/compiler/src/rendering/*` |
| Docs and contracts parity | `README.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, `docs/CLI_PRODUCT_VOCABULARY.md`, `docs/CLI_COMMAND_HIERARCHY.md`, `DESIGN.md`, `docs/contracts/C-01-*`, `docs/contracts/C-02-*`, `docs/contracts/C-04-*`, `PLAN.md` |
| Proof corpus and regressions | `crates/cli/tests/`, `crates/compiler/tests/`, committed snapshots or setup fixture support |

Overbuild traps to reject:

- a generic onboarding engine
- repo inference or import of starter content from ambient files
- a second mutation helper stack separate from `repo_file_access`
- auto-heal behavior inside `doctor` instead of explicit `setup` execution
- a general refresh planner that scans arbitrary repo content looking for "staleness"

### Test Review

```text
SETUP SURFACE COVERAGE
======================
[+] bare `system setup`
    │
    ├── [GAP] [->CLI] missing .system routes to `setup init`
    ├── [GAP] [->CLI] valid .system root routes to `setup refresh`
    └── [GAP] [->CLI] routing line is short, exact, and deterministic

INIT COVERAGE
=============
[+] `system setup init`
    │
    ├── [GAP] [->CLI/COMPILER] creates .system scaffold and three starter files
    ├── [GAP] [->TEST] `PROJECT_CONTEXT.md` template is marked optional
    ├── [GAP] [->TEST] refuses when .system already exists as a valid canonical root
    ├── [GAP] [->TEST] refuses symlinked or invalid target paths
    └── [GAP] [->SNAPSHOT] success output lists created paths and ends with `system doctor`

REFRESH COVERAGE
================
[+] `system setup refresh`
    │
    ├── [GAP] [->CLI/COMPILER] preserves existing canonical files by default
    ├── [GAP] [->TEST] repairs missing scaffold pieces without rewriting preserved files
    ├── [GAP] [->TEST] `--rewrite` rewrites only setup-owned starter files
    ├── [GAP] [->TEST] `--reset-state` prunes runtime .system/state only
    ├── [GAP] [->TEST] refuses when canonical .system root is missing or invalid
    └── [GAP] [->SNAPSHOT] success output reports preserved / created / rewritten / reset paths exactly

RECOVERY AND DOCS CUTOVER COVERAGE
==================================
[+] existing product surfaces
    │
    ├── [GAP] [->TEST] `generate` missing-root refusal points to `system setup`
    ├── [GAP] [->TEST] `inspect` missing-root refusal points to `system setup`
    ├── [GAP] [->TEST] `doctor` blocker guidance points to `system setup`
    ├── [GAP] [->SNAPSHOT] top-level help no longer says `setup` is placeholder-only
    └── [GAP] [->DOCS] README / START_HERE / vocabulary / hierarchy / contracts agree on the new setup story

---------------------------------
COVERAGE: 0/19 gaps closed in plan text alone
  Setup surface: 0/3
  Init: 0/5
  Refresh: 0/6
  Recovery/docs cutover: 0/5
QUALITY TARGET: every gap above closes before `M6` is marked complete
---------------------------------
```

Required test artifacts:

1. one uninitialized-repo happy path for bare `system setup` routing to `init`
2. one initialized-repo happy path for bare `system setup` routing to `refresh`
3. one `setup init` success snapshot covering scaffold creation and next-step checklist
4. one `setup init` refusal snapshot for already-initialized repo
5. one `setup refresh` default-preserve success snapshot
6. one `setup refresh --rewrite` success snapshot proving setup-owned rewrite only
7. one `setup refresh --reset-state` success snapshot proving runtime-state-only reset
8. one symlink / invalid-path refusal test for setup mutations
9. one regression test proving `generate`, `inspect`, and `doctor` now point to `setup`
10. one help drift update and doc parity pass proving placeholder-only wording is gone from shipped surfaces

Required assertions:

- starter templates are written only to approved repo-relative paths
- no setup mutation crosses the repo root or follows symlinks
- `PROJECT_CONTEXT.md` remains optional in wording, not in path existence
- `refresh` default path preserves canonical file bodies byte-for-byte
- `--rewrite` does not mutate non-setup-owned files
- `--reset-state` does not mutate canonical artifact files
- success output stays compact and deterministic
- `doctor` continues to aggregate blockers from shared resolver truth rather than recomputing setup logic independently

### Performance Review

Performance and determinism rules:

- setup detection should inspect only the bounded canonical `.system` surface, not scan the full repo
- template writes must use the existing repo-relative atomic write helpers
- success output ordering must be deterministic so committed snapshots stay reviewable
- default refresh should compute a bounded action plan over setup-owned paths only
- no new persisted cache, setup history log, or background refresh mechanism is justified

Expected hot paths:

- repo-root discovery
- a handful of `symlink_metadata` checks under `.system`
- three starter-template writes on `init`
- one bounded preserve / create / rewrite / reset plan on `refresh`

Performance smells that fail review:

- full-repo scans to decide `init` versus `refresh`
- rewriting canonical files on every `refresh` run even when no flag requests it
- non-deterministic output or path ordering in success rendering
- extra filesystem state invented only to remember that setup happened

### Failure Modes Registry

| Failure mode | Severity | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- | --- |
| bare `setup` still dead-ends or looks like a placeholder | Critical | yes | hard regression failure | explicit supported setup surface |
| `init` overwrites existing canonical files without an explicit operator choice | Critical | yes | hard refusal | explicit preserve-or-refresh guidance |
| `refresh --reset-state` mutates canonical `.system/*` files | Critical | yes | hard refusal or failed test | explicit runtime-state-only reset guarantee |
| setup writes through a symlinked parent or escaped path | Critical | yes | hard refusal | explicit path-safety refusal |
| `generate`, `inspect`, or `doctor` keep pointing at a non-existent guided path | High | yes | regression failure | explicit `system setup` next-safe-action |
| docs and help claim setup is real while the binary still behaves differently | High | yes | doc/help drift failure | exact docs/help parity |
| refresh silently repairs too much and hides destructive behavior | High | yes | explicit action report | visible preserve / rewrite / reset summary |

Critical gaps:

- any path with no test, no refusal, and no explicit operator-visible outcome is a release blocker for `M6`
- any silent canonical rewrite is a release blocker

### Required Deliverables

1. one real `setup` family in the Rust CLI with `init`, `refresh`, and bare-command auto-routing
2. one compiler-owned setup module that reuses the existing repo-safe mutation helpers
3. one locked starter-template set for `CHARTER.md`, `FEATURE_SPEC.md`, and optional `PROJECT_CONTEXT.md`
4. one exact refresh contract covering preserve-by-default, `--rewrite`, and `--reset-state`
5. one refusal and `doctor` cutover so missing canonical truth points at `setup`
6. one docs/help/contracts cutover removing placeholder-only setup language from shipped product surfaces
7. one proof corpus and snapshot set for new repo, initialized repo, partial repo repair, and destructive-flag paths
8. one explicit historical-reference cleanup pass that leaves legacy material as evidence only

### Deferred Work

These items remain explicitly deferred behind the `M6` wedge:

- thin MCP/UI companion from [TODOS.md](TODOS.md)
- review/fix packet family from [TODOS.md](TODOS.md)
- live slice lineage and live execution packets from [TODOS.md](TODOS.md)
- public CLI distribution from [TODOS.md](TODOS.md)
- CLI release workflow from [TODOS.md](TODOS.md)
- operator-outcome scoreboard from [TODOS.md](TODOS.md)
- richer post-setup onboarding beyond the immediate `doctor` handoff from [TODOS.md](TODOS.md)
- `pipeline validate` preflight surface from [TODOS.md](TODOS.md)
- structured run provenance for `resolve`, `compile`, and `state set` from [TODOS.md](TODOS.md)

If a session proposes one of these before the front door is honest, the answer should usually be "not yet."

### NOT In Scope

- do not widen `setup` into an everyday incremental pipeline rebuild command
- do not invent a generic onboarding chat or runtime orchestrator
- do not broaden the canonical `.system/*` input contract beyond the existing three canonical starter files
- do not infer starter file content from ambient repo evidence
- do not add a second recovery surface that competes with `doctor`
- do not add release automation or public packaging work
- do not use docs cleanup as a substitute for a real command surface

### Worktree Parallelization Strategy

Dependency table:

| Step | Modules touched | Depends on |
| --- | --- | --- |
| A. Lock setup routing, scaffold contract, and destructive-flag semantics | `PLAN.md`, `docs/contracts/`, `DESIGN.md` | --- |
| B. Add compiler-owned setup execution core and starter templates | `crates/compiler/` | A |
| C. Add CLI `setup` wiring, bare-command routing, and help snapshots | `crates/cli/`, help snapshots | A, B |
| D. Cut over resolver, `doctor`, and next-safe-action wording plus docs parity | `crates/compiler/src/resolver.rs`, `crates/compiler/src/rendering/*`, `README.md`, `docs/` | A, B |
| E. Add end-to-end proof corpus, refusal snapshots, and regression coverage | `crates/cli/tests/`, `crates/compiler/tests/`, test fixture support | B, C, D |

Parallel lanes:

- Lane A: A
- Lane B: B
- Lane C: C
- Lane D: D
- Lane E: E

Execution order:

1. land Lane A first and freeze the setup contract
2. launch Lane B immediately after A
3. once Lane B freezes the typed setup outcome surface, launch Lane C and Lane D in parallel worktrees
4. merge C and D
5. finish Lane E last so the proof corpus and snapshots target the final command wording and refusal story

Conflict flags:

- Lanes C and D both touch operator wording, so keep CLI help text and docs wording aligned before snapshots are finalized
- Lane E touches both CLI and compiler tests, so do not start it before the command surface and refusal copy stabilize

### Exit Criteria

- `system setup` is a real supported surface, not a placeholder
- new repos can establish canonical `.system/` truth from the Rust CLI alone
- existing repos can deliberately re-establish that truth through `setup refresh` without silent canonical rewrites
- `generate`, `inspect`, and `doctor` all point at the same setup story
- docs, help, contracts, and drift guards stop describing `setup` as placeholder-only
- historical reference material remains explicit evidence and stops acting like active setup authority
- the proof corpus honestly covers first-run bootstrap, partial-repo repair, refresh preserve behavior, and explicit destructive flags

### Completion Summary

- Step 0: Scope Challenge, scope accepted as-is and narrowed to one bounded setup family plus parity cleanup
- Architecture Review: 7 architecture decisions locked, no new infrastructure admitted
- Code Quality Review: 5 minimum-diff guardrails locked, no generic framework work admitted
- Test Review: diagram produced, 19 required coverage gaps enumerated
- Performance Review: 4 hot-path and determinism rules locked
- NOT in scope: written
- What already exists: written
- TODOS.md updates: none required, current deferred items remain correct
- Failure modes: 4 release-blocking critical gaps identified
- Outside voice: CEO decisions from 2026-04-17 absorbed into the milestone contract
- Parallelization: 5 lanes total, 2 lanes parallelizable after contract freeze, 3 sequential gates
- Lake Score: complete option preserved across command surface, recovery story, parity work, and proof coverage

Stay on the wedge until the front door is honest.

## Historical Review Appendices

If any historical appendix below conflicts with the active milestone text above, the milestone text wins. The appendices are evidence for how the plan evolved, not the current execution contract.

## AUTOPLAN REVIEW (2026-04-15, historical M3.5 rebase)

Review basis:

- Active plan file: this file
- Branch / commit reviewed: `main` at `c5f3072` (`feat: ship m3 pipeline capture wedge (#6)`)
- Recent artifacts consulted:
  - `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/ceo-plans/2026-04-10-rust-pipeline-parity.md`
  - `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-design-20260409-125135.md`
- Environment note:
  - the installed environment did not contain the downstream `plan-ceo-review`, `plan-design-review`, or `plan-eng-review` skill files that `/autoplan` expected
  - Claude subagent delegation was unavailable in this session
  - this review followed the `/autoplan` method directly, used external `codex exec` CEO and engineering challenges, and verified the branch with `cargo test --quiet` (full suite green)

### Phase 1: CEO Review

Premise challenge:

| Premise | Verdict | Why |
| --- | --- | --- |
| `system` should stay the generator/compiler layer, not the whole runtime stack | Accepted | The current codebase and shipped command surface are coherent around compiler ownership. Widening into runtime/orchestration now would blur the product boundary before one useful flow is proven. |
| The immediate user pain is repeated repo research and context shuttling | Accepted with condition | The code now covers resolve, compile, and capture. The open issue is no longer whether the compiler can own those pieces, it is whether operators and downstream consumers actually trust the handoff enough to stop rereading repo truth. |
| It is still correct for the active plan to frame M3 as the next implementation step | Rejected | `main` already ships M3 capture code, contract docs, goldens, CLI surface, and green tests. Keeping M3 as “Immediate Next Work” makes the active plan mis-sequence future sessions. |
| It is safe to assume canonical docs already exist and are rich enough to be useful | Questioned | That assumption keeps the current wedge narrow, but it may also hide the true bottleneck. The next milestone must prove that pre-populated canonical docs are enough to replace one real planning loop, not just one idealized one. |

What already exists:

- Route truth and runtime state persistence live in [`crates/compiler/src/route_state.rs`](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/route_state.rs).
- Stage compile proof is shipped in [`crates/compiler/src/pipeline_compile.rs`](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/pipeline_compile.rs) for `pipeline.foundation_inputs` + `stage.10_feature_spec`.
- Stage output materialization is shipped in [`crates/compiler/src/pipeline_capture.rs`](/Users/spensermcconnell/__Active_Code/system/crates/compiler/src/pipeline_capture.rs) for `stage.05_charter_synthesize` and `stage.07_foundation_pack`.
- CLI exposure is shipped through [`crates/cli/src/main.rs`](/Users/spensermcconnell/__Active_Code/system/crates/cli/src/main.rs) and backed by CLI surface/help tests.
- The proof corpus and goldens already exist under [`tests/fixtures/pipeline_proof_corpus/foundation_inputs/`](/Users/spensermcconnell/__Active_Code/system/tests/fixtures/pipeline_proof_corpus/foundation_inputs/).

Dream state delta:

| Horizon | State |
| --- | --- |
| Current branch reality | M3 is shipped. The compiler now owns route truth, one compile wedge, and two capture wedges with green tests. |
| What this plan still says | Implement M3, then later think about proving one end-to-end flow. |
| 12-month ideal | One real planning flow runs end to end, one downstream consumer trusts the generated artifacts without reopening the repo, and operator-outcome evidence shows babysitting actually dropped. |

Implementation alternatives:

| Approach | Effort | Decision | Reason |
| --- | --- | --- | --- |
| Rebaseline the plan immediately around post-M3 proof, adoption, and the first end-to-end flow | S | Recommended | This matches shipped reality and moves the plan back onto the unsolved user-value question. |
| Continue treating M3 as the active implementation milestone | S | Rejected | It would spend more sessions re-proving shipped work. |
| Widen now into onboarding/runtime/platform work | L | Rejected | The current plan is still missing proof that the compiler-owned handoff itself is enough to matter. |

NOT in scope from this review:

- widening into UI/MCP companion work
- reopening the archived reduced-v1 baseline as active strategy
- redefining the repo as a full runtime/orchestration product before one real planning loop is replaced

Error & Rescue Registry:

| Risk | Trigger | Rescue |
| --- | --- | --- |
| Active-plan drift | A later session follows “Immediate Next Work” literally | Advance the active plan to post-M3 status before more implementation begins. |
| Wrong bottleneck | The real operator pain is canonical-doc creation or downstream trust, not route/compile/capture mechanics | Make the next milestone prove one real operator flow and one downstream consumer handoff. |
| Adoption proof never arrives | Outcome measurement and provenance stay deferred | Pull operator proof and handoff evidence into the next approved milestone instead of leaving them as vague future work. |

Failure Modes Registry:

| Failure mode | Severity | Why it matters |
| --- | --- | --- |
| `PLAN.md` remains the “active execution source of truth” while describing already-shipped M3 work | Critical | The next session can spend effort on solved infrastructure and miss the actual product-risk questions. |
| The product keeps shipping compiler machinery without downstream consumer trust proof | Critical | Internal correctness without adoption proof does not remove operator tax in the real workflow. |
| The canonical-doc assumption remains untested against real operator behavior | High | The wedge may be optimizing a narrow happy path instead of the true workflow bottleneck. |

CODEX SAYS (CEO, codex-only external voice):

- The plan is strategically stale because it still treats M3 as prospective work.
- The plan defers the exact operator-outcome and downstream-trust proof that would validate the product thesis.
- The next milestone should prove one real operator can complete one real job and one downstream consumer can reuse the outputs without reopening repo truth.

CEO DUAL VOICES - CONSENSUS TABLE:

| Dimension | Claude Subagent | Codex | Consensus |
| --- | --- | --- | --- |
| Premises valid? | N/A | partial challenge | single-external-voice |
| Right problem to solve? | N/A | challenge | single-external-voice |
| Scope calibration correct? | N/A | challenge | single-external-voice |
| Alternatives sufficiently explored? | N/A | challenge | single-external-voice |
| Competitive / market risks covered? | N/A | challenge | single-external-voice |
| 6-month trajectory sound? | N/A | challenge | single-external-voice |

CEO completion summary:

| Section | Status | Notes |
| --- | --- | --- |
| Premise challenge | complete | One premise rejected, one questioned, two accepted. |
| Existing-code leverage map | complete | Route, compile, capture, CLI, proof corpus are already shipped. |
| Dream-state / alternatives | complete | The main delta is adoption proof, not more M3 mechanics. |
| Strategic risk scan | complete | The biggest risk is solving internal correctness before proving user value. |

### Phase 2: Design Review

Skipped, no UI scope.

Reason:

- UI-scope detection only matched incidental CLI/design-contract vocabulary, not a real screen/component/layout surface.
- This plan is about CLI behavior, compiler boundaries, and planning-flow replacement, so a dedicated product-design pass would add noise rather than signal in this turn.

### Phase 3: Engineering Review

Scope challenge:

- The shipped branch already includes `pipeline_capture.rs`, the capture contract, capture goldens, CLI help snapshots, and compiler/CLI tests.
- The full Rust test suite is green on this branch via `cargo test --quiet`.
- The engineering gap is not “is M3 implemented?” It is “does the next milestone describe the remaining flow gaps honestly?”

Architecture ASCII diagram:

```text
pipeline resolve
  -> route_state.rs
  -> persisted route_basis under .system/state/pipeline/<id>.yaml

pipeline compile
  -> pipeline_compile.rs
  -> supports stage.10_feature_spec only
  -> payload / explain proof to stdout

pipeline capture
  -> pipeline_capture.rs
  -> supports stage.05_charter_synthesize and stage.07_foundation_pack only
  -> preview cache + locked apply + rollback + deterministic state updates

foundation_inputs flow
  -> stage.04_charter_inputs (output-producing, not yet in shipped capture surface)
  -> stage.05_charter_synthesize (captured)
  -> stage.06_project_context_interview (optional branch, output-producing, not yet in shipped capture surface)
  -> stage.07_foundation_pack (captured)
  -> stage.10_feature_spec (compiled only, no shipped materialization path)

downstream consumer handoff
  -> not yet proven by plan or code as an accepted end-to-end trust boundary
```

Test diagram:

| Flow / codepath | Current evidence | Gap |
| --- | --- | --- |
| `pipeline resolve` route truth and persisted `route_basis` | compiler route/state tests, full suite green | no current plan-level proof artifact showing operator outcome |
| `pipeline compile` for `stage.10_feature_spec` | compiler tests, CLI tests, payload/explain goldens | no shipped materialization path for `FEATURE_SPEC.md` |
| `pipeline capture` for stage 05 | compiler tests, CLI tests, preview/apply goldens, repo mirror assertions | manual `needs_project_context` handoff remains operator-owned |
| `pipeline capture` for stage 07 | compiler tests, CLI tests, preview/apply goldens, rollback/state persistence assertions | no proof that downstream consumers can use generated outputs without rereading the repo |
| stage 04 / stage 06 materialization | none in shipped capture surface | major M4 boundary gap |
| downstream consumer trust handoff | none | major M4/M5 boundary gap |

Failure Modes Registry:

| Failure mode | Severity | Current status | Next move |
| --- | --- | --- | --- |
| M4 claims “end-to-end foundation flow” without specifying how stages 04, 06, and 10 are written or handed off | High | unresolved | Add an explicit milestone or contract for the missing materialization boundary. |
| M3 text says it “must also cover” `needs_project_context`, but later says the variable stays manual | Medium | unresolved | Pick one posture and document it once. |
| Transactional apply wording reads like arbitrary-writer safety, while the implementation only locks route state | Medium | unresolved | Narrow the guarantee to `system`-coordinated writers or add a stronger file-level concurrency boundary. |

CODEX SAYS (engineering, codex-only external voice):

- `PLAN.md` is stale enough to mis-sequence implementation.
- M4 is underspecified because the shipped compile/capture wedges do not yet compose into a full `foundation_inputs` replacement.
- The `needs_project_context` story and the rollback guarantee both need tighter boundaries in the plan.

ENG DUAL VOICES - CONSENSUS TABLE:

| Dimension | Claude Subagent | Codex | Consensus |
| --- | --- | --- | --- |
| Architecture sound? | N/A | concern | single-external-voice |
| Test coverage sufficient? | N/A | partial concern | single-external-voice |
| Performance risks addressed? | N/A | no major concern raised | single-external-voice |
| Security / safety threats covered? | N/A | partial concern | single-external-voice |
| Error paths handled? | N/A | partial concern | single-external-voice |
| Deployment / release risk manageable? | N/A | concern | single-external-voice |

Engineering test-plan artifact:

- [/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-test-plan-20260415-145214.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-test-plan-20260415-145214.md)

Engineering completion summary:

| Section | Status | Notes |
| --- | --- | --- |
| Scope challenge against real code | complete | Verified against shipped compiler, CLI, contracts, proof corpus, and tests. |
| Architecture review | complete | M3 internals are coherent; the next gap is cross-stage flow composition. |
| Test review | complete | Full suite green; M4 still lacks flow-level coverage for stage 04 / 06 / 10 materialization and downstream consumer trust. |
| Performance / safety review | complete | No immediate runtime regressions found; wording overstates transactionality relative to the current lock boundary. |

Cross-phase themes:

- The active plan is stale after M3 shipping, and that is now the highest-severity issue.
- The next milestone has to prove operator and downstream-consumer value, not more isolated M3 machinery.
- The M4 boundary is not concrete enough yet. Stage 04, stage 06, and stage 10 still need an explicit materialization / handoff story.
- Measurement and provenance are drifting toward “nice to have” status even though they are part of the trust thesis.

## Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
| --- | --- | --- | --- | --- | --- | --- |
| 1 | Intake | Treat root `PLAN.md` as the active plan target for `/autoplan` | mechanical | explicit over clever | It is the repo-declared source of truth and the only plan file aligned to current branch work. | scanning historical or seam-local plans as the primary target |
| 2 | CEO | Keep the repo boundary as compiler/generator first | auto-decided | pragmatic | The shipped code already coheres around that boundary, and widening now would blur the proof target. | runtime/orchestration expansion now |
| 3 | CEO | Flag “M3 is still next work” as a user challenge | user_challenge | choose completeness | Both the primary review and the external CEO voice agree the plan must advance past shipped M3 reality. | silently accepting the stale milestone ordering |
| 4 | CEO | Elevate operator-proof / downstream-trust validation into the next milestone conversation | user_challenge | bias toward action | Without proof of reuse, the compiler spine remains internally correct but externally unproven. | leaving proof entirely deferred to M5+ |
| 5 | Design | Skip dedicated design review | mechanical | pragmatic | No actual UI scope was detected beyond CLI wording and output contracts. | forcing a design phase on non-UI work |
| 6 | Eng | Use real branch evidence, including full `cargo test --quiet`, instead of plan prose alone | mechanical | explicit over clever | The code, contracts, and tests are already present, so review quality depends on inspecting them directly. | reviewing the plan in isolation |
| 7 | Eng | Treat M4 materialization/handoff definition as the main engineering gap | taste | completeness | The shipped surfaces do not yet compose into a true end-to-end `foundation_inputs` replacement. | assuming stdout compile plus partial capture is already sufficient |
| 8 | Eng | Surface the transactionality wording as a boundary decision, not a blocker | taste | explicit over clever | The implementation is coherent for `system`-coordinated flows, but the plan text currently reads broader than the lock model proves. | claiming arbitrary-writer transactional safety |
| 9 | M3.5 | Reuse `pipeline capture` as the only writer surface for stages `04`, `06`, and `10` | auto-decided | explicit over clever | The existing capture boundary already owns preview, apply, rollback, and route-freshness checks. Extending that whitelist is lower risk than adding a second writer model. | adding `pipeline run`, `pipeline compile --write`, or a new writer command |
| 10 | M3.5 | Keep `needs_project_context` manual and exact | auto-decided | explicit over clever | The stage-set variable is a human judgment call, and the current compiler contract already has one safe handoff: `pipeline state set`, then `pipeline resolve`. | auto-setting the variable inside capture or leaving the handoff ambiguous |
| 11 | M3.5 | Make `stage.10_feature_spec` materialization flow through compile-to-capture | taste | pragmatic | `pipeline compile` already owns the payload/proof contract for stage 10, so capture should materialize that body rather than redefining compile semantics. | broadening compile to write files directly |
| 12 | M3.5 | Prove the stage-10 handoff with a real compile-to-capture integration test | auto-decided | choose completeness | A synthetic markdown-only capture test would leave the documented operator path unverified. | relying only on isolated compile tests plus isolated capture tests |
| 13 | Intake | Treat shipped `M3.5` on `main` as the active baseline for M4 planning | mechanical | explicit over clever | `8ac7aeb` and the green suite are stronger truth than stale milestone prose. | continuing to plan from the pre-ship M3.5 review state |
| 14 | CEO | Define M4 around operator replacement proof, not more surface-area expansion | auto-decided | pragmatic | The product risk is usefulness now, not missing command verbs. | inventing new orchestration commands before proving the current flow matters |
| 15 | CEO | Pull one bounded downstream consumer-readiness check into M4, but keep actual consumer adoption in M5 | taste | choose completeness | An operator-only path would duplicate what the repo already nearly proves; one light handoff check closes the usefulness gap without widening into real downstream integration. | stopping M4 at a pure operator journey or moving full downstream adoption into M4 |
| 16 | Eng | Require the actual stage-10 shell pipe in M4 proof coverage | auto-decided | explicit over clever | The current route-progression test still hands capture an in-process compiled payload, which is weaker than the documented operator path. | treating the existing stage-10 shortcut as sufficient end-to-end proof |
| 17 | Eng | Use a dedicated realistic demo corpus for M4 instead of overloading the shared proof corpus | taste | pragmatic | The proof corpus should stay small and contract-focused; M4 needs a more believable operator story fixture. | cramming adoption-proof concerns into the contract regression corpus |
| 18 | Intake | Refresh the restore point on `main` before reworking M5 | mechanical | explicit over clever | The top-of-file restore marker still pointed at a `feat/m4` snapshot, which made the new M5 rewrite harder to reverse cleanly. | editing the active plan against a stale restore breadcrumb |
| 19 | CEO | Reframe M5 from artifact substitution to workflow replacement | user_challenge | choose completeness | Both outside CEO voices agreed that “consumer can start from artifacts” is too weak. The proof target has to be one named consumer finishing one real job without repo archaeology. | treating smaller handoff input alone as product proof |
| 20 | CEO | Name `feature-slice-decomposer` as the first M5 consumer | taste | bias toward action | A generic “downstream consumer” is not falsifiable. Naming one exact feature-to-slice consumer forces a concrete input contract, output target, and failure condition. | leaving the first consumer abstract |
| 21 | Eng | Keep `.system/*` canonical and introduce one explicit non-canonical downstream handoff bundle | auto-decided | explicit over clever | Current contracts reject raw `artifacts/*` as canonical runtime inputs. The cleanest path is a derived, versioned bundle plus manifest rather than silent promotion or shadow truth. | direct raw-artifact adoption or hidden canonical promotion |
| 22 | Eng | Pull minimal provenance and trust classes into M5 itself | auto-decided | choose completeness | M5 cannot ask a downstream consumer to trust derived outputs while leaving the trust model in TODO-only form. Route-basis fingerprint, payload hash, producer version, and trust class are part of the milestone. | deferring all provenance to later generic work |
| 23 | Eng | Add a before / after scorecard as an M5 deliverable | auto-decided | pragmatic | The repo’s success criteria already hinge on reduced rereads and less babysitting. Without a concrete scorecard, M5 can pass on narrative instead of evidence. | claiming operator-pain reduction without measurement |
| 24 | Eng | Replace the stale M4 “Immediate Next Work” list with M5 actions | mechanical | explicit over clever | Leaving shipped M4 tasks at the active frontier would blur the post-M4 baseline and make the next milestone look half-open. | keeping old M4 closure tasks as the active next work list |

## GSTACK REVIEW REPORT

Historical note:

- This table records the earlier pre-ship M3 review snapshot.
- The current active direction is the `/autoplan` review above, which re-baselines the next deliverable as `M3.5`.

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR | 4 proposals, 1 accepted, 2 deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 5 | CLEAR | 3 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**UNRESOLVED:** historical snapshot only

**VERDICT:** HISTORICAL M3 PRE-SHIP REVIEW. SUPERSEDED BY THE `/autoplan` M3.5 REBASE ABOVE.

## AUTOPLAN REVIEW (2026-04-15, M4 solidification)

Review basis:

- Active plan file: this file
- Branch / commit reviewed: `main` at `8ac7aeb` (`feat: complete M3.5 foundation-inputs capture surface (#7)`)
- Current supporting evidence reviewed:
  - [`docs/START_HERE.md`](docs/START_HERE.md)
  - [`docs/SUPPORTED_COMMANDS.md`](docs/SUPPORTED_COMMANDS.md)
  - [`core/stages/04_charter_inputs.md`](core/stages/04_charter_inputs.md)
  - [`core/stages/05_charter_synthesize.md`](core/stages/05_charter_synthesize.md)
  - [`core/stages/06_project_context_interview.md`](core/stages/06_project_context_interview.md)
  - [`core/stages/07_foundation_pack.md`](core/stages/07_foundation_pack.md)
  - [`core/stages/10_feature_spec.md`](core/stages/10_feature_spec.md)
  - [`crates/compiler/tests/pipeline_capture.rs`](crates/compiler/tests/pipeline_capture.rs)
  - [`crates/compiler/tests/pipeline_compile.rs`](crates/compiler/tests/pipeline_compile.rs)
  - [`crates/cli/tests/cli_surface.rs`](crates/cli/tests/cli_surface.rs)
- Verification:
  - `cargo test --quiet` is green on this branch
  - external Codex challenge partially completed before timeout and still surfaced one useful engineering gap: the existing route-progression test does not exercise the literal stage-10 shell pipe

### Phase 1: CEO Review

Premise challenge:

| Premise | Verdict | Why |
| --- | --- | --- |
| `M4` can stay vague because `M3.5` already shipped the path | Rejected | The repo now proves components and most of the sequence. A vague next milestone would just create more “prove the path exists” churn. |
| `M4` should prove a real operator outcome, not add more verbs | Accepted | The missing question is usefulness, not surface area. |
| `M5` should still own actual downstream consumer adoption | Accepted with expansion | `M4` needs one bounded consumer-readiness check so the handoff is not vibes, but full consumer adoption still belongs in `M5`. |

What already exists:

- the docs already state the exact `foundation_inputs` command path, including the manual `needs_project_context` handoff and the stage-10 compile-to-capture boundary
- capture goldens already exist for stages `04`, `05`, `06`, `07`, and `10`
- the CLI suite already includes [`pipeline_foundation_inputs_route_progression_supports_full_m35_path`](crates/cli/tests/cli_surface.rs), which proves most of the sequence

Error & Rescue Registry:

| Risk | Trigger | Rescue |
| --- | --- | --- |
| M4 turns into another surface-area milestone | The plan focuses on new commands instead of one believable proof | Keep M4 pinned to one demo corpus, one operator journey, and one bounded handoff check. |
| M4 duplicates existing evidence | The team adds more route/capture unit coverage but still does not prove the operator path matters | Require the real stage-10 shell pipe and a realistic journey artifact. |
| M5 gets collapsed into M4 | The plan tries to wire real downstream consumers immediately | Keep M4 at readiness-check level and defer actual consumer adoption to M5. |

### Phase 2: Design Review

Skipped, no UI scope.

Reason:

- this milestone is still CLI/compiler proof work, not a screen, workflow UI, or design-system surface

### Phase 3: Engineering Review

Architecture ASCII diagram:

```text
realistic foundation-flow demo fixture
  -> system pipeline resolve
  -> capture 04
  -> capture 05
  -> manual needs_project_context decision
  -> resolve
  -> optional capture 06
  -> resolve
  -> capture 07
  -> system pipeline compile stage.10
  -> shell pipe into system pipeline capture stage.10
  -> artifacts/feature_spec/FEATURE_SPEC.md
  -> bounded downstream-readiness validator
```

Test diagram:

| Flow / codepath | Current evidence | M4 gap to close |
| --- | --- | --- |
| Full `04 -> 05 -> state set -> 06? -> 07 -> 10` route progression | [`crates/cli/tests/cli_surface.rs`](crates/cli/tests/cli_surface.rs) route-progression test | upgrade stage `10` from in-process payload injection to the literal CLI shell pipe |
| Stage `10` payload/explain correctness | compile goldens plus compiler/CLI tests | tie the real CLI pipe into the same end-to-end operator proof |
| Capture boundaries for `04`, `06`, and `10` | preview/apply goldens already committed | prove them on a believable demo corpus, not only the shared contract corpus |
| Downstream trust | none beyond artifact existence | add a bounded readiness checker for generated `FEATURE_SPEC.md` |

Failure Modes Registry:

| Failure mode | Severity | Why it matters |
| --- | --- | --- |
| M4 proves only another synthetic path test | High | That does not answer whether the operator can trust the result as planning basis. |
| The stage-10 handoff still bypasses the literal CLI shell pipe | High | The repo would keep documenting a path it does not actually prove end to end. |
| The generated `FEATURE_SPEC.md` exists but is not checked for downstream-readiness | High | File existence alone is not handoff quality. |

Engineering test-plan artifact:

- [/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-test-plan-20260415-203952.md](/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-main-test-plan-20260415-203952.md)

Cross-phase themes:

- `M4` has to answer usefulness now, not surface coverage
- the exact stage-10 handoff is the engineering edge that still needs explicit end-to-end proof
- bounded downstream-readiness belongs in `M4`, but real downstream consumer adoption still belongs in `M5`

Latest verdict:

- `M3.5` is shipped and no longer the planning target
- `M4` is now concrete enough to execute
- `M5` remains the first milestone that should claim actual downstream consumer adoption
