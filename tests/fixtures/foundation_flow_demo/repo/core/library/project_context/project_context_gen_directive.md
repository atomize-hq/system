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
