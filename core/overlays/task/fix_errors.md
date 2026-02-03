# Overlay: Fix Errors (Rapid Debug Loop)

Use when a stage fails quality gates or when execution output shows failures.

## Process
1) Reproduce the failure (run the exact failing command).
2) Capture evidence: command, exit code, tail of logs.
3) Identify root cause category:
   - syntax/formatting
   - type/static analysis
   - test failure (logic, fixture, environment)
   - dependency/env issue
4) Fix the smallest thing that makes the gate pass.
5) Re-run the minimal gate first, then the full gate set.

## Anti-patterns
- Do not “paper green” by disabling rules.
- Do not add broad ignores; scope exceptions tightly and document them.
