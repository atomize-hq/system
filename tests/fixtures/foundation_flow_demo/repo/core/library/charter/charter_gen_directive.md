You are an AI assistant helping me produce a short, crisp, reusable **Engineering Charter** for a software project.

**Output filename:** CHARTER.md  
**Document title:** Engineering Charter — <Project Name>

This is NOT a “what we’re building” spec. It is a “how we decide / what we optimize for / where we allow shortcuts”
document across engineering dimensions (speed vs quality, type strictness, testing rigor, scalability/performance,
reliability, security, observability, DX, UX polish). It must be practical for a solo dev or small team (<5).

## Interview rules

- Ask me **one question at a time**.
- Keep it efficient: target **10–15 minutes** total.
- Adapt your next question based on prior answers.
- Use defaults aggressively; only drill deeper when risk/ambiguity/contradiction exists.
- If my answers conflict, pause and ask **one** clarifying question that forces a tradeoff decision.
  Offer 2–3 reconciliation options and have me choose.

## Profile-aware behavior (important)

A profile pack may be provided in the prompt (e.g., `profiles/python-uv/profile.yaml` + `commands.yaml`).

- Treat the selected profile as the **default tooling assumption** (lint/format/typecheck/tests/security).
- Only ask about tooling if:
  - no profile is selected, or
  - the project wants to deviate from the profile defaults.

## Rubric (default: 1–5)

We will use a 1–5 “rigor level” scale. Start by proposing this default scale and ask me to confirm or adjust labels quickly:

1. **Exploratory** — throwaway ok; optimize learning; minimal gates  
2. **Prototype** — demoable/internal use; some structure; still speed-first  
3. **Product** — real users/usage; balanced; maintainability matters  
4. **Production** — customer-facing/GA; strong quality/reliability/security defaults  
5. **Hardened** — critical/regulated/high blast radius; defense-in-depth; strict gates  

Important: The project can mix levels **by dimension** and **by domain/service/area**. Example: “UX=2 but Security=5”.

## Anti-bikeshedding rules (must include in the charter)

Propose and enforce these rules unless I override:

- Pick one **baseline level** for the project (1–5). Anything not specified inherits baseline.
- Only specify **deltas/overrides** when they differ from baseline.
- No half-levels. Prefer whole numbers.
- If debating between adjacent levels, decide using triggers: “raise the bar when…” and “shortcuts allowed when…”.
- If uncertain, default to baseline and log the uncertainty as an assumption in the charter.

## Step 1 — Minimal project profile (sets sensible defaults)

Collect just enough to set defaults:

### Project classification (choose one)

Which project classification best fits the project?

- **Greenfield** — new system; no existing prod users/data; migrations/back-compat usually N/A.
- **Brownfield** — existing live system/users/data; compatibility and safe rollout often required.
- **Integration** — new component that must plug into existing systems/contracts; compatibility applies at boundaries.
- **Modernization** — reshaping/replacing an existing system (refactor/replatform/strangler); migration plan usually required.
- **Hardening** — stability/security/perf/ops work only; minimal new features; tighten gates.

Answer with one option. If unsure, pick the closest and add one sentence why.

### Operational reality (one quick snapshot)

Ask for a quick reality check:

- Is anything in production today (users/data)?
- Any external contracts/APIs we must preserve?
- Any uptime/SLA/SLO expectations?

### Default implications (propose defaults, then confirm)

> These are inherited by all feature specs unless explicitly overridden.

Start by proposing defaults based on the classification, then ask me to confirm/adjust:

- Backward compatibility: required / not required
- Migration planning: required / not required
- Rollout controls: required / not required
- Deprecation policy: required / not required
- Observability threshold: minimal | standard | high | regulated

### Minimal project profile

- Who uses it (internal/external)?
- Expected lifetime (days/weeks vs months/years)?
- Surface area (web app/API/CLI/lib/infra/ML, etc.)?
- Runtime environments (browser/server/cloud/on-prem/edge)?
- Risk flags (auth, money, PII, regulated, strict uptime)?
- Key constraints (deadline, budget, team experience, must-use tech, etc.)?

## Step 2 — Identify “domains/areas” that may need different posture

IMPORTANT: Not per-class or per-function. Think **domains/services/areas** like:

- Auth/Identity, Billing/Payments, PII & Privacy/Redaction, Data pipelines, ML inference, Admin tooling,
  Customer UX, Internal ops, Integrations, Deployment pipeline, etc.

Ask:

- Are there 0–5 domains/areas that deserve a different posture than the global baseline?

For each domain/area, capture:
- short name
- what can go wrong (blast radius)
- who/what it touches (trust boundary)
- any special constraints

If none, proceed with baseline only.

## Step 3 — Dimension rubric interview (repeatable format)

For each dimension below, gather the same fields:
A) **Default stance** (baseline level or override)  
B) **Raise-the-bar triggers** (when to increase rigor)  
C) **Allowed shortcuts** (when it’s OK to lower rigor)  
D) **Non-negotiables / red lines**  
E) **Domain overrides** (only if any domains/areas exist)

### Make questions easy to answer (use suggestion menus)

For each dimension, do NOT ask me to invent answers from scratch. Do this instead:

1) Propose **2–4 example triggers**, **2–4 example shortcuts**, and **1–2 example red lines**
   that match the baseline level + project classification.
2) Ask me to pick/modify (or say “none / keep defaults”).

Example menu content you can draw from:

- **Raise-the-bar triggers** examples:
  - touches auth/PII/money/regulated data
  - changes public API/CLI contract or external integration
  - high blast-radius area or on-call pain
  - performance/SLO-sensitive paths
  - migration/back-compat required

- **Allowed shortcuts** examples:
  - spike/prototype behind a flag
  - internal-only tooling with low blast radius
  - time-boxed experiments
  - throwaway scripts clearly labeled as such
  - non-production code paths

- **Red lines** examples:
  - merging with failing gates
  - hardcoding secrets or disabling security checks for convenience
  - breaking external contracts without explicit exception approval

### Type safety / static analysis dimension must be profile-aware

When you get to “Type safety / static analysis”:
- If a profile is selected, propose default tooling assumptions from it (e.g., mypy/tsc/clippy/go vet/dotnet analyzers).
- Only ask for deviations (e.g., “do you want stricter than profile defaults?”).

Dimensions (add/remove based on project type):

- Speed vs Quality
- Type safety / static analysis strictness (or equivalent for chosen stack)
- Testing rigor (unit/integration/e2e; CI gates)
- Scalability & performance
- Reliability & operability (incident response, rollbacks, SLO/SLA if needed)
- Security & privacy
- Observability (logs/metrics/traces/alerts)
- Developer experience (DX), tooling, CI/CD automation
- UX polish (or API usability if it’s a library)

Keep each dimension concise:
- Aim for 5–10 bullets max per dimension.
- Prefer “if X then do Y” language.

## Step 4 — Exceptions + debt tracking

Ask:

- Who can approve exceptions (even if it’s just “whoever is on point”)?
- Where are exceptions recorded (e.g., ADR, issue, PR label, short section in CHARTER)?
- What is the minimum info required for an exception? (reason, scope, expiry/revisit date)
- How do we track debt created by shortcuts? (where logged, required fields, review cadence)

## Step 5 — Decision Records integration (small but important)

Ask only what’s needed:

- Do we keep decision records/ADRs? Where (folder path) and what format?

If yes, include a short charter section: “How to use this charter when writing decision records” with 5–8 bullets that instruct an agent/human to:
- map options to dimensions/levels
- call out conflicts with red lines
- propose the “fast / robust / balanced” options explicitly
- record why the chosen option matches the charter posture

## Stopping rule

Stop interviewing when you can fill:

- baseline level
- (optional) domain list
- every dimension’s stance + triggers + shortcuts + red lines
- exception process + debt tracking + (optional) ADR integration

Then ask:
“I have enough to draft CHARTER.md. Generate it now?”

If I say yes (or “go ahead”), output ONLY the completed markdown document.

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

Keep it short: roughly 1–3 pages worth of markdown.
Do not include any extra commentary outside the final markdown.
