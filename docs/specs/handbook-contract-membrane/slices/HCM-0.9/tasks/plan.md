# Execution Plan: HCM-0.9 Contract-Catalog Topology and Selective Routing

## Overview

Execute a docs/control-only structural split of the frozen HCM-0.4 contract catalog. The plan deliberately separates baseline/proof tooling, bounded leaf groups, index cutover, mutable routing updates, verification, review, and closeout. No semantic correction is permitted.

## Authority

- [`../SPEC.md`](../SPEC.md)
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md#hcm-09-corrective-maintenance-contract`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md#hcm-09-contract-catalog-structural-parity-and-routing-proof-gate`
- baseline commit `214a5b8eb182fce74478df49d4f55d226d65fdf5`
- baseline SHA-256 `c7f61db209a81ba20690f365b4069dd01f11e395335bfa10d2ce21143cc2985d`

## Dependency graph

```text
baseline identity + immutable-history snapshot
  -> parity/routing verifier
  -> leaf groups 01-04
  -> leaf groups 05-08
  -> leaf groups 09-11
  -> full payload parity
  -> stable-index cutover
  -> mutable semantic/proof refs
  -> mutable orchestration/dispatch/handoff refs
  -> complete proof wall
  -> fresh review -> remediation -> different-fresh review
  -> reviewed execution commit
  -> v1.2 parent handoff + deterministic ledger
  -> mechanical closeout commit
```

Leaf groups are prepared sequentially as non-canonical shadow copies. The monolith remains canonical until all groups pass full parity and Task 5 performs the index cutover. No intermediate split is committed.

## Required skill chain

Execution parent:

1. `using-agent-skills`
2. `context-engineering`
3. `spec-driven-development` only if this approved packet must be repaired
4. `planning-and-task-breakdown` only if decomposition must change
5. `documentation-and-adrs`
6. `debugging-and-error-recovery` for any failed validation
7. `code-review-and-quality` through fresh independent reviewers
8. `git-workflow-and-versioning` after proof and review are clean

Every delegated dispatch lists `using-agent-skills` first. Review dispatches use `using-agent-skills`, then `code-review-and-quality`.

## Task 1: Revalidate the frozen baseline and capture immutable history

**Description:** Fail fast before edits. Verify branch/HEAD/worktree, HCM-0.4 ancestry, exact baseline bytes, live mutable references, and an exact Git-backed start boundary for every pre-existing handoff/dispatch. Persist that boundary in one execution-start evidence record.

**Acceptance criteria:**

- [ ] baseline `git show` SHA-256 equals the frozen value;
- [ ] current work is attributable and non-overlapping;
- [ ] live H2/H3 inventory matches the specification counts;
- [ ] the immutable roots contain no untracked files before capture;
- [ ] `execution-start.json` records the exact execution-start HEAD, frozen baseline commit, and immutable roots;
- [ ] every pre-existing history path/hash can be derived from the recorded start commit.

**Verification:**

```bash
git branch --show-current
git rev-parse HEAD
git status --short
git merge-base --is-ancestor 214a5b8eb182fce74478df49d4f55d226d65fdf5 HEAD
git show 214a5b8eb182fce74478df49d4f55d226d65fdf5:docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md | shasum -a 256
rg -n --glob '!archived/**' --glob '!target/**' '05-contracts-schemas-and-gates\.md' docs
history=docs/specs/handbook-contract-membrane/handoffs
test -z "$(git ls-files --others --exclude-standard -- "$history/records" "$history/dispatches")"
start_head="$(git rev-parse HEAD)"
mkdir -p docs/specs/handbook-contract-membrane/slices/HCM-0.9/evidence
jq -n --arg head "$start_head" --arg baseline "214a5b8eb182fce74478df49d4f55d226d65fdf5" '{
  schema_id: "handbook.hcm-0.9-execution-start",
  schema_version: "1.0",
  execution_start_head: $head,
  baseline_commit: $baseline,
  immutable_roots: [
    "docs/specs/handbook-contract-membrane/handoffs/records",
    "docs/specs/handbook-contract-membrane/handoffs/dispatches"
  ]
}' > docs/specs/handbook-contract-membrane/slices/HCM-0.9/evidence/execution-start.json
```

**Dependencies:** none.  
**Files touched:** `slices/HCM-0.9/evidence/execution-start.json`.  
**Estimated scope:** S.

## Task 2: Create the slice-local parity and routing verifier

**Description:** Add one deterministic verifier that derives the frozen heading/fence payload from Git, validates the exact topology map, reconstructs the semantic payload, verifies index aliases/targets/links, parses baseline-parseable YAML/JSON examples, and checks historical immutability inputs.

**Acceptance criteria:**

- [ ] verifier requires the exact baseline commit and SHA;
- [ ] verifier encodes the eleven ordered leaves and 48 H2 assignments from `SPEC.md`;
- [ ] verifier validates the execution-start record and compares all start-commit handoff/dispatch Git blobs while permitting additive HCM-0.9 artifacts;
- [ ] verifier classifies every frozen positional and named contract dependency and requires the exact source-triggered routing table for leaves 8-11;
- [ ] negative self-tests cover a missing section, duplicate section, changed payload byte, broken alias, broken fence, wrong baseline SHA, omission or breakage of every leaf-8/9/10/11 dependency group, an unjustified extra or unclassified cross-leaf dependency, modified/deleted/renamed history, and an allowed additive dispatch;
- [ ] verifier is read-only and has no network dependency.

**Verification:**

```bash
python3 docs/specs/handbook-contract-membrane/slices/HCM-0.9/verify_contract_catalog.py --self-test
```

**Dependencies:** Task 1.  
**Files touched:** `slices/HCM-0.9/verify_contract_catalog.py`.  
**Estimated scope:** S.

## Task 3: Materialize canonical leaf shadows 01-04

**Description:** Generate the first four leaf files from exact contiguous baseline byte ranges. They are shadow copies only; `05` remains canonical.

**Acceptance criteria:**

- [ ] each file has only its specified H1, any exact verifier-approved routing-only dependency block, and exact assigned payload;
- [ ] H2 order and subordinate H3/fence bytes match baseline;
- [ ] group boundaries occur only before an H2 and outside fences.

**Verification:** run the verifier's bounded group mode for leaves 01-04 and inspect exact diffs.

**Dependencies:** Task 2.  
**Files touched:** leaf files 01-04.  
**Estimated scope:** M (4 files).

## Task 4: Materialize canonical leaf shadows 05-08

**Description:** Generate the next four exact leaf payloads while retaining monolith authority.

**Acceptance criteria:** same as Task 3 for leaves 05-08; leaf 8's routing-only block exactly matches its combined ordered dependency-table targets.  
**Verification:** run bounded group verification and inspect exact diffs.  
**Dependencies:** Task 3.  
**Files touched:** leaf files 05-08.  
**Estimated scope:** M (4 files).

## Task 5: Materialize leaves 09-11 and cut over the stable index

**Description:** Generate the final three leaves, prove all eleven reconstruct the baseline, then replace `05` with the routing/compatibility index. Never cut over before complete parity passes.

**Acceptance criteria:**

- [ ] all eleven leaf payloads reconstruct the baseline body byte-for-byte;
- [ ] index has the unchanged H1, topology, exact H2 map, selective-loading rules, and 70 H2/H3 forwarding aliases;
- [ ] every alias target resolves to the owning leaf heading;
- [ ] index contains no copied normative payload or baseline fence.
- [ ] leaves 9-11 contain their exact combined ordered dependency-table targets and the complete positional/named-dependency audit has no unclassified occurrence.

**Verification:** run the full parity verifier and changed-link/anchor/fence checks.

**Dependencies:** Task 4.  
**Files touched:** leaf files 09-11 and `05-contracts-schemas-and-gates.md`.  
**Estimated scope:** M (4 files).

## Checkpoint A: Structural cutover

- [ ] full baseline/parity/mapping/anchor/fence/example verification passes;
- [ ] monolith-to-index cutover is complete and no partial split is staged;
- [ ] no semantic payload difference exists;
- [ ] no immutable history changed.

## Task 6: Update mutable semantic and proof routing

**Description:** Update active control-pack references to exact leaves and close only the structural catalog gate after proof exists.

**Acceptance criteria:**

- [ ] README exposes index and leaves without duplicating authority;
- [ ] semantic model references exact owning leaves;
- [ ] crosswalk records landed topology without promoting runtime semantics;
- [ ] phase map sequencing remains HCM-0.4 -> HCM-0.9 -> HCM-0.5;
- [ ] proof ledger routes HCM-0.2/0.3/0.4 gates to exact leaves and changes only `PG-CATALOG-01` when justified.

**Verification:** `rg` old mutable refs, link/anchor verifier, exact scoped diff inspection.

**Dependencies:** Checkpoint A.  
**Files touched:** `00-README.md`, `02-semantic-model.md`, `03-seam-crosswalk.md`, `04-phase-slice-map.md`, `06-proof-and-regression-ledger.md`.  
**Estimated scope:** M (5 files).

## Task 7: Update orchestration, dispatch, and handoff routing

**Description:** Make current agents select the minimum exact contract leaf set while preserving all historical records/dispatches.

**Acceptance criteria:**

- [ ] `07` requires exact leaf selection and complete-catalog selection only when justified;
- [ ] `07` applies dependency rows only when their triggering source contract is in scope and never loads unrelated advertised dependencies by default;
- [ ] `08` requires exact leaf refs in new dispatch/handoff evidence and explicitly preserves historical monolith refs;
- [ ] JSON dispatch template uses the development-orchestration leaf example;
- [ ] Markdown dispatch guide requests exact leaf path/anchor;
- [ ] no handoff/dispatch schema changes are introduced.

**Verification:** all handoff validator modes plus `rg` for mutable monolith authority refs.

**Dependencies:** Task 6.  
**Files touched:** `07-orchestration-onboarding-prompt.md`, `08-handoff-ledger-and-escalation-protocol.md`, `handoffs/internal-dispatch-template.json`, `handoffs/dispatch-template.md`.  
**Estimated scope:** M (4 files).

## Task 8: Run the complete proof wall

**Description:** Execute all validation before review and capture exact results for the review dispatch and closeout.

**Acceptance criteria:**

- [ ] parity verifier and self-tests pass;
- [ ] Markdown links/anchors/fences and YAML/JSON examples pass;
- [ ] applicable semantic assertions pass;
- [ ] archive boundary and self-test pass;
- [ ] all handoff validator modes pass;
- [ ] historical bytes match the execution-start inventory;
- [ ] `git diff --check` and scope assertions pass;
- [ ] no HCM-0.5 or runtime change exists.

**Verification:** run both the verifier `--self-test` and full verifier commands in `SPEC.md` at the final subject fingerprint, then retain command/result summaries.

**Dependencies:** Task 7.  
**Files touched:** none unless a failed gate triggers bounded remediation.  
**Estimated scope:** S.

## Task 9: Execute bounded fresh review and remediation

**Description:** Submit the complete subject to fresh isolated read-only built-in `default` reviewers. Findings are ordered Critical, Required, Optional, Nit. Validate findings against live truth, remediate valid findings, rerun the proof wall, and use a different fresh reviewer.

**Acceptance criteria:**

- [ ] every review dispatch manifest binds the exact complete subject and begins required skills with `using-agent-skills`;
- [ ] maximum four complete-subject submissions and three remediations;
- [ ] review stops immediately on CLEAN;
- [ ] every valid findings round has typed successful remediation and a different-fresh re-review;
- [ ] no self-approval or severity weakening occurs.

**Verification:** replay dispatch manifests; validate review lineage; confirm final verdict CLEAN or execute the budget-exhaustion stop.

**Dependencies:** Task 8.  
**Files touched:** additive HCM-0.9 review dispatches; bounded subject files only when remediating valid findings.  
**Estimated scope:** M.

## Checkpoint B: Review-clean execution subject

- [ ] complete proof wall passes at the exact final subject fingerprint;
- [ ] final fresh review is CLEAN;
- [ ] no review budget limit was exceeded;
- [ ] no unreviewed edit followed the clean fingerprint.

## Task 10: Commit the reviewed HCM-0.9 execution

**Description:** Apply git workflow only after Checkpoint B. Stage only HCM-0.9 execution/control artifacts, run staged GitNexus change detection, inspect cached diff, and commit one scoped Conventional Commit.

**Acceptance criteria:**

- [ ] staged paths match the reviewed subject plus additive review dispatches;
- [ ] staged GitNexus detection is low/expected and has zero runtime process impact;
- [ ] cached diff and whitespace checks pass;
- [ ] commit message is scoped and docs/control-only.

**Verification:**

```bash
npx gitnexus detect-changes -r handbook -s staged
git diff --cached --check
git diff --cached --name-status
git diff --cached
```

**Dependencies:** Checkpoint B.  
**Files touched:** Git index only.  
**Estimated scope:** S.

## Task 11: Create the mechanical execution closeout

**Description:** Create one current-schema v1.2 parent handoff for completed structural execution, rebuild the ledger deterministically, validate all modes, and commit only the new record/ledger in a second commit. Do not start HCM-0.5.

**Acceptance criteria:**

- [ ] handoff records exact execution commit, packet paths, review lineage/CLEAN verdict, validation evidence, and structural-only claim;
- [ ] `status: completed`, `stop_reason: completed`, and `resume.execution_target: none` are used only if execution itself is proof-complete;
- [ ] deterministic ledger and all validator modes pass;
- [ ] second commit contains only the new parent record and rebuilt ledger.

**Verification:** all three handoff validator modes, deterministic ledger parity, staged scope/diff checks.

**Dependencies:** Task 10.  
**Files touched:** one new handoff record and `handoffs/ledger.jsonl`.  
**Estimated scope:** S (2 files).

## Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Manual copy changes semantic bytes | Critical | generate from frozen Git ranges; byte-level reconstruction gate |
| Fence crosses a leaf boundary | Critical | boundaries only at frozen H2 lines outside fences; verifier fails closed |
| Old anchors break | Required | generate/verify all frozen H2/H3 aliases and leaf targets |
| Frozen “above/below” prose loses its referent | Critical | classify every positional reference; require exact routing-only cross-leaf dependencies and negative tests |
| Index becomes duplicate authority | Required | prohibit baseline fences/normative payload in index; leaf-only ownership rule |
| Current agents still load monolith | Required | update `07`, `08`, both dispatch templates, and direct mutable refs |
| Historical records are rewritten | Critical | derive every pre-existing path/blob from the persisted execution-start HEAD; fail modified/deleted/renamed and allow only additive artifacts |
| Semantic defect discovered mid-split | Required stop/defer | record exact leaf/heading; never fix inside HCM-0.9 |
| Review budget exhausted | Human decision | no fifth review; partial human-input handoff with exact unresolved findings |
| HCM-0.5 starts prematurely | Scope violation | explicit sequence/stop gates and closeout prohibition |

## Parallelization

Do not parallelize edits. Leaf groups share one ordered parity closure, and control references depend on the final topology. Fresh reviewers are isolated and sequential after each remediation. Read-only evidence collection may be parallel only if it cannot mutate repository state, but parallelism is not required.

## Open questions

None. Stop on any topology, baseline, semantic, or scope ambiguity rather than inventing a new decision.
