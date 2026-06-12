# Tasks: Handbook Engine Extraction Phase 1 Slice 5 (Slice 1.5) - Layout Parameterization Closeout

Plan reference: [handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md](./handbook-engine-extraction-phase-1-slice-5-layout-parameterization-closeout-plan.md)

## Prerequisite: parameterization must stay behavior-preserving and narrowly bounded

Slice 1.5 exists to finish the root-plan parameterization claim honestly, not to redesign the handbook layout or broaden into target/config/platform work.

- The current product-owned default layout stays `.handbook/**`, `.handbook/state/**`, and `artifacts/handoff/feature_slice/**` unless the user explicitly changes scope.
- Making those paths changeable through the newly landed typed parameters/default owners is a hard requirement, even though this slice must keep the effective defaults unchanged.
- Product-shell wording, docs, and tests may still mention those paths when they are asserting or describing the contract.

## Packet 1.5.1: Parameterized Canonical Layout Contract

- [ ] Task: Freeze the typed reusable-layout contract and residual-literal policy
  - Acceptance: the implementation names one explicit current-product default layout owner for canonical, runtime-state, and handoff roots; makes path changes possible through typed parameters/default owners in principle; and distinguishes forbidden reusable-owner literals from acceptable product-shell/doc/test literals.
  - Verify: `rg -n --glob '!**/tests/**' --glob '!**/*test*' "\\.handbook|artifacts/handoff|state/pipeline|feature_slice" crates/engine/src crates/pipeline/src crates/flow/src crates/compiler/src && rg -n "SYSTEM_ROOT_RELATIVE|RUNTIME_STATE_ROOT_RELATIVE|HANDOFF_FEATURE_SLICE_DIR_RELATIVE|HANDBOOK_ROOT_PATH" crates/engine/src crates/pipeline/src crates/flow/src crates/compiler/src`
  - Files: `crates/engine/src/canonical_paths.rs`, `crates/pipeline/src/layout.rs`, optional narrow shared support module(s), optional narrow compile-through alignment in `crates/compiler/src/**`

- [ ] Task: Parameterize engine canonical path ownership without changing canonical artifact behavior
  - Acceptance: `handbook-engine` canonical path derivation consumes the approved typed layout/default owner instead of depending on fixed engine literals alone; that owner can represent different canonical path values in principle; and canonical artifact identities, manifest behavior, and freshness semantics remain unchanged for the current default layout.
  - Verify: `cargo test -p handbook-engine --test canonical_artifacts_ingest && cargo test -p handbook-engine --test freshness_computation`
  - Files: `crates/engine/src/canonical_paths.rs`, `crates/engine/src/canonical_artifacts.rs`, optionally `crates/engine/src/lib.rs`, `crates/engine/tests/canonical_artifacts_ingest.rs`, `crates/engine/tests/freshness_computation.rs`

## Packet 1.5.2: Parameterized Pipeline Storage Layout Adoption

- [ ] Task: Parameterize runtime-state, capture, provenance, and handoff layout ownership in `handbook-pipeline`
  - Acceptance: pipeline storage owners derive `.handbook/state/**` and `artifacts/handoff/feature_slice/**` through typed layout/default inputs rather than fixed pipeline-root literals; those inputs can represent different storage roots in principle; and the current product default still produces the same relative paths and behaviors.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_state_store && cargo test -p handbook-pipeline --test pipeline_capture && cargo test -p handbook-pipeline --test pipeline_handoff`
  - Files: `crates/pipeline/src/layout.rs`, `crates/pipeline/src/route_state.rs`, `crates/pipeline/src/pipeline_capture.rs`, `crates/pipeline/src/stage_10_feature_spec_provenance.rs`, `crates/pipeline/src/pipeline_handoff.rs`, optionally `crates/pipeline/tests/pipeline_state_store.rs`, `crates/pipeline/tests/pipeline_capture.rs`, `crates/pipeline/tests/pipeline_handoff.rs`

- [ ] Task: Keep the storage closeout scoped away from orchestration-target work
  - Acceptance: the packet does not change supported pipeline ids, stage ids, consumer ids, or target-registry behavior beyond any minimal compile-through plumbing required for layout parameterization.
  - Verify: `rg -n "pipeline\\.foundation_inputs|stage\\.10_feature_spec|feature-slice-decomposer" crates/pipeline/src crates/compiler/src && cargo check --workspace`
  - Files: verification against packet-local files only

## Packet 1.5.3: Remaining Reusable Caller Adoption And Residual-Literal Closeout

- [ ] Task: Adopt remaining reusable callers onto the approved parameterized layout owners/defaults
  - Acceptance: reusable callers such as `crates/flow/src/resolver.rs` stop owning separate fallback root literals where approved reusable owners already exist, consume the newly landed parameterized owners/defaults instead, and preserve current refusal/budget behavior for the current product default.
  - Verify: `cargo test -p handbook-flow --test resolver_core && cargo check --workspace`
  - Files: `crates/flow/src/resolver.rs`, optional narrow adapter files in `crates/compiler/src/**`, only if required for compile-through alignment

- [ ] Task: Bound and justify any remaining fixed literals after reusable adoption lands
  - Acceptance: any remaining path literals are explicitly product-owned, doc-owned, fixture-owned, or test-owned; reusable path derivation no longer depends on them as the active owner contract.
  - Verify: `rg -n --glob '!**/tests/**' --glob '!**/*test*' "\\.handbook|artifacts/handoff|state/pipeline|feature_slice" crates/engine/src crates/pipeline/src crates/flow/src crates/compiler/src crates/cli/src`
  - Files: verification against source files plus any packet-local cleanup files only

## Final Slice Verification

- [ ] Task: Run the full Slice 1.5 verification wall after all packets land
  - Acceptance: reusable engine/pipeline/layout owners are parameterized, the landed owners/defaults make path changes possible in principle, current product-default paths remain unchanged in this slice, reusable callers no longer own duplicate fallback roots, residual literals are truthfully bounded, and the workspace is format-clean, clippy-clean, and test-green.
  - Verify: `cargo fmt --all -- --check && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace`
  - Files: verification only

## Human Review Gate

Stop after the Slice 1.5 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
