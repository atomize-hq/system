# Plan: Handbook Engine Extraction Phase 4 Slice 3 (Slice 4.3) - Pipeline Crate Migration

## Objective

Turn `handbook-pipeline` from a Slice 4.1 scaffold into the real implementation owner for reusable runtime pipeline behavior: declarative loading, supported-target registry, route resolution, route-state persistence, trusted-session validation, compile/capture/handoff mechanics, stage-10 provenance, and the approved setup-helper seam, while keeping `handbook-cli` behavior stable through a temporary `handbook-compiler` compatibility facade.

Spec reference: [handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md](./handbook-engine-extraction-phase-4-slice-3-pipeline-migration-spec.md)

## Major Modules

1. Pipeline crate package and public surface
   - `crates/pipeline/Cargo.toml`
   - `crates/pipeline/src/lib.rs`
   - owns the package dependency posture and public pipeline-facing export boundary

2. Pipeline foundation migration
   - `crates/compiler/src/pipeline.rs`
   - `crates/compiler/src/pipeline_route.rs`
   - `crates/compiler/src/route_state.rs`
   - destination equivalents under `crates/pipeline/src/**`
   - becomes the first real implementation family owned by `handbook-pipeline`

3. Supporting infra required for real ownership
   - `crates/compiler/src/declarative_roots.rs`
   - `crates/compiler/src/repo_file_access.rs`
   - pipeline-focused portions of `crates/compiler/src/layout.rs`
   - destination equivalents or a narrow split under `crates/pipeline/src/**`
   - exists only to the extent necessary to keep the migrated pipeline modules independent from `handbook-compiler`

4. Runtime execution migration
   - `crates/compiler/src/pipeline_compile.rs`
   - `crates/compiler/src/pipeline_capture.rs`
   - `crates/compiler/src/pipeline_handoff.rs`
   - destination equivalents under `crates/pipeline/src/**`
   - moves compile/capture/handoff behavior without moving CLI shell ownership

5. Provenance and setup-helper alignment
   - `crates/compiler/src/stage_10_feature_spec_provenance.rs`
   - reusable pipeline-safe portions of `crates/compiler/src/setup.rs`
   - destination equivalents under `crates/pipeline/src/**`
   - keeps provenance and reset/setup helper seams aligned with the new runtime owner while leaving full setup product behavior outside the crate

6. Compiler compatibility facade and regression coverage
   - `crates/compiler/Cargo.toml`
   - `crates/compiler/src/lib.rs`
   - `crates/compiler/tests/{pipeline_loader,pipeline_catalog,pipeline_route_resolution,pipeline_state_store,pipeline_compile,pipeline_capture,pipeline_handoff,setup}.rs`
   - `crates/pipeline/tests/**`
   - `crates/cli/tests/{cli_surface,help_drift_guard}.rs`
   - proves the new crate owns what moved and the public product surface stays stable

## Dependencies And Order

### Prerequisite: freeze the dependency and supporting-infra posture

Why first:

- Slice 4.3 cannot succeed if `handbook-pipeline` still conceptually wraps `handbook-compiler`
- the repo needs one explicit rule for shared pipeline-safe helpers before module moves start: move or split only what is required for real pipeline ownership
- compile/capture/handoff migration depends on route-state and declarative loading already having a stable pipeline-owned home

Output:

- one agreed dependency rule: no cycle, no duplicate long-term implementations, and no broad direct CLI rewire
- one agreed supporting-infra rule: only the pipeline-safe subset of declarative/file/layout helpers moves or splits in this slice
- one agreed compatibility rule: `handbook-compiler` may remain as a thin facade, but migrated logic must live in `handbook-pipeline`

### Packet 4.3.1: Pipeline Loading Route And Route-State Migration

Why first:

- declarative loading, route resolution, and route-state are the foundation required by compile, capture, handoff, and setup reset behavior
- trusted-session validation and route-basis persistence already form a coherent pipeline-safe runtime family
- landing this foundation first prevents later packets from depending on compiler-owned pipeline state after the crate split is supposed to be real

Output:

- `handbook-pipeline` owns pipeline loading, supported-target registry, route evaluation, and route-state/trusted-session logic
- the temporary `handbook-pipeline -> handbook-compiler` dependency is removed and replaced with an acyclic compatibility posture
- any required declarative/file/layout helper ownership needed by those modules lives behind `handbook-pipeline`
- pipeline-owned regression tests exist for loader, catalog, route evaluation, and route-state behavior

### Packet 4.3.2: Compile Capture And Handoff Migration

Why second:

- compile/capture/handoff all depend directly on the pipeline foundation and route-state/trusted-session behavior from Packet 4.3.1
- moving these execution flows together avoids split-brain runtime ownership where compiler still owns execution while pipeline owns only metadata/state
- keeping provenance and setup-helper alignment for the final packet keeps Packet 4.3.2 focused on the three runtime execution flows themselves

Output:

- `handbook-pipeline` owns compile, capture, and handoff runtime mechanics
- `handbook-compiler` keeps only thin re-exports or adapters for the migrated pipeline-safe symbols
- pipeline-owned regression tests exist for runtime execution behavior while compiler and CLI compatibility guards still pass

### Packet 4.3.3: Setup Helper And Provenance Alignment

Why third:

- stage-10 provenance depends on landed compile/capture/handoff ownership and should align only after those seams are stable
- the approved setup-helper seam depends on route-state and capture/handoff behavior already being pipeline-owned
- leaving this packet last keeps the slice from widening prematurely into full setup redesign

Output:

- `handbook-pipeline` owns stage-10 provenance generation/validation and the approved reusable setup-helper seam
- `handbook-compiler` setup remains a thin product-facing facade that still owns any non-pipeline starter-template or shell behavior left outside the slice
- final regression evidence proves provenance, setup reset, and CLI behavior stay stable after the ownership shift

## Risks And Mitigations

### Risk: supporting infra causes a slice-wide scope explosion

Mitigation:

- move or split only the pipeline-safe portions of `declarative_roots`, `repo_file_access`, and `layout` required by the migrated runtime modules
- keep non-pipeline consumers working through thin compiler compatibility layers if that is cheaper than broad refactoring now
- treat a brand-new generic shared-infra abstraction as out of scope unless the current narrow move proves impossible

### Risk: dependency inversion creates a compiler/pipeline or engine/pipeline/compiler cycle

Mitigation:

- change package dependencies before broad code movement so ownership direction is explicit
- inspect both `cargo tree -p handbook-pipeline -e normal` and `cargo tree -p handbook-compiler -e normal`
- allow `handbook-pipeline` to depend on `handbook-engine` if needed, but never on `handbook-compiler`

### Risk: migrated logic ends up duplicated in both crates

Mitigation:

- move implementation once, then leave only explicit compatibility re-exports or thin adapters behind
- scan the repo for duplicate ownership after each packet
- treat copied implementation bodies in both crates as packet leakage unless the docs are updated first

### Risk: setup-helper alignment drags starter-template or canonical-artifact ownership into `handbook-pipeline`

Mitigation:

- keep the approved setup-helper seam narrow and runtime-focused
- leave starter-template bytes and canonical artifact ownership where Slice 4.2 put them unless a tiny compatibility adjustment is unavoidable
- keep `setup.rs` behavior tests running during the transition to prove product behavior stays intact

### Risk: compile/capture/handoff behavior regresses while crate ownership improves

Mitigation:

- keep compiler integration tests and CLI regression tests running during each packet
- move tests with the ownership shift, but preserve compiler and CLI guards until direct caller rewires are explicitly approved
- prefer compatibility re-exports over broad CLI edits

### Risk: Slice 4.3 widens into flow migration or caller rewires

Mitigation:

- keep Packet 4.3.1 limited to pipeline foundation and route-state ownership
- keep Packet 4.3.2 limited to compile/capture/handoff mechanics
- keep Packet 4.3.3 limited to provenance and the approved setup-helper seam
- treat any `resolver`, `packet_result`, `budget`, or direct CLI dependency rewires as adjacent-slice leakage

## Parallel Vs Sequential

Sequential:

- freeze dependency/supporting-infra posture before moving implementation ownership
- land pipeline foundation before moving compile/capture/handoff
- land compile/capture/handoff before aligning provenance and setup helpers
- verify pipeline ownership and compiler compatibility before checking CLI behavior

Parallel opportunities after Packet 4.3.1 lands:

- package-local pipeline tests can be refined in parallel with compiler-facade cleanup
- compile and capture migration prep can happen in parallel once the new pipeline foundation namespace and supporting helper ownership are fixed
- Packet 4.3.3 provenance updates and setup-helper delegation can be prepared in parallel once Packet 4.3.2 lands its core runtime APIs

## Verification Checkpoints

### Checkpoint 1: dependency posture matches pipeline ownership intent

```bash
cargo tree -p handbook-pipeline -e normal
cargo tree -p handbook-compiler -e normal
cargo check --workspace
```

### Checkpoint 2: pipeline foundation is pipeline-owned and tested

```bash
rg -n 'load_pipeline_catalog|resolve_pipeline_route|load_route_state|persist_route_basis|load_trusted_pipeline_session|plan_runtime_state_reset' crates/pipeline crates/compiler/src
cargo test -p handbook-pipeline
cargo test -p handbook-compiler --test pipeline_loader
cargo test -p handbook-compiler --test pipeline_catalog
cargo test -p handbook-compiler --test pipeline_route_resolution
cargo test -p handbook-compiler --test pipeline_state_store
```

### Checkpoint 3: compile/capture/handoff surfaces are pipeline-owned while compiler compatibility stays intact

```bash
rg -n 'compile_pipeline_stage|capture_pipeline_output|emit_pipeline_handoff_bundle|preview_pipeline_capture|validate_pipeline_handoff_bundle' crates/pipeline crates/compiler/src
cargo test -p handbook-pipeline
cargo test -p handbook-compiler --test pipeline_compile
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
```

### Checkpoint 4: provenance and setup-helper alignment stay stable

```bash
rg -n 'stage_10_feature_spec|plan_runtime_state_reset|apply_runtime_state_reset|reset_state' crates/pipeline crates/compiler/src/setup.rs crates/compiler/src/stage_10_feature_spec_provenance.rs crates/compiler/src/route_state.rs
cargo test -p handbook-pipeline
cargo test -p handbook-compiler --test setup
cargo test -p handbook-cli --test cli_surface
```

### Final checkpoint

```bash
cargo check --workspace
cargo test -p handbook-pipeline
cargo test -p handbook-cli --test cli_surface
```

Optional final public-help guard:

```bash
cargo test -p handbook-cli --test help_drift_guard
```

## Exit Conditions

The slice is ready for human review when:

- `handbook-pipeline` is the real implementation owner for the approved Slice 4.3 module families
- the package dependency graph is acyclic and no longer treats `handbook-pipeline` as a wrapper over `handbook-compiler`
- `handbook-compiler` remains only a temporary compatibility facade for the migrated pipeline-safe surfaces
- provenance and the approved setup-helper seam move without dragging broad setup or CLI shell behavior into the pipeline crate
- pipeline-owned regression coverage exists and passes
- the CLI surface still passes without a broad direct-caller rewire
- no Phase 4.4 or 4.5 work leaked into the landing
