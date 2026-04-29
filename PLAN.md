<!-- PLAN rewrite restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-system/feat-m8-plan-rewrite-restore-20260429-113428.md -->
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

1. The repo contains a source packaging layer under `tools/codex/` plus shared authoring assets under existing `core/library/authoring/` and `core/library/charter/`.
2. The repo can generate exactly two Codex-facing outputs:
   - `.agents/skills/system-charter-intake/`
   - `.agents/skills/system/`
3. `.agents/skills/system-charter-intake/` contains the discoverable Codex `SKILL.md` for the shipped user command.
4. `.agents/skills/system/` contains the shared runtime root content used by generated and installed Codex assets.
5. `tools/codex/install.sh` installs or refreshes exactly:
   - `~/.codex/skills/system-charter-intake/`
   - `~/.codex/skills/system/`
6. `tools/codex/relink.sh` or equivalent refresh command can reinstall the generated home assets without manual deletion.
7. `tools/codex/dev-setup.sh` creates a dev-only symlinked setup from the home skill discovery/runtime entries back to the current repo-generated assets.
8. The installed runtime root at `~/.codex/skills/system/` contains at minimum:
   - `SKILL.md`
   - `runtime-manifest.json`
   - `bin/system-charter-intake`
   - any prompt/example/template assets required by the runtime
9. The generated skill preamble resolves `SYSTEM_CODEX_ROOT` by checking the current git root for `.agents/skills/system/`, then falling back to `~/.codex/skills/system/`.
10. Before asking any questions, the installed skill verifies:
    - `system` is on `PATH`
    - `system --version` matches the installed `runtime-manifest.json`
    - the shared runtime root exists and is complete
11. The skill resolves the target repo from the current working directory or enclosing git root and refuses if no valid target repo exists.
12. The skill calls `system doctor --json` as its first readiness action.
13. `system doctor --json` emits valid UTF-8 JSON for ready and ordinary non-ready states. It does not mix prose and JSON.
14. If doctor reports a missing or invalid `.system` root, the skill runs bare `system setup`, then reruns `system doctor --json`.
15. If doctor reports existing valid charter truth, the skill refuses overwrite and surfaces the compiler-owned next safe action.
16. The conversation layer only gathers charter facts and normalizes them into one structured YAML input file.
17. The runtime writes each run artifact under `~/.local/state/system/intake/runs/<timestamp>/`, including:
    - `doctor.before.json`
    - `doctor.after_setup.json` when setup ran
    - `doctor.after_write.json`
    - `charter_inputs.yaml`
    - `validate.result.txt` or structured equivalent
    - `author.result.txt` or structured equivalent
18. Before any write, the skill calls `system author charter --validate --from-inputs <path|->`.
19. `system author charter --validate --from-inputs <path|->`:
    - parses the same YAML as the real write path
    - runs the same structured-input validation
    - runs the same repo preflight
    - performs no mutation
20. Only after validation succeeds does the skill call `system author charter --from-inputs <path|->`.
21. `system author charter --from-inputs <path|->` performs a deterministic compiler-owned render/write with no `codex exec` dependency.
22. Guided `system author charter` remains the only charter path that may still invoke Codex.
23. After a successful write, the skill reruns `system doctor --json` and reports that charter is complete while other baseline artifacts may still remain.
24. `tools/ci/install-smoke.sh` proves fresh install, refresh, and stale-runtime refusal for the installed Codex packaging surface.
25. One bounded live Codex smoke proves that the installed `system-charter-intake` skill can complete the happy path against a fresh temp repo.

## Architecture Review

### Layered ownership

The important architecture change is not another runtime.

It is a clean three-layer split:

```text
repo source assets
  |
  +--> tools/codex/                   (installer, relink, dev setup, templates)
  +--> core/library/authoring/        (shared authoring method assets)
  +--> core/library/charter/          (shared charter assets)
  |
  v
generated Codex layer
  |
  +--> .agents/skills/system-charter-intake/
  +--> .agents/skills/system/
  |
  v
installed home layer
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

### Runtime execution flow

```text
Codex invokes system-charter-intake
  |
  v
resolve SYSTEM_CODEX_ROOT
  |
  +--> repo-local .agents/skills/system/ override exists? use it
  +--> else use ~/.codex/skills/system/
  |
  v
check runtime-manifest.json and `system --version`
  |
  v
resolve target repo root
  |
  v
system doctor --json
  |
  +--> invalid/missing root? -> system setup -> system doctor --json
  +--> existing charter complete? -> refuse overwrite
  +--> otherwise continue
  |
  v
conversational capture -> charter_inputs.yaml
  |
  v
system author charter --validate --from-inputs
  |
  +--> refusal? stop with exact repair action
  |
  v
system author charter --from-inputs
  |
  v
system doctor --json
  |
  v
persist run artifacts and report next safe action
```

### Ownership map

| Area | Ownership | Required change |
| --- | --- | --- |
| `crates/compiler/src/doctor.rs` | compiler | expose a JSON-serializable doctor contract without changing doctor semantics |
| `crates/cli/src/main.rs` | CLI | add `doctor --json`; add `author charter --validate --from-inputs`; keep human-readable defaults stable |
| `crates/compiler/src/author/charter.rs` | compiler | split guided Codex synthesis from deterministic structured-input render/write |
| `crates/compiler/src/author/project_context.rs` | compiler reference | no redesign; use as the deterministic authoring template |
| `crates/compiler/tests/doctor.rs` | compiler tests | prove doctor JSON fields, exit behavior, and non-ready semantics |
| `crates/compiler/tests/author.rs` | compiler tests | prove validation-only mode, deterministic sink, overwrite refusal, and no-Codex regression |
| `crates/cli/tests/author_cli.rs` + `help_drift_guard.rs` | CLI tests | prove new flag parsing, output wording, help/docs parity, and exit codes |
| `tools/codex/` | packaging/install runtime | add install, relink, dev-setup, generator, runtime manifest, and runtime entrypoint |
| `.agents/skills/` | generated Codex assets | generate the discoverable skill and the shared runtime root |
| `tools/ci/install-smoke.sh` + live smoke helpers | smoke harness | extend from CLI install smoke to Codex packaging install/update/runtime smoke |
| `README.md`, `docs/`, `docs/contracts/`, `DESIGN.md` | docs/contracts | align the new packaging story with the existing CLI trust story |

## Implementation Plan

### Workstream 1: CLI and compiler contract hardening

This is the first hard gate. The installed skill cannot ship honestly until these surfaces are real.

Deliverables:

- add `doctor --json` to the existing `doctor` command
- add `--validate` to `system author charter`, valid only alongside `--from-inputs <path|->`
- make `system author charter --from-inputs <path|->` deterministic and compiler-owned
- keep guided `system author charter` as the only charter path allowed to invoke Codex

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

- `doctor --json` emits the locked machine-readable contract
- `author charter --validate --from-inputs` performs full preflight with zero mutation
- deterministic charter `--from-inputs` no longer shells out to `codex exec`

### Workstream 2: Codex generation and runtime-root packaging

This workstream creates the actual Codex product surface. No generic host abstraction is allowed.

Deliverables:

- one handwritten source packaging layer under `tools/codex/`
- generated `.agents/skills/system-charter-intake/`
- generated `.agents/skills/system/`
- shared runtime manifest and runtime entrypoint under the generated root
- exact runtime-root resolution order in the generated skill preamble

Files/modules:

- `tools/codex/generate.sh`
- `tools/codex/install.sh`
- `tools/codex/relink.sh`
- `tools/codex/dev-setup.sh`
- `tools/codex/templates/system-charter-intake.SKILL.md.tmpl`
- `tools/codex/runtime/`
- `.agents/skills/system-charter-intake/` (generated)
- `.agents/skills/system/` (generated)

Exit condition:

- the repo can generate the two Codex outputs deterministically
- the generated skill resolves the shared runtime root correctly
- the runtime manifest expresses exact compatibility with the `system` binary

### Workstream 3: Home install, refresh, and smoke proof

This workstream makes the packaging real instead of repo-local theater.

Deliverables:

- install or refresh of `~/.codex/skills/system-charter-intake/`
- install or refresh of `~/.codex/skills/system/`
- idempotent refresh behavior
- stale-runtime refusal
- fresh install smoke
- one bounded live installed-skill happy-path smoke

Files/modules:

- `tools/codex/install.sh`
- `tools/codex/relink.sh`
- `tools/ci/install-smoke.sh`
- `tools/ci/codex-skill-live-smoke.sh`
- fixture or transcript evidence under `tests/fixtures/` or `.implemented/`

Exit condition:

- a user can install the Codex skill assets into `~/.codex/skills/`
- rerunning install/refresh is idempotent
- stale runtime/binary mismatches are refused before conversation starts

### Workstream 4: Docs, contracts, and cutover proof

This workstream lands last. It tells one product story after the real surfaces exist.

Deliverables:

- docs/help/contract alignment for `doctor --json`, `--validate`, and Codex packaging install
- packaging proof artifacts
- explicit local-only distribution wording
- no conflict between shell installer naming and the `system setup` CLI verb

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
- the distribution story remains local-first and honest
- the install/update/runtime proof is visible to a future implementer without extra context

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
    ├-- [GAP] typed DoctorReport serializes exact locked top-level fields
    ├-- [GAP] non-ready ordinary states still emit valid JSON with non-zero exit
    ├-- [GAP] catastrophic inspection failure emits refusal text, not partial JSON
    └-- [GAP] checklist items keep exact subject/path/action semantics

[+] author charter --validate --from-inputs
    |
    ├-- [GAP] valid YAML -> exit 0, no mutation
    ├-- [GAP] malformed YAML -> refusal, no mutation
    ├-- [GAP] incomplete YAML -> refusal, no mutation
    └-- [GAP] existing valid charter -> refusal before mutation

[+] author charter --from-inputs
    |
    ├-- [GAP] deterministic render/write with no codex exec
    ├-- [GAP] malformed YAML -> refusal
    ├-- [GAP] incomplete YAML -> refusal
    ├-- [GAP] overwrite refusal on existing valid charter
    └-- [GAP] invalid write target -> mutation refused

[+] Codex packaging generation/install
    |
    ├-- [GAP] generate .agents/skills/system-charter-intake deterministically
    ├-- [GAP] generate .agents/skills/system runtime root deterministically
    ├-- [GAP] install copies both roots into ~/.codex/skills/
    └-- [GAP] refresh is idempotent and cleans stale files safely

[+] installed skill runtime
    |
    ├-- [GAP] runtime root fallback order works
    ├-- [GAP] runtime manifest version mismatch refuses early
    ├-- [GAP] missing `system` binary refuses early
    └-- [GAP] run artifacts persist under ~/.local/state/system/intake/runs/
```

### User flow coverage to add

```text
USER FLOW COVERAGE
==================
[+] Fresh install and discovery flow [->E2E]
    |
    ├-- [GAP] install -> ~/.codex/skills/system-charter-intake appears
    └-- [GAP] install -> ~/.codex/skills/system runtime root appears and is readable

[+] Fresh target repo happy path [->E2E]
    |
    ├-- [GAP] doctor --json -> setup -> doctor --json -> validate -> write -> doctor --json
    └-- [GAP] final doctor reports remaining baseline work truthfully

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
- `crates/cli/tests/author_cli.rs`
  - new flag parsing
  - refusal text
  - exit behavior
- `crates/cli/tests/help_drift_guard.rs`
  - help/docs parity for `doctor --json` and `author charter --validate --from-inputs`
- `tools/ci/install-smoke.sh`
  - generate/install/refresh/stale-runtime checks for Codex packaging
- `tools/ci/codex-skill-live-smoke.sh`
  - bounded installed-skill happy path
- proof artifacts
  - fresh install proof
  - fresh target repo happy path
  - existing charter refusal
  - stale runtime refusal

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
  - optional `system setup`
  - one follow-up `system doctor --json`
  - one validation call
  - one write call
  - one final `system doctor --json`
- The install script should copy only the generated skill/runtime assets. It should not compile Rust code itself.
- Run artifacts should stream to disk as they are produced. Do not hold long transcripts or JSON blobs in memory unnecessarily.
- No daemon, file watcher, background sync loop, or persisted readiness cache ships in `M9.5`.
- Install and refresh must be idempotent. A second run should converge on the same home layout without manual cleanup.

## Failure Modes Registry

| Codepath | Realistic production failure | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- | --- |
| generated skill discovery | `~/.codex/skills/system-charter-intake/` missing or half-installed | yes | install/refesh refusal with exact missing path | explicit install failure |
| runtime root resolution | repo-local override missing files or malformed manifest | yes | hard refusal before conversation | exact broken runtime root shown |
| binary/runtime drift | installed skill version does not match `system --version` | yes | early compatibility refusal | explicit "rerun install" guidance |
| target repo detection | skill launched outside a real repo | yes | repo-root refusal | exact failing directory shown |
| doctor JSON | CLI emits prose or invalid JSON for ordinary non-ready states | yes | parse refusal, no guessing | explicit contract failure |
| setup routing | skill skips bare `system setup` after doctor says root missing/invalid | yes | regression-blocking smoke | no write attempt, exact setup action shown |
| charter validation | malformed or incomplete YAML reaches the write path | yes | validation refusal | field-specific repair guidance |
| deterministic charter sink | `--from-inputs` still invokes Codex | yes, critical | regression test | ship blocker |
| existing canonical truth | installed skill overwrites valid authored charter | yes | compiler-owned refusal | exact next action to inspect existing charter |
| run-artifact persistence | skill writes artifacts into `~/.codex/skills/system/` or target repo accidentally | yes | fixed state-path enforcement | no mutable runtime-root writes |
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
| 2. Codex generation and runtime packaging | `tools/codex/`, `.agents/skills/` | frozen command contracts from this plan |
| 3. Home install and smoke proof | `tools/ci/`, packaging fixtures/evidence | Steps 1 and 2 |
| 4. Docs and contract cutover | `README.md`, `docs/`, `DESIGN.md`, `PLAN.md` | Steps 1, 2, and 3 |

### Parallel lanes

- Lane A: Step 1
  - sequential inside the lane because `doctor`, charter authoring, and CLI flag semantics share one trust boundary
- Lane B: Step 2
  - can launch in parallel with Lane A because the file set is separate once the command names and runtime-root policy are frozen
- Lane C: Step 3 -> Step 4
  - sequential after A and B merge because smoke proof and docs must reflect the final implemented interfaces

### Execution order

Launch Lane A and Lane B in parallel worktrees after this plan is approved.

Merge both.

Then run Lane C for smoke proof and docs/contracts cutover.

### Conflict flags

- Lane A and Lane B must not both edit `README.md`, `docs/`, or help snapshots. Reserve those for Lane C.
- Lane A owns `crates/cli/src/` and `crates/compiler/src/author/charter.rs`. Lane B must not touch those modules.
- Lane B owns `tools/codex/` and generated `.agents/skills/`. Lane A must not create packaging logic there.
- Lane C is the only lane allowed to finalize wording in docs, help snapshots, and smoke naming.

Result: 3 lanes total, 2 launchable in parallel, 1 final sequential integration lane.

## Acceptance Criteria

1. `system doctor --json` is documented, tested, and emits the exact locked contract.
2. `system author charter --validate --from-inputs <path|->` is documented, tested, and mutation-free.
3. `system author charter --from-inputs <path|->` is deterministic and does not invoke Codex.
4. Guided `system author charter` still works and remains clearly separate from the deterministic structured-input sink.
5. The repo can generate `.agents/skills/system-charter-intake/` and `.agents/skills/system/` deterministically.
6. `tools/codex/install.sh` installs or refreshes `~/.codex/skills/system-charter-intake/` and `~/.codex/skills/system/`.
7. The installed skill refuses early when the runtime manifest and `system --version` do not match.
8. The installed skill can complete the fresh target repo charter happy path end to end.
9. The installed skill persists run artifacts under `~/.local/state/system/intake/runs/`.
10. The installed skill refuses overwrite of existing valid charter truth.
11. Install/update smoke covers fresh install, refresh, stale-runtime refusal, and live skill happy path.
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
- Code Quality Review: generic host framework rejected, shell-based packaging chosen
- Test Review: full coverage diagram produced, installed-skill smoke and deterministic-sink regression tests required
- Performance Review: no daemon/cache layer allowed, runtime shell-out count bounded
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 0 new items proposed
- Failure modes: written, with deterministic sink, readiness guessing, and version drift blockers called out
- Outside voice: checkpoint + design doc + local `gstack` packaging inspection incorporated; no fresh dual-model challenge was run in this rewrite
- Parallelization: 3 lanes, 2 parallel / 1 sequential
- Lake Score: 9/9 major recommendations chose the complete option over the shortcut

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | carried forward | checkpoint + latest design narrowed the wedge to Codex-first, charter-only packaging |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 1 | rewritten plan | scope challenge, architecture, test diagram, failure modes, and parallelization are all refreshed for `M9.5` |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | skipped, no UI scope | — |

**VERDICT:** Fresh `M9.5` implementation contract is ready. If you want an additional independent challenge pass before coding starts, re-run `/autoplan` against this rewritten plan.
