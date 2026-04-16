#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage: tools/qa/prepare_fixture_checkout.sh --fixture-root <path> [--nested-cwd <relative-path>]
EOF
}

fixture_root=""
nested_cwd=""

while [[ $# -gt 0 ]]; do
  case "$1" in
    --fixture-root)
      [[ $# -ge 2 ]] || {
        echo "error: --fixture-root requires a value" >&2
        usage >&2
        exit 1
      }
      fixture_root="$2"
      shift 2
      ;;
    --nested-cwd)
      [[ $# -ge 2 ]] || {
        echo "error: --nested-cwd requires a value" >&2
        usage >&2
        exit 1
      }
      nested_cwd="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "error: unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

[[ -n "$fixture_root" ]] || {
  echo "error: --fixture-root is required" >&2
  usage >&2
  exit 1
}

[[ -d "$fixture_root" ]] || {
  echo "error: fixture root does not exist: $fixture_root" >&2
  exit 1
}

case "$nested_cwd" in
  "")
    ;;
  /*)
    echo "error: --nested-cwd must be relative" >&2
    exit 1
    ;;
  .|..|../*|*/../*|*/..)
    echo "error: --nested-cwd must stay within the checkout root" >&2
    exit 1
    ;;
esac

tmp_parent="$(mktemp -d "${TMPDIR:-/tmp}/system-fixture-checkout.XXXXXX")"
checkout_root="$tmp_parent/checkout"
mkdir -p "$checkout_root"
cp -R "$fixture_root"/. "$checkout_root"/
mkdir -p "$checkout_root/.git"

effective_cwd="$checkout_root"
if [[ -n "$nested_cwd" ]]; then
  effective_cwd="$checkout_root/$nested_cwd"
  mkdir -p "$effective_cwd"
fi

printf 'CHECKOUT_ROOT=%q\n' "$checkout_root"
printf 'EFFECTIVE_CWD=%q\n' "$effective_cwd"
