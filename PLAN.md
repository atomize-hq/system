# PLAN

## Status

This is the corrective implementation plan for `M10.5` on branch `feat/m10`.

It replaces the current `M10` plan because the repo still codifies the wrong packaging model. The current tree and smokes still assume:

- repo-root install-home source
- a heavier installed `~/system/` payload than intended
- a second installed helper binary
- direct runtime dependencies on `runtime-manifest.json` and `share/**`

`M10.5` fixes that. No shape improvisation. No fallback interpretation.

The locked sentence for this milestone is:

> We want a gstack-style installed home, but populated from a curated `install/system-home/` source subtree, not from repo root and not by installing the repo itself; `~/system/bin/system` is the only installed executable, and `system-charter-intake` is only a skill/discovery surface, not a second binary.

## Objective

Land the curated installed-home model and remove the repo-clone model completely.

The shipped topology must become:

1. authored install-home source under `install/system-home/`
2. thin generated repo projections under `.agents/skills/*`
3. curated installed product home under `~/system/`
4. thin Codex discovery links under `~/.codex/skills/*`

Nothing else is the installed home.

## Locked Target Shape

### Repo Source Shape

```text
install/system-home/
  SKILL.md.tmpl
  agents/
    openai.yaml
  charter-intake/
    SKILL.md.tmpl
```

Rules:

- `install/system-home/` is the authored source of truth for files that will be installed into `~/system/`.
- `tools/codex/` is tooling only.
- repo `.agents/skills/*` is generated output only, never authored truth.
- repo root must not contain install-home authored files like `SKILL.md.tmpl`, `SKILL.md`, `agents/openai.yaml`, or `charter-intake/SKILL.md.tmpl`.
- shared content may still come from canonical library locations like `core/library/authoring/**` and `core/library/charter/**` if needed to generate installed skill text, but the installer must copy only the curated subset needed for `~/system/`.
- the installer must never install the repo itself.

### Installed Home Shape

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

- `~/system/` is the real installed home.
- `~/system/` is curated and install-owned, not a git checkout.
- `~/system/bin/system` is the only installed executable for this Codex skill surface.
- there must not be a second helper executable like `~/system/bin/system-charter-intake`.
- `system-charter-intake` is a skill/discovery name, not a separate installed binary.
- `~/system/runtime-manifest.json` remains part of the installed runtime contract.
- `~/system/resources/**` is allowed static runtime content with a semantically accurate name.
- `~/system/.agents/skills/*` must stay thin:
  - only `SKILL.md`
  - only `agents/openai.yaml`
  - no `bin/`
  - no `runtime-manifest.json`
  - no `resources/`
- the leaf skill `system-charter-intake` must invoke `~/system/bin/system` directly.

### Codex Discovery Shape

```text
~/.codex/skills/
  system -> ~/system/.agents/skills/system
  system-charter-intake -> ~/system/.agents/skills/system-charter-intake
```

Rules:

- `~/.codex/skills/*` is discovery glue only.
- it is not the installed home.
- it must point into `~/system/.agents/skills/*`.
- normal install must restore this topology.
- dev setup may temporarily point discovery to repo-generated `.agents/skills/*`, but that is override mode only.

### Behavioral Contract

`tools/codex/generate.sh`:

- writes repo outputs only
- generates thin repo `.agents/skills/system*`
- does not mutate `$HOME`
- does not install anything into `~/system/`

`tools/codex/install.sh`:

- owns all install mutation under `$HOME`
- installs only the curated subset into `~/system/`
- does not copy the repo
- requires `system` on `PATH`
- verifies the `system` binary version matches repo `VERSION`
- copies that verified binary to `~/system/bin/system`

### Explicit Non-Goals

We do not want:

- repo-root install-home source files
- `~/system/` as a repo clone
- `~/.codex/skills/system/` as the installed home
- runtime payload under repo `.agents/skills/*`
- runtime payload under `~/.codex/skills/*`
- a second installed helper binary `bin/system-charter-intake`

## Step 0: Scope Challenge

### What already exists

| Surface | Current state | Keep / change |
| --- | --- | --- |
| `tools/codex/generate.sh` | already generates thin repo `.agents/skills/system*` | keep purpose, change source inputs to `install/system-home/`, stop writing repo-root skill files |
| `tools/codex/install.sh` | already owns `~/system/` install mutation and PATH/version gating | keep staging model, shrink installed payload to exact curated file set |
| `tools/codex/dev-setup.sh` | already supports discovery override mode | keep behavior, make it explicitly dev-only |
| `tools/codex/relink.sh` | currently aliases dev setup behavior | keep only if it still means "restore chosen discovery target", otherwise simplify |
| repo `.agents/skills/system*` | already thin | keep thinness invariant |
| current root `SKILL.md*`, `agents/openai.yaml`, `charter-intake/` | currently treated as authored truth at repo root | migrate into `install/system-home/`, then remove root ownership |
| `tools/codex/runtime/runtime-manifest.json.tmpl` | still part of install contract | keep in installed home as explicit runtime metadata |
| `tools/codex/runtime/bin/system-charter-intake.tmpl` | still drives the second helper binary | remove; the leaf skill must call `~/system/bin/system` directly |
| `tools/ci/install-smoke.sh` | already checks topology in isolated home | keep harness, replace assertions with exact M10.5 file set |
| `tools/ci/codex-skill-live-smoke.sh` | already verifies end-to-end skill execution | keep harness, remove wrapper assumptions and rename `share/**` references to `resources/**` |

### Minimum change set

This milestone touches more than 8 files. That is not scope creep here, it is the blast radius of one packaging contract spread across scripts, generated skill surfaces, docs, and smokes.

The minimum viable complete fix is:

1. move authored install-home source to `install/system-home/`
2. rewrite generator inputs to that subtree
3. rewrite installed-home file set to the exact curated shape
4. remove the second helper binary contract
5. keep `runtime-manifest.json`, rename installed `share/**` payload to `resources/**`, and update all skill/runtime references
6. cut docs to the new topology

Anything smaller leaves two truths in the repo. That is worse than waiting.

### Search and boring-tech check

This plan introduces no new infrastructure. It stays with:

- shell scripts
- file copies
- symlinks
- generated markdown

That is the right call. No innovation tokens needed.

### Completeness check

Shortcut version: move files but leave the helper binary and old `share/**` name in place.

Complete version: remove every remaining assumption that `system-charter-intake` is a runtime wrapper, preserve `runtime-manifest.json`, rename installed static payload from `share/**` to `resources/**`, and keep extra payload out of `.agents/skills/*`.

Recommendation: do the complete version. The difference is a few script and smoke edits now versus another corrective milestone immediately after this one.

## Architecture

### Current wrong topology

```text
repo root
  ├─ authored install-home files mixed into root
  ├─ generate.sh writes thin repo projections
  ├─ install.sh installs extra runtime payload
  └─ live skill flow depends on helper wrapper + runtime-manifest + share/** naming that should become resources/**

~/.codex/skills/system*
  └─ discovery is thin, but the overall contract still behaves like a heavier runtime surface
```

### Target topology

```text
install/system-home/            .agents/skills/*                 ~/system/                     ~/.codex/skills/*
authored truth                  generated thin repo output       curated installed home        discovery glue only
---------------------           ---------------------------      ------------------------      ----------------------
SKILL.md.tmpl         ------->  system/SKILL.md           ---->  SKILL.md.tmpl               system -> ~/system/.agents/skills/system
agents/openai.yaml    ------->  system/agents/openai.yaml ---->  SKILL.md                    system-charter-intake -> ~/system/.agents/skills/system-charter-intake
charter-intake/                system-charter-intake/...        agents/openai.yaml
  SKILL.md.tmpl                                                   bin/system
                                                                  charter-intake/SKILL.md.tmpl
                                                                  charter-intake/SKILL.md
                                                                  .agents/skills/system/*
                                                                  .agents/skills/system-charter-intake/*
```

### Ownership model

| Layer | Owner | Allowed contents | Forbidden contents |
| --- | --- | --- | --- |
| `install/system-home/` | authored source | templates and agent metadata | generated outputs, runtime payload |
| repo `.agents/skills/*` | generator | `SKILL.md`, `agents/openai.yaml` | `bin/`, `runtime-manifest.json`, `resources/` |
| `~/system/` | installer | exact curated installed file set | repo clone, extra helper binaries, heavy runtime payload not listed in target shape |
| `~/.codex/skills/*` | installer or dev-setup | symlinks or thin copies to skill projections | standalone runtime payload |

### Direct invocation contract for `system-charter-intake`

`system-charter-intake` stops being a wrapper binary contract.

After `M10.5`, the leaf skill flow is:

1. resolve `SYSTEM_HOME`, defaulting to `~/system`
2. assert `~/system/bin/system` exists
3. assert `~/system/runtime-manifest.json` exists
4. assert required static guidance exists under `~/system/resources/**`
5. run the existing charter-intake workflow by invoking `~/system/bin/system` directly
6. keep run evidence under `~/.local/state/system/intake/runs/`

No helper executable. Manifest-based version checks remain allowed. Installed static guidance lives under `resources/**`, not `share/**`.

### Existing-code leverage map

| Sub-problem | Existing code to reuse | M10.5 action |
| --- | --- | --- |
| generate thin repo projections | `tools/codex/generate.sh` | rewrite inputs, preserve thin output contract |
| stage and atomically install `~/system/` | `tools/codex/install.sh` temp-dir swap pattern | preserve pattern, shrink payload |
| discovery override mode | `tools/codex/dev-setup.sh` | preserve as explicit override mode |
| reinstall restoring normal topology | `tools/codex/install.sh` | keep |
| end-to-end skill smoke | `tools/ci/codex-skill-live-smoke.sh` | preserve scenario coverage, change runtime expectations |
| isolated install smoke | `tools/ci/install-smoke.sh` | preserve harness, replace file-set assertions |

## Implementation Plan

### Phase 1. Canonical source migration

Create the authored subtree:

```text
install/system-home/
  SKILL.md.tmpl
  agents/openai.yaml
  charter-intake/
    SKILL.md.tmpl
```

Tasks:

1. move authored root skill template from repo root into `install/system-home/SKILL.md.tmpl`
2. move authored agent metadata into `install/system-home/agents/openai.yaml`
3. move authored leaf skill template into `install/system-home/charter-intake/SKILL.md.tmpl`
4. remove repo-root authored install-home ownership
5. update all generator, installer, docs, and smoke references to the new source paths

### Phase 2. Generator rewrite

Rewrite `tools/codex/generate.sh` so it:

1. reads only from `install/system-home/`
2. generates only repo `.agents/skills/system/SKILL.md`
3. generates only repo `.agents/skills/system/agents/openai.yaml`
4. generates only repo `.agents/skills/system-charter-intake/SKILL.md`
5. generates only repo `.agents/skills/system-charter-intake/agents/openai.yaml`
6. does not write repo-root `SKILL.md`
7. does not write repo-root `charter-intake/SKILL.md`
8. does not write outside the repo

Thin projection invariant:

```text
.agents/skills/system/
  SKILL.md
  agents/openai.yaml

.agents/skills/system-charter-intake/
  SKILL.md
  agents/openai.yaml
```

### Phase 3. Installed-home rewrite

Rewrite `tools/codex/install.sh` so it stages and installs only:

```text
~/system/
  SKILL.md.tmpl
  SKILL.md
  agents/openai.yaml
  runtime-manifest.json
  bin/system
  charter-intake/SKILL.md.tmpl
  charter-intake/SKILL.md
  resources/authoring/charter_authoring_method.md
  resources/charter/CHARTER_INPUTS.yaml.tmpl
  resources/charter/charter_inputs_directive.md
  .agents/skills/system/SKILL.md
  .agents/skills/system/agents/openai.yaml
  .agents/skills/system-charter-intake/SKILL.md
  .agents/skills/system-charter-intake/agents/openai.yaml
```

Required installer behavior:

1. require `system` on `PATH`
2. verify `system --version` matches repo `VERSION`
3. copy that verified binary to `~/system/bin/system`
4. copy curated authored files from `install/system-home/`
5. render `runtime-manifest.json` into `~/system/`
6. copy static runtime guidance into `~/system/resources/**`
7. copy generated thin projections from repo `.agents/skills/*`
8. refresh `~/.codex/skills/*` to point into `~/system/.agents/skills/*`
9. leave no extra files from prior heavy installs

Required deletions from the installed contract:

- `~/system/bin/system-charter-intake`
- `~/system/share/**`
- any update or uninstall helper binaries not listed in the target shape

### Phase 4. Skill invocation rewrite

Rewrite the generated `system-charter-intake` skill template so it:

1. validates repo context
2. resolves `SYSTEM_HOME`
3. asserts `~/system/bin/system`
4. asserts `~/system/runtime-manifest.json`
5. reads installed static guidance from `~/system/resources/**`
6. invokes the existing charter flow through `~/system/bin/system`
7. preserves current evidence and refusal behavior where topology is not part of the behavior

This is the critical behavioral correction. Discovery stays thin, but the workflow no longer depends on a second installed executable.

### Phase 5. Docs and contracts cutover

Update every packaging-facing doc so it says exactly:

- authored install-home truth lives under `install/system-home/`
- repo `.agents/skills/*` is generated and thin
- `~/system/` is the installed home
- `~/system/runtime-manifest.json` remains part of the installed runtime contract
- installed static guidance lives under `~/system/resources/**`
- `~/.codex/skills/*` is discovery glue only
- `~/system/bin/system` is the only installed executable for this skill surface
- `system-charter-intake` is a skill/discovery surface, not a second binary

### Phase 6. Smoke and regression rails

Rewrite install and live smokes to enforce the exact target shape and fail loudly on drift.

## Test Review

### Code path coverage

```text
CODE PATH COVERAGE
===========================
[+] tools/codex/generate.sh
    ├── render root skill from install/system-home/SKILL.md.tmpl
    ├── copy install/system-home/agents/openai.yaml
    ├── render leaf skill from install/system-home/charter-intake/SKILL.md.tmpl
    └── refuse any extra generated payload in repo .agents/skills/*

[+] tools/codex/install.sh
    ├── require system on PATH
    ├── compare PATH binary version to repo VERSION
    ├── stage curated ~/system/ home
    ├── copy verified binary to ~/system/bin/system
    ├── copy installed authored files
    ├── copy installed thin skill projections
    ├── replace prior heavy install leftovers
    └── relink ~/.codex/skills/* to ~/system/.agents/skills/*

[+] tools/codex/dev-setup.sh
    ├── point ~/.codex/skills/system -> repo .agents/skills/system
    └── point ~/.codex/skills/system-charter-intake -> repo .agents/skills/system-charter-intake

[+] system-charter-intake skill flow
    ├── happy path in git repo via ~/system/bin/system
    ├── runtime-manifest.json still available for version/metadata checks
    ├── resources/** still available for installed static guidance
    ├── refusal outside git repo
    ├── refusal when ~/system/bin/system is missing
    └── evidence lands only under ~/.local/state/system/intake/runs/
```

### User flow coverage

```text
USER FLOW COVERAGE
===========================
[+] Normal install
    ├── [REQ] generate repo projections
    ├── [REQ] install curated ~/system/ home
    └── [REQ] relink ~/.codex/skills/* into ~/system/.agents/skills/*

[+] Dev override
    ├── [REQ] dev-setup points discovery to repo-generated .agents/skills/*
    └── [REQ] normal install restores installed-home discovery topology

[+] Charter intake execution
    ├── [REQ] leaf skill invokes ~/system/bin/system directly
    ├── [REQ] no helper binary needed
    └── [REQ] run evidence stays under ~/.local/state/system/intake/runs/
```

### Required test additions

`tools/ci/install-smoke.sh` must assert:

1. repo `.agents/skills/system*` contains only `SKILL.md` and `agents/openai.yaml`
2. installed `~/system/` contains the exact curated file set and nothing extra
3. `~/system/bin/system` exists and version-matches repo `VERSION`
4. `~/system/runtime-manifest.json` exists and validates
5. `~/system/resources/**` exists with the renamed static payload
6. `~/.codex/skills/system*` points into `~/system/.agents/skills/*`
7. reinstall removes stale heavy-install files like `share/**` and `bin/system-charter-intake`
8. normal install after `dev-setup.sh` restores installed discovery topology

`tools/ci/codex-skill-live-smoke.sh` must assert:

1. `system-charter-intake` works without `~/system/bin/system-charter-intake`
2. the leaf skill invokes `~/system/bin/system` directly
3. the leaf skill can still resolve `runtime-manifest.json`
4. the leaf skill can still resolve installed static guidance under `resources/**`
5. refusal outside a git repo still works
6. run evidence appears only under `~/.local/state/system/intake/runs/`
7. no run evidence lands under `~/system/` or `~/.codex/skills/`

## Failure Modes Registry

| Failure mode | Test covers it | Error handling | User-visible outcome |
| --- | --- | --- | --- |
| generator still writes repo-root skill files | install smoke + repo file-set check | fail smoke | immediate CI failure |
| install drops `runtime-manifest.json` unexpectedly | install smoke | exact file-set assertion | immediate CI failure |
| install leaves old `share/**` behind or fails to create `resources/**` | install smoke | installer replacement + exact file-set assertion | immediate CI failure |
| install leaves `bin/system-charter-intake` behind | install smoke | exact file-set assertion | immediate CI failure |
| discovery points to repo projections after normal install | install smoke | reinstall restores links | immediate CI failure |
| leaf skill still depends on helper binary | live smoke | direct invocation rewrite | immediate CI failure |
| leaf skill cannot find renamed installed resources | live smoke | path assertions | immediate CI failure |
| missing `~/system/bin/system` after install | install smoke | install hard-refuses | explicit install failure |
| run evidence lands under installed/discovery trees | live smoke | path assertions | immediate CI failure |

No silent failures are acceptable in this milestone. Packaging drift must fail in CI.

## Code Quality Review

### DRY and clarity requirements

- one authored source subtree, not duplicated root and subtree truths
- one installed binary, not a primary binary plus a helper binary pretending to be part of the public surface
- one discovery topology in normal mode
- one override topology in dev mode
- one semantically accurate installed static-content name, `resources/**`, instead of the vaguer `share/**`

### Explicit-over-clever requirements

- keep `generate.sh` and `install.sh` as straightforward shell scripts
- prefer exact file-set assertions over implicit "contains enough files" checks
- prefer deleting stale heavy-install payloads over compatibility shims that preserve the wrong shape

### Diagram maintenance requirement

Any nearby ASCII diagrams in packaging docs or scripts that describe the old heavier topology must be updated in the same milestone. Stale topology diagrams are a bug.

## Performance Review

This is not a performance-heavy milestone. The risks are correctness and drift, not runtime throughput.

Relevant performance constraints:

- `generate.sh` must stay O(number of generated skill files), which is tiny
- `install.sh` should remain stage-then-swap, so repeated installs stay predictable
- smoke assertions should use exact small file-set checks, not recursive expensive content scans across the repo

## NOT in Scope

- changing `system doctor --json`
- changing the semantic output of the existing charter authoring flow
- adding more Codex-discoverable skills
- adding `.claude/skills/*` generation
- introducing a package manager or updater channel
- turning `~/system/resources/**` into a dumping ground for arbitrary runtime assets

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| Source migration | `install/`, repo skill templates, generator inputs | — |
| Thin projection rewrite | `tools/codex/generate.sh`, `.agents/skills` contract | Source migration |
| Installed-home rewrite | `tools/codex/install.sh`, `tools/codex/dev-setup.sh`, `tools/codex/relink.sh` | Thin projection rewrite |
| Live skill rewrite | install-home skill templates, live smoke | Installed-home rewrite |
| Docs cutover | docs, README, DESIGN, contracts | Source migration |
| Smoke rewrite | `tools/ci/*` | Thin projection rewrite + installed-home rewrite + live skill rewrite |

### Parallel lanes

- Lane A: source migration -> thin projection rewrite
- Lane B: installed-home rewrite (starts after Lane A)
- Lane C: docs cutover (starts after source migration, can run parallel to Lane B)
- Lane D: live skill rewrite -> smoke rewrite (starts after Lane B)

### Execution order

1. Launch Lane A first. It freezes the canonical source layout.
2. Launch Lane B and Lane C in parallel once Lane A is merged.
3. Launch Lane D once Lane B is stable.
4. Merge docs and smokes only after the installed-home contract is proven.

### Conflict flags

- Lane B and Lane D both touch packaging behavior and smoke assumptions. Keep them sequential.
- Lane C must not invent new topology wording. It follows the exact locked shape only.

## Acceptance Checklist

- [ ] authored install-home truth exists only under `install/system-home/`
- [ ] repo root no longer owns install-home authored files
- [ ] repo `.agents/skills/system*` stays thin
- [ ] `generate.sh` writes repo outputs only
- [ ] `install.sh` installs only the curated `~/system/` shape
- [ ] `~/system/bin/system` is the only installed executable for the Codex skill surface
- [ ] `~/system/bin/system-charter-intake` does not exist
- [ ] `~/system/runtime-manifest.json` exists
- [ ] `~/system/resources/` exists
- [ ] `~/system/share/` does not exist
- [ ] `~/.codex/skills/system*` points into `~/system/.agents/skills/*`
- [ ] dev setup can temporarily point discovery to repo `.agents/skills/*`
- [ ] normal install restores the installed-home discovery topology
- [ ] live skill flow calls `~/system/bin/system` directly
- [ ] no run evidence lands under `~/system/` or `~/.codex/skills/`
- [ ] docs match the new topology exactly
- [ ] install and live smokes fail on any topology drift

## Completion Summary

- Step 0: Scope Challenge, complete. Scope is accepted as-is because partial correction would preserve dual truth.
- Architecture Review: one canonical source subtree, one installed binary, one thin discovery model.
- Code Quality Review: root-vs-subtree authored duplication is the main DRY violation to remove.
- Test Review: full packaging and live-skill coverage defined above.
- Performance Review: no throughput concerns, correctness-first milestone.
- NOT in scope: written.
- What already exists: written.
- Failure modes: defined, all mapped to smoke enforcement.
- Parallelization: 4 lanes, with source migration first and smoke rewrite last.
- Lake Score: 6/6, this plan chooses the complete option everywhere the old heavier shape could linger.
