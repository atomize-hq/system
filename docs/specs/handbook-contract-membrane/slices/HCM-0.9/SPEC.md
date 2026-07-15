# HCM-0.9 Specification: Abandoned

**Status:** abandoned; execution prohibited

**Canonical catalog:** `../../05-contracts-schemas-and-gates.md`

**Terminal evidence checkpoint:** `f3a33ddb55443d37f3a51ffb58f1c85b74a28b23`

## Decision

The human-authorized simplified decomposition plan received Redesign Review 1,
one remediation, and terminal fresh Redesign Review 2. Review 2 was not CLEAN:
the negative proof required a forbidden runtime **or** HCM-0.5 fixture rather
than two distinct fixtures covering both categories.

The authorized result is abandonment:

- do not remediate the terminal finding;
- do not run Redesign Review 3;
- do not decompose `05`;
- do not create catalog leaves, an index cutover, or a decomposition verifier;
- do not build an automatic semantic routing engine;
- retain the monolithic `05-contracts-schemas-and-gates.md` as canonical;
- do not execute HCM-0.9 or start HCM-0.5 in this session.

## Historical evidence

The rejected subjects remain immutable, non-authoritative evidence:

- initial stopped planning checkpoint: `754970ef141c4e9efcce8ca6e76c683dbb248a11`;
- continuation checkpoint: `5e3d5ba82af32087916e8ab53573c3797c87a86c`;
- simplified redesign checkpoint: `f3a33ddb55443d37f3a51ffb58f1c85b74a28b23`;
- terminal redesign fingerprint:
  `sha256:c0a719f7d35f7eff0ce73cb008baf2593b73ef43001fbf5068b902f46492451c`;
- Redesign Review 1 dispatch:
  `../../handoffs/dispatches/20260715T184659Z--HCM-0-9--redesign-planning-review-1.json`;
- terminal Redesign Review 2 dispatch:
  `../../handoffs/dispatches/20260715T185741Z--HCM-0-9--redesign-planning-review-2.json`.

The prior audit/inventory, execution plan, and checklist are tombstones in the
current tree. Their exact rejected bytes remain recoverable from the evidence
commits above.

## Surviving manifest rule

Review subject manifests contain only files whose bytes are under review.
Changed files are manifested when their bytes are reviewed. Unchanged
contextual authority remains in `authority_refs` and/or
`contracts_and_gates`; it is never added merely because a reviewer reads it.

## Revival boundary

HCM-0.9 has no execution selector and no automatic resume. Any future catalog
decomposition requires a new explicit human decision, a new slice/packet rather
than a repair of this one, and a fresh review budget. Until then, the monolith
is the only current catalog authority.
