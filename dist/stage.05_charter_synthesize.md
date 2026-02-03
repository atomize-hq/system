# stage.05_charter_synthesize — Charter Synthesis (from inputs) â CHARTER.md

Produces CHARTER.md from CHARTER_INPUTS.yaml in a single shot. This is designed for development/testing and CI-like iteration.


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

### core/library/charter/charter_synthesize_directive.md

You are an AI assistant generating an Engineering Charter (`CHARTER.md`) from a provided `CHARTER_INPUTS.yaml`.

## Rules

- **Do not ask questions.**
- Output **only** the final `CHARTER.md` markdown (no preface, no commentary).
- Treat `CHARTER_INPUTS.yaml` as the source of truth.
- If fields are missing/empty, make conservative assumptions and record them in a short "Assumptions" subsection near the top of the charter.
- Keep the charter short (roughly 1–3 pages of markdown).

## Profile-aware behavior

A profile pack may be included in the prompt (e.g., `profiles/python-uv/profile.yaml` + `commands.yaml`).

- Treat the selected profile as the default tooling assumptions.
- Only mention tooling that is consistent with the profile pack.

## Output requirements (CHARTER.md)

Produce a markdown doc with:

1. Title: “Engineering Charter — <Project Name>”
2. One-paragraph definition of what this charter is and how to use it
3. Rubric scale (1–5) + the anti-bikeshedding rules
4. Baseline level + rationale (2–4 bullets)
5. Project classification + default implications (back-compat, migration, rollout controls, deprecation, observability threshold)
6. Operational reality (one paragraph): prod status, users, data, contracts
7. Planning defaults summary (single line):
   “Back-compat: X; Migration: Y; Rollout: Z; Deprecation: A; Observability: B”
8. Domains/areas (if any) + their risk notes
9. Dimensions section in a consistent format:
   - Default stance (baseline or override)
   - Raise-the-bar triggers
   - Allowed shortcuts
   - Non-negotiables / red lines
   - Domain overrides (if any)
10. Exceptions/override process
11. Debt tracking expectations
12. Decision Records section (if applicable)

### core/library/charter/charter.md.tmpl

# Engineering Charter — {{PROJECT_NAME}}

> **File:** `CHARTER.md`  
> **Created (UTC):** {{NOW_UTC}}  
> **Owner:** {{OWNER}}  
> **Team:** {{TEAM}}  
> **Repo / Project:** {{REPO_OR_PROJECT_REF}}

## What this is
{{ONE_PARAGRAPH_DEFINITION}}
<!--
Example (replace with your own):
“This Engineering Charter defines our default tradeoffs and decision guardrails across engineering dimensions (speed, correctness, testing, reliability, security, etc.). It’s a day-to-day decision tool: it clarifies what we optimize for, where shortcuts are allowed, and what is non-negotiable.”
-->

## How to use this charter
- **Primary use:** When making engineering choices, pick options that fit the baseline posture + dimension stances below.
- **When unsure:** default to baseline, and log an assumption or open question.
- **When writing Decision Records (ADRs):** map each option to these dimensions/levels and check against red lines.
- **Scope:** applies to {{SCOPE_DESCRIPTION}}.
- **Non-goals:** {{NON_GOALS}}

---

## Rubric: 1–5 rigor levels
**We use a 1–5 scale across dimensions.** (A higher level means more rigor, safety, and long-term cost—usually slower delivery.)

| Level | Label | Meaning |
|------:|-------|---------|
| 1 | {{LEVEL_1_LABEL}} | {{LEVEL_1_MEANING}} |
| 2 | {{LEVEL_2_LABEL}} | {{LEVEL_2_MEANING}} |
| 3 | {{LEVEL_3_LABEL}} | {{LEVEL_3_MEANING}} |
| 4 | {{LEVEL_4_LABEL}} | {{LEVEL_4_MEANING}} |
| 5 | {{LEVEL_5_LABEL}} | {{LEVEL_5_MEANING}} |

<!-- Defaults (edit freely):
1 Exploratory — throwaway ok; optimize learning; minimal gates
2 Prototype — demoable/internal use; some structure; still speed-first
3 Product — real users; balanced; maintainability matters
4 Production — GA/customer-facing; strong quality/reliability/security defaults
5 Hardened — critical/regulated/high blast radius; strict gates; defense-in-depth
-->

### Anti-bikeshedding rules
- **Baseline first:** choose one baseline rigor level for the project; everything inherits it unless overridden.
- **Override only deltas:** only specify overrides where you truly differ from baseline.
- **Whole numbers only:** no half-levels.
- **Use triggers to decide:** “raise the bar when…” and “shortcuts allowed when…” settle adjacent-level debates.
- **If uncertain:** use baseline and record an assumption + revisit trigger.

---

## Project baseline posture
- **Baseline level:** {{BASELINE_LEVEL}} — {{BASELINE_LABEL}}
- **Rationale (2–4 bullets):**
  - {{BASELINE_RATIONALE_1}}
  - {{BASELINE_RATIONALE_2}}
  - {{BASELINE_RATIONALE_3}}
  - {{BASELINE_RATIONALE_4}}

### Context snapshot
- **Users:** {{USERS_INTERNAL_OR_EXTERNAL}}
- **Lifetime:** {{LIFETIME_EXPECTATION}}
- **Runtime environments:** {{RUNTIME_ENVIRONMENTS}}
- **Stack (expected / unknowns):** {{STACK_EXPECTED_AND_UNKNOWNS}}
- **Risk flags:** {{RISK_FLAGS}}
  - Examples: auth, payments, PII, regulated data, critical uptime, model inference, supply chain risk

### Project classification (planning defaults)
- **Type:** {{PROJECT_TYPE}}
  - Options (choose one):
    - **Greenfield** — new system; no existing prod users/data; migrations/back-compat usually N/A.
    - **Brownfield** — existing live system/users/data; compatibility and safe rollout often required.
    - **Integration** — new component that must plug into existing systems/contracts; compatibility applies at boundaries.
    - **Modernization** — reshaping/replacing an existing system (refactor/replatform/strangler); migration plan usually required.
    - **Hardening** — stability/security/perf/ops work only; minimal new features; tighten gates.
- **Operational reality:** {{OPERATIONAL_REALITY}}
  - e.g., prod today?, live users?, existing data?, SLAs/SLOs?, external contracts?
- **Default implications (inherit unless overridden by a feature):**
  - **Backward compatibility:** {{DEFAULT_BACKWARD_COMPATIBILITY}}
  - **Migration planning:** {{DEFAULT_MIGRATION_PLANNING}}
  - **Rollout controls (flags/canary/gradual):** {{DEFAULT_ROLLOUT_CONTROLS}}
  - **Deprecation policy:** {{DEFAULT_DEPRECATION_POLICY}}
  - **Observability threshold:** {{DEFAULT_OBSERVABILITY_THRESHOLD}}

---

## Domains / areas (optional overrides)
> Use this section for **coarse areas** (domains/services) like Auth/Identity, PII/Privacy, Billing, ML inference, Customer UX, Admin tools, Integrations, Deployment pipeline.  
> **Not** per-class or per-function.

<!-- If none, write: “None — baseline applies everywhere.” -->
{{DOMAINS_SUMMARY}}

<!-- COPY/PASTE this block for each domain/area that needs special posture -->
### {{DOMAIN_NAME}}
- **What it is:** {{DOMAIN_DESCRIPTION}}
- **Touches / trust boundary:** {{DOMAIN_TRUST_BOUNDARY}}
- **What can go wrong (blast radius):** {{DOMAIN_BLAST_RADIUS}}
- **Special constraints:** {{DOMAIN_CONSTRAINTS}}
- **Overrides (if any):**
  - {{DOMAIN_OVERRIDE_1}}
  - {{DOMAIN_OVERRIDE_2}}

---

## Posture at a glance (quick scan)
> If a field below is blank, it inherits the baseline level.

| Dimension | Default level (1–5) | Notes / intent |
|---|---:|---|
| Speed vs Quality | {{LVL_SPEED_QUALITY}} | {{NOTE_SPEED_QUALITY}} |
| Type safety / static analysis | {{LVL_TYPE_SAFETY}} | {{NOTE_TYPE_SAFETY}} |
| Testing rigor | {{LVL_TESTING}} | {{NOTE_TESTING}} |
| Scalability & performance | {{LVL_PERFORMANCE}} | {{NOTE_PERFORMANCE}} |
| Reliability & operability | {{LVL_RELIABILITY}} | {{NOTE_RELIABILITY}} |
| Security & privacy | {{LVL_SECURITY}} | {{NOTE_SECURITY}} |
| Observability | {{LVL_OBSERVABILITY}} | {{NOTE_OBSERVABILITY}} |
| Developer experience (DX) | {{LVL_DX}} | {{NOTE_DX}} |
| UX polish (or API usability) | {{LVL_UX}} | {{NOTE_UX}} |

---

## Dimensions (details + guardrails)

> **Format per dimension:**  
> - Default stance (level)  
> - Raise-the-bar triggers  
> - Allowed shortcuts  
> - Non-negotiables (red lines)  
> - Domain/area overrides (only where needed)

---

### 1) Speed vs Quality
- **Default stance (level):** {{LVL_SPEED_QUALITY}}
- **Default posture statement:** {{SPEED_QUALITY_POSTURE_STATEMENT}}

**Raise the bar when:**
- {{SPEED_QUALITY_RAISE_1}}
- {{SPEED_QUALITY_RAISE_2}}
- {{SPEED_QUALITY_RAISE_3}}

**Allowed shortcuts when:**
- {{SPEED_QUALITY_SHORTCUT_1}}
- {{SPEED_QUALITY_SHORTCUT_2}}
- {{SPEED_QUALITY_SHORTCUT_3}}

**Non-negotiables / red lines:**
- {{SPEED_QUALITY_REDLINE_1}}
- {{SPEED_QUALITY_REDLINE_2}}

**Domain overrides (if any):**
- {{SPEED_QUALITY_DOMAIN_OVERRIDE_NOTES}}

---

### 2) Type safety / static analysis strictness
- **Default stance (level):** {{LVL_TYPE_SAFETY}}
- **Default posture statement:** {{TYPE_SAFETY_POSTURE_STATEMENT}}
- **Tooling assumptions:** {{TYPE_SAFETY_TOOLING_ASSUMPTIONS}}
  - e.g., TS `strict`, lint rules, formatters, static analysis, schema validation

**Raise the bar when:**
- {{TYPE_SAFETY_RAISE_1}}
- {{TYPE_SAFETY_RAISE_2}}

**Allowed shortcuts when:**
- {{TYPE_SAFETY_SHORTCUT_1}}
- {{TYPE_SAFETY_SHORTCUT_2}}

**Non-negotiables / red lines:**
- {{TYPE_SAFETY_REDLINE_1}}
- {{TYPE_SAFETY_REDLINE_2}}

**Domain overrides (if any):**
- {{TYPE_SAFETY_DOMAIN_OVERRIDE_NOTES}}

---

### 3) Testing rigor
- **Default stance (level):** {{LVL_TESTING}}
- **Default posture statement:** {{TESTING_POSTURE_STATEMENT}}
- **Test pyramid expectation:** {{TEST_PYRAMID_EXPECTATION}}
  - e.g., unit vs integration vs e2e; contract tests; CI gating

**Raise the bar when:**
- {{TESTING_RAISE_1}}
- {{TESTING_RAISE_2}}
- {{TESTING_RAISE_3}}

**Allowed shortcuts when:**
- {{TESTING_SHORTCUT_1}}
- {{TESTING_SHORTCUT_2}}

**Non-negotiables / red lines:**
- {{TESTING_REDLINE_1}}
- {{TESTING_REDLINE_2}}

**Domain overrides (if any):**
- {{TESTING_DOMAIN_OVERRIDE_NOTES}}

---

### 4) Scalability & performance
- **Default stance (level):** {{LVL_PERFORMANCE}}
- **Default posture statement:** {{PERFORMANCE_POSTURE_STATEMENT}}
- **Performance targets (if any):** {{PERFORMANCE_TARGETS}}

**Raise the bar when:**
- {{PERFORMANCE_RAISE_1}}
- {{PERFORMANCE_RAISE_2}}

**Allowed shortcuts when:**
- {{PERFORMANCE_SHORTCUT_1}}
- {{PERFORMANCE_SHORTCUT_2}}

**Non-negotiables / red lines:**
- {{PERFORMANCE_REDLINE_1}}

**Domain overrides (if any):**
- {{PERFORMANCE_DOMAIN_OVERRIDE_NOTES}}

---

### 5) Reliability & operability
- **Default stance (level):** {{LVL_RELIABILITY}}
- **Default posture statement:** {{RELIABILITY_POSTURE_STATEMENT}}
- **Reliability targets (if any):** {{RELIABILITY_TARGETS}}
  - e.g., basic SLOs, rollback expectations, on-call/ownership

**Raise the bar when:**
- {{RELIABILITY_RAISE_1}}
- {{RELIABILITY_RAISE_2}}

**Allowed shortcuts when:**
- {{RELIABILITY_SHORTCUT_1}}
- {{RELIABILITY_SHORTCUT_2}}

**Non-negotiables / red lines:**
- {{RELIABILITY_REDLINE_1}}
- {{RELIABILITY_REDLINE_2}}

**Domain overrides (if any):**
- {{RELIABILITY_DOMAIN_OVERRIDE_NOTES}}

---

### 6) Security & privacy
- **Default stance (level):** {{LVL_SECURITY}}
- **Default posture statement:** {{SECURITY_POSTURE_STATEMENT}}
- **Threat model scope:** {{THREAT_MODEL_SCOPE}}
- **Data sensitivity:** {{DATA_SENSITIVITY}}

**Raise the bar when:**
- {{SECURITY_RAISE_1}}
- {{SECURITY_RAISE_2}}
- {{SECURITY_RAISE_3}}

**Allowed shortcuts when:**
- {{SECURITY_SHORTCUT_1}}
- {{SECURITY_SHORTCUT_2}}

**Non-negotiables / red lines:**
- {{SECURITY_REDLINE_1}}
- {{SECURITY_REDLINE_2}}
- {{SECURITY_REDLINE_3}}

**Domain overrides (if any):**
- {{SECURITY_DOMAIN_OVERRIDE_NOTES}}

---

### 7) Observability
- **Default stance (level):** {{LVL_OBSERVABILITY}}
- **Default posture statement:** {{OBSERVABILITY_POSTURE_STATEMENT}}
- **Minimum telemetry:** {{MIN_TELEMETRY_EXPECTATION}}
  - e.g., structured logs, metrics, traces, alerts

**Raise the bar when:**
- {{OBSERVABILITY_RAISE_1}}
- {{OBSERVABILITY_RAISE_2}}

**Allowed shortcuts when:**
- {{OBSERVABILITY_SHORTCUT_1}}
- {{OBSERVABILITY_SHORTCUT_2}}

**Non-negotiables / red lines:**
- {{OBSERVABILITY_REDLINE_1}}
- {{OBSERVABILITY_REDLINE_2}}

**Domain overrides (if any):**
- {{OBSERVABILITY_DOMAIN_OVERRIDE_NOTES}}

---

### 8) Developer experience (DX) & automation
- **Default stance (level):** {{LVL_DX}}
- **Default posture statement:** {{DX_POSTURE_STATEMENT}}
- **Automation baseline:** {{DX_AUTOMATION_BASELINE}}
  - e.g., CI, formatting, linting, release automation, local dev scripts

**Raise the bar when:**
- {{DX_RAISE_1}}
- {{DX_RAISE_2}}

**Allowed shortcuts when:**
- {{DX_SHORTCUT_1}}
- {{DX_SHORTCUT_2}}

**Non-negotiables / red lines:**
- {{DX_REDLINE_1}}

**Domain overrides (if any):**
- {{DX_DOMAIN_OVERRIDE_NOTES}}

---

### 9) UX polish (or API usability if no UI)
- **Default stance (level):** {{LVL_UX}}
- **Default posture statement:** {{UX_POSTURE_STATEMENT}}
- **Usability targets:** {{UX_TARGETS}}
  - e.g., accessibility baseline, performance perception, error messaging clarity

**Raise the bar when:**
- {{UX_RAISE_1}}
- {{UX_RAISE_2}}

**Allowed shortcuts when:**
- {{UX_SHORTCUT_1}}
- {{UX_SHORTCUT_2}}

**Non-negotiables / red lines:**
- {{UX_REDLINE_1}}
- {{UX_REDLINE_2}}

**Domain overrides (if any):**
- {{UX_DOMAIN_OVERRIDE_NOTES}}

---

## Cross-cutting red lines (global non-negotiables)
- {{GLOBAL_REDLINE_1}}
- {{GLOBAL_REDLINE_2}}
- {{GLOBAL_REDLINE_3}}

---

## Exceptions / overrides process
- **Who can approve:** {{EXCEPTION_APPROVER}}
- **Where exceptions are recorded:** {{EXCEPTION_RECORD_LOCATION}}
- **Minimum required fields for an exception:**
  - **What:** {{EXCEPTION_FIELD_WHAT}}
  - **Why:** {{EXCEPTION_FIELD_WHY}}
  - **Scope:** {{EXCEPTION_FIELD_SCOPE}}
  - **Risk:** {{EXCEPTION_FIELD_RISK}}
  - **Expiry / revisit date:** {{EXCEPTION_FIELD_EXPIRY}}
  - **Owner:** {{EXCEPTION_FIELD_OWNER}}
- **Default rule:** exceptions are time-boxed; if not renewed, revert to baseline posture.

---

## Debt tracking expectations
- **Where debt is tracked:** {{DEBT_TRACKING_LOCATION}}
- **What counts as “debt” worth logging:** {{DEBT_DEFINITION}}
- **Required fields per debt item:**
  - {{DEBT_FIELD_1}}
  - {{DEBT_FIELD_2}}
  - {{DEBT_FIELD_3}}
- **Review cadence:** {{DEBT_REVIEW_CADENCE}}
- **Paydown trigger(s):** {{DEBT_PAYDOWN_TRIGGERS}}

---

## Decision Records (ADRs): how to use this charter
- **Decision record format:** {{ADR_FORMAT}}
- **Decision record location:** {{ADR_LOCATION}}
- When evaluating options, explicitly map each option to:
  - impacted dimensions
  - expected level (1–5) per impacted dimension
  - conflicts with any red lines
- Always include at least:
  - **Fast path** option (optimize speed / lower rigor where allowed)
  - **Robust path** option (optimize reliability/security/maintainability)
  - **Balanced** option (default unless project says otherwise)
- The chosen decision must state **why it matches this charter**, or link to an approved exception.

---

## Review & updates
- **Review cadence:** {{CHARTER_REVIEW_CADENCE}}
- **Update triggers:** {{CHARTER_UPDATE_TRIGGERS}}
  - e.g., production launch, first external customers, incident, scope change, new domain added

<!-- End of CHARTER.md.tmpl -->


## Artifact Inputs

### artifacts/charter/CHARTER_INPUTS.yaml (required)

schema_version: 0.1.0
project:
  name: InputsFlow
  classification: greenfield
  team_size: 2
  users: internal
  expected_lifetime: years
  surfaces: [api]
  runtime_environments: [server]
  constraints: {deadline: "", budget: "", experience_notes: "", must_use_tech: []}
  operational_reality:
    in_production_today: false
    prod_users_or_data: ""
    external_contracts_to_preserve: []
    uptime_expectations: "best effort"
  default_implications:
    backward_compatibility: not_required
    migration_planning: not_required
    rollout_controls: lightweight
    deprecation_policy: not_required_yet
    observability_threshold: standard
posture:
  rubric_scale: "1-5"
  baseline_level: 3
  baseline_rationale: ["balanced quality vs speed"]
domains: []
dimensions: []
exceptions: {approvers: [project_owner], record_location: "CHARTER.md#exceptions", minimum_fields: [what, why, scope, risk, owner, expiry_or_revisit_date]}
debt_tracking: {system: issues, labels: [debt], review_cadence: monthly}
decision_records: {enabled: true, path: "docs/decisions", format: md}


## Outputs


### Artifacts

- artifacts/charter/CHARTER.md


### Repo Files

- artifacts/_repo_root_test2/CHARTER.md


## Gating Notes

- Final response must be ONLY the completed CHARTER.md markdown.


## Stage Body

<!-- Stage body intentionally minimal. -->
