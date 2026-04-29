#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
EVIDENCE_DIR="$ROOT_DIR/.implemented/m9.5-codex-skill-packaging"
LOG_PATH="$EVIDENCE_DIR/install-smoke.log"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

mkdir -p "$EVIDENCE_DIR"
exec > >(tee "$LOG_PATH") 2>&1

DISCOVERY_INSTALL_ROOT="$HOME/.codex/skills/system-charter-intake"
RUNTIME_INSTALL_ROOT="$HOME/.codex/skills/system"
RUNTIME_WRAPPER="$RUNTIME_INSTALL_ROOT/bin/system-charter-intake"
FIXTURE_INPUTS="$ROOT_DIR/tools/fixtures/charter_inputs/runtime_smoke_valid.yaml"

manifest_backup=""
tmp_root=""

cleanup() {
  if [[ -n "$manifest_backup" && -f "$manifest_backup" ]]; then
    cp "$manifest_backup" "$RUNTIME_INSTALL_ROOT/runtime-manifest.json"
    rm -f "$manifest_backup"
  fi
  if [[ -n "$tmp_root" && -d "$tmp_root" ]]; then
    rm -rf "$tmp_root"
  fi
}
trap cleanup EXIT

assert_runtime_root_exact_file_set() {
  local root="$1"
  local actual
  actual="$(
    find "$root" -maxdepth 4 -type f -print \
      | sed "s#^$root/##" \
      | sort
  )"
  local expected
  expected="$(cat <<'EOF'
SKILL.md
bin/system-charter-intake
runtime-manifest.json
share/authoring/charter_authoring_method.md
share/charter/CHARTER_INPUTS.yaml.tmpl
share/charter/charter_inputs_directive.md
EOF
)"
  [[ "$actual" == "$expected" ]] || {
    echo "unexpected runtime root file set under $root"
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

assert_discovery_root() {
  [[ -f "$DISCOVERY_INSTALL_ROOT/SKILL.md" ]] || {
    echo "missing installed discovery SKILL.md"
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
bash tools/codex/install.sh
assert_discovery_root
assert_runtime_root_exact_file_set "$RUNTIME_INSTALL_ROOT"
assert_manifest_fields "$RUNTIME_INSTALL_ROOT/runtime-manifest.json"

echo "==> reinstall smoke"
before_listing="$(find "$RUNTIME_INSTALL_ROOT" -maxdepth 4 -type f -print | sort)"
bash tools/codex/install.sh
after_listing="$(find "$RUNTIME_INSTALL_ROOT" -maxdepth 4 -type f -print | sort)"
[[ "$before_listing" == "$after_listing" ]] || {
  echo "reinstall changed installed file set"
  exit 1
}

echo "==> stale runtime refusal smoke"
tmp_root="$(mktemp -d)"
repo_root="$tmp_root/stale-runtime-repo"
mkdir -p "$repo_root"
git -C "$repo_root" init -q
manifest_backup="$(mktemp)"
cp "$RUNTIME_INSTALL_ROOT/runtime-manifest.json" "$manifest_backup"
tamper_manifest_version "$RUNTIME_INSTALL_ROOT/runtime-manifest.json"

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
cp "$manifest_backup" "$RUNTIME_INSTALL_ROOT/runtime-manifest.json"
rm -f "$manifest_backup"
manifest_backup=""

echo "==> dev-setup symlink smoke"
bash tools/codex/dev-setup.sh
test -L "$DISCOVERY_INSTALL_ROOT"
test -L "$RUNTIME_INSTALL_ROOT"

echo "==> install-mode crossover smoke"
bash tools/codex/install.sh
test ! -L "$DISCOVERY_INSTALL_ROOT"
test ! -L "$RUNTIME_INSTALL_ROOT"
test -d "$DISCOVERY_INSTALL_ROOT"
test -d "$RUNTIME_INSTALL_ROOT"
assert_discovery_root
assert_runtime_root_exact_file_set "$RUNTIME_INSTALL_ROOT"

echo "OK"
