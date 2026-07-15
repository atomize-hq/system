# HCM-0.9 Specification: Mechanical Contract-Catalog Decomposition

**Status:** redesigned planning authority; execution not started
**Phase / slice:** `HCM-0` / `HCM-0.9`  
**Sequence:** immediately after completed `HCM-0.4`; before `HCM-0.5`  
**Change class:** documentation/control maintenance only  
**Execution packet:** [`tasks/plan.md`](tasks/plan.md) and [`tasks/todo.md`](tasks/todo.md)

## Human redesign decision

The 2026-07-15 human decision abandons and supersedes the proposed
machine-readable trigger, route-inventory, co-activation, operation-fixture,
fanout, and semantic-dependency completeness design. HCM-0.9 will not build an
automatic semantic routing engine.

The earlier planning subjects, dispatches, handoffs, fingerprints, and commits
remain immutable historical evidence. They are not current planning authority.
The exact preservation boundary is recorded in
[`evidence/dependency-audit.md`](evidence/dependency-audit.md).

## Objective

Decompose the frozen `05-contracts-schemas-and-gates.md` catalog into eight
cohesive canonical leaf documents without changing any frozen payload byte.
Retain `05-contracts-schemas-and-gates.md` as the stable discovery and
historical-anchor compatibility index. Future slice packets and dispatches must
explicitly list the leaf files and anchors they need; no system infers semantic
dependencies or expands that list automatically.

The outcome is a smaller explicit context surface and clearer file ownership,
not a contract revision. Planning approval does not authorize decomposition.

## Frozen baseline

| Property | Frozen value |
|---|---|
| Baseline commit | `214a5b8eb182fce74478df49d4f55d226d65fdf5` |
| Baseline path | `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` |
| Full-file SHA-256 | `c7f61db209a81ba20690f365b4069dd01f11e395335bfa10d2ce21143cc2985d` |
| Baseline body SHA-256 | `487d698979131bbea319c394d39c27dbd270dc5953f714f30edfe8b7f9ae9202` |
| Baseline size | 3,757 lines; 343,081 bytes |
| Baseline headings | one H1 plus 48 H2 and 22 H3 headings |
| Frozen authority | completed HCM-0.2, HCM-0.3, and HCM-0.4 contract semantics |

Execution retrieves the baseline from Git and stops unless the full-file digest
matches:

```bash
git show 214a5b8eb182fce74478df49d4f55d226d65fdf5:docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md \
  | shasum -a 256
```

## Scope

HCM-0.9 execution may only:

1. add the eight exact canonical leaf documents in the topology below;
2. move each frozen H2 span and all subordinate bytes exactly once;
3. replace the current `05` body with a stable index and compatibility aliases;
4. add one slice-local mechanical decomposition verifier;
5. update mutable control-pack and dispatch guidance to require explicit leaf
   selection by future packets and dispatches;
6. prove byte parity, heading/anchor/link/fence/example parity, immutable
   historical evidence, manifest correctness, and scope;
7. obtain fresh independent execution review under the later execution packet;
8. commit reviewed execution separately from mechanical handoff/ledger closeout.

## Invariants

1. **Zero semantic delta:** after removing each new leaf H1 and its following
   blank line, concatenating leaf payloads in order equals the frozen baseline
   body byte-for-byte.
2. **Eight contiguous owners:** every frozen H2 section belongs to exactly one
   of eight leaves and each leaf owns one contiguous frozen span.
3. **No split subordinate structure:** H3 content and fenced blocks remain with
   their owning H2; no fence crosses a leaf boundary.
4. **Heading preservation:** every frozen H2/H3 heading is copied byte-for-byte
   and retains its generated leaf anchor.
5. **Stable path and H1:** `05-contracts-schemas-and-gates.md` remains present
   with the unchanged H1 `Contracts, Schemas, and Gates`.
6. **Historical anchors:** every frozen H2/H3 fragment at the stable `05` path
   resolves through one compatibility alias/link to the same heading in its
   owning leaf.
7. **Index-only metadata:** the stable index contains discovery metadata,
   ordered leaf links, heading ownership, and compatibility aliases; it does
   not duplicate normative payload or fenced examples.
8. **Explicit selection only:** future slice packets and dispatches enumerate
   exact leaf paths/anchors themselves. There is no trigger registry, semantic
   dependency graph, inferred closure, co-activation rule, operation fixture,
   fanout threshold, or transitive leaf loading.
9. **Immutable history:** no handoff record or dispatch that exists at execution
   start is edited, renamed, deleted, or re-fingerprinted.
10. **No semantic correction:** defects discovered while moving payload are
    recorded for later work; their bytes remain unchanged in HCM-0.9.
11. **No runtime claim:** `PG-CATALOG-01` may close only for mechanical document
    decomposition. No runtime or public-interface gate changes state.
12. **No next-slice activation:** HCM-0.5 remains unstarted.

## Non-goals

- automatic semantic routing or dependency completeness;
- triggers, route IDs, co-activation, operation fixtures, fanout computation,
  broad-use classification, or leaf-transitive selection;
- Rust, Cargo, runtime, CLI, Tauri, Substrate, SDK, or public API changes;
- new or changed contract fields, defaults, examples, validation rules, schema
  versions, operation IDs, DTOs, lifecycle states, or dock semantics;
- correction, clarification, simplification, reformatting, or reordering of
  frozen semantic payload;
- HCM-0.5 planning or execution;
- rewriting historical handoffs, dispatches, or evidence commits;
- opportunistic cleanup outside the exact mutable surface.

## Canonical leaf topology

All leaf paths are relative to
`docs/specs/handbook-contract-membrane/contracts/`. Exact line spans, digests,
and heading assignments are frozen in
[`evidence/decomposition-inventory.md`](evidence/decomposition-inventory.md).

| Order | Canonical leaf | Frozen section span | Purpose |
|---:|---|---|---|
| 1 | `01-profile-and-artifact-foundations.md` | `Status` through `Artifact instance descriptor contract` | schema policy, profiles, registries, artifact kinds/capabilities, and instance descriptors |
| 2 | `02-intake-canonical-and-validation.md` | `Artifact intake definition contract` through `Artifact validation layers` | intake, candidates, Charter canonicalization, and validation |
| 3 | `03-context-resolution-and-projection.md` | `Vocabulary contract` through `Projection result` | vocabulary, Resolution, escalation, disclosure, and Projection contracts |
| 4 | `04-snapshot-memory.md` | `Snapshot capture policy` through `Snapshot redaction and retention` | Snapshot capture, consistency, deltas, projections, redaction, and retention |
| 5 | `05-posture-and-synthesis.md` | `Project posture kernel and recommendation contracts` through `Optional synthesis-candidate contract` | posture evaluation/recommendation/transition and optional synthesis candidates |
| 6 | `06-orchestration-and-sdk.md` | `Development orchestration contracts` through `Capability bootstrap and catalog discovery` | delegated orchestration, crate ownership, SDK operations, and discovery |
| 7 | `07-transport-adapters-and-publication.md` | `Transport request contract` through `Published Rust API proof plan` | transport, DTO/schema generation, adapters, Substrate boundaries, and publication proof |
| 8 | `08-contract-evidence-docks-and-gates.md` | `Contract record` through `Schema compatibility posture` | contract/evidence/verdict/gate records, docks, public proof gates, and compatibility |

Each leaf adds exactly one H1 and one following blank line before its first
frozen H2. It adds no routing block or other pre-payload scaffold.

## Stable-index behavior

The new stable `05` index must contain:

1. the unchanged H1;
2. a statement that canonical payload lives in the eight leaves;
3. the ordered leaf table above;
4. an exact H2/H3 ownership table generated from the frozen baseline;
5. exactly one compatibility anchor and link for every frozen H2/H3 fragment;
6. instructions that future packets/dispatches explicitly select leaf paths and
   anchors and that the index does not infer additional authority.

The index is appropriate for discovery. It is not a default authority bundle,
an automatic router, or a duplicate contract catalog.

## Explicit leaf selection and subject manifests

Leaf selection and review-subject identity are distinct:

- A future slice packet states the exact canonical leaf paths/anchors it needs.
- A delegated dispatch repeats that explicit selection in `authority_refs`
  and/or `contracts_and_gates`. The author decides the list from the packet;
  no machine adds semantic dependencies.
- `subject_manifest` contains only files whose current bytes are under review.
- A changed index, leaf, verifier, packet, proof file, or routing-guidance file
  is manifested when its bytes are part of the review subject.
- Unchanged contextual authority read by a reviewer is never manifested merely
  because it was read; it stays in `authority_refs` and/or
  `contracts_and_gates`.
- Complete-decomposition review manifests the changed index, eight changed
  leaves, verifier, and every other changed/reviewed file. It does not add
  unchanged contextual authority by default.
- Manifests remain sorted, unique, repository-relative, and replayable at their
  recorded aggregate fingerprint.

At each review boundary, manifest completeness is mechanical:

1. derive the changed path set from `git diff --name-only` plus untracked files
   relative to the persisted execution-start HEAD;
2. require the manifest path set to equal that changed path set after removing
   only additive HCM-0.9 review dispatches under `handoffs/dispatches/`;
3. the current review dispatch is necessarily exempt because its manifest and
   fingerprint are computed before that dispatch can bind them; earlier
   additive review dispatches are immutable orchestration evidence, schema-
   validated separately, and receive the same narrow exemption;
4. do not exempt `execution-start.json`, the index, leaves, verifier, mutable
   guidance/proof files, or any other changed planning/execution byte;
5. before the reviewed execution commit, require staged paths to equal the
   reviewed manifest paths plus the exact additive review-dispatch exemptions;
6. mechanical closeout handoff/ledger bytes are created only after the reviewed
   execution commit and are validated/committed separately.

An intentionally reviewed unchanged file may be manifested only when the
dispatch explicitly says that its bytes, rather than its authority, are under
review. It does not excuse an omitted changed path.

## Mutable execution surface

Execution re-inventories live references before editing. Expected mutable
surfaces are:

| Path | Required execution update |
|---|---|
| `00-README.md` | identify `05` as stable index and expose the eight canonical leaves |
| `02-semantic-model.md` | replace mutable monolith refs with explicit owning leaf refs where relevant |
| `03-seam-crosswalk.md` | record landed decomposition without a semantic-routing claim |
| `04-phase-slice-map.md` | bind HCM-0.9 closeout evidence without changing sequence |
| `05-contracts-schemas-and-gates.md` | replace body with the stable index/compatibility surface |
| `06-proof-and-regression-ledger.md` | close only `PG-CATALOG-01` after proof |
| `07-orchestration-onboarding-prompt.md` | require packet-authored explicit leaf selection |
| `08-handoff-ledger-and-escalation-protocol.md` | require exact leaf refs in new dispatch/handoff evidence |
| `handoffs/internal-dispatch-template.json` | demonstrate explicit leaf authority refs |
| `handoffs/dispatch-template.md` | instruct authors to list exact leaf paths/anchors |

Expected new execution surfaces are the eight leaves,
`slices/HCM-0.9/verify_contract_catalog.py`, and
`slices/HCM-0.9/evidence/execution-start.json`. New review dispatches and the
final handoff are additive evidence, not rewrites of old evidence.

## Immutable execution surface

- every record under `handoffs/records/` at execution start;
- every dispatch under `handoffs/dispatches/` at execution start;
- historical schema versions and admission hashes;
- baseline/closeout commits `214a5b8...` and `3d993ab...`;
- archived documentation and provenance;
- all Rust, Cargo, CLI, SDK, Tauri, Substrate, runtime, test, and fixture files.

Execution records its start HEAD and immutable roots in
`evidence/execution-start.json`. The verifier compares every pre-existing
handoff/dispatch path and Git blob against that start commit. Missing, renamed,
or changed historical evidence fails; additive HCM-0.9 artifacts are allowed.

## Mechanical decomposition proof

Execution adds `slices/HCM-0.9/verify_contract_catalog.py`. It must fail closed
unless:

1. the Git baseline and full-file SHA-256 match exactly;
2. the baseline H1, following blank line, body digest, line count, byte count,
   and 48 H2/22 H3 inventory match;
3. the eight actual leaf paths, exact H1 byte strings, single following blank
   lines, contiguous source spans, per-span digests, and heading assignments
   equal the decomposition inventory; no additional scaffold exists;
4. stripping each leaf H1 plus one blank line and concatenating payloads yields
   the exact frozen body bytes;
5. every frozen H2 and H3 appears exactly once, in order, under the same owning
   H2 span;
6. the heading sequence, anchor set, fence sequence, fence info strings, and
   fence bodies equal the baseline;
7. every YAML/JSON example that parsed at baseline still parses and has the same
   byte digest;
8. the stable index has the exact ordered leaf and heading maps, every legacy
   alias/link resolves once, and no normative frozen payload/fence is copied;
9. every changed Markdown relative link and fragment resolves;
10. every pre-existing handoff/dispatch path and blob remains identical to the
    execution-start commit, while additive HCM-0.9 artifacts are accepted;
11. every review manifest includes every changed non-dispatch path relative to
    the execution-start HEAD and contains no unchanged contextual authority by
    default; only the narrowly defined additive review-dispatch paths are
    exempt from membership equality;
12. every changed path is inside the exact mutable/new execution surfaces in
    this specification; any Rust, Cargo, runtime, CLI implementation, Tauri,
    Substrate, SDK implementation/public API, schema-version, HCM-0.5, archive,
    or unrelated path fails.

Negative self-tests cover wrong baseline identity, missing/changed/additional
leaf H1 or scaffold, missing/duplicate/reordered sections, one changed payload
byte, wrong span digest, broken legacy alias, broken link, changed fence,
invalid formerly-parseable example, modified/deleted/renamed historical
evidence, one omitted changed manifest path, and one forbidden runtime or
HCM-0.5 path. The verifier contains no trigger, semantic-dependency,
co-activation, operation-fixture, or inferred-selection logic.

## Validation matrix

| Gate | Required evidence |
|---|---|
| Baseline | exact commit/path/file and body digests, line/byte counts |
| Topology | eight exact contiguous spans and exact heading ownership |
| Parity | ordered leaf payload reconstructs the frozen body byte-for-byte |
| Anchors | all 70 frozen H2/H3 aliases and leaf targets resolve exactly once |
| Links | every changed Markdown relative path and fragment resolves |
| Fences/examples | fence sequence and bodies are identical; baseline-parseable YAML/JSON still parses |
| History | all execution-start handoff/dispatch paths and blobs are unchanged |
| Manifests | exact changed non-dispatch path equality, narrow review-dispatch exemption, subject-only bytes, sorted/unique repository-relative entries, fingerprint replay |
| Handoffs/archive | repository-required validators and self-tests pass |
| Scope | exact approved path allowlist; forbidden runtime and HCM-0.5 negative fixtures fail |
| Review | fresh isolated complete-subject reviewer returns CLEAN within budget |
| Git | `git diff --check` and staged GitNexus change detection pass |

## Required execution verification commands

```bash
python3 docs/specs/handbook-contract-membrane/slices/HCM-0.9/verify_contract_catalog.py --self-test
python3 docs/specs/handbook-contract-membrane/slices/HCM-0.9/verify_contract_catalog.py
python3 tools/check_archive_boundary.py
python3 tools/check_archive_boundary.py --self-test
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract
git diff --check
npx gitnexus detect-changes -r handbook -s staged
```

Equivalent repository-correct command flags may be used if the installed
GitNexus CLI reports a different accepted repository selector; the staged
change-detection gate remains mandatory.

## Rollback and stop behavior

- Keep the monolith canonical while leaves are generated as uncommitted shadow
  files. Cut over only after all eight reconstruct the baseline.
- If any parity, heading, anchor, link, fence, example, history, manifest, or
  scope proof fails, restore the monolith and remove uncommitted shadows.
- If a semantic correction appears necessary, stop and defer it outside
  HCM-0.9; do not repair frozen payload.
- If a future packet cannot choose its leaf authority explicitly, that packet
  must stop for human/context design; HCM-0.9 does not add inference.
- Never leave a partial decomposition as canonical.

## Execution review budget

The future execution session may submit at most four complete execution
subjects and perform at most three remediations, stopping immediately on CLEAN.
Reviewers are different fresh isolated built-in `default` agents; every review
dispatch lists `using-agent-skills` first and `code-review-and-quality` second.
This budget is not execution authority in the current session.

## Redesign planning review budget

This human-authorized redesign has exactly two possible review submissions:

1. fresh Redesign Review 1 over the complete current planning subject;
2. at most one remediation for validated actionable findings;
3. a different fresh Redesign Review 2 over the complete remediated subject.

Stop immediately if Review 1 is CLEAN. Never run Review 3. If Review 2 is not
CLEAN, abandon HCM-0.9, retain the monolithic `05` as canonical, do not
authorize decomposition, and write a terminal human-input handoff recording
that outcome.

## Acceptance criteria

HCM-0.9 execution is complete only when:

- [ ] all eight exact leaf files exist with their specified contiguous spans;
- [ ] all 48 H2 and 22 H3 headings map exactly once and remain byte-identical;
- [ ] the ordered leaf payload equals the frozen baseline body byte-for-byte;
- [ ] `05` is a stable index with all 70 compatibility aliases and no duplicate
  normative payload;
- [ ] future packet/dispatch guidance requires explicit leaf selection without
  any automatic semantic routing machinery;
- [ ] subject manifests contain only reviewed bytes and unchanged authority is
  referenced outside the manifest;
- [ ] all pre-existing handoff/dispatch bytes are unchanged;
- [ ] links, anchors, fences, parseable examples, manifests, handoffs, archive,
  parity, scope, and Git gates pass;
- [ ] no Rust/runtime/public API/schema/HCM-0.5/unrelated work occurred;
- [ ] a fresh isolated complete-subject execution reviewer returns CLEAN;
- [ ] reviewed execution and separate mechanical handoff/ledger commits exist;
- [ ] closeout claims mechanical decomposition only.

## Open questions

None. Any change to the eight-leaf topology, frozen baseline, payload, or
review budgets requires a new explicit human decision.
