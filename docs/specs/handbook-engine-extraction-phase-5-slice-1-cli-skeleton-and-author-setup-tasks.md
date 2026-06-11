# Tasks: Handbook Engine Extraction Phase 5 Slice 1 (Slice 5.1) - CLI Skeleton And Author Setup Extraction

Plan reference: [handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-plan.md](./handbook-engine-extraction-phase-5-slice-1-cli-skeleton-and-author-setup-plan.md)

## Prerequisite: the first CLI shell split must stay bounded

Slice 5.1 exists to start thinning `handbook-cli`, not to finish the whole CLI rewrite. Keep the cut limited to module skeleton work plus the `setup` and `author` command families.

- Later families (`pipeline`, `inspect`, `doctor`) stay in `main.rs` for this slice.

## Packet 5.1.1: CLI Module Skeleton And Shared Command Helper Wiring

- [ ] Task: Establish the first CLI module skeleton under `crates/cli/src/`
  - Acceptance: `main.rs` declares and uses dedicated helper modules for the first extracted shell families, while top-level clap registration and unsupported-for-now families remain easy to find.
  - Verify: `cargo test -p handbook-cli && cargo test -p handbook-cli --test help_drift_guard`
  - Files: `crates/cli/src/main.rs`, new `crates/cli/src/*.rs` helper modules for the chosen skeleton

- [ ] Task: Extract only the shared shell helpers that setup and author both need immediately
  - Acceptance: any shared shell utilities moved out of `main.rs` are narrow, clearly named, and directly justified by the setup/author split rather than by speculative future reuse.
  - Verify: `cargo test -p handbook-cli && rg -n '^fn (setup|author|execute_author_|render_setup_)' crates/cli/src/main.rs`
  - Files: `crates/cli/src/main.rs`, new shared helper module(s) under `crates/cli/src/`

## Packet 5.1.2: Author And Setup Command-Family Extraction

- [ ] Task: Move the setup command family into its dedicated CLI module
  - Acceptance: `setup`, `setup init`, and `setup refresh` routing plus setup success/refusal rendering no longer live inline in `main.rs`, while public behavior and wording remain unchanged.
  - Verify: `cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
  - Files: `crates/cli/src/main.rs`, chosen setup module file(s) under `crates/cli/src/`

- [ ] Task: Move the author command family into its dedicated CLI module
  - Acceptance: `author charter`, `author project-context`, and `author environment-inventory` shell orchestration no longer live inline in `main.rs`, while guided and deterministic author behavior remain unchanged.
  - Verify: `cargo test -p handbook-cli --test author_cli && cargo test -p handbook-cli`
  - Files: `crates/cli/src/main.rs`, chosen author module file(s) under `crates/cli/src/`

- [ ] Task: Leave later CLI families untouched while completing the first shell split
  - Acceptance: `pipeline`, `inspect`, and `doctor` remain outside the Slice 5.1 extraction scope, and the packet does not introduce unrelated command-surface or renderer redesign work.
  - Verify: `rg -n '^fn (pipeline|inspect|doctor)' crates/cli/src/main.rs && cargo test -p handbook-cli --test cli_surface`
  - Files: verification against `crates/cli/src/main.rs`, plus any packet-local module files only

## Final Slice Verification

- [ ] Task: Run the focused Slice 5.1 verification wall after both packets land
  - Acceptance: the CLI has a real module skeleton, `setup` and `author` moved out of `main.rs`, later command families stayed scoped out, and the focused behavior/help regressions are green.
  - Verify: `cargo fmt --all -- --check && cargo clippy -p handbook-cli --all-targets -- -D warnings && cargo test -p handbook-cli --test author_cli && cargo test -p handbook-cli --test cli_surface && cargo test -p handbook-cli --test help_drift_guard`
  - Files: verification only

## Human Review Gate

Stop after the Slice 5.1 spec, plan, and tasks are reviewed and approved. Do not start implementation packets until the human approves this authority set.
