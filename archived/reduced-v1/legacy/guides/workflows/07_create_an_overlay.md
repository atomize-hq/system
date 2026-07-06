# Workflow: Create a new Overlay

Overlays are optional, composable “behavior modules” that add guidance without changing core contracts.

## When you need an overlay

- A pattern applies sometimes, not always (e.g., bounded sprints)
- You want a policy “lens” that can be toggled per run
- You want to keep the core stage prompts small and stable

## Steps

1) Create a new overlay file

If overlays are flat:

```bash
touch core/overlays/<name>.md
```

If overlays are organized in subdirectories:

```bash
mkdir -p core/overlays/task
touch core/overlays/task/<name>.md
```

2) Write the overlay

A good overlay has:
- Goal
- When to use
- Constraints / rules
- Output expectations (if it affects artifacts)

3) Include it when compiling/running

If your harness supports `--overlays`:

```bash
python3 tools/harness.py run stage.07_foundation_pack --overlays task/<name>
```

Or comma-separated:

```bash
python3 tools/harness.py run stage.07_foundation_pack --overlays "task/<name>,sprint/bounded_sprints_lanes"
```

4) Verify the overlay appears in `dist/<stage>.md`

## Do / Don’t

✅ Do:
- keep overlays orthogonal (one concern each)
- avoid redefining core output contracts
- use overlays to add heuristics, not new truth sources

❌ Don’t:
- hardcode stack-specific commands (profiles own commands)
- make overlays required unless the pipeline always includes them
