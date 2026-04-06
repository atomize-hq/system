---
slice_id: S1
seam_id: SEAM-6
slice_kind: implementation
execution_horizon: active
status: decomposed
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to fixture discovery, fixture ordering, or demo invocation parameters requires downstream revalidation.
gates:
  pre_exec:
    review: inherited
    contract: inherited
    revalidation: inherited
  post_exec:
    landing: pending
    closeout: pending
threads:
  - THR-06
contracts_produced: []
contracts_consumed:
  - C-04
  - C-06
open_remediations:
  - REM-002
---

### S1 - Fixture Lineage and Demo Request Surface

- **User/system value**: The demo can be invoked deterministically and is auditable as fixture-backed (not live capability).
- **Scope (in/out)**:
  - In:
    - define the canonical fixture root and fixture lineage layout under the approved repo surface
    - define deterministic fixture selection and ordering rules (no incidental FS traversal ordering)
    - define the demo request surface (CLI flag vs tooling vs test-only) and keep it labeled as fixture-backed
  - Out:
    - live slice discovery or any live execution packet support
- **Acceptance criteria**:
  - A deterministic fixture lineage can be enumerated and explained in inspect output (S2).
  - The demo invocation surface is explicit enough that `SEAM-7` can write conformance tests without guessing.

#### Decided request surface (no new verbs)

- The execution demo is requested via the existing `generate` verb (per `C-02`), by selecting a packet identity (per `C-04`):
  - default packet: `planning.packet`
  - demo packet: `execution.demo.packet`
- Canonical CLI surface to implement:
  - `system generate --packet <packet_id>`
  - For the demo: `system generate --packet execution.demo.packet --fixture-set <fixture_set_id>`

#### Fixture root + fixture set layout

- Canonical fixture root (repo-relative): `tests/fixtures/execution_demo/`
- Fixture set directory: `tests/fixtures/execution_demo/<fixture_set_id>/`
- Required contents of a fixture set:
  - `.system/` subtree containing fixture-backed canonical artifacts (inputs) used as the basis for the demo request

#### Determinism requirements (fixture selection + lineage ordering)

- Fixture set selection MUST be explicit (no "pick one" default and no glob-based selection).
- The demo MUST enumerate fixture lineage evidence in a deterministic order as defined by `C-06`:
  1. `CHARTER`, then `PROJECT_CONTEXT` (if present), then `FEATURE_SPEC`
  2. inherited dependency artifacts (if used), ordered lexically by dependency id
  3. any additional fixture-only evidence items, ordered lexically by repo-relative path

#### Execution checklist (planning-only)

- Wire CLI request selection:
  - parse `--packet` and map to the `ResolveRequest.packet_id` used by the resolver
  - require `--fixture-set` when `--packet execution.demo.packet` is selected
- Implement fixture basis loading:
  - resolve `tests/fixtures/execution_demo/<fixture_set_id>/.system/` as the basis root for the demo request
  - refuse explicitly when the fixture set is missing or malformed
- Add conformance-facing proof hooks:
  - ensure `inspect` can display `fixture_set_id` and the ordered fixture lineage list for the demo request
