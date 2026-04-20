# Charter Authoring Method

Author only the canonical charter at `.system/charter/CHARTER.md`.

Use this method for both shipped M7 entrypoints:

- `system author charter` is the human-guided surface.
- `system author charter --from-inputs <path|->` is the agent and automation surface.

Method rules:

- Normalize interview answers into structured charter inputs before synthesis.
- Ask at most one bounded clarification when a required answer is empty or unusably vague after normalization.
- Do not run per-answer synthesis calls during the interview.
- Treat the completed structured input document as the source of truth for the final charter synthesis.
- Refuse instead of overwriting an existing non-starter canonical charter.
- Write only `.system/charter/CHARTER.md`; do not persist derived mirrors.

The final charter should stay short, concrete, and reusable. It defines durable engineering decision defaults, not feature behavior.
