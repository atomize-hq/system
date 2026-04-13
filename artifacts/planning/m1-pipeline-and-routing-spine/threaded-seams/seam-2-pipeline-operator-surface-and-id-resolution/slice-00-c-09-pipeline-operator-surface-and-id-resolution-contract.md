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
    - Any rename of supported commands, id-lookup semantics, normalized render contracts, or help posture after `C-09` is drafted requires revalidation before execution continues.
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
  - C-09
contracts_consumed:
  - C-08
  - C-02
open_remediations: []
---

### S00 - Define `C-09` Pipeline Operator Surface and ID Resolution Contract

- **User/system value**: Downstream seams get one explicit operator-surface contract for supported `pipeline` commands, id lookup, normalized renders, and refusal posture instead of inferring support from ad hoc CLI behavior.
- **Scope (in/out)**:
  - In:
    - choose the canonical contract artifact path `docs/contracts/pipeline-operator-surface-and-id-resolution.md`
    - define normative rules for supported `pipeline` subcommands, canonical pipeline/stage ids, shorthand ambiguity posture, normalized default renders, and help exposure
    - define compatibility and revalidation triggers for consumers of `THR-02`
    - define the contract verification checklist and target test/help surfaces
  - Out:
    - compile payload semantics or `pipeline compile` exposure (`SEAM-3`)
    - docs/help cutover outside the shipped command subset (`SEAM-4`)
- **Acceptance criteria**:
  - `C-09` has one canonical descriptive home at `docs/contracts/pipeline-operator-surface-and-id-resolution.md`.
  - The contract names the only supported M1 commands: `pipeline list`, `pipeline show`, `pipeline resolve`, and `pipeline state set`.
  - The contract makes canonical-id and shorthand lookup concrete enough to implement: unique canonical ids, ambiguity refusal posture, unknown-id refusal posture, and recovery guidance.
  - The verification checklist names the concrete CLI/help/test surfaces needed for this seam to later pass `gates.pre_exec.contract`.
- **Dependencies**:
  - Inputs: `../../threading.md`, `../../scope_brief.md`, `../../seam-2-pipeline-operator-surface-and-id-resolution.md`, current CLI help posture, and the published compiler route/state surfaces from `C-08`
  - External contract constraints: `docs/contracts/pipeline-route-and-state-core.md`, `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- **Verification**:
  - Document-level: a downstream planner can answer "which `pipeline` commands ship", "how ids resolve", and "how ambiguity/unknown input fails" without reading CLI code.
  - Planned tests and snapshots live under `crates/cli/tests/` plus any help snapshot surface tied to the shipped command subset.

#### Contract baseline

- The canonical contract lives at `docs/contracts/pipeline-operator-surface-and-id-resolution.md`.
- Supported M1 command surface:
  - `pipeline list`
  - `pipeline show`
  - `pipeline resolve`
  - `pipeline state set`
- Unsupported or deferred M1 surface:
  - `pipeline compile`
  - raw file-path targeting as first-class operator input
- Canonical-id rules:
  - canonical ids come from pipeline YAML and stage front matter
  - shorthand is allowed only when it resolves uniquely
  - ambiguity and absence are distinct refusal classes with distinct recovery guidance
- Render/help rules:
  - default `show` and `resolve` outputs are normalized typed views over compiler-owned data
  - raw YAML remains repo evidence, not the operator contract
  - help exposes only the shipped M1 `pipeline` subset and must remain aligned with `C-02`

#### Owner execution checklist

- `S1` must land canonical pipeline discovery, `list` / `show`, and deterministic canonical-id lookup semantics without raw path targeting.
- `S2` must land `resolve` and `state set` command handlers that wrap published compiler-owned route/state surfaces rather than redefining them.
- `S3` must land help exposure, ambiguity/unknown-id refusal evidence, and proof-facing CLI test coverage that keeps the shipped surface explicit and auditable.
- Publication or acceptance of the canonical contract remains post-exec evidence for seam exit and thread publication; it is not a pre-exec dependency for this producer seam.
