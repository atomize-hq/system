# HCM-0.9 Execution Checklist

**Status:** not started; approved packet does not authorize execution in the planning session  
**Baseline:** `214a5b8eb182fce74478df49d4f55d226d65fdf5` / `c7f61db209a81ba20690f365b4069dd01f11e395335bfa10d2ce21143cc2985d`

## 1. Preflight and immutable baseline

- [ ] Invoke `using-agent-skills` first and apply the packet skill chain.
- [ ] Confirm repository root, branch, HEAD, and clean/attributable status.
- [ ] Confirm `214a5b8...` is an ancestor of the execution start.
- [ ] Confirm frozen `05` full-file SHA-256.
- [ ] Derive and compare the frozen 48 H2 / 22 H3 inventory.
- [ ] Inventory all live mutable `05` references.
- [ ] Confirm the immutable roots have no untracked files.
- [ ] Write `evidence/execution-start.json` with exact start HEAD, baseline commit, and immutable roots.
- [ ] Derive every pre-existing handoff/dispatch path and blob hash from the recorded start HEAD.
- [ ] Stop on mismatch or unsafe overlap.

## 2. Parity/routing verifier

- [ ] Create `slices/HCM-0.9/verify_contract_catalog.py`.
- [ ] Encode the eleven ordered leaf paths and exact H2 ownership map.
- [ ] Verify exact baseline commit/SHA and H1/body boundary.
- [ ] Verify the execution-start record and every pre-existing history blob while allowing additive HCM-0.9 artifacts.
- [ ] Verify contiguous leaf spans and byte-identical reconstruction.
- [ ] Verify all H2/H3 headings, anchors, links, fences, and parseable YAML/JSON.
- [ ] Verify no duplicated semantic payload in the index.
- [ ] Classify every frozen positional and named contract dependency and require the exact source-triggered routing table for leaves 8-11.
- [ ] Add negative self-tests for wrong SHA, missing/duplicate section, changed byte, broken alias/fence, every omitted/broken leaf-8/9/10/11 dependency group, unjustified extra or unclassified cross-leaf dependency, modified/deleted/renamed history, and allowed additive history.
- [ ] Run verifier self-tests.

## 3. Leaf shadows 01-04

- [ ] Generate `contracts/01-schema-profile-and-artifact-registry.md`.
- [ ] Generate `contracts/02-intake-charter-and-validation.md`.
- [ ] Generate `contracts/03-vocabulary-and-context-resolution.md`.
- [ ] Generate `contracts/04-projection-contracts.md`.
- [ ] Run bounded group parity/fence checks.
- [ ] Keep current `05` canonical; do not commit partial split.

## 4. Leaf shadows 05-08

- [ ] Generate `contracts/05-snapshot-memory-contracts.md`.
- [ ] Generate `contracts/06-posture-and-synthesis-contracts.md`.
- [ ] Generate `contracts/07-development-orchestration-contracts.md`.
- [ ] Generate `contracts/08-sdk-operations-and-capability-discovery.md`.
- [ ] Add leaf 8's exact combined ordered cross-leaf dependency block.
- [ ] Run bounded group parity/fence checks.
- [ ] Keep current `05` canonical; do not commit partial split.

## 5. Leaves 09-11 and stable-index cutover

- [ ] Generate `contracts/09-machine-transport-and-adapter-contracts.md`.
- [ ] Generate `contracts/10-substrate-integration-and-publication.md`.
- [ ] Generate `contracts/11-contract-evidence-gates-and-docks.md`.
- [ ] Prove all eleven leaf payloads reconstruct the frozen body byte-for-byte.
- [ ] Replace `05` body with routing/compatibility metadata only.
- [ ] Preserve unchanged H1.
- [ ] Add ordered leaf catalog and exact H2 map.
- [ ] Add exactly one forwarding alias for every frozen H2/H3 anchor.
- [ ] Add the exact combined ordered dependency blocks for leaves 9, 10, and 11.
- [ ] Confirm every positional or named contract dependency is same-leaf, non-layout-dependent, or explicitly source-triggered and routed.
- [ ] Verify every alias target and relative link.
- [ ] Confirm no baseline fence/normative payload remains duplicated in the index.

## 6. Mutable semantic/proof routing

- [ ] Update `00-README.md` to expose index/leaves/selective loading.
- [ ] Update both `02-semantic-model.md` references to exact leaves.
- [ ] Update the `03` catalog-topology row with proven current truth only.
- [ ] Preserve HCM-0.4 -> HCM-0.9 -> HCM-0.5 ordering in `04`.
- [ ] Route HCM-0.2 proof refs to leaves 01, 02, 03, and 06 as applicable.
- [ ] Route HCM-0.3 proof refs to leaves 03, 04, and 05 as applicable.
- [ ] Route HCM-0.4 proof refs to leaves 08, 09, 10, and 11 as applicable.
- [ ] Close only `PG-CATALOG-01` when structural proof exists.
- [ ] Confirm no runtime gate promotion.

## 7. Orchestration/dispatch/handoff routing

- [ ] Update `07` to load minimum exact contract leaves.
- [ ] Apply dependency-table targets only when their triggering source contract is in scope.
- [ ] Update `08` to require exact leaf refs in new dispatches/handoffs.
- [ ] Preserve historical monolith refs as immutable evidence.
- [ ] Update `handoffs/internal-dispatch-template.json` to the leaf-07 example.
- [ ] Update `handoffs/dispatch-template.md` to request exact leaf path/anchor.
- [ ] Confirm no handoff/dispatch schema change.
- [ ] Confirm every delegated dispatch begins `required_skills` with `using-agent-skills`.

## 8. Complete proof wall

- [ ] Run full parity/routing verifier.
- [ ] Run verifier negative self-tests at the final subject fingerprint.
- [ ] Validate Markdown links and fragments.
- [ ] Validate all frozen/leaf anchors.
- [ ] Validate Markdown fences and identical fence bodies.
- [ ] Parse every baseline-parseable YAML/JSON example.
- [ ] Run applicable HCM-0.2/HCM-0.3/HCM-0.4 semantic assertions.
- [ ] Run archive-boundary check and self-test.
- [ ] Run normal handoff validation.
- [ ] Run v1 admission self-test.
- [ ] Run orchestration-contract self-test.
- [ ] Replay review subject manifests.
- [ ] Compare all start-commit handoff/dispatch Git blobs through the persisted execution-start record.
- [ ] Prove modified/deleted/renamed history fails and a new additive HCM-0.9 dispatch passes.
- [ ] Run `git diff --check`.
- [ ] Inspect exact scoped diff and status.
- [ ] Confirm no semantic correction occurred.
- [ ] Confirm no Rust/Cargo/runtime/public API/schema/HCM-0.5/unrelated change.

## 9. Fresh independent review loop

- [ ] Assemble complete-subject Review 1 with full diff, authority, baseline, packet, non-goals, verification, and budget.
- [ ] Spawn fresh isolated built-in `default` reviewer.
- [ ] Require findings first: Critical, Required, Optional, Nit.
- [ ] Stop immediately if CLEAN.
- [ ] If actionable, validate and remediate Round 1; rerun proof wall.
- [ ] Assemble Review 2 with the new subject fingerprint.
- [ ] Spawn a different fresh isolated reviewer.
- [ ] Stop immediately if CLEAN.
- [ ] If actionable, validate and remediate Round 2; rerun proof wall.
- [ ] Assemble Review 3 and spawn a different fresh reviewer.
- [ ] Stop immediately if CLEAN.
- [ ] If actionable, validate and remediate Round 3; rerun proof wall.
- [ ] Assemble Review 4 and spawn a different fresh reviewer.
- [ ] Stop immediately if CLEAN.
- [ ] Do not run Review 5.
- [ ] On unresolved valid Critical/Required findings, write the partial human-input handoff; do not self-approve or authorize HCM-0.5.

## 10. Reviewed execution commit

- [ ] Confirm final CLEAN fingerprint matches current bytes.
- [ ] Stage only reviewed HCM-0.9 execution/control artifacts and additive review dispatches.
- [ ] Run `npx gitnexus detect-changes -r handbook -s staged`.
- [ ] Run `git diff --cached --check`.
- [ ] Inspect cached name-status and full diff.
- [ ] Confirm zero affected runtime processes and no unrelated paths.
- [ ] Commit with scoped Conventional Commit.
- [ ] Record exact execution commit.

## 11. Mechanical handoff/ledger closeout

- [ ] Create one v1.2 parent handoff for HCM-0.9 execution.
- [ ] Record exact execution commit, packet paths, review lineage/CLEAN verdict, verification, and structural-only claim.
- [ ] Do not claim semantic correction, runtime completion, or HCM-0.5 start.
- [ ] Rebuild deterministic ledger from canonical records.
- [ ] Run all three handoff validator modes.
- [ ] Run closeout `git diff --check` and staged scope inspection.
- [ ] Commit only new handoff/ledger mechanical artifacts.
- [ ] Return execution commit, closeout commit, handoff ID, and exact selector.

## Mandatory stop reminders

- [ ] Stop on baseline mismatch or unattributable overlap.
- [ ] Stop rather than changing semantic payload.
- [ ] Stop rather than widening outside the packet.
- [ ] Stop if historical bytes drift.
- [ ] Stop if mandatory validation or fresh review is unavailable.
- [ ] Stop at review budget; never self-approve or weaken findings.
- [ ] Never start HCM-0.5 from this checklist.
