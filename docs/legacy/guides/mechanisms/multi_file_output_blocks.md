# Mechanism: Multi-file output blocks

## What it is

Some stages must output multiple files (e.g., Foundation Pack).
To make capture deterministic, the model outputs **file blocks**:

```text
--- FILE: artifacts/foundation/FOUNDATION_STRATEGY.md ---
<complete contents>

--- FILE: artifacts/foundation/quality_gates.yaml ---
<complete contents>
```

The harness parses these blocks and writes each file.

## Why it exists

- Avoids “where should I put this?” ambiguity
- Allows a single model response to create multiple artifacts
- Enables machine parsing without fragile heuristics

## How it works today

- The stage directive must require the wrapper format
- The harness looks for lines matching:
  - `--- FILE: <path> ---`

## Creating a multi-file stage

1) Declare multiple outputs in stage front matter (`outputs.artifacts`)
2) In the directive, include:
- “Output Formatting (MANDATORY — multi-file wrapper)”
- exact required file blocks and their order
- “no extra text” rule

## Do / Don’t

✅ Do:
- require exact paths (copy/paste safe)
- require “no extra commentary” outside blocks
- keep ordering stable

❌ Don’t:
- allow code fences around the entire output (breaks parsing in some harnesses)
- allow the model to choose file names
