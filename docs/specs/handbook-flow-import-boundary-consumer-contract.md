# Handbook Flow Import-Boundary Consumer Contract

## Status

- Packet: **6.B.3 — Formalize Consumer Contract**.
- Scope: `handbook-flow` import-boundary contract only; no production-code changes are part of this document.
- Contract basis: live cleaned source inspected on **2026-06-17** in `crates/flow/src/{lib,budget,packet_result,resolver}.rs`, plus the caller-owned rendering seams in `crates/cli/src/rendering.rs` and `crates/compiler/src/rendering/shared.rs`.
- Provenance: the preserved **Packet 6.B.1** evidence section remains below for audit history. Where that preserved evidence describes the pre-cleanup shell-leakage state, the contract sections above it are the current authority because they reflect the post-**6.B.2** live source.

## Packet 6.B.2 Cleanup Outcome Reference

Live source now reflects the narrow cleanup required before freezing this contract:

- `rg -n 'run \`doctor\`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/` returned zero matches, so `handbook-flow` no longer exposes final shell command strings or product-shell wording directly from `crates/flow/src/`.
- `crates/flow/src/packet_result.rs` now keeps ready-path next-step semantics typed as `ReadyPacketNextSafeAction`, and `PacketDecisionSummary.ready_next_safe_action` is that enum rather than a rendered `String`.
- `crates/flow/src/resolver.rs` still exposes typed refusal/blocker semantics through `ResolverNextSafeAction`, `ResolverRefusalCategory`, `ResolverBlockerCategory`, and `PacketSelectionStatus`.
- `crates/cli/src/rendering.rs` and `crates/compiler/src/rendering/shared.rs` now own final shell wording by rendering those typed flow-side enums into operator-facing copy.
- `flow_contract_version() -> &'static str` delegates to `handbook_engine::workspace_contract_version()`.

## Frozen Consumer Boundary

### Public re-export surface from `crates/flow/src/lib.rs`

- `budget`: `evaluate_budget`, `BudgetDisposition`, `BudgetOutcome`, `BudgetPolicy`, `BudgetReason`, `BudgetTarget`, `NextSafeAction`
- `packet_result`: `PacketBodyNote`, `PacketBodyNoteKind`, `PacketDecisionSummary`, `PacketFixtureContext`, `PacketResult`, `PacketSection`, `PacketSectionMode`, `PacketSourceSummary`, `PacketVariant`, `ReadyPacketNextSafeAction`
- `resolver`: `resolve`, `PacketSelection`, `PacketSelectionStatus`, `ResolveRequest`, `ResolverBlocker`, `ResolverBlockerCategory`, `ResolverNextSafeAction`, `ResolverRefusal`, `ResolverRefusalCategory`, `ResolverResult`, `ResolverSubjectRef`, `C04_RESULT_VERSION`
- `flow_contract_version() -> &'static str`

### Exact public symbols and allowed transitive type dependencies

#### `budget`

| Exact public symbol | Transitive type dependencies | Boundary result |
|---|---|---|
| `BudgetDisposition` | self only | flow-local only |
| `BudgetReason` | self only | flow-local only |
| `NextSafeAction` | `&'static str` payload in `ReduceCanonicalArtifactSize` | std + flow-local only |
| `BudgetTarget` | `&'static str`, `u64` | std + flow-local only |
| `BudgetOutcome` | `BudgetDisposition`, `BudgetReason`, `Vec<BudgetTarget>`, `Option<NextSafeAction>` | std + flow-local only |
| `BudgetPolicy` | `Option<u64>` fields | std + flow-local only |
| `evaluate_budget(artifacts: &[CanonicalArtifactIdentity], policy: BudgetPolicy) -> BudgetOutcome` | `CanonicalArtifactIdentity`, `ArtifactPresence` from `handbook_engine`; `BudgetPolicy`, `BudgetOutcome` from flow | engine-public + std + flow-local only |

#### `packet_result`

| Exact public symbol | Transitive type dependencies | Boundary result |
|---|---|---|
| `PacketVariant` | self only | flow-local only |
| `PacketVariant::as_str(self) -> &'static str` | `PacketVariant`, `&'static str` | std + flow-local only |
| `PacketSourceSummary` | `CanonicalArtifactKind`, `ArtifactPresence` from `handbook_engine`; `&'static str`, `bool`, `Option<u64>`, `Option<String>` | engine-public + std only |
| `PacketBodyNoteKind` | self only | flow-local only |
| `PacketSectionMode` | self only | flow-local only |
| `PacketBodyNote` | `PacketBodyNoteKind`, `String` | std + flow-local only |
| `PacketSection` | `CanonicalArtifactKind` from `handbook_engine`; `&'static str`, `String`, `PacketSectionMode` | engine-public + std + flow-local only |
| `PacketFixtureContext` | `String`, `Vec<PacketSourceSummary>` | engine-public + std + flow-local only |
| `ReadyPacketNextSafeAction` | self only | flow-local only |
| `PacketDecisionSummary` | `PacketSelectionStatus`, `BudgetDisposition`, `BudgetReason`, `usize`, `String`, `ReadyPacketNextSafeAction` | std + flow-local only |
| `PacketResult` | `String`, `PacketVariant`, `Option<PacketFixtureContext>`, `Vec<PacketSourceSummary>`, `Vec<PacketBodyNote>`, `PacketDecisionSummary`, `Vec<PacketSection>` | engine-public + std + flow-local only |
| `PacketResult::is_ready(&self) -> bool` | `PacketDecisionSummary.packet_status`, `PacketSelectionStatus`, `bool` | std + flow-local only |

#### `resolver`

| Exact public symbol | Transitive type dependencies | Boundary result |
|---|---|---|
| `C04_RESULT_VERSION` | `&'static str` | std only |
| `ResolverRefusalCategory` | self only | flow-local only |
| `ResolverSubjectRef` | `CanonicalArtifactKind` from `handbook_engine`; `&'static str`, `String`, `Option<String>` | engine-public + std + flow-local only |
| `ResolverNextSafeAction` | `&'static str` payloads for canonical paths and packet ids | std + flow-local only |
| `ResolverRefusal` | `ResolverRefusalCategory`, `String`, `ResolverSubjectRef`, `ResolverNextSafeAction` | engine-public + std + flow-local only |
| `ResolverBlockerCategory` | self only | flow-local only |
| `ResolverBlocker` | `ResolverBlockerCategory`, `ResolverSubjectRef`, `String`, `ResolverNextSafeAction` | engine-public + std + flow-local only |
| `ResolveRequest` | `BudgetPolicy`, `&'static str` | std + flow-local only |
| `PacketSelectionStatus` | self only | flow-local only |
| `PacketSelection` | `String`, `PacketSelectionStatus` | std + flow-local only |
| `ResolverResult` | `String`, `u32`, `PacketResult`, `Vec<String>`, `BudgetOutcome`, `PacketSelection`, `Option<ResolverRefusal>`, `Vec<ResolverBlocker>` | engine-public + std + flow-local only |
| `resolve(repo_root: impl AsRef<Path>, request: ResolveRequest) -> Result<ResolverResult, ManifestError>` | `AsRef<Path>`, `Path` from `std`; `ResolveRequest`, `ResolverResult` from flow; `ManifestError` from `handbook_engine` | engine-public + std + flow-local only |

### In-boundary typed semantics after Packet 6.B.2

The following semantics remain explicitly in boundary because they are typed, machine-readable result data rather than final shell copy:

- `PacketSelectionStatus::{Selected, Blocked}` remains the packet-level readiness status.
- `ReadyPacketNextSafeAction::{InspectProof, Generate, RunDoctor}` remains the ready-path action enum.
- `ResolverNextSafeAction::{RunSetup, RunSetupInit, RunSetupRefresh, RunAuthorCharter, RunAuthorProjectContext, RunAuthorEnvironmentInventory, CreateSystemRoot { canonical_repo_relative_path }, EnsureSystemRootIsDirectory { canonical_repo_relative_path }, RemoveSystemRootSymlink { canonical_repo_relative_path }, CreateCanonicalArtifact { canonical_repo_relative_path }, FillCanonicalArtifact { canonical_repo_relative_path }, ReduceCanonicalArtifactSize { canonical_repo_relative_path }, RunGenerate { packet_id }, RunDoctor}` remains the refusal/blocker action enum.
- `ResolverRefusalCategory`, `ResolverBlockerCategory`, `BudgetDisposition`, `BudgetReason`, `PacketVariant`, and `NextSafeAction` remain typed classifiers or typed budget actions.
- `PacketDecisionSummary.summary_line: String` remains in boundary as flow-owned status summary text, but it is not the shell-command rendering seam that Packet 6.B.2 removed.

### Explicitly out of boundary

The following responsibilities are now explicitly outside the frozen `handbook-flow` import boundary:

- Rendering `ReadyPacketNextSafeAction` into operator-facing strings such as `run \`handbook inspect --packet ...\` for proof`, `run \`handbook generate --packet ...\``, and `run \`doctor\``.
- Rendering `ResolverNextSafeAction` into operator-facing recovery strings such as `run \`handbook setup\``, `run \`handbook setup refresh\``, `run \`handbook author ...\``, and `run \`handbook generate --packet ...\``.
- CLI/compiler-specific wording choices, status labels, fallback shell phrasing, and fixture-aware command assembly.
- Any shell/product-surface copy that depends on presentation context rather than typed flow-state semantics.

Current caller-owned rendering seams observed during this packet:

- `crates/cli/src/rendering.rs`
- `crates/compiler/src/rendering/shared.rs`

### Version contract

- `flow_contract_version() -> &'static str` delegates directly to `handbook_engine::workspace_contract_version()`.
- At the inspected live source, `handbook_engine::workspace_contract_version()` returns `"C-02"`, so `handbook-flow` inherits the workspace contract version instead of defining an independent flow-only version constant.

### Verification references used for this contract

Commands run during Packet 6.B.3 verification:

- `git status --short --branch`
- `sed -n '1,220p' docs/specs/handbook-flow-import-boundary-consumer-contract.md`
- `rg -n "pub (fn|struct|enum|type|const)" crates/flow/src/resolver.rs crates/flow/src/budget.rs crates/flow/src/packet_result.rs`
- `rg -n "flow_contract_version" crates/flow/src/`
- `sed -n '1,260p' crates/flow/src/lib.rs`
- `sed -n '1,220p' crates/flow/src/budget.rs`
- `sed -n '1,200p' crates/flow/src/packet_result.rs`
- `sed -n '1,620p' crates/flow/src/resolver.rs`
- `sed -n '620,1120p' crates/flow/src/resolver.rs`
- `sed -n '1,120p' crates/compiler/src/rendering/shared.rs`
- `sed -n '500,620p' crates/cli/src/rendering.rs`
- `sed -n '860,980p' crates/cli/src/rendering.rs`
- `rg -n 'run \`doctor\`|handbook inspect --packet|handbook generate --packet|handbook setup' crates/flow/src/`
- `cargo tree -p handbook-flow`
- `cargo test -p handbook-flow`
- `cargo check --workspace`

## Evidence preservation note

The following `## Evidence` section is preserved from **Packet 6.B.1** as historical provenance. It intentionally captures the pre-6.B.2 evidence state, including the then-live shell-leakage observations. The contract sections above are the current post-cleanup authority.

## Evidence

### Packet scope

- Packet: **6.B.1 — Gather Flow Import-Boundary Evidence** only.
- Authority followed exactly:
  - `docs/specs/handbook-engine-extraction-phase-6-remaining-work-spec.md`
  - `docs/specs/handbook-engine-extraction-phase-6-remaining-work-plan.md`
  - `docs/specs/handbook-engine-extraction-phase-6-remaining-work-tasks.md`
- This file intentionally records **evidence only**. It does **not** formalize the full frozen consumer contract yet.

### Required live verification run before writing

#### `git status --short --branch`

```text
## feat/seam-extraction...origin/feat/seam-extraction [ahead 2]
```

#### `cargo tree -p handbook-flow`

Exact output:

```text
handbook-flow v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/flow)
└── handbook-engine v0.1.0 (/Users/spensermcconnell/__Active_Code/system/crates/engine)
    ├── libc v0.2.184
    ├── serde v1.0.228
    │   ├── serde_core v1.0.228
    │   └── serde_derive v1.0.228 (proc-macro)
    │       ├── proc-macro2 v1.0.106
    │       │   └── unicode-ident v1.0.24
    │       ├── quote v1.0.45
    │       │   └── proc-macro2 v1.0.106 (*)
    │       └── syn v2.0.117
    │           ├── proc-macro2 v1.0.106 (*)
    │           ├── quote v1.0.45 (*)
    │           └── unicode-ident v1.0.24
    ├── serde_yaml_bw v2.5.4
    │   ├── base64 v0.22.1
    │   ├── indexmap v2.13.1
    │   │   ├── equivalent v1.0.2
    │   │   └── hashbrown v0.16.1
    │   ├── itoa v1.0.18
    │   ├── num-traits v0.2.19
    │   │   [build-dependencies]
    │   │   └── autocfg v1.5.0
    │   ├── regex v1.12.3
    │   │   ├── aho-corasick v1.1.4
    │   │   │   └── memchr v2.8.0
    │   │   ├── memchr v2.8.0
    │   │   ├── regex-automata v0.4.14
    │   │   │   ├── aho-corasick v1.1.4 (*)
    │   │   │   ├── memchr v2.8.0
    │   │   │   └── regex-syntax v0.8.10
    │   │   └── regex-syntax v0.8.10
    │   ├── saphyr-parser-bw v0.0.611
    │   │   ├── arraydeque v0.5.1
    │   │   ├── smallvec v1.15.1
    │   │   └── thiserror v2.0.18
    │   │       └── thiserror-impl v2.0.18 (proc-macro)
    │   │           ├── proc-macro2 v1.0.106 (*)
    │   │           ├── quote v1.0.45 (*)
    │   │           └── syn v2.0.117 (*)
    │   ├── serde v1.0.228 (*)
    │   ├── unsafe-libyaml-norway v0.2.15
    │   └── zmij v1.0.21
    └── sha2 v0.10.9
        ├── cfg-if v1.0.4
        ├── cpufeatures v0.2.17
        │   └── libc v0.2.184
        └── digest v0.10.7
            ├── block-buffer v0.10.4
            │   └── generic-array v0.14.7
            │       └── typenum v1.19.0
            │       [build-dependencies]
            │       └── version_check v0.9.5
            └── crypto-common v0.1.7
                ├── generic-array v0.14.7 (*)
                └── typenum v1.19.0
[dev-dependencies]
└── tempfile v3.27.0
    ├── fastrand v2.4.1
    ├── getrandom v0.4.2
    │   ├── cfg-if v1.0.4
    │   └── libc v0.2.184
    ├── once_cell v1.21.4
    └── rustix v1.1.4
        ├── bitflags v2.11.0
        ├── errno v0.3.14
        │   └── libc v0.2.184
        └── libc v0.2.184
```

Finding: `handbook-flow` has exactly one intra-workspace dependency: `handbook-engine`.

#### `rg -n "handbook_compiler|handbook_cli|handbook_pipeline" crates/flow/src/ crates/flow/tests/`

Exact stdout:

```text
```

Exit status: `1` (zero matches).

#### Required source inspections

- Required packet commands run exactly:
  - `sed -n '70,210p' crates/flow/src/resolver.rs`
  - `sed -n '1,140p' crates/flow/src/packet_result.rs`
  - `sed -n '1,220p' crates/flow/src/budget.rs`
  - `sed -n '518,590p' crates/cli/src/rendering.rs`
  - `sed -n '315,360p' crates/compiler/src/rendering/shared.rs`
- Additional live review-fix verification recorded for the ready-path passthrough claim:
  - `sed -n '1,60p' crates/compiler/src/rendering/shared.rs`
  - `rg -n "ready_next_safe_action|packet\.decision_summary\.ready_next_safe_action" crates/compiler/src/rendering/shared.rs crates/compiler/src/rendering/inspect.rs crates/cli/src/rendering.rs`
- Additional resolver/engine coverage recorded for this Packet 6.B.1 review fix:
  - `sed -n '1,120p' crates/flow/src/resolver.rs`
  - `sed -n '400,470p' crates/flow/src/resolver.rs`
  - `sed -n '1,240p' crates/engine/src/lib.rs`
- Supplemental live cross-checks used to keep the evidence honest:
  - `sed -n '1,260p' crates/flow/src/lib.rs`
  - `sed -n '1060,1110p' crates/flow/src/resolver.rs`
  - `rg -n 'run \`doctor\`|handbook inspect --packet|handbook generate --packet|handbook setup|matches_setup_starter_template|next_safe_action_for_ready_packet|ready_next_safe_action' crates/flow/src/resolver.rs crates/flow/src/packet_result.rs crates/cli/src/rendering.rs crates/compiler/src/rendering/shared.rs`

Resolver symbol coverage used below is anchored to the recorded live ranges: `sed -n '1,120p' crates/flow/src/resolver.rs` covers `C04_RESULT_VERSION`, `ResolverRefusalCategory`, `ResolverSubjectRef`, and the `handbook_engine` imports; `sed -n '70,210p' crates/flow/src/resolver.rs` covers `ResolverNextSafeAction`, `ResolverRefusal`, `ResolverBlockerCategory`, and `ResolverBlocker`; `sed -n '400,470p' crates/flow/src/resolver.rs` covers `ResolveRequest`, `PacketSelectionStatus`, `PacketSelection`, `ResolverResult`, and the `resolve(...)` entrypoint; `sed -n '1060,1110p' crates/flow/src/resolver.rs` is retained only for the ready-path shell-wording cross-check.

### Import-surface inventory from `crates/flow/src/lib.rs`

Public surface re-exported by `handbook-flow`:

- `budget`: `evaluate_budget`, `BudgetDisposition`, `BudgetOutcome`, `BudgetPolicy`, `BudgetReason`, `BudgetTarget`, `NextSafeAction`
- `packet_result`: `PacketBodyNote`, `PacketBodyNoteKind`, `PacketDecisionSummary`, `PacketFixtureContext`, `PacketResult`, `PacketSection`, `PacketSectionMode`, `PacketSourceSummary`, `PacketVariant`
- `resolver`: `resolve`, `PacketSelection`, `PacketSelectionStatus`, `ResolveRequest`, `ResolverBlocker`, `ResolverBlockerCategory`, `ResolverNextSafeAction`, `ResolverRefusal`, `ResolverRefusalCategory`, `ResolverResult`, `ResolverSubjectRef`, `C04_RESULT_VERSION`
- `flow_contract_version()` delegates to `handbook_engine::workspace_contract_version()`

### `use`-statement exclusion proof for `crates/flow/src/*.rs`

Observed `use` lines:

```text
crates/flow/src/packet_result.rs:1:use crate::budget::{BudgetDisposition, BudgetReason};
crates/flow/src/packet_result.rs:2:use crate::resolver::PacketSelectionStatus;
crates/flow/src/packet_result.rs:3:use handbook_engine::{ArtifactPresence, CanonicalArtifactKind};
crates/flow/src/resolver.rs:1:use crate::budget::{
crates/flow/src/resolver.rs:5:use crate::packet_result::{
crates/flow/src/resolver.rs:9:use handbook_engine::{
crates/flow/src/resolver.rs:17:use std::cmp::Ordering;
crates/flow/src/resolver.rs:18:use std::path::Path;
crates/flow/src/budget.rs:1:use handbook_engine::{ArtifactPresence, CanonicalArtifactIdentity};
```

Finding: every `use` statement in `crates/flow/src/*.rs` references only `crate::*`, `handbook_engine::*`, or `std::*`.

Additional fully-qualified `handbook_engine::` uses found outside `use` statements:

```text
crates/flow/src/resolver.rs:40:    handbook_engine::baseline_validation::baseline_artifact_validations(
crates/flow/src/lib.rs:22:    handbook_engine::workspace_contract_version()
```

These also stay inside the `handbook_engine` public surface.

### Transitive type-dependency traces for the in-boundary symbol set

#### `budget`

| Symbol | Transitive type dependencies | Result |
|---|---|---|
| `BudgetDisposition` | none beyond self | std/flow-only |
| `BudgetReason` | none beyond self | std/flow-only |
| `NextSafeAction` | `&'static str` | std-only |
| `BudgetTarget` | `&'static str`, `u64` | std-only |
| `BudgetOutcome` | `BudgetDisposition` -> self, `BudgetReason` -> self, `Vec<BudgetTarget>` -> std + self, `Option<NextSafeAction>` -> std + self | std/flow-only |
| `BudgetPolicy` | `Option<u64>` | std-only |
| `evaluate_budget(artifacts: &[CanonicalArtifactIdentity], policy: BudgetPolicy) -> BudgetOutcome` | `CanonicalArtifactIdentity` -> `handbook_engine` public type, `BudgetPolicy` -> std/flow-only, `BudgetOutcome` -> std/flow-only; implementation also reads `ArtifactPresence` -> `handbook_engine` public enum | engine-public + std only |

Implementation note: `evaluate_budget` uses only `handbook_engine::{ArtifactPresence, CanonicalArtifactIdentity}` and public fields on `CanonicalArtifactIdentity`; no compiler/cli/pipeline/setup/doctor types are imported.

#### `packet_result`

| Symbol | Transitive type dependencies | Result |
|---|---|---|
| `PacketVariant` | none beyond self | std/flow-only |
| `PacketVariant::as_str(self) -> &'static str` | `PacketVariant` -> self, `&'static str` | std/flow-only |
| `PacketSourceSummary` | `CanonicalArtifactKind` -> `handbook_engine` public enum, `&'static str`, `bool`, `ArtifactPresence` -> `handbook_engine` public enum, `Option<u64>`, `Option<String>` | engine-public + std only |
| `PacketBodyNoteKind` | none beyond self | std/flow-only |
| `PacketSectionMode` | none beyond self | std/flow-only |
| `PacketBodyNote` | `PacketBodyNoteKind` -> self, `String` | std/flow-only |
| `PacketSection` | `CanonicalArtifactKind` -> `handbook_engine` public enum, `&'static str`, `String`, `PacketSectionMode` -> self | engine-public + std only |
| `PacketFixtureContext` | `String`, `Vec<PacketSourceSummary>` -> std + engine-public via `PacketSourceSummary` | engine-public + std only |
| `PacketDecisionSummary` | `PacketSelectionStatus` -> flow resolver type, `BudgetDisposition`/`BudgetReason` -> flow budget types, `usize`, `String` | std/flow-only |
| `PacketResult` | `String`, `PacketVariant`, `Option<PacketFixtureContext>`, `Vec<PacketSourceSummary>`, `Vec<PacketBodyNote>`, `PacketDecisionSummary`, `Vec<PacketSection>` | engine-public + std only via nested packet types |
| `PacketResult::is_ready(&self) -> bool` | `PacketDecisionSummary.packet_status` -> `PacketSelectionStatus`, `bool` | std/flow-only |

Implementation note: `packet_result.rs` imports only `crate::budget`, `crate::resolver::PacketSelectionStatus`, and `handbook_engine::{ArtifactPresence, CanonicalArtifactKind}`.

#### `resolver`

| Symbol | Transitive type dependencies | Result |
|---|---|---|
| `C04_RESULT_VERSION` | `&'static str` | std-only |
| `ResolverRefusalCategory` | none beyond self | std/flow-only |
| `ResolverSubjectRef` | `CanonicalArtifactKind` -> `handbook_engine` public enum, `&'static str`, `String`, `Option<String>` | engine-public + std only |
| `ResolverNextSafeAction` | `&'static str` fields only | std-only |
| `ResolverRefusal` | `ResolverRefusalCategory`, `String`, `ResolverSubjectRef`, `ResolverNextSafeAction` | engine-public + std only through nested `ResolverSubjectRef` |
| `ResolverBlockerCategory` | none beyond self | std/flow-only |
| `ResolverBlocker` | `ResolverBlockerCategory`, `ResolverSubjectRef`, `String`, `ResolverNextSafeAction` | engine-public + std only through nested `ResolverSubjectRef` |
| `ResolveRequest` | `BudgetPolicy`, `&'static str` | std/flow-only |
| `Default for ResolveRequest` | `BudgetPolicy::default()` -> flow budget type with std fields only, `DEFAULT_PACKET_ID` -> `&'static str` | std/flow-only |
| `PacketSelectionStatus` | none beyond self | std/flow-only |
| `PacketSelection` | `String`, `PacketSelectionStatus` | std/flow-only |
| `ResolverResult` | `String`, `u32`, `PacketResult`, `Vec<String>`, `BudgetOutcome`, `PacketSelection`, `Option<ResolverRefusal>`, `Vec<ResolverBlocker>` | engine-public + std only through nested `PacketResult` / `ResolverRefusal` / `ResolverBlocker` |
| `resolve(repo_root: impl AsRef<Path>, request: ResolveRequest) -> Result<ResolverResult, ManifestError>` | `AsRef<Path>` -> std, `ResolveRequest` -> std/flow-only, `ResolverResult` -> engine-public + std only through nested flow types, `ManifestError` -> `handbook_engine` public enum | engine-public + std only |

Implementation-only engine surface used by `resolve` and helper routines:

- public engine functions/modules:
  - `CanonicalArtifacts::load`
  - `ArtifactManifest::from_canonical_artifacts`
  - `ManifestInputs::default`
  - `baseline_artifact_validation_for_path`
  - `handbook_engine::baseline_validation::baseline_artifact_validations`
  - `default_canonical_layout_contract`
  - `validate_charter_markdown`
  - `validate_project_context_markdown`
  - `validate_environment_inventory_markdown`
- public engine data types/enums:
  - `ArtifactIngestIssueKind`
  - `ArtifactManifest`
  - `ArtifactPresence`
  - `BaselineArtifactValidation`
  - `BaselineArtifactVerdict`
  - `CanonicalArtifact`
  - `CanonicalArtifactKind`
  - `CanonicalArtifacts`
  - `FreshnessIssueKind`
  - `FreshnessStatus`
  - `ManifestError`
  - `ManifestInputs`
  - `SystemRootStatus`

Finding: all resolver implementation dependencies that cross the crate boundary resolve through `handbook_engine` public modules or `pub use` re-exports exposed by `crates/engine/src/lib.rs`. No resolver symbol requires engine-internal/private modules.

#### `lib`

| Symbol | Transitive type dependencies | Result |
|---|---|---|
| `flow_contract_version() -> &'static str` | return type `&'static str`; implementation delegates to `handbook_engine::workspace_contract_version()` -> engine-public function | engine-public + std only |

### Flag check: engine types beyond the engine public surface

Flagged symbols: **none**.

Every cross-crate type/function reference observed in `crates/flow/src/{resolver,budget,packet_result,lib}.rs` resolves through `handbook_engine` public modules or public re-exports declared by `crates/engine/src/lib.rs`.

### Evidence and limits for CLI/compiler/doctor/setup/pipeline concerns

1. **Compiler / CLI / pipeline imports:** excluded by the zero-match `rg` result across `crates/flow/src/` and `crates/flow/tests/`.
2. **All `use` statements in `crates/flow/src/*.rs`:** limited to `crate::*`, `handbook_engine::*`, and `std::*`.
3. **Pipeline coupling:** no imports or type references to pipeline loading/selection/compile/capture/handoff/route modules or types were found in `crates/flow/src/*.rs`.
4. **Compiler glue coupling:** no imports or type references to compiler rendering/refusal/error glue were found in `crates/flow/src/*.rs`.
5. **CLI shell-module coupling:** no imports or type references to clap/help/exit-code/product-shell modules were found in `crates/flow/src/*.rs`.
6. **Live public/observable flow surface still includes doctor/setup/CLI-adjacent behavior:**
   - `ResolverNextSafeAction` publicly exposes `RunSetup`, `RunSetupInit`, `RunSetupRefresh`, `RunGenerate`, and `RunDoctor` in `crates/flow/src/resolver.rs:79-106`.
   - `next_safe_action_for_ready_packet()` returns final user-facing command strings such as `run \`doctor\`` and `run \`handbook inspect --packet ...\` for proof` in `crates/flow/src/resolver.rs:1079-1098`.
   - `PacketDecisionSummary.ready_next_safe_action: String` keeps that final rendered shell copy on the public flow surface in `crates/flow/src/packet_result.rs:69-74`.
   - The recorded ready-path passthrough lives in the live ranges used for this fix: `sed -n '518,590p' crates/cli/src/rendering.rs` plus the recorded grep hit at `crates/cli/src/rendering.rs:540`, and `sed -n '1,60p' crates/compiler/src/rendering/shared.rs` plus the recorded grep hit at `crates/compiler/src/rendering/shared.rs:42`, both of which return `packet.decision_summary.ready_next_safe_action.clone()` for ready packets.
   - The separately recorded refusal/blocker shell-wording helpers remain in `sed -n '518,590p' crates/cli/src/rendering.rs` and `sed -n '315,360p' crates/compiler/src/rendering/shared.rs`, where typed `NextSafeAction` values are rendered into final shell wording for non-ready paths.
   - Together those recorded live ranges show the remaining shell-owned seam honestly: CLI/compiler already own typed refusal/blocker rendering, but ready packets still pass through the final `ready_next_safe_action` shell copy coming from `handbook-flow`. That is the residual seam Packet 6.B.2 must move out of `handbook-flow`.
   - setup-starter-template handling still participates in resolver packet logic via `matches_setup_starter_template` branches in `crates/flow/src/resolver.rs:802-804` and `866-872`.
7. **Typed semantics vs final shell wording:** the typed/machine-readable side of the flow surface still consists of enums and categories such as `ResolverNextSafeAction`, `ResolverBlockerCategory`, `ResolverRefusalCategory`, `PacketSelectionStatus`, and the budget/result enums. The final shell-owned/operator-facing side is the rendered command/copy string surface: `next_safe_action_for_ready_packet()` in `resolver.rs` plus `PacketDecisionSummary.ready_next_safe_action: String` in `packet_result.rs`.
8. **Honest conclusion for Packet 6.B.1 evidence:** the live source proves absence of extra crate imports/coupling to `handbook_cli`, `handbook_compiler`, and `handbook_pipeline`, and it proves resolver/budget/packet-result implementation dependencies stay within `handbook_engine` + std + flow-local types. It does **not** prove full exclusion of doctor/setup/CLI concerns from the public or observable `handbook-flow` surface, because the public flow result still carries final rendered shell wording via `ready_next_safe_action`, even though CLI/compiler already own most typed-next-action rendering for refusal/blocker paths.

### Exclusion proof for tests

The coupling scan covered `crates/flow/tests/` as well as `crates/flow/src/`. The zero-match result means the test surface also contains no `handbook_compiler`, `handbook_cli`, or `handbook_pipeline` references.
