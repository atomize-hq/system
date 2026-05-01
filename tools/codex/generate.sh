#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SOURCE_ROOT="$ROOT_DIR/install/system-home"

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/system-codex-generate.XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

render_template() {
  local template_path="$1"
  local destination_path="$2"

  python3 - "$template_path" "$destination_path" <<'PY'
import pathlib
import sys

template_path = pathlib.Path(sys.argv[1])
destination_path = pathlib.Path(sys.argv[2])
text = template_path.read_text(encoding="utf-8")
destination_path.parent.mkdir(parents=True, exist_ok=True)
destination_path.write_text(text, encoding="utf-8")
PY
}

copy_file() {
  local source_path="$1"
  local destination_path="$2"

  mkdir -p "$(dirname "$destination_path")"
  cp "$source_path" "$destination_path"
}

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
    echo "unexpected generated file set under $root" >&2
    echo "actual:" >&2
    printf '%s\n' "$actual" >&2
    echo "expected:" >&2
    printf '%s\n' "$expected" >&2
    exit 1
  }
}

generated_root_skill_tmp="$tmp_dir/system-skill.md"
generated_leaf_skill_tmp="$tmp_dir/system-charter-intake-skill.md"
generated_root="$tmp_dir/.agents/skills"
root_projection_tmp="$generated_root/system"
leaf_projection_tmp="$generated_root/system-charter-intake"

render_template "$SOURCE_ROOT/SKILL.md.tmpl" "$generated_root_skill_tmp"
render_template "$SOURCE_ROOT/charter-intake/SKILL.md.tmpl" "$generated_leaf_skill_tmp"

copy_file "$generated_root_skill_tmp" "$root_projection_tmp/SKILL.md"
copy_file "$SOURCE_ROOT/agents/openai.yaml" "$root_projection_tmp/agents/openai.yaml"
copy_file "$generated_leaf_skill_tmp" "$leaf_projection_tmp/SKILL.md"
copy_file "$SOURCE_ROOT/agents/openai.yaml" "$leaf_projection_tmp/agents/openai.yaml"

assert_exact_file_set "$root_projection_tmp" "$(cat <<'EOF'
SKILL.md
agents/openai.yaml
EOF
)"
assert_exact_file_set "$leaf_projection_tmp" "$(cat <<'EOF'
SKILL.md
agents/openai.yaml
EOF
)"

mkdir -p "$ROOT_DIR/.agents/skills"
rm -rf "$ROOT_DIR/.agents/skills/system" "$ROOT_DIR/.agents/skills/system-charter-intake"
cp -R "$root_projection_tmp" "$ROOT_DIR/.agents/skills/system"
cp -R "$leaf_projection_tmp" "$ROOT_DIR/.agents/skills/system-charter-intake"
