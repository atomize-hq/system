#!/usr/bin/env python3
"""
validate_profile.py

Validates profile directories under system/profiles/.

A "profile" is a stack pack: commands + conventions + metadata that stages can
reference without hardcoding language/tool commands in core prompts.
"""
from __future__ import annotations

import argparse
import sys
from pathlib import Path
from typing import Any, Dict, List, Tuple

try:
    import yaml  # type: ignore
except ModuleNotFoundError:  # pragma: no cover
    import yaml_lite as yaml  # type: ignore

ROOT = Path(__file__).resolve().parents[1]
PROFILES_DIR = ROOT / "profiles"


REQUIRED_FILES = ["profile.yaml", "commands.yaml", "conventions.md"]


def _read_text(path: Path) -> str:
    return path.read_text(encoding="utf-8", errors="replace")


def validate_one(profile_id: str) -> List[str]:
    errs: List[str] = []
    pdir = PROFILES_DIR / profile_id
    if not pdir.exists():
        return [f"Profile dir missing: {pdir}"]

    for f in REQUIRED_FILES:
        fp = pdir / f
        if not fp.exists():
            errs.append(f"Missing file: profiles/{profile_id}/{f}")
        elif fp.stat().st_size == 0:
            errs.append(f"Empty file: profiles/{profile_id}/{f}")

    # parse profile.yaml
    py = pdir / "profile.yaml"
    if py.exists() and py.stat().st_size > 0:
        try:
            data = yaml.safe_load(_read_text(py)) or {}
            if not isinstance(data, dict):
                errs.append(f"profile.yaml must be a mapping: profiles/{profile_id}/profile.yaml")
            else:
                for key in ["kind", "id", "version", "title", "description"]:
                    if key not in data:
                        errs.append(f"Missing key '{key}' in profiles/{profile_id}/profile.yaml")
                # id should match folder
                pid = str(data.get("id", "") or "")
                if pid and pid != profile_id:
                    errs.append(f"profile id mismatch: folder '{profile_id}' vs profile.yaml id '{pid}'")
        except Exception as e:
            errs.append(f"Failed to parse profile.yaml for {profile_id}: {e}")

    # parse commands.yaml
    cy = pdir / "commands.yaml"
    if cy.exists() and cy.stat().st_size > 0:
        try:
            data = yaml.safe_load(_read_text(cy)) or {}
            if not isinstance(data, dict) or "commands" not in data or not isinstance(data["commands"], dict):
                errs.append(f"commands.yaml must contain top-level 'commands:' mapping: profiles/{profile_id}/commands.yaml")
        except Exception as e:
            errs.append(f"Failed to parse commands.yaml for {profile_id}: {e}")

    return errs


def list_profiles() -> List[str]:
    if not PROFILES_DIR.exists():
        return []
    return sorted([p.name for p in PROFILES_DIR.iterdir() if p.is_dir() and not p.name.startswith(".")])


def main(argv: List[str] | None = None) -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--all", action="store_true", help="Validate all profiles")
    parser.add_argument("profile", nargs="?", help="Profile id (folder name) to validate")
    args = parser.parse_args(argv)

    targets: List[str]
    if args.all:
        targets = [p for p in list_profiles() if p != "_template"]
    elif args.profile:
        targets = [args.profile]
    else:
        parser.print_help()
        return 2

    any_err = False
    for pid in targets:
        errs = validate_one(pid)
        if errs:
            any_err = True
            print(f"[FAIL] {pid}")
            for e in errs:
                print(f"  - {e}")
        else:
            print(f"[OK]   {pid}")
    return 1 if any_err else 0


if __name__ == "__main__":
    raise SystemExit(main())
