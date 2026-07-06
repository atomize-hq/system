# MAP: Handbook Canonical Artifact Model + Validation Alignment

Status: planned (replaces the completed published-import decoupling MAP, archived 2026-06-26)  
Scope: canonical artifact structure, validation alignment, YAML-backed artifact truth, and resolution-aware rendered views  
Primary repo: `/Users/spensermcconnell/__Active_Code/system`  
Related downstream consumers: Substrate and any future consumer that needs to merge, validate, transform, or render handbook artifacts programmatically  

---

## ACTIVE AUTHORITY STACK FOR THIS SEAM

Use the following authority order for the canonical-artifact + validation-alignment workstream:

1. `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`
   - root authority for exact objective, exact intent, boundary principles, and set sequencing
2. the currently active set triplet under `/Users/spensermcconnell/__Active_Code/system/docs/specs/`
   - once created, the active `spec.md`, `plan.md`, and `tasks.md` for the current set become the execution authority for that set
   - until a set triplet exists, this MAP is the sole active planning authority
3. `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-published-import-decoupling/`
   - provenance only; this prior workstream is complete and archived
4. `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/`
   - broader historical context only; do not treat archive docs as active execution authority
5. live repo truth in `crates/engine/`, `crates/pipeline/`, `core/pipelines/`, and `core/stages/`
   - implementation truth and current constraint surface

If any older document conflicts with this MAP plus the active set triplet, treat the older document as superseded for active planning.

---

## EXACT OBJECTIVE

Handbook artifacts must become **reviewed, reusable, programmatically manipulable canonical contracts** so that downstream consumers can:

- validate them consistently
- merge and update them structurally
- reconstruct the same underlying artifact at different resolutions
- render human-readable views without treating freeform markdown as the only source of truth

In practice, that means the artifact families that matter for downstream planning/runtime work must move toward a **canonical structured representation** — YAML where that structure should be durable and machine-meaningful — with validation and rendering built around that canonical truth.

The target is **not** “YAML everywhere for its own sake.”

The target **is** a system where:

1. the canonical artifact model is structured and durable,
2. validation support is consistent across supported artifact families,
3. rendered markdown remains available where useful for human review,
4. downstream consumers can request different detail/resolution views from the same artifact truth,
5. the implementation avoids duplicate, drifting sources of truth.

## EXACT INTENT

The goal is **maximum downstream artifact reuse and manipulability with minimum duplicate truth and minimum throwaway validation work**.

This work is **not** trying to preserve markdown as the permanent canonical artifact format if that blocks structured manipulation.

This work is **not** trying to rush a big-bang rewrite of every artifact family at once.

This work **is** trying to ensure that:

1. Every supported artifact family has a clear canonical validation contract.
2. The trio (`handbook-engine` / `handbook-pipeline` / `handbook-flow`) exposes reusable artifact capabilities through the right owner seams.
3. Structured artifact truth can support:
   - merge/update operations
   - resolution-aware reconstruction
   - rendered markdown or other views
4. Near-term implementation effort goes into seams that survive the YAML migration, rather than rich markdown-only logic that will be thrown away.
5. Reviews measure progress against the real target:
   - **Does this change move us closer to one canonical structured artifact truth?**
   - **Does it improve reusable validation/manipulation/rendering support without multiplying duplicate representations?**

If a change does not improve one of those two things, it is not real progress toward this objective.

---

## GUIDING LIGHT FOR EVERY REVIEW

Every review should explicitly answer these questions:

1. **Canonical truth check:** Does this change move an artifact family closer to a single structured canonical source of truth?
2. **Validation check:** Does this change improve reusable validation support at the right owner seam, instead of adding one-off special cases?
3. **Resolution check:** Does this preserve or improve the ability to reconstruct the same artifact at different levels of detail from the same underlying truth?
4. **Drift check:** Did we introduce or preserve duplicate representations that can diverge from one another?
5. **Boundary check:** Is the exposed surface the minimum reusable contract needed, without leaking handbook product-shell or implementation-only behavior?
6. **Migration check:** Is this a low-regret step that survives the YAML-backed canonical model, or are we over-investing in temporary markdown-only logic?

Any review that cannot answer those questions clearly is not complete.

---

## WHAT SUCCESS LOOKS LIKE

This effort is successful only when all of the following are true:

1. Supported artifact families have a declared canonical contract and validation profile.
2. Capture/authoring/runtime flows validate against that canonical contract at the correct owner seam.
3. Rendered markdown becomes a derived view where appropriate, not the only durable truth.
4. At least the highest-value structured artifact families can be merged, updated, and transformed programmatically.
5. The same canonical artifact truth can produce more than one resolution/detail view without hand-maintained duplicate content.
6. The boundary remains intentionally scoped:
   - reusable artifact contracts are exposed
   - product-shell wording, CLI-only behavior, and implementation-only helpers do not leak by accident.

---

## WHAT DOES **NOT** COUNT AS SUCCESS

The following are explicitly insufficient:

- adding more markdown-only validators for artifact families that are about to be replaced by canonical YAML-backed forms
- keeping both markdown and YAML as independent editable truths for the same artifact family
- stage-specific capture rules that solve one artifact locally but do not create a reusable validation seam
- YAML files that are only opaque text dumps with no real schema/semantic validation value
- claiming multi-resolution support when each resolution is hand-maintained rather than derived from shared underlying truth
- moving logic into `handbook-compiler` that should be owned by `handbook-engine` or `handbook-pipeline`

---

## CURRENT REPO TRUTH

As of 2026-06-26, the honest state is:

- the handbook published-import decoupling workstream is complete and has been archived under:
  - `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/handbook-published-import-decoupling/`
- `handbook-engine` already contains reusable validation primitives for:
  - charter structured input
  - charter markdown
  - project-context structured input / markdown
  - environment-inventory markdown
- `handbook-pipeline` already owns the runtime surfaces for compile / capture / handoff, but validation depth is uneven across artifact families
- `pipeline compile` is currently narrow and effectively centered on the currently supported feature-spec compile target
- `pipeline capture` currently supports only a bounded stage set, not every declared stage uniformly
- `stage.10_feature_spec` has the strongest current capture/provenance validation story
- several other artifact families remain markdown-first and are weakly validated relative to the feature-spec path
- `CHARTER_INPUTS.yaml` and `quality_gates.yaml` already show that some artifact families naturally want structured machine-oriented truth
- if downstream consumers need clean merge/update/reconstruction workflows, the current markdown-first artifact posture is not sufficient as the long-term canonical model

The honest starting point is:

> We already have useful reusable validation pieces and one relatively strong end-to-end feature-spec path, but the artifact system is still uneven, too markdown-first in the wrong places, and not yet organized around a single structured canonical truth that can support merge, transformation, and multi-resolution rendering cleanly.

---

## REQUIRED BOUNDARY PRINCIPLE

Use this rule throughout the work:

> **Canonical data first; rendered views second.**

What should become durable canonical truth:

- structured artifact fields that downstream consumers need to read, merge, validate, update, or transform
- schema versions
- typed semantic expectations
- provenance-worthy structural content
- fields that must survive re-rendering at different resolutions

What may remain derived views:

- markdown review presentations
- consumer-specific phrasing/rendering
- abbreviated and expanded detail projections
- presentation ordering or formatting choices that do not change underlying meaning

What should stay out of the canonical contract unless proven necessary:

- CLI/product-shell wording
- implementation-only formatting shims
- handbook-specific presentational scaffolding that downstream consumers do not actually need semantically
- duplicate manually maintained copies of the same artifact in different formats

If a downstream consumer needs to manipulate or merge an artifact reliably, prefer to move that artifact family toward structured canonical truth instead of building more parsing around prose.

---

## TOP-LEVEL SET MAP

This work is currently planned as **four sets**.

| Set | Name | Purpose | Output |
|---|---|---|---|
| 1 | Validation seam foundation + low-regret alignment | Freeze the validation owner seam, wire existing reusable validators into runtime capture, and align the highest-value existing artifact families | code + tests + active spec/plan/tasks triplet |
| 2 | Foundation-family canonical YAML cutover | Move the foundation-oriented artifact family toward canonical structured truth with derived views where needed | schemas/models + renderers + validation + migration docs |
| 3 | Core artifact canonicalization | Extend the canonical structured model to the remaining core artifact families that should not stay markdown-first | code + tests + cutover docs + proof |
| 4 | Reconstruction / merge / resolution proof + guard rails | Prove that the canonical artifact system supports merge/update/re-render workflows cleanly and lock in regression protection | proof wall + downstream usage proof + guard rails |

The sets are sequential. Do not skip ahead.

---

## SET 1 — VALIDATION SEAM FOUNDATION + LOW-REGRET ALIGNMENT

### Purpose

Create the reusable validation seam that survives the canonical YAML migration, then use it to align the artifact families where the validator logic already exists or is obviously worth hardening now.

### This set should establish

- engine-owned artifact validation profiles / dispatch
- pipeline-owned invocation of those validators during capture/runtime enforcement
- consistent refusal/reporting shape for validation failures
- immediate alignment for low-regret artifact families such as:
  - `CHARTER_INPUTS.yaml`
  - `CHARTER.md`
  - `PROJECT_CONTEXT.md`
  - `ENVIRONMENT_INVENTORY.md`
  - `quality_gates.yaml` if it can be hardened without speculative overbuild

### This set must avoid

- rich markdown-only semantics for artifact families that are likely to be replaced by canonical YAML shortly
- pushing durable validation ownership into `handbook-compiler`
- pretending that stage-specific one-offs are the same thing as a reusable validation seam

### Key output

The first active set triplet under `/Users/spensermcconnell/__Active_Code/system/docs/specs/`.

### Review focus

Reviews in Set 1 should primarily ask:

- did we create a reusable validation seam at the correct owner boundary?
- did we wire existing high-confidence validators into runtime enforcement?
- did we avoid inventing rich temporary markdown-only logic for artifact families that are about to migrate?

### Stop boundary

Do not widen Set 1 into broad artifact-family redesign.
Set 1 is about owner seams and low-regret validation alignment.

### Transition posture into Set 2

Set 1 is complete only when future set planning can assume:

- there is one clear validation-owner story,
- runtime capture knows how to invoke reusable validators,
- and the next step is no longer “where should validation live?” but “which artifact families now become canonical structured truth first?”

---

## SET 2 — FOUNDATION-FAMILY CANONICAL YAML CUTOVER

### Purpose

Move the foundation-family artifacts toward canonical structured truth so they can be merged, updated, rendered, and reconstructed at different resolutions from one underlying representation.

### This set should focus on

- the foundation-family artifacts where structure is durable and machine-meaningful
- replacing fragile markdown-first canonical assumptions with YAML-backed truth where that improves downstream manipulability
- keeping markdown available as a derived review/output surface where useful

### Likely artifact candidates

- `quality_gates.yaml`
- foundation-pack outputs that should become structured rather than prose-primary
- any directly adjacent artifact metadata needed to reconstruct those outputs at different detail levels

### This set must avoid

- creating permanent dual editable truths
- treating “YAML dump plus unrelated markdown file” as a real canonical model
- overfitting the structured model to one presentation surface

### Key output

An active Set 2 triplet plus implementation/proof that at least one real foundation-family artifact group is:

- canonically structured,
- validated at the structured layer,
- and rendered from that structured truth rather than manually maintained as independent prose.

### Review focus

Reviews in Set 2 should primarily ask:

- did we create one structured canonical truth rather than parallel editable markdown + YAML?
- is the YAML/schema/model actually meaningful for merge/update/reconstruction, rather than just a serialized copy of prose?
- are rendered views clearly derived from canonical truth?

### Stop boundary

Do not claim Set 2 complete until at least one meaningful foundation-family artifact group can:

1. validate structurally/semantically,
2. render a human-facing view, and
3. support programmatic merge/update without prose parsing.

### Transition posture into Set 3

Set 2 is complete only when future set planning can assume:

- the repo has at least one proven canonical-YAML artifact pattern,
- the renderer/validation split is no longer hypothetical,
- and Set 3 can extend the pattern artifact-family-by-artifact-family instead of inventing it from scratch again.

---

## SET 3 — CORE ARTIFACT CANONICALIZATION

### Purpose

Extend the canonical structured model to the remaining artifact families that should not remain markdown-first if downstream consumers need reliable transformation, projection, and reuse.

### This set should decide, artifact by artifact

- which artifacts should become YAML-backed canonical truth
- which artifacts may remain rendered views only
- which existing markdown validators should be retained, minimized, or retired

### Likely focus families

- charter-related artifacts
- project-context artifacts
- feature-spec ownership boundaries, if structured canonical truth improves downstream reconstruction and mergeability enough to justify the cutover

### This set must avoid

- forcing every artifact family through the same representation regardless of real need
- destabilizing well-proven paths without replacement proof
- conflating render quality with canonical data quality

### Key output

An active Set 3 triplet plus artifact-family decisions and implementations that make the remaining important core artifacts honest about:

- what the canonical truth is,
- what the derived views are,
- and which legacy markdown-first assumptions are retired.

### Review focus

Reviews in Set 3 should primarily ask:

- for each artifact family, did we choose the right canonical representation rather than blindly applying one template?
- did we eliminate silent drift paths between structured truth and rendered views?
- did we preserve or improve downstream manipulability and resolution-aware reconstruction?

### Stop boundary

Do not claim Set 3 complete until each targeted artifact family has:

- one canonical source of truth,
- a declared validation contract,
- an explicit rendering/projection story,
- and no silent drift path between structured truth and rendered view.

### Transition posture into Set 4

Set 3 is complete only when future set planning can assume:

- the important artifact families have settled canonical truth,
- the remaining question is not representation design but proof of operational value,
- and Set 4 can focus on proving merge/update/reconstruction and locking in guard rails.

---

## SET 4 — RECONSTRUCTION / MERGE / RESOLUTION PROOF + GUARD RAILS

### Purpose

Prove that the new artifact system is not merely “structured on paper,” but actually supports the workflows that motivated the migration.

### Required proof types

#### A. Merge/update proof

There must be a real proof that canonical artifact truth can be:

- updated partially,
- merged structurally,
- and re-rendered without corrupting meaning.

#### B. Resolution-aware reconstruction proof

There must be a real proof that the same canonical artifact truth can produce:

- a lower-detail requester view,
- a normal-detail view,
- and a higher-detail or expanded view,

without manual duplication of independent truth.

#### C. Runtime / downstream proof

There must be a real proof that the resulting artifact seams remain usable by real downstream consumers.

### Guard rails

This set should also lock in protections against:

- reintroducing duplicate editable truths
- bypassing canonical validation paths
- adding new artifact families directly as markdown-first without explicit approval
- regressions in structured merge/update/re-render workflows

### Stop boundary

Do not claim Set 4 complete until the proof wall demonstrates the actual reasons this migration was worthwhile.

### Key output

An active Set 4 triplet plus proof artifacts / tests / guard rails that demonstrate:

- structured merge/update works,
- multi-resolution reconstruction works,
- and future contributors cannot casually bypass the canonical artifact model without detection.

### Review focus

Reviews in Set 4 should primarily ask:

- did we prove the workflows that motivated the migration, not just the presence of new schemas?
- did we make regressions against canonical-truth discipline easier to catch?
- is the resulting system genuinely better for downstream consumers than the markdown-first baseline?

---

## TRANSITION RULE BETWEEN SETS

The MAP must remain strong enough to guide the repo during the periods between:

- finishing one set,
- planning the next set,
- and beginning implementation of that next set.

During those transition periods, use this rule:

1. the completed set defines what is now safe to assume,
2. this MAP defines what the next set must accomplish and what it must not widen into,
3. the next active set triplet should only add execution detail, not replace the MAP's objective/intent/boundary logic.

If a proposed next-set plan cannot be justified directly from this MAP's exact objective, exact intent, set purpose, review focus, and transition posture, it is not ready to become active execution authority.

---

## OWNER SPLIT FOR THIS WORKSTREAM

Use this owner split unless live implementation truth proves otherwise:

### `handbook-engine` should own

- reusable artifact validation contracts
- canonical structured artifact models
- schema/semantic validation
- renderer inputs and artifact-family transformation logic that is not pipeline-runtime-specific

### `handbook-pipeline` should own

- runtime capture/compile/handoff orchestration
- validator invocation during capture/runtime flows
- state/provenance/handoff enforcement
- stage support gates and capture/refusal integration

### `handbook-flow` should own

- only flow/resolver/budget/packet concerns
- not canonical artifact validation ownership unless a specific flow contract truly needs it

### `handbook-compiler` should retain only transition glue where explicitly necessary

If a durable reusable validation or canonical artifact contract lives only in `handbook-compiler`, that is usually a sign the boundary is landing in the wrong place.

---

## REVIEW POSTURE FOR FUTURE TASKS / SPECS / PACKETS

When future set docs, tasks, and packet prompts land, they should be judged against this MAP first.

The right question is not merely:

> “Is this code locally correct?”

The right questions are:

- does it move an artifact family toward canonical structured truth?
- does it reduce drift risk?
- does it improve reusable validation at the right owner seam?
- does it preserve the ability to derive multiple views from one underlying truth?
- does it avoid throwaway markdown-only overbuild where a structured cutover is the real destination?

That is the guiding light for this workstream.
