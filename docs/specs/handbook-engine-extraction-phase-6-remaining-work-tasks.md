# Tasks: Handbook Engine Extraction — Phase 6 Remaining Work

Plan reference: [handbook-engine-extraction-phase-6-remaining-work-plan.md](./handbook-engine-extraction-phase-6-remaining-work-plan.md)

Spec reference: [handbook-engine-extraction-phase-6-remaining-work-spec.md](./handbook-engine-extraction-phase-6-remaining-work-spec.md)

---

## Lane B: Flow Import-Boundary Proof

### Packet 6.B.1: Gather Evidence

- [ ] Task: Capture dependency-tree evidence for `handbook-flow`
  - Acceptance: `cargo tree -p handbook-flow` output recorded, showing only `handbook-engine` as intra-workspace dependency.
  - Verify: `cargo tree -p handbook-flow`
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (evidence section)

- [ ] Task: Capture coupling-exclusion evidence
  - Acceptance: `rg` output recorded, showing zero matches for `handbook_compiler`, `handbook_cli`, `handbook_pipeline` in `crates/flow/src/` and `crates/flow/tests/`.
  - Verify: `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/`
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (evidence section)

- [ ] Task: Trace transitive type dependencies for all in-boundary symbols
  - Acceptance: For each public symbol in the in-boundary set (from `resolver`, `budget`, `packet_result`), its type dependencies are recorded and confirmed to resolve only to `handbook-engine` types or std types. Any symbol whose implementation pulls in engine types beyond what is already in the engine public surface is flagged.
  - Verify: Source inspection of `crates/flow/src/resolver.rs`, `crates/flow/src/budget.rs`, `crates/flow/src/packet_result.rs` cross-referenced against `crates/engine/src/lib.rs` exports.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (transitive dependency table)

- [ ] Task: Confirm exclusion of CLI/compiler/doctor/setup/pipeline concerns
  - Acceptance: Evidence recorded confirming that no in-boundary symbol's implementation touches CLI shell behavior, compiler rendering/refusal/error glue, doctor/setup concerns, or pipeline loading/selection/compile/capture/handoff/route surfaces.
  - Verify: Source inspection of `crates/flow/src/*.rs` — confirm `use` statements reference only `crate::*` and `handbook_engine::*` and std.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (exclusion section)

### Packet 6.B.2: Formalize Consumer Contract

- [ ] Task: Write the `handbook-flow` import-boundary consumer contract document
  - Acceptance: A standalone doc at `docs/specs/handbook-flow-import-boundary-consumer-contract.md` that records:
    - The frozen in-boundary symbol set (all public re-exports from `budget`, `packet_result`, `resolver`)
    - Their transitive type dependencies (all engine-only or std)
    - Explicit exclusions (CLI shell, compiler glue, doctor/setup, pipeline surfaces)
    - The contract version function (`flow_contract_version()`)
    - Evidence references (cargo tree output, rg output, source inspection conclusions)
  - Verify: Doc exists and is internally consistent with live source.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md`

### Packet 6.B.3: Verification Wall

- [ ] Task: Run the Lane B verification wall
  - Acceptance: All of the following pass:
    - `cargo tree -p handbook-flow` shows only `handbook-engine` as intra-workspace dependency
    - `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` returns zero matches
    - `cargo test -p handbook-flow` passes
    - `cargo check --workspace` passes
    - `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` passes
  - Verify: Run each command and record pass/fail.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (completion notes)

---

## Lane C: Engine Optional Boundary Freeze (Optional — Currently Deferred)

### Packet 6.C.1: Defer Or Activate (Decision Task)

- [ ] Task: Record Lane C deferral decision
  - Acceptance: The tasks doc explicitly records that Lane C is deferred — engine's current public surface is the working boundary. If later review (e.g., from Lane D's import plan) indicates a narrower surface is needed, Lane C can be activated at that time.
  - Verify: This section states "deferred" with rationale.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`

---

## Lane D: Final Substrate Import Plan

### Packet 6.D.1: Write Import/Adoption Plan

- [ ] Task: Write the phased import plan for engine + pipeline + flow
  - Acceptance: A standalone doc at `docs/specs/handbook-substrate-import-adoption-plan.md` that records:
    - Import order: engine first (no intra-workspace deps), then pipeline (depends on engine), then flow (depends on engine)
    - Rationale for the phased order
    - Per-crate frozen boundary summary:
      - Engine: current public surface (Lane C deferred)
      - Pipeline: documented frozen subset from Lane A closeout (in-boundary modules listed)
      - Flow: Lane B consumer contract (in-boundary symbols listed)
    - Adapter/facade assessment (current evidence: none needed; record the assessment with evidence)
    - Import verification gate per phase (what checks Substrate must pass after importing each crate)
    - Substrate-side constraints (resolved from live repo inspection, 2026-06-17):
      - License field: add `license = "MIT"` to the three crate Cargo.toml files before import
      - Workspace integration: recommend workspace member pattern (path deps) vs external dep
      - YAML crate divergence: `serde_yaml_bw` (handbook) vs `serde_yaml` (substrate) — record keep-both or migrate decision
      - No feature flags needed; edition/resolver/sha2/libc/serde all compatible
  - Verify: Doc exists and is consistent with the three crate surfaces and the Lane B consumer contract.
  - Files: `docs/specs/handbook-substrate-import-adoption-plan.md`

### Packet 6.D.2: Human Review Gate

- [ ] Task: Human review of the import plan
  - Acceptance: The plan has been reviewed by a human engineer who confirms it is consistent with live crate surfaces, frozen boundaries, and the root plan's migration gate. Any review feedback is addressed.
  - Verify: Human sign-off recorded in this tasks doc.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (completion note)

---

## Wider-Seam Guardrail

Stop after Lane B and Lane D land. Do not:
- Execute the actual Substrate import
- Reopen Lane A
- Widen into CLI shell redesign, compiler retirement, publication, or crates.io work
- Make `substrate-context` become handbook
- Introduce compatibility aliases as a long-term architecture substitute

Those remain outside Phase 6 scope and require separate authority.

---

## Lane Status Summary

| Lane | Status | Blocks Lane D? |
|------|--------|----------------|
| A | Closed (2026-06-17) | N/A — done |
| B | Pending | Yes |
| C | Deferred (optional) | No |
| D | Pending (after B) | — |
