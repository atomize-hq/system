---
contract_id: C-05
seam_id: SEAM-5
owner_seam: SEAM-5
version: reduced-v1
currentness: current
status: published
revalidation_triggers:
  - Any change to markdown, JSON, or inspect surface semantics or required fields.
  - Any change to trust-header field set, ordering, or omission rules.
  - Any change to proof ordering, stable sort keys, or fallback surface behavior.
  - Any change to renderer failure isolation or the rule that renderer views stay pure over `C-04`.
  - Any change to accessibility or narrow-terminal output requirements.
---

# C-05 Renderer and Proof Surfaces Contract

## Purpose

This contract defines the reduced-v1 renderer and proof-surface truth for `SEAM-5`.

It exists so downstream seams, especially `SEAM-7`, can treat markdown, JSON, inspect, and doctor output as deterministic view layers over typed compiler-owned result models without recomputing semantics.

`C-04` remains authoritative for doctor baseline-readiness semantics, blocker meaning, and ordering. `C-05` is authoritative only for how those truths are rendered and ordered for operator and machine consumption.

## Canonical Location

- Canonical artifact: `docs/contracts/C-05-renderer-and-proof-surfaces.md`
- Downstream seams that consume this contract: `SEAM-7`
- Derived consumers (read-only): `SEAM-6`

## Owned Surface

`C-05` is authoritative about the **rendered surfaces** that present the `C-04` resolver result:

- Markdown packet output for operator-facing review.
- JSON packet output for machine-facing consumption.
- Inspect proof output for evidence review and troubleshooting.
- Trust-header layout and field ordering.
- Proof ordering rules and stable sort keys.
- JSON fallback behavior when evidence is dense or otherwise too large for a narrow terminal view.
- Renderer failure isolation from a successful typed resolver result.
- Accessibility and narrow-terminal readability rules for the rendered surfaces.

`C-05` is **not** authoritative about:

- How refusals or blockers are defined, categorized, or prioritized; `C-04` owns that meaning.
- How packets are selected or why the resolver refused; `C-04` owns that truth.
- Conformance rails or downstream verification policy; `SEAM-7` owns that.

## Normative Rules

### Pure-view boundary

- The markdown, JSON, and inspect renderers MUST be pure views over the typed `C-04` resolver result.
- The renderers MUST NOT recompute refusals, blockers, budget decisions, selection reasons, or freshness truth.
- The renderers MUST NOT derive alternate semantic meaning from presentation order, formatting, or display width.
- The renderers MUST preserve the `C-04` meaning of every refusal, blocker, budget outcome, and decision-log entry they display.

### Markdown surface

- Markdown output MUST be deterministic for identical `C-04` inputs.
- Markdown output MUST begin with a trust header before the body of the packet view.
- Markdown output MUST present the operator-facing facts in this order:
  1. outcome
  2. object
  3. next safe action
- Markdown output MUST remain compact and narrow-terminal-friendly.
- Markdown output MUST avoid burying the outcome/object/next-action facts behind long prose, decorative spacing, or unstable wrapping assumptions.
- Markdown output MUST reference refusal or blocker semantics by name only when those semantics are already present in `C-04`; it MUST NOT restate or reinterpret them.

### JSON surface

- JSON output MUST be a deterministic serialization of the typed `C-04` resolver result.
- JSON output MUST NOT recompute semantic fields or emit alternate interpretations of the resolver truth.
- JSON output MUST preserve the same typed meaning as the markdown and inspect views.
- JSON output MUST be stable across identical inputs, including the ordering of ordered collections and proof payloads.
- JSON output MUST provide a machine-readable fallback when dense evidence would make a terminal-friendly surface insufficient on its own.
- JSON output MUST keep `packet_result` present for ready and non-ready outcomes alike.
- Packet body sections inside `packet_result` MUST be serialized only for ready outcomes.
- For blocked or refused outcomes, JSON output MUST redact packet body sections rather than serializing body contents through the fallback surface.
- Non-ready redaction MUST preserve refusal, blocker, budget, and packet metadata evidence so clients can explain why no packet body is available.

### Inspect surface

- Inspect output MUST present proof evidence in a stable, contract-defined order.
- Inspect output MUST explain inclusion, exclusion, freshness, and budget decisions using the typed `C-04` evidence only.
- Inspect output MUST NOT change meaning based on incidental iteration order or formatter behavior.
- Inspect output MUST remain suitable for evidence review without requiring rerun of resolver logic.

## Trust Header Rules

The trust header is the first operator-facing summary block in the rendered surfaces.

### Required fields

The trust header MUST include:

- `outcome`
- `object`
- `next safe action`

The trust header MAY include additional fields only if they do not obscure the three required facts and do not change the canonical ordering below.

### Deterministic ordering

The trust header MUST use this field order:

1. `outcome`
2. `object`
3. `next safe action`

If additional fields are present, they MUST appear after the required fields and MUST use a deterministic order defined by the renderer implementation and frozen for the version of this contract.

## Proof Ordering Rules

Proof surfaces MUST be stable and deterministic.

### Stable sort keys

Proof items MUST be ordered using explicit stable sort keys, in this precedence:

1. Contract-defined proof group or surface kind.
2. Primary subject identity from `C-04`.
3. Secondary evidence identity, if present.
4. Deterministic tie-breaker derived from a stable identifier, never from map iteration, filesystem order, or wall-clock time.

### Ordering constraints

- Proof ordering MUST NOT depend on hash map traversal, set traversal, directory listing order, or platform-specific sort quirks.
- Proof ordering MUST remain identical for identical `C-04` inputs.
- Proof ordering MUST be consistent across markdown, JSON, and inspect views when the same proof items are surfaced.

## Dense Evidence Fallback

- When the evidence set is dense enough that a narrow-terminal markdown or inspect view would become unreadable, the renderer MUST provide a machine-readable fallback surface.
- The fallback MUST preserve the same typed `C-04` meaning and MUST NOT drop required proof facts.
- The fallback MUST be deterministic and MUST preserve the same proof ordering rules as the primary view.
- The fallback MUST remain discoverable from the primary surface so operators can find the underlying evidence without guessing.

## Renderer Failure Isolation

- Renderer failures MUST NOT erase or invalidate a successfully resolved typed `C-04` result.
- If one rendered surface fails, the implementation MUST preserve the typed resolver result and MUST allow other surfaces to render independently when possible.
- A rendering failure MUST be reported as a presentation failure, not as a semantic change to the underlying resolver result.
- The implementation MUST keep failure isolation deterministic so the same `C-04` input produces the same success or failure boundary for each surface.

## Accessibility and Narrow-Terminal Rules

- Markdown output MUST be readable in a narrow terminal without relying on wide horizontal scanning.
- The first trust-header facts MUST remain visible before long evidence blocks or verbose tables.
- Line wrapping MUST not alter semantic order or hide the trust header.
- Rendering MUST favor plain, scannable labels over decorative formatting that obscures the operator decision path.
- Any visual emphasis MUST remain secondary to the ordered facts and MUST NOT change the meaning of the output.

## Compatibility and Downstream Revalidation

- Any change to surface semantics, trust-header ordering, proof ordering, fallback behavior, or failure isolation MUST trigger downstream revalidation.
- Any change that would require `SEAM-7` to reinterpret rendered output as anything other than a pure view over `C-04` MUST be treated as a contract revision.
- Any change that broadens renderer inputs beyond the typed `C-04` result MUST be rejected or revalidated.

## Verification Checklist

- [ ] Markdown, JSON, and inspect are documented as pure views over the typed `C-04` result.
- [ ] The contract explicitly delegates refusal and blocker meaning to `C-04`.
- [ ] The trust header includes exactly the required facts and keeps the required order.
- [ ] Proof ordering uses stable sort keys and does not depend on incidental iteration order.
- [ ] JSON fallback behavior for dense evidence is explicit and machine-readable.
- [ ] Renderer failure isolation preserves a successful typed resolver result.
- [ ] Narrow-terminal and accessibility rules keep the trust header and operator action visible first.
- [ ] `SEAM-7` can verify rendered output without re-running resolver logic.
- [ ] `docs/README.md` links to `C-05` alongside the other reduced-v1 contracts.
