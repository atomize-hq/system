# Mechanism: Quality Gates (human + machine)

## What it is

Quality gates are explicit, exhaustive definitions of what “passing” means.

Common pattern:
- `QUALITY_GATES_SPEC.md` (human-readable, exhaustive)
- `quality_gates.yaml` (machine-readable, executable spec)

## Why it exists

- Prevent “paper green” (claims without evidence)
- Make automation possible (CI/CD can run the same gates)
- Keep gate policy grounded in Charter + Foundation posture

## How it works today

- Foundation pack synthesis typically produces the gate specs.
- Runners/executors reference profile command keys to run gates.
- Evidence policy requires you to capture output for any claimed pass.

## Designing gates

A good gate is:
- deterministic
- executable
- tied to an exit code or clear check
- versioned and stable

Examples:
- formatting/lint checks
- unit tests
- type checks
- security scans
- coverage thresholds

## Do / Don’t

✅ Do:
- tie gate levels/thresholds back to Charter posture
- keep the machine spec executable without an LLM

❌ Don’t:
- rely on “AI says it passed” without command evidence
- define gates that cannot be automated
