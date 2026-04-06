# Mechanism: Runners

## What it is

A **runner** describes the agent’s capabilities and behavioral rules:

- can execute shell commands or not
- can edit files or not
- evidence capture requirements
- output formatting discipline (single doc vs multi-file blocks)

Runners typically live under `runners/<id>.md`.

## Why it exists

Your pipeline may be run via:
- a tool-enabled CLI agent (codex-cli)
- an editor agent (cursor)
- a plain-chat situation (no tools)

The runner lets the prompts adjust behavior without changing the system core.

## How it works today

- Stage `includes:` usually inject `runners/${runner}.md`
- The compiled prompt tells the LLM how to present evidence and outputs

## Creating a new runner

1) Create `runners/<runner-id>.md`
2) Define:
   - command execution rules (how to reference profile commands)
   - evidence format requirements
   - file editing discipline
   - output discipline
3) Compile a stage with `--runner <runner-id>` and verify inclusion.

## Do / Don’t

✅ Do:
- explicitly define evidence capture fields (cmd, exit, tail, timestamp)
- enforce output wrappers for multi-file stages

❌ Don’t:
- put stack commands here (profiles own commands)
- claim tooling capabilities that aren’t actually present
