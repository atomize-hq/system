# HCM-0.9 Specification: Contract-Catalog Topology and Selective Routing

**Status:** planning authority; execution not started  
**Phase / slice:** `HCM-0` / `HCM-0.9`  
**Sequence:** immediately after completed `HCM-0.4`; before `HCM-0.5`  
**Change class:** documentation/control maintenance only  
**Execution packet:** [`tasks/plan.md`](tasks/plan.md) and [`tasks/todo.md`](tasks/todo.md)

## Objective

Decompose the frozen `05-contracts-schemas-and-gates.md` contract catalog into focused canonical leaf documents without changing any contract meaning. Retain `05-contracts-schemas-and-gates.md` as the stable discovery, routing, and historical-anchor compatibility index. Bind current orchestration, dispatch, proof, and handoff flows to exact leaf documents so future sessions load only the contract authority they need.

The intended outcome is lower context load and clearer canonical ownership, not a contract revision. HCM-0.9 must complete before HCM-0.5 adds contract-membrane or dock semantics.

## Frozen baseline

| Property | Frozen value |
|---|---|
| Baseline commit | `214a5b8eb182fce74478df49d4f55d226d65fdf5` |
| Baseline path | `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` |
| Full-file SHA-256 | `c7f61db209a81ba20690f365b4069dd01f11e395335bfa10d2ce21143cc2985d` |
| Baseline size | 3,757 lines |
| Baseline headings | one H1 plus 48 H2 and 22 H3 headings |
| Frozen authority | completed HCM-0.2, HCM-0.3, and HCM-0.4 contract semantics |
| Completion evidence | HCM-0.4 planning commit `214a5b8...` and closeout commit `3d993ab...` |

Execution must retrieve the baseline from Git, not from a working-tree copy or a historical handoff attachment:

```bash
git show 214a5b8eb182fce74478df49d4f55d226d65fdf5:docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md \
  | shasum -a 256
```

The command must return the frozen SHA-256 above before any decomposition edit begins. A mismatch is an immediate stop.

## Scope

HCM-0.9 execution will:

1. create the eleven canonical leaf documents in the exact topology below;
2. move each frozen H2 section and all of its subordinate content to exactly one leaf;
3. replace the current `05` body with a stable routing index and compatibility aliases;
4. add a slice-local mechanical parity/routing verifier;
5. update mutable control-pack and orchestration references to select the minimum exact leaf set;
6. prove links, anchors, fences, examples, manifests, handoffs, parity, immutability, and scope;
7. obtain fresh independent complete-subject review within the execution budget;
8. commit reviewed execution separately from its mechanical handoff/ledger closeout.

## Invariants

1. **Zero semantic delta:** every byte after the frozen baseline H1 plus its following blank line appears once, in the same order, in the canonical leaf payload sequence after verifier-approved leaf routing scaffolds are removed.
2. **One canonical owner:** each frozen H2 section belongs to exactly one leaf. The stable index and other control-pack files may route or summarize, but may not restate normative contract content.
3. **Contiguous ownership:** each leaf owns one contiguous span of frozen H2 sections. Leaves are ordered so concatenating their payloads reconstructs the baseline body.
4. **Heading preservation:** every frozen H2/H3 heading text is copied byte-for-byte into its owning leaf and retains its generated anchor.
5. **Stable path:** `05-contracts-schemas-and-gates.md` remains present with the unchanged H1 `Contracts, Schemas, and Gates`.
6. **Historical anchor preservation:** every frozen H2/H3 anchor remains resolvable at the stable `05` path through a forwarding alias that points to the same heading in its owning leaf.
7. **Selective authority:** new mutable orchestration, dispatch, proof, and handoff references name exact leaf paths and anchors rather than loading the whole catalog by default.
8. **Immutable history:** no pre-HCM-0.9 file under `handoffs/records/` or `handoffs/dispatches/` is edited, renamed, deleted, or re-fingerprinted.
9. **Historical truth remains historical:** old line refs and monolith manifests continue to describe their committed state. The stable index and Git history preserve navigability; execution does not rewrite those records.
10. **No semantic correction:** unclear, inconsistent, or improvable contract text is moved unchanged and recorded as a future finding. It is never corrected inside HCM-0.9.
11. **No proof promotion:** `PG-CATALOG-01` may close for topology/parity only. No runtime or public-interface gate changes state.
12. **No next-slice activation:** HCM-0.5 remains unstarted in this session and in HCM-0.9 execution closeout.
13. **Referential context preservation:** every frozen spatial/positional reference and every named contract dependency whose target moves to another leaf is mechanically classified. Each cross-leaf dependency has an exact verifier-approved routing target; byte parity without referential parity fails.

## Non-goals

- Rust, Cargo, runtime, CLI, Tauri, Substrate, SDK, or public API changes;
- new contract fields, defaults, examples, validation rules, schema versions, operation IDs, DTOs, lifecycle states, or dock semantics;
- correction, clarification, simplification, reformatting, or reordering of frozen semantic payload;
- HCM-0.5 decomposition, specification, planning, or execution;
- proof promotion other than the HCM-0.9 structural topology gate;
- rewriting immutable historical handoffs or dispatches;
- changing handoff or dispatch schemas merely to support leaf refs;
- selecting shipped artifact defaults or revisiting HCM-0.2/HCM-0.3/HCM-0.4 decisions;
- opportunistic cleanup outside the exact mutable surface inventory.

## Canonical leaf topology

All paths are relative to `docs/specs/handbook-contract-membrane/`.

| Order | Canonical leaf | Frozen section span | Purpose |
|---:|---|---|---|
| 1 | `contracts/01-schema-profile-and-artifact-registry.md` | `Status` through `Artifact instance descriptor contract` | shared schema policy, profile, registry, kind, capability, and instance authority |
| 2 | `contracts/02-intake-charter-and-validation.md` | `Artifact intake definition contract` through `Artifact validation layers` | intake, candidate, Charter, promotion, and validation authority |
| 3 | `contracts/03-vocabulary-and-context-resolution.md` | `Vocabulary contract` through `Resolution escalation and memory promotion` | vocabulary, Resolution stack/envelope, escalation, and memory-promotion authority |
| 4 | `contracts/04-projection-contracts.md` | `Projection disclosure policy` through `Projection result` | disclosure, support, definition, request, and result authority |
| 5 | `contracts/05-snapshot-memory-contracts.md` | `Snapshot capture policy` through `Snapshot redaction and retention` | snapshot capture, consistency, delta, grounding, redaction, and retention authority |
| 6 | `contracts/06-posture-and-synthesis-contracts.md` | `Project posture kernel and recommendation contracts` through `Optional synthesis-candidate contract` | posture resolution/recommendation/transition and candidate-only synthesis authority |
| 7 | `contracts/07-development-orchestration-contracts.md` | `Development orchestration contracts` only | internal dispatch, delegated evidence, and review choreography authority |
| 8 | `contracts/08-sdk-operations-and-capability-discovery.md` | `HCM-0.4 crate ownership and dependency contract` through `Capability bootstrap and catalog discovery` | crate ownership, SDK use cases, operations, bootstrap, and discovery authority |
| 9 | `contracts/09-machine-transport-and-adapter-contracts.md` | `Transport request contract` through `Tauri adapter contract` | requests, responses, DTO/schema generation, CLI JSON, and Tauri transport authority |
| 10 | `contracts/10-substrate-integration-and-publication.md` | `Substrate integration contracts` through `Published Rust API proof plan` | transitional/permanent Substrate boundaries and publication proof authority |
| 11 | `contracts/11-contract-evidence-gates-and-docks.md` | `Contract record` through `Schema compatibility posture` | contract/evidence/verdict/gate/dock authority plus public proof gates and compatibility posture |

Each leaf begins with `# <leaf title>` and one blank line. All eleven leaves
then contain one verifier-approved trigger-indexed routing block bounded by
`<!-- hcm-0.9-routing-only:start -->` and
`<!-- hcm-0.9-routing-only:end -->`, followed by one blank line. Its heading is
exactly `Frozen cross-leaf authority dependency routes:`. It contains a Markdown
table with the exact columns `Route`, `Exact trigger`, and `Required authority`,
and one row for every source-leaf `R01`-`R98` route in global route order.
Targets remain grouped by route; they are not flattened or de-duplicated across
triggers. The block is non-normative routing metadata, not contract content. The
semantic payload begins at the first H2 and remains byte-for-byte frozen. The
verifier validates and strips the complete scaffold before parity comparison.

Continuation Review 1 demonstrated that the prior flat target-union topology
would load seven of eleven leaves for `Instance profile contract`, `Resolved
instance profile`, and `SDK ordinary-use-case contract`. That topology is
rejected. The replacement topology activates only an exact route trigger or
ordinary-operation fixture from `evidence/dependency-audit.md`; a named H2/H3
is a source locator and never activates all routes beneath it. Each exact route
targets at most three sibling leaves. A multi-trigger task derives its complete
source-plus-target union before loading authority. A union of six or more leaves
is classified `broad-only`, cannot proceed as normal selective routing, and
requires explicit multi-owner/complete-catalog assembly or a future reviewed
partition redesign. Loading a target anchor never activates unrelated routes in
that target leaf. Adding, removing, overlapping, or reordering a trigger or
target requires a reviewed packet change before execution continues.

### Exact cross-leaf dependency table

All paths below are relative to `contracts/`. Route IDs, frozen line evidence,
50 ordinary-operation fixtures, 11 explicit named non-route groups, and the
70-heading aggregate-fanout audit are defined by
`evidence/dependency-audit.md`. Route target order is normative; repeated
targets in different trigger rows remain separate routing evidence.

| Route | Source leaf | Triggering frozen source | Required exact targets |
|---|---|---|---|
| `R01` | `01` | instance-profile `artifact_instances[].intake_definition_ref` materialization | `02-intake-charter-and-validation.md#artifact-intake-definition-contract` |
| `R02` | `01` | instance-profile `vocabulary_ref` and resolved vocabulary identity | `03-vocabulary-and-context-resolution.md#vocabulary-contract` |
| `R03` | `01` | instance-profile `context_resolution_ref` and resolved stack identity | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition` |
| `R04` | `01` | instance-profile Projection catalog, including the named snapshot-grounding definition | `04-projection-contracts.md#projection-definition`<br>`05-snapshot-memory-contracts.md#snapshot-grounding-projection-definition` |
| `R05` | `01` | instance-profile posture-evaluation-policy pair | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R06` | `01` | instance-profile `dock_requirement_refs` and later dock compatibility | `11-contract-evidence-gates-and-docks.md#dock-capability-manifest` |
| `R07` | `01` | artifact-kind `projection_definition_refs` and fixed-renderer/Projection distinction | `04-projection-contracts.md#projection-definition` |
| `R08` | `01` | artifact-instance `intake_definition_ref` and exact target-kind validation | `02-intake-charter-and-validation.md#artifact-intake-definition-contract` |
| `R09` | `01` | artifact-instance `projection_definition_refs` selection | `04-projection-contracts.md#projection-definition` |
| `R10` | `02` | intake schema identity, target `artifact_kind_ref`, and candidate-schema closure | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract` |
| `R11` | `02` | intake-definition coverage evidence, confidence, freshness, waiver, and typed evaluation | `11-contract-evidence-gates-and-docks.md#evidence-record` |
| `R12` | `02` | intake-record `coverage_results` typed evidence/confidence/freshness values | `11-contract-evidence-gates-and-docks.md#evidence-record` |
| `R13` | `02` | intake-record constitutional-root instance/capability selection | `01-schema-profile-and-artifact-registry.md#semantic-capability-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R14` | `02` | Charter constitutional-root instance/capability selection | `01-schema-profile-and-artifact-registry.md#semantic-capability-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R15` | `02` | Charter fixed-renderer exclusion from Resolution-aware Projection authority | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result` |
| `R16` | `02` | artifact-validation fixed-renderer exclusion from Resolution-aware Projection authority | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result` |
| `R17` | `02` | artifact-validation external docks, normalized evidence, and required evidence gates | `11-contract-evidence-gates-and-docks.md#dock-capability-manifest`<br>`11-contract-evidence-gates-and-docks.md#dock-requestresult`<br>`11-contract-evidence-gates-and-docks.md#gate-contract` |
| `R18` | `03` | vocabulary stable-role registry pair and role/absorption closure | `01-schema-profile-and-artifact-registry.md#stable-role-registry-contract` |
| `R19` | `03` | vocabulary schema identity and resolved-profile compatibility | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R20` | `03` | Resolution-stack schema identity and selected-profile compatibility | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R21` | `03` | Resolution-envelope schema identity and selected-profile compatibility | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R22` | `03` | escalation/memory-promotion Projection, Snapshot, and delta source inputs | `04-projection-contracts.md#projection-result`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta` |
| `R23` | `04` | Projection-disclosure mapping of immutable upstream redaction dispositions | `05-snapshot-memory-contracts.md#snapshot-redaction-and-retention` |
| `R24` | `04` | Projection-disclosure definition/profile/source/kind/capability/schema validation | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#semantic-capability-contract` |
| `R25` | `04` | Projection-disclosure profile-stack and Resolution compatibility | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R26` | `04` | Projection-support evaluator source/kind/capability/schema metadata | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#semantic-capability-contract` |
| `R27` | `04` | Projection-definition profile/source/kind/capability/schema validation | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#semantic-capability-contract` |
| `R28` | `04` | Projection-request profile, vocabulary, stack, and Resolution-envelope pairs | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#vocabulary-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R29` | `04` | Projection-result profile, vocabulary, stack, and Resolution-envelope pairs | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#vocabulary-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R30` | `04` | Projection-definition `claim_refs` decision-completeness effects | `11-contract-evidence-gates-and-docks.md#contract-record` |
| `R31` | `04` | Projection-result omission `claim_refs` and proof effects | `11-contract-evidence-gates-and-docks.md#contract-record` |
| `R32` | `05` | capture-policy memory horizons and capture-envelope membership | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R33` | `05` | Context Memory Snapshot resolved-profile and Resolution-envelope pairs | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R34` | `05` | snapshot-grounding Projection definition, minimum Resolution, disclosure, support, and currentness | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`04-projection-contracts.md#projection-disclosure-policy`<br>`04-projection-contracts.md#projection-support-evaluator-definition`<br>`04-projection-contracts.md#projection-definition` |
| `R35` | `05` | snapshot Projection DTOs extend generic request/result and retain profile/vocabulary/envelope semantics | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#vocabulary-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result` |
| `R36` | `05` | snapshot redaction original/retained mapping into generic Projection disclosure | `04-projection-contracts.md#projection-disclosure-policy` |
| `R37` | `05` | retention holds from handoff, evidence, verdict, gate, and active-promotion refs | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion`<br>`07-development-orchestration-contracts.md#development-orchestration-contracts`<br>`11-contract-evidence-gates-and-docks.md#evidence-record`<br>`11-contract-evidence-gates-and-docks.md#verdict-contract`<br>`11-contract-evidence-gates-and-docks.md#gate-contract` |
| `R38` | `06` | posture-kernel constitutional artifact and resolved-profile inputs | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract`<br>`02-intake-charter-and-validation.md#charter-intake-and-canonical-contract` |
| `R39` | `06` | posture contract/evidence/snapshot inputs, freshness, and non-causal snapshot drift | `05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta`<br>`11-contract-evidence-gates-and-docks.md#contract-record`<br>`11-contract-evidence-gates-and-docks.md#evidence-record` |
| `R40` | `06` | posture-transition intake reassessment and canonical Charter compare-and-write | `02-intake-charter-and-validation.md#artifact-intake-definition-contract`<br>`02-intake-charter-and-validation.md#charter-intake-and-canonical-contract` |
| `R41` | `07` | dispatch execution-envelope active Resolution field | `03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R42` | `08` | ordinary operation `profile.list` exact semantic owner | `01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R43` | `08` | ordinary operation `profile.resolve` exact semantic owner | `01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R44` | `08` | ordinary operation `schema.list` exact semantic owner | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract` |
| `R45` | `08` | ordinary operation `schema.read` exact semantic owner | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract` |
| `R46` | `08` | ordinary operation `vocabulary.read` exact semantic owner | `03-vocabulary-and-context-resolution.md#vocabulary-contract` |
| `R47` | `08` | ordinary operation `resolution.stack.read` exact semantic owner | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition` |
| `R48` | `08` | ordinary operation `projection.definition.read` exact semantic owner | `04-projection-contracts.md#projection-definition` |
| `R49` | `08` | ordinary operation `artifact.kind.list` exact semantic owner | `01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract` |
| `R50` | `08` | ordinary operation `artifact.instance.list` exact semantic owner | `01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R51` | `08` | ordinary operation `artifact.read` exact semantic owner | `01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R52` | `08` | ordinary operation `artifact.validate` exact semantic owner | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract`<br>`02-intake-charter-and-validation.md#artifact-validation-layers` |
| `R53` | `08` | ordinary operation `artifact.render` exact semantic owner | `01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R54` | `08` | ordinary operation `intake.definition.read` exact semantic owner | `02-intake-charter-and-validation.md#artifact-intake-definition-contract` |
| `R55` | `08` | ordinary operation `intake.coverage.evaluate` exact semantic owner | `02-intake-charter-and-validation.md#artifact-intake-definition-contract` |
| `R56` | `08` | ordinary operation `intake.record.append` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts` |
| `R57` | `08` | ordinary operation `record.list` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts`<br>`03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion`<br>`06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R58` | `08` | ordinary operation `record.read` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts`<br>`03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion`<br>`06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R59` | `08` | ordinary operation `artifact.candidate.validate` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts`<br>`02-intake-charter-and-validation.md#artifact-validation-layers` |
| `R60` | `08` | ordinary operation `artifact.candidate.append` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts` |
| `R61` | `08` | ordinary operation `artifact.approval.append` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts` |
| `R62` | `08` | ordinary operation `artifact.candidate.promote` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts` |
| `R63` | `08` | ordinary operation `posture.resolve` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R64` | `08` | ordinary operation `posture.recommendation.evaluate` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R65` | `08` | ordinary operation `posture.recommendation.append` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R66` | `08` | ordinary operation `posture.recommendation.acknowledge` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R67` | `08` | ordinary operation `posture.transition.apply` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R68` | `08` | ordinary operation `projection.create` exact semantic owner | `04-projection-contracts.md#projection-definition`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result` |
| `R69` | `08` | ordinary operation `resolution.escalation.request.append` exact semantic owner | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion` |
| `R70` | `08` | ordinary operation `resolution.escalation.disposition.append` exact semantic owner | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion` |
| `R71` | `08` | ordinary operation `memory.promotion.request.append` exact semantic owner | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion` |
| `R72` | `08` | ordinary operation `memory.promotion.disposition.append` exact semantic owner | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion` |
| `R73` | `08` | ordinary operation `snapshot.capture` exact semantic owner | `05-snapshot-memory-contracts.md#snapshot-capture-policy`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R74` | `08` | ordinary operation `snapshot.read` exact semantic owner | `05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R75` | `08` | ordinary operation `snapshot.delta` exact semantic owner | `05-snapshot-memory-contracts.md#snapshot-delta` |
| `R76` | `08` | ordinary operation `snapshot.project` exact semantic owner | `04-projection-contracts.md#projection-definition`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result`<br>`05-snapshot-memory-contracts.md#snapshot-grounding-projection-definition`<br>`05-snapshot-memory-contracts.md#snapshot-projection-requestresult` |
| `R77` | `08` | ordinary operation `snapshot.verify_current` exact semantic owner | `05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R78` | `08` | ordinary operation `snapshot.resolve_applicable` exact semantic owner | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`05-snapshot-memory-contracts.md#snapshot-capture-policy`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R79` | `08` | ordinary operation `repository.setup.plan` exact semantic owner | `01-schema-profile-and-artifact-registry.md#schema-policy`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R80` | `08` | normalized-valid postselection request identity explicitly “defined below” | `09-machine-transport-and-adapter-contracts.md#transport-request-contract` |
| `R81` | `08` | operation-definition recovery-control/open/release semantics and adapter-local preflight boundary | `10-substrate-integration-and-publication.md#transitional-bundled-cli-bridge` |
| `R82` | `09` | transport-request operation selection, bootstrap, admission, and idempotency authority | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R83` | `09` | transport-request Resolution-envelope and Snapshot bindings | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R84` | `09` | transport-response operation selection, bootstrap, and idempotency authority | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R85` | `09` | transport-response Resolution-envelope and Snapshot bindings | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R86` | `09` | DTO/schema generated package and catalog fixtures | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R87` | `09` | CLI JSON operation-definition selection and exact typed response mapping | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R88` | `09` | Tauri exact SDK operation mapping and capability/Resolution checks | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R89` | `09` | transport-request bridge open/request mismatch and adapter-local pre-spawn fixtures | `10-substrate-integration-and-publication.md#transitional-bundled-cli-bridge` |
| `R90` | `10` | Substrate bridge preflight, admission, recovery control, request/response, and CLI invocation | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery`<br>`09-machine-transport-and-adapter-contracts.md#transport-request-contract`<br>`09-machine-transport-and-adapter-contracts.md#transport-response-and-expected-outcome-contract`<br>`09-machine-transport-and-adapter-contracts.md#cli-json-contract` |
| `R91` | `10` | published-Rust proof ownership, SDK boundary, independent review, and evidence record | `07-development-orchestration-contracts.md#review-choreography`<br>`08-sdk-operations-and-capability-discovery.md#hcm-04-crate-ownership-and-dependency-contract`<br>`08-sdk-operations-and-capability-discovery.md#sdk-ordinary-use-case-contract`<br>`11-contract-evidence-gates-and-docks.md#evidence-record` |
| `R92` | `11` | Evidence-record Resolution-envelope and Snapshot/delta bindings | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta` |
| `R93` | `11` | Gate-contract Projection and Snapshot/delta grounding refs | `04-projection-contracts.md#projection-result`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta` |
| `R94` | `11` | Dock request/result Resolution-envelope and provenance bindings | `03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R95` | `11` | Dock request canonical-artifact refs | `01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R96` | `11` | Dock request projection-artifact refs | `04-projection-contracts.md#projection-result` |
| `R97` | `11` | Dock request Snapshot/delta refs | `05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta` |
| `R98` | `11` | Public API proof gates reference the bridge and publication plan “above” | `10-substrate-integration-and-publication.md#transitional-bundled-cli-bridge`<br>`10-substrate-integration-and-publication.md#published-rust-api-proof-plan` |

## Exact H2-to-leaf ownership map

| Frozen H2 section | Canonical leaf |
|---|---|
| `Status` | `contracts/01-schema-profile-and-artifact-registry.md` |
| `Schema policy` | `contracts/01-schema-profile-and-artifact-registry.md` |
| `Instance profile contract` | `contracts/01-schema-profile-and-artifact-registry.md` |
| `Schema registry entry contract` | `contracts/01-schema-profile-and-artifact-registry.md` |
| `Stable-role registry contract` | `contracts/01-schema-profile-and-artifact-registry.md` |
| `Artifact kind definition contract` | `contracts/01-schema-profile-and-artifact-registry.md` |
| `Artifact instance descriptor contract` | `contracts/01-schema-profile-and-artifact-registry.md` |
| `Artifact intake definition contract` | `contracts/02-intake-charter-and-validation.md` |
| `Intake record and artifact candidate contracts` | `contracts/02-intake-charter-and-validation.md` |
| `Charter intake and canonical contract` | `contracts/02-intake-charter-and-validation.md` |
| `Artifact validation layers` | `contracts/02-intake-charter-and-validation.md` |
| `Vocabulary contract` | `contracts/03-vocabulary-and-context-resolution.md` |
| `Context Resolution stack definition` | `contracts/03-vocabulary-and-context-resolution.md` |
| `Context Resolution envelope` | `contracts/03-vocabulary-and-context-resolution.md` |
| `Resolution escalation and memory promotion` | `contracts/03-vocabulary-and-context-resolution.md` |
| `Projection disclosure policy` | `contracts/04-projection-contracts.md` |
| `Projection support evaluator definition` | `contracts/04-projection-contracts.md` |
| `Projection definition` | `contracts/04-projection-contracts.md` |
| `Projection request` | `contracts/04-projection-contracts.md` |
| `Projection result` | `contracts/04-projection-contracts.md` |
| `Snapshot capture policy` | `contracts/05-snapshot-memory-contracts.md` |
| `Context Memory Snapshot` | `contracts/05-snapshot-memory-contracts.md` |
| `Snapshot delta` | `contracts/05-snapshot-memory-contracts.md` |
| `Snapshot-grounding Projection definition` | `contracts/05-snapshot-memory-contracts.md` |
| `Snapshot projection request/result` | `contracts/05-snapshot-memory-contracts.md` |
| `Snapshot redaction and retention` | `contracts/05-snapshot-memory-contracts.md` |
| `Project posture kernel and recommendation contracts` | `contracts/06-posture-and-synthesis-contracts.md` |
| `Optional synthesis-candidate contract` | `contracts/06-posture-and-synthesis-contracts.md` |
| `Development orchestration contracts` | `contracts/07-development-orchestration-contracts.md` |
| `HCM-0.4 crate ownership and dependency contract` | `contracts/08-sdk-operations-and-capability-discovery.md` |
| `SDK ordinary-use-case contract` | `contracts/08-sdk-operations-and-capability-discovery.md` |
| `Operation definition contract` | `contracts/08-sdk-operations-and-capability-discovery.md` |
| `Capability bootstrap and catalog discovery` | `contracts/08-sdk-operations-and-capability-discovery.md` |
| `Transport request contract` | `contracts/09-machine-transport-and-adapter-contracts.md` |
| `Transport response and expected-outcome contract` | `contracts/09-machine-transport-and-adapter-contracts.md` |
| `DTO and JSON Schema generation contract` | `contracts/09-machine-transport-and-adapter-contracts.md` |
| `CLI JSON contract` | `contracts/09-machine-transport-and-adapter-contracts.md` |
| `Tauri adapter contract` | `contracts/09-machine-transport-and-adapter-contracts.md` |
| `Substrate integration contracts` | `contracts/10-substrate-integration-and-publication.md` |
| `Published Rust API proof plan` | `contracts/10-substrate-integration-and-publication.md` |
| `Contract record` | `contracts/11-contract-evidence-gates-and-docks.md` |
| `Evidence record` | `contracts/11-contract-evidence-gates-and-docks.md` |
| `Verdict contract` | `contracts/11-contract-evidence-gates-and-docks.md` |
| `Gate contract` | `contracts/11-contract-evidence-gates-and-docks.md` |
| `Dock capability manifest` | `contracts/11-contract-evidence-gates-and-docks.md` |
| `Dock request/result` | `contracts/11-contract-evidence-gates-and-docks.md` |
| `Public API proof gates` | `contracts/11-contract-evidence-gates-and-docks.md` |
| `Schema compatibility posture` | `contracts/11-contract-evidence-gates-and-docks.md` |

The mechanical verifier must derive the actual frozen H2 list from Git and compare it with this table. A missing, duplicate, unexpected, reordered, or differently assigned H2 fails closed.

## Canonical ownership and no-duplication rules

- Normative contract content lives only in the leaf named by the ownership map.
- The index owns file discovery, leaf order, routing metadata, compatibility aliases, baseline/proof metadata, and selective-loading instructions only.
- Crosswalk, phase, proof, orchestration, and handoff documents may describe why or when to load a leaf. They may not copy its field contracts or truth tables.
- A concept that crosses leaf boundaries is referenced by exact leaf path/anchor; it is not restated in a second leaf.
- A later semantic correction must run under the owning semantic slice and change the relevant leaf directly. It must not be smuggled into a topology repair.
- The stable index is not a fallback semantic authority. If index text and leaf payload ever conflict, the leaf is canonical and the routing defect blocks closeout.

## Cross-leaf context preservation

The complete reviewed inventory is
`evidence/dependency-audit.md`. The verifier independently derives all eleven
spans, 48 H2 and 22 H3 headings, 84 frozen positional occurrences, 98 exact
cross-leaf routes, all 50 ordinary-operation fixtures, 11 named non-route
groups, and the aggregate fanout of every H2/H3. Each positional or named
reference is classified as:

1. same-leaf;
2. non-layout-dependent; or
3. cross-leaf with one exact declared trigger and target list.

Every leaf has outbound routes. The redesigned trigger-indexed topology rejects
both omission and over-routing: a route is selected only by its exact source
field, statement, or operation fixture, never by a heading alone or by loading a
target. The verifier fails on an unclassified reference, missing route, broken
target, wrong/overlapping source trigger, reordered target, unjustified extra
route, missing operation fixture, or stale heading-fanout classification.

## Stable-index responsibilities

After execution, `05-contracts-schemas-and-gates.md` must contain only:

1. the unchanged H1 `Contracts, Schemas, and Gates`;
2. a topology status and frozen-baseline identifier;
3. the ordered leaf catalog with exact relative links and scope summaries labeled as routing metadata;
4. the exact H2-to-leaf ownership table;
5. the exact cross-leaf dependency table and source-triggered selective-loading rules;
6. an explicit statement that leaf files are canonical and the index is non-normative routing metadata;
7. one forwarding alias for every frozen H2/H3 generated anchor;
8. parity/proof pointers to HCM-0.9 and `PG-CATALOG-01`.

It must not contain copied schema blocks, field definitions, contract prose, defaults, validation rules, examples, or semantic compatibility statements from the frozen payload.

## Historical-anchor behavior

- The unchanged H1 preserves `05-contracts-schemas-and-gates.md#contracts-schemas-and-gates`.
- For every frozen H2/H3 heading, the index contains an explicit HTML anchor using the frozen generated slug and an adjacent Markdown link to the same heading in its canonical leaf.
- The leaf heading text remains byte-identical, so its generated fragment is the same as the historical fragment.
- The verifier derives all 70 frozen H2/H3 aliases from the baseline, requires each alias exactly once in the index, and requires the target path/fragment to exist.
- An old `05#fragment` link therefore lands on the stable index at a visible forwarding link. HCM-0.9 does not claim an HTTP redirect or rewrite old committed references.
- Historical `path:line-range` citations remain commit-bounded evidence. The index must not fabricate equivalent current line numbers.

## Selective-routing behavior

### Orchestration

1. Use the stable index only to discover the owning source leaf and its trigger-indexed route rows.
2. A task concept or named H2/H3 is a locator only. Match the task to one or more exact `R*` source triggers or one `O*` ordinary-operation fixture; a heading by itself activates no dependency.
3. Derive the distinct source-plus-target union before loading targets. Load only exact route targets; loading them does not activate their unrelated rows.
4. If the union contains six or more of the eleven leaves, classify the task `broad-only`. Do not force it through selective routing; use explicit multi-owner/complete-catalog assembly or stop for a reviewed partition redesign.
5. HCM-0.5 selects `contracts/11-contract-evidence-gates-and-docks.md` and only exact sibling triggers it invokes; it does not reopen the monolith.

### Internal dispatch and subject manifests

- `authority_refs` and `contracts_and_gates` name exact leaf paths/anchors.
- A dispatch subject manifest contains files whose bytes are under review, not every authority file read for context.
- Any changed routing, proof, audit, index, leaf, verifier, or other file whose
  bytes are part of the review subject appears in the manifest. Unchanged
  contextual authority appears only in `authority_refs` and/or
  `contracts_and_gates`; reading it never makes it a subject file.
- A leaf-local semantic change manifests the changed leaf and only the other
  files whose bytes are changed or intentionally reviewed. It does not include
  unrelated or unchanged authority.
- Complete-catalog topology/parity review manifests the stable index, all eleven
  leaves, verifier, and other routing/proof files only when those exact bytes are
  changed or intentionally under review; unchanged contextual authority is not
  added by default.
- Every dispatch keeps `required_skills[0] = using-agent-skills`.
- Existing dispatch JSON/Markdown remains immutable even when it names the monolith.

### Proof

- `06` proof gates reference the exact leaves that own the semantics under proof.
- `PG-CATALOG-01` additionally binds the stable index, all leaves, the mapping, the frozen baseline commit/SHA, and verifier output.
- Runtime proof gates remain open. Structural parity does not prove implementation or public adoption.

### Parent handoff

- New parent handoffs use exact leaf refs in `authority_refs`, `deliverables`, `validation`, and `reviewed_state.proof_refs` when contract semantics are relevant.
- A topology-wide HCM-0.9 execution handoff may reference the stable index plus all leaves; a later leaf-local handoff names only the affected leaves.
- Historical handoffs remain byte-immutable. Their old monolith refs stay valid as historical commit evidence and through index aliases when fragment-based.
- No handoff schema change is required; current repository-relative string/path references already support exact leaves.

## Mutable execution surface

Execution must re-inventory refs from the live tree before editing. The expected existing mutable surfaces are:

| Path | Required execution update |
|---|---|
| `00-README.md` | make `05` explicitly the stable index and expose the ordered leaf catalog/selective-loading rule |
| `02-semantic-model.md` | replace its two generic `05` references with the exact owning leaf refs |
| `03-seam-crosswalk.md` | update only the contract-catalog topology row/current evidence after parity lands |
| `04-phase-slice-map.md` | preserve HCM-0.9 sequencing and bind closeout evidence; do not change semantic phase scope |
| `05-contracts-schemas-and-gates.md` | replace semantic body with the stable routing/compatibility index |
| `06-proof-and-regression-ledger.md` | route HCM-0.2/0.3/0.4 proof bullets to exact leaves and close only `PG-CATALOG-01` when proven |
| `07-orchestration-onboarding-prompt.md` | select the minimum exact contract leaf set rather than generic “contracts from 05” |
| `08-handoff-ledger-and-escalation-protocol.md` | require exact leaf refs for new dispatch/handoff evidence and preserve historical refs |
| `handoffs/internal-dispatch-template.json` | replace the monolith example with `contracts/07-development-orchestration-contracts.md#development-orchestration-contracts` |
| `handoffs/dispatch-template.md` | instruct authors to name exact contract leaf paths/anchors |

Expected new execution surfaces are the eleven leaves, `slices/HCM-0.9/verify_contract_catalog.py`, and `slices/HCM-0.9/evidence/execution-start.json`. New HCM-0.9 review dispatches, the final parent handoff, and the ledger entry are generated proof/closeout artifacts, not rewrites of existing history.

If live inventory identifies another mutable active reference, execution may update it only when it is necessary to prevent a broken or non-selective current reference and remains inside the control pack. Otherwise stop for scope review.

## Immutable execution surface

- every record that exists under `handoffs/records/` at execution start;
- every dispatch that exists under `handoffs/dispatches/` at execution start;
- historical schema versions and their admission hashes;
- commits `214a5b8...` and `3d993ab...` as immutable baseline/closeout evidence;
- archived documentation and reduced-v1 provenance;
- all Rust, Cargo, CLI, SDK, Tauri, Substrate, runtime, test, and fixture files.

New additive HCM-0.9 dispatches and the final parent handoff are allowed. Pre-existing files are not.

At preflight, execution writes `slices/HCM-0.9/evidence/execution-start.json` with the exact live `execution_start_head`, frozen baseline commit, and the two immutable roots. The roots must contain no untracked files before capture. The verifier derives the complete pre-existing path set from `git ls-tree -r --name-only <execution_start_head>` and compares every working-tree path byte-for-byte with `git cat-file blob <execution_start_head>:<path>`. A missing path proves deletion or rename and fails; a changed hash fails; a new path absent from the start commit is allowed only as an additive HCM-0.9 dispatch/record.

## Mechanical parity proof

Execution creates `slices/HCM-0.9/verify_contract_catalog.py`. The verifier must fail closed unless all of the following are true:

1. `git show` returns the exact baseline file and its SHA-256 equals the frozen value.
2. The baseline begins with the exact H1 and one blank line.
3. The actual frozen H2 sequence equals the specification map exactly.
4. Every leaf exists, has its specified H1 and only its allowed routing-only scaffold before the first H2, and contains exactly its contiguous assigned H2 span.
5. Removing all validated bytes before each leaf's first H2 and concatenating leaf payload bytes in topology order equals the frozen baseline body byte-for-byte.
6. The concatenated H2/H3 sequence, heading text, and fence-open/fence-close sequence equals baseline.
7. Each frozen H2 appears in one and only one leaf; no unassigned or duplicate payload remains.
8. Every baseline YAML/JSON fence body in the reconstructed payload has the same byte digest and remains parseable when the baseline example was parseable.
9. The index contains the exact ordered leaf links, exact H2 ownership map, and exactly one forwarding alias/target for every frozen H2/H3 anchor.
10. Every index/leaf relative path and fragment resolves.
11. The stable index contains no baseline fenced payload or duplicated frozen H2 semantic body.
12. The verifier derives the complete all-eleven-leaf audit: 48 H2, 22 H3,
    exactly 84 positional occurrences, `R01`-`R98`, `O01`-`O50`, `N01`-`N11`,
    and every heading aggregate. Each leaf block equals its trigger-indexed route
    rows in global order; headings alone activate none; the three seven-leaf
    aggregates remain `broad-only`; omitted, broken, overlapping, unjustified,
    or unclassified routes/references fail.
13. Every pre-existing handoff/dispatch path derived from `execution_start_head` still exists with identical bytes; modified/deleted/renamed cases fail and additive HCM-0.9 artifacts pass.

The full-file leaf bytes do not equal the old file because each leaf adds one H1 scaffold and the index is new routing metadata. The semantic-payload comparison defined above is the sole parity authority; prose similarity or reviewer judgment is not a substitute.

## Validation matrix

| Gate | Required evidence |
|---|---|
| Baseline | exact commit/path/full-file SHA-256 |
| Mapping | 48 frozen H2 sections, exact order, exactly one leaf each |
| Parity | ordered payload reconstruction byte-equals frozen body |
| Anchors | all frozen H2/H3 aliases exist once and target an existing leaf fragment |
| Links | every changed Markdown relative path and fragment resolves |
| Fences | balanced fence sequence, identical info strings/bodies, no cross-leaf split |
| Schema/examples | all baseline-parseable YAML/JSON examples still parse; byte digests unchanged |
| Semantics | applicable HCM-0.2/0.3/0.4 assertions pass against reconstructed payload; every exact cross-leaf dependency row and source trigger is proven |
| Dependency audit | all 11 spans, 48 H2/22 H3, 84 positional occurrences, `R01`-`R98`, `O01`-`O50`, `N01`-`N11`, and every heading aggregate derive exactly; omitted, broken, overlapping, unjustified, unclassified, and stale-fanout negatives fail |
| Dispatch manifests | sorted unique repo-relative paths; SHA-256 aggregate replay succeeds |
| Handoffs | normal validation plus v1 admission and orchestration self-tests pass |
| History | execution-start record is valid; every start-commit handoff/dispatch path still exists with the same Git blob hash; modified/deleted/renamed negatives fail and additive HCM-0.9 artifacts pass |
| Archive | archive-boundary check and self-test pass |
| Scope | no Rust/Cargo/runtime/public API/HCM-0.5/unrelated path changed |
| Git | `git diff --check`, exact diff inspection, staged GitNexus detection |
| Review | fresh isolated complete-subject reviewer returns CLEAN within budget |

## Required verification commands

Run from repository root. The parity verifier owns detailed Markdown anchor/link/fence/example checks for the index and leaves.

```bash
python3 docs/specs/handbook-contract-membrane/slices/HCM-0.9/verify_contract_catalog.py --self-test
python3 docs/specs/handbook-contract-membrane/slices/HCM-0.9/verify_contract_catalog.py \
  --baseline-commit 214a5b8eb182fce74478df49d4f55d226d65fdf5 \
  --expected-baseline-sha256 c7f61db209a81ba20690f365b4069dd01f11e395335bfa10d2ce21143cc2985d \
  --execution-start-record docs/specs/handbook-contract-membrane/slices/HCM-0.9/evidence/execution-start.json
python3 tools/check_archive_boundary.py
python3 tools/check_archive_boundary.py --self-test
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
git diff --check
npx gitnexus detect-changes -r handbook -s staged
```

Also inspect:

```bash
git status --short
git diff --name-status
git diff -- docs/specs/handbook-contract-membrane
git diff --cached --check
git diff --cached --name-only
```

## Rollback and stop behavior

- Perform the split from frozen Git bytes, not manual copy/paste.
- Keep the monolithic `05` body intact until all leaf shadow copies pass group mapping/fence checks.
- Make the stable-index cutover only after all eleven leaf payloads reconstruct the baseline.
- Do not commit or dispatch a partially canonical split.
- Before Task 10's reviewed execution commit, rollback is deletion of untracked leaf/verifier/evidence files plus restoration of changed mutable control files from the execution-start commit. Never use a broad reset that could discard unrelated work.
- After the reviewed execution commit, rollback is a scoped revert of that commit; never edit historical handoffs/dispatches or rewrite published history.
- Stop immediately on baseline hash mismatch, overlap with unattributable work, noncontiguous section ownership, fence split, parity failure, unresolved mutable reference, historical-byte drift, required scope widening, or inability to run mandatory validation/review.
- If a semantic defect is discovered, record its exact leaf/heading and stop or defer it. Do not combine the fix with decomposition.

## Execution review budget

- Maximum four complete-subject review submissions.
- Maximum three remediation rounds.
- Reviewers are fresh isolated built-in `default` agents and read-only.
- Review dispatch `required_skills` begins with `using-agent-skills`, followed by `code-review-and-quality`.
- Stop immediately on CLEAN.
- Use a different fresh reviewer after every remediation.
- Do not run Review 5.
- Do not self-approve, reuse a reviewer, suppress a valid finding, weaken severity, or claim clean when the budget expires.

If Review 4 still has valid Critical or Required findings, do not authorize HCM-0.5 or claim HCM-0.9 execution complete. Write an honest v1.2 parent handoff with `status: partial`, `stop_reason: human_input`, `resume.execution_target: human_interactive`, the unresolved findings, and the exact choice required: continue the topology plan, redesign it, or abandon HCM-0.9.

## Planning packet review budget

The original planning/control-pack registration budget was at most three
complete-subject reviews and two remediations. It stopped after Review 3 with
the immutable partial human-input handoff; that evidence is not rewritten.

The explicit human continuation authorizes one new bounded pass: at most
Continuation Review 1, one intervening remediation, and a different-fresh
Continuation Review 2. It stops immediately on CLEAN and never runs a third
continuation review. If Continuation Review 2 retains an actionable finding,
the parent does not authorize execution, writes a new partial human-input
handoff, and recommends redesign or abandonment rather than another automatic
extension.

## Acceptance criteria

HCM-0.9 execution is complete only when:

- [ ] all eleven exact leaf files exist with the specified contiguous section spans;
- [ ] all 48 frozen H2 sections map exactly once and all 22 frozen H3 headings stay under the same owning H2 span;
- [ ] the ordered leaf semantic payload is byte-identical to the frozen baseline body;
- [ ] `05` is a stable routing/compatibility index with all frozen H2/H3 aliases and no duplicated normative payload;
- [ ] current mutable control-pack/orchestration/dispatch/proof/handoff references use exact minimum leaf selection;
- [ ] every pre-existing handoff/dispatch byte is unchanged;
- [ ] every frozen positional or named contract dependency remains same-leaf, non-layout-dependent, or backed by the exact verified source-triggered cross-leaf routing dependency table;
- [ ] the trigger-indexed audit derives `R01`-`R98`, `O01`-`O50`,
  `N01`-`N11`, all 70 heading aggregates, and broad-only classification for
  every six-or-more-leaf union; a heading alone activates no dependency;
- [ ] links, anchors, fences, parseable YAML/JSON, semantic assertions, manifests, handoffs, archive, parity, scope, and Git gates pass;
- [ ] no Rust/runtime/public API/schema-version/proof-promotion/HCM-0.5/unrelated change occurred;
- [ ] a fresh isolated complete-subject reviewer returns CLEAN within the execution budget;
- [ ] the reviewed execution commit exists;
- [ ] the separate mechanical v1.2 parent handoff/ledger closeout validates;
- [ ] the closeout claims structural parity/selective routing only and does not claim new contract semantics or runtime completion.

## Open questions

None. Any request to change the topology, baseline, semantic payload, or review budget requires an explicit reviewed packet change before execution continues.
