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

handbook_pipeline_released_proof version="0.1.2":
  bash tools/proof/handbook_pipeline_released_boundary.sh --version {{version}}

handbook_published_import_set3_guardrails version="0.1.2":
  bash tools/proof/handbook_pipeline_released_boundary.sh --version {{version}}
  rg -n 'Set 2 packaged proof' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'Set 3 released external proof' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'Set 3 downstream Substrate proof' docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-{spec,plan,tasks}.md
  rg -n 'Packet 4\.2 remains (explicitly classified only as|classified only as|only as|`engine \+ flow` only)' docs/specs/MAP.md docs/specs/handbook-published-import-decoupling-set-2-minimal-public-capability-boundary-*.md docs/specs/handbook-published-import-decoupling-set-3-published-consumer-proof-substrate-proof-guard-rails-*.md
