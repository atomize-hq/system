---
contract_id: C-09
seam_id: SEAM-2
owner_seam: SEAM-2
version: m1-v1
currentness: current
status: drafted
revalidation_triggers:
  - Any change to the supported `pipeline` subcommands in the reviewed M1 wedge.
  - Any change to canonical-id lookup, shorthand ambiguity handling, or unknown-id refusal semantics.
  - Any change to the rule that raw file paths are evidence rather than first-class operator input.
  - Any change to normalized operator render wording for `pipeline list`, `show`, `resolve`, or `state set`.
  - Any change to help-surface posture or the rule that the reviewed `pipeline` family is not treated as shipped until later slices land.
---

# Pipeline Operator Surface and ID Resolution Contract

## Purpose

This contract defines the reviewed M1 operator surface for `pipeline` and the ID-resolution rules that support it.

It exists so the operator-facing `pipeline` family can be treated as one explicit contract baseline for `list`, `show`, `resolve`, and `state set` without reinterpreting compiler truth in CLI-only code paths.

## Canonical Location

- Canonical artifact: `docs/contracts/pipeline-operator-surface-and-id-resolution.md`
- Downstream consumers: later pipeline seams and docs/help conformance rails

## Owned Surface

This contract is authoritative for:

- the reviewed `pipeline` subcommands in the M1 wedge:
  - `pipeline list`
  - `pipeline show`
  - `pipeline resolve`
  - `pipeline state set`
- canonical pipeline and stage ids
- shorthand lookup rules for canonical ids
- ambiguity and unknown-id refusal classes
- normalized operator render expectations for the reviewed `pipeline` surfaces
- the rule that raw file paths are evidence, not first-class operator input
- the compiler-boundary rule that CLI code wraps compiler-owned declared pipeline, route, and state data instead of redefining those semantics

This contract is not authoritative for:

- CLI parsing internals
- route computation or state mutation semantics owned by `docs/contracts/pipeline-route-and-state-core.md`
- compile payload generation
- later-slice command behavior
- proof-corpus shape or conformance rails owned by later seams

## Normative Rules

### Supported M1 command family

- The reviewed `pipeline` surface MUST consist of `list`, `show`, `resolve`, and `state set`.
- `pipeline compile` MUST remain out of this contract baseline and out of the reviewed M1 help surface.
- The reviewed `pipeline` family is contractually defined now, but it is not considered shipped until the later slices that publish command handlers, help evidence, and proof-facing alignment.

### Compiler boundary

- `pipeline list` and `pipeline show` MUST consume compiler-owned declared pipeline data.
- CLI glue for `pipeline list` and `pipeline show` MUST NOT reparse YAML as an independent source of truth or redefine declared-pipeline semantics.
- The compiler-owned metadata path for `pipeline list` and `pipeline show` MUST remain metadata-first and MUST NOT fail because of unrelated malformed pipeline or stage files elsewhere in the repo.
- `pipeline show` MUST still surface an explicit catalog refusal when the selected pipeline's own declared metadata cannot be loaded.
- `pipeline resolve` and `pipeline state set` MUST wrap the compiler-owned route/state surfaces defined by `docs/contracts/pipeline-route-and-state-core.md`.
- CLI glue for `pipeline resolve` and `pipeline state set` MUST NOT reinterpret route statuses, state schema, mutation protocol, or refusal semantics owned by `docs/contracts/pipeline-route-and-state-core.md`.
- The reviewed operator surface MAY render or adapt compiler-owned truth, but it MUST NOT become a second authority for declared pipeline structure or route/state semantics.
- Later compile-boundary work MUST reuse these canonical-id and shorthand rules as-is and MUST keep raw file paths evidence only rather than introducing a parallel selector contract.

### Canonical ids

- Canonical ids are the source of truth for operator selection.
- The command surface MUST accept canonical pipeline ids and canonical stage ids where a command expects an id.
- Canonical ids MUST remain stable, explicit, and auditable.

### Shorthand lookup

- Shorthand lookup MAY strip the `pipeline.` or `stage.` prefix only when the resulting shorthand is unambiguous.
- For metadata-only inventory surfaces, selector resolution MUST operate on successfully indexed metadata entries rather than unrelated malformed files that were excluded from the metadata index.
- If shorthand lookup is unambiguous, the CLI MAY resolve it to the matching canonical id.
- If shorthand lookup is ambiguous, the CLI MUST refuse and MUST:
  - say that overlapping ids were found
  - list the conflicting canonical ids
  - instruct the operator to use the full canonical id or rename the conflicting ids
- Ambiguity refusal and unknown-id refusal MUST remain distinct refusal classes.
- Ambiguity refusal MUST direct the operator to use the full canonical id or rename the conflicting ids.
- Unknown-id refusal MUST direct the operator to use a matching canonical id or inspect the available reviewed pipeline inventory.
- If no matching id exists, the CLI MUST refuse with an explicit unknown-id response and recovery guidance.

### Raw file paths

- Raw file paths MUST remain evidence only.
- Raw file paths MUST NOT become a first-class operator input for the reviewed `pipeline` surface.
- `list` may expose backing repo-relative file paths as evidence, but those paths MUST NOT redefine the selection contract.

### Normalized render posture

- `pipeline list` MUST render the available pipeline identities in a deterministic, normalized form.
- `pipeline show` MUST render a normalized typed view of declared pipeline configuration, including canonical identity and backing-file provenance.
- `pipeline resolve` MUST render a compact ordered route report whose output is aligned with the reviewed route-truth contract.
- `pipeline state set` MUST use a compact mutation-oriented surface that clearly distinguishes success from refusal.
- Any operator-facing wording used by these commands MUST remain consistent with the reviewed help and docs surface.

### Help posture

- The reviewed `pipeline` family MUST be presented as a supported surface only when its code, help text, docs, contracts, tests, and proof-corpus gates land together.
- Help and docs MUST not overclaim `pipeline` as already shipped before the later slices land.
- Help and docs MUST not expose `pipeline compile` as part of the reviewed M1 supported surface.

## Compatibility and Downstream Revalidation

- Any change to supported `pipeline` subcommands, id-resolution semantics, or help posture requires downstream revalidation.
- Any change to the reviewed `pipeline` wording or command ordering that would alter operator expectations requires contract revision.
- Any change that makes raw file paths a primary operator input requires contract revision.

## Verification Checklist

- [ ] `crates/cli/src/main.rs` treats `pipeline list` and `pipeline show` as thin wrappers over compiler-owned declared pipeline data.
- [ ] `crates/cli/src/main.rs` treats `pipeline resolve` and `pipeline state set` as thin wrappers over compiler-owned route/state surfaces from `docs/contracts/pipeline-route-and-state-core.md`.
- [ ] `crates/cli/tests/cli_surface.rs` covers the reviewed `pipeline` operator surface without collapsing ambiguity refusal into unknown-id refusal.
- [ ] `crates/cli/tests/help_drift_guard.rs` pins the reviewed surface wording and the unsupported-vs-shipped posture.
- [ ] `docs/SUPPORTED_COMMANDS.md` names the reviewed `pipeline` baseline without claiming shipped runtime support.
- [ ] Help snapshot surfaces under `crates/cli/tests/snapshots/` stay aligned with the reviewed contract wording.
- [ ] Canonical ids are first-class and shorthand is explicitly constrained to unambiguous cases.
- [ ] Ambiguity refusal and unknown-id refusal are distinct classes with distinct recovery guidance.
- [ ] Raw file paths remain evidence only.
- [ ] The contract does not claim `pipeline` is shipped before later slices land.
- [ ] `pipeline compile` is out of the reviewed M1 contract baseline.
