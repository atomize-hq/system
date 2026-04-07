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

## Post-Implementation Audit Follow-Ups

### Support Boundary Reconciliation

**What:** Reconcile `PLAN.md`, `README.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, CLI help, and runtime behavior so reduced v1 does not claim supported planning packet generation before `system generate` exits `0` with a non-placeholder packet body.

**Why:** The current repo is caught between underclaim and overclaim. Docs say live planning packet resolution is supported, while CLI help still describes a scaffold and the ready path still returns placeholder body text.

**Effort:** S
**Priority:** P1
**Depends on:** Finished ready-path packet body

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

### Repo Discovery And Recovery Transition Tests

**What:** Define repo discovery semantics and add tests for retry-after-repair, partial `.system/` trees, malformed inputs, and docs/help/runtime vocabulary drift.

**Why:** Current coverage is strong on deterministic static states, but weak on state transitions and normal operator invocation paths.

**Effort:** M
**Priority:** P2
**Depends on:** Setup ownership decision, packet body contract direction
