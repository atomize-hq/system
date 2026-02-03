# Mechanism: Rules + Work Levels + Scoped Blocks

## What it is

- `core/rules/*.md` are universal rules (P0/P1, evidence, traceability).
- **Work levels** (L0–L3) let you scope certain rules to certain phases.
- **Scoped blocks** let one rules file contain multiple “only at L2/L3” constraints.

## Why it exists

You want:
- parallel planning workstreams at higher levels
- strict single-threaded discipline during slice execution
- concise context packs (don’t include irrelevant rules at a given stage)

## Work levels (typical meaning)

- **L0 Program**: roadmap/release sequencing (parallel ok)
- **L1 Project/Planning**: charter/context/foundation/feature specs (parallel ok)
- **L2 Slice Execution**: implement one slice in one worktree/branch (execution discipline)
- **L3 Quality Gate & Merge**: final verification and integration discipline

## Scoped blocks (how it works)

In included markdown (often rules files), you can mark blocks like:

```text
<!-- SCOPE: L2,L3 -->
... only included for L2 or L3 ...
<!-- END_SCOPE -->
```

When a stage has `work_level: L1`, scoped L2/L3 blocks are excluded from the compiled prompt.

## How to create/update rules

1) Keep “always true” rules outside scoped blocks
2) Put execution-only constraints inside `SCOPE: L2,L3`
3) Keep wording language-agnostic; profile/runner carry tool details

## Do / Don’t

✅ Do:
- keep P0 small and enforceable
- scope “one slice in flight” to L2 (or L2/L3)
- use evidence policy to prevent “paper green”

❌ Don’t:
- duplicate rules across many files; prefer scoped blocks
- put project-specific requirements in universal rules (use Charter/Foundation)
