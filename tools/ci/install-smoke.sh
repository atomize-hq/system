#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
EVIDENCE_DIR="$ROOT_DIR/.implemented/m10-orchestration"
LOG_PATH="$EVIDENCE_DIR/install-smoke.log"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

mkdir -p "$EVIDENCE_DIR"
exec > >(tee "$LOG_PATH") 2>&1

SYSTEM_HOME="$HOME/system"
INSTALLED_ROOT_SKILL="$SYSTEM_HOME/.agents/skills/system"
INSTALLED_DISCOVERY_SKILL="$SYSTEM_HOME/.agents/skills/system-charter-intake"
CODEX_ROOT_SKILL="$HOME/.codex/skills/system"
CODEX_DISCOVERY_SKILL="$HOME/.codex/skills/system-charter-intake"
RUNTIME_WRAPPER="$SYSTEM_HOME/bin/system-charter-intake"
FIXTURE_INPUTS="$ROOT_DIR/tools/fixtures/charter_inputs/runtime_smoke_valid.yaml"
RELEASE_VERSION="$(tr -d '[:space:]' <"$ROOT_DIR/VERSION")"

manifest_backup=""
tmp_root=""

cleanup() {
  if [[ -n "$manifest_backup" && -f "$manifest_backup" ]]; then
    cp "$manifest_backup" "$SYSTEM_HOME/runtime-manifest.json"
    rm -f "$manifest_backup"
  fi
  if [[ -n "$tmp_root" && -d "$tmp_root" ]]; then
    rm -rf "$tmp_root"
  fi
}
trap cleanup EXIT

assert_exact_file_set() {
  local root="$1"
  local expected="$2"
  local actual

  actual="$(
    find "$root" -type f -print \
      | sed "s#^$root/##" \
      | sort
  )"
  [[ "$actual" == "$expected" ]] || {
    echo "unexpected file set under $root"
    echo "actual:"
    printf '%s\n' "$actual"
    echo "expected:"
    printf '%s\n' "$expected"
    exit 1
  }
}

assert_manifest_fields() {
  local manifest_path="$1"
  python3 - "$manifest_path" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as handle:
    data = json.load(handle)

required = {
    "skill_name",
    "system_release_version",
    "manifest_version",
    "generated_at_utc",
}
missing = sorted(required.difference(data))
if missing:
    raise SystemExit(f"missing manifest fields: {missing}")
PY
}

assert_repo_projection_thin() {
  assert_exact_file_set "$ROOT_DIR/.agents/skills/system" "$(cat <<'EOF'
SKILL.md
agents/openai.yaml
EOF
)"
  assert_exact_file_set "$ROOT_DIR/.agents/skills/system-charter-intake" "$(cat <<'EOF'
SKILL.md
agents/openai.yaml
EOF
)"
}

assert_installed_home_file_set() {
  assert_exact_file_set "$SYSTEM_HOME" "$(cat <<'EOF'
.agents/skills/system-charter-intake/SKILL.md
.agents/skills/system-charter-intake/agents/openai.yaml
.agents/skills/system/SKILL.md
.agents/skills/system/agents/openai.yaml
SKILL.md
SKILL.md.tmpl
agents/openai.yaml
bin/system
bin/system-charter-intake
charter-intake/SKILL.md
charter-intake/SKILL.md.tmpl
runtime-manifest.json
share/authoring/charter_authoring_method.md
share/charter/CHARTER_INPUTS.yaml.tmpl
share/charter/charter_inputs_directive.md
EOF
)"
}

assert_discovery_links_to_system_home() {
  [[ "$(readlink "$CODEX_ROOT_SKILL")" == "$INSTALLED_ROOT_SKILL" ]] || {
    echo "unexpected root discovery link target"
    readlink "$CODEX_ROOT_SKILL" || true
    exit 1
  }
  [[ "$(readlink "$CODEX_DISCOVERY_SKILL")" == "$INSTALLED_DISCOVERY_SKILL" ]] || {
    echo "unexpected leaf discovery link target"
    readlink "$CODEX_DISCOVERY_SKILL" || true
    exit 1
  }
}

assert_dev_setup_links_to_repo() {
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
}

assert_system_binary_version_match() {
  local installed_version
  installed_version="$("$SYSTEM_HOME/bin/system" --version | awk '{print $NF}')"
  [[ "$installed_version" == "$RELEASE_VERSION" ]] || {
    echo "installed system version mismatch: repo=$RELEASE_VERSION installed=$installed_version"
    exit 1
  }
}

tamper_manifest_version() {
  local manifest_path="$1"
  python3 - "$manifest_path" <<'PY'
import json
import sys

path = sys.argv[1]
with open(path, encoding="utf-8") as handle:
    data = json.load(handle)
data["system_release_version"] = "0.0.0-stale"
with open(path, "w", encoding="utf-8") as handle:
    json.dump(data, handle, indent=2, sort_keys=True)
    handle.write("\n")
PY
}

echo "==> cargo install (crates/cli)"
cd "$ROOT_DIR"
cargo install --locked --force --path crates/cli

echo "==> help smoke"
system --help >/dev/null
system doctor --help >/dev/null
system author charter --help >/dev/null

echo "==> generator/install smoke"
bash tools/codex/generate.sh
assert_repo_projection_thin
bash tools/codex/install.sh
assert_installed_home_file_set
assert_discovery_links_to_system_home
assert_system_binary_version_match
assert_manifest_fields "$SYSTEM_HOME/runtime-manifest.json"

echo "==> reinstall smoke"
before_listing="$(find "$SYSTEM_HOME" -type f -print | sort)"
bash tools/codex/install.sh
after_listing="$(find "$SYSTEM_HOME" -type f -print | sort)"
[[ "$before_listing" == "$after_listing" ]] || {
  echo "reinstall changed installed file set"
  exit 1
}
assert_installed_home_file_set
assert_discovery_links_to_system_home

echo "==> stale runtime refusal smoke"
tmp_root="$(mktemp -d)"
repo_root="$tmp_root/stale-runtime-repo"
mkdir -p "$repo_root"
git -C "$repo_root" init -q
manifest_backup="$(mktemp)"
cp "$SYSTEM_HOME/runtime-manifest.json" "$manifest_backup"
tamper_manifest_version "$SYSTEM_HOME/runtime-manifest.json"

set +e
stale_output="$(
  cd "$repo_root"
  "$RUNTIME_WRAPPER" --inputs "$FIXTURE_INPUTS" 2>&1
)"
stale_status=$?
set -e
if [[ $stale_status -eq 0 ]]; then
  echo "expected nonzero exit for stale runtime refusal"
  exit 1
fi
[[ "$stale_output" == *"runtime version mismatch"* ]] || {
  echo "expected runtime version mismatch refusal"
  echo "$stale_output"
  exit 1
}
cp "$manifest_backup" "$SYSTEM_HOME/runtime-manifest.json"
rm -f "$manifest_backup"
manifest_backup=""

echo "==> dev-setup symlink smoke"
bash tools/codex/dev-setup.sh
test -L "$CODEX_ROOT_SKILL"
test -L "$CODEX_DISCOVERY_SKILL"
assert_dev_setup_links_to_repo

echo "==> install-mode crossover smoke"
bash tools/codex/install.sh
test -L "$CODEX_ROOT_SKILL"
test -L "$CODEX_DISCOVERY_SKILL"
assert_discovery_links_to_system_home
assert_installed_home_file_set

echo "OK"
