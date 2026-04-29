<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-system/feat-m8-autoplan-restore-20260429-164522.md -->
# PLAN

## Status

This is the implementation plan for `M10` on branch `feat/m8`.

`M9.5` already shipped and passed verification. `M10` is not a behavior rewrite, not a new host milestone, and not release packaging. It is the source-of-truth correction that makes `system` match the `gstack` style of Codex skill authorship and projection, while intentionally preserving the installed family name `system`.

## Checkpoint Resume

Recovered from `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/checkpoints/20260429-142427-m95-codex-packaging-landed-m10-gstack-parity-gap.md`.

Locked facts from that checkpoint:

- `M9.5` landed and is usable.
- The install and discovery surface already works.
- The gap is canonical packaging authorship, not runtime behavior for operators.
- `gstack` parity means repo-owned `SKILL.md.tmpl` sources, generated host projections, and a real root host skill pattern.
- The root host skill pattern stays in scope for `system`.

## Locked Direction

These points are not up for reinterpretation inside `M10`:

1. Match the `gstack` type of install surface and source layout, not an approximation invented during implementation.
2. Keep the root host skill pattern.
3. Keep the discoverable leaf skill `system-charter-intake`.
4. Keep the installed family name `system`, not a repo-name alias.
5. Keep user-home runtime naming under `system`, including `~/.codex/skills/system`, `~/.codex/skills/system-charter-intake`, and `~/.local/state/system/intake/runs/`.
6. Preserve the shipped `M9.5` operator contract unless a change is required to make the packaging surface truly match the target shape.
7. Do not reopen earlier reviewed reduced-v1 pipeline boundaries during `M10`; the M1 activation clause shape remains boolean-only in the form `variables.<name> == true|false`.

## Objective

Move Codex packaging authorship out of `tools/codex/` and into a repo-owned canonical skill layout that mirrors `gstack`:

- repo root owns the root host skill source
- leaf skill directory owns the discoverable skill source
- authored skill content lives in `SKILL.md.tmpl`
- generated host outputs live under `.agents/skills/`
- installed outputs live under `~/.codex/skills/`
- runtime payload membership is driven by one explicit manifest

The result should be boring:

- one authored source layout
- one generator
- one generated repo-local projection
- one installed projection
- one place to edit each class of truth

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

- Root `SKILL.md.tmpl` is the canonical authored source for the root host skill `system`.
- `charter-intake/SKILL.md.tmpl` is the canonical authored source for the discoverable leaf skill `system-charter-intake`.
- `runtime-manifest.json.tmpl` is the canonical authored source for runtime manifest metadata.
- `runtime-assets.json` is the only authored list of non-template runtime payloads copied into the root runtime.
- The orchestration currently authored in `tools/codex/runtime/bin/system-charter-intake.tmpl` moves into `charter-intake/SKILL.md.tmpl`.
- There are no handwritten runtime shell programs left under `tools/codex/runtime/bin/` after `M10`.
- `bin/` in generated and installed runtime roots contains the packaged `system` binary, not a template wrapper.
- Runtime payload content that already belongs in `core/library/**` stays there. `M10` does not create a second handwritten content tree.

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

That is the intentional naming tweak. It is not a different surface model.

## Scope Lock

### In scope

- add repo-root `SKILL.md.tmpl` for the root host skill
- add repo-root `runtime-manifest.json.tmpl`
- add repo-root `runtime-assets.json`
- add leaf `charter-intake/SKILL.md.tmpl`
- rewrite `tools/codex/generate.sh` to project from those authored sources
- package the real `system` binary into the root runtime as `bin/system`
- move intake orchestration from a runtime wrapper template into the generated leaf `SKILL.md`
- keep `.agents/skills/system` and `.agents/skills/system-charter-intake` as generated outputs
- keep `tools/codex/install.sh`, `tools/codex/dev-setup.sh`, and `tools/codex/relink.sh` as the install and dev entrypoints
- update smoke rails and docs so they describe the new source-of-truth model exactly
- preserve repo-local runtime override precedence: `<repo>/.agents/skills/system` first, then `~/.codex/skills/system`

### NOT in scope

- changing `system doctor --json`
- changing `system author charter --validate --from-inputs`
- changing `system author charter --from-inputs`
- adding more discoverable skills
- adding `.claude/skills/` generation
- adding a generic multi-host packaging framework
- changing the repo-local `.agents/skills/` override precedence model
- adding stale-override invalidation logic that does not exist in the target `gstack` shape
- public release packaging or registry distribution

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing code or asset | `M10` decision |
| --- | --- | --- |
| generator entrypoint | `tools/codex/generate.sh` | keep, but make it projection-only |
| install flow | `tools/codex/install.sh` | keep |
| dev symlink flow | `tools/codex/dev-setup.sh`, `tools/codex/relink.sh` | keep |
| generated output names | `.agents/skills/system`, `.agents/skills/system-charter-intake` | keep |
| installed output names | `~/.codex/skills/system`, `~/.codex/skills/system-charter-intake` | keep |
| root runtime templates | `tools/codex/runtime/SKILL.md.tmpl`, `tools/codex/runtime/runtime-manifest.json.tmpl` | replace with repo-root authored sources |
| leaf skill template | `tools/codex/templates/system-charter-intake.SKILL.md.tmpl` | move to `charter-intake/SKILL.md.tmpl` |
| runtime wrapper logic | `tools/codex/runtime/bin/system-charter-intake.tmpl` | remove as an authored runtime file, inline into leaf `SKILL.md.tmpl` |
| runtime payload files | `core/library/authoring/charter_authoring_method.md`, `core/library/charter/**` | keep as content sources, copy via manifest |
| runtime binary contract | installed `system` CLI on `PATH` | copy that exact binary into generated root runtime after version check |
| conformance rails | `tools/ci/install-smoke.sh`, `tools/ci/codex-skill-live-smoke.sh`, `docs/contracts/C-07-conformance-rails-and-docs-cutover.md` | extend for the new source model |

### Minimum change set

The minimum honest implementation is:

1. introduce repo-root and leaf authored sources
2. rewrite generation to consume them
3. package `bin/system` instead of an authored wrapper script
4. keep generated and installed output names stable
5. prove the new source model through smoke rails and docs

Anything less leaves `tools/codex/` as the real authored home. That would miss the whole point.

### Complexity check

This plan touches more than 8 files, but it does not introduce new crates, new services, or new infrastructure. The blast radius is packaging sources, packaging scripts, generated assets, smoke rails, and docs/contracts only. That is acceptable.

### Search and parity check

The target pattern is grounded in the actual `gstack` shape already present on disk:

- a repo-owned root `SKILL.md.tmpl`
- repo-owned leaf `SKILL.md.tmpl` sources
- generated `.agents/skills/<family>` and `.agents/skills/<family>-<skill>` projections
- installed `~/.codex/skills/<family>` and `~/.codex/skills/<family>-<skill>` surfaces

The current `system` layout is weaker because authored truth is split across `tools/codex/runtime/`, `tools/codex/templates/`, and `core/library/`. `M10` fixes that.

### TODOS cross-reference

`TODOS.md` already has downstream items for public CLI distribution and later host surfaces. Nothing in `TODOS.md` blocks `M10`, and `M10` does not need new TODO entries if it lands as written.

### Completeness check

The complete version is cheap here. Half-moving the source model would save almost no effort and force another cleanup milestone immediately after this one.

### Distribution check

`M10` does not introduce a new artifact type. Local Codex installation remains the only supported distribution path in this milestone. Public CLI release automation stays deferred.

## Architecture Review

### Current to target

```text
CURRENT
=======
tools/codex/runtime/*           tools/codex/templates/*
             \                  /
              \                /
               +-- generate.sh +
                        |
                        v
              .agents/skills/system*
                        |
                        v
              ~/.codex/skills/system*

TARGET
======
repo root SKILL.md.tmpl
repo root runtime-manifest.json.tmpl
repo root runtime-assets.json
charter-intake/SKILL.md.tmpl
             |
             v
     tools/codex/generate.sh
             |
             +--> .agents/skills/system/
             |      - SKILL.md
             |      - runtime-manifest.json
             |      - bin/system
             |      - share/**
             |
             +--> .agents/skills/system-charter-intake/
                    - SKILL.md
             |
             v
     tools/codex/install.sh
             |
             v
     ~/.codex/skills/system/
     ~/.codex/skills/system-charter-intake/
```

### Ownership contract

| Concern | Authoritative source | Generated output | Not allowed after `M10` |
| --- | --- | --- | --- |
| root host skill text | `SKILL.md.tmpl` | `.agents/skills/system/SKILL.md` | authored root skill text in `tools/codex/runtime/` |
| leaf skill text | `charter-intake/SKILL.md.tmpl` | `.agents/skills/system-charter-intake/SKILL.md` | authored leaf skill text in `tools/codex/templates/` |
| runtime manifest metadata | `runtime-manifest.json.tmpl` | `.agents/skills/system/runtime-manifest.json` | inline manifest JSON in shell |
| runtime payload membership | `runtime-assets.json` | `.agents/skills/system/share/**` | hidden copy lists inside `generate.sh` |
| packaged executable | installed `system` binary on `PATH`, version-matched to repo `VERSION` | `.agents/skills/system/bin/system` | generated wrapper as the primary runtime program |
| intake orchestration | `charter-intake/SKILL.md.tmpl` | `.agents/skills/system-charter-intake/SKILL.md` | authored `tools/codex/runtime/bin/system-charter-intake.tmpl` |

### Root host skill contract

The root host skill stays, but it is not the main discoverable workflow. Its job is:

- root skill metadata for the `system` family
- runtime manifest ownership
- packaged binary ownership
- shared payload ownership
- install and override anchor for the family

It should describe the runtime root clearly and refuse hand edits to generated output.

### Leaf skill orchestration contract

The discoverable workflow remains `system-charter-intake`. After `M10`, the generated leaf `SKILL.md` owns the orchestration that `M9.5` previously put in `bin/system-charter-intake`.

The generated leaf `SKILL.md` must do all of this explicitly:

1. resolve `repo_root` with `git rev-parse --show-toplevel`
2. resolve runtime root in this order:
   - `<repo>/.agents/skills/system`
   - `~/.codex/skills/system`
3. require this exact runtime root file set:
   - `SKILL.md`
   - `runtime-manifest.json`
   - `bin/system`
   - `share/authoring/charter_authoring_method.md`
   - `share/charter/CHARTER_INPUTS.yaml.tmpl`
   - `share/charter/charter_inputs_directive.md`
4. export `SYSTEM_CODEX_ROOT` to the resolved runtime root
5. run the shipped loop using `"$SYSTEM_CODEX_ROOT/bin/system"` for every command, not `PATH` lookup:
   - `doctor --json`
   - `setup` if required
   - `author charter --validate --from-inputs`
   - `author charter --from-inputs`
   - post-write `doctor --json`
6. keep the run-evidence contract at `~/.local/state/system/intake/runs/<timestamp-pid>/`
7. keep `system doctor --json` as the only machine-parsed output

This is the key architectural decision in `M10`: orchestration moves into the generated skill document, while the packaged runtime root contains only the real `system` binary plus shared assets.

### Generator contract

`tools/codex/generate.sh` becomes purely a projection script. It must:

1. read repo `VERSION`
2. require `system` on `PATH`
3. assert `system --version` matches repo `VERSION`
4. render root `SKILL.md.tmpl` into `.agents/skills/system/SKILL.md`
5. render `runtime-manifest.json.tmpl` into `.agents/skills/system/runtime-manifest.json`
6. render `charter-intake/SKILL.md.tmpl` into `.agents/skills/system-charter-intake/SKILL.md`
7. copy the resolved `system` binary into `.agents/skills/system/bin/system`
8. copy payload assets declared in `runtime-assets.json` into `.agents/skills/system/share/**`
9. remove outputs that no longer belong, including `bin/system-charter-intake`

The generator may not contain any second copy of runtime payload membership once `runtime-assets.json` exists.

### Install behavior

Install behavior stays the shipped `M9.5` story:

- `bash tools/codex/install.sh` regenerates and copies both roots into `~/.codex/skills/`
- `bash tools/codex/dev-setup.sh` regenerates and symlinks both roots into `~/.codex/skills/`
- normal install after dev setup replaces symlinks with copied directories

No new install surface is introduced. No repo clone is copied into the home directory.

## Code Quality Review

### Required design rules

- `tools/codex/` becomes tooling only, not authored skill content
- generated `.agents/skills/**` remains ignored build output
- no duplicated handwritten copies of `core/library/**` payload files
- no hidden runtime shell wrapper authored under `tools/codex/runtime/bin/`
- no generic host abstraction layer
- no new crate
- no new command surface

### Directory hygiene

After `M10`, a contributor should be able to answer these questions in under 30 seconds:

1. Where is the authored root skill source?
2. Where is the authored leaf skill source?
3. Where is the authoritative runtime manifest template?
4. Where is the authoritative runtime payload list?
5. What script turns authored sources into generated projections?

If any answer still starts with "part of it is in `tools/codex/` and part of it is somewhere else", the plan failed.

## Implementation Plan

### Step 1: Introduce canonical authored sources

Add:

- `SKILL.md.tmpl`
- `runtime-manifest.json.tmpl`
- `runtime-assets.json`
- `charter-intake/SKILL.md.tmpl`

Authorship rules:

- the two `SKILL.md.tmpl` files are the only handwritten skill documents
- root `SKILL.md.tmpl` describes the runtime family, not the intake workflow
- leaf `charter-intake/SKILL.md.tmpl` contains the intake workflow, including the orchestration now authored in the wrapper template
- no template files belong under runtime `bin/`

### Step 2: Rewire generation

Update `tools/codex/generate.sh` so it:

- renders the root and leaf `SKILL.md.tmpl` sources
- renders the runtime manifest from the repo-root template
- reads `runtime-assets.json` as the only copy-membership source
- copies the version-matched `system` binary into `.agents/skills/system/bin/system`
- deletes legacy output files that no longer belong, especially `bin/system-charter-intake`
- regenerates `.agents/skills/system/**` and `.agents/skills/system-charter-intake/**`

### Step 3: Keep install and relink entrypoints stable

Keep the external commands unchanged:

- `bash tools/codex/generate.sh`
- `bash tools/codex/install.sh`
- `bash tools/codex/dev-setup.sh`
- `bash tools/codex/relink.sh`

Only their source inputs and expected runtime file set change.

### Step 4: Update generated outputs and expectations

- regenerate `.agents/skills/system/**`
- regenerate `.agents/skills/system-charter-intake/**`
- update any tests or snapshots that assert generated content
- make the exact runtime root file set match the new contract

### Step 5: Update smoke rails and docs

Update:

- `tools/ci/install-smoke.sh`
- `tools/ci/codex-skill-live-smoke.sh`
- `README.md`
- `DESIGN.md`
- `docs/SUPPORTED_COMMANDS.md`
- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`

The doc story after `M10` must say one coherent thing:

- canonical Codex packaging authorship lives at repo root plus `charter-intake/`
- `tools/codex/` is projection and install tooling
- `.agents/skills/**` is generated output
- `~/.codex/skills/**` is installed output
- installed naming stays `system` and `system-charter-intake`

## Test Review

### Test framework and proof surface

This repo already has the right proof layers:

- Rust workspace tests through `cargo test --workspace`
- packaging smoke via `tools/ci/install-smoke.sh`
- installed-skill live smoke via `tools/ci/codex-skill-live-smoke.sh`

No new framework is needed. The smoke rails just need to test the corrected source model.

### Coverage diagram

```text
CODE PATH COVERAGE
==================
[+] authored source mapping
    |
    ├-- [REQ] root SKILL.md.tmpl renders .agents/skills/system/SKILL.md
    ├-- [REQ] leaf charter-intake/SKILL.md.tmpl renders .agents/skills/system-charter-intake/SKILL.md
    ├-- [REQ] runtime-manifest.json.tmpl renders .agents/skills/system/runtime-manifest.json
    └-- [REQ] runtime-assets.json fully defines copied share/** payload membership

[+] generation flow
    |
    ├-- [REQ] generate.sh refuses when PATH system version != repo VERSION
    ├-- [REQ] generate.sh copies bin/system, not bin/system-charter-intake
    ├-- [REQ] generated SKILL.md files are host-valid and include required frontmatter
    └-- [REQ] generated runtime root file set matches contract exactly

[+] installed runtime flow
    |
    ├-- [REQ] install.sh installs both roots
    ├-- [REQ] dev-setup.sh symlinks both roots
    ├-- [REQ] install after dev-setup replaces symlinks with copied dirs
    └-- [REQ] repo-local runtime override still wins over ~/.codex/skills/system

[+] intake orchestration flow
    |
    ├-- [REQ] generated leaf SKILL contains runtime-root resolution in the correct order
    ├-- [REQ] generated leaf SKILL invokes "$SYSTEM_CODEX_ROOT/bin/system" for doctor/setup/validate/write
    ├-- [REQ] happy path preserves run-evidence files under ~/.local/state/system/intake/runs/
    ├-- [REQ] existing-charter refusal still produces refusal evidence
    └-- [REQ] outside-git-repo refusal still happens before any mutation
```

### Required tests and assertions

1. generation smoke must assert that repo-root `SKILL.md.tmpl` is the source for `.agents/skills/system/SKILL.md`
2. generation smoke must assert that `charter-intake/SKILL.md.tmpl` is the source for `.agents/skills/system-charter-intake/SKILL.md`
3. smoke must assert that generated `SKILL.md` files contain valid frontmatter
4. smoke must assert that root runtime contains `bin/system` and does not contain `bin/system-charter-intake`
5. smoke must assert the exact runtime root file set
6. smoke must assert that `runtime-assets.json` is the only payload-membership truth used by generation
7. live smoke must assert the generated leaf `SKILL.md` still contains the exact orchestration commands and paths
8. live smoke must reproduce the happy path and refusal behavior using the generated-skill contract, not a handwritten runtime wrapper

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

| Failure mode | Test covers it? | Error handling exists? | User-visible? | Critical gap if missing? |
| --- | --- | --- | --- | --- |
| root `SKILL.md.tmpl` exists but generator still reads `tools/codex/runtime/SKILL.md.tmpl` | yes, generation smoke | fail generation | yes | yes |
| leaf `SKILL.md.tmpl` exists but output name drifts from `system-charter-intake` | yes, generation smoke | fail generation | yes | yes |
| runtime root still contains `bin/system-charter-intake` and omits `bin/system` | yes, install smoke | fail generation or install smoke | yes | yes |
| runtime payload list is duplicated in shell and manifest | yes, code review plus smoke | collapse back to `runtime-assets.json` only | indirectly | yes |
| generated leaf `SKILL.md` calls `system` on `PATH` instead of `"$SYSTEM_CODEX_ROOT/bin/system"` | yes, live smoke grep plus execution | fail smoke | yes | yes |
| repo-local runtime override no longer wins | yes, live smoke | fail smoke | yes | yes |
| docs still describe `tools/codex/` as the authored source | yes, doc review | patch in same milestone | yes | no |
| run-evidence files disappear from `~/.local/state/system/intake/runs/` | yes, live smoke | fail smoke | yes | yes |

There should be no silent failure path in `M10`. Every break in source mapping or runtime packaging must fail in smoke.

## Performance Review

`M10` is packaging-heavy, not CPU-heavy.

Acceptable cost:

- one manifest read during generation
- one runtime-assets manifest read during generation
- no meaningful runtime slowdown
- no new install-time network dependency

Unacceptable cost:

- a generic packaging framework
- new runtime indirection during normal skill execution
- duplicate scans of the repo to infer payload membership implicitly
- rebuilding the CLI from scratch inside every smoke step when a version-matched installed binary already exists

## Docs And Contract Updates

Docs and contracts must say one coherent thing after the change:

- authored Codex packaging source is repo-root `SKILL.md.tmpl` plus `charter-intake/SKILL.md.tmpl`
- `tools/codex/generate.sh` is the projection tool
- `.agents/skills/**` is generated
- `~/.codex/skills/**` is installed
- installed naming stays `system` and `system-charter-intake`
- the generated leaf skill owns the orchestration contract, not a packaged wrapper script

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

Once Lane A freezes the final source paths, generated file names, and `bin/system` contract:

- launch Lane B and Lane C in parallel
- merge both
- run the full verification pass once all three lanes land

### Conflict flags

- Lane B and Lane C both touch packaging wording. Keep one final wording owner for docs.
- Lane A and Lane C both depend on the exact runtime file set. Do not start Lane C before Lane A locks it.
- Lane C owns the final truth for smoke expectations. Lane B must not reword docs in a way that contradicts smoke assertions.

## Acceptance Criteria

`M10` is done only when all of this is true:

1. repo root contains the canonical root host skill source as `SKILL.md.tmpl`
2. `charter-intake/SKILL.md.tmpl` is the canonical leaf skill source
3. `tools/codex/generate.sh` consumes those authored sources, not `tools/codex/` templates
4. `runtime-assets.json` is the only authored runtime payload membership list
5. root runtime `bin/` contains `system`, not a templated wrapper artifact
6. intake orchestration now lives in the generated leaf `SKILL.md`, not in a packaged wrapper script
7. `.agents/skills/system/` and `.agents/skills/system-charter-intake/` regenerate deterministically
8. `~/.codex/skills/system/` and `~/.codex/skills/system-charter-intake/` install exactly as before, except for the corrected root-runtime binary packaging shape
9. generated `SKILL.md` outputs are host-valid
10. `cargo test --workspace`, `tools/ci/install-smoke.sh`, and `tools/ci/codex-skill-live-smoke.sh` pass
11. docs no longer describe `tools/codex/` as the authored packaging home
12. the shipped `M9.5` operator-visible happy path and refusal behavior remain unchanged

## Completion Summary

- Step 0: Scope Challenge, accepted as canonical-source parity work with no behavior rewrite
- Architecture Review: root host skill retained, authored source moved to repo root plus leaf directory, orchestration moved into leaf `SKILL.md`
- Code Quality Review: `tools/codex/` narrowed to tooling only, no generic abstraction added
- Test Review: coverage tightened around source mapping truth, runtime file set truth, generated-skill orchestration truth, and refusal-path preservation
- Performance Review: no runtime expansion allowed
- NOT in scope: written
- What already exists: written
- Failure modes: written
- Parallelization: 3 lanes, Lane A first, then Lane B + Lane C in parallel

This plan is ready to implement as written.
