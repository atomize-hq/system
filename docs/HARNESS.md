# Harness

## Legacy Status

This document describes the current Python harness mechanics that still exist in the repo.

- It is **legacy reference material**, not the supported v1 product path.
- The reviewed v1 direction is a Rust CLI context compiler.
- Python is not a compatibility wrapper and should not be extended as if it were the long-term runtime.

The harness is the executable glue for this repo’s **human‑in‑the‑loop** workflow.

It does two jobs:

1) **Compile** a stage prompt into `dist/<stage_id>.md` by assembling:
   - stage front matter
   - included rules/runner/profile docs
   - referenced library directives + templates
   - referenced upstream artifacts (if present)
2) **Capture** model output from stdin and write it to the stage’s declared output paths.

The harness does **not** call any LLM APIs.

For “what should I run next?” see the [Stage reference](stages/README.md).

## Commands

These commands are for the legacy harness workflow only.

All commands are run from `system/`:

### List stages
```bash
./tools/harness.sh list
```

### List overlays
```bash
./tools/harness.sh overlays
```

### Compile prompts
Compile up to a stage:
```bash
./tools/harness.sh compile --until stage.06_project_context_interview \
  --profile python-uv \
  --runner codex-cli \
  --project-name "MyProject" \
  --repo-or-project-ref "github.com/me/myproject"
```



Compile using an alternate pipeline:
```bash
./tools/harness.sh compile --pipeline pipelines/release.yaml --until stage.01_release_plan \
  --release-id release-001 --release-type minor
```

Compile only one stage:
```bash
./tools/harness.sh compile --only stage.05_charter_interview
```

Force compile even if activation conditions are false:
```bash
./tools/harness.sh compile --only stage.06_project_context_interview --force
```

Optional flags supported by the harness:
- `--pipeline <path>` (select an alternate pipeline YAML; default is `pipeline.yaml`)
- `--enable-complexity true|false` (controls auto overlay injection)
- `--needs-project-context true|false` (forces stage 06 activation)
- `--test-mode true|false` (if your harness is patched for it; used for Charter test-mode directive swapping)
- `--release-id <id>` and `--release-type <type>` (used by release planning outputs)
- `--sprint-id <id>` and `--sprint-slot <slot>` (used by sprint planning outputs)
- `--prev-sprint-id <id>` (optional; used to include previous sprint report inputs when available)

### Run (compile + capture)
Convenience command:
```bash
./tools/harness.sh run stage.05_charter_interview
# paste model output, then Ctrl-D
```

Charter test mode (if supported):
```bash
./tools/harness.sh compile --only stage.05_charter_interview --test-mode true
# paste dist/stage.05_charter_interview.md into your LLM
./tools/harness.sh capture stage.05_charter_interview
```

### Capture (write outputs)
Capture expects you to paste the model output on stdin:
```bash
./tools/harness.sh capture stage.05_charter_interview
# paste model output, then Ctrl-D
```

## Variables and state

The harness stores state at:
- `artifacts/_harness_state.yaml`

Variable precedence:
1) CLI flags (e.g., `--profile`, `--runner`)
2) `artifacts/_harness_state.yaml`
3) `pipeline.yaml` defaults

Some pipeline stages declare `sets:` variables (e.g., Charter sets `needs_project_context`).
After you capture such a stage, the harness will prompt you to set that value (with a heuristic default).

## Output formats

### Single-file stages
If a stage declares exactly one artifact output, the harness writes the pasted content to:
- `outputs.artifacts[0]`
- and any `outputs.repo_files` declared for that stage

### Multi-file stages (FILE blocks)
If a stage declares multiple artifact outputs (e.g., Foundation Pack), the model output must be formatted as:

```md
--- FILE: artifacts/foundation/FOUNDATION_STRATEGY.md ---
<contents>

--- FILE: artifacts/foundation/TECH_ARCH_BRIEF.md ---
<contents>

...etc...
```

Rules:
- The `--- FILE: ... ---` line must match exactly.
- Do not wrap file contents in code fences.
- The harness ignores extra blocks not declared in stage outputs.

#### Repo file outputs (canonical docs)
For `outputs.repo_files`, the harness will:
- write a repo file block if the model emitted it, OR
- copy from an artifact output with the same filename (basename match)

Example: if a stage declares `${repo_root}/ENVIRONMENT_INVENTORY.md`, the harness will copy from `artifacts/foundation/ENVIRONMENT_INVENTORY.md` if needed.

## Profile validation

Profiles can be validated with:
```bash
python3 tools/validate_profile.py --all
```
