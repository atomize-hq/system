# Supported Commands (Reduced v1, Rust-first)

The supported reduced-v1 entrypoint is the Rust CLI (crate `system-cli`, binary `system`).

For the authoritative command surface and help ordering, see [`C-02`](contracts/C-02-rust-workspace-and-cli-command-surface.md).

## Local invocation (development)

From repo root:

```bash
cargo run -p system-cli -- --help
cargo run -p system-cli -- setup
cargo run -p system-cli -- generate
cargo run -p system-cli -- inspect
cargo run -p system-cli -- doctor
```

## Current command meanings

- `setup` is the reserved setup-first entrypoint for the reduced-v1 trust flow.
- `generate` produces the packet surface and refuses compactly when canonical `.system/` inputs are missing or unsupported.
- `inspect` is the proof surface for packet composition and decision evidence.
- `doctor` is the recovery surface for blockers and safe next actions.

## What to expect right now

- `setup` is still a placeholder, but it is part of the supported command surface and help ordering.
- If `.system/` is missing, `generate`, `inspect`, and `doctor` refuse or block with a deterministic next safe action.
- Once `.system/` canonical artifacts exist, planning packet resolution becomes available.
- Execution packets are only supported as fixture-backed demos, live execution is refused.
