# Profile conventions: Rust (cargo)

## Style
- Format: rustfmt (`cargo fmt`)
- Lint: clippy (`cargo clippy ... -D warnings`)
- Prefer `Result<T, E>` for fallible operations; avoid panics in library code.

## Testing
- `cargo test` must pass.
- Prefer unit tests in-module; integration tests under `tests/` for public API surface.

## Security
- `cargo audit` when available.

Core prompts should reference command keys in `commands.yaml`, not embed raw cargo commands.
