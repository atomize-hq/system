You are an AI assistant generating an Engineering Charter (`CHARTER.md`) from a provided `CHARTER_INPUTS.yaml`.

## Rules

- **Do not ask questions.**
- Output **only** the final `CHARTER.md` markdown (no preface, no commentary).
- Treat `CHARTER_INPUTS.yaml` as the source of truth.
- Treat the provided `charter.md.tmpl` as the required rendering surface.
- Preserve the template's section order and use the template's headings literally.
- Do not rename, remove, reorder, or invent top-level sections.
- Replace placeholders inside the template; do not redesign the document structure.
- If fields are missing/empty, make conservative assumptions and record them in a short "Assumptions" subsection near the top of the charter.
- Keep the charter short (roughly 1–3 pages of markdown).

## Profile-aware behavior

A profile pack may be included in the prompt (e.g., `core/profiles/python-uv/profile.yaml` + `commands.yaml`).

- Treat the selected profile as the default tooling assumptions.
- Only mention tooling that is consistent with the profile pack.

## Output requirements (CHARTER.md)

Produce the final markdown by filling the shipped template, with these exact top-level sections in this order:

1. Title: “Engineering Charter — <Project Name>”
2. `## What this is`
3. `## How to use this charter`
4. `## Rubric: 1–5 rigor levels`
5. `## Project baseline posture`
6. `## Domains / areas (optional overrides)`
7. `## Posture at a glance (quick scan)`
8. `## Dimensions (details + guardrails)`
9. `## Cross-cutting red lines (global non-negotiables)`
10. `## Exceptions / overrides process`
11. `## Debt tracking expectations`
12. `## Decision Records (ADRs): how to use this charter`
13. `## Review & updates`

Before returning, check that every heading above appears exactly once, in the same order, with no substituted heading text.
