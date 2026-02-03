# Workflow: Create a new Runner

Runners explain what capabilities the “agent” has when executing:

- can it run shell commands?
- can it edit files?
- how is evidence captured?
- how should outputs be formatted?

Runners are documentation + rules; they are not code.

## Steps

1) Create the runner file

```bash
touch runners/<runner-id>.md
```

2) Include required sections

A strong runner file defines:
- command execution rules
- evidence capture requirements
- file edit discipline
- output discipline (single doc vs multi-file blocks)

3) Use it

```bash
python3 tools/harness.py compile --only stage.05_charter_interview --runner <runner-id>
```

4) Verify it appears in `dist/<stage>.md`

## Do / Don’t

✅ Do:
- be explicit about evidence capture format
- state the interaction model (tools vs no-tools)

❌ Don’t:
- put project-specific stack commands here
- contradict core rules (P0/P1)
