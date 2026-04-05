#!/usr/bin/env bash
set -euo pipefail

HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "${HERE}/.." && pwd)"

case "${1:-}" in
  -h|--help|help)
    cat <<'EOF'
Legacy reference material only.
Rust-first is the supported direction. See docs/contracts/C-01-approved-repo-surface.md.
Allowed changes: bug fixes, link corrections, and narrow wording fixes only until cutover.
EOF
    exec python3 "${ROOT}/tools/harness.py" --help
    ;;
esac

python3 "${ROOT}/tools/harness.py" "$@"
