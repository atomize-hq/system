# HCM-0.6 Plan

## Goal

Turn the reserved shipped-default question into an evidence-backed user
decision while preserving the frozen kind/instance/requiredness contracts and
remaining documentation-only.

## Dependency graph

```text
completed HCM-0.2 semantic contracts
  + completed HCM-0.3/HCM-0.4/HCM-0.5 non-regression boundaries
  + completed HCM-0.8 orchestration controls
    -> current-surface inventory
    -> external primary-source research
    -> shared rubric and three candidate sets
    -> clean review of decision-ready research subject
    -> explicit user brainstorming/decision
    -> approved decision + narrow control-pack updates
    -> different-fresh final review
    -> proof wall and closeout
```

## Task 1 — Freeze the slice-local decision method

Create `SPEC.md`, this plan, and `todo.md`. Confirm that the packet reserves all
approval to the user, distinguishes shipped kinds from default instances, and
does not authorize implementation.

**Verify:** inspect the scoped diff; confirm the HCM-0.6 row, `PG-DEFAULT-01`,
kind/instance separation, requiredness modes, and constitutional-root invariant
are represented.

## Task 2 — Assemble two independent research inputs

Dispatch fresh isolated read-only built-in agents for:

1. repository-truth inventory and contract constraints; and
2. external primary-source precedent research and failure-mode extraction.

The parent validates both results and retains source URLs and exact repository
references. Research agents may recommend, but cannot approve, a default.

**Verify:** replay each immutable dispatch manifest; record agent identity,
final status, validation, and parent disposition.

## Task 3 — Produce the decision-ready research subject

Write the research dossier and one shared-rubric minimal/standard/full
comparison. Keep sourced facts, local inferences, implementation precedent, and
the parent recommendation visibly separate. Frame the first user decision as
one focused question with a stated hypothesis rather than a batch survey.

**Verify:** source links resolve; every candidate lists shipped kinds, selected
default instances, requiredness, lifecycle/scope posture, benefits, costs, and
failure risks; no language implies approval.

## Task 4 — Review the pre-decision subject

Run targeted documentation checks, create an immutable final-subject review
dispatch, and obtain a fresh independent read-only review. Remediate valid
findings and use a different fresh reviewer after any remediation.

**Verify:** final pre-decision reviewer reports `CLEAN`; the exact reviewed
manifest is replayable.

## Task 5 — Stop for explicit user input when necessary

If no explicit decision exists, commit the reviewed decision-ready slice state,
write one parent-owned `human_input` handoff, rebuild/validate the ledger, and
commit the mechanical closeout separately. The resume boundary asks exactly one
brainstorming question and does not claim `PG-DEFAULT-01` complete.

**Verify:** both commits exist; handoff validation and all self-tests pass;
resume points back to HCM-0.6 only.

## Task 6 — Record and close the approved decision on resume

After the user explicitly decides, write the decision record, update only
affected canonical rows, run a different-fresh final slice review, complete the
proof wall, promote `PG-DEFAULT-01` only if earned, and use the completed
two-commit closeout protocol.

**Verify:** approved kind/default-instance/requiredness lists are exact and
internally valid; all final proof/review/handoff gates pass.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| Current four artifacts become defaults by inertia | Label implementation inventory as precedent/cost evidence only. |
| Shipped kinds and default instances collapse | Require separate lists and compare them separately in every candidate. |
| Too many always-required artifacts | Make empty-document pressure and onboarding burden explicit rubric axes. |
| Minimal set omits durable decision/risk truth | Compare coverage and custom-kind/conditional escape hatches explicitly. |
| Work-scoped and project-scoped artifacts conflate | Record scope/lifecycle for every candidate kind and selected instance. |
| Assistant recommendation is mistaken for approval | Require an explicit user decision record and keep the gate open until then. |
