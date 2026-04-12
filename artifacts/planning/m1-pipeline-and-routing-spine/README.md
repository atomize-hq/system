# M1 Pipeline And Routing Spine - seam extraction

Source: `PLAN.md` (`M1. Pipeline And Routing Spine`)

This pack captures seam briefs, authoritative threading, pack-level review surfaces, seam-exit intent, and governance scaffolds for the `M1` pipeline-and-routing milestone. It intentionally stays one level above seam-local decomposition.

- Start here: `scope_brief.md`
- Seam overview: `seam_map.md`
- Threading: `threading.md`
- Pack review surfaces: `review_surfaces.md`
- Governance: `governance/remediation-log.md`

Execution horizon:

- Active seam: `SEAM-3`
- Next seam: `SEAM-4`
- Landed seams outside the forward window: `SEAM-1`, `SEAM-2`

Policy:

- only the active seam is eligible for authoritative downstream sub-slices by default
- the next seam may later receive seam-local review + slices, and only provisional candidate-subslice hints
- `SEAM-3` now owns the active compile-boundary planning window after `SEAM-2` published `C-09` and `THR-02`
- active and next seams must eventually terminate in a dedicated final `S99` `seam-exit-gate` slice once seam-local planning begins
- seams that own undefined contracts may reserve `S00` as a contract-definition boundary slice once seam-local planning begins
- future seams remain seam briefs
- canonical contract docs live in `docs/contracts/` and must remain descriptive-only
