# Handbook Substrate Import/Adoption Plan

## Status

- Packet: **6.D.1 — Write Import/Adoption Plan**
- Scope: planning artifact only for importing `handbook-engine`, `handbook-pipeline`, and `handbook-flow` into Substrate.
- Non-scope: no Substrate import execution, no production-code edits, no Lane B/Lane C implementation, no CLI/compiler/publication work.

## Prerequisite posture

- **Lane A:** closed. The frozen `handbook-pipeline` boundary is preserved under `docs/specs/archive/phase-6-pipeline-boundary-cleanup/`.
- **Lane B:** treated as complete for this planning artifact based on live evidence in:
  - `docs/specs/handbook-flow-import-boundary-consumer-contract.md` (`Status: Packet 6.B.3`, cleaned frozen surface, caller-owned rendering out of boundary)
  - `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md` (`Packet 6.B.4 completion notes`, including PASS results for the dependency proof, coupling proof, public-surface inspection, tests, workspace check, fmt, and clippy)
- **Lane C:** deferred. `handbook-engine`'s full current public surface in `crates/engine/src/lib.rs` is the working boundary unless a later Substrate consumer proves a narrower facade is needed.

## Substrate-side constraints to lock before import

1. **License fields before import**
   - Add `license = "MIT"` to the `Cargo.toml` files for `handbook-engine`, `handbook-pipeline`, and `handbook-flow` before importing them into Substrate.
   - Rationale: the Phase 6 remaining-work spec records that Substrate's `deny.toml` performs license checks.

2. **Workspace-member integration pattern**
   - Recommended decision: import the three crates as **Substrate workspace members using path dependencies**, not as external git/crates.io dependencies.
   - Rationale: the constraints table records that the existing Substrate pattern is sibling/path-based workspace membership, and the handbook crates already relate to each other via local path dependencies.

3. **YAML divergence**
   - Recorded decision: **keep both crates for the first import wave** (`serde_yaml_bw` in handbook crates, `serde_yaml` in Substrate), and treat any migration to `serde_yaml` as a later follow-up only if Substrate wants dependency convergence.
   - Rationale: the spec's constraints table records no direct package-name conflict and notes that Substrate's multiple-version policy is warning-only.

4. **Compatibility posture**
   - No feature flags are required for import.
   - Edition/resolver/dependency compatibility is already acceptable for first-wave adoption:
     - Edition `2021`
     - Resolver `2`
     - `sha2` `0.10`
     - `libc` `0.2`
     - `serde` `1` with derive

## Recommended import order

### Phase 1 — `handbook-engine` first

**Why first**

- `handbook-engine` has no intra-workspace handbook dependencies.
- Lane C is deferred, so the current engine public surface is the accepted working boundary now.
- Importing engine first gives Substrate the common contract/version/types surface required by the later crates.

**Frozen boundary to adopt**

- Treat the **full current public surface exposed by `crates/engine/src/lib.rs`** as the working boundary for first-wave import.
- That public surface currently includes:
  - public modules:
    - `artifact_manifest`
    - `author`
    - `baseline_validation`
    - `canonical_artifacts`
    - `freshness`
  - crate-root re-exports from those modules (the public artifact-manifest, authoring, baseline-validation, canonical-artifact, and freshness functions/types exposed by `lib.rs`)
  - public canonical layout contract re-exports:
    - `default_canonical_layout_contract()`
    - `CanonicalLayoutContract`
  - public version functions:
    - `workspace_contract_version()`
    - `engine_contract_version()`

**Substrate verification gate after Phase 1**

- The imported crate manifest includes `license = "MIT"`.
- The crate is wired as a Substrate workspace member/path dependency and resolves cleanly.
- `cargo tree -p handbook-engine` still shows no handbook-* intra-workspace dependency chain.
- `cargo test -p handbook-engine` passes in the integrated workspace.
- `cargo check --workspace` passes after the import.
- `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` remain green.
- Substrate's normal license/deny checks remain green with the new crate present.

### Phase 2 — `handbook-pipeline` second

**Why second**

- `handbook-pipeline` depends on `handbook-engine`, so engine must land first.
- Lane A is already closed, and the durable boundary decision is the documented frozen subset of the current public surface.
- Existing evidence says a narrower facade would be premature before a real Substrate consumer proves it is needed.

**Frozen boundary to adopt**

- In-boundary modules from the Lane A closeout:
  - `pipeline`
  - `pipeline_capture`
  - `pipeline_compile`
  - `pipeline_handoff`
  - `pipeline_route`
  - `route_state`
- Public version function:
  - `pipeline_contract_version()`

**Boundary exclusions to preserve**

- Do **not** treat these as part of the Substrate import contract:
  - `setup`
  - `declarative_roots`
  - private `layout`
  - private `repo_file_access`
  - private `stage_10_feature_spec_provenance`

**Substrate verification gate after Phase 2**

- The imported crate manifest includes `license = "MIT"`.
- `handbook-pipeline` is wired against the already-imported `handbook-engine` workspace member/path dependency.
- Substrate callers consume only the documented frozen subset above; no direct dependency is introduced on `setup` or `declarative_roots`.
- `cargo test -p handbook-pipeline --test pipeline_catalog` passes.
- `cargo test -p handbook-pipeline --test pipeline_compile` passes.
- `cargo test -p handbook-pipeline --test pipeline_capture` passes.
- `cargo test -p handbook-pipeline --test pipeline_handoff` passes.
- `cargo check --workspace` passes after the import.
- `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` remain green.
- If Substrate integration reveals a real need for a narrower facade, stop and open a consumer-driven follow-up instead of inventing one during the first import.

### Phase 3 — `handbook-flow` third

**Why third**

- `handbook-flow` depends on `handbook-engine`, so engine must land first.
- Lane B was the remaining import-readiness blocker; this plan assumes that blocker is closed because the consumer contract is now frozen and the verification wall has been recorded as passing.
- Importing flow after engine and pipeline keeps the final required import target last, after its cleaned consumer contract is already available as a stable authority.

**Frozen boundary to adopt**

- Use the frozen consumer boundary from `docs/specs/handbook-flow-import-boundary-consumer-contract.md`.
- Public modules/symbol groups remain centered on:
  - `resolver`
  - `budget`
  - `packet_result`
  - `flow_contract_version()`
- Approved typed semantics may remain in boundary:
  - `ReadyPacketNextSafeAction`
  - `ResolverNextSafeAction`
  - `ResolverRefusalCategory`
  - `ResolverBlockerCategory`
  - `PacketSelectionStatus`
  - budget/result classifiers and other machine-readable flow-owned enums/structs recorded in the consumer contract
- Explicitly out of boundary:
  - final shell-owned/operator-facing command strings
  - caller presentation wording
  - CLI/compiler-specific rendering choices

**Substrate verification gate after Phase 3**

- The imported crate manifest includes `license = "MIT"`.
- `handbook-flow` is wired against the already-imported `handbook-engine` workspace member/path dependency.
- Re-run the Lane B dependency proof on the imported copy: `cargo tree -p handbook-flow` still shows only `handbook-engine` as the handbook intra-workspace dependency.
- Re-run the Lane B coupling proof on the imported copy: no new `handbook_cli`, `handbook_compiler`, or `handbook_pipeline` coupling is introduced on the flow surface.
- Re-run the Lane B public-surface proof on the imported copy: Substrate consumes typed flow results only, and final shell/operator wording remains outside `handbook-flow`.
- `cargo test -p handbook-flow` passes in the integrated workspace.
- `cargo check --workspace` passes after the import.
- `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings` remain green.
- Substrate-side callers own rendering of `ReadyPacketNextSafeAction` and `ResolverNextSafeAction` into operator-facing copy.

## Frozen boundary summary by crate

| Crate | Frozen boundary for first-wave import | Posture |
|---|---|---|
| `handbook-engine` | Full public surface of `crates/engine/src/lib.rs`: public modules `artifact_manifest`, `author`, `baseline_validation`, `canonical_artifacts`, `freshness`; their crate-root re-exports; `default_canonical_layout_contract()`, `CanonicalLayoutContract`, `workspace_contract_version()`, `engine_contract_version()` | Working boundary now; Lane C deferred |
| `handbook-pipeline` | Documented frozen subset: `pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, `pipeline_contract_version()` | Closed in Lane A; import-ready at the documented subset |
| `handbook-flow` | Lane B consumer contract: cleaned `resolver` + `budget` + `packet_result` surface with typed semantics only where contract-approved and final shell copy moved out of boundary | Import-ready after Lane B contract + verification wall |

## Architectural ownership decision by crate

- **Overall Phase 6 ownership outcome:** keep **architectural ownership in handbook for all three crates** and have Substrate adopt them via the recommended workspace-member/path integration shape. The integration shape is a packaging choice for import, not an ownership transfer.
- **`handbook-engine`: handbook remains the architectural owner.**
  - Rationale: the Phase 6 decision rule says to move a crate only if its center of gravity becomes substrate-specific. This plan keeps engine as a reusable handbook-owned contract surface that Substrate consumes directly rather than absorbing the CLI shell.
- **`handbook-pipeline`: handbook remains the architectural owner.**
  - Rationale: Lane A already froze the reusable import boundary, and this plan recommends importing that crate as-is rather than moving its architecture into Substrate. Nothing in the current evidence shows a substrate-specific center of gravity that would justify ownership transfer.
- **`handbook-flow`: handbook remains the architectural owner for this plan.**
  - Rationale: Lane B made the flow surface importable by freezing the typed consumer contract and moving final shell wording out of boundary, but the root-plan rule still weighs against transfer until substrate-specific pressure is proven. For Packet 6.D.2, Substrate is the consumer/importer, not the architectural owner.

## Adapter/facade assessment

**Assessment: no additional adapter or facade is needed before the first Substrate import wave.**

### Evidence basis

- **Engine:** the full current public surface exposed by `crates/engine/src/lib.rs` is the accepted working boundary, and Lane C is explicitly deferred rather than required.
- **Pipeline:** the archived Lane A closeout records a deliberate decision for a **documented frozen subset of the current public surface**, not a new facade. Its rationale is that the technical blocker is gone, the module structure already maps to the reviewed import contract, and a facade would be premature without a real Substrate consumer.
- **Flow:** Lane B already performed the only required boundary-shaping work by moving final shell copy out of the flow import surface and freezing the consumer contract around typed semantics plus caller-owned rendering.

### Practical conclusion

- Import the three crates directly as workspace members/path dependencies.
- Preserve the documented/frozen boundaries above.
- Do **not** add a speculative adapter/facade up front.
- If actual Substrate consumption later proves a narrower API is necessary, introduce that as a separate consumer-driven follow-up backed by live integration evidence.

## Adoption guidance

- Treat this as a **phased import plan**, not a one-shot multi-crate drop.
- Do not import `handbook-cli`.
- Do not use this plan to pre-decide `handbook-compiler`'s long-term Substrate posture.
- Keep the Phase 6 separation intact:
  - Lane A = pipeline boundary freeze already closed
  - Lane B = flow consumer contract already frozen
  - Lane C = optional/deferred
  - Lane D = this import/adoption planning artifact only

## Provenance

- Flow frozen boundary authority: `docs/specs/handbook-flow-import-boundary-consumer-contract.md`
- Pipeline frozen boundary provenance: `docs/specs/archive/phase-6-pipeline-boundary-cleanup/`
- Substrate compatibility and import-constraint source: the **Substrate-Side Constraints** table in `docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
- Active triplet authority:
  - `docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`
