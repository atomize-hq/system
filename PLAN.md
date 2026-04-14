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
  - route-relevant booleans such as `needs_project_context` and `charter_gaps_detected`
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
   - Successful `capture apply` deletes the cache entry.
   - Refused or failed `capture apply` keeps the cache entry so the operator can retry after repair.
   - M3 does not add cache GC, cache listing, or a `capture show` surface.
6. Define the write protocol.
   - Preview builds the full capture plan but performs no writes outside the runtime cache.
   - Apply must:
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
     - `capture_refuses_single_file_with_file_wrapper`
     - `capture_refuses_missing_declared_block`
     - `capture_refuses_duplicate_declared_block`
     - `capture_refuses_undeclared_block`
     - `capture_refuses_stale_route_basis`
     - `capture_refuses_inactive_stage`
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
    │   └── [GAP] ambiguous/missing repo mirror refused
    │
    ├── preview cache
    │   ├── [GAP] preview writes runtime cache entry
    │   └── [GAP] apply reuses cache entry by capture_id
    │
    └── apply path
        ├── [GAP] fresh route basis required
        ├── [GAP] inactive stage refused
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
    └── [GAP] [→CLI] stale cached preview refuses after route-state mutation

─────────────────────────────────
COVERAGE: 0/17 paths tested today for M3
  Code paths: 0/12
  User flows: 0/5
QUALITY: ★★★: 0  ★★: 0  ★: 0
GAPS: 17 paths need tests
─────────────────────────────────

Failure modes that M3 must name explicitly:

| Codepath | Real failure mode | Test required | Error handling required | User-visible posture |
| --- | --- | --- | --- | --- |
| single-file capture | model emits FILE wrapper for a single-file stage | yes | yes | explicit refusal |
| multi-file capture | one declared artifact block is missing | yes | yes | explicit refusal |
| multi-file capture | model emits an extra undeclared block | yes | yes | explicit refusal |
| repo mirror derivation | required repo mirror cannot be derived from artifacts | yes | yes | explicit refusal |
| preview/apply cache | cached preview exists but route basis is stale by apply time | yes | yes | explicit refusal with rerun resolve |
| file commit | one target rename fails after another already committed | yes | yes | rollback + refusal |
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
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 4 | CLEAR | 8 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**UNRESOLVED:** 0

**VERDICT:** ENG CLEARED — M3 spec is ready to implement.
