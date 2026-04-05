#!/usr/bin/env python3
from __future__ import annotations

import argparse
import shutil
import sys
import tempfile
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[1]
FIXTURE_ROOT = Path(__file__).resolve().parent / "fixtures" / "archive-boundary"
FORBIDDEN_TOKEN = "archived/"


def discover_supported_roots(root: Path) -> list[Path]:
    roots: list[Path] = []
    cargo_toml = root / "Cargo.toml"
    crates_dir = root / "crates"
    if cargo_toml.is_file():
        roots.append(cargo_toml)
    if crates_dir.is_dir():
        roots.append(crates_dir)
    return roots


def iter_scannable_files(paths: list[Path]) -> list[Path]:
    files: list[Path] = []
    for path in paths:
        if path.is_file():
            files.append(path)
            continue
        for candidate in path.rglob("*"):
            if not candidate.is_file():
                continue
            if candidate.name.startswith("."):
                continue
            if candidate.suffix not in {".rs", ".toml"} and candidate.name != "build.rs":
                continue
            files.append(candidate)
    return files


def scan_root(root: Path) -> int:
    supported_roots = discover_supported_roots(root)
    if not supported_roots:
        print(f"No supported runtime roots found under {root}; skipping archive boundary scan.")
        return 0

    violations: list[tuple[Path, int, str]] = []
    for file_path in iter_scannable_files(supported_roots):
        try:
            lines = file_path.read_text(encoding="utf-8", errors="replace").splitlines()
        except OSError as exc:
            print(f"{file_path}: unable to read file: {exc}", file=sys.stderr)
            return 2
        for line_number, line in enumerate(lines, start=1):
            if FORBIDDEN_TOKEN in line:
                violations.append((file_path, line_number, line.strip()))

    if violations:
        for file_path, line_number, excerpt in violations:
            relative = file_path.relative_to(root)
            print(f"{relative}:{line_number}: forbidden archived/ reference in supported runtime code")
            print(f"  {excerpt}")
        return 1

    print("Archive boundary check passed: no archived/ references found in supported runtime roots.")
    return 0


def copy_fixture_tree(source_name: str, destination: Path) -> Path:
    source = FIXTURE_ROOT / source_name
    if not source.exists():
        raise FileNotFoundError(f"Missing fixture tree: {source}")
    target = destination / source_name
    shutil.copytree(source, target)
    return target


def self_test() -> int:
    with tempfile.TemporaryDirectory() as tmpdir:
        temp_root = Path(tmpdir)
        pass_root = copy_fixture_tree("pass", temp_root)
        fail_root = copy_fixture_tree("fail", temp_root)

        pass_code = scan_root(pass_root)
        if pass_code != 0:
            print("Self-test failed: clean fixture unexpectedly failed.", file=sys.stderr)
            return 1

        fail_code = scan_root(fail_root)
        if fail_code != 1:
            print("Self-test failed: rejected fixture did not fail as expected.", file=sys.stderr)
            return 1

    print("Self-test passed: clean fixture passes and rejected fixture fails.")
    return 0


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        description="Check supported runtime code for forbidden archived/ references.",
        formatter_class=argparse.RawTextHelpFormatter,
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=REPO_ROOT,
        help="Repository root to scan (defaults to the current repo root).",
    )
    parser.add_argument(
        "--self-test",
        action="store_true",
        help="Run synthetic pass/fail fixture checks.",
    )
    return parser


def main(argv: list[str] | None = None) -> int:
    parser = build_parser()
    args = parser.parse_args(argv)
    if args.self_test:
        return self_test()
    return scan_root(args.root.resolve())


if __name__ == "__main__":
    raise SystemExit(main())
