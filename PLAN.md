<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/system/main-autoplan-restore-20260406-212846.md -->
# PLAN

## Status

This is the implementation plan for the reviewed reduced v1 wedge.

It is derived from:

- [reviewed reduced-v1 seam pack](artifacts/planning/reduced-v1-seam-pack/README.md)
- CEO review decisions embedded in that seam pack
- engineering review decisions recorded in that seam pack's `## GSTACK REVIEW REPORT`

This plan is the current execution source of truth for the remaining reduced-v1 closure work: repo shape, cutover order, and operator-surface completion. The canonical repo-surface contract lives at [docs/contracts/C-01-approved-repo-surface.md](docs/contracts/C-01-approved-repo-surface.md).

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
- `doctor` is the required v1 recovery surface, not a post-v1 nicety.
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

## Current Implementation Snapshot

The old milestone tree is no longer the active execution plan.

These parts of reduced v1 are already implemented and intentionally removed from active planning scope:

- root Rust workspace with `crates/cli` and `crates/compiler`
- reduced-v1 command surface: `setup`, `generate`, `inspect`, `doctor`
- canonical `.system/` ingest for `CHARTER`, optional `PROJECT_CONTEXT`, and `FEATURE_SPEC`
- deterministic manifest generation, freshness truth, budget policy, refusal logic, blocker aggregation, and decision logging
- markdown, JSON, and inspect renderers driven by one typed resolver result
- ready-path planning packet generation with a real packet body
- fixture-backed execution demo generation and explicit live execution refusal
- nested-repo resolution and retry-after-repair behavior
- CI rails for `fmt`, `clippy`, `cargo test`, archive-boundary checks, and install smoke on `macOS arm64` and `Linux x86_64`
- reduced-v1 contracts, docs, CLI vocabulary, tone rules, output anatomy, operator journey review, and `DESIGN.md`

This file now tracks only the unfinished work required to close reduced v1 honestly.

## Remaining Reduced-v1 Work

### R1. Make `setup` a real handoff surface

Outcome:

- `system setup` remains placeholder-only, but no longer dead-ends
- the command names one exact current guided setup path
- the front door feels incomplete only once, not ambiguous every time

Work:

- update `system setup` output so it points to the current guided setup entry path directly
- align `README.md`, `docs/START_HERE.md`, and `docs/SUPPORTED_COMMANDS.md` with the same handoff wording
- add coverage that proves the handoff stays explicit in runtime output and docs

Exit criteria:

- a new operator can run `system setup` and know the exact next safe action without reading repo internals
- no support-facing doc implies that Rust setup already exists
- the handoff wording is stable across help text, docs, and runtime output

### R2. Finish the `doctor` product surface

Outcome:

- `doctor` reads like a finished recovery surface instead of an implementation dump
- blocked and ready states match the CLI interaction contract

Work:

- replace debug-shaped blocker rendering with human-facing trust-header output
- use stable operator language, especially `NEXT SAFE ACTION`
- make ready output say what is ready and why retrying `generate` is safe
- align `doctor` anatomy with [`docs/CLI_OUTPUT_ANATOMY.md`](docs/CLI_OUTPUT_ANATOMY.md) and [`DESIGN.md`](DESIGN.md)

Exit criteria:

- `doctor` blocked output leads with outcome, object, and next safe action
- `doctor` no longer exposes raw debug formatting in operator-facing output
- ready output is informative enough that an operator can confidently retry `generate`

### R3. Fix `inspect` ready-path handoff semantics

Outcome:

- `inspect` remains the proof surface
- the ready-path next action is intentional instead of self-referential

Work:

- replace the current ready-path next action with a non-self-referential handoff
- keep inspect dense and auditable
- add a regression test for the ready-path next-action wording

Exit criteria:

- `inspect` never tells the operator to run `inspect` while already in `inspect`
- proof ordering remains unchanged apart from the corrected handoff

### R4. Complete physical legacy cutover

Outcome:

- repo root reflects the approved product surface only
- legacy harness code and wrappers no longer live on the active root path

Work:

- move the frozen Python harness and wrappers under an explicit archived location
- update any remaining references that still assume the legacy harness lives at the root
- keep legacy material runnable as reference-only, but off the supported product surface

Exit criteria:

- the repo root reads as the Rust product surface plus approved docs and build infrastructure
- legacy Python remains available only as archived reference material
- no supported runtime path imports, shells out to, or wraps archived legacy code

## Workstreams

### Lane A: CLI interaction closure

Scope:

- `setup` handoff
- `doctor` surface alignment
- `inspect` next-action fix

Depends on:

- current Rust CLI surface
- current interaction docs and contracts

### Lane B: docs and cutover cleanup

Scope:

- support-story parity
- legacy archive move
- root cleanup

Depends on:

- Lane A wording decisions

## Execution Order

1. Finish `setup` handoff first so the front door becomes honest and actionable.
2. Finish `doctor` next because recovery is the largest remaining product gap.
3. Fix `inspect` ready-path handoff once the shared next-action language is final.
4. Do the physical legacy archive move after the supported operator story is fully closed.

## Risks

### Risk: the front door still feels fake

Mitigation:

- make `system setup` name one exact current guided path
- keep docs and help text verbatim-aligned with that path

### Risk: `doctor` keeps leaking implementation shape into product output

Mitigation:

- route output through the shared interaction contract
- add regression coverage for blocked and ready states

### Risk: legacy root clutter keeps confusing contributors

Mitigation:

- move legacy harness material out of the active root surface once CLI interaction closure lands
- keep archived material clearly labeled as reference-only

## Deliverables

- updated `PLAN.md` that reflects only unfinished reduced-v1 work
- explicit `setup` handoff wording in runtime output and docs
- `doctor` output aligned with the CLI interaction contract
- corrected `inspect` ready-path next action
- legacy harness moved under an archived location at cutover

## Definition Of Done For Reduced V1

- `system setup` names the exact current guided setup path
- `generate` remains the supported live planning packet surface
- `inspect` remains the proof surface without self-referential ready-path guidance
- `doctor` reports blockers and readiness using finished operator-facing output
- execution packet demo remains fixture-backed only
- unsupported live execution requests refuse clearly
- docs, help text, and runtime output match reality
- repo root reflects the approved Rust-first product surface
