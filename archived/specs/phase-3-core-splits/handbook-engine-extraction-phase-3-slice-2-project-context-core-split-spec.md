# Spec: Handbook Engine Extraction Phase 3 Slice 2 (Slice 3.2) - Project-Context Deterministic Core Split

## Assumptions

1. Phase 3 Slice 1 is complete, so the authoring-module split pattern established for charter (`public facade` + `deterministic core` + `shell/runtime helpers`) is now an approved in-repo precedent for the remaining Phase 3 authoring surfaces.
2. Phase 2 Slice 3 is complete, so template/library resolution is already parameterized where needed; Slice 3.2 must not reopen template ownership or shipped-default selection rules.
3. Live project-context guided answer collection already lives in `crates/cli/src/main.rs`, not in `crates/compiler/src/author/project_context.rs`; in this slice, the compiler-side “guided/product-shell” cleanup means preserving CLI-owned interview behavior while removing recovery wording and runtime authoring concerns from the deterministic compiler core.
4. Live `crates/compiler/src/author/project_context.rs` currently mixes at least four concerns in one module:
   - deterministic structured-input types, normalization, factuality checks, render-safety checks, markdown rendering, and markdown validation
   - non-deterministic render timestamp resolution via env/clock access
   - canonical authoring preflight, lock acquisition, canonical-write validation, and repo mutation
   - product-shell refusal summaries and next-safe-action wording that reference `handbook author project-context`
5. Slice 3.2 must preserve the current public compiler and CLI surface: `handbook author project-context`, `--from-inputs`, the guided TTY interview, canonical write targets, refusal posture, and current markdown shape stay behavior-stable unless the human approves a separate shell redesign.
6. `environment_inventory`, `setup`, `doctor`, and CLI-thinning work remain deferred to later Phase 3 / Phase 5 slices even if live code shows parallel cleanup opportunities.
7. The smallest durable seam is expected to keep `crates/compiler/src/author/project_context.rs` as the public facade while extracting new project-context-local subowners such as `project_context_core.rs` and `project_context_shell.rs`; if live implementation proves a different file split is cleaner, the ownership contract still must match this spec.

## Objective

Separate reusable deterministic project-context logic from runtime authoring and product-shell behavior inside the current compiler crate, without changing the public project-context authoring contract.

The maintainer needs this slice so the future `handbook-engine` boundary is visible before any crate move begins. Success means:

- project-context structured-input types, normalization, factuality validation, markdown rendering, and markdown validation live behind one engine-shaped deterministic boundary
- render timestamp resolution, canonical authoring preflight, lock/write orchestration, and CLI-oriented refusal wording live outside that deterministic boundary
- `author_project_context`, `author_project_context_from_input`, and preflight entrypoints become thin orchestrators over the split instead of owning every concern directly
- the CLI-guided interview remains CLI-owned and behavior-stable
- no environment-inventory, setup, doctor, or CLI-thinning work leaks in

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Existing authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-tasks.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-compiler --test author
```

Public-surface fallout guard:

```bash
cargo test -p handbook-cli --test author_cli
```

Shell-vs-core ownership evidence scan:

```bash
rg -n "AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR|resolve_project_context_now_utc|next_safe_action|handbook author project-context|acquire_authoring_lock|write_repo_relative_bytes|validate_canonical_write_target|CanonicalArtifacts::load" crates/compiler/src/author/project_context*.rs crates/compiler/src/author/mod.rs
```

Core API evidence scan:

```bash
rg -n "parse_project_context_structured_input_yaml|validate_project_context_structured_input|render_project_context_markdown|validate_project_context_markdown|normalized_project_context_structured_input|collect_render_safety_issues|validate_known_fake_project_context_markers" crates/compiler/src/author/project_context*.rs crates/compiler/src/author/mod.rs
```

Final workspace verification wall:

```bash
cargo check --workspace
cargo test --workspace
```

## Project Structure

```text
crates/compiler/src/author/project_context.rs          -> current public project-context facade; should become a thin orchestration layer after the split
crates/compiler/src/author/project_context_core.rs     -> proposed deterministic owner for structured-input types, normalization, validation, deterministic markdown rendering, and markdown validation
crates/compiler/src/author/project_context_shell.rs    -> proposed shell owner for timestamp resolution, authoring preflight, lock/write flow, and product-shell refusal wording
crates/compiler/src/author/mod.rs                      -> author-surface reexports that should keep the outward compiler API stable
crates/compiler/src/lib.rs                             -> compiler public surface that should remain stable for CLI callers
crates/compiler/tests/author.rs                        -> compiler-level regression coverage for project-context parsing, rendering, validation, and authoring flows
crates/cli/src/main.rs                                 -> CLI-guided interview owner that should remain behavior-stable and stay outside the compiler deterministic core
crates/cli/tests/author_cli.rs                         -> CLI-level project-context behavior and refusal-surface regression coverage
```

If live implementation proves `author/project_context/{core,shell}.rs` is cleaner than sibling files, that layout is acceptable only if the same ownership split and public API stability hold.

## Code Style

Prefer a thin public adapter over one file that owns deterministic modeling, time resolution, repo mutation, and operator-facing refusal copy all at once.

```rust
let normalized = project_context_core::normalized_structured_input(input);
project_context_core::validate_structured_input(&normalized)?;

let now_utc = project_context_shell::resolve_render_timestamp()?;
let markdown = project_context_core::render_markdown(&normalized, &now_utc)?;
project_context_core::validate_markdown(&markdown)?;

project_context_shell::write_canonical_project_context(repo_root, &markdown)
```

Conventions:

- deterministic project-context core must not read environment variables, derive the current clock time, acquire authoring locks, or mutate the repo
- shell adapters may own `.handbook` root inspection, baseline eligibility checks, timestamp resolution, lock handling, canonical writes, and CLI-oriented refusal wording
- keep the public `handbook_compiler` exports stable through `author/mod.rs`
- keep CLI-guided project-context interviewing in `crates/cli/src/main.rs`; the compiler core should consume typed inputs, not prompt the operator
- prefer typed handoffs between core and shell instead of ad hoc helper sharing

## Testing Strategy

- Framework: existing Rust integration tests in `crates/compiler/tests/` and CLI regression tests in `crates/cli/tests/`
- Primary test levels:
  - compiler author tests for deterministic parse/render/validate behavior
  - compiler author tests for preflight and canonical-write authoring behavior
  - CLI author tests to prove the outward `handbook author project-context` contract stays stable for guided and `--from-inputs` flows
- Coverage focus:
  - deterministic project-context entrypoints remain free of env/clock/filesystem mutation concerns
  - timestamp-dependent rendering remains deterministic once shell-owned metadata is provided
  - canonical authoring preflight and write flows still refuse safely without partial writes
  - refusal summaries and next-safe-action wording remain stable at the CLI surface unless the spec changes first
  - the CLI-guided interview continues to own answer collection instead of pushing prompt behavior into compiler internals
- Coverage expectation:
  - Packet 3.2.1 proves the deterministic project-context core boundary without changing public behavior
  - Packet 3.2.2 proves authoring/refusal shell behavior and CLI regressions still pass after the split

## Slice Scope

In scope:

- extract deterministic project-context structured-input types, normalization, validation, markdown rendering, and markdown validation into one engine-shaped owner
- reduce `project_context.rs` to a thin orchestrator and public facade
- isolate render timestamp resolution, canonical authoring preflight, lock acquisition, canonical write flow, and refusal wording away from the deterministic core
- preserve the current public compiler and CLI project-context contract
- preserve CLI ownership of guided project-context interviewing while making the compiler boundary cleaner

Out of scope:

- changing `environment_inventory`
- redesigning CLI prompts, interview wording, flags, success rendering, or help text
- changing project-context markdown semantics, required headings, or authoring defaults
- adding new public authoring configuration or runtime knobs
- moving code into new crates before Phase 4
- widening into setup, doctor, refusal, or general shell-copy cleanup outside project-context authoring

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- Slice 3.1 authority set:
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-3-slice-1-charter-core-split-tasks.md`
- Live project-context surfaces:
  - `crates/compiler/src/author/project_context.rs`
  - `crates/compiler/src/author/mod.rs`
  - `crates/compiler/src/lib.rs`
  - `crates/cli/src/main.rs`
- Live regression evidence:
  - `crates/compiler/tests/author.rs`
  - `crates/cli/tests/author_cli.rs`

## Current Mixed Responsibilities To Untangle

| Responsibility | Current live owner | Slice 3.2 requirement |
| --- | --- | --- |
| structured-input model, normalization, factuality checks, and render-safety checks | `ProjectContextStructuredInput` and helpers such as `validate_project_context_structured_input`, `normalized_project_context_structured_input`, `require_factual_*`, and `collect_render_safety_issues` in `author/project_context.rs` | move behind one deterministic core boundary with no env/clock/filesystem or product-copy ownership |
| compiler-owned markdown rendering and markdown validation | `render_project_context_markdown`, `validate_project_context_markdown`, and heading/placeholder validation helpers in `author/project_context.rs` | keep as reusable engine-safe behavior and make shell adapters consume it |
| render timestamp resolution | `resolve_project_context_now_utc` and `AUTHOR_PROJECT_CONTEXT_NOW_UTC_ENV_VAR` in `author/project_context.rs` | isolate as shell/runtime behavior so deterministic rendering depends on explicit metadata instead of ambient time/env state |
| canonical authoring preflight and write flow | `preflight_author_project_context`, `author_project_context`, `author_project_context_from_input`, lock handling, and canonical write validation in `author/project_context.rs` | leave as thin orchestration around core + shell helpers instead of mixed owner logic |
| CLI-oriented refusal summaries and next-safe-action wording | refusal constructors and mutation/preflight paths in `author/project_context.rs` that reference `handbook author project-context` | keep shell-owned so future engine migration does not inherit product-specific recovery copy as core behavior |
| guided answer collection | `collect_guided_project_context_input` and related interview helpers in `crates/cli/src/main.rs` | remain CLI-owned and out of the compiler deterministic core; Slice 3.2 must not pull interview logic into compiler internals |

## Boundaries

- Always:
  - keep deterministic project-context code free of env reads, clock resolution, authoring locks, and repo mutation
  - preserve current public project-context behavior first; this slice is an internal seam cleanup
  - keep CLI-guided interviewing in CLI
  - prove both compiler-level and CLI-level project-context regressions after the split
  - keep required-heading and placeholder/boilerplate validation semantics stable unless a spec update says otherwise
- Ask first:
  - renaming public `handbook_compiler` project-context functions or types
  - introducing new CLI flags, prompts, or runtime configuration
  - changing the rendered metadata lines or timestamp format intentionally
  - extracting shared authoring shell helpers that also touch environment-inventory, setup, or doctor
  - changing the canonical project-context write contract or lock-file contract
- Never:
  - move Phase 4 crate-boundary work into this slice
  - let the deterministic core depend directly on env vars, wall-clock time, lock acquisition, or canonical writes
  - move CLI-guided interview behavior into compiler internals
  - widen into environment-inventory, setup, doctor, or CLI thinning
  - change project-context document semantics as a side effect of the split

## Success Criteria

- A deterministic project-context core boundary exists for structured-input types, normalization, validation, markdown rendering, and markdown validation.
- `author_project_context`, `author_project_context_from_input`, and preflight entrypoints become thin orchestrators over deterministic core and shell helpers.
- Render timestamp resolution, canonical authoring preflight, lock/write flow, and CLI-oriented refusal wording are isolated outside the deterministic core.
- The CLI-guided interview remains CLI-owned and behavior-stable.
- `cargo test -p handbook-compiler --test author` and `cargo test -p handbook-cli --test author_cli` continue to pass against the split architecture.
- No environment-inventory, setup, doctor, or Phase 4 crate-move work leaked in.

## Open Questions

- Should the deterministic core accept a fully typed render-metadata struct, or is an explicit timestamp string enough for Slice 3.2 as long as ambient time/env access leaves the core?
- Is a dedicated `project_context_shell.rs` the smallest durable shell owner, or does live implementation truth prove that a thinner split inside `project_context.rs` is sufficient as long as the same core-vs-shell contract holds?
