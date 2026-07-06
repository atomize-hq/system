# Spec: Handbook Engine Extraction Phase 5 Slice 1 (Slice 5.1) - CLI Skeleton And Author Setup Extraction

## Assumptions

1. Phase 4 Slice 5 is complete enough in live code that direct caller rewires and compiler narrowing already landed, so Phase 5 can focus on CLI shell shape rather than crate-ownership cleanup.
2. `crates/cli/src/main.rs` is still the dominant CLI integration bucket after Phase 4: it is currently 4,238 lines long and still mixes clap declarations, command dispatch, setup orchestration, author orchestration, guided prompting, refusal rendering, and multiple other command families.
3. Slice 5.1 is the first CLI-thinning landing, not the whole Phase 5 closeout. It should introduce the shell/module skeleton and move only the `setup` and `author` command families first.
4. The approved ownership model from Phase 4 remains in force during this slice: `handbook-cli` owns parsing and product-shell wiring, `handbook-engine` owns structured authoring core, `handbook-flow` owns resolver/result/budget runtime logic, `handbook-pipeline` owns pipeline runtime logic, and `handbook-compiler` remains only for explicitly retained compatibility/support seams.
5. `setup` and `author` extraction may move refusal/success rendering and small shared shell helpers into CLI-local modules when that is the smallest durable cut, but Slice 5.1 must not widen into pipeline/inspect/doctor extraction or a broad renderer redesign.
6. Help text, subcommand ordering, and the supported command surface defined by `C-02` must remain behaviorally unchanged through this slice even if clap structs or helper functions move to new module files.
7. The slice verifier stays focused on CLI behavior preservation: `cargo test -p handbook-cli --test cli_surface` and `cargo test -p handbook-cli --test help_drift_guard` remain the slice-map wall, with `author_cli` and the package test suite used as focused packet guards.

## Objective

Introduce a real `handbook-cli` module skeleton and move the `setup` and `author` command-family orchestration out of `crates/cli/src/main.rs` so the CLI begins acting like a product shell instead of a single-file integration bucket.

The maintainer needs this slice so Phase 4's crate extraction does not leave the product shell trapped in a 4k-line `main.rs`. Success means:

- `crates/cli/src/main.rs` becomes a thinner entrypoint that keeps top-level command registration and dispatch but stops directly housing the full `setup` and `author` command-family implementation bodies
- dedicated CLI-local helper modules exist for the first extracted command families and any minimal shared shell helpers they require
- `setup` and `author` behavior, help text, refusal wording, and success rendering remain stable across the extraction
- Slice 5.1 stops before pipeline/inspect/doctor extraction, broader prompt/rendering isolation, or exit-code closeout work
- `cargo test -p handbook-cli --test cli_surface` and `cargo test -p handbook-cli --test help_drift_guard` pass, with author-focused regression coverage still green

## Tech Stack

- Rust 2021 workspace
- Crates in play:
  - `handbook-cli`
  - `handbook-compiler`
  - `handbook-engine`
  - `handbook-flow`
  - `handbook-pipeline`
- CLI framework:
  - `clap = "4"` with derive macros
- Authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-{spec,plan,tasks}.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
```

Focused author/setup regression guards:

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli
```

CLI shell inventory and drift checks:

```bash
wc -l crates/cli/src/main.rs
rg -n '^fn (author|setup|execute_author_|render_setup_|print_subcommand_help)' crates/cli/src/main.rs
rg -n 'handbook setup|handbook author|setup refresh|author project-context' crates/cli/tests/help_drift_guard.rs crates/cli/tests/snapshots
```

Formatting and final package hygiene:

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-cli --all-targets -- -D warnings
```

## Project Structure

```text
crates/cli/src/main.rs                               -> top-level CLI entrypoint that should shrink to registration and dispatch glue
crates/cli/src/*.rs                                  -> new helper modules for the extracted command families and minimal shared shell helpers
crates/cli/Cargo.toml                                -> CLI package manifest; should stay stable unless module extraction forces small package-level support changes
crates/cli/tests/author_cli.rs                       -> regression coverage for guided and deterministic author flows
crates/cli/tests/cli_surface.rs                      -> CLI integration surface coverage that must stay green after module extraction
crates/cli/tests/help_drift_guard.rs                 -> snapshot/help guard that must remain truthful after the shell split
crates/cli/tests/snapshots/*                         -> expected help output that should remain stable unless an intentional doc-approved wording change occurs
crates/compiler/src/lib.rs                           -> retained compatibility/support seam that setup/author CLI modules may still call into where Phase 4 left support logic
crates/engine/src/lib.rs                             -> engine-owned structured authoring surface consumed by compiler or CLI-local helpers as already established
crates/flow/src/lib.rs                               -> unchanged in this slice; explicitly out of the first command-family extraction
crates/pipeline/src/lib.rs                           -> unchanged in this slice; pipeline extraction is Slice 5.2 work
```

## Code Style

Prefer a thin `main.rs` that wires command families into dedicated modules instead of defining each family inline.

```rust
mod author;
mod setup;
mod shell_shared;

impl Command {
    fn run(self) -> ExitCode {
        match self {
            Command::Setup(args) => setup::run(args),
            Command::Author(args) => author::run(args),
            Command::Pipeline(args) => pipeline(args),
            Command::Generate(args) => generate(args),
            Command::Inspect(args) => inspect(args),
            Command::Doctor(args) => doctor(args),
        }
    }
}
```

Conventions:

- keep `main.rs` focused on top-level clap surface, command registration, and high-level dispatch
- move command-family-local helpers into the owning CLI module instead of keeping them as free-floating `main.rs` utilities
- extract only the minimal shared helpers needed by both `setup` and `author` in this slice
- preserve existing wording and output structure while moving code; module extraction is not a license to redesign refusal or success text
- do not use Slice 5.1 to introduce a new abstraction layer that hides owner-crate boundaries or command behavior

## Testing Strategy

- Framework: Cargo package tests plus existing integration and help snapshot tests
- Primary test levels:
  - `author_cli` for guided/deterministic author flows and setup-adjacent baseline expectations
  - `cli_surface` for command-surface stability after extraction
  - `help_drift_guard` for verb ordering, help copy, and snapshot preservation
  - package-level `cargo test -p handbook-cli` for unit coverage inside the extracted modules
- Coverage focus:
  - module extraction does not change supported `setup` or `author` command behavior
  - help snapshots remain stable
  - top-level dispatch still routes to the correct command family
  - shared shell helpers introduced in Packet 5.1.1 stay narrow and do not pull pipeline/doctor work into the slice
- Coverage expectation:
  - Packet 5.1.1 proves the new module skeleton compiles and preserves help/output behavior
  - Packet 5.1.2 proves `setup` and `author` command families live outside `main.rs` while the author/setup regression suite stays green

## Slice Scope

In scope:

- introduce the first real CLI helper-module skeleton under `crates/cli/src/`
- move `setup` command-family orchestration out of `main.rs`
- move `author` command-family orchestration out of `main.rs`
- move the minimal setup/author-adjacent shared helpers required to support that split
- keep command behavior, help surface, and approved ownership boundaries stable while extracting the shell

Out of scope:

- pipeline, inspect, or doctor command-family extraction
- broad prompting-helper isolation beyond what setup/author extraction strictly needs
- product wording, rendering, or exit-code closeout work that belongs to Slice 5.3
- new runtime features, new crate-boundary decisions, or compiler-posture reversals
- Substrate ownership/integration planning after extraction

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md`
- `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md`
- `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md`
- `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- Live CLI package truth:
  - `crates/cli/Cargo.toml`
  - `crates/cli/src/main.rs`
  - `crates/cli/tests/author_cli.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
  - `crates/cli/tests/snapshots/**`

## Current CLI Burden To Reduce

| Surface | Current live posture | Slice 5.1 requirement |
| --- | --- | --- |
| top-level shell file | `crates/cli/src/main.rs` is 4,238 lines and still houses clap declarations, dispatch, setup handlers, author handlers, guided prompting, rendering helpers, and other command families together | reduce `main.rs` to a thinner entrypoint with extracted command-family modules |
| setup family | `setup`, `render_setup_success`, `render_setup_refusal`, and related helpers still live in `main.rs` | move setup orchestration into a dedicated CLI module without changing setup routing or wording |
| author family | `author`, `execute_author_*`, guided-input plumbing, and related refusal/success rendering still live in `main.rs` | move author orchestration into dedicated CLI modules without changing guided or deterministic behavior |
| shared shell helpers | setup/author reuse shell-local helpers such as dispatch, repo-root discovery, or family-local rendering utilities directly from `main.rs` | extract only the minimal shared helpers needed to support the first module split |
| unextracted families | pipeline / inspect / doctor remain in `main.rs` today | leave them alone in Slice 5.1 so the first CLI-thinning cut stays bounded |

## Boundaries

- Always:
  - keep top-level command behavior and help surface stable while moving code into modules
  - extract only the `setup` and `author` families plus the smallest necessary shared shell helpers
  - preserve the Phase 4 ownership model instead of reopening crate-boundary debates
  - use the existing CLI tests and help snapshots as the primary truth source for behavior preservation
- Ask first:
  - changing supported verb names, help ordering, or user-visible command wording
  - pulling pipeline, inspect, or doctor extraction into the same slice
  - moving broad prompting/rendering/exit-code helpers that are not required for the setup/author split
  - changing `C-02` crate-ownership claims rather than implementing within them
- Never:
  - treat module extraction as permission to redesign the CLI surface
  - widen Slice 5.1 into the full Phase 5 shell closeout
  - move reusable runtime logic back into `handbook-cli`
  - leave `main.rs` thinner only by pushing large unrelated helpers into a generic dumping-ground module

## Success Criteria

- `crates/cli/src/main.rs` no longer directly houses the full `setup` and `author` command-family implementation bodies.
- Dedicated CLI helper modules exist for the extracted `setup` and `author` families, plus only the shared shell helpers actually required by those families.
- `handbook setup`, `handbook setup init`, `handbook setup refresh`, `handbook author charter`, `handbook author project-context`, and `handbook author environment-inventory` preserve their current behavior and help posture.
- `cargo test -p handbook-cli --test author_cli`, `cargo test -p handbook-cli --test cli_surface`, and `cargo test -p handbook-cli --test help_drift_guard` all pass.
- Slice 5.1 stops before pipeline/doctor extraction and before the broader prompting/rendering/help closeout work reserved for later Phase 5 slices.

## Open Questions

- What is the smallest durable module layout for the first shell split: flat `crates/cli/src/{author,setup,...}.rs` files, or a small nested `commands/**` structure that still keeps Slice 5.1 packet-sized?
- Which setup/author-adjacent helpers are truly shared enough to extract in Packet 5.1.1, and which ones should stay family-local until later slices prove a wider reuse need?
