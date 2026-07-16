# HCM-0.6 Shipped Default Artifact Set Research

## Status

Research complete. The later approved authority is
[`../decision/shipped-default-artifact-set-decision.md`](../decision/shipped-default-artifact-set-decision.md);
this dossier remains evidence and rationale rather than the selected default.

At research capture this was a decision-ready draft and `PG-DEFAULT-01` was
open. This document still supplies evidence and inference rather than approval;
the later decision and final review/proof own the selected values and gate
closure.

Research date: 2026-07-16.

## Method and evidence boundary

The research combined:

1. the frozen HCM-0.2 kind/instance/requiredness/constitutional-root contracts;
2. live current Handbook implementation and declarative-library inventory;
3. current primary sources from original projects, standards, authors, and
   official product documentation; and
4. explicit inference from those sources into candidate design principles.

Repository implementation is used only to show current support and adoption
cost. External precedents are used only to identify useful roles, lifecycle
patterns, and failure modes. Neither source class decides the product default.

## Frozen decision constraints

- A shipped **kind** is a reusable schema/behavior definition. A default
  **instance** selects one kind plus a concrete ID, label, canonical path,
  requiredness, dependencies, intake, and renderers.
- Shipping a kind does not require selecting an instance of it in the default
  profile.
- Every valid profile has exactly one `constitutional_root` instance, and that
  instance is `always` required.
- Every other instance is explicitly `always`, `conditional`, or `optional`;
  a conditional instance names one exact condition ref.
- Current enums, filenames, templates, and examples confer no authority.
- Work-scoped records must not silently become project-singleton artifacts.
- Phase 0 remains documentation/design-only.

## Current Handbook surface: evidence, not recommendation

| Current surface | Live fact | Decision implication (inference) |
|---|---|---|
| Charter | `CanonicalArtifactKind::Charter`; packet and baseline required; setup scaffolded; deterministic structured authoring exists | Strong implementation precedent for the constitutional-root role, but the future binding still depends on capability conformance rather than the word `Charter` |
| Project Context | fixed kind; baseline required and setup scaffolded; its starter copy calls itself optional; deterministic structured authoring exists | The live baseline/wording tension is direct evidence that future requiredness must be an explicit product decision, not copied from current flags or prose |
| Environment Inventory | fixed kind; baseline required and setup scaffolded; deterministic structured authoring exists | Environment truth has real support, but empty-document pressure is likely for non-deployable repositories; conditional selection deserves explicit comparison |
| Feature Spec | fixed kind; not baseline required and not setup scaffolded; current doctor guidance asks the user to fill the path rather than invoke a dedicated author flow | This behaves more like work-scoped, on-demand truth than a universal project singleton; the future kind may ship without a default project instance |
| Backlog, work catalog, release, sprint, tasks | active templates and pipeline stages exist outside the four canonical artifact kinds | Workflow state and canonical project truth are already distinct in practice; not every durable template should become a shipped artifact kind |
| Foundation strategy, technical architecture brief, test strategy, quality gates | active foundation-pack templates exist as a grouped stage output | These roles may be useful inputs/projections or profile-specific custom kinds; splitting every heading into a default kind would increase overlap and drift risk |
| Architecture decisions | the foundation strategy template explicitly calls for ADRs only for significant decisions | Decision history is an active role but is modular/multi-instance rather than one mandatory singleton document |

Two current splits are especially important cost evidence. Feature Spec is a
single canonical `.handbook/feature_spec/FEATURE_SPEC.md` to the engine but an
`artifacts/feature_spec/FEATURE_SPEC.md` pipeline capture/handoff subject; the
work-level stage itself produces one spec for one feature. Environment truth is
also present both as a canonical Environment Inventory family and as a
Foundation Pack output. These are concrete duplicate/stale-mirror risks, not a
reason to preserve either path as a future default.

Key live references:

- `crates/engine/src/canonical_artifacts.rs:12-30,100-152`
- `crates/engine/src/canonical_paths.rs:28-145`
- `crates/compiler/src/setup.rs:314-366,443-484`
- `crates/compiler/src/doctor.rs:155-163`
- `crates/compiler/src/doctor_shell.rs:3-24`
- `core/library/`
- `core/stages/`

## Primary-source precedent

### Constitutional authority and work-artifact sequencing

- GitHub Spec Kit separates a project constitution from the per-change
  `Spec -> Plan -> Tasks -> Implement` chain. Its constitution workflow also
  requires consistency propagation into dependent templates. This supports one
  durable constitutional root plus separately scoped work records, not one
  undifferentiated project document.
  Sources: [Spec Kit overview](https://github.github.com/spec-kit/index.html),
  [constitution workflow](https://github.com/github/spec-kit/blob/main/templates/commands/constitution.md?plain=1).
- Python PEP 13 keeps one current governance/process authority with explicit
  mandate, delegation, voting, conflict, removal, amendment, and history rules,
  while superseded governance documents remain historical. This reinforces the
  frozen exactly-one constitutional-root model and the need to keep frequently
  changing observations out of the constitutional artifact.
  Source: [Python PEP 13](https://peps.python.org/pep-0013/).
- Kubernetes Enhancement Proposals are required for most non-trivial changes
  while leaving very small changes outside the process. KEPs preserve common
  metadata, review/approval, status, and discoverable history. This supports a
  work-specification kind that is conditional/on-demand rather than an
  always-present project singleton.
  Source: [Kubernetes KEP process](https://github.com/kubernetes/enhancements/blob/master/keps/README.md).

### Project/architecture context should be progressive, not exhaustive

- arc42 covers goals, constraints, context, solution strategy, structure,
  runtime, deployment, decisions, quality, risks, and glossary, while describing
  the template as pragmatic and tailorable. This is evidence for a broad
  context schema/capability surface, not for twelve mandatory artifacts.
  Source: [arc42 template overview](https://arc42.org/overview).
- The C4 model explicitly says teams need only the diagram levels that add
  value and that system-context plus container diagrams are sufficient for most
  teams. This supports progressive/conditional detail inside project context
  rather than defaulting every architectural view.
  Source: [C4 diagrams](https://c4model.com/diagrams).

### Decisions benefit from small append-only records

- Michael Nygard's original ADR proposal argues that large documents are rarely
  kept current, and uses short records with context, decision, status, and
  consequences. Superseded decisions remain as history. This supports a shipped
  multi-instance decision-record kind and argues against a mandatory monolithic
  decision document.
  Source: [Documenting Architecture Decisions](https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions).
- arc42 likewise advises recording important/expensive/risky decisions while
  avoiding redundant text and choosing central versus local placement with
  judgment.
  Source: [arc42 architecture decisions](https://docs.arc42.org/section-9/).

### Inventories need typed identity and explicit authority boundaries

- Backstage stores typed entity metadata in source-controlled YAML and supports
  multiple entity kinds and multiple files. It uses ownership and relationships
  for discovery across diverse software. Its graph guidance says the catalog is
  meant to capture human mental models rather than exhaustively inventory every
  possible thing, and should not replace upstream sources of truth. This
  supports typed configurable kinds and a conditional topology/inventory role,
  while warning against an exhaustive default catalog or derived-view authority.
  Sources: [Backstage Software Catalog](https://backstage.io/docs/features/software-catalog/),
  [catalog graph guidance](https://backstage.io/docs/features/software-catalog/creating-the-catalog-graph/).
- The Twelve-Factor App separates deploy-varying configuration such as service
  handles, credentials, and per-deploy values from code, and warns that checked
  in config files can leak secrets or scatter environment truth. A Handbook
  environment artifact should therefore describe variables, owners,
  provenance, and authoritative locations without copying secret/live values.
  Source: [The Twelve-Factor App: Config](https://www.12factor.net/config).

### Operational records are valuable but high-drift and applicability-specific

- Google SRE notes that playbook details drift at the same rate as production,
  recommends a minimal structured content decision, and suggests automating
  deterministic repeated command sequences. This supports an operational
  runbook kind for operated systems, not a universal always-required instance.
  Source: [Google SRE Workbook: On-Call](https://sre.google/workbook/on-call/).
- AWS Well-Architected describes runbooks as outcome-focused procedures with
  tools, permissions, error handling, exceptions, escalation, ownership,
  validation by another operator, and continuous updates; it also recommends
  increasing automation as the library grows. This supports conditional
  runbook instances and explicit lifecycle/review triggers.
  Source: [AWS OPS07-BP03](https://docs.aws.amazon.com/wellarchitected/latest/framework/ops_ready_to_support_use_runbooks.html).

### Risk and quality posture should not collapse into a universal score/file

- OpenSSF Scorecard describes its checks as opinionated heuristics, explicitly
  rejects one-size-fits-all interpretation, and warns that aggregate scores can
  hide which behaviors are actually present. This supports typed evidence and
  applicability-driven risk records rather than a universal required risk
  scorecard artifact.
  Source: [OpenSSF Scorecard](https://github.com/ossf/scorecard).
- NIST SP 800-30 treats risk assessment as prepare, conduct, communicate, and
  maintain work with explicit scope, assumptions, uncertainty, rationale,
  evidence, validity horizon, and reassessment triggers; it also allows
  organizations to choose rigor and frequency. This supports a
  posture-conditional risk record rather than false universal precision.
  Sources: [NIST SP 800-30 Rev. 1](https://csrc.nist.gov/pubs/sp/800/30/r1/final),
  [official PDF](https://nvlpubs.nist.gov/nistpubs/Legacy/SP/nistspecialpublication800-30r1.pdf).
- Google SRE uses event triggers for reviewed postmortems and requires tracked
  preventive actions; its SLO guidance ties measures to authors, reviewers,
  business approvers, review dates, rationale, and enforcement consequences.
  These are event/service-scoped lifecycle patterns, not evidence for empty
  universal project files.
  Sources: [Postmortem Culture](https://sre.google/sre-book/postmortem-culture/),
  [Implementing SLOs](https://sre.google/workbook/implementing-slos/).
- Diataxis separates documentation by user need and notes that reference
  material should follow the product it describes and link rather than duplicate
  other documentation modes. This supports distinct roles with explicit
  dependencies and projections, not overlapping default documents.
  Source: [Diataxis reference guidance](https://diataxis.fr/reference/).
- OWASP ASVS selects among three assurance levels according to application risk
  and requires versioned requirement references. This is further evidence that
  a shared rubric and exact versioned refs should drive minimal/standard/full
  rigor instead of making the maximum tier universal.
  Sources: [OWASP ASVS](https://owasp.org/www-project-application-security-verification-standard/),
  [OWASP Developer Guide](https://devguide.owasp.org/en/03-requirements/05-asvs/).

## Cross-source synthesis

The following are inferences from the combined evidence, not sourced facts:

1. **Use a small always-required project core.** The constitutional root is
   mandatory by contract. A compact project-context instance is the strongest
   additional universal candidate because every repository has purpose,
   boundaries, ownership, and current-state facts.
2. **Ship more reusable kinds than the default profile selects.** Decision,
   work-specification, runbook, and risk roles recur across credible systems,
   but their instances have different cardinality and applicability.
3. **Keep project scope and work scope separate.** Work specifications,
   delivery plans, and tasks should be created for qualifying work rather than
   represented as one permanent project singleton.
4. **Prefer conditional or on-demand operational detail.** Environment,
   deployment, risk, and runbook truth becomes valuable when the repository has
   the corresponding runtime or governance surface; requiring empty placeholders
   creates predictable drift.
5. **Do not multiply authority to improve discoverability.** Catalogs,
   projections, indexes, and rendered views should point to canonical sources.
6. **Lifecycle rules matter as much as initial content.** Small modular records
   only reduce drift when owners, review triggers, supersession, and validation
   are explicit.
7. **Avoid a generic quality-strategy default.** Frozen contract/evidence/gate
   semantics and work-spec proof gates already own much of that behavior; a
   separate mandatory quality document would risk duplicate authority.

## Candidate building blocks

| Candidate kind role | Natural cardinality/scope | Default-instance question |
|---|---|---|
| Project Charter | one project singleton; normative constitutional authority | always selected and always required; exact label/path still needs approval |
| Project Context | one project singleton; observational/rationale current truth | always versus optional is a core product decision |
| Environment Inventory | one project singleton or bounded environment family; observational current truth | conditional for repositories with managed runtime/external-service state versus merely optional |
| Work Specification | many work-scoped instances over project lifetime | ship the kind, but create instances on qualifying work rather than select one project singleton |
| Decision Record | many append-only project/work-scoped instances | ship the kind, but create on significant decisions; no empty default record |
| Risk Register | one project singleton or scoped registers | optional/conditional default instance versus repository-selected extension |
| Operational Runbook | many service/procedure-scoped instances | ship in a full catalog or leave profile/custom; never universal always-required |
| Quality/Verification Strategy | project or work scoped | likely compose Charter, work spec, and contract/gate truth instead of adding a default kind |
| Software/Service Catalog | one or many topology records | likely profile-specific/custom for complex estates; Project Context can cover the universal minimum |

## Failure modes the decision must prevent

- copying the current four-variant enum into the shipped catalog;
- requiring a placeholder file only because setup currently scaffolds it;
- one `Feature Spec` singleton standing in for concurrent or historical work;
- one large project document combining policy, observations, decisions,
  procedures, and work state;
- retaining a completed work specification as current after implementation has
  diverged, or losing its historical status/supersession chain;
- separate artifacts repeating the same owner, constraint, architecture, risk,
  or environment fact without source refs;
- always-required environment/runbook/risk files in libraries or non-deployable
  repositories;
- an exhaustive catalog whose derived view is mistaken for canonical truth;
- a decision log that loses superseded rationale;
- operational instructions that drift instead of being tested or automated;
- environment prose that copies secrets or volatile live values instead of
  referencing their authoritative systems;
- aggregate quality/risk scores that hide applicable missing evidence; and
- treating assistant recommendation, subagent consensus, or user silence as
  approval.

## Remaining human authority

The research narrows but cannot answer these product questions:

1. Should the shipped v1 catalog optimize for a very small opinionated product
   or ship a broader first-party kind library with a lean selected default?
2. Is compact Project Context universal enough to be `always` required?
3. Should Environment Inventory be a selected conditional default instance or
   merely a shipped kind available to repository profiles?
4. Should Risk Register be first-party in v1, and if so, selected optional,
   selected conditional, or unselected by default?
5. Should Operational Runbook be first-party in v1 or deferred to a later
   profile/custom-kind proof?
6. Which exact stable IDs, labels, paths, condition refs, lifecycle policies,
   intake definitions, and fixed renderers should the approved instances use?

The first decision should choose the product/default posture. Later questions
can then refine exact kinds and instances without presenting a large batch
survey.
