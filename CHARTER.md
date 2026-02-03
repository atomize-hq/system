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
