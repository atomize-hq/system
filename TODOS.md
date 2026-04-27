# TODOS

## Context Compiler

### Thin MCP/UI Companion

**What:** Add a thin MCP/UI companion for setup progress, drift status, packet inspection, and health.

**Why:** The Rust core should stay CLI/library-first in v1, but a small steering surface will make the system much easier to operate once the packet, provenance, freshness, and health contracts are stable.

**Context:** The CEO review accepted a library-first Rust core with inspect output, structured decision traces, a health/status command, and explicit freshness/repair workflows. It explicitly deferred UI so the team does not ship a pretty wrapper around weak compiler guts. This work should start only after the Rust packet model, provenance headers, structured diagnostics, and health outputs are stable enough that a UI can attach without redefining contracts.

**Effort:** M
**Priority:** P2
**Depends on:** Stable Rust packet core, inspect/explain output, health/status command, structured diagnostics, stable packet provenance and policy contracts

### Review/Fix Packet Family

**What:** Add a post-v1 review/fix packet family that reuses lineage, standards, changed-surface detection, and failure context.

**Why:** The long-term product should support review and repair loops, not just planning and execution packet generation.

**Context:** The eng review for the v1 context compiler intentionally narrowed the first wedge to `project -> feature -> slice` lineage plus planning and execution packets. Review/fix packets were deferred to keep the first version focused and to avoid widening the packet resolver before packet quality is proven. This work should start only after the shared metadata schema, artifact index, source-of-truth ownership rules, refusal behavior, and v1 packet tests are stable.

**Effort:** M
**Priority:** P2
**Depends on:** Stable v1 packet resolver, shared metadata schema, artifact index, refusal-path tests

### Live Slice Lineage And Execution Packets

**What:** Add live slice lineage and real execution packet generation after the reduced v1 planning-packet wedge ships.

**Why:** The product promise is still planning plus execution packets, but reduced v1 intentionally proves the compiler first and only demos execution packets from fixtures.

**Context:** The current repo has implemented project and feature planning surfaces, but the slice stages listed in `pipeline.yaml` are still empty placeholders and `docs/legacy/stages/README.md` only documents stages through feature spec as implemented. The eng review narrowed v1 so execution packets are fixture-backed only, not a live supported flow. This follow-on should start once the Rust packet core, project/feature metadata contract, manifest/freshness logic, and planning-packet path are stable enough that a real `project -> feature -> slice` lineage can land without reopening the whole wedge.

**Effort:** M
**Priority:** P2
**Depends on:** Stable Rust packet core, stable project/feature metadata contract, manifest/freshness logic, successful v1 planning packet adoption

### Persisted Derived State After Profiling

**What:** Add persisted derived state or an on-disk manifest only if real usage or profiling shows the in-memory v1 path is insufficient.

**Why:** The review accepted request-scoped, in-memory manifest generation for v1 to avoid premature state-machine complexity, but repeated inspect or health workflows may later justify persisted derived state.

**Context:** Reduced v1 is intentionally small: a few canonical `artifact_inputs/`, deterministic freshness, and request-scoped packet resolution. Adding saved derived state now would create rebuild, cleanup, and concurrency complexity before there is evidence it helps. Revisit this only after stable v1 adoption or profiling shows repeated packet inspection and health checks are materially slowed by recomputing the same derived view.

**Effort:** M
**Priority:** P3
**Depends on:** Stable v1 packet resolver, real usage data, profiling evidence

### Public CLI Distribution

**What:** Add public CLI distribution and release packaging after the local install path is stable.

**Why:** V1 only commits to explicit local installation on supported development targets, but a real CLI product will eventually need repeatable release artifacts, installation instructions, and update mechanics.

**Context:** The review locked v1 distribution to local Rust CLI installation on `macOS arm64` and `Linux x86_64`, with package-manager and public publishing work explicitly deferred. Once CLI UX and install smoke are stable, the next step is defining release artifacts, supported targets, checksums, and a boring installation/update path that does not require cloning the repo.

**Effort:** M
**Priority:** P3
**Depends on:** Stable CLI UX, stable install smoke on supported targets, release channel decision

### CLI Release Workflow

**What:** Add a GitHub Actions release workflow that builds and publishes versioned CLI artifacts for the supported targets.

**Why:** The Rust CLI now ships as a real product surface, but this branch intentionally deferred distribution automation. Without a release workflow, users still need the repo and local toolchain to install it.

**Context:** `/ship` detected the new standalone CLI surface and the existing CI only validates build quality. It does not publish downloadable artifacts, checksums, or tagged releases. The local install smoke is now in place, so the next concrete step is a boring release workflow that packages the CLI for `macOS arm64` and `Linux x86_64` and attaches those artifacts to tagged releases.

**Effort:** S
**Priority:** P1
**Depends on:** Stable install smoke, release artifact naming, version tag convention

### Live Authoring Smoke Coverage

**What:** Add real `codex exec` smoke coverage for the live authoring paths instead of relying mostly on stubbed runtime tests.

**Why:** The current stubbed tests catch a lot of CLI and validation regressions, but they do not prove that the real subprocess, prompt handoff, model invocation, and canonical write path still work together end-to-end.

**Context:** The live authoring surfaces now include real `codex exec` integration for authored canonical truth, but most review-time and CI proof still comes from fake binaries and stubbed outputs in the compiler and CLI test suites. That is good for deterministic coverage, but it leaves a gap where real-model invocation failures, prompt-contract drift, or subprocess integration breakage can land without being exercised by a true live smoke. This follow-on should add a bounded real-runtime smoke path for the shipped live authoring flows and keep it explicit about required env/model configuration.

**Effort:** S
**Priority:** P2
**Depends on:** Stable live authoring command surface, available `codex exec` credentials/model configuration, bounded smoke-test budget in CI or release checks

### Post-Setup Onboarding Upgrade

**What:** Extend the `setup` success path beyond the current scaffolded-or-ready handoff with a richer onboarding flow once the Rust front door is stable.

**Why:** `M6` can honestly end at `system doctor`, but that still leaves a lot of operator guidance value on the table. After the front door is real, the next improvement is a tighter onboarding path that helps the operator move from scaffolded `.system/` files to a ready planning flow with less guesswork.

**Context:** The current setup family now establishes or refreshes canonical `.system/` truth, reports `SCAFFOLDED` while required starter templates still need real content, and reports `READY` once the repo can hand off to `system doctor`. That keeps `M6` bounded and honest. It also intentionally defers a richer onboarding experience so the team does not mix front-door truth establishment with a bigger guidance or workflow-orchestration redesign. Revisit this after the Rust `setup` family is shipped, docs/help drift is updated, and the team has real usage feedback on where operators still get stuck after the current scaffolded-or-ready handoff.

**Effort:** S
**Priority:** P2
**Depends on:** Shipped Rust `setup` family, updated `doctor` readiness guidance, real operator feedback on post-setup friction

### Claude Code Conversational Intake Surface

**What:** Add the second conversational-intake agent surface for Claude Code after the Codex-first charter slice lands.

**Why:** The target architecture is a portable conversational intake protocol, not a Codex-only wrapper. Proving Claude Code next prevents Codex-specific behavior from silently becoming the protocol.

**Context:** The accepted eng-review scope for the first slice is explicit: one preferred-agent surface first, Codex now and Claude Code next. This follow-on should reuse the same canonical protocol assets, the same `~/.system` thin adapter install/update path, the same setup-state routing through `system setup*` and `system doctor`, and the same deterministic sink via `system author ... --from-inputs`. It should validate portability without reopening slice-1 scope or rebuilding per-agent business logic.

**Effort:** S
**Priority:** P1
**Depends on:** Shipped Codex-first conversational intake slice with schema versioning, live smoke coverage, and `~/.system` adapter compatibility checks

### Operator Outcome Scoreboard

**What:** Add an operator-outcome scoreboard for the pipeline/compiler wedge.

**Why:** The product goal is reduced operator work, not just parser correctness. The team should be able to measure whether the Rust spine actually removes manual steps, repeated inputs, and oversized downstream grounding.

**Context:** The CEO review intentionally kept this out of `M1` so the first wedge can ship on hard compiler, contract, and proof-corpus gates. Once the first wedge is stable, the next useful step is a small scoreboard that measures human outcomes instead of only technical behavior. Good candidate metrics are manual steps removed, repeated routing/context inputs removed, and grounding size reduction for downstream planning consumers.

**Effort:** S
**Priority:** P2
**Depends on:** Stable foundation-family wedge, locked proof corpus, repeatable command flows

### Pipeline Validate Surface

**What:** Add a first-class `system pipeline validate --id <pipeline>` preflight surface.

**Why:** Operators should be able to check duplicate ids, shorthand ambiguity, activation drift, and schema issues before they hit a real `resolve` or `compile` refusal in the middle of work.

**Context:** The CEO review required immediate activation-drift enforcement and strict validation behavior, but deferred the operator-facing validation command so the first wedge can stay focused on route truth and stage compilation. This follow-on should package the existing validation rules into one boring preflight command once the core `pipeline` family is stable.

**Effort:** S
**Priority:** P2
**Depends on:** Stable `pipeline` family, stable validation rules, compiler-owned typed identity layer

### Activation Evaluator Expansion

**What:** Expand the `pipeline` activation evaluator beyond the narrow `M1` subset when real pipeline maturity requires it.

**Why:** `M1` intentionally supports only `when.any`, `when.all`, and variable-path equality against boolean literals. That keeps the first wedge boring and auditable, but future pipelines may eventually need string or numeric equality once real usage justifies the added surface area.

**Context:** The eng review locked `M1` to the smallest activation subset that covers the current foundation-family proof corpus: `when.any` / `when.all` plus boolean equality in the form `variables.<name> == true|false`. Anything broader was treated as accidental complexity for the first wedge. Revisit this only when real pipeline definitions, repeated refusals, or operator pain show that the boolean-only evaluator is blocking useful workflows, and expand it with explicit schema/tests rather than ad hoc compatibility hacks.

**Effort:** S
**Priority:** P2
**Depends on:** Stable `pipeline` route core, stable `M1` proof corpus, evidence that real pipeline usage needs broader activation semantics

### Structured Run Provenance

**What:** Add structured run provenance for `pipeline resolve`, `pipeline compile`, and `pipeline state set`.

**Why:** When a route changes, a compile refuses, or a state mutation causes confusion, the product should provide enough structured provenance to reconstruct what happened without guesswork.

**Context:** The CEO review accepted the need for structured run provenance but deferred it out of `M1`. Mutation audit history remains required for `pipeline state set` now. This follow-on extends that observability posture across command runs so failures can be classified by route truth, compile payload, schema/validation, or proof rendering instead of leaving the operator to reconstruct history from raw terminal output.

**Effort:** S
**Priority:** P2
**Depends on:** Stable `pipeline` command family, stable refusal classes, stable proof corpus and failure classification

### Capture Apply Multi-Writer Safety

**What:** Strengthen `pipeline capture apply` from the current `system`-coordinated single-writer rollback model to a safer boundary for arbitrary concurrent repo writers touching the same output paths.

**Why:** The shipped `M3`/`M3.5` posture is intentionally exact: capture apply locks pipeline route state and rolls back its own writes, but it does not claim full protection if some unrelated external process edits the same artifact or repo-file targets mid-apply. That narrower claim is correct today, but the stronger boundary is still valuable technical debt if this surface becomes more heavily used.

**Context:** `/autoplan` on 2026-04-15 explicitly narrowed the capture transactionality claim in `PLAN.md` and `docs/contracts/pipeline-capture-preview-and-apply.md` to `system`-coordinated single-writer flows. This follow-on is the work required to move beyond that exact-but-limited guarantee. Good candidate directions include file-level compare-and-swap semantics, stronger per-target locking, or another explicit writer-coordination boundary that can be proven in tests and stated honestly in contracts/docs.

**Effort:** M
**Priority:** P3
**Depends on:** Stable capture surface, clear target-path ownership rules, added concurrency test strategy

## CLI Product Interaction Design

### Chosen Interaction Direction (2026-04-08)

The interaction direction for the CLI product is:

- audited utilitarian
- narrow-terminal first
- exact and boring in the good way
- explicit about shipped versus transitional behavior
- optimized for both humans and agents

What this means for task design:

- do not add new top-level verbs to compensate for weak handoffs
- prioritize fixing `setup`, `inspect`, and `doctor` transitions over copy polish elsewhere
- treat `doctor` as a product surface, not just a diagnostic helper
- keep proof dense and auditable rather than conversational

Recommended implementation order for the remaining interaction backlog:

1. R2, because it is a small fix with immediate trust payoff
2. R3, because the front door should stop dead-ending
3. R1, because it is the largest remaining product-shape gap

### Phase 1

#### D1: Canonical Product Vocabulary

**What:** Lock the canonical operator-facing product vocabulary for the CLI, docs, examples, and runtime output.

**Why:** This is the first dependency for every later interaction-design artifact. If the nouns and verbs drift, command hierarchy, tone, output anatomy, and the CLI interaction contract all become inconsistent and hard to automate cleanly.

**Context:** `PLAN.md` already establishes the preferred language for reduced v1, including `setup`, `generate`, `inspect`, `doctor`, `canonical artifacts`, `derived views`, `refusal`, and `next safe action`. That guidance exists, but it still needs to be turned into an explicit decision artifact with coverage rules and repo-surface parity checks.

**Target artifact:** Source-of-truth vocabulary document plus parity updates for `README.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, CLI help, contracts, and future `DESIGN.md`.

**Open decisions:** Which terms are mandatory, which legacy synonyms are explicitly banned, where operator-facing wording may differ from internal implementation wording, and which phrases must remain stable for tests and automation.

**Research needed:** Repo scan for current vocabulary drift across docs, help text, contracts, snapshots, tests, and examples. No external research required unless the team wants calibration against other trust-heavy CLIs.

**Acceptance criteria:** There is one explicit vocabulary source of truth, all top-level user surfaces map to it, banned synonyms are documented, and later D-items can reference this artifact instead of re-deciding language.

**Unlocks:** D2, D3, D4, D5, D6
**Effort:** S
**Priority:** P1
**Depends on:** None
**Status:** Complete (2026-04-08)

#### D2: Command Hierarchy And Front Door

**What:** Define the canonical command hierarchy and startup routing model, including the true front door for new, initialized, stale, and blocked repos.

**Why:** Once the vocabulary is locked, the next operator decision is navigation. The user should know in the first few seconds whether they are establishing truth, generating a packet, proving a result, or repairing a broken state.

**Context:** `PLAN.md` already argues for setup first, packet generation second, proof third, and repair fourth. The remaining work is to convert that into a concrete hierarchy and routing contract that docs, help text, examples, and runtime behavior all follow.

**Target artifact:** Source-of-truth command hierarchy and entry-routing document plus downstream parity updates for docs, help text, onboarding examples, and tests.

**Resolved decisions:** `setup` is the visible front door, bare `system setup` routes to `setup init` for absent or invalid canonical `.system/` truth and to `setup refresh` otherwise, and setup-family next safe actions now distinguish scaffolded repos from ready repos.

**Research needed:** Repo scan of current routing language in `PLAN.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, CLI help, install smoke scripts, and recovery-path tests. Compare current behavior against the intended hierarchy and note contradictions.

**Acceptance criteria:** There is one front-door model, one hierarchy, one startup-routing model, and a clear mapping from repo state to the correct first command and next safe action.

**Unlocks:** D3, D4, D5, D6
**Effort:** S
**Priority:** P1
**Depends on:** D1
**Status:** Complete (2026-04-08)

### Phase 2

#### D3: Tone Rules For Docs, Help, And Runtime

**What:** Define the canonical tone rules for operator-facing communication across README/docs, CLI help, command success output, refusal output, proof surfaces, and recovery surfaces.

**Why:** Once vocabulary and hierarchy are fixed, the next source of drift is voice. The product is a trust tool, so the tone has to be strict but guided, terse but not cryptic, and stable enough to automate.

**Context:** `PLAN.md` already says the output should be strict but guided, never chatty, never cryptic, and should avoid filler when the exact blocker is known. This needs to become an enforceable tone contract instead of a loose preference.

**Target artifact:** Tone-rules source of truth for docs/help/runtime plus wording examples for success, refusal, proof, and recovery states.

**Open decisions:** How terse the default output should be, what words are reserved for blocked states, what filler phrases are banned, when guidance appears inline versus deferred to `inspect` or `doctor`, and how help text differs from runtime output without drifting in tone.

**Research needed:** Repo scan for existing tone drift across README/docs/help/runtime and test fixtures. Optional external research on trust-heavy CLIs only if useful for calibration, not imitation.

**Acceptance criteria:** One explicit tone contract exists, example wording covers the main command states, and future generated docs/help/output can be checked against it.

**Unlocks:** D4, D5, D6
**Effort:** S
**Priority:** P1
**Depends on:** D1, D2
**Status:** Complete (2026-04-08)

#### D4: Output Anatomy For Success, Refusal, Proof, And Recovery

**What:** Define the exact output anatomy for success, refusal, proof, and recovery across the CLI surfaces.

**Why:** Tone alone is not enough. Operators need stable information ordering so the first three lines already answer outcome, object of interest, and next action. This is also the layer where narrow-terminal behavior and screen-reader order become real.

**Context:** `PLAN.md` already describes a narrow-first output strategy, with packet identity first, then included sources, then omission/budget notes, then next actions. It also says refusal should stay compact while `doctor` owns the full blocker report. That structure needs to be turned into an explicit render contract.

**Target artifact:** Output-anatomy source of truth covering `setup`, `generate`, `inspect`, and `doctor`, with state templates for ready, missing, stale, contradictory, unsupported, and repaired outcomes.

**Open decisions:** What the first three lines must contain per command, which sections are mandatory versus conditional, where machine-readable fallbacks appear, how narrow-terminal truncation behaves, and how proof output differs from diagnostic dumps.

**Research needed:** Repo scan of current renderer behavior, snapshot tests, CLI help examples, and contracts `C-04` through `C-06`. Identify where behavior already matches the intended anatomy and where it does not.

**Acceptance criteria:** Each command state has a stable section order, compact versus full-report boundaries are explicit, and the structure is precise enough to drive fixtures, tests, and docs generation.

**Unlocks:** D5, D6
**Effort:** M
**Priority:** P1
**Depends on:** D1, D2, D3
**Status:** Complete (2026-04-08)

#### D5: CLI Interaction Contract (`DESIGN.md`)

**What:** Create a `DESIGN.md` that acts as the CLI product interaction contract rather than a visual brand document.

**Why:** The repo needs one place future agents can read before changing docs, help text, examples, fixtures, or runtime copy. Without that contract, every edit risks re-litigating vocabulary, hierarchy, tone, and output structure.

**Context:** This product is CLI-only right now. The design problem is interaction design, trust signaling, and recovery UX, not colors or typography. `DESIGN.md` should therefore codify operator experience rules, not invent a fake visual system for a terminal product.

**Target artifact:** `DESIGN.md` in the repo root, written as the canonical CLI interaction contract and referenced from `CLAUDE.md` and related repo guidance.

**Open decisions:** The exact structure of `DESIGN.md`, what it owns versus what contracts own, whether it cites or incorporates D1-D4 directly, and which parts are normative for tests versus advisory for prose.

**Research needed:** Minimal external research. Primarily synthesize D1-D4 plus current plan/contracts into one durable source of truth.

**Acceptance criteria:** `DESIGN.md` exists, clearly defines the CLI interaction contract, points to the right dependent contracts, and is usable as the preflight read for future product-facing changes.

**Unlocks:** D6 and future automated docs/help/runtime generation
**Effort:** S
**Priority:** P1
**Depends on:** D1, D2, D3, D4
**Status:** Complete (2026-04-08)

### Phase 3

#### D6: Operator Journey From First Run To Trust Recovery

**What:** Define and pressure-test the full operator journey from first run through setup, generation, proof, refusal, repair, and successful retry.

**Why:** This is the final conformance pass. It should not just describe the journey, it should expose where D1-D5 still feel wrong, incomplete, or contradictory when experienced as one product.

**Context:** `PLAN.md` already sketches the emotional arc: confidence, momentum, then controlled caution. The remaining work is to turn that arc into a concrete validation pass that flags revisions required in the previously landed interaction artifacts.

**Target artifact:** Operator-journey and conformance-review document, plus a revision backlog against D1-D5 if the journey reveals drift or weak transitions.

**Open decisions:** Which operator scenarios are canonical, which failure/recovery loops must be covered, what counts as a journey-breaking contradiction, and whether new routing or output revisions are required before calling the interaction design stable.

**Research needed:** End-to-end walkthrough of repo docs, help text, command states, and recovery loops. Optional external benchmarking against trust-heavy developer tools if a gap needs calibration.

**Acceptance criteria:** The full journey is mapped, each step references the controlling D-artifacts, contradictions are turned into explicit revision items, and the result acts as a final fit-and-finish audit rather than a disconnected narrative.

**Unlocks:** Iterations and revision tasks for D1-D5
**Effort:** M
**Priority:** P1
**Depends on:** D1, D2, D3, D4, D5
**Status:** Complete (2026-04-08)

### D6 Revision Backlog

#### R1: Align `doctor` To The CLI Interaction Contract

**What:** Replace the current raw debug-shaped `doctor` output with the trust-header, shared-language recovery surface described by D3-D5.

**Why:** `doctor` is the clearest shipped mismatch against the CLI interaction contract. It is functionally correct, but it still prints implementation-shaped subject and next-action data instead of a finished recovery experience.

**Product target:** `doctor` should feel like the operator can recover from a bad state without reverse-engineering Rust enums or guessing the retry order.

**Implementation scope:**
- add a stable trust header to both blocked and ready `doctor` output
- choose one stable object label for the surface and use it consistently
- replace debug-shaped `SUBJECT: Policy { ... }` and `NEXT ACTION: CreateSystemRoot { ... }` output with human-facing shared renderers
- switch runtime wording from `NEXT ACTION` to `NEXT SAFE ACTION`
- make the ready state more informative than a bare `READY`
- update docs and conformance notes so the shipped anatomy no longer claims a transitional exception after this lands

**Primary files:** `crates/cli/src/main.rs`, `crates/compiler/src/rendering/shared.rs`, `docs/CLI_OUTPUT_ANATOMY.md`, `docs/CLI_OPERATOR_JOURNEY.md`, `docs/SUPPORTED_COMMANDS.md`, `crates/cli/tests/`, `crates/compiler/tests/`

**Acceptance criteria:**
- blocked `doctor` output starts with `OUTCOME`, `OBJECT`, and `NEXT SAFE ACTION`
- blocker groups use human-facing labels and do not contain `{:?}` debug rendering
- the first blocker and the global handoff agree on the safest retry path
- ready `doctor` output confirms readiness and points cleanly back to `generate`
- docs, help, and tests all describe the upgraded surface consistently

**Test coverage:**
- snapshot or rendering coverage for ready and blocked `doctor`
- drift-guard coverage that fails if debug-shaped blocker output or `NEXT ACTION` wording reappears
- retry-path coverage proving the next safe action is stable for missing `.system/`, malformed artifacts, and ready repos

**Effort:** M
**Priority:** P1
**Depends on:** D6 findings

#### R2: Fix `inspect` Ready-Path Next Action

**What:** Replace the current self-referential ready-path next action in `inspect` with a semantically correct handoff.

**Why:** `inspect` currently tells the operator to run `inspect` for proof while they are already in `inspect`. The rest of the proof surface is strong, but that line makes the product feel templated instead of intentional.

**Product target:** once the operator is already on the proof surface, the handoff should point back to the next productive step, not back into the same proof view.

**Implementation scope:**
- make `inspect` compute its own ready-path handoff instead of reusing the `generate` proof handoff unchanged
- preserve fixture context when the inspected packet is `execution.demo.packet`
- keep the rest of the proof ordering unchanged unless the fix exposes a real contradiction
- update the output anatomy and operator-journey docs so the contract names the corrected handoff

**Primary files:** `crates/compiler/src/rendering/inspect.rs`, `crates/compiler/src/rendering/shared.rs`, `crates/compiler/tests/rendering_surface.rs`, `docs/CLI_OUTPUT_ANATOMY.md`, `docs/CLI_OPERATOR_JOURNEY.md`

**Acceptance criteria:**
- ready `inspect` output no longer instructs the operator to run `inspect`
- the replacement next safe action is semantically correct for both planning and execution-demo packets
- `generate` ready output is unaffected and can still point to `inspect` for proof
- docs and tests encode the new handoff explicitly

**Test coverage:**
- update rendering snapshots for planning and execution-demo inspect success paths
- add a regression assertion that no ready `inspect` output contains `run \`system inspect`

**Effort:** S
**Priority:** P1
**Depends on:** D6 findings

## Post-Implementation Audit Follow-Ups

### Support Boundary Reconciliation

**What:** Reconcile `PLAN.md`, `README.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, CLI help, and runtime behavior so reduced v1 does not claim supported planning packet generation before `system generate` exits `0` with a non-placeholder packet body.

**Why:** The current repo is caught between underclaim and overclaim. Docs say live planning packet resolution is supported, while CLI help still describes a scaffold and the ready path still returns placeholder body text.

**Effort:** S
**Priority:** P1
**Depends on:** Finished ready-path packet body
**Status:** Complete (2026-04-07)

### Setup Ownership And Entry Routing

**What:** Define one canonical setup ownership boundary and one startup routing model for new repo, initialized repo, stale repo, and unsupported repo.

**Why:** The current plan and docs split setup authority between the legacy scaffold and the Rust CLI, which leaves the operator without one obvious front door.

**Effort:** S
**Priority:** P1
**Depends on:** Support-boundary reconciliation

### Packet Body Contract

**What:** Add a typed packet-body contract to the compiler output model, or explicitly narrow reduced-v1 claims until that contract exists.

**Why:** The current rendering model carries trust metadata, decision evidence, refusals, and blockers, but not the actual planning packet body that docs imply is already supported.

**Effort:** M
**Priority:** P1
**Depends on:** None

**Chosen direction (2026-04-06):** Finish the ready-path packet body first. This is the selected path from the post-implementation `/autoplan` review.
**Status:** Complete (2026-04-07)

### Repo Discovery And Recovery Transition Tests

**What:** Define repo discovery semantics and add tests for retry-after-repair, partial `.system/` trees, malformed inputs, and docs/help/runtime vocabulary drift.

**Why:** Current coverage is strong on deterministic static states, but weak on state transitions and normal operator invocation paths.

**Effort:** M
**Priority:** P2
**Depends on:** Setup ownership decision, packet body contract direction

## Completed

### Canonical `.system/` Bootstrap Flow

**What:** Shipped the Rust-owned setup family that creates or refreshes canonical repo-local `.system/` truth through `system setup`, `system setup init`, and `system setup refresh`.

**Why:** `generate`, `inspect`, and `doctor` already depended on canonical `.system/` inputs, so the product needed a real front door instead of a placeholder setup surface.

**Completed:** v0.5.2.0 (2026-04-18)
