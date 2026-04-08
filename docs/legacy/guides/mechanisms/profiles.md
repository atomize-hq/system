# Mechanism: Profiles

## What it is

A **profile** is a “stack pack” describing:
- language/toolchain assumptions
- command keys for install/lint/test/typecheck/security
- conventions for layout and style

Typical profile directory:

- `profiles/<id>/profile.yaml`
- `profiles/<id>/commands.yaml`
- `profiles/<id>/conventions.md`

## Why it exists

- Keep core prompts language-agnostic.
- Avoid hardcoding `uv`, `poetry`, `cargo`, `pnpm`, etc. into stages/rules.
- Make gate execution deterministic: the runner calls commands by key.

## How it works today

- The harness inlines the selected profile into `dist/<stage>.md`.
- Stages/runners refer to **command keys** (e.g., `commands.tests`) instead of raw commands.
- Placeholders (like `{code_dirs}`) can be expanded by the executor/runner layer.

## Creating a new profile

1) Copy a template profile directory (if present)
2) Fill `profile.yaml`:
   - `id`, `version`, `compatibility`
   - `project_defaults` (code dirs, test dirs, config files)
   - `gates.required` / `gates.optional`
3) Fill `commands.yaml` with deterministic commands
4) Write `conventions.md` with norms and evidence expectations
5) Compile a stage with `--profile <id>` to verify it is included.

## Do / Don’t

✅ Do:
- keep commands deterministic and non-interactive
- document placeholders and expected repo layouts
- include security tooling if appropriate for the stack

❌ Don’t:
- let profiles drift from reality (update them when tools change)
- duplicate profile commands inside rules or stage directives
