---
seam_id: SEAM-2
seam_slug: rust-workspace-and-cli-skeleton
status: exec-ready
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-2-rust-workspace-and-cli-skeleton.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts:
    - SEAM-1
  required_threads:
    - THR-01
  stale_triggers:
    - Any change to the approved root surface or supported verb vocabulary after SEAM-1 lands.
    - Any change to the desired crate split or local-install target matrix in PLAN.md.
gates:
  pre_exec:
    review: passed
    contract: passed
    revalidation: passed
  post_exec:
    landing: pending
    closeout: pending
seam_exit_gate:
  required: true
  planned_location: S99
  status: pending
open_remediations: []
---

# SEAM-2 - Rust Workspace and CLI Skeleton

## Seam Brief (Restated)

- **Goal / value**: Establish the Rust workspace, library/CLI crate split, and supported command hierarchy so downstream seams have one authoritative ownership map.
- **Type**: platform
- **Scope**
  - In:
    - root `Cargo.toml` workspace
    - `crates/compiler`
    - `crates/cli`
    - initial shared packet-result and decision-log type scaffolding (placeholders only)
    - CLI help posture and supported verb skeleton (`setup`, `generate`, `inspect`, `doctor`)
  - Out:
    - canonical artifact manifest behavior (`SEAM-3`)
    - packet selection logic + doctor semantics (`SEAM-4`)
    - renderer logic (`SEAM-5`)
    - execution demo implementation (`SEAM-6`)
- **Touch surface**:
  - `Cargo.toml`, `Cargo.lock`
  - `crates/cli/`, `crates/compiler/`
  - command parsing surfaces + help text
- **Verification**:
  - The crate boundaries make ownership obvious (CLI vs compiler core) and do not imply Python runtime coupling.
  - For the owned contract (`C-02`), the command surface and crate split are concrete enough (rules + verification checklist) that downstream seams can plan without waiting for downstream behavior to land.
  - Avoid requiring “published contract artifact exists” as a pre-exec input; reserve publication evidence for `S99` and closeout.
- **Basis posture**:
  - Currentness: current (revalidated against `SEAM-1` closeout + published `C-01` wording)
  - Upstream closeouts assumed: `SEAM-1`
  - Required threads: `THR-01` is revalidated (`C-01` is published with closeout evidence)
  - Stale triggers:
    - Any change to the approved root surface or supported verb vocabulary after `SEAM-1` lands.
    - Any change to the desired crate split or local-install target matrix in `PLAN.md`.
- **Threading constraints**
  - Upstream blockers: `SEAM-1` (consume `C-01` via `THR-01`)
  - Downstream blocked seams: `SEAM-3`, `SEAM-4`, `SEAM-5`, `SEAM-6`, `SEAM-7` (consume `C-02` via `THR-02`)
  - Contracts produced: `C-02`
  - Contracts consumed: `C-01`

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Why this seam needs an explicit exit gate**: `C-02` is the ownership and command-surface contract that every downstream capability seam consumes; downstream promotion must have a closeout-backed signal that the workspace + verb surface are now publishable truth.
- **Expected contracts to publish**: `C-02`
- **Expected threads to publish / advance**: `THR-02` (publish); downstream `THR-07` (conformance) consumes the published `C-02` alongside other contracts later.
- **Likely downstream stale triggers**:
  - any rename of supported verbs or changes to verb hierarchy/help posture
  - any change to crate ownership boundaries (`crates/cli` vs `crates/compiler`)
  - any change to install target assumptions that affects CLI commands or packaging posture
- **Expected closeout evidence**:
  - landed workspace scaffold (workspace members + crate layout) with build + basic CLI help evidence
  - recorded `C-02` contract rules + verification checklist
  - recorded downstream revalidation requirements for `SEAM-3` through `SEAM-7` if contract deltas occurred

## Slice index

- `S00` -> `slice-00-c-02-rust-workspace-and-cli-command-surface-contract.md`
- `S1` -> `slice-1-workspace-and-crate-boundaries.md`
- `S2` -> `slice-2-cli-command-skeleton-and-help-posture.md`
- `S3` -> `slice-3-shared-types-skeleton-for-downstream-seams.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-2-closeout.md`
