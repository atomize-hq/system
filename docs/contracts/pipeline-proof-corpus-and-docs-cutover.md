---
contract_id: C-11
seam_id: SEAM-4
owner_seam: SEAM-4
version: m1-v1
currentness: current
status: drafted
revalidation_triggers:
  - Any change to the proof-corpus shape, fixture ownership, or shared golden outputs.
  - Any change to malformed pipeline or malformed route-state refusal classes, wording, or recovery guidance.
  - Any change to docs/help parity claims for the supported `pipeline` subset.
  - Any change to the M1 performance, security, or operability boundary for `pipeline`.
  - Any change to the upstream contracts consumed by this contract: `C-08`, `C-09`, or `C-10`.
---

# C-11 Pipeline Proof Corpus and Docs Cutover Contract

## Purpose

This contract defines the conformance baseline for the shipped M1 `pipeline` surface.

`C-11` exists so later milestone packs can rely on one explicit truth for:

- the shared proof corpus used by compiler and CLI conformance checks
- the golden outputs that pin route truth and refusal behavior
- the malformed pipeline and malformed route-state refusal classes that must stay explicit
- the docs/help parity boundary for the reviewed `pipeline` subset
- the M1 performance, security, and operability boundary around `pipeline`

`C-11` is owned by `SEAM-4` and is downstream-facing: it binds proof, docs, and safety rails to the upstream contracts it consumes without redefining route/state or operator-surface semantics.

## Canonical location

- Canonical artifact: `docs/contracts/pipeline-proof-corpus-and-docs-cutover.md`
- Canonical proof corpus root: `tests/fixtures/pipeline_proof_corpus/`
- Canonical M1 shared route-bearing corpus case: `tests/fixtures/pipeline_proof_corpus/foundation_inputs/`
- Canonical shared goldens for `pipeline resolve` / `pipeline state set`: `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/`
- Producing seam: `SEAM-4`

## Consumed contracts (inputs)

`C-11` consumes:

- [`C-08`](pipeline-route-and-state-core.md) compiler-owned route/state truth
- [`C-09`](pipeline-operator-surface-and-id-resolution.md) reviewed `pipeline` operator surface and id-resolution rules
- [`C-10`](stage-compile-boundary-and-route-freshness.md) compile-boundary and route-freshness handoff rules

## Owned surface

`C-11` is authoritative about:

- the required shared proof corpus for M1 `pipeline` conformance
- the golden-output and refusal-output surfaces that must be pinned by tests
- the docs/help parity baseline for the reviewed `pipeline` subset
- the explicit M1 performance, security, and operability boundary for `pipeline`
- the downstream revalidation triggers that later milestone packs must honor

`C-11` is not authoritative about:

- the route/state semantics owned by `C-08`
- the operator-surface semantics owned by `C-09`
- the compile-boundary semantics owned by `C-10`
- the concrete implementation of future compiler or CLI tests
- any future payload field names, materialization steps, or file-write details for later compile work

## Normative rules

### Shared proof corpus

- The M1 proof corpus MUST be a single shared repository-owned corpus used by both compiler and CLI conformance checks.
- The corpus MUST cover the foundation-family wedge that exercises the shipped `pipeline` story without widening into later milestone behavior.
- The corpus MUST be realistic enough to prove operator value, not just parser correctness.
- The corpus MUST be shared rather than duplicated across compiler and CLI test suites.
- The corpus MUST live outside `.system/` and outside runtime-state directories; it is proof data, not canonical project truth and not runtime truth.
- The corpus SHOULD be colocated with repo test fixtures so later slices can reference it from compiler and CLI suites without inventing separate fixture trees.
- The shipped M1 route-bearing corpus MUST live at `tests/fixtures/pipeline_proof_corpus/foundation_inputs/`.
- The shared repo fixture for that corpus MUST live under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/repo/`.
- Persisted malformed/revision-conflict seed state used by the corpus MUST live under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/state_seeds/`.

The proof corpus MUST include cases that exercise:

- pipeline loading and stage-order preservation
- supported activation evaluation
- route truth for active, skipped, blocked, and next stages
- shorthand ambiguity and unknown-id refusal behavior
- malformed pipeline refusal behavior
- malformed route-state refusal behavior
- lock/revision conflict behavior for narrow state mutation
- state mutation success and refusal distinctions

### Golden outputs

- Every corpus case that affects operator trust MUST have a deterministic golden output or snapshot expectation.
- Golden outputs MUST pin route ordering, stage status, refusal class, and recovery guidance where those are part of the contract.
- Golden outputs MUST be shared across compiler and CLI conformance checks when they describe the same contract surface.
- Golden changes MUST be reviewed as contract changes, not as cosmetic test updates.
- Golden outputs MUST remain deterministic for identical inputs and must not depend on filesystem traversal order, environment-specific paths, or non-deterministic timestamps.
- Shared route-bearing goldens MUST live under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/goldens/`.
- Path-bearing proof output MUST normalize temp-repo evidence into committed placeholder tokens before comparing against goldens.
- The M1 placeholder tokens are `{{REPO_ROOT}}` for the temp proof repo root and `{{STATE_PATH}}` for persisted route-state evidence paths.

### Malformed-refusal classes

The M1 `pipeline` surface MUST keep the following refusal classes explicit and distinct:

- malformed pipeline definition refusal
- malformed route state refusal
- unsupported variable refusal
- revision conflict refusal
- ambiguous selector refusal
- unknown selector refusal
- out-of-contract activation input refusal

These refusals MUST remain non-overlapping in user-facing and test-facing behavior. One refusal class MUST NOT silently collapse into another.

Malformed pipeline and malformed route-state refusals MUST stay explicit:

- malformed pipeline definitions MUST not auto-heal
- malformed route state MUST not silently repair or downgrade to best-effort behavior
- revision conflicts MUST not overwrite newer state
- unsupported variable writes MUST not mutate state

### Docs/help parity

- The supported `pipeline` subset MUST be described consistently across root docs, support-story docs, contract docs, and help snapshots.
- Docs and help MUST present `pipeline` as the reviewed operator surface for `list`, `show`, `resolve`, and `state set`.
- Docs and help MUST keep `pipeline compile` out of the shipped M1 supported surface.
- Docs and help MUST not imply that proof-corpus checks, docs/help cutover, or safety rails are optional once the M1 `pipeline` surface is presented as supported.
- Docs and help MUST not diverge from the reviewed operator-surface language in `C-09`.
- Docs and help MUST keep the compile boundary deferred as future work consistent with `C-10`.

The docs/help parity boundary for `C-11` is:

- root support-story docs that describe the supported `pipeline` subset
- CLI help snapshots that pin the surfaced command ordering and wording
- contract docs that bind the proof corpus and safety rails to upstream truth

### Performance, security, and operability boundary

- M1 `pipeline` conformance MUST remain repo-local and deterministic.
- The `pipeline` proof and docs surfaces MUST not require network access, external services, or ambient machine state beyond the repository and its managed runtime state.
- `pipeline list` and `pipeline show` MUST remain metadata-first surfaces.
- `pipeline resolve` MUST remain the first command allowed to load route-bearing state.
- `pipeline state set` MUST remain narrow, typed, and auditable.
- The contract MUST not smuggle caching, background refresh, or silent repair into M1.
- The contract MUST keep raw file paths as evidence only; they MUST not become a new primary selection surface.
- The contract MUST keep the shipped help/docs posture from implying broader execution or compile behavior than the upstream contracts allow.

### Downstream revalidation

Later milestone packs MUST treat `C-11` as a stale-trigger source for any change to:

- proof-corpus shape or fixture ownership
- golden outputs or snapshot expectations
- malformed pipeline or malformed route-state refusal behavior
- docs/help claims for the supported `pipeline` subset
- command hierarchy wording that changes the operator story
- the M1 performance, security, or operability boundary

Any change to `C-08`, `C-09`, or `C-10` that affects route truth, operator-surface wording, or the compile boundary MUST revalidate `C-11` before downstream consumers treat it as current.

## Verification checklist

The following checklist is normative for seam-local execution and closeout:

- [ ] The shared proof corpus exists as one repository-owned corpus used by both compiler and CLI conformance checks.
- [ ] The shared M1 route-bearing corpus lives under `tests/fixtures/pipeline_proof_corpus/foundation_inputs/` with `repo/`, `state_seeds/`, and `goldens/` subtrees.
- [ ] The corpus covers pipeline loading, activation, ordered route truth, shorthand ambiguity, malformed pipeline refusal, malformed route-state refusal, revision conflicts, and mutation semantics.
- [ ] Golden outputs or snapshots pin the same shared corpus across compiler and CLI surfaces.
- [ ] Path-bearing proof output is normalized to committed placeholder tokens before golden comparison.
- [ ] Malformed pipeline refusal and malformed route-state refusal remain explicit and distinct from other refusal classes.
- [ ] Docs and help snapshots describe `pipeline` as `list`, `show`, `resolve`, and `state set` only.
- [ ] Docs and help do not imply `pipeline compile` is shipped in M1.
- [ ] The contract stays aligned with `C-08`, `C-09`, and `C-10` without redefining their semantics.
- [ ] The M1 performance, security, and operability boundary remains repo-local, deterministic, and free of silent repair.
