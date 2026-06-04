# Spec: Candidate 2 Trusted Pipeline Session Deepening

## Assumptions

1. The change stays inside the existing Rust workspace and does not introduce a new crate.
2. The primary goal is a deeper library seam in `handbook-compiler`, not a CLI redesign.
3. Reduced-v1 behavior for `pipeline.foundation_inputs` remains authoritative; this work must preserve existing compile, capture, and handoff semantics.
4. The eventual `substrate` consumer should be able to call one compiler-owned trust seam without reconstructing route-basis freshness or refusal behavior itself.

## Objective

Deepen the route-basis trust module inside `handbook-compiler` so compile, capture, and handoff stop reimplementing the same route-state loading, canonical route-basis rebuild, freshness checks, revision checks, and stale-trust refusal mapping.

The user is the maintainer of this repo today and the future library consumer inside `/Users/spensermcconnell/__Active_Code/atomize-hq/substrate`.

Success looks like one compiler-owned trust seam that:

- validates selected pipeline/stage trust preconditions once
- returns one typed, reusable trust result to compile/capture/handoff flows
- preserves reduced-v1 refusal posture and recovery directions
- makes library consumption shallower for downstream Rust callers

## Tech Stack

- Rust 2021 workspace
- `handbook-compiler` library crate
- `handbook-cli` binary crate
- Current trust inputs and contracts:
  - `docs/contracts/pipeline-route-and-state-core.md`
  - `docs/contracts/stage-compile-boundary-and-route-freshness.md`
  - `docs/contracts/C-13-pipeline-handoff-and-downstream-trust.md`

## Commands

Build:

```bash
cargo check --workspace
```

Test targeted compiler surfaces:

```bash
cargo test -p handbook-compiler --test pipeline_compile
cargo test -p handbook-compiler --test pipeline_capture
cargo test -p handbook-compiler --test pipeline_handoff
cargo test -p handbook-compiler --test pipeline_state_store
```

Test CLI fallout:

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test pipeline_handoff_refusals
cargo test -p handbook-cli --test help_drift_guard
```

Repo verification wall:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
cargo check --workspace
```

## Project Structure

```text
crates/compiler/src/route_state.rs      -> Route-state load/persist logic and canonical route-basis rebuild helpers
crates/compiler/src/pipeline_compile.rs -> Compile flow and compile-side trust refusals
crates/compiler/src/pipeline_capture.rs -> Capture preview/apply flow and capture-side trust refusals
crates/compiler/src/pipeline_handoff.rs -> Handoff validation/emission trust checks
crates/compiler/src/lib.rs              -> Library exports for compiler consumers
crates/compiler/tests/                  -> Compiler integration coverage
crates/cli/tests/                       -> CLI regression and help-surface coverage
docs/contracts/                         -> Existing reduced-v1 product contracts
docs/specs/                             -> New spec/plan/tasks documents for this deepening work
```

## Code Style

Prefer typed trust outcomes and one owner for refusal mapping over repeated inline validation ladders.

```rust
match trusted_result {
    Ok(trusted) => use_trusted_state(trusted),
    Err(refusal) => return Err(refusal),
}
```

Conventions:

- keep compiler-owned trust logic in the library crate, not in CLI adapters
- prefer typed data over ad hoc tuples or repeated local variables
- preserve existing refusal classifications and next-safe-action posture unless the spec explicitly widens them
- keep machine-local paths normalized before they cross compile-facing seams

## Testing Strategy

- Framework: existing Rust integration tests under `crates/compiler/tests/` and `crates/cli/tests/`
- Primary test levels:
  - compiler integration tests for trust-session correctness
  - CLI regression tests only where public refusal wording or proof surfaces must remain stable
- Focus areas:
  - missing, stale, malformed, and inactive route-basis cases
  - revision-conflict behavior for capture apply
  - stage-10 provenance validation during handoff
  - `${repo_root}` normalization across compile-facing surfaces
- Coverage expectation:
  - every migrated caller path must retain explicit tests for success and trust refusal behavior

## Boundaries

- Always:
  - preserve reduced-v1 compile/capture/handoff behavior
  - keep trust logic compiler-owned and library-first
  - add or update targeted tests for every migrated caller
  - keep contract language and implementation aligned
- Ask first:
  - widening beyond `pipeline.foundation_inputs`
  - changing public CLI flags or help posture
  - introducing new dependencies or a new crate
  - changing published contract semantics instead of only deepening implementation
- Never:
  - duplicate route-basis trust logic in a new caller after introducing the deep seam
  - bypass canonical route-basis rebuild/freshness checks with best-effort behavior
  - leak machine-local `repo_root` paths into compile-facing trust output
  - silently change refusal classifications or recovery actions

## Success Criteria

- `pipeline_compile`, `pipeline_capture`, and `pipeline_handoff` consume one compiler-owned trust seam instead of each rebuilding the same trust ladder.
- The new trust seam lives in `handbook-compiler` and is suitable for future direct consumption by another Rust workspace such as `substrate`.
- Existing refusal classifications for missing, stale, malformed, inactive, and revision-conflict trust failures remain stable unless a reviewed contract update explicitly changes them.
- Compile-facing trust output continues to normalize `run.repo_root` to `${repo_root}`.
- Targeted compiler and CLI regression tests pass without broadening the public command surface.

## Open Questions

- Should the new deep seam remain internal to `handbook-compiler` at first, or become a small public export immediately for downstream crate consumers?
- Should handoff validation consume the exact same trust result shape as compile/capture, or should it consume a narrower adapter over the same implementation?
