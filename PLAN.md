<!-- PLAN rewrite restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-system/feat-m8-plan-solidify-restore-20260429-160200.md -->
# PLAN

## Status

This is the consolidated execution plan for `M9.5`, the Codex skill-packaging milestone on branch `feat/m8`.

The branch name still says `m8` because this work builds directly on the shipped M8 baseline-authoring and doctor/setup foundation. The active milestone is `M9.5`.

This pass replaces the older `M9` intake-plumbing plan with one implementation contract grounded in:

- the checkpoint at `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/checkpoints/20260429-112437-m95-codex-skill-packaging.md`
- the design doc at `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-feat-m8-design-20260429-112243.md`
- the current repository code and contracts
- the observed `gstack` Codex packaging pattern under `/Users/spensermcconnell/gstack/` and `~/.codex/skills/gstack*`

## Active Objective

Ship the first real installed Codex skill surface for `system` charter intake.

The job is not more authoring logic.

The job is:

- make Codex discovery and autocomplete work through installed `~/.codex/skills/system-*` entries
- install one minimal shared runtime root at `~/.codex/skills/system/`
- keep the target working repo as the mutation target, not the installation home
- preserve `system` as the only authority for readiness, validation, refusal, and canonical writes

If this milestone lands correctly, the user experience changes from "run a repo-local wrapper script" to "use an installed Codex skill that can target any working repo and still terminate through compiler-owned truth."

## Review Inputs

- Latest checkpoint outcome:
  - the right target is no longer generic `M9` intake plumbing
  - the right target is `M9.5`, a `gstack`-style Codex packaging layer for the existing charter-intake wedge
- Latest design outcome:
  - mimic the `gstack` three-layer Codex pattern
  - stay Codex-first and charter-only
  - keep the runtime root minimal and global
- Current code reality:
  - `crates/compiler/src/doctor.rs` already produces a typed `DoctorReport`
  - `crates/cli/src/main.rs` still exposes only human-readable `system doctor`, not `system doctor --json`
  - `crates/compiler/src/author/project_context.rs` already shows the correct deterministic `--from-inputs` pattern
  - `crates/compiler/src/author/charter.rs` still routes `system author charter --from-inputs` through `synthesize_charter_markdown()`, which shells out to `codex exec`
  - there is no `--validate` preflight mode for charter structured inputs yet
  - there is no repo `.agents/skills/` layer, no Codex installer flow, and no runtime root under `~/.codex/skills/system/`
  - `tools/ci/install-smoke.sh` only proves CLI install and a fixture-backed packet path, not installed skill packaging
- Reference packaging reality from `gstack`:
  - generated Codex skills live in repo `.agents/skills/`
  - discoverable installed entries live in `~/.codex/skills/gstack-*`
  - a separate minimal runtime root lives in `~/.codex/skills/gstack/`

## Scope Lock

### In scope

- one Codex skill only: `system-charter-intake`
- one generated repo skill layer at `.agents/skills/`
- one installed discovery layer at `~/.codex/skills/`
- one installed shared runtime root at `~/.codex/skills/system/`
- one version/compatibility manifest between installed skill assets and the `system` binary
- `system doctor --json` as the machine-readable readiness surface the skill consumes
- `system author charter --validate --from-inputs <path|->` as the mutation-free preflight surface
- deterministic compiler-owned `system author charter --from-inputs <path|->`
- one local install/relink/dev-setup story for Codex packaging
- one run-artifact persistence surface under `~/.local/state/system/intake/runs/`
- docs, smoke proof, and one bounded live Codex happy-path smoke for the installed skill

### NOT in scope

- `project-context` or `environment-inventory` Codex skills
- Claude Code support or any multi-host generator
- a generic host-agnostic skill DSL
- public package-manager or release-channel distribution
- a new Node/Bun build pipeline just to render one skill
- moving canonical truth outside `.system/`
- expanding the CLI verb set beyond the exact new `doctor` and `author charter` flags needed for this milestone

## Frozen Decisions

1. `M9.5` is a packaging milestone, not another round of intake-plumbing exploration.
2. The first and only shipped Codex skill name is `system-charter-intake`.
3. The source layer stays in this repo, the generated Codex layer lives in `.agents/skills/`, and the installed runtime/discovery layer lives under `~/.codex/skills/`.
4. `.agents/skills/` is generated output. It is not the primary handwritten source of truth.
5. The installed runtime root at `~/.codex/skills/system/` contains immutable runtime assets only. Mutable run artifacts go to `~/.local/state/system/intake/`.
6. Production install copies generated discovery/runtime assets into `~/.codex/skills/`. Symlinks are reserved for explicit dev setup only.
7. The generated skill preamble resolves the runtime root in this order:
   - current repo `<git-root>/.agents/skills/system/` when present
   - otherwise `~/.codex/skills/system/`
8. The installer flow lives under `tools/codex/`. We do **not** add a repo-root shell script named `setup`, because that collides with the shipped `system setup` product surface.
9. We stay boring by default: shell templates and copy/symlink install mechanics are enough. No new generator framework, no extra crate, no extra package manager.
10. The installed skill never computes readiness itself and never writes canonical truth directly. It only orchestrates `system doctor`, `system setup`, `system author charter --validate`, and `system author charter --from-inputs`.
11. Installed skill assets and the `system` binary must agree on an exact runtime version string before any conversation begins.
12. Guided `system author charter` may remain Codex-backed, but the structured-input sink used by the installed skill must be deterministic and compiler-owned.

## Step 0: Scope Challenge

### Current-state diagnosis

The right seam is smaller and more product-shaped than the old `M9` plan.

The repository already has the beginnings of the intake story:

- a typed doctor report
- a thin CLI crate
- a correct deterministic reference implementation in `project_context`
- existing live Codex smoke coverage for compiler-owned charter synthesis

What it does **not** have is the installed Codex product surface:

- no generated `.agents/skills/`
- no installed discovery entries under `~/.codex/skills/system-*`
- no installed shared runtime root under `~/.codex/skills/system/`
- no machine-readable `doctor` surface for the skill to trust
- no mutation-free charter preflight mode
- no deterministic structured-input charter sink

That means the minimal complete lake is:

- one skill
- one runtime root
- one install/update story
- the exact CLI/compiler contract hardening required to make that skill honest

Anything smaller is fake completeness. A discoverable skill that still depends on human-readable `doctor` output or a Codex-backed `--from-inputs` path is not a real trust boundary.

### What already exists

| Sub-problem | Existing code or asset | Reuse decision |
| --- | --- | --- |
| typed baseline readiness model | `crates/compiler/src/doctor.rs` | Reuse exactly. Add JSON serialization at the CLI/compiler boundary, do not rebuild doctor semantics in shell. |
| deterministic structured-input authoring pattern | `crates/compiler/src/author/project_context.rs` | Reuse as the implementation model for deterministic charter `--from-inputs`. |
| charter structured-input schema and validation | `crates/compiler/src/author/charter.rs` | Reuse the input model and validation, split only the synthesis/write ownership. |
| CLI install smoke harness | `tools/ci/install-smoke.sh` | Reuse and extend instead of creating a second independent smoke harness. |
| live Codex transport smoke pattern | `tools/ci/author-live-smoke.sh` | Reuse for compiler-owned authoring transport proof; add installed-skill smoke alongside it. |
| product vocabulary and command contracts | `DESIGN.md`, `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`, `docs/contracts/C-04-resolver-result-and-doctor-blockers.md` | Reuse and revise. Do not invent a second public command story. |
| Codex packaging reference shape | `/Users/spensermcconnell/gstack/`, `~/.codex/skills/gstack*` | Mimic the layering, not the entire toolchain. |

### Complexity check

This plan will touch more than eight files and multiple module directories. That is a smell, but it is a justified one.

The packaging wedge crosses four real boundaries:

- compiler semantics
- CLI flags and output
- installed Codex skill/runtime assets
- docs and smoke proof

The plan stays "engineered enough" by refusing the usual overbuilds:

- no second authoring framework
- no generic multi-host generator
- no new crate
- no Node/Bun generator stack
- no more than one discoverable skill

### Search and best-practice check

- **[Layer 1]** Codex discovery belongs under `~/.codex/skills/`, because that is the proven home-level discovery surface already used by the reference `gstack` install.
- **[Layer 1]** The Rust CLI install story remains `cargo install --path crates/cli`. We do not invent a second binary install mechanism in this milestone.
- **[EUREKA]** The `gstack` *shape* should be copied, but not necessarily its persistent symlink semantics. `gstack` lives in a stable home-level repo install; `system` today is cargo-installed from arbitrary clones. That makes copy-based home installation the boring product choice, and symlink-based setup the explicit dev-only choice.

### TODOS cross-reference

Relevant deferred items already exist in `TODOS.md`:

- `Claude Code Conversational Intake Surface`
- `Public CLI Distribution`
- `CLI Release Workflow`

This plan does not silently absorb any of them.

### Completeness and distribution verdict

Because this milestone introduces a new user-facing artifact type, an installed Codex skill, the plan includes its whole distribution loop:

- generation
- install
- relink/update
- version compatibility
- smoke proof

No half-shipped wrapper story. No "we'll add the install/update part later."

## Exact Shipped Behavior

### Source of truth

1. Handwritten packaging source lives under `tools/codex/`. That directory owns generation, install, dev-setup, runtime templates, and the discoverable `SKILL.md` template.
2. Shared charter methodology assets continue to live under existing repo truth:
   - `core/library/authoring/charter_authoring_method.md`
   - `core/library/charter/CHARTER_INPUTS.yaml.tmpl`
   - `core/library/charter/charter_inputs_directive.md`
3. `.agents/skills/` is generated output only. It is never hand-edited. If the repo does not already ignore `.agents/skills/`, this milestone adds that ignore rule as part of the packaging work.

### Generated and installed assets

4. `tools/codex/generate.sh` generates exactly two repo-local outputs:
   - `.agents/skills/system-charter-intake/SKILL.md`
   - `.agents/skills/system/`
5. `.agents/skills/system/` is the full generated runtime root. It contains exactly:
   - `SKILL.md`
   - `runtime-manifest.json`
   - `bin/system-charter-intake`
   - `share/authoring/charter_authoring_method.md`
   - `share/charter/CHARTER_INPUTS.yaml.tmpl`
   - `share/charter/charter_inputs_directive.md`
6. `runtime-manifest.json` contains at minimum:
   - `skill_name`
   - `system_release_version`
   - `manifest_version`
   - `generated_at_utc`
7. `tools/codex/install.sh` is the normal product installer. It runs generation first, then copies exactly:
   - `.agents/skills/system-charter-intake/` -> `~/.codex/skills/system-charter-intake/`
   - `.agents/skills/system/` -> `~/.codex/skills/system/`
8. `tools/codex/dev-setup.sh` is the only symlink-based path. It is explicitly for local development, not normal installation. It generates assets first, then recreates:
   - `~/.codex/skills/system-charter-intake` as a symlink to the repo-generated discovery directory
   - `~/.codex/skills/system` as a symlink to the repo-generated runtime root
9. `tools/codex/relink.sh` is a developer convenience wrapper around the dev-symlink flow. It does not introduce a second product install mode.

### Runtime contract

10. The generated discoverable skill resolves `SYSTEM_CODEX_ROOT` in one order only:
   - current working repo git root + `/.agents/skills/system/`, but only if that directory exists and contains a complete runtime manifest
   - otherwise `~/.codex/skills/system/`
11. Before any conversational intake, the runtime performs these checks in order:
    - `system` exists on `PATH`
    - `runtime-manifest.json` exists
    - `system --version` matches the manifest release version exactly after trimming the leading binary name from clap version output
    - the runtime root contains the required `bin/` and `share/` assets
12. The discoverable skill resolves the target repo from the current working directory or its enclosing git root. If the command is launched outside a real git repo, it refuses before asking questions.
13. The runtime never derives readiness by inspecting repo files directly. It trusts only `system doctor --json`.

### CLI and compiler contract

14. `system doctor --json` is the only machine-parsed readiness surface for this milestone.
15. `system doctor --json` writes valid UTF-8 JSON to stdout for ready and ordinary non-ready states, with no prose mixed into stdout.
16. The JSON shape is the serialized `DoctorReport` contract and includes these top-level fields:
    - `c04_result_version`
    - `c03_schema_version`
    - `c03_manifest_generation_version`
    - `baseline_state`
    - `blockers`
    - `status`
    - `system_root_status`
    - `checklist`
    - `next_safe_action`
17. `system doctor --json` keeps the current exit-code semantics:
    - exit `0` only for `BaselineComplete`
    - exit non-zero for ordinary scaffolded, partial, or invalid baseline states
18. `system author charter --validate --from-inputs <path|->` is a new preflight-only mode.
19. `--validate` is legal only when `--from-inputs <path|->` is also present. Guided `system author charter` does not accept `--validate`.
20. The runtime does not machine-parse `author charter` success or refusal output. For validate and write calls it relies on exit code, preserves stdout/stderr transcripts, and stores them as evidence.
21. `system author charter --validate --from-inputs <path|->` must:
    - parse the exact same YAML schema as the write path
    - run the same structured-input validation
    - run the same repo preflight and overwrite checks
    - perform zero mutation
22. `system author charter --from-inputs <path|->` becomes a deterministic compiler-owned render/write path with no `codex exec` dependency.
23. Guided `system author charter` remains the only charter path that may invoke Codex synthesis.

### Intake and run-artifact contract

24. The discoverable skill only gathers charter facts and normalizes them into one YAML file. It never writes canonical markdown itself.
25. Each run persists to `~/.local/state/system/intake/runs/<timestamp>-<pid>/`.
26. Every run directory contains:
   - `session.json`
   - `doctor.before.json`
   - `doctor.after_setup.json` when setup ran
   - `doctor.after_write.json` when write ran
   - `charter_inputs.yaml`
   - `validate.stdout.txt`
   - `validate.stderr.txt`
   - `validate.exit`
   - `author.stdout.txt`
   - `author.stderr.txt`
   - `author.exit`
27. `session.json` records at minimum:
   - `started_at_utc`
   - `repo_root`
   - `runtime_root`
   - `system_release_version`
   - `runtime_manifest_version`
28. On the happy path the runtime sequence is fixed:
   - `system doctor --json`
   - optional bare `system setup`
   - `system doctor --json` again if setup ran
   - `system author charter --validate --from-inputs`
   - `system author charter --from-inputs`
   - final `system doctor --json`
29. If the first or second doctor report shows charter already at `ValidCanonicalTruth`, the skill refuses overwrite and surfaces the compiler-owned next action instead of interviewing further.
30. After a successful write, the runtime reports charter success even if other baseline artifacts remain scaffolded. It does not pretend the whole baseline is complete unless doctor says so.

### Proof requirements

31. `tools/ci/install-smoke.sh` proves fresh install, reinstall, stale-runtime refusal, and copy-based idempotence for the installed Codex surface.
32. `tools/ci/codex-skill-live-smoke.sh` is the one bounded real Codex smoke. It proves the installed `system-charter-intake` surface against a fresh temp repo.
33. Live smoke and install smoke evidence lands under `.implemented/m9.5-codex-skill-packaging/`. This plan does not use `tests/fixtures/` as a catch-all proof bucket for human-readable transcripts.

## Architecture Review

### Layered ownership

The architecture is one trust boundary with three packaging layers around it.

```text
handwritten repo source
  |
  +--> tools/codex/                         generation + install + dev setup
  +--> core/library/authoring/             charter authoring method source
  +--> core/library/charter/               charter input template + intake directive
  |
  v
generated repo-local Codex assets
  |
  +--> .agents/skills/system-charter-intake/SKILL.md
  +--> .agents/skills/system/
  |
  v
installed home Codex assets
  |
  +--> ~/.codex/skills/system-charter-intake/
  +--> ~/.codex/skills/system/
  |
  v
target working repo
  |
  +--> system doctor --json
  +--> system setup
  +--> system author charter --validate --from-inputs
  +--> system author charter --from-inputs
  |
  v
.system/charter/CHARTER.md
```

Nothing in the top two layers is allowed to become a second source of canonical repo truth. They only package access to the existing compiler boundary.

### Runtime execution flow

```text
Codex invokes system-charter-intake
  |
  v
resolve SYSTEM_CODEX_ROOT
  |
  +--> repo-local override complete? use <repo>/.agents/skills/system/
  +--> else use ~/.codex/skills/system/
  |
  v
validate runtime-manifest.json + required runtime files
  |
  v
run `system --version` and compare exact release version
  |
  v
resolve target repo root
  |
  v
system doctor --json
  |
  +--> missing or invalid `.system` root? -> run bare `system setup`
  |                                      -> rerun `system doctor --json`
  +--> charter already valid? -> refuse overwrite
  +--> otherwise continue
  |
  v
conversational capture -> charter_inputs.yaml
  |
  v
system author charter --validate --from-inputs
  |
  +--> non-zero exit? stop and surface saved transcripts
  |
  v
system author charter --from-inputs
  |
  +--> non-zero exit? stop and surface saved transcripts
  |
  v
system doctor --json
  |
  v
persist run evidence + print next safe action
```

### Ownership map

| Area | Ownership | Required change |
| --- | --- | --- |
| `crates/compiler/src/doctor.rs` | compiler | keep doctor semantics, add serde-backed JSON contract so CLI can emit the existing report structurally |
| `crates/cli/src/main.rs` | CLI | add `doctor --json`; add `author charter --validate --from-inputs`; preserve current human-readable defaults |
| `crates/compiler/src/author/charter.rs` | compiler | split guided Codex synthesis from deterministic render/write so `--from-inputs` never shells out |
| `crates/compiler/src/author/project_context.rs` | compiler reference | reuse its deterministic authoring shape as the model for charter refactor |
| `crates/compiler/tests/doctor.rs` | compiler tests | lock JSON fields, checklist serialization, and non-ready exit semantics |
| `crates/compiler/tests/author.rs` | compiler tests | lock validation-only mode, deterministic sink, overwrite refusal, and no-Codex regression |
| `crates/cli/tests/author_cli.rs` + `help_drift_guard.rs` | CLI tests | lock flag parsing, help text, refusal wording, and exit-code behavior |
| `tools/codex/` | packaging/install runtime | add generator, installer, dev setup, relink wrapper, runtime manifest, and runtime entrypoint |
| `.agents/skills/` | generated build output | hold the generated discoverable skill and runtime root, never handwritten source |
| `tools/ci/install-smoke.sh` + `tools/ci/codex-skill-live-smoke.sh` | smoke harness | prove install, runtime compatibility, and one bounded live happy path |
| `.implemented/m9.5-codex-skill-packaging/` | proof artifacts | store install and live smoke evidence for docs and cutover review |
| `README.md`, `docs/`, `docs/contracts/`, `DESIGN.md` | docs/contracts | align product wording with the new machine-readable readiness and installed skill story |

## Implementation Plan

Implementation order is fixed at the gate level. No later gate is considered done until its exit condition is met.

### Gate 1: Harden the CLI and compiler contract

This gate makes the installed skill honest. Without it, the packaging layer would still be guessing.

Deliverables:

- add `doctor --json` to the existing `doctor` command
- serialize the current `DoctorReport` shape without changing doctor semantics
- add `--validate` to `system author charter`, valid only with `--from-inputs <path|->`
- refactor charter authoring so deterministic `--from-inputs` never calls `codex exec`
- keep guided `system author charter` as the only Codex-backed path

Files/modules:

- `crates/cli/src/main.rs`
- `crates/compiler/src/doctor.rs`
- `crates/compiler/src/author/charter.rs`
- `crates/compiler/src/lib.rs`
- `crates/compiler/tests/doctor.rs`
- `crates/compiler/tests/author.rs`
- `crates/cli/tests/author_cli.rs`
- `crates/cli/tests/help_drift_guard.rs`

Exit condition:

- `doctor --json` emits the locked top-level contract and preserves current exit semantics
- `author charter --validate --from-inputs` performs full preflight with zero mutation
- deterministic charter `--from-inputs` no longer shells out to `codex exec`

### Gate 2: Generate the Codex packaging surface

This gate creates the repo-local build output for the discoverable skill and shared runtime root.

Deliverables:

- add `tools/codex/generate.sh`
- add one discoverable `SKILL.md` template for `system-charter-intake`
- add one runtime-root template set under `tools/codex/runtime/`
- generate `.agents/skills/system-charter-intake/`
- generate `.agents/skills/system/`
- define the exact runtime-root resolution preamble
- add `.agents/skills/` ignore rules if the repo does not already ignore generated output

Files/modules:

- `tools/codex/generate.sh`
- `tools/codex/templates/system-charter-intake.SKILL.md.tmpl`
- `tools/codex/runtime/`
- `.gitignore`
- `.agents/skills/system-charter-intake/` (generated)
- `.agents/skills/system/` (generated)

Exit condition:

- the repo can generate the two Codex outputs deterministically from handwritten source
- the generated runtime root contains the exact locked file set
- the generated skill resolves repo-local override first, then installed home runtime

### Gate 3: Install, reinstall, and prove the home-level skill

This gate makes the surface real in `~/.codex/skills/` rather than repo-local theater.

Deliverables:

- add `tools/codex/install.sh`
- add `tools/codex/dev-setup.sh`
- add `tools/codex/relink.sh`
- copy-based install or reinstall into `~/.codex/skills/`
- symlink-based dev setup into `~/.codex/skills/`
- stale-runtime refusal
- install smoke
- one bounded live installed-skill smoke

Files/modules:

- `tools/codex/install.sh`
- `tools/codex/dev-setup.sh`
- `tools/codex/relink.sh`
- `tools/ci/install-smoke.sh`
- `tools/ci/codex-skill-live-smoke.sh`
- `.implemented/m9.5-codex-skill-packaging/`

Exit condition:

- a user can install the skill assets into `~/.codex/skills/` from any repo checkout
- rerunning install is idempotent and converges on the same copied home layout
- rerunning dev setup recreates the two expected symlinks cleanly
- stale runtime or binary mismatches refuse before any conversational intake begins

### Gate 4: Cut over docs, contracts, and proof

This gate lands last so the docs describe the implemented surface instead of the aspirational one.

Deliverables:

- docs/help/contract alignment for `doctor --json`
- docs/help/contract alignment for `author charter --validate --from-inputs`
- explicit install, dev-setup, and relink documentation for Codex packaging
- local-first distribution wording
- linked proof artifacts for install smoke and live smoke
- no naming collision between packaging scripts and the `system setup` product verb

Files/modules:

- `README.md`
- `DESIGN.md`
- `docs/SUPPORTED_COMMANDS.md`
- `docs/START_HERE.md`
- `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
- `PLAN.md`

Exit condition:

- docs and help describe the same shipped surface
- the distribution story remains local-first and exact about what installs where
- a future implementer can see the proof trail without hunting through chat history

## Code Quality Review

### Boring-by-default rules

- Do not add a generic multi-host skill generator.
- Do not add a new crate just for Codex packaging.
- Do not add Bun or Node as a required build dependency for one generated skill.
- Do not introduce a repo-root shell script named `setup`.
- Do not let the installed skill parse human-readable doctor output.

### DRY rules

- Reuse `project_context` as the deterministic `--from-inputs` implementation model.
- Reuse existing charter structured-input types and validation.
- Reuse the existing install smoke harness instead of building a second test harness from scratch.
- Reuse one shared preamble/runtime-root resolver for generated Codex skills rather than duplicating shell snippets per skill.

### Explicitness rules

- One discoverable skill, one shared runtime root, one manifest file.
- One compatibility policy: exact runtime-manifest version must match `system --version`.
- One place for mutable run state: `~/.local/state/system/intake/`.
- One packaging build-output rule: `.agents/skills/` is generated, ignored, and never hand-edited.

### Required inline diagrams during implementation

- `crates/compiler/src/author/charter.rs`
  - guided path vs deterministic structured-input path
- `crates/cli/src/main.rs`
  - doctor human output vs doctor JSON output routing
- `tools/codex/runtime/bin/system-charter-intake`
  - runtime-root resolution -> version check -> doctor/setup/validate/write loop
- `tools/codex/install.sh`
  - source assets -> generated assets -> home install copy flow

## Test Review

### Framework baseline

No new test framework is needed.

Current layers are already the right shape:

- compiler tests under `crates/compiler/tests/`
- CLI tests under `crates/cli/tests/`
- shell smoke under `tools/ci/`

Test-plan artifact written to:

- `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-feat-m8-eng-review-test-plan-20260429-113817.md`

### Code path coverage to add

```text
CODE PATH COVERAGE
==================
[+] doctor --json
    |
    ├-- [GAP] typed `DoctorReport` serializes the exact locked top-level fields
    ├-- [GAP] non-ready ordinary states still emit valid JSON with non-zero exit
    ├-- [GAP] catastrophic inspection failure emits refusal prose, not partial JSON
    └-- [GAP] checklist items keep exact subject/path/action semantics

[+] author charter --validate --from-inputs
    |
    ├-- [GAP] valid YAML -> exit 0, no mutation
    ├-- [GAP] malformed YAML -> refusal, no mutation
    ├-- [GAP] incomplete YAML -> refusal, no mutation
    ├-- [GAP] existing valid charter -> refusal before mutation
    └-- [GAP] `--validate` without `--from-inputs` -> CLI refusal

[+] author charter --from-inputs
    |
    ├-- [GAP] deterministic render/write with no codex exec
    ├-- [GAP] malformed YAML -> refusal
    ├-- [GAP] incomplete YAML -> refusal
    ├-- [GAP] overwrite refusal on existing valid charter
    ├-- [GAP] invalid write target -> mutation refused
    └-- [GAP] guided path still works and remains the only Codex-backed route

[+] Codex packaging generation/install
    |
    ├-- [GAP] generate .agents/skills/system-charter-intake deterministically
    ├-- [GAP] generate .agents/skills/system runtime root deterministically
    ├-- [GAP] install copies both roots into ~/.codex/skills/
    ├-- [GAP] dev setup creates the two expected symlinks only
    └-- [GAP] reinstall is idempotent and cleans stale files safely

[+] installed skill runtime
    |
    ├-- [GAP] runtime root fallback order works
    ├-- [GAP] runtime manifest version mismatch refuses early
    ├-- [GAP] missing `system` binary refuses early
    ├-- [GAP] runtime root missing required `share/` assets refuses early
    └-- [GAP] run artifacts persist under ~/.local/state/system/intake/runs/<timestamp>-<pid>/
```

### User flow coverage to add

```text
USER FLOW COVERAGE
==================
[+] Fresh install and discovery flow [->E2E]
    |
    ├-- [GAP] install -> ~/.codex/skills/system-charter-intake appears
    ├-- [GAP] install -> ~/.codex/skills/system runtime root appears and is readable
    └-- [GAP] reinstall keeps the same file set without manual cleanup

[+] Fresh target repo happy path [->E2E]
    |
    ├-- [GAP] doctor --json -> setup -> doctor --json -> validate -> write -> doctor --json
    ├-- [GAP] final doctor reports remaining baseline work truthfully
    └-- [GAP] run directory contains the exact locked evidence files

[+] Existing charter refusal flow
    |
    └-- [GAP] installed skill refuses overwrite and surfaces compiler-owned next action

[+] Stale runtime / binary drift flow
    |
    └-- [GAP] runtime manifest and `system --version` mismatch refuse before questioning

[+] Dev override flow
    |
    └-- [GAP] repo-local .agents/skills/system override wins over ~/.codex/skills/system

[+] Live installed Codex happy path [->E2E]
    |
    └-- [GAP] bounded real Codex smoke proves the installed skill surface, not just compiler transport
```

### Required tests and proofs

- `crates/compiler/tests/doctor.rs`
  - doctor JSON contract
  - checklist field coverage
  - non-ready exit behavior with valid JSON
- `crates/compiler/tests/author.rs`
  - mutation-free charter validation path
  - deterministic charter structured-input sink
  - regression test that deterministic `--from-inputs` does not invoke Codex
  - regression test that guided `system author charter` still invokes the synthesis path
- `crates/cli/tests/author_cli.rs`
  - new flag parsing
  - refusal text
  - exit behavior
- `crates/cli/tests/help_drift_guard.rs`
  - help/docs parity for `doctor --json` and `author charter --validate --from-inputs`
- `tools/ci/install-smoke.sh`
  - generate/install/reinstall/stale-runtime checks for Codex packaging
- `tools/ci/codex-skill-live-smoke.sh`
  - bounded installed-skill happy path
- proof artifacts
  - fresh install proof
  - fresh target repo happy path
  - existing charter refusal
  - stale runtime refusal
  - dev override proof

### Exact test-plan expectations

The implementation is not done until the new tests prove both the trust boundary and the packaging boundary.

- `doctor --json` tests must assert field names and exit semantics, not just "parses as JSON".
- `author charter --validate --from-inputs` tests must assert zero mutation by checking that `.system/charter/CHARTER.md` is untouched on both success and refusal.
- deterministic `author charter --from-inputs` tests must fail if `codex exec` is invoked, even accidentally through a leftover helper.
- packaging tests must assert the exact installed file set under `~/.codex/skills/system/`, not just directory existence.
- live smoke must exercise the installed skill surface, not a repo-local wrapper or direct binary invocation.

### Regression rules

- The current Codex dependency in charter `--from-inputs` is a regression gap. The fix is mandatory and its regression test is mandatory.
- Any installed skill behavior that guesses readiness without `system doctor --json` is a regression against the trust boundary.
- Any install path that leaves discoverable skill entries without a compatible shared runtime root is a packaging regression.

## Performance and Reliability Review

This milestone is packaging-heavy, not CPU-heavy, but it still has real operational constraints.

- The installed skill must not scan the repo to infer readiness. It should call `system`.
- The runtime should shell out at most:
  - one `system --version` compatibility check
  - one initial `system doctor --json`
  - optional bare `system setup`
  - one follow-up `system doctor --json` if setup ran
  - one validation call
  - one write call
  - one final `system doctor --json`
- `tools/codex/install.sh` must not compile Rust code itself. It installs already-built packaging assets and assumes the user has installed or rebuilt the `system` binary separately.
- Run artifacts should stream to disk as they are produced. Do not hold long transcripts or JSON blobs in memory unnecessarily.
- No daemon, file watcher, background sync loop, or persisted readiness cache ships in `M9.5`.
- Install and reinstall must be idempotent. A second run should converge on the same copied home layout without manual cleanup.
- Dev setup must be reversible. Re-running normal install after dev setup must replace symlinks with copied directories cleanly.

## Failure Modes Registry

| Codepath | Realistic production failure | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- | --- |
| generated skill discovery | `~/.codex/skills/system-charter-intake/` missing or half-installed | yes | install/reinstall refusal with exact missing path | explicit install failure |
| runtime root resolution | repo-local override missing files or malformed manifest | yes | hard refusal before conversation | exact broken runtime root shown |
| binary/runtime drift | installed skill version does not match `system --version` | yes | early compatibility refusal | explicit "rerun install" guidance |
| target repo detection | skill launched outside a real repo | yes | repo-root refusal | exact failing directory shown |
| doctor JSON | CLI emits prose or invalid JSON for ordinary non-ready states | yes | parse refusal, no guessing | explicit contract failure |
| setup routing | skill skips bare `system setup` after doctor says root missing/invalid | yes | regression-blocking smoke | no write attempt, exact setup action shown |
| charter validation | malformed or incomplete YAML reaches the write path | yes | validation refusal | field-specific repair guidance |
| deterministic charter sink | `--from-inputs` still invokes Codex | yes, critical | regression test | ship blocker |
| existing canonical truth | installed skill overwrites valid authored charter | yes | compiler-owned refusal | exact next action to inspect existing charter |
| run-artifact persistence | skill writes artifacts into `~/.codex/skills/system/` or target repo accidentally | yes | fixed state-path enforcement | no mutable runtime-root writes |
| install-mode crossover | normal install leaves symlinks behind after dev setup | yes | installer replaces symlinks with copied directories | explicit install success, no dangling dev links |
| operator output | compatibility or refusal output hides the exact next step | yes | exact repair wording | no mystery failure |

Critical gap rule: any path that writes canonically without doctor JSON truth, without validation, or without deterministic compiler ownership is a ship blocker.

## TODOS Cross-Reference

The existing TODOs remain the right follow-ons:

- `Claude Code Conversational Intake Surface`
- `Public CLI Distribution`
- `CLI Release Workflow`

No new TODOs are added by this plan. The right move is to land one honest installed Codex packaging slice first.

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| 1. CLI/compiler contract hardening | `crates/compiler/src/author/`, `crates/compiler/src/doctor.rs`, `crates/cli/src/`, compiler/CLI tests | --- |
| 2. Codex generation and runtime packaging | `tools/codex/`, `.agents/skills/`, `.gitignore` | frozen command names and runtime policy from this plan |
| 3. Home install and smoke proof | `tools/ci/`, `.implemented/m9.5-codex-skill-packaging/` | Steps 1 and 2 |
| 4. Docs and contract cutover | `README.md`, `docs/`, `DESIGN.md`, `PLAN.md` | Steps 1, 2, and 3 |

### Parallel lanes

- Lane A: Step 1
  - sequential inside the lane because `doctor`, charter authoring, and CLI flag semantics share one trust boundary
- Lane B: Step 2
  - launchable in parallel with Lane A because it touches packaging assets only and the command names are already frozen in this plan
- Lane C: Step 3
  - waits for A and B because install smoke needs the real CLI contract and the real generated packaging surface
- Lane D: Step 4
  - waits for C because docs and contracts must match the final proven behavior

### Execution order

Launch Lane A and Lane B in parallel worktrees after this plan is approved.

Merge both.

Then run Lane C for install and smoke proof.

Then run Lane D for docs/contracts cutover.

### Conflict flags

- Lane A and Lane B must not both edit `README.md`, `docs/`, or help snapshots. Reserve those for Lane D.
- Lane A owns `crates/cli/src/` and `crates/compiler/src/author/charter.rs`. Lane B must not touch those modules.
- Lane B owns `tools/codex/`, `.gitignore`, and generated `.agents/skills/`. Lane A must not create packaging logic there.
- Lane C owns `tools/ci/` and `.implemented/m9.5-codex-skill-packaging/`. Lanes A and B should not create final proof artifacts there.
- Lane D is the only lane allowed to finalize wording in docs, help snapshots, and contract prose.

Result: 4 lanes total, 2 launchable in parallel, 2 final sequential integration lanes.

## Acceptance Criteria

1. `system doctor --json` is documented, tested, and emits the exact locked contract.
2. `system author charter --validate --from-inputs <path|->` is documented, tested, and mutation-free.
3. `system author charter --from-inputs <path|->` is deterministic and does not invoke Codex.
4. Guided `system author charter` still works and remains clearly separate from the deterministic structured-input sink.
5. The repo can generate `.agents/skills/system-charter-intake/` and `.agents/skills/system/` deterministically.
6. `tools/codex/install.sh` installs or reinstalls `~/.codex/skills/system-charter-intake/` and `~/.codex/skills/system/`.
7. The installed skill refuses early when the runtime manifest and `system --version` do not match.
8. The installed skill can complete the fresh target repo charter happy path end to end.
9. The installed skill persists run artifacts under `~/.local/state/system/intake/runs/`.
10. The installed skill refuses overwrite of existing valid charter truth.
11. Install/reinstall smoke covers fresh install, reinstall, stale-runtime refusal, dev-override behavior, and live skill happy path.
12. Docs, help, contracts, and smoke artifacts all describe the same shipped packaging story.

## Unresolved Decisions

None.

The important design choices that were open in the draft design doc are now locked:

- skill name: `system-charter-intake`
- shared runtime root: `~/.codex/skills/system/`
- install mode: copy for production, symlink for dev setup only
- installer location: `tools/codex/`, not repo-root `setup`

## Completion Summary

- Step 0: Scope Challenge, accepted as one-skill, Codex-first, packaging-first, deterministic-sink-backed
- Architecture Review: one source layer, one generated Codex layer, one installed runtime/discovery layer
- Code Quality Review: generic host framework rejected, shell-based packaging chosen, generated assets locked as build output
- Test Review: full coverage diagram produced, installed-skill smoke and deterministic-sink regression tests required
- Performance Review: no daemon/cache layer allowed, runtime shell-out count bounded, install/dev-setup reversibility required
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 0 new items proposed
- Failure modes: written, with deterministic sink, readiness guessing, version drift, and install-mode crossover blockers called out
- Outside voice: checkpoint + design doc + local `gstack` packaging inspection incorporated; no fresh dual-model challenge was run in this rewrite
- Parallelization: 4 lanes, 2 parallel / 2 sequential
- Lake Score: 10/10 major recommendations chose the complete option over the shortcut

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | carried forward | checkpoint + latest design narrowed the wedge to Codex-first, charter-only packaging |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | rewritten plan | scope challenge, locked contracts, test diagram, failure modes, and parallelization are all refreshed for `M9.5` |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped, no UI scope | — |

**VERDICT:** Fresh `M9.5` implementation contract is ready. If you want an additional independent challenge pass before coding starts, re-run `/autoplan` against this rewritten plan.
