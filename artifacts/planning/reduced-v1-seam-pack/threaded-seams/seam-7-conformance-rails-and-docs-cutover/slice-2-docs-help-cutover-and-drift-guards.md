---
slice_id: S2
seam_id: SEAM-7
slice_kind: documentation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to supported-vs-legacy wording or help examples requires revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-07
contracts_produced: []
contracts_consumed:
  - C-01
  - C-02
  - C-05
  - C-06
open_remediations: []
---

### S2 - Docs/Help Cutover and Drift Guards

- **User/system value**: A cold reader sees one supported Rust-first story, and docs/help cannot drift away from runtime behavior without failing checks.
- **Scope (in/out)**:
  - In:
    - update docs/help examples to match the CLI surface and refusal semantics
    - add drift checks (where appropriate) so help output and docs remain aligned
  - Out:
    - adding new runtime capabilities

- **Acceptance criteria**:
  - Root-facing docs and CLI help reflect the supported reduced-v1 story and do not imply Python support (`C-01`).
  - Docs link to the authoritative contracts they rely on (`C-01..C-07`) and do not restate conflicting semantics.
  - A drift guard exists (test or script) for at least one high-risk docs/help surface (e.g., help text or a short excerpt) so it fails fast on mismatch.

#### Execution checklist (planning-only)

- Re-scan `README.md` and `docs/README.md` for:
  - any Python-supported wording that contradicts `C-01`
  - any implication of live slice execution (contradicts reduced v1 boundary; `C-06`)
- Ensure help examples (in docs or README) match the verbs and ordering in `C-02`.
- Add a minimal drift guard (snapshot/golden or scripted) for a high-risk surface:
  - `system --help` (or verb help) excerpt, or
  - a short “supported story” paragraph that must remain aligned with `C-01`
