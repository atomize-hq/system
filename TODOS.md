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
