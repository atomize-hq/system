---
contract_id: C-02
seam_id: SEAM-2
owner_seam: SEAM-2
version: reduced-v1
currentness: current
status: published
revalidation_triggers:
  - Any change to the supported verb set or help hierarchy in README.md, PLAN.md, docs/README.md, or this contract.
  - Any change to the workspace member list, crate ownership boundaries, or the location of CLI entrypoints.
  - Any change to the reduced-v1 local install target matrix or the supported-vs-legacy runtime boundary.
---

# C-02 Rust Workspace and CLI Command-Surface Contract

## Purpose

This contract defines the reduced-v1 Rust workspace and CLI command-surface truth for `SEAM-2`. It is the source of truth for crate ownership, supported verbs, help posture, and the runtime boundary between supported Rust code and legacy reference material.

## Canonical Location

- Canonical artifact: `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- Repo-facing truth surfaces that should point here: `README.md`, `PLAN.md`, and `docs/README.md`
- Downstream seams that consume this contract: `SEAM-4`, `SEAM-5`, `SEAM-6`, and `SEAM-7`

## Owned Surface

- Workspace root:
  - `Cargo.toml` at the repo root is the authoritative Rust workspace entrypoint.
  - The workspace members are `crates/cli` and `crates/compiler`.
- Crate ownership:
  - `crates/cli` owns binary entrypoints, argument parsing, command dispatch, and user-facing help text.
  - `crates/compiler` owns shared types, packet/decision-log scaffolding, and core compilation or resolution logic.
  - `crates/cli` MUST NOT become the home for resolver logic, packet selection, or shared domain types beyond thin wiring.
  - `crates/compiler` MUST NOT parse CLI arguments or own the supported help surface.

## Normative Rules

### Workspace and crate boundaries

- The workspace MUST expose one obvious split between the operator-facing CLI and the compiler core.
- The CLI crate MUST remain a thin orchestration layer that delegates shared logic into the compiler crate.
- The compiler crate MUST be the compile-time home for shared packet-result and decision-log types used by downstream seams.
- For the first supported `M2` compile wedge, the CLI crate MUST stay thin and the compiler crate MUST own compile assembly, compile proof, and compile refusal logic without spreading that behavior across a new abstraction stack.
- For the first supported `M2` compile wedge, plain `pipeline compile` and `pipeline compile --explain` MUST render from one shared compiler-owned typed compile result rather than maintaining separate assembly paths.
- For the first supported `M2` compile wedge, legacy Python compile behavior MAY be used as content reference, but the supported Rust payload shape, refusal wording, and proof wording MUST follow current Rust contracts rather than byte-for-byte legacy formatting.
- The Rust CLI MUST be the only supported packet-resolution authority once Rust setup exists.

### CLI verbs and help posture

- The reviewed reduced-v1 command surface consists of `setup`, `pipeline`, `generate`, `inspect`, and `doctor`.
- When the `pipeline` family lands, code, help text, docs, contracts, tests, and proof-corpus gates MUST land together before `pipeline` is treated as a supported surface.
- Help text MUST present the surfaces in setup-first order: `setup`, then `pipeline`, then `generate`, then `inspect`, then `doctor`.
- Help text and docs MUST make clear that the public setup family is `system setup`, `system setup init`, and `system setup refresh`.
- `setup` MUST remain the durable product term, and `init` MUST remain only the concrete first-run subcommand name.
- Bare `system setup` MUST route to `setup init` when canonical `.system/` truth is absent or invalid; otherwise it MUST route to `setup refresh`.
- `setup refresh` MUST preserve canonical files by default.
- `setup refresh --rewrite` MUST rewrite only the setup-owned starter files:
  - `.system/charter/CHARTER.md`
  - `.system/feature_spec/FEATURE_SPEC.md`
  - `.system/project_context/PROJECT_CONTEXT.md`
- `setup refresh --reset-state` MUST reset only `.system/state/**`.
- `PROJECT_CONTEXT.md` MUST remain optional semantically for planning packets while still being created as a starter file by setup.
- The shipped setup starter templates MUST be treated as scaffolding only. Required starter files MUST NOT satisfy planning readiness until the operator replaces them with completed canonical truth.
- Scaffolded setup-family flows MUST end with `fill canonical artifact at <required starter path>`.
- Ready setup-family flows MUST end with `system doctor`.
- Help text MUST make clear that `pipeline` is the orchestration surface for route resolution, explicit stage compilation, explicit stage-output capture, and route-state operations, `generate` is the packet surface, `inspect` is the packet proof surface, and `doctor` is the recovery surface.
- Help text and command-surface copy MUST match the actual shipped boundary without underclaiming or overclaiming support.
- `pipeline` MUST own route resolution, explicit stage compilation, and narrow pipeline-run state mutation for the supported wedge.
- The shipped M2 compile wedge MUST expose exactly `system pipeline compile --id <pipeline-id> --stage <stage-id>` and `system pipeline compile --id <pipeline-id> --stage <stage-id> --explain`.
- Successful `pipeline compile` output MUST remain a payload surface, not a proof surface. Route-basis evidence, freshness detail, and decision proof remain the responsibility of refusal output and `pipeline compile --explain`.
- Compile-specific proof MUST be exposed through `pipeline compile --explain`, not by broadening `inspect` beyond its packet-proof meaning.
- The shipped M3 writer wedge MUST expose exactly:
  - `system pipeline capture --id <pipeline-id> --stage <stage-id>`
  - `system pipeline capture --id <pipeline-id> --stage <stage-id> --preview`
  - `system pipeline capture apply --capture-id <capture-id>`
- `pipeline capture` MUST remain the only supported stage-output writer surface in M3 / M3.5.
- The shipped `pipeline.foundation_inputs` capture target set for that writer wedge is:
  - `stage.04_charter_inputs`
  - `stage.05_charter_synthesize`
  - `stage.06_project_context_interview`
  - `stage.07_foundation_pack`
  - `stage.10_feature_spec`
- `pipeline compile` MUST remain payload-only stdout for the shipped M2 / M3.5 wedge.
- The shipped M4 stage-10 materialization path is:
  - `pipeline compile ... --stage stage.10_feature_spec` emits model input payload
  - `pipeline capture ... --stage stage.10_feature_spec` refuses raw `pipeline compile` payload as `invalid_capture_input`
  - an external operator or model runner produces the completed `FEATURE_SPEC.md`
  - `pipeline capture ... --stage stage.10_feature_spec` materializes that completed body
- The supported `foundation_inputs` operator sequence MUST keep `needs_project_context` as one explicit manual handoff:
  - capture `stage.05_charter_synthesize`
  - `system pipeline state set --id pipeline.foundation_inputs --var needs_project_context=<true|false>`
  - `system pipeline resolve --id pipeline.foundation_inputs`
- `generate` MUST be the supported reduced-v1 packet-generation surface for canonical repo-local `.system/` inputs.
- `inspect` MUST be the supported proof surface for packet composition and decision evidence.
- `doctor` MUST be the supported recovery surface for blockers and next safe actions.
- Missing-root, invalid-root, and missing-artifact recovery guidance MUST point to the setup family rather than to raw file-creation instructions.
- Fixture-backed execution demo support MUST remain scoped to the existing `generate` / `inspect` request surface and defer detailed boundary semantics to [`C-06`](C-06-fixture-execution-demo-boundary.md).
- Packet body structure, proof ordering, and renderer-specific output guarantees are owned by [`C-05`](C-05-renderer-and-proof-surfaces.md), not this contract.

### Runtime boundary

- The legacy Python harness remains legacy reference material only.
- The Rust CLI MUST NOT shell out to, wrap, or depend on the legacy harness as a supported runtime path.
- Docs and help text MUST preserve the distinction between supported Rust code and legacy reference material.

### Reduced-v1 install targets

- Reduced-v1 local install support MUST explicitly cover `macOS arm64` and `Linux x86_64`.
- The contract MUST NOT imply broader package-manager publishing or release automation beyond those local install targets.
- Any expansion of the target matrix or supported distribution story requires a contract revision.

### Compatibility and revalidation

- Changes to verb names, help ordering, crate ownership, or runtime boundary wording MUST trigger revalidation of downstream seams that consume `C-02`.
- Changes to the reduced-v1 install target matrix MUST also trigger revalidation because they affect the supported command story.
- This contract MAY be tightened by downstream implementation details, but it MUST NOT be broadened silently.

## Verification Checklist

- [ ] `Cargo.toml` defines a root workspace with `crates/cli` and `crates/compiler` as members.
- [ ] `crates/cli` owns parsing, dispatch, and help text.
- [ ] `crates/compiler` owns shared types and compiler-core logic.
- [ ] `--help` shows the supported surface in setup-first order and presents the setup family as `setup`, `setup init`, and `setup refresh`.
- [ ] Help text matches the supported reduced-v1 command story, documents the routed setup family, documents `pipeline` as the orchestration surface, and exposes the M2 compile wedge plus the M3 capture wedge.
- [ ] `pipeline` owns route resolution, explicit stage compilation, and narrow pipeline-run state mutation once the family lands.
- [ ] `pipeline capture` is documented as the explicit stage-output writer surface for the bounded M3 / M3.5 wedge.
- [ ] The documented `pipeline.foundation_inputs` capture targets are `stage.04_charter_inputs`, `stage.05_charter_synthesize`, `stage.06_project_context_interview`, `stage.07_foundation_pack`, and `stage.10_feature_spec`.
- [ ] `pipeline compile` remains payload-only stdout, raw stage-10 compile payload is documented as refused by `pipeline capture` with `invalid_capture_input`, and stage `10` materialization is documented only as `compile -> external model output -> capture`.
- [ ] `needs_project_context` remains documented as a manual `pipeline state set` plus `pipeline resolve` step rather than an automatic capture side effect.
- [ ] Setup docs and help state that `setup refresh` preserves canonical files by default, that `--rewrite` touches only the three setup-owned starter files, and that `--reset-state` touches only `.system/state/**`.
- [ ] Setup docs and help state that scaffolded setup flows end with `fill canonical artifact at <required starter path>` and ready setup flows end with `system doctor`.
- [ ] `generate` supports ready-path planning packet output from canonical repo-local `.system/` inputs.
- [ ] `inspect` is documented as the packet proof surface.
- [ ] `doctor` is documented as the recovery surface.
- [ ] Fixture-backed execution demo support is described only at the command-surface level here and defers detailed boundary semantics to `C-06`.
- [ ] The Rust CLI does not invoke the legacy Python harness as a supported runtime dependency.
- [ ] The reduced-v1 local install targets are explicitly limited to `macOS arm64` and `Linux x86_64`.
- [ ] `README.md`, `PLAN.md`, and `docs/README.md` each point to this contract.
- [ ] Downstream seams can revalidate against this contract without hidden context.
