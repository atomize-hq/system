# Evidence Policy

This system treats “it passed” as a claim that must be backed by evidence.

## Evidence requirements
When you claim a gate passed, include an evidence record with:
- `cmd`: the exact command executed
- `exit`: exit code
- `tail`: last N lines of output (stderr + stdout)
- `timestamp`: UTC timestamp

## Format (recommended)
```yaml
- cmd: "<command>"
  exit: 0
  timestamp: "2026-01-27T00:00:00Z"
  tail: |
    <last ~80 lines>
```

## No-tools context
If you cannot run commands:
- Do not claim pass/fail.
- Provide the exact commands to run, using the selected profile (e.g., `commands.tests`).
