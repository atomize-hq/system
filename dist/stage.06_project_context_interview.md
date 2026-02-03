# stage.06_project_context_interview — Project Context Interview → PROJECT_CONTEXT.md

Optional stage that produces PROJECT_CONTEXT.md as a factual reality
snapshot. Runs only when the Charter still contains unknowns that would impact planning
(prod/users/data/back-compat/migrations/integrations/environments).


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

### core/library/project_context/project_context_gen_directive.md

You are an AI assistant helping me produce a short, factual, reusable **Project Context** document for a software project.

**Output filename:** PROJECT_CONTEXT.md  
**Document title:** Project Context — <Project Name>

This is NOT a charter and NOT a feature spec.

- The **Charter** defines posture + guardrails (“how we decide”).
- The **Project Context** defines reality (“what exists / what constraints are true”).

## Primary goal

Fill in the factual gaps that matter for planning and execution so agents don’t invent:

- migrations/back-compat when none exist
- rollout constraints when there’s no prod
- deployment/infra assumptions that aren’t real
- integration boundaries that aren’t accurate

## Required input

You MUST read `CHARTER.md` first.

## When to run (optional stage)

This stage is OPTIONAL and should only run if the Charter still has unknowns that would affect planning.
Examples of “unknowns that matter”:

- whether any production users or data exist
- whether backward compatibility is required
- known external integrations/contracts
- deployment environments/topology
- where decisions/exceptions/debt are recorded (if not in Charter)

If the Charter is already sufficiently complete on these, confirm that and produce a minimal Project Context.

## Interview rules

- Ask **one question at a time**.
- Keep it efficient: target **5–10 minutes** total.
- Only ask questions that fill **unknowns**, **ambiguities**, or **contradictions** left after reading the Charter.
- If the project appears **greenfield**, explicitly avoid migration/back-compat questions unless the user introduces legacy constraints.

## Question strategy (in order)

0. **Project summary (fast grounding)**: One-line what this project is + primary surface + primary users + top 1–3 workflows.
   - Only ask if not already clear from CHARTER.md.
1. **Operational reality**: Is anything live today (users/data/prod)? Any SLAs?
2. **Project classification implications**: Do we require backward compatibility? Any migrations? Any deprecations policy?
3. **Boundaries**: What do we own vs integrate with (internal/external)? Identify the top 1–5 integrations/contracts.
4. **Environments & delivery**: dev/stage/prod? CI/CD? release method? secrets/config handling?
5. **Data reality**: existing DBs/tables/events? retention? migration history? data sensitivity boundaries?
6. **Constraints**: deadlines, budget, required tech, hosting, compliance, performance, reliability expectations.
7. **Known unknowns**: what’s explicitly undecided but should be tracked.

## Stopping rule

Stop when:

- live-ness (prod/users/data) is clear,
- back-compat/migrations are explicitly “required” or “not applicable,”
- system boundaries/integrations are enumerated (even if minimal),
- environments are stated (even if “dev only”),
- remaining unknowns are listed with owners/triggers.

Then ask:
“I have enough to draft PROJECT_CONTEXT.md. Generate it now?”
If I say yes (or “go ahead”), output ONLY the completed markdown document.

## Output requirements (PROJECT_CONTEXT.md)

Ensure the “Project Summary (factual, 3–6 bullets)” section is filled.
If unknown, write “Unknown—track in Known Unknowns” rather than guessing.

Produce a markdown doc using the provided template:

- factual, concise, no fluff
- defaults allowed (“None”, “Not applicable”, “Unknown—track as open question”)
- no extra commentary outside the final markdown

### core/library/project_context/PROJECT_CONTEXT.md.tmpl

# Project Context — {{PROJECT_NAME}}

> **File:** `PROJECT_CONTEXT.md`  
> **Created (UTC):** {{NOW_UTC}}  
> **Owner:** {{OWNER}}  
> **Team:** {{TEAM}}  
> **Repo / Project:** {{REPO_OR_PROJECT_REF}}  
> **Charter Ref:** {{CHARTER_REF}}

## What this is
A factual snapshot of project reality used to prevent incorrect planning assumptions.  
(“What exists, what’s live, what constraints are true, what we integrate with.”)

## How to use this
- Use this document to ground **feature specs**, **phase plans**, **slice specs**, and **execution** in reality.
- If something is unknown, record it explicitly in **Known Unknowns** and avoid planning around invented constraints.
- This doc should change rarely; update it when reality changes (first prod launch, new integration, major migration, etc.).

---

## 0) Project Summary (factual, 3–6 bullets)
- **What this project is:** {{PROJECT_SUMMARY_ONE_LINE}}
- **Primary surface:** {{PRIMARY_SURFACE}}
  <!-- CLI / API-service / library / web app / desktop / infra-tooling / mixed -->
- **Primary users:** {{PRIMARY_USERS}}
- **Key workflows (top 1–3):**
  - {{WORKFLOW_1}}
  - {{WORKFLOW_2}}
  - {{WORKFLOW_3}}
- **Non-goals (optional):** {{NON_GOALS_OR_NONE}}

## 1) Operational Reality (the most important section)
- **Is anything live in production today?** {{YES_NO_TBD}}
- **Users:** {{USERS_NONE_INTERNAL_EXTERNAL_TBD}}
- **Data in production:** {{DATA_NONE_SOME_TBD}}
- **Uptime expectations / SLA:** {{SLA_NONE_TBD}}
- **Incident/on-call reality:** {{ONCALL_NONE_TBD}}
- **Primary risk flags present:** {{RISK_FLAGS_FROM_CHARTER_OR_DELTA}}

## 2) Project Classification Implications (planning guardrails)
> This prevents unnecessary migration/back-compat planning.

- **Project type (from Charter):** {{PROJECT_TYPE}}
- **Backward compatibility required?** {{YES_NO_NA}}  
  - Notes: {{BACKCOMPAT_NOTES}}
- **Migration planning required?** {{YES_NO_NA}}  
  - Notes: {{MIGRATION_NOTES}}
- **Deprecation policy exists?** {{YES_NO_NA}}  
  - Notes: {{DEPRECATION_NOTES}}
- **Rollout controls required (flags/canary/staged)?** {{YES_NO_NA}}  
  - Notes: {{ROLLOUT_NOTES}}

## 3) System Boundaries (what we own vs integrate with)
### What we own
- {{OWNED_AREA_1}}
- {{OWNED_AREA_2}}

### What we do NOT own (but may depend on)
- {{EXTERNAL_AREA_1}}
- {{EXTERNAL_AREA_2}}

## 4) Integrations & Contracts (top 1–5)
> Enumerate only what matters. If none, write “None.”

- **Integration 1:** {{INTEGRATION_1_NAME}}
  - Type: {{INTEGRATION_1_TYPE}} (internal service / external SaaS / API / DB / event bus / file / other)
  - Contract surface: {{INTEGRATION_1_CONTRACT}} (endpoints/events/schemas)
  - Authentication/authorization: {{INTEGRATION_1_AUTH}}
  - Failure mode expectations: {{INTEGRATION_1_FAILURES}}
- **Integration 2:** {{INTEGRATION_2_NAME}} (optional)
  - Type: {{INTEGRATION_2_TYPE}}
  - Contract surface: {{INTEGRATION_2_CONTRACT}}
  - Auth: {{INTEGRATION_2_AUTH}}
  - Failure modes: {{INTEGRATION_2_FAILURES}}

## 5) Environments & Delivery
- **Environments that exist:** {{ENVS_DEV_STAGE_PROD_TBD}}
- **Deployment model:** {{DEPLOYMENT_MODEL_TBD}} (single host / containers / k8s / serverless / desktop / mobile / other)
- **CI/CD reality:** {{CICD_NONE_BASIC_ADVANCED_TBD}}
- **Release cadence:** {{RELEASE_CADENCE_TBD}}
- **Config & secrets:** {{SECRETS_STRATEGY_TBD}}
- **Observability stack (if any):** {{OBS_STACK_NONE_TBD}}

## 6) Data Reality
> Keep this high level; just enough for planning.

- **Primary data stores:** {{DATA_STORES_NONE_TBD}}
- **Data classification:** {{DATA_CLASSIFICATION_NONE_PII_REGULATED_TBD}}
- **Retention requirements:** {{RETENTION_NONE_TBD}}
- **Backups / DR reality:** {{BACKUPS_NONE_TBD}}
- **Existing migrations/history:** {{MIGRATION_HISTORY_NONE_TBD}}

## 7) Repo / Codebase Reality (brownfield-friendly, but safe for greenfield)
- **Codebase exists today?** {{YES_NO}}
- **If yes:** current maturity: {{MATURITY_TINY_MEDIUM_LARGE_TBD}}
- **Key modules/areas to be aware of:** {{KEY_MODULES_TBD}}
- **Known constraints from existing code:** {{CODE_CONSTRAINTS_NONE_TBD}}

## 8) Constraints
- **Deadline/time constraints:** {{DEADLINE_NONE_TBD}}
- **Budget constraints:** {{BUDGET_NONE_TBD}}
- **Must-use tech / prohibited tech:** {{TECH_CONSTRAINTS_NONE_TBD}}
- **Compliance/legal constraints:** {{COMPLIANCE_NONE_TBD}}
- **Performance constraints:** {{PERF_CONSTRAINTS_NONE_TBD}}
- **Security constraints:** {{SEC_CONSTRAINTS_NONE_TBD}}

## 9) Known Unknowns (explicitly tracked)
> List anything that’s uncertain but would change planning decisions.

- {{UNKNOWN_1}} (owner: {{UNKNOWN_1_OWNER}}, revisit trigger: {{UNKNOWN_1_TRIGGER}})
- {{UNKNOWN_2}} (owner: {{UNKNOWN_2_OWNER}}, revisit trigger: {{UNKNOWN_2_TRIGGER}})

## 10) Update Triggers
Update this doc when:
- first production launch
- first external users
- new major integration/contract introduced
- major migration/modernization begins
- posture changes in Charter require new operational constraints


## Artifact Inputs

### artifacts/charter/CHARTER.md (required)

# Engineering Charter — InputsFlow

This charter defines how we make engineering tradeoffs (quality vs speed, testing, security, operability). Use it to set defaults and record justified exceptions.

<!-- generated from CHARTER_INPUTS.yaml -->

## Rubric (1–5)
1. Exploratory
2. Prototype
3. Product
4. Production
5. Hardened

## Anti-bikeshedding rules
- Baseline level applies unless overridden.
- Only specify deltas.

## Baseline
- Level: 3 (balanced quality vs speed)

## Project classification + implications
- Classification: Greenfield
- Planning defaults: Back-compat: not required; Migration: not required; Rollout: lightweight; Deprecation: not required yet; Observability: standard

## Operational reality
Nothing is in production today. Internal users. No external contracts to preserve.

## Domains/areas
None.

## Dimensions
(omitted in this fixture)

## Exceptions
Record exceptions in `CHARTER.md#exceptions` with: what/why/scope/risk/owner/expiry.

## Debt tracking
Track debt in issues with label `debt`; review monthly.

## Decision records
Use ADRs in `docs/decisions`.


## Outputs


### Artifacts

- artifacts/project_context/PROJECT_CONTEXT.md


### Repo Files

- artifacts/_repo_root_test2/PROJECT_CONTEXT.md


## Gating Notes

- Final response must be ONLY the completed PROJECT_CONTEXT.md markdown.
