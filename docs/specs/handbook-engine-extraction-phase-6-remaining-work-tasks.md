# Tasks: Handbook Engine Extraction — Phase 6 Remaining Work

Plan reference: [handbook-engine-extraction-phase-6-remaining-work-plan.md](./handbook-engine-extraction-phase-6-remaining-work-plan.md)

Spec reference: [handbook-engine-extraction-phase-6-remaining-work-spec.md](./handbook-engine-extraction-phase-6-remaining-work-spec.md)

---

## Lane B: Flow Required-Import Boundary Cleanup + Contract Freeze

### Packet 6.B.1: Gather Evidence

- [ ] Task: Capture dependency-tree evidence for `handbook-flow`
  - Acceptance: `cargo tree -p handbook-flow` output recorded, showing only `handbook-engine` as the intra-workspace dependency.
  - Verify: `cargo tree -p handbook-flow`
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (evidence section)

- [ ] Task: Capture coupling-exclusion evidence
  - Acceptance: `rg` output recorded, showing zero matches for `handbook_compiler`, `handbook_cli`, `handbook_pipeline` in `crates/flow/src/` and `crates/flow/tests/`.
  - Verify: `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/`
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (evidence section)

- [ ] Task: Trace transitive type dependencies for all in-boundary symbols
  - Acceptance: For each public symbol in the in-boundary set (from `resolver`, `budget`, `packet_result`), its type dependencies are recorded and confirmed to resolve only to `handbook-engine` public types, std types, or flow-local types.
  - Verify: Source inspection of `crates/flow/src/resolver.rs`, `crates/flow/src/budget.rs`, `crates/flow/src/packet_result.rs` cross-referenced against `crates/engine/src/lib.rs` exports.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (transitive dependency table)

- [ ] Task: Record residual shell-ownership leakage separately from the clean import boundary proof
  - Acceptance: The evidence section explicitly records that the crate/type dependency boundary is clean **and** that final shell-owned/operator-facing copy still leaks through the live flow import surface. It must distinguish typed next-action/status semantics that may remain machine-readable from final shell wording/command strings that Packet 6.B.2 must move out.
  - Verify: Source inspection cross-referencing `crates/flow/src/resolver.rs`, `crates/flow/src/packet_result.rs`, `crates/cli/src/rendering.rs`, and `crates/compiler/src/rendering/shared.rs`.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (evidence section)

### Packet 6.B.2: Clean Flow Import-Surface Shell Ownership

- [ ] Task: Remove final shell-owned/operator-facing copy from the public flow import surface
  - Acceptance: `handbook-flow` no longer returns final shell command strings or product-shell action wording from its public import surface. Any remaining next-action data exposed by flow is typed/machine-readable rather than final rendered shell copy.
  - Verify: Source inspection of `crates/flow/src/resolver.rs` and `crates/flow/src/packet_result.rs`; `rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/`; `cargo test -p handbook-flow`.
  - Files: `crates/flow/src/resolver.rs`, `crates/flow/src/packet_result.rs`, impacted flow tests if needed

- [ ] Task: Keep CLI/compiler responsible for final shell rendering without widening into redesign
  - Acceptance: Any caller-side changes stay narrowly bounded to the rendering/adapter files required by the flow cleanup. Typed next-action/status semantics may remain, but final shell copy is rendered outside flow. No broader CLI shell redesign is introduced.
  - Verify: Source inspection of `crates/cli/src/rendering.rs`, `crates/compiler/src/rendering/shared.rs`; `cargo check --workspace`.
  - Files: `crates/cli/src/rendering.rs`, `crates/compiler/src/rendering/shared.rs`, impacted tests if needed

### Packet 6.B.3: Formalize Consumer Contract

- [ ] Task: Write the `handbook-flow` import-boundary consumer contract document against the cleaned surface
  - Acceptance: A standalone doc at `docs/specs/handbook-flow-import-boundary-consumer-contract.md` records:
    - The frozen in-boundary symbol set (public re-exports from `budget`, `packet_result`, `resolver`)
    - Their transitive type dependencies (engine-public, std, or flow-local only)
    - Which typed next-action/status semantics remain in-boundary after Packet 6.B.2
    - Which shell-owned/operator-facing copy and rendering responsibilities are explicitly out of boundary
    - The contract version function (`flow_contract_version()`)
    - Evidence references from Packet 6.B.1 and cleanup references from Packet 6.B.2
  - Verify: Doc exists and is internally consistent with live source.
  - Files: `docs/specs/handbook-flow-import-boundary-consumer-contract.md`

### Packet 6.B.4: Verification Wall

- [ ] Task: Run the Lane B verification wall
  - Acceptance: All of the following pass:
    - `cargo tree -p handbook-flow` shows only `handbook-engine` as the intra-workspace dependency
    - `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/` returns zero matches
    - Source inspection of the public `handbook-flow` surface (`crates/flow/src/lib.rs`, `crates/flow/src/budget.rs`, `crates/flow/src/packet_result.rs`, `crates/flow/src/resolver.rs`) confirms no final shell-owned/operator-facing copy remains on that surface; any remaining next-action/status data is typed/machine-readable only and stays within the Packet 6.B.3 consumer contract
    - `rg -n 'run `doctor`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/` returns zero matches as a supporting spot-check, not the sole proof
    - `cargo test -p handbook-flow` passes
    - `cargo check --workspace` passes
    - `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` passes
  - Verify: Run each command, perform and record the required source inspection, and record pass/fail for both the broader surface proof and the supporting grep spot-check.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (completion notes)

#### Packet 6.B.4 completion notes

- Pre-wall packet verification: **PASS**
  - **Packet 6.B.1** verified complete from `docs/specs/handbook-flow-import-boundary-consumer-contract.md` preserving the Packet 6.B.1 evidence section, including the dependency-tree evidence, coupling-exclusion evidence, transitive dependency table, and preserved pre-cleanup shell-leakage provenance.
  - **Packet 6.B.2** verified complete from the same contract doc's `Packet 6.B.2 Cleanup Outcome Reference`, plus live source inspection showing `PacketDecisionSummary.ready_next_safe_action` is now `ReadyPacketNextSafeAction`, `ResolverNextSafeAction` remains typed, and final shell wording is caller-owned in CLI/compiler rendering.
  - **Packet 6.B.3** verified complete because the contract doc status is `Packet: 6.B.3 — Formalize Consumer Contract`, and the documented frozen surface still matches the live `crates/flow/src/{lib,budget,packet_result,resolver}.rs` exports inspected for this packet.

- `cargo tree -p handbook-flow`: **PASS**
  - Exact output:
    ```text
    handbook-flow v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/flow)
    └── handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
        ├── libc v0.2.184
        ├── serde v1.0.228
        │   ├── serde_core v1.0.228
        │   └── serde_derive v1.0.228 (proc-macro)
        │       ├── proc-macro2 v1.0.106
        │       │   └── unicode-ident v1.0.24
        │       ├── quote v1.0.45
        │       │   └── proc-macro2 v1.0.106 (*)
        │       └── syn v2.0.117
        │           ├── proc-macro2 v1.0.106 (*)
        │           ├── quote v1.0.45 (*)
        │           └── unicode-ident v1.0.24
        ├── serde_yaml_bw v2.5.4
        │   ├── base64 v0.22.1
        │   ├── indexmap v2.13.1
        │   │   ├── equivalent v1.0.2
        │   │   └── hashbrown v0.16.1
        │   ├── itoa v1.0.18
        │   ├── num-traits v0.2.19
        │   │   [build-dependencies]
        │   │   └── autocfg v1.5.0
        │   ├── regex v1.12.3
        │   │   ├── aho-corasick v1.1.4
        │   │   │   └── memchr v2.8.0
        │   │   ├── memchr v2.8.0
        │   │   ├── regex-automata v0.4.14
        │   │   │   ├── aho-corasick v1.1.4 (*)
        │   │   │   ├── memchr v2.8.0
        │   │   │   └── regex-syntax v0.8.10
        │   │   └── regex-syntax v0.8.10
        │   ├── saphyr-parser-bw v0.0.611
        │   │   ├── arraydeque v0.5.1
        │   │   ├── smallvec v1.15.1
        │   │   └── thiserror v2.0.18
        │   │       └── thiserror-impl v2.0.18 (proc-macro)
        │   │           ├── proc-macro2 v1.0.106 (*)
        │   │           ├── quote v1.0.45 (*)
        │   │           └── syn v2.0.117 (*)
        │   ├── serde v1.0.228 (*)
        │   ├── unsafe-libyaml-norway v0.2.15
        │   └── zmij v1.0.21
        └── sha2 v0.10.9
            ├── cfg-if v1.0.4
            ├── cpufeatures v0.2.17
            │   └── libc v0.2.184
            └── digest v0.10.7
                ├── block-buffer v0.10.4
                │   └── generic-array v0.14.7
                │       └── typenum v1.19.0
                │       [build-dependencies]
                │       └── version_check v0.9.5
                └── crypto-common v0.1.7
                    ├── generic-array v0.14.7 (*)
                    └── typenum v1.19.0
    [dev-dependencies]
    └── tempfile v3.27.0
        ├── fastrand v2.4.1
        ├── getrandom v0.4.2
        │   ├── cfg-if v1.0.4
        │   └── libc v0.2.184
        ├── once_cell v1.21.4
        └── rustix v1.1.4
            ├── bitflags v2.11.0
            ├── errno v0.3.14
            │   └── libc v0.2.184
            └── libc v0.2.184
    ```
  - Result: only `handbook-engine` appears as the intra-workspace dependency.

- `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/`: **PASS**
  - Exact output: `(no matches; stdout empty; rg exit code 1)`

- Public `handbook-flow` source inspection against the Packet 6.B.3 consumer contract: **PASS**
  - `crates/flow/src/lib.rs` re-exports only `budget`, `packet_result`, and `resolver` symbols, plus `flow_contract_version()` delegating to `handbook_engine::workspace_contract_version()`.
  - `crates/flow/src/budget.rs` exposes typed budget outcomes and `NextSafeAction::ReduceCanonicalArtifactSize { canonical_repo_relative_path }`; no public rendered shell command strings remain.
  - `crates/flow/src/packet_result.rs` keeps ready-path action semantics typed via `ReadyPacketNextSafeAction::{InspectProof, Generate, RunDoctor}` and `PacketDecisionSummary.ready_next_safe_action: ReadyPacketNextSafeAction`; the remaining `summary_line: String` is a flow-owned status summary, not caller-shell rendering.
  - `crates/flow/src/resolver.rs` keeps refusal/blocker actions typed via `ResolverNextSafeAction` variants with typed payloads (`packet_id`, `canonical_repo_relative_path`) and uses `next_safe_action_for_ready_packet(...) -> ReadyPacketNextSafeAction`; the remaining public `summary: String` fields on `ResolverRefusal` and `ResolverBlocker` are status/diagnostic summaries, not final shell/product-shell command copy.
  - Conclusion: no final shell-owned/operator-facing command copy remains on the public `handbook-flow` surface. Remaining next-action/status semantics are typed and machine-readable where action routing matters, and the remaining summary strings stay within the Packet 6.B.3 contract as flow-owned status text rather than shell-rendered instructions.

- `rg -n 'run \`doctor\`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/`: **PASS**
  - Exact output: `(no matches; stdout empty; rg exit code 1)`
  - Supporting evidence only; not used as the sole proof.

- `cargo test -p handbook-flow`: **PASS**
  - Result summary:
    ```text
    running 11 tests
    test flow_resolver_blocks_missing_system_root_with_typed_refusal ... ok
    test flow_resolver_prioritizes_system_root_missing_over_live_execution_refusal ... ok
    test flow_resolver_refuses_when_budget_is_exhausted ... ok
    test flow_resolver_refuses_required_artifact_malformed_path_read_error ... ok
    test flow_resolver_refuses_symlinked_canonical_artifact_as_non_canonical_input ... ok
    test flow_resolver_refuses_live_execution_packets_without_fixture_backing ... ok
    test flow_resolver_summarizes_optional_sources_when_budget_demands_it ... ok
    test flow_resolver_blocks_optional_artifact_read_error_without_refusal ... ok
    test flow_resolver_builds_ready_planning_packet_body ... ok
    test flow_resolver_excludes_optional_sources_when_total_budget_demands_it ... ok
    test flow_resolver_builds_fixture_context_for_execution_demo_packets ... ok

    test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
    ```

- `cargo check --workspace`: **PASS**
  - Exact output:
    ```text
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.05s
    ```

- `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings`: **PASS**
  - Exact output:
    ```text
    Checking handbook-flow v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/flow)
    Checking handbook-compiler v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/compiler)
    Checking handbook-cli v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/cli)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 11.72s
    ```

- Packet 6.B.4 scope note:
  - Only this completion-notes subsection was updated in this packet.
  - No production code, Lane C, Lane D, publication work, or Substrate integration surfaces were changed.

---

## Lane C: Engine Optional Boundary Freeze (Optional — Currently Deferred)

### Packet 6.C.1: Defer Or Activate (Decision Task)

- [x] Task: Record Lane C deferral decision
  - Decision: Lane C is deferred. `handbook-engine`'s current public surface is the working boundary for the Phase 6 remaining-work seam.
  - Rationale: The spec keeps Lane C optional and only activates it if a stricter publishable API / narrower engine surface is later needed. If Lane D's import plan shows that the current engine public surface is too broad for Substrate adoption, Lane C can be activated then; otherwise the current engine public surface remains the accepted working boundary.
  - Acceptance: The tasks doc explicitly records that Lane C is deferred — engine's current public surface is the working boundary. If later review (e.g., from Lane D's import plan) indicates a narrower surface is needed, Lane C can be activated at that time.
  - Verify: This section states "deferred" with rationale.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`

---

## Lane D: Final Substrate Import Plan

### Packet 6.D.1: Write Import/Adoption Plan

- [x] Task: Write the phased import plan for engine + pipeline + flow
  - Acceptance: A standalone doc at `docs/specs/handbook-substrate-import-adoption-plan.md` records:
    - Import order: engine first (no intra-workspace deps), then pipeline (depends on engine), then flow (depends on engine)
    - Rationale for the phased order
    - Per-crate frozen boundary summary:
      - Engine: current public surface (Lane C deferred)
      - Pipeline: documented frozen subset from Lane A closeout (in-boundary modules listed)
      - Flow: Lane B consumer contract (clean import surface, typed semantics only where contract-approved, final shell copy out of boundary)
    - Adapter/facade assessment (current evidence: none needed beyond the Lane B cleanup; record the assessment with evidence)
    - Import verification gate per phase (what checks Substrate must pass after importing each crate)
    - Substrate-side constraints (resolved from live repo inspection, 2026-06-17):
      - License field: add `license = "MIT"` to the three crate Cargo.toml files before import
      - Workspace integration: recommend workspace member pattern (path deps) vs external dep
      - YAML crate divergence: `serde_yaml_bw` (handbook) vs `serde_yaml` (substrate) — record keep-both or migrate decision
      - No feature flags needed; edition/resolver/sha2/libc/serde all compatible
  - Verify: Doc exists and is consistent with the three crate surfaces and the Lane B consumer contract.
  - Files: `docs/specs/handbook-substrate-import-adoption-plan.md`
  - Completion note (2026-06-18): `docs/specs/handbook-substrate-import-adoption-plan.md` is present in repo truth and records the phased import order, per-crate boundary posture, adapter/facade assessment, per-phase verification gates, and resolved Substrate-side constraints. Live review confirmed consistency with `crates/{engine,pipeline,flow}/src/lib.rs`, the Lane A frozen-boundary status in the remaining-work spec, the Lane B consumer contract in `docs/specs/handbook-flow-import-boundary-consumer-contract.md`, and the root plan's migration-gate posture.

### Packet 6.D.2: Human Review Gate

- [ ] Task: Human review of the import plan
  - Acceptance: The plan has been reviewed by a human engineer who confirms it is consistent with live crate surfaces, frozen boundaries, and the root plan's migration gate. Any review feedback is addressed.
  - Verify: Human sign-off recorded in this tasks doc.
  - Files: `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (completion note)
  - Review-prep note (2026-06-18): Packet 6.D.1 is verified as review-ready. Human sign-off is still pending; this packet stops at review preparation and does not record approval on behalf of a human reviewer.

---

## Wider-Seam Guardrail

Stop after Lane B and Lane D land. Do not:
- Execute the actual Substrate import
- Reopen Lane A
- Widen into full CLI shell redesign, compiler retirement, publication, or crates.io work
- Make `substrate-context` become handbook
- Introduce compatibility aliases as a long-term architecture substitute

Those remain outside Phase 6 scope and require separate authority.

---

## Lane Status Summary

| Lane | Status | Blocks Lane D? |
|------|--------|----------------|
| A | Closed (2026-06-17) | N/A — done |
| B | Closed (Packet 6.B.4 recorded) | No |
| C | Deferred (optional) | No |
| D | 6.D.1 complete; 6.D.2 human review pending | — |
