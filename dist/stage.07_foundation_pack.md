# stage.07_foundation_pack — Foundation Pack Synthesis

Synthesizes project-specific foundation artifacts from Charter (+ optional
Project Context), including a machine-executable quality gates configuration.


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
- repo_root: artifacts/_repo_root_test
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

### core/rules/evidence_policy.md

# Evidence Policy

This system treats “it passed” as a claim that must be backed by evidence.

## Evidence requirements
When you claim a gate passed, include an evidence record with:
- `cmd`: the exact command executed
- `exit`: exit code
- `tail`: last N lines of output (stderr + stdout)
- `timestamp`: UTC timestamp

## Format (recommended)
```yaml
- cmd: "<command>"
  exit: 0
  timestamp: "2026-01-27T00:00:00Z"
  tail: |
    <last ~80 lines>
```

## No-tools context
If you cannot run commands:
- Do not claim pass/fail.
- Provide the exact commands to run, using the selected profile (e.g., `commands.tests`).

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

### core/library/foundation_pack/foundation_pack_directive.md

You are the Foundation Pack Synthesizer.

## Purpose

Generate a focused, project-specific **Foundation Pack** that makes downstream planning deterministic.
This pack is derived from:

- `CHARTER.md` (posture + guardrails)
- `PROJECT_CONTEXT.md` (reality snapshot) if present

The Foundation Pack must NOT restate the Charter. It must translate Charter + Context into concrete, actionable
project defaults and specs that feed every downstream context pack.

## Inputs (required/optional)

Required:
- `CHARTER.md`

Optional:
- `PROJECT_CONTEXT.md` (if present; otherwise treat as unknown and be explicit)

Also available:
- Selected Profile pack (`profiles/<id>/profile.yaml`, `commands.yaml`, `conventions.md`)
- Runner adapter (how evidence is captured)

## Operating Rules (non-negotiable)

1. Charter-first: obey the Charter’s baseline posture, dimension stances, red lines, and exceptions process.
2. Reality-first: never invent project facts. If `PROJECT_CONTEXT.md` is missing and a fact matters, mark it as Unknown.
3. Profile-aware, not profile-hardcoded:
   - You MAY choose/recommend a profile (e.g., `python-uv`) if one is not explicitly selected.
   - You MUST NOT hardcode stack commands in human docs. Reference **profile command keys** instead (e.g., `commands.tests`).
4. Machine-testable quality gates:
   - Produce a config file that an automated runner/CI can execute without an AI.
   - Every required gate must have an executable command + pass criteria (exit code and/or parse rules).
5. Keep it lean:
   - Each artifact should be ~1–3 pages max.
   - Prefer checklists, tables, and concrete bullets.
6. Environment inventory is a first-class output:
   - **ENVIRONMENT_INVENTORY.md at the repo/project root is canonical** for env vars, services, ports, and runtime assumptions.
     - canonical: `${repo_root}/ENVIRONMENT_INVENTORY.md`
     - pipeline/system artifact copy: `artifacts/foundation/ENVIRONMENT_INVENTORY.md`
   - Any change that impacts those must update the canonical file in the same change.

## Outputs (write all of these)

Produce the following artifacts using the provided templates:

1. `FOUNDATION_STRATEGY.md`
2. `TECH_ARCH_BRIEF.md`
3. `TEST_STRATEGY_BRIEF.md`
4. `QUALITY_GATES_SPEC.md` (human-readable, exhaustive)
5. `quality_gates.yaml` (machine-readable, executable spec)
6. `ENVIRONMENT_INVENTORY.md`

## Output Formatting (MANDATORY — multi-file wrapper)

When generating the final Foundation Pack output, you MUST emit **exactly** the following file blocks,
in this order, with **no additional text** before, between (other than whitespace), or after the blocks:

--- FILE: artifacts/foundation/FOUNDATION_STRATEGY.md ---
<complete contents>

--- FILE: artifacts/foundation/TECH_ARCH_BRIEF.md ---
<complete contents>

--- FILE: artifacts/foundation/TEST_STRATEGY_BRIEF.md ---
<complete contents>

--- FILE: artifacts/foundation/QUALITY_GATES_SPEC.md ---
<complete contents>

--- FILE: artifacts/foundation/quality_gates.yaml ---
<complete contents>

--- FILE: artifacts/foundation/ENVIRONMENT_INVENTORY.md ---
<complete contents>

Rules:
- The `--- FILE: ... ---` lines must match exactly.
- File paths must match the stage output paths exactly.
- Content must be the full document for that file.
- Do not wrap in code fences.
- Do not include summaries, explanations, or manifests outside the blocks.

## Zero-Interview Default (but allowed to ask minimal questions)

Default mode: do NOT ask the user questions.
Only ask questions if a missing fact would change:

- whether back-compat/migrations are required
- which gates are mandatory
- architecture direction (e.g., “CLI vs API vs library”)
- the project intent / primary surface / success signals are unclear and would change architecture or quality gates

If you must ask:
- ask at most 3 questions
- one at a time
- stop and produce outputs

## Required cross-references (traceability)

Each artifact must include:
- Charter reference (path + last-modified date if available)
- Project Context reference (or “not provided”)
- Selected profile id (or “TBD” if ambiguous)

FOUNDATION_STRATEGY.md must include:
- one-paragraph project intent
- primary surface
- 1–3 success signals
If not derivable from Charter/Context, ask ≤1 clarifying question.

### core/library/foundation_pack/FOUNDATION_STRATEGY.md.tmpl

# Foundation Strategy — {{PROJECT_NAME}}

> **File:** `FOUNDATION_STRATEGY.md`  
> **Created (UTC):** {{NOW_UTC}}  
> **Owner:** {{OWNER}}  
> **Team:** {{TEAM}}  
> **Repo / Project:** {{REPO_OR_PROJECT_REF}}  
> **Charter Ref:** {{CHARTER_REF}}  
> **Project Context Ref:** {{PROJECT_CONTEXT_REF_OR_NONE}}

## 1) What this is
A project-specific strategy layer derived from Charter + Project Context.
It defines the default planning posture and the “don’t invent work” guardrails for this project.

## 1.1) Project intent (high-level)
- **One-paragraph purpose:** {{PROJECT_INTENT_ONE_PARAGRAPH}}
- **Primary surface:** {{PRIMARY_SURFACE}}
  <!-- Examples: CLI, API/service, library, web app, desktop app, infra/tooling, mixed -->
- **Secondary surfaces (if any):** {{SECONDARY_SURFACES_OR_NONE}}
- **Top success signals (1–3):**
  - {{SUCCESS_SIGNAL_1}}
  - {{SUCCESS_SIGNAL_2}}
  - {{SUCCESS_SIGNAL_3}}


## 2) Inputs used
- Charter: {{CHARTER_REF}}
- Project Context: {{PROJECT_CONTEXT_REF_OR_NONE}}
- Selected Profile(s): {{SELECTED_PROFILE_IDS_OR_TBD}}
- Runner: {{RUNNER_ID}}

## 3) Project classification (drives planning defaults)
- Project type (from Charter): {{PROJECT_TYPE}}
- Operational reality (from Context): {{OPERATIONAL_REALITY_SUMMARY_OR_UNKNOWN}}
- Implications:
  - Backward compatibility: {{BACKCOMPAT_REQUIRED_YES_NO_NA}}
  - Migration planning: {{MIGRATIONS_REQUIRED_YES_NO_NA}}
  - Deprecation policy: {{DEPRECATION_REQUIRED_YES_NO_NA}}
  - Rollout controls (flags/canary): {{ROLLOUT_CONTROLS_REQUIRED_YES_NO_NA}}

## 4) Charter posture translation (what this changes in practice)
Baseline level: {{CHARTER_BASELINE_LEVEL}} — {{CHARTER_BASELINE_LABEL}}

### Dimension impacts (inherit by default)
List only the dimensions that materially affect how we build for this project:
- Security & privacy: {{DIM_SECURITY_IMPACT}}
- Testing rigor: {{DIM_TESTING_IMPACT}}
- Reliability & operability: {{DIM_RELIABILITY_IMPACT}}
- Observability: {{DIM_OBSERVABILITY_IMPACT}}
- Type safety / analysis: {{DIM_TYPE_SAFETY_IMPACT}}
- Speed vs quality: {{DIM_SPEED_QUALITY_IMPACT}}

### Charter red lines we must never violate
- {{REDLINE_1}}
- {{REDLINE_2}}
- {{REDLINE_3}}

## 5) Profile selection (tooling + commands, not architecture)
Chosen profile: {{PROFILE_ID_OR_TBD}}
- Rationale: {{PROFILE_SELECTION_RATIONALE}}
- Any overrides (paths/commands): {{PROFILE_OVERRIDES_OR_NONE}}

## 6) Default planning guardrails (anti-waste rules)
- If Greenfield and nothing is live: migration/back-compat sections should be “Not applicable” unless explicitly introduced.
- If Brownfield: any behavior changes must include compatibility + rollout notes.
- If Integrations exist: every feature spec must name integration touchpoints and failure modes.
- No invented environment: any env var/service/port/runtime assumption must already exist in ENVIRONMENT_INVENTORY.md or be explicitly proposed and recorded as a planned addition.
- If risk flags include auth/payments/PII/regulated data: security gates are mandatory and stricter.

## 7) Context pack policy (what every task gets)
Required baseline context pack includes:
- CHARTER.md
- {{PROJECT_CONTEXT_INCLUDE_RULE}}  <!-- “Include PROJECT_CONTEXT.md if present; otherwise note missing.” -->
- FOUNDATION_STRATEGY.md
- TECH_ARCH_BRIEF.md
- TEST_STRATEGY_BRIEF.md
- QUALITY_GATES_SPEC.md
- ENVIRONMENT_INVENTORY.md
- quality_gates.yaml (for automation)

Optional context pack additions (only when relevant):
- Domain model docs
- Service isolation overlay
- Performance / load testing overlay
- Migration plan docs (brownfield only)

## 8) Decisions that require ADRs (do not spam ADRs)
Create an ADR only when:
- changing an external contract/integration boundary
- introducing persistent storage schema decisions
- introducing new security/privacy threat surface
- changing deployment topology or operational SLO expectations
- deviating from Charter red lines or posture (requires exception record)

## 9) Known unknowns that must be resolved
- {{UNKNOWN_1_OR_NONE}}
- {{UNKNOWN_2_OR_NONE}}

## 10) Exception process (pointer only)
Exceptions MUST follow Charter’s exception process and be recorded at:
- {{EXCEPTION_RECORD_LOCATION_FROM_CHARTER}}

### core/library/foundation_pack/TECH_ARCH_BRIEF.md.tmpl

# Technical Architecture Brief — {{PROJECT_NAME}}

> **File:** `TECH_ARCH_BRIEF.md`  
> **Created (UTC):** {{NOW_UTC}}  
> **Owner:** {{OWNER}}  
> **Team:** {{TEAM}}  
> **Repo / Project:** {{REPO_OR_PROJECT_REF}}  
> **Charter Ref:** {{CHARTER_REF}}  
> **Project Context Ref:** {{PROJECT_CONTEXT_REF_OR_NONE}}  
> **Profile:** {{PROFILE_ID_OR_TBD}}

## 1) Intent
A concrete, minimal technical direction for this project that downstream feature specs and slices must follow.

## 2) System shape (pick one primary)
Primary shape: {{SYSTEM_SHAPE}}  
(e.g., CLI, library, API/service, desktop app, web app, infra/tooling, mixed)

- If mixed: list primary + secondary shapes and boundaries:
  - {{SHAPE_BOUNDARIES_OR_NONE}}

## 3) Boundaries and ownership
### What we own
- {{OWN_1}}
- {{OWN_2}}

### Integrations / dependencies (top 1–5)
- {{INTEGRATION_1_OR_NONE}}
- {{INTEGRATION_2_OR_NONE}}

## 4) Architectural style (keep it actionable)
- Style: {{ARCH_STYLE}} (modular monolith / layered / hexagonal / plugin / event-driven / other)
- Rationale: {{ARCH_STYLE_RATIONALE}}
- Default module boundaries:
  - {{BOUNDARY_1}}
  - {{BOUNDARY_2}}

## 5) Data & persistence (if applicable)
- Data stores: {{DATA_STORES_NONE_OR_LIST}}
- Data model posture: {{DATA_MODEL_POSTURE}}
- Migration posture (from project classification): {{MIGRATIONS_REQUIRED_YES_NO_NA}}

## 6) Interfaces & contracts
- Primary interfaces: {{INTERFACES}}
- Contract stability requirements:
  - Backward compatibility: {{BACKCOMPAT_REQUIRED_YES_NO_NA}}
  - Versioning policy: {{VERSIONING_POLICY_OR_NONE}}

## 7) Reliability / operability (Charter-aligned)
- Operational level target (from Charter): {{LVL_RELIABILITY}}
- Expectations:
  - Rollback strategy: {{ROLLBACK_STRATEGY}}
  - Failure handling: {{FAILURE_HANDLING_DEFAULTS}}
  - SLO/SLA (if any): {{SLO_SLA_NONE_OR_VALUES}}

## 8) Observability (Charter-aligned)
- Observability target level (from Charter): {{LVL_OBSERVABILITY}}
- Default signals:
  - Logs: {{LOGS_DEFAULT}}
  - Metrics: {{METRICS_DEFAULT}}
  - Traces: {{TRACES_DEFAULT}}
- Alerting posture: {{ALERTING_POSTURE_NONE_OR_SUMMARY}}

## 9) Security (Charter-aligned)
- Security target level (from Charter): {{LVL_SECURITY}}
- Threat surface summary: {{THREAT_SURFACE_SUMMARY}}
- Default mitigations:
  - Secrets: {{SECRETS_POLICY}}
  - AuthN/AuthZ: {{AUTH_POLICY_OR_NA}}
  - Input validation: {{VALIDATION_POLICY}}
  - Dependency supply chain: {{SUPPLY_CHAIN_POLICY}}

## 10) “Do not do” list (prevents drift)
- {{DONT_DO_1}}
- {{DONT_DO_2}}
- {{DONT_DO_3}}

## 11) ADR triggers for architecture
Create an ADR if we:
- change architectural style/module boundaries materially
- introduce persistent storage or new integration contract
- introduce new security posture or regulated data handling
- introduce new deployment topology / production operations

## 12) Known unknowns / TBDs
- {{ARCH_UNKNOWN_1_OR_NONE}}
- {{ARCH_UNKNOWN_2_OR_NONE}}

### core/library/foundation_pack/TEST_STRATEGY_BRIEF.md.tmpl

# Test Strategy Brief — {{PROJECT_NAME}}

> **File:** `TEST_STRATEGY_BRIEF.md`  
> **Created (UTC):** {{NOW_UTC}}  
> **Owner:** {{OWNER}}  
> **Team:** {{TEAM}}  
> **Repo / Project:** {{REPO_OR_PROJECT_REF}}  
> **Charter Ref:** {{CHARTER_REF}}  
> **Project Context Ref:** {{PROJECT_CONTEXT_REF_OR_NONE}}  
> **Profile:** {{PROFILE_ID_OR_TBD}}

## 1) Intent
A project-specific testing strategy that aligns with Charter testing rigor and risk flags.
This doc defines what tests we write, where they live, and what gates must pass.

## 2) Charter alignment
- Testing rigor target level (from Charter): {{LVL_TESTING}}
- Key risk flags affecting tests: {{RISK_FLAGS_RELEVANT_TO_TESTING}}

## 3) Test layers (what we prioritize)
- Unit tests: {{UNIT_TESTS_POLICY}}
- Integration tests: {{INTEGRATION_TESTS_POLICY}}
- End-to-end tests (if applicable): {{E2E_POLICY_OR_NA}}
- Negative tests: {{NEGATIVE_TESTS_POLICY}}
- Performance tests (if applicable): {{PERF_TESTS_POLICY_OR_NA}}

## 4) Boundary rules (prevents useless tests)
- Do not test standard library or trivial glue.
- Prefer business-logic-focused assertions.
- Mocking policy (stack-agnostic):
  - Allowed: pure unit isolation of external services/contracts
  - Avoid: mocking the system under test or “fake green” integration paths
  - For integration tests: prefer real dependencies in a controlled environment when feasible

## 5) Coverage posture
- Coverage target: {{COVERAGE_TARGET_OR_POLICY}}
- What must be covered:
  - Security/auth/validation paths (if applicable)
  - Core business logic
  - Error handling and “red line” behavior

## 6) Test data and fixtures
- Fixtures policy: {{FIXTURES_POLICY}}
- Seed data policy (if any): {{SEED_DATA_POLICY_OR_NA}}
- Determinism requirements: {{DETERMINISM_REQUIREMENTS}}

## 7) CI / gate coupling (profile-driven)
Do not hardcode commands here; reference profile keys.

Required gates for tests:
- Primary test command key: {{PROFILE_CMD_TESTS_KEY}}  <!-- e.g., commands.tests -->
- Optional additional test commands: {{PROFILE_CMD_TESTS_EXTRA_KEYS_OR_NONE}}

## 8) Fast feedback loops
- Local “quick check” set: {{QUICK_CHECK_GATES}}
- Full validation set: {{FULL_CHECK_GATES}}

## 9) Known testing risks
- {{TEST_RISK_1_OR_NONE}}
- {{TEST_RISK_2_OR_NONE}}

### core/library/foundation_pack/QUALITY_GATES_SPEC.md.tmpl

# Quality Gates Specification — {{PROJECT_NAME}}

> **File:** `QUALITY_GATES_SPEC.md`  
> **Created (UTC):** {{NOW_UTC}}  
> **Owner:** {{OWNER}}  
> **Team:** {{TEAM}}  
> **Repo / Project:** {{REPO_OR_PROJECT_REF}}  
> **Charter Ref:** {{CHARTER_REF}}  
> **Project Context Ref:** {{PROJECT_CONTEXT_REF_OR_NONE}}  
> **Profile:** {{PROFILE_ID_OR_TBD}}

## 1) Intent (automation-first)
This document defines the **complete, explicit, machine-enforceable** quality gates for this project.
No gate may depend on human judgment or an AI agent’s opinion to pass/fail.

All required gates MUST be:
- executable by automation (local + CI)
- deterministically pass/fail via exit codes and/or parse rules
- enforced as merge blockers (unless an explicit exception is approved and recorded)

## 2) Inputs that determine strictness
- Charter baseline posture: Level {{CHARTER_BASELINE_LEVEL}} — {{CHARTER_BASELINE_LABEL}}
- Relevant dimension targets:
  - Security: {{LVL_SECURITY}}
  - Testing: {{LVL_TESTING}}
  - Reliability: {{LVL_RELIABILITY}}
  - Observability: {{LVL_OBSERVABILITY}}
  - Type safety: {{LVL_TYPE_SAFETY}}
- Project classification implications:
  - Back-compat: {{BACKCOMPAT_REQUIRED_YES_NO_NA}}
  - Migrations: {{MIGRATIONS_REQUIRED_YES_NO_NA}}
  - Live/prod reality: {{OPERATIONAL_REALITY_SUMMARY_OR_UNKNOWN}}

## 3) Enforcement surfaces (no “paper green”)
### Local developer enforcement
- Entry point(s): {{LOCAL_GATE_ENTRYPOINTS}}  <!-- e.g., make quality / task check / scripts/quality.sh -->
- Pre-commit hooks: {{PRECOMMIT_POLICY_OR_NONE}}
- What must run before commit: {{LOCAL_REQUIRED_PRECOMMIT_GATES}}

### CI enforcement (merge blockers)
- CI pipeline: {{CI_SYSTEM_OR_NONE}}
- Required status checks: {{REQUIRED_STATUS_CHECKS}}
- Policy: merges are blocked unless ALL required gates pass.

### Exception/override policy
Exceptions are allowed only via Charter exception process:
- Approver: {{EXCEPTION_APPROVER}}
- Record location: {{EXCEPTION_RECORD_LOCATION}}
- Required fields: what/why/scope/risk/expiry/owner

## 4) Gate catalog (exhaustive)
> Each gate must specify:
> - ID
> - Purpose
> - Command reference (profile key) and resolved command (optional)
> - Scope (paths/modules)
> - Pass criteria (exit code, parse rules)
> - Failure action

### Gate Table (authoritative)
| Gate ID | Required | Purpose | Profile command key | Scope | Pass criteria | Notes |
|---|---:|---|---|---|---|---|
| {{GATE_1_ID}} | {{GATE_1_REQUIRED}} | {{GATE_1_PURPOSE}} | {{GATE_1_CMD_KEY}} | {{GATE_1_SCOPE}} | {{GATE_1_PASS}} | {{GATE_1_NOTES}} |
| {{GATE_2_ID}} | {{GATE_2_REQUIRED}} | {{GATE_2_PURPOSE}} | {{GATE_2_CMD_KEY}} | {{GATE_2_SCOPE}} | {{GATE_2_PASS}} | {{GATE_2_NOTES}} |
| {{GATE_3_ID}} | {{GATE_3_REQUIRED}} | {{GATE_3_PURPOSE}} | {{GATE_3_CMD_KEY}} | {{GATE_3_SCOPE}} | {{GATE_3_PASS}} | {{GATE_3_NOTES}} |

## 5) Standard required gates (baseline set)
> Tailor required/optional based on Charter dimension targets and profile capabilities.

- Tests (required): {{GATE_TESTS_POLICY}}
- Lint (required): {{GATE_LINT_POLICY}}
- Format (required): {{GATE_FORMAT_POLICY}}
- Typecheck (required/optional): {{GATE_TYPECHECK_POLICY}}
- Security scan (required/optional): {{GATE_SECURITY_POLICY}}
- Complexity (required/optional): {{GATE_COMPLEXITY_POLICY}}
- Docs/docstrings (required/optional): {{GATE_DOCS_POLICY}}
- Coverage (required/optional): {{GATE_COVERAGE_POLICY}}
- Build/package verification (required/optional): {{GATE_BUILD_POLICY}}

## 6) Evidence requirements (mandatory for any “PASS” claim)
A gate is only considered PASS if:
- command executed
- exit code captured
- output excerpt captured (tail or failure snippet)
- timestamp recorded

Evidence is stored at:
- {{EVIDENCE_LOG_PATH}}  <!-- e.g., artifacts/execution/<slice>/evidence.jsonl or artifacts/foundation/gates_evidence.jsonl -->

## 7) How gates run (order + optimization)
- Fast set (developer loop): {{FAST_GATE_SET}}
- Full set (CI / pre-merge): {{FULL_GATE_SET}}
- Order (recommended): format → lint → typecheck → tests → security → complexity → docs → coverage → build

## 8) Non-negotiable alignment to Charter red lines
List any Charter red lines that map directly to gates:
- {{REDLINE_GATE_MAPPING_1}}
- {{REDLINE_GATE_MAPPING_2}}

## 9) Outputs (machine config)
This spec is paired with a machine-readable gate config:
- `quality_gates.yaml`

That file is the automation source of truth for gate execution.
This doc explains rationale and enforcement, but automation uses the YAML.

### core/library/foundation_pack/quality_gates.yaml.tmpl

version: "0.1.0"
project:
  name: "{{PROJECT_NAME}}"
  repo_or_project_ref: "{{REPO_OR_PROJECT_REF}}"
  charter_ref: "{{CHARTER_REF}}"
  project_context_ref: "{{PROJECT_CONTEXT_REF_OR_NONE}}"

profile:
  id: "{{PROFILE_ID_OR_TBD}}"
  # Optional: profile overrides applied for this project
  overrides: "{{PROFILE_OVERRIDES_OR_NONE}}"

execution:
  cwd: "{{REPO_ROOT}}"
  # how evidence is captured by the automation runner
  evidence_log_path: "{{EVIDENCE_LOG_PATH}}"   # e.g., artifacts/foundation/quality_gates_evidence.jsonl
  required_evidence_fields: ["cmd", "exit_code", "timestamp", "stdout_tail", "stderr_tail"]

policy:
  # required gates must pass for CI/merge
  fail_fast: false
  required_all_must_pass: true
  allow_manual_override: false

gates:
  - id: "format"
    required: true
    kind: "command"
    command_ref: "commands.format"
    scope:
      paths: ["{{CODE_DIRS}}", "{{TEST_DIRS}}"]
    pass:
      exit_code: 0

  - id: "lint"
    required: true
    kind: "command"
    command_ref: "commands.lint"
    scope:
      paths: ["{{CODE_DIRS}}", "{{TEST_DIRS}}"]
    pass:
      exit_code: 0

  - id: "typecheck"
    required: "{{TYPECHECK_REQUIRED_TRUE_FALSE}}"
    kind: "command"
    command_ref: "commands.typecheck"
    scope:
      paths: ["{{CODE_DIRS}}"]
    pass:
      exit_code: 0

  - id: "tests"
    required: true
    kind: "command"
    command_ref: "commands.tests"
    scope:
      paths: ["{{TEST_DIRS}}"]
    pass:
      exit_code: 0

  - id: "security"
    required: "{{SECURITY_REQUIRED_TRUE_FALSE}}"
    kind: "command"
    command_ref: "commands.security"
    pass:
      exit_code: 0

  - id: "complexity"
    required: "{{COMPLEXITY_REQUIRED_TRUE_FALSE}}"
    kind: "command"
    command_ref: "commands.complexity"
    scope:
      paths: ["{{CODE_DIRS}}"]
    pass:
      exit_code: 0

  - id: "docs"
    required: "{{DOCS_REQUIRED_TRUE_FALSE}}"
    kind: "command"
    command_ref: "commands.docs"
    scope:
      paths: ["{{CODE_DIRS}}"]
    pass:
      exit_code: 0

  - id: "coverage"
    required: "{{COVERAGE_REQUIRED_TRUE_FALSE}}"
    kind: "command"
    command_ref: "commands.coverage"
    scope:
      paths: ["{{TEST_DIRS}}"]
    pass:
      exit_code: 0

  - id: "build"
    required: "{{BUILD_REQUIRED_TRUE_FALSE}}"
    kind: "command"
    command_ref: "commands.build"
    scope:
      paths: ["{{REPO_ROOT}}"]
    pass:
      exit_code: 0

order:
  fast: ["format", "lint", "tests"]
  full: ["format", "lint", "typecheck", "tests", "security", "complexity", "docs", "coverage", "build"]

### core/library/environment_inventory/environment_inventory_directive.md

You are the Environment Inventory Synthesizer.

## Purpose

Produce a canonical, project-specific **ENVIRONMENT_INVENTORY.md** that acts as the “store of record” for:

- environment variables (including secrets policy)
- runtime dependencies and external services
- local dev / CI / prod assumptions (ports, endpoints, required services)
- update contract (how changes must be recorded)

This is NOT a setup guide and NOT a profile definition.

- Profiles define reusable tooling/commands for a stack.
- ENVIRONMENT_INVENTORY defines what THIS project requires reminder-for-reminder.

## Canonical location

- **Canonical file (repo/project root):** `${repo_root}/ENVIRONMENT_INVENTORY.md`
- **Pipeline artifact copy (for traceability):** `artifacts/foundation/ENVIRONMENT_INVENTORY.md`

You will output the document content once; the harness/orchestrator is responsible for writing/syncing
the canonical repo file and the artifact copy.

## Inputs

Required:

- `CHARTER.md`
  Optional (use if present):
- `PROJECT_CONTEXT.md`
- Foundation artifacts if already generated (TECH_ARCH_BRIEF, TEST_STRATEGY_BRIEF, QUALITY_GATES_SPEC)

## Operating Rules

1. No invention: if a concrete fact is unknown, mark it as Unknown and add it to “Known Unknowns”.
2. Prefer completeness over elegance: inventories should be exhaustive but concise.
3. Automation-first structure: format entries so future automation can diff/check them.
4. Charter-aligned: respect Charter security posture, red lines, and exception process.
5. Be explicit about what exists today vs what is planned.

## Output Contract

Output ONLY the completed `ENVIRONMENT_INVENTORY.md` using the provided template.
No extra commentary outside the final markdown.

## Mandatory sections to fill

- Environment Variables inventory (required vs optional, secret vs non-secret, defaults/examples, validation notes)
- External services & infrastructure dependencies
- Local dev requirements
- CI requirements
- Prod/runtime requirements (even if “none yet”)
- Update Contract (rules for keeping inventory current)
- Known Unknowns

### core/library/environment_inventory/ENVIRONMENT_INVENTORY.md.tmpl

# Environment Inventory — {{PROJECT_NAME}}

> **File:** `ENVIRONMENT_INVENTORY.md`  
> **Created (UTC):** {{NOW_UTC}}  
> **Owner:** {{OWNER}}  
> **Team:** {{TEAM}}  
> **Repo / Project:** {{REPO_OR_PROJECT_REF}}  
> **Charter Ref:** {{CHARTER_REF}}  
> **Project Context Ref:** {{PROJECT_CONTEXT_REF_OR_NONE}}

## What this is
The canonical store of record for this project’s environment and runtime requirements.
It is used to prevent drift and to keep changes (env vars, services, ports, runtime assumptions) explicitly recorded.

## How to use
- Use this as the authoritative inventory for env vars, services, and runtime assumptions.
- Feature specs and slices should reference this instead of inventing env details.
- Any change that adds/changes env vars or runtime dependencies MUST update this file in the same change.

---

## 1) Environment Variables (Inventory)
> Mark required vs optional, and secret vs non-secret. Avoid putting real secrets in this doc.
> Prefer describing where the secret lives (platform secret store, CI vars, etc.).

| Name | Required | Secret | Default/Example | Validation / Notes | Where used | Source of truth |
|---|---:|---:|---|---|---|---|
| {{ENV_VAR_1_NAME}} | {{YES_NO}} | {{YES_NO}} | {{ENV_VAR_1_EXAMPLE}} | {{ENV_VAR_1_VALIDATION}} | {{ENV_VAR_1_USAGE}} | {{ENV_VAR_1_SOURCE}} |
| {{ENV_VAR_2_NAME}} | {{YES_NO}} | {{YES_NO}} | {{ENV_VAR_2_EXAMPLE}} | {{ENV_VAR_2_VALIDATION}} | {{ENV_VAR_2_USAGE}} | {{ENV_VAR_2_SOURCE}} |

### 1.1) Secret handling (Charter-aligned)
- Secret posture (from Charter): {{SECURITY_POSTURE_SUMMARY}}
- Storage location(s): {{SECRETS_STORAGE_LOCATIONS}}
- Rotation expectations: {{ROTATION_EXPECTATIONS_OR_NONE}}
- Never allowed in repo: real tokens, private keys, raw credentials

---

## 2) External Services / Infrastructure Dependencies
> List what the system depends on at runtime (even if “dev only” today).

| Dependency | Required | Environments | Purpose | Connection details | Notes |
|---|---:|---|---|---|---|
| {{DEP_1_NAME}} | {{YES_NO}} | {{DEV_STAGE_PROD}} | {{DEP_1_PURPOSE}} | {{DEP_1_CONN}} | {{DEP_1_NOTES}} |
| {{DEP_2_NAME}} | {{YES_NO}} | {{DEV_STAGE_PROD}} | {{DEP_2_PURPOSE}} | {{DEP_2_CONN}} | {{DEP_2_NOTES}} |

Examples: Postgres, Redis, S3-compatible storage, external APIs, vector DB, message broker.

---

## 3) Runtime Assumptions (Ports, Paths, Storage, Limits)
- Required listening ports: {{PORTS_OR_NONE}}
- Filesystem needs (paths, permissions): {{FS_REQUIREMENTS_OR_NONE}}
- Persistent storage requirements: {{STORAGE_REQUIREMENTS_OR_NONE}}
- Network assumptions (outbound allowed? proxies?): {{NETWORK_ASSUMPTIONS_OR_NONE}}
- Performance/time budgets that impact env: {{PERF_BUDGETS_OR_NONE}}

---

## 4) Local Development Requirements
- Local prerequisites: {{LOCAL_PREREQS_OR_NONE}}
- “Works on my machine” prevention notes: {{LOCAL_NOTES_OR_NONE}}
- Recommended local env var file pattern: {{ENV_FILE_PATTERN_OR_NONE}}  <!-- e.g., .env.example + .env.local -->

---

## 5) CI Requirements
- CI system (if any): {{CI_SYSTEM_OR_NONE}}
- Required CI secrets/vars: {{CI_VARS_OR_NONE}}
- Services required during CI (db/redis/etc): {{CI_SERVICES_OR_NONE}}
- Artifacts produced/required: {{CI_ARTIFACTS_OR_NONE}}

---

## 6) Production / Deployment Requirements (even if not live yet)
- Is there production today? {{YES_NO_TBD}}
- Hosting model: {{HOSTING_MODEL_OR_TBD}}
- Runtime environment(s): {{RUNTIME_ENVS_OR_TBD}}
- Required secrets in prod: {{PROD_SECRETS_OR_NONE_TBD}}
- Observability endpoints/keys: {{OBS_KEYS_OR_NONE_TBD}}
- Backup/DR requirements: {{DR_REQUIREMENTS_OR_NONE_TBD}}

---

## 7) Dependency & Tooling Inventory (project-specific)
> Do not duplicate full profile docs. Record what this repo uses.

- Primary language/runtime: {{LANG_RUNTIME_OR_TBD}}
- Package manager / build system: {{PKG_MANAGER_OR_TBD}}
- Lockfile(s): {{LOCKFILES_OR_NONE}}
- Lint/type/test tools (names only): {{TOOLS_SUMMARY_OR_TBD}}
- Minimum supported versions (if known): {{MIN_VERSIONS_OR_TBD}}

---

## 8) Update Contract (non-negotiable)
Any change that impacts environment MUST update this file:
- Adding/changing an env var
- Adding/changing an external service dependency
- Changing ports, secrets locations, runtime assumptions
- Changing prod/CI requirements

If a change would violate Charter security posture or red lines:
- Follow the Charter exception process
- Record exception at: {{EXCEPTION_RECORD_LOCATION_FROM_CHARTER}}

---

## 9) Known Unknowns
- {{UNKNOWN_1_OR_NONE}}
- {{UNKNOWN_2_OR_NONE}}


## Artifact Inputs

### artifacts/charter/CHARTER.md (required)

# CHARTER.md

TEST CHARTER CONTENT

### artifacts/project_context/PROJECT_CONTEXT.md (optional)

# PROJECT_CONTEXT.md

TEST PROJECT CONTEXT


## Outputs


### Artifacts

- artifacts/foundation/FOUNDATION_STRATEGY.md
- artifacts/foundation/TECH_ARCH_BRIEF.md
- artifacts/foundation/TEST_STRATEGY_BRIEF.md
- artifacts/foundation/QUALITY_GATES_SPEC.md
- artifacts/foundation/quality_gates.yaml
- artifacts/foundation/ENVIRONMENT_INVENTORY.md


### Repo Files

- artifacts/_repo_root_test/ENVIRONMENT_INVENTORY.md


## Stage Body

# core/stages/07_foundation_pack.md

<!--
Stage body intentionally minimal.
The directive + templates define the content.
Orchestrator injects:
- CHARTER.md (+ PROJECT_CONTEXT.md if present)
- selected profile id (or ask the stage to recommend one)
Outputs must be written to artifacts/foundation/.
-->
