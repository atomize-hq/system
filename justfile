checks:
  cargo fmt --all
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace
  cargo check --workspace

checks-with-smoke:
  cargo fmt --all
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  SYSTEM_RUN_LIVE_AUTHOR_CHARTER_SMOKE=1 SYSTEM_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL=gpt-5.3-codex-spark cargo test --workspace
  cargo check --workspace
