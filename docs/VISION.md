# North Star Vision for System

## Review Status

This file is a broad vision statement.

- For the reviewed reduced-v1 implementation boundary, use the reduced-v1 seam pack at [artifacts/planning/reduced-v1-seam-pack/README.md](../artifacts/planning/reduced-v1-seam-pack/README.md).
- The reviewed v1 is narrower than this vision: Rust-first, live `project + feature` planning packets, fixture-backed execution packet demo only, then implementation planning next.

## Purpose

Build a **modular, deterministic context + posture generation system** that enables AI agents to plan and execute work with **minimal human involvement** after initial project setup.

This system exists to make AI-driven delivery safe and repeatable by ensuring agents always have:

* the right constraints (posture, guardrails)
* the right reality (what exists, what's live)
* the right tooling assumptions (profiles)
* the right workflow enhancements (overlays)
* the right output contracts (machine-parseable, traceable artifacts)

## Human-in-the-loop philosophy

Human involvement should be **front-loaded** and then **release-scoped**:

1. **Project setup (human-guided / interactive)**

   * Humans participate to establish project posture, constraints, and reality.
   * Outputs become canonical artifacts that agents can reference indefinitely.

2. **After setup: minimal human check-ins**

   * Human check-ins should occur mostly at the **Release** level.
   * Humans communicate what they want to focus on next (features, tech debt, bugs, parking-lot items, priorities).
   * Once a backlog is sufficiently populated, agents should have enough information to:

     * plan releases
     * plan sprints
     * generate tasks (including gates, research, decisions)
     * execute workstreams continuously
   * Human intervention is primarily for prioritization and directional decisions, not day-to-day execution.

## Core architectural principle

**The system is fundamentally context generation for planning + execution.**
It is not the product backlog system, but it enables backlog-driven automation by producing the artifacts that keep planning grounded and execution disciplined.

## Key artifacts and their roles

### 1) Charter (posture/standards)

**CHARTER.md** is the source of truth for:

* tradeoffs (speed vs quality)
* rigor levels per dimension (testing, security, reliability, etc.)
* raise-the-bar triggers, allowed shortcuts, red lines
* exceptions process / debt tracking expectations

This prevents the system from having to rediscover how strict to be for every feature.

### 2) Project Context (reality snapshot, optional)

**PROJECT_CONTEXT.md** exists to prevent agents from inventing constraints like:

* migrations/back-compat
* prod rollout controls
* external contracts/integrations
* infra assumptions

It is optional and triggered when Charter leaves planning-critical unknowns.

### 3) Foundation Pack (project-specific defaults derived from Charter + Context)

Foundation Pack outputs concretize posture into reusable defaults:

* `FOUNDATION_STRATEGY.md` (planning guardrails)
* `TECH_ARCH_BRIEF.md` (architecture direction)
* `TEST_STRATEGY_BRIEF.md` (testing approach aligned to posture)
* `QUALITY_GATES_SPEC.md` (exhaustive, explicit gate policy)
* `quality_gates.yaml` (machine-readable gates definition)
* `ENVIRONMENT_INVENTORY.md` (canonical store of env vars/services/ports/runtime assumptions)

Foundation Pack is what makes downstream planning deterministic and reduces repeated questioning.

### 4) Feature Spec (per-feature contract)

**FEATURE_SPEC.md** is the per-feature plan that must align with:

* Charter (posture + red lines)
* Foundation Pack (defaults)
* Project Context (facts)

It must not re-decide posture; it inherits posture and only declares deltas.

## Modularity mechanisms (why the system stays flexible)

### Profiles (stack packs)

Profiles store stack/tooling assumptions **outside** core prompts:

* commands (`commands.yaml`)
* conventions (`conventions.md`)
* default dirs/tools (`profile.yaml`)

This stops the core system from hardcoding Poetry/uv/cargo/pnpm and reduces ambiguity.

### Overlays (layered enhancements)

Overlays are optional modules that add workflow structure without bloating the core:

* sprint/lane structures
* task typing (research/discovery, gates-as-tasks)
* release types (major/minor/hardening)
* stricter quality or security posture add-ons

Overlays should be organized by purpose (e.g., `task/`, `sprint/`, `release/`, `quality/`) and can be activated per pipeline/run.

### Work-level hierarchy (scoping and concision)

Introduce a level system (`L0`-`L3`) to scope strict rules and keep context packs small:

* L0 Program (big picture)
* L1 Planning (charter/context/foundation/specs)
* L2 Execution (single slice/worktree discipline)
* L3 Quality Gate & Merge (final verification discipline)

This allows parallel workstreams in planning while enforcing discipline at execution/merge.

## Execution model (how this operates today)

* A harness compiles a stage prompt from front matter + includes (rules/runner/profile/overlays) + library directives/templates.
* The LLM produces outputs that follow strict output contracts.
* The harness writes artifacts and maintains lightweight state.

## Release and sprint planning direction (where the system is going next)

* **Features/backlog** are created outside the main pipelines (discovery/brainstorming).
* **Release planning** is the primary human check-in:

  * selects focus from backlog (features/bugs/tech debt/etc)
  * defines multi-sprint intent and gates
  * explicitly references which features are in focus (to prevent sprint invented scope)
* **Sprint planning** derives tasks from:

  * release intent
  * previous sprint reality (what was actually completed/blocked)
  * feature plans/specs
* Gates are represented as **typed tasks** inside sprints, which block sprint close.

This keeps the system continuously operational with minimal human involvement beyond release-level steering.
