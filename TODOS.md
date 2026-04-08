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

## CLI Product Interaction Design

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

**Open decisions:** Whether `setup` remains the visible front door while Rust setup is placeholder-only, how nested repos and uninitialized repos should route, what the first-run default path is, and how `doctor` versus `setup refresh` is chosen as the next safe action.

**Research needed:** Repo scan of current routing language in `PLAN.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, CLI help, install smoke scripts, and recovery-path tests. Compare current behavior against the intended hierarchy and note contradictions.

**Acceptance criteria:** There is one front-door model, one hierarchy, one startup-routing model, and a clear mapping from repo state to the correct first command and next safe action.

**Unlocks:** D3, D4, D5, D6
**Effort:** S
**Priority:** P1
**Depends on:** D1

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
