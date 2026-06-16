# Spec: Handbook Engine Extraction Phase 6 - `handbook-pipeline` Boundary Cleanup

## Assumptions

1. Packet 4 names five downstream seams, but it does **not** impose a mandatory execution order among the non-optional seams.
2. When ordering is not forced, the next seam should be the one with the most concrete live blocker and the least dependency on unresolved adjacent seams.
3. The optional `handbook-engine` adapter / boundary-freeze seam stays closed unless later human review explicitly says the current public engine surface is still insufficient.
4. Retained `handbook-compiler` narrowing is not the first seam because Packet 4 says that work waits until remaining support adapters have explicit homes and downstream callers no longer need compiler-routed transition helpers.
5. Absent a newer phase taxonomy in repo truth, this triplet should stay under the existing Phase 6 naming family instead of inventing a new Phase 7 label.
6. The only known pipeline-owned compiler-backed coupling in live repo truth is `crates/pipeline/tests/pipeline_catalog.rs`; `pipeline_loader`, `pipeline_route_resolution`, and `pipeline_state_store` remain adjacent evidence unless a new compiler-backed import appears there.
7. The approved execution posture should prefer a pipeline-owned test/support fixture or literal source inside `crates/pipeline/tests/**` before introducing any compiler-neutral shared helper or new public pipeline API.

## Objective

Land one downstream seam only: the `handbook-pipeline` boundary cleanup seam named by Packet 4 of the landed Phase 6 ownership/integration planning family.

This slice now uses the landed planning packets plus the approved execution packet defined in the tasks doc and packet-prompts artifact for this seam.

This execution authority must:

- preserve the reviewed supported-target importer boundary that Packet 2 said is the only durable `handbook-pipeline` import posture
- remove the specific compiler-backed fixture/support coupling that still prevents the full current `handbook-pipeline` crate surface from being blessed as the durable importer contract
- keep the cleanup bounded to pipeline catalog/runtime proof rather than widening into CLI shell/support clarification, `handbook-flow` importer-proof work, retained `handbook-compiler` narrowing, or optional `handbook-engine` boundary-freeze work
- prefer a pipeline-owned test/support fixture or literal source for the shipped template defaults needed by `pipeline_catalog`, and only introduce a compiler-neutral helper if live code proves the pipeline-owned proof source cannot stay narrow

Implementation is approved only for the bounded packet defined by this slice. Publication, crates.io work, Substrate consumption, and broader integration implementation remain out of scope.

## Tech Stack

- Rust 2021 workspace
- `cargo` for build/test/check orchestration
- owner crate under change: `crates/pipeline`
- retained compatibility evidence: `crates/compiler`
- primary proof surfaces: Rust package tests under `crates/pipeline/tests/**`

## Why This Seam Is The Next Planning Target

Packet 2 already made `handbook-pipeline` the only imported-core crate whose durable importer posture stayed intentionally narrower than its current public re-export surface. Packet 4 then turned that residual gap into an explicit seam with concrete live evidence: define the reviewed supported-target importer boundary and remove or relocate the remaining compiler-backed fixture/support coupling currently evidenced by `crates/pipeline/tests/pipeline_catalog.rs` importing `handbook_compiler::author::template_library`.

Compared with the other Packet 4 seams:

- the `handbook-engine` seam is optional rather than required
- the `handbook-flow` seam is a proof-or-reject exercise, not a concrete already-evidenced coupling cleanup
- retained `handbook-compiler` narrowing is explicitly downstream of assigning support adapters to real homes
- the CLI shell/support seam is product-shell clarification work, not the importer-boundary cleanup that Packet 2 already identified as the concrete residual `handbook-pipeline` blocker

## Authority Inputs

Primary authority for why this seam exists:

- `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-spec.md`
- `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-plan.md`
- `docs/specs/handbook-engine-extraction-phase-6-ownership-and-integration-planning-tasks.md`

Supporting repo-truth authorities for seam sizing and earlier boundary language:

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- `docs/specs/handbook-engine-extraction-phase-2-slice-1-supported-target-contract-{spec,plan,tasks}.md`

Live code and test surfaces this seam must plan around:

- `crates/pipeline/src/lib.rs`
- `crates/pipeline/src/pipeline.rs`
- `crates/pipeline/tests/pipeline_catalog.rs`
- `crates/compiler/src/lib.rs`
- `crates/compiler/src/author/mod.rs`
- `crates/compiler/src/template_library.rs`

## Commands

Current live-evidence verification:

```bash
rg -n "pub use|pub mod|mod " crates/pipeline/src/lib.rs
rg -n "template_library|resolve_shipped_template_library|TemplateLibraryRequest|TemplateLibrarySelection|load_pipeline_catalog|load_pipeline_catalog_metadata|render_pipeline_list|supported_target" crates/pipeline/src crates/pipeline/tests/pipeline_catalog.rs
rg -n "template_library|pub use" crates/compiler/src/lib.rs crates/compiler/src/author/mod.rs crates/compiler/src/template_library.rs
cargo tree -p handbook-pipeline
cargo test -p handbook-pipeline --test pipeline_catalog
```

Future seam verification wall for implementation packets derived from this triplet:

```bash
cargo test -p handbook-pipeline --test pipeline_catalog
cargo test -p handbook-pipeline --test pipeline_compile
cargo test -p handbook-pipeline --test pipeline_capture
cargo test -p handbook-pipeline --test pipeline_handoff
cargo test -p handbook-compiler --test author
cargo check --workspace
```

Triplet-consistency verification:

```bash
rg -n "reviewed supported-target importer boundary|compiler-backed fixture/support coupling|template_library|setup stays out of the reviewed importer boundary|Approved Implementation Boundary|Wider-Seam Guardrail" docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-{spec,plan,tasks}.md
```

## Project Structure

```text
docs/specs/                                                         -> seam authority docs plus packet prompt artifact
crates/pipeline/src/lib.rs                                          -> current public `handbook-pipeline` re-export surface
crates/pipeline/src/pipeline.rs                                     -> catalog/loading/selection logic and supported-target topology
crates/pipeline/tests/pipeline_catalog.rs                           -> live evidence of compiler-backed fixture/support coupling
crates/pipeline/tests/support/                                      -> preferred home for any packet-local pipeline-owned fixture/support helper
crates/compiler/src/lib.rs                                          -> current retained compiler re-export surface
crates/compiler/src/author/mod.rs                                   -> path-based re-export of template-library support into authoring flows
crates/compiler/src/template_library.rs                             -> current shipped-template and override-resolution support
docs/specs/handbook-engine-extraction-phase-6-handbook-pipeline-boundary-cleanup-packet-prompts.md -> ready-to-paste orchestration prompt for the approved execution packet
```

## Code Style

Prefer the smallest pipeline-owned proof source that can express the expected stage-library inputs without routing through compiler-owned authoring helpers:

```rust
let charter_stage =
    load_stage_compile_definition(&root, &pipeline, "stage.05_charter_synthesize")
        .expect("charter synthesize stage");
assert_eq!(
    charter_stage.inputs.library,
    expected_charter_library_inputs()
);
```

Conventions for this seam:

- reuse the exact crate names and Packet 2 / Packet 4 vocabulary already landed
- keep replacement proof sources local to pipeline ownership first: `crates/pipeline/tests/**` before `crates/pipeline/src/**`
- avoid new shared abstractions unless the packet cannot close honestly without them
- keep one dominant seam: reviewed `handbook-pipeline` importer-boundary cleanup plus the pipeline-side compiler-backed fixture/support decoupling question

## Testing Strategy

This slice is now implementation-authorized, so verification must prove both the narrow decoupling and the unchanged supported-target wedge.

- **Packet-local proof:** `pipeline_catalog` must stop importing compiler-owned template-library support.
- **Dependency proof:** `cargo tree -p handbook-pipeline` must no longer show `handbook-compiler` as a dev-dependency unless a still-in-scope pipeline-owned test can justify it explicitly.
- **Implementation-proof wall:** `pipeline_catalog`, `pipeline_compile`, `pipeline_capture`, `pipeline_handoff`, compiler `author`, and workspace compile health must all stay green.
- **Boundary-proof expectation:** implementation must preserve the Packet 2 rule that only the reviewed supported-target wedge, not the full crate surface, is the durable importer contract.

## Packet 3 Verification Wall

Future implementation for this seam must clear the full verification wall below before the seam can be considered implementation-complete:

- `cargo test -p handbook-pipeline --test pipeline_catalog`
- `cargo test -p handbook-pipeline --test pipeline_compile`
- `cargo test -p handbook-pipeline --test pipeline_capture`
- `cargo test -p handbook-pipeline --test pipeline_handoff`
- `cargo test -p handbook-compiler --test author`
- `cargo check --workspace`

This verification wall is the required acceptance wall for the approved execution packet. Publication, crates.io work, Substrate consumption, and broader integration implementation remain out of scope.

### Adjacent evidence vs. seam-specific wall

- `pipeline_catalog` stays inside the mandatory Packet 3 wall because live repo truth still shows it importing `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`.
- `pipeline_loader` remains adjacent evidence for loading/selection surfaces, and `pipeline_route_resolution` plus `pipeline_state_store` remain adjacent evidence for route-state surfaces.
- Those adjacent tests do **not** currently import compiler template-library support, so they are evidence for the reviewed boundary shape rather than additional seam-specific proof that the compiler-backed coupling was removed.
- The mandatory Packet 3 wall therefore stays intentionally narrow: prove the known compiler-backed coupling seam (`pipeline_catalog`), preserve compile/capture/handoff proof for the reviewed supported-target wedge, preserve compiler authoring proof, and keep workspace compile health green.

## Approved Implementation Choice

- **Chosen first posture:** use a pipeline-owned test/support fixture or literal source inside `crates/pipeline/tests/**` to express the shipped template defaults that `pipeline_catalog` must compare against.
- **Why this posture first:** the only known coupling is in a pipeline-owned test, so the narrowest honest fix is to keep the proof source local to pipeline-owned test/support code instead of introducing a new shared helper or widening the public pipeline surface.
- **Escalation rule:** only introduce a compiler-neutral helper if the packet cannot preserve declarative stage-library truth with a pipeline-owned proof source.
- **Secondary cleanup expectation:** once the proof source is local to pipeline ownership, remove `handbook-compiler` from `crates/pipeline` dev-dependencies unless another still-in-scope pipeline-owned test proves it is still required.

## Seam Scope

In scope:

- restate the landed Packet 2 `handbook-pipeline` boundary text as the starting point for this seam
- define the reviewed supported-target importer boundary more concretely around the loading/selection, compile, capture, handoff, and route-state surfaces needed for supported pipeline execution
- explicitly record that `setup` remains outside the reviewed importer boundary for this seam because Packet 4 routes `setup` ownership clarification to the CLI shell/support seam
- record the live compiler-backed fixture/support coupling evidenced by `crates/pipeline/tests/pipeline_catalog.rs` importing compiler template-library support
- land the bounded cleanup needed so pipeline catalog/runtime verification stops depending on compiler-owned template-library support
- preserve the Packet 4 routing that this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge

Out of scope:

- blessing the entire current `handbook-pipeline` public re-export surface as the durable importer boundary
- changing `handbook-engine` ownership/import posture or opening the optional engine adapter seam
- proving a narrower `handbook-flow` importer contract
- clarifying CLI shell/support ownership for `rendering`, `refusal`, `error`, `doctor`, `setup`, prompting, operator wording, or exit-code policy
- narrowing or retiring retained `handbook-compiler` as a whole
- crate publication, crates.io work, Substrate consumption approval, or integration implementation

## Packet 2 Freeze: Evidence Ledger And Cleanup Target

### Live evidence ledger

- `crates/pipeline/src/lib.rs` currently re-exports a broad surface that includes catalog/loading/selection helpers, compile/capture/handoff helpers, route-state helpers, and `setup`.
- Packet 2 already decided that Substrate must **not** consume that full current surface as the durable boundary; only a thinner reviewed supported-target wedge is approved.
- `crates/pipeline/tests/pipeline_catalog.rs` still imports `handbook_compiler::author::template_library::{resolve_shipped_template_library, TemplateLibraryRequest, TemplateLibrarySelection}`.
- `cargo tree -p handbook-pipeline` still shows `handbook-compiler` as a dev-dependency rather than a runtime owner, so the remaining compiler edge is bounded support coupling rather than runtime center-of-gravity inversion.
- `cargo test -p handbook-pipeline --test pipeline_catalog` passes today, which means the current coupling is live repo truth rather than dead code.

### Retained compiler context

- `crates/compiler/src/author/mod.rs` still owns the path-based authoring hook via `#[path = "../template_library.rs"] pub mod template_library;`.
- `crates/compiler/src/lib.rs` still re-exports `author::template_library::{resolve_shipped_template_library, resolve_template_library, ...}`, which is why pipeline tests can currently reach shipped-template support through retained compiler-owned surfaces.
- `crates/compiler/src/template_library.rs` remains the live implementation home for `resolve_shipped_template_library`, `TemplateLibraryRequest`, and `TemplateLibrarySelection`.

### Bounded cleanup target

- Implementation Packet 1 must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof.
- That target stays intentionally narrow: the goal is to decouple pipeline catalog/runtime proof from compiler-owned template-library support without changing the Packet 2 rule that only the reviewed supported-target wedge, not the full crate surface, is the durable importer contract.

### Explicit non-goals

- Do not widen this seam into retained `handbook-compiler` retirement.
- Do not widen this seam into broader authoring-stack relocation.
- Do not widen this seam into CLI shell/support reassignment.
- Do not treat this seam as approval to reopen the optional `handbook-engine` boundary-freeze seam or the separate `handbook-flow` importer-proof seam.

## Boundary Freeze This Seam Must Produce

### Packet 1 authority freeze inherited from Phase 6 Packet 2 and Packet 4

- `handbook-pipeline` remains handbook-owned.
- Only the reviewed supported-target importer boundary is the durable importer posture for this seam; the full current public re-export surface is **not** the durable importer contract.
- This seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question for the catalog/runtime wedge.

### Reviewed supported-target importer boundary

This seam must make the reviewed supported-target importer boundary concrete enough that later implementation packets can test against it without rediscovering scope.

The planned durable boundary for `handbook-pipeline` should stay limited to the Packet 2 supported-target wedge:

- loading/selection surfaces
- compile surfaces
- capture surfaces
- handoff surfaces
- route-state surfaces required for supported pipeline execution

This seam must **not** silently expand that reviewed supported-target importer boundary to include every current public re-export. In particular:

- `setup` stays outside this reviewed supported-target importer boundary for now
- CLI/product-shell helpers stay outside this boundary
- compiler-routed compatibility helpers stay outside this boundary unless they are the minimum temporary bridge required to remove the specific `pipeline_catalog` coupling this seam owns

### Bounded cleanup target

This seam must remove or relocate the remaining compiler-backed fixture/support coupling that currently exists because `pipeline_catalog` reaches into compiler template-library support.

Implementation Packet 1 must remove or relocate the pipeline-side compiler-backed fixture/support dependency needed by pipeline catalog/runtime proof.

The cleanup target is intentionally narrower than “move template_library out of compiler everywhere.” The seam succeeds only if later packets can make `handbook-pipeline` catalog/runtime proof independent from compiler-owned template-library support **without** widening into retained `handbook-compiler` retirement, broader authoring-stack relocation, or CLI shell/support reassignment.

The approved first posture is a pipeline-owned test/support fixture or literal source for the specific shipped template defaults that `pipeline_catalog` needs. A compiler-neutral helper is acceptable only if the packet cannot close honestly without it, and this slice still does not authorize or require a full authoring-stack relocation.

## Boundaries

- Always:
  - preserve the landed Packet 2 statement that `handbook-pipeline` remains handbook-owned and only the reviewed supported-target wedge is the durable importer posture
  - preserve the landed Packet 4 routing that this seam owns the pipeline-side `template_library` / compiler-backed fixture-support decoupling question
  - keep `setup` outside the reviewed importer boundary for this seam
  - prefer a pipeline-owned test/support proof source before introducing a compiler-neutral helper
  - remove `handbook-compiler` from `crates/pipeline` dev-dependencies if the packet no longer needs it
  - keep one dominant seam and make adjacent seams explicit non-goals
- Ask first:
  - any attempt to widen the reviewed importer boundary beyond loading/selection, compile, capture, handoff, and route-state surfaces
  - any attempt to move CLI shell/support ownership questions into this seam
  - any attempt to treat retained `handbook-compiler` narrowing as part of this seam rather than a later follow-on
  - any attempt to keep a compiler-backed pipeline dev-dependency after the packet can otherwise close honestly
  - any attempt to move the replacement proof source into a new shared production API rather than pipeline-owned test/support code
- Never:
  - bless the full current `handbook-pipeline` crate surface as the durable importer contract
  - reopen the Phase 6 ownership verdicts for `handbook-engine`, `handbook-flow`, `handbook-cli`, or retained `handbook-compiler`
  - widen the packet into retained `handbook-compiler` retirement, CLI shell/support reassignment, publication, crates.io work, Substrate consumption, or broader integration implementation

## Success Criteria

- The slice preserves the landed Packet 2 and Packet 4 boundary language instead of replacing it with new generic wording.
- The slice preserves a concrete reviewed importer boundary for `handbook-pipeline` that is narrower than the full current public re-export surface.
- The slice explicitly states that `setup` stays out of this reviewed importer boundary and remains routed to the CLI shell/support seam.
- `crates/pipeline/tests/pipeline_catalog.rs` no longer imports compiler-owned template-library support.
- The expected stage-library proof source lives under pipeline ownership first: `crates/pipeline/tests/**` before any compiler-neutral escalation.
- `crates/pipeline/Cargo.toml` no longer carries `handbook-compiler` as a dev-dependency unless another packet-in-scope pipeline-owned test can justify it explicitly.
- The full implementation verification wall passes without widening into retained `handbook-compiler` retirement, broader authoring-stack relocation, or CLI shell/support reassignment.

## Approved Implementation Boundary

This slice is approved only for the bounded implementation packet and prompt artifact defined by the existing spec/plan/tasks plus the packet-prompts doc. Publication, crates.io work, Substrate consumption, and broader integration implementation still require separate authority.

## Open Questions

- Does any current `handbook-pipeline` export outside the reviewed supported-target wedge still need temporary transitional documentation, or can this triplet treat the non-reviewed remainder simply as out of boundary?
- Current live repo truth says `pipeline_loader`, `pipeline_route_resolution`, and `pipeline_state_store` cover adjacent loading/selection and route-state behavior without importing compiler template-library support; if a later repo change introduces another compiler-backed pipeline test beyond `pipeline_catalog`, add it to the bounded evidence ledger before implementation starts.
