# Handbook Substrate Parameterization Three-Set Map

## Purpose

This file is the planning-entry map for the three new spec-driven sets required after Phase 6 Remaining Work Packet 6.D.2.

Its job is to turn the repo-truth findings in `docs/specs/handbook-substrate-parameterization-gap-map.md` into a bounded **Set -> Triplet -> Packet -> Packet-Prompt** execution model for the import-target crates only:

- `handbook-engine`
- `handbook-pipeline`
- `handbook-flow`

This file exists because the current import/adoption planning artifacts are not enough by themselves to make handbook align honestly with the intended Substrate-owned layout under `.substrate/handbook/**`.

This file should be used as the anchor while landing the next three sets **one at a time**.

---

## Why This Map Exists Now

The current repo truth is:

- Packet **6.D.1** produced `docs/specs/handbook-substrate-import-adoption-plan.md`
- Packet **6.D.2** then identified additional repo-truth gaps that still block honest alignment with the intended Substrate-owned handbook layout
- those gaps are now recorded in `docs/specs/handbook-substrate-parameterization-gap-map.md`

The gap map shows that the remaining work naturally separates into three different import-target seams:

1. `handbook-pipeline` import-layout parameterization
2. `handbook-flow` public canonical-layout injection
3. residual import-surface default / validation cleanup inside the import-target crates

Those seams should **not** be collapsed into one large triplet because they have different acceptance stories, different verifier walls, and different stopping points.

---

## Scope And Non-Scope

### In scope

- parameterization work required for the import-target crates to live honestly under a Substrate-owned namespace such as `.substrate/handbook/**`
- `handbook-engine`, `handbook-pipeline`, and `handbook-flow`
- structural import seams first, then residual import-surface honesty cleanup
- spec/plan/tasks/prompt authoring for **one set at a time**

### Out of scope

- `handbook-cli`
- `handbook-compiler`
- actual Substrate import execution
- compatibility shims as a long-term substitute for real parameterization
- widening into a full handbook-product shell redesign
- crates.io/publication cleanup

---

## Authority And Sequencing Rules

This file does **not** replace:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md` as the root-plan authority
- `docs/specs/handbook-substrate-parameterization-gap-map.md` as the repo-truth gap inventory
- `docs/specs/handbook-substrate-import-adoption-plan.md` as the already-written import/adoption planning artifact

Instead, this file is the **decomposition and sequencing authority** for the next three sets.

### Core execution vocabulary

To keep the execution model unambiguous, use these terms exactly:

- **Set** = one authoritative triplet:
  - `spec.md`
  - `plan.md`
  - `tasks.md`
- **Packet** = a bounded subset/group of tasks inside that set's `tasks.md`
  - usually there are multiple packets inside one set's `tasks.md`
- **Packet-prompts file** = one separate file for that set containing one orchestration prompt per packet

So the real per-set file shape is:

- `docs/specs/<stem>-spec.md`
- `docs/specs/<stem>-plan.md`
- `docs/specs/<stem>-tasks.md`
- `docs/specs/<stem>-packet-prompts.md`

### Sequential-authoring rule

Because this work will be landed one set at a time, future agents should follow this rule strictly:

1. this map names all three sets up front
2. only the **current active set** gets its own triplet:
   - `spec.md`
   - `plan.md`
   - `tasks.md`
3. the active set may also get its own `packet-prompts.md` file containing one orchestration prompt per packet in that set's `tasks.md`
4. do **not** pre-author the later sets' triplets or prompts just because this map names them
5. when one set is reviewed and its stopping condition is reached, return to this map before opening the next set

### File-stem convention

Each set should eventually get exactly one quartet of active authority files:

- `docs/specs/<stem>-spec.md`
- `docs/specs/<stem>-plan.md`
- `docs/specs/<stem>-tasks.md`
- `docs/specs/<stem>-packet-prompts.md`

The responsibilities are:

- the triplet (`spec.md`, `plan.md`, `tasks.md`) defines the set
- the `tasks.md` file contains the set's packets
- the `packet-prompts.md` file contains one orchestration prompt per packet
- packets do **not** get their own spec/plan/tasks triplets

### Packet-prompt authoring note

When a future agent is asked to write prompts for a set's packets, the prompts should follow this orchestration pattern unless the user explicitly changes it:

1. write **one orchestration prompt per packet**
2. each prompt should instruct a fresh-session orchestration agent to start with a `/goal ...`
3. the orchestration agent should spawn a fresh **GPT-5.4 high** implementation subagent for that packet
4. the implementation subagent should be instructed to use the `$incremental-implementation` skill
5. when implementation completes, the orchestration agent should spawn a fresh **GPT-5.4 high** review subagent
6. the review subagent should be instructed to use the `$code-review-and-quality` skill
7. if the review subagent flags issues, the orchestration agent should spawn a fresh fix subagent to address the flagged issues
8. the orchestration flow should commit changes between each packet / review / fix step

This map does **not** need to restate the full packet prompt body for every set. It only records the required choreography that the future `*-packet-prompts.md` files must follow.

---

## Current Repo-Truth Summary

The gap map establishes these important truths:

- `handbook-engine` already has a public canonical layout contract for canonical handbook artifacts
- `handbook-pipeline` already has an internal typed storage-layout contract, but it is not yet a supported public import-facing seam
- `handbook-pipeline` declarative roots still assume repo-level `core/pipelines/**`, `core/profiles/**`, `core/runners/**`
- `handbook-pipeline` stage discovery / validation still assumes repo-level `core/stages/**`
- `handbook-flow` public resolution still defaults to the handbook-product canonical root instead of consuming an injected layout contract
- residual `.handbook` / `core/**` wording and fallback assumptions still exist inside the import-target crates even after the main structural seams are separated
- CLI/compiler product-shell work is broader and remains intentionally out of scope for this three-set map

That means the next work should land in this order:

1. close the largest remaining structural gap in `handbook-pipeline`
2. then close the remaining `handbook-flow` public import-contract gap
3. then clean the residual import-surface wording/defaults so the final story is honest

---

## Planned Order

The three sets should land in this order:

1. **Set 1** — `handbook-pipeline` import-layout parameterization
2. **Set 2** — `handbook-flow` canonical-layout injection
3. **Set 3** — import-surface default / validation honesty cleanup

Why this order:

- Set 1 closes the biggest structural blocker first
- Set 2 depends on the canonical-layout story being treated as a real downstream import seam, not just an engine-local capability
- Set 3 should follow the structural seams so wording, refusal text, and residual defaults can derive from the actual landed contracts instead of guessing ahead of them

---

## Set 1 — `handbook-pipeline` Import-Layout Parameterization

### Status

- **Next set to open**
- No set-specific triplet has been written yet

### Proposed Triplet Stem

- `handbook-substrate-parameterization-set-1-pipeline-import-layout`

### Seam

Create the real import-facing parameterization seams required for `handbook-pipeline` to support a Substrate-owned handbook layout without mirroring handbook's repo-level `core/**` and `.handbook/state/**` defaults.

### Why This Is Its Own Set

- this is the largest remaining structural import gap
- most of the work centers on `handbook-pipeline`
- it has a different verifier wall from flow-facing API changes or residual wording cleanup
- it is the set most likely to need several implementation packets even though it should still live under one triplet

### Structural Gaps This Set Owns

1. declarative-root parameterization for:
   - pipeline catalog root
   - profile-pack root
   - runner root
2. stage-root parameterization so supported source-path assumptions stop being fixed to `core/stages/**`
3. public/import-facing pipeline storage-layout injection for:
   - state root
   - pipeline state directory
   - stage-capture provenance directory
   - capture-cache directory
   - handoff feature-slice bundle root

### Primary Authority Inputs

- `docs/specs/handbook-substrate-parameterization-gap-map.md`
- `docs/specs/handbook-substrate-import-adoption-plan.md`
- `crates/pipeline/src/declarative_roots.rs`
- `crates/pipeline/src/pipeline.rs`
- `crates/pipeline/src/layout.rs`
- adjacent `crates/pipeline/src/*` callers that currently assume repo-level `core/**` or handbook-product state roots

### Assumptions Future Agents Should Surface In `SPECIFY`

1. The goal is import-target parameterization for Substrate-owned layout, not a redesign of handbook's declarative model.
2. The first-class target layout is `.substrate/handbook/**`, but the immediate code requirement is a typed import-facing contract rather than one hardcoded replacement literal.
3. This set should not widen into CLI/compiler help text or product-shell repair.
4. This set may touch validation behavior where validation is inseparable from root ownership, but broader user-visible wording cleanup belongs to Set 3.
5. This set should preserve a bounded supported model rather than inventing an open-ended multi-layout plugin system.

### Suggested Live Evidence Sweep

```bash
rg -n "core/pipelines|core/profiles|core/runners|core/stages|\.handbook/state|artifacts/handoff/feature_slice" crates/pipeline/src
```

### Likely Packet Shape

1. `Packet 1.1: Declarative Root Contract And Owner Boundary`
2. `Packet 1.2: Stage-Root Discovery And Validation Adoption`
3. `Packet 1.3: Public Pipeline Storage Layout Injection`
4. `Packet 1.4: Final Set Proof`

### Set-Level Success Criteria To Carry Into `SPECIFY`

- `handbook-pipeline` no longer requires repo-level `core/pipelines/**`, `core/profiles/**`, `core/runners/**`, or `core/stages/**` as fixed import assumptions
- pipeline storage layout is available through a supported public/import-facing seam rather than crate-private internals only
- Substrate can supply the relevant declarative/storage roots through the supported boundary without reaching into crate-private implementation details
- any remaining fixed `core/**` or `.handbook/state/**` assumptions are explicitly bounded and justified

### Gate Before Opening Set 2

Do not open Set 2 until Set 1's triplet has reached a reviewed stopping point and the active authority clearly records the public/import-facing declarative + storage layout contract story.

---

## Set 2 — `handbook-flow` Canonical-Layout Injection

### Status

- Blocked on Set 1 finishing first
- No set-specific triplet has been written yet

### Proposed Triplet Stem

- `handbook-substrate-parameterization-set-2-flow-canonical-layout-injection`

### Seam

Make the public `handbook-flow` import surface consume a non-default canonical layout contract instead of always resolving against the handbook-product canonical root.

### Why This Is Its Own Set

- it is a public API / import-contract seam, not a pipeline declarative-root seam
- most of the work centers on `handbook-flow` with a narrow engine contract dependency
- it has a simpler acceptance story than Set 1 and should stay isolated from residual wording cleanup

### Structural Gaps This Set Owns

1. flow-facing public entry points must accept or otherwise honor the active canonical layout contract
2. downstream resolution must stop defaulting unconditionally to `CanonicalArtifacts::load(...)`
3. any engine/flow boundary decisions needed to support this import path must be recorded explicitly

### Primary Authority Inputs

- `docs/specs/handbook-substrate-parameterization-gap-map.md`
- `docs/specs/handbook-substrate-import-adoption-plan.md`
- `docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
- `crates/flow/src/resolver.rs`
- `crates/engine/src/canonical_paths.rs`
- any flow tests that prove canonical-root resolution behavior

### Assumptions Future Agents Should Surface In `SPECIFY`

1. The engine canonical layout contract remains the canonical owner for handbook artifact roots.
2. The required change is to make that contract usable through the flow public import surface, not to create a second competing layout model.
3. This set should not reopen the broader pipeline layout contract work from Set 1.
4. Residual `.handbook` wording cleanup belongs primarily to Set 3 unless it blocks the public API shape directly.

### Suggested Live Evidence Sweep

```bash
rg -n "CanonicalArtifacts::load\(|load_with_contract|missing canonical \.handbook root|canonical \.handbook root" crates/flow/src crates/engine/src
```

### Likely Packet Shape

1. `Packet 2.1: Flow Public API Contract Shape`
2. `Packet 2.2: Resolver Adoption And Test Coverage`
3. `Packet 2.3: Final Set Proof`

### Set-Level Success Criteria To Carry Into `SPECIFY`

- a supported public `handbook-flow` import path can resolve using a non-default canonical layout contract
- the flow-facing canonical-root behavior is consistent with the engine-owned canonical layout contract
- downstream consumers do not need to rely on handbook-product default canonical-root behavior when integrating flow under `.substrate/handbook/**`

### Gate Before Opening Set 3

Do not open Set 3 until Set 2's triplet has reached a reviewed stopping point and the active authority clearly records the flow-facing canonical-layout injection story.

---

## Set 3 — Import-Surface Default / Validation Honesty Cleanup

### Status

- Blocked on Sets 1 and 2 finishing first
- No set-specific triplet has been written yet

### Proposed Triplet Stem

- `handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup`

### Seam

Clean the remaining import-target-crate defaults, validation text, refusal text, and fallback wording so the user-visible and developer-visible story matches the landed layout contracts honestly.

### Why This Is Its Own Set

- it is a residual honesty and contract-bounding seam, not the primary structural parameterization work
- it should derive from the actual Set 1 and Set 2 results rather than pre-guess them
- it is the right place to separate true reusable-import assumptions from intentionally retained handbook-product defaults

### Gaps This Set Owns

1. residual `.handbook` fallback/refusal wording in `handbook-flow`
2. residual `core/stages/` and `core/pipelines/` validation/refusal wording in `handbook-pipeline`
3. engine-side handbook-product default references that still need either:
   - parameterized derivation from the active contract
   - or explicit bounding as product-default references outside the reusable import promise
4. final explicit statement of what import-target defaults remain intentionally code-owned after the structural sets land

### Primary Authority Inputs

- `docs/specs/handbook-substrate-parameterization-gap-map.md`
- the landed Set 1 triplet
- the landed Set 2 triplet
- `crates/flow/src/resolver.rs`
- `crates/pipeline/src/pipeline.rs`
- `crates/engine/src/canonical_artifacts.rs`
- `crates/engine/src/author/*.rs` where canonical handbook-product references still appear inside the import-target crates

### Assumptions Future Agents Should Surface In `SPECIFY`

1. CLI/compiler product-shell cleanup remains out of scope even though those crates may still mention `.handbook/**`.
2. This set should clean or explicitly bound only the residual defaults that affect the import-target crate story.
3. Not every handbook-product default literal must disappear if it is intentionally outside the reusable import contract, but the boundary must be explicit and honest.
4. This set should not reopen public API design unless Set 1 or Set 2 left a documented blocker.

### Suggested Live Evidence Sweep

```bash
rg -n "missing canonical \.handbook root|canonical \.handbook root|core/stages/|core/pipelines/|\.handbook/" crates/engine/src crates/pipeline/src crates/flow/src
```

### Likely Packet Shape

1. `Packet 3.1: Flow Residual Refusal And Fallback Cleanup`
2. `Packet 3.2: Pipeline Validation / Refusal Wording Cleanup`
3. `Packet 3.3: Engine Residual Default Bounding And Final Proof`

### Set-Level Success Criteria To Carry Into `SPECIFY`

- import-target crate validation/refusal/default wording no longer misrepresents the active layout contract story
- any remaining handbook-product defaults inside the import-target crates are explicitly bounded and justified
- the final import-target story is honest: structural parameterization is real, residual product defaults are either removed or clearly declared out of the reusable import promise

### Final Stop Condition

After Set 3 lands, stop and reassess from live repo truth before opening any further family. Do **not** automatically widen into CLI/compiler work.

---

## Execution Summary

| Set | Status | Blocks next set? | Main owner seam |
| --- | --- | --- | --- |
| Set 1 | Next / not started | Yes | `handbook-pipeline` declarative + stage + storage layout parameterization |
| Set 2 | Planned / blocked on Set 1 | Yes | `handbook-flow` public canonical-layout injection |
| Set 3 | Planned / blocked on Sets 1-2 | — | residual import-target default / validation honesty cleanup |

---

## Recommended Future-Agent Flow

When resuming this work:

1. read this file
2. read `docs/specs/handbook-substrate-parameterization-gap-map.md`
3. confirm which set is currently active
4. author the quartet for **that set only**:
   - spec
   - plan
   - tasks
   - packet prompts
5. land or review the active set
6. return to this map before opening the next set

Do **not** use this file to justify writing all three triplets up front.
