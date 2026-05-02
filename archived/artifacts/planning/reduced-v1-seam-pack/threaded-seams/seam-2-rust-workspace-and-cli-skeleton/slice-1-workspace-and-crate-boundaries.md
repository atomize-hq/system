---
slice_id: S1
seam_id: SEAM-2
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to workspace membership or crate ownership boundaries after landing requires `C-02` revalidation by downstream seams.
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
contracts_produced: []
contracts_consumed:
  - C-01
  - C-02
open_remediations: []
---

### S1 - Land the Rust Workspace Scaffold and Crate Ownership Boundaries

- **User/system value**: There is one Rust workspace root and one obvious crate split so downstream seams can implement without guessing where behavior belongs.
- **Scope (in/out)**:
  - In:
    - root `Cargo.toml` workspace with explicit members
    - create `crates/cli` and `crates/compiler` scaffolds with clear module boundaries
    - minimal compile-time wiring so the CLI can call into compiler APIs (placeholders only)
  - Out:
    - implementing manifest ingest, resolver semantics, renderers, or demo execution logic
- **Acceptance criteria**:
  - Workspace builds on the reduced v1 target platforms named in `scope_brief.md`.
  - Crate boundaries match `C-02` rules (CLI owns argument parsing + command dispatch; compiler owns shared types + core logic).
  - No runtime dependency on the legacy Python harness exists or is implied.
- **Dependencies**:
  - Requires `S00` `C-02` rules to avoid drifting boundaries during implementation.
  - Requires `THR-01` / `C-01` revalidation before execution (enforced by `gates.pre_exec.revalidation` at seam level).
- **Verification**:
  - `cargo build` (workspace) succeeds; `cargo test` either succeeds or is intentionally empty with placeholder tests.
  - Lint/format baseline decisions recorded (if the repo already has Rustfmt/clippy posture, follow it).
- **Rollout/safety**:
  - Keep the scaffold honest: do not add stub behavior that reads canonical `.system/` artifacts yet (owned by `SEAM-3`/`SEAM-4`).
- **Review surface refs**: `../../review_surfaces.md` (R2 runtime boundary, R3 touch surface map)

#### S1.T1 - Create the workspace root and member crate skeletons

- **Outcome**: root `Cargo.toml` workspace + `crates/cli` + `crates/compiler` exist and compile.
- **Thread/contract refs**: `THR-02`, `C-02`

#### S1.T2 - Encode crate ownership boundaries in structure and naming

- **Outcome**: module layout makes “CLI vs compiler core” obvious and discourages cross-crate leakage.
- **Thread/contract refs**: `THR-02`, `C-02`
