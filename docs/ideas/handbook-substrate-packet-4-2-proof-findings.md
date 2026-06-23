# Handbook → Substrate Packet 4.2 Proof Findings

## Status

Findings memo from the dedicated Packet 4.2 throwaway worktree proof. This note is intended to preserve what the experiment taught us. It is **not** the final architecture authority for how handbook should ultimately integrate into Substrate.

## Purpose of the proof

The goal of the Packet 4.2 proof was to answer one concrete question:

> Can Substrate consume the **published** handbook crate boundaries in a real downstream code seam, without relying on sibling-path behavior or unpublished internals?

The answer is **yes**.

## What landed in the proof worktree

Proof worktree:

- `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054`

Proof commit:

- `7f1db2eab613d103d91b4194492c56917ce7563b`
- `feat: add handbook-backed prompt advisory seam`

Changed file:

- `crates/shell/src/execution/prompt_fulfillment.rs`

Landed seam:

- `compose_prompt_with_host_toolbox_contract(...)`

Published handbook APIs used:

- `handbook_flow::resolve_with_contract(...)`
- `handbook_flow::ResolveRequest`
- `handbook_flow::ResolverResult`
- `handbook_flow::PacketSelectionStatus`
- `handbook_flow::ResolverNextSafeAction`
- `handbook_flow::ResolverSubjectRef`
- `handbook_engine::default_canonical_layout_contract()`

Behavior added by the proof:

1. Substrate walks up from the current working directory looking for a handbook root.
2. If a handbook root exists, Substrate resolves the published handbook planning baseline.
3. Substrate translates the typed resolver result into a **Substrate-owned advisory prompt block**.
4. The advisory is compositional only; it does not hard-fail the runtime or gate shell startup.

## Why this counted as a successful proof

This proof was accepted because it demonstrated all of the following at once:

- **real published-crate consumption**
  - Substrate did not merely depend on the crates in `Cargo.toml`; it actually called published handbook APIs
- **narrow downstream seam**
  - one production file changed
  - one existing prompt-composition seam was extended
- **Substrate-owned rendering**
  - handbook returned typed status/subject/action information
  - Substrate kept ownership of the final human-facing wording
- **no architectural overclaim**
  - this did not pretend to be the final handbook integration model
  - it only proved that published handbook consumption can work honestly inside Substrate

## What this proof now establishes

After this experiment, we know that:

1. **Substrate can consume published handbook crates honestly**
   - this is no longer just a manifest-wiring claim

2. **`handbook_flow` + `handbook_engine` are immediately usable published surfaces**
   - especially the flow resolver path and the default canonical layout contract

3. **The lowest-friction first consumer seam was prompt composition**
   - prompt enrichment was the narrowest place to consume typed published outputs without forcing a large runtime redesign

4. **Substrate should keep ownership of wording**
   - handbook surfaces are more useful as typed planning/readiness signals than as final operator-facing copy

5. **A handbook-backed integration can stay advisory first**
   - not every first integration needs to be a hard policy gate or a stateful runtime dependency

## What this proof does **not** establish

This worktree does **not** prove that we have found the final desired integration shape.

In particular, it does **not** mean that:

- prompt advisory is the right long-term foundation for handbook inside Substrate
- handbook should remain only an advisory context source
- `handbook-pipeline` is already naturally integrated
- Substrate has adopted handbook as a core planning/runtime primitive
- the real product integration should live in `prompt_fulfillment.rs`

This was a **proof of feasible published-boundary consumption**, not the final product design.

## Main findings from the implementation attempt

### 1. The first real consumer seam had to be created, not merely adapted

The earlier Packet 4.2 wording sounded like “adapt existing downstream call sites/adapters,” but live repo truth only showed manifest wiring. There was no pre-existing handbook consumer seam waiting to be updated.

Implication:

- the first honest Substrate integration required creating a small new consumer seam
- future planning should assume that “first published consumer” is real feature work, even if the seam is intentionally narrow

### 2. Prompt composition was the easiest safe insertion point

The strongest proof landed at the authoritative host prompt composition seam because it already:

- gathers context
- shapes agent-visible instructions
- has clear ownership boundaries
- can tolerate advisory enrichment without destabilizing control flow

Implication:

- prompt composition is a good bootstrap seam for proving integration
- it is not automatically the best long-term seam for handbook as a system foundation

### 3. `handbook_flow::resolve_with_contract(...)` is the most immediately useful published capability

The resolver gives Substrate typed answers to questions like:

- is the canonical baseline ready or blocked?
- what is the earliest broken subject?
- what is the next safe action?
- which canonical sources are selected?

That makes it a strong fit for:

- readiness checks
- advisory context
- future planning surfaces
- future doctor/status commands

### 4. Substrate-owned wording is the right boundary

The proof worked best when handbook supplied typed semantics and Substrate turned those into final copy. This preserved a clean ownership split:

- handbook owns canonical planning/readiness semantics
- Substrate owns runtime/operator language and behavior

Implication:

- future integrations should prefer typed handbook outputs plus Substrate rendering
- avoid pushing shell/operator wording back into handbook crates

### 5. `handbook-pipeline` is **not** yet a drop-in Substrate foundation

The proof and prior discovery showed that `handbook-pipeline` expects repo structures and state roots that do not naturally exist in today’s Substrate checkout, such as:

- `core/pipelines`
- `core/profiles`
- `core/runners`
- `core/stages`
- `.handbook/state/pipeline`
- `artifacts/handoff/feature_slice`

Implication:

- using `handbook-pipeline` as a deeper Substrate foundation is possible, but it is a larger intentional integration project
- it should not be treated as a trivial next step from the Packet 4.2 proof

### 6. Handbook root discovery via cwd ancestry is a practical pattern

The proof successfully used ancestor walking from the current working directory to find the handbook root, which means nested work locations can still discover handbook context without additional session plumbing.

Implication:

- repo-local handbook awareness can be added incrementally without first redesigning global session routing

## Why this is probably not the final integration we want

If the intent is for Substrate to use handbook “as a foundation to build its system,” then prompt-time advisory context is probably too shallow to be the final answer.

This proof is useful because it shows:

- **how handbook can enter Substrate honestly**
- **which published surfaces are easiest to consume first**
- **where the ownership boundary should sit**

But it likely falls short of the real target because it does not yet make handbook:

- a first-class planning primitive
- a first-class doctor/readiness primitive
- a first-class command surface
- a first-class stateful runtime substrate

## Most likely next-step integration targets

If we return later to build the integration we actually want, the strongest candidates are:

### Option A: Handbook doctor / readiness surface

Add a Substrate-visible command or doctor/readiness path that uses the published resolver to report whether handbook baseline truth is ready, blocked, stale, or malformed.

Why it is attractive:

- strongly aligned with the resolver’s current strengths
- still narrow
- more operator-visible than prompt enrichment

### Option B: Handbook-backed planning surface

Add a planning-oriented Substrate surface that explicitly consumes handbook packet selection / readiness outputs to shape planning work more deeply than a prompt advisory can.

Why it is attractive:

- closer to “handbook as system foundation”
- still compatible with Substrate-owned wording

### Option C: Explicit handbook command surface

Expose handbook-backed behaviors through a dedicated Substrate command or subcommand family rather than only injecting context into prompts.

Why it is attractive:

- makes handbook integration visible, testable, and intentional
- avoids burying the capability inside prompt composition

### Option D: Deeper `handbook-pipeline` integration

Use `handbook-pipeline` only if we intentionally want Substrate to adopt handbook’s declarative roots, route state, and handoff/storage model.

Why it is risky:

- this is a larger design decision, not a trivial follow-on from the proof
- it likely requires explicit decisions about ownership, storage layout, and runtime lifecycle

## Recommended reuse of this proof

When coming back to do the real integration, treat this worktree as a source of:

- **boundary evidence**
  - published crate consumption is viable
- **API evidence**
  - `handbook_flow` + `handbook_engine` are the most natural first surfaces
- **ownership evidence**
  - typed handbook semantics plus Substrate-owned wording is the cleanest split
- **scope evidence**
  - prompt composition is a good proof seam, but likely not the final system seam

Do **not** treat this proof as saying:

- “the prompt advisory is the architecture”
- “Packet 4.2 should be cherry-picked as the final design”

Instead, treat it as:

> a validated spike that reduced uncertainty about how published handbook boundaries actually meet Substrate

## Verification evidence from the proof

Implementation verification used in the proof worktree included:

- `cargo fmt --all -- --check`
- `cargo test -p shell execution::prompt_fulfillment::tests:: -- --nocapture`
- `cargo check --workspace`
- independent review pass: clean

## Bottom line

The Packet 4.2 proof was worth doing.

It showed that Substrate can now consume published handbook boundaries honestly, and it surfaced the most important design insight for the real integration:

> The right near-term handbook value in Substrate is likely **typed planning/readiness semantics rendered through Substrate-owned product/runtime surfaces**, not handbook-owned wording and not accidental deep coupling to handbook-pipeline before that architecture is explicitly chosen.
