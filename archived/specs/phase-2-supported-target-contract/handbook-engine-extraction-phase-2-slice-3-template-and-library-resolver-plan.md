# Plan: Handbook Engine Extraction Phase 2 Slice 3 (Slice 2.3) - Template And Library Resolver Boundary

## Objective

Move shipped charter and environment-inventory library/template selection behind one typed resolver boundary, preserve current shipped-default behavior, and add bounded override rules without widening into new content, runtime target work, or Phase 3 shell cleanup.

Spec reference: [handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md](./handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md)

## Major Modules

1. Typed template/library resolver owner
   - proposed as `crates/compiler/src/template_library.rs`, adjacent to `pipeline.rs` so stage catalog truth can stay authoritative
   - owns typed asset identities, shipped-default selection, and bounded override validation for the approved asset families

2. Charter authoring adopter
   - `crates/compiler/src/author/charter.rs`
   - consumes the resolver for authoring-method, synthesize-directive, and template selection while keeping charter-specific validation and wording local

3. Environment-inventory authoring adopter
   - `crates/compiler/src/author/environment_inventory.rs`
   - consumes the same resolver for directive/template selection while keeping environment-inventory-specific validation and wording local

4. Declarative observer layer
   - `crates/compiler/src/template_library.rs`
- `crates/compiler/src/pipeline.rs`
   - `core/stages/05_charter_synthesize.md`
   - `core/stages/07_foundation_pack.md`
   - remains the authoritative declarative source for stage library inputs and protects the resolver from becoming a second catalog

5. Compatibility and regression layer
   - `crates/compiler/src/canonical_artifacts.rs`
   - `crates/compiler/src/setup.rs`
   - `crates/compiler/tests/author.rs`
   - `crates/compiler/tests/pipeline_catalog.rs`
   - `crates/compiler/tests/setup.rs`
   - `crates/cli/tests/author_cli.rs`

## Dependencies And Order

### Prerequisite: Slice 2.2 stays frozen

Why first:

- Slice 2.3 is template/library-only work
- runtime target adoption, supported-target ownership, and stage/consumer wedge decisions are already approved elsewhere

Output:

- one stable runtime-target authority set
- one stable declaration that Slice 2.3 owns template/library parameterization only

### Packet 2.3.1: Typed resolver contract and shipped-default posture

Why first:

- the slice needs one owner before authoring modules can stop hardcoding their own shipped asset selection
- zero-config shipped-default behavior must be preserved before any override rules are introduced

Output:

- one typed set of approved asset identities for charter and environment-inventory authoring
- one shipped-default selection path that preserves current live behavior
- charter and environment-inventory authoring migrated off local shipped asset ownership
- declarative stage library truth remains an observer and regression guard, not a rewritten owner

### Packet 2.3.2: Validated override and selection rules

Why second:

- override behavior is only safe once the shipped-default contract is already explicit and regression-covered
- validation must be layered onto the new boundary, not spread back across individual authoring modules

Output:

- optional typed override requests for approved asset families
- refusal posture for invalid paths, invalid asset-family pairings, missing files, or out-of-bounds selections
- preserved zero-config behavior after override support lands

## Risks And Mitigations

### Risk: Slice 2.3 widens into new library content or prompt redesign

Mitigation:

- keep the slice focused on ownership and validated selection only
- reject packet work that rewrites template text, heading order, or author-facing wording beyond tiny compatibility fixes

### Risk: authoring modules still keep duplicate shipped-asset owners after the resolver lands

Mitigation:

- require Packet 2.3.1 to remove or demote module-local selection ownership
- use author and CLI regression tests to prove the resolver is the only approved owner of shipped asset selection

### Risk: override support becomes an arbitrary file-path escape hatch

Mitigation:

- accept overrides only through typed requests for approved asset families
- refuse absolute paths, traversal, out-of-root selections, and asset-kind mismatches

### Risk: stage catalog truth and the new resolver drift apart

Mitigation:

- keep stage front matter authoritative for compile-time library inputs
- use `pipeline_catalog` regression coverage as an explicit guardrail during both packets

### Risk: setup starter-template behavior regresses while shipped-default posture is tightened

Mitigation:

- treat setup and canonical-artifact semantics as compatibility observers unless a tiny wiring change is strictly required
- run `setup` regression coverage if any compatibility plumbing touches starter-template posture

## Parallel Vs Sequential

Sequential:

- typed resolver contract before any authoring adoption
- shipped-default adoption before override support
- override validation before final workspace wall

Parallel opportunities after Packet 2.3.1 starts:

- charter adoption and environment-inventory adoption can proceed in parallel once the typed resolver contract is landed
- CLI regression updates can land with the adopter packet that changes the observed behavior

## Verification Checkpoints

### Checkpoint 1: Shipped-default resolver contract and charter adoption

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-cli --test author_cli
```

### Checkpoint 2: Environment-inventory adoption and declarative observer guard

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-compiler --test pipeline_catalog
```

### Checkpoint 3: Validated override and compatibility guard

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-compiler --test setup
```

### Hardcoded-owner spot-check

```bash
rg -n "include_str!\(|setup_starter_template|core/library/.+tmpl|core/library/.+directive" crates/compiler/src/author crates/compiler/src/canonical_artifacts.rs crates/compiler/src/setup.rs
```

### Final checkpoint

```bash
cargo check --workspace
cargo test --workspace
```

## Exit Conditions

The slice is ready for human review when:

- one typed template/library resolver boundary exists for the approved authoring asset families
- `author/charter.rs` and `author/environment_inventory.rs` consume that boundary for shipped-default selection
- zero-config behavior remains stable under author, CLI, and setup regression coverage
- validated override rules exist and refuse unsafe or out-of-scope selections
- stage library declarations remain authoritative and regression-covered
- no new library content, Slice 2.2 target work, or Phase 3 shell cleanup leaked in
