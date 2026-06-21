# Tasks: Handbook Substrate Parameterization — Set 3: Import-Surface Default / Validation Honesty Cleanup

Plan reference: [handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-plan.md](./handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-plan.md)

Spec reference: [handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md](./handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-spec.md)

---

## Packet 3.1: Flow Residual Refusal And Fallback Cleanup

- [ ] Task: Clean or explicitly bound residual flow refusal/blocker/result surfaces that still encode handbook-product canonical-root wording
  - Acceptance: the supported flow import-facing story no longer reports contract-sensitive refusal/blocker/result surfaces as if `.handbook/**` were still the active truth when a non-default canonical layout contract is in use. Any remaining `.handbook/**` references in flow source are explicitly bounded as default-helper or proof-only behavior.
  - Verify: Source inspection of `crates/flow/src/resolver.rs` and `crates/flow/tests/resolver_core.rs`; `cargo test -p handbook-flow --test resolver_core && cargo test -p handbook-flow`.
  - Files: `crates/flow/src/resolver.rs`, `crates/flow/tests/resolver_core.rs`

- [ ] Task: Preserve the explicit default wrapper path while keeping downstream-visible proof surfaces honest
  - Acceptance: `resolve(...)` remains the explicit handbook-product default helper path, but proof/result surfaces such as packet fixture context or other downstream-visible summaries no longer silently force default-root semantics back into the supported import-facing story.
  - Verify: `rg -n "missing canonical \.handbook root|canonical \.handbook root|\.handbook/" crates/flow/src crates/flow/tests` cross-checked against the active contract-aware path and source inspection of the ready/refusal/blocker packet-result surfaces.
  - Files: `crates/flow/src/resolver.rs`, `crates/flow/tests/resolver_core.rs`

## Packet 3.2: Pipeline Validation / Refusal Wording Cleanup

- [ ] Task: Derive import-facing pipeline validation and refusal wording from the active declarative roots
  - Acceptance: import-facing pipeline/stage path checks no longer hardcode handbook-product `core/pipelines/` or `core/stages/` roots when the active declarative contract differs.
  - Verify: Source inspection of `crates/pipeline/src/pipeline.rs` plus `cargo test -p handbook-pipeline --test pipeline_loader && cargo test -p handbook-pipeline --test pipeline_compile && cargo test -p handbook-pipeline --test pipeline_route_resolution`.
  - Files: `crates/pipeline/src/pipeline.rs`, `crates/pipeline/tests/pipeline_loader.rs`, `crates/pipeline/tests/pipeline_compile.rs`, `crates/pipeline/tests/pipeline_route_resolution.rs`

- [ ] Task: Keep explicit default-helper surfaces bounded while removing misleading validation text from the reusable import story
  - Acceptance: explicit handbook-product defaults in `declarative_roots.rs` / `layout.rs` remain valid as named defaults, but import-facing validation/refusal messaging no longer implies those defaults are the only supported layout.
  - Verify: `cargo test -p handbook-pipeline --test pipeline_catalog` plus `rg -n "core/stages/|core/pipelines/|must live under|pipeline YAML must" crates/pipeline/src/pipeline.rs crates/pipeline/tests` cross-checked against the active declarative contract story.
  - Files: `crates/pipeline/src/pipeline.rs`, `crates/pipeline/src/declarative_roots.rs`, `crates/pipeline/src/layout.rs`, `crates/pipeline/tests/pipeline_catalog.rs`

## Packet 3.3: Engine Residual Default Bounding And Final Proof

- [x] Task: Clean or explicitly bound engine-side residual default references that affect the import-target contract story
  - Acceptance: engine import-target surfaces either derive contract-sensitive user-visible defaults from the active canonical layout contract or explicitly bound the remaining `.handbook/**` references as handbook-product authoring/default behavior outside the reusable import promise.
  - Verify: Source inspection of `crates/engine/src/canonical_artifacts.rs`, `crates/engine/src/canonical_paths.rs`, and `crates/engine/src/author/*.rs` plus `cargo test -p handbook-engine --test canonical_artifacts_ingest && cargo test -p handbook-engine --test baseline_validation && cargo test -p handbook-engine --test author_core`.
  - Files: `crates/engine/src/canonical_artifacts.rs`, `crates/engine/src/canonical_paths.rs`, `crates/engine/src/author/charter_core.rs`, `crates/engine/src/author/environment_inventory_core.rs`, `crates/engine/tests/author_core.rs`

- [ ] Task: Run the Set 3 verification wall and record the final bounded-default inventory honestly
  - Acceptance: all of the following pass:
    - `cargo test -p handbook-flow --test resolver_core`
    - `cargo test -p handbook-flow`
    - `cargo test -p handbook-pipeline --test pipeline_catalog`
    - `cargo test -p handbook-pipeline --test pipeline_loader`
    - `cargo test -p handbook-pipeline --test pipeline_compile`
    - `cargo test -p handbook-pipeline --test pipeline_route_resolution`
    - `cargo test -p handbook-engine --test canonical_artifacts_ingest`
    - `cargo test -p handbook-engine --test baseline_validation`
    - `cargo test -p handbook-engine --test author_core`
    - `cargo check --workspace`
    - `cargo fmt --all -- --check`
    - `cargo clippy --workspace --all-targets -- -D warnings`
  - Verify: Run each command and record pass/fail in the completion notes below.
  - Files: `docs/specs/handbook-substrate-parameterization-set-3-import-surface-honesty-cleanup-tasks.md`

#### Packet 3.3 completion notes

- Verification wall:
  - PASS — `cargo test -p handbook-flow --test resolver_core`
  - PASS — `cargo test -p handbook-flow`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_catalog`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_loader`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_compile`
  - PASS — `cargo test -p handbook-pipeline --test pipeline_route_resolution`
  - PASS — `cargo test -p handbook-engine --test canonical_artifacts_ingest`
  - PASS — `cargo test -p handbook-engine --test baseline_validation`
  - PASS — `cargo test -p handbook-engine --test author_core`
  - PASS — `cargo check --workspace`
  - FAIL — `cargo fmt --all -- --check` (reran after `rustfmt` on Packet 3.3 files; remaining workspace drift is outside Packet 3.3 in `crates/flow/src/resolver.rs:1119` and `crates/flow/tests/resolver_core.rs:196`)
  - FAIL — `cargo clippy --workspace --all-targets -- -D warnings` (`clippy::explicit_auto_deref` at `crates/pipeline/src/pipeline.rs:3012`; outside Packet 3.3 scope)
- Final bounded-default inventory:
  - Removed misleading engine import-facing default wording: `ArtifactIngestError` no longer hardcodes `.handbook` in canonical system-root display text.
  - Acceptable retained handbook-product defaults inside the import-target crates:
    - `crates/engine/src/canonical_paths.rs` keeps `DEFAULT_CANONICAL_LAYOUT_CONTRACT` and the derived `CANONICAL_*` constants as the explicit handbook-product default owner for callers that choose the default layout; non-default callers still flow through `CanonicalLayoutContract`.
    - `crates/engine/src/author/charter_core.rs` keeps `DEFAULT_EXCEPTION_RECORD_LOCATION` as a handbook-product default for engine-authored charter markdown, explicitly bounded as code-owned authoring behavior rather than reusable import-contract truth.
    - `crates/engine/src/author/environment_inventory_core.rs` keeps the handbook-product canonical-file / project-context reference lines as engine-owned synthesized-markdown validation defaults, explicitly bounded outside the reusable import promise.
  - No remaining engine-side misleading defaults were found that require reopening Packet 3.1 or Packet 3.2; the remaining engine `.handbook/**` hits are explicit default-owner or authoring-only surfaces, not active import-facing contract claims.
  - Packet 3.3 proof is still blocked on out-of-scope workspace hygiene failures in `handbook-flow` / `handbook-pipeline`, so this packet should not be marked fully closed until those pre-existing `fmt` / `clippy` failures are resolved or explicitly waived.
- Proof-only rule:
  - Respected. Packet 3.3 stayed inside engine residual default bounding plus proof-note closeout; no CLI/compiler cleanup or earlier-set structural redesign was folded into this packet.

---

## Stop Boundary

Stop after Packet 3.3 for this set. Do not:

- reopen Set 1 (`handbook-pipeline` import layout) or Set 2 (`handbook-flow` canonical-layout injection) without explicit contradictory proof
- widen into CLI/compiler product-shell cleanup
- execute the actual Substrate import
- automatically widen beyond the import-target crates after Set 3 closes
