<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/system/main-autoplan-restore-20260406-212846.md -->
# PLAN

## Status

This is the implementation plan for the reviewed reduced v1 wedge.

It is derived from:

- [reviewed reduced-v1 seam pack](artifacts/planning/reduced-v1-seam-pack/README.md)
- CEO review decisions embedded in that seam pack
- engineering review decisions recorded in that seam pack's `## GSTACK REVIEW REPORT`

This plan is the current execution source of truth for repo shape, migration order, and milestone sequencing. The canonical repo-surface contract lives at [docs/contracts/C-01-approved-repo-surface.md](docs/contracts/C-01-approved-repo-surface.md).

## Locked Decisions

- Rust is the only supported packet-resolution authority for v1.
- The current Python harness is legacy reference material only.
- Legacy Python stays frozen and clearly labeled in place until the Rust planning packet path is proven. The physical move under `archived/` happens during cutover, not before.
- The repo root becomes approved surface only.
- Nothing under `archived/` is imported, executed, or wrapped by the supported runtime path.
- Live v1 packet resolution is scoped to existing `project + feature` artifacts.
- V1 execution packets are fixture-backed demos only.
- Live slice lineage and live execution packets are deferred.
- Canonical project truth for managed repos lives under a hidden repo-local `.system/` directory, not under user-home state.
- V1 direct packet inputs are `CHARTER`, optional `PROJECT_CONTEXT`, and `FEATURE_SPEC`.
- `FOUNDATION_STRATEGY`, `TECH_ARCH_BRIEF`, `TEST_STRATEGY_BRIEF`, `QUALITY_GATES_SPEC`, and `ENVIRONMENT_INVENTORY` are inherited posture dependencies. Lower-level artifacts may override them only with explicit rationale captured in artifact content and the decision log.
- Repo-facing copies may exist for humans, but they are derived views, not runtime inputs.
- CLI-owned templates and static prompt assets live in the `system` product repo, not in each managed project repo unless they are intentionally materialized there.
- User-level hidden state may exist later for cache, config, or diagnostics, but never for canonical project truth.
- V1 metadata/schema work is limited to those direct packet inputs plus inherited posture dependencies and one request-scoped derived manifest.
- V1 freshness is deterministic: file presence, file hash, schema version, manifest generation version, and declared dependency checks.
- V1 manifest state is request-scoped and in-memory by default. Persist detailed diagnostics only on request or on failure.
- Renderers are pure views over one typed resolver result plus typed decision log.
- `doctor` or `health` is a required v1 command surface, not a post-v1 nicety.
- Packet budgets are a first-class typed policy contract with deterministic keep, summarize, exclude, and refuse behavior.
- V1 performance stays simple until measurement proves otherwise.
- V1 distribution is a Rust CLI with explicit local install support for `macOS arm64` and `Linux x86_64`. Public package-manager and release publishing are deferred.

## Goal

Ship a reduced v1 that proves the product honestly:

- guided project setup remains the true front door to the product because it establishes `CHARTER`, posture docs, and the canonical artifact base that every later command depends on
- live planning packet generation over existing project + feature artifacts
- fixture-backed execution packet demo only
- explicit refusal for unsupported live slice execution requests
- explicit `doctor` guidance for stale, missing, or contradictory packet inputs
- Rust CLI as the only supported product path

## Operator Experience Architecture

The product journey starts before packet generation.

Reduced v1 still centers implementation on Rust packet resolution over existing artifacts, but the operator's mental model must stay explicit:

- first they establish project posture and standards through guided setup that produces `CHARTER` and the rest of the canonical artifact base
- then they use packet generation and inspect surfaces to request the minimum correct context for planning work
- when context is stale, missing, or contradictory, `doctor` is the canonical recovery surface that explains what is wrong and how to repair it

Reduced v1 does not need to re-implement the entire setup flow in Rust, but docs, help text, and command hierarchy must present setup as stage zero of the supported experience rather than pretending packet generation is the beginning of the story.

The honest framing for reduced v1 is:

- setup is the true product entrypoint and remains supported because the product cannot work without canonical posture and standards artifacts
- the current guided setup flow may still be powered by the existing scaffold while the Rust replacement is not ready
- the reduced v1 Rust CLI owns packet resolution, inspect, and doctor truth after setup artifacts exist
- docs and help text must never blur "supported workflow" with "already reimplemented in Rust"

### Operator Flow

```text
new repo or new project
    |
    v
guided setup / refresh existing setup
    |
    v
canonical artifacts exist in `.system/`
(`CHARTER`, optional `PROJECT_CONTEXT`, `FEATURE_SPEC`, inherited posture docs)
    |
    +--> generate planning packet
    |
    +--> inspect packet composition and decision log
    |
    +--> doctor when artifacts are stale, missing, contradictory, or unsupported
```

### CLI Surface Hierarchy

The reduced-v1 command-surface contract is [C-02 Rust Workspace and CLI Command-Surface Contract](docs/contracts/C-02-rust-workspace-and-cli-command-surface.md).
The reduced-v1 canonical `.system/` manifest + freshness contract is [C-03 Canonical Artifact Manifest Contract](docs/contracts/C-03-canonical-artifact-manifest-contract.md).

| Surface | Job | When the operator reaches for it | Required first impression |
|---------|-----|----------------------------------|---------------------------|
| Setup / setup refresh | Establish or refresh canonical project posture and standards | New repo, changed architecture, stale posture docs | "You are establishing the truth this system will trust later." |
| `generate` | Produce the minimum correct planning packet from established artifacts | Normal repeat-use path after setup exists | "Here is the packet, what was included, and why it is safe to trust." |
| `inspect` | Explain inclusion, exclusion, lineage, freshness, and budget decisions | User wants proof, debugging, or teaching | "Here is how the resolver thought." |
| `doctor` | Diagnose blockers and give safe next actions | `generate` or setup/refresh cannot proceed cleanly | "Here is what is wrong, whether it is safe to continue, and what to do next." |

Setup-specific support note:

- until Rust setup exists, the supported docs path must explicitly send the operator through the existing guided setup flow to establish canonical artifacts
- once those artifacts exist, the Rust CLI becomes the supported packet-resolution authority
- setup refresh messaging must distinguish "refresh your canonical artifacts" from "generate a packet"

### Information Hierarchy Rules

- README and help text must present setup first, packet generation second, and repair third.
- Successful `generate` output must show packet identity first, then included sources, then omission and budget notes, then next actions.
- Refusal output must show the blocking reason first, then the exact artifact or dependency at fault, then the safe repair command or workflow.
- `inspect` must read as a proof surface, not a dump of internal structs.
- `doctor` must aggregate blockers into one view so the operator does not play command whack-a-mole.
- `health` may exist only as an alias or later summary surface, never as a competing canonical recovery command in docs, help text, or examples.

## Interaction State Coverage

Reduced v1 must specify operator-visible states for setup, generation, proof, and recovery surfaces. The user experience goal is simple: every command should either complete with useful output or fail in a way that names the exact blocker and the exact next action.

### State Table

| Surface | Loading / in-progress | Empty / missing setup | Error / stale / contradictory | Success | Partial / unsupported |
|---------|------------------------|------------------------|-------------------------------|---------|-----------------------|
| Setup / setup refresh | Explain which artifact is being established or refreshed and what remains | Tell the operator which canonical artifact does not exist yet and why it matters | Name the artifact or dependency that cannot be established and the safe retry path | Confirm the canonical artifacts that are now trusted for later packet requests | If only some setup artifacts are refreshed, mark which truths are current and which still block packet generation |
| `generate` | Show requested packet identity and the lineage scope being resolved | Refuse compactly, name the missing canonical artifact, and point to setup or setup refresh | Refuse compactly, name the stale or contradictory artifact or dependency, and point to the exact `doctor` or refresh action | Show packet identity first, then included sources, then omission/budget notes, then next actions | Unsupported live slice requests refuse explicitly and explain that reduced v1 only supports live planning packets plus fixture-backed execution demos |
| `inspect` | Show that the resolver is loading decision evidence, not generating a new packet | Explain that there is no trusted packet basis to inspect yet and point to setup | Show the broken lineage, freshness, or policy rule with proof-friendly wording | Show inclusion, exclusion, freshness, and budget reasoning in a human-readable proof order | For zero-content outcomes, show the reason category and whether refusal thresholds were crossed |
| `doctor` | Show which checks are running and whether safe auto-repair is being attempted | Summarize all missing artifacts in one report and show the safest setup path | Summarize all stale, contradictory, or invalid inputs in one report with safe next actions | Confirm packet-readiness status, current trusted artifacts, and whether `generate` is safe to retry | If auto-repair fixes some issues but not all, show fixed items first, remaining blockers second, and the next safe action last |

### Refusal And Recovery Rules

- `generate` must not expand into a full diagnostic dump on failure.
- `generate` refusal must stay compact and structured: blocker summary, broken artifact or dependency, exact next action.
- The default next action is the narrowest safe path, either setup/setup refresh when canonical truth is missing or `doctor` when deeper diagnosis is needed.
- `doctor` owns the full blocker report and may aggregate multiple issues in one view.
- Retrying after repair must be clean. The operator should not have to infer whether stale negative state is still cached.

## User Journey And Emotional Arc

Reduced v1 is a trust product. The operator should feel three things in sequence:

- confidence during setup because the system is establishing durable truth rather than asking the same questions forever
- momentum during packet generation because the system returns a tight, useful packet without extra archaeology
- controlled caution during refusal and repair because the system stops unsafe work but immediately points to the fastest safe recovery path

The tone for refusal and repair is strict but guided. The system should refuse when trust is broken, but it should never make the operator guess what failed or what to do next.

### Journey Storyboard

| Step | User does | User feels | Plan must support |
|------|-----------|------------|-------------------|
| 1 | Starts with a new repo or project | "I need to establish truth once, not feed context forever." | Setup is clearly presented as the real front door and explains which canonical artifacts will exist after completion |
| 2 | Runs guided setup or refreshes stale setup | "This is work, but it should pay off later." | Each setup step names what artifact is being established and why later commands depend on it |
| 3 | Runs `generate` for planning work | "Give me the minimum correct context fast." | Success output is concise, trustworthy, and explicit about included sources and omissions |
| 4 | Uses `inspect` to verify why a packet looks the way it does | "Prove it." | Inspect reads as a human-auditable explanation of inclusion, exclusion, freshness, and budget decisions |
| 5 | Hits stale, missing, or contradictory context | "Stop me if this is unsafe, but do not waste my time." | Refusal copy is strict but guided: blocker first, exact artifact or dependency second, exact recovery action third |
| 6 | Runs `doctor` and repairs the issue | "Tell me everything relevant once so I can fix it cleanly." | The recovery report aggregates blockers, highlights safe auto-fixes first, and ends with the next safe retry action |
| 7 | Re-runs `generate` after repair | "I should be back on track, not in a loop." | Retry-clean behavior is explicit and avoids stale negative state or repeated hidden blockers |

### Time-Horizon Design

- First 5 seconds: the operator must understand whether they are in setup, generation, proof, or repair.
- First 5 minutes: the operator must complete one meaningful task without reading internal architecture docs.
- Long-term relationship: the system earns trust by refusing unsafe requests consistently and by making repair feel procedural rather than mysterious.

## Design System Alignment

No `DESIGN.md` exists in this repo. Reduced v1 should therefore use a small explicit CLI interaction language so docs, help text, examples, and rendered outputs do not drift into mixed product vocabularies.

### CLI Design Language

- One canonical recovery verb: `doctor`.
- One canonical generation verb: `generate`.
- One canonical proof verb: `inspect`.
- Use "setup" and "setup refresh" for posture-establishing flows. Do not rename the same workflow as bootstrap, init, hydrate, or health repair in other surfaces.
- Use "canonical artifacts" for trusted project truth and "derived views" for human-facing copies.
- Use "refusal" when the system stops unsafe work. Do not soften this into vague words like warning or issue when the command is actually blocked.
- Use "next safe action" for the recovery handoff line. This keeps repair output action-oriented.

### Copy And Output Rules

- Command help and README examples must use the same verbs and nouns as runtime output.
- Default output tone is strict but guided, never chatty and never cryptic.
- Success output should read like an operator summary, not a celebratory message.
- Failure output should avoid generic filler such as "something went wrong" or "unable to process request" when the exact artifact, dependency, or policy rule is known.
- Inspect output should privilege evidence order over internal module order.

## Responsive And Accessibility

Reduced v1 is a CLI product, so responsive design means terminal width, text density, screen-reader order, keyboard-only operation, and color independence. The output should remain readable in a normal narrow terminal without hiding the most important truth behind formatting.

### Output Layout Rules

- Default output strategy is adaptive but narrow-first.
- On narrow terminals, commands should use stacked summaries with one fact per line in stable order.
- On wider terminals, commands may use aligned sections or compact tables only when the same information remains readable if wrapped.
- Dense or multi-item evidence views must always have a machine-readable fallback such as JSON or inspect-friendly line output.
- The first three lines of any command result must still work when read aloud by a screen reader: outcome, object of interest, next action.

### Accessibility Rules

- Never rely on color alone to communicate refusal, success, or warning state.
- Important state changes must be labeled with words like `REFUSED`, `READY`, `STALE`, or `NEXT SAFE ACTION`.
- Output order must remain stable so keyboard users and screen readers do not need to rediscover where blockers or next steps appear.
- Help text and examples must assume keyboard-only use.
- Touch nothing fancy in v1 that breaks copy/paste, piping, or text selection for terminal users.
- If output is too dense for narrow terminals, the product should prefer truncating secondary detail and pointing to `inspect` or JSON rather than wrapping primary facts into noise.

## Resolved Design Decisions

| Decision | Chosen direction | Why |
|----------|------------------|-----|
| True front door | Guided project setup / setup refresh | The product depends on canonical posture artifacts before packet generation can be trusted |
| Reduced v1 setup framing | Setup remains supported, but reduced v1 stays explicit that the current guided setup flow may still use the existing scaffold until Rust replacement exists | Keeps the workflow honest without pretending the Rust rewrite already owns setup |
| `generate` failure behavior | Compact structured refusal plus exact repair handoff | Preserves packet-shrinking discipline even on the failure path |
| Refusal tone | Strict but guided | Trust products should stop unsafe work without making users do archaeology |
| Canonical recovery command | `doctor` | One memorable recovery verb beats split naming |
| Output layout strategy | Adaptive, narrow-first summaries with machine-readable fallbacks | CLI accessibility matters as much as visual responsiveness does on the web |
| `generate` default success surface | Short trust header, then full human-readable planning packet | The packet is the product, not a receipt pointing elsewhere |

## Remaining Design Constraints For Implementation

- `generate` should print a short trust header before the packet body rather than burying trust metadata after the content.
- JSON stays opt-in for dense machine-readable workflows, not the primary default for human operators.
- `inspect` remains the deep proof surface, not a second default packet renderer.
- Runtime help text, README examples, and test fixtures must use the same command vocabulary established above.

## What Already Exists

- `pipeline.yaml` already declares the live artifact graph for `CHARTER`, optional `PROJECT_CONTEXT`, and `FEATURE_SPEC`.
- `tools/harness.py` already implements include resolution, artifact input loading, output routing, and stage assembly as legacy reference behavior.
- `core/stages/10_feature_spec.md` already declares a concrete feature-spec output plus optional inherited posture inputs from foundation artifacts.
- The repo already documents that pipeline artifacts are the deterministic truth source and repo-facing copies are for human-facing durability.
- The current docs already distinguish implemented stages from placeholder slice/execution scaffolding.

## Storage Model

Reduced v1 needs a clean split between product code, managed-project truth, and optional machine-local state.

- Canonical project truth lives inside the managed project repo under `.system/`.
- Root-facing docs may exist for humans, but they are derived views and are never the runtime source of truth.
- The `system` product repo contains the CLI/library source, packaged templates, and tests.
- User-home hidden state such as `~/.system/` is reserved for non-canonical cache, config, diagnostics, or telemetry only. It must never become the only copy of project posture or planning truth.

## NOT in scope

- Do not preserve Python as a supported runtime path.
- Do not build live `project -> feature -> slice` lineage in v1.
- Do not build review/fix packets in v1.
- Do not build MCP UI in v1.
- Do not normalize every existing artifact into the metadata system in v1.
- Do not add an on-disk derived-state cache or semantic freshness layer in v1.
- Do not do public package-manager or release publishing in v1.

## Repo Migration Contract

### Root Rule

The repository root is the approved product surface only.

Anything in the root must satisfy one of these:

- part of the supported Rust CLI/compiler path
- a canonical artifact intentionally kept at root
- repo infrastructure required to build, test, validate, or document the supported path

### Archive Rule

Legacy Python scaffold material moves under `archived/`.

That includes:

- Python harness code
- legacy harness shell wrappers
- legacy harness docs that describe the supported runtime as Python
- legacy generated prompt scaffolding that is retained only for reference

### Promotion Rule

Files or ideas may move from `archived/` back into the approved surface only when they meet all of these:

- they are needed by the reviewed reduced v1 scope
- they are rewritten or re-approved intentionally
- they do not pull Python runtime coupling back into the supported path
- their role is documented in this plan or the reviewed design

### Runtime Boundary

- The supported runtime path must not import, shell out to, or wrap anything in `archived/`.
- `archived/` is evidence and reference material, not an execution dependency.

## Repo Shapes

### `system` Product Repo Shape

```text
system/
├── archived/
│   └── python-harness/
├── crates/
│   ├── compiler/
│   └── cli/
├── templates/
├── tests/
│   ├── fixtures/
│   └── golden/
├── docs/
├── Cargo.toml
├── Cargo.lock
├── PLAN.md
├── README.md
└── canonical artifacts retained at root only if explicitly approved
```

### Managed Project Repo Shape

```text
my-project/
├── .system/
│   ├── charter/
│   │   └── CHARTER.md
│   ├── project_context/
│   │   └── PROJECT_CONTEXT.md
│   ├── feature_spec/
│   │   └── FEATURE_SPEC.md
│   └── foundation/
│       ├── FOUNDATION_STRATEGY.md
│       ├── TECH_ARCH_BRIEF.md
│       ├── TEST_STRATEGY_BRIEF.md
│       ├── QUALITY_GATES_SPEC.md
│       └── ENVIRONMENT_INVENTORY.md
├── src/...
├── README.md
└── root-facing derived docs only if explicitly approved
```

## Milestones

### M1. Freeze The Legacy Scaffold

Outcome:

- legacy Python scaffold clearly reads as frozen reference material
- root clearly communicates Rust-first direction without losing the executable reference surface too early
- no ambiguity about supported vs legacy paths

Work:

- relabel legacy docs so the root docs do not present Python as the active product path
- freeze Python harness mechanics in place as reference-only behavior
- leave only approved root docs and canonical artifacts in place
- update references so `PLAN.md` and the reviewed design are easy to find

Exit criteria:

- a new contributor can tell in under 30 seconds that Python is legacy
- nothing at the root implies Python is the supported runtime
- the legacy harness remains runnable as a reference surface until Rust planning packet parity exists

### M2. Scaffold The Rust Workspace

Outcome:

- Rust workspace exists at the root with library + CLI split

Work:

- add root `Cargo.toml`
- add `crates/compiler`
- add `crates/cli`
- add initial shared types for packet result and decision log
- define CLI command surface skeleton

Exit criteria:

- `cargo check` passes
- CLI help exists
- there is one obvious place to add compiler logic

### M3. Define Minimal Packet Inputs And Manifest

Outcome:

- minimal live packet-input contract exists

Work:

- define typed ingest for `.system/charter/CHARTER.md`, optional `.system/project_context/PROJECT_CONTEXT.md`, and `.system/feature_spec/FEATURE_SPEC.md`
- define source-of-truth rules for canonical `.system/` artifacts versus derived repo-facing copies
- define inherited posture dependency handling for `FOUNDATION_STRATEGY`, `TECH_ARCH_BRIEF`, `TEST_STRATEGY_BRIEF`, `QUALITY_GATES_SPEC`, and `ENVIRONMENT_INVENTORY`
- define explicit override-with-rationale rules for lower-level artifacts that diverge from inherited posture
- define request-scoped derived manifest shape
- define deterministic freshness fields
- define supported target matrix for local installation: `macOS arm64` and `Linux x86_64`
- define how packaged templates in the `system` product repo materialize canonical `.system/` artifacts without becoming runtime project truth themselves
- document explicit triggers for expanding metadata/schema to more artifacts

Expansion triggers:

- an artifact becomes a required live packet input
- an artifact becomes a refusal source
- an artifact becomes a provenance dependency shown to the user
- an artifact becomes necessary to explain inclusion or exclusion decisions

Exit criteria:

- manifest can be built deterministically from approved live inputs
- unsupported artifacts are ignored explicitly, not implicitly
- inherited posture dependencies can mark packets stale without becoming mandatory packet body inputs

### M4. Implement Planning Packet Resolution

Outcome:

- live planning packets work over project + feature artifacts

Work:

- implement ingest
- implement manifest build
- implement deterministic freshness checks
- implement planning packet selection
- implement typed budget policy with deterministic keep, summarize, exclude, and refuse behavior
- implement typed decision log
- implement explicit refusal behavior
- implement `doctor` or `health` for blockers, stale reasons, safe next actions, and packet-readiness status

Exit criteria:

- same inputs yield same packet and same decision log
- stale or missing required inputs refuse clearly
- `doctor` reports the same blocker and freshness truth that packet generation uses
- budget behavior is deterministic and inspectable

### M5. Implement Renderers

Outcome:

- markdown, JSON, and inspect views all render from the same typed result

Work:

- add markdown renderer
- add JSON renderer
- add inspect renderer
- prove no renderer changes packet selection logic
- prove inspect output reflects the same decision log and budget policy as markdown and JSON

Exit criteria:

- inspect explains the same decision log used by markdown and JSON
- renderer failure does not destroy a successful resolver result

### M6. Add Fixture-Backed Execution Demo

Outcome:

- execution packet capability is demonstrated honestly without pretending live slice support

Work:

- define fixture lineage for execution packet demos
- implement execution demo path
- implement explicit refusal for unsupported live slice requests

Exit criteria:

- fixture execution packet demo works
- live slice execution requests refuse with clear wording

### M7. Add Test And CI Rails

Outcome:

- the Rust path is validated, not hoped into existence

Work:

- unit tests for ingest, metadata validation, manifest build, freshness checks, and refusal logic
- unit tests for inherited posture dependency freshness and override-with-rationale rules
- unit tests for budget policy: keep, summarize, exclude, and refuse
- unit tests for renderer failure isolation
- integration tests for planning packet resolution
- golden tests for markdown, JSON, and inspect outputs
- fixture-backed execution packet tests
- CLI E2E tests for install, help, non-repo-root invocation, `doctor`, and refusal flows
- drift tests for canonical `.system/` artifacts versus derived published docs
- cutover regression tests proving Python is not advertised as supported
- CI workflow for format, lint, test, and install smoke on `macOS arm64` and `Linux x86_64`

Exit criteria:

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- install smoke passes in CI on both supported targets

### M8. Docs And Cutover

Outcome:

- repo tells one story

Work:

- update top-level README for Rust-first product path
- keep legacy docs under clearly marked legacy/archive locations
- document how to use the Rust CLI for reduced v1
- document what is deferred
- move the frozen Python harness under `archived/python-harness/` once Rust planning packet parity and cutover validation are complete

Exit criteria:

- help text, README, and docs index all agree
- no top-level doc presents Python as the supported product path

## Workstreams

### Lane A: Repo Reshape

Scope:

- archive move
- root cleanup
- doc relabeling

Depends on:

- none

### Lane B: Rust Workspace

Scope:

- workspace scaffold
- compiler and CLI crate setup

Depends on:

- M1 root decisions locked

### Lane C: Resolver Core

Scope:

- ingest
- manifest
- freshness
- planning packet selection
- renderers

Depends on:

- M2
- M3

### Lane D: Validation Rail

Scope:

- tests
- golden fixtures
- CLI E2E
- CI

Depends on:

- M2 for workspace
- M4 for real behavior

## Execution Order

1. Do M1 first. This is the repo contract.
2. Start M2 immediately after M1.
3. Run M3 and the early part of M4 after M2.
4. Run M5 after the first typed resolver result exists.
5. Run M6 after planning packet resolution is stable.
6. Run M7 in parallel with late M4 to M6 once the command surface is real.
7. Finish with M8 so the docs match what actually shipped.

## Risks

### Risk: Legacy Freeze Leaves Support Messaging Ambiguous

Mitigation:

- relabel aggressively now
- keep one obvious Rust-first story in root docs
- do the physical archive move only after the Rust path is proven

### Risk: Python Patterns Leak Back Into Runtime Design

Mitigation:

- reference-only rule for `archived/`
- no runtime imports or wrappers
- promotion requires explicit approval

### Risk: Execution Demo Gets Mistaken For Live Capability

Mitigation:

- call it fixture-backed everywhere
- add explicit refusal for live slice requests
- test help text and docs for this wording

### Risk: Metadata Scope Grows Unbounded

Mitigation:

- expansion only through the trigger list in M3
- no artifact enters the schema by vibes

## Deliverables

- `PLAN.md`
- frozen legacy scaffold clearly labeled as reference-only, then archived under `archived/python-harness/` at cutover
- Rust workspace at root
- planning packet resolver
- `doctor` or `health` command
- fixture-backed execution packet demo
- tests and CI
- updated docs

## Definition Of Done For Reduced V1

- root repo shape reflects the approved Rust-first direction
- legacy Python is clearly labeled as frozen during implementation, then lives under `archived/` at cutover
- Rust CLI is the only supported product path
- live planning packets work over approved project + feature inputs
- inherited posture dependency freshness and override rationale are enforced
- packet budgets behave deterministically and are explained by inspect output
- `doctor` reports blockers and safe next actions
- execution packet demo works from fixtures only
- unsupported live slice requests refuse clearly
- docs and help text match reality
- CI validates build, lint, test, and install smoke on both supported targets

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | CLEAR | 5 proposals, 4 accepted, 1 deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 5 | CLEAR | 15 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 1 | CLEAR | score: 5/10 → 9/10, 7 decisions |

**UNRESOLVED:** 0
**VERDICT:** CEO + ENG + DESIGN CLEARED — ready to implement.

## AUTOPLAN REVIEW ADDENDUM (2026-04-06)

### Review Basis

This `/autoplan` run was executed as a post-implementation source-of-truth audit, not as a greenfield pre-implementation review.

Reason: the repo already contains the core Rust wedge that the plan still describes as future work:

- root workspace `Cargo.toml`
- `crates/compiler`
- `crates/cli`
- resolver, rendering, refusal, blocker, and manifest code
- CLI surface tests and help snapshots
- contracts `C-01` through `C-07`

Validation run during this review:

- `cargo test -q`
- `cargo clippy --all-targets --all-features -- -D warnings`
- `cargo install --path crates/cli --force`
- `cargo run -q -- --help`
- `cargo run -q -- generate --packet execution.demo.packet --fixture-set basic`
- `cargo run -q -- inspect --packet execution.demo.packet --fixture-set basic`

Observed reality:

- code and tests say the Rust wedge exists
- `docs/START_HERE.md` says live planning packet resolution is supported
- CLI help still calls the product a scaffold with reserved placeholders
- `generate` returns a placeholder packet body and exits non-zero on the ready path

This addendum records the review outputs needed to reconcile that drift.

### Phase 1: CEO Review

#### 0A. Premise Challenge

Examined: the plan framing in the status, goal, operator journey, milestones, and definition-of-done sections, plus current docs and CLI behavior.

Result:

- The plan still treats setup-first `.system/` truth as a settled fact, but current runtime behavior has not earned that as the only viable front door.
- The repo has already shipped enough of the Rust wedge that the real strategic question is no longer "should we build this?" It is "what can we honestly claim is already supported?"
- The highest-risk premise is not Rust itself. It is that support messaging can move ahead of runtime truth without hurting trust.

Premise gate outcome:

- On 2026-04-06, the user clarified that `PLAN.md` should be treated as "the plan that was just implemented."
- This review therefore uses a post-implementation audit lens for every phase.

#### 0B. Existing Code Leverage

| Sub-problem | Existing code / artifact already in repo |
|---|---|
| Canonical artifact ingest | `crates/compiler/src/canonical_artifacts.rs` |
| Request-scoped manifest generation | `crates/compiler/src/artifact_manifest.rs` |
| Freshness and fingerprinting | `crates/compiler/src/freshness.rs` |
| Refusal and blocker taxonomy | `crates/compiler/src/refusal.rs`, `crates/compiler/src/blocker.rs`, `crates/compiler/src/resolver.rs` |
| Budget policy | `crates/compiler/src/budget.rs` |
| Output surfaces | `crates/compiler/src/rendering/*.rs` |
| CLI surface and exit behavior | `crates/cli/src/main.rs` |
| Runtime drift guards | `crates/cli/tests/cli_surface.rs`, `crates/cli/tests/help_drift_guard.rs` |
| Product contracts | `docs/contracts/C-01` through `docs/contracts/C-07` |

#### 0C. Dream State Mapping

```text
CURRENT REPO
  -> Rust wedge exists, tests pass, install smoke passes
  -> docs and CLI disagree about what is supported
  -> plan still reads like M1-M8 are future work

THIS REVIEW
  -> reclassify PLAN.md as historical implementation plan + current audit record
  -> add explicit release gates for support claims
  -> pin setup ownership, command vocabulary, and packet-body truth

12-MONTH IDEAL
  -> one supported story
  -> `system generate` exits 0 with a non-placeholder packet body
  -> `setup`, `generate`, `inspect`, and `doctor` share one documented state model
  -> docs, help text, tests, and runtime output stay in lockstep
```

#### 0C-bis. Implementation Alternatives

| Approach | Effort | Risk | Pros | Cons | Decision |
|---|---|---|---|---|---|
| Keep treating `PLAN.md` as a future implementation plan | S | High | Lowest immediate doc churn | Misstates shipped reality and hides support drift | Rejected |
| Keep the Rust-first wedge, but rewrite the review around shipped-vs-claimed truth | M | Low | Preserves implementation work and fixes the highest-signal mismatch | Requires plan and docs cleanup now | Chosen |
| Reframe the whole wedge around "messy repo first" and demote Rust to an implementation detail | M-L | Medium | Stronger long-term product story | Too disruptive for this audit pass | Deferred to future strategy review |

#### 0D. Mode Selection

- Mode: `SELECTIVE_EXPANSION`
- Why: the repo does not need a new product thesis to benefit from this review. It needs the support boundary, setup ownership boundary, and command truth tightened first.

#### 0E. Temporal Interrogation

- HOUR 1:
  - user reads `docs/START_HERE.md`
  - expects live planning packet generation to be supported
  - runs CLI help and immediately sees "scaffold" and "reserved placeholders"
- HOUR 6:
  - user can install the CLI and hit deterministic refusal / inspect / demo paths
  - still cannot get a non-placeholder planning packet body from the ready path
- SIX MONTHS:
  - if unresolved, the project ships a coherent contract stack with an incoherent product claim

#### 0F. CEO Dual Voices

`CODEX SAYS (CEO — strategy challenge)`

- The plan over-indexes on trust mechanics, setup posture, and migration hygiene.
- The biggest strategic risk is claiming support before runtime truth matches docs.
- The most credible wedge is "trusted packet output quickly," not "structured setup first at all costs."

`CLAUDE SUBAGENT (CEO — strategic independence)`

- The plan optimizes the Rust rewrite more than the operator's time-to-value.
- Key assumptions, setup-first flow, hidden `.system/` truth, and `project + feature` sufficiency, are still bets, not facts.
- The six-month regret case is a two-system product with weak distribution and unclear external value.

CEO DUAL VOICES — CONSENSUS TABLE:

| Dimension | Claude | Codex | Consensus |
|---|---|---|---|
| Premises valid? | Concern | Concern | Confirmed concern |
| Right problem to solve? | Concern | Concern | Confirmed concern |
| Scope calibration correct? | Concern | Concern | Confirmed concern |
| Alternatives sufficiently explored? | Concern | Concern | Confirmed concern |
| Competitive / market risks covered? | Concern | Concern | Confirmed concern |
| 6-month trajectory sound? | Concern | Concern | Confirmed concern |

CEO consensus:

- `6/6` dimensions raised confirmed concern
- `0/6` dimensions produced a material disagreement

#### CEO Review Sections

##### Section 1: Architecture Review

Examined: `PLAN.md`, `README.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, `crates/cli/src/main.rs`, `crates/compiler/src/resolver.rs`.

Finding: the architecture is already partly shipped, but the plan still treats M2-M6 as future milestones. That is now a planning bug, because it hides the real problem: docs and runtime disagree on the support boundary.

##### Section 2: Error & Rescue Map

| Scenario | Current repo behavior | Risk | Required next action |
|---|---|---|---|
| User reads docs first | `docs/START_HERE.md` implies live planning is supported | Overclaim | Gate support claims on runtime truth |
| User reads CLI help first | help says scaffold / reserved placeholders | Underclaim | Align help with actual shipped surfaces |
| User runs `generate` without `.system/` | deterministic refusal with next safe action | Good | Preserve |
| User runs `generate` with ready inputs | placeholder body + non-zero exit | Support contradiction | Do not call planning packets supported yet |
| User needs setup | `system setup` placeholder, legacy bridge described only in docs | Split ownership | Define one canonical setup path |

##### Section 3: Security & Threat Model

Examined: `PLAN.md` storage and manifest sections, `crates/compiler/src/canonical_artifacts.rs`, `crates/compiler/src/freshness.rs`.

What was examined:

- symlink handling exists for `.system/` root and canonical artifacts
- no explicit plan-level trust model exists for malformed markdown, large files, encoding failures, or prompt-injection-style repo content

Flagged concern:

- the plan should label repo-controlled content as untrusted input and define validation boundaries explicitly

##### Section 4: Data Flow And Interaction Edge Cases

Examined: state table, command hierarchy, current CLI cwd behavior, demo invocation paths.

Flagged concerns:

- repo discovery is not specified, even though current CLI uses `current_dir()` as the effective repo root
- first-run routing is unresolved for new repo, initialized repo, stale repo, and unsupported repo
- partial success is underdefined for `generate`

##### Section 5: Code Quality Review

Examined: plan vocabulary and repeated command naming.

Flagged concern:

- `doctor` vs `health` appears in mutually inconsistent ways. The plan says `doctor` is canonical, then later says "`doctor` or `health`" is required. That will drift docs, code, and tests.

##### Section 6: Test Review

Examined: `cargo test -q`, CLI tests, renderer tests, install smoke.

Current strength:

- deterministic refusal, inspect, help ordering, and fixture-backed demo flows are tested

Current gap:

- there is no explicit test gate that says "do not claim supported planning packet generation until `generate` exits `0` with a non-placeholder body"

##### Section 7: Performance Review

Examined: plan claims and current freshness implementation.

Finding:

- the current performance posture is acceptable for reduced scope
- the plan still under-specifies large-repo behavior and whether repeated hashing remains request-scoped forever or only until profiling proves otherwise

##### Section 8: Observability & Debuggability Review

Examined: inspect output and doctor output.

Finding:

- `inspect` has a useful decision-log surface
- `doctor` still prints raw debug-style subject and next-action structs instead of the cleaner shared rendering language

##### Section 9: Deployment & Rollout Review

Examined: install smoke, help text, top-level docs.

Finding:

- install smoke passes locally
- rollout is still blocked on support-language consistency
- the release gate should be docs/runtime parity first, wider distribution second

##### Section 10: Long-Term Trajectory Review

Flagged concern:

- if unreconciled, this project will keep shipping contracts and tests that are more internally coherent than the product story users actually experience

#### CEO Mandatory Outputs

##### NOT in scope

- No full product-theory rewrite in this review
- No public distribution expansion beyond current local install smoke
- No widening of v1 into review/fix packets or live slice execution

##### What already exists

- Rust workspace
- compiler/CLI split
- manifest and freshness model
- refusal and blocker taxonomy
- markdown/json/inspect renderers
- fixture-backed execution demo
- CLI and renderer drift tests
- install smoke path

##### Failure Modes Registry

| Failure mode | Current status | Severity |
|---|---|---|
| Docs claim support before runtime does | Present | High |
| Setup path split across docs and placeholder CLI | Present | High |
| Packet-body contract missing from typed model | Present | High |
| `doctor` / `health` vocabulary drift | Present | Medium |
| Large-repo / malformed-input handling under-specified | Present | Medium |

##### Dream state delta

- The current repo is much closer to the desired architecture than the base plan suggests.
- The missing gap is no longer "build the Rust wedge." It is "close the gap between what shipped and what the product claims."

##### CEO Completion Summary

| Item | Status |
|---|---|
| Strategic framing updated to post-implementation audit | Done |
| Existing-code leverage mapped | Done |
| Error and rescue map produced | Done |
| Failure modes registry produced | Done |
| Scope-expansion decisions logged | Done |
| Unresolved strategic issue | Support boundary still overclaimed |

### Phase 2: Design Review

UI scope decision:

- Run as applicable.
- Rationale: this product is CLI-first, but the plan contains explicit information hierarchy, state coverage, accessibility, and output-layout rules. That is enough design scope for review.

#### Design Setup

- `DESIGN.md`: absent
- existing design leverage:
  - `docs/START_HERE.md`
  - `docs/SUPPORTED_COMMANDS.md`
  - `crates/compiler/src/rendering/*.rs`
  - `crates/cli/tests/cli_surface.rs`

#### Design Dual Voices

`CODEX SAYS (design — UX challenge)`

- The UX hierarchy still serves the system designer more than the operator.
- The plan speaks in good CLI principles, but not enough exact templates.
- Setup dominates too much of the story for a partially shipped wedge.

`CLAUDE SUBAGENT (design — independent review)`

- startup routing is ambiguous
- first-failure states are incomplete
- partial `generate` states and doctor terminal states are underdefined
- command templates are still pattern-level rather than concrete

DESIGN LITMUS SCORECARD:

| Dimension | Claude | Codex | Consensus |
|---|---|---|---|
| Information architecture | Concern | Concern | Confirmed concern |
| Interaction state coverage | Concern | Concern | Confirmed concern |
| User journey / emotional arc | Concern | Concern | Confirmed concern |
| Specificity of output templates | Concern | Concern | Confirmed concern |
| Recovery vocabulary consistency | Concern | Concern | Confirmed concern |
| Responsive / accessibility concreteness | Concern | Concern | Confirmed concern |
| Drift resistance | Concern | Concern | Confirmed concern |

Design consensus:

- `7/7` dimensions raised confirmed concern
- `0/7` dimensions produced a material disagreement

#### Design Passes

| Pass | Score | Review outcome |
|---|---|---|
| Information architecture | 5/10 | Setup is over-emphasized relative to the ready-path user |
| Interaction state coverage | 5/10 | State table is thoughtful but misses launch-state and partial-state details |
| User journey and emotional arc | 6/10 | Trust posture is clear, but momentum arrives too late |
| AI slop risk | 8/10 | Vocabulary is deliberate, not generic |
| Design system alignment | 6/10 | `doctor` / `health` inconsistency still creates drift risk |
| Responsive and accessibility | 6/10 | narrow-first intent is solid, but no concrete width/output acceptance rules |
| Unresolved design decisions | 4/10 | command templates and first-run routing remain ambiguous |

##### Design NOT in scope

- No visual mockups
- No ANSI styling system
- No alternate terminal UI layer

##### Design What already exists

- trust-header-first output ordering
- refusal and inspect structure
- CLI help ordering
- golden tests for header ordering and inspect JSON fallback

##### Design Completion Summary

- The CLI design language is directionally correct.
- The missing work is not aesthetic. It is exact templates, exact routing, and exact command-state closure.

### Phase 3: Engineering Review

#### Scope Challenge

This phase reviewed shipped code against the plan instead of assuming milestones were pending.

Primary engineering finding:

- support claims outran the current typed runtime model

#### Engineering Dual Voices

`CODEX SAYS (eng — architecture challenge)`

- there is no hard release gate for calling planning packet generation supported
- setup ownership is split
- render model does not yet contain packet body content
- repo discovery and doctor scope are under-specified

`CLAUDE SUBAGENT (eng — independent review)`

- untrusted repo content lacks an explicit trust boundary
- malformed / partial / high-volume repo states are under-specified
- state-transition testing is still missing for repair and retry flows

ENG DUAL VOICES — CONSENSUS TABLE:

| Dimension | Claude | Codex | Consensus |
|---|---|---|---|
| Architecture sound? | Concern | Concern | Confirmed concern |
| Test coverage sufficient? | Concern | Concern | Confirmed concern |
| Performance risks addressed? | Partial concern | Concern | Confirmed concern |
| Security threats covered? | Concern | Concern | Confirmed concern |
| Error paths handled? | Concern | Concern | Confirmed concern |
| Deployment risk manageable? | Concern | Concern | Confirmed concern |

Engineering consensus:

- `6/6` dimensions raised confirmed concern
- `0/6` dimensions produced a material disagreement

#### Architecture ASCII Diagram

```text
system CLI
  |
  +--> clap command parsing
  |
  +--> generate / inspect / doctor
          |
          +--> system_compiler::resolve(...)
                  |
                  +--> CanonicalArtifacts::load
                  +--> ArtifactManifest::generate
                  +--> compute_freshness
                  +--> evaluate_budget
                  +--> compute_refusal / compute_blockers
          |
          +--> build_output_model
                  |
                  +--> render_markdown / render_inspect / render_json

docs + contracts + tests
  |
  +--> claim support boundary
  +--> currently drift from CLI help / ready-path behavior
```

#### Code Quality Review

Flagged concerns:

- duplicate support vocabulary across `PLAN.md`, `README.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, and CLI help
- raw debug formatting in `doctor`
- typed output model still stops at metadata instead of packet content

#### Test Review

Current coverage examined:

- `crates/compiler/tests/resolver_core.rs`
- `crates/compiler/tests/rendering_surface.rs`
- `crates/cli/tests/cli_surface.rs`
- `crates/cli/tests/help_drift_guard.rs`

Test diagram:

| Codepath / UX flow | Current coverage | Gap |
|---|---|---|
| Missing `.system` refusal | Covered | None |
| Ready planning path | Covered only as placeholder-ready state | Need real packet-body success gate |
| Fixture-backed execution demo | Covered | None |
| Live execution refusal | Covered | None |
| Help ordering | Covered | None |
| Support-language parity across docs/help/runtime | Partially covered | Need docs/help/runtime golden |
| Retry after repair | Missing | Add |
| Partial `.system` tree | Missing | Add |
| Malformed markdown / encoding / large inputs | Missing | Add |
| Non-repo-root invocation behavior | Mentioned in plan, not defined | Define and then test |

Test plan artifact:

- `/Users/spensermcconnell/.gstack/projects/system/spensermcconnell-main-test-plan-20260406-215805.md`

#### Performance Review

What was examined:

- current freshness implementation hashes only the small canonical artifact set
- no cache layer exists

Finding:

- fine for reduced scope
- still needs an explicit large-repo policy before broader support claims

#### Engineering NOT in scope

- No new runtime code in this review
- No public distribution rollout
- No new packet families

#### Engineering What already exists

- deterministic manifest generation
- deterministic refusal/blocker sorting
- install smoke
- help drift guards
- demo packet path

#### Failure Modes Registry

| Failure mode | Current evidence | Severity |
|---|---|---|
| Supported planning path still returns placeholder body | `generate` ready path | High |
| Setup ownership split between docs and placeholder CLI | docs + `system setup` | High |
| Packet-body contract absent from typed render model | `RenderOutputModel` | High |
| Repo-root discovery undefined | CLI uses cwd | Medium |
| Untrusted repo input policy absent in plan | plan gap | Medium |
| Repair / retry transitions under-tested | test gap | Medium |

#### Engineering Completion Summary

| Item | Status |
|---|---|
| Actual code reviewed | Done |
| Architecture diagram produced | Done |
| Test diagram produced | Done |
| Test-plan artifact written | Done |
| Install smoke verified | Done |
| Critical unresolved issue | Planning packet support is overclaimed |

### Cross-Phase Themes

| Theme | Phases | Why it matters |
|---|---|---|
| Support boundary drift | CEO, Design, Eng | Docs, help, and runtime disagree about what reduced v1 actually supports |
| Setup ownership ambiguity | CEO, Design, Eng | Users cannot infer one canonical entry path from repo state |
| Missing concrete command templates | Design, Eng | Principles exist, but exact fields, states, and terminal outcomes are still underspecified |

### Decision Audit Trail

| # | Phase | Decision | Classification | Principle | Rationale | Rejected |
|---|---|---|---|---|---|---|
| 1 | Intake | Treat `PLAN.md` as a post-implementation audit target, not a greenfield build plan | Mechanical | Pragmatic | The repo already contains the milestones the plan still describes as future work | Continue pretending M1-M8 are wholly unshipped |
| 2 | CEO | Preserve the Rust-first wedge for this review, but defer broader product reframing | Taste | Bias toward action | The fastest high-signal fix is support-boundary cleanup, not a new product thesis | Full wedge rewrite during audit |
| 3 | Design | Run design review because CLI interaction language and state coverage are explicit plan surfaces | Mechanical | Completeness | CLI UX drift is central to this repo's current risk | Skip design because there is no browser UI |
| 4 | CEO / Design / Eng | Elevate docs/help/runtime parity as the highest priority gap | Mechanical | Completeness | Multiple sources overclaim support while runtime still returns placeholders | Treat parity as a docs-only cleanup |
| 5 | Design / Eng | Make `doctor` the only canonical recovery verb in v1 | Mechanical | Explicit over clever | One public recovery command is easier to test and document | Keep `doctor` / `health` ambiguity |
| 6 | Eng | Define "supported planning packet generation" as zero-exit, non-placeholder packet body, with matching docs/help | Mechanical | Explicit over clever | Current ready-path behavior is not honest enough to call supported | Continue claiming support based on metadata-only readiness |
| 7 | Eng | Add repo-discovery, malformed-input, and retry-after-repair transitions to the required test plan | Mechanical | Choose completeness | Current tests are good on static states and weak on recovery transitions | Keep only current happy-path and refusal coverage |

### User Override (2026-04-06)

The user chose the complete path for the main taste decision:

- prioritize finishing the ready-path packet body first
- then reconcile docs/help/runtime support language around the finished behavior

This overrides the lighter recommendation to narrow support claims first.

### Review Verdict

- This repo is farther along than the base plan says.
- Reduced v1 is not blocked on core Rust scaffolding anymore.
- Reduced v1 is blocked on truthfulness: the project must either narrow its support claims or finish the packet-body and setup-ownership surfaces it already advertises.

### P1 Closeout (2026-04-07)

- `system generate` now exits `0` on the ready path with a non-placeholder packet body for `planning.packet` and the fixture-backed demo packet.
- CLI help and support-facing docs were reconciled to the shipped reduced-v1 boundary.
- `Packet Body Contract` and `Support Boundary Reconciliation` were completed in `TODOS.md` without reopening historical milestones.
