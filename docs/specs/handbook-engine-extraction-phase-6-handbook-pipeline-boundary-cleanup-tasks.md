# Tasks: Handbook Engine Extraction Phase 6 - `handbook-pipeline` Boundary Cleanup

Plan reference: [handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md](./handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md)

## Prerequisites Already Landed

- Phase 6 ownership/integration planning Packet 2 already states that `handbook-pipeline` remains handbook-owned and that Substrate should import only through a thinner reviewed supported-target boundary rather than the full current public re-export surface.
- Phase 6 ownership/integration planning Packet 4 already names this seam as: the `handbook-pipeline` boundary cleanup seam, owned by the reviewed importer-boundary question plus the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge.
- Separate human review has now approved this seam for bounded implementation. Execution must stay inside the implementation packet below and must not widen into publication, crates.io work, Substrate consumption, or adjacent Phase 6 seams.
- The completed Packet 1 / Packet 2 / Packet 3 sections below remain as historical planning provenance. Current execution authority begins at **Implementation Packet 1**.

## Packet 1: Freeze The Seam Boundary

- [x] Task: Re-state the landed Packet 2 and Packet 4 boundary language as the authority for this seam
  - Acceptance: The spec, plan, and tasks docs all state that `handbook-pipeline` remains handbook-owned, that only a thinner reviewed supported-target importer boundary is the durable posture, and that this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question.
  - Verify: `rg -n "handbook-owned|reviewed supported-target importer boundary|template_library|compiler-backed fixture/support coupling" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: The triplet now re-states the landed Packet 2 / Packet 4 authority in quote-ready form: `handbook-pipeline` remains handbook-owned; only the reviewed supported-target importer boundary is durable; and this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge.

- [x] Task: Freeze what is inside and outside the reviewed importer boundary
  - Acceptance: The triplet explicitly records that loading/selection surfaces, compile surfaces, capture surfaces, handoff surfaces, and route-state surfaces needed for supported pipeline execution are the reviewed importer boundary, while `setup`, CLI/product-shell helpers, and compiler-routed compatibility helpers remain outside this seam.
  - Verify: `rg -n "loading/selection|compile surfaces|capture surfaces|handoff surfaces|route-state surfaces|setup stays outside|CLI/product-shell helpers|compiler-routed compatibility helpers" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: The triplet now freezes the reviewed supported-target importer boundary as loading/selection surfaces, compile surfaces, capture surfaces, handoff surfaces, and route-state surfaces needed for supported pipeline execution. It also states that `setup` stays outside this reviewed supported-target importer boundary, that CLI/product-shell helpers stay outside this boundary, and that compiler-routed compatibility helpers stay outside this boundary.

## Packet 2: Freeze The Coupling Evidence Ledger And Cleanup Target

- [x] Task: Record the live compiler-backed fixture/support coupling evidence
  - Acceptance: The triplet explicitly labels a **live evidence ledger** and a **retained compiler context**. It records that `crates/pipeline/tests/pipeline_catalog.rs` imports `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`, and that `cargo tree -p handbook-pipeline` still shows `handbook-compiler` only as a dev-dependency rather than a runtime owner.
  - Verify: `rg -n "resolve_shipped_template_library|TemplateLibraryRequest|TemplateLibrarySelection" crates/pipeline/tests/pipeline_catalog.rs && cargo tree -p handbook-pipeline`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: Packet 2 now freezes the live evidence ledger around the exact `pipeline_catalog` import and the dev-dependency-only `cargo tree -p handbook-pipeline` result, then separates the retained compiler context into the `crates/compiler/src/lib.rs`, `crates/compiler/src/author/mod.rs`, and `crates/compiler/src/template_library.rs` surfaces that currently expose that support.

- [x] Task: Freeze the bounded cleanup target without widening into retained compiler narrowing
  - Acceptance: The triplet explicitly labels a **bounded cleanup target** and **explicit non-goals**. It states that later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof, but does not plan wholesale retained `handbook-compiler` retirement, broader authoring-stack relocation, or CLI shell/support reassignment.
  - Verify: `rg -n "remove or relocate|pipeline catalog/runtime proof|retained handbook-compiler retirement|authoring-stack relocation|CLI shell/support" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: Packet 2 now freezes the cleanup target as “Later implementation must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof,” and pairs that sentence with an explicit non-goals list that keeps retained `handbook-compiler` retirement, broader authoring-stack relocation, and CLI shell/support reassignment out of scope.

## Packet 3: Prepare Future Implementation Verification And Stop At Review

- [x] Task: Record the future implementation verification wall for this seam
  - Acceptance: The triplet records `pipeline_catalog`, `pipeline_compile`, `pipeline_capture`, `pipeline_handoff`, compiler `author`, and `cargo check --workspace` as the future implementation verification wall for this seam.
  - Verify: `rg -n "pipeline_catalog|pipeline_compile|pipeline_capture|pipeline_handoff|handbook-compiler --test author|cargo check --workspace" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: Packet 3 now records the full future implementation verification wall in quote-ready form across the triplet: `pipeline_catalog`, `pipeline_compile`, `pipeline_capture`, `pipeline_handoff`, compiler `author`, and `cargo check --workspace` all remain required proof for a later execution packet.
  - Scope note: `pipeline_loader`, `pipeline_route_resolution`, and `pipeline_state_store` remain adjacent evidence for loading/selection and route-state coverage rather than new mandatory wall entries because live repo truth does not show them importing compiler template-library support; `pipeline_catalog` remains the known compiler-backed coupling seam this wall must prove.

- [x] Task: Preserve the human review gate and planning-only stop condition
  - Acceptance: The triplet states plainly that it is planning-only; implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.
  - Verify: `rg -n "planning-only|packet-prompt authoring|production edits|publication|crates\.io|Substrate consumption|integration implementation|Human Review Gate" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md`
  - Files: `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-spec.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-plan.md`, `docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-tasks.md`
  - Completion note: Packet 3 now ends at an explicit **Human Review Gate** in all three docs. The stop condition is quote-ready and says that implementation, packet-prompt authoring for later execution, production edits, publication, crates.io work, Substrate consumption, and integration implementation remain blocked until a human separately reviews this triplet and explicitly approves a later execution packet.

## Implementation Packet 1: Pipeline Catalog Fixture/Support Decoupling

- [ ] Task: Move the `pipeline_catalog` proof off compiler-owned template-library support
  - Acceptance: `crates/pipeline/tests/pipeline_catalog.rs` no longer imports `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`. The `stage_library_inputs_remain_the_authoritative_declarative_source` assertion uses a pipeline-owned test/support fixture or literal source rooted in the declared stage-library contract rather than compiler-owned authoring helpers.
  - Verify: `rg -n "handbook_compiler::author::template_library|resolve_shipped_template_library|TemplateLibraryRequest|TemplateLibrarySelection" crates/pipeline/tests crates/pipeline/src && cargo test -p handbook-pipeline --test pipeline_catalog`
  - Files: `crates/pipeline/tests/pipeline_catalog.rs`, optionally `crates/pipeline/tests/support/**`, optionally `crates/pipeline/src/**`

- [ ] Task: Remove the pipeline package's compiler-backed proof dependency if decoupling leaves no remaining in-scope need
  - Acceptance: `crates/pipeline/Cargo.toml` no longer lists `handbook-compiler` under `[dev-dependencies]`, unless another pipeline-owned test still requires it and that requirement is explicitly documented before the packet can close. `cargo tree -p handbook-pipeline` therefore no longer shows `handbook-compiler` as a dev-dependency in the normal packet path.
  - Verify: `cargo tree -p handbook-pipeline && cargo test -p handbook-pipeline --test pipeline_catalog`
  - Files: `crates/pipeline/Cargo.toml`, any pipeline test/support files touched by the proof rewrite

- [ ] Task: Prove the reviewed supported-target wedge stays stable after the proof decoupling
  - Acceptance: `pipeline_catalog`, `pipeline_compile`, `pipeline_capture`, `pipeline_handoff`, compiler `author`, and `cargo check --workspace` all pass without widening into CLI/setup ownership, flow proof, retained `handbook-compiler` retirement, publication, crates.io work, or Substrate integration implementation.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-pipeline --test pipeline_compile && cargo test -p handbook-pipeline --test pipeline_capture && cargo test -p handbook-pipeline --test pipeline_handoff && cargo test -p handbook-compiler --test author && cargo check --workspace`
  - Files: verification only, plus only the packet-local files above if a narrow repair is required

## Final Slice Verification

- [ ] Task: Run the full boundary-cleanup verification wall after Implementation Packet 1 lands
  - Acceptance: the only known pipeline-owned compiler-backed coupling is gone, the pipeline package no longer carries the now-unneeded compiler dev-dependency unless explicitly justified, the reviewed supported-target wedge remains stable, and no adjacent-seam leakage appears.
  - Verify: `cargo tree -p handbook-pipeline && cargo test -p handbook-pipeline --test pipeline_catalog && cargo test -p handbook-pipeline --test pipeline_compile && cargo test -p handbook-pipeline --test pipeline_capture && cargo test -p handbook-pipeline --test pipeline_handoff && cargo test -p handbook-compiler --test author && cargo check --workspace`
  - Files: verification only

## Wider-Seam Guardrail

Stop after the bounded implementation packet above lands review-clean. Do not widen this slice into retained `handbook-compiler` retirement, CLI shell/support reassignment, publication, crates.io work, Substrate consumption, or broader integration implementation without separate authority.
