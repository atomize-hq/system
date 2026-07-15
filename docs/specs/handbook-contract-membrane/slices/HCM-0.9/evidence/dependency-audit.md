# HCM-0.9 all-eleven-leaf dependency audit

**Status:** planning evidence only; HCM-0.9 execution is not authorized
**Frozen source:** `git:214a5b8eb182fce74478df49d4f55d226d65fdf5:docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`
**Frozen SHA-256:** `c7f61db209a81ba20690f365b4069dd01f11e395335bfa10d2ce21143cc2985d`

## Method

- Audit all eleven contiguous spans defined by `../SPEC.md`; no leaf is presumed dependency-free.
- Scan frozen prose outside fenced code for every whole-word positional token and inspect every named cross-leaf authority use.
- Classify every occurrence as same-leaf, non-layout-dependent, or cross-leaf with one exact source trigger and target.
- Split ordinary-use-case families into one fixture per frozen operation. A route exists only when that exact operation invokes a frozen cross-leaf owner.
- Derive aggregate fanout for every frozen H2/H3. Named sections are locators, not route activation units; exact route/operation triggers own selective activation.

## Completeness and topology result

- Frozen spans audited: **11 of 11**; frozen headings audited: **48 H2 and 22 H3**.
- Positional tokens outside fences: **84** on **70** source lines: **31 same-leaf**, **43 non-layout-dependent**, **10 cross-leaf**.
- Exact cross-leaf routes: **98** (`R01`-`R98`), with outbound routes from every proposed leaf.
- Frozen ordinary operations: **50** (`O01`-`O50`); **38** route to exact cross-leaf owners and **12** are explicit same-leaf/non-route fixtures.
- Continuation Review 1 proved that flat named-section union routing would load 7 of 11 leaves for both `Instance profile contract` and `SDK ordinary-use-case contract`. The prior flat routing topology is therefore rejected.
- The redesigned topology is trigger-indexed: one exact `R*` trigger (or one exact `O*` fixture) is the normal selective unit, and no such route targets more than three sibling leaves. A named H2/H3 alone activates no route.
- A multi-trigger task must derive the union before loading targets. A source-plus-target union of six or more leaves is `broad-only`, never normal selective routing, and requires explicit multi-owner/complete-catalog assembly or a future partition redesign. Loading a target never activates that target leaf’s unrelated routes.

## Named cross-leaf authority inventory

| Route | Source leaf | Frozen line evidence | Exact triggering source | Required exact targets |
|---|---:|---|---|---|
| `R01` | 1 | `75, 175` | instance-profile `artifact_instances[].intake_definition_ref` materialization | `02-intake-charter-and-validation.md#artifact-intake-definition-contract` |
| `R02` | 1 | `80, 123, 181, 239-245` | instance-profile `vocabulary_ref` and resolved vocabulary identity | `03-vocabulary-and-context-resolution.md#vocabulary-contract` |
| `R03` | 1 | `81, 124, 211, 244` | instance-profile `context_resolution_ref` and resolved stack identity | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition` |
| `R04` | 1 | `82-84, 125, 183-184, 245` | instance-profile Projection catalog, including the named snapshot-grounding definition | `04-projection-contracts.md#projection-definition`<br>`05-snapshot-memory-contracts.md#snapshot-grounding-projection-definition` |
| `R05` | 1 | `85-87, 126, 189-191, 247` | instance-profile posture-evaluation-policy pair | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R06` | 1 | `88, 127, 192, 220, 246` | instance-profile `dock_requirement_refs` and later dock compatibility | `11-contract-evidence-gates-and-docks.md#dock-capability-manifest` |
| `R07` | 1 | `380, 395, 418` | artifact-kind `projection_definition_refs` and fixed-renderer/Projection distinction | `04-projection-contracts.md#projection-definition` |
| `R08` | 1 | `487, 524` | artifact-instance `intake_definition_ref` and exact target-kind validation | `02-intake-charter-and-validation.md#artifact-intake-definition-contract` |
| `R09` | 1 | `490, 526` | artifact-instance `projection_definition_refs` selection | `04-projection-contracts.md#projection-definition` |
| `R10` | 2 | `541, 908-910` | intake schema identity, target `artifact_kind_ref`, and candidate-schema closure | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract` |
| `R11` | 2 | `901, 917-926, 937` | intake-definition coverage evidence, confidence, freshness, waiver, and typed evaluation | `11-contract-evidence-gates-and-docks.md#evidence-record` |
| `R12` | 2 | `1253` | intake-record `coverage_results` typed evidence/confidence/freshness values | `11-contract-evidence-gates-and-docks.md#evidence-record` |
| `R13` | 2 | `948, 1139` | intake-record constitutional-root instance/capability selection | `01-schema-profile-and-artifact-registry.md#semantic-capability-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R14` | 2 | `1297` | Charter constitutional-root instance/capability selection | `01-schema-profile-and-artifact-registry.md#semantic-capability-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R15` | 2 | `1287` | Charter fixed-renderer exclusion from Resolution-aware Projection authority | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result` |
| `R16` | 2 | `1314` | artifact-validation fixed-renderer exclusion from Resolution-aware Projection authority | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result` |
| `R17` | 2 | `1309-1316` | artifact-validation external docks, normalized evidence, and required evidence gates | `11-contract-evidence-gates-and-docks.md#dock-capability-manifest`<br>`11-contract-evidence-gates-and-docks.md#dock-requestresult`<br>`11-contract-evidence-gates-and-docks.md#gate-contract` |
| `R18` | 3 | `1327, 1355-1361` | vocabulary stable-role registry pair and role/absorption closure | `01-schema-profile-and-artifact-registry.md#stable-role-registry-contract` |
| `R19` | 3 | `1353, 1355` | vocabulary schema identity and resolved-profile compatibility | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R20` | 3 | `1457` | Resolution-stack schema identity and selected-profile compatibility | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R21` | 3 | `1511` | Resolution-envelope schema identity and selected-profile compatibility | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R22` | 3 | `1598-1603` | escalation/memory-promotion Projection, Snapshot, and delta source inputs | `04-projection-contracts.md#projection-result`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta` |
| `R23` | 4 | `1627-1640` | Projection-disclosure mapping of immutable upstream redaction dispositions | `05-snapshot-memory-contracts.md#snapshot-redaction-and-retention` |
| `R24` | 4 | `1629-1631` | Projection-disclosure definition/profile/source/kind/capability/schema validation | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#semantic-capability-contract` |
| `R25` | 4 | `1631, 1635` | Projection-disclosure profile-stack and Resolution compatibility | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R26` | 4 | `1680` | Projection-support evaluator source/kind/capability/schema metadata | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#semantic-capability-contract` |
| `R27` | 4 | `1747, 1756-1758` | Projection-definition profile/source/kind/capability/schema validation | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#semantic-capability-contract` |
| `R28` | 4 | `1765, 1775-1788` | Projection-request profile, vocabulary, stack, and Resolution-envelope pairs | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#vocabulary-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R29` | 4 | `1803-1806, 1919` | Projection-result profile, vocabulary, stack, and Resolution-envelope pairs | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#vocabulary-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R30` | 4 | `1760` | Projection-definition `claim_refs` decision-completeness effects | `11-contract-evidence-gates-and-docks.md#contract-record` |
| `R31` | 4 | `1854-1867, 1924-1925` | Projection-result omission `claim_refs` and proof effects | `11-contract-evidence-gates-and-docks.md#contract-record` |
| `R32` | 5 | `1943-1946, 2000` | capture-policy memory horizons and capture-envelope membership | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R33` | 5 | `2032-2033, 2147` | Context Memory Snapshot resolved-profile and Resolution-envelope pairs | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R34` | 5 | `2293-2331` | snapshot-grounding Projection definition, minimum Resolution, disclosure, support, and currentness | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`04-projection-contracts.md#projection-disclosure-policy`<br>`04-projection-contracts.md#projection-support-evaluator-definition`<br>`04-projection-contracts.md#projection-definition` |
| `R35` | 5 | `2333-2442` | snapshot Projection DTOs extend generic request/result and retain profile/vocabulary/envelope semantics | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#vocabulary-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result` |
| `R36` | 5 | `2509, 2578` | snapshot redaction original/retained mapping into generic Projection disclosure | `04-projection-contracts.md#projection-disclosure-policy` |
| `R37` | 5 | `2548` | retention holds from handoff, evidence, verdict, gate, and active-promotion refs | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion`<br>`07-development-orchestration-contracts.md#development-orchestration-contracts`<br>`11-contract-evidence-gates-and-docks.md#evidence-record`<br>`11-contract-evidence-gates-and-docks.md#verdict-contract`<br>`11-contract-evidence-gates-and-docks.md#gate-contract` |
| `R38` | 6 | `2652-2680, 2817` | posture-kernel constitutional artifact and resolved-profile inputs | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract`<br>`02-intake-charter-and-validation.md#charter-intake-and-canonical-contract` |
| `R39` | 6 | `2632-2642, 2658, 2818, 2832, 2844-2848` | posture contract/evidence/snapshot inputs, freshness, and non-causal snapshot drift | `05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta`<br>`11-contract-evidence-gates-and-docks.md#contract-record`<br>`11-contract-evidence-gates-and-docks.md#evidence-record` |
| `R40` | 6 | `2793-2795, 2838, 2850` | posture-transition intake reassessment and canonical Charter compare-and-write | `02-intake-charter-and-validation.md#artifact-intake-definition-contract`<br>`02-intake-charter-and-validation.md#charter-intake-and-canonical-contract` |
| `R41` | 7 | `2902` | dispatch execution-envelope active Resolution field | `03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R42` | 8 | `3006` | ordinary operation `profile.list` exact semantic owner | `01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R43` | 8 | `3007` | ordinary operation `profile.resolve` exact semantic owner | `01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R44` | 8 | `3008` | ordinary operation `schema.list` exact semantic owner | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract` |
| `R45` | 8 | `3009` | ordinary operation `schema.read` exact semantic owner | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract` |
| `R46` | 8 | `3010` | ordinary operation `vocabulary.read` exact semantic owner | `03-vocabulary-and-context-resolution.md#vocabulary-contract` |
| `R47` | 8 | `3011` | ordinary operation `resolution.stack.read` exact semantic owner | `03-vocabulary-and-context-resolution.md#context-resolution-stack-definition` |
| `R48` | 8 | `3012` | ordinary operation `projection.definition.read` exact semantic owner | `04-projection-contracts.md#projection-definition` |
| `R49` | 8 | `3013` | ordinary operation `artifact.kind.list` exact semantic owner | `01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract` |
| `R50` | 8 | `3014` | ordinary operation `artifact.instance.list` exact semantic owner | `01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R51` | 8 | `3015` | ordinary operation `artifact.read` exact semantic owner | `01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R52` | 8 | `3016` | ordinary operation `artifact.validate` exact semantic owner | `01-schema-profile-and-artifact-registry.md#schema-registry-entry-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract`<br>`02-intake-charter-and-validation.md#artifact-validation-layers` |
| `R53` | 8 | `3017` | ordinary operation `artifact.render` exact semantic owner | `01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract`<br>`01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R54` | 8 | `3018` | ordinary operation `intake.definition.read` exact semantic owner | `02-intake-charter-and-validation.md#artifact-intake-definition-contract` |
| `R55` | 8 | `3019` | ordinary operation `intake.coverage.evaluate` exact semantic owner | `02-intake-charter-and-validation.md#artifact-intake-definition-contract` |
| `R56` | 8 | `3020` | ordinary operation `intake.record.append` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts` |
| `R57` | 8 | `3021` | ordinary operation `record.list` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts`<br>`03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion`<br>`06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R58` | 8 | `3022` | ordinary operation `record.read` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts`<br>`03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion`<br>`06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R59` | 8 | `3023` | ordinary operation `artifact.candidate.validate` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts`<br>`02-intake-charter-and-validation.md#artifact-validation-layers` |
| `R60` | 8 | `3024` | ordinary operation `artifact.candidate.append` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts` |
| `R61` | 8 | `3025` | ordinary operation `artifact.approval.append` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts` |
| `R62` | 8 | `3026` | ordinary operation `artifact.candidate.promote` exact semantic owner | `02-intake-charter-and-validation.md#intake-record-and-artifact-candidate-contracts` |
| `R63` | 8 | `3027` | ordinary operation `posture.resolve` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R64` | 8 | `3028` | ordinary operation `posture.recommendation.evaluate` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R65` | 8 | `3029` | ordinary operation `posture.recommendation.append` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R66` | 8 | `3030` | ordinary operation `posture.recommendation.acknowledge` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R67` | 8 | `3031` | ordinary operation `posture.transition.apply` exact semantic owner | `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `R68` | 8 | `3032` | ordinary operation `projection.create` exact semantic owner | `04-projection-contracts.md#projection-definition`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result` |
| `R69` | 8 | `3033` | ordinary operation `resolution.escalation.request.append` exact semantic owner | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion` |
| `R70` | 8 | `3034` | ordinary operation `resolution.escalation.disposition.append` exact semantic owner | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion` |
| `R71` | 8 | `3035` | ordinary operation `memory.promotion.request.append` exact semantic owner | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion` |
| `R72` | 8 | `3036` | ordinary operation `memory.promotion.disposition.append` exact semantic owner | `03-vocabulary-and-context-resolution.md#resolution-escalation-and-memory-promotion` |
| `R73` | 8 | `3037` | ordinary operation `snapshot.capture` exact semantic owner | `05-snapshot-memory-contracts.md#snapshot-capture-policy`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R74` | 8 | `3038` | ordinary operation `snapshot.read` exact semantic owner | `05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R75` | 8 | `3039` | ordinary operation `snapshot.delta` exact semantic owner | `05-snapshot-memory-contracts.md#snapshot-delta` |
| `R76` | 8 | `3040` | ordinary operation `snapshot.project` exact semantic owner | `04-projection-contracts.md#projection-definition`<br>`04-projection-contracts.md#projection-request`<br>`04-projection-contracts.md#projection-result`<br>`05-snapshot-memory-contracts.md#snapshot-grounding-projection-definition`<br>`05-snapshot-memory-contracts.md#snapshot-projection-requestresult` |
| `R77` | 8 | `3041` | ordinary operation `snapshot.verify_current` exact semantic owner | `05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R78` | 8 | `3042` | ordinary operation `snapshot.resolve_applicable` exact semantic owner | `01-schema-profile-and-artifact-registry.md#instance-profile-contract`<br>`03-vocabulary-and-context-resolution.md#context-resolution-stack-definition`<br>`03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`05-snapshot-memory-contracts.md#snapshot-capture-policy`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R79` | 8 | `3043` | ordinary operation `repository.setup.plan` exact semantic owner | `01-schema-profile-and-artifact-registry.md#schema-policy`<br>`01-schema-profile-and-artifact-registry.md#instance-profile-contract` |
| `R80` | 8 | `3164` | normalized-valid postselection request identity explicitly “defined below” | `09-machine-transport-and-adapter-contracts.md#transport-request-contract` |
| `R81` | 8 | `3158-3208` | operation-definition recovery-control/open/release semantics and adapter-local preflight boundary | `10-substrate-integration-and-publication.md#transitional-bundled-cli-bridge` |
| `R82` | 9 | `3386, 3391` | transport-request operation selection, bootstrap, admission, and idempotency authority | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R83` | 9 | `3374, 3391` | transport-request Resolution-envelope and Snapshot bindings | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R84` | 9 | `3470, 3476, 3482` | transport-response operation selection, bootstrap, and idempotency authority | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R85` | 9 | `3435, 3484, 3517` | transport-response Resolution-envelope and Snapshot bindings | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot` |
| `R86` | 9 | `3529` | DTO/schema generated package and catalog fixtures | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R87` | 9 | `3533-3557` | CLI JSON operation-definition selection and exact typed response mapping | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R88` | 9 | `3561-3571` | Tauri exact SDK operation mapping and capability/Resolution checks | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `R89` | 9 | `3407` | transport-request bridge open/request mismatch and adapter-local pre-spawn fixtures | `10-substrate-integration-and-publication.md#transitional-bundled-cli-bridge` |
| `R90` | 10 | `3573-3594` | Substrate bridge preflight, admission, recovery control, request/response, and CLI invocation | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract`<br>`08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery`<br>`09-machine-transport-and-adapter-contracts.md#transport-request-contract`<br>`09-machine-transport-and-adapter-contracts.md#transport-response-and-expected-outcome-contract`<br>`09-machine-transport-and-adapter-contracts.md#cli-json-contract` |
| `R91` | 10 | `3604-3616` | published-Rust proof ownership, SDK boundary, independent review, and evidence record | `07-development-orchestration-contracts.md#review-choreography`<br>`08-sdk-operations-and-capability-discovery.md#hcm-04-crate-ownership-and-dependency-contract`<br>`08-sdk-operations-and-capability-discovery.md#sdk-ordinary-use-case-contract`<br>`11-contract-evidence-gates-and-docks.md#evidence-record` |
| `R92` | 11 | `3642-3656` | Evidence-record Resolution-envelope and Snapshot/delta bindings | `03-vocabulary-and-context-resolution.md#context-resolution-envelope`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta` |
| `R93` | 11 | `3678-3692` | Gate-contract Projection and Snapshot/delta grounding refs | `04-projection-contracts.md#projection-result`<br>`05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta` |
| `R94` | 11 | `3723, 3738` | Dock request/result Resolution-envelope and provenance bindings | `03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `R95` | 11 | `3724` | Dock request canonical-artifact refs | `01-schema-profile-and-artifact-registry.md#artifact-instance-descriptor-contract` |
| `R96` | 11 | `3724` | Dock request projection-artifact refs | `04-projection-contracts.md#projection-result` |
| `R97` | 11 | `3725` | Dock request Snapshot/delta refs | `05-snapshot-memory-contracts.md#context-memory-snapshot`<br>`05-snapshot-memory-contracts.md#snapshot-delta` |
| `R98` | 11 | `3741-3751` | Public API proof gates reference the bridge and publication plan “above” | `10-substrate-integration-and-publication.md#transitional-bundled-cli-bridge`<br>`10-substrate-integration-and-publication.md#published-rust-api-proof-plan` |

## SDK ordinary-operation fixtures

Every frozen ordinary operation has exactly one fixture. `—` is a proved non-route, not an omission.

| Fixture | Frozen operation / line | Cross-leaf route | Classification |
|---|---|---|---|
| `O01` | `capabilities.describe` / `3005` | `—` | same-leaf capability/bootstrap catalog authority |
| `O02` | `profile.list` / `3006` | `R42` | cross-leaf exact owner |
| `O03` | `profile.resolve` / `3007` | `R43` | cross-leaf exact owner |
| `O04` | `schema.list` / `3008` | `R44` | cross-leaf exact owner |
| `O05` | `schema.read` / `3009` | `R45` | cross-leaf exact owner |
| `O06` | `vocabulary.read` / `3010` | `R46` | cross-leaf exact owner |
| `O07` | `resolution.stack.read` / `3011` | `R47` | cross-leaf exact owner |
| `O08` | `projection.definition.read` / `3012` | `R48` | cross-leaf exact owner |
| `O09` | `artifact.kind.list` / `3013` | `R49` | cross-leaf exact owner |
| `O10` | `artifact.instance.list` / `3014` | `R50` | cross-leaf exact owner |
| `O11` | `artifact.read` / `3015` | `R51` | cross-leaf exact owner |
| `O12` | `artifact.validate` / `3016` | `R52` | cross-leaf exact owner |
| `O13` | `artifact.render` / `3017` | `R53` | cross-leaf exact owner |
| `O14` | `intake.definition.read` / `3018` | `R54` | cross-leaf exact owner |
| `O15` | `intake.coverage.evaluate` / `3019` | `R55` | cross-leaf exact owner |
| `O16` | `intake.record.append` / `3020` | `R56` | cross-leaf exact owner |
| `O17` | `record.list` / `3021` | `R57` | cross-leaf exact owner |
| `O18` | `record.read` / `3022` | `R58` | cross-leaf exact owner |
| `O19` | `artifact.candidate.validate` / `3023` | `R59` | cross-leaf exact owner |
| `O20` | `artifact.candidate.append` / `3024` | `R60` | cross-leaf exact owner |
| `O21` | `artifact.approval.append` / `3025` | `R61` | cross-leaf exact owner |
| `O22` | `artifact.candidate.promote` / `3026` | `R62` | cross-leaf exact owner |
| `O23` | `posture.resolve` / `3027` | `R63` | cross-leaf exact owner |
| `O24` | `posture.recommendation.evaluate` / `3028` | `R64` | cross-leaf exact owner |
| `O25` | `posture.recommendation.append` / `3029` | `R65` | cross-leaf exact owner |
| `O26` | `posture.recommendation.acknowledge` / `3030` | `R66` | cross-leaf exact owner |
| `O27` | `posture.transition.apply` / `3031` | `R67` | cross-leaf exact owner |
| `O28` | `projection.create` / `3032` | `R68` | cross-leaf exact owner |
| `O29` | `resolution.escalation.request.append` / `3033` | `R69` | cross-leaf exact owner |
| `O30` | `resolution.escalation.disposition.append` / `3034` | `R70` | cross-leaf exact owner |
| `O31` | `memory.promotion.request.append` / `3035` | `R71` | cross-leaf exact owner |
| `O32` | `memory.promotion.disposition.append` / `3036` | `R72` | cross-leaf exact owner |
| `O33` | `snapshot.capture` / `3037` | `R73` | cross-leaf exact owner |
| `O34` | `snapshot.read` / `3038` | `R74` | cross-leaf exact owner |
| `O35` | `snapshot.delta` / `3039` | `R75` | cross-leaf exact owner |
| `O36` | `snapshot.project` / `3040` | `R76` | cross-leaf exact owner |
| `O37` | `snapshot.verify_current` / `3041` | `R77` | cross-leaf exact owner |
| `O38` | `snapshot.resolve_applicable` / `3042` | `R78` | cross-leaf exact owner |
| `O39` | `repository.setup.plan` / `3043` | `R79` | cross-leaf exact owner |
| `O40` | `repository.setup.apply` / `3044` | `—` | SDK-local operational setup state; no exact frozen cross-leaf semantic owner is named |
| `O41` | `repository.doctor` / `3045` | `—` | generic SDK composition over unnamed owners; no exact frozen cross-leaf owner is named |
| `O42` | `flow.resolve` / `3046` | `—` | the operation row names only the same-leaf flow owner; no exact frozen cross-leaf contract is invoked |
| `O43` | `pipeline.catalog.list` / `3047` | `—` | same-leaf pipeline owner; no exact frozen cross-leaf contract is invoked |
| `O44` | `pipeline.catalog.read` / `3048` | `—` | same-leaf pipeline owner; no exact frozen cross-leaf contract is invoked |
| `O45` | `pipeline.route.resolve` / `3049` | `—` | same-leaf pipeline owner; no exact frozen cross-leaf contract is invoked |
| `O46` | `pipeline.compile` / `3050` | `—` | same-leaf pipeline owner; no exact frozen cross-leaf contract is invoked |
| `O47` | `pipeline.capture.plan` / `3051` | `—` | same-leaf pipeline owner; no exact frozen cross-leaf contract is invoked |
| `O48` | `pipeline.capture.apply` / `3052` | `—` | same-leaf pipeline owner; no exact frozen cross-leaf contract is invoked |
| `O49` | `pipeline.handoff.emit` / `3053` | `—` | same-leaf pipeline owner; no exact frozen cross-leaf contract is invoked |
| `O50` | `pipeline.state.apply` / `3054` | `—` | same-leaf pipeline owner; no exact frozen cross-leaf contract is invoked |

## Named-section aggregate-fanout audit

This table derives the union for every H2/H3 as a diagnostic and stop gate. It does not make a heading an activation unit. Six or more total leaves is `broad-only`.

| Heading | Source leaf | Applicable routes | Source + target leaves | Count | Classification |
|---|---:|---|---|---:|---|
| `## Status` | 1 | `—` | `1` | 1 | selective-capable |
| `## Schema policy` | 1 | `—` | `1` | 1 | selective-capable |
| `### Uniform exact-definition identity` | 1 | `—` | `1` | 1 | selective-capable |
| `## Instance profile contract` | 1 | `R01,R02,R03,R04,R05,R06` | `1,2,3,4,5,6,11` | 7 | broad-only |
| `### Resolved instance profile` | 1 | `R01,R02,R03,R04,R05,R06` | `1,2,3,4,5,6,11` | 7 | broad-only |
| `## Schema registry entry contract` | 1 | `—` | `1` | 1 | selective-capable |
| `## Stable-role registry contract` | 1 | `—` | `1` | 1 | selective-capable |
| `## Artifact kind definition contract` | 1 | `R07` | `1,4` | 2 | selective-capable |
| `### Semantic capability contract` | 1 | `—` | `1` | 1 | selective-capable |
| `## Artifact instance descriptor contract` | 1 | `R08,R09` | `1,2,4` | 3 | selective-capable |
| `## Artifact intake definition contract` | 2 | `R10,R11` | `1,2,11` | 3 | selective-capable |
| `## Intake record and artifact candidate contracts` | 2 | `R12,R13` | `1,2,11` | 3 | selective-capable |
| `## Charter intake and canonical contract` | 2 | `R14,R15` | `1,2,3,4` | 4 | selective-capable |
| `## Artifact validation layers` | 2 | `R16,R17` | `2,3,4,11` | 4 | selective-capable |
| `## Vocabulary contract` | 3 | `R18,R19` | `1,3` | 2 | selective-capable |
| `## Context Resolution stack definition` | 3 | `R20` | `1,3` | 2 | selective-capable |
| `## Context Resolution envelope` | 3 | `R21` | `1,3` | 2 | selective-capable |
| `## Resolution escalation and memory promotion` | 3 | `R22` | `3,4,5` | 3 | selective-capable |
| `## Projection disclosure policy` | 4 | `R23,R24,R25` | `1,3,4,5` | 4 | selective-capable |
| `## Projection support evaluator definition` | 4 | `R26` | `1,4` | 2 | selective-capable |
| `## Projection definition` | 4 | `R27,R30` | `1,4,11` | 3 | selective-capable |
| `## Projection request` | 4 | `R28` | `1,3,4` | 3 | selective-capable |
| `## Projection result` | 4 | `R29,R31` | `1,3,4,11` | 4 | selective-capable |
| `## Snapshot capture policy` | 5 | `R32` | `3,5` | 2 | selective-capable |
| `## Context Memory Snapshot` | 5 | `R33` | `1,3,5` | 3 | selective-capable |
| `### Snapshot consistency` | 5 | `—` | `5` | 1 | selective-capable |
| `### Snapshot fingerprints` | 5 | `—` | `5` | 1 | selective-capable |
| `### Snapshot authority` | 5 | `—` | `5` | 1 | selective-capable |
| `## Snapshot delta` | 5 | `—` | `5` | 1 | selective-capable |
| `## Snapshot-grounding Projection definition` | 5 | `R34` | `3,4,5` | 3 | selective-capable |
| `## Snapshot projection request/result` | 5 | `R35` | `1,3,4,5` | 4 | selective-capable |
| `## Snapshot redaction and retention` | 5 | `R36,R37` | `3,4,5,7,11` | 5 | selective-capable |
| `### Shared HCM-0.3 extension rule` | 5 | `—` | `5` | 1 | selective-capable |
| `### HCM-0.3 required conformance scenarios` | 5 | `R36` | `4,5` | 2 | selective-capable |
| `## Project posture kernel and recommendation contracts` | 6 | `R38,R39,R40` | `1,2,5,6,11` | 5 | selective-capable |
| `### Posture trigger definition` | 6 | `—` | `6` | 1 | selective-capable |
| `### Freshness evaluation basis` | 6 | `R39` | `5,6,11` | 3 | selective-capable |
| `### Project posture kernel` | 6 | `R38,R39` | `1,2,5,6,11` | 5 | selective-capable |
| `### Posture evaluation policy` | 6 | `—` | `6` | 1 | selective-capable |
| `### Posture recommendation` | 6 | `—` | `6` | 1 | selective-capable |
| `### Posture transition` | 6 | `R38,R39,R40` | `1,2,5,6,11` | 5 | selective-capable |
| `## Optional synthesis-candidate contract` | 6 | `—` | `6` | 1 | selective-capable |
| `## Development orchestration contracts` | 7 | `R41` | `3,7` | 2 | selective-capable |
| `### Dispatch execution envelope` | 7 | `R41` | `3,7` | 2 | selective-capable |
| `### Parent-owned delegated-run evidence` | 7 | `—` | `7` | 1 | selective-capable |
| `### Review choreography` | 7 | `—` | `7` | 1 | selective-capable |
| `## HCM-0.4 crate ownership and dependency contract` | 8 | `—` | `8` | 1 | selective-capable |
| `## SDK ordinary-use-case contract` | 8 | `R42,R43,R44,R45,R46,R47,R48,R49,R50,R51,R52,R53,R54,R55,R56,R57,R58,R59,R60,R61,R62,R63,R64,R65,R66,R67,R68,R69,R70,R71,R72,R73,R74,R75,R76,R77,R78,R79` | `1,2,3,4,5,6,8` | 7 | broad-only |
| `## Operation definition contract` | 8 | `R80,R81` | `8,9,10` | 3 | selective-capable |
| `## Capability bootstrap and catalog discovery` | 8 | `—` | `8` | 1 | selective-capable |
| `### Preselection admission-refusal envelope` | 8 | `—` | `8` | 1 | selective-capable |
| `## Transport request contract` | 9 | `R82,R83,R89` | `3,5,8,9,10` | 5 | selective-capable |
| `## Transport response and expected-outcome contract` | 9 | `R84,R85` | `3,5,8,9` | 4 | selective-capable |
| `## DTO and JSON Schema generation contract` | 9 | `R86` | `8,9` | 2 | selective-capable |
| `## CLI JSON contract` | 9 | `R87` | `8,9` | 2 | selective-capable |
| `## Tauri adapter contract` | 9 | `R88` | `8,9` | 2 | selective-capable |
| `## Substrate integration contracts` | 10 | `R90` | `8,9,10` | 3 | selective-capable |
| `### Transitional bundled-CLI bridge` | 10 | `R90` | `8,9,10` | 3 | selective-capable |
| `### Permanent published-Rust boundary` | 10 | `—` | `10` | 1 | selective-capable |
| `## Published Rust API proof plan` | 10 | `R91` | `7,8,10,11` | 4 | selective-capable |
| `## Contract record` | 11 | `—` | `11` | 1 | selective-capable |
| `## Evidence record` | 11 | `R92` | `3,5,11` | 3 | selective-capable |
| `## Verdict contract` | 11 | `—` | `11` | 1 | selective-capable |
| `## Gate contract` | 11 | `R93` | `4,5,11` | 3 | selective-capable |
| `## Dock capability manifest` | 11 | `—` | `11` | 1 | selective-capable |
| `## Dock request/result` | 11 | `R94,R95,R96,R97` | `1,3,4,5,11` | 5 | selective-capable |
| `## Public API proof gates` | 11 | `R98` | `10,11` | 2 | selective-capable |
| `### CLI bridge gate` | 11 | `R98` | `10,11` | 2 | selective-capable |
| `### Published Rust API gate` | 11 | `R98` | `10,11` | 2 | selective-capable |
| `## Schema compatibility posture` | 11 | `—` | `11` | 1 | selective-capable |

## Named references reviewed without a route

These groups were inspected but do not create an authority edge. Keeping them explicit prevents omitted dependencies and unjustified extra routes.

| Group | Source leaf / frozen lines | Classification | Reason |
|---|---|---|---|
| `N01` | leaf 1 / `5-7` | non-layout-dependent | `Status` summarizes frozen/current/future catalog state; it invokes no summarized owner for a leaf-1 task. |
| `N02` | leaf 1 / `19, 34, 246` | non-layout-dependent | generic future policy, signal, evidence, and later-phase refs name no single frozen leaf owner. |
| `N03` | leaf 2 / `889, 1283-1301` | same-leaf | Charter intake owns constitutional posture fields; derived posture-kernel consumers route from leaf 6, so generic Charter posture nouns do not create a reverse edge. |
| `N04` | leaf 3 / `1462, 1590-1603` | same-leaf | escalation/promotion policy and terminal-record behavior are owned in leaf 3; only exact external source types route. |
| `N05` | leaf 4 / `1618-1620, 1747` | non-layout-dependent | generic source-kind allowlists do not select Snapshot authority; exact selected definitions/DTOs route separately. |
| `N06` | leaf 5 / `2038-2088` | same-leaf | snapshot payload arrays observe refs/statuses without evaluating their external semantics; retention holds route separately. |
| `N07` | leaf 8 / `3056` | non-layout-dependent | absent future contract/dock/evidence/verdict/gate operations are an HCM-0.5 extension boundary, not current leaf-11 authority. |
| `N08` | leaves 9-10 / `3491-3517, 3594, 3616` | non-layout-dependent | generic DTO evidence/artifact nouns and external proof IDs do not add semantic owner routes; exact fields do. |
| `N09` | leaf 8 / `3005` | same-leaf | `capabilities.describe` is owned by the same-leaf capability/bootstrap catalog. |
| `N10` | leaf 8 / `3044-3046` | non-layout-dependent | repository apply/doctor and `flow.resolve` name SDK/flow composition but no exact frozen cross-leaf contract owner. |
| `N11` | leaf 8 / `3047-3054` | same-leaf | pipeline operations name the same-leaf pipeline owner; no exact frozen cross-leaf domain contract is invoked. |

## Positional-reference inventory

Occurrence ordinals restart on each frozen source line. The cue is evidence only; the frozen Git blob is authoritative.

| Occurrence | Token | Classification | Route | Frozen cue |
|---|---|---|---|---|
| `L1:5.1` | `later` | 2 non-layout-dependent | `—` | They are implementation authority for later slice packets, not published API guarantees or evidence t |
| `L1:34.1` | `later` | 2 non-layout-dependent | `—` | quirement is admissible even when its later behavioral contract is not yet frozen only if its exact v |
| `L1:127.1` | `later` | 2 non-layout-dependent | `—` | \| \`dock_requirement_refs\` \| later HCM-0.5 authority \| root: explicit empty; child omission |
| `L1:127.2` | `later` | 2 non-layout-dependent | `—` | ist replaces whole \| exact refs after later contract lands \| no executable validator in a profile \| |
| `L1:246.1` | `later` | 2 non-layout-dependent | `—` | \| remaining later-phase refs/lists \| their named profile field/later contra |
| `L1:246.2` | `later` | 2 non-layout-dependent | `—` | efs/lists \| their named profile field/later contract owns them \| explicit null/empty as shown until l |
| `L1:246.3` | `later` | 2 non-layout-dependent | `—` | \| explicit null/empty as shown until later freezes \| later contract compatibility \| no premature doc |
| `L1:246.4` | `later` | 2 non-layout-dependent | `—` | /empty as shown until later freezes \| later contract compatibility \| no premature dock behavior \| |
| `L1:506.1` | `above` | 1 same-leaf | `—` | The \`project_context\` descriptor above is a shape example only. It does not select a shipped kin |
| `L2:915.1` | `later` | 2 non-layout-dependent | `—` | ence-only non-field coverage awaits a later typed disposition contract; no implicit free-text placeme |
| `L2:1254.1` | `later` | 2 non-layout-dependent | `—` | deterministic normalized record \| no later candidate/approval/promotion forward link or mutation \| |
| `L2:1316.1` | `later` | 2 non-layout-dependent | `—` | quired external evidence gates when a later contract demands them. Passing a later layer cannot waive |
| `L2:1316.2` | `later` | 2 non-layout-dependent | `—` | ater contract demands them. Passing a later layer cannot waive an earlier failure. Unknown required s |
| `L2:1316.3` | `earlier` | 2 non-layout-dependent | `—` | Passing a later layer cannot waive an earlier failure. Unknown required semantics fail closed; warnings |
| `L3:1507.1` | `later` | 2 non-layout-dependent | `—` | alized; omission never means “inherit later.” A child cites at most one exact parent envelope and rep |
| `L3:1596.1` | `above` | 2 non-layout-dependent | `—` | missing condition; authority resolves above current bound \| no “need more context” without a concrete |
| `L4:1627.1` | `earlier` | 2 non-layout-dependent | `—` | ever covered merely because it shares earlier path segments; a request for that exact retained field ev |
| `L4:1760.1` | `below` | 1 same-leaf | `—` | exact inclusion/omission truth table below; every potentially omitted target pointer is structurally |
| `L5:2153.1` | `below` | 2 non-layout-dependent | `—` | quence is the greatest eligible value below current \| no self/future/cycle/wrong-stream/wrong-boundar |
| `L5:2155.1` | `below` | 1 same-leaf | `—` | state identity \| none \| normalization below \| no trigger/time/snapshot-ID sensitivity \| |
| `L5:2156.1` | `below` | 1 same-leaf | `—` | ecord identity \| none \| normalization below \| no mutable record after publication \| |
| `L5:2509.1` | `earlier` | 2 non-layout-dependent | `—` | ginal pointer's subtree; it may share earlier path segments but is never equal to or a descendant of \`o |
| `L5:2556.1` | `later` | 2 non-layout-dependent | `—` | ned identity, retained descendant, or later mutation \| |
| `L5:2571.1` | `later` | 2 non-layout-dependent | `—` | Later implementation packets must turn these design examples in |
| `L6:2591.1` | `later` | 2 non-layout-dependent | `—` | rity path. If scoped posture state is later required, it needs a separate contract and migration. |
| `L6:2816.1` | `above` | 1 same-leaf | `—` | y input \| none \| normalization stated above \| no wall-clock lookup outside the record \| |
| `L6:2822.1` | `above` | 1 same-leaf | `—` | ntity \| none \| explicit normalization above; same inputs/basis replay exactly \| no ambient clock or p |
| `L6:2828.1` | `above` | 1 same-leaf | `—` | dentity \| none \| normalization stated above includes trigger/rule/notification closure \| no changed p |
| `L6:2831.1` | `above` | 1 same-leaf | `—` | ually exclusive null fields as stated above \| exact current trigger pair or exact policy rule fingerp |
| `L6:2853.1` | `later` | 2 non-layout-dependent | `—` | Required conformance scenarios for later implementation packets: |
| `L6:2874.1` | `later` | 2 non-layout-dependent | `—` | If later approved: |
| `L7:2950.1` | `earlier` | 2 non-layout-dependent | `—` | and subject fingerprints must match. Earlier records and schemas remain immutable historical evidence. |
| `L7:2952.1` | `earlier` | 2 non-layout-dependent | `—` | t record and rebuilds \`ledger.jsonl\`. Earlier review manifests remain immutable identities of supersede |
| `L7:2952.2` | `later` | 2 non-layout-dependent | `—` | are not incorrectly compared with the later repaired tree. |
| `L8:2984.1` | `later` | 2 non-layout-dependent | `—` | /capture/handoff/state sequencing and later sequencing of contract use cases \| \`handbook-engine\`; \`ha |
| `L8:2999.1` | `below` | 1 same-leaf | `—` | The frozen inventory below is data-oriented: custom kind IDs, instance IDs, vocabula |
| `L8:3056.1` | `later` | 2 non-layout-dependent | `—` | freezes their semantics. Adding them later extends this catalog; it cannot change the owner/DTO/tran |
| `L8:3058.1` | `earlier` | 2 non-layout-dependent | `—` | print; no step mutates or replaces an earlier record. Posture evaluation is pure and always returns one |
| `L8:3107.1` | `above` | 1 same-leaf | `—` | \| \`owner_crate\` \| owner matrix above \| exact target owner; SDK composition does not change sem |
| `L8:3110.1` | `below` | 1 same-leaf | `—` | st match the legal-combination matrix below \| |
| `L8:3115.1` | `below` | 1 same-leaf | `—` | g and conditional writes are declared below, and read-only operations require \`[]\` \| |
| `L8:3158.1` | `above` | 1 same-leaf | `—` | nt, returns the recovery-hold refusal above. A compare-and-write retry with the original key replays |
| `L8:3162.1` | `later` | 2 non-layout-dependent | `—` | _frame, adapter_phase}\` record or its later \`release_pending\` replacement including the complete term |
| `L8:3164.1` | `below` | 3 cross-leaf | `R80` | tion request-identity variant defined below may enter a private open, idempotency lookup, reservation |
| `L8:3164.2` | `later` | 2 non-layout-dependent | `—` | eserving the released tombstone, so a later delivery attempt may use a new hold ID as required. Direc |
| `L8:3171.1` | `above` | 1 same-leaf | `—` | key_fingerprint\` is the scoped digest above for a mutation and null for a read-only operation. Values |
| `L8:3174.1` | `above` | 1 same-leaf | `—` | the complete schema-valid open frame above after deleting exactly \`idempotency_key\` and \`open_finger |
| `L8:3186.1` | `above` | 1 same-leaf | `—` | sponse-schema/request-payload binding above. The duplicate/binding checks occur after the individual |
| `L8:3208.1` | `later` | 2 non-layout-dependent | `—` | . After either release outcome, every later execution attempt must first fsync a new unique hold ID a |
| `L8:3218.1` | `later` | 2 non-layout-dependent | `—` | e content-addressed catalog snapshot; later pages read that same snapshot despite concurrent registry |
| `L8:3220.1` | `below` | 1 same-leaf | `—` | ptor-bound admission-refusal envelope below rather than fabricating trusted ordinary operation fields |
| `L8:3312.1` | `below` | 1 same-leaf | `—` | nly the winning field evidence listed below; later-selection fields are omitted, not set to null or o |
| `L8:3312.2` | `later` | 2 non-layout-dependent | `—` | winning field evidence listed below; later-selection fields are omitted, not set to null or opportun |
| `L8:3323.1` | `below` | 1 same-leaf | `—` | , and sorted in the fixed field order below \| |
| `L8:3334.1` | `later` | 2 non-layout-dependent | `—` | ytes outside the parsed value and any later field are excluded. |
| `L8:3336.1` | `later` | 2 non-layout-dependent | `—` | ant and never includes evidence for a later step. \`definition_pin_missing\` is exclusively an absent o |
| `L8:3342.1` | `above` | 1 same-leaf | `—` | correlation and no-response matrices above. |
| `L9:3385.1` | `below` | 1 same-leaf | `—` | refusal under the total response rule below; it never re-enters the admission envelope. |
| `L9:3386.1` | `above` | 3 cross-leaf | `R82` | ric request-schema identity preflight above precedes selection and ledger lookup. The API context mus |
| `L9:3391.1` | `above` | 3 cross-leaf | `R82` | level RFC 8785/SHA-256 closure frozen above. \`request_payload_fingerprint\` covers exactly \`{repositor |
| `L9:3393.1` | `above` | 1 same-leaf | `—` | ind: normalized\` is the valid closure above and is the only variant usable for idempotency scope, loo |
| `L9:3407.1` | `later` | 2 non-layout-dependent | `—` | ive ordinary \`mismatch\` comparator. A later bridge open/request tuple mismatch remains adapter-local |
| `L9:3407.2` | `above` | 1 same-leaf | `—` | ils schema over the constant evidence above; wrong-type extensions select \`extensions_malformed\`, and |
| `L9:3407.3` | `below` | 1 same-leaf | `—` | \`request_id_invalid\` evidence defined below, with its raw value deleted before the object digest. Dir |
| `L9:3470.1` | `above` | 1 same-leaf | `—` | by the total postselection algorithm above; failures before bounded UTF-8/JSON-object normalization |
| `L9:3470.2` | `above` | 3 cross-leaf | `R84` | n-null-fingerprint admission envelope above. \`idempotency\` is a closed discriminated union: |
| `L9:3476.1` | `above` | 3 cross-leaf | `R84` | t equal to the scoped closure defined above, original result null on first execution, and original re |
| `L9:3480.1` | `earlier` | 2 non-layout-dependent | `—` | lying adapter failed while serving an earlier validation step. No other stage/status/category triple is |
| `L9:3482.1` | `above` | 3 cross-leaf | `R84` | ry-promotion stale-terminal exception above establishes after structural/admission/actor-authority va |
| `L9:3498.1` | `below` | 1 same-leaf | `—` | table catalog-snapshot digest defined below \| |
| `L9:3504.1` | `above` | 1 same-leaf | `—` | /stage/Problem-binding fields defined above; \`established\` exposes only scoped \`idempotency_key_finge |
| `L9:3511.1` | `above` | 1 same-leaf | `—` | e nullable/omittable only where named above and never enter machine decisions. Unknown enum values, u |
| `L9:3527.1` | `later` | 2 non-layout-dependent | `—` | 7. OpenAPI may later describe an HTTP adapter but cannot become authority for |
| `L10:3580.1` | `below` | 1 same-leaf | `—` | e protocol-control entry point frozen below; |
| `L10:3590.1` | `above` | 3 cross-leaf | `R90` | stdin under the 64-KiB admission rule above. It is excluded from public help, the ordinary operation |
| `L10:3592.1` | `above` | 3 cross-leaf | `R90` | zed ceiling and sensitive-input rules above, and retries only that byte-identical semantic request wi |
| `L10:3592.2` | `earlier` | 2 non-layout-dependent | `—` | ete private record\`; no step may move earlier. The bridge never reopens a released hold, reconstructs a |
| `L10:3592.3` | `later` | 2 non-layout-dependent | `—` | ersistence, namespace retirement, and later result compaction. |
| `L10:3602.1` | `below` | 1 same-leaf | `—` | the published API and real-seam proof below pass. At that point the Tier-2 adapter is removed from th |
| `L10:3606.1` | `following` | 1 same-leaf | `—` | tended SDK or owner API must pass the following ordered proof: |
| `L10:3610.1` | `earlier` | 2 non-layout-dependent | `—` | published node after revalidating all earlier checksums. |
| `L11:3745.1` | `above` | 3 cross-leaf | `R98` | itional bundled-CLI bridge** contract above: exact binary version/checksum, exact operation/schema/ca |
| `L11:3749.1` | `above` | 3 cross-leaf | `R98` | ete **Published Rust API proof plan** above passes, including packaged-artifact isolation, exact crat |
| `L11:3751.1` | `earlier` | 2 non-layout-dependent | `—` | The CLI bridge may ship earlier, but it cannot satisfy this Rust API gate. |

## Required mechanical proof

The future `verify_contract_catalog.py` must derive rather than trust this inventory and fail closed unless all of the following hold:

1. the eleven frozen spans cover the baseline body exactly once and all 48 H2/22 H3 headings are assigned;
2. the positional scan derives exactly 84 occurrences and every occurrence has one valid classification;
3. every `R01`-`R98` trigger/ordered target list exists exactly once, every target anchor resolves, and every `N01`-`N11` remains a justified non-route;
4. all `O01`-`O50` operation fixtures derive exactly, including per-operation targets and the 12 explicit non-routes;
5. every leaf routing block contains its exact trigger-indexed route rows in global order; a heading alone activates none;
6. aggregate fanout derives for all 48 H2/22 H3 headings; the three seven-leaf heading unions remain `broad-only`, every other heading stays below six, and every normal exact route stays at source plus at most three siblings;
7. deleting any route, breaking any target, overlapping an exact source-trigger unit, adding an unjustified route, or injecting an unclassified positional/named dependency fails;
8. adding or changing a cross-leaf dependency, operation fixture, heading aggregate, or broad-only classification without updating this audit and reviewed planning topology fails.
