# Spec: Handbook Engine Extraction Phase 4 Slice 5 (Set 3 / Slice 4.5 Refresh) - Direct Caller Rewires + Compiler Narrowing Closeout

## Assumptions

1. This is a refresh/closeout of an existing Phase 4 seam, not a brand-new architecture slice.
2. `handbook-engine`, `handbook-pipeline`, and `handbook-flow` already own most extracted behavior in live code; the remaining gap is mainly caller/dependency honesty and explicit boundary truth.
3. `handbook-compiler` is already materially narrower than the original umbrella crate posture. This slice should keep it intentionally narrow, not treat retirement as the default goal.
4. Remaining CLI imports of `handbook_compiler::*` must be classified carefully: some are stale convenience imports that should move to owner crates, while others may still be legitimate CLI-facing compatibility/support seams.
5. Phase 1 layout/storage parameterization, Phase 2 orchestration-target parameterization, and Phase 5 CLI shell closeout remain separate seams unless a tiny supporting adjustment is strictly required to close the Slice 4.5 ownership boundary honestly.
6. The final closeout verdict must be stricter than “tests pass”: this slice is only complete when direct callers, manifests, and repo-facing docs all tell the same ownership story.

## Objective

Finish the remaining Phase 4 closeout gap so the crate split is operationally real in direct callers and dependency posture, while `handbook-compiler` remains a deliberately narrow compatibility/support seam instead of drifting back into the default integration center.

The maintainer needs this refresh because the workspace has already landed the major split, but live caller truth is mixed:

- `crates/cli` already depends directly on `handbook-engine`, `handbook-pipeline`, and `handbook-flow`
- `crates/flow/src/lib.rs` no longer forwards through `handbook-compiler`
- CLI tests already use owner crates directly in many places
- `crates/compiler/src/lib.rs` already presents itself as a narrow support seam
- but several CLI source modules still import `handbook_compiler::*`, and the refreshed slice must distinguish legitimate retained support seams from stale owner-crate indirection

Success means:

- no direct caller still uses `handbook-compiler` merely as a convenience facade for engine-owned, pipeline-owned, or flow-owned logic
- any remaining compiler-root imports are explicitly justified as part of the retained narrow compatibility/support seam
- `handbook-cli` dependency posture remains honest about real owners plus the retained compiler seam
- repo-facing docs and guards clearly state what still belongs to `handbook-compiler` and what no longer does

## Tech Stack

- Rust 2021 workspace
- workspace crates:
  - `handbook-cli`
  - `handbook-compiler`
  - `handbook-engine`
  - `handbook-flow`
  - `handbook-pipeline`
- authority docs:
  - `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
  - `docs/specs/handbook-engine-extraction-slice-map.md`
  - `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
  - existing Slice 4.5 triplet
  - existing Slice 4.2 / 4.3 / 4.4 triplets
  - `README.md`
  - `docs/README.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`

## Commands

Residual compiler-caller inventory:

```bash
rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli/src crates/cli/tests crates/flow crates/compiler/src/lib.rs
```

Dependency-posture checks:

```bash
cargo tree -p handbook-cli -e normal
cargo tree -p handbook-compiler -e normal
```

Focused caller and support-seam verification:

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-engine
cargo test -p handbook-pipeline
cargo test -p handbook-flow
cargo test -p handbook-compiler --test author
cargo test -p handbook-compiler --test doctor
cargo test -p handbook-compiler --test setup
```

Final verification wall:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Project Structure

```text
Cargo.toml                                                     -> Workspace membership and high-level dependency truth
crates/cli/Cargo.toml                                          -> Direct dependency posture for real owner crates plus the retained compiler seam
crates/cli/src/main.rs                                         -> Top-level CLI dispatch and a small remaining set of compiler-root type references
crates/cli/src/author.rs                                       -> CLI-facing author orchestration and refusal handling that still consumes compiler-root author support today
crates/cli/src/setup.rs                                        -> CLI-facing setup orchestration still consuming compiler setup support
crates/cli/src/doctor.rs                                       -> CLI-facing doctor entrypoint still consuming compiler doctor support
crates/cli/src/doctor_rendering.rs                             -> CLI-facing rendering over compiler doctor/report support types
crates/cli/src/rendering.rs                                    -> CLI-facing render/refusal/blocker adapters that still consume compiler support types
crates/cli/tests/author_cli.rs                                 -> Authoring CLI regression proof, already using engine-owned surfaces in several places
crates/cli/tests/cli_surface.rs                                -> CLI surface regression proof for setup/pipeline/handoff behavior
crates/cli/tests/help_drift_guard.rs                           -> Help and ownership truth guard
crates/flow/src/lib.rs                                         -> Already owner-rooted flow public surface; should stay free of compiler forwarding
crates/compiler/Cargo.toml                                     -> Narrow compatibility/support seam dependency posture
crates/compiler/src/lib.rs                                     -> Reviewed compiler-root export surface that must stay intentionally narrow
crates/compiler/tests/{author,doctor,setup,rendering_surface}.rs -> Proof that retained compiler-owned support seams remain coherent
README.md, docs/README.md                                      -> Repo-facing ownership summary
docs/contracts/C-02-rust-workspace-and-cli-command-surface.md  -> Contract for crate boundaries and retained compiler seam truth
docs/specs/                                                    -> Slice 4.5 authority documents
```

## Code Style

Prefer direct imports from the crate that owns extracted logic, and reserve `handbook-compiler` imports for the reviewed compatibility/support seam only.

```rust
use handbook_engine::parse_charter_structured_input_yaml;
use handbook_flow::resolve;
use handbook_pipeline::compile_pipeline_stage_with_runtime;
use handbook_compiler::{author_charter, AuthorCharterRefusal};
```

Conventions:

- owner-crate imports should be the default when behavior already belongs to `handbook-engine`, `handbook-pipeline`, or `handbook-flow`
- `handbook-compiler` imports should survive only when the retained compiler seam still owns the CLI-facing compatibility/support behavior in live code
- do not reintroduce umbrella re-export modules or broad facade habits just because the compiler crate still exists
- keep CLI behavior stable while correcting import paths, dependency posture, and ownership claims
- do not turn this slice into Phase 5 CLI module decomposition or wording cleanup

## Testing Strategy

- Framework: Cargo workspace checks plus existing crate and CLI integration tests
- Primary test levels:
  - CLI regression tests for behavior and help-surface truth
  - owner-crate tests for engine/pipeline/flow stability after rewires
  - compiler tests for the retained narrow support seam only
- Coverage focus:
  - stale compiler-facade imports are removed or explicitly justified
  - `handbook-cli` keeps its current behavior while imports and manifests become more honest
  - `handbook-flow` stays free of compiler forwarding
  - `handbook-compiler` remains narrow and reviewable rather than drifting back into umbrella ownership
  - repo-facing docs and help guards describe the same boundary the code implements
- Coverage expectation:
  - each implementation packet proves its narrowed surface with targeted tests
  - the slice is not complete until the full workspace wall passes

## Slice Scope

In scope:

- refresh the live inventory of remaining `handbook_compiler::*` callers across CLI source and adjacent tests
- classify each remaining caller as either stale extracted-logic indirection or legitimate retained compatibility/support usage
- move any stale extracted-logic callers directly to `handbook-engine`, `handbook-pipeline`, or `handbook-flow`
- keep `handbook-cli` manifests and dependency posture aligned with the actual owner graph
- keep `handbook-compiler` limited to the reviewed narrow support seam
- update the minimum docs and help/ownership guards required to keep the boundary truthful

Out of scope:

- broad Phase 5 CLI decomposition, prompt-flow redesign, help-text rewrites, or product-shell cleanup
- reopening Phase 1 layout/storage parameterization as the main job of this slice
- reopening Phase 2 orchestration-target parameterization as the main job of this slice
- retiring `handbook-compiler` unless live code proves that is the smallest coherent closeout and the user explicitly approves the scope change
- introducing new engine/pipeline/flow runtime features
- widening into new consumer/platform architecture work

## Authority Inputs

- `HANDBOOK_ENGINE_EXTRACTION_PLAN.md`
- `docs/specs/handbook-engine-extraction-slice-map.md`
- `docs/specs/handbook-engine-extraction-closeout-four-set-map.md`
- existing Slice 4.5 authority set:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-5-caller-rewire-and-compiler-narrowing-tasks.md`
- earlier Phase 4 authority sets:
  - `docs/specs/handbook-engine-extraction-phase-4-slice-2-engine-migration-*.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-3-pipeline-migration-*.md`
  - `docs/specs/handbook-engine-extraction-phase-4-slice-4-flow-migration-*.md`
- live code/doc truth:
  - `Cargo.toml`
  - `crates/cli/Cargo.toml`
  - `crates/cli/src/{main,author,setup,doctor,doctor_rendering,rendering}.rs`
  - `crates/cli/tests/{author_cli,cli_surface,help_drift_guard}.rs`
  - `crates/flow/src/lib.rs`
  - `crates/compiler/Cargo.toml`
  - `crates/compiler/src/lib.rs`
  - `README.md`
  - `docs/README.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`

## Current Repo-Truth Gaps This Closeout Must Finish

| Surface | Current live posture | Slice 4.5 refresh requirement |
| --- | --- | --- |
| `crates/cli/src/main.rs` | already uses `handbook-pipeline` for supported help constants, but still references a small set of `handbook_compiler` author/support types | rewire any stale extracted-logic imports to owner crates and explicitly justify any remaining compiler-root support references |
| `crates/cli/src/author.rs` | still fronts compiler-root author orchestration and refusal/result types | decide which compiler-root author usages are legitimate retained CLI-facing support seams and move any leftover engine-owned helpers directly to `handbook-engine` |
| `crates/cli/src/setup.rs`, `doctor.rs`, `doctor_rendering.rs`, `rendering.rs` | still consume compiler-root setup/doctor/rendering support surfaces | keep these only if they are part of the retained narrow compiler seam; do not let them justify broader umbrella imports elsewhere |
| `crates/cli/tests/*.rs` | tests already use owner crates directly in many places, but the refresh must ensure no stale facade pattern is reintroduced | preserve owner-rooted tests and tighten any remaining compiler-facing test assumptions to the retained support seam only |
| `crates/flow/src/lib.rs` | already exposes flow-owned surfaces directly without compiler forwarding | keep that posture frozen and treat regressions here as Slice 4.5 failures |
| `crates/compiler/src/lib.rs` + `crates/compiler/Cargo.toml` | compiler already presents itself as a narrow compatibility/support seam over engine/flow/pipeline dependencies | keep the compiler export surface small, reviewable, and explicitly non-umbrella |
| repo-facing docs and guard rails | README/docs/contract text already says compiler is narrow, but the refreshed slice docs must match live caller truth exactly | align docs and help guards with the actual retained compiler seam and the actual direct-owner caller graph |

## Boundaries

- Always:
  - keep this slice focused on caller rewires, dependency honesty, and compiler narrowing closeout
  - distinguish “retained narrow support seam” from “umbrella implementation center”
  - preserve already-landed direct-owner surfaces such as `handbook-flow`
  - refresh the existing Slice 4.5 authority rather than inventing a different seam
- Ask first:
  - any compiler-retirement attempt
  - any new public API expansion created only to simplify one caller rewire
  - any change that meaningfully widens Phase 5 CLI-shell work
- Never:
  - widen into CLI shell redesign or broad copy/wording cleanup
  - reopen Phase 2 target-parameterization as the main job of this slice
  - treat passing tests alone as proof that the ownership boundary is now honest
  - allow `handbook-compiler` to regain broad facade ownership for engine-, pipeline-, or flow-owned logic

## Success Criteria

- Every remaining `handbook_compiler::*` caller in the CLI-adjacent surfaces is either removed in favor of the real owner crate or explicitly justified as part of the retained narrow support seam.
- `handbook-cli` dependencies and import paths reflect the real owner graph: `handbook-engine`, `handbook-pipeline`, and `handbook-flow` for extracted logic, plus `handbook-compiler` only for retained support seams.
- `crates/flow/src/lib.rs` and other already-owner-rooted surfaces stay free of compiler forwarding.
- `crates/compiler/src/lib.rs` remains intentionally narrow and does not drift back into umbrella re-export ownership.
- Repo-facing docs and help/guard coverage accurately describe the resulting boundary.
- `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo test --workspace` pass after the closeout.

## Open Questions

- Which current `crates/cli/src/author.rs` compiler-root imports are still legitimately part of the retained CLI-facing author support seam, and which should now move directly to `handbook-engine`?
- Is the smallest honest closeout for `crates/cli/src/rendering.rs` to keep compiler-root rendering/refusal adapters in place, or has live code reduced that seam enough that some of those types should now be consumed from owner crates directly?
