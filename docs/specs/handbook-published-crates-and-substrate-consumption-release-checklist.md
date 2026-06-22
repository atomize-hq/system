# First-Wave Release Contract + Checklist: handbook-engine, handbook-pipeline, handbook-flow

Packet 3.1 authority for the first crates.io release wave in `/Users/spensermcconnell/__Active_Code/system`.

This document records the manual release contract that Packet 3.2 must execute. It is **planning truth only**, not publish proof.

## Verified Starting Point (live repo truth, 2026-06-22)

- `crates/engine/Cargo.toml`, `crates/pipeline/Cargo.toml`, and `crates/flow/Cargo.toml` all currently declare version `0.1.0`.
- `handbook-pipeline` and `handbook-flow` currently depend on `handbook-engine` as `handbook-engine = { version = "0.1.0", path = "../engine" }`.
- `cargo package -p handbook-engine --allow-dirty` currently passes.
- `cargo package -p handbook-pipeline --allow-dirty` and `cargo package -p handbook-flow --allow-dirty` currently fail only because Cargo cannot yet resolve `handbook-engine` from the crates.io index:

  ```text
  no matching package named `handbook-engine` found
  location searched: crates.io index
  ```

- Lane 1 manifest hardening and Lane 2 boundary hardening are already landed in the spec/plan/tasks trio; Lane 3.2 real release execution and Lane 4 downstream adoption are still pending.

## Release Contract

### 1. First-Wave Versioning Policy

- The first real release train targets a coordinated `0.1.0` publish for:
  - `handbook-engine`
  - `handbook-pipeline`
  - `handbook-flow`
- Packet 3.2 may keep the coordinated `0.1.0` train only when the release-candidate crate sources/manifests stay unchanged and the blocker is transient/operator-only rather than a publishable-crate fix.
- Keeping `0.1.0` is still allowed for cases like crates.io index lag before engine publication, auth/token issues, operator interruption, or docs-only/evidence-note corrections that do not change any first-wave crate manifest, source, or publish-relevant boundary.
- If a real release-blocking fix is discovered **before any real publish happens** and that fix changes any first-wave crate manifest, source, or publish-relevant boundary, abandon `0.1.0`, bump all three crates together to the next coordinated version, and rerun the full Packet 3.2 checklist from the start.
- For this first-wave seam, **next coordinated version** means one shared semver literal across all three crates and both `handbook-engine` dependency literals in `handbook-pipeline` / `handbook-flow`; the default successor to an abandoned `0.1.0` train is `0.1.1` unless the approved fix intentionally requires a higher semver bump.
- Packet 3.2 must record the exact published versions in the release evidence.
- If a release-blocking fix is discovered **after `handbook-engine` has already been published**, stop and record the partial state honestly instead of improvising a mixed or split first-wave train inside Packet 3.2.

### 2. Approved Publish Order

The first-wave publish order is fixed:

1. `handbook-engine`
2. `handbook-pipeline`
3. `handbook-flow`

Rules:
- `handbook-engine` must publish first because both dependent crates require a resolvable published `handbook-engine` version for honest dependent dry-runs.
- `handbook-pipeline` publishes before `handbook-flow` for the approved first-wave train; Packet 3.2 should not reorder these crates ad hoc.

### 3. Dependency Pin Semantics

#### 3.1 System Release-Candidate Manifests

- `handbook-pipeline` and `handbook-flow` keep the Packet 1.2 publishable dependency shape:

  ```toml
  handbook-engine = { version = "0.1.0", path = "../engine" }
  ```

- If the coordinated train version changes before the first real publish, update the literal version in both manifests to that same coordinated version and keep the same `version + path` shape.
- This is the approved first-wave manifest contract:
  - local development keeps the sibling path
  - packaging and publish dry-runs resolve against the registry version
- Do **not** switch this seam back to path-only, git, wildcard, or undocumented broader dependency forms during Packet 3.2.

#### 3.2 Downstream Substrate Adoption Input

- Packet 4.1 should consume the first published wave with exact crate pins:
  - `handbook-engine = "=0.1.0"`
  - `handbook-pipeline = "=0.1.0"`
  - `handbook-flow = "=0.1.0"`
- If Packet 3.2 bumps the coordinated release train before the first real publish, Packet 4.1 should use the exact published replacement versions instead.
- Substrate should refresh its lockfile after wiring those exact pins and should not fall back to sibling path dependencies for the first published-consumption proof.

### 4. Packet 3.2 Recording Authorities

- `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md` is the **execution-evidence authority** for Packet 3.2. Record all release-candidate and publish evidence here.
- `docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md` is the **packet/lane status mirror**. Update it only after the checklist contains the execution evidence or stop-state record.
- `docs/specs/handbook-published-crates-and-substrate-consumption-plan.md` is the **cross-lane handoff/status mirror**. Update it only after the checklist contains the execution evidence or stop-state record.

Packet 3.2 must use those docs as follows:

| Evidence item | Required authority doc |
|---|---|
| release-candidate commit hash | `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md` |
| engine + dependent dry-run outputs | `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md` |
| real publish outputs / terminal proof | `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md` |
| final published versions and Packet 4.1 pin handoff | `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md`, then mirror status in `...-plan.md` and `...-tasks.md` |
| partial-wave stop state if the release halts midstream | `docs/specs/handbook-published-crates-and-substrate-consumption-release-checklist.md`, then mirror status in `...-plan.md` and `...-tasks.md` |

## Required Evidence Before Any Real Publish

Packet 3.2 must treat each item below as required evidence, not implied truth.

### A. Release-Candidate Preflight

- [ ] Re-read:
  - `docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`
  - `docs/specs/handbook-published-crates-and-substrate-consumption-plan.md`
  - `docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md`
  - this release checklist
- [ ] Confirm the release-candidate commit is explicit and recorded **in this checklist** before any real publish.
- [ ] Reconfirm the manifest versions for all three crates are still the same coordinated train version.
- [ ] Reconfirm `handbook-pipeline` and `handbook-flow` still use the approved `version + path` dependency form against `handbook-engine`.
- [ ] Reconfirm the current packageability truth on the exact release-candidate commit:

  ```bash
  cargo package -p handbook-engine --allow-dirty
  cargo package -p handbook-pipeline --allow-dirty
  cargo package -p handbook-flow --allow-dirty
  ```

  Expected interpretation:
  - `handbook-engine` passes.
  - `handbook-pipeline` and `handbook-flow` may still fail, but only because the chosen `handbook-engine` version is not yet resolvable from crates.io.
  - A missing dependency version or other new manifest-only failure is release-blocking.

- [ ] Re-run the release-candidate verification wall on the same commit:

  ```bash
  cargo check --workspace
  cargo fmt --all -- --check
  cargo clippy --workspace --all-targets -- -D warnings
  cargo test -p handbook-engine
  cargo test -p handbook-pipeline --test pipeline_catalog
  cargo test -p handbook-pipeline --test pipeline_compile
  cargo test -p handbook-pipeline --test pipeline_capture
  cargo test -p handbook-pipeline --test pipeline_handoff
  cargo test -p handbook-flow
  cargo test -p handbook-compiler --test author
  ```

- [ ] Confirm the published-boundary authorities still match live source:
  - `crates/engine/src/lib.rs`
  - `crates/pipeline/src/lib.rs`
  - `crates/flow/src/lib.rs`
  - `docs/specs/handbook-flow-import-boundary-consumer-contract.md`
  - `docs/specs/archive/phase-6-pipeline-boundary-cleanup/`

### B. Engine Dry-Run Gate

- [ ] Run:

  ```bash
  cargo publish --dry-run -p handbook-engine
  ```

- [ ] Record the dry-run command output **in this checklist** as evidence.
- [ ] Do not perform a real publish unless the engine dry-run succeeds and explicit human authorization for real crates.io publication is still present in the Packet 3.2 session.

### C. Engine Publish + Resolution Gate

- [ ] Publish `handbook-engine` first:

  ```bash
  cargo publish -p handbook-engine
  ```

- [ ] Record the published version and the terminal proof for that publish **in this checklist**.
- [ ] Wait for the published engine version to become resolvable from crates.io before treating dependent dry-runs as meaningful.
- [ ] The known pre-release failure mode:

  ```text
  no matching package named `handbook-engine` found
  ```

  must clear before continuing. If that exact failure still appears, keep waiting or retry the dependent dry-runs later; do not treat it as a pipeline/flow release defect yet.

### D. Dependent Dry-Run Gate

- [ ] After the published `handbook-engine` version is resolvable, run:

  ```bash
  cargo publish --dry-run -p handbook-pipeline
  cargo publish --dry-run -p handbook-flow
  ```

- [ ] Record both dry-run outputs **in this checklist**.
- [ ] Both dependent dry-runs must succeed before any dependent real publish begins.
- [ ] If either dry-run fails for a reason other than the transient unresolved-index condition above, stop Packet 3.2 and fix or re-plan before any dependent real publish.

### E. Dependent Publish Order

- [ ] Publish `handbook-pipeline`:

  ```bash
  cargo publish -p handbook-pipeline
  ```

- [ ] Publish `handbook-flow`:

  ```bash
  cargo publish -p handbook-flow
  ```

- [ ] Record exact published versions and terminal evidence for both commands **in this checklist**.
- [ ] If `handbook-pipeline` publishes but `handbook-flow` cannot be honestly published afterward, record the partial state honestly **in this checklist** and stop; do not claim the first wave is complete.

### F. Post-Publish Recording

- [ ] Record the final published train versions in this checklist.
- [ ] Record the exact dry-run / publish command evidence used for Packet 3.2 in this checklist.
- [ ] Record any crates.io index lag or operator notes that mattered during the staged release in this checklist.
- [ ] Hand forward the exact published `=` versions from this checklist to Packet 4.1 as the only approved downstream pin targets.
- [ ] After this checklist is complete, mirror the final Lane 3 / Lane 4 status in `docs/specs/handbook-published-crates-and-substrate-consumption-plan.md` and `docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md`.

### G. Partial-Wave Stop Record

- [ ] If the wave stops after any real publish, add a stop-state note to this checklist before leaving the session.
- [ ] Record: last successful crate publish, published version(s), failing next step, relevant terminal output, whether the issue was transient or a real release blocker, and the exact resume point.
- [ ] Mirror that partial-publish status into `docs/specs/handbook-published-crates-and-substrate-consumption-plan.md` and `docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md` only after the checklist stop-state note exists.

## Honest Status Language

Use these labels precisely:

- **Packet 3.1 landed** = the release contract and checklist are documented.
- **publish-ready** = the release-candidate commit satisfies the preflight evidence and dry-run gates, but no real crates.io publication has happened yet.
- **partially published** = one or more crates were published, but the full `engine → pipeline → flow` wave is not complete.
- **published** = all three crates were published in the approved order and the exact versions were recorded.
- **Substrate-consume-ready** = Packet 4 has switched Substrate to the published crates and passed its downstream verification wall.

Do not collapse these states into one another.
