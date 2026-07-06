# Plan: Handbook Engine Extraction Phase 4 Slice 2 (Slice 4.2) - Engine Crate Migration

## Objective

Turn `handbook-engine` from a Slice 4.1 scaffold into the real implementation owner for the approved engine-safe code: canonical artifacts, manifest generation, freshness, baseline validation, and deterministic authoring-core surfaces, while keeping `handbook-cli` behavior stable through a temporary `handbook-compiler` compatibility facade.

Spec reference: [handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md](./handbook-engine-extraction-phase-4-slice-2-engine-migration-spec.md)

## Major Modules

1. Engine crate package and public surface
   - `crates/engine/Cargo.toml`
   - `crates/engine/src/lib.rs`
   - owns the package dependency posture and the public engine-facing export boundary

2. Canonical artifact stack migration
   - `crates/compiler/src/artifact_manifest.rs`
   - `crates/compiler/src/canonical_artifacts.rs`
   - `crates/compiler/src/freshness.rs`
   - `crates/compiler/src/baseline_validation.rs`
   - destination equivalents under `crates/engine/src/**`
   - becomes the first real implementation family owned by `handbook-engine`

3. Approved authoring-core migration
   - `crates/compiler/src/author/charter_core.rs`
   - `crates/compiler/src/author/project_context_core.rs`
   - `crates/compiler/src/author/environment_inventory_core.rs`
   - relevant public facades in `crates/compiler/src/author/*.rs`
   - destination equivalents under `crates/engine/src/author/**`
   - moves deterministic authoring logic without moving shell/runtime/template/write behavior

4. Compiler compatibility facade
   - `crates/compiler/Cargo.toml`
   - `crates/compiler/src/lib.rs`
   - `crates/compiler/src/author/*.rs`
   - remains as the temporary compatibility layer used by the CLI and any existing compiler-facing tests

5. Regression coverage
   - `crates/engine/tests/**`
   - `crates/compiler/tests/artifact_manifest_interface.rs`
   - `crates/compiler/tests/canonical_artifacts_ingest.rs`
   - `crates/compiler/tests/freshness_computation.rs`
   - `crates/compiler/tests/author.rs`
   - `crates/cli/tests/cli_surface.rs`
   - proves the new crate owns what moved and the public product surface stays stable

## Dependencies And Order

### Prerequisite: freeze the ownership and dependency inversion posture

Why first:

- Slice 4.2 cannot succeed if `handbook-engine` still conceptually wraps `handbook-compiler`
- the repo needs one explicit rule for how compatibility works after ownership flips: engine owns the code, compiler re-exports or adapts it temporarily
- author-core migration depends on the canonical artifact and baseline stack already having a stable engine home

Output:

- one agreed dependency rule: no cycle, no duplicate long-term implementations, and no broad direct CLI rewire
- one agreed compatibility rule: `handbook-compiler` may remain as a thin facade, but migrated logic must live in `handbook-engine`

### Packet 4.2.1: Canonical Artifact Manifest Freshness And Baseline Migration

Why first:

- the canonical artifact stack is the most self-contained engine-safe module family currently exported from `handbook-compiler`
- baseline validation depends directly on canonical artifact identities and freshness truth, so moving them together avoids temporary split-brain ownership
- author-shell and CLI paths already depend on this stack indirectly, so landing it first gives Packet 4.2.2 a stable engine-owned foundation

Output:

- `handbook-engine` owns the canonical artifact, manifest, freshness, and baseline-validation implementation modules
- the temporary `handbook-engine -> handbook-compiler` dependency is removed or replaced with an acyclic compatibility posture
- `handbook-compiler` keeps only thin re-exports or adapters for the migrated engine-safe symbols
- engine-owned regression tests exist for the migrated canonical stack

### Packet 4.2.2: Approved Authoring Core Migration

Why second:

- the Phase 3 work already separated deterministic authoring cores from shell/runtime responsibilities
- once canonical artifacts and baseline validation live in `handbook-engine`, the deterministic authoring-core modules can move without dragging repo-state inspection or write flows with them
- keeping author-core migration separate from the canonical stack keeps packet scope narrow and makes shell leakage easier to detect

Output:

- `handbook-engine` owns the deterministic authoring-core modules and their engine-safe public types/functions
- `handbook-compiler` author facades remain thin wrappers over engine-owned core plus compiler-owned shell helpers
- shell/runtime/template/timestamp/write behavior stays outside the engine crate
- engine-owned regression tests exist for the moved deterministic authoring-core behavior

## Risks And Mitigations

### Risk: dependency inversion creates a compiler/engine cycle

Mitigation:

- change package dependencies before broad code movement so the ownership direction is explicit
- inspect both `cargo tree -p handbook-engine -e normal` and `cargo tree -p handbook-compiler -e normal`
- treat any need for mutual dependencies as a design failure to correct before continuing

### Risk: migrated logic ends up duplicated in both crates

Mitigation:

- move implementation once, then leave only explicit compatibility re-exports or thin adapters behind
- scan the repo for duplicate module-family ownership after each packet
- treat copied implementation bodies in both crates as packet leakage unless the docs are updated first

### Risk: author-core migration drags shell/runtime concerns into `handbook-engine`

Mitigation:

- use the Phase 3 authority docs as the exact boundary for what is deterministic vs shell-owned
- keep template-library resolution, Codex runtime transport, timestamp/env resolution, canonical preflight, and write/lock orchestration outside the engine crate
- keep compiler author tests running during the transition to prove the shell wrappers stay intact

### Risk: CLI behavior regresses because compiler compatibility surfaces drift

Mitigation:

- keep `handbook-cli` on its current dependency posture during Slice 4.2
- use `cargo test -p handbook-cli --test cli_surface` as the public behavior guard after each packet
- prefer compatibility re-exports over broad CLI edits

### Risk: project-context deterministic rendering gets coupled back to ambient time/env state

Mitigation:

- move only the explicit-input deterministic core into `handbook-engine`
- leave timestamp resolution and ambient env handling in compiler shell code
- make the engine-facing render API explicit about required inputs if a new public core surface is needed

### Risk: Slice 4.2 widens into setup/doctor/pipeline/flow work

Mitigation:

- keep Packet 4.2.1 limited to the canonical artifact family
- keep Packet 4.2.2 limited to approved deterministic author-core modules
- treat any `setup`, `doctor`, `template_library`, `pipeline*`, `resolver`, `packet_result`, or `budget` move as adjacent-slice leakage unless a tiny compile-through fix is unavoidable

## Parallel Vs Sequential

Sequential:

- freeze dependency inversion and compatibility posture before moving implementation ownership
- land the canonical artifact stack before moving deterministic authoring-core modules
- verify engine ownership and compiler compatibility before checking CLI behavior

Parallel opportunities after Packet 4.2.1 lands:

- engine package tests for the migrated canonical stack can be refined in parallel with compiler-facade cleanup
- within Packet 4.2.2, the three deterministic author-core module moves can be prepared in parallel once the engine author namespace and compatibility pattern are fixed
- engine test migration and compiler shell-facade simplification can run in parallel once the public core surface is settled

## Verification Checkpoints

### Checkpoint 1: dependency posture matches engine ownership intent

```bash
cargo tree -p handbook-engine -e normal
cargo tree -p handbook-compiler -e normal
cargo check --workspace
```

### Checkpoint 2: canonical artifact stack is engine-owned and tested

```bash
rg -n 'artifact_manifest|canonical_artifacts|freshness|baseline_validation' crates/engine crates/compiler/src
cargo test -p handbook-engine
```

### Checkpoint 3: deterministic author-core surfaces are engine-owned while compiler shell stays intact

```bash
rg -n 'charter_core|project_context_core|environment_inventory_core' crates/engine crates/compiler/src/author
rg -n 'use handbook_engine|pub use handbook_engine|handbook_engine::' crates/compiler/src/author crates/compiler/src/lib.rs
cargo test -p handbook-compiler --test author
```

### Checkpoint 4: public CLI behavior stays stable

```bash
cargo test -p handbook-cli --test cli_surface
```

### Final checkpoint

```bash
cargo check --workspace
cargo test -p handbook-engine
cargo test -p handbook-cli --test cli_surface
```

## Exit Conditions

The slice is ready for human review when:

- `handbook-engine` is the real implementation owner for the approved engine-safe module families in Slice 4.2
- the package dependency graph is acyclic and no longer treats `handbook-engine` as a wrapper over `handbook-compiler`
- `handbook-compiler` remains only a temporary compatibility facade for the migrated engine-safe surfaces
- deterministic authoring-core logic moved without dragging shell/runtime/template/write responsibilities into the engine crate
- engine-owned regression coverage exists and passes
- the CLI surface still passes without a broad direct-caller rewire
- no Phase 4.3, 4.4, or 4.5 work leaked into the landing
