# HCM-0.6 Candidate Comparison

## Status

Historical decision-ready comparison and rationale, not current authority.
Names and paths below remain the candidate values presented for discussion.
The later explicit user decision record is the only authority that selected or
changed them.

## Decision outcome

The user selected the Standard-catalog/lean-default product posture, then
resolved each subordinate kind, role, instance, requiredness, condition,
lifecycle, and support question independently. The authoritative result is
[`../decision/shipped-default-artifact-set-decision.md`](../decision/shipped-default-artifact-set-decision.md).
It closes the catalog at six kinds but selects only three root instances and
does not retain every candidate name, risk instance, or condition value below.

## Comparison convention

- **Shipped kind:** a first-party reusable schema/behavior definition available
  to profiles.
- **Selected default instance:** a concrete instance descriptor included in the
  shipped root profile.
- **Unselected shipped kind:** available for repository/profile/work layers but
  absent from the root profile's instance registry.
- **On-demand multi-instance kind:** a kind intended for new exact descriptors
  as decisions, work, or procedures arise; never one permanent project
  singleton by implication.

The candidate paths intentionally use canonical YAML and avoid current Markdown
filenames. They are discussion values, not migration decisions.

## Candidate A — Minimal product

### Shipped kinds

1. `handbook.kind.project-charter@1.0.0`
2. `handbook.kind.project-context@1.0.0`
3. `handbook.kind.work-specification@1.0.0`
4. `handbook.kind.decision-record@1.0.0`

### Selected default instances

| Instance | Path | Requiredness | Role |
|---|---|---|---|
| `project_charter` | `.handbook/project/charter.yaml` | `always` | exactly one `constitutional_root` plus `constitutional_authority` role |
| `project_context` | `.handbook/project/context.yaml` | `always` | compact purpose, boundaries, owner, current architecture/topology, and constraints |

`work-specification` and `decision-record` ship as on-demand multi-instance
kinds but have no root-profile singleton instance.

### Lifecycle, scope, rendering, and transport posture

| Kind/instance family | Cardinality and scope | Lifecycle and review trigger | Renderer / Projection posture |
|---|---|---|---|
| `project_charter` | one project singleton | long-lived constitutional authority; review on amendment, exception-policy, delegation, or governance change | fixed deterministic human renderer before Phase 3; later Projection may add audience views but cannot replace the canonical source |
| `project_context` | one project singleton | long-lived factual context; review when boundaries, owners, topology, or material constraints change | fixed deterministic human renderer before Phase 3; later architecture/context Projections may be supplemental |
| `work-specification` | zero or more work-scoped instances | proposed/active/completed or superseded per change; review at scope, graduation, rollback, or terminal-status change | fixed renderer whenever instantiated; later work/status Projections are supplemental |
| `decision-record` | zero or more project- or work-scoped instances | proposed/accepted/deprecated/superseded with preserved history; review at acceptance or supersession | fixed renderer whenever instantiated; later decision indexes are Projections rather than new authority |

All four kinds use the frozen owner split: engine/domain types own semantics, the
SDK carries typed descriptors and records, and transports carry bytes without
interpreting requiredness, lifecycle, or authority.

### Benefits

- smallest setup and maintenance burden;
- clear normative versus observational project authority;
- preserves first-party work and decision semantics without empty project files;
- easiest Phase 1/2 implementation and adoption path.

### Costs and risks

- environment, risk, and operations schemas begin as profile/custom kinds;
- teams may invent incompatible versions before first-party kinds arrive;
- default projections have less structured operational/risk context;
- pushes more work into repository-profile authoring.

## Candidate B — Standard catalog, lean selected default

### Shipped kinds

1. `handbook.kind.project-charter@1.0.0`
2. `handbook.kind.project-context@1.0.0`
3. `handbook.kind.environment-inventory@1.0.0`
4. `handbook.kind.work-specification@1.0.0`
5. `handbook.kind.decision-record@1.0.0`
6. `handbook.kind.risk-register@1.0.0`

### Selected default instances

| Instance | Path | Requiredness | Candidate condition | Role |
|---|---|---|---|---|
| `project_charter` | `.handbook/project/charter.yaml` | `always` | none | exactly one `constitutional_root` plus `constitutional_authority` role |
| `project_context` | `.handbook/project/context.yaml` | `always` | none | compact purpose, boundaries, owner, current architecture/topology, and constraints |
| `environment_inventory` | `.handbook/project/environment.yaml` | `conditional` | `handbook.condition.project.managed-runtime-present@1.0.0` | runtime assumptions, environment families, external services, deployment/runtime dependencies, and safe config metadata |
| `risk_register` | `.handbook/project/risks.yaml` | `optional` | none | explicit current risks, owners, treatments, evidence, acceptance, and review triggers |

`work-specification` and `decision-record` ship as on-demand multi-instance
kinds but have no root-profile singleton instance.

### Lifecycle, scope, rendering, and transport posture

| Kind/instance family | Cardinality and scope | Lifecycle and review trigger | Renderer / Projection posture |
|---|---|---|---|
| `project_charter` | one project singleton | long-lived constitutional authority; review on amendment, exception-policy, delegation, or governance change | fixed deterministic human renderer before Phase 3; later Projection may add audience views but cannot replace canonical source |
| `project_context` | one project singleton | long-lived factual context; review when boundaries, owners, topology, or material constraints change | fixed deterministic human renderer before Phase 3; later architecture/context Projections may be supplemental |
| `environment_inventory` | zero or one selected project instance, conditional on managed-runtime applicability | active while the condition holds; review when runtime/deployment dependencies or the applicability result changes | fixed deterministic renderer before Phase 3 when selected; later deployment/operations Projections are supplemental and must not copy live secrets |
| `risk_register` | zero or one optional selected project instance | maintained while selected; review on material risk, evidence, acceptance, ownership, or validity-horizon change | fixed deterministic renderer before Phase 3 when selected; later risk views are Projections |
| `work-specification` | zero or more work-scoped instances | proposed/active/completed or superseded per change; review at scope, graduation, rollback, or terminal-status change | fixed renderer whenever instantiated; later work/status Projections are supplemental |
| `decision-record` | zero or more project- or work-scoped instances | proposed/accepted/deprecated/superseded with preserved history; review at acceptance or supersession | fixed renderer whenever instantiated; later decision indexes are Projections rather than new authority |

Every kind keeps the frozen owner split: engine/domain types own semantics, the
SDK carries typed descriptors and records, and transports carry bytes without
interpreting conditions, requiredness, lifecycle, or authority.

### Benefits

- broad enough to cover the recurring project, work, decision, environment, and
  risk roles found in the research;
- still requires only two universal documents;
- avoids a singleton active feature/decision record;
- gives repository layers first-party schemas for common high-value needs;
- keeps operational runbooks out of repos that do not operate a service.

### Costs and risks

- larger first-party schema/intake/renderer implementation surface than Minimal;
- exact environment condition semantics must be frozen and proven;
- an optional selected risk path may still create discoverability or empty-file
  expectations unless setup/doctor clearly distinguish optional from missing;
- Project Context and Environment Inventory need explicit non-overlap rules.

## Candidate C — Full governance/operations catalog

### Shipped kinds

Everything in Standard, plus:

7. `handbook.kind.operational-runbook@1.0.0`
8. `handbook.kind.quality-strategy@1.0.0`
9. `handbook.kind.software-catalog@1.0.0`

### Selected default instances

| Instance | Path | Requiredness | Candidate condition | Role |
|---|---|---|---|---|
| `project_charter` | `.handbook/project/charter.yaml` | `always` | none | exactly one `constitutional_root` plus `constitutional_authority` role |
| `project_context` | `.handbook/project/context.yaml` | `always` | none | project/system context |
| `environment_inventory` | `.handbook/project/environment.yaml` | `always` | none | environment and deployment truth |
| `risk_register` | `.handbook/project/risks.yaml` | `always` | none | risk posture and treatment |
| `quality_strategy` | `.handbook/project/quality.yaml` | `always` | none | project-wide verification strategy |
| `software_catalog` | `.handbook/project/catalog.yaml` | `conditional` | `handbook.condition.project.multi-component@1.0.0` | owned component/resource/API topology |

`work-specification`, `decision-record`, and `operational-runbook` are shipped
on-demand multi-instance kinds; runbook instances are created only for operated
procedures/services.

### Lifecycle, scope, rendering, and transport posture

| Kind/instance family | Cardinality and scope | Lifecycle and review trigger | Renderer / Projection posture |
|---|---|---|---|
| `project_charter` | one project singleton | long-lived constitutional authority; review on amendment, exception-policy, delegation, or governance change | fixed deterministic human renderer before Phase 3; later Projection may add audience views but cannot replace canonical source |
| `project_context` | one project singleton | long-lived factual context; review when boundaries, owners, topology, or material constraints change | fixed deterministic human renderer before Phase 3; later architecture/context Projections may be supplemental |
| `environment_inventory` | one always-selected project singleton | maintained continuously; review on runtime/deployment dependency change | fixed deterministic renderer before Phase 3; later deployment/operations Projections are supplemental and must not copy live secrets |
| `risk_register` | one always-selected project singleton | maintained continuously; review on material risk, evidence, acceptance, ownership, or validity-horizon change | fixed deterministic renderer before Phase 3; later risk views are Projections |
| `quality_strategy` | one always-selected project singleton | maintained continuously; review on verification-policy, quality-target, or enforcement change | fixed deterministic renderer before Phase 3; later quality views are Projections |
| `software_catalog` | zero or one selected project instance, conditional on multi-component applicability | active while the condition holds; review on owned component, API/resource topology, or applicability change | fixed deterministic renderer before Phase 3 when selected; later topology views are Projections and must not duplicate a live external catalog |
| `work-specification` / `decision-record` | zero or more work/project instances as described in Minimal and Standard | event/status driven with preserved history | fixed renderer whenever instantiated; later indexes and status views are supplemental Projections |
| `operational-runbook` | zero or more service/procedure-scoped instances | active/deprecated/superseded; review after rehearsal, incident, automation, dependency, or ownership change | fixed renderer whenever instantiated; later service/on-call views are supplemental Projections |

Every kind keeps the frozen owner split: engine/domain types own semantics, the
SDK carries typed descriptors and records, and transports carry bytes without
interpreting conditions, requiredness, lifecycle, or authority.

### Benefits

- richest structured context and future Projection surface;
- first-party coverage for complex platforms and regulated/operated systems;
- less need for early custom kinds.

### Costs and risks

- substantial onboarding, schema, intake, renderer, and lifecycle burden;
- high empty-document and drift pressure for small libraries/tools;
- `quality_strategy` risks duplicating Charter, work-spec proof gates, and the
  contract/evidence/gate system;
- `software_catalog` risks duplicating live topology or external catalogs;
- requiring environment/risk/quality universally conflicts with the
  applicability lessons in the research.

## Shared-rubric comparison

Scores are parent inference on a 1 (weak) to 5 (strong) scale. They are intended
to expose tradeoffs, not manufacture objective authority.

| Rubric axis | Minimal | Standard | Full |
|---|---:|---:|---:|
| Clear authority and exactly one constitutional root | 5 | 5 | 5 |
| Low default onboarding burden | 5 | 4 | 2 |
| Stable project context versus per-change/work separation | 5 | 5 | 4 |
| Low overlap/drift risk | 5 | 4 | 2 |
| Conditional applicability with low empty-document pressure | 5 | 4 | 1 |
| Deterministic lifecycle and review-trigger clarity | 5 | 4 | 3 |
| Repository diversity with low custom-kind reliance | 2 | 4 | 5 |
| Bounded fixed-renderer needs before Phase 3 and useful later Projection surface | 5 | 4 | 2 |
| Compatibility with frozen SDK ownership and semantics-free transport | 5 | 5 | 5 |
| Low implementation and adoption cost | 5 | 3 | 1 |
| Operations/risk readiness | 2 | 4 | 5 |
| Evidence-weighted overall fit | 4 | 5 | 2 |

## Parent recommendation — not approval

Recommend **Candidate B: Standard catalog, lean selected default**, with two
possible refinements for the user session:

1. decide whether `risk_register` should be a selected `optional` instance or an
   unselected shipped kind; and
2. explicitly reject `quality_strategy` and `software_catalog` as v1 shipped
   defaults unless a concrete first consumer needs them.

Why: the primary sources repeatedly favor progressive detail, conditional work
or operational artifacts, small modular decision records, typed source
boundaries, and avoiding one-size-fits-all checklists. Standard supplies the
recurring first-party schemas without forcing every schema into every
repository. It also makes the critical architectural distinction explicit:
**the shipped kind catalog can be broader than the selected default instance
registry**.

## First brainstorming decision

Before debating individual names or paths, choose the product posture:

- **Lean product:** Candidate A; optimize for the smallest universal system and
  let repository profiles/custom kinds absorb more variation.
- **Broad catalog, lean default:** Candidate B; ship first-party schemas for the
  most common roles but select only a small project core by default.
- **Governance-heavy product:** Candidate C; optimize for structured coverage at
  the cost of more default burden and overlap risk.

The parent hypothesis is **Broad catalog, lean default**. The exact user answer
must be recorded before the candidate is refined or approved.
