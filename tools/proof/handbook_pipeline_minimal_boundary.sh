#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
FIXTURE_DIR="$ROOT/tests/fixtures/external_consumers/handbook_pipeline_minimal_boundary"
PROOF_CORPUS_SOURCE="$ROOT/tests/fixtures/pipeline_proof_corpus/foundation_inputs"
PIPELINE_MANIFEST="$ROOT/crates/pipeline/Cargo.toml"
PIPELINE_VERSION="$(sed -n 's/^version = "\([^"]*\)"$/\1/p' "$PIPELINE_MANIFEST" | head -n1)"
PACKAGE_TARBALL="$ROOT/target/package/handbook-pipeline-$PIPELINE_VERSION.crate"

if [[ -z "$PIPELINE_VERSION" ]]; then
  echo "error: could not determine handbook-pipeline version from $PIPELINE_MANIFEST" >&2
  exit 1
fi

if [[ ! -f "$PACKAGE_TARBALL" ]]; then
  echo "error: missing packaged artifact $PACKAGE_TARBALL" >&2
  echo "run 'cargo package -p handbook-pipeline --allow-dirty' first" >&2
  exit 1
fi

if [[ ! -d "$FIXTURE_DIR" ]]; then
  echo "error: missing fixture directory $FIXTURE_DIR" >&2
  exit 1
fi

if [[ ! -d "$PROOF_CORPUS_SOURCE" ]]; then
  echo "error: missing proof corpus directory $PROOF_CORPUS_SOURCE" >&2
  exit 1
fi

TMPDIR_ROOT="${TMPDIR:-/tmp}"
WORKDIR="$(mktemp -d "$TMPDIR_ROOT/handbook-pipeline-proof.XXXXXX")"
cleanup() {
  rm -rf "$WORKDIR"
}
trap cleanup EXIT

PACKAGE_DIR="$WORKDIR/package"
CONSUMER_DIR="$WORKDIR/consumer"
PROOF_CORPUS_DIR="$WORKDIR/proof_corpus"
CARGO_TARGET_DIR="$WORKDIR/target"
export CARGO_TARGET_DIR

mkdir -p "$PACKAGE_DIR" "$CONSUMER_DIR" "$PROOF_CORPUS_DIR"
tar -xzf "$PACKAGE_TARBALL" -C "$PACKAGE_DIR"
cp -R "$FIXTURE_DIR/." "$CONSUMER_DIR/"
cp -R "$PROOF_CORPUS_SOURCE/." "$PROOF_CORPUS_DIR/"

PACKAGE_PATH="$PACKAGE_DIR/handbook-pipeline-$PIPELINE_VERSION"
if [[ ! -d "$PACKAGE_PATH" ]]; then
  echo "error: expected unpacked package at $PACKAGE_PATH" >&2
  exit 1
fi

python3 - "$CONSUMER_DIR/Cargo.toml" "$PACKAGE_PATH" <<'PY'
from pathlib import Path
import sys
manifest_path = Path(sys.argv[1])
package_path = Path(sys.argv[2]).resolve()
contents = manifest_path.read_text()
contents = contents.replace("__HANDBOOK_PIPELINE_PACKAGE_PATH__", package_path.as_posix())
manifest_path.write_text(contents)
PY

echo "[proof] unpacked packaged crate: $PACKAGE_PATH"
echo "[proof] copied external consumer fixture: $CONSUMER_DIR"
echo "[proof] copied proof corpus: $PROOF_CORPUS_DIR"

cargo run --quiet --manifest-path "$CONSUMER_DIR/Cargo.toml" -- "$PROOF_CORPUS_DIR"

echo "[proof] handbook_pipeline_minimal_boundary: PASS"
