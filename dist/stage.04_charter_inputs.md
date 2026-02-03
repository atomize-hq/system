# stage.04_charter_inputs — Charter Inputs (Dev/Test) â CHARTER_INPUTS.yaml

Generates a stable CHARTER_INPUTS.yaml fixture for testable charter generation. This is a development/testing path that avoids multi-turn interviewing.


## Run Variables

- charter_gaps_detected: False
- charter_ref: artifacts/charter/CHARTER.md
- enable_complexity: False
- needs_project_context: False
- now_utc: 2026-01-28T18:35:10Z
- owner: 
- prev_sprint_id: sprint-000
- profile: python-uv
- project_context_ref: artifacts/project_context/PROJECT_CONTEXT.md
- project_name: InputsFlow
- release_id: release-001
- release_type: minor
- repo_or_project_ref: local/system
- repo_root: artifacts/_repo_root_test2
- runner: codex-cli
- sprint_id: sprint-001
- sprint_slot: slot-1
- team: 
- test_mode: True
- work_level: L1


## Selected Runner

### runners/codex-cli.md

# Runner: codex-cli

This runner assumes the agent can:
- run shell commands in the repo
- read/write files
- show command evidence (cmd, exit code, tail)

## Command execution rules
- Use the selected profile’s command keys (e.g., `commands.tests`) as the source of truth.
- Capture evidence for every gate you claim passed (see Evidence Policy).

## File edits
- Prefer minimal diffs.
- Keep changes within slice scope.
- If a change introduces/changes env vars, services, ports, or runtime assumptions:
  - update `ENVIRONMENT_INVENTORY.md` at the repo/project root (the repo root for this run)
  - keep the pipeline artifact copy in sync when applicable (`artifacts/foundation/ENVIRONMENT_INVENTORY.md`)

## Output discipline
- For “document stages”, output only the document content (no extra commentary).
- For multi-artifact stages, output `--- FILE: <path> ---` blocks exactly.


## Selected Profile

### profiles/python-uv/profile.yaml

kind: profile
id: python-uv
version: 0.1.0
title: "Python (uv)"
description: >
  Python projects managed with uv. Assumes ruff for lint/format, mypy for typing, pytest for tests.

compatibility:
  languages: ["python"]
  package_managers: ["uv"]
  runners: ["codex-cli", "cursor", "claude-tools", "plain-chat"]

project_defaults:
  python_requires: ">=3.11"
  code_dirs: ["src"]
  test_dirs: ["tests"]
  config_files:
    - pyproject.toml
    - uv.lock

gates:
  required: ["format", "lint", "tests"]
  optional: ["typecheck", "security", "docs", "complexity", "coverage"]

evidence:
  required_fields: ["cmd", "exit", "tail", "timestamp"]
  tail_lines: 80

### profiles/python-uv/commands.yaml

commands:
  install: "uv sync"
  format: "uv run ruff format {code_dirs} {test_dirs}"
  format_check: "uv run ruff format --check {code_dirs} {test_dirs}"
  lint: "uv run ruff check {code_dirs} {test_dirs}"
  typecheck: "uv run mypy {code_dirs}"
  tests: "uv run pytest -q"
  coverage: "uv run pytest --cov={code_dirs} --cov-report=term-missing --cov-fail-under={coverage_min}"
  docs: "uv run ruff check --select D {code_dirs}"
  security: "uv run pip-audit && uv run bandit -r {code_dirs}"
  complexity: "uv run ruff check --select C90 {code_dirs}"

### profiles/python-uv/conventions.md

# Profile conventions: Python (uv)

These are **project/tooling conventions** for Python projects managed with **uv**.
Core system prompts should not hardcode any of the commands below; they should reference
**command keys** from `commands.yaml` (e.g., `commands.tests`).

## Layout assumptions
- Preferred code root: `src/` (unless repo already uses a different layout)
- Preferred tests root: `tests/`
- Keep module depth shallow; prefer composition.

## Style
- Formatting: `ruff format` (Black-compatible)
- Linting: `ruff check`
- Docstrings: Google-style recommended; enforce via Ruff `D` rules if enabled.

## Typing
- Use type annotations for public APIs and non-trivial functions.
- Prefer `mypy` (strict posture for new modules if feasible).
- Avoid blanket `# type: ignore` (allowed only with justification and minimal scope).

## Testing
- Use `pytest`.
- New modules: target 100% coverage (Charter/Foundation Pack may override).
- Prefer unit tests for business logic; integration tests only when crossing boundaries.

## Evidence capture (runner-agnostic)
When you run a command, capture:
- command line
- exit code
- last ~80 lines of stdout/stderr
- timestamp (UTC)

## Placeholders used in commands.yaml
- `{code_dirs}`: typically `src`
- `{test_dirs}`: typically `tests`
- `{coverage_min}`: numeric threshold (e.g., 90.0)


## Includes

### core/rules/p0_absolute.md

# P0 Absolute Rules (Scoped)

These rules are **non-negotiable** across languages, stacks, and runners.

Some P0 rules are scoped by **work level** so you can run parallel workstreams
at higher planning levels without creating execution drift.

## Work levels (hierarchy)

- **L0 Program**: roadmap/portfolio decisions and sequencing. Parallelism is normal.
- **L1 Project/Planning**: charter, project context, foundation, feature specs, phase plans. Parallelism is normal.
- **L2 Slice Execution**: implementing *one* slice in a single working context (worktree/branch). Keep execution single-threaded.
- **L3 Quality Gate & Merge**: final verification/integration/merge discipline.

### Scoped rule blocks

This ruleset supports optional scoped blocks in the form:

- `<!-- SCOPE: L2,L3 -->` (start)
- `<!-- END_SCOPE -->` (end)

Only include scoped blocks when the current stage `work_level` matches.

## P0-ABSOLUTE: Safety & Security
- **Never compromise security** for speed or convenience.
- **Never introduce secret leakage**:
  - do not hardcode credentials/tokens/keys
  - do not print secrets in logs
  - prefer secure defaults

## P0-ABSOLUTE: Truthfulness & Evidence
- **No evidence, no pass**: do not claim tests/lint/typecheck/security passed unless you have evidence.
- If you cannot run commands (no tools), you must:
  - state that you did not run them
  - provide the exact command(s) to run (from the selected profile)

## P0-ABSOLUTE: Scope & Determinism
- Do only what the current slice/stage requires.
- If requirements are ambiguous and would change behavior, **ask** (do not guess).
- Prefer deterministic, machine-checkable outputs:
  - avoid vague acceptance criteria
  - prefer explicit checks and exit-code based gates

## P0-ABSOLUTE: Change Discipline
- Keep changes minimal and reversible when touching contracts or production systems.
- If a change introduces/changes env vars, services, ports, or runtime assumptions:
  - update `ENVIRONMENT_INVENTORY.md` **at the project root** (repo root), in the same change.
    - canonical: `${repo_root}/ENVIRONMENT_INVENTORY.md`
    - pipeline/system artifact copy: `artifacts/foundation/ENVIRONMENT_INVENTORY.md`



## P0-ABSOLUTE: Output Contract
- When a stage/template requires “output only the final document”, do not add extra commentary.
- When a stage requires multi-file output blocks, follow the wrapper format exactly.

### core/rules/p1_pragmatic.md

# P1 Pragmatic Rules (Universal)

These rules are strong defaults intended to prevent over-engineering.

## P1-PRAGMATIC: Prefer Existing Helpers
- Use existing project utilities and patterns before inventing new ones.
- Prefer small, composable functions and modules.

## P1-PRAGMATIC: Don’t Over-Build
- Don’t add “frameworks for the future” unless required by the Charter/Foundation posture.
- Optimize only when you have measurements or explicit requirements.

## P1-PRAGMATIC: Tests Should Match Complexity
- Simple logic → unit tests.
- Boundary crossings (IO/network/db) → integration tests.
- Avoid testing the standard library or trivial plumbing.

## P1-PRAGMATIC: Keep Docs Lean
- Prefer short checklists and explicit contracts over long prose.
- Avoid restating upstream docs; reference them instead.

## P1-PRAGMATIC: Bias to Clarity
- Use explicit naming, stable IDs, and traceability links.
- Keep artifacts readable by both humans and machines.

### core/rules/traceability_policy.md

# Traceability Policy

Traceability prevents drift between roadmap → feature spec → slices → execution.

## Stable IDs
- Every feature spec must have a stable Spec ID (e.g., `FS-2026-001`).
- Slices must have stable IDs (e.g., `S0.1`, `S1.2`).
- Quality gate reports should reference the slice ID they validate.

## Upstream anchors
Artifacts should reference upstream sources, not paraphrase them:
- Feature spec references Charter + Foundation Pack outputs.
- Slice plan references Feature spec sections and acceptance criteria IDs.
- Execution report references Slice ID and includes evidence log pointers.

## Drift prevention
If downstream artifacts introduce requirements not present upstream:
- mark them explicitly as “new requirement”
- link to the decision source (Charter exception / ADR / stakeholder approval)

### runners/codex-cli.md

# Runner: codex-cli

This runner assumes the agent can:
- run shell commands in the repo
- read/write files
- show command evidence (cmd, exit code, tail)

## Command execution rules
- Use the selected profile’s command keys (e.g., `commands.tests`) as the source of truth.
- Capture evidence for every gate you claim passed (see Evidence Policy).

## File edits
- Prefer minimal diffs.
- Keep changes within slice scope.
- If a change introduces/changes env vars, services, ports, or runtime assumptions:
  - update `ENVIRONMENT_INVENTORY.md` at the repo/project root (the repo root for this run)
  - keep the pipeline artifact copy in sync when applicable (`artifacts/foundation/ENVIRONMENT_INVENTORY.md`)

## Output discipline
- For “document stages”, output only the document content (no extra commentary).
- For multi-artifact stages, output `--- FILE: <path> ---` blocks exactly.

### profiles/python-uv/conventions.md

# Profile conventions: Python (uv)

These are **project/tooling conventions** for Python projects managed with **uv**.
Core system prompts should not hardcode any of the commands below; they should reference
**command keys** from `commands.yaml` (e.g., `commands.tests`).

## Layout assumptions
- Preferred code root: `src/` (unless repo already uses a different layout)
- Preferred tests root: `tests/`
- Keep module depth shallow; prefer composition.

## Style
- Formatting: `ruff format` (Black-compatible)
- Linting: `ruff check`
- Docstrings: Google-style recommended; enforce via Ruff `D` rules if enabled.

## Typing
- Use type annotations for public APIs and non-trivial functions.
- Prefer `mypy` (strict posture for new modules if feasible).
- Avoid blanket `# type: ignore` (allowed only with justification and minimal scope).

## Testing
- Use `pytest`.
- New modules: target 100% coverage (Charter/Foundation Pack may override).
- Prefer unit tests for business logic; integration tests only when crossing boundaries.

## Evidence capture (runner-agnostic)
When you run a command, capture:
- command line
- exit code
- last ~80 lines of stdout/stderr
- timestamp (UTC)

## Placeholders used in commands.yaml
- `{code_dirs}`: typically `src`
- `{test_dirs}`: typically `tests`
- `{coverage_min}`: numeric threshold (e.g., 90.0)

### profiles/python-uv/profile.yaml

kind: profile
id: python-uv
version: 0.1.0
title: "Python (uv)"
description: >
  Python projects managed with uv. Assumes ruff for lint/format, mypy for typing, pytest for tests.

compatibility:
  languages: ["python"]
  package_managers: ["uv"]
  runners: ["codex-cli", "cursor", "claude-tools", "plain-chat"]

project_defaults:
  python_requires: ">=3.11"
  code_dirs: ["src"]
  test_dirs: ["tests"]
  config_files:
    - pyproject.toml
    - uv.lock

gates:
  required: ["format", "lint", "tests"]
  optional: ["typecheck", "security", "docs", "complexity", "coverage"]

evidence:
  required_fields: ["cmd", "exit", "tail", "timestamp"]
  tail_lines: 80

### profiles/python-uv/commands.yaml

commands:
  install: "uv sync"
  format: "uv run ruff format {code_dirs} {test_dirs}"
  format_check: "uv run ruff format --check {code_dirs} {test_dirs}"
  lint: "uv run ruff check {code_dirs} {test_dirs}"
  typecheck: "uv run mypy {code_dirs}"
  tests: "uv run pytest -q"
  coverage: "uv run pytest --cov={code_dirs} --cov-report=term-missing --cov-fail-under={coverage_min}"
  docs: "uv run ruff check --select D {code_dirs}"
  security: "uv run pip-audit && uv run bandit -r {code_dirs}"
  complexity: "uv run ruff check --select C90 {code_dirs}"


## Library Inputs

### core/library/charter/charter_inputs_directive.md

You are an AI assistant helping produce `CHARTER_INPUTS.yaml`, a **stable, testable** input file used to generate `CHARTER.md` without an interview loop.

## Goal

Output a single YAML document that matches the provided template.

This stage is for **development/testing** of the system:
- It avoids multi-turn interviews.
- It produces deterministic-ish fixtures that the next stage can consume.

## Rules

- **Do not ask questions.**
- **Do not output any prose.**
- Output **only** the YAML content for `CHARTER_INPUTS.yaml`.
- If a value is unknown, use an empty string, empty list, or `null` (do not write "TBD").
- Keep lists short and crisp (prefer 2–5 bullets max per list).

## Profile-aware behavior

A profile pack may be included (e.g., `profiles/python-uv/profile.yaml` + `commands.yaml`).

- Use the selected profile as the default tooling assumption.
- Do not invent unrelated tooling choices that contradict the profile.

## Required output

Emit a complete `CHARTER_INPUTS.yaml` matching the template structure.

### core/library/charter/CHARTER_INPUTS.yaml.tmpl

# CHARTER_INPUTS.yaml (template)
#
# Purpose: Stable, testable inputs for generating CHARTER.md without an interview loop.
# - Keep values small and explicit.
# - Prefer selecting from enumerations over free-form prose.
# - Use this file as a fixture during system development.
#
# Notes:
# - `baseline_level` is the default for all dimensions unless overridden.
# - `dimensions[*].level` is optional; if omitted, baseline_level applies.
# - For triggers/shortcuts/red_lines: keep bullets crisp (if X then Y).
#
schema_version: 0.1.0

project:
  name: "{{PROJECT_NAME}}"
  classification: greenfield # greenfield|brownfield|integration|modernization|hardening
  team_size: 1
  users: internal # internal|external|mixed
  expected_lifetime: "months" # days|weeks|months|years
  surfaces: ["api"] # e.g. ["web_app","api","cli","lib","infra","ml"]
  runtime_environments: ["server"] # e.g. ["browser","server","cloud","on_prem","edge"]
  constraints:
    deadline: ""
    budget: ""
    experience_notes: ""
    must_use_tech: []
  operational_reality:
    in_production_today: false
    prod_users_or_data: ""
    external_contracts_to_preserve: []
    uptime_expectations: ""
  default_implications:
    backward_compatibility: "not_required" # required|not_required|boundary_only
    migration_planning: "not_required" # required|not_required
    rollout_controls: "lightweight" # none|lightweight|required
    deprecation_policy: "not_required_yet" # required|not_required_yet
    observability_threshold: "standard" # minimal|standard|high|regulated

posture:
  rubric_scale: "1-5"
  baseline_level: 3
  baseline_rationale:
    - ""

domains: []
# domains:
#   - name: "auth"
#     blast_radius: "high"
#     touches: ["pii","money","external_users"]
#     constraints: []

dimensions:
  - name: "speed_vs_quality"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

  - name: "type_safety_static_analysis"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

  - name: "testing_rigor"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

  - name: "scalability_performance"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

  - name: "reliability_operability"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

  - name: "security_privacy"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

  - name: "observability"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

  - name: "dx_tooling_automation"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

  - name: "ux_polish_api_usability"
    level: null
    default_stance: ""
    raise_the_bar_triggers: []
    allowed_shortcuts: []
    red_lines: []
    domain_overrides: []

exceptions:
  approvers: ["project_owner"]
  record_location: "CHARTER.md#exceptions"
  minimum_fields: ["what", "why", "scope", "risk", "owner", "expiry_or_revisit_date"]

debt_tracking:
  system: "issues"
  labels: ["debt"]
  review_cadence: "monthly"

decision_records:
  enabled: true
  path: "docs/decisions"
  format: "md"


## Artifact Inputs

(none)


## Outputs


### Artifacts

- artifacts/charter/CHARTER_INPUTS.yaml


### Repo Files

(none declared)


## Gating Notes

- Final response must be ONLY the YAML content for CHARTER_INPUTS.yaml.


## Stage Body

<!-- Stage body intentionally minimal. -->
