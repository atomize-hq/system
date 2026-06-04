checks:
  cargo fmt --all
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  cargo test --workspace
  cargo check --workspace

checks-with-smoke:
  cargo fmt --all
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  HANDBOOK_RUN_LIVE_AUTHOR_CHARTER_SMOKE=1 HANDBOOK_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL=gpt-5.3-codex-spark HANDBOOK_AUTHOR_CHARTER_CODEX_MODEL=gpt-5.3-codex-spark cargo test --workspace
  cargo check --workspace
