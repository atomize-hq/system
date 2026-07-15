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
| 8 | `contracts/08-sdk-operations-and-capability-discovery.md` | `HCM-0.4 crate ownership and dependency contract` through `Capability bootstrap and catalog discovery` | crate ownership, SDK use cases, operations, bootstrap, and discovery authority; routing metadata exposes its named HCM-0.2/HCM-0.3 domain owners |
| 9 | `contracts/09-machine-transport-and-adapter-contracts.md` | `Transport request contract` through `Tauri adapter contract` | requests, responses, DTO/schema generation, CLI JSON, and Tauri transport authority; routing metadata preserves its operation/bootstrap dependencies in leaf 8 |
| 10 | `contracts/10-substrate-integration-and-publication.md` | `Substrate integration contracts` through `Published Rust API proof plan` | transitional/permanent Substrate boundaries and publication proof authority; routing metadata preserves its operation/bootstrap and transport/CLI dependencies in leaves 8 and 9 |
| 11 | `contracts/11-contract-evidence-gates-and-docks.md` | `Contract record` through `Schema compatibility posture` | contract/evidence/verdict/gate/dock authority plus public proof gates and compatibility posture; routing metadata preserves its Resolution and public-proof dependencies in leaves 3 and 10 |

Each leaf begins with `# <leaf title>` and one blank line. Leaves 8-11 then contain one verifier-approved routing-only dependency block bounded by `<!-- hcm-0.9-routing-only:start -->` and `<!-- hcm-0.9-routing-only:end -->`, followed by one blank line. The block contains only the exact Markdown links derived from the dependency table below, in table order with duplicate targets removed at first occurrence. Its heading is exactly `Frozen cross-leaf authority dependencies:` and each link label is the target's frozen heading text. It is non-normative navigation, not contract content. Leaves 1-7 have no block because their layout-dependent references remain within their assigned spans and no fixed cross-leaf dependency was derived. The semantic payload begins at the first H2 and remains byte-for-byte frozen. The verifier validates and strips the complete scaffold before parity comparison.

No block may add an inferred semantic rule or force every advertised dependency into every task's context. Orchestration uses the triggering source column below: it loads a target only when that source contract is in scope, while a topology/parity review loads every source and target. Adding, removing, or reordering a dependency requires a reviewed packet change before execution continues.

### Exact cross-leaf dependency table

All paths below are relative to `contracts/`. A slash-separated target list preserves the listed order.

| Source leaf | Triggering frozen source | Required exact targets |
|---|---|---|
| `08-sdk-operations-and-capability-discovery.md` | `SDK ordinary-use-case contract` and `Operation definition contract` use of HCM-0.2/HCM-0.3 domain identities and authority | `01-schema-profile-and-artifact-registry.md#schema-policy` / `01-schema-profile-and-artifact-registry.md#artifact-kind-definition-contract` / `02-intake-charter-and-validation.md#artifact-intake-definition-contract` / `03-vocabulary-and-context-resolution.md#context-resolution-envelope` / `04-projection-contracts.md#projection-definition` / `05-snapshot-memory-contracts.md#context-memory-snapshot` / `06-posture-and-synthesis-contracts.md#project-posture-kernel-and-recommendation-contracts` |
| `09-machine-transport-and-adapter-contracts.md` | all five leaf-9 transport/DTO/CLI/Tauri H2 contracts use operation selection, generic-request admission, two-level request identity, or typed SDK operation authority | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract` / `08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` |
| `10-substrate-integration-and-publication.md` | `Substrate integration contracts` bridge preflight, 64-KiB admission, recovery-control, request/response validation, and CLI invocation rules | `08-sdk-operations-and-capability-discovery.md#operation-definition-contract` / `08-sdk-operations-and-capability-discovery.md#capability-bootstrap-and-catalog-discovery` / `09-machine-transport-and-adapter-contracts.md#transport-request-contract` / `09-machine-transport-and-adapter-contracts.md#transport-response-and-expected-outcome-contract` / `09-machine-transport-and-adapter-contracts.md#cli-json-contract` |
| `10-substrate-integration-and-publication.md` | `Published Rust API proof plan` crate/publication ordering and SDK-consumer boundary | `08-sdk-operations-and-capability-discovery.md#hcm-04-crate-ownership-and-dependency-contract` / `08-sdk-operations-and-capability-discovery.md#sdk-ordinary-use-case-contract` |
| `11-contract-evidence-gates-and-docks.md` | `Dock request/result` Resolution-envelope input | `03-vocabulary-and-context-resolution.md#context-resolution-envelope` |
| `11-contract-evidence-gates-and-docks.md` | `Public API proof gates` references to the complete bridge contract and published-Rust plan “above” | `10-substrate-integration-and-publication.md#transitional-bundled-cli-bridge` / `10-substrate-integration-and-publication.md#published-rust-api-proof-plan` |

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

The split must preserve referential structure as well as payload bytes. The verifier inventories frozen prose outside code fences for positional terms including `above`, `below`, `preceding`, `following`, `earlier`, and `later`, plus named dependencies on domain lineage, operation/admission rules, transport/CLI rules, Resolution input, bridge authority, and publication proof. Each occurrence or dependency is classified as:

1. a same-leaf reference whose target remains in the owning leaf;
2. non-positional prose whose meaning does not depend on document layout; or
3. a cross-leaf reference with an exact declared routing-only dependency and triggering source.

The exact dependency table is the reviewed audit result. It captures leaf 8's named HCM-0.2/HCM-0.3 authority, leaf 9's generic-request/admission and two-level identity authority in leaf 8, leaf 10's operation/bootstrap plus transport/CLI authority in leaves 8 and 9, and leaf 11's Resolution plus public-proof authority in leaves 3 and 10. An unclassified referential dependency, missing/broken target, wrong source trigger, unjustified extra dependency, or new cross-leaf reference fails closed.

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

1. Use the stable index only to discover the owning leaf.
2. Resolve task concepts and named H2/H3 sections to the minimum leaf set, then apply only dependency-table rows whose triggering source is in scope.
3. Load exact source and required target path/anchor authority. Do not load unrelated advertised dependencies or all leaves unless the task changes topology, spans multiple owners, or requires a complete-catalog proof.
4. HCM-0.5 selects `contracts/11-contract-evidence-gates-and-docks.md` and any explicitly named sibling leaf; it does not reopen the monolith.

### Internal dispatch and subject manifests

- `authority_refs` and `contracts_and_gates` name exact leaf paths/anchors.
- A dispatch subject manifest contains files whose bytes are under review, not every authority file read for context.
- A leaf-local semantic change includes that leaf plus any changed routing/proof files. It does not include unrelated leaves.
- Complete-catalog topology/parity review includes the stable index, all eleven leaves, the verifier, and every changed mutable routing/proof file.
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
12. Every frozen positional and named contract dependency is classified; leaves 8-11 have exactly the source-triggered routing targets in the dependency table; omitted, broken, reordered, unjustified, or unclassified cross-leaf dependencies fail.
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

The planning/control-pack registration that approves this packet has a separate budget: at most three complete-subject planning reviews and at most two remediation rounds. It stops immediately on CLEAN. If Review 3 retains valid Critical or Required findings, the parent writes the user-directed partial handoff required by the planning session and does not authorize execution.

## Acceptance criteria

HCM-0.9 execution is complete only when:

- [ ] all eleven exact leaf files exist with the specified contiguous section spans;
- [ ] all 48 frozen H2 sections map exactly once and all 22 frozen H3 headings stay under the same owning H2 span;
- [ ] the ordered leaf semantic payload is byte-identical to the frozen baseline body;
- [ ] `05` is a stable routing/compatibility index with all frozen H2/H3 aliases and no duplicated normative payload;
- [ ] current mutable control-pack/orchestration/dispatch/proof/handoff references use exact minimum leaf selection;
- [ ] every pre-existing handoff/dispatch byte is unchanged;
- [ ] every frozen positional or named contract dependency remains same-leaf, non-layout-dependent, or backed by the exact verified source-triggered cross-leaf routing dependency table;
- [ ] links, anchors, fences, parseable YAML/JSON, semantic assertions, manifests, handoffs, archive, parity, scope, and Git gates pass;
- [ ] no Rust/runtime/public API/schema-version/proof-promotion/HCM-0.5/unrelated change occurred;
- [ ] a fresh isolated complete-subject reviewer returns CLEAN within the execution budget;
- [ ] the reviewed execution commit exists;
- [ ] the separate mechanical v1.2 parent handoff/ledger closeout validates;
- [ ] the closeout claims structural parity/selective routing only and does not claim new contract semantics or runtime completion.

## Open questions

None. Any request to change the topology, baseline, semantic payload, or review budget requires an explicit reviewed packet change before execution continues.
