# HCM-0.9 Execution Checklist

> Planning checklist only. Do not execute HCM-0.9 in the redesign-planning
> session. The monolithic `05` remains canonical until an approved execution
> session completes the full cutover.

## 1. Preflight and immutable baseline

- [ ] Confirm exact branch, HEAD, worktree, and attributable dirt.
- [ ] Confirm baseline commit ancestry and full-file SHA-256.
- [ ] Derive body SHA-256, byte/line counts, 48 H2, and 22 H3.
- [ ] Derive all eight exact span digests from frozen Git bytes.
- [ ] Inventory live mutable `05` references.
- [ ] Confirm history roots contain no untracked files.
- [ ] Write `evidence/execution-start.json` with exact HEAD/baseline/roots.

## 2. Mechanical verifier

- [ ] Add `verify_contract_catalog.py` with no network dependency.
- [ ] Validate baseline identity and eight-span inventory.
- [ ] Validate each inventory-defined exact H1 plus one blank line and reject a
  missing, changed, duplicate, or additional scaffold byte.
- [ ] Validate exact payload reconstruction.
- [ ] Validate H2/H3 ownership, order, and anchors.
- [ ] Validate index contents and all 70 compatibility aliases/targets.
- [ ] Validate changed links, fence sequence/bodies, and parseable examples.
- [ ] Validate immutable historical paths/blobs and scope.
- [ ] Validate review-manifest equality to every changed non-dispatch path from
  execution-start HEAD, with only additive HCM-0.9 review dispatches exempt.
- [ ] Add omitted-changed-manifest-path and forbidden runtime/HCM-0.5 negative
  fixtures plus all other specified self-tests.
- [ ] Confirm no trigger/route/co-activation/operation-fixture/semantic-dependency
  or inferred-selection logic exists.

## 3. Eight non-canonical leaf shadows

- [ ] Generate leaves 01-08 from exact frozen byte spans.
- [ ] Give each leaf only its approved H1, one blank line, and payload.
- [ ] Verify each span digest and heading assignment.
- [ ] Verify no H3 or fence crosses a boundary.
- [ ] Prove all eight payloads reconstruct the frozen body byte-for-byte.
- [ ] Keep the monolith canonical through this checkpoint.

## 4. Stable-index cutover

- [ ] Replace the `05` body only after complete shadow parity passes.
- [ ] Preserve the exact H1.
- [ ] Add ordered eight-leaf discovery table.
- [ ] Add exact H2/H3 ownership table and 70 compatibility aliases/links.
- [ ] Verify every leaf path and fragment.
- [ ] Verify no normative frozen payload or fence is duplicated in the index.
- [ ] State that future packets/dispatches explicitly select leaf authority and
  the index performs no inference.

## 5. Active control-pack guidance

- [ ] Update only live mutable refs proven necessary by inventory.
- [ ] Make future slice packets list exact leaf paths/anchors.
- [ ] Make dispatches repeat explicit selection in `authority_refs` and/or
  `contracts_and_gates`.
- [ ] Keep `subject_manifest` limited to current bytes under review.
- [ ] Manifest changed index/leaves/verifier/control files when reviewed.
- [ ] Keep unchanged contextual authority outside the manifest.
- [ ] Preserve all pre-existing handoff/dispatch bytes.
- [ ] Do not add automatic semantic routing or selection closure.

## 6. Complete proof wall

- [ ] Run verifier self-tests and full verifier.
- [ ] Run archive boundary check and self-test.
- [ ] Run normal, v1 admission, and orchestration handoff validators.
- [ ] Replay all current review manifests/fingerprints.
- [ ] Compare each manifest to every changed non-dispatch path at its review
  boundary and validate the additive review-dispatch exemption separately.
- [ ] Verify immutable history against execution-start HEAD.
- [ ] Enforce the exact approved path allowlist and run `git diff --check`.
- [ ] Run omitted-manifest-path and forbidden runtime/HCM-0.5 negative fixtures.
- [ ] Confirm no Rust/runtime/API/schema/HCM-0.5/unrelated change.

## 7. Fresh independent execution review

- [ ] Create complete-subject review dispatch with `required_skills` beginning
  `using-agent-skills`, then `code-review-and-quality`.
- [ ] Use a fresh isolated built-in `default` reviewer.
- [ ] Validate actionable findings against live authority.
- [ ] After remediation, rerun the full proof wall and use a different fresh
  reviewer.
- [ ] Stop immediately on CLEAN; never exceed four reviews/three remediations.
- [ ] Do not self-approve or weaken findings.

## 8. Reviewed execution commit

- [ ] Stage only reviewed HCM-0.9 execution/control files and review dispatches.
- [ ] Run staged GitNexus change detection.
- [ ] Run cached whitespace, scope, and diff inspection.
- [ ] Commit one documentation/control-only execution change.

## 9. Mechanical handoff/ledger closeout

- [ ] Create one v1.2 parent handoff recording exact execution commit, proof,
  reviewer lineage, CLEAN result, and structural-only scope.
- [ ] Do not activate HCM-0.5.
- [ ] Rebuild and validate the ledger deterministically.
- [ ] Commit only the handoff record and ledger separately.

## Mandatory stop reminders

- [ ] Stop and retain the monolith if complete parity cannot be proven.
- [ ] Stop rather than correct frozen semantic bytes.
- [ ] Stop rather than infer missing leaf authority for a future packet.
- [ ] Never leave a partial decomposition canonical.
- [ ] Never implement Rust or begin HCM-0.5 here.
