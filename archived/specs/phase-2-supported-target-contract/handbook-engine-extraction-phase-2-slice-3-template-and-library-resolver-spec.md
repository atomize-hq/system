# Spec: Handbook Engine Extraction Phase 2 Slice 3 (Slice 2.3) - Template And Library Resolver Boundary

## Assumptions

1. Phase 2 Slice 2 is complete, so supported-target runtime adoption is already owned by the approved Slice 2.1 and Slice 2.2 authority set and does not need to be reopened here.
2. Live pipeline compilation already loads `inputs.library` declaratively from stage definitions under `core/stages/**`, but direct authoring flows still hardcode shipped library assets inside Rust modules via `include_str!` constants.
3. The current hardcoded authoring owners are in `crates/compiler/src/author/charter.rs` and `crates/compiler/src/author/environment_inventory.rs`; those are the smallest live seams to move behind a typed resolver boundary without widening into Phase 3 shell cleanup.
4. Shipped defaults must remain the current repo-owned handbook defaults for this slice; Slice 2.3 may parameterize selection, but it must not add new library content, rewrite the author-facing documents, or redesign stage bodies.
5. Validated override support in this slice can be internal and typed; it does not require new public CLI flags or a new repo-level user configuration file unless live implementation work proves that one is strictly necessary and approved separately.
6. `project_context` deterministic rendering, Phase 1 setup/layout semantics, and Phase 3 shell-wording cleanup remain separate concerns unless a tiny compatibility adjustment is required to keep shipped-default posture coherent.

## Objective

Define one typed template/library resolver boundary for the remaining authoring surfaces that still freeze library selection in implementation details, while preserving current shipped defaults and adding bounded override rules.

The maintainer needs this slice so Phase 2 no longer mixes two different models:

- declarative stage library inputs in `core/stages/**` for pipeline compilation, and
- ad hoc embedded template/directive selection inside authoring modules.

Success means:

- charter and environment-inventory authoring select their shipped method/directive/template inputs through one typed resolver boundary instead of local hardcoded owners
- the zero-config shipped-default behavior remains unchanged
- any override or alternate selection path is validated, bounded, and explicit
- declarative stage library truth stays authoritative for stage compilation
- no new library content, target-adoption work, or Phase 3 shell cleanup leaks in

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Declarative stage truth under `core/stages/**`
- Repo-owned library assets under `core/library/**`
- Existing authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-compiler --test author
cargo test -p handbook-compiler --test pipeline_catalog
```

Authoring and CLI fallout guard:

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-compiler --test setup
```

Hardcoded-owner evidence scan:

```bash
rg -n "include_str!\(|setup_starter_template|core/library/.+tmpl|core/library/.+directive" crates/compiler/src/author crates/compiler/src/canonical_artifacts.rs crates/compiler/src/setup.rs
```

Final workspace verification wall:

```bash
cargo check --workspace
cargo test --workspace
```

## Project Structure

```text
crates/compiler/src/author/charter.rs                -> Guided charter authoring; currently owns shipped method/directive/template selection locally
crates/compiler/src/author/environment_inventory.rs  -> Guided environment-inventory authoring; currently owns shipped directive/template selection locally
crates/compiler/src/template_library.rs              -> Proposed typed resolver owner for shipped-default assets and bounded override validation
crates/compiler/src/pipeline.rs                      -> Declarative pipeline/stage catalog truth that should remain the authoritative observer for compile-time stage library inputs
crates/compiler/src/canonical_artifacts.rs           -> Canonical artifact identities and starter-template posture that may need to observe or align with shipped-default resolver behavior
crates/compiler/src/setup.rs                         -> Setup scaffold behavior and starter-template semantics that must not regress if shipped-default posture is tightened
crates/compiler/tests/author.rs                      -> Compiler-level authoring regression coverage
crates/compiler/tests/pipeline_catalog.rs            -> Declarative stage/library catalog regression coverage
crates/compiler/tests/setup.rs                       -> Setup starter-template regression coverage
crates/cli/tests/author_cli.rs                       -> Public authoring CLI regression coverage
core/library/**                                      -> Repo-owned directives, templates, and authoring-method content
core/stages/05_charter_synthesize.md                 -> Declarative stage reference for charter synthesis library inputs
core/stages/07_foundation_pack.md                    -> Declarative stage reference for environment-inventory template usage inside foundation-pack synthesis
docs/specs/                                          -> Slice 2.3 authority documents
```

## Code Style

Prefer a small typed selection contract plus thin consumers over module-local string constants that each decide their own template or directive source.

```rust
let selection = template_library_resolver.resolve(
    TemplateLibraryRequest::charter_synthesis().with_override(override_request),
)?;

prompt.push_str(selection.document(TemplateLibraryAsset::CharterAuthoringMethod)?.contents());
prompt.push_str(selection.document(TemplateLibraryAsset::CharterSynthesizeDirective)?.contents());
prompt.push_str(selection.document(TemplateLibraryAsset::CharterTemplate)?.contents());
```

Conventions:

- model library/template assets with typed identities, not raw stringly-typed file ownership spread across consumers
- keep shipped-default behavior explicit and zero-config
- keep stage catalog truth authoritative for compile-time stage inputs
- keep authoring output validation local to each authoring module even after selection moves behind the resolver
- refuse unsafe override paths instead of silently falling back or widening scope

## Testing Strategy

- Framework: existing Rust integration tests in `crates/compiler/tests/` and CLI regression tests in `crates/cli/tests/`
- Primary test levels:
  - compiler authoring tests for shipped-default prompt assembly and output validation
  - pipeline catalog tests to ensure declarative library truth remains authoritative
  - setup tests to guard starter-template posture if any compatibility plumbing touches canonical defaults
  - CLI authoring tests to guard public behavior for `handbook author charter` and `handbook author environment-inventory`
- Coverage focus:
  - shipped-default charter and environment-inventory selection remains byte-for-byte behavior-compatible where intended
  - override requests are accepted only for approved asset families and refused for invalid paths or mismatched asset kinds
  - stage library declarations in `core/stages/**` remain unchanged and authoritative
  - starter-template and baseline/status behavior stays stable unless the approved slice explicitly changes it
- Coverage expectation:
  - Packet 2.3.1 proves zero-config shipped-default adoption first
  - Packet 2.3.2 adds explicit acceptance and refusal coverage for bounded override rules

## Slice Scope

In scope:

- define a typed template/library selection boundary for the shipped charter and environment-inventory authoring assets
- remove module-local ownership of shipped authoring method/directive/template selection from `author/charter.rs` and `author/environment_inventory.rs`
- preserve declarative stage library truth as the authoritative observer for compile-time stage inputs
- define approved asset families, shipped-default posture, and typed selection results
- add validated override and alternate-selection rules for approved asset families within bounded repo-relative rules
- add regression coverage proving zero-config behavior stays stable and invalid override attempts refuse safely

Out of scope:

- adding new library content, new templates, or new authoring documents
- changing supported-target runtime ownership from Slice 2.2
- redesigning `project_context` deterministic rendering
- broad setup/canonical-artifact redesign beyond tiny compatibility plumbing
- Phase 3 shell-wording cleanup, prompt rewriting, or operator-copy redesign
- introducing new public CLI flags or unbounded user-provided file-path selection
- changing stage front matter or pipeline catalog truth into a secondary source of library selection

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 2.2 authority set:
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-2-runtime-target-adoption-tasks.md`
- Live shipped-default owners and observers:
  - `crates/compiler/src/author/charter.rs`
  - `crates/compiler/src/author/environment_inventory.rs`
  - `crates/compiler/src/template_library.rs`
  - `crates/compiler/src/pipeline.rs`
  - `crates/compiler/src/canonical_artifacts.rs`
  - `crates/compiler/src/setup.rs`
  - `core/stages/05_charter_synthesize.md`
  - `core/stages/07_foundation_pack.md`
  - `core/library/**`
- Live regression evidence:
  - `crates/compiler/tests/author.rs`
  - `crates/compiler/tests/pipeline_catalog.rs`
  - `crates/compiler/tests/setup.rs`
  - `crates/cli/tests/author_cli.rs`

## Current Hardcoded Owners To Retire

| Surface | Current live owner | Slice 2.3 requirement |
| --- | --- | --- |
| charter authoring | `AUTHORING_METHOD_MARKDOWN`, `CHARTER_SYNTHESIZE_DIRECTIVE_MARKDOWN`, and `CHARTER_TEMPLATE_MARKDOWN` in `author/charter.rs` | select the shipped charter assets through one typed resolver boundary while leaving charter-specific validation and wording local |
| environment-inventory authoring | `ENVIRONMENT_INVENTORY_SYNTHESIZE_DIRECTIVE_MARKDOWN` and `ENVIRONMENT_INVENTORY_TEMPLATE_MARKDOWN` in `author/environment_inventory.rs` | select the shipped environment-inventory assets through the same typed boundary while leaving inventory-specific validation and wording local |
| declarative stage compilation | `inputs.library` in `core/stages/**`, loaded via `pipeline.rs` and `pipeline_compile.rs` | remain the authoritative declarative source; Slice 2.3 must not demote stage truth to an afterthought |
| starter-template posture | `setup_starter_template*` helpers and setup semantics in `canonical_artifacts.rs` and `setup.rs` | stay behavior-stable unless tiny compatibility plumbing is needed to keep shipped-default posture coherent |

## Boundaries

- Always:
  - preserve shipped-default behavior for zero-config authoring flows first
  - keep stage front matter and `core/library/**` as repo-owned truth
  - keep charter and environment-inventory output validation local to those modules
  - make override acceptance explicit, typed, and bounded
  - prove resolver adoption with author, catalog, and CLI regression coverage
- Ask first:
  - adding a new public CLI flag, config file, or user-editable manifest for template selection
  - expanding override support beyond the approved charter/environment-inventory asset families
  - moving setup starter-template ownership fully behind the new resolver if that requires semantic changes
  - changing stage front matter, library layout, or library file names
- Never:
  - allow arbitrary absolute paths, traversal, or out-of-root file selection
  - add new library content as part of the boundary extraction
  - reopen Slice 2.2 runtime target adoption or Phase 3 shell cleanup in this slice
  - let authoring modules and the new resolver both remain competing owners of shipped asset selection
  - treat declarative stage library truth as optional or stale after the resolver lands

## Success Criteria

- One typed resolver boundary exists for the approved charter and environment-inventory shipped assets.
- `author/charter.rs` and `author/environment_inventory.rs` consume that boundary instead of owning local shipped asset selection.
- Zero-config shipped-default behavior remains stable under `author`, `author_cli`, and `setup` regression coverage.
- Approved override and alternate-selection rules are typed, bounded, and refusal-tested.
- Declarative stage library truth in `core/stages/**` remains authoritative and regression-covered.
- No Slice 2.2 target-adoption work, new library content, or Phase 3 shell cleanup leaked in.

## Open Questions

- Is the smallest durable owner a new sibling resolver module, or does live code truth make `pipeline.rs` the right home for the typed asset identities and selection boundary?
- Should starter-template compatibility remain a guarded observer concern in Slice 2.3, or does live implementation prove it must participate directly in the same resolver boundary?
