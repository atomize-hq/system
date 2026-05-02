---
slice_id: S1
seam_id: SEAM-4
slice_kind: implementation
execution_horizon: active
status: exec-ready
plan_version: v1
basis:
  currentness: current
  basis_ref: seam.md#basis
  stale_triggers:
    - Any change to manifest inputs (`C-03`) or CLI command surface (`C-02`) before revalidation must update selection/refusal expectations.
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
  - THR-04
contracts_produced: []
contracts_consumed:
  - C-02
  - C-03
open_remediations: []
---

### S1 - Resolver Core: Packet Selection, Determinism, and Budget Outcomes

- **User/system value**: One resolver entrypoint produces a deterministic “packet + decision log + budget outcome” result for both `generate` and `doctor`.
- **Scope (in/out)**:
  - In:
    - consume manifest + freshness truth from `C-03`
    - define selection inputs and ordering rules (stable across platforms)
    - implement packet selection logic that is deterministic and inspectable
    - implement typed budget policy and budget outcomes (inspectable; deterministic)
    - emit decision-log entries that explain why a packet was selected (or why resolution refused/blocked)
  - Out:
    - formatting/wordsmithing of human-facing output (`SEAM-5`)
    - fixture-execution semantics (`SEAM-6`)
- **Acceptance criteria**:
  - Resolver core produces the same packet and the same ordered decision-log for identical inputs.
  - Budget outcomes are typed and include a single explicit recovery action.
  - No implicit filesystem or system-time reads influence selection results beyond the canonical `C-03` inputs.
- **Dependencies**:
  - `C-02` defines the CLI verbs that call into this resolver.
  - `C-03` defines canonical inputs, freshness fields, and refusal sources.
- **Verification**:
  - Unit-level: determinism checks that compare results across repeated runs.
  - Contract-level: resolver result aligns with `C-04` fields and ordering guarantees.
- **Rollout/safety**:
  - When refusing or blocking, return typed refusal/blocker categories instead of encoding semantics in strings.

#### S1.T1 - Define resolver entrypoint + typed result boundary

- **Outcome**: A single resolver entrypoint produces a typed `C-04` result consumed by both `generate` and `doctor`.
- **Thread/contract refs**: `THR-04`, `C-04`

#### S1.T2 - Implement deterministic packet selection + decision log

- **Outcome**: Packet selection reads only canonical inputs (from `C-03`) and emits an ordered decision log explaining inclusion/exclusion and final selection.
- **Thread/contract refs**: `THR-03`, `C-03`, `THR-04`, `C-04`

#### S1.T3 - Implement typed budget policy + outcomes

- **Outcome**: Budget policy yields typed outcomes (including recovery actions) and is reflected in the decision log.
- **Thread/contract refs**: `THR-04`, `C-04`
