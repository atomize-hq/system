# Prompt Pipeline System (Scaffold)

## Current Status

This repo is in transition.

- The reviewed v1 direction is a **Rust-first planning/compiler CLI**.
- The current Python harness remains in the repo as **legacy reference material only**.
- Python is not the supported product path, not a compatibility wrapper, and will be archived during cutover, then removed.
- The governing repo-surface truth is [C-01 Approved Repo-Surface Contract](docs/contracts/C-01-approved-repo-surface.md).
- The command-surface truth for the Rust workspace and CLI is [C-02 Rust Workspace and CLI Command-Surface Contract](docs/contracts/C-02-rust-workspace-and-cli-command-surface.md).
- The operator-facing product language is locked in [CLI Product Vocabulary](docs/CLI_PRODUCT_VOCABULARY.md).
- The front door and steady-state routing model is locked in [CLI Command Hierarchy And Front Door](docs/CLI_COMMAND_HIERARCHY.md).
- The operator-facing tone rules live in [CLI Tone Rules](docs/CLI_TONE_RULES.md).
- The output-shape and section-order rules live in [CLI Output Anatomy](docs/CLI_OUTPUT_ANATOMY.md).
- The composed CLI interaction contract lives in [DESIGN.md](DESIGN.md).
- The shipped journey and revision findings live in [CLI Operator Journey And Conformance Review](docs/CLI_OPERATOR_JOURNEY.md).
- The canonical `.system/` manifest + freshness truth is [C-03 Canonical Artifact Manifest Contract](docs/contracts/C-03-canonical-artifact-manifest-contract.md).
- The reduced live v1 scope is **route resolution, explicit stage compilation, and planning packet generation over existing project + feature artifacts**.
- Planning packet generation is supported from canonical repo-local `.system/`.
- Fixture-backed execution demo generation is supported via `execution.demo.packet`.
- Live slice lineage and live execution packets are deferred. Live execution is explicitly refused.
- `pipeline` is the orchestration surface once its code, docs, contracts, tests, and proof-corpus gates land together.
- `inspect` is the proof surface and `doctor` is the recovery surface.
- `setup` is still a placeholder entrypoint until Rust setup exists.

The next artifact is the implementation plan for the reduced v1 wedge at [PLAN.md](PLAN.md). The reviewed reduced-v1 seam pack lives at [artifacts/planning/reduced-v1-seam-pack/README.md](artifacts/planning/reduced-v1-seam-pack/README.md).

This repo is a **human-in-the-loop** prompt pipeline that produces structured artifacts
(Charter, Project Context, Foundation Pack, Feature Specs, etc.) using a selected **profile** (stack pack)
and **runner** (how the agent interacts with the repo).

It intentionally does **not** call any LLM APIs. You copy/paste prompts into your LLM of choice and paste
the outputs back into the harness.

## Legacy Quick Start

The steps below describe the legacy Python harness that still exists in the repo today. They do **not** describe the reviewed Rust-first product path.

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

> NOTE: `artifacts/` is legacy-harness output. A snapshot of legacy generated outputs is kept under `archived/legacy-generated-artifacts/` to keep the repo easier to navigate during the Rust-first transition.

Some stages write a canonical document **into the project repo** (via `${repo_root}/...`)
and also keep a pipeline copy under `artifacts/...` for traceability.

Example: `ENVIRONMENT_INVENTORY.md` is canonical at the repo/project root, with a pipeline copy at
`artifacts/foundation/ENVIRONMENT_INVENTORY.md`.
