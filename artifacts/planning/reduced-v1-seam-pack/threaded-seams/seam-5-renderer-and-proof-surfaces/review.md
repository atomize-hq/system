---
seam_id: SEAM-5
review_phase: pre_exec
execution_horizon: active
basis_ref: seam.md#basis
---

# Review Bundle - SEAM-5 Renderer and Proof Surfaces

This artifact feeds `gates.pre_exec.review`.

## Falsification questions

- Can markdown/JSON/inspect drift into different semantics (e.g., different omission reasons, different refusal meaning) because they are implemented independently?
- Can trust header / proof ordering become unstable across runs due to accidental iteration order?
- Can renderer failures destroy a successfully resolved typed packet, instead of isolating failures and preserving the typed resolver result?
- Can narrow-terminal output bury the “outcome, object, next action” facts behind formatting noise?

## Guardrails

- Renderers MUST be pure views over the typed resolver result (`C-04`).
- Renderers MUST NOT recompute refusals, blockers, or budget decisions.
- Output ordering MUST be deterministic and derived from explicit ordering rules and stable sort keys.

