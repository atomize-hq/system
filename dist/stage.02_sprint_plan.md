# stage.02_sprint_plan — Sprint Planning (Opening Gate)

Uses release intent + previous sprint reality to produce a concrete sprint plan and task list. Tasks must reference release-selected work items and be machine-checkable.


## Run Variables

- charter_ref: artifacts/charter/CHARTER.md
- enable_complexity: False
- needs_project_context: True
- now_utc: 2026-01-28T18:35:10Z
- owner: 
- prev_sprint_id: sprint-000
- profile: python-uv
- project_context_ref: 
- project_name: TestProject
- release_id: release-001
- release_type: minor
- repo_or_project_ref: local/system
- repo_root: .
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


## Library Inputs

### core/library/sprint/sprint_planning_directive.md

# Sprint Planning Directive

You are the **Sprint Planning Agent**.

## Purpose
Turn release intent into an executable sprint by producing:
- a sprint plan (human readable)
- a sprint machine file
- a machine-readable task list (typed tasks)

Sprint planning is an **opening planning gate**:
- the release provides rough intent (slot goal + focus items)
- the sprint plan fleshes out the concrete task list needed to deliver that intent
- the sprint plan MUST incorporate what the **previous sprint actually accomplished** when that input is provided

## Required inputs
You may receive these artifacts:
- Release plan files: `RELEASE_PLAN.md` and `release.yaml`
- Optional previous sprint report: `SPRINT_REPORT.md`
- Work catalog: `backlog/WORK_CATALOG.yaml`
- Feature specs referenced by release items (if provided)

## Rules
- **Do not invent work items.** Every task MUST reference at least one work item ID present in the Release Plan selection.
- Prefer small, typed tasks that are machine-checkable.
- Keep language/tooling agnostic (no stack commands).

## Lane model
If a sprint/lanes overlay is included, follow it.
Otherwise, keep lane count small (2–4) or omit lanes entirely.

## Task types (minimum)
Use task types that allow deterministic close-out:
- `planning_gate` (required): confirms the sprint task list is complete and linked to release + work items
- `research_discovery` (only when needed)
- `decision_registry` (optional for now; can be a placeholder)
- `execution_slice` (implementation work; will later map to slice pipelines)
- `review_cycle` (review/refactor/polish)
- `integration_gate` (required if multiple lanes or cross-cutting changes)

## Output formatting (MANDATORY — multi-file wrapper)
When generating the final output, you MUST emit **exactly** one `--- FILE: ... ---` block per declared artifact output
(in the exact order shown under **Outputs → Artifacts** in this prompt), with **no additional text** outside the blocks.

Rules:
- The `--- FILE: ... ---` lines must match the declared output paths exactly.
- Content must be the full document for that file.
- Do not wrap in code fences.
- Do not include summaries, explanations, or manifests outside the blocks.

## Output contract
- Follow the templates provided in the prompt.
- The YAML files must be valid.

### core/library/sprint/SPRINT_PLAN.md.tmpl

# Sprint Plan — {{SPRINT_ID}}

- Sprint ID: {{SPRINT_ID}}
- Release ID: {{RELEASE_ID}}
- Release Slot: {{SPRINT_SLOT_ID}}
- Created (UTC): {{NOW_UTC}}
- Owner: {{OWNER}}
- Team: {{TEAM}}
- Repo / Project: {{REPO_OR_PROJECT_REF}}
- Charter Ref: {{CHARTER_REF}}
- Release Ref: {{RELEASE_REF}}
- Work Catalog Ref: {{WORK_CATALOG_REF_OR_NONE}}
- Previous Sprint Report Ref: {{PREV_SPRINT_REPORT_REF_OR_NONE}}

## 1) Sprint goal (from release slot + updated by reality)
{{SPRINT_GOAL_ONE_PARAGRAPH}}

## 2) Sprint definition of done (exit criteria)
> Sprint cannot close unless these are met.

- {{SPRINT_DOD_1}}
- {{SPRINT_DOD_2}}
- {{SPRINT_DOD_3}}

## 3) Focus work items (must be from Release selection)
{{FOCUS_ITEM_IDS_LIST}}

## 4) Plan inputs
### Release intent
- Slot goal: {{RELEASE_SLOT_GOAL}}
- Required task types: {{RELEASE_REQUIRED_TASK_TYPES}}

### Previous sprint reality (if any)
- Summary: {{PREV_SPRINT_SUMMARY_OR_NONE}}
- Carry-over items: {{PREV_SPRINT_CARRYOVER_OR_NONE}}

## 5) Lanes (optional)
{{LANES_SECTION_OR_NONE}}

<!-- Example lanes section:
### Lane: discovery
- Goal: ...
- Focus items: [FEAT-001]

### Lane: delivery
- Goal: ...
- Focus items: [FEAT-001, BUG-002]

### Lane: integration
- Goal: ...
- Focus items: [FEAT-001]
-->

## 6) Task plan (summary)
> Detailed tasks live in `tasks.yaml`. This section is a human index.

- Planning gate: {{PLANNING_GATE_TASK_ID}}
- Integration gate: {{INTEGRATION_GATE_TASK_ID_OR_NONE}}
- Other key tasks:
  - {{KEY_TASK_1}}
  - {{KEY_TASK_2}}
  - {{KEY_TASK_3}}

## 7) Notes
- Risks: {{RISKS_OR_NONE}}
- Dependencies: {{DEPENDENCIES_OR_NONE}}
- Open questions: {{OPEN_QUESTIONS_OR_NONE}}

## 8) Traceability
- Release slot → tasks.yaml: {{TRACE_RELEASE_SLOT_TO_TASKS}}
- Work items → tasks.yaml: {{TRACE_WORK_ITEMS_TO_TASKS}}

### core/library/sprint/sprint.yaml.tmpl

kind: sprint
version: 0.1.0
id: {{SPRINT_ID}}
release_id: {{RELEASE_ID}}
slot_id: {{SPRINT_SLOT}}
title: {{SPRINT_TITLE}}
created_utc: {{NOW_UTC}}

refs:
  charter: {{CHARTER_REF}}
  foundation:
    quality_gates_yaml: artifacts/foundation/quality_gates.yaml
  release_yaml: artifacts/releases/{{RELEASE_ID}}/release.yaml

intent:
  goal: {{SPRINT_GOAL}}
  target_item_ids: [{{TARGET_ITEM_IDS_CSV}}]

lanes:
  - id: {{LANE_1_ID}}
    purpose: {{LANE_1_PURPOSE}}
    target_item_ids: [{{LANE_1_TARGETS_CSV}}]
  - id: {{LANE_2_ID}}
    purpose: {{LANE_2_PURPOSE}}
    target_item_ids: [{{LANE_2_TARGETS_CSV}}]

tasks_file: tasks.yaml

close_policy:
  blocks_on_gate_tasks: true
  required_gate_types:
    - gate.planning
    - gate.quality

### core/library/sprint/tasks.yaml.tmpl

(missing)


## Artifact Inputs

### artifacts/base/BASE_CONTEXT.md (optional)

(missing)

### artifacts/charter/CHARTER.md (required)

# Engineering Charter — HandoffHub
  <!-- TEST MODE: synthetic charter for pipeline iteration -->

  > File: CHARTER.md
  > Created (UTC): 2026-01-28T18:35:10Z
  > Owner: Spenser McConnell
  > Team: 2 engineers
  > Repo / Project: sys (HandoffHub)

  ## What this is

  This Engineering Charter defines our default tradeoffs and decision guardrails across engineering dimensions (speed,
  correctness, testing, reliability, security, observability, DX, and UX/API usability). It’s a day-to-day decision
  tool: it clarifies what we optimize for, where shortcuts are allowed, and what is non-negotiable.

  ## How to use this charter

  - Primary use: When making engineering choices, pick options that fit the baseline posture + dimension stances below.
  - When unsure: default to baseline, and log an assumption or open question.
  - When writing Decision Records (ADRs): map each option to these dimensions/levels and check against red lines.
  - Scope: applies to the entire HandoffHub repo (API service, background jobs, data model, CI/CD, infra-as-code where
    present).
  - Non-goals: a feature roadmap; UI/UX specifications; detailed architecture diagrams.

  ———

  ## Rubric: 1–5 rigor levels

  We use a 1–5 scale across dimensions. (A higher level means more rigor, safety, and long-term cost—usually slower
  delivery.)

  | Level | Label | Meaning |
  |------:|-------|---------|
  | 1 | Exploratory | throwaway ok; optimize learning; minimal gates |
  | 2 | Prototype | demoable/internal use; some structure; still speed-first |
  | 3 | Product | real users/usage; balanced; maintainability matters |
  | 4 | Production | customer-facing/GA; strong quality/reliability/security defaults |
  | 5 | Hardened | critical/regulated/high blast radius; defense-in-depth; strict gates |

  ### Anti-bikeshedding rules

  - Baseline first: choose one baseline rigor level for the project; everything inherits it unless overridden.
  - Override only deltas: only specify overrides where you truly differ from baseline.
  - Whole numbers only: no half-levels.
  - Use triggers to decide: “raise the bar when…” and “shortcuts allowed when…” settle adjacent-level debates.
  - If uncertain: use baseline and record an assumption + revisit trigger.

  ———

  ## Project baseline posture

  - Baseline level: 3 — Product
  - Rationale (2–4 bullets):
      - External users are expected within 8–10 weeks (MVP → paid pilot), so maintainability matters early.
      - Small team needs strong automation to move fast without accumulating unpayable debt.
      - Mild risk flags (auth + PII) require stronger security defaults than a pure prototype.
      - No production users yet, so we can still iterate quickly and revise APIs without migration burden.

  ### Context snapshot

  - Users: External (small businesses) + internal admin users (support/ops)
  - Lifetime: 2+ years (intended to become a core product)
  - Runtime environments: Cloud-hosted API + worker (Linux containers), managed Postgres, browser-based admin UI later
  - Stack (expected / unknowns): Python 3.11+; uv for deps; FastAPI (expected); Postgres (expected); Redis (possible);
    hosting TBD (Fly.io/AWS/GCP)
  - Risk flags: auth, PII (names/emails), webhooks/integrations
      - Examples: auth, payments, PII, regulated data, critical uptime, model inference, supply chain risk

  ### Project classification (planning defaults)

  - Type: Greenfield
      - Options (choose one):
          - Greenfield — new system; no existing prod users/data; migrations/back-compat usually N/A.
          - Brownfield — existing live system/users/data; compatibility and safe rollout often required.
          - Integration — new component that must plug into existing systems/contracts; compatibility applies at
            boundaries.
          - Modernization — reshaping/replacing an existing system (refactor/replatform/strangler); migration plan
            usually required.
          - Hardening — stability/security/perf/ops work only; minimal new features; tighten gates.
  - Operational reality: Not in production yet; staging-only; no legacy clients; no external API contracts; best-effort
    uptime until first paid pilot.
  - Default implications (inherit unless overridden by a feature):
      - Backward compatibility: not required yet (pre-1.0 APIs can change)
      - Migration planning: not required (no prod data), but avoid knowingly painting into a corner
      - Rollout controls (flags/canary/gradual): lightweight feature flags for risky behavior changes
      - Deprecation policy: not required yet; prefer fast iteration with clear changelog notes
      - Observability threshold: standard baseline (structured logs + basic metrics + error tracking before beta)

  ———

  ## Domains / areas (optional overrides)

  Two domains require stricter posture than baseline.

  ### Auth & Sessions

  - What it is: login, session/token handling, password/magic-link flows, admin authorization checks
  - Touches / trust boundary: internet-facing; protects all user data; boundary with identity provider/email sender
  - What can go wrong (blast radius): account takeover, privilege escalation, data exposure across tenants
  - Special constraints: must resist common web threats; must be auditable; must fail closed
  - Overrides (if any):
      - Security & privacy to level 4
      - Testing rigor to level 4 for auth/permission logic and session validation paths

  ### Customer Data (PII) & Tenant Isolation

  - What it is: storage and access control for customer contact info, notes, and assignment history
  - Touches / trust boundary: PII storage; internal admin access; multi-tenant boundary
  - What can go wrong (blast radius): cross-tenant data leakage; accidental PII exposure in logs/exports
  - Special constraints: least privilege by default; log hygiene; safe export/delete operations
  - Overrides (if any):
      - Security & privacy to level 4
      - Observability to level 4 for access logging and anomaly signals on cross-tenant queries

  ———

  ## Posture at a glance (quick scan)

  > If a field below is blank, it inherits the baseline level.

  | Dimension | Default level (1–5) | Notes / intent |
  |---|---:|---|
  | Speed vs Quality | 3 | Move fast, but keep code shippable and reviewable |
  | Type safety / static analysis | 3 | Gradual typing with enforced checks where it matters |
  | Testing rigor | 3 | Unit-first; integration tests on boundaries; auth/PII raises bar |
  | Scalability & performance | 2 | Optimize for correctness; measure before tuning |
  | Reliability & operability | 3 | Basic SLO thinking and safe deploys before beta |
  | Security & privacy | 4 | Strong defaults due to auth + PII |
  | Observability | 3 | Standard telemetry baseline; auth/PII raises bar |
  | Developer experience (DX) | 3 | Automated gates; reproducible local dev |
  | UX polish (or API usability) | 3 | Clear APIs/errors; basic accessibility for any UI |

  ———

  ## Dimensions (details + guardrails)

  > Format per dimension:
  >
  > - Default stance (level)
  > - Raise-the-bar triggers
  > - Allowed shortcuts
  > - Non-negotiables (red lines)
  > - Domain/area overrides (only where needed)

  ———

  ### 1) Speed vs Quality

  - Default stance (level): 3
  - Default posture statement: Ship weekly increments, but don’t knowingly create “rewrite later” architecture; prefer
    simple, correct implementations with clean interfaces.

  Raise the bar when:

  - A change touches auth, permissions, tenant isolation, or PII handling.
  - A change introduces a new external contract (public API, webhook payload, import/export format).
  - A change will be hard to unwind (schema changes, irreversible background jobs, data backfills).

  Allowed shortcuts when:

  - Proving product value in MVP UI flow, with feature flags and reversible changes.
  - Non-critical internal tooling or admin-only pages that can be replaced quickly.
  - One-off scripts used locally (not checked into production runtime), as long as they don’t handle secrets/PII.

  Non-negotiables / red lines:

  - No “quick hacks” that bypass authorization checks or weaken tenant isolation.
  - No shipping known data-loss bugs or unsafe migrations to shared environments.

  Domain overrides (if any):

  - Auth & Sessions, Customer Data (PII): shortcuts are narrower; prefer correctness over speed.

  ———

  ### 2) Type safety / static analysis strictness

  - Default stance (level): 3
  - Default posture statement: Use types to prevent common errors at boundaries and in core logic; keep typing pragmatic
    and incremental.
  - Tooling assumptions: uv + ruff format + ruff check; mypy on src/; typed request/response models (e.g., Pydantic);
    schema validation at boundaries.

  Raise the bar when:

  - Adding/altering persistence models, multi-tenant query filters, or permission logic (require explicit types and
    mypy-clean code in touched modules).
  - Introducing new boundary surfaces (webhooks, CSV import/export, third-party APIs) (require validated schemas and
    typed adapters).

  Allowed shortcuts when:

  - Early-stage glue code where types would slow iteration, as long as the boundary is validated and a follow-up debt
    item is logged.
  - Narrow, localized cast() usage with tests demonstrating behavior.

  Non-negotiables / red lines:

  - No unchecked “dict soup” across service boundaries; external input must be validated before use.
  - No blanket disabling of type checking across large areas (no global mypy off switches); exceptions must be time-
    boxed.

  Domain overrides (if any):

  - Auth & Sessions: prefer fully typed auth/claims objects and explicit return types for permission checks.

  ———

  ### 3) Testing rigor

  - Default stance (level): 3
  - Default posture statement: Tests protect core logic and boundaries; CI gates prevent obvious regressions while
    keeping feedback fast.
  - Test pyramid expectation: 70% unit, 25% integration (DB/API boundaries), 5% e2e smoke (only if/when UI exists); CI
    runs unit+integration by default.

  Raise the bar when:

  - Touching auth, permissions, session validation, or password/magic-link flows (add unit tests + integration tests).
  - Touching tenant isolation or any query that filters by tenant/org (add integration test proving isolation).
  - Fixing a bug that could recur (add a regression test in the smallest scope possible).

  Allowed shortcuts when:

  - Spiking a new feature behind a flag where interfaces are still in flux (must add at least “happy path” unit tests
    before flag defaults to on).
  - UI polish changes (copy/layout) with no behavioral impact (manual verification allowed).

  Non-negotiables / red lines:

  - No merging changes that reduce auth/tenant isolation coverage in touched areas.
  - No “tests are flaky, ignore it” culture: flaky tests must be quarantined (skipped with issue + owner) and fixed
    quickly.

  Domain overrides (if any):

  - Auth & Sessions: level 4 expectation (tests required for all critical paths and failure modes).
  - Customer Data (PII): integration tests required for access control and exports.

  ———

  ### 4) Scalability & performance

  - Default stance (level): 2
  - Default posture statement: Build for clarity and correctness first; profile and optimize only with evidence.
  - Performance targets (if any): MVP target p95 API latency < 500ms for common endpoints in staging-like env;
    background jobs should complete within minutes, not hours.

  Raise the bar when:

  - p95 latency or error rate materially impacts onboarding or core workflows (then move impacted endpoints to level 3
    behavior: profiling + targeted indexes + caching).
  - A feature introduces N+1 query risk or large scans in multi-tenant tables.

  Allowed shortcuts when:

  - Low-traffic internal admin endpoints where correctness is more important than speed.
  - Temporary in-memory caching for MVP (must be safe per-tenant and easy to remove).

  Non-negotiables / red lines:

  - No unbounded operations on user-controlled input (unbounded list endpoints, unlimited exports) without pagination/
    limits.

  Domain overrides (if any):

  - None beyond baseline; revisit after first pilot traffic.

  ———

  ### 5) Reliability & operability

  - Default stance (level): 3
  - Default posture statement: Operate like we will have users soon: safe deploys, predictable migrations, and basic
    incident readiness—even before GA.
  - Reliability targets (if any): Before beta: health checks + error budgets mindset; for pilot: aim for 99.5% monthly
    availability for core API.

  Raise the bar when:

  - Moving from staging-only to beta/pilot (require rollback strategy, migration discipline, and on-call ownership).
  - Introducing scheduled/background processing that can duplicate or drop work (require idempotency and retry
    strategy).

  Allowed shortcuts when:

  - Early MVP with manual operational steps documented (as long as steps are safe and repeatable).
  - Non-critical jobs can run “best effort” if they don’t affect data correctness (e.g., notifications that can be
    retried).

  Non-negotiables / red lines:

  - No destructive migrations without backups and a tested rollback path for schema changes.
  - No silent failure: failures must surface via logs/alerts/error tracking.

  Domain overrides (if any):

  - Customer Data (PII): extra caution on migrations, exports, and deletes (require explicit runbooks/checklists).

  ———

  ### 6) Security & privacy

  - Default stance (level): 4
  - Default posture statement: Treat auth and PII as first-class from day one; prefer secure-by-default patterns even if
    it costs some speed.
  - Threat model scope: internet attacker, malicious tenant user, compromised API token, accidental insider access,
    third-party integration failures
  - Data sensitivity: moderate PII (names/emails/notes), multi-tenant business data; no payments/SSNs/medical data
    assumed

  Raise the bar when:

  - Adding new integrations/webhooks (require signed requests or allowlists where feasible; least-privilege tokens).
  - Adding admin capabilities (require explicit role checks, audit logging, and safe defaults).
  - Storing new categories of user data (explicit classification + retention policy).

  Allowed shortcuts when:

  - Using managed services with strong defaults (managed Postgres, managed secret store) rather than building custom
    security controls.
  - Deferring advanced features (device fingerprinting, risk scoring) if core hygiene is met (rate limits, lockouts,
    token rotation where applicable).

  Non-negotiables / red lines:

  - No secrets in repo, logs, or client-side bundles; use environment/secret managers only.
  - No authorization by UI convention: every server-side action must enforce authorization and tenant scoping.
  - No PII in plaintext logs; redact at the boundary and standardize structured logging fields.

  Domain overrides (if any):

  - Auth & Sessions: require careful token/session handling and secure defaults (httpOnly cookies where applicable, CSRF
    protections for cookie auth, password hashing best practices).
  - Customer Data (PII): require retention considerations and safe export/delete behavior.

  ———

  ### 7) Observability

  - Default stance (level): 3
  - Default posture statement: Make failures diagnosable quickly with minimal overhead; add deeper signals as we
    approach beta/pilot.
  - Minimum telemetry: structured logs (JSON), request IDs, basic metrics (requests, latency, errors), health endpoints,
    error tracking (e.g., Sentry) before beta.

  Raise the bar when:

  - Beta/pilot begins (alerts on elevated error rate and latency; dashboards for core endpoints).
  - Background jobs become critical (metrics on queue depth, retries, dead-letter counts).
  - Investigating a security/tenant isolation concern (add targeted audit logs and detection queries).

  Allowed shortcuts when:

  - Early MVP can start with logs + error tracking only, if metrics/alerts are scheduled before beta.
  - Manual runbooks are acceptable early if they are short, accurate, and kept current.

  Non-negotiables / red lines:

  - No logging of credentials, raw tokens, or sensitive PII fields.
  - No “unknown failures”: any exception path must at least log with request/job context and surface in error tracking.

  Domain overrides (if any):

  - Customer Data (PII): level 4 expectation for access/audit signals (who accessed what tenant, when, and from where—
    within reason).

  ———

  ### 8) Developer experience (DX) & automation

  - Default stance (level): 3
  - Default posture statement: Make the happy path easy: reproducible local setup, fast checks, and consistent
    formatting.
  - Automation baseline: uv sync for setup; ruff format and ruff check on CI; pytest on CI; optional mypy in CI once
    baseline types are in place.

  Raise the bar when:

  - Team grows beyond 2 engineers (tighten CI and add pre-commit hooks as standard).
  - The project reaches beta (add typecheck gate and a basic security audit step).

  Allowed shortcuts when:

  - Early MVP can keep CI minimal (format/lint/tests) if checks run in <5 minutes and failures are addressed
    immediately.
  - Temporary local scripts are fine if documented and not relied on for production deploys.

  Non-negotiables / red lines:

  - No “works on my machine” dependencies: local dev must be reproducible with documented steps and pinned tooling.

  Domain overrides (if any):

  - None.

  ———

  ### 9) UX polish (or API usability if no UI)

  - Default stance (level): 3
  - Default posture statement: Prioritize clarity and predictability: good error messages, stable contracts, and
    accessible defaults for any UI.
  - Usability targets: clear API error shape; consistent pagination; actionable validation errors; basic accessibility
    (keyboard nav/labels) for any admin UI.

  Raise the bar when:

  - External customers start onboarding (level 4 behavior for onboarding-critical flows: copy, validation, and edge
    cases must be polished).
  - Supporting non-technical admins (reduce ambiguity, add guardrails and confirmations on destructive actions).

  Allowed shortcuts when:

  - Visual polish can lag if core flows are correct and understandable.
  - Early UI can be functional-first if error states and loading states are present.

  Non-negotiables / red lines:

  - No destructive actions without confirmation and clear consequences.
  - No opaque 500s for validation/user errors: return structured errors with user-safe messages.

  Domain overrides (if any):

  - None.

  ———

  ## Cross-cutting red lines (global non-negotiables)

  - Never ship code that can read/write cross-tenant data without explicit tenant scoping and tests for isolation.
  - Never log secrets or sensitive PII; enforce redaction at boundaries and in structured logging.
  - Never bypass auth/authorization checks for “speed”; any exception must be documented, time-boxed, and approved.

  ———

  ## Exceptions / overrides process

  - Who can approve: Project owner (Spenser McConnell)
  - Where exceptions are recorded: docs/exceptions.md (append-only log with links to issues/PRs)
  - Minimum required fields for an exception:
      - What: concrete rule being bypassed and the affected components
      - Why: business/engineering justification and why alternatives were rejected
      - Scope: repos/modules/endpoints/jobs impacted; environments impacted
      - Risk: what could go wrong + blast radius + detection/mitigation plan
      - Expiry / revisit date: a specific date (max 30 days for security/testing exceptions)
      - Owner: person responsible for follow-up/removal
  - Default rule: exceptions are time-boxed; if not renewed, revert to baseline posture.

  ———

  ## Debt tracking expectations

  - Where debt is tracked: issue tracker with labels debt and (if applicable) risk-accepted
  - What counts as “debt” worth logging: any shortcut that increases future cost or risk (skipped tests, partial typing,
    temporary schemas, manual ops steps, known perf issues)
  - Required fields per debt item:
      - scope + affected area
      - why it exists + what “done” looks like
      - risk level + suggested payoff plan
  - Review cadence: every 2 weeks (or each milestone) during MVP
  - Paydown trigger(s): beta launch readiness, repeated bug class, rising support load, or when a shortcut blocks a new
    feature

  ———

  ## Decision Records (ADRs): how to use this charter

  - Decision record format: Markdown ADR (ADR-YYYYMMDD-<slug>.md)
  - Decision record location: docs/adr/
  - When evaluating options, explicitly map each option to:
      - impacted dimensions
      - expected level (1–5) per impacted dimension
      - conflicts with any red lines
  - Always include at least:
      - Fast path option (optimize speed / lower rigor where allowed)
      - Robust path option (optimize reliability/security/maintainability)
      - Balanced option (default unless project says otherwise)
  - The chosen decision must state why it matches this charter, or link to an approved exception.

  ———

  ## Review & updates

  - Review cadence: monthly, or at the start of each milestone
  - Update triggers: first beta user, first paid pilot, first incident, addition of payments/regulatory scope, new
    integration domain, or material architecture change

### artifacts/releases/release-001/RELEASE_PLAN.md (required)

(missing)

### artifacts/releases/release-001/release.yaml (required)

(missing)

### ./backlog/WORK_CATALOG.yaml (optional)

(missing)

### artifacts/sprints/sprint-000/SPRINT_REPORT.md (optional)

(missing)


## Outputs


### Artifacts

- artifacts/sprints/sprint-001/SPRINT_PLAN.md
- artifacts/sprints/sprint-001/sprint.yaml
- artifacts/sprints/sprint-001/tasks.yaml


### Repo Files

- ./sprints/sprint-001/SPRINT_PLAN.md
- ./sprints/sprint-001/sprint.yaml
- ./sprints/sprint-001/tasks.yaml


## Stage Body

# core/stages/02_sprint_plan.md

<!--
Stage body intentionally minimal.
This stage should turn release slot intent into a concrete task plan.
If a previous sprint report is present, incorporate carry-over work and adjust scope.
-->
