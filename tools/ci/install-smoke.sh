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

assert_path_absent() {
  local path="$1"

  [[ ! -e "$path" ]] || {
    echo "unexpected path present: $path"
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
if data["skill_name"] != "system-charter-intake":
    raise SystemExit(f"unexpected manifest skill_name: {data['skill_name']}")
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

assert_repo_root_install_sources_absent() {
  assert_path_absent "$ROOT_DIR/SKILL.md"
  assert_path_absent "$ROOT_DIR/SKILL.md.tmpl"
  assert_path_absent "$ROOT_DIR/agents"
  assert_path_absent "$ROOT_DIR/charter-intake"
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
charter-intake/SKILL.md
charter-intake/SKILL.md.tmpl
resources/authoring/charter_authoring_method.md
resources/charter/CHARTER_INPUTS.yaml.tmpl
resources/charter/charter_inputs_directive.md
runtime-manifest.json
EOF
)"
}

assert_installed_runtime_contract() {
  test -x "$SYSTEM_HOME/bin/system"
  test -d "$SYSTEM_HOME/resources/authoring"
  test -d "$SYSTEM_HOME/resources/charter"
  assert_path_absent "$SYSTEM_HOME/bin/system-charter-intake"
  assert_path_absent "$SYSTEM_HOME/share"
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
assert_repo_root_install_sources_absent
bash tools/codex/install.sh
assert_installed_home_file_set
assert_installed_runtime_contract
assert_discovery_links_to_system_home
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
assert_installed_runtime_contract
assert_discovery_links_to_system_home

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
assert_installed_runtime_contract

echo "OK"
