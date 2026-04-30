# system

Root host skill for the `system` family.

- Installed home: `~/system/`
- Installed thin projections: `~/system/.agents/skills/system/` and `~/system/.agents/skills/system-charter-intake/`
- Codex discovery glue: `~/.codex/skills/system*`

This root skill is informational and packaging-oriented. Use `system-charter-intake` for the charter intake workflow.

Do not edit generated projections under `.agents/skills/**` by hand. Edit `SKILL.md.tmpl`, `charter-intake/SKILL.md.tmpl`, or `agents/openai.yaml`, then rerun `bash tools/codex/generate.sh`.
