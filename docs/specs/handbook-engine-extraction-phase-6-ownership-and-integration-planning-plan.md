# Plan: Handbook Engine Extraction Phase 6 - Ownership And Integration Planning

## Status

- Planned planning-only family.
- This plan operationalizes the exact follow-on family named by Packet 6.1.4 after the Phase 6 Slice 1 READY verdict.
- This file is the PLAN stage artifact for the family. It assumes the spec is the source of truth, keeps implementation blocked, and exists to produce a reviewable execution approach before any coding starts.
- The packet order below is sequential and intentionally bounded so the family does not spill into implementation.
- Core planning outputs stay inside this SPEC/PLAN/TASKS triplet. The separately approved `handbook-engine-extraction-phase-6-ownership-and-integration-planning-packet-prompts.md` artifact may exist to orchestrate packet landings, but it does not authorize implementation by itself.

## Objective

Turn the Phase 6 ownership question into an explicit, reviewable decision set: who owns each extracted crate architecturally, how Substrate should integrate with it, and which later execution seams would be required after approval.

Spec reference: [handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md](./handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md)

## Sequencing And Parallelism

- Sequential by default: Task Group 1 → Task Group 2 → Task Group 3 → Task Group 4.
- Task Group 1 must land first because every later ownership call depends on the current authority chain and READY starting gate.
- Task Groups 2 and 3 can gather evidence independently, but the authoritative decision table still lives in the same triplet, so they should be reviewed as one bounded planning family rather than treated as separate execution lanes.
- Task Group 4 stays last because downstream seam naming only becomes honest after the per-crate ownership posture is explicit.
- No implementation-parallel work is authorized from this plan; human approval of the triplet is still required before any IMPLEMENT phase begins.

## Major Artifacts

1. Ownership decision framework
   - translates the root plan's Phase 6 rule into explicit crate-by-crate decision criteria
   - keeps current code truth separate from future execution intent

2. Per-crate ownership and integration boundary table
   - covers `handbook-engine`, `handbook-pipeline`, `handbook-flow`, `handbook-cli`, and retained `handbook-compiler`
   - records owner, Substrate posture, intended boundary, and follow-on seam

3. Handbook shell/glue classification
   - makes reusable core vs product shell vs transition glue explicit
   - prevents CLI and retained compiler from being treated as ambiguous leftovers

4. Downstream execution seam map
   - names what later work would have to happen after approval
   - keeps those seams bounded and reviewable instead of hidden in planning prose

## Planned Task Group Order

### Task Group 1: Freeze current authority and decision rule

Why first:

- this family is only honest if it starts from the already-READY Phase 6 reassessment
- Packet 6.1.2 is useful input, but it cannot be copied forward unmodified because the later engine layout-boundary fix changed the readiness posture
- the plan must prove it is grounded in current HEAD and not in stale pre-fix wording

Output:

- explicit assumption set tied to `aa882af...` and `a883d16...`
- explicit restatement of the root ownership decision rule
- explicit scope guard that this family is docs-only and planning-only

### Task Group 2: Decide handbook-owned imported-core boundaries

Why second:

- `handbook-engine` and `handbook-pipeline` are the strongest import candidates, but they are not identical
- engine already looks like handbook-owned reusable core, while pipeline still carries bounded runtime/fixture coupling that must be named honestly
- separating them from the shell/glue surfaces prevents a false all-crates-at-once decision

Output:

- one explicit ownership/import call for `handbook-engine`
- one explicit ownership/import call for `handbook-pipeline`
- intended import boundary text for both crates
- explicit downstream seam for any remaining pipeline boundary cleanup

### Task Group 3: Decide handbook-side deferred boundaries and non-targets

Why third:

- `handbook-flow`, `handbook-cli`, and retained `handbook-compiler` are where agents are most likely to overstate certainty or widen scope
- flow still needs explicit long-term posture even if it stays handbook-owned longer
- CLI shell and retained compiler glue need explicit non-target treatment so later execution does not invent migration work that Phase 6 never approved

Output:

- one explicit ownership/import call for `handbook-flow`
- one explicit non-target/product-shell call for `handbook-cli`
- one explicit transition-glue/non-target call for retained `handbook-compiler`
- explicit downstream seam text for future flow clarification and compiler/CLI narrowing work

### Task Group 4: Define downstream execution seams and review gate

Why last:

- the family is not done when ownership prose exists; it is done when later implementation can start from bounded seams instead of from vague “follow up later” language
- ending with an explicit review gate prevents this triplet from being mistaken for execution approval

Output:

- one bounded downstream seam map for approved later execution
- one explicit statement of what is still out of scope
- one human review gate before code or packet-prompt work begins

## Expected Decision Shape

The plan should converge on this shape unless live repo truth disproves it:

- `handbook-engine`
  - handbook remains the architectural owner
  - Substrate consumes through the engine crate's public export surface
  - any future work is about adapter freeze or boundary documentation, not repo transfer
- `handbook-pipeline`
  - handbook remains the architectural owner
  - Substrate consumes through the reviewed pipeline surface
  - later execution may narrow fixture/support coupling before any stronger portability claim
- `handbook-flow`
  - handbook remains the architectural owner for now
  - any future import story must be narrower and separately justified
  - current planning should not pretend this is an active move target
- `handbook-cli`
  - remains handbook-owned product shell
  - not an import target
  - any future shell redesign stays outside this family
- retained `handbook-compiler`
  - remains handbook-owned transition glue
  - not an import target
  - later narrowing/retirement timing is a separate execution concern

## Risks And Mitigations

### Risk: The plan repeats Packet 6.1.2 without updating for the later boundary fix

Mitigation:

- require the current HEAD / `aa882af...` / `a883d16...` chain to be named explicitly
- require the later READY verdict to supersede stale `handbook_product_canonical_layout_contract` framing

### Risk: Pipeline coupling is misread as either a blocker or proof of Substrate ownership

Mitigation:

- treat the current pipeline posture as handbook-owned imported core with explicit bounded coupling
- move any cleanup into a later named seam instead of inflating the current planning claim

### Risk: CLI shell or retained compiler get treated as migration targets by omission

Mitigation:

- require explicit non-target language for `handbook-cli`
- require explicit transition-glue language for retained `handbook-compiler`

### Risk: Planning widens into implementation or earlier closeout reopen

Mitigation:

- keep core planning conclusions inside the spec/plan/tasks triplet
- allow the explicitly approved packet-prompts artifact to exist as orchestration support only, not as execution approval
- name any contradiction with its owning seam instead of silently changing code scope
- end with a review gate rather than an execution checklist

## Verification Checkpoints

### Checkpoint 1: Current authority chain is explicit

Confirm:

- current HEAD is recorded
- the docs-only delta from `aa882af...` to current HEAD is explicit
- the READY Phase 6 reassessment is treated as the starting gate

Suggested verification:

```bash
git status --short --branch
git rev-parse HEAD
git diff --stat aa882af42792a250cc02a6740bd1e2123178caff..HEAD
rg -n "READY|Packet 6\.1\.4|ownership-and-integration-planning" docs/specs/handbook-engine-extraction-phase-6-slice-1-migration-readiness-reassessment-*.md
```

### Checkpoint 2: Every crate has an explicit ownership/import posture

Confirm:

- no crate is left implied
- owner, Substrate posture, intended boundary, and follow-on seam are explicit for each crate

Suggested verification:

```bash
rg -n "pub use|pub mod|mod " crates/engine/src/lib.rs crates/pipeline/src/lib.rs crates/flow/src/lib.rs crates/compiler/src/lib.rs crates/cli/src/main.rs
cargo tree -p handbook-engine
cargo tree -p handbook-pipeline
cargo tree -p handbook-flow
cargo tree -p handbook-compiler
```

### Checkpoint 3: Shell and glue boundaries are treated honestly

Confirm:

- `handbook-cli` is recorded as product shell rather than reusable imported core
- retained `handbook-compiler` is recorded as transition glue rather than future owner target
- `handbook-flow` is not overclaimed as a settled move target

Suggested verification:

```bash
rg -n "CommandFactory|ExitCode|pipeline_help|doctor|rendering|prompt" crates/cli/src/main.rs crates/cli/src
rg -n "rendering|refusal|doctor|setup|template_library" crates/compiler/src
rg -n "default_canonical_layout_contract|resolver|PacketResult|BudgetOutcome" crates/engine/src crates/flow/src crates/compiler/src
```

### Checkpoint 4: Downstream seams are named without starting execution

Confirm:

- the triplet names bounded later seams
- no code-task language sneaks into the current planning pass
- no extra artifacts beyond the requested triplet are required to understand the next move

Suggested verification:

- manual review against the scope, boundaries, and task ledger in this triplet

## Proposed Downstream Execution Seams After Approval

These are named here so later work starts bounded; they are not started by this family.

1. `handbook-engine` adapter/boundary freeze seam
   - only if human review decides the current engine export surface needs a thinner documented import boundary

2. `handbook-pipeline` boundary cleanup seam
   - isolates any remaining compiler-backed fixture/support coupling from the runtime ownership story

3. `handbook-flow` ownership clarification seam
   - only if human review wants a narrower future importable slice rather than handbook-owned longer-term composition

4. retained `handbook-compiler` narrowing seam
   - reduces remaining support/glue exposure after the owner-layer decisions are accepted

5. CLI shell/support seam
   - only if later work needs to clarify shell-facing rendering/refusal/help ownership without broad redesign

## Planned Exit Conditions

This planning family should be considered complete only when all of the following are true:

- the triplet encodes the current Phase 6 starting gate honestly
- each in-scope crate has an explicit ownership/import posture
- reusable core vs product shell vs transition glue is explicit
- downstream execution seams are named and bounded
- no implementation work has started
- the result is ready for human review and approval before any execution slice begins
