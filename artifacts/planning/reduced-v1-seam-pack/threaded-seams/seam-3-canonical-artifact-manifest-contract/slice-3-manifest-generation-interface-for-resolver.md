---
slice_id: S3
seam_id: SEAM-3
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to the manifest output shape consumed by `SEAM-4` requires a `C-03` update and downstream revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-03
contracts_produced: []
contracts_consumed:
  - C-03
open_remediations: []
---

### S3 - Land the Manifest Generation Interface for the Resolver

- **User/system value**: `SEAM-4` can consume one stable manifest/freshness interface without embedding `.system/` filesystem assumptions or re-implementing `C-03` logic.
- **Scope (in/out)**:
  - In:
    - define and implement the public Rust types/API surface in `crates/compiler` that `SEAM-4` consumes
    - guarantee stable ordering and stable identity semantics for manifest entries
    - ensure the interface carries enough refusal-relevant context for downstream diagnosis without defining refusal copy
  - Out:
    - resolver result contract (`C-04`) and refusal taxonomy (`SEAM-4`)
    - rendering formats and proof ordering (`SEAM-5`)
- **Acceptance criteria**:
  - `SEAM-4` can treat the manifest as authoritative input for canonical inputs + freshness + override rationale without reading files directly.
  - The interface is versioned per `C-03` (schema version + manifest generation version distinction is respected).
- **Dependencies**:
  - Requires `S1` ingest and `S2` freshness/override semantics to exist.
  - Must remain aligned with `C-03` normative rules and versioning policy.
- **Verification**:
  - Compile-time: downstream crate/module boundaries prove ownership (`C-02`): the resolver consumes the manifest API, it does not implement it.
  - Unit tests: stable ordering + stable serialization (if any) for the manifest entry list.
- **Rollout/safety**:
  - Treat interface changes as contract changes: update `C-03` and force downstream revalidation.
- **Review surface refs**: `../../review_surfaces.md` (R2, R3)

#### S3.T1 - Define the “manifest to resolver” Rust API boundary

- **Outcome**: One public type entrypoint (e.g., `ArtifactManifest`) plus supporting structs/enums that capture:
  - canonical input inventory
  - freshness truth and version stamps
  - override-with-rationale records (where applicable)
- **Thread/contract refs**: `THR-03`, `C-03`

#### S3.T2 - Ensure stable ordering and identity semantics

- **Outcome**: Manifest entry ordering is deterministic and independent of filesystem iteration order; identity fields are explicit and testable.
- **Thread/contract refs**: `THR-03`, `C-03`
