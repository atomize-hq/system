# Plan: Handbook Engine Extraction Phase 5 Slice 1 (Slice 5.1) - CLI Skeleton And Author Setup Extraction

## Objective

Start Phase 5 by turning `handbook-cli` into a more honest product shell: introduce the first real CLI module skeleton, then move the `setup` and `author` command families out of `crates/cli/src/main.rs` without changing supported behavior.

Spec reference: [handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-spec.md](./handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-spec.md)

## Major Modules

1. CLI shell skeleton and dispatch boundary
   - `crates/cli/src/main.rs`
   - new `crates/cli/src/*` helper modules
   - establishes where top-level clap registration stops and extracted command-family modules begin

2. Setup command-family module
   - current `setup` handlers and rendering helpers from `crates/cli/src/main.rs`
   - becomes the dedicated home for `setup` request routing, success rendering, and refusal rendering

3. Author command-family module
   - current `author` handlers, guided/deterministic entrypoints, and author-family rendering helpers from `crates/cli/src/main.rs`
   - becomes the dedicated home for `author charter`, `author project-context`, and `author environment-inventory` shell orchestration

4. Minimal shared shell helpers
   - only the shared utilities that `setup` and `author` actually need after extraction
   - keeps reuse explicit without turning this slice into a generic helper sweep

5. Verification surfaces
   - `crates/cli/tests/author_cli.rs`
   - `crates/cli/tests/cli_surface.rs`
   - `crates/cli/tests/help_drift_guard.rs`
   - prove shell behavior and help posture remain stable

## Dependencies And Order

### Prerequisite: freeze the target shell boundary before moving families

Why first:

- Slice 5.1 should thin the CLI, not reopen the Phase 4 ownership model
- a clear boundary is needed to prevent Packet 5.1.1 from degenerating into an open-ended helper shuffle
- deciding the module skeleton first keeps Packet 5.1.2 focused on moving bounded command families into the new shape

Output:

- one agreed module layout for the first CLI split
- one agreed rule for shared helpers: only extract what `setup` and `author` both need now
- one agreed fence that pipeline / inspect / doctor stay in `main.rs` for this slice

### Packet 5.1.1: CLI Module Skeleton And Shared Command Helper Wiring

Why first:

- `setup` and `author` extraction should land into a prepared shell structure rather than inventing the layout mid-refactor
- moving a few targeted shared helpers first reduces the chance of duplicate logic or accidental circular dependencies once the families move
- keeping this packet behavior-preserving makes it the safest place to prove the new CLI skeleton compiles, dispatches, and keeps help stable

Output:

- `main.rs` declares and uses the new module skeleton
- minimal shared helpers required by the first extraction live outside `main.rs`
- the CLI still behaves identically for the still-inline and newly wired surfaces

### Packet 5.1.2: Author And Setup Command-Family Extraction

Why second:

- once the shell skeleton exists, the two command families can move cleanly into their dedicated modules
- `setup` and `author` are the smallest next cut because they sit early in the public command surface and have strong existing regression coverage
- delaying this packet until after the skeleton exists avoids mixing structural and behavioral changes into one hard-to-review diff

Output:

- `setup` family orchestration is extracted from `main.rs`
- `author` family orchestration is extracted from `main.rs`
- `main.rs` retains top-level registration and dispatch while delegating the first families to their new modules
- author/setup behavior and help output remain stable

## Risks And Mitigations

### Risk: the first shell split widens into a full Phase 5 CLI rewrite

Mitigation:

- keep Packet 5.1.1 limited to skeleton and minimal shared helper wiring
- treat pipeline / inspect / doctor as explicitly frozen for this slice
- reject unrelated cleanup that does not directly enable setup/author extraction

### Risk: clap declarations and handler moves create a confusing module layout

Mitigation:

- keep top-level command registration obvious from `main.rs`
- use dedicated family modules instead of a large generic utility file
- keep naming aligned with the command families they serve

### Risk: author extraction drags broad prompting/rendering work forward from Slice 5.3

Mitigation:

- move only the prompt/rendering code that the author family already owns operationally
- do not generalize prompting or wording helpers for other command families yet
- treat any cross-family rendering cleanup as future work unless required to compile

### Risk: help text or command routing drifts during refactor

Mitigation:

- run `help_drift_guard` throughout the slice, not just at the end
- keep snapshot changes out unless an intentional, approved wording change is required
- use `author_cli` and `cli_surface` as the behavior-preservation wall for extracted families

## Parallel Vs Sequential

Sequential:

- choose the module skeleton before moving command families
- land minimal shared helpers before extracting full `setup` and `author` implementations
- move `setup` and `author` before considering any later CLI families
- run the final slice verifier after both packets land

Parallel opportunities after Packet 5.1.1 lands:

- setup and author extraction subtasks can be worked independently once the skeleton and any shared helper seams are stable
- help snapshot verification and package-level tests can run in parallel while the final packet is being cleaned up

## Verification Checkpoints

### Checkpoint 1: the shell skeleton compiles and preserves help posture

```bash
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-cli
```

### Checkpoint 2: setup and author extraction preserve family behavior

```bash
cargo test -p handbook-cli --test author_cli
cargo test -p handbook-cli --test cli_surface
```

### Checkpoint 3: `main.rs` is materially thinner and later families remain untouched

```bash
wc -l crates/cli/src/main.rs
rg -n '^fn (setup|author|execute_author_|render_setup_)' crates/cli/src/main.rs
```

### Final checkpoint

```bash
cargo fmt --all -- --check
cargo clippy -p handbook-cli --all-targets -- -D warnings
cargo test -p handbook-cli --test cli_surface
cargo test -p handbook-cli --test help_drift_guard
cargo test -p handbook-cli --test author_cli
```

## Exit Conditions

The slice is ready for human review when:

- `main.rs` has a real module skeleton for the CLI shell
- `setup` and `author` orchestration no longer live inline in `main.rs`
- only the shared helpers required by those two families have been extracted
- help posture and command behavior remain stable
- pipeline / inspect / doctor extraction has not leaked into the slice
- the focused CLI verification wall passes
