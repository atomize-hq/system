#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
export PATH="${CARGO_HOME:-$HOME/.cargo}/bin:$PATH"

cd "$ROOT_DIR"

echo "==> cargo install (crates/cli)"
cargo install --locked --force --path crates/cli

echo "==> help smoke"
system --help >/dev/null
system generate --help >/dev/null
system inspect --help >/dev/null

echo "==> execution demo smoke (fixture-backed)"
demo_out="$(system generate --packet execution.demo.packet --fixture-set basic || true)"
echo "$demo_out" | grep -qF "fixture-backed" || {
  echo "expected fixture-backed labeling in execution demo output"
  exit 1
}
echo "$demo_out" | grep -qF "FIXTURE SET: basic" || {
  echo "expected fixture set id in execution demo output"
  exit 1
}

echo "==> live execution refusal smoke (UnsupportedRequest)"
tmp_root="$(mktemp -d)"
cleanup() { rm -rf "$tmp_root"; }
trap cleanup EXIT

mkdir -p "$tmp_root/.system/charter" "$tmp_root/.system/feature_spec"
printf "charter" >"$tmp_root/.system/charter/CHARTER.md"
printf "feature" >"$tmp_root/.system/feature_spec/FEATURE_SPEC.md"

set +e
live_out="$(cd "$tmp_root" && system generate --packet execution.live.packet 2>&1)"
live_status=$?
set -e

if [[ $live_status -eq 0 ]]; then
  echo "expected nonzero exit for live execution packet refusal"
  exit 1
fi

echo "$live_out" | grep -qF "CATEGORY: UnsupportedRequest" || {
  echo "expected UnsupportedRequest refusal category; got:"
  echo "$live_out"
  exit 1
}

echo "OK"
