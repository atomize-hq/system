# Prompt Pipeline System (Scaffold)

This repo is a **human-in-the-loop** prompt pipeline that produces structured artifacts
(Charter, Project Context, Foundation Pack, Feature Specs, etc.) using a selected **profile** (stack pack)
and **runner** (how the agent interacts with the repo).

It intentionally does **not** call any LLM APIs. You copy/paste prompts into your LLM of choice and paste
the outputs back into the harness.

## Quick start

### 1) Pick a profile + runner
Profiles live in `profiles/<profile-id>/` (commands + conventions).

Runners live in `runners/<runner-id>.md`.

### 2) Compile prompts for the stages you want
```bash
./tools/harness.sh compile --until stage.06_project_context_interview \
  --profile python-uv \
  --runner codex-cli \
  --project-name "MyProject" \
  --repo-or-project-ref "github.com/me/myproject"
```

This writes compiled prompts to `dist/` (e.g., `dist/stage.05_charter_interview.md`).

### 3) Copy/paste into your LLM, then capture outputs
For **single-file** stages, paste the model output directly into:

```bash
./tools/harness.sh capture stage.05_charter_interview
# paste output, then Ctrl-D
```

For **multi-file** stages (e.g., `stage.07_foundation_pack`), the model must output `--- FILE: ... ---` blocks.
The harness will write each declared artifact.

### 4) State is stored automatically
The harness stores run variables (runner/profile/etc.) in:
- `artifacts/_harness_state.yaml`

After capturing `stage.05_charter_interview`, the harness will prompt you to set `needs_project_context`
(because it’s required to decide whether `stage.06_project_context_interview` should run).

## Useful commands

List stages:
```bash
./tools/harness.sh list
```

List overlays:
```bash
./tools/harness.sh overlays
```

Validate profiles:
```bash
python3 tools/validate_profile.py --all
```

## Design notes
- Core stages/rules are language-agnostic.
- Stack/tool commands belong in **profiles/**.
- Optional “modules” belong in **core/overlays/**.
- Generated artifacts live in **artifacts/**.

## Work levels and scoped rules
Stages declare a `work_level` (`L0`..`L3`). Included markdown can use scoped blocks:

```md
<!-- SCOPE: L2,L3 -->
...only included for those levels...
<!-- END_SCOPE -->
```

The harness filters these blocks when compiling prompts, which keeps context packs
lean while still enforcing strict execution/merge discipline.

## Repo outputs vs pipeline artifacts
Some stages write a canonical document **into the project repo** (via `${repo_root}/...`)
and also keep a pipeline copy under `artifacts/...` for traceability.

Example: `ENVIRONMENT_INVENTORY.md` is canonical at the repo/project root, with a pipeline copy at
`artifacts/foundation/ENVIRONMENT_INVENTORY.md`.
