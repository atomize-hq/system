# Spec: Handbook Engine Extraction Phase 3 Slice 1 (Slice 3.1) - Charter Deterministic Core Split

## Assumptions

1. Phase 2 Slice 3 is complete, so `template_library.rs` is now the approved shipped-default owner for charter authoring assets and must stay that way during this slice.
2. The next migration gate is not new behavior; it is an internal boundary cleanup inside `crates/compiler/src/author/charter.rs` so Phase 4 can later move reusable charter logic into `handbook-engine` without dragging Codex runtime and product-shell behavior with it.
3. Live charter code currently mixes at least four concerns in one module:
   - deterministic structured-input parsing, normalization, validation, and markdown rendering
   - canonical authoring preflight, lock, and write orchestration
   - guided synthesis prompt assembly and `codex exec` process transport
   - product-shell refusal summaries and next-safe-action wording that reference `handbook author charter`
4. Slice 3.1 must preserve the current public compiler and CLI surface: `handbook author charter`, `--from-inputs`, `--validate`, canonical write targets, and refusal posture all stay behavior-stable unless the human approves a separate shell redesign.
5. `project_context`, `environment_inventory`, `setup`, `doctor`, and general CLI thinning remain deferred to later Phase 3 / Phase 5 slices even if live code shows similar duplication.
6. The smallest durable seam is expected to keep `crates/compiler/src/author/charter.rs` as the public facade while extracting new charter-local subowners such as `charter_core.rs` and `charter_shell.rs`; if live implementation proves a different file split is cleaner, the ownership contract still must match this spec.

## Objective

Separate reusable deterministic charter logic from guided synthesis and product-shell behavior inside the current compiler crate, without changing the public authoring contract.

The maintainer needs this slice so the future `handbook-engine` boundary is already visible before any crate move begins. Success means:

- charter structured-input types, normalization, validation, markdown rendering, and markdown validation live behind one engine-shaped deterministic boundary
- guided synthesis prompt assembly, Codex runtime transport, environment-variable overrides, temp-output handling, and CLI-oriented refusal wording live outside that deterministic boundary
- `author_charter`, `author_charter_guided`, and preflight entrypoints become thin orchestrators over the split instead of owning every concern directly
- shipped template-library ownership from Slice 2.3 remains intact
- no project-context, environment-inventory, setup, doctor, or CLI-thinning work leaks in

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Existing charter authoring assets resolved via `crates/compiler/src/template_library.rs`
- Existing authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-compiler --test author
```

Public-surface fallout guard:

```bash
cargo test -p handbook-cli --test author_cli
```

Deterministic-vs-shell ownership evidence scan:

```bash
rg -n "Command::new|Stdio|AUTHOR_CHARTER_CODEX_|std::env::var|temp_dir|next_safe_action|handbook author charter" crates/compiler/src/author/charter*.rs crates/compiler/src/author/mod.rs
```

Core API evidence scan:

```bash
rg -n "parse_charter_structured_input_yaml|normalize_charter_free_text|validate_charter_structured_input|render_charter_markdown|validate_charter_markdown" crates/compiler/src/author/charter*.rs crates/compiler/src/author/mod.rs
```

Final workspace verification wall:

```bash
cargo check --workspace
cargo test --workspace
```

## Project Structure

```text
crates/compiler/src/author/charter.rs         -> current public charter facade; should become a thin orchestration layer after the split
crates/compiler/src/author/charter_core.rs    -> proposed deterministic charter owner for types, normalization, validation, rendering, and markdown validation
crates/compiler/src/author/charter_shell.rs   -> proposed guided synthesis / preflight / lock / write / refusal shell owner
crates/compiler/src/author/mod.rs             -> author-surface reexports that should keep the outward compiler API stable
crates/compiler/src/template_library.rs       -> approved shipped-default asset resolver from Slice 2.3; shell-only dependency for guided synthesis
crates/compiler/tests/author.rs               -> compiler-level charter regression coverage for deterministic and guided paths
crates/cli/tests/author_cli.rs                -> CLI-level charter behavior and refusal-surface regression coverage
crates/cli/src/main.rs                        -> existing CLI consumer that should remain behavior-stable without becoming the owner of charter internals
```

If live implementation proves `author/charter/{core,shell}.rs` is cleaner than sibling files, that layout is acceptable only if the same ownership split and public API stability hold.

## Code Style

Prefer a thin public adapter over a monolithic file that owns both reusable modeling and product runtime behavior.

```rust
let normalized = charter_core::normalize_structured_input(input.clone());
charter_core::validate_structured_input(&normalized)?;

let markdown = charter_core::render_markdown(&normalized)?;
charter_core::validate_markdown(&markdown)?;

let prompt = charter_shell::build_synthesis_prompt(&normalized)?;
let synthesized = charter_shell::run_guided_synthesis(repo_root, prompt)?;
```

Conventions:

- deterministic charter core must not spawn processes, read environment overrides, create temp files, or mutate the repo
- shell adapters may own `codex exec`, lock handling, canonical writes, and CLI-oriented refusal wording
- keep the public `handbook_compiler` exports stable through `author/mod.rs`
- keep `template_library.rs` as the only shipped-asset selector; do not reintroduce local charter asset ownership
- prefer typed handoffs between core and shell instead of cross-calling helper internals ad hoc

## Testing Strategy

- Framework: existing Rust integration tests in `crates/compiler/tests/` and CLI regression tests in `crates/cli/tests/`
- Primary test levels:
  - compiler author tests for deterministic parse/render/validate behavior
  - compiler author tests for guided synthesis prompt/runtime refusal behavior
  - CLI author tests to prove the outward `handbook author charter` contract stays stable
- Coverage focus:
  - deterministic charter entrypoints remain pure and do not invoke Codex or runtime overrides
  - guided charter entrypoints still resolve shipped template assets through `template_library.rs`
  - synthesis failures still refuse without partial canonical writes
  - refusal summaries and next-safe-action wording remain stable at the CLI surface unless the spec changes first
  - public parse/render/validate functions stay exported and usable without forcing callers through shell runtime paths
- Coverage expectation:
  - Packet 3.1.1 proves the deterministic charter core boundary without changing guided behavior
  - Packet 3.1.2 proves guided synthesis and shell orchestration still pass compiler and CLI regression rails after the split

## Slice Scope

In scope:

- extract deterministic charter types, normalization, validation, rendering, and markdown-validation behavior into one engine-shaped owner
- reduce `charter.rs` to a thin orchestrator and public facade
- isolate guided synthesis prompt assembly, Codex runtime transport, env-var overrides, synthesized-markdown validation, and refusal wording away from the deterministic core
- isolate authoring preflight / lock / canonical write orchestration away from deterministic charter modeling where practical inside this slice
- preserve the current public compiler and CLI charter contract

Out of scope:

- changing `project_context` or `environment_inventory`
- redesigning CLI prompts, flags, success rendering, or help text
- changing charter markdown semantics, template contents, or template-library selection rules
- adding new public authoring configuration or runtime knobs
- moving code into new crates before Phase 4
- widening into setup, doctor, refusal, or general shell-copy cleanup outside charter authoring

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 2.3 authority set:
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-2-slice-3-template-and-library-resolver-tasks.md`
- Live charter surfaces:
  - `crates/compiler/src/author/charter.rs`
  - `crates/compiler/src/author/mod.rs`
  - `crates/compiler/src/template_library.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/cli/src/main.rs`
- Live regression evidence:
  - `crates/compiler/tests/author.rs`
  - `crates/cli/tests/author_cli.rs`

## Current Mixed Responsibilities To Untangle

| Responsibility | Current live owner | Slice 3.1 requirement |
| --- | --- | --- |
| structured-input model, normalization, and validation | `CharterStructuredInput` plus `parse_charter_structured_input_yaml`, `normalize_charter_free_text`, and `validate_charter_structured_input` in `author/charter.rs` | move behind one deterministic core boundary with no process/env/filesystem ownership |
| compiler-owned markdown rendering and validation | `render_charter_markdown`, `validate_charter_markdown`, and related render helpers in `author/charter.rs` | keep as reusable engine-safe behavior and make shell adapters consume it |
| guided prompt assembly and shipped asset consumption | `build_charter_synthesis_prompt` and template-library calls in `author/charter.rs` | keep shell-owned and dependent on the Slice 2.3 resolver, not on the deterministic core |
| Codex runtime transport and synthesized-output validation | `synthesize_charter_markdown`, env-var overrides, temp output handling, and synthesized-markdown refusal paths in `author/charter.rs` | isolate as shell/runtime behavior outside the deterministic core |
| canonical preflight, lock, and write orchestration | `preflight_author_charter*`, `author_charter`, and `author_charter_guided` in `author/charter.rs` | leave as thin orchestration around core + shell helpers instead of mixed owner logic |
| CLI-oriented refusal summaries and next-safe-action wording | refusal constructors in `author/charter.rs` that reference `handbook author charter` | keep shell-owned so Phase 4 engine migration does not inherit product copy as core behavior |

## Boundaries

- Always:
  - keep deterministic charter code free of process spawning, env-var reads, temp files, and repo mutation
  - preserve current public charter behavior first; this slice is an internal seam cleanup
  - keep `template_library.rs` as the only shipped charter asset owner
  - prove both compiler-level and CLI-level charter regressions after the split
  - keep charter-specific refusal classification and validation semantics stable unless a spec update says otherwise
- Ask first:
  - renaming public `handbook_compiler` charter functions or types
  - introducing new CLI flags, prompts, or runtime configuration
  - extracting shared authoring shell helpers that also touch project-context or environment-inventory
  - changing the canonical charter write contract or lock-file contract
- Never:
  - move Phase 4 crate-boundary work into this slice
  - let the deterministic core depend directly on `Command`, `Stdio`, env overrides, or `template_library.rs`
  - reintroduce shipped template ownership into `charter.rs` or the new core
  - widen into project-context, environment-inventory, setup, doctor, or CLI thinning
  - change charter document semantics or template text as a side effect of the split

## Success Criteria

- A deterministic charter core boundary exists for structured-input types, normalization, validation, markdown rendering, and markdown validation.
- `author_charter`, `author_charter_guided`, and preflight entrypoints become thin orchestrators over deterministic core and shell helpers.
- Guided prompt assembly, Codex runtime transport, env-var overrides, temp-output handling, and CLI-oriented refusal wording are isolated outside the deterministic core.
- Slice 2.3 template-library ownership remains intact and unchanged.
- `cargo test -p handbook-compiler --test author` and `cargo test -p handbook-cli --test author_cli` continue to pass against the split architecture.
- No project-context, environment-inventory, setup, doctor, or Phase 4 crate-move work leaked in.

## Open Questions

- Should the thin public facade remain `crates/compiler/src/author/charter.rs` with sibling `charter_core.rs` / `charter_shell.rs`, or does a nested `author/charter/` module layout make the later Phase 4 engine migration materially cleaner?
- Is it enough for Slice 3.1 to keep preflight / lock / write orchestration charter-local in the shell adapter, or does live implementation prove a shared authoring-shell helper is necessary later?
