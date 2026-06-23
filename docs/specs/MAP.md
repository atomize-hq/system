# MAP: Handbook Published-Import Decoupling for Substrate

Status: active  
Scope: handbook published-boundary work needed for real Substrate consumption  
Primary repo: `/Users/spensermcconnell/__Active_Code/system`  
Related downstream proof/worktree: `/Users/spensermcconnell/.codex/worktrees/substrate-packet-4-2-20260622-133054`  

---

## ACTIVE AUTHORITY STACK FOR THIS SEAM

Use the following authority order for the handbook published-import decoupling workstream:

1. `/Users/spensermcconnell/__Active_Code/system/docs/specs/MAP.md`
   - root authority for exact objective, exact intent, and set sequencing
2. the active Set 1 triplet under `/Users/spensermcconnell/__Active_Code/system/docs/specs/`
   - `handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`
   - `handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`
   - `handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`
   - current execution authority for Set 1 details and acceptance
3. `/Users/spensermcconnell/__Active_Code/system/HANDBOOK_PUBLISHED_IMPORT_DECOUPLING_AUDIT_2026-06-23.md`
   - freshness evidence and provenance input, not stronger authority than this MAP plus the active Set 1 triplet
4. `/Users/spensermcconnell/__Active_Code/system/docs/specs/archive/`
   - provenance only; do not treat archive docs as active execution authority
5. `/Users/spensermcconnell/.codex/worktrees/9b83/substrate/docs/`
   - stale, non-canonical context only; may inform provider-boundary thinking but must not overrule live `system` repo truth

If any older document conflicts with this MAP plus the active Set 1 triplet, treat the older document as superseded for active planning.

---

## EXACT OBJECTIVE

Substrate must be able to consume **all reusable handbook capabilities it actually needs**, including the full reusable `handbook-pipeline` capability set, through a **reviewed, stable, published boundary**.

That means Substrate must be able to:

- choose its own declarative roots
- choose its own state / capture / handoff storage roots
- use the reusable pipeline mechanics
- preserve Substrate ownership of product/runtime wording and behavior
- consume handbook through published crate boundaries rather than private internals or sibling-path accidents

## EXACT INTENT

The goal is **maximum functional capability for Substrate with minimum unnecessary public surface**.

This work is **not** trying to make all of handbook public.

This work **is** trying to ensure that:

1. Substrate gets every reusable capability it truly requires.
2. Only the seams required to provide those capabilities become public/stable.
3. Handbook product-shell internals, wording, CLI-specific behavior, and implementation-only helpers do **not** get exposed by accident.
4. Reviews measure progress against the real target:
   - **Does this change move us closer to full required Substrate capability?**
   - **Does it avoid exposing more than necessary?**

If a change does not improve one of those two things, it is not real progress toward this objective.

---

## GUIDING LIGHT FOR EVERY REVIEW

Every review should explicitly answer these questions:

1. **Capability check:** Does this change give Substrate more of the reusable handbook capability it actually requires?
2. **Boundary check:** Is the newly exposed surface the smallest stable API that can provide that capability?
3. **Leak check:** Did we accidentally expose handbook product-shell behavior, wording, CLI assumptions, or implementation-only helpers?
4. **Proof check:** Is the claimed capability validated through an external/published-consumer proof, not just internal tests?
5. **Intent check:** Does this preserve the intended ownership split?
   - handbook owns reusable typed mechanics and contracts
   - Substrate owns product/runtime wording, integration behavior, and downstream experience

Any review that cannot answer those questions clearly is not complete.

---

## WHAT SUCCESS LOOKS LIKE

This effort is successful only when all of the following are true:

1. Substrate can access the full reusable handbook capability it needs, including the reusable `handbook-pipeline` capability set.
2. That access happens through a reviewed public boundary, not private modules or repo-local accidents.
3. The public boundary is intentionally scoped:
   - required capability is exposed
   - unnecessary internals stay private
4. External consumer proof passes against the published surface.
5. Downstream Substrate proof passes against the published surface.
6. Final wording/rendering/runtime behavior remains Substrate-owned where that is the intended ownership boundary.

---

## WHAT DOES **NOT** COUNT AS SUCCESS

The following are explicitly insufficient:

- internal parameterization that is still private in the published crate
- docs that claim a seam is public when crates.io consumers cannot use it
- Substrate proof that only exercises `engine + flow` while claiming `pipeline` is also complete
- making broad handbook internals public “just to unblock Substrate”
- preserving capability only through sibling-path coupling or workspace accidents

---

## CURRENT REPO TRUTH

As of 2026-06-23, the honest state is:

- `handbook-engine` is published and externally consumable
- `handbook-flow` is published and externally consumable
- Substrate has a real published-consumption proof for a narrow `engine + flow` seam
- `handbook-pipeline` is **not yet** a verified public import seam for the full reusable capability Substrate needs

The key current gap is:

> Substrate needs the reusable `handbook-pipeline` capability set, but the current published surface still keeps the key declarative-root and layout-control seams private.

That is the gap this map is routing toward closure.

---

## REQUIRED BOUNDARY PRINCIPLE

Use this rule throughout the work:

> **Expose capabilities, not guts.**

What should become public:

- the minimum reviewed control surface Substrate needs
- typed contracts
- constructors / validation entrypoints
- pipeline-facing entrypoints that accept the contracts
- typed results/errors/outputs Substrate must use

What should stay private unless proven necessary:

- implementation-only helper structs
- default constants that only represent handbook’s own product defaults
- product-shell wording
- CLI-specific behavior
- internal repo/file/path plumbing
- convenience shims that exist only for handbook as its own product

If Substrate needs a capability and the only current implementation lives behind a private helper, the preferred fix is:

1. expose a narrow reviewed public seam, or
2. wrap the helper in a narrower façade,

instead of simply making all internals public.

---

## TOP-LEVEL SET MAP

This work is currently planned as **three sets**.

| Set | Name | Purpose | Output |
|---|---|---|---|
| 1 | Authority reconciliation + objective lock | Make the exact objective/intent explicit and reconcile conflicting prior authorities | active spec/plan/tasks triplet |
| 2 | Minimal public capability boundary for `handbook-pipeline` | Expose the smallest stable public API that gives Substrate the full reusable pipeline capability it requires | code + tests + updated boundary docs |
| 3 | Published-consumer proof + Substrate proof + guard rails | Prove the boundary works externally and downstream, then lock in regression protection | proof wall + downstream proof + release/update guard rails |

The sets are sequential. Do not skip ahead.

---

## SET 1 — AUTHORITY RECONCILIATION + OBJECTIVE LOCK

### Purpose

Resolve the conflict between:

- archived docs that overclaimed pipeline public readiness
- current published-crate truth
- the actual Substrate requirement that full reusable capability must become available

### Key output

The active Set 1 triplet:

- `/Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-spec.md`
- `/Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-plan.md`
- `/Users/spensermcconnell/__Active_Code/system/docs/specs/handbook-published-import-decoupling-set-1-pipeline-boundary-authority-reconciliation-tasks.md`

### Set 1 intent

Set 1 must make one thing explicit:

> Substrate requires full reusable handbook capability, but Set 2 must expose only the minimum reviewed public surface needed to provide it.

### Stop boundary

Set 1 is docs-only.  
Do not implement production code in Set 1.

---

## SET 2 — MINIMAL PUBLIC CAPABILITY BOUNDARY FOR `handbook-pipeline`

### Purpose

Implement the smallest reviewed public boundary that gives Substrate the reusable `handbook-pipeline` capability it actually requires.

### This set should enable

- declarative root control
- stage-root-aware reusable pipeline behavior
- storage layout control for state/capture/handoff
- reusable pipeline entrypoints through a supported public seam
- typed downstream integration without private-module reach-in

### This set must avoid

- exposing handbook product-shell wording
- exposing CLI/product-only behavior unless truly required
- publishing raw internals when a narrower façade can provide the same capability
- claiming completion based only on internal tests

### Expected shape

Set 2 may expose:

- public root/layout contracts
- public validation/constructor APIs
- public pipeline entrypoints that accept those contracts
- any additional typed outputs/errors Substrate must use

Set 2 should try to keep private:

- implementation helpers
- default-only product constants
- repo plumbing
- product-shell affordances

### Stop boundary

Do not claim Set 2 complete until an external published-consumer proof demonstrates the capability from outside the crate boundary.

---

## SET 3 — PUBLISHED-CONSUMER PROOF + SUBSTRATE PROOF + GUARD RAILS

### Purpose

Prove that the Set 2 boundary works:

1. for an external published consumer
2. for real Substrate downstream usage
3. without reopening accidental overexposure

### Required proof types

#### A. External consumer proof

There must be a real external consumer proof that can:

- configure the required roots/layouts
- exercise the reusable pipeline capability
- do so through the published boundary only

#### B. Downstream Substrate proof

Substrate must prove it can use the capability it actually needs through the reviewed boundary, while keeping:

- Substrate-owned wording
- Substrate-owned runtime behavior
- dedicated worktree discipline for downstream source-touching proof

#### C. Guard rails

Add protection so we do not regress back to:

- “internally configurable but externally private”
- “docs say public but crates.io says private”
- “Substrate proof only covered `engine + flow` but was treated as pipeline proof”

### Stop boundary

Do not close the overall effort until both external and downstream proofs are honest and reproducible.

---

## ORDER OF EXECUTION

1. **Set 1 first**
   - objective lock
   - authority reconciliation
   - exact Set 2 target

2. **Set 2 second**
   - capability boundary implementation
   - minimal reviewed public surface

3. **Set 3 third**
   - external proof
   - downstream Substrate proof
   - guard rails

---

## DOWNSTREAM SUBSTRATE CONTEXT RULE

Substrate intent is a required context input for this work.

That means:

- we must evaluate boundary decisions against how Substrate actually needs to use handbook
- but stale Substrate notes do **not** override live `system` repo truth by themselves

Useful non-canonical context may inform the boundary shape, especially around provider/context ownership, but current live code and active authority docs remain stronger sources.

---

## DOWNSTREAM WORKTREE RULE

If downstream Substrate source-touching proof is required:

- do it in a dedicated Substrate worktree
- do **not** use the main Substrate checkout as the write path

This is a standing execution rule for downstream proof work.

---

## REVIEW FILTER FOR FUTURE CHANGES

Before accepting any future change in this workstream, ask:

### Promote

Promote a seam if:

- Substrate truly needs the capability
- the capability cannot be accessed through the existing public boundary
- the new public seam is stable and minimal

### Keep private

Keep a seam private if:

- it is only an implementation detail
- it exists only for handbook product-shell behavior
- Substrate does not need to depend on it directly
- a narrower façade can expose the needed capability instead

---

## NORTH STAR SENTENCE

> Give Substrate the full reusable handbook capability it truly needs through the smallest reviewed public boundary that preserves the intended ownership split and avoids accidental exposure of handbook-only internals.
