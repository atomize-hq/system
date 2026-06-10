# Plan: Handbook Engine Extraction Phase 3 Slice 1 (Slice 3.1) - Charter Deterministic Core Split

## Objective

Split reusable deterministic charter behavior away from guided synthesis and product-shell orchestration inside the current compiler crate, while preserving the existing public `handbook author charter` contract.

Spec reference: [handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md](./handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md)

## Major Modules

1. Deterministic charter core
   - proposed as `crates/compiler/src/author/charter_core.rs`
   - owns structured-input types, normalization, validation, compiler-owned markdown rendering, markdown validation, and pure render helpers

2. Charter shell / orchestration layer
   - `crates/compiler/src/author/charter.rs`
   - stays as the outward facade and thin orchestration layer for preflight, canonical write flow, and public function wiring

3. Guided synthesis runtime layer
   - proposed as `crates/compiler/src/author/charter_shell.rs`
   - owns prompt assembly, template-library selection, `codex exec` transport, env-var overrides, temp-output handling, synthesized-markdown validation, and CLI-oriented refusal wording

4. Public export and caller layer
   - `crates/compiler/src/author/mod.rs`
   - `crates/compiler/src/lib.rs`
   - `crates/cli/src/main.rs`
   - should preserve current exported functions and type names even if ownership moves behind new internal modules

5. Regression layer
   - `crates/compiler/tests/author.rs`
   - `crates/cli/tests/author_cli.rs`
   - proves deterministic and guided paths both survive the boundary cleanup

## Dependencies And Order

### Prerequisite: Freeze Phase 2 charter asset ownership

Why first:

- Slice 3.1 is internal boundary cleanup only
- Slice 2.3 already established `template_library.rs` as the charter asset owner
- the deterministic core split must not reopen template selection or shipped-default rules

Output:

- one frozen asset-selection boundary from Slice 2.3
- one clean statement that Slice 3.1 is about deterministic-core separation only

### Packet 3.1.1: Charter parse / render / validate core extraction

Why first:

- the reusable deterministic core must exist before shell cleanup can target it
- later Phase 4 engine migration needs a stable engine-shaped charter API to move
- guided synthesis and preflight logic should depend on the deterministic core, not the other way around

Output:

- one deterministic charter owner for types, normalization, validation, rendering, and markdown validation
- a thinner `author/charter.rs` facade that delegates deterministic behavior instead of owning it inline
- preserved public exports and regression coverage for parse/render/validate entrypoints

### Packet 3.1.2: Charter synthesis and shell adapter cleanup

Why second:

- shell cleanup is only safe once the deterministic contract is already explicit
- guided runtime transport, refusal wording, and canonical write orchestration should wrap a stable core, not migrate simultaneously with it

Output:

- one shell/runtime owner for prompt assembly, Codex transport, env overrides, temp-output handling, synthesized-output validation, and CLI-oriented refusal wording
- thin `author_charter`, `author_charter_guided`, and preflight adapters over core + shell helpers
- preserved CLI behavior after the internal split

## Risks And Mitigations

### Risk: the split changes the public compiler API or CLI behavior

Mitigation:

- keep `author/mod.rs` and `lib.rs` exports stable
- use compiler and CLI regression tests as required rails for both packets
- treat public behavior changes as out of scope unless the spec changes first

### Risk: deterministic code still retains shell/runtime dependencies

Mitigation:

- define the core boundary first and keep it free of `Command`, `Stdio`, env-var reads, temp-file helpers, and repo mutation
- use ownership scans after each packet to confirm runtime constructs stayed outside the core

### Risk: Packet 3.1.2 widens into project-context or environment-inventory cleanup

Mitigation:

- keep the work scoped to charter-only files and tests
- defer any shared helper extraction unless it is charter-local or explicitly approved later

### Risk: the split weakens Slice 2.3 template-library ownership

Mitigation:

- keep `template_library.rs` as a shell-side dependency for guided synthesis only
- refuse any implementation that recreates charter asset ownership inside the core

### Risk: preflight / lock / write cleanup becomes a broad authoring refactor

Mitigation:

- keep orchestration thin but charter-local in Slice 3.1
- do not generalize into multi-surface authoring helpers unless live code proves it is unavoidable and still within scope

## Parallel Vs Sequential

Sequential:

- deterministic core extraction before any shell/runtime cleanup
- public export stabilization before CLI-surface verification
- shell/runtime cleanup before final workspace wall

Parallel opportunities after Packet 3.1.1 starts:

- compiler regression updates for deterministic functions and CLI regression updates for guided flow can proceed in parallel once the core boundary is explicit
- shell/runtime helper extraction and refusal-surface assertions can proceed together as long as they do not reopen deterministic-core ownership

## Verification Checkpoints

### Checkpoint 1: Deterministic charter core exists and public parse / render / validate still work

```bash
cargo test -p handbook-compiler --test author
```

### Checkpoint 2: Guided synthesis and shell orchestration still preserve CLI behavior

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-cli --test author_cli
```

### Ownership spot-check: runtime behavior stays outside the deterministic core

```bash
rg -n "Command::new|Stdio|AUTHOR_CHARTER_CODEX_|std::env::var|temp_dir|next_safe_action|handbook author charter" crates/compiler/src/author/charter*.rs crates/compiler/src/author/mod.rs
```

### Ownership spot-check: deterministic charter API remains explicit

```bash
rg -n "parse_charter_structured_input_yaml|normalize_charter_free_text|validate_charter_structured_input|render_charter_markdown|validate_charter_markdown" crates/compiler/src/author/charter*.rs crates/compiler/src/author/mod.rs
```

### Final checkpoint

```bash
cargo check --workspace
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- deterministic charter logic is isolated behind one engine-shaped boundary
- guided synthesis and product-shell behavior are isolated behind shell/runtime helpers
- `author/charter.rs` is a thin facade instead of the monolithic owner of every charter concern
- public compiler and CLI charter behavior remain stable under regression coverage
- no template-ownership regression or adjacent-slice cleanup leaked in
