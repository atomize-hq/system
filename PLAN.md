<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-system/feat-m10-plan-solidify-restore-20260430-111534.md -->
# PLAN

## Status

This is the implementation plan for `M10` on branch `feat/m10`.

`M9.5` already shipped and passed verification. `M10` is the correction pass for the parts of `M9.5` that landed with the wrong packaging topology.

The key correction is simple:

- `~/.codex/skills/system*` is **not** the real installed home
- `.agents/skills/system*` is **not** the real runtime root
- `~/system/` **is** the real installed home

`~/system/` is not a repo clone. It is an intentionally installed, curated product-owned home that stores the `system` utilities, helper scripts, generated skill artifacts, and admin/update/uninstall tools needed for the Codex-facing install surface.

## Checkpoint Resume

Recovered from `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/checkpoints/20260429-142427-m95-codex-packaging-landed-m10-gstack-parity-gap.md`.

Locked facts from that checkpoint:

- `M9.5` landed and is usable.
- `M9.5` got the install/discovery wedge working.
- `M9.5` did **not** land the correct source-of-truth or installed-home shape.
- `gstack` parity means repo-owned authored sources, generated `.agents/skills` projections, and a real root host skill pattern.
- the root host skill stays in scope for `system`

## What M9.5 Got Wrong

`M9.5` landed with three wrong assumptions:

1. it treated `tools/codex/` as the practical authored home for packaging
2. it treated `.agents/skills/system/` as a packaged runtime root instead of a thin generated skill projection
3. it treated `~/.codex/skills/system/` as the installed runtime home instead of making `~/system/` the real installed home

That is the gap `M10` exists to close.

## Locked Direction

These points are not up for reinterpretation inside `M10`:

1. Match the `gstack` type of authored-source and generated-skill shape.
2. Keep the root host skill pattern.
3. Keep the discoverable leaf skill `system-charter-intake`.
4. `~/system/` is the actual installed home.
5. `~/system/` is a curated install, not a clone of `/Users/spensermcconnell/__Active_Code/system`.
6. `~/.codex/skills/system*` becomes a thin Codex discovery layer, not the primary installed home.
7. Preserve the shipped `M9.5` operator-visible happy path and refusal behavior unless a change is required by the corrected install topology.
8. Keep mutable run evidence under `~/.local/state/system/intake/runs/`, not under `~/system/` and not under `~/.codex/skills/`.
9. Do not reopen earlier reviewed reduced-v1 pipeline boundaries during `M10`; the M1 activation clause shape remains boolean-only in the form `variables.<name> == true|false`.

## Objective

Land the actual four-layer shape:

1. repo authored source
2. repo-local generated `.agents/skills/` for dev
3. installed `~/system/` home
4. thin `~/.codex/skills/` discovery links into the installed home

The result should be boring:

- one place to author skill truth
- one place to install product-owned runtime/admin tooling
- one thin generated skill tree
- one thin discovery layer for Codex
- no confusion about which directory is canonical, generated, or installed

## Exact Target Shape

### 1. Repo authored source

The repository keeps the authored source and generation logic. The important shape is:

```text
/Users/spensermcconnell/__Active_Code/system/
  SKILL.md.tmpl
  SKILL.md
  agents/
    openai.yaml
  charter-intake/
    SKILL.md.tmpl
    SKILL.md
  core/
    library/
      authoring/
        charter_authoring_method.md
      charter/
        CHARTER_INPUTS.yaml.tmpl
        charter_inputs_directive.md
  tools/codex/
    generate.sh
    install.sh
    dev-setup.sh
    relink.sh
    runtime/
      runtime-manifest.json.tmpl
      bin/
        system-charter-intake.tmpl
```

Rules:

- root `SKILL.md.tmpl` is the authored source for the root host skill
- `charter-intake/SKILL.md.tmpl` is the authored source for the leaf discoverable skill
- repo `SKILL.md` siblings are generated/projected companions to those templates, matching the local `~/gstack` pattern
- `agents/openai.yaml` is canonical authored agent metadata for both repo and installed projections
- `tools/codex/runtime/runtime-manifest.json.tmpl` stays the canonical manifest source unless `M10` explicitly promotes it elsewhere
- `tools/codex/runtime/bin/system-charter-intake.tmpl` stays the canonical helper-wrapper source unless `M10` explicitly removes the wrapper and replaces its behavior
- `core/library/authoring/**` and `core/library/charter/**` remain canonical shared content sources copied into the installed home
- `tools/codex/` is tooling only after `M10`

### 2. Repo-local generated dev projection

This is for working on the repo locally. It is not the installed home.

```text
/Users/spensermcconnell/__Active_Code/system/.agents/skills/
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

- this tree is generated
- it stays ignored build output
- it is thin, like `~/gstack/.agents/skills/*`
- it does **not** carry the installed runtime payload tree

### 3. Installed home

This is the real installed `system` home and the main correction to the `M9.5` model.

```text
~/system/
  SKILL.md.tmpl
  SKILL.md
  agents/
    openai.yaml
  runtime-manifest.json
  bin/
    system
    system-update
    system-uninstall
    system-charter-intake
  charter-intake/
    SKILL.md.tmpl
    SKILL.md
  share/
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

- `~/system/` is intentionally installed and curated
- `~/system/` is not a git checkout
- `~/system/bin/system` is the canonical executable the generated skills and helper scripts use
- `~/system/bin/system-charter-intake` is allowed only as a thin helper that shells through `~/system/bin/system`; it is not a second runtime root
- `~/system/runtime-manifest.json` and `~/system/share/**` are part of the installed runtime contract
- `~/system/.agents/skills/*` is the installed generated skill tree
- install/update/uninstall utilities belong here
- product-owned generated artifacts that support the installed Codex surface belong here
- mutable run evidence does **not** belong here

### 4. Thin Codex discovery layer

```text
~/.codex/skills/
  system -> ~/system/.agents/skills/system
  system-charter-intake -> ~/system/.agents/skills/system-charter-intake
```

Rules:

- this layer is thin
- it should link to the installed `~/system/.agents/skills/*` tree instead of duplicating it
- it is discovery glue for Codex, not the product home

### Mutable run evidence

```text
~/.local/state/system/
  intake/
    runs/
      <timestamp-pid>/
        ...
```

Rules:

- all run evidence stays here
- no run evidence under `~/system/`
- no run evidence under `~/.codex/skills/`

## Scope Lock

### In scope

- correct the plan to target `~/system/` as the installed home
- make repo-root and leaf `SKILL.md.tmpl` sources canonical
- make repo-root and leaf `SKILL.md` siblings part of the projected/installable shape
- make `.agents/skills/*` thin generated projections
- make `~/.codex/skills/*` thin discovery links into `~/system/.agents/skills/*`
- install `~/system/bin/system` as the executable used by the generated skills
- install update/uninstall helpers into `~/system/bin/`
- update smoke rails and docs so they describe the real shape instead of the `M9.5` approximation

### NOT in scope

- changing `system doctor --json`
- changing `system author charter --validate --from-inputs`
- changing `system author charter --from-inputs`
- adding more discoverable skills
- adding `.claude/skills/` generation
- adding a generic multi-host packaging framework
- public release packaging or registry distribution
- moving mutable run evidence into `~/system/`

## Step 0: Scope Challenge

### What already exists

| Sub-problem | Existing code or asset | `M10` decision |
| --- | --- | --- |
| generator entrypoint | `tools/codex/generate.sh` | keep, but retarget outputs and ownership model |
| install flow | `tools/codex/install.sh` | keep, but make it install `~/system/` plus Codex links |
| dev symlink flow | `tools/codex/dev-setup.sh`, `tools/codex/relink.sh` | keep |
| root skill authored shape | local `~/gstack/SKILL.md.tmpl`, `~/gstack/SKILL.md`, `~/gstack/agents/openai.yaml` | mirror the pattern at repo root |
| leaf skill authored shape | local `~/gstack/<skill>/SKILL.md.tmpl`, `SKILL.md` | mirror the pattern under `charter-intake/` |
| current authored text hiding under tooling paths | `tools/codex/runtime/SKILL.md.tmpl`, `tools/codex/templates/system-charter-intake.SKILL.md.tmpl` | move skill-truth ownership to repo root and `charter-intake/`; leave tooling-only templates only for helper/runtime metadata that still belongs under `tools/codex/` |
| runtime manifest source | `tools/codex/runtime/runtime-manifest.json.tmpl` | keep as the manifest source unless `M10` intentionally promotes it |
| helper-wrapper source | `tools/codex/runtime/bin/system-charter-intake.tmpl` | keep behavior, but retarget it to `~/system/` and stop treating `.agents/skills/system/` as its runtime root |
| shared runtime content | `core/library/authoring/charter_authoring_method.md`, `core/library/charter/*` | keep canonical here, copy into `~/system/share/**` |
| generated skill projection shape | local `~/gstack/.agents/skills/gstack*` | mirror the pattern |
| current wrong runtime-root assumption | `.agents/skills/system/**` and `~/.codex/skills/system/**` | remove as the primary installed-home model |
| executable | installed `system` CLI binary | install into `~/system/bin/system` |
| current packaging smoke rails | `tools/ci/install-smoke.sh`, `tools/ci/codex-skill-live-smoke.sh` | keep, but rewrite assertions around `~/system/`, thin projections, and discovery links |
| mutable run evidence contract | `~/.local/state/system/intake/runs/` | preserve |

### Minimum change set

The minimum honest implementation is:

1. define the correct `~/system/` home shape, including binary, manifest, helper, share payload, and installed thin projections
2. move skill authored truth out of `tools/codex/runtime/` and `tools/codex/templates/` into repo-root `SKILL.md.tmpl` and `charter-intake/SKILL.md.tmpl`
3. make generation produce repo-local thin `.agents/skills/*` trees and generated repo `SKILL.md` siblings, but never mutate `$HOME`
4. make install build `~/system/` as the only normal installed home and install `~/system/bin/system`
5. make `~/.codex/skills/*` point into that installed home during normal install, while keeping explicit dev-only override behavior isolated to `dev-setup.sh`
6. update docs, contracts, and smoke rails to assert that exact topology and fail if repo projections become heavy again

Anything less leaves the `M9.5` mistake in place.

### Complexity check

This plan touches more than 8 files, so it does trip the smell threshold. The reason it is still acceptable is that the work stays inside one packaging seam and does not add new runtime concepts.

The real blast radius is:

- packaging scripts under `tools/codex/`
- generated skill outputs under repo `.agents/skills/`
- installed-home topology under `~/system/`
- smoke rails under `tools/ci/`
- docs/contracts that still describe `~/.codex/skills/` as the installed home

No new crates or infrastructure are needed.

The scope reduction rule for `M10` is simple: no new host abstraction, no new packaging framework, no new machine-readable authoring contract. This is a topology correction, not a platform rewrite.

### Completeness check

The complete version is still cheap here. If we leave `~/system/` out, or if we fix the directories without fixing the install/update/uninstall contract and smoke rails, we will need another cleanup immediately after this one.

The complete version for `M10` includes:

- a full source-of-truth map for skill text, manifest text, helper wrapper text, and shared authoring assets
- an explicit rule for how `~/system/bin/system` appears
- an explicit rule for what Codex actually executes after the topology cutover
- smoke rails that fail on both wrong install shape and wrong repo-generated shape

## Architecture Review

### Current to target

```text
M9.5 LANDED
===========
tools/codex/runtime/* and tools/codex/templates/* hold practical authored truth
        |
        v
.agents/skills/system*  (too heavy, doubles as runtime payload)
        |
        v
~/.codex/skills/system* (incorrectly treated as installed home)

M10 TARGET
==========
repo authored source
  - SKILL.md.tmpl
  - SKILL.md
  - agents/openai.yaml
  - charter-intake/SKILL.md.tmpl
  - charter-intake/SKILL.md
  - tools/codex/runtime/runtime-manifest.json.tmpl
  - tools/codex/runtime/bin/system-charter-intake.tmpl
  - core/library/authoring/**
  - core/library/charter/**
        |
        v
repo .agents/skills/system*      (thin dev projection)
        |
        +----------------------+
                               |
                               v
                         ~/system/
                           - root skill source/projection
                           - charter-intake source/projection
                           - runtime-manifest.json
                           - bin/system
                           - bin/system-charter-intake
                           - share/authoring/**
                           - share/charter/**
                           - update/uninstall helpers
                           - installed .agents/skills/system*
                               |
                               v
                     ~/.codex/skills/system*
                     (thin links into ~/system/.agents/skills/*)
```

### Ownership contract

| Concern | Authoritative location after `M10` | Not allowed after `M10` |
| --- | --- | --- |
| root skill authored text | repo `SKILL.md.tmpl` | authored root skill text in `tools/codex/runtime/` |
| leaf skill authored text | repo `charter-intake/SKILL.md.tmpl` | authored leaf text in `tools/codex/templates/` |
| root skill generated sibling | repo `SKILL.md` and installed `~/system/SKILL.md` | source-of-truth hidden only inside generator |
| leaf skill generated sibling | repo `charter-intake/SKILL.md` and installed `~/system/charter-intake/SKILL.md` | source-of-truth hidden only inside generator |
| root and leaf agent metadata | repo `agents/openai.yaml`, copied into installed home and generated thin projections | agent metadata authored separately inside generated `.agents/skills/**` trees |
| runtime manifest text | `tools/codex/runtime/runtime-manifest.json.tmpl`, rendered to `~/system/runtime-manifest.json` | manifest treated as discovered state inside `.agents/skills/system/` |
| helper-wrapper text | `tools/codex/runtime/bin/system-charter-intake.tmpl`, rendered to `~/system/bin/system-charter-intake` if the helper remains | wrapper hidden under generated `.agents/skills/system/bin/` as the runtime root |
| shared authoring payload | `core/library/authoring/**` and `core/library/charter/**`, copied to `~/system/share/**` | canonical copies living only under generated runtime payload trees |
| generated skill projection | `.agents/skills/*` in repo and installed home | treating it as the runtime payload root |
| installed executable | `~/system/bin/system` | treating `~/.codex/skills/system/` as the real executable home |
| install mutation of `$HOME` | `tools/codex/install.sh` and explicit admin helpers only | `tools/codex/generate.sh` mutating `~/system/` or `~/.codex/skills/` |
| Codex discovery | `~/.codex/skills/*` thin links | heavy copied runtime trees as the installed-home model |
| mutable run evidence | `~/.local/state/system/intake/runs/*` | run artifacts under `~/system/` or `~/.codex/skills/` |

### Root host skill contract

The root host skill stays, but it is not the main workflow. Its job is:

- family-level root skill metadata
- shared agent metadata
- install anchor for the `system` family
- reference point for the installed home

The root generated skill under both repo-local and installed `.agents/skills/system/` stays thin:

- `SKILL.md`
- `agents/openai.yaml`

It must not carry `bin/`, `share/`, or `runtime-manifest.json`.

### Leaf skill contract

The discoverable workflow remains `system-charter-intake`.

The generated leaf skill under both repo-local and installed `.agents/skills/` must stay thin in filesystem shape, but it still needs a fully specified execution contract.

The execution contract is:

1. resolve `repo_root` with `git rev-parse --show-toplevel`
2. in normal install mode, treat `~/system/` as `SYSTEM_HOME`
3. in explicit dev mode only, allow `dev-setup.sh` to wire Codex directly to repo-generated thin skills, but the skill must still resolve a repo-owned helper or home contract without treating repo `.agents/skills/system/` as a runtime payload root
4. use `~/system/bin/system` as the canonical executable
5. if `M10` keeps `~/system/bin/system-charter-intake`, that helper is allowed only as a thin orchestrator for the existing doctor/setup/validate/write flow and must shell through `~/system/bin/system`
6. keep `system doctor --json` as the only machine-parsed output
7. keep validate/write proof on exit codes plus persisted stdout/stderr transcripts
8. keep run evidence under `~/.local/state/system/intake/runs/<timestamp-pid>/`

### Installed-home contract

`~/system/` is the product-owned install root. It must be able to answer:

1. what executable is being used
2. how to update it
3. how to uninstall it
4. what generated skill projections belong to this install

That is why `~/system/` exists. It is not just a passive runtime asset folder.

The file-set contract for `~/system/` is:

- root skill authored/projection companions
- leaf skill authored/projection companions
- `agents/openai.yaml`
- `runtime-manifest.json`
- `bin/system`
- `bin/system-update`
- `bin/system-uninstall`
- `bin/system-charter-intake` only if the helper wrapper remains
- `share/authoring/charter_authoring_method.md`
- `share/charter/CHARTER_INPUTS.yaml.tmpl`
- `share/charter/charter_inputs_directive.md`
- `.agents/skills/system/**` and `.agents/skills/system-charter-intake/**` as thin generated projections

Normal install is idempotent and copy-based. It may stage into a temporary directory and swap into place, but it must never leave a half-written `~/system/` tree behind.

The install contract for `~/system/bin/system` is explicit:

- `install.sh` does **not** compile Rust
- `install.sh` must require `system` to already exist on `PATH`
- `install.sh` must verify the discovered binary version matches repo `VERSION`
- `install.sh` must copy that binary into `~/system/bin/system`
- install must refuse loudly if the binary is missing or mismatched

`system-update` may stay local-source-only in `M10`, but it must be honest: re-run the local install flow from an operator-provided repo checkout or refuse with exact guidance. It must not pretend public distribution exists.

### Codex discovery contract

`~/.codex/skills/*` should be as thin as possible.

After `M10`, the preferred shape is:

- `~/.codex/skills/system` links to `~/system/.agents/skills/system`
- `~/.codex/skills/system-charter-intake` links to `~/system/.agents/skills/system-charter-intake`

This keeps the product home and the Codex discovery layer distinct.

Normal install should use symlinks on supported targets. A thin copy fallback is acceptable only as a defensive fallback for unsupported filesystems; it is not the primary design target for `macOS arm64` or `Linux x86_64`.

`dev-setup.sh` is the only allowed exception to the normal discovery contract. It may wire Codex directly to repo-generated thin skills for local iteration, but `install.sh` must always restore the normal `~/.codex/skills/* -> ~/system/.agents/skills/*` topology.

## Code Quality Review

### Required design rules

- `tools/codex/` becomes tooling only
- `.agents/skills/*` is always thin
- `~/system/` is the installed home
- `~/.codex/skills/*` is discovery glue only
- no repo clone is installed into `~/system/`
- no mutable run evidence under `~/system/`
- no mutable run evidence under `~/.codex/skills/`
- repo `.agents/skills/system/**` and installed `.agents/skills/system/**` may contain only skill-discovery assets, not runtime payload
- runtime payload belongs under `~/system/`, not under `.agents/skills/**`
- `generate.sh` may mutate repo outputs only; `install.sh` owns all `$HOME` mutation
- no generic host abstraction layer
- no new crate

### Directory hygiene

After `M10`, a contributor should be able to answer these questions instantly:

1. Where do I edit the authored root skill?
2. Where do I edit the authored leaf skill?
3. Where is the real installed home?
4. Where does Codex discover the skills?
5. Where do run artifacts go?
6. Where does the runtime manifest come from?
7. Where do shared authoring assets come from?
8. Which script is allowed to touch `$HOME`?

If those answers are ambiguous, the plan failed.

## Implementation Plan

### Step 1: Define canonical repo-authored shape

Add or normalize:

- `SKILL.md.tmpl`
- `SKILL.md`
- `agents/openai.yaml`
- `charter-intake/SKILL.md.tmpl`
- `charter-intake/SKILL.md`

Rules:

- templates are authored truth
- generated sibling `SKILL.md` files match the local `~/gstack` style shape
- root and leaf source live in obvious locations, not hidden inside `tools/codex/`
- root and leaf authored truth moves out of `tools/codex/runtime/SKILL.md.tmpl` and `tools/codex/templates/system-charter-intake.SKILL.md.tmpl`
- `tools/codex/runtime/runtime-manifest.json.tmpl` and `tools/codex/runtime/bin/system-charter-intake.tmpl` stay tooling-owned sources unless explicitly promoted
- `core/library/authoring/**` and `core/library/charter/**` remain shared content sources copied into `~/system/share/**`

### Step 2: Rewire generation

Update `tools/codex/generate.sh` so it:

- renders repo-local `SKILL.md` siblings from the templates where needed
- generates thin repo-local `.agents/skills/system*`
- renders target-specific thin skill projections that know whether they are operating in repo-dev mode or installed-home mode
- never writes to `~/system/` or `~/.codex/skills/`
- never treats `.agents/skills/system/` as a runtime payload root
- emits a repo-generated projection smoke failure if repo `.agents/skills/system/**` regains `bin/`, `runtime-manifest.json`, or `share/`

Ownership boundary:

- `generate.sh` owns repo outputs only
- `install.sh` owns installed-home and Codex discovery outputs only

### Step 3: Build the real installed home

Update the install flow so it creates and maintains:

```text
~/system/
```

with:

- root skill files
- leaf skill files
- `agents/openai.yaml`
- `runtime-manifest.json`
- `bin/system`
- `bin/system-charter-intake` if the helper wrapper remains
- update/uninstall helpers
- `share/authoring/**`
- `share/charter/**`
- installed `.agents/skills/system*`

This is the main `M10` landing change.

Install behavior must be explicit:

- verify `system` exists on `PATH`
- verify the discovered binary version matches repo `VERSION`
- copy that binary into `~/system/bin/system`
- render/copy the helper wrapper, manifest, and shared payload into `~/system/`
- stage and replace prior installs cleanly
- migrate a preexisting heavy `~/.codex/skills/system` install into the new `~/system/` home instead of leaving duplicate truth behind
- handle a preexisting `~/system/` by replacing only the curated install-owned file set
- leave mutable run evidence untouched under `~/.local/state/system/intake/runs/`

### Step 4: Make Codex discovery thin

Update install/dev flows so:

- `~/.codex/skills/system` points to `~/system/.agents/skills/system`
- `~/.codex/skills/system-charter-intake` points to `~/system/.agents/skills/system-charter-intake`

If the environment prevents symlink use, the fallback may be a thin copy, but the design target is links.

Normal install and dev setup are different on purpose:

- `install.sh` restores the real product topology
- `dev-setup.sh` may keep the explicit local-dev override path
- `relink.sh` remains the fast way to reassert that local-dev override
- none of those paths may make repo `.agents/skills/system/` the runtime payload root again

### Step 5: Preserve state and runtime behavior

Keep:

- `system doctor --json`
- `system author charter --validate --from-inputs`
- `system author charter --from-inputs`
- existing-charter refusal
- outside-git-repo refusal
- run evidence under `~/.local/state/system/intake/runs/`

The runtime sequence stays pinned:

1. `system doctor --json`
2. optional `system setup`
3. `system doctor --json` again if setup ran
4. `system author charter --validate --from-inputs`
5. `system author charter --from-inputs`
6. final `system doctor --json`

If `M10` keeps `bin/system-charter-intake`, it must preserve:

- refusal ordering
- session artifact layout
- validate/write transcript capture
- version-manifest guardrail

The correction is topology, not behavior drift.

### Step 6: Update docs and smoke rails

Update:

- `README.md`
- `DESIGN.md`
- `docs/START_HERE.md`
- `docs/SUPPORTED_COMMANDS.md`
- `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
- `tools/ci/install-smoke.sh`
- `tools/ci/codex-skill-live-smoke.sh`

The docs must stop describing `~/.codex/skills/system/` as the installed home.

Search-and-fix rule:

- every surviving reference to installed runtime assets under `~/.codex/skills/` must either be deleted or rewritten as thin discovery wording
- every surviving reference to packaging-only install must be rewritten so `install.sh` clearly owns `~/system/`
- every smoke that still expects a heavy repo `.agents/skills/system/**` tree must be rewritten

## Test Review

### Test framework and proof surface

This repo already has the right proof layers:

- Rust workspace tests through `cargo test --workspace`
- packaging smoke via `tools/ci/install-smoke.sh`
- installed-skill live smoke via `tools/ci/codex-skill-live-smoke.sh`

No new framework is needed.

### Coverage diagram

```text
CODE PATH COVERAGE
==================
[+] repo authored shape and source map
    |
    ├-- [REQ] repo root `SKILL.md.tmpl` is canonical for root skill text
    ├-- [REQ] `charter-intake/SKILL.md.tmpl` is canonical for leaf skill text
    ├-- [REQ] `agents/openai.yaml` is canonical authored agent metadata
    ├-- [REQ] runtime manifest source stays explicit
    ├-- [REQ] helper-wrapper source stays explicit
    └-- [REQ] shared authoring payload source stays explicit

[+] repo generation path (`tools/codex/generate.sh`)
    |
    ├-- [REQ] repo `SKILL.md` siblings regenerate from canonical templates
    ├-- [REQ] repo `.agents/skills/system/**` is thin
    ├-- [REQ] repo `.agents/skills/system-charter-intake/**` is thin
    ├-- [REQ] no runtime-payload tree is hidden inside repo `.agents/skills/system/**`
    └-- [REQ] generator never mutates `$HOME`

[+] installed-home build path (`tools/codex/install.sh`)
    |
    ├-- [REQ] ~/system/ exists after install
    ├-- [REQ] ~/system/bin/system exists
    ├-- [REQ] `~/system/bin/system` version matches repo `VERSION`
    ├-- [REQ] update/uninstall helpers exist
    ├-- [REQ] manifest + share payload exist under `~/system/`
    ├-- [REQ] installed `.agents/skills/system*` is thin
    └-- [REQ] prior heavy discovery installs migrate cleanly

[+] Codex discovery layer
    |
    ├-- [REQ] ~/.codex/skills/system points to ~/system/.agents/skills/system
    └-- [REQ] ~/.codex/skills/system-charter-intake points to ~/system/.agents/skills/system-charter-intake

[+] runtime behavior and evidence
    |
    ├-- [REQ] happy path still passes
    ├-- [REQ] stale-version refusal still passes
    ├-- [REQ] existing-charter refusal still passes
    ├-- [REQ] outside-git-repo refusal still passes
    ├-- [REQ] repo-local dev override still works if `dev-setup.sh` advertises it
    └-- [REQ] run-evidence still lands under `~/.local/state/system/intake/runs/`
```

### Required tests and assertions

1. `tools/ci/install-smoke.sh` must assert that `~/system/` is created
2. `tools/ci/install-smoke.sh` must assert that `~/system/bin/system` exists and version-matches the repo release
3. `tools/ci/install-smoke.sh` must assert the curated installed-home file set, including manifest and `share/**`
4. `tools/ci/install-smoke.sh` must assert that installed `.agents/skills/system*` exists and is thin
5. `tools/ci/install-smoke.sh` must assert that `~/.codex/skills/system*` points into `~/system/.agents/skills/*` after normal install
6. `tools/ci/install-smoke.sh` must assert that running `bash tools/codex/generate.sh` leaves repo `.agents/skills/system/**` thin and free of `bin/`, `runtime-manifest.json`, and `share/`
7. `tools/ci/install-smoke.sh` must assert that reinstall after `dev-setup.sh` returns the system to the normal copy-installed `~/system/` plus discovery-link topology
8. `tools/ci/codex-skill-live-smoke.sh` must continue to prove the happy path and refusal behavior
9. `tools/ci/codex-skill-live-smoke.sh` must assert that the recorded runtime root is `~/system/` in normal install mode, not `~/.codex/skills/system`
10. `tools/ci/codex-skill-live-smoke.sh` must keep the explicit repo-local dev-override smoke only if `dev-setup.sh` continues to advertise that mode
11. live smoke must assert that run evidence still lands under `~/.local/state/system/intake/runs/`

### Verification commands

These stay mandatory:

```bash
cargo fmt --all -- --check
cargo test --workspace
bash tools/codex/generate.sh
cargo install --locked --force --path crates/cli
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh
```

## Failure Modes

| Failure mode | Test covers it? | Expected handling |
| --- | --- | --- |
| installer still treats `~/.codex/skills/system/` as the installed home | yes, install smoke | fail smoke |
| `~/system/` is missing after install | yes, install smoke | fail smoke |
| `~/system/` is a copied repo clone instead of a curated install | yes, install smoke and file-set assertions | fail smoke |
| repo `.agents/skills/system/**` regains `bin/`, `runtime-manifest.json`, or `share/` | yes, generation/install smoke | fail smoke |
| installed `.agents/skills/system/**` regains runtime payload content | yes, install smoke | fail smoke |
| `~/.codex/skills/*` duplicates heavy content instead of linking into `~/system/.agents/skills/*` | yes, install smoke | fail smoke on supported targets |
| `~/system/bin/system` missing, wrong, or version-mismatched | yes, install and live smoke | install refuses loudly |
| helper wrapper still shells to PATH instead of `~/system/bin/system` | yes, live smoke and targeted shell assertion | fail smoke |
| preexisting heavy discovery install is left in place beside `~/system/` | yes, install smoke | fail smoke |
| run evidence lands under `~/system/` or `~/.codex/skills/` | yes, live smoke | fail smoke |
| docs still claim `~/.codex/skills/system/` is the installed home | yes, doc review | patch before merge |

There should be no silent failure path in `M10`. The corrected install topology is the milestone.

## Performance Review

`M10` is packaging-heavy, not CPU-heavy.

Acceptable cost:

- an explicit install buildout for `~/system/`
- thin generated skill projections
- thin Codex discovery links
- one extra binary copy during install
- one explicit version check between repo `VERSION`, PATH `system`, and installed `~/system/bin/system`

Unacceptable cost:

- cloning the repo into `~/system/`
- using `~/.codex/skills/` as the de facto product home
- hiding mutable artifacts in the installed home
- making `generate.sh` mutate `$HOME`
- keeping heavy runtime payload duplicated in both repo `.agents/skills/` and installed home

## Docs And Contract Updates

Docs and contracts must say one coherent thing after the change:

- repo owns authored skill truth
- repo `.agents/skills/*` is generated dev projection
- `~/system/` is the installed product home
- `~/.codex/skills/*` is the Codex discovery layer
- `~/.local/state/system/intake/runs/*` is mutable evidence state
- `install.sh` owns the `~/system/` install mutation
- `generate.sh` owns repo-local projections only

Required contract updates:

- `docs/START_HERE.md`
- `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
- any doc that still describes `~/.codex/skills/system/` as the installed home
- any doc that implies `.agents/skills/system/` is the runtime payload root
- any doc that still says `install.sh` is packaging-only after `M10`

## Worktree Parallelization Strategy

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Canonical source layout | repo root skill files, `charter-intake/`, `agents/`, `tools/codex/runtime/`, `core/library/` | — |
| Generator rewrite | `tools/codex/`, repo `.agents/skills/` | Canonical source layout |
| Installed-home topology | `tools/codex/install.sh`, `tools/codex/dev-setup.sh`, `tools/codex/relink.sh`, `~/system` contract | Canonical source layout, Generator rewrite |
| Smoke and conformance updates | `tools/ci/`, docs/contracts | Installed-home topology |
| Doc surface cleanup | `README.md`, `DESIGN.md`, `docs/`, docs/contracts | Canonical source layout, Installed-home topology contract freeze |

### Parallel lanes

- Lane A: Canonical source layout -> Generator rewrite
- Lane B: Installed-home topology
- Lane C: Doc surface cleanup
- Lane D: Smoke and conformance updates

### Execution order

Launch Lane A first.

Then:

- launch Lane B and Lane C in parallel once source paths and the runtime-entry contract are frozen
- launch Lane D after Lane B freezes the final install topology and Lane C freezes the doc vocabulary for that topology

### Conflict flags

- Lane A and Lane B both touch `tools/codex/`; do not split them across parallel workers until Step 1 is merged
- Lane B and Lane D both depend on the exact `~/system/` file set
- Lane C and Lane D both touch packaging language and smoke expectations
- Lane A must freeze the final authored-source paths and runtime-entry contract before the others proceed

## Acceptance Criteria

`M10` is done only when all of this is true:

1. repo root contains the canonical root skill source as `SKILL.md.tmpl`
2. `charter-intake/SKILL.md.tmpl` is the canonical leaf skill source
3. repo root and leaf generated `SKILL.md` siblings exist in the correct `gstack`-style shape
4. repo `.agents/skills/system*` regenerates as a thin dev projection
5. `~/system/` is created as the installed home
6. `~/system/` is curated and install-owned, not a repo clone
7. `~/system/bin/system` exists
8. `~/system/runtime-manifest.json` and `~/system/share/**` exist in the curated file set
9. `~/system/.agents/skills/system*` exists and is thin
10. if `system-charter-intake` helper remains, it shells through `~/system/bin/system`
11. `~/.codex/skills/system*` is a thin discovery layer pointing into `~/system/.agents/skills/*`
12. no mutable run evidence lands under `~/system/` or `~/.codex/skills/`
13. `cargo test --workspace`, `tools/ci/install-smoke.sh`, and `tools/ci/codex-skill-live-smoke.sh` pass
14. docs no longer describe `~/.codex/skills/system/` as the installed home
15. docs no longer describe `install.sh` as packaging-only
16. the shipped `M9.5` operator-visible happy path and refusal behavior remain unchanged

## Completion Summary

- Step 0: Scope Challenge, accepted as install-topology correction work
- Architecture Review: `~/system/` established as the real installed home, skill authored truth rehomed, runtime payload separated from thin projections, and Codex discovery narrowed to links
- Code Quality Review: `tools/codex/` narrowed to tooling only, generator/install ownership separated cleanly, repo-clone install explicitly rejected
- Test Review: coverage tightened around source-map truth, installed-home topology, thin-projection truth, version-checked binary install, and run-state separation
- Performance Review: no heavy runtime indirection, no duplicate payload trees, no `$HOME` mutation from generation
- NOT in scope: written
- What already exists: written
- Failure modes: written
- Parallelization: 4 lanes, A first, then B + C, then D, with `tools/codex/` conflicts called out explicitly

This plan is ready to implement as written.
