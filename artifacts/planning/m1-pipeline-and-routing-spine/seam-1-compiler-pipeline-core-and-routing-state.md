---
seam_id: SEAM-1
seam_slug: compiler-pipeline-core-and-routing-state
type: capability
status: landed
execution_horizon: future
plan_version: v1
basis:
  currentness: current
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts: []
  required_threads: []
  stale_triggers:
    - If the approved parser base changes from `serde_yaml_bw` or the two-document pipeline shape changes, this seam must revalidate before execution.
    - If the canonical-vs-runtime `.system/` boundary changes in upstream docs/contracts, this seam must revalidate route-state storage assumptions.
gates:
  pre_exec:
    review: passed
    contract: passed
    revalidation: passed
  post_exec:
    landing: passed
    closeout: passed
seam_exit_gate:
  required: true
  planned_location: S99
  status: passed
open_remediations: []
---

# SEAM-1 - Compiler Pipeline Core and Routing State

- **Goal / value**: Make the compiler the single source of truth for pipeline loading, deterministic route computation, activation evaluation, and narrow route-state mutation semantics.
- **Scope**
  - In:
    - pipeline model types and YAML loading over the approved two-document shape
    - repo-safe pipeline path resolution inside the approved repo surface
    - deterministic stage ordering and explicit route statuses (`active`, `skipped`, `blocked`, `next`)
    - supported activation subset validation and refusal semantics
    - `.system/state/pipeline/<pipeline-id>.yaml` schema, bounded audit history, lock/revision/atomic-write protocol, and malformed-state refusal
  - Out:
    - public help/docs cutover for `pipeline` as a supported surface
    - compile payload generation or artifact materialization
    - generalized state-machine behavior or open-ended state schema growth
- **Primary interfaces**
  - Inputs:
    - `pipelines/foundation.yaml`
    - `pipelines/foundation_inputs.yaml`
    - `core/stages/*.md`
    - `.system/state/pipeline/<pipeline-id>.yaml`
    - current repo-boundary and `.system/` rules from existing contracts
  - Outputs:
    - published route/state core contract `C-08`
    - typed compiler APIs for declared pipeline config, resolved route result, and state mutation outcomes
- **Key invariants / rules**:
  - compiler route truth lives in a dedicated compiler-owned module and typed route result, separate from the packet resolver
  - only the narrow typed activation subset needed for the foundation wedge is supported in `M1`
  - unsupported activation shapes fail at load/validation time, not as late resolve surprises
  - `.system/state/**` is runtime-only and must not become canonical project truth
  - state writes must use advisory locking, revision checks, and atomic write-then-rename; silent last-write-wins is invalid
- **Dependencies**
  - Direct blockers:
    - `M0.5` parser gate completion
  - Transitive blockers:
    - existing CLI/runtime boundary decisions in `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
    - existing canonical-vs-runtime `.system/` rules in `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
  - Direct consumers:
    - `SEAM-2`
    - `SEAM-3`
    - `SEAM-4`
  - Derived consumers:
    - future `M2` compile implementation
    - future `M4` end-to-end foundation flow
- **Touch surface**:
  - `crates/compiler/`
  - `pipelines/`
  - `core/stages/`
  - `.system/state/pipeline/`
  - compiler-core tests and proof fixtures for route/state semantics
- **Verification**:
  - The canonical route/state contract, compiler-owned route evaluation surface, and runtime-only state store are landed and recorded in `governance/seam-1-closeout.md`.
  - Downstream seams now consume published `C-08` and the realized `THR-01` handoff rather than provisional seam-brief intent.
- **Canonical contract refs**:
  - `docs/contracts/pipeline-route-and-state-core.md`
  - `docs/contracts/C-03-canonical-artifact-manifest-contract.md`
- **Risks / unknowns**:
  - Risk: route-state concerns bloat the seam into a generalized orchestration state machine.
  - De-risk plan: keep the state schema closed and tied only to route-relevant keys named in the `M1` milestone.
  - Risk: stage metadata requirements for `resolve` quietly expand command cost or blur the `list` / `show` / `resolve` boundary.
  - De-risk plan: make the per-command loading boundary an explicit review target in seam-local planning.
- **Rollout / safety**:
  - prefer explicit refusal over implicit fallback for malformed pipelines, unsupported activation shapes, malformed persisted state, or revision conflicts
  - keep state audit history bounded and inspection-oriented rather than turning it into a provenance log
- **Downstream decomposition context**:
  - This seam has left the forward planning window after publishing `C-08` and `THR-01`.
  - `THR-01` is the dominant thread; `SEAM-2`, `SEAM-3`, and `SEAM-4` all consume it directly.
  - The first seam-local review should focus on contract ownership boundaries inside `crates/compiler`, route determinism, and whether state-mutation semantics are small enough to stay inside one compiler-owned seam.
  - Authoritative seam-local planning now lives under `threaded-seams/seam-1-compiler-pipeline-core-and-routing-state/`.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-08`
  - Threads likely to advance:
    - `THR-01`
  - Review-surface areas likely to shift after landing:
    - `R2`
    - `R3`
  - Downstream seams most likely to require revalidation:
    - `SEAM-2`
    - `SEAM-3`
    - `SEAM-4`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.
