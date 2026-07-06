# Plan: Handbook Engine Extraction Phase 1 Slice 4 - Authoring Layout

## Objective

Extend the compiler-local layout family so the authoring modules consume typed authoring ownership for canonical write targets and authoring lock paths without changing current charter, project-context, or environment-inventory behavior.

Spec reference: [handbook-engine-extraction-phase-1-slice-4-authoring-layout-spec.md](./handbook-engine-extraction-phase-1-slice-4-authoring-layout-spec.md)

## Major Artifacts

1. Authoring layout owner surface
   - lives in `crates/compiler/src/layout.rs`
   - owns canonical authoring target derivation for charter, project context, and environment inventory
   - owns authoring lock-path derivation under `.handbook/state/authoring/**`

2. Authoring consumer adoptions
   - updates `crates/compiler/src/author/charter.rs`
   - updates `crates/compiler/src/author/project_context.rs`
   - updates `crates/compiler/src/author/environment_inventory.rs`
   - preserves validation, prompt, refusal, and write behavior inside those modules

3. Shared authoring helper alignment
   - keeps `crates/compiler/src/author/mod.rs` available for narrow compile-through helper wiring only if the authoring modules need shared path accessors or helper signatures adjusted
   - does not widen the shared helper layer into prompt cleanup or a public API redesign

4. Regression coverage
   - keeps `crates/compiler/tests/author.rs` as the primary safety wall
   - adjusts tests only where the new owner seam needs direct proof

## Dependencies And Order

### Packet 1.4.1 only: Authoring Roots And Lock Paths Adoption

Why one packet is enough:

- Slice 1.4 has one dominant seam: authoring canonical-target and lock-path ownership
- the three authoring modules share the same ownership domain even though their behavior differs
- the `author` test wall already spans the three affected modules, so the packet can stop cleanly once all authoring consumers use the layout seam

Recommended internal order:

1. Define the authoring layout owner in `layout.rs`
2. Adopt `author/charter.rs` and `author/project_context.rs` onto that owner first because both use the shared authoring lock helper and canonical write-target validation pattern
3. Adopt `author/environment_inventory.rs` last because it has bespoke lock acquisition and upstream canonical-truth checks that should consume the same owner without forcing helper redesign
4. Re-run the full authoring wall and confirm prompt/template/shell boundaries stayed intact

Output:

- one extended `layout.rs` family that defines authoring canonical-target and lock-path owner surfaces
- authoring modules that consume those owners for write-target and lock ownership
- unchanged canonical identities, prompt text, structured-input schemas, template behavior, and refusal posture

## Risks And Mitigations

### Risk: Slice 1.4 drifts into prompt, template, or shell-wording cleanup

Mitigation:

- keep prompt text, template strings, default exception location, and refusal wording local in the existing modules
- reject changes whose primary purpose is prompt cleanup or CLI/operator wording cleanup

### Risk: authoring adoption redefines canonical artifact identity instead of consuming it

Mitigation:

- preserve the Slice 1.2 canonical owner as-is
- route authoring target ownership through the approved layout family instead of inventing new canonical paths
- require repo-relative targets to remain identical to current canonical artifact identities

### Risk: environment-inventory behavior drifts while lock-path ownership is adopted

Mitigation:

- preserve upstream charter/project-context dependency checks and synthesis-validation semantics as-is
- adopt lock-path ownership only, not the surrounding process-execution behavior
- keep the `author` test wall mandatory because it exercises the cross-artifact authoring flows

### Risk: the layout family becomes a monolithic all-storage object too early

Mitigation:

- preserve the separate layout type-family contract from Slice 1.1
- prefer a dedicated authoring owner layered into the existing family instead of one mega-struct
- keep the layout module compiler-internal until a later slice proves the outward API we want to freeze

### Risk: tests stay green while authoring-path boundaries drift

Mitigation:

- pair the `author` test wall with an explicit inventory grep over authoring path literals and lock-path literals
- treat required changes to prompt strings, setup flow, or CLI wording as out-of-scope leakage unless they are proven compile-through wiring only

## Parallel Vs Sequential

Sequential:

- define the authoring layout owner before any authoring module adopts it
- adopt charter/project-context before environment inventory if shared helper alignment is needed
- run the full authoring wall after all three consumer adoptions are in place

Not parallel:

- do not split the authoring layout owner and authoring consumer adoption across simultaneous packets
- do not start Phase 2 target/template work from the authoring modules
- do not mix prompt cleanup, shell-wording cleanup, or setup changes into this slice

## Verification Checkpoints

### Checkpoint 1: Authoring layout owner introduced

Confirm the compiler now has authoring ownership accessors for canonical write targets and authoring lock paths.

Suggested verification:

```bash
rg -n "Authoring|authoring|lock|CANONICAL_(CHARTER|PROJECT_CONTEXT|ENVIRONMENT_INVENTORY)_REPO_PATH|state/authoring" \
  crates/compiler/src/layout.rs \
  crates/compiler/src/author/charter.rs \
  crates/compiler/src/author/project_context.rs \
  crates/compiler/src/author/environment_inventory.rs

cargo check -p handbook-compiler
```

### Checkpoint 2: Authoring consumers adopted

Confirm the authoring modules now consume the extended layout family for target and lock ownership.

Suggested verification:

```bash
rg -n "state/authoring|CANONICAL_(CHARTER|PROJECT_CONTEXT|ENVIRONMENT_INVENTORY)_REPO_PATH" \
  crates/compiler/src/layout.rs \
  crates/compiler/src/author/charter.rs \
  crates/compiler/src/author/project_context.rs \
  crates/compiler/src/author/environment_inventory.rs

cargo test -p handbook-compiler --test author
cargo check -p handbook-compiler
```

### Checkpoint 3: Slice boundary remains intact

Confirm the slice stayed within the approved authoring corpus and still compiles cleanly.

Suggested verification:

```bash
cargo test -p handbook-compiler --test author
cargo check -p handbook-compiler
```

## Exit Conditions

Slice 1.4 is ready for human review when:

- authoring canonical-target and lock-path ownership have one typed compiler-local owner
- `author/charter.rs`, `author/project_context.rs`, and `author/environment_inventory.rs` all consume that owner
- canonical identities, prompt text, template behavior, and refusal semantics remain unchanged
- no Phase 2 target/template work or Phase 3 shell cleanup leaked into the slice
- the `author` test wall passes cleanly

Slice 1.4 is ready for implementation only after the human reviews and accepts the spec/plan/tasks set.
