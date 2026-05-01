#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

usage() {
  cat <<'EOF'
Usage: package-system-home.sh --binary <path> --label <label> --output-dir <dir> [--version <version>]

Build a release tarball containing the exact curated ~/system home for one target.

Required:
  --binary      Path to the built system binary to package
  --label       Target label, for example macos_arm64 or linux_x86_64
  --output-dir  Directory where the release tarball will be written

Optional:
  --version     Version string to embed in the asset name. Defaults to VERSION file.
EOF
}

fatal() {
  printf '[package-system-home][ERROR] %s\n' "$*" >&2
  exit 1
}

BINARY_PATH=""
LABEL=""
OUTPUT_DIR=""
VERSION=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --binary)
      BINARY_PATH="${2:-}"
      shift 2
      ;;
    --label)
      LABEL="${2:-}"
      shift 2
      ;;
    --output-dir)
      OUTPUT_DIR="${2:-}"
      shift 2
      ;;
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage
      fatal "unknown argument: $1"
      ;;
  esac
done

[[ -n "$BINARY_PATH" ]] || fatal "--binary is required"
[[ -n "$LABEL" ]] || fatal "--label is required"
[[ -n "$OUTPUT_DIR" ]] || fatal "--output-dir is required"
[[ -x "$BINARY_PATH" ]] || fatal "binary is missing or not executable: $BINARY_PATH"

if [[ -z "$VERSION" ]]; then
  VERSION="$(tr -d '[:space:]' <"$ROOT_DIR/VERSION")"
fi
[[ -n "$VERSION" ]] || fatal "version is empty"

tmp_root="$(mktemp -d "${TMPDIR:-/tmp}/system-release-package.XXXXXX")"
cleanup() {
  rm -rf "$tmp_root"
}
trap cleanup EXIT

home_dir="$tmp_root/home"
bin_dir="$tmp_root/bin"
mkdir -p "$home_dir" "$bin_dir" "$OUTPUT_DIR"

cp "$BINARY_PATH" "$bin_dir/system"
chmod 0755 "$bin_dir/system"

binary_version="$("$bin_dir/system" --version | awk '{print $NF}')"
[[ "$binary_version" == "$VERSION" ]] || {
  fatal "binary version mismatch: expected $VERSION, found $binary_version"
}

export HOME="$home_dir"
export PATH="$bin_dir:$PATH"

bash "$ROOT_DIR/tools/codex/install.sh"

[[ -d "$HOME/system" ]] || fatal "expected packaged system home at $HOME/system"

asset_name="system-v${VERSION}-${LABEL}.tar.gz"
asset_path="$OUTPUT_DIR/$asset_name"

COPYFILE_DISABLE=1 tar -C "$HOME" -czf "$asset_path" system

printf '%s\n' "$asset_path"
