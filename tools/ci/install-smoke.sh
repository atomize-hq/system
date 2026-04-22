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
[[ "$demo_out" == *"fixture-backed"* ]] || {
  echo "expected fixture-backed labeling in execution demo output"
  exit 1
}
[[ "$demo_out" == *"FIXTURE SET: basic"* ]] || {
  echo "expected fixture set id in execution demo output"
  exit 1
}

echo "==> live execution refusal smoke (UnsupportedRequest)"
tmp_root="$(mktemp -d)"
cleanup() { rm -rf "$tmp_root"; }
trap cleanup EXIT

mkdir -p "$tmp_root/.system/charter" "$tmp_root/.system/feature_spec"
cat >"$tmp_root/.system/charter/CHARTER.md" <<'EOF'
# Engineering Charter — Smoke Fixture

## What this is
Body.

## How to use this charter
Use it.

## Rubric: 1–5 rigor levels
Levels.

## Project baseline posture
Baseline.

## Domains / areas (optional overrides)
None.

## Posture at a glance (quick scan)
Snapshot.

## Dimensions (details + guardrails)
Details.

## Cross-cutting red lines (global non-negotiables)
- Keep trust boundaries intact.

## Exceptions / overrides process
- **Approvers:** project_owner
- **Record location:** docs/exceptions.md
- **Minimum required fields:**
  - what
  - why
  - scope
  - risk
  - owner
  - expiry_or_revisit_date

## Debt tracking expectations
Tracked in issues.

## Decision Records (ADRs): how to use this charter
Use ADRs.

## Review & updates
Review monthly.
EOF
printf "feature" >"$tmp_root/.system/feature_spec/FEATURE_SPEC.md"

set +e
live_out="$(cd "$tmp_root" && system generate --packet execution.live.packet 2>&1)"
live_status=$?
set -e

if [[ $live_status -eq 0 ]]; then
  echo "expected nonzero exit for live execution packet refusal"
  exit 1
fi

[[ "$live_out" == *"CATEGORY: UnsupportedRequest"* ]] || {
  echo "expected UnsupportedRequest refusal category; got:"
  echo "$live_out"
  exit 1
}

if [[ "$live_out" != *'fixture-backed execution demos'* ]] &&
   [[ "$live_out" != *'NEXT SAFE ACTION: run `system generate --packet planning.packet`'* ]]; then
  echo "expected live refusal to mention the execution boundary or planning fallback; got:"
  echo "$live_out"
  exit 1
fi

echo "OK"
