# Reduced V1 Rust-First CLI Cutover - seam extraction

Source: `PLAN.md`

This pack captures seam briefs, authoritative threading, pack-level review surfaces, seam-exit intent, and governance scaffolds for the reduced v1 implementation plan. Seam-local pre-exec planning (when present) lives under `threaded-seams/`.

- Start here: `scope_brief.md`
- Seam overview: `seam_map.md`
- Threading: `threading.md`
- Pack review surfaces: `review_surfaces.md`
- Governance: `governance/remediation-log.md`
- Seam-local planning: `threaded-seams/`

Execution horizon:

- Active seam: `SEAM-4`
- Next seam: `SEAM-5`

Policy:

- only the active seam is eligible for authoritative downstream sub-slices by default
- the next seam may later receive seam-local review + slices, and only provisional candidate-subslice hints if its basis remains stable enough
- `SEAM-2` is expected to need authoritative contract-carrying work, not merely a spike-grade provisional seam, so this extractor does not pre-seed candidate subslices
- active and next seams must eventually terminate in a dedicated final `S99` `seam-exit-gate` slice once seam-local planning begins
- seams that own undefined contracts may reserve `S00` as a contract-definition boundary slice once seam-local planning begins
- future seams remain seam briefs
