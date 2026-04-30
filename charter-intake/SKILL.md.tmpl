# system-charter-intake

Generated discovery surface for the `system` charter intake workflow. `~/.codex/skills/system-charter-intake` is discovery glue only, not the installed home.

Use this skill only from inside a real git work tree. Resolve the installed `system` home and export `SYSTEM_HOME` before invoking the helper:

```bash
repo_root="$(git rev-parse --show-toplevel 2>/dev/null)" || {
  echo "REFUSED: run this skill from inside a real git repository." >&2
  exit 1
}
system_home="${SYSTEM_HOME:-$HOME/system}"
for required in \
  "$system_home/bin/system" \
  "$system_home/bin/system-charter-intake" \
  "$system_home/runtime-manifest.json" \
  "$system_home/share/authoring/charter_authoring_method.md" \
  "$system_home/share/charter/CHARTER_INPUTS.yaml.tmpl" \
  "$system_home/share/charter/charter_inputs_directive.md"; do
  [[ -f "$required" ]] || {
    echo "REFUSED: missing installed system home prerequisite: $required" >&2
    exit 1
  }
done
export SYSTEM_HOME="$system_home"
```

Workflow:

1. Review `$SYSTEM_HOME/share/authoring/charter_authoring_method.md` and `$SYSTEM_HOME/share/charter/charter_inputs_directive.md`.
2. Gather only the charter facts needed to fill the normalized YAML shape from `$SYSTEM_HOME/share/charter/CHARTER_INPUTS.yaml.tmpl`.
3. Write the normalized YAML to a temporary file outside the target repo.
4. Invoke `$SYSTEM_HOME/bin/system-charter-intake --inputs <path|->`.
5. Treat `~/.local/state/system/intake/runs/.../` as the evidence surface. Only `system doctor --json` may be machine-parsed.

Repo-local `.agents/skills/system*` trees remain thin generated projections. They are never the runtime payload root.
