---
slice_id: S00
seam_id: SEAM-2
slice_kind: contract_definition
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any rename of supported verbs, crate ownership, package layout, or CLI help hierarchy after `C-02` is published.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-02
contracts_produced:
  - C-02
contracts_consumed:
  - C-01
open_remediations: []
---

### S00 - Define `C-02` Rust Workspace and CLI Command-Surface Contract

- **User/system value**: Downstream seams can plan and implement against one explicit Rust workspace + CLI command-surface truth (crate split + verbs + help posture) without guessing or drifting.
- **Scope (in/out)**:
  - In:
    - write the concrete `C-02` rules (workspace membership, crate ownership boundaries, CLI verb hierarchy, help posture)
    - define revalidation triggers + compat expectations for consumers (`SEAM-3` through `SEAM-7`)
    - define minimal stability guarantees (what may change without a contract revision vs what requires a new version)
  - Out:
    - canonical artifact manifest semantics (`C-03`)
    - resolver behavior + refusal taxonomy (`C-04`)
    - renderer ordering and copy (`C-05`)
    - demo execution boundary semantics (`C-06`)
- **Acceptance criteria**:
  - `C-02` exists as a concrete artifact (doc + stable location) referenced by CLI help and/or root docs as appropriate.
  - Contract includes a verification checklist that can later mark `gates.pre_exec.contract = passed` for `SEAM-2`.
  - Contract explicitly inherits `C-01` runtime-boundary posture (legacy harness is reference-only; not a supported runtime dependency).
- **Dependencies**:
  - Inputs: `C-01` (`../../threaded-seams/seam-1-approved-surface-and-legacy-freeze/seam.md` + future closeout), `PLAN.md` verb vocabulary and install target constraints, `../../threading.md` contract registry for `C-02`
  - Upstream must publish: `THR-01` / `C-01` before `SEAM-2` execution is legal (captured by `gates.pre_exec.revalidation`)
- **Verification**:
  - Document-level: a downstream seam planner can answer “which crate owns which behavior” and “which verbs exist and in what order” without reading code.
  - Spot-check: `threading.md` contract registry definition for `C-02` matches the contract nouns/verbs and is referenced consistently.
- **Rollout/safety**:
  - Treat `C-02` as a public stability boundary for the pack: avoid “silent” verb or crate-boundary changes.
  - Keep `gates.pre_exec.revalidation` explicitly blocking until `SEAM-1` closeout publishes `C-01`, and require revalidation to be marked `passed` before `SEAM-2` becomes `exec-ready`.
- **Review surface refs**: `../../review_surfaces.md` (R1 operator workflow, R3 touch surface map)

For `C-02`, the contract must be concrete enough that the producer seam can later satisfy `gates.pre_exec.contract` without requiring downstream behavior seams (`SEAM-3` through `SEAM-7`) to have landed.

#### S00.T1 - Choose the canonical contract artifact location and format

- **Outcome**: `C-02` has one repo-relative home (example: `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`) and is referenced where drift would otherwise occur (CLI help, docs index, or root plan anchors).
- **Thread/contract refs**: `THR-02`, `C-02`

#### S00.T2 - Write the contract rules (normative)

- **Outcome**: A rules section with MUST/MUST NOT/SHOULD language covering:
  - workspace membership and crate split (`crates/cli` vs `crates/compiler`) and what “ownership” means (where parsing, IO, selection logic, rendering logic must live)
  - supported verbs and hierarchy: `setup`, `generate`, `inspect`, `doctor`
  - help posture: setup-first; explicit “what exists today vs planned” language (avoid implying downstream features landed early)
  - local install target assumptions for reduced v1 (as recorded in `PLAN.md` / scope brief)
  - compatibility and revalidation triggers for consumers
- **Thread/contract refs**: `THR-02`, `C-02` (and `C-01` as an upstream boundary constraint)

#### S00.T3 - Add a verification checklist for the contract gate

- **Outcome**: A checklist that can later be used to mark `gates.pre_exec.contract = passed`, including:
  - `cargo build` / `cargo test` basics for the workspace
  - `cli --help` (or equivalent) showing verb skeleton + setup-first ordering
  - crate-boundary spot checks (no resolver logic in CLI; no CLI arg parsing in compiler)
  - explicit statement that legacy harness is not a supported runtime dependency
- **Thread/contract refs**: `THR-02`, `C-02`
