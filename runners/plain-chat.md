# Runner: plain-chat (no tools)

This runner assumes the agent cannot run commands or edit files directly.

## Rules
- Do not claim gates passed.
- Provide exact commands to run (from the selected profile).
- Provide patch-style outputs (file paths + contents) when code changes are required.

## Evidence
- Provide a “to-run” checklist.
- If a gate fails, request the evidence output and iterate.
