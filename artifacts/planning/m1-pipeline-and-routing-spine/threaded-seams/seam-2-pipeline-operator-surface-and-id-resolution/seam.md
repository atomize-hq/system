---
seam_id: SEAM-2
seam_slug: pipeline-operator-surface-and-id-resolution
status: exec-ready
execution_horizon: active
plan_version: v1
basis:
  currentness: current
  source_seam_brief: ../../seam-2-pipeline-operator-surface-and-id-resolution.md
  source_scope_ref: ../../scope_brief.md
  upstream_closeouts:
    - ../../governance/seam-1-closeout.md
  required_threads:
    - THR-01
  stale_triggers:
    - If `SEAM-1` changes route status names, reason payloads, activation semantics, route ordering, or repo-safe path rules, this seam must revalidate before execution.
    - If `SEAM-1` changes `.system/state/pipeline/` schema or mutation semantics, this seam must revalidate `pipeline resolve` and `pipeline state set` behavior before execution.
    - If `C-02` changes supported top-level command posture or help-order rules, this seam must revalidate naming and help-surface assumptions before execution.
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

# SEAM-2 - Pipeline Operator Surface and ID Resolution

## Seam Brief (Restated)

- **Goal / value**: Expose one supported `pipeline` operator surface that lets the operator list, inspect, resolve, and mutate pipeline state without reinterpreting compiler truth in CLI-only code paths.
- **Type**: platform
- **Scope**
  - In:
    - `pipeline list`, `pipeline show`, `pipeline resolve`, and `pipeline state set`
    - canonical pipeline and stage ids plus unambiguous shorthand lookup rules
    - ambiguity and unknown-id refusal classes with explicit operator recovery guidance
    - normalized default render contracts for declared config and resolved route output
    - help posture for the shipped M1 `pipeline` subset
  - Out:
    - `pipeline compile` as a shipped M1 help/docs surface
    - raw file-path targeting as a first-class operator input
    - compile payload semantics, output materialization, or downstream packet generation behavior
- **Touch surface**:
  - `crates/cli/src/main.rs`
  - CLI parsing, help, and integration tests under `crates/cli/tests/`
  - `README.md`
  - `docs/CLI_PRODUCT_VOCABULARY.md`
  - `docs/CLI_COMMAND_HIERARCHY.md`
  - `docs/SUPPORTED_COMMANDS.md`
- **Verification**:
  - The operator surface must consume published `C-08` outputs from `crates/compiler` without duplicating route or state logic in CLI code.
  - Ambiguous shorthand, unknown canonical ids, and malformed operator input must remain distinct refusal classes with explicit recovery guidance.
  - Help, tests, and proof-facing render contracts must agree on the shipped M1 subset: `list`, `show`, `resolve`, and `state set`.
- **Canonical contract refs**:
  - `docs/contracts/pipeline-operator-surface-and-id-resolution.md`
  - `docs/contracts/pipeline-route-and-state-core.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- **Basis posture**:
  - Currentness: current (revalidated against `SEAM-1` closeout, published `C-08`, and the current CLI contract posture in `C-02`)
  - Upstream closeouts assumed:
    - `../../governance/seam-1-closeout.md`
  - Required threads:
    - `THR-01`
  - Stale triggers:
    - If `SEAM-1` changes route status names, reason payloads, activation semantics, route ordering, or repo-safe path rules, this seam must revalidate before execution.
    - If `SEAM-1` changes `.system/state/pipeline/` schema or mutation semantics, this seam must revalidate `pipeline resolve` and `pipeline state set` behavior before execution.
    - If `C-02` changes supported top-level command posture or help-order rules, this seam must revalidate naming and help-surface assumptions before execution.
- **Threading constraints**
  - Upstream blockers: none beyond the already published `SEAM-1` closeout and `THR-01`
  - Downstream blocked seams: `SEAM-3`, `SEAM-4`
  - Contracts produced: `C-09`
  - Contracts consumed:
    - `C-08`
    - `C-02`
  - Canonical contract refs:
    - `docs/contracts/pipeline-operator-surface-and-id-resolution.md`
    - `docs/contracts/pipeline-route-and-state-core.md`
    - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`

## Review bundle

- `review.md` is the authoritative artifact for `gates.pre_exec.review`.

## Seam-exit gate plan

- **Planned location**: `S99`
- **Why this seam needs an explicit exit gate**: `C-09` and `THR-02` are the operator-facing handoff for the compile-boundary and conformance seams. Promotion must consume one closeout-backed signal that CLI behavior, help posture, and refusal/render contracts are aligned with the published compiler truth.
- **Expected contracts to publish**: `C-09`
- **Expected threads to publish / advance**: `THR-02`
- **Likely downstream stale triggers**:
  - any change to supported `pipeline` subcommands, help posture, or default render wording
  - any change to canonical-id or shorthand ambiguity semantics
  - any change to how `pipeline resolve` or `pipeline state set` surface compiler-owned refusal classes
- **Expected closeout evidence**:
  - landed `C-09` contract text and evidence links
  - CLI command handlers and help evidence for `list`, `show`, `resolve`, and `state set`
  - tests proving canonical-id lookup, ambiguity refusal, normalized render contracts, and supported help posture
  - downstream revalidation triggers recorded for `SEAM-3` and `SEAM-4`

## Slice index

- `S00` -> `slice-00-c-09-pipeline-operator-surface-and-id-resolution-contract.md`
- `S1` -> `slice-1-pipeline-list-show-and-canonical-id-discovery.md`
- `S2` -> `slice-2-pipeline-resolve-and-state-set-command-surface.md`
- `S3` -> `slice-3-help-ambiguity-refusals-and-proof-rails.md`
- `S99` -> `slice-99-seam-exit-gate.md`

## Governance pointers

- Pack remediation log: `../../governance/remediation-log.md`
- Seam closeout: `../../governance/seam-2-closeout.md`
