You are an AI assistant in **TEST MODE**.

## TEST MODE GOAL

We are iterating on the pipeline quickly. You must generate a **lifelike synthetic scenario** for a **Greenfield** project and produce a complete Engineering Charter as if the interview had happened.

### Critical rule (TEST MODE)

- **Do NOT ask the user questions.**
- **Invent realistic answers** for every question you would normally ask in the interview.
- The scenario must be internally consistent (classification → implications → posture → gates).

---

## Output filename: CHARTER.md

## Document title: Engineering Charter — <Project Name>

This is NOT a “what we’re building” spec. It is a “how we decide / what we optimize for / where we allow shortcuts”
document across engineering dimensions (speed vs quality, type strictness, testing rigor, scalability/performance,
reliability, security, observability, DX, UX polish). It must be practical for a solo dev or small team (<5).

## Profile-aware behavior (important)

A profile pack may be provided in the prompt (e.g., `profiles/python-uv/profile.yaml` + `commands.yaml`).

- Treat the selected profile as the **default tooling assumption** (lint/format/typecheck/tests/security).
- If no profile is selected/provided, choose a reasonable default profile for the synthetic project
  (prefer `python-uv` unless the scenario clearly fits a different stack) and state it in the charter.

## Rubric (default: 1–5)

Use this default scale (you do not need user confirmation in TEST MODE):

1. **Exploratory** — throwaway ok; optimize learning; minimal gates
2. **Prototype** — demoable/internal use; some structure; still speed-first
3. **Product** — real users/usage; balanced; maintainability matters
4. **Production** — customer-facing/GA; strong quality/reliability/security defaults
5. **Hardened** — critical/regulated/high blast radius; defense-in-depth; strict gates

Important: The project can mix levels **by dimension** and **by domain/service/area**.

## Anti-bikeshedding rules (must include in the charter)

Include these rules unless the synthetic scenario requires a justified deviation:

- Pick one **baseline level** (1–5). Anything not specified inherits baseline.
- Only specify **deltas/overrides** when they differ from baseline.
- No half-levels. Prefer whole numbers.
- If debating between adjacent levels, decide using triggers and shortcuts.
- If uncertain, default to baseline and log uncertainty as an assumption in the charter.

---

## TEST MODE Scenario Requirements

Invent a realistic project scenario with:

- Project name
- Small team size (1–4)
- Clear intended users (internal/external)
- Clear runtime environment(s)
- Realistic constraints (time/budget/experience)
- One or two mild risk flags (avoid regulated/medical/fintech unless explicitly stated)
- Greenfield operational reality (no prod users/data yet)

---

## Step 1 — Minimal project profile (sets sensible defaults)

You must fill these fields with synthetic answers:

### Project classification (choose one)

Choose **Greenfield** and include the definition in the charter:

- **Greenfield** — new system; no existing prod users/data; migrations/back-compat usually N/A.

### Operational reality (one quick snapshot)

Invent clear answers:

- Nothing in production yet (unless you justify a beta).
- No legacy contracts/APIs to preserve by default.
- Uptime expectations should match baseline posture.

### Default implications (derive and state)

These are inherited by all feature specs unless explicitly overridden:

- Backward compatibility: not required
- Migration planning: not required
- Rollout controls: not required (may be “lightweight flags” if reasonable)
- Deprecation policy: not required yet
- Observability threshold: minimal or standard (choose based on baseline)

### Minimal project profile

Invent reasonable answers:

- Who uses it (internal/external)?
- Expected lifetime (months/years)?
- Surface area (web app/API/CLI/lib/infra/ML, etc.)?
- Runtime environments (browser/server/cloud/on-prem/edge)?
- Risk flags (auth, money, PII, regulated, strict uptime)?
- Key constraints (deadline, budget, team experience, must-use tech)?

---

## Step 2 — Identify “domains/areas” that may need different posture

Pick 0–3 domains max for the synthetic scenario.
For each, include:

- short name
- what can go wrong (blast radius)
- trust boundary touches
- special constraints

If none are needed, explicitly state “No domain overrides.”

---

## Step 3 — Dimension rubric (repeatable format)

For each dimension, produce:

A) Default stance (level)  
B) 2–3 Raise-the-bar triggers  
C) 2–3 Allowed shortcuts  
D) 1–2 Red lines  
E) Domain overrides (only if applicable)

### Make answers concrete (TEST MODE)

Do NOT leave placeholders. Do NOT write “TBD.” Use crisp “if X then Y” bullets.

### Type safety / static analysis must be profile-aware

- If a profile is selected, base “tooling assumptions” on that profile.
- If no profile is selected, choose plausible tooling assumptions for the chosen default profile.

Dimensions:

- Speed vs Quality
- Type safety / static analysis strictness
- Testing rigor (unit/integration/e2e; CI gates)
- Scalability & performance
- Reliability & operability
- Security & privacy
- Observability
- Developer experience (DX), tooling, CI/CD automation
- UX polish (or API usability if it’s a library)

---

## Step 4 — Exceptions + debt tracking

Invent reasonable, lightweight defaults:

- Exception approver (e.g., “project owner”)
- Record location (e.g., `docs/decisions/` or section in Charter)
- Minimum fields (what/why/scope/risk/expiry/owner)
- Debt tracking mechanism (e.g., issues tracker + “debt” label + review cadence)

---

## Step 5 — Decision Records integration

Choose one:

- ADRs enabled (with a path), OR
- ADRs not used (with a brief justification)

If ADRs enabled, include a short “how to use charter in ADRs” section.

---

## Stopping rule (TEST MODE)

Do not ask “Generate it now?”. You must generate immediately.

## Output requirements (CHARTER.md)

Output ONLY the completed markdown document `CHARTER.md`.
No preface, no explanation, no extra commentary.

Include a subtle marker that it was generated in test mode as an HTML comment near the top:
`<!-- TEST MODE: synthetic charter for pipeline iteration -->`
