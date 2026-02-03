#!/usr/bin/env python3
"""
assemble_prompts.py

Compatibility wrapper around harness.py for prompt compilation.

This project evolved from a simple "assemble prompts into dist/" script into a
human-in-the-loop harness. For now, assemble_prompts delegates to:

  python tools/harness.py compile ...

so existing docs can keep referring to assemble_prompts.
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]


def main(argv: list[str] | None = None) -> int:
    argv = list(sys.argv[1:] if argv is None else argv)
    cmd = [sys.executable, str(ROOT / "tools" / "harness.py"), "compile"] + argv
    return subprocess.call(cmd)


if __name__ == "__main__":
    raise SystemExit(main())
