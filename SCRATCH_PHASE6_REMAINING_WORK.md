# Phase 6 Remaining Work — Scratch Plan

## Next Steps (in order)

### 1. Close out Lane A — Pipeline boundary cleanup

- [ ] Update the pipeline-boundary-cleanup tasks doc to reflect that Implementation Packet 1 landed (commit `2dfb9b7`)
- [ ] Run the full verification wall:
  ```
  cargo test -p handbook-pipeline --test pipeline_catalog
  cargo test -p handbook-pipeline --test pipeline_compile
  cargo test -p handbook-pipeline --test pipeline_capture
  cargo test -p handbook-pipeline --test pipeline_handoff
  cargo test -p handbook-compiler --test author
  cargo check --workspace
  ```
- [ ] Make the durable boundary decision: narrower public facade vs. documented frozen subset of the current public surface
- [ ] Record the decision and close out the slice

### 2. Docs hygiene / archive pass — DONE

- Keep active: `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`, `docs/specs/handbook-engine-extraction-slice-map.md`, `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- Archive: all landed Phase 1–5 slice triplets, all old packet-prompt artifacts for landed work, the Phase 6 Slice 6.1 packet artifacts, likely the current Phase 6 packet-prompt artifacts, possibly the current Phase 6 planning triplets after lifting still-valid conclusions
- Add: one archive index / map

### 3. Write fresh remaining-work triplet

- `docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
- `...plan.md`
- `...tasks.md`
- State up front: target Substrate crate set is engine + pipeline + flow; handbook-cli is out; retained handbook-compiler is not an import target; handbook-engine is import-ready; handbook-pipeline needs boundary freeze; handbook-flow needs import-boundary proof

### 4. Execute Lane B — Flow import-boundary proof

- Prove Substrate can consume `resolve`, `ResolveRequest`, `ResolverResult`, `budget`, `packet_result` without dragging in CLI shell behavior, compiler rendering/refusal/error glue, or doctor/setup concerns
- Formalize the stable consumer contract
- Run the verification wall

### 5. Execute Lane D — Final Substrate import plan

- Write the actual import/adoption plan for engine + pipeline + flow
- (Lane C — engine optional boundary freeze — is optional and not a blocker)
