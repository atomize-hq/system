# Execution Plan: HCM-0.9 Mechanical Contract-Catalog Decomposition

## Overview

Execute a documentation/control-only split of the frozen HCM-0.4 contract
catalog into eight canonical leaves. The plan verifies exact Git bytes, creates
all leaves as shadows, proves complete reconstruction, cuts over the stable
index once, updates active guidance for explicit packet-authored leaf selection,
and closes only after independent execution review. It contains no automatic
semantic routing engine.

## Authority

- [`../SPEC.md`](../SPEC.md)
- [`../evidence/decomposition-inventory.md`](../evidence/decomposition-inventory.md)
- [`../evidence/dependency-audit.md`](../evidence/dependency-audit.md) (supersession boundary only)
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md#hcm-09-corrective-maintenance-contract`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md#hcm-09-contract-catalog-mechanical-decomposition-proof-gate`
- baseline commit `214a5b8eb182fce74478df49d4f55d226d65fdf5`

## Dependency graph

```text
baseline identity + immutable-history snapshot
  -> mechanical decomposition verifier
  -> all eight non-canonical leaf shadows
  -> full byte/heading/anchor/link/fence/example parity
  -> one stable-index cutover
  -> active explicit-selection guidance
  -> complete proof wall
  -> fresh execution review/remediation loop
  -> reviewed execution commit
  -> v1.2 execution handoff + deterministic ledger
  -> mechanical closeout commit
```

No intermediate split is committed. No task derives or expands semantic leaf
dependencies.

## Required skill chain

Execution parent:

1. `using-agent-skills`
2. `context-engineering`
3. `spec-driven-development` only if the approved packet must be repaired
4. `planning-and-task-breakdown` only if decomposition must change
5. `documentation-and-adrs`
6. `debugging-and-error-recovery` for failed proof
7. `code-review-and-quality` through fresh independent reviewers
8. `git-workflow-and-versioning` after proof and review are clean

Every delegated dispatch lists `using-agent-skills` first. Review dispatches
then list `code-review-and-quality`.

## Task 1: Revalidate baseline and capture immutable history

**Description:** Verify the live branch/worktree, HCM-0.4 ancestry, exact Git
baseline, exact decomposition inventory, live mutable refs, and every
pre-existing handoff/dispatch path. Persist one execution-start record.

**Acceptance criteria:**

- [ ] full-file and body digests, line/byte counts, and 48 H2/22 H3 counts match;
- [ ] each of the eight inventory span digests derives from the Git baseline;
- [ ] current dirt is attributable and non-overlapping;
- [ ] immutable history roots have no untracked files before capture;
- [ ] `execution-start.json` records exact start HEAD, baseline, and roots.

**Dependencies:** none.  
**Files touched:** `slices/HCM-0.9/evidence/execution-start.json`.

## Task 2: Create the mechanical decomposition verifier

**Description:** Add one deterministic, read-only verifier for baseline identity,
the eight spans, payload reconstruction, headings/anchors, links, fences,
examples, index contents, immutable history, and scope.

**Acceptance criteria:**

- [ ] verifier encodes only the mechanical inventory in the approved packet;
- [ ] verifier strips exactly each leaf H1 plus one blank line before parity;
- [ ] verifier checks all requirements in `SPEC.md#mechanical-decomposition-proof`;
- [ ] self-tests cover missing/changed/additional H1/scaffold, each parity and
  history negative, an omitted changed manifest path, and a forbidden runtime
  or HCM-0.5 path without network access;
- [ ] verifier has no trigger, route, co-activation, operation-fixture,
  semantic-dependency, fanout, or inferred-selection logic.

**Verification:**

```bash
python3 docs/specs/handbook-contract-membrane/slices/HCM-0.9/verify_contract_catalog.py --self-test
```

**Dependencies:** Task 1.  
**Files touched:** `slices/HCM-0.9/verify_contract_catalog.py`.

## Task 3: Materialize all eight leaf shadows

**Description:** Generate the eight leaf files from exact contiguous baseline
byte spans. Each receives only its approved H1, one blank line, and frozen
payload. The monolith remains canonical.

**Acceptance criteria:**

- [ ] filenames, exact H1 byte strings, one following blank line, source spans,
  span digests, and heading ownership match `evidence/decomposition-inventory.md`;
- [ ] all eight shadows together reconstruct the baseline body exactly;
- [ ] no routing block or other pre-payload scaffold exists;
- [ ] no H3 or fence is split.

**Dependencies:** Task 2.  
**Files touched:** `contracts/01-...md` through `contracts/08-...md`.

## Checkpoint A: Shadow parity

- [ ] complete byte reconstruction passes;
- [ ] all heading/anchor/link/fence/example checks pass;
- [ ] `05` remains the unchanged canonical monolith;
- [ ] no immutable history changed.

## Task 4: Cut over the stable index

**Description:** Only after Checkpoint A, replace the monolith body with the
stable discovery/compatibility index in one change.

**Acceptance criteria:**

- [ ] unchanged H1 and exact ordered eight-leaf table exist;
- [ ] exact 48 H2/22 H3 ownership and compatibility aliases exist;
- [ ] every alias target resolves exactly once;
- [ ] index contains no copied normative payload or baseline fence;
- [ ] index instructs future packets/dispatches to list leaf authority
  explicitly and disclaims inferred selection.

**Dependencies:** Checkpoint A.  
**Files touched:** `05-contracts-schemas-and-gates.md`.

## Task 5: Update active explicit-selection guidance

**Description:** Update only live control-pack and template references needed to
discover the eight leaves and require exact packet-authored leaf selection.

**Acceptance criteria:**

- [ ] README, semantic model, crosswalk, phase map, and proof ledger link to
  exact owning leaves where their current bytes require authority;
- [ ] orchestration/handoff guidance and dispatch templates require authors to
  enumerate exact leaf paths/anchors in `authority_refs` and/or
  `contracts_and_gates`;
- [ ] subject manifests contain only changed/reviewed bytes; unchanged authority
  is not added merely because it was read;
- [ ] no guidance defines triggers, inferred dependency closure, co-activation,
  operation fixtures, fanout, or transitive loading;
- [ ] pre-existing records and dispatches remain byte-identical.

**Dependencies:** Task 4.
**Files touched:** only the mutable surfaces listed in `SPEC.md` that the live
inventory proves require change.

## Task 6: Run the complete proof wall

**Description:** Run all mechanical, archive, handoff, manifest, history, scope,
and Git checks at one exact subject fingerprint before review.

**Acceptance criteria:**

- [ ] verifier and self-tests pass;
- [ ] archive boundary and self-test pass;
- [ ] all handoff validator modes pass;
- [ ] every review manifest replays exactly and its paths equal all changed
  non-dispatch paths relative to execution-start HEAD;
- [ ] only additive HCM-0.9 review dispatches use the self-referential dispatch
  exemption, and every such dispatch validates separately;
- [ ] historical bytes match execution-start inventory;
- [ ] changed paths equal the exact specification allowlist and
  `git diff --check` passes;
- [ ] omitted-manifest-path and forbidden runtime/HCM-0.5 negative fixtures
  fail closed.

**Dependencies:** Task 5.
**Files touched:** none unless proof exposes an in-scope defect.

## Task 7: Run bounded fresh execution review

**Description:** Submit the complete execution subject and proof wall to fresh
isolated read-only built-in `default` reviewers. Validate actionable findings,
remediate, rerun proof, and use a different fresh reviewer. Stop on CLEAN.

**Acceptance criteria:**

- [ ] each dispatch manifest lists only files whose current bytes are reviewed;
- [ ] changed index/leaves/verifier/control files are manifested;
- [ ] manifest membership equals every changed non-dispatch path at that review
  boundary; additive review dispatches are the only exemption;
- [ ] unchanged context authority appears only outside the manifest;
- [ ] required skills begin `using-agent-skills`, then
  `code-review-and-quality`;
- [ ] at most four submissions and three remediations;
- [ ] no self-approval, reviewer reuse, or severity weakening.

**Dependencies:** Task 6.
**Files touched:** additive review dispatches and validated in-scope remediation
only.

## Checkpoint B: Review-clean execution subject

- [ ] proof wall passes at the exact reviewed fingerprint;
- [ ] a fresh reviewer returned CLEAN;
- [ ] no unreviewed byte changed afterward.

## Task 8: Commit reviewed execution

Stage only the reviewed HCM-0.9 execution/control subject and additive review
dispatches. Run staged GitNexus change detection, cached whitespace/scope
checks, inspect the cached diff, and create one documentation/control commit.

## Task 9: Create mechanical execution closeout

Create one current-schema v1.2 parent handoff recording the execution commit,
CLEAN review lineage, proof, structural-only result, and no HCM-0.5 activation.
Rebuild/validate the ledger and commit only the new record and ledger in a
second commit.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| Payload byte changes during copy | generate exact Git spans; reconstruct and hash before cutover |
| Partial decomposition becomes canonical | keep shadows uncommitted; one cutover after full parity |
| Old fragments break | generate and verify all 70 compatibility aliases/targets |
| Index duplicates authority | prohibit frozen payload/fences in index |
| A packet omits needed authority | packet author must explicitly correct its list; do not infer it |
| Historical evidence changes | compare every start-commit history path/blob; fail closed |
| Semantic defect discovered | record and defer; never change frozen bytes here |
| HCM-0.5 starts | explicit scope and closeout prohibition |

## Parallelization

Do not parallelize edits. The leaves share one ordered byte-parity closure and
the index cutover depends on all eight. Fresh reviewers are isolated and
sequential after remediation.

## Open questions

None. Stop on any topology, baseline, semantic, or scope ambiguity.
