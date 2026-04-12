---
seam_id: SEAM-2
seam_slug: pipeline-operator-surface-and-id-resolution
type: platform
status: landed
execution_horizon: future
plan_version: v1
basis:
  currentness: current
  source_scope_ref: scope_brief.md
  source_scope_version: v1
  upstream_closeouts:
    - SEAM-1
  required_threads:
    - THR-01
  stale_triggers:
    - If `SEAM-1` changes route status names, state mutation outcome semantics, or repo-safe pipeline path rules, this seam must revalidate before execution.
    - If existing CLI command-hierarchy docs or `C-02` change the supported top-level command posture, this seam must revalidate naming and help-surface assumptions.
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

# SEAM-2 - Pipeline Operator Surface and ID Resolution

- **Goal / value**: Expose one supported `pipeline` operator surface that lets the operator discover, inspect, resolve, and mutate pipeline state without hiding route truth inside legacy or packet-only paths.
- **Scope**
  - In:
    - `pipeline list`, `pipeline show`, `pipeline resolve`, and `pipeline state set`
    - canonical pipeline and stage ids plus unambiguous shorthand lookup rules
    - ambiguity and unknown-id refusal classes with explicit operator recovery guidance
    - normalized default render contracts for declared config and resolved route output
    - help-surface posture that exposes only the shipped M1 `pipeline` subset
  - Out:
    - exposing `pipeline compile` as a shipped M1 help/docs surface
    - raw file-path targeting as a first-class operator input
    - compile payload semantics, output materialization, or downstream packet generation behavior
- **Primary interfaces**
  - Inputs:
    - `C-08`
    - existing CLI skeleton and help posture from `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
    - canonical ids embedded in pipeline YAML and stage front matter
  - Outputs:
    - published operator-surface contract `C-09`
    - CLI-visible list/show/resolve/state-set behavior and help evidence
- **Key invariants / rules**:
  - `pipeline` owns orchestration; `generate packet` remains the downstream packet surface and `doctor` remains recovery
  - default `show` and `resolve` renders are typed, normalized views; raw YAML is repo evidence, not the operator contract
  - shorthand lookup is allowed only when unambiguous
  - ambiguous shorthand and unknown canonical ids remain distinct refusal classes with distinct recovery posture
  - `pipeline` is not a shadow or partially documented family; it becomes supported only when code, help, docs, tests, and proof outputs agree
- **Dependencies**
  - Direct blockers:
    - `SEAM-1`
  - Transitive blockers:
    - existing command-surface baseline in `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - Direct consumers:
    - `SEAM-3`
    - `SEAM-4`
  - Derived consumers:
    - future `M2` compile surface
    - downstream planning consumers that rely on stable route reports
- **Touch surface**:
  - `crates/cli/src/main.rs`
  - CLI integration and help snapshot tests
  - `README.md`
  - `docs/START_HERE.md`
  - `docs/CLI_PRODUCT_VOCABULARY.md`
  - `docs/CLI_COMMAND_HIERARCHY.md`
  - `docs/SUPPORTED_COMMANDS.md`
- **Verification**:
  - `SEAM-1` closeout now publishes `C-08` and `THR-01`, so this seam may consume current route/state truth directly.
  - Seam-local planning under `threaded-seams/seam-2-pipeline-operator-surface-and-id-resolution/` now makes the operator-surface contract, review posture, and execution slices concrete enough for implementation.
- **Canonical contract refs**:
  - `docs/contracts/pipeline-operator-surface-and-id-resolution.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- **Risks / unknowns**:
  - Risk: command-surface copy and docs drift into a competing packet-first story before the surface is fully shipped.
  - De-risk plan: bind this seam directly to the docs/help cutover work reserved in `SEAM-4` and treat unsupported copy as a conformance failure.
  - Risk: shorthand ergonomics create unstable lookup semantics once more pipelines or stages are added.
  - De-risk plan: make ambiguity handling and conflicting canonical-id reporting a contract-level review focus.
- **Rollout / safety**:
  - keep `pipeline compile` hidden from shipped `M1` help/docs until `SEAM-3` and later M2 work publish the compile contract and implementation
  - refuse ambiguous or malformed operator input instead of guessing
- **Downstream decomposition context**:
  - This seam is now landed and outside the forward planning window after publishing the operator-surface handoff.
  - `THR-01` was the incoming dependency and `THR-02` is now the published outgoing operator-surface thread.
  - Downstream planning should consume the closeout-backed contract, help, and refusal evidence rather than reopening the seam-local review bundle.
- **Expected seam-exit concerns**:
  - Contracts likely to publish:
    - `C-09`
  - Threads likely to advance:
    - `THR-02`
  - Review-surface areas likely to shift after landing:
    - `R1`
    - `R3`
  - Downstream seams most likely to require revalidation:
    - `SEAM-3`
    - `SEAM-4`
  - Accepted or published owned-contract artifacts belong here and in closeout evidence, not in pre-exec verification for the producing seam.
