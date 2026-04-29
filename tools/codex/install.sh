#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"
GENERATE_SCRIPT="$SCRIPT_DIR/generate.sh"
GENERATED_ROOT="$ROOT_DIR/.agents/skills"
INSTALL_ROOT="$HOME/.codex/skills"

DISCOVERY_NAME="system-charter-intake"
RUNTIME_NAME="system"

DISCOVERY_SOURCE="$GENERATED_ROOT/$DISCOVERY_NAME"
RUNTIME_SOURCE="$GENERATED_ROOT/$RUNTIME_NAME"
DISCOVERY_DEST="$INSTALL_ROOT/$DISCOVERY_NAME"
RUNTIME_DEST="$INSTALL_ROOT/$RUNTIME_NAME"

require_file() {
  local path="$1"

  if [[ ! -f "$path" ]]; then
    printf 'missing required file: %s\n' "$path" >&2
    exit 1
  fi
}

require_directory() {
  local path="$1"

  if [[ ! -d "$path" ]]; then
    printf 'missing required directory: %s\n' "$path" >&2
    exit 1
  fi
}

install_copy() {
  local source_path="$1"
  local dest_path="$2"

  rm -rf "$dest_path"
  cp -R "$source_path" "$dest_path"
}

require_file "$GENERATE_SCRIPT"
bash "$GENERATE_SCRIPT"

require_directory "$DISCOVERY_SOURCE"
require_directory "$RUNTIME_SOURCE"

mkdir -p "$INSTALL_ROOT"

install_copy "$DISCOVERY_SOURCE" "$DISCOVERY_DEST"
install_copy "$RUNTIME_SOURCE" "$RUNTIME_DEST"
