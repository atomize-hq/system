# First-Wave Release Contract + Checklist: handbook-engine, handbook-pipeline, handbook-flow

Packet 3.1 authority for the first crates.io release wave in `/Users/spensermcconnell/__Active_Code/system`.

This document records the manual release contract that Packet 3.2 must execute. The Packet 3.1 sections below remain planning truth, and the Packet 3.2 execution log added below records live release evidence / stop-state notes.

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

## Packet 3.2 Execution Log (2026-06-22)

### Outcome

- Packet 3.2 is **blocked before any real crates.io publish**.
- Release-candidate crate-source commit: `5c5bf437168b47c9ab749aee5307a190841502d8` (`5c5bf43` short).
- Starting workspace status:

  ```text
  ## feat/seam-extraction...origin/feat/seam-extraction [ahead 3]
   M AGENTS.md
   M CLAUDE.md
  ```

- Explicit human authorization for real crates.io publication remains present in the parent Packet 3.2 session message, but that authorization was **not exercised** because the required release-candidate verification wall failed before any real `cargo publish`.

### Preflight Evidence Captured

- Re-read authorities:
  - `docs/specs/handbook-published-crates-and-substrate-consumption-spec.md`
  - `docs/specs/handbook-published-crates-and-substrate-consumption-plan.md`
  - `docs/specs/handbook-published-crates-and-substrate-consumption-tasks.md`
  - this release checklist
- Manifest versions reconfirmed live: `handbook-engine`, `handbook-pipeline`, and `handbook-flow` all still declare `0.1.0`.
- Dependency form reconfirmed live: `handbook-pipeline` and `handbook-flow` still use `handbook-engine = { version = "0.1.0", path = "../engine" }`.
- Published-boundary source inspection reconfirmed the current live exports:
  - `crates/engine/src/lib.rs` still exposes the accepted first-wave engine API, including `default_canonical_layout_contract`, author/baseline/canonical/freshness surfaces, and `engine_contract_version()`.
  - `crates/pipeline/src/lib.rs` still physically exposes only `pipeline`, `pipeline_capture`, `pipeline_compile`, `pipeline_handoff`, `pipeline_route`, `route_state`, and `pipeline_contract_version()` at the crate root.
  - `crates/flow/src/lib.rs` still exposes only `budget`, `packet_result`, `resolver`, the typed re-exports, and `flow_contract_version()`.

#### Packageability Truth

`cargo package -p handbook-engine --allow-dirty`

```text
   Packaging handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
    Updating crates.io index
    Packaged 20 files, 229.5KiB (41.1KiB compressed)
   Verifying handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
   Compiling proc-macro2 v1.0.106
   Compiling unicode-ident v1.0.24
   Compiling quote v1.0.45
   Compiling typenum v1.19.0
   Compiling version_check v0.9.5
   Compiling autocfg v1.5.0
   Compiling serde_core v1.0.228
   Compiling libc v0.2.184
   Compiling thiserror v2.0.18
   Compiling memchr v2.8.0
   Compiling regex-syntax v0.8.10
   Compiling zmij v1.0.21
   Compiling serde v1.0.228
   Compiling generic-array v0.14.7
   Compiling aho-corasick v1.1.4
   Compiling num-traits v0.2.19
   Compiling arraydeque v0.5.1
   Compiling smallvec v1.15.1
   Compiling hashbrown v0.16.1
   Compiling syn v2.0.117
   Compiling equivalent v1.0.2
   Compiling itoa v1.0.18
   Compiling regex-automata v0.4.14
   Compiling indexmap v2.13.1
   Compiling base64 v0.22.1
   Compiling cfg-if v1.0.4
   Compiling unsafe-libyaml-norway v0.2.15
   Compiling cpufeatures v0.2.17
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling digest v0.10.7
   Compiling sha2 v0.10.9
   Compiling thiserror-impl v2.0.18
   Compiling serde_derive v1.0.228
   Compiling regex v1.12.3
   Compiling saphyr-parser-bw v0.0.611
   Compiling serde_yaml_bw v2.5.4
   Compiling handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/target/package/handbook-engine-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.66s
```

`cargo package -p handbook-pipeline --allow-dirty`

```text
   Packaging handbook-pipeline v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/pipeline)
    Updating crates.io index
error: failed to prepare local package for uploading

Caused by:
  no matching package named `handbook-engine` found
  location searched: crates.io index
  required by package `handbook-pipeline v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/pipeline)`
```

`cargo package -p handbook-flow --allow-dirty`

```text
   Packaging handbook-flow v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/flow)
    Updating crates.io index
error: failed to prepare local package for uploading

Caused by:
  no matching package named `handbook-engine` found
  location searched: crates.io index
  required by package `handbook-flow v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/flow)`
```

#### Release-Candidate Verification Wall

- `cargo check --workspace` → pass
- `cargo fmt --all -- --check --color never` → **fail**
- `cargo clippy --workspace --all-targets -- -D warnings` → pass
- `cargo test -p handbook-engine` → pass
- `cargo test -p handbook-pipeline --test pipeline_catalog` → pass
- `cargo test -p handbook-pipeline --test pipeline_compile` → pass
- `cargo test -p handbook-pipeline --test pipeline_capture` → pass
- `cargo test -p handbook-pipeline --test pipeline_handoff` → pass
- `cargo test -p handbook-flow` → pass
- `cargo test -p handbook-compiler --test author` → pass

Exact failing `cargo fmt` output:

```text
Diff in /Users/spensermcconnell/__Active_Code/system/crates/compiler/src/route_state.rs:167:
             let child_file_name = child.file_name();
             let child_name = child_file_name.to_string_lossy();
             let child_display_path = format!("{display_path}/{child_name}");
-            collect_runtime_state_reset_entries(
-                &child.path(),
-                &child_display_path,
-                reset_entries,
-            )?;
+            collect_runtime_state_reset_entries(&child.path(), &child_display_path, reset_entries)?;
         }
 
         reset_entries.push(RuntimeStateResetEntry {
Diff in /Users/spensermcconnell/__Active_Code/system/crates/pipeline/src/layout.rs:151:
     );
 
 pub(crate) fn handbook_product_pipeline_storage_layout_contract(
-) -> &'static PipelineStorageLayoutContract
-{
+) -> &'static PipelineStorageLayoutContract {
     &HANDBOOK_PRODUCT_PIPELINE_STORAGE_LAYOUT
 }
 
Diff in /Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_capture.rs:8:
     pipeline_capture::{
         apply_pipeline_capture, capture_pipeline_output, load_pipeline_capture_cache_entry,
         preview_pipeline_capture, render_pipeline_capture_apply_result,
-        render_pipeline_capture_preview, render_pipeline_capture_refusal, PipelineCaptureCacheEntry,
-        PipelineCapturePlan, PipelineCaptureRefusalClassification, PipelineCaptureRequest,
-        PipelineCaptureStateUpdate, PipelineCaptureStateValue,
+        render_pipeline_capture_preview, render_pipeline_capture_refusal,
+        PipelineCaptureCacheEntry, PipelineCapturePlan, PipelineCaptureRefusalClassification,
+        PipelineCaptureRequest, PipelineCaptureStateUpdate, PipelineCaptureStateValue,
     },
     pipeline_compile::{
         compile_pipeline_stage_with_runtime, render_pipeline_compile_explain,
Diff in /Users/spensermcconnell/__Active_Code/system/crates/pipeline/tests/pipeline_handoff.rs:9:
     pipeline_handoff::{
         emit_pipeline_handoff_bundle, validate_pipeline_handoff_bundle, PipelineHandoffEmitRequest,
         PipelineHandoffManifest, PipelineHandoffRefusalClassification, PipelineHandoffTrustClass,
-        PipelineHandoffValidatedBundle,
-        PipelineHandoffValidationFailureClassification,
+        PipelineHandoffValidatedBundle, PipelineHandoffValidationFailureClassification,
     },
 };
```

### Engine Dry-Run Gate

`cargo publish --dry-run -p handbook-engine`

```text
    Updating crates.io index
   Packaging handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
    Updating crates.io index
    Packaged 20 files, 229.5KiB (41.1KiB compressed)
   Verifying handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
   Compiling proc-macro2 v1.0.106
   Compiling unicode-ident v1.0.24
   Compiling quote v1.0.45
   Compiling typenum v1.19.0
   Compiling version_check v0.9.5
   Compiling thiserror v2.0.18
   Compiling memchr v2.8.0
   Compiling autocfg v1.5.0
   Compiling libc v0.2.184
   Compiling serde_core v1.0.228
   Compiling regex-syntax v0.8.10
   Compiling serde v1.0.228
   Compiling zmij v1.0.21
   Compiling generic-array v0.14.7
   Compiling aho-corasick v1.1.4
   Compiling num-traits v0.2.19
   Compiling hashbrown v0.16.1
   Compiling syn v2.0.117
   Compiling arraydeque v0.5.1
   Compiling equivalent v1.0.2
   Compiling smallvec v1.15.1
   Compiling indexmap v2.13.1
   Compiling cfg-if v1.0.4
   Compiling regex-automata v0.4.14
   Compiling itoa v1.0.18
   Compiling unsafe-libyaml-norway v0.2.15
   Compiling base64 v0.22.1
   Compiling cpufeatures v0.2.17
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling digest v0.10.7
   Compiling sha2 v0.10.9
   Compiling thiserror-impl v2.0.18
   Compiling serde_derive v1.0.228
   Compiling regex v1.12.3
   Compiling saphyr-parser-bw v0.0.611
   Compiling serde_yaml_bw v2.5.4
   Compiling handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/target/package/handbook-engine-0.1.0)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.23s
   Uploading handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
warning: aborting upload due to dry run
```

### Publish Decision / Resume Point

- No real `cargo publish` command was run.
- `handbook-engine` was not published, so dependent dry-runs for `handbook-pipeline` / `handbook-flow` were not started.
- crates.io index-resolution wait/retry behavior: none; no post-publish resolution wait began.
- Current blocker: the required release-candidate verification wall is not green because `cargo fmt --all -- --check` fails on pre-existing formatting drift in:
  - `crates/compiler/src/route_state.rs`
  - `crates/pipeline/src/layout.rs`
  - `crates/pipeline/tests/pipeline_capture.rs`
  - `crates/pipeline/tests/pipeline_handoff.rs`
- Exact resume point: fix or explicitly waive that fmt-check blocker, rerun Checklist Section A on the same coordinated train, rerun `cargo publish --dry-run -p handbook-engine`, then continue with the approved `handbook-engine` → `handbook-pipeline` → `handbook-flow` publish order only after the verification wall is honestly green.

### Release-Candidate Recovery on Coordinated `0.1.1` (2026-06-22)

- Packet 3.2 required a real pre-publish blocker-removal fix before any real publish because the release-candidate verification wall was not green.
- The fix changed first-wave crate source (`crates/pipeline/src/layout.rs`) before any real publish, so per this checklist's versioning contract the coordinated `0.1.0` train was abandoned and the first-wave train was bumped to coordinated `0.1.1`.
- Blocker-removal crate-source commit: `b88086ae58a66c8d9c6adf71e98a9555ee5c6e9a` (`b88086a` short).
- Release-candidate workspace status when re-running the clean pre-publish wall:

  ```text
  ## feat/seam-extraction...origin/feat/seam-extraction [ahead 5]
   M AGENTS.md
   M CLAUDE.md
  ```

- The unrelated `AGENTS.md` / `CLAUDE.md` dirt remained intentionally preserved and was not swept into the Packet 3.2 crate-source commit.

#### Coordinated Version / Dependency Reality

- `crates/engine/Cargo.toml` now declares `version = "0.1.1"`.
- `crates/pipeline/Cargo.toml` now declares `version = "0.1.1"` and `handbook-engine = { version = "0.1.1", path = "../engine" }`.
- `crates/flow/Cargo.toml` now declares `version = "0.1.1"` and `handbook-engine = { version = "0.1.1", path = "../engine" }`.

#### Clean Release-Candidate Verification Wall (`b88086a`)

- `cargo fmt --all -- --check --color never` → pass
- `cargo check --workspace` → pass
- `cargo clippy --workspace --all-targets -- -D warnings` → pass
- `cargo test -p handbook-engine` → pass
- `cargo test -p handbook-pipeline --test pipeline_catalog` → pass
- `cargo test -p handbook-pipeline --test pipeline_compile` → pass
- `cargo test -p handbook-pipeline --test pipeline_capture` → pass
- `cargo test -p handbook-pipeline --test pipeline_handoff` → pass
- `cargo test -p handbook-flow` → pass
- `cargo test -p handbook-compiler --test author` → pass

#### Packageability Truth on `0.1.1`

`cargo package -p handbook-engine --allow-dirty`

```text
   Packaging handbook-engine v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
    Updating crates.io index
    Packaged 20 files, 229.5KiB (41.1KiB compressed)
   Verifying handbook-engine v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
   Compiling proc-macro2 v1.0.106
   Compiling unicode-ident v1.0.24
   Compiling quote v1.0.45
   Compiling version_check v0.9.5
   Compiling typenum v1.19.0
   Compiling serde_core v1.0.228
   Compiling memchr v2.8.0
   Compiling autocfg v1.5.0
   Compiling thiserror v2.0.18
   Compiling libc v0.2.184
   Compiling serde v1.0.228
   Compiling regex-syntax v0.8.10
   Compiling zmij v1.0.21
   Compiling generic-array v0.14.7
   Compiling num-traits v0.2.19
   Compiling aho-corasick v1.1.4
   Compiling hashbrown v0.16.1
   Compiling syn v2.0.117
   Compiling arraydeque v0.5.1
   Compiling equivalent v1.0.2
   Compiling smallvec v1.15.1
   Compiling indexmap v2.13.1
   Compiling cfg-if v1.0.4
   Compiling base64 v0.22.1
   Compiling regex-automata v0.4.14
   Compiling itoa v1.0.18
   Compiling unsafe-libyaml-norway v0.2.15
   Compiling cpufeatures v0.2.17
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.7
   Compiling thiserror-impl v2.0.18
   Compiling serde_derive v1.0.228
   Compiling digest v0.10.7
   Compiling sha2 v0.10.9
   Compiling regex v1.12.3
   Compiling saphyr-parser-bw v0.0.611
   Compiling serde_yaml_bw v2.5.4
   Compiling handbook-engine v0.1.1 (/Users/spensermcconnell/__Active_Code/system/target/package/handbook-engine-0.1.1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.55s
```

`cargo package -p handbook-pipeline --allow-dirty`

```text
   Packaging handbook-pipeline v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/pipeline)
    Updating crates.io index
error: failed to prepare local package for uploading

Caused by:
  no matching package named `handbook-engine` found
  location searched: crates.io index
  required by package `handbook-pipeline v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/pipeline)`
```

`cargo package -p handbook-flow --allow-dirty`

```text
   Packaging handbook-flow v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/flow)
    Updating crates.io index
error: failed to prepare local package for uploading

Caused by:
  no matching package named `handbook-engine` found
  location searched: crates.io index
  required by package `handbook-flow v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/flow)`
```

#### Engine Dry-Run Gate on `0.1.1`

`cargo publish --dry-run -p handbook-engine`

```text
    Updating crates.io index
   Packaging handbook-engine v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
    Updating crates.io index
    Packaged 20 files, 229.5KiB (41.1KiB compressed)
   Verifying handbook-engine v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
   Compiling proc-macro2 v1.0.106
   Compiling quote v1.0.45
   Compiling unicode-ident v1.0.24
   Compiling typenum v1.19.0
   Compiling version_check v0.9.5
   Compiling thiserror v2.0.18
   Compiling libc v0.2.184
   Compiling serde_core v1.0.228
   Compiling memchr v2.8.0
   Compiling autocfg v1.5.0
   Compiling regex-syntax v0.8.10
   Compiling zmij v1.0.21
   Compiling serde v1.0.228
   Compiling generic-array v0.14.7
   Compiling num-traits v0.2.19
   Compiling aho-corasick v1.1.4
   Compiling arraydeque v0.5.1
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.16.1
   Compiling smallvec v1.15.1
   Compiling syn v2.0.117
   Compiling indexmap v2.13.1
   Compiling regex-automata v0.4.14
   Compiling base64 v0.22.1
   Compiling itoa v1.0.18
   Compiling cfg-if v1.0.4
   Compiling unsafe-libyaml-norway v0.2.15
   Compiling crypto-common v0.1.7
   Compiling block-buffer v0.10.4
   Compiling digest v0.10.7
   Compiling cpufeatures v0.2.17
   Compiling sha2 v0.10.9
   Compiling thiserror-impl v2.0.18
   Compiling serde_derive v1.0.228
   Compiling regex v1.12.3
   Compiling saphyr-parser-bw v0.0.611
   Compiling serde_yaml_bw v2.5.4
   Compiling handbook-engine v0.1.1 (/Users/spensermcconnell/__Active_Code/system/target/package/handbook-engine-0.1.1)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 5.62s
   Uploading handbook-engine v0.1.1 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
warning: aborting upload due to dry run
```

#### Updated Packet 3.2 State Before Real Publish

- Packet 3.2 is now **publish-ready** on coordinated `0.1.1`.
- Explicit human authorization for real crates.io publication remains present in the parent Packet 3.2 session message, but as of this pre-publish evidence update no real `cargo publish` has run yet.
- Real publish sequence remains the same: `handbook-engine` first, then wait for crates.io resolution, then dependent dry-runs, then `handbook-pipeline`, then `handbook-flow`.

## Honest Status Language

Use these labels precisely:

- **Packet 3.1 landed** = the release contract and checklist are documented.
- **publish-ready** = the release-candidate commit satisfies the preflight evidence and dry-run gates, but no real crates.io publication has happened yet.
- **partially published** = one or more crates were published, but the full `engine → pipeline → flow` wave is not complete.
- **published** = all three crates were published in the approved order and the exact versions were recorded.
- **Substrate-consume-ready** = Packet 4 has switched Substrate to the published crates and passed its downstream verification wall.

Do not collapse these states into one another.
