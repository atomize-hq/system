# Glossary (Legacy Harness Terms)

This file preserves the Python-harness terms that still appear in the frozen legacy docs.

For the supported reduced-v1 product language, start with:

- [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md)
- [`docs/CLI_COMMAND_HIERARCHY.md`](CLI_COMMAND_HIERARCHY.md)
- [`docs/SUPPORTED_COMMANDS.md`](SUPPORTED_COMMANDS.md)

Use this glossary when you need to decode older scaffold docs, `pipeline.yaml`, or legacy stage references.

> Tip: If you’re trying to understand the legacy stage runner ("what runs when"), start with [`pipeline.yaml`](../pipeline.yaml) and then the stage reference in [`docs/legacy/stages/`](legacy/stages/README.md).

---

## Pipeline
A YAML file (default: `system/pipeline.yaml`, or another file selected via `--pipeline`) that defines:
- pipeline defaults (runner/profile and a few booleans)
- ordered list of stages
- a small amount of routing (e.g., `sets`, and stage activation)

The harness uses the pipeline to know **which stage IDs exist** and **what order to run them**.

Related:
- **Stage**
- **Activation**
- **sets**

## Stage
A pipeline step represented by a markdown file under `core/stages/`.
Each stage file contains:
- YAML **front matter** describing inputs/outputs/includes, and
- an optional markdown body (typically minimal)

Stages are compiled into a single prompt per stage under `dist/`.

Related:
- **Front matter**
- **Compiled prompt**
- **Work level**

## Work level
A hierarchy label attached to stages (e.g., `L0`, `L1`, `L2`, `L3`).
It exists to keep prompts concise while still enforcing stricter rules when appropriate.

Current levels:
- **L0 Program** — portfolio/roadmap sequencing
- **L1 Project/Planning** — charter, project context, foundation, feature specs
- **L2 Slice Execution** — implementing one slice in a single working context
- **L3 Quality Gate & Merge** — final verification/merge discipline

Work level is used today to filter **scoped rule blocks**.

## Scoped rule block
A section inside an included markdown file that only appears in the compiled prompt when the stage’s `work_level` matches.

Syntax:

```md
<!-- SCOPE: L2,L3 -->
This content only appears at L2/L3.
<!-- END_SCOPE -->
```

This is implemented today by the harness during compilation.

## Front matter
The YAML block at the top of a stage markdown file, wrapped by `---` lines.
The harness parses it to determine:
- `includes` (what gets inlined)
- `inputs` (what is provided to the model)
- `outputs` (what the model must produce)
- optional `activation` rules

## Includes
A list of files to inline into the compiled prompt.
Used for:
- core rules (`core/rules/*.md`)
- a runner module (`runners/<runner>.md`)
- profile guidance (`profiles/<profile>/*`)

Includes support simple variable substitution in paths, e.g. `runners/${runner}.md`.

## Inputs
The things a stage provides to the model. In practice, the compiled prompt shows them as:
- **Run Variables** (resolved values)
- **Library Inputs** (directives/templates)
- **Artifact Inputs** (previously generated outputs)

Front matter keys used today:
- `inputs.library`
- `inputs.artifacts`
- `inputs.variables`

## Library input
A directive or template the model must follow (usually under `core/library/…`).
Examples:
- `core/library/charter/charter_gen_directive.md` (directive)
- `core/library/charter/charter.md.tmpl` (template)

Library inputs are inlined into the compiled prompt.

## Directive
A library markdown file that tells the LLM how to behave (interview vs synthesis, what to ask, output contract).

Directives are *content generators*, not stage definitions.
Stages reference directives via `inputs.library`.

## Template
A “.tmpl” markdown file that defines the expected output shape (placeholders + section structure).
Templates are inlined into prompts so the model can fill them.

## Profile
A “stack pack” directory under `profiles/<id>/` that keeps the core system language/tooling agnostic.

A profile contains:
- `profile.yaml` — metadata + default assumptions
- `commands.yaml` — canonical command strings keyed by gate name
- `conventions.md` — naming/style/layout conventions

Profiles are included into prompts so the model can reference tooling without hardcoding it into core rules.

### Profile command keys
The stable *names* used to reference commands in prompts.
Example keys in a Python profile might include:
- `commands.tests`
- `commands.lint`
- `commands.typecheck`

The point is: core prompts say “run `commands.tests`” rather than “run pytest …”.

## Runner
A guidance module under `runners/` describing how an execution agent should operate.
Example: `runners/codex-cli.md`.

Runners describe:
- how to run commands
- how to show evidence
- how to format output (single document vs multi‑file blocks)

## Overlay
An optional policy module under `core/overlays/` (including subdirectories) that can be injected into compiled prompts.

Overlays can be included:
- explicitly via `--overlays overlay_a,overlay_b`, or
- automatically (today: `quality/complexity_assessment` is auto‑included when `enable_complexity=true` when available)

Overlays are meant to be additive; they should not redefine core rules.

## Harness
The copy/paste orchestration tool under `tools/harness.py` (with a small shell wrapper `tools/harness.sh`).

Two key behaviors that work today:
- **compile** — writes `dist/<stage_id>.md`
- **capture** — reads model output from stdin and writes declared outputs

The harness is intentionally not an LLM client.

Related:
- **Compiled prompt**
- **State file**

## Compiled prompt
A single markdown file written under `dist/` that represents *everything the model needs for that stage*:
- stage header
- run variables
- includes/rules/runner/profile
- library inputs (directive + templates)
- artifact inputs (upstream outputs)
- outputs list and gating notes

This is what you paste into your LLM.

## Dist
`system/dist/` — the directory containing compiled prompts (generated by the harness).

## Capture
A harness operation that reads the model’s output from stdin and writes files to the stage’s declared outputs.

## Output contract
The rule that the model must output in the exact format required for capture.

Two formats are supported today:
- **single‑file**: output only the document content
- **multi‑file**: output `--- FILE: <path> ---` blocks

## FILE blocks (multi‑file output)
A deterministic wrapper format for multi‑artifact stages (e.g., Foundation Pack).

Example:

```md
--- FILE: artifacts/foundation/FOUNDATION_STRATEGY.md ---
<contents>

--- FILE: artifacts/foundation/quality_gates.yaml ---
<contents>
```

The harness splits the pasted output and writes each declared file.

## Artifacts
Generated files written under `system/artifacts/...`.
Artifacts are the pipeline’s output ledger.

Examples:
- `artifacts/charter/CHARTER.md`
- `artifacts/project_context/PROJECT_CONTEXT.md`
- `artifacts/foundation/QUALITY_GATES_SPEC.md`

## Repo files
Stage outputs written into the project repo itself (usually `${repo_root}/...`) via `outputs.repo_files`.

Example:
- `${repo_root}/CHARTER.md`
- `${repo_root}/ENVIRONMENT_INVENTORY.md`

### Repo file copy fallback
If a stage declares a repo file but the model only produced the artifact copy, the harness can copy by basename.
Example: copy `artifacts/foundation/ENVIRONMENT_INVENTORY.md` → `${repo_root}/ENVIRONMENT_INVENTORY.md`.

## State file
`artifacts/_harness_state.yaml` — a YAML file created/updated by the harness to remember:
- selected runner/profile
- run variables like `project_name`, `repo_or_project_ref`
- stage‑set booleans like `needs_project_context`

Variable precedence today:
1) CLI flags
2) state file
3) pipeline defaults

## sets
A pipeline mechanism (in `pipeline.yaml`) that tells the harness:
“after capturing this stage, prompt the user to set these variables.”

Used today by the Charter stage to set:
- `needs_project_context`

## Activation
A stage front matter condition that determines whether the stage runs.

Example used today:
- `stage.06_project_context_interview` runs only when `variables.needs_project_context == true`.

Reduced v1 Rust pipeline loading supports boolean equality checks only.
Legacy harness reference material may describe broader activation forms, but that is not the current Rust compiler contract.

## enable_complexity
A boolean run variable that triggers auto‑inclusion of the `quality/complexity_assessment` overlay (when present) during compilation.

## needs_project_context
A boolean that determines whether the optional Project Context stage should run.
It is typically set after capturing the Charter stage (via `sets`).

## test_mode
An optional harness variable (when enabled) used to swap certain directives for test/synthetic variants.

Today, the intended usage is:
- for the Charter stage, use a “test mode” directive that generates a realistic synthetic charter without questions

(If your harness has been patched to support `--test-mode`, this is active. If not, treat it as a manual stage file swap.)

---

# Canonical generated documents (working today)

## BASE_CONTEXT.md
Produced by `stage.00_base`.
A small baseline record of run metadata and expected artifact locations.

## CHARTER.md
Produced by `stage.05_charter_interview`.
Defines posture/standards (“how we decide”) across dimensions like testing/security/reliability.

## PROJECT_CONTEXT.md
Produced by `stage.06_project_context_interview` (optional).
Defines project reality (“what exists / what’s live / constraints”) to prevent invented migrations/back‑compat, etc.

## Foundation Pack
Produced by `stage.07_foundation_pack`.
A bundle of artifacts derived from Charter (+ optional Project Context):
- `FOUNDATION_STRATEGY.md`
- `TECH_ARCH_BRIEF.md`
- `TEST_STRATEGY_BRIEF.md`
- `QUALITY_GATES_SPEC.md`
- `quality_gates.yaml`
- `ENVIRONMENT_INVENTORY.md`

## ENVIRONMENT_INVENTORY.md
Produced by `stage.07_foundation_pack`.
Canonical store of record for env vars/services/ports/runtime assumptions.

Canonical location:
- `${repo_root}/ENVIRONMENT_INVENTORY.md`
Pipeline copy:
- `artifacts/foundation/ENVIRONMENT_INVENTORY.md`

## FEATURE_SPEC.md
Produced by `stage.10_feature_spec`.
A per‑feature contract aligned with Charter + (optionally) the Foundation Pack.

---

## Work Catalog
A machine-readable list of work items (features/bugs/chores) used to keep planning grounded.

- Canonical location (recommended): `${repo_root}/backlog/WORK_CATALOG.yaml`
- Template: `core/library/work_catalog/WORK_CATALOG.yaml.tmpl`

Release and sprint planning stages should reference **only** IDs in this catalog (or explicitly ask for a minimal list).

## Release
A bounded queue of work intended to ship together, often spanning multiple sequential sprints.

Artifacts:
- `artifacts/releases/<release_id>/RELEASE_PLAN.md` (human)
- `artifacts/releases/<release_id>/release.yaml` (machine)

Related:
- **Sprint slot**
- **Sprint**

## Sprint slot
A sequential container inside a release (e.g., `slot-1`, `slot-2`) describing intent:
- slot goal
- focus work item IDs
- required task types

Sprint planning uses the slot as the starting point and refines it into concrete tasks.

## Sprint
A sequential execution window that advances a release slot by producing concrete deliverables.

Artifacts:
- `artifacts/sprints/<sprint_id>/SPRINT_PLAN.md`
- `artifacts/sprints/<sprint_id>/sprint.yaml`
- `artifacts/sprints/<sprint_id>/tasks.yaml`

## tasks.yaml (Sprint tasks)
A machine-readable task list for a sprint. Tasks are typed so they can be validated and closed deterministically.

Key idea: **gates are tasks** (e.g., `planning_gate`, `integration_gate`).

## Lane
An optional grouping mechanism inside a sprint that clusters tasks by workstream.

When the bounded lanes overlay is active (`sprint/bounded_sprints_lanes`):
- a sprint defines 2–5 lanes
- each task belongs to exactly one lane

## Multiple pipelines
The system can support multiple pipeline definitions (e.g., foundation vs release vs sprint workflows).

Today, the harness can be pointed at a specific pipeline YAML using `--pipeline`.
