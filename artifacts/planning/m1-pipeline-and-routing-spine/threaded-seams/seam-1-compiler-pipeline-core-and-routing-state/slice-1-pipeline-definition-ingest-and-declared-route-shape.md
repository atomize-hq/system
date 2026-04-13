---
slice_id: S1
seam_id: SEAM-1
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to the approved two-document pipeline YAML shape or repo-safe path rules requires this slice to revalidate.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-01
contracts_produced: []
contracts_consumed:
  - C-08
open_remediations: []
---

### S1 - Pipeline Definition Ingest and Declared Route Shape

- **User/system value**: The compiler exposes one typed declared-pipeline model with deterministic stage order and repo-safe file boundaries, so later route evaluation does not need to rediscover YAML or filesystem semantics.
- **Scope (in/out)**:
  - In:
    - pipeline file loading over the approved two-document YAML shape
    - repo-relative pipeline path validation and stage file validation under `core/stages/`
    - declared stage ordering, stage ids, `sets`, and supported activation clauses as typed compiler inputs
    - clear module/API boundary between declared pipeline ingest and later route evaluation/state mutation work
  - Out:
    - resolved-route status computation
    - persisted route-state read/write behavior
    - CLI-facing render formatting or shorthand id lookup
- **Acceptance criteria**:
  - The compiler exposes a typed declared-pipeline surface rooted in `crates/compiler`, not in CLI code.
  - The accepted YAML shape and file-boundary rules match the `C-08` contract and refuse out-of-scope files deterministically.
  - The declared stage list preserves source order for `foundation`, `foundation_inputs`, `release`, and `sprint` pipelines.
  - The ingest surface is narrow enough that downstream route evaluation consumes typed data rather than reparsing YAML.
- **Dependencies**:
  - Inputs: `crates/compiler/src/pipeline.rs`, `pipelines/*.yaml`, `core/stages/*.md`
  - External constraints: approved repo surface from `docs/contracts/C-01-approved-repo-surface.md`, runtime-zone posture from `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
- **Verification**:
  - Extend `crates/compiler/tests/pipeline_loader.rs` to cover deterministic declared order, repo-safe path refusal, duplicate stage refusal, and supported activation-clause parsing.
  - Add any needed unit coverage around declared-pipeline types in `crates/compiler/src/pipeline.rs` or a split module if the file is refactored.
- **Rollout/safety**:
  - Keep pipeline ingest compiler-owned so `SEAM-2` cannot silently redefine route inputs in CLI wiring.
  - Refuse path or schema drift early rather than letting it leak into route evaluation.
- **Review surface refs**: `../../review_surfaces.md` (`R2`, `R3`)
