---
version: m3-v1
currentness: current
status: drafted
revalidation_triggers:
  - Any change to the accepted pipeline YAML document count, repo-safe path rules, or stage-file boundary.
  - Any change to the supported activation syntax, boolean clause grammar, or variable-name grammar.
  - Any change to the resolved route status vocabulary or the rule that ordering follows pipeline declaration order.
  - Any change to the runtime-only `.system/state/pipeline/` schema, revision protocol, audit trimming policy, or mutation refusal semantics.
  - Any change to the rule that route state is runtime-only under the `.system/` boundary and never canonical project truth.
---

# Pipeline Route and State Core Contract

## Purpose

This contract defines the compiler-owned truth for reduced-v1 pipeline loading, route evaluation, and narrow route-state persistence.

It exists so downstream pipeline surfaces can consume one explicit route/state baseline without rediscovering YAML shape, activation limits, or mutation safety rules from scattered code or seam-local planning notes.

## Canonical Location

- Canonical artifact: `docs/contracts/pipeline-route-and-state-core.md`
- Downstream consumers: the pipeline operator surface, the compile-boundary handoff, the capture preview/apply boundary, and the proof/docs conformance layer

## Owned Surface

This contract is authoritative for:

- the declared pipeline YAML shape accepted by the compiler loader
- repo-safe path rules for pipeline files and referenced stage markdown files
- the supported activation subset for reduced-v1 pipeline routing
- the resolved route status vocabulary and its deterministic ordering rules
- the runtime-only persisted state shape under `.system/state/pipeline/`
- the mutation safety protocol for route-state writes

This contract is not authoritative for CLI wording, help exposure, shorthand ID lookup, compile payload generation, or docs/help cutover work.

## Normative Rules

### Declared pipeline ingest

- A pipeline definition MUST be loaded from a repo-relative path. Absolute paths, parent-directory escapes, and empty paths MUST be refused.
- The pipeline file MUST contain exactly two YAML documents: one header document and one body document.
- The header document MUST define non-empty `kind`, `id`, `version`, `title`, and `description` fields.
- `kind` MUST be exactly `pipeline`.
- The body document MUST define non-empty `defaults.runner` and `defaults.profile` fields plus at least one stage entry.
- Unknown fields in the header, body, stage, or activation documents MUST be refused.
- Declared stage ordering MUST preserve the order written in the pipeline YAML. Compiler consumers MUST NOT reorder stages by filesystem traversal, map iteration, or derived priority.

### Stage file boundary

- Every stage entry MUST define a non-empty `id` and `file`.
- Stage IDs MUST be unique within one pipeline definition.
- Stage files MUST remain repo-relative.
- Stage files MUST live under `core/stages/`.
- Stage files MUST use the `.md` extension.
- Stage files MUST resolve to an existing regular file.
- Stage file paths that escape the repo root, point outside `core/stages/`, use a non-markdown extension, are missing, or are not regular files MUST be refused.

### Supported activation subset

- Reduced-v1 pipeline routing supports only `activation.when.any` and `activation.when.all`.
- Exactly one of `any` or `all` MUST be present for one activation block.
- Each activation clause MUST use exactly one equality operator in the form `variables.<name> == true|false`.
- Variable names MUST begin with an ASCII letter or `_`, and remaining characters MUST be ASCII alphanumeric or `_`.
- Activation values MUST be boolean literals only. String, numeric, inequality, or nested-expression clauses MUST be refused.
- Empty activation clause lists MUST be refused.
- Activation evaluation belongs to pipeline routing, not stage front matter.

### Resolved route truth

- The only supported reduced-v1 stage statuses are `active`, `skipped`, `blocked`, and `next`.
- Route reports MUST preserve declared pipeline order.
- Every non-`active` stage MUST carry an explicit machine-readable reason payload; downstream consumers MUST NOT infer why a stage is non-active from position alone.
- `skipped` means the stage is out of the current route because its activation clauses evaluated false.
- `blocked` means the stage would otherwise matter to the current route, but the route cannot safely proceed because required runtime state is missing, malformed, stale, or otherwise outside contract.
- `next` means the stage is the first actionable stage that would become active after the required route-state change is made. A route report MUST NOT mark more than one stage as `next`.
- Any later stage that depends on a currently `blocked` or `next` stage MUST stay non-active rather than being reported as independently active.

### Runtime-only route-state schema

- Route state lives only at `.system/state/pipeline/<pipeline-id>.yaml`.
- Route state is runtime-only and MUST NOT become canonical project truth.
- A missing state file MAY be treated as an empty runtime state during read-only resolution and MUST be created on the first successful mutation.
- When present, the state file MUST be a YAML mapping with exactly these top-level keys:
  - `schema_version`
  - `pipeline_id`
  - `revision`
  - `routing`
  - `refs`
  - `run`
  - `audit`
  - `route_basis`
- `schema_version` MUST be `m2-pipeline-state-v3` for newly written state.
- Read compatibility MAY accept legacy `m1-pipeline-state-v2` state that omits `route_basis`, but legacy schema writes are no longer the current shape.
- `pipeline_id` MUST match the canonical pipeline ID from the loaded pipeline definition.
- `revision` MUST be a monotonically increasing non-negative integer.
- `routing` MUST be a mapping from supported variable name to boolean value. Keys MUST follow the same variable-name grammar as activation clauses.
- `refs` MUST be a mapping with exactly these optional string fields:
  - `charter_ref`
  - `project_context_ref`
- `run` MUST be a mapping with exactly these optional string fields:
  - `runner`
  - `profile`
  - `repo_root`
- `refs.*` values, when present, MUST be non-empty repo-relative paths.
- `run.runner` and `run.profile`, when present, MUST match declared allowlisted IDs discovered under `runners/` and `profiles/`.
- `run.repo_root`, when present, MUST be a clean absolute path string naming the repo root bound to the successful mutation that last persisted the state file.
- `run.repo_root` is compiler-derived runtime state. In persisted route state it remains an absolute provenance path; in the published route-basis/compile-facing view it MUST be normalized to the stable symbolic root `${repo_root}`.
- `run.repo_root` is not a direct user-writable mutation field.
- `audit` MUST be a sequence of mutation records. Each record MUST contain exactly:
  - `revision`
  - `field_path`
  - `value`
- `audit.field_path` MUST be one of:
  - `routing.<variable-name>`
  - `refs.charter_ref`
  - `refs.project_context_ref`
  - `run.runner`
  - `run.profile`
- `audit.value` MUST be a boolean for `routing.*` entries and a string for `refs.*` / `run.*` entries.
- Derived runtime-state fields such as `run.repo_root` MUST NOT appear in `audit.field_path`; audit records track direct typed mutations only.
- `route_basis`, when present, MUST be a bounded compiler-owned snapshot written by `pipeline resolve`.
- `route_basis` MUST contain:
  - its own explicit schema/version field
  - the canonical `pipeline_id`
  - the repo-relative pipeline file path plus its content fingerprint
  - the state revision captured at resolve time
  - exact snapshots of `routing`, `refs`, and `run` as resolve used them
  - the ordered resolved route snapshot for all declared stages, including repo-relative stage file paths, persisted statuses/reasons, and stage file fingerprints
  - the selected runner id plus runner file path/fingerprint
  - the selected profile id plus fingerprints for `profile.yaml`, `commands.yaml`, and `conventions.md`
- Accepted or persisted `route_basis` snapshots MUST exactly match the selected pipeline's declared stage list/order and the canonical resolve result for the captured `routing` snapshot; consumers MUST refuse mismatches rather than best-effort continuing.
- `route_basis.run.repo_root`, when present, MUST use the stable symbolic root `${repo_root}`. Readers MAY accept legacy absolute values for compatibility, but compiler-owned comparisons and compile-facing output MUST canonicalize them to `${repo_root}`.
- `route_basis` MUST NOT contain compiled payload bytes, explain output bytes, copied include/library/artifact contents, duplicated audit history, compile-only overrides, or wall-clock timestamps.
- Unknown top-level keys, unknown nested keys, invalid routing variable names, invalid field paths, or wrong scalar types MUST be refused as malformed state.
- Audit history MUST be bounded to a fixed implementation-defined maximum entry count and MUST trim oldest-first after a successful mutation. The bound MUST remain stable within one implementation revision and be covered by tests.

### Mutation protocol

- Route-state mutation MUST be compiler-owned and typed. CLI surfaces may wrap it, but they MUST NOT redefine its concurrency or refusal semantics.
- Every mutation MUST validate the pipeline ID, schema version, routing-variable grammar, and field value type before attempting persistence.
- Routing mutations MUST target `routing.<variable-name>` and accept boolean values only.
- `refs.charter_ref` and `refs.project_context_ref` MUST accept repo-relative string values only.
- `run.runner` and `run.profile` MUST accept only declared allowlisted IDs discovered under `runners/` and `profiles/`.
- `run.repo_root` MUST NOT be accepted as a direct mutation field. Successful compiler-owned mutation persistence MUST derive and persist it from the bound repo root instead.
- `pipeline resolve` MAY derive a compile-facing route-basis copy of `run.repo_root`, but it MUST normalize that copy to `${repo_root}` instead of leaking the machine-local checkout path into downstream proof or payload surfaces.
- Every mutation MUST acquire an advisory lock before the read-modify-write sequence begins.
- Every mutation MUST compare an expected revision supplied by the caller with the persisted revision. On mismatch, the mutation MUST refuse rather than silently overwrite newer state.
- Successful writes MUST use write-then-rename atomic replacement within the same state directory.
- Silent last-write-wins behavior is forbidden.
- Mutation outcomes MUST distinguish success from refusal, and refusal outcomes MUST distinguish malformed-state refusal, unsupported-variable refusal, and revision-conflict refusal.
- `pipeline resolve` is the only compiler-owned write path allowed to persist `route_basis`.
- Persisting `route_basis` MUST NOT silently recompute route state from stale inputs; it must snapshot the already accepted route plus the exact `routing` / `refs` / `run` surfaces used during that resolve.

## Compatibility and Downstream Revalidation

- Any change to accepted pipeline shape, activation grammar, stage-status vocabulary, or runtime-state schema requires downstream revalidation.
- Any change to route-state mutation semantics, `route_basis` persistence rules, revision handling, or audit trimming behavior requires downstream revalidation.
- Any change to the runtime-only posture of `.system/state/**` requires downstream revalidation.
- `THR-01` publication requires this contract to name the downstream revalidation targets explicitly, and any later change to `C-08` must revalidate `SEAM-2`, `SEAM-3`, and `SEAM-4` before the thread can be treated as current.

## Verification Checklist

Existing loader evidence already lives in `crates/compiler/src/pipeline.rs` and `crates/compiler/tests/pipeline_loader.rs`. Landing this contract also requires the following implementation and verification work:

- Keep loader coverage passing for two-document parsing, repo-relative pipeline paths, `core/stages/*.md` boundaries, unique stage IDs, and supported activation clause parsing.
- Add `crates/compiler/tests/pipeline_route_resolution.rs` with:
  - `resolved_route_preserves_declared_stage_order`
  - `resolved_route_marks_false_activation_as_skipped`
  - `resolved_route_emits_single_next_stage_when_state_is_required`
  - `resolved_route_refuses_out_of_contract_activation_inputs`
- Add `crates/compiler/tests/pipeline_state_store.rs` with:
  - `state_store_round_trips_revisioned_routing_refs_and_run_fields`
  - `route_basis_round_trips_when_written_by_resolve`
  - `legacy_m1_state_without_route_basis_still_loads`
  - `malformed_route_basis_is_distinct_from_malformed_route_state`
  - `state_store_refuses_unknown_keys_and_wrong_scalar_types`
  - `state_store_refuses_revision_conflict_without_overwrite`
  - `state_store_trims_audit_history_oldest_first`
  - `state_store_uses_atomic_replace_under_lock`
  - `state_store_refuses_legacy_flat_schema_version`
- Record `THR-01` publication evidence that cites `C-08` and the downstream revalidation targets `SEAM-2`, `SEAM-3`, and `SEAM-4`.
- Pass criteria:
  - identical pipeline and state inputs produce identical ordered route results
  - malformed or out-of-contract state never downgrades into best-effort behavior
  - revision conflicts never overwrite newer state
  - audit trimming is bounded and deterministic
