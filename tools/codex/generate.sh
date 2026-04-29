#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
SKILL_NAME="system-charter-intake"
MANIFEST_VERSION="1"

release_version="$(tr -d '[:space:]' <"$ROOT_DIR/VERSION")"

if [[ -n "${SOURCE_DATE_EPOCH:-}" ]]; then
  generated_at_utc="$(python3 - <<'PY'
import datetime
import os
print(datetime.datetime.fromtimestamp(int(os.environ["SOURCE_DATE_EPOCH"]), datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"))
PY
)"
elif git -C "$ROOT_DIR" rev-parse --verify HEAD >/dev/null 2>&1; then
  generated_at_utc="$(python3 - "$ROOT_DIR" <<'PY'
import datetime
import subprocess
import sys

root = sys.argv[1]
epoch = subprocess.check_output(
    ["git", "-C", root, "show", "-s", "--format=%ct", "HEAD"],
    text=True,
).strip()
print(datetime.datetime.fromtimestamp(int(epoch), datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"))
PY
)"
else
  generated_at_utc="1970-01-01T00:00:00Z"
fi

export ROOT_DIR SKILL_NAME MANIFEST_VERSION release_version generated_at_utc

tmp_dir="$(mktemp -d "${TMPDIR:-/tmp}/system-codex-generate.XXXXXX")"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

render_template() {
  local template_path="$1"
  local destination_path="$2"

  python3 - "$template_path" "$destination_path" <<'PY'
import os
import pathlib
import sys

template_path = pathlib.Path(sys.argv[1])
destination_path = pathlib.Path(sys.argv[2])
text = template_path.read_text(encoding="utf-8")
replacements = {
    "__SKILL_NAME__": os.environ["SKILL_NAME"],
    "__SYSTEM_RELEASE_VERSION__": os.environ["release_version"],
    "__MANIFEST_VERSION__": os.environ["MANIFEST_VERSION"],
    "__GENERATED_AT_UTC__": os.environ["generated_at_utc"],
}
for token, value in replacements.items():
    text = text.replace(token, value)
destination_path.parent.mkdir(parents=True, exist_ok=True)
destination_path.write_text(text, encoding="utf-8")
PY
}

generated_root="$ROOT_DIR/.agents/skills"
discovery_dir_tmp="$tmp_dir/system-charter-intake"
runtime_dir_tmp="$tmp_dir/system"

mkdir -p "$discovery_dir_tmp" "$runtime_dir_tmp/bin" \
  "$runtime_dir_tmp/share/authoring" "$runtime_dir_tmp/share/charter"

render_template \
  "$ROOT_DIR/tools/codex/templates/system-charter-intake.SKILL.md.tmpl" \
  "$discovery_dir_tmp/SKILL.md"
render_template \
  "$ROOT_DIR/tools/codex/runtime/SKILL.md.tmpl" \
  "$runtime_dir_tmp/SKILL.md"
render_template \
  "$ROOT_DIR/tools/codex/runtime/runtime-manifest.json.tmpl" \
  "$runtime_dir_tmp/runtime-manifest.json"
render_template \
  "$ROOT_DIR/tools/codex/runtime/bin/system-charter-intake.tmpl" \
  "$runtime_dir_tmp/bin/system-charter-intake"

cp "$ROOT_DIR/core/library/authoring/charter_authoring_method.md" \
  "$runtime_dir_tmp/share/authoring/charter_authoring_method.md"
cp "$ROOT_DIR/core/library/charter/CHARTER_INPUTS.yaml.tmpl" \
  "$runtime_dir_tmp/share/charter/CHARTER_INPUTS.yaml.tmpl"
cp "$ROOT_DIR/core/library/charter/charter_inputs_directive.md" \
  "$runtime_dir_tmp/share/charter/charter_inputs_directive.md"

chmod 0755 "$runtime_dir_tmp/bin/system-charter-intake"

mkdir -p "$generated_root"
rm -rf "$generated_root/system-charter-intake" "$generated_root/system"
cp -R "$discovery_dir_tmp" "$generated_root/system-charter-intake"
cp -R "$runtime_dir_tmp" "$generated_root/system"
