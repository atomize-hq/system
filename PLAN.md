# PLAN

## Status

This is the corrective implementation plan for `M10.5` on branch `feat/m10`.

It replaces the earlier `M10` packaging story because the repo still codifies the wrong model in three places:

- authored install-home inputs still live at repo root instead of under a curated install subtree
- the installed runtime still includes a second helper executable, `~/system/bin/system-charter-intake`
- installed static guidance still uses `share/**` naming even though this is product runtime content and should be `resources/**`

The milestone lock is:

> We want a gstack-style installed home, but populated from a curated `install/system-home/` source subtree, not from repo root and not by installing the repo itself; `~/system/bin/system` is the only installed executable, and `system-charter-intake` is only a skill/discovery surface, not a second binary.

## Objective

Land one coherent packaging and runtime contract:

1. Authored install-home skill inputs live under `install/system-home/`.
2. Repo `.agents/skills/*` remains thin generated projection output only.
3. `~/system/` is the only installed product home.
4. `~/.codex/skills/*` remains discovery glue only.
5. `~/system/bin/system` is the only installed executable for this Codex surface.

Success means all of the following are true at once:

- repo root no longer owns install-home authored files
- normal install produces a curated `~/system/` home, not a repo clone
- the installed runtime has `runtime-manifest.json` plus `resources/**`, but no `bin/system-charter-intake` and no `share/**`
- normal install restores discovery to `~/system/.agents/skills/*` even after dev override mode
- the `system-charter-intake` skill executes by invoking `~/system/bin/system` directly

## Step 0: Scope Challenge

### What already exists

| Surface | Current state | Keep / change |
| --- | --- | --- |
| `tools/codex/generate.sh` | already generates thin repo `.agents/skills/system*` | keep purpose, change inputs and stop writing repo-root skill files |
| `tools/codex/install.sh` | already owns `~/system/` mutation, PATH checks, version checks, stage-then-swap install | keep install spine, change staged payload and delete legacy runtime wrapper contract |
| `tools/codex/dev-setup.sh` | already points Codex discovery at repo-generated projections | keep as the only explicit dev override flow |
| `tools/codex/relink.sh` | currently aliases `dev-setup.sh` and therefore reinforces the wrong default mental model | remove or repurpose; `install.sh` should be the only production relink path |
| repo `.agents/skills/system*` | already thin | keep thinness invariant |
| repo-root `SKILL.md.tmpl`, `SKILL.md`, `agents/openai.yaml`, `charter-intake/**` | currently treated as install-home truth | migrate authored truth into `install/system-home/`, then delete repo-root ownership |
| `tools/codex/runtime/runtime-manifest.json.tmpl` | still part of the installed runtime contract | keep, but only as install-time manifest source |
| `tools/codex/runtime/bin/system-charter-intake.tmpl` | still encodes the second helper-binary contract | delete this contract entirely |
| `core/library/authoring/**` and `core/library/charter/**` | already hold the static guidance the installed runtime needs | keep as the source of installed `resources/**` payload |
| `tools/ci/install-smoke.sh` | already validates install topology in an isolated home | keep harness, rewrite assertions to the exact `M10.5` file set |
| `tools/ci/codex-skill-live-smoke.sh` | already validates end-to-end skill execution | keep harness, rewrite it around direct `~/system/bin/system` invocation and `resources/**` |

### Existing-code leverage map

| Sub-problem | Existing code to reuse | `M10.5` action |
| --- | --- | --- |
| generate thin repo projections | `tools/codex/generate.sh` | rewrite inputs, preserve thin output contract |
| stage and atomically replace `~/system/` | `tools/codex/install.sh` temp-dir swap pattern | preserve pattern, shrink staged payload to the curated contract |
| dev discovery override | `tools/codex/dev-setup.sh` | preserve as explicit override mode only |
| restore production discovery topology | `tools/codex/install.sh` | make reinstall the only supported production relink path |
| installed runtime manifest rendering | `tools/codex/runtime/runtime-manifest.json.tmpl` | preserve manifest role, keep it install-owned |
| installed static guidance | `core/library/authoring/**`, `core/library/charter/**` | preserve source locations, rename installed destination to `resources/**` |
| live skill smoke | `tools/ci/codex-skill-live-smoke.sh` | preserve scenario coverage, change runtime expectations |
| install smoke | `tools/ci/install-smoke.sh` | preserve file-set and symlink assertions, update the expected contract |

### Minimum change set

This milestone touches more than 8 files. That is not scope creep here, it is one packaging contract spread across scripts, templates, generated projections, smoke rails, and user-facing docs.

The minimum complete fix is:

1. move authored install-home skill inputs into `install/system-home/`
2. make `generate.sh` read only from that subtree and emit only thin repo projections
3. make `install.sh` build the curated installed home from explicit sources instead of repo-root install-home files
4. delete the second helper-binary contract entirely
5. rename installed static guidance from `share/**` to `resources/**`
6. cut docs and smokes to the same exact topology

Anything smaller leaves dual truth in the repo. That is worse than waiting.

### Complexity ruling

The plan is intentionally not being scope-reduced below the six items above.

Why:

- repo-root authorship, helper-binary runtime, and `share/**` naming are all the same bug, one incorrect packaging story split across different files
- changing only one layer would guarantee another corrective pass
- the blast radius is bounded and boring: shell scripts, markdown, symlinks, and exact file-set assertions

### Search and boring-tech check

This plan introduces no new infrastructure and spends zero innovation tokens.

It stays with:

- shell scripts
- deterministic file copies
- rendered templates
- symlink-based discovery glue
- exact smoke assertions

That is the right call.

### Completeness check

Shortcut version:

- move files under `install/system-home/`
- keep the helper binary
- keep `share/**`
- leave docs and smokes partially stale

Complete version:

- move authored install-home skill inputs under `install/system-home/`
- install only `~/system/bin/system`
- delete the helper-binary contract
- rename installed static guidance to `resources/**`
- make docs and smoke rails fail on any drift

Recommendation: do the complete version. The delta is minutes of shell and docs work now versus another cleanup milestone immediately after this one.

### Distribution check

This milestone does not add a new public artifact type. Distribution remains explicit local installation on supported development targets only.

Out of scope for `M10.5`:

- GitHub Releases
- package-manager distribution
- installer/updater channels
- multi-platform release automation

That deferral must stay explicit. `M10.5` is about fixing the local installed-home contract, not widening distribution.

## Canonical Contract

### Source-of-truth layers

| Layer | Owner | Purpose | Allowed contents | Forbidden contents |
| --- | --- | --- | --- | --- |
| `install/system-home/` | authored source | skill-facing install-home inputs | `SKILL.md.tmpl`, `agents/openai.yaml`, leaf skill templates | generated outputs, runtime manifest, installed resources payload |
| `tools/codex/runtime/` | install-time runtime templates | installed runtime metadata only | `runtime-manifest.json.tmpl` | helper executables, discovery projections |
| `core/library/authoring/**`, `core/library/charter/**` | canonical library content | installed static guidance | markdown/templates copied into `resources/**` | install logic, generated projections |
| repo `.agents/skills/*` | generator output | thin repo-local discovery projections | `SKILL.md`, `agents/openai.yaml` | `bin/`, `runtime-manifest.json`, `resources/` |
| `~/system/` | installer-owned runtime | real installed product home | exact curated file set below | repo clone, helper binaries, stray legacy payload |
| `~/.codex/skills/*` | discovery glue | Codex discovery only | symlinks or thin copies into `~/system/.agents/skills/*` or repo projections in dev mode | runtime payload, install-home source |

### Repo source shape

```text
install/system-home/
  SKILL.md.tmpl
  agents/
    openai.yaml
  charter-intake/
    SKILL.md.tmpl
```

Rules:

- `install/system-home/` is the authored source of truth for install-home skill content.
- repo root must not retain authored install-home files after this migration.
- shared installed runtime guidance still comes from `core/library/**`, not from `install/system-home/`.
- the installer never installs the repo itself.

### Installed home shape

```text
~/system/
  SKILL.md.tmpl
  SKILL.md
  agents/
    openai.yaml
  runtime-manifest.json
  bin/
    system
  charter-intake/
    SKILL.md.tmpl
    SKILL.md
  resources/
    authoring/
      charter_authoring_method.md
    charter/
      CHARTER_INPUTS.yaml.tmpl
      charter_inputs_directive.md
  .agents/
    skills/
      system/
        SKILL.md
        agents/
          openai.yaml
      system-charter-intake/
        SKILL.md
        agents/
          openai.yaml
```

Rules:

- `~/system/` is curated and install-owned, not a git checkout.
- `~/system/bin/system` is the only installed executable for this Codex surface.
- `~/system/runtime-manifest.json` remains part of the runtime contract.
- `~/system/resources/**` is the installed static guidance root.
- `~/system/.agents/skills/*` stays thin.
- there is no `~/system/bin/system-charter-intake`.
- there is no `~/system/share/**`.

### Codex discovery shape

```text
~/.codex/skills/
  system -> ~/system/.agents/skills/system
  system-charter-intake -> ~/system/.agents/skills/system-charter-intake
```

Rules:

- `~/.codex/skills/*` is discovery glue only.
- normal install must restore this topology.
- dev setup may temporarily repoint discovery to repo `.agents/skills/*`, but only in explicit override mode.

### Installed artifact map

| Installed path | Source | Produced by |
| --- | --- | --- |
| `~/system/SKILL.md.tmpl` | `install/system-home/SKILL.md.tmpl` | `install.sh` copy |
| `~/system/SKILL.md` | `install/system-home/SKILL.md.tmpl` | `install.sh` render |
| `~/system/agents/openai.yaml` | `install/system-home/agents/openai.yaml` | `install.sh` copy |
| `~/system/charter-intake/SKILL.md.tmpl` | `install/system-home/charter-intake/SKILL.md.tmpl` | `install.sh` copy |
| `~/system/charter-intake/SKILL.md` | `install/system-home/charter-intake/SKILL.md.tmpl` | `install.sh` render |
| `~/system/runtime-manifest.json` | `tools/codex/runtime/runtime-manifest.json.tmpl` | `install.sh` render |
| `~/system/bin/system` | verified `system` on `PATH` matching repo `VERSION` | `install.sh` copy |
| `~/system/resources/authoring/charter_authoring_method.md` | `core/library/authoring/charter_authoring_method.md` | `install.sh` copy |
| `~/system/resources/charter/CHARTER_INPUTS.yaml.tmpl` | `core/library/charter/CHARTER_INPUTS.yaml.tmpl` | `install.sh` copy |
| `~/system/resources/charter/charter_inputs_directive.md` | `core/library/charter/charter_inputs_directive.md` | `install.sh` copy |
| `~/system/.agents/skills/system/**` | repo `.agents/skills/system/**` | `install.sh` copy after generation |
| `~/system/.agents/skills/system-charter-intake/**` | repo `.agents/skills/system-charter-intake/**` | `install.sh` copy after generation |

### Explicit non-goals

We do not want:

- repo-root install-home source files
- `~/system/` as a repo clone
- runtime payload under repo `.agents/skills/*`
- runtime payload under `~/.codex/skills/*`
- a second installed helper binary
- compatibility shims that keep `share/**` around

## Behavioral Contract

### `tools/codex/generate.sh`

`generate.sh` must:

1. read authored skill inputs only from `install/system-home/`
2. render repo `.agents/skills/system/SKILL.md`
3. render repo `.agents/skills/system-charter-intake/SKILL.md`
4. copy `agents/openai.yaml` into both thin projections
5. assert the thin file set exactly
6. never write repo-root `SKILL.md`
7. never write repo-root `charter-intake/SKILL.md`
8. never mutate `$HOME`

### `tools/codex/install.sh`

`install.sh` must:

1. require `system` on `PATH`
2. verify `system --version` matches repo `VERSION`
3. call `generate.sh` or otherwise guarantee fresh thin projections before staging
4. stage the curated file set into a temp dir
5. render installed `SKILL.md` files directly from `install/system-home/**/*.tmpl`
6. render `runtime-manifest.json` from `tools/codex/runtime/runtime-manifest.json.tmpl`
7. copy static runtime guidance into `~/system/resources/**`
8. copy only the thin repo projections into `~/system/.agents/skills/*`
9. remove legacy heavy-install leftovers by replacing the entire installed root
10. refresh `~/.codex/skills/*` to point into `~/system/.agents/skills/*`

### `tools/codex/dev-setup.sh` and `tools/codex/relink.sh`

`dev-setup.sh` remains the only explicit override mode. Its job is simple: point Codex discovery at repo `.agents/skills/*`.

`relink.sh` in its current form is ambiguous because it is just another way to enable dev mode. `M10.5` should remove that ambiguity:

- either delete `tools/codex/relink.sh`
- or repurpose it to mean "restore production discovery topology" by delegating to install behavior

Recommendation: delete it. `dev-setup.sh` is the override path. `install.sh` is the production relink path. One command for each story.

### `system-charter-intake` runtime flow

After `M10.5`, the leaf skill flow is:

1. refuse immediately if the user is not inside a real git repo
2. resolve `SYSTEM_HOME`, defaulting to `~/system`
3. assert `~/system/bin/system` exists and is executable
4. assert `~/system/runtime-manifest.json` exists
5. assert required runtime guidance exists under `~/system/resources/**`
6. validate manifest fields and verify the installed binary version still matches the manifest
7. run the existing doctor/setup/validate/write loop by invoking `~/system/bin/system` directly
8. persist transcripts and session evidence under `~/.local/state/system/intake/runs/`

Rules:

- the leaf skill remains a skill/discovery surface, not an installed executable
- no writes land under `~/system/`
- no writes land under `~/.codex/skills/`
- the manifest still records `skill_name = system-charter-intake`
- installed static guidance is read from `resources/**`, never `share/**`

### Repo and runtime paths that must disappear

The plan is incomplete if any of these remain as active contract paths:

- repo-root `SKILL.md.tmpl`
- repo-root `SKILL.md`
- repo-root `agents/openai.yaml`
- repo-root `charter-intake/SKILL.md.tmpl`
- repo-root `charter-intake/SKILL.md`
- `tools/codex/runtime/bin/system-charter-intake.tmpl`
- `~/system/bin/system-charter-intake`
- `~/system/share/**`

## Architecture

### Packaging dependency graph

```text
install/system-home/*.tmpl            core/library/**                 tools/codex/runtime/**
          │                                   │                                  │
          ├──────────────┐                    │                                  │
          │              │                    │                                  │
          ▼              ▼                    ▼                                  ▼
   tools/codex/generate.sh          tools/codex/install.sh <──────────── runtime-manifest.json.tmpl
          │                                   │
          │                                   ├── render installed SKILL.md files
          │                                   ├── copy verified ~/system/bin/system
          │                                   ├── copy installed resources/**
          │                                   ├── copy thin .agents projections
          │                                   └── refresh ~/.codex/skills/*
          ▼
repo .agents/skills/system*
          │
          ▼
~/system/.agents/skills/system*
          │
          ▼
~/.codex/skills/system*
```

### Runtime invocation flow

```text
Codex discovery: ~/.codex/skills/system-charter-intake
        │
        ▼
installed thin skill: ~/system/.agents/skills/system-charter-intake/SKILL.md
        │
        ▼
resolve repo root -> resolve SYSTEM_HOME -> validate manifest/resources/binary
        │
        ▼
invoke ~/system/bin/system
        │
        ├── doctor --json
        ├── optional setup guidance
        ├── author charter --validate --from-inputs
        └── author charter --from-inputs
        │
        ▼
persist evidence under ~/.local/state/system/intake/runs/
```

### Exact modules and documents in scope

| Module / directory | Why it is in scope |
| --- | --- |
| `install/system-home/` | new authored install-home source of truth |
| `tools/codex/generate.sh` | generator input and output contract |
| `tools/codex/install.sh` | installed-home staging, rendering, binary copy, discovery refresh |
| `tools/codex/dev-setup.sh` | explicit dev override topology |
| `tools/codex/relink.sh` | remove or repurpose ambiguity |
| `tools/codex/runtime/**` | manifest template retained, helper-binary template removed |
| `tools/ci/install-smoke.sh` | exact installed-home and discovery assertions |
| `tools/ci/codex-skill-live-smoke.sh` | direct invocation and evidence-path assertions |
| `README.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, `DESIGN.md`, `docs/contracts/C-07-conformance-rails-and-docs-cutover.md` | packaging story and contract docs that currently expose the old topology |

## Implementation Plan

### Phase 1. Source migration and ownership cleanup

Create the authored source subtree:

```text
install/system-home/
  SKILL.md.tmpl
  agents/openai.yaml
  charter-intake/SKILL.md.tmpl
```

Tasks:

1. move repo-root authored install-home files into `install/system-home/`
2. update scripts to read from the new subtree
3. remove repo-root authored ownership completely
4. make it impossible for future generation to recreate repo-root install-home files

Exit criteria:

- repo root no longer contains active install-home authored files
- every consumer reads from `install/system-home/`

### Phase 2. Thin projection rewrite

Rewrite `tools/codex/generate.sh` so it:

1. reads only `install/system-home/SKILL.md.tmpl`
2. reads only `install/system-home/charter-intake/SKILL.md.tmpl`
3. copies only `install/system-home/agents/openai.yaml`
4. writes only repo `.agents/skills/system/**`
5. writes only repo `.agents/skills/system-charter-intake/**`
6. asserts the exact thin file set
7. performs no repo-root writes

Thin projection invariant:

```text
.agents/skills/system/
  SKILL.md
  agents/openai.yaml

.agents/skills/system-charter-intake/
  SKILL.md
  agents/openai.yaml
```

Exit criteria:

- repo `.agents/skills/*` remains thin and deterministic
- repo root stays free of generated install-home files

### Phase 3. Installed-home rewrite

Rewrite `tools/codex/install.sh` so it stages and installs only the locked curated file set under `~/system/`.

Required behaviors:

1. verify the `system` binary on `PATH` matches repo `VERSION`
2. render installed root and leaf `SKILL.md` files directly from `install/system-home/**/*.tmpl`
3. copy installed templates alongside the rendered outputs
4. render `runtime-manifest.json`
5. copy static guidance into `resources/**`
6. copy only thin skill projections into `.agents/skills/*`
7. replace the whole installed root so stale heavy-install payload cannot survive
8. refresh Codex discovery to the installed projections

Required deletions from the installed contract:

- `~/system/bin/system-charter-intake`
- `~/system/share/**`
- any runtime helper or updater binaries not listed in the target shape

Exit criteria:

- install produces the exact curated file set
- reinstall after old installs removes stale helper/share payload
- reinstall after dev override restores production discovery topology

### Phase 4. Runtime invocation rewrite

Delete the helper-binary runtime contract and move the leaf skill to direct CLI invocation.

Tasks:

1. remove `tools/codex/runtime/bin/system-charter-intake.tmpl`
2. update the leaf skill template so the skill itself performs the runtime checks and invokes `~/system/bin/system`
3. preserve the existing refusal order and transcript/evidence contract where topology is not the changing behavior
4. change every runtime resource reference from `share/**` to `resources/**`

Exit criteria:

- the skill works without `~/system/bin/system-charter-intake`
- runtime guidance is resolved from `resources/**`
- evidence lands only under `~/.local/state/system/intake/runs/`

### Phase 5. Docs and contract cutover

Update the packaging-facing docs so they all say the same thing:

- authored install-home skill truth lives under `install/system-home/`
- repo `.agents/skills/*` is thin generated projection output only
- `~/system/` is the installed home
- `runtime-manifest.json` remains part of the installed runtime contract
- installed static guidance lives under `~/system/resources/**`
- `~/.codex/skills/*` is discovery glue only
- `~/system/bin/system` is the only installed executable for this skill surface
- `system-charter-intake` is a skill/discovery surface, not a binary

Required doc targets:

- `README.md`
- `docs/START_HERE.md`
- `docs/SUPPORTED_COMMANDS.md`
- `DESIGN.md`
- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`

Exit criteria:

- no packaging doc still mentions `bin/system-charter-intake`
- no packaging doc still treats `share/**` as the installed resource root
- the contract doc matches the actual smoke assertions

### Phase 6. Smoke and regression rails

Rewrite install and live smokes to enforce the exact target shape and fail loudly on drift.

Required smoke assertions:

1. repo `.agents/skills/*` contains only thin projection files
2. installed `~/system/` matches the exact curated file set
3. `~/system/bin/system` exists and version-matches repo `VERSION`
4. `~/system/runtime-manifest.json` exists and validates its required fields
5. `~/system/resources/**` exists
6. `~/system/share/**` does not exist
7. `~/system/bin/system-charter-intake` does not exist
8. normal install after `dev-setup.sh` restores discovery to the installed projections
9. the leaf skill invokes `~/system/bin/system` directly
10. runtime evidence lands only under `~/.local/state/system/intake/runs/`

Exit criteria:

- both smoke scripts fail on any topology drift
- both smoke scripts pass on the locked target shape

## Test Review

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] install/system-home/*
    ├── source templates moved out of repo root
    └── agent metadata becomes the only authored install-home metadata source

[+] tools/codex/generate.sh
    ├── render root thin skill from install/system-home/SKILL.md.tmpl
    ├── render leaf thin skill from install/system-home/charter-intake/SKILL.md.tmpl
    ├── copy install/system-home/agents/openai.yaml into both projections
    └── refuse any extra generated payload in repo .agents/skills/*

[+] tools/codex/install.sh
    ├── require system on PATH
    ├── compare PATH binary version to repo VERSION
    ├── stage curated ~/system/ home
    ├── render installed SKILL.md files
    ├── render runtime-manifest.json
    ├── copy resources/**
    ├── copy thin installed projections
    ├── remove legacy helper/share payload by replacing the install root
    └── relink ~/.codex/skills/* to ~/system/.agents/skills/*

[+] tools/codex/dev-setup.sh
    ├── point ~/.codex/skills/system -> repo .agents/skills/system
    └── point ~/.codex/skills/system-charter-intake -> repo .agents/skills/system-charter-intake

[+] system-charter-intake skill flow
    ├── refusal outside git repo
    ├── refusal when ~/system/bin/system is missing
    ├── refusal when runtime-manifest.json is missing
    ├── refusal when resources/** is incomplete
    ├── direct invocation of ~/system/bin/system
    └── evidence lands only under ~/.local/state/system/intake/runs/
```

### User flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Normal install
    ├── [REQ] generate thin repo projections
    ├── [REQ] install curated ~/system/ home
    ├── [REQ] remove helper/share leftovers from prior installs
    └── [REQ] relink ~/.codex/skills/* into ~/system/.agents/skills/*

[+] Dev override
    ├── [REQ] dev-setup points discovery to repo-generated .agents/skills/*
    └── [REQ] normal install restores production discovery topology

[+] Charter intake execution
    ├── [REQ] leaf skill invokes ~/system/bin/system directly
    ├── [REQ] no helper binary is required
    ├── [REQ] runtime-manifest.json remains available
    ├── [REQ] installed guidance resolves from resources/**
    └── [REQ] run evidence stays under ~/.local/state/system/intake/runs/
```

### Required test additions

`tools/ci/install-smoke.sh` must assert:

1. repo `.agents/skills/system*` contains only `SKILL.md` and `agents/openai.yaml`
2. repo root does not contain active install-home authored files
3. installed `~/system/` contains the exact curated file set and nothing extra
4. `~/system/bin/system` exists and version-matches repo `VERSION`
5. `~/system/runtime-manifest.json` exists and validates required fields
6. `~/system/resources/**` exists with the renamed static payload
7. `~/system/share/**` does not exist
8. `~/system/bin/system-charter-intake` does not exist
9. `~/.codex/skills/system*` points into `~/system/.agents/skills/*` after normal install
10. normal install after `dev-setup.sh` restores installed discovery topology

`tools/ci/codex-skill-live-smoke.sh` must assert:

1. `system-charter-intake` works without `~/system/bin/system-charter-intake`
2. the leaf skill invokes `~/system/bin/system` directly
3. the leaf skill can still resolve `runtime-manifest.json`
4. the leaf skill can still resolve installed static guidance under `resources/**`
5. refusal outside a git repo still happens before questioning
6. refusal when the installed binary is missing is explicit
7. run evidence appears only under `~/.local/state/system/intake/runs/`
8. no run evidence lands under `~/system/` or `~/.codex/skills/`

Critical regression tests:

- helper-binary absence is mandatory, not optional
- `share/**` absence is mandatory, not optional
- repo-root install-home file absence is mandatory, not optional

## Failure Modes Registry

| Failure mode | Test covers it | Error handling | User-visible outcome |
| --- | --- | --- | --- |
| generator still reads repo-root skill files | install smoke + repo-root absence check | exact file-set assertion | immediate CI failure |
| generator still writes repo-root generated skill files | install smoke + repo-root absence check | exact file-set assertion | immediate CI failure |
| installed home drops `runtime-manifest.json` | install smoke | exact file-set assertion | immediate CI failure |
| installed home still contains `share/**` | install smoke | replace-whole-root install + absence assertion | immediate CI failure |
| installed home still contains `bin/system-charter-intake` | install smoke | replace-whole-root install + absence assertion | immediate CI failure |
| Codex discovery stays in repo-dev mode after normal install | install smoke | reinstall refreshes discovery | immediate CI failure |
| leaf skill still depends on helper-binary logic | live smoke | direct invocation rewrite | immediate CI failure |
| leaf skill cannot resolve `resources/**` | live smoke | runtime path assertions | immediate CI failure |
| installed `system` binary version does not match repo `VERSION` | install smoke + install hard-refusal | explicit install refusal | explicit install failure |
| evidence lands inside installed/discovery trees | live smoke | exact evidence-path assertion | immediate CI failure |

No silent failures are acceptable in this milestone. Packaging drift must fail in CI.

## Code Quality Review

### DRY and clarity requirements

- one authored install-home subtree, not duplicated root and subtree truth
- one installed executable, not a primary binary plus a helper-binary shadow surface
- one production discovery topology
- one explicit dev override topology
- one semantically accurate installed resource root, `resources/**`

### Explicit-over-clever requirements

- keep `generate.sh` and `install.sh` straightforward shell scripts
- prefer exact file-set assertions over "contains enough files" checks
- prefer removing ambiguous helpers like `relink.sh` over preserving them with unclear semantics
- prefer rendering installed `SKILL.md` files directly from templates instead of depending on intermediate repo-root copies

### Diagram maintenance requirement

Any nearby ASCII diagrams or docs that describe the old heavier install shape must be updated in the same milestone. Stale topology diagrams are a bug.

## Performance Review

This is not a throughput-sensitive milestone. The real risks are correctness, drift, and install determinism.

Performance constraints that still matter:

- `generate.sh` stays O(number of projected skill files), which is tiny
- `install.sh` stays stage-then-swap so repeated installs remain predictable
- smoke assertions should stay exact and small, not recursive repo-wide content scans

## NOT in Scope

- changing `system doctor --json`
- changing the semantic output of the existing charter authoring flow
- adding more Codex-discoverable skills
- adding `.claude/skills/*` generation
- adding release automation or package-manager distribution
- turning `~/system/resources/**` into a dumping ground for arbitrary runtime assets

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Source migration | `install/`, repo-root skill source removal, generator input paths | — |
| Thin projection rewrite | `tools/codex/generate.sh`, repo `.agents/skills/*` contract | Source migration |
| Installed-home rewrite | `tools/codex/install.sh`, `tools/codex/dev-setup.sh`, `tools/codex/relink.sh`, `tools/codex/runtime/**` | Thin projection rewrite |
| Runtime invocation rewrite | `install/system-home/charter-intake/`, live skill behavior, runtime path references | Installed-home rewrite |
| Docs and contract cutover | `README.md`, `docs/`, `DESIGN.md`, contract docs | Source migration |
| Smoke rewrite | `tools/ci/*` | Thin projection rewrite + installed-home rewrite + runtime invocation rewrite |

### Parallel lanes

- Lane A: source migration -> thin projection rewrite
- Lane B: installed-home rewrite -> runtime invocation rewrite
- Lane C: docs and contract cutover
- Lane D: smoke rewrite

### Execution order

1. Launch Lane A first. It freezes the canonical source layout and projection contract.
2. Launch Lane B after Lane A lands. Installer and runtime behavior depend on the new projection contract.
3. Launch Lane C after Lane A lands. Docs can move in parallel with Lane B once the source layout is locked.
4. Launch Lane D after Lane B lands. Smoke rails must validate the final installed/runtime behavior, not an in-between state.

### Conflict flags

- Lane B and Lane D both touch packaging behavior assumptions. Keep them sequential.
- Lane A and Lane C both define public wording around source-of-truth paths. Lane C must follow Lane A's exact shape, not invent new names.
- If `relink.sh` is kept instead of deleted, it conflicts conceptually with both Lane B and Lane C because its semantics must be restated everywhere.

## Acceptance Checklist

- [ ] authored install-home truth exists only under `install/system-home/`
- [ ] repo root no longer owns install-home authored files
- [ ] repo `.agents/skills/system*` stays thin
- [ ] `generate.sh` reads only `install/system-home/` and writes only repo projections
- [ ] `install.sh` installs only the curated `~/system/` shape
- [ ] `~/system/bin/system` is the only installed executable for the Codex skill surface
- [ ] `~/system/bin/system-charter-intake` does not exist
- [ ] `~/system/runtime-manifest.json` exists
- [ ] `~/system/resources/` exists
- [ ] `~/system/share/` does not exist
- [ ] `~/.codex/skills/system*` points into `~/system/.agents/skills/*` after normal install
- [ ] dev setup can temporarily point discovery to repo `.agents/skills/*`
- [ ] normal install restores the installed-home discovery topology
- [ ] live skill flow calls `~/system/bin/system` directly
- [ ] no run evidence lands under `~/system/` or `~/.codex/skills/`
- [ ] docs match the new topology exactly
- [ ] install and live smokes fail on any topology drift

## Completion Summary

- Step 0: Scope Challenge, complete. Scope is accepted as-is because partial correction would preserve dual truth.
- Architecture Review: one authored source subtree, one installed binary, one thin discovery model.
- Code Quality Review: repo-root install-home ownership and helper-binary duplication are the main DRY violations to remove.
- Test Review: full packaging and live-skill coverage is defined above.
- Performance Review: no throughput concerns, correctness-first milestone.
- NOT in scope: written.
- What already exists: written.
- Failure modes: defined and mapped to smoke enforcement.
- Parallelization: 4 lanes, with source migration first and smoke rewrite last.
- Lake Score: 7/7. This plan chooses the complete option everywhere the old heavier shape could linger.
