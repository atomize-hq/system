# Implementation Plan: HCM-0.5 Contract Membrane and Dock Protocol Freeze

## Overview

Execute a documentation-only semantic freeze through a closed review-budget state machine. A clean Review 3 would have authorized the canonical monolithic control-pack edits, the complete proof wall, and Review 4 as the final-design review. Because Review 3 instead returned bounded planning findings, the live path is now Remediation 3 followed by Review 4 as planning closure, then an honest non-completion handoff. Canonical design edits, final-design approval, and the two-commit completion path are unavailable on this branch. Runtime contract/dock work remains deferred to Phase 5.

## Authority and sequencing decisions

- Use the explicit HCM-0.5 selection; `HANDOFF_SELECTOR: none` means no resume record.
- Treat the HCM-0.4 completion record as dependency evidence only.
- Ignore HCM-0.9 as resume/design authority and create no catalog leaves.
- Keep `05-contracts-schemas-and-gates.md` canonical.
- Separate contract lifecycle from evaluation/verdict/gate outcomes.
- Use exact contract ref/fingerprint selection, a closed SemVer-impact table, a total lifecycle transition/authority table, and closed evidence-cardinality/precedence rules.
- Bind each all-of evidence-kind requirement to its own case/cardinality/stability tuple; omitted score weight means no score participation.
- Select the Draft 2020-12 JSON Schema process dock as the first proof target because the live repository and design lineage establish a deterministic, offline, low-authority precedent.
- Deny all process-dock networking in protocol v1.
- Bind every dock manifest/run/evidence record to one exact content-addressed implementation bundle, entrypoint digest, and bundled runtime/dependency closure verified before spawn.
- Extend the frozen HCM-0.4 ordinary-operation catalog; do not modify its owner/DTO/transport/bridge/publication rules.
- Keep runtime proof gates open and make no seam classification promotion beyond frozen target design.

## Dependency graph

```text
live preflight + completed HCM-0.2/0.3/0.4/0.8 evidence
  -> SPEC/plan/todo
  -> Review 1 findings -> Remediation 1
  -> Review 2 findings -> Remediation 2
  -> Review 3
       CLEAN -> canonical 01-06 design freeze + ordinary operations
             -> complete proof wall (SPEC items 1-12 with captured results)
             -> immutable Review 4 final-design dispatch
                  CLEAN -> byte-identical proof replay + staging checks
                        -> primary reviewed-slice commit
                        -> completed v1.2 handoff + ledger closeout commit
                  FINDINGS -> stop without completion; no remediation or Review 5
       FINDINGS -> bounded Remediation 3
                -> Review 4 planning-closure dispatch
                     CLEAN or FINDINGS -> stop without canonical edits or completion;
                                          write exact non-completion handoff; no Review 5
```

## Phase 1: Planning authority

### Task 1: Validate scope, dependencies, and candidate authority

**Description:** Reconfirm the live branch/status, phase row, completed HCM-0.4 evidence, abandoned HCM-0.9 boundary, canonical `05` fingerprint, active control-pack sections, and existing JSON Schema/validator precedent.

**Acceptance criteria:**
- [x] HCM-0.5 exists and is authorized as docs/design-only.
- [x] Worktree is attributable and has no overlapping uncommitted work.
- [x] HCM-0.4 completion and current handoff validator modes validate.
- [x] HCM-0.9 is excluded from resume and implementation authority.
- [x] First-dock candidates are compared using live evidence.

**Verification:** `git status --short`; exact `jq` record/ledger queries; three `validate_handoffs.py` modes; scoped `rg` evidence inspection.

**Dependencies:** None.

**Files likely touched:** None.

**Estimated scope:** S.

### Task 2: Create the independently executable packet

**Description:** Create `SPEC.md`, `tasks/plan.md`, and `tasks/todo.md` with exact semantic questions, owner boundaries, lifecycle, evidence/verdict/gate and process-dock contracts, security posture, negative cases, first-proof selection, affected 01-06 sections, proof wall, non-goals, review budget, and stop rules.

**Acceptance criteria:**
- [x] All three required files exist and use only repository-relative durable refs.
- [x] Every requested planning topic is explicit and testable.
- [x] No Rust/runtime/HCM-0.6/HCM-0.9 authority is introduced.

**Verification:** required-heading/phrase assertions; Markdown fence/link checks; `git diff --check`; scoped diff inspection.

**Dependencies:** Task 1.

**Files likely touched:** `slices/HCM-0.5/SPEC.md`, `slices/HCM-0.5/tasks/plan.md`, `slices/HCM-0.5/tasks/todo.md`.

**Estimated scope:** M.

### Checkpoint A: Fresh planning review and closed budget branch

- [x] Build immutable JSON Review 1-3 dispatches with sorted path/SHA-256 manifests over the three planning files.
- [x] Use a different fresh isolated built-in `default` reviewer for every submission with `using-agent-skills`, `code-review-and-quality`, `api-and-interface-design`, and `security-and-hardening`.
- [x] Require `CLEAN` or typed actionable findings; provide no prior reviewer conclusions.
- [x] Apply bounded Remediations 1-3 and rerun planning verification after each.
- [ ] Submit Review 4 only as the planning-closure review over Remediation 3.
- [ ] After Review 4, stop without canonical design edits or completion and write the exact non-completion handoff; Review 5 is forbidden.

## Phase 2: Canonical design freeze

### Task 3: Freeze architecture and semantic boundaries

**Description:** Update affected 00-04 sections with the frozen lifecycle/evaluation separation, claim/applicability rules, evidence/Resolution limits, witness authority, dock posture, first proof target, and accurate runtime classification.

**Acceptance criteria:**
- [ ] HCM-0.2/0.3/0.4 frozen semantics are unchanged.
- [ ] Validator witness and Resolution proof limits are consistent across 01-04.
- [ ] HCM-0.5 exit rules and JSON Schema first-proof selection are explicit.
- [ ] HCM-0.6 remains unstarted/unselected.

**Verification:** targeted cross-file assertions and scoped diff inspection.

**Dependencies:** Checkpoint A Review 3 `CLEAN`. This dependency is not satisfied on the live Review 3 findings branch, so Tasks 3-10 must not execute.

**Files likely touched:** `00-README.md`, `01-target-architecture.md`, `02-semantic-model.md`, `03-seam-crosswalk.md`, `04-phase-slice-map.md`.

**Estimated scope:** M (documentation-only).

### Task 4: Freeze canonical contract and dock schemas/rules

**Description:** Replace only the preliminary HCM-0.5 sections in canonical `05` with exact field tables, lifecycle/claim/applicability/evidence/verdict/gate rules, manifest and process request/result contracts, execution/failure matrices, validator authority boundary, first proof-dock contract, and ordinary operation definitions.

**Acceptance criteria:**
- [ ] Every field has owner/default or omission/validation/non-goal semantics.
- [ ] Contract identity/version compatibility, exact lifecycle adjacency/transitions/authorities, all-of per-kind evidence cardinality, repeated-evidence consistency/precedence, score omission, and gate-effect/verdict compatibility are closed matrices.
- [ ] State machines and failure/refusal behavior are total and fail closed.
- [ ] Process JSON framing, content-addressed implementation/runtime selection, default-deny isolation/no-network/timeout/cancellation rules are executable without guessing.
- [ ] Rust-native future transport preserves identical semantic DTO/evidence meaning.
- [ ] HCM-0.4 sections are not modified except the preauthorized ordinary catalog extension point.

**Verification:** fenced-example parse harness; lifecycle/partition/failure/operation matrix assertions; prior-section hash comparisons where feasible.

**Dependencies:** Task 3.

**Files likely touched:** `05-contracts-schemas-and-gates.md`.

**Estimated scope:** M (one canonical file, large semantic change).

### Task 5: Freeze proof and regression authority

**Description:** Add the HCM-0.5 documentation-freeze proof gate and permanent regression rules while keeping PG-CONTRACT-01, PG-DOCK-01, and PG-GATE-01 open.

**Acceptance criteria:**
- [ ] Proof gate covers every objective and negative case.
- [ ] No runtime/classification proof is promoted.
- [ ] Canonical monolith and no-leaf boundary are asserted.

**Verification:** gate/row/regression assertions plus no-runtime promotion scan.

**Dependencies:** Task 4.

**Files likely touched:** `06-proof-and-regression-ledger.md`.

**Estimated scope:** S.

### Checkpoint B: Design verification

- [ ] Complete every SPEC proof-wall item 1-12 and capture its result before constructing any final-design Review 4 dispatch.
- [ ] Parse all new JSON/YAML examples.
- [ ] Run identity/SemVer, exact lifecycle adjacency/transition/authority, applicability, all-of per-kind evidence cardinality/precedence, score omission, exhaustive gate-effect x verdict, implementation-substitution, protocol, isolation/no-network, and ordinary-operation assertions.
- [ ] Run HCM-0.2/0.3/0.4 regression assertions.
- [ ] Run Markdown links/anchors/fences, archive boundary, three handoff validator modes, no-Rust/no-HCM-0.6/no-leaf checks, `git diff --check`, and scoped diff inspection.

## Phase 3: Independent final review and remediation

### Task 6: Submit the complete design subject to a fresh reviewer

**Description:** Assemble a fresh immutable review dispatch over the complete HCM-0.5 packet, affected 00-06 files, and proof results. Use a fresh isolated built-in `default` reviewer with no implementation narrative or previous findings.

**Acceptance criteria:**
- [ ] Subject manifest and aggregate fingerprint recompute.
- [ ] Reviewer checks correctness, API stability, security/isolation, owner boundaries, Resolution semantics, HCM-0.4 regression, and proof coverage.
- [ ] Result is `CLEAN` or provides typed, bounded findings.

**Verification:** parent recomputes manifest and validates result fingerprint/status.

**Dependencies:** Review 3 `CLEAN` and Checkpoint B items 1-12 green with captured results. These dependencies are not satisfied on the live path.

**Files likely touched:** one immutable review dispatch; no reviewer edits.

**Estimated scope:** S.

### Task 7: Enforce the terminal Review 4 branches

**Description:** Review 4 is terminal. On the alternate Review 3-clean path, Review 4 reviews the complete final design after the full proof wall; `CLEAN` permits only byte-identical proof replay/staging, while any finding stops without completion. On the live Review 3-findings path, Review 4 reviews only Remediation 3 planning bytes; every outcome stops without canonical design edits or completion because no independent final-design review remains.

**Acceptance criteria:**
- [ ] Immutable lineage contains distinct Reviews 1-4 and no duplicate round number.
- [ ] No Review 4 finding is remediated and presented as review-clean without an independent review.
- [ ] No Review 5 is scheduled or invoked.
- [ ] Completion is possible only when Review 3 was `CLEAN` and final-design Review 4 is `CLEAN` over bytes produced after the complete proof wall.
- [ ] The live Review 3-findings branch ends with a non-completion handoff after planning Review 4, regardless of its verdict.

**Verification:** mechanically assert the Review 1-4 lineage, the two Review 3 branches, every Review 4 outcome, the final-design-clean completion precondition, and rejection of Review 5 scheduling.

**Dependencies:** Review 4 result. This task makes no subject edit.

**Files likely touched:** one exact-status v1.2 non-completion handoff and the deterministic ledger on every terminal non-completion branch; no reviewed subject edit.

**Estimated scope:** bounded by finding.

## Phase 4: Proof, commit, and closeout

### Task 8: Replay proof byte-identically and run staged scope detection

**Description:** On the alternate final-design-`CLEAN` branch only, rerun every Checkpoint B proof against byte-identical reviewed bytes, then stage those exact bytes and run repository-required GitNexus change detection. This is a replay/staging gate, not the first complete proof wall, and it may not repair or otherwise change the subject after Review 4.

**Acceptance criteria:**
- [ ] The complete Checkpoint B proof wall and captured results existed before the final-design Review 4 dispatch.
- [ ] All proof commands pass again on byte-identical reviewed bytes.
- [ ] GitNexus reports documentation-only expected scope.
- [ ] Staged paths contain only reviewed HCM-0.5 subject/dispatch evidence.
- [ ] The staged subject fingerprint equals the Review 4 subject fingerprint exactly.

**Verification:** captured command results, staged diff, `git diff --cached --check`, staged change detection.

**Dependencies:** Review 3 `CLEAN`, Checkpoint B green before dispatch, and final-design Review 4 `CLEAN`. These dependencies are not satisfied on the live path.

**Files likely touched:** None beyond proof artifacts already in approved scope.

**Estimated scope:** S.

### Task 9: Commit reviewed HCM-0.5 planning and design authority

**Description:** Stage only the reviewed subject and immutable proof-relevant HCM-0.5 dispatches, then create one scoped documentation commit.

**Acceptance criteria:**
- [ ] Primary commit contains the exact final reviewed subject.
- [ ] Commit message explains the HCM-0.5 semantic freeze.
- [ ] Worktree contains no accidental unrelated staged changes.

**Verification:** `git show --stat --oneline HEAD`; replay final manifest against the primary commit.

**Dependencies:** Task 8.

**Files likely touched:** Git index only.

**Estimated scope:** XS.

### Task 10: Create the completed v1.2 handoff and mechanical closeout commit

**Description:** Create one parent-owned completed handoff bound to the primary commit/final review, rebuild the ledger deterministically, run every validator mode and mechanical diff check, run GitNexus change detection, and commit only the handoff/ledger closeout.

**Acceptance criteria:**
- [ ] Handoff uses `status: completed`, `stop_reason: completed`, and `resume.execution_target: none`.
- [ ] It records planning/final review lineage and remediations exactly.
- [ ] Ledger rebuild is byte-identical and all validator modes pass.
- [ ] Second commit contains only the new record and ledger mutation.
- [ ] HCM-0.6 is not started.

**Verification:** three validator modes; exact record/index parity; final `git status`; two commit hashes.

**Dependencies:** Task 9.

**Files likely touched:** one `handoffs/records/*.json`, `handoffs/ledger.jsonl`.

**Estimated scope:** S.

## Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| Lifecycle/evaluation states remain conflated | High | explicit separate state machines and mechanical vocabulary assertions |
| Dock output is mistaken for canonical evidence | High | candidate -> membrane validation -> immutable evidence boundary; reviewer gate |
| Process dock receives ambient host authority | High | default-deny manifest/grant intersection, unconditional v1 network denial, and negative isolation matrix |
| Dock executable/runtime substitution preserves a trusted identity | High | content-addressed bundle manifest, exact entrypoint/runtime closure, host allowlist mapping, and pre-spawn digest verification |
| HCM-0.4 transport contracts drift | High | append only named operation definitions; compare frozen sections and review explicitly |
| First dock is selected by implementation inertia | High | evidence-backed three-candidate comparison and narrow selection criteria |
| Review budget exhausted | Medium | prioritize correctness/authority/security findings; Review 4 only for bounded corrections; honest handoff otherwise |
| Monolith size makes review unfocused | Medium | exact section ranges and a bounded manifest; no decomposition or leaf creation |

## Open questions

None at planning time. If live review establishes that JSON Schema first-dock selection needs product authority beyond the HCM-0.5 objective and existing pack/repository evidence, stop with `human_input` and present the candidate table rather than choosing implicitly.
