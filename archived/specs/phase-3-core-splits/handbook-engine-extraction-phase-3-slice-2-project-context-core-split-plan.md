# Plan: Handbook Engine Extraction Phase 3 Slice 2 (Slice 3.2) - Project-Context Deterministic Core Split

## Objective

Split reusable deterministic project-context behavior away from runtime authoring and product-shell orchestration inside the current compiler crate, while preserving the existing public `handbook author project-context` contract.

Spec reference: [handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md](./handbook-engine-extraction-phase-3-slice-2-project-context-core-split-spec.md)

## Major Modules

1. Deterministic project-context core
   - proposed as `crates/compiler/src/author/project_context_core.rs`
   - owns structured-input types, normalization, factuality validation, render-safety validation, deterministic markdown rendering, markdown validation, and render helpers

2. Project-context shell / orchestration layer
   - `crates/compiler/src/author/project_context.rs`
   - stays as the outward facade and thin orchestration layer for authoring preflight, public function wiring, and compiler-facing refusal mapping

3. Runtime authoring helper layer
   - proposed as `crates/compiler/src/author/project_context_shell.rs`
   - owns render timestamp resolution, canonical-root inspection, baseline eligibility checks, lock acquisition, canonical-write validation, repo mutation, and CLI-oriented refusal wording

4. Public export and caller layer
   - `crates/compiler/src/author/mod.rs`
   - `crates/compiler/src/lib.rs`
   - `crates/cli/src/main.rs`
   - should preserve current exported functions and type names even if ownership moves behind new internal modules

5. Regression layer
   - `crates/compiler/tests/author.rs`
   - `crates/cli/tests/author_cli.rs`
   - proves deterministic and authoring flows both survive the boundary cleanup

## Dependencies And Order

### Prerequisite: Freeze the Slice 3.1 split pattern and current public project-context contract

Why first:

- Slice 3.2 is internal boundary cleanup only
- Slice 3.1 already established the in-place authoring split pattern for Phase 3
- the public project-context CLI and compiler contract must stay stable while internals are reorganized

Output:

- one frozen statement that Slice 3.2 is about deterministic-core separation only
- one stable public contract for `handbook author project-context`, `--from-inputs`, guided TTY interviewing, refusal posture, and canonical writes

### Packet 3.2.1: Project-context deterministic model split

Why first:

- the reusable deterministic core must exist before shell cleanup can target it
- later Phase 4 engine migration needs a stable engine-shaped project-context API to move
- authoring and refusal flows should depend on the deterministic core, not the other way around

Output:

- one deterministic project-context owner for structured-input types, normalization, validation, rendering, and markdown validation
- a thinner `author/project_context.rs` facade that delegates deterministic behavior instead of owning it inline
- preserved public exports and regression coverage for parse/render/validate entrypoints

### Packet 3.2.2: Project-context recovery wording and shell cleanup

Why second:

- shell cleanup is only safe once the deterministic contract is already explicit
- timestamp resolution, authoring preflight, lock/write flow, and refusal wording should wrap a stable core, not migrate simultaneously with it

Output:

- one shell/runtime owner for timestamp resolution, preflight, canonical-write mutation, lock handling, and CLI-oriented refusal wording
- thin `author_project_context`, `author_project_context_from_input`, and preflight adapters over core + shell helpers
- preserved CLI behavior after the internal split

## Risks And Mitigations

### Risk: the split changes the public compiler API or CLI behavior

Mitigation:

- keep `author/mod.rs` and `lib.rs` exports stable
- use compiler and CLI regression tests as required rails for both packets
- treat public behavior changes as out of scope unless the spec changes first

### Risk: deterministic code still retains env/clock/filesystem dependencies

Mitigation:

- define the core boundary first and keep it free of env reads, current-time resolution, lock helpers, and canonical writes
- use ownership scans after each packet to confirm runtime constructs stayed outside the core

### Risk: guided interview behavior leaks into compiler internals during the split

Mitigation:

- keep guided answer collection explicitly in `crates/cli/src/main.rs`
- treat compiler-owned work as typed-input processing and authoring orchestration only

### Risk: Packet 3.2.2 widens into environment-inventory, setup, or doctor cleanup

Mitigation:

- keep the work scoped to project-context files and tests
- defer any shared helper extraction unless it is project-context-local or explicitly approved later

### Risk: timestamp handling changes rendered markdown behavior unexpectedly

Mitigation:

- keep the existing timestamp format stable
- prove rendered markdown stability through existing compiler and CLI regression rails
- isolate timestamp resolution as shell behavior without changing outward markdown semantics

## Parallel Vs Sequential

Sequential:

- deterministic core extraction before any shell/runtime cleanup
- public export stabilization before CLI-surface verification
- shell/runtime cleanup before final workspace wall

Parallel opportunities after Packet 3.2.1 starts:

- compiler regression updates for deterministic functions and CLI regression updates for authoring flows can proceed in parallel once the core boundary is explicit
- shell helper extraction and refusal-surface assertions can proceed together as long as they do not reopen deterministic-core ownership

## Verification Checkpoints

### Checkpoint 1: Deterministic project-context core exists and public parse / render / validate still work

```bash
cargo test -p handbook-compiler --test author
```

### Checkpoint 2: Authoring preflight, canonical writes, and CLI behavior still preserve the public contract

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-cli --test author_cli
```

### Ownership spot-check: runtime behavior stays outside the deterministic core

```bash
rg -n "AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR|resolve_project_context_now_utc|next_safe_action|handbook author project-context|acquire_authoring_lock|write_repo_relative_bytes|validate_canonical_write_target|CanonicalArtifacts::load" crates/compiler/src/author/project_context*.rs crates/compiler/src/author/mod.rs
```

### Ownership spot-check: deterministic project-context API remains explicit

```bash
rg -n "parse_project_context_structured_input_yaml|validate_project_context_structured_input|render_project_context_markdown|validate_project_context_markdown|normalized_project_context_structured_input|collect_render_safety_issues|validate_known_fake_project_context_markers" crates/compiler/src/author/project_context*.rs crates/compiler/src/author/mod.rs
```

### Final checkpoint

```bash
cargo check --workspace
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- deterministic project-context logic is isolated behind one engine-shaped boundary
- timestamp resolution, authoring mutation flow, and product-shell behavior are isolated behind shell/runtime helpers
- `author/project_context.rs` is a thin facade instead of the monolithic owner of every project-context concern
- public compiler and CLI project-context behavior remain stable under regression coverage
- no environment-inventory, setup, doctor, or adjacent-slice cleanup leaked in
