# Packet 4.5.1 Residual Caller Inventory And Boundary Freeze

This note freezes the live `handbook_compiler::*` caller inventory for Set 3 / Slice 4.5 after the closeout-refresh reframing.

## Scope checked

- `crates/cli/src/**`
- `crates/cli/tests/**`
- `crates/flow/**`
- `crates/compiler/src/lib.rs`

Evidence commands:

```bash
rg -n 'handbook_compiler::|use handbook_compiler|extern crate handbook_compiler' crates/cli/src crates/cli/tests crates/flow crates/compiler/src/lib.rs
cargo tree -p handbook-cli -e normal
```

## Frozen clean surfaces

These surfaces already satisfy the Packet 4.5.1 boundary and should stay untouched by Packet 4.5.2 except for regression protection:

- `crates/flow/src/lib.rs` has no compiler-forwarding callers.
- `crates/cli/tests/author_cli.rs` has no compiler-root callers.
- `crates/cli/tests/cli_surface.rs` has no compiler-root callers.
- `crates/compiler/src/lib.rs` is not itself a residual caller site; it is the reviewed narrow export seam.

## Residual caller classification

### Legitimate retained narrow support-seam usage

These files still consume compiler-root APIs because the live code keeps setup, doctor, author-shell, and rendering-support seams rooted at `handbook-compiler` for now.

| File | Residual compiler-root usage | Classification | Why it remains allowed in Packet 4.5.1 |
| --- | --- | --- | --- |
| `crates/cli/src/setup.rs` | `SetupRequest`, `SetupMode`, `run_setup`, `SetupOutcome`, `SetupDisposition`, `SetupActionLabel`, `SetupRefusal`, `SetupRefusalKind` | legitimate retained narrow support seam | setup is still exposed as a compiler-owned CLI-facing support surface; this packet only records that boundary |
| `crates/cli/src/doctor.rs` | `doctor`, `DoctorBaselineStatus` | legitimate retained narrow support seam | doctor entry and baseline-status reporting still route through the compiler support seam |
| `crates/cli/src/doctor_rendering.rs` | `DoctorReport`, `render_next_safe_action_value`, `SubjectRef`, `DoctorBaselineStatus`, `SystemRootStatus`, `DoctorArtifactStatus` | legitimate retained narrow support seam | doctor rendering still formats compiler-owned support types for the CLI |
| `crates/cli/src/rendering.rs` | `RenderOutputModel`, `render_markdown`, `render_inspect`, `build_output_model`, `ResolverResult`, `DecisionLog`, `Refusal`, `Blocker`, `RefusalCategory`, `BlockerCategory`, `SubjectRef`, `NextSafeAction` | legitimate retained narrow support seam | the CLI still uses compiler-owned rendering/refusal adapters to present `handbook-flow` results |
| `crates/cli/src/author.rs` | `preflight_author_*`, `author_*`, `Author*Result`, `Author*Refusal`, `Author*RefusalKind` | legitimate retained narrow support seam | the CLI author shell still depends on compiler-owned preflight/write/refusal adapters even though some authoring core logic already lives in `handbook-engine` |
| `crates/cli/src/main.rs` (production code) | `DecisionLog`, `CompilerError`, `Refusal` in the compile-time surface check | legitimate retained narrow support seam | the top-level CLI still asserts compatibility with the retained compiler-root support types |
| `crates/cli/src/main.rs` (embedded tests) | `AuthorCharterResult`, `AuthorCharterRefusal`, `AuthorCharterRefusalKind` | legitimate retained narrow support seam | these tests inject compiler-owned author-shell result/refusal contracts that still match the live CLI boundary |

### Stale extracted-logic indirection queued for Packet 4.5.2

| File | Residual compiler-root usage | Classification | Packet 4.5.2 follow-on |
| --- | --- | --- | --- |
| `crates/cli/src/main.rs` (embedded tests) | `parse_charter_structured_input_yaml` | stale extracted-logic indirection | switch this helper call to the direct `handbook-engine` parser while preserving the current author-shell test contract unless Packet 4.5.2 also narrows that contract |

## Packet fences

- Packet 4.5.1 does **not** rewire the stale caller above; it only freezes the inventory.
- Packet 4.5.2 should target stale extracted-logic convenience uses first and leave the retained support seam explicit.
- Packet 4.5.3 should revisit whether any of the currently allowed compiler-root author/rendering/setup/doctor types can shrink further without widening into Phase 5 CLI shell work.
