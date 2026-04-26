---
title: "GStack Pattern Synthesis for Substrate — Consolidated Final"
date: "2026-04-18"
status: "consolidated-reference"
primary_source: "garrytan/gstack"
secondary_sources:
  - "Every compound-knowledge-plugin"
  - "Compound Engineering"
  - "LangGraph memory docs"
  - "MemGPT"
  - "Generative Agents"
  - "Reflexion"
purpose: "All-encompassing baseline reference for later alternatives research on intake/planning completeness, memory, persistence, and execution structure."
notes:
  - "This is a synthesis document, not an RFC."
  - "It merges the original baseline with a second deep-research packet and fills verified gaps."
---

# GStack Pattern Synthesis for Substrate — Consolidated Final

## What this final pass adds

- Compiled template/resolver architecture as a first-class pattern.
- The shared preamble as a runtime kernel for retrieval, session awareness, and operational policy.
- Checkpoint/resume as a distinct artifact type, not a side effect.
- Builder/developer profile memory and the ongoing migration from `builder-profile.jsonl` to `developer-profile.json`.
- Global retro and cross-tool discovery as a separate meta-memory plane.
- Autoplan’s explicit phase gates, required outputs, review report footer, and on-disk decision audit trail.
- Telemetry separation and version-sensitive operational nuances.
- LangGraph memory taxonomy and Reflexion as adjacent references alongside MemGPT and Generative Agents.

## Purpose

This document is the consolidated baseline reference for later substrate research. It does not try
to choose a final architecture. It distills the essential patterns, methods, structures, caveats,
and adjacent concepts uncovered while studying gstack, then organizes them so later alternatives
documents can compare against a stable baseline.

## Core synthesis

- gstack is best understood as a compiled artifact-first workflow runtime, not a bag of prompts.
- Its strongest abstraction is the separation between methodology, runtime metadata, and persisted artifacts.
- A “complete” plan in gstack is not a filled template; it is a plan that has passed the required review axes for its task class, under an explicit mode, with mandatory Step 0 outputs and no unresolved critical gaps.
- Its durable memory system is layered: session awareness, project-local ledgers, checkpoints, review memory, behavioral profile memory, and global retro/meta-memory.
- Compounding happens because the right slice of prior artifacts is retrieved before the next skill runs, and because the system makes that compounding visible.
- The repo’s append-only, local-first persistence model is a major strength for auditability and hackability, but it also reveals where substrate should add stronger typing and validation.
- The repo is currently in a mild migration state around builder-profile versus developer-profile, which is itself a useful pattern study for backward-compatible evolution.
- The main ceiling in gstack is not planning taste; it is deterministic verification of machine-checkable claims.

## Confidence legend

- **High** — explicit in multiple gstack files/scripts or explicit in a core workflow file.
- **Medium** — explicit in one major source or strongly implied by multiple related sources.
- **Adjacency** — not a gstack fact; included because it sharpens or extends the baseline.

## Operating model extracted from gstack

| Layer | What lives there | Why it matters |
|---|---|---|
| Methodology | Human-authored `SKILL.md.tmpl` workflow logic | Keeps the judgment-heavy process legible and editable |
| Runtime metadata | Generated preambles, dashboards, test/bootstrap, optional cross-model blocks, host-aware setup | Keeps operational details synchronized with code and shared across skills |
| Persisted artifacts | Design docs, review logs, learnings, timelines, checkpoints, behavioral profile, retros | This is where value survives sessions and compounds |

The most portable abstraction here is that gstack separates the workflow method from the runtime
scaffolding and from the durable artifacts. That triad is more important than any single prompt in
the repo. Refs: [G01], [G02], [G07], [G09], [G10], [G18], [G20]

## Session-type map extracted from gstack

| Session type | Primary role | Typical input | Required output | Primary completeness axis |
|---|---|---|---|---|
| `/office-hours` | intake, premise challenge, alternatives generation | user request + local repo context + prior designs/profile | design doc + profile event | problem clarity, alternatives, premise challenge |
| `/plan-ceo-review` | strategy/scope review | design doc / plan | reviewed plan + strategic outputs | right problem, right scope, ambition, reversibility |
| `/plan-eng-review` | engineering completeness review | design doc / plan | reviewed plan + failure/test/execution artifacts | architecture, tests, failure modes, implementation shape |
| `/plan-design-review` | UX/design completeness review | plan + optional designs/system docs | reviewed plan + design pass outputs | IA, states, journey, responsiveness, accessibility |
| `/plan-devex-review` | developer experience completeness review | plan + docs + developer-facing scope | reviewed plan + DX artifacts | onboarding, docs, API ergonomics, TTHW, persona fit |
| `/autoplan` | orchestrated synthesis and gating | design doc + skill files + plan file | living reviewed plan + review logs + audit trail | multi-axis completeness with explicit handoffs |
| `/review` | implementation diff review | git diff + prior learnings + prior review memory | merged findings + persisted review result | shipped-code risk, regressions, production failure exposure |
| `/checkpoint` | save/resume working state | current branch, working tree, conversation state | checkpoint artifact | continuity and resumability |
| `/learn` | inspect and curate learnings | project learnings ledger | searched/pruned/exported learnings | memory hygiene |
| `/retro global` | cross-project behavioral synthesis | discovered local AI-coding session metadata + repos | global retro snapshot | meta-memory and operating pattern visibility |

## Structural inventory worth carrying forward

| Class | What question it answers | Scope | Typical producer(s) | Typical consumer(s) | Notes |
|---|---|---|---|---|---|
| `design_doc` | What problem are we solving, under what assumptions, with what alternatives? | branch / feature | /office-hours | /plan-*, /autoplan | supports supersedes lineage; branch-scoped durable intake artifact |
| `builder_or_developer_profile_event` | What do we know about the builder/operator across sessions? | user / machine | /office-hours, developer-profile tooling | future office-hours sessions, behavioral tuning | currently mid-migration from builder-profile.jsonl to developer-profile.json |
| `reviewed_plan` | What exactly are we building and why? | branch / feature | /plan-*, /autoplan | implementation, /ship, future sessions | living object with review report, phase outputs, and decision audit trail |
| `error_and_rescue_registry` | How do non-happy paths fail and recover? | plan-local | /plan-ceo-review, /autoplan | implementers, QA, future reviews | focuses on user-visible rescue paths, not just exceptions |
| `failure_modes_registry` | What realistic production failures still exist? | plan-local | /plan-ceo-review, /plan-eng-review | implementation, QA | used to surface silent-failure critical gaps |
| `test_plan_artifact` | What must be tested and where? | branch / feature | /plan-eng-review | /qa, implementation | bridges planning into validation |
| `checkpoint` | Where are we right now, what was decided, and what remains? | branch / session | /checkpoint | resume logic, context recovery | captures decisions made, remaining work, notes, modified files, duration |
| `learnings_ledger` | What do we know now that should compound later? | project, optionally cross-project | many skills, manual /learn | many skills via preamble or explicit search | typed, source-labeled, confidence-scored, trust-gated |
| `timeline_ledger` | What happened, in what order, on what branch? | project | all skills automatically | retro, context recovery | activity memory, not semantic memory |
| `review_log` | Which gates have run, on which commit, and with what unresolved risk? | branch | /plan-*, /review, /autoplan | /ship, dashboards, staleness checks | release-gating memory |
| `decision_audit_trail` | What autonomous decisions were made, under which principle? | plan-local | /autoplan | humans, future auditors | kept on disk to avoid conversation-context bloat |
| `global_retro_snapshot` | What patterns recur across repos, tools, and time windows? | user / machine | /retro global | weekly retros, operating-pattern analysis | distinct from project memory |
| `telemetry_analytics` | How is the product being used? | product analytics | skill preambles when enabled | maintainers | should be kept separate from operational memory |

## Essential pattern catalog

### Workflow architecture patterns

#### P01 — Compiled methodology / runtime / artifact separation
**Confidence:** High

**Observed in gstack:** gstack keeps the judgment-heavy workflow in `SKILL.md.tmpl`, then compiles live `SKILL.md` files using `gen-skill-docs.ts` and resolvers that inject preambles, dashboards, test bootstrap, and optional cross-model blocks.

**Why it matters:** This reduces prompt drift without freezing the methodology into code. The method stays hand-authored; the operational sections stay synchronized with implementation.

**Carry-forward extraction:** Treat substrate planning/intake as a compiled prompt DSL or template system. Separate methodology, runtime metadata, and persisted outputs as different layers.

**Refs:** [G01]

#### P02 — Shared preamble as runtime kernel
**Confidence:** High

**Observed in gstack:** Every skill starts with a generated preamble that performs update checks, session tracking, active-session counting, contributor mode, universal question formatting, search-before-building policy, learning lookup, and timeline writes.

**Why it matters:** This makes important runtime behaviors consistent across all skills and gives memory retrieval a predictable insertion point.

**Carry-forward extraction:** Implement a policy-layer preamble in substrate that runs before task-specific logic: context recovery, session awareness, retrieval, telemetry gating, and standardized user decision formatting.

**Refs:** [G01], [G10], [G21]

#### P03 — Artifact-first session typing
**Confidence:** High

**Observed in gstack:** gstack sessions are defined less by chat persona and more by what artifact they transform: office-hours creates a design doc, plan reviews enrich it, review logs persist gating results, checkpoint captures working state.

**Why it matters:** This creates typed handoffs instead of loosely related conversations.

**Carry-forward extraction:** Define substrate session kinds as artifact transforms with required inputs, outputs, completeness gates, freshness rules, and persistence rules.

**Refs:** [G02], [G03], [G04], [G07], [G11]

#### P04 — Intake emits multiple related artifacts, not one plan blob
**Confidence:** High

**Observed in gstack:** `/office-hours` appends a builder/developer-profile event and writes a design doc; downstream reviews discover that design automatically.

**Why it matters:** Different artifacts answer different downstream questions: planning state, builder state, review state, resume state.

**Carry-forward extraction:** At intake time, write at least a framing artifact, a project-local design/plan artifact, and a behavioral/session event. Do not collapse them into one file.

**Refs:** [G02], [G17], [G18]

#### P05 — Lineage is first-class
**Confidence:** High

**Observed in gstack:** Design docs can carry a `Supersedes:` link; review logs are branch+commit scoped; checkpoints capture branch and timestamp; global retros snapshot windows for later trend comparison.

**Why it matters:** Lineage is what turns “more context” into trustworthy context.

**Carry-forward extraction:** Every substrate artifact should carry identifiers such as artifact_id, parent_ids, supersedes, session_id, project_id, branch, commit, mode, status, timestamps.

**Refs:** [G02], [G07], [G11], [G14], [G20]

#### P06 — Search before building / landscape awareness
**Confidence:** High

**Observed in gstack:** The preamble contains a search-before-building policy; office-hours adds a dedicated landscape-awareness phase; review/search phases verify that proposed patterns are still current best practice.

**Why it matters:** It prevents the workflow from locking onto stale or needlessly bespoke solutions.

**Carry-forward extraction:** Make “search first when unfamiliar or current-best-practice matters” a runtime rule, not an optional reviewer habit.

**Refs:** [G01], [G02], [G08], [G04]

### Planning completeness patterns

#### P07 — Mode selection before depth
**Confidence:** High

**Observed in gstack:** CEO review forces an explicit review posture: scope expansion, selective expansion, hold scope, or scope reduction, with defaults based on task shape and file-count heuristics.

**Why it matters:** The same review posture should not serve greenfield ideation, bugfix hardening, and overbuilt-plan reduction.

**Carry-forward extraction:** Make planning mode explicit at intake or pre-review time. “Complete” depends on what sort of completeness the workflow is optimizing for.

**Refs:** [G03]

#### P08 — Pre-review system audit before planning judgments
**Confidence:** High

**Observed in gstack:** CEO review begins with recent git history, diff stats, stash inspection, TODO/FIXME/HACK scans, and recently touched files before evaluating scope or architecture.

**Why it matters:** This grounds the review in the actual working state and avoids plan review in a vacuum.

**Carry-forward extraction:** Before deep review, run a cheap system audit: branch context, diff shape, deferred backlog, pending hacks, and surrounding recent work.

**Refs:** [G03]

#### P09 — Mandatory Step 0 before detailed review
**Confidence:** High

**Observed in gstack:** Engineering review starts with a Step 0 scope challenge: existing code reuse, minimum viable change, complexity smell test, search check, TODO cross-reference, completeness check, and distribution check.

**Why it matters:** Many planning failures happen before architecture review even begins: rebuilding what exists, hiding deferrals, or ignoring distribution and silent failure.

**Carry-forward extraction:** Require a Step 0 contract ahead of plan detail. Substrate should not allow architecture review to start until Step 0 outputs exist.

**Refs:** [G04]

### Workflow architecture patterns

#### P10 — Phase transitions gate on written outputs
**Confidence:** High

**Observed in gstack:** `/autoplan` refuses to begin the next phase until the current phase’s mandatory artifacts are written to the plan file and the relevant gate has passed.

**Why it matters:** This is one of the strongest reasons gstack plans feel finished: handoffs are artifact-gated, not token-budget-gated.

**Carry-forward extraction:** In substrate, transitions should depend on persisted outputs and gate checks, not just model confidence or “I think I’m done.”

**Refs:** [G07]

### Planning completeness patterns

#### P11 — Completion summaries are machine-gatable handoff contracts
**Confidence:** High

**Observed in gstack:** CEO and eng reviews end with compact summaries that count issues, note whether negative-space sections were written, report gaps, diagram counts, outside-voice status, and unresolved items.

**Why it matters:** A compact summary lets later stages decide what ran and what still blocks execution without re-reading the entire plan.

**Carry-forward extraction:** Emit structured completion summaries from every review axis. Treat them as state for downstream routing and dashboards.

**Refs:** [G03], [G04], [G07]

### Workflow architecture patterns

#### P12 — Living plan status belongs on disk
**Confidence:** High

**Observed in gstack:** `/autoplan` can append a `GSTACK REVIEW REPORT` footer to the plan file if one is missing, making review state part of the plan’s living status.

**Why it matters:** A plan is more useful when its freshness and review coverage are visible in the artifact itself.

**Carry-forward extraction:** Make reviewed-plan artifacts self-describing: include review state, freshness, unresolved gaps, and next-step hints inside the artifact.

**Refs:** [G07]

#### P13 — Decision audit trail stays on disk, not in chat history
**Confidence:** High

**Observed in gstack:** `/autoplan` appends one audit-trail row per auto-decision into the plan file so decision rationale does not accumulate only in ephemeral conversation context.

**Why it matters:** This preserves accountability while minimizing context-window clutter.

**Carry-forward extraction:** Persist autonomous decisions as a structured decision log linked to the artifact they modified.

**Refs:** [G07]

### Planning completeness patterns

#### P14 — Completeness is multi-axis, not scalar
**Confidence:** High

**Observed in gstack:** gstack decomposes review into strategy, engineering, design, DX, and implementation diff review rather than one universal “good plan” pass.

**Why it matters:** Different failure modes require different passes. A plan can be strategically sound but implementation-incomplete or great technically but weak in DX.

**Carry-forward extraction:** Model completeness as the conjunction of required axes for a task class rather than a single global score.

**Refs:** [G03], [G04], [G05], [G06], [G07], [G08]

#### P15 — Alternatives are mandatory before commitment
**Confidence:** High

**Observed in gstack:** Office-hours and CEO review require multiple materially different approaches instead of letting the first plausible path become the plan.

**Why it matters:** Alternatives expose hidden assumptions and create an honest trade-space between minimal change and ideal architecture.

**Carry-forward extraction:** Require 2–3 materially different approaches for non-trivial work and preserve the rejected options as part of the artifact history.

**Refs:** [G02], [G03]

#### P16 — Explicit reuse inventory
**Confidence:** High

**Observed in gstack:** Plan reviews repeatedly force a “What already exists” section mapping sub-problems to current code or infrastructure.

**Why it matters:** Without an explicit reuse pass, agents routinely propose parallel systems instead of extending what is already there.

**Carry-forward extraction:** Every serious plan should include a reuse inventory with evidence and reuse strategy, not just a sentence saying “existing code exists.”

**Refs:** [G03], [G04], [G07]

#### P17 — Explicit negative space / `NOT in scope`
**Confidence:** High

**Observed in gstack:** CEO and eng reviews insist on a written `NOT in scope` section and push deferred items into it rather than letting them disappear.

**Why it matters:** Completeness is not “include everything”; it is “make omissions explicit and reviewable.”

**Carry-forward extraction:** Track deferments as first-class artifacts with rationale, not as silent absences.

**Refs:** [G03], [G04], [G07]

#### P18 — Distribution path is part of completeness
**Confidence:** High

**Observed in gstack:** Eng review treats distribution as mandatory when a plan introduces a new artifact type such as a CLI, library, container, or mobile app: CI/CD, target platforms, and install path must be present or explicitly deferred.

**Why it matters:** Code that cannot be built, published, or installed is not complete for artifact-producing work.

**Carry-forward extraction:** Substrate plan schemas should include distribution/publish/install fields whenever the deliverable is something users download or integrate.

**Refs:** [G04], [G02]

#### P19 — Future-implementer temporal interrogation
**Confidence:** High

**Observed in gstack:** CEO review asks what the implementer will wish had been planned for in hour 1, hours 2–3, and hours 6+.

**Why it matters:** This surfaces the details that are cheapest to specify now and most expensive to rediscover mid-execution.

**Carry-forward extraction:** Bake a temporal interrogation pass into plan hardening: what future execution loops will regret not having now?

**Refs:** [G03]

#### P20 — Hidden-path rendering and mandatory diagrams
**Confidence:** High

**Observed in gstack:** CEO review requires diagrams for architecture, data flow, state machine, error flow, deployment sequence, and rollback; design/eng reviews force loading, empty, error, boundary, and partial states into view.

**Why it matters:** The most dangerous implementation gaps are often the paths the plan merely implies.

**Carry-forward extraction:** Represent critical flows explicitly. Hidden-path rendering should be a gate, not a “nice to have.”

**Refs:** [G03], [G04], [G05], [G07]

#### P21 — Failure-mode surfacing and silent-failure critical gaps
**Confidence:** High

**Observed in gstack:** For each new codepath, eng review asks how it can fail in production, whether a test covers it, whether error handling exists, and whether the user sees a clear error or a silent failure. No test + no handling + silent user experience becomes a critical gap.

**Why it matters:** This is a practical way to convert edge cases into actionable risk state.

**Carry-forward extraction:** Use a failure-mode registry that links each risk to test coverage, error handling, and user-visible outcome.

**Refs:** [G04]

#### P22 — Anti-skip rule: if nothing is wrong, say what was checked
**Confidence:** High

**Observed in gstack:** Review skills require sections with no findings to still say what was evaluated and why nothing was flagged.

**Why it matters:** This prevents fake completeness where a pass is “done” without demonstrating that it actually ran.

**Carry-forward extraction:** Require every axis to leave a trace, even when the result is clean.

**Refs:** [G05], [G06], [G07]

### Planning → execution bridge patterns

#### P23 — Planning should emit execution-lane hints
**Confidence:** High

**Observed in gstack:** Eng review includes a worktree parallelization strategy that groups work into independent lanes or explicitly marks the plan as sequential.

**Why it matters:** This is the bridge from planning to a modular execution framework.

**Carry-forward extraction:** Have planning artifacts emit dependency lanes, critical path hints, or an execution graph rather than just a prose step list.

**Refs:** [G04]

### Memory and persistence patterns

#### P24 — Memory is layered into planes, not one store
**Confidence:** High

**Observed in gstack:** gstack separates session awareness, project-local learnings and timelines, checkpoints, review logs, builder/developer profile data, and global retros.

**Why it matters:** These stores answer different questions and should not all be queried or injected the same way.

**Carry-forward extraction:** Use at least session, project, builder/operator, and global scopes. Within them, separate semantic learnings, episodic timeline, checkpoints, review facts, and behavioral profile.

**Refs:** [G09], [G10], [G11], [G18], [G20]

#### P25 — Append-only local artifacts with read-time consolidation
**Confidence:** High

**Observed in gstack:** Learnings, timeline events, review logs, builder-profile events, and checkpoints are written append-first or file-first, with dedup and summary logic happening on read.

**Why it matters:** Append-only writes are robust, auditable, and concurrency-friendly enough for local agent workflows.

**Carry-forward extraction:** Prefer append-only event or artifact writes, then consolidate on read or in synthesis jobs. Preserve the raw trail.

**Refs:** [G09], [G12], [G13], [G14], [G15], [G18], [G20]

#### P26 — Typed learnings with provenance, trust, and confidence
**Confidence:** High

**Observed in gstack:** Learnings carry constrained types, sources, keys, confidence, branch, commit, and related files; the logger validates and sanitizes inputs before storage.

**Why it matters:** This prevents “memory” from becoming an uninspectable blob and reduces prompt-injection or low-quality carryover risk.

**Carry-forward extraction:** Memory records should encode what kind of thing they are, where they came from, how much to trust them, and where they apply.

**Refs:** [G09], [G12]

#### P27 — Confidence decay and cross-project trust gating
**Confidence:** High

**Observed in gstack:** Observed and inferred learnings decay over time; cross-project retrieval is opt-in; untrusted cross-project learnings are filtered out to avoid contamination.

**Why it matters:** Portable memory should be harder to earn than local memory.

**Carry-forward extraction:** Make portability, trust tier, and decay policy explicit fields, not retrieval-side guesswork.

**Refs:** [G09], [G13], [G08]

#### P28 — Context recovery and cross-session injection belong in the preamble
**Confidence:** High

**Observed in gstack:** Session intelligence proposes and skills implement a preamble that reads recent plans, reviews, checkpoints, and timelines after compaction or on session start, and can emit a `Last session:` one-liner.

**Why it matters:** Memory only compounds if the right slice is loaded before the next reasoning step.

**Carry-forward extraction:** Put retrieval policy in a pre-task context composer rather than relying on each planner prompt to remember to retrieve the right things.

**Refs:** [G10], [G11], [G01]

#### P29 — Visible compounding beats silent compounding
**Confidence:** High

**Observed in gstack:** When a current review finding matches a past learning, gstack tells the user: “Prior learning applied …” so compounding is visible.

**Why it matters:** Users need to see that memory helped, otherwise memory becomes an invisible source of unexplained behavior.

**Carry-forward extraction:** Expose retrieved-memory influence explicitly when it changes the current review or plan.

**Refs:** [G08], [G09]

#### P30 — Checkpoint/resume is a first-class artifact
**Confidence:** High

**Observed in gstack:** `/checkpoint` writes a durable markdown artifact with summary, decisions made, remaining work, notes, files modified, branch, timestamp, and session duration; resume mode reads it back and warns on branch mismatch.

**Why it matters:** This is what makes long-running work and cross-branch handoffs survivable.

**Carry-forward extraction:** Keep checkpoint artifacts distinct from plans and learnings. They answer “where was I?” not “what do we believe?”

**Refs:** [G11], [G10]

#### P31 — Behavioral builder/developer profile is its own memory plane
**Confidence:** High

**Observed in gstack:** Office-hours appends builder-profile events; the repo now includes a unified developer-profile with tiers, accumulated signals, cross-project detection, resources shown, inferred dimensions, and mismatch checks.

**Why it matters:** Some useful memory is about how the operator works, not about the codebase.

**Carry-forward extraction:** Keep operator-profile memory separate from project memory. Use it for guidance, tuning, and resource curation, not for code facts.

**Refs:** [G02], [G17], [G18]

#### P32 — Global retros provide cross-project meta-memory
**Confidence:** High

**Observed in gstack:** `/retro global` plus `gstack-global-discover` scan local tool metadata from multiple AI coding tools, resolve repos, deduplicate by normalized remote URL, and save cross-project snapshots with session and streak metrics.

**Why it matters:** This creates a higher plane of memory about how the builder works across repos and tools, not just within one project.

**Carry-forward extraction:** If substrate needs cross-project learning, distinguish project memory from behavioral meta-memory and keep their retrieval/use cases separate.

**Refs:** [G19], [G20]

#### P33 — Telemetry is separate from operational memory
**Confidence:** High

**Observed in gstack:** Config exposes telemetry modes (`off | anonymous | community`), and skill preambles gate analytics/remote telemetry behavior separately from project-local learnings and timelines.

**Why it matters:** Product analytics and durable task memory should not be the same system.

**Carry-forward extraction:** Keep analytics, memory, and review state as separate stores with separate policies and consent models.

**Refs:** [G21], [G01]

#### P34 — Review memory is release-gating memory
**Confidence:** High

**Observed in gstack:** Review log scripts store branch-scoped JSONL facts that later phases and `/ship` can use to know which gates ran, on what commit, and with what unresolved risk.

**Why it matters:** Knowing that “a review happened once” is not enough; the system needs commit-aware review freshness.

**Carry-forward extraction:** Persist review facts as their own memory class with branch, commit, status, unresolved counts, and source phase.

**Refs:** [G07], [G08], [G14]

### Execution and review patterns

#### P35 — Independent voices are useful when they remain independent
**Confidence:** High

**Observed in gstack:** `/autoplan` runs Codex and a Claude subagent independently, treats outside-voice findings as informational until explicitly accepted, and logs consensus/disagreement instead of silently merging everything.

**Why it matters:** Agreement is signal; disagreement reveals judgment boundaries.

**Carry-forward extraction:** Treat multi-voice review as a consensus engine with explicit merge rules and disagreement surfacing, not as invisible ensemble averaging.

**Refs:** [G03], [G04], [G07], [G08]

#### P36 — Parallelize by dependency, not enthusiasm
**Confidence:** High

**Observed in gstack:** Specialist subagents can run in parallel, but major `autoplan` phases are explicitly serialized because each phase depends on written outputs from the previous one.

**Why it matters:** Blind parallelism often shifts cost into reconciliation.

**Carry-forward extraction:** Only parallelize independent lanes. Serialize artifact-dependent stages and use the plan’s execution graph to decide where concurrency is actually safe.

**Refs:** [G07], [G08], [G04]

#### P37 — Tier ceremony to task complexity
**Confidence:** High

**Observed in gstack:** OpenClaw/gstack-lite patterns show a reduced-ceremony path for simple work and fuller pipelines for larger or more ambiguous work.

**Why it matters:** The full process is too heavy for small changes, but too little structure creates ambiguity debt on large work.

**Carry-forward extraction:** Add explicit ceremony tiers based on change size, uncertainty, and artifact requirements.

**Refs:** [G22]

#### P38 — Prompt-only structure has a ceiling
**Confidence:** High

**Observed in gstack:** Issue reports show that even a strong review structure can miss wrong file paths, math, CLI flags, and other machine-verifiable claims when the system relies only on narrative review.

**Why it matters:** A plan can sound complete while still being concretely wrong.

**Carry-forward extraction:** Separate narrative review from deterministic verification. Substrate should lint or verify concrete claims such as file paths, APIs, CLI flags, package versions, and arithmetic.

**Refs:** [GI04], [G08], [G04]

## Reusable methods, prompts, and templates worth extracting

### M01 — Six forcing questions with smart routing
Ask one forcing question at a time; route only the relevant subset based on stage (pre-product, has
users, paying customers, pure infra).

**Refs:** [G02]

### M02 — Related-design discovery before starting fresh
After the user states the problem, search prior design docs for keyword overlap and ask whether to
build on them or start fresh.

**Refs:** [G02]

### M03 — Landscape awareness / search before building
After understanding the problem, search conventional wisdom and recent best practice before
committing to infrastructure or architecture choices.

**Refs:** [G01], [G02], [G08], [G04]

### M04 — Mode selection card
Force an explicit review posture: scope expansion, selective expansion, hold scope, or scope
reduction.

**Refs:** [G03]

### M05 — Step 0 scope challenge
Before detailed review, answer: what already exists, minimum viable change, complexity smell, search
check, deferred backlog, completeness check, distribution path.

**Refs:** [G04]

### M06 — Alternatives-first gate
Require 2–3 materially different approaches with effort, risk, pros/cons, and reuse story before
locking the plan.

**Refs:** [G02], [G03]

### M07 — Reuse inventory
Map each sub-problem to existing code, assets, or flows that can be extended rather than rebuilt.

**Refs:** [G03], [G04], [G07]

### M08 — Negative-space section
Write a `NOT in scope` section with rationale for every deferred item.

**Refs:** [G03], [G04], [G07]

### M09 — Distribution gate
If the deliverable is a CLI, library, package, container, mobile app, or similar artifact, require
build/publish/install details or explicitly defer them.

**Refs:** [G04], [G02]

### M10 — Temporal interrogation
Ask what the implementer will wish had been planned for in hour 1, hours 2–3, and hours 6+.

**Refs:** [G03]

### M11 — Happy / nil / empty / error / partial rendering
For every new flow or user-facing state, render the path instead of implying it.

**Refs:** [G03], [G04], [G05]

### M12 — Failure-mode registry
For each new codepath, list a realistic production failure and whether it has test coverage, error
handling, and clear user-visible behavior.

**Refs:** [G04]

### M13 — Phase-transition summary
End each phase with a written summary and do not advance until the required outputs are on disk.

**Refs:** [G07]

### M14 — Decision audit trail row
For every autonomous decision, write phase, classification, principle, rationale, and rejected
alternative to an audit table on disk.

**Refs:** [G07]

### M15 — Prior-learning callout
When a current finding matches prior learning, say so explicitly with key, confidence, and date.

**Refs:** [G08], [G09]

### M16 — Checkpoint template
Persist summary, decisions made, remaining work, notes, modified files, branch, timestamp, and
duration as a resumable artifact.

**Refs:** [G11]

### M17 — After-action reflection into operational learnings
At skill end, reflect on failures, wrong approaches, and project quirks, then log them as
operational learnings.

**Refs:** [G01], [G09]

### M18 — DX persona and TTHW target
Choose the developer persona and target time-to-hello-world tier before reviewing docs, onboarding,
and API ergonomics.

**Refs:** [G06]

### M19 — Cross-phase theme extraction
Surface concerns that appeared independently in two or more phases as high-confidence themes.

**Refs:** [G07]

## Substrate carry-forward structures (non-RFC)

These are not final schemas. They are the minimum structural primitives that showed up repeatedly
enough in gstack to justify preserving them as explicit design constraints in substrate.

### 1) Artifact lineage fields
`artifact_id`, `parent_ids`, `supersedes`, `session_id`, `project_id`, `branch`, `commit`, `mode`, `status`, `created_at`, `updated_at`

### 2) Minimum plan-bundle sections
`problem_statement`, `constraints`, `premises`, `alternatives`, `selected_approach`, `what_already_exists`, `not_in_scope`, `minimum_viable_change`, `distribution_path`, `architecture`, `user_states_and_edge_paths`, `error_and_rescue_registry`, `failure_modes_registry`, `test_plan`, `observability`, `rollout`, `rollback`, `execution_lanes`, `completion_summary`, `review_state`

### 3) Memory-record core fields
`memory_id`, `scope`, `class`, `type`, `key`, `value`, `source`, `derivation`, `trust_level`, `portable_across_projects`, `confidence`, `decay_policy`, `related_artifacts`, `lineage`, `timestamps`

### 4) Gate checks for “complete enough to advance”
- all required review axes ran for the task class
- all mandatory outputs exist on disk
- critical gaps == 0 or explicitly accepted
- staleness / commit drift is below threshold or review re-ran
- negative-space sections are present
- decision log and completion summary were written

## Questions to use in later alternatives documents

- Does the alternative separate methodology, runtime metadata, and persisted artifacts, or does it collapse everything into prompts?
- Does intake emit multiple related artifacts (design/plan, behavioral/session event, lineage metadata), or just one document?
- Can the alternative enforce mode selection and a mandatory Step 0 before detailed review?
- Does it require negative-space outputs such as reuse inventory, `NOT in scope`, distribution path, and failure modes?
- Can it express completeness as required review axes rather than a single generic score?
- Does it support append-only lineage and read-time or synthesis-time consolidation?
- Does memory have distinct scopes or planes (session, project, builder/operator, global) with explicit trust and portability?
- Can the system recover context after compaction or session loss via checkpoints, timeline, and recent artifact injection?
- Does it persist review/gate state with commit awareness so later phases can reason about freshness?
- Can it expose which memories were applied so compounding is visible to operators?
- Does it distinguish telemetry/product analytics from operational memory and planning artifacts?
- Does it pair narrative planning/review with deterministic verification for machine-checkable claims?

## Risks, current-state nuances, and caution flags

### R01 — Hidden storage hurts discoverability
Persisting value mostly under `~/.gstack/` is practical, but users can lose track of where the
canonical artifacts live.

**Refs:** [GI01]

### R02 — Canonical-source drift is a real risk
When multiple plan or design file locations can exist (`.gstack`, `.claude/plans`, etc.), downstream
skills may read the wrong one unless there is a single canonical artifact ID/path.

**Refs:** [GI02]

### R03 — Workflow-state resume needs stronger machine state
Checkpoint and review memory help, but issue reports show that users still want more explicit
workflow-state tracking for session resume.

**Refs:** [GI03], [G11], [G10]

### R04 — Behavioral profile is mid-migration
The repo currently bridges legacy `builder-profile.jsonl` and a newer `developer-profile.json`;
office-hours still appends builder-profile events while reader tooling is already migrating to the
unified profile.

**Refs:** [G02], [G17], [G18]

### R05 — Local-first JSONL is excellent for auditability but lightly structured
The file-based append-only model is easy to inspect, grep, and debug, but it will need stronger
typing, migration handling, and concurrency semantics if substrate becomes a shared multi-crate
core.

**Refs:** [G09], [G12], [G13], [G14], [G15], [G18]
**Confidence:** Synthesis

### R06 — Prompt compliance is not the same as validation
Even strong prompts can omit required fields or approve concrete claims that should have been
machine-checked.

**Refs:** [GI04], [G07], [G04]

### R07 — Telemetry and analytics behavior is version-sensitive
Config and current preambles clearly separate telemetry settings from operational memory, but
analytics behaviors should still be treated as version-sensitive and policy-controlled.

**Refs:** [G21], [G01]

## Adjacent systems that sharpen the baseline

### A01 — Every compound-knowledge-plugin
Reinforces the value of git-tracked knowledge artifacts, multi-granularity plans, and explicit
compounding across work sessions.

**Refs:** [A01]

### A02 — Compound Engineering (Every)
Sharpens outcome-focused instructions and parallel workstream thinking that pair well with
execution-lane planning.

**Refs:** [A02]

### A03 — LangGraph memory taxonomy
Adds a clean split between thread-scoped short-term memory and namespace-scoped long-term memory,
plus a useful memory-class vocabulary.

**Refs:** [A03], [A04]

### A04 — MemGPT
Contributes the hot/warm/cold or tiered-memory metaphor for tight context injection.

**Refs:** [A05]

### A05 — Generative Agents
Adds the reflection layer: raw event streams become more useful when periodically synthesized into
higher-order learnings that guide planning.

**Refs:** [A06]

### A06 — Reflexion
Strengthens the case for an explicit after-action reflection artifact or service for repeated task
families.

**Refs:** [A07]

## Source map

### GStack sources

- **[G01]** gstack/ARCHITECTURE.md — <https://github.com/garrytan/gstack/blob/main/ARCHITECTURE.md>
  - Template/compiler architecture, preamble responsibilities, committed generated docs, review dashboard placeholders.
- **[G02]** gstack/office-hours/SKILL.md.tmpl — <https://github.com/garrytan/gstack/blob/main/office-hours/SKILL.md.tmpl>
  - Six forcing questions, related-design discovery, landscape awareness, builder profile append, design doc path, Supersedes chain, distribution plan in design template.
- **[G03]** gstack/plan-ceo-review/SKILL.md — <https://github.com/garrytan/gstack/blob/main/plan-ceo-review/SKILL.md>
  - Mode selection, pre-review system audit, mandatory diagrams, outside voice rules, completion summary, scope proposal handling.
- **[G04]** gstack/plan-eng-review/SKILL.md — <https://github.com/garrytan/gstack/blob/main/plan-eng-review/SKILL.md>
  - Step 0 scope challenge, boil-the-lake completeness, distribution check, failure modes, worktree parallelization, completion summary.
- **[G05]** gstack/plan-design-review/SKILL.md — <https://github.com/garrytan/gstack/blob/main/plan-design-review/SKILL.md>
  - Seven-pass UX/design completeness review and anti-skip behavior.
- **[G06]** gstack/plan-devex-review/SKILL.md — <https://github.com/garrytan/gstack/blob/main/plan-devex-review/SKILL.md>
  - Developer persona, TTHW targeting, DX artifact outputs.
- **[G07]** gstack/autoplan/SKILL.md — <https://github.com/garrytan/gstack/blob/main/autoplan/SKILL.md>
  - Restore points, phase checklists, phase outputs, review report footer, decision audit trail, dual voices, cross-phase themes, review logging.
- **[G08]** gstack/review/SKILL.md — <https://github.com/garrytan/gstack/blob/main/review/SKILL.md>
  - Prior learnings retrieval, visible compounding, cross-project opt-in, specialist synthesis, review result persistence.
- **[G09]** gstack/docs/designs/SELF_LEARNING_V0.md — <https://github.com/garrytan/gstack/blob/main/docs/designs/SELF_LEARNING_V0.md>
  - Learning taxonomy, append-only JSONL, confidence decay, cross-project trust gating, timeline/checkpoint/health separation.
- **[G10]** gstack/docs/designs/SESSION_INTELLIGENCE.md — <https://github.com/garrytan/gstack/blob/main/docs/designs/SESSION_INTELLIGENCE.md>
  - Persistent artifact directory, context recovery, session timeline, cross-session injection, checkpoint and health layers.
- **[G11]** gstack/checkpoint/SKILL.md — <https://github.com/garrytan/gstack/blob/main/checkpoint/SKILL.md>
  - Checkpoint save schema, resume flow, branch-aware listing, remaining work and notes, cross-branch handoff warning.
- **[G12]** gstack/bin/gstack-learnings-log — <https://github.com/garrytan/gstack/blob/main/bin/gstack-learnings-log>
  - Learning write path, schema validation, injection sanitization, trust assignment.
- **[G13]** gstack/bin/gstack-learnings-search — <https://github.com/garrytan/gstack/blob/main/bin/gstack-learnings-search>
  - Confidence decay, latest-winner dedup, type/query filtering, trusted cross-project restriction.
- **[G14]** gstack/bin/gstack-review-log — <https://github.com/garrytan/gstack/blob/main/bin/gstack-review-log>
  - Branch-scoped persisted review memory.
- **[G15]** gstack/bin/gstack-timeline-log — <https://github.com/garrytan/gstack/blob/main/bin/gstack-timeline-log>
  - Timeline event persistence.
- **[G16]** gstack/bin/gstack-timeline-read — <https://github.com/garrytan/gstack/blob/main/bin/gstack-timeline-read>
  - Timeline reading windows, branch filters, recent-entry retrieval.
- **[G17]** gstack/bin/gstack-builder-profile — <https://github.com/garrytan/gstack/blob/main/bin/gstack-builder-profile>
  - Legacy shim showing builder-profile migration to developer-profile.
- **[G18]** gstack/bin/gstack-developer-profile — <https://github.com/garrytan/gstack/blob/main/bin/gstack-developer-profile>
  - Unified behavioral profile, tiers, accumulated signals, cross-project detection, inferred dimensions, mismatch checks.
- **[G19]** gstack/bin/gstack-global-discover.ts — <https://github.com/garrytan/gstack/blob/main/bin/gstack-global-discover.ts>
  - Cross-tool session discovery, repo resolution, normalized remote dedup, structured JSON output.
- **[G20]** gstack/retro/SKILL.md — <https://github.com/garrytan/gstack/blob/main/retro/SKILL.md>
  - Repo retros, global retro mode, snapshot saving, all-project overview and trend comparisons.
- **[G21]** gstack/bin/gstack-config — <https://github.com/garrytan/gstack/blob/main/bin/gstack-config>
  - Telemetry modes, configuration surface.
- **[G22]** gstack/docs/OPENCLAW.md — <https://github.com/garrytan/gstack/blob/main/docs/OPENCLAW.md>
  - Tiered ceremony and routing by task size.
- **[G23]** gstack/docs/skills.md — <https://github.com/garrytan/gstack/blob/main/docs/skills.md>
  - Skill inventory and pipeline overview.

### GStack issue / caution sources

- **[GI01]** gstack issue #401 — <https://github.com/garrytan/gstack/issues/401>
  - Hidden `.gstack` discoverability pain.
- **[GI02]** gstack issue #542 — <https://github.com/garrytan/gstack/issues/542>
  - `.gstack` vs `.claude/plans` drift / canonical-source risk.
- **[GI03]** gstack issue #341 — <https://github.com/garrytan/gstack/issues/341>
  - Resume/workflow-state brittleness across sessions.
- **[GI04]** gstack issue #973 — <https://github.com/garrytan/gstack/issues/973>
  - Prompt-only spec review blind spots for file paths, CLI flags, math, and similar concrete claims.

### Adjacent sources

- **[A01]** Every compound-knowledge-plugin — <https://github.com/EveryInc/compound-knowledge-plugin>
  - Compounding knowledge-work loop and git-tracked docs/plans.
- **[A02]** Every / Compound Engineering guide — <https://every.to/guides/compound-engineering>
  - Outcome-focused compound engineering and parallel workstream ideas.
- **[A03]** LangGraph memory overview — <https://docs.langchain.com/oss/python/langgraph/memory>
  - Short-term vs long-term memory in LangGraph, thread-scoped vs namespace-scoped.
- **[A04]** LangChain long-term memory docs — <https://docs.langchain.com/oss/python/langchain/long-term-memory>
  - JSON-document memory with namespace + key organization.
- **[A05]** MemGPT: Towards LLMs as Operating Systems — <https://arxiv.org/abs/2310.08560>
  - Hierarchical memory tiers and paging metaphor.
- **[A06]** Generative Agents: Interactive Simulacra of Human Behavior — <https://arxiv.org/abs/2304.03442>
  - Memory stream + reflection + planning architecture.
- **[A07]** Reflexion: Language Agents with Verbal Reinforcement Learning — <https://arxiv.org/abs/2303.11366>
  - After-action verbal reflection as episodic memory for later trials.

## Closing note

The simplest way to remember this entire synthesis is: gstack’s real contribution is not a single
“complete plan” schema. It is a disciplined, compiled, artifact-first workflow that turns intake
into a durable design object, runs it through orthogonal review axes, persists the resulting state
across sessions, and makes compounding visible.