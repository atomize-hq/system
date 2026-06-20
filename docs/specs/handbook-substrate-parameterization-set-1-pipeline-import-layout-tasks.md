# Tasks: Handbook Substrate Parameterization — Set 1: Pipeline Import Layout

Plan reference: [handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-plan.md)

Spec reference: [handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md](./handbook-substrate-parameterization-set-1-pipeline-import-layout-spec.md)

---

## Packet 1.1: Declarative Root Contract And Owner Boundary

- [x] Task: Introduce the supported public/import-facing declarative root contract
  - Acceptance: `handbook-pipeline` has a typed declarative root owner that can represent non-default pipeline/profile/runner/stage roots, and handbook-product defaults remain available through an explicit default helper instead of being the only model.
  - Verify: Source inspection of `crates/pipeline/src/declarative_roots.rs` and `crates/pipeline/src/lib.rs`; `cargo test -p handbook-pipeline --test pipeline_catalog`.
  - Files: `crates/pipeline/src/declarative_roots.rs`, `crates/pipeline/src/lib.rs`, `crates/pipeline/tests/pipeline_catalog.rs`

- [x] Task: Adopt the declarative contract in root-owner helpers without changing default behavior
  - Acceptance: root derivation for pipeline/profile/runner/stage access no longer depends on raw repo-level literals at the ownership boundary; the default helper still preserves handbook-product behavior for existing callers/tests.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-pipeline --test pipeline_loader`
  - Files: `crates/pipeline/src/declarative_roots.rs`, `crates/pipeline/src/pipeline.rs`, `crates/pipeline/tests/pipeline_loader.rs`

## Packet 1.2: Stage-Root Discovery And Validation Adoption

> Reconciliation note (2026-06-20 live repo truth): Packet 1.2 was briefly reopened after the prior false-complete state. The follow-up commits `2719d82` and `aeffb29` replaced the old rejection proof with positive contract-driven coverage and moved the import-facing discovery / validation seams onto active declarative roots. Packet 1.4 can now proceed as a proof-only packet.

- [x] Task: Move supported stage-source ownership onto the active declarative contract
  - Acceptance: supported stage-source path derivation and pipeline/stage catalog discovery no longer depend on handbook-product default roots as active truth; stage-root behavior is derived from the active declarative contract.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-pipeline --test pipeline_compile && cargo test -p handbook-pipeline --test pipeline_route_resolution` plus source inspection of the Packet 1.2 discovery call sites in `crates/pipeline/src/pipeline.rs`.
  - Files: `crates/pipeline/src/pipeline.rs`, `crates/pipeline/tests/pipeline_catalog.rs`, `crates/pipeline/tests/pipeline_compile.rs`, `crates/pipeline/tests/pipeline_route_resolution.rs`

- [x] Task: Adopt contract-driven stage discovery and inseparable path validation
  - Acceptance: stage discovery and stage/pipeline path validation derive in-scope root checks from the active contract where structural correctness requires it, and loader validation no longer codifies rejection of non-default stage roots. Broader wording-only cleanup remains deferred to Set 3.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_loader && cargo test -p handbook-pipeline --test pipeline_compile` plus source inspection of `crates/pipeline/tests/pipeline_loader.rs` to confirm the old rejection proof is removed or replaced with positive contract-driven acceptance coverage.
  - Files: `crates/pipeline/src/pipeline.rs`, `crates/pipeline/tests/pipeline_loader.rs`, `crates/pipeline/tests/pipeline_compile.rs`

## Packet 1.3: Public Pipeline Storage Layout Injection

- [x] Task: Promote the storage layout contract to the supported public/import-facing boundary
  - Acceptance: downstream importers can access a supported `PipelineStorageLayoutContract` and an explicit handbook-product default helper through the crate boundary; containment validation for runtime-state-owned paths remains enforced.
  - Verify: Source inspection of `crates/pipeline/src/layout.rs`, `crates/pipeline/src/lib.rs`, and `crates/pipeline/tests/pipeline_storage_layout_contract.rs`; `cargo test -p handbook-pipeline --test pipeline_storage_layout_contract && cargo test -p handbook-pipeline --test pipeline_state_store`
  - Files: `crates/pipeline/src/layout.rs`, `crates/pipeline/src/lib.rs`, `crates/pipeline/tests/pipeline_storage_layout_contract.rs`, `crates/pipeline/tests/pipeline_state_store.rs`

- [x] Task: Adopt the public storage contract in route-state entry points
  - Acceptance: route-state persistence behavior can honor non-default storage roots through the supported contract boundary without crate-private access, while default handbook-product behavior stays unchanged.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_state_store`
  - Files: `crates/pipeline/src/layout.rs`, `crates/pipeline/src/route_state.rs`, `crates/pipeline/tests/pipeline_state_store.rs`

- [x] Task: Adopt the public storage contract in capture and handoff entry points
  - Acceptance: capture provenance/cache behavior and handoff bundle behavior can honor non-default storage roots through the supported contract boundary, with handbook-product defaults preserved through the explicit default helper.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_capture && cargo test -p handbook-pipeline --test pipeline_handoff`
  - Files: `crates/pipeline/src/layout.rs`, `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/pipeline_compile.rs`, `crates/pipeline/src/pipeline_handoff.rs`, `crates/pipeline/tests/pipeline_capture.rs`, `crates/pipeline/tests/pipeline_handoff.rs`

## Packet 1.4: Final Set Proof

- [x] Task: Run the Set 1 verification wall
  - Acceptance: all of the following pass:
    - `cargo test -p handbook-pipeline --test pipeline_catalog`
    - `cargo test -p handbook-pipeline --test pipeline_loader`
    - `cargo test -p handbook-pipeline --test pipeline_compile`
    - `cargo test -p handbook-pipeline --test pipeline_route_resolution`
    - `cargo test -p handbook-pipeline --test pipeline_state_store`
    - `cargo test -p handbook-pipeline --test pipeline_capture`
    - `cargo test -p handbook-pipeline --test pipeline_handoff`
    - `cargo check --workspace`
    - `cargo fmt --all -- --check`
    - `cargo clippy --workspace --all-targets -- -D warnings`
  - Verify: Run each command and record pass/fail in the completion notes below.
  - Files: `docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md`

- [x] Task: Record the bounded residual-default inventory honestly
  - Acceptance: the completion notes explicitly distinguish any remaining acceptable handbook-product defaults from structural import blockers. This note must not pretend that every `core/**` or handbook-product literal disappeared if some remain as explicit default-helper or product-default behavior.
  - Verify: `rg -n "core/pipelines|core/profiles|core/runners|core/stages|\.handbook/state|artifacts/handoff/feature_slice" crates/pipeline/src crates/pipeline/tests` plus source inspection cross-referenced against the active public contract surface.
  - Files: `docs/specs/handbook-substrate-parameterization-set-1-pipeline-import-layout-tasks.md`

#### Packet 1.4 completion notes

- Verification wall:
  - PASS — `cargo test -p handbook-pipeline --test pipeline_catalog`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_loader`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_compile`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_route_resolution`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_state_store`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_capture`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_handoff`
  - PASS — `cargo check --workspace`
  - PASS — `cargo fmt --all -- --check`
  - PASS — `cargo clippy --workspace --all-targets -- -D warnings`
- Residual bounded-default inventory:
  - Acceptable retained handbook-product defaults:
    - `crates/pipeline/src/declarative_roots.rs` still defines the explicit default helper roots `core/pipelines`, `core/profiles`, `core/runners`, and `core/stages`.
    - `crates/pipeline/src/layout.rs` still defines the explicit default storage helper roots `.handbook/state`, `.handbook/state/pipeline`, `.handbook/state/pipeline/stage_capture`, `.handbook/state/pipeline/capture`, and `artifacts/handoff/feature_slice`.
    - repo-owned tests / fixtures still mention those handbook-product defaults where they are proving default-helper behavior, product-default behavior, or negative-path validation wording.
  - No structural import blockers remain in the supported import-facing seams covered by Set 1: Packet 1.2 now has active-root discovery / validation entry points and positive explicit-roots proof coverage, while Packet 1.3 still exposes the public storage-layout contract for non-default state / capture / handoff roots.
- Proof-fix note:
  - The first proof-wall attempt exposed two minimal verification-surface fixes that were required before the wall could pass cleanly:
    - remove needless borrows in `crates/pipeline/tests/pipeline_loader.rs`
    - update `crates/compiler/tests/pipeline_loader.rs` to match `StageFileValidationError::OutsideStageDirectory { stage_root }`
- Scope note: Packet 1.4 should record proof and bounded residuals only. If verification reveals missing structural work from Packets 1.1–1.3, stop and reopen the relevant earlier packet explicitly instead of silently widening Packet 1.4.

---

## Stop Boundary

Stop after Packet 1.4 for this set. Do not:

- start Set 2 (`handbook-flow` canonical-layout injection)
- start Set 3 (import-surface default / validation honesty cleanup)
- widen into CLI/compiler product-shell cleanup
- execute the actual Substrate import
- generalize the bounded default consumer into a broader multi-consumer platform
