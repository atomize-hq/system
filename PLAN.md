# PLAN

## Status

This is the fresh implementation plan for `M10` on branch `feat/m8`.

`M9.5` already shipped. `M10` is not a behavior rewrite and not a packaging side quest. It is the source-of-truth correction that makes `system` match the `gstack` style of Codex skill packaging, with the intentional `system` naming tweak for user-home install/runtime paths.

## Checkpoint Resume

Recovered from `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/checkpoints/20260429-142427-m95-codex-packaging-landed-m10-gstack-parity-gap.md`.

Locked facts from that checkpoint:

- `M9.5` landed and passed verification.
- The install and discovery surface already works.
- The remaining gap is canonical packaging authorship, not operator-facing runtime behavior.
- `gstack` parity means repo-owned `SKILL.md.tmpl` sources, generated host projections, and a real root host skill pattern.
- The root host skill is in scope for `system`.

## Locked Direction

These points are not up for reinterpretation inside this plan:

1. Match the `gstack` type of install surface and source layout, not an approximation invented by review feedback.
2. Keep the root host skill pattern.
3. Keep the discoverable leaf skill `system-charter-intake`.
4. Keep the installed family name `system`, not a repo-name alias.
5. Keep user-home runtime naming under `system`, for example `~/.codex/skills/system`, `~/.local/state/system`, and related existing `system`-named paths.
6. Preserve the shipped `M9.5` runtime contract unless a change is required to make the packaging surface truly match the target shape.

## Objective

Move Codex packaging authorship out of `tools/codex/` and into a repo-owned canonical skill layout that mirrors `gstack`:

- repo root owns the root host skill source
- leaf skill directory owns the discoverable skill source
- authored content lives in `SKILL.md.tmpl`
- generated host outputs live under `.agents/skills/`
- installed outputs live under `~/.codex/skills/`
- runtime payload membership is driven by one explicit manifest

The result should be boring. One authored source layout. One generator. One generated repo-local projection. One installed projection.

## Exact Target Shape

### Canonical authored source

```text
SKILL.md.tmpl
runtime-manifest.json.tmpl
runtime-assets.json
charter-intake/
  SKILL.md.tmpl
```

Rules:

- Root `SKILL.md.tmpl` is the canonical authored source for the root host skill.
- `charter-intake/SKILL.md.tmpl` is the canonical authored source for the discoverable leaf skill.
- `runtime-assets.json` is the only authored list of non-template runtime payloads copied into the root runtime.
- `bin/` is runtime payload territory. It is expected to contain the packaged `system` binary in generated and installed runtime roots, not template files in authored source.
- Runtime payload content that already belongs in `core/library/**` stays there. `M10` does not duplicate that authored content into a second handwritten tree.

### Generated repo-local projection

```text
.agents/skills/system/
  SKILL.md
  runtime-manifest.json
  bin/system
  share/authoring/charter_authoring_method.md
  share/charter/CHARTER_INPUTS.yaml.tmpl
  share/charter/charter_inputs_directive.md

.agents/skills/system-charter-intake/
  SKILL.md
```

### Installed user-home projection

```text
~/.codex/skills/system/
~/.codex/skills/system-charter-intake/
```

### Runtime naming tweak, intentionally preserved

This plan keeps the user-home family name as `system`, not a repo-name alias:

- `~/.codex/skills/system`
- `~/.codex/skills/system-charter-intake`
- `~/.local/state/system/intake/runs/`

That is the intentional deviation from blindly copying `gstack` names. It is not a different surface model.

## Scope Lock

### In scope

- add repo-root `SKILL.md.tmpl` for the root host skill
- add repo-root `runtime-manifest.json.tmpl`
- add repo-root `runtime-assets.json`
- add leaf `charter-intake/SKILL.md.tmpl`
- update `tools/codex/generate.sh` to project from those authored sources
- keep `.agents/skills/system` and `.agents/skills/system-charter-intake` as generated outputs
- make the root runtime `bin/` contain `system`, not a templated wrapper
- keep `tools/codex/install.sh`, `tools/codex/dev-setup.sh`, and `tools/codex/relink.sh` as the install/dev entrypoints
- update smoke tests and docs so they describe the new source-of-truth model exactly
- ensure generated `SKILL.md` outputs are host-valid and match the `gstack` style shape

### NOT in scope

- changing `system doctor --json`
- changing `system author charter --validate --from-inputs`
- changing `system author charter --from-inputs`
- adding more discoverable skills
- adding `.claude/skills/` generation
- adding a generic multi-host packaging framework
- changing the repo-local `.agents/skills/` override precedence model
- adding stale-override invalidation logic that does not exist in the target `gstack` style surface
- public release packaging or registry distribution

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing code or asset | Reuse decision |
| --- | --- | --- |
| generator entrypoint | `tools/codex/generate.sh` | keep, but make it projection-only |
| install flow | `tools/codex/install.sh` | keep |
| dev symlink flow | `tools/codex/dev-setup.sh`, `tools/codex/relink.sh` | keep |
| generated output names | `.agents/skills/system`, `.agents/skills/system-charter-intake` | keep |
| installed output names | `~/.codex/skills/system`, `~/.codex/skills/system-charter-intake` | keep |
| runtime payload files | `core/library/authoring/charter_authoring_method.md`, `core/library/charter/**` | keep as content sources, copy via manifest |
| runtime binary contract | shipped `system` CLI binary | package it under root-runtime `bin/system`, not a templated wrapper |
| conformance rails | `tools/ci/install-smoke.sh`, `tools/ci/codex-skill-live-smoke.sh`, `docs/contracts/C-07-conformance-rails-and-docs-cutover.md` | extend for new source model |

### Minimum change set

The minimum honest implementation is:

1. introduce the repo-root and leaf authored sources
2. rewrite generation to consume them
3. keep generated and installed output names stable
4. prove the new source model through smoke tests and docs

Anything less leaves `tools/codex/` as the real authored home. That would miss the whole point.

### Complexity check

This plan touches more than 8 files, but it does not introduce new services, crates, or infrastructure. That is acceptable. The blast radius is packaging scripts, generated assets, smoke tests, and docs/contracts only.

### Completeness check

The complete version is cheap here. We are already in the packaging layer. Doing only half the source move would save almost no effort and create a second cleanup milestone immediately after this one.

## Architecture Review

### Source ownership

The root host skill must become a real authored source, not just a generated runtime bucket.

```text
repo authored sources
    |
    +-- SKILL.md.tmpl
    +-- runtime-manifest.json.tmpl
    +-- runtime-assets.json
    +-- charter-intake/SKILL.md.tmpl
    |
    v
tools/codex/generate.sh
    |
    +-- .agents/skills/system/
    +-- .agents/skills/system-charter-intake/
    |
    v
tools/codex/install.sh
    |
    v
~/.codex/skills/system/
~/.codex/skills/system-charter-intake/
```

This is the whole game for `M10`.

### Projection rules

Projection must be deterministic and mechanical:

- `SKILL.md.tmpl` -> `.agents/skills/system/SKILL.md`
- `runtime-manifest.json.tmpl` -> `.agents/skills/system/runtime-manifest.json`
- packaged `system` binary -> `.agents/skills/system/bin/system`
- `runtime-assets.json` -> copy listed `core/library/**` assets into `.agents/skills/system/share/**`
- `charter-intake/SKILL.md.tmpl` -> `.agents/skills/system-charter-intake/SKILL.md`

Required rule:

- the generator may not contain hidden asset membership truth inline once `runtime-assets.json` exists
- the runtime `bin/` directory must contain the `system` binary, never handwritten template files

### Install behavior

Install behavior stays the shipped `M9.5` story:

- normal install copies generated outputs into `~/.codex/skills/`
- dev setup symlinks generated outputs into `~/.codex/skills/`
- normal install after dev setup replaces symlinks with copied directories

No new install surface is introduced. No repo clone is copied into the home directory.

### Root host skill rationale

The root host skill stays because it matches the actual `gstack` pattern and gives `system` one obvious authored home for:

- root skill metadata
- runtime manifest template
- runtime binary packaging contract
- runtime payload membership

Without that root source, the runtime root would remain a generated implementation detail with no clean authored owner. Not great.

## Code Quality Review

### Required design rules

- `tools/codex/` becomes tooling only, not authored skill content
- generated `.agents/skills/**` remains ignored build output
- no duplicated handwritten copies of `core/library/**` payload files
- no generic host abstraction layer
- no new crate
- no new command surface

### Directory hygiene

After `M10`, a contributor should be able to answer these questions in under 30 seconds:

1. Where is the authored root skill source?
2. Where is the authored leaf skill source?
3. Where is the authoritative runtime payload list?
4. What script turns authored sources into generated projections?

If any answer still starts with "well, part of it is in `tools/codex/` and part of it is...", the plan failed.

## Implementation Plan

### Step 1: Introduce canonical authored sources

- add repo-root `SKILL.md.tmpl`
- add repo-root `runtime-manifest.json.tmpl`
- add repo-root `runtime-assets.json`
- add `charter-intake/SKILL.md.tmpl`

Authorship rules:

- the two `SKILL.md.tmpl` files are the only handwritten skill documents
- both generated `SKILL.md` outputs must include the host-valid frontmatter and generated marker comments expected by the current Codex surface
- no template files belong under runtime `bin/`

### Step 2: Rewire generation

Update `tools/codex/generate.sh` so it:

- renders the root and leaf `SKILL.md.tmpl` sources
- renders the runtime manifest from the new repo-root template
- materializes `bin/system` into the root runtime from the built `system` binary, not a template
- copies runtime payload assets by iterating `runtime-assets.json`
- regenerates `.agents/skills/system/**` and `.agents/skills/system-charter-intake/**`

### Step 3: Keep install/dev entrypoints stable

Keep the external commands unchanged:

- `bash tools/codex/generate.sh`
- `bash tools/codex/install.sh`
- `bash tools/codex/dev-setup.sh`
- `bash tools/codex/relink.sh`

Only their source inputs change.

### Step 4: Update generated outputs and expectations

- regenerate `.agents/skills/system/**`
- regenerate `.agents/skills/system-charter-intake/**`
- update any tests or snapshots that assert generated content
- keep the runtime root file set exactly aligned with the conformance contract

### Step 5: Update smoke rails and docs

Update:

- `tools/ci/install-smoke.sh`
- `tools/ci/codex-skill-live-smoke.sh`
- `README.md`
- `DESIGN.md`
- `docs/SUPPORTED_COMMANDS.md`
- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`

Doc story after `M10`:

- canonical Codex packaging authorship lives at repo root plus `charter-intake/`
- `tools/codex/` is projection/install tooling
- `.agents/skills/**` is generated output
- `~/.codex/skills/**` is installed output

## Test Review

### Test framework and proof surface

This repo already has the right proof layers:

- Rust workspace tests through `cargo test --workspace`
- packaging smoke via `tools/ci/install-smoke.sh`
- live skill smoke via `tools/ci/codex-skill-live-smoke.sh`

No new test framework is needed.

### Coverage diagram

```text
CODE PATH COVERAGE
==================
[+] canonical authored sources
    |
    ├-- [GAP] root SKILL.md.tmpl renders .agents/skills/system/SKILL.md
    ├-- [GAP] leaf charter-intake/SKILL.md.tmpl renders .agents/skills/system-charter-intake/SKILL.md
    ├-- [GAP] runtime-manifest.json.tmpl renders .agents/skills/system/runtime-manifest.json
    └-- [GAP] runtime-assets.json fully defines copied share/** payload membership

[+] generation flow
    |
    ├-- [TESTED] bash tools/codex/generate.sh regenerates both output roots
    ├-- [GAP] root runtime bin contains system binary, not a wrapper template
    ├-- [GAP] generated SKILL.md files are host-valid and include required frontmatter
    └-- [GAP] generated runtime root file set matches contract exactly

[+] install flow
    |
    ├-- [TESTED] bash tools/codex/install.sh installs both roots
    ├-- [TESTED] bash tools/codex/dev-setup.sh symlinks both roots
    └-- [TESTED] normal install after dev setup replaces symlinks with copied dirs

[+] live runtime flow
    |
    ├-- [TESTED] doctor --json -> validate -> write -> doctor --json
    ├-- [TESTED] outside-git-repo refusal
    └-- [TESTED] existing-charter refusal
```

### Required tests and assertions

Add or tighten these checks:

1. generation smoke must assert that root `SKILL.md.tmpl` is the source for `.agents/skills/system/SKILL.md`
2. generation smoke must assert that `charter-intake/SKILL.md.tmpl` is the source for `.agents/skills/system-charter-intake/SKILL.md`
3. smoke must assert that generated `SKILL.md` files contain valid frontmatter
4. smoke must assert that root runtime `bin/system` exists and is the packaged binary surface
5. smoke must assert the exact runtime root file set
6. smoke must assert that `runtime-assets.json` is the only payload-membership truth used by generation
7. live smoke must keep proving the shipped happy path and refusal behavior unchanged

### Verification commands

These stay mandatory:

```bash
cargo fmt --all -- --check
cargo test --workspace
bash tools/codex/generate.sh
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh
```

## Failure Modes

| Failure mode | Detection | Expected handling |
| --- | --- | --- |
| root `SKILL.md.tmpl` exists but generator still reads `tools/codex/runtime/SKILL.md.tmpl` | generation smoke | fail immediately |
| leaf `SKILL.md.tmpl` exists but output name drifts from `system-charter-intake` | generation smoke | fail immediately |
| runtime `bin/` contains templated wrappers instead of packaged `system` binary | generation smoke | fail immediately |
| runtime payload list is duplicated in shell script and manifest | code review + smoke | collapse back to `runtime-assets.json` only |
| generated `SKILL.md` lacks host-valid frontmatter | smoke or live load | fail as a ship blocker |
| install copies the wrong directory names into `~/.codex/skills/` | install smoke | fail as a ship blocker |
| docs still describe `tools/codex/` as the authored source | doc review | patch in same milestone |
| runtime behavior changes while moving authored sources | live smoke | treat as regression and fix before merge |

No new silent failure modes should be introduced. This is a layout correction, not a semantics expansion.

## Performance Review

`M10` is packaging-heavy, not CPU-heavy.

Acceptable cost:

- one extra manifest read during generation
- no meaningful runtime slowdown
- no new install-time network or build dependency

Unacceptable cost:

- a generic packaging framework
- new runtime indirection during normal skill execution
- duplicated scans of the repo to infer payload membership implicitly

## Docs And Contract Updates

Docs and contracts must say one coherent thing after the change:

- authored Codex packaging source is repo-root `SKILL.md.tmpl` plus `charter-intake/SKILL.md.tmpl`
- `tools/codex/generate.sh` is the projection tool
- `.agents/skills/**` is generated
- `~/.codex/skills/**` is installed
- installed naming stays `system` and `system-charter-intake`

Required contract updates:

- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
- any doc that still says authored packaging source lives under `tools/codex/`

## Worktree Parallelization Strategy

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Canonical source layout | repo root packaging sources, `charter-intake/` | — |
| Generator rewrite | `tools/codex/`, generated `.agents/skills/` | Canonical source layout |
| Smoke and conformance updates | `tools/ci/`, docs/contracts | Generator rewrite |
| Doc surface cleanup | `README.md`, `DESIGN.md`, `docs/` | Canonical source layout |

### Parallel lanes

- Lane A: Canonical source layout -> Generator rewrite
- Lane B: Doc surface cleanup
- Lane C: Smoke and conformance updates

### Execution order

Launch Lane A first.

Once Lane A freezes the final source paths and generated output names:

- launch Lane B and Lane C in parallel
- merge both
- run the full verification pass once all three lanes land

### Conflict flags

- Lane B and Lane C both touch docs language around packaging truth. Keep one owner for final wording pass.
- Lane A and Lane C both depend on exact generated file names. Do not start Lane C before Lane A locks them.

## Acceptance Criteria

`M10` is done only when all of this is true:

1. repo root contains the canonical root host skill source as `SKILL.md.tmpl`
2. `charter-intake/SKILL.md.tmpl` is the canonical leaf skill source
3. `tools/codex/generate.sh` consumes those authored sources, not `tools/codex/` templates
4. `runtime-assets.json` is the only authored runtime payload membership list
5. root runtime `bin/` contains `system`, not a templated wrapper artifact
6. `.agents/skills/system/` and `.agents/skills/system-charter-intake/` regenerate deterministically
7. `~/.codex/skills/system/` and `~/.codex/skills/system-charter-intake/` install exactly as before, except for the corrected root-runtime binary packaging shape
8. generated `SKILL.md` outputs are host-valid
9. `cargo test --workspace`, `tools/ci/install-smoke.sh`, and `tools/ci/codex-skill-live-smoke.sh` pass
10. docs no longer describe `tools/codex/` as the authored packaging home
11. the shipped `M9.5` happy path remains unchanged from the operator point of view, except where required to package `bin/system` correctly

## Completion Summary

- Step 0: Scope Challenge, accepted as canonical-source parity work with no behavior rewrite
- Architecture Review: root host skill retained, authored source moved to repo root plus leaf directory
- Code Quality Review: `tools/codex/` narrowed to tooling only, no generic abstraction added
- Test Review: packaging coverage tightened around projection truth, file set truth, and host-valid skill output
- Performance Review: no runtime expansion allowed
- NOT in scope: written
- What already exists: written
- Failure modes: written
- Parallelization: 3 lanes, A first, then B + C in parallel

This plan is ready to implement as written.
