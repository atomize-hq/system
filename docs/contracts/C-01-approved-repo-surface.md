---
contract_id: C-01
seam_id: SEAM-1
owner_seam: SEAM-1
version: reduced-v1
currentness: current
status: published
revalidation_triggers:
  - Any change to supported-vs-legacy wording in README.md, PLAN.md, docs/README.md, or this contract.
  - Any change to legacy-harness freeze policy or the archive/runtime boundary.
  - Any move of the legacy harness into archived/ or any supported-path reference to archived/.
---

# C-01 Approved Repo-Surface Contract

## Purpose

This contract defines the reduced-v1 repo-surface truth for `SEAM-1`. It is the source of truth for what is supported, what is legacy reference material, and what `archived/` may mean if and when it exists.

## Approved Repo Surface

- Top-level repo surface map:
  - `README.md`: approved
  - `PLAN.md`: approved
  - `docs/`: approved
  - `core/`: approved
  - `profiles/`: approved
  - `runners/`: approved
  - `pipelines/`: approved
  - `pipeline.yaml`: approved
  - `tools/`: legacy/transitional
  - `dist/`: generated
  - `artifacts/`: generated
  - `archived/`: archived
- The repo-facing truth surfaces for supported-vs-legacy decisions are: `README.md`, `PLAN.md`, `docs/README.md`, and this contract at `docs/contracts/C-01-approved-repo-surface.md`.
- Any future repo-facing doc that speaks for reduced-v1 MUST cite `C-01` and use the same nouns and verbs defined here.
- Files outside that surface MAY exist as implementation or reference material, but they MUST NOT contradict this contract.

## Supported Workflow During Transition

- Rust-first is the supported product direction.
- Until Rust setup exists, the guided setup flow MAY still be powered by the legacy harness if that is required to establish canonical artifacts.
- After setup artifacts exist, the Rust CLI becomes the supported packet-resolution authority.
- Docs and help text MUST NOT imply that the Rust CLI already exists before it does.
- Docs and help text MUST continue to distinguish `supported product direction` from `legacy reference material only` during the transition.

## Supported-vs-Legacy Wording

- `supported product path` means the Rust-first reduced-v1 direction.
- `legacy reference material` means the current Python harness, its helper docs, and related scaffold that remain in place for reference until cutover.
- `supported` MUST NOT be used to describe the current Python harness.
- `legacy` MUST NOT be used to imply that unsupported behavior is acceptable as the long-term product path.
- Any doc that describes the current Python harness MUST label it `legacy reference material only` or equivalent wording that cannot be mistaken for supported runtime.

## Legacy Harness Freeze Policy

- The legacy Python harness MAY receive bug fixes, link corrections, and narrow doc wording fixes needed to keep it runnable as reference material.
- The legacy Python harness MUST NOT gain new supported product features, new CLI surface, or messaging that implies it is the supported runtime path.
- The legacy Python harness MUST remain in place until cutover work explicitly moves it to `archived/` or removes it.
- Any move to `archived/` is a cutover action, not a reduced-v1 feature increment.

## Archive/Runtime Boundary

- `archived/` is reference-only.
- Supported-path code is the supported runtime implementation once Rust exists (example: future `crates/`, future `Cargo.toml`, and future Rust entrypoints).
- Supported-path docs are the approved repo docs that describe the supported product direction (example: `README.md`, `PLAN.md`, `docs/README.md`, and this contract).
- `tools/` and `archived/` are never runtime dependencies for supported-path code.
- Supported-path code and supported-path docs MUST NOT import, execute, or wrap anything under `archived/`.
- Supported-path docs MAY link to archived content for reference, but they MUST clearly mark it non-runtime and non-supported.
- Any future supported runtime surface that needs archived material MUST first move that dependency outside `archived/` or keep it as reference-only.

## Verification Checklist

- [ ] `README.md`, `PLAN.md`, and `docs/README.md` each link to `docs/contracts/C-01-approved-repo-surface.md`.
- [ ] Each of those docs uses the same supported-vs-legacy nouns as this contract.
- [ ] No supported-path doc says the current Python harness is the supported runtime path.
- [ ] No supported-path doc says anything under `archived/` is executable, importable, or wrapped.
- [ ] `./tools/harness.sh --help` emits a legacy-only banner and a contract pointer.
- [ ] `python3 tools/check_archive_boundary.py` exits `0` when no `crates/` tree exists yet.
- [ ] `python3 tools/check_archive_boundary.py --self-test` proves the guardrail rejects a supported-path fixture that references `archived/`.
- [ ] A cold reader can identify the supported path, the legacy-only status, and the archive boundary in under 30 seconds.
- [ ] Downstream seams `SEAM-2` and `SEAM-7` can revalidate against this contract without needing hidden context.
