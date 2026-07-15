# Implementation Plan: HCM-0.5 Contract Membrane and Dock Protocol Freeze

## Overview

Execute one forward documentation/design freeze across the affected canonical `00`-`06` sections. Preserve the frozen HCM-0.4 owner, ordinary-operation, DTO, transport, bridge, and publication contracts while replacing the preliminary HCM-0.5 contract/dock material with implementation-grade target authority. Runtime implementation remains deferred to Phase 5.

Generic orchestration, independent review, remediation lineage, true-stop, handoff, ledger, and two-commit mechanics are owned by [`07-orchestration-onboarding-prompt.md`](../../../07-orchestration-onboarding-prompt.md) and [`08-handoff-ledger-and-escalation-protocol.md`](../../../08-handoff-ledger-and-escalation-protocol.md). A non-completion handoff is orchestration closeout, not an HCM-0.5 feature task.

## Authority and sequencing decisions

- Treat the completed HCM-0.4 record and reviewed commit as dependency evidence only; change none of its frozen interface semantics.
- Treat HCM-0.9 as abandoned evidence only; create no catalog leaves, topology replacement, compatibility index, or routing engine.
- Keep `05-contracts-schemas-and-gates.md` canonical and monolithic.
- Separate contract lifecycle from evaluation, verdict, and gate state.
- Preserve exact contract identity/fingerprint selection, the closed compatibility table, immutable lifecycle records, and exact lifecycle transition authority.
- Preserve all-of evidence requirements, per-kind/case cardinality, freshness/provenance/Resolution limits, repeated-evidence consistency, closed verdict semantics, and hard-fail precedence.
- Preserve the exact content-addressed dock implementation bundle/runtime closure, process JSON framing, default-deny isolation, unconditional v1 network denial, timeout/cancellation/failure rules, and validator-as-witness boundary.
- Select the local Draft 2020-12 JSON Schema process dock as the first proof target without claiming runtime proof.
- Append only the HCM-0.4-compatible ordinary contract/dock operation definitions; do not alter existing operation or transport contracts.
- Keep `PG-CONTRACT-01`, `PG-DOCK-01`, and `PG-GATE-01` open and do not start HCM-0.6.

## Execution-entry gate

```text
review-clean packet commit -> validated v1.2 context-boundary handoff binding that commit, this packet path, and the reviewed packet fingerprint -> fresh design-freeze session selecting that exact handoff -> live repository and packet-identity validation by that fresh session -> canonical HCM-0.5 design edits may begin
```

The approved selector is supplied by the review-clean packet closeout and is not hard-coded in this plan.

## Dependency graph

```text
Task 1: validate entry and live authority
  -> Task 2: freeze affected canonical 00-06 design
  -> Task 3: run the complete proof and regression wall
  -> Task 4: obtain fresh independent final review and resolve accepted blockers
  -> Task 5: commit the exact reviewed subject and perform mechanical closeout
```

Dependencies are direct and acyclic. Task 4 contains any required in-scope remediation, complete proof rerun, and different-fresh re-review through canonical `07`/`08`; it does not create a separate historical-review task.

## Task 1: Validate approved entry handoff and packet identity

**Description:** In a fresh design-freeze session, validate the exact approved selector, context-boundary record, packet commit/fingerprint/path binding, live branch/HEAD/status, HCM-0.4 dependency evidence, HCM-0.9 abandoned boundary, and canonical `05` identity before any canonical edit.

**Acceptance criteria:**
- [ ] The selected v1.2 handoff is schema/ledger-valid and binds the review-clean packet commit, exact packet path, and recomputable packet fingerprint.
- [ ] Live packet bytes and repository state match the approved entry identity, with no unsafe overlapping work.
- [ ] HCM-0.4 is confirmed as completed dependency evidence and HCM-0.9 supplies no forward authority.
- [ ] The session confirms documentation/design-only scope and keeps every runtime proof gate open.

**Verification:** Validate the selected record and ledger through canonical `08`; recompute the packet manifest/fingerprint; inspect `git status`, branch, HEAD, and scoped diffs; verify the HCM-0.4 reviewed commit and current canonical `05` bytes.

**Dependencies:** Execution-entry gate.

**Files likely touched:** None.

**Estimated scope:** S.

## Task 2: Apply the canonical contract/dock design freeze

**Description:** Update the affected canonical `00`-`06` sections as one coherent design subject. Replace only preliminary HCM-0.5 authority, preserve already-frozen dependencies, and author the final intended `00-README.md` HCM-0.5 status bytes before proof and review.

**Acceptance criteria:**
- [ ] `00`-`04` consistently state the HCM-0.5 owner boundaries, validator-as-witness rule, process-first/future-Rust parity, selected proof dock, exit gate, and still-open runtime classification.
- [ ] Canonical `05` closes contract identity/version compatibility/lifecycle, claims/applicability, evidence identity/provenance/freshness/cardinality/precedence/Resolution limits, verdict vocabulary, and gate composition/hard-fail behavior.
- [ ] Canonical `05` closes dock manifest, implementation-bundle/runtime-closure identity, process JSON request/result, grants/isolation/no-network, timeout/cancellation/refusal/crash/cleanup, and canonical evidence admission semantics.
- [ ] Canonical `05` selects `handbook.dock.json-schema@1.0.0` with bounded Draft 2020-12 responsibility and appends only the approved ordinary HCM-0.4-compatible operation definitions.
- [ ] Canonical `06` adds the documentation-freeze proof/regression authority while runtime gates remain open.
- [ ] HCM-0.2, HCM-0.3, and HCM-0.4 semantics are unchanged; HCM-0.6, HCM-0.9, runtime code, schemas, validators, and catalog leaves are untouched.

**Verification:** Inspect the scoped `00`-`06` diff; run targeted cross-document owner/identity/state/protocol assertions; compare frozen HCM-0.2/0.3/0.4 sections; confirm the final `00-README.md` bytes are already present.

**Dependencies:** Task 1.

**Files likely touched:**
- `docs/specs/handbook-contract-membrane/00-README.md`
- `docs/specs/handbook-contract-membrane/01-target-architecture.md`
- `docs/specs/handbook-contract-membrane/02-semantic-model.md`
- `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md`
- `docs/specs/handbook-contract-membrane/04-phase-slice-map.md`
- `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`
- `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md`

**Estimated scope:** M, documentation/design only.

## Task 3: Run the complete proof and regression wall

**Description:** Execute every proof-wall item in `SPEC.md` against the complete canonical design subject and retain the exact results before final canonical review.

**Acceptance criteria:**
- [ ] All JSON/YAML examples parse with the required duplicate-key behavior, and Markdown links, anchors, and fences validate.
- [ ] Mechanical checks close identity/SemVer, lifecycle adjacency/authority, applicability, evidence partition/cardinality/precedence/freshness/Resolution, verdict/gate, manifest/runtime-closure, protocol, isolation/no-network, timeout/cancellation/failure, and ordinary-operation matrices.
- [ ] Negative/refusal cases prove malformed, stale, substituted, under-Resolution, missing, conflicting, failed, refused, timed-out, cancelled, crashed, oversized, or cleanup-uncertain inputs cannot produce false green or partial evidence.
- [ ] HCM-0.2/0.3/0.4 regressions, canonical-monolith/no-leaf rules, open runtime gates, archive boundary, portability, docs-only scope, and no-HCM-0.6 boundary pass.
- [ ] Handoff validators, `git diff --check`, scoped diff inspection, and repository-required change detection pass for the complete subject.

**Verification:** Run and preserve the complete `SPEC.md` proof wall, including all negative cases and regression comparisons; record any unavailable proof honestly rather than promoting it.

**Dependencies:** Task 2.

**Files likely touched:** No new semantic files; only proof artifacts already authorized by the top-level session, if canonical `06` requires them.

**Estimated scope:** M.

## Task 4: Obtain fresh independent final review and resolve accepted blockers

**Description:** Submit the complete canonical `00`-`06` subject, all three packet files, and complete proof results to fresh independent review through canonical `07`/`08`. Validate findings against live authority. Remediate only accepted in-scope blockers, rerun the complete proof wall, and obtain different-fresh re-review.

**Acceptance criteria:**
- [ ] The review subject manifest binds every intended canonical byte, including the final `00-README.md` status bytes, plus this packet and complete proof evidence.
- [ ] Review is fresh and independent; accepted blockers receive bounded remediation and different-fresh re-review through canonical mechanics.
- [ ] The complete proof wall is rerun after every accepted subject mutation.
- [ ] Final `CLEAN` binds the exact subject later replayed and staged; a non-clean subject is never treated as complete.
- [ ] No subject byte changes after `CLEAN`.

**Verification:** Recompute every review manifest and aggregate fingerprint; validate result/finding/remediation lineage through canonical `08`; compare final live bytes to the clean manifest.

**Dependencies:** Task 3.

**Files likely touched:** The Task 2 files only when an accepted in-scope blocker requires correction; immutable internal review dispatches are orchestration evidence, not feature authority.

**Estimated scope:** Bounded by validated findings.

## Task 5: Commit the reviewed subject and perform mechanical v1.2 closeout

**Description:** Replay proof and stage the clean subject byte-identically, commit the reviewed HCM-0.5 design authority, then create and separately commit the parent-owned v1.2 handoff and deterministic ledger closeout through canonical `08`.

**Acceptance criteria:**
- [ ] Proof replay passes on bytes identical to the clean manifest, including the exact `00-README.md` hash.
- [ ] Staged change detection and `git diff --cached --check` report only the reviewed HCM-0.5 subject and authorized proof evidence.
- [ ] The primary commit contains the exact reviewed subject and no mechanical handoff/ledger mutation.
- [ ] The completed v1.2 handoff binds the primary commit, clean subject fingerprint, proof, and review/remediation lineage; all validator modes and deterministic ledger parity pass.
- [ ] The second commit contains only mechanical handoff/ledger closeout artifacts.
- [ ] HCM-0.6 is not started.

**Verification:** Replay the clean manifest at the primary commit; inspect both scoped commits; run all canonical `08` validators, ledger parity, mechanical diff checks, and repository-required staged change detection; finish with a clean attributable status.

**Dependencies:** Task 4 with final `CLEAN`.

**Files likely touched:**
- the exact reviewed Task 2 subject and authorized proof/review evidence for the primary commit;
- one `docs/specs/handbook-contract-membrane/handoffs/records/*.json` record and `docs/specs/handbook-contract-membrane/handoffs/ledger.jsonl` for the separate mechanical closeout commit.

**Estimated scope:** S.

## Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Contract lifecycle and evaluation state remain conflated | High | closed, separately named lifecycle/verdict/gate models plus mechanical adjacency assertions |
| Dock output is mistaken for canonical evidence | High | candidate -> membrane validation -> immutable evidence boundary; validator remains witness |
| Process execution gains ambient authority | High | exact implementation closure, default-deny grants, unconditional v1 network denial, bounded resources, and failure matrix |
| HCM-0.4 interface semantics drift | High | append only named operations and regression-compare frozen owner/operation/DTO/transport sections |
| Complete proof or final status bytes fall outside final review | High | finish proof and `00-README.md` bytes first, then manifest every intended canonical byte |
| Closeout mutates the reviewed subject | High | no mutation after `CLEAN`; byte-identical replay/staging; separate mechanical closeout commit |
| Review findings widen into adjacent slices | Medium | validate findings against packet authority and use canonical true-stop behavior for broader scope |

## Open runtime gates

HCM-0.5 freezes design only. `PG-CONTRACT-01`, `PG-DOCK-01`, `PG-GATE-01`, actual process supervision, Rust-native binding, public SDK/CLI/Tauri surfaces, real validator execution, published APIs, and consumer adoption remain future implementation/proof work. The next slice is never started automatically.
