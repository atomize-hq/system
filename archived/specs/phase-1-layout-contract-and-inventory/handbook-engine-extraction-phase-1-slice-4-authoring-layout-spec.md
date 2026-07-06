# Spec: Handbook Engine Extraction Phase 1 Slice 4 - Authoring Layout

## Assumptions

1. Slice 1.1 is already the accepted authority for the separate-layout-types contract and the Phase 1 ownership-domain names.
2. Slice 1.2 already landed the canonical layout owner in `crates/compiler/src/layout.rs`, and Slice 1.4 must reuse that family for canonical authoring targets rather than introducing a competing layout entrypoint.
3. Live repo truth after the current Phase 1 work shows `layout.rs` already carries canonical, runtime-state, capture-provenance, and handoff-bundle owners; Slice 1.4 should extend that same compiler-local family without reopening earlier slice decisions.
4. Slice 1.4 is the remaining **code-adoption** slice in Phase 1 for authoring storage ownership: it moves canonical authoring target paths and authoring lock-file paths behind the approved layout seam, but it must stay behavior-neutral.
5. Prompt wording, template text, structured-input schemas, refusal wording, and operator-facing guidance inside the authoring modules remain local here; Slice 1.4 moves storage ownership only, not shell/prompt cleanup.
6. Canonical artifact semantics already frozen by Slice 1.2 must remain unchanged while the authoring modules adopt the layout seam underneath their write targets.

## Objective

Land the final behavior-neutral layout adoption slice for Phase 1 by moving authoring canonical target paths and authoring lock-file paths behind the approved layout family without widening into prompt cleanup, canonical identity redesign, or Phase 2 target/template work.

The user is the maintainer of the handbook workspace and the reviewer of the extraction sequence. The immediate outcome is not a product-surface change. The immediate outcome is a narrow internal adoption that proves the Slice 1.1 authoring ownership domain can replace scattered authoring path literals while preserving current charter, project-context, and environment-inventory behavior.

Success means all of the following are true:

- authoring path ownership has one typed compiler-local owner instead of remaining duplicated inside the authoring modules
- `author/charter.rs`, `author/project_context.rs`, and `author/environment_inventory.rs` consume layout accessors for canonical write targets and authoring lock paths
- canonical artifact identities and write destinations remain unchanged
- charter, project-context, and environment-inventory authoring semantics remain unchanged
- prompt text, template expectations, and refusal wording remain local and unchanged

## Slice Scope

In scope:

- extend the compiler-local layout family for authoring canonical-target and lock-path ownership
- move charter, project-context, and environment-inventory canonical write-target ownership behind that surface
- move authoring lock-file path ownership under `.handbook/state/authoring/**` behind that surface
- preserve existing authoring validation, synthesis, refusal, and mutation semantics while improving ownership boundaries

Out of scope:

- changing prompt wording, template text, headings, or structured-input schemas
- changing canonical artifact identities or canonical layout ownership already frozen by Slice 1.2
- changing runtime-state, capture-provenance, or handoff-bundle ownership already handled by Slice 1.3
- changing test-only prompt-capture paths such as `.handbook/state/authoring/last_prompt.txt`
- widening into Phase 2 target/template parameterization
- widening into Phase 3 shell-wording cleanup

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-tasks.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-tasks.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-spec.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-plan.md`
- `docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-tasks.md`
- Current Slice 1.4 implementation corpus:
  - `crates/compiler/src/layout.rs`
  - `crates/compiler/src/author/mod.rs`
  - `crates/compiler/src/author/charter.rs`
  - `crates/compiler/src/author/project_context.rs`
  - `crates/compiler/src/author/environment_inventory.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/compiler/tests/author.rs`

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- compiler-internal typed layout ownership surface under `crates/compiler/src/layout.rs`
- repo-local canonical authoring targets under `.handbook/charter/**`, `.handbook/project_context/**`, and `.handbook/environment_inventory/**`
- repo-local authoring locks under `.handbook/state/authoring/**`
- authoring regression coverage in `crates/compiler/tests/author.rs`

## Commands

Slice 1.4 inventory query:

```bash
rg -n "\\.handbook/(charter|project_context|environment_inventory)|\\.handbook/state/authoring|CANONICAL_(CHARTER|PROJECT_CONTEXT|ENVIRONMENT_INVENTORY)_REPO_PATH|LOCK_REPO_PATH" \
  crates/compiler/src/layout.rs \
  crates/compiler/src/author/mod.rs \
  crates/compiler/src/author/charter.rs \
  crates/compiler/src/author/project_context.rs \
  crates/compiler/src/author/environment_inventory.rs
```

Primary packet verification rail:

```bash
cargo test -p handbook-compiler --test author
```

Behavior-neutral compile rail:

```bash
cargo check -p handbook-compiler
```

Repo verification wall for a landed Slice 1.4 packet:

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-compiler --all-targets -- -D warnings
cargo test -p handbook-compiler --test author
cargo check -p handbook-compiler
```

## Project Structure

```text
HANDBOOK_ENGINE_EXTRACTION_PLAN.md                                               -> Root phase-order authority
docs/specs/handbook-engine-extraction-slice-map.md                               -> Phase -> Slice -> Packet authority
docs/specs/handbook-engine-extraction-phase-1-slice-1-layout-contract-and-inventory-*.md
                                                                                 -> Slice 1.1 ownership-domain authority
docs/specs/handbook-engine-extraction-phase-1-slice-2-canonical-and-setup-layout-*.md
                                                                                 -> Slice 1.2 canonical adoption authority
docs/specs/handbook-engine-extraction-phase-1-slice-3-stateful-storage-layout-*.md
                                                                                 -> Slice 1.3 stateful-storage adoption authority
docs/specs/handbook-engine-extraction-phase-1-slice-4-authoring-layout-*.md      -> Slice 1.4 authority set
crates/compiler/src/layout.rs                                                    -> Compiler-internal layout family extended by this slice
crates/compiler/src/author/mod.rs                                                -> Shared authoring helpers and lock/write-path utilities
crates/compiler/src/author/charter.rs                                            -> Charter authoring flow, validation, guided synthesis, and canonical write target
crates/compiler/src/author/project_context.rs                                    -> Project-context authoring flow, validation, and canonical write target
crates/compiler/src/author/environment_inventory.rs                              -> Environment-inventory authoring flow, validation, and canonical write target
crates/compiler/tests/author.rs                                                  -> Authoring regression coverage across charter, project context, and environment inventory
```

## Code Style

Prefer a narrow typed authoring owner over duplicated path constants or ad hoc path assembly inside authoring modules.

```rust
let repo = RepoLayoutRoot::new(repo_root);
let authoring = repo.authoring();

let charter_target = authoring.charter().canonical_target();
let charter_lock = authoring.charter().lock_path();
let project_context_target = authoring.project_context().canonical_target();
let environment_inventory_lock = authoring.environment_inventory().lock_path();
```

Conventions for this slice:

- `layout.rs` owns authoring canonical-target derivation and authoring lock-path derivation
- `author/charter.rs` keeps structured-input validation, guided-synthesis semantics, default exception location, and refusal wording local while consuming layout accessors for write-target and lock ownership
- `author/project_context.rs` keeps metadata rendering, structured-input validation, and refusal wording local while consuming layout accessors for write-target and lock ownership
- `author/environment_inventory.rs` keeps prompt generation, upstream canonical-truth checks, synthesis behavior, and refusal wording local while consuming layout accessors for write-target and lock ownership
- `author/mod.rs` may assist with compile-through helper wiring, but this slice does not widen shared helpers into a public layout contract
- do not introduce a “global layout” type that simultaneously claims canonical, runtime-state, authoring, and Phase 2 target ownership in this slice
- do not move prompt-facing `.handbook/...` references just because they resemble storage paths; move only the owner rules first

## Testing Strategy

Primary verification for this slice is **behavior-preserving authoring-path adoption**, not new product functionality.

Test levels:

- targeted integration coverage in `crates/compiler/tests/author.rs`
- `cargo check -p handbook-compiler` after the authoring owners are integrated

Coverage expectations:

- charter canonical write target and lock-path behavior remain unchanged
- project-context canonical write target and lock-path behavior remain unchanged
- environment-inventory canonical write target and lock-path behavior remain unchanged
- guided authoring, from-input authoring, synthesis validation, and refusal semantics remain unchanged
- prompt text, heading validation, metadata wording, and default exception location remain unchanged
- no Phase 2 target/template or Phase 3 shell-wording work is required to land Slice 1.4

## Boundaries

- Always:
  - preserve the Slice 1.1 separate-layout-types contract and the Slice 1.2 canonical-layout decisions
  - keep Slice 1.4 limited to authoring canonical-target and authoring lock-path ownership adoption
  - preserve authoring write-destination, lock behavior, validation, refusal, and prompt semantics
  - keep prompt wording, templates, and user-facing guidance local until their approved later slices
- Ask first:
  - changing canonical repo-relative artifact paths or lock-file relative identities
  - changing `DEFAULT_EXCEPTION_RECORD_LOCATION` or other prompt-facing path strings beyond narrow compile-through wiring
  - widening into `setup.rs`, `doctor.rs`, `refusal.rs`, `pipeline*`, or non-authoring modules beyond narrow compile-through wiring
  - changing prompt text, template content, or CLI/operator wording
  - introducing new crates, dependencies, or public API promises beyond what this slice needs
- Never:
  - collapse the layout family into one monolithic layout object
  - change canonical artifact identity semantics as a side effect of authoring-path adoption
  - rewrite guided-authoring prompts or template rules as part of storage adoption
  - start Phase 2 or Phase 3 cleanup from inside Slice 1.4

## Success Criteria

- Slice 1.4 extends the compiler-local layout family with typed authoring ownership surfaces for canonical targets and authoring lock paths.
- `author/charter.rs` consumes the authoring layout owner for canonical write-target and lock-path derivation.
- `author/project_context.rs` consumes the authoring layout owner for canonical write-target and lock-path derivation.
- `author/environment_inventory.rs` consumes the authoring layout owner for canonical write-target and lock-path derivation.
- Existing charter, project-context, and environment-inventory authoring semantics remain unchanged.
- `cargo test -p handbook-compiler --test author` passes.
- `cargo check -p handbook-compiler` passes.
- Phase 2 and Phase 3 adoption remains deferred.

## Open Questions

- Should the authoring layout owner delegate canonical artifact targets through `CanonicalLayout::artifact_path`, or should it expose artifact-specific authoring accessors directly while still preserving Slice 1.2 ownership decisions?
- Should `environment_inventory.rs` keep its bespoke lock-acquisition implementation while consuming only layout-owned lock-path derivation, or should a later slice unify that behavior with the shared authoring lock helper?

## Packet Breakdown

### Packet 1.4.1: Authoring Roots And Lock Paths Adoption

Goal:

- route authoring canonical write-target and lock-path ownership through the extended layout family while leaving prompt wording, template text, and canonical semantics untouched

Required outcome:

- `crates/compiler/src/layout.rs` owns authoring canonical-target and authoring lock-path derivation
- `crates/compiler/src/author/charter.rs`, `crates/compiler/src/author/project_context.rs`, and `crates/compiler/src/author/environment_inventory.rs` become consumers of those owners without changing authoring behavior
