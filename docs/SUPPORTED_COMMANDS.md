# Supported Commands (Reduced v1, Rust-first)

The supported reduced-v1 entrypoint is the Rust CLI (crate `system-cli`, binary `system`).

For the authoritative command surface and help ordering, see [`C-02`](contracts/C-02-rust-workspace-and-cli-command-surface.md).

## Local invocation (development)

From repo root:

```bash
cargo run -p system-cli -- --help
cargo run -p system-cli -- doctor
cargo run -p system-cli -- inspect
cargo run -p system-cli -- generate
```

## What to expect

- If `.system/` is missing, commands refuse or block with a deterministic “next safe action”.
- Once `.system/` canonical artifacts exist, planning packet resolution becomes available.
- Execution packets are only supported as fixture-backed demos, live execution is refused.

