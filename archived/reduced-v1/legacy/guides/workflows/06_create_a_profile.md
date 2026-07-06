# Workflow: Create a new Profile (stack pack)

Profiles define *project tooling assumptions* (install/lint/tests/typecheck/security), **without** hardcoding them into core prompts.

## When you need a new profile

- A different language/toolchain (Rust, Node, Go, etc.)
- A different package manager for the same language (Python-uv vs Python-poetry)
- A different gate set (e.g., mandatory `clippy`, mandatory `cargo deny`)

## What a profile contains

Typical files:

- `profiles/<id>/profile.yaml`
- `profiles/<id>/commands.yaml`
- `profiles/<id>/conventions.md`

## Steps

1) Copy the template profile directory (if you have one)

```bash
cp -R profiles/_template profiles/<your-profile-id>
```

2) Edit `profile.yaml`

- Set `id`, `version`, `compatibility`
- Declare `gates.required` and `gates.optional`

3) Edit `commands.yaml`

Use command **keys** that the system can reference, e.g.:

- `commands.install`
- `commands.format`
- `commands.lint`
- `commands.tests`
- `commands.typecheck`
- `commands.security`

Keep commands runnable by the runner (no interactive prompts).

4) Edit `conventions.md`

Document:
- layout assumptions
- style/typing/test norms
- evidence capture expectations

5) Test by compiling a stage

```bash
python3 tools/harness.py compile --only stage.00_base --profile <your-profile-id>
```

If the compiled prompt includes your profile content, you’re good.

## Do / Don’t

✅ Do:
- keep it minimal and accurate
- prefer deterministic commands
- include placeholders if needed (e.g., `{code_dirs}`)

❌ Don’t:
- embed profile commands inside core rules/stages
- depend on undocumented global shell state
