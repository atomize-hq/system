#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

SMOKE_MODEL_ENV_VAR="SYSTEM_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL"
RUNTIME_MODEL_ENV_VAR="SYSTEM_AUTHOR_CHARTER_CODEX_MODEL"

is_relevant_change() {
  local path="$1"
  case "$path" in
    crates/compiler/src/author.rs|\
    crates/compiler/tests/author.rs|\
    crates/cli/src/main.rs|\
    crates/cli/tests/author_cli.rs|\
    .github/workflows/ci.yml|\
    tools/ci/author-live-smoke.sh)
      return 0
      ;;
    core/library/authoring/*)
      return 0
      ;;
    core/library/charter/*)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

should_run_live_smoke() {
  local base_sha="${PR_BASE_SHA:-${PUSH_BASE_SHA:-}}"
  local head_sha="${HEAD_SHA:-HEAD}"

  if [[ -z "$base_sha" || "$base_sha" == "0000000000000000000000000000000000000000" ]]; then
    if git rev-parse --verify HEAD^ >/dev/null 2>&1; then
      base_sha="$(git rev-parse HEAD^)"
    else
      echo "unable to determine diff base; treating live smoke as required"
      return 0
    fi
  fi

  while IFS= read -r path; do
    [[ -n "$path" ]] || continue
    if is_relevant_change "$path"; then
      echo "author live smoke required by change: $path"
      return 0
    fi
  done < <(git diff --name-only "$base_sha" "$head_sha")

  return 1
}

if ! should_run_live_smoke; then
  echo "no authoring transport changes detected; skipping live Codex smoke"
  exit 0
fi

if [[ -z "${CODEX_API_KEY:-}" ]]; then
  if [[ "${IS_FORK_PR:-false}" == "true" ]]; then
    echo "CODEX_API_KEY is unavailable on fork PRs; skipping live Codex smoke"
    exit 0
  fi

  echo "CODEX_API_KEY is required for the author live smoke job" >&2
  exit 1
fi

echo "==> install codex cli"
npm install -g @openai/codex

echo "==> codex version"
codex --version

selected_model="${SYSTEM_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL:-gpt-5.4-mini}"
export SYSTEM_AUTHOR_CHARTER_CODEX_MODEL="$selected_model"

echo "==> live author smoke model"
echo "${RUNTIME_MODEL_ENV_VAR}=${SYSTEM_AUTHOR_CHARTER_CODEX_MODEL}"
echo "Local fast example: ${SMOKE_MODEL_ENV_VAR}=gpt-5.3-codex-spark"
echo "CI-like example: ${SMOKE_MODEL_ENV_VAR}=gpt-5.4-mini"

echo "==> run live author smoke"
SYSTEM_RUN_LIVE_AUTHOR_CHARTER_SMOKE=1 \
  cargo test -p system-cli --test author_cli \
  structured_inputs_author_charter_succeeds_with_live_codex_transport \
  -- --exact
