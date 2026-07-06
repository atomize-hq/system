# Spec: Handbook Engine Extraction Phase 5 Slice 2 (Slice 5.2) - CLI Runtime Command Shell

## Assumptions

1. Phase 5 Slice 1 is complete enough in live code that `setup` and `author` already route through dedicated CLI modules (`crates/cli/src/setup.rs`, `crates/cli/src/author.rs`, and `crates/cli/src/shell_shared.rs`), so Slice 5.2 can focus on the remaining runtime command families.
2. `crates/cli/src/main.rs` is still materially overloaded after Slice 5.1: it is currently 1,950 lines long and still directly houses the `pipeline`, `inspect`, and `doctor` command-family orchestration plus many family-local helper functions.
3. The slice-map authority remains binding: Slice 5.2 is specifically “Pipeline, Inspect, And Doctor Extraction,” not the final CLI closeout. Prompting helpers, broad wording/rendering isolation, and exit-code cleanup remain Slice 5.3 work unless a tiny local move is required to compile.
4. The approved ownership model from `C-02` remains in force during this slice: `handbook-cli` owns parsing, command dispatch, and help text; `handbook-pipeline` owns pipeline runtime logic; `handbook-flow` owns resolver/packet runtime logic; `handbook-compiler` remains the narrow compatibility/support seam for doctor/setup/rendering adapters that still need it.
5. Family-local render/refusal helpers may move with the extracted `pipeline`, `inspect`, or `doctor` modules when that is the smallest durable cut, but Slice 5.2 must not widen into a shell-wide rendering abstraction or a rewrite of help copy.
6. Help text, verb ordering, fixture semantics, ready/blocking behavior, JSON readiness behavior, and the supported command surface defined by `C-02`, `pipeline-route-and-state-core.md`, and `C-04` must remain behaviorally unchanged through this slice.

## Objective

Move the remaining runtime-facing CLI command families—`pipeline`, `inspect`, and `doctor`—out of `crates/cli/src/main.rs` so the CLI shell stops being the implementation bucket for runtime orchestration.

The maintainer needs this slice so Phase 5 can keep thinning the product shell without changing the supported reduced-v1 command story. Success means:

- `crates/cli/src/main.rs` becomes thinner again and stops directly housing the runtime command-family implementation bodies
- dedicated CLI-local modules exist for `pipeline`, `inspect`, and `doctor`, plus only the local helpers they actually need
- the supported command surface, refusal/ready semantics, help output, and fixture-backed behavior remain stable
- Slice 5.2 stops before the broader prompting/rendering/help/exit-code closeout reserved for Slice 5.3
- the existing CLI verification wall remains green, especially `cli_surface` and `help_drift_guard`

## Tech Stack

- Rust 2021 workspace
- Crates in play:
  - `handbook-cli`
  - `handbook-compiler`
  - `handbook-flow`
  - `handbook-pipeline`
- CLI framework:
  - `clap = "4"` with derive macros
- Authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-{spec,plan,tasks}.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `docs/contracts/pipeline-route-and-state-core.md`
  - `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`

## Commands

Primary slice verifier:

```bash
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
```

Focused runtime-command regression guards:

```bash
cargo test -p handbook-cli --test pipeline_handoff_refusals
cargo test -p handbook-cli --test manual_qa_fixture_checkout
cargo test -p handbook-cli
```

CLI shell inventory and drift checks:

```bash
wc -l crates/cli/src/main.rs
rg -n '^fn (pipeline|pipeline_list|pipeline_show|pipeline_resolve|pipeline_compile|pipeline_capture|pipeline_handoff|pipeline_state_set|inspect|doctor|render_doctor_json|parse_route_state_mutation|render_pipeline_)' crates/cli/src/main.rs
rg -n 'pipeline|inspect|doctor' crates/cli/tests/help_drift_guard.rs crates/cli/tests/snapshots crates/cli/tests/cli_surface.rs
```

Formatting and final package hygiene:

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-cli --all-targets -- -D warnings
```

## Project Structure

```text
crates/cli/src/main.rs                               -> top-level CLI entrypoint that should keep shrinking toward registration and dispatch glue
crates/cli/src/author.rs                             -> already-extracted author shell module from Slice 5.1; unchanged in this slice
crates/cli/src/setup.rs                              -> already-extracted setup shell module from Slice 5.1; unchanged in this slice
crates/cli/src/shell_shared.rs                       -> shared shell helpers that may remain the home only for truly cross-family helpers
crates/cli/src/*.rs                                  -> new runtime command-family modules for pipeline / inspect / doctor and any minimal local support files
crates/cli/tests/cli_surface.rs                      -> primary runtime command behavior wall
crates/cli/tests/help_drift_guard.rs                 -> help/output drift wall for command surface stability
crates/cli/tests/pipeline_handoff_refusals.rs        -> focused pipeline refusal coverage
crates/cli/tests/manual_qa_fixture_checkout.rs       -> focused inspect fixture/nested-checkout coverage
crates/cli/tests/snapshots/*                         -> expected help output that should remain stable unless an intentional doc-approved wording change occurs
crates/pipeline/src/lib.rs                           -> pipeline-owned runtime surface consumed by the extracted pipeline shell
crates/flow/src/lib.rs                               -> packet-resolution runtime surface consumed by the extracted inspect shell
crates/compiler/src/lib.rs                           -> retained compatibility/support seam still used by doctor and inspect rendering paths
```

## Code Style

Prefer a thin `main.rs` that delegates runtime command families into dedicated CLI modules instead of defining their handlers and render helpers inline.

```rust
mod doctor;
mod inspect;
mod pipeline;

impl Command {
    fn run(self) -> ExitCode {
        match self {
            Command::Pipeline(args) => pipeline::run(args),
            Command::Inspect(args) => inspect::run(args),
            Command::Doctor(args) => doctor::run(args),
            Command::Setup(args) => setup::run(args),
            Command::Author(args) => author::run(args),
            Command::Generate(args) => generate(args),
        }
    }
}
```

Conventions:

- keep `main.rs` focused on clap declarations, command registration, and top-level dispatch
- move family-local helpers with the family that owns them rather than leaving large runtime sections inline
- extract shared shell helpers only when they are genuinely cross-family now, not because a future slice might reuse them
- preserve existing wording and output structure while moving code; module extraction is not a license to redesign refusal text, JSON shape, or proof formatting
- keep runtime ownership obvious: CLI modules route into `handbook-pipeline`, `handbook-flow`, and `handbook-compiler` rather than re-implementing their semantics

## Testing Strategy

- Framework: Cargo package tests plus integration and help snapshot tests in `crates/cli/tests/**`
- Primary test levels:
  - `cli_surface` for the end-to-end command-surface behavior of `pipeline`, `inspect`, and `doctor`
  - `help_drift_guard` for verb ordering, help copy, and snapshot preservation
  - `pipeline_handoff_refusals` for focused pipeline refusal and proof-path coverage
  - `manual_qa_fixture_checkout` for fixture-backed inspect routing and nested checkout behavior
  - package-level `cargo test -p handbook-cli` for any new unit coverage inside extracted modules
- Coverage focus:
  - `pipeline` subcommands still preserve list/show/resolve/compile/capture/handoff/state behavior
  - `inspect` still preserves packet selection, execution-demo fixture requirements, and ready/blocking semantics
  - `doctor` still preserves text and JSON readiness surfaces plus exact baseline exit semantics
  - help snapshots remain stable and continue matching docs/contracts
  - shared helpers introduced during extraction stay narrow and do not pull Slice 5.3 work forward
- Coverage expectation:
  - Packet 5.2.1 proves `pipeline` orchestration lives outside `main.rs` with no command-surface drift
  - Packet 5.2.2 proves `inspect` and `doctor` live outside `main.rs` while runtime behavior and help posture stay green

## Slice Scope

In scope:

- introduce dedicated CLI-local modules for the `pipeline`, `inspect`, and `doctor` command families
- move pipeline-family orchestration out of `main.rs`
- move inspect-family orchestration out of `main.rs`
- move doctor-family orchestration out of `main.rs`
- move the minimal family-local or truly shared helper functions required to support those extractions
- preserve command behavior, fixture behavior, help posture, and contract-backed ownership boundaries during the shell split

Out of scope:

- broader prompting helper isolation
- shell-wide wording/rendering abstraction work
- help-surface redesign or exit-code cleanup reserved for Slice 5.3
- new runtime features, new command verbs, or crate-boundary reversals
- migration-readiness or Substrate ownership planning from Phase 6

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-spec.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-plan.md`
- `docs/specs/handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-tasks.md`
- `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- `docs/contracts/pipeline-route-and-state-core.md`
- `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
- Live CLI package truth:
  - `crates/cli/src/main.rs`
  - `crates/cli/src/author.rs`
  - `crates/cli/src/setup.rs`
  - `crates/cli/src/shell_shared.rs`
  - `crates/cli/tests/cli_surface.rs`
  - `crates/cli/tests/help_drift_guard.rs`
  - `crates/cli/tests/pipeline_handoff_refusals.rs`
  - `crates/cli/tests/manual_qa_fixture_checkout.rs`
  - `crates/cli/tests/snapshots/**`

## Current CLI Burden To Reduce

| Surface | Current live posture | Slice 5.2 requirement |
| --- | --- | --- |
| top-level shell file | `crates/cli/src/main.rs` is still 1,950 lines and directly houses the `pipeline`, `inspect`, and `doctor` implementations plus family-local helper functions | reduce `main.rs` to a thinner entrypoint by moving the remaining runtime command families into dedicated modules |
| pipeline family | `pipeline`, `pipeline_list`, `pipeline_show`, `pipeline_resolve`, `pipeline_compile`, `pipeline_capture`, `pipeline_handoff`, `pipeline_state_set`, and their render/refusal helpers still live in `main.rs` | move pipeline orchestration and its shell-local helpers into a dedicated CLI module without changing supported behavior |
| doctor family | `doctor`, `render_doctor_json`, and the baseline-status name helpers still live in `main.rs` | move doctor orchestration and doctor-local rendering helpers into a dedicated CLI module without changing text/JSON semantics |
| inspect family | `inspect` plus packet/fixture helper wiring such as packet parsing, fixture-set validation, demo fixture section injection, and output-model adaptation still live in `main.rs` | move inspect orchestration and the smallest required helper surface into dedicated CLI module(s) without changing proof behavior |
| later shell cleanup | prompting/rendering/help/exit-code closeout still remains as broad shell debt | leave that work for Slice 5.3 rather than overloading this extraction slice |

## Boundaries

- Always:
  - keep the public command surface and help posture stable while moving code into modules
  - extract only the runtime families and helper seams that Slice 5.2 requires now
  - preserve the `C-02`, pipeline-route/state, and `C-04` contracts rather than reopening them
  - use the existing CLI integration tests and help snapshots as the primary truth source for behavior preservation
- Ask first:
  - changing supported verb names, subcommand ordering, or user-visible help wording
  - broadening Slice 5.2 into prompt/render/help/exit-code cleanup intended for Slice 5.3
  - moving setup/author logic again unless a tiny import adjustment is strictly required to compile
  - changing contract-backed JSON readiness shape, proof ordering, or fixture semantics
- Never:
  - treat module extraction as permission to redesign the runtime command story
  - move pipeline, inspect, or doctor domain logic back into `handbook-cli`
  - silently widen Slice 5.2 into a whole-shell refactor
  - leave `main.rs` thinner only by relocating unrelated helpers into a generic dumping-ground module

## Success Criteria

- `crates/cli/src/main.rs` no longer directly houses the full `pipeline`, `inspect`, or `doctor` command-family implementation bodies.
- Dedicated CLI helper modules exist for the extracted runtime command families, plus only the minimal helper files they truly need.
- `handbook pipeline`, `handbook inspect`, and `handbook doctor` preserve their current command behavior, refusal/ready semantics, fixture handling, and help posture.
- `handbook doctor --json` preserves the existing machine-readable readiness contract and exit behavior.
- `cargo test -p handbook-cli --test cli_surface`, `cargo test -p handbook-cli --test help_drift_guard`, `cargo test -p handbook-cli --test pipeline_handoff_refusals`, and `cargo test -p handbook-cli --test manual_qa_fixture_checkout` all pass.
- Slice 5.2 stops before the broader prompting/rendering/help/exit-code closeout reserved for Slice 5.3.

## Open Questions

- What is the smallest durable runtime-shell layout after Slice 5.1: one module per family (`pipeline.rs`, `inspect.rs`, `doctor.rs`) or a slightly nested structure that still keeps Packet 5.2 packet-sized?
- Which inspect-adjacent helpers are truly inspect-owned versus good candidates for `shell_shared.rs`, and can that boundary stay narrow enough to avoid pre-spending Slice 5.3 cleanup?
