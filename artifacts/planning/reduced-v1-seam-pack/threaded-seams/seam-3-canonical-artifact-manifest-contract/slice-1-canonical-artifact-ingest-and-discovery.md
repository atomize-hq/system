---
slice_id: S1
seam_id: SEAM-3
slice_kind: implementation
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to canonical `.system/` paths or optional/required artifact rules requires revalidation against `C-03`.
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
  - THR-03
contracts_produced: []
contracts_consumed:
  - C-02
  - C-03
open_remediations: []
---

### S1 - Implement Canonical Artifact Discovery and Typed Ingest

- **User/system value**: The compiler core can deterministically discover and ingest the reduced v1 canonical `.system/` artifacts into one typed representation without reading non-canonical derived sources.
- **Scope (in/out)**:
  - In:
    - implement canonical-path discovery rooted at repo-local `.system/`
    - implement typed ingest for:
      - `CHARTER` (required)
      - `PROJECT_CONTEXT` (optional)
      - `FEATURE_SPEC` (required)
    - produce a stable “artifact identity” representation (path + required/optional + content identity inputs)
  - Out:
    - resolver packet selection, refusals, or budget logic (`SEAM-4`)
    - any renderer-specific formatting or inspect output (`SEAM-5`)
- **Acceptance criteria**:
  - All reads are constrained to `.system/` canonical paths defined by `C-03`.
  - Optional artifact absence is represented explicitly and deterministically (distinct from “empty content”).
  - The ingest API lives in `crates/compiler` per `C-02` crate ownership boundaries.
- **Dependencies**:
  - Requires `S00` contract-definition (`C-03`) to be concrete enough to implement against.
  - Revalidation is satisfied by `SEAM-2` closeout publishing `C-02` (`THR-02`).
- **Verification**:
  - Unit tests covering:
    - required artifact missing vs present
    - optional artifact missing vs present vs empty
    - path refusal for non-canonical inputs (must not be silently read)
- **Rollout/safety**:
  - Prefer request-scoped ingest (no caching) until a later seam owns persistence semantics explicitly.
- **Review surface refs**: `../../review_surfaces.md` (R1, R2, R3)

#### S1.T1 - Define the ingest API surface and error taxonomy (manifest-local)

- **Outcome**: A small Rust API (e.g., `CanonicalArtifacts::load(repo_root) -> Result<CanonicalArtifacts, ArtifactIngestError>`) that does not yet decide “packet selection,” but provides deterministic ingest results.
- **Thread/contract refs**: `THR-03`, `C-03`, `C-02`

#### S1.T2 - Implement canonical-path resolution and filesystem reads

- **Outcome**: Repo-root detection + `.system/...` path resolution + file reads exist, with explicit refusal for non-canonical reads.
- **Thread/contract refs**: `THR-03`, `C-03`

#### S1.T3 - Add unit tests for presence/absence and optional semantics

- **Outcome**: Tests prove the ingest representation matches `C-03` optional/required semantics and never silently collapses “missing” into “empty”.
- **Thread/contract refs**: `THR-03`, `C-03`
