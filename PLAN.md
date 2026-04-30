<!-- /autoplan restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-system/feat-m8-autoplan-restore-20260429-164522.md -->
# PLAN

## Status

This is the implementation plan for `M10` on branch `feat/m8`.

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
  bin/
    ... repo-owned install/build helper scripts only ...
  tools/codex/
    generate.sh
    install.sh
    dev-setup.sh
    relink.sh
```

Rules:

- root `SKILL.md.tmpl` is the authored source for the root host skill
- `charter-intake/SKILL.md.tmpl` is the authored source for the leaf discoverable skill
- repo `SKILL.md` siblings are generated/projected companions to those templates, matching the local `~/gstack` pattern
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
  bin/
    system
    system-update
    system-uninstall
    ... any small product-owned helper scripts needed by the install ...
  charter-intake/
    SKILL.md.tmpl
    SKILL.md
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
- `~/system/bin/system` is the executable the generated skills use
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
| root skill authored shape | local `~/gstack/SKILL.md.tmpl`, `~/gstack/SKILL.md`, `~/gstack/agents/openai.yaml` | mirror the pattern |
| leaf skill authored shape | local `~/gstack/<skill>/SKILL.md.tmpl`, `SKILL.md` | mirror the pattern |
| generated skill projection shape | local `~/gstack/.agents/skills/gstack*` | mirror the pattern |
| current wrong runtime-root assumption | `.agents/skills/system/**` and `~/.codex/skills/system/**` | remove as the primary installed-home model |
| executable | installed `system` CLI binary | install into `~/system/bin/system` |
| mutable run evidence contract | `~/.local/state/system/intake/runs/` | preserve |

### Minimum change set

The minimum honest implementation is:

1. define the correct `~/system/` home shape
2. make generation produce thin `.agents/skills/*` trees instead of runtime-payload roots
3. make install build `~/system/` as the real home
4. make `~/.codex/skills/*` point into that installed home
5. update docs and smoke rails to assert that exact topology

Anything less leaves the `M9.5` mistake in place.

### Complexity check

This plan touches many files, but the blast radius is still bounded:

- packaging scripts
- generated skill outputs
- install topology
- smoke rails
- docs/contracts

No new crates or infrastructure are needed.

### Completeness check

The complete version is still cheap here. If we leave `~/system/` out and keep pretending the installed home is `~/.codex/skills/system/`, we will need another cleanup immediately after this one.

## Architecture Review

### Current to target

```text
M9.5 LANDED
===========
tools/codex/* authored truth
        |
        v
.agents/skills/system*  (too heavy)
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
                           - bin/system
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
| generated skill projection | `.agents/skills/*` in repo and installed home | treating it as the runtime payload root |
| installed executable | `~/system/bin/system` | treating `~/.codex/skills/system/` as the real executable home |
| Codex discovery | `~/.codex/skills/*` thin links | heavy copied runtime trees as the installed-home model |
| mutable run evidence | `~/.local/state/system/intake/runs/*` | run artifacts under `~/system/` or `~/.codex/skills/` |

### Root host skill contract

The root host skill stays, but it is not the main workflow. Its job is:

- family-level root skill metadata
- shared agent metadata
- install anchor for the `system` family
- reference point for the installed home

### Leaf skill contract

The discoverable workflow remains `system-charter-intake`.

The generated leaf skill under both repo-local and installed `.agents/skills/` must:

1. resolve `repo_root` with `git rev-parse --show-toplevel`
2. resolve executable home in this order:
   - repo-local dev install if intentionally configured
   - otherwise `~/system/bin/system`
3. use the real `system` executable from the installed home, not an inline runtime wrapper
4. keep `system doctor --json` as the only machine-parsed output
5. keep run evidence under `~/.local/state/system/intake/runs/<timestamp-pid>/`

### Installed-home contract

`~/system/` is the product-owned install root. It must be able to answer:

1. what executable is being used
2. how to update it
3. how to uninstall it
4. what generated skill projections belong to this install

That is why `~/system/` exists. It is not just a passive runtime asset folder.

### Codex discovery contract

`~/.codex/skills/*` should be as thin as possible.

After `M10`, the preferred shape is:

- `~/.codex/skills/system` links to `~/system/.agents/skills/system`
- `~/.codex/skills/system-charter-intake` links to `~/system/.agents/skills/system-charter-intake`

This keeps the product home and the Codex discovery layer distinct.

## Code Quality Review

### Required design rules

- `tools/codex/` becomes tooling only
- `.agents/skills/*` is always thin
- `~/system/` is the installed home
- `~/.codex/skills/*` is discovery glue only
- no repo clone is installed into `~/system/`
- no mutable run evidence under `~/system/`
- no mutable run evidence under `~/.codex/skills/`
- no generic host abstraction layer
- no new crate

### Directory hygiene

After `M10`, a contributor should be able to answer these questions instantly:

1. Where do I edit the authored root skill?
2. Where do I edit the authored leaf skill?
3. Where is the real installed home?
4. Where does Codex discover the skills?
5. Where do run artifacts go?

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

### Step 2: Rewire generation

Update `tools/codex/generate.sh` so it:

- renders repo-local `SKILL.md` siblings from the templates where needed
- generates thin repo-local `.agents/skills/system*`
- generates thin installed-home `.agents/skills/system*` under `~/system/.agents/skills/`
- never treats `.agents/skills/system/` as a runtime payload root

### Step 3: Build the real installed home

Update the install flow so it creates and maintains:

```text
~/system/
```

with:

- root skill files
- leaf skill files
- `bin/system`
- update/uninstall helpers
- installed `.agents/skills/system*`

This is the main `M10` landing change.

### Step 4: Make Codex discovery thin

Update install/dev flows so:

- `~/.codex/skills/system` points to `~/system/.agents/skills/system`
- `~/.codex/skills/system-charter-intake` points to `~/system/.agents/skills/system-charter-intake`

If the environment prevents symlink use, the fallback may be a thin copy, but the design target is links.

### Step 5: Preserve state and runtime behavior

Keep:

- `system doctor --json`
- `system author charter --validate --from-inputs`
- `system author charter --from-inputs`
- existing-charter refusal
- outside-git-repo refusal
- run evidence under `~/.local/state/system/intake/runs/`

### Step 6: Update docs and smoke rails

Update:

- `README.md`
- `DESIGN.md`
- `docs/SUPPORTED_COMMANDS.md`
- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
- `tools/ci/install-smoke.sh`
- `tools/ci/codex-skill-live-smoke.sh`

The docs must stop describing `~/.codex/skills/system/` as the installed home.

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
[+] repo authored shape
    |
    ├-- [REQ] root SKILL.md.tmpl is canonical
    ├-- [REQ] root SKILL.md sibling exists in the correct projected shape
    ├-- [REQ] leaf charter-intake/SKILL.md.tmpl is canonical
    └-- [REQ] leaf charter-intake/SKILL.md sibling exists in the correct projected shape

[+] generated thin projections
    |
    ├-- [REQ] repo .agents/skills/system* is thin
    ├-- [REQ] ~/system/.agents/skills/system* is thin
    └-- [REQ] no runtime-payload tree is hidden inside .agents/skills/system/

[+] installed home
    |
    ├-- [REQ] ~/system/ exists after install
    ├-- [REQ] ~/system/bin/system exists
    ├-- [REQ] update/uninstall helpers exist
    └-- [REQ] root and leaf skill files exist under ~/system/

[+] Codex discovery layer
    |
    ├-- [REQ] ~/.codex/skills/system points to ~/system/.agents/skills/system
    └-- [REQ] ~/.codex/skills/system-charter-intake points to ~/system/.agents/skills/system-charter-intake

[+] runtime behavior
    |
    ├-- [REQ] happy path still passes
    ├-- [REQ] existing-charter refusal still passes
    ├-- [REQ] outside-git-repo refusal still passes
    └-- [REQ] run-evidence still lands under ~/.local/state/system/intake/runs/
```

### Required tests and assertions

1. install smoke must assert that `~/system/` is created
2. install smoke must assert that `~/system/bin/system` exists
3. install smoke must assert that `~/system/.agents/skills/system*` exists
4. install smoke must assert that `~/.codex/skills/system*` points into `~/system/.agents/skills/*`
5. generation smoke must assert that repo `.agents/skills/system*` is thin and does not carry the old runtime-root payload tree
6. live smoke must continue to prove the happy path and refusal behavior
7. live smoke must assert that run evidence still lands under `~/.local/state/system/intake/runs/`

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

| Failure mode | Test covers it? | Expected handling |
| --- | --- | --- |
| installer still treats `~/.codex/skills/system/` as the installed home | yes, install smoke | fail smoke |
| `~/system/` is missing after install | yes, install smoke | fail smoke |
| `~/system/` is a copied repo clone instead of a curated install | yes, install smoke and file-set assertions | fail smoke |
| `.agents/skills/system/` still contains the old runtime-payload tree | yes, generate/install smoke | fail smoke |
| `~/.codex/skills/*` duplicates heavy content instead of linking into `~/system/.agents/skills/*` | yes, install smoke | fail smoke |
| `~/system/bin/system` missing or wrong | yes, install and live smoke | fail smoke |
| run evidence lands under `~/system/` or `~/.codex/skills/` | yes, live smoke | fail smoke |
| docs still claim `~/.codex/skills/system/` is the installed home | yes, doc review | patch before merge |

There should be no silent failure path in `M10`. The corrected install topology is the milestone.

## Performance Review

`M10` is packaging-heavy, not CPU-heavy.

Acceptable cost:

- an explicit install buildout for `~/system/`
- thin generated skill projections
- thin Codex discovery links

Unacceptable cost:

- cloning the repo into `~/system/`
- using `~/.codex/skills/` as the de facto product home
- hiding mutable artifacts in the installed home

## Docs And Contract Updates

Docs and contracts must say one coherent thing after the change:

- repo owns authored skill truth
- repo `.agents/skills/*` is generated dev projection
- `~/system/` is the installed product home
- `~/.codex/skills/*` is the Codex discovery layer
- `~/.local/state/system/intake/runs/*` is mutable evidence state

Required contract updates:

- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
- any doc that still describes `~/.codex/skills/system/` as the installed home
- any doc that implies `.agents/skills/system/` is the runtime payload root

## Worktree Parallelization Strategy

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Canonical source layout | repo root skill files, `charter-intake/`, `agents/` | — |
| Generator rewrite | `tools/codex/`, repo `.agents/skills/`, installed-home `.agents/skills/` | Canonical source layout |
| Installed-home topology | `tools/codex/install.sh`, `tools/codex/dev-setup.sh`, `tools/codex/relink.sh` | Generator rewrite |
| Smoke and conformance updates | `tools/ci/`, docs/contracts | Installed-home topology |
| Doc surface cleanup | `README.md`, `DESIGN.md`, `docs/` | Canonical source layout |

### Parallel lanes

- Lane A: Canonical source layout -> Generator rewrite
- Lane B: Installed-home topology
- Lane C: Doc surface cleanup
- Lane D: Smoke and conformance updates

### Execution order

Launch Lane A first.

Then:

- launch Lane B and Lane C in parallel once source paths are frozen
- launch Lane D after Lane B freezes the final install topology

### Conflict flags

- Lane B and Lane D both depend on the exact `~/system/` file set
- Lane C and Lane D both touch packaging language
- Lane A must freeze the final authored-source paths before the others proceed

## Acceptance Criteria

`M10` is done only when all of this is true:

1. repo root contains the canonical root skill source as `SKILL.md.tmpl`
2. `charter-intake/SKILL.md.tmpl` is the canonical leaf skill source
3. repo root and leaf generated `SKILL.md` siblings exist in the correct `gstack`-style shape
4. repo `.agents/skills/system*` regenerates as a thin dev projection
5. `~/system/` is created as the installed home
6. `~/system/` is curated and install-owned, not a repo clone
7. `~/system/bin/system` exists
8. `~/system/.agents/skills/system*` exists
9. `~/.codex/skills/system*` is a thin discovery layer pointing into `~/system/.agents/skills/*`
10. no mutable run evidence lands under `~/system/` or `~/.codex/skills/`
11. `cargo test --workspace`, `tools/ci/install-smoke.sh`, and `tools/ci/codex-skill-live-smoke.sh` pass
12. docs no longer describe `~/.codex/skills/system/` as the installed home
13. the shipped `M9.5` operator-visible happy path and refusal behavior remain unchanged

## Completion Summary

- Step 0: Scope Challenge, accepted as install-topology correction work
- Architecture Review: `~/system/` established as the real installed home, `.agents/skills/*` narrowed to thin projections, `~/.codex/skills/*` narrowed to thin discovery links
- Code Quality Review: `tools/codex/` narrowed to tooling only, repo-clone install explicitly rejected
- Test Review: coverage tightened around installed-home topology, thin-projection truth, and run-state separation
- Performance Review: no heavy runtime indirection allowed
- NOT in scope: written
- What already exists: written
- Failure modes: written
- Parallelization: 4 lanes, A first, then B + C, then D

This plan is ready to implement as written.
