#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
EVIDENCE_DIR="$ROOT_DIR/.implemented/m10-orchestration"
LOG_PATH="$EVIDENCE_DIR/codex-skill-live-smoke.log"
STATE_ROOT="${XDG_STATE_HOME:-$HOME/.local/state}/system/intake/runs"
SYSTEM_HOME="$HOME/system"
CODEX_ROOT_SKILL="$HOME/.codex/skills/system"
CODEX_DISCOVERY_SKILL="$HOME/.codex/skills/system-charter-intake"
RUNTIME_WRAPPER="$SYSTEM_HOME/bin/system-charter-intake"
FIXTURE_INPUTS="$ROOT_DIR/tools/fixtures/charter_inputs/runtime_smoke_valid.yaml"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

mkdir -p "$EVIDENCE_DIR" "$STATE_ROOT"
exec > >(tee "$LOG_PATH") 2>&1

tmp_root=""
cleanup() {
  if [[ -n "$tmp_root" && -d "$tmp_root" ]]; then
    rm -rf "$tmp_root"
  fi
}
trap cleanup EXIT

latest_run_dir() {
  find "$STATE_ROOT" -mindepth 1 -maxdepth 1 -type d | sort | tail -n 1
}

extract_run_dir() {
  local output="$1"
  printf '%s\n' "$output" | awk -F': ' '/^RUN ARTIFACTS: / {print $2}' | tail -n 1
}

assert_session_fields() {
  local session_path="$1"
  local expected_runtime_root="$2"
  python3 - "$session_path" "$expected_runtime_root" <<'PY'
import json
import os
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)
required = {
    "started_at_utc",
    "repo_root",
    "runtime_root",
    "system_release_version",
    "runtime_manifest_version",
}
missing = sorted(required.difference(data))
if missing:
    raise SystemExit(f"missing session fields: {missing}")
actual_runtime_root = os.path.realpath(data["runtime_root"])
expected_runtime_root = os.path.realpath(sys.argv[2])
if actual_runtime_root != expected_runtime_root:
    raise SystemExit(
        f"unexpected runtime_root: {data['runtime_root']} != {sys.argv[2]}"
    )
PY
}

assert_run_dir_file_set() {
  local run_dir="$1"
  local expected="$2"
  local actual
  actual="$(
    find "$run_dir" -maxdepth 1 -type f -print \
      | sed "s#^$run_dir/##" \
      | sort
  )"
  [[ "$actual" == "$expected" ]] || {
    echo "unexpected run artifact file set under $run_dir"
    echo "actual:"
    printf '%s\n' "$actual"
    echo "expected:"
    printf '%s\n' "$expected"
    exit 1
  }
}

assert_no_misplaced_run_evidence() {
  local root="$1"
  local misplaced
  misplaced="$(
    find "$root" -type f \( \
      -name 'session.json' -o \
      -name 'doctor.before.json' -o \
      -name 'doctor.after_setup.json' -o \
      -name 'doctor.after_write.json' -o \
      -name 'author.stdout.txt' -o \
      -name 'author.stderr.txt' -o \
      -name 'validate.stdout.txt' -o \
      -name 'validate.stderr.txt' \
    \) -print
  )"
  [[ -z "$misplaced" ]] || {
    echo "unexpected run evidence under $root"
    printf '%s\n' "$misplaced"
    exit 1
  }
}

common_success_files() {
  cat <<'EOF'
author.exit
author.stderr.txt
author.stdout.txt
charter_inputs.yaml
doctor.after_setup.json
doctor.after_write.json
doctor.before.json
session.json
validate.exit
validate.stderr.txt
validate.stdout.txt
EOF
}

existing_truth_refusal_files() {
  cat <<'EOF'
author.exit
author.stderr.txt
author.stdout.txt
charter_inputs.yaml
doctor.before.json
session.json
validate.exit
validate.stderr.txt
validate.stdout.txt
EOF
}

echo "==> install current system binary"
cd "$ROOT_DIR"
cargo install --locked --force --path crates/cli

echo "==> install generated codex skill assets"
bash tools/codex/install.sh
[[ -x "$RUNTIME_WRAPPER" ]] || {
  echo "missing installed runtime wrapper: $RUNTIME_WRAPPER"
  exit 1
}

tmp_root="$(mktemp -d)"

echo "==> happy path smoke against installed runtime"
happy_repo="$tmp_root/happy-repo"
mkdir -p "$happy_repo"
git -C "$happy_repo" init -q
happy_output="$(
  cd "$happy_repo"
  "$RUNTIME_WRAPPER" --inputs "$FIXTURE_INPUTS"
)"
happy_run_dir="$(extract_run_dir "$happy_output")"
[[ -n "$happy_run_dir" && -d "$happy_run_dir" ]] || {
  echo "failed to capture happy-path run dir"
  echo "$happy_output"
  exit 1
}
assert_run_dir_file_set "$happy_run_dir" "$(common_success_files)"
assert_session_fields "$happy_run_dir/session.json" "$SYSTEM_HOME"
test -f "$happy_repo/.system/charter/CHARTER.md"

echo "==> existing charter refusal smoke"
existing_repo="$tmp_root/existing-charter-repo"
mkdir -p "$existing_repo"
git -C "$existing_repo" init -q
(
  cd "$existing_repo"
  system setup >/dev/null
  system author charter --from-inputs "$FIXTURE_INPUTS" >/dev/null
)
before_existing="$(latest_run_dir || true)"
set +e
existing_output="$(
  cd "$existing_repo"
  "$RUNTIME_WRAPPER" --inputs "$FIXTURE_INPUTS" 2>&1
)"
existing_status=$?
set -e
if [[ $existing_status -eq 0 ]]; then
  echo "expected refusal when canonical charter already exists"
  exit 1
fi
[[ "$existing_output" == *"canonical charter already exists"* ]] || {
  echo "expected existing charter refusal"
  echo "$existing_output"
  exit 1
}
existing_run_dir="$(latest_run_dir)"
[[ -n "$existing_run_dir" && "$existing_run_dir" != "$before_existing" ]] || {
  echo "expected a new run dir for existing-truth refusal"
  exit 1
}
assert_run_dir_file_set "$existing_run_dir" "$(existing_truth_refusal_files)"
assert_session_fields "$existing_run_dir/session.json" "$SYSTEM_HOME"

echo "==> dev-setup discovery override keeps installed runtime root"
bash tools/codex/dev-setup.sh
[[ "$(readlink "$CODEX_ROOT_SKILL")" == "$ROOT_DIR/.agents/skills/system" ]] || {
  echo "unexpected dev root discovery link target"
  readlink "$CODEX_ROOT_SKILL" || true
  exit 1
}
[[ "$(readlink "$CODEX_DISCOVERY_SKILL")" == "$ROOT_DIR/.agents/skills/system-charter-intake" ]] || {
  echo "unexpected dev leaf discovery link target"
  readlink "$CODEX_DISCOVERY_SKILL" || true
  exit 1
}
override_repo="$tmp_root/dev-override-runtime-root"
mkdir -p "$override_repo"
git -C "$override_repo" init -q
override_output="$(
  cd "$override_repo"
  "$RUNTIME_WRAPPER" --inputs "$FIXTURE_INPUTS"
)"
override_run_dir="$(extract_run_dir "$override_output")"
[[ -n "$override_run_dir" && -d "$override_run_dir" ]] || {
  echo "failed to capture dev-override run dir"
  echo "$override_output"
  exit 1
}
assert_session_fields "$override_run_dir/session.json" "$SYSTEM_HOME"

echo "==> restore installed discovery topology"
bash tools/codex/install.sh
[[ "$(readlink "$CODEX_ROOT_SKILL")" == "$SYSTEM_HOME/.agents/skills/system" ]] || {
  echo "unexpected restored root discovery link target"
  readlink "$CODEX_ROOT_SKILL" || true
  exit 1
}
[[ "$(readlink "$CODEX_DISCOVERY_SKILL")" == "$SYSTEM_HOME/.agents/skills/system-charter-intake" ]] || {
  echo "unexpected restored leaf discovery link target"
  readlink "$CODEX_DISCOVERY_SKILL" || true
  exit 1
}

echo "==> outside-git-repo refusal smoke"
outside_dir="$tmp_root/not-a-repo"
mkdir -p "$outside_dir"
set +e
outside_output="$(
  cd "$outside_dir"
  "$RUNTIME_WRAPPER" --inputs "$FIXTURE_INPUTS" 2>&1
)"
outside_status=$?
set -e
if [[ $outside_status -eq 0 ]]; then
  echo "expected refusal outside a git repo"
  exit 1
fi
[[ "$outside_output" == *"run this skill from inside a real git repository"* ]] || {
  echo "expected outside-repo refusal"
  echo "$outside_output"
  exit 1
}

assert_no_misplaced_run_evidence "$SYSTEM_HOME"
assert_no_misplaced_run_evidence "$HOME/.codex/skills"

echo "OK"
