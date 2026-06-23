# Handbook Rename Cutover Plan

## Purpose

This document is the canonical plan for renaming the active product and repo contract from `system` to `handbook`.

The target is a coherent cutover across the supported code, packaging, fixtures, generated assets, and current docs in this repository. The cutover must not modify archived material under `archived/**` or frozen legacy docs/code under `docs/legacy/**` unless a live test or build surface explicitly depends on them. Generated/build output under `target/**` and stale worktree output under `.worktrees/**` are not edit targets.

## Outcome

Land one atomic rename so the supported product surface consistently uses `handbook` instead of `system`.

Success means all of the following are true at once:

- The shipped CLI binary name is `handbook`.
- Rust package/library references use the `handbook` namespace instead of `system`.
- Canonical repo-local truth lives under `.handbook/**` instead of `.system/**`.
- Installed home and packaging contract use `~/handbook/` instead of `~/system/`.
- Generated skill/discovery surfaces use `handbook` and `handbook-charter-intake`.
- Current docs, help text, tests, fixtures, release surfaces, and GitHub raw URLs teach `handbook` as the active product name.
- No supported surface still instructs operators to use `system`, `.system/`, `~/system`, `SYSTEM_*`, `system-cli`, `system-compiler`, or `system-charter-intake`, except where a historical mention is explicitly marked as historical-only.

## Verification Surface

The cutover is complete only if the integrated tree satisfies all of these checks:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `bash tools/ci/install-smoke.sh`
- `bash tools/ci/author-live-smoke.sh` if required secrets are available; otherwise record that this rail was not runnable and why
- Grep sweeps on supported surfaces, excluding `archived/**`, `docs/legacy/**`, `target/**`, `.worktrees/**`, and `.git/**`, confirm no stale active references remain for:
  - `\bsystem\b`
  - `.system/`
  - `~/system`
  - `SYSTEM_`
  - `system-cli`
  - `system-compiler`
  - `system-charter-intake`
  - `atomize-hq/system`

## Constraints

- Do not edit anything under `archived/**`.
- Do not broaden into unrelated cleanup or refactors.
- Do not treat ordinary English uses of the noun "system" as rename targets unless they clearly refer to the product, binary, namespace, path, skill, package, env var, or release surface.
- Do not hand-edit generated projections if the source template or generator is the true authority.
- Keep the cutover atomic: do not ship a mixed `system`/`handbook` active contract.
- Do not introduce long-lived compatibility aliases unless a hard blocker is discovered and documented.

## Boundaries

### In scope

- Rust workspace packages, binary names, library names, module references, help text, diagnostics, tests, and snapshots
- Repo-local canonical truth and state root rename from `.system/**` to `.handbook/**`
- Install/home packaging and release scripts
- Generated skill sources and generated projections
- Current supported docs and contracts in repo root and `docs/**`, excluding `docs/legacy/**`
- Current fixtures and proof corpus under `tests/**`
- CI workflows, smoke rails, env vars, and release metadata
- GitHub raw/release URL references that point at the live repo slug

### Out of scope

- `archived/**`
- `docs/legacy/**`, unless a live supported rail breaks without a narrow compatibility note
- `target/**`
- stale `.worktrees/**` output
- unrelated product changes not required for the rename

## Canonical Rename Contract

Unless a live dependency proves otherwise, the target mapping is:

| Current | Target |
| --- | --- |
| `system` binary | `handbook` |
| `system-cli` | `handbook-cli` |
| `system-compiler` | `handbook-compiler` |
| `system_compiler` | `handbook_compiler` |
| `.system/**` | `.handbook/**` |
| `~/system/` | `~/handbook/` |
| `install/system-home/` | `install/handbook-home/` |
| `scripts/system/install.sh` | `scripts/handbook/install.sh` |
| `tools/release/package-system-home.sh` | `tools/release/package-handbook-home.sh` |
| `system` skill | `handbook` skill |
| `system-charter-intake` | `handbook-charter-intake` |
| `SYSTEM_*` env vars | `HANDBOOK_*` env vars |
| `atomize-hq/system` | `atomize-hq/handbook` |

If any row cannot be applied directly, the implementing session must document the blocker and the smallest safe alternative before proceeding.

## Workstreams

### Workstream A: Runtime and source rename

Update the live Rust product surface so the active binary/package/library names and operator-facing command/help text use `handbook`.

Owned surface includes:

- `crates/cli/**`
- `crates/compiler/**`
- root `Cargo.toml`
- command/help strings
- diagnostics and refusal text
- hard-coded path/env-var constants

Acceptance criteria:

- `cargo run -p handbook-cli -- --help` is the new supported help surface, or the equivalent workspace command if package naming changes require a different invocation
- the built binary reports `handbook`
- no active Rust code or tests depend on `system` names except explicitly historical-only assertions

### Workstream B: Canonical truth, fixtures, and proof data

Rename the repo-local truth contract and aligned fixtures from `.system/**` to `.handbook/**`.

Owned surface includes:

- repo root `.system/**` to `.handbook/**`
- fixture repos under `tests/fixtures/**`
- expected output and evidence files that refer to the canonical truth root

Acceptance criteria:

- supported commands and tests read/write `.handbook/**`
- fixture-backed proof rails no longer assert `.system/**` as active truth
- the repo has no active supported instructions pointing operators at `.system/**`

### Workstream C: Packaging, install home, skills, and release surfaces

Rename the installed-home and distribution contract from `system` to `handbook`.

Owned surface includes:

- `install/system-home/**`
- `.agents/skills/system/**`
- `.agents/skills/system-charter-intake/**`
- `tools/codex/*.sh`
- `scripts/system/install.sh`
- `tools/release/package-system-home.sh`
- `.github/workflows/**`
- `.github/release-template.md`

Acceptance criteria:

- generated skill source authority is updated and projections regenerate from the new source
- install smoke validates `~/handbook/`, `~/handbook/bin/handbook`, and `handbook*` discovery surfaces
- release asset names, tarball layout, and URLs use `handbook`

### Workstream D: Current docs and contracts

Update the supported documentation set so it teaches one active `handbook` contract.

Owned surface includes:

- root docs such as `README.md`, `DESIGN.md`, `PLAN.md`, `ORCH_PLAN.md`, `TODOS.md`, and `CHANGELOG.md`
- supported docs under `docs/**`, excluding `docs/legacy/**`
- current contract docs under `docs/contracts/**`

Acceptance criteria:

- current docs consistently describe `handbook` as the product
- any remaining `system` wording is clearly historical-only or domain-generic, not active product instruction
- no supported doc sends the operator to stale paths, stale binary names, or stale GitHub URLs

## Merge Strategy

Use one parent cutover branch and keep the rename integrated before landing. The preferred order is:

1. Runtime/source rename
2. Canonical truth and fixtures
3. Packaging/install/release surfaces
4. Current docs/contracts
5. Regenerate generated assets from source authority
6. Run the full validation bar
7. Land the integrated rename

This order can be adjusted if a discovered dependency forces it, but the end state must still land atomically.

## Iteration Policy

The implementing session should work in evidence-backed loops:

1. Inspect one bounded rename surface.
2. Make the smallest coherent change for that surface.
3. Run the narrowest relevant verifier.
4. Record what changed, what passed or failed, and the next best action.
5. Expand only when the current surface is coherent.

If generated assets drift from source templates, fix the source authority first, regenerate, and then re-run the narrow verifier before moving on.

## Blocked Stop Condition

Stop and report blocked only if one of these is true:

- a required rename surface cannot be updated without editing excluded legacy or archived material in a way that changes the promised scope
- a live verifier proves that a compatibility alias is temporarily required to keep the supported product operational
- the repo slug, external release location, or another external dependency must change outside the local repo before the local cutover can be validated
- a repeated failure remains after exhausting the defensible local paths and the next step genuinely requires user input or an external state change

A blocked report must include:

- what was attempted
- what evidence was gathered
- the exact blocker
- whether the blocker is local or external
- the smallest next input or state change needed to continue

## Final Acceptance Checklist

- [ ] Active code, tests, and snapshots use `handbook` instead of `system`
- [ ] Canonical repo-local truth uses `.handbook/**`
- [ ] Installed home and packaging use `~/handbook/`
- [ ] Generated skill/discovery surfaces use `handbook` naming
- [ ] Current supported docs/contracts teach `handbook` as the active product
- [ ] Archived and frozen legacy surfaces were not edited
- [ ] Required validation commands passed, or any unavailable rail is explicitly documented with reason
- [ ] Supported-surface grep sweeps show no stale active `system` references
- [ ] Any remaining `system` wording is explicitly historical-only or legitimate domain prose rather than active product naming

## Fresh-Session Goal Contract

The fresh session should treat this file as the single rename authority and use a Goal that includes:

- Outcome: complete the active `system` -> `handbook` cutover
- Verification: the validation bar and stale-reference grep sweeps in this document
- Constraints: no archived edits, no legacy churn, no unrelated cleanup
- Boundaries: supported code/docs/tests/packaging only
- Iteration policy: evidence-backed loop with narrow verifiers
- Blocked stop condition: report exact blocker and next input needed
