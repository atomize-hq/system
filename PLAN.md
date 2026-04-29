<!-- PLAN rewrite restore point: /Users/spensermcconnell/.gstack/projects/atomize-hq-system/feat-m8-plan-rewrite-restore-20260428-204203.md -->
# PLAN

## Status

This is the consolidated execution plan for `M9`, the conversational intake milestone on branch `feat/m8`.

The branch name still says `m8` because this work builds directly on the shipped M8 baseline-authoring foundation. The active milestone is `M9`.

This pass replaces the earlier accreted review-style plan text with one implementation contract. It folds in:

- the design doc at `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-feat-m8-design-20260426-145728.md`
- the latest eng-review test plan at `/Users/spensermcconnell/.gstack/projects/atomize-hq-system/spensermcconnell-feat-m8-eng-review-test-plan-20260428-192306.md`
- the prior `/autoplan` and `/plan-eng-review` scope reductions already recorded for this branch
- the gstack pattern synthesis references in:
  - `/Users/spensermcconnell/__Active_Code/system/substrate_gstack_pattern_synthesis_consolidated_2026-04-18 (1).md`
  - `/Users/spensermcconnell/__Active_Code/system/substrate_gstack_pattern_synthesis_consolidated_2026-04-18 (1).json`

## Active Objective

Ship `M9`, the first real conversational intake surface for baseline authoring on top of the M8 foundation.

The job is not "make the guided interview nicer." The job is:

- let the agent own the conversational capture loop
- keep `system` as the only authority for readiness, validation, refusal, and canonical writes
- prove that the charter intake path can gather better truth conversationally without weakening the compiler-owned trust boundary

## Review Inputs

- Prior CEO/eng review outcome: reduce the first wedge to one agent, one artifact, one deterministic sink, one thin adapter boundary.
- Prior eng review outcome: keep `project-context` as the model for deterministic `--from-inputs`; do not invent a second authoring architecture.
- Current code reality:
  - `system_compiler::doctor()` already returns a typed `DoctorReport`
  - `system author project-context --from-inputs` is already deterministic
  - `system author charter --from-inputs` is not deterministic yet, it still shells out to `codex exec`
  - `tools/intake/` does not exist yet
  - `tools/ci/install-smoke.sh` currently proves CLI install only, not intake-bundle install/update/runtime behavior

## Scope Lock

### In scope

- one supported conversational intake surface: Codex first
- charter authoring only
- `system doctor --json` as the machine-readable readiness surface
- `system author charter --validate --from-inputs <path|->` as a mutation-free preflight
- deterministic `system author charter --from-inputs <path|->`
- explicit split between guided charter authoring and deterministic structured-input charter authoring
- repo-owned conversational intake assets
- thin home-level install/update/runtime boundary under `~/.config/system`, `~/.local/state/system`, and `~/.local/bin/system`
- run-artifact persistence for intake sessions
- docs, contracts, help, smoke proof, and one bounded live Codex happy-path smoke for this shipped surface

### NOT in scope

- `project-context` conversational intake
- `environment-inventory` conversational intake
- Claude Code support in this milestone
- a generic multi-agent framework
- repo-evidence auto-drafting or compiler-side intake inference
- a hosted service, remote API, or background daemon
- package-manager or public binary distribution
- a redesign of the human-guided charter interview beyond the minimum separation needed to keep `--from-inputs` deterministic

## Frozen Decisions

1. `M9` is the post-M8 conversational intake milestone, not more M8 baseline-core work.
2. The first shipped wedge is Codex-only. Claude Code is a follow-on already captured in `TODOS.md`.
3. The only authored artifact in `M9` is `.system/charter/CHARTER.md`.
4. `system doctor` and `system setup` remain the only truth for readiness and repair routing.
5. `system author charter --from-inputs` must become deterministic and Codex-free.
6. Guided `system author charter` may remain Codex-backed in `M9`, but it must no longer share the deterministic sink path.
7. The adapter is allowed to install, version-check, route commands, and persist artifacts. It is not allowed to reimplement readiness, validation, or write semantics.
8. The milestone ships one repo-owned intake bundle plus a thin home-level runtime boundary. It does not ship a second compiler, a second readiness model, or a second schema language.

## Step 0: Scope Challenge

### Current-state diagnosis

The actual seam is narrow and concrete:

- `crates/compiler/src/doctor.rs` already computes the typed baseline report. The missing piece is a first-class JSON CLI surface.
- `crates/compiler/src/author/project_context.rs` already shows the target shape: validate structured input, preflight the repo, render deterministically, write canonically.
- `crates/compiler/src/author/charter.rs` still routes both guided and `--from-inputs` flows through `synthesize_charter_markdown()`, which invokes `codex exec`. That is the core trust-boundary bug this milestone fixes.
- `tools/intake/` does not exist, which means the bundle, installer, wrapper, and happy-path smoke still need to be defined from zero.

This is why the right plan is charter-only plus one agent. Anything larger is an ocean.

### What already exists

| Sub-problem | Existing code or asset | Reuse decision |
| --- | --- | --- |
| baseline readiness computation | `crates/compiler/src/doctor.rs`, `docs/contracts/C-04-resolver-result-and-doctor-blockers.md` | Reuse exactly. Add JSON CLI rendering, do not rebuild readiness logic in the adapter. |
| setup routing | `crates/cli/src/main.rs` + compiler setup flow | Reuse exactly. The adapter may call bare `system setup`, never infer init vs refresh itself. |
| deterministic structured-input authoring pattern | `crates/compiler/src/author/project_context.rs` | Reuse as the implementation model for deterministic charter writes. |
| charter structured-input parser and validator | `crates/compiler/src/author/charter.rs` | Reuse and split. Keep the validation logic, remove Codex from the deterministic path. |
| baseline authoring preflight and refusal posture | compiler authoring helpers in `crates/compiler/src/author/` | Reuse. New surfaces must preserve existing refusal classes and write-target checks. |
| CLI help/test drift posture | `crates/cli/tests/help_drift_guard.rs`, CLI snapshots | Reuse and extend instead of inventing a new docs-proof mechanism. |
| install smoke conventions | `tools/ci/install-smoke.sh` | Reuse and extend instead of building a second smoke harness. |
| repo-owned content assets | `core/library/authoring/`, `core/library/charter/`, `core/schemas/` | Reuse as the source tree for the intake bundle assets. |

### Scope reduction verdict

The minimal complete lake is:

- one agent surface
- one artifact write path
- one machine-readable readiness surface
- one thin adapter/runtime boundary

Anything smaller is fake completeness:

- a prompt wrapper with no `doctor --json`
- a deterministic flag that still shells out to Codex
- a conversation surface with no proof artifacts or version checks

Anything larger is premature:

- multi-agent portability now
- `project-context` authoring now
- environment inventory now
- a shared baseline-intake super-artifact now

### Distribution check

`M9` introduces a user-facing distribution shape, but it stays local-first:

- repo-owned source assets
- installer-managed home assets
- one stable `system` wrapper entrypoint under `~/.local/bin/system`
- no public publishing pipeline yet

Public distribution remains a follow-on TODO, not silent scope creep.

## Exact Shipped Behavior

`M9` is done only when all of the following are true:

1. The repo ships one supported conversational entrypoint for this milestone:
   - `tools/intake/run_codex_charter_intake.sh --repo-root <path>`
2. That entrypoint installs or refreshes home assets under:
   - `~/.config/system/intake/`
   - `~/.local/state/system/intake/`
   - `~/.local/bin/system`
3. The home wrapper at `~/.local/bin/system` is thin. It resolves the real CLI, prepares the runtime, and dispatches. It does not duplicate compiler logic.
4. The adapter performs version-compatibility checks before starting a run and refuses if the installed home assets are stale or incompatible with the repo-owned assets.
5. The adapter always calls `system doctor --json` before any baseline questioning.
6. If doctor reports a missing or invalid `.system` root, the adapter runs bare `system setup`, then reruns `system doctor --json`.
7. If doctor reports existing valid charter truth, the adapter refuses overwrite and surfaces the compiler-owned next action.
8. The conversation layer captures and normalizes charter inputs only. It does not write canonical truth directly.
9. The adapter writes normalized YAML to a temp/run path under `~/.local/state/system/intake/runs/<timestamp>/`.
10. The adapter calls `system author charter --validate --from-inputs <path|->` before any write attempt.
11. `system author charter --validate --from-inputs <path|->`:
    - parses exactly as `--from-inputs` does
    - runs the same structured-input validation rules
    - runs the same repo preflight that `--from-inputs` uses
    - performs no mutation
12. The adapter calls `system author charter --from-inputs <path|->` only after validation succeeds.
13. `system author charter --from-inputs <path|->` performs a deterministic compiler-owned render/write with no `codex exec` dependency.
14. Guided `system author charter` remains available and Codex-backed, but it is a separate codepath from deterministic `--from-inputs`.
15. `system doctor --json` emits one UTF-8 JSON object to stdout on success and ordinary non-ready states. The top-level object includes exactly:
    - `c04_result_version`
    - `c03_schema_version`
    - `c03_manifest_generation_version`
    - `baseline_state`
    - `status`
    - `system_root_status`
    - `checklist`
    - `blockers`
    - `next_safe_action`
16. Every checklist item in that JSON includes exactly:
    - `artifact_label`
    - `subject`
    - `author_command`
    - `kind`
    - `canonical_repo_relative_path`
    - `status`
    - `next_safe_action`
17. Ordinary non-ready states still emit valid JSON plus a non-zero exit code.
18. After a successful write, the adapter reruns `system doctor --json` and reports that charter is complete while `project-context` and `environment-inventory` may still remain.
19. The adapter persists a run artifact containing at minimum:
    - `doctor.before.json`
    - `doctor.after_setup.json` when setup ran
    - `doctor.after_write.json`
    - `charter_inputs.yaml`
    - `validate_inputs.result.json` or equivalent structured result
    - `author_from_inputs.result.json` or equivalent structured result
20. The repo ships proof for:
    - a fresh repo happy path
    - an existing-authored-charter refusal
    - a broken `.system` repair path
    - install/update smoke
    - one live Codex happy-path smoke for the shipped intake surface

## Architecture Review

### System boundary

The compiler stays the trust boundary. The conversation layer is just capture and routing.

```text
human
  |
  v
Codex intake bundle
  |
  v
home-level intake adapter
  |
  +--> system doctor --json
  +--> system setup
  +--> system author charter --validate --from-inputs
  +--> system author charter --from-inputs
  |
  v
compiler-owned readiness + refusal + deterministic write
  |
  v
.system/charter/CHARTER.md
```

### Command/data-flow

```text
operator -> run_codex_charter_intake.sh --repo-root <path>
            |
            v
      install/update preflight
            |
            v
      system doctor --json
            |
            +--> missing/invalid root? -> system setup -> system doctor --json
            +--> valid charter already exists? -> refuse overwrite
            +--> otherwise continue
            |
            v
      conversational capture
            |
            +--> examples
            +--> clarifying follow-ups
            +--> completeness checks
            +--> normalize to charter_inputs.yaml
            |
            v
      system author charter --validate --from-inputs <path|->
            |
            +--> refusal? surface exact broken fields and stop
            |
            v
      system author charter --from-inputs <path|->
            |
            +--> deterministic render/write
            +--> canonical .system/charter/CHARTER.md
            |
            v
      system doctor --json
            |
            +--> truthfully report remaining baseline work
            +--> persist run artifacts
```

### Ownership map

| Area | Ownership | Required change |
| --- | --- | --- |
| `crates/compiler/src/doctor.rs` | compiler | keep `DoctorReport` semantics stable, expose JSON-serializable contract cleanly |
| `crates/cli/src/main.rs` | CLI | add `doctor --json`; add `author charter --validate --from-inputs`; keep human-readable default output intact |
| `crates/compiler/src/author/charter.rs` | compiler | split guided Codex synthesis from deterministic `--from-inputs`; preserve preflight/refusal logic |
| `crates/compiler/src/author/project_context.rs` | compiler reference | no redesign, use as the deterministic pattern to copy |
| `crates/compiler/tests/author.rs` | compiler tests | prove deterministic charter sink, validation-only surface, overwrite refusal, malformed/incomplete input refusals |
| `crates/compiler/tests/doctor.rs` | compiler tests | prove JSON contract fields, status behavior, blockers, and checklist semantics |
| `crates/cli/tests/author_cli.rs` + `help_drift_guard.rs` | CLI tests | prove new flags, exit behavior, and docs/help parity |
| `core/library/authoring/`, `core/library/charter/`, `core/schemas/` | repo-owned protocol assets | add intake protocol, charter examples, and structured-input schema |
| `tools/intake/` | adapter/runtime | add installer, runtime entrypoint, and Codex happy-path launcher |
| `tools/ci/install-smoke.sh` | smoke harness | extend from plain CLI install smoke to intake-bundle install/update/runtime smoke |
| `README.md`, `docs/`, `docs/contracts/`, `DESIGN.md` | docs/contracts | align the public command story, trust boundary, and proof expectations |

## Implementation Plan

### Workstream 1: Compiler sink hardening

This is the first hard gate. Nothing else is allowed to invent around it.

Deliverables:

- add `--json` to the existing `doctor` command, not a second command name
- add `--validate` to `system author charter`, valid only alongside `--from-inputs <path|->`
- keep `--from-inputs <path|->` as the source selector and `--validate` as the mutation-free mode selector
- move deterministic charter rendering/writing onto a compiler-owned non-Codex path
- keep guided `system author charter` as the only charter flow that may still invoke Codex

Files/modules:

- `crates/cli/src/main.rs`
- `crates/compiler/src/doctor.rs`
- `crates/compiler/src/author/charter.rs`
- `crates/compiler/tests/doctor.rs`
- `crates/compiler/tests/author.rs`
- `crates/cli/tests/author_cli.rs`
- `crates/cli/tests/help_drift_guard.rs`

Exit condition:

- charter `--from-inputs` no longer shells out to `codex exec`
- `doctor --json` is stable and covered
- `--validate --from-inputs` proves preflight without mutation

### Workstream 2: Intake bundle and home runtime

This workstream may start once Workstream 1 command contracts are written down, but it must not redefine them.

Deliverables:

- repo-owned protocol asset for the conversational intake method
- repo-owned charter example/coaching asset
- repo-owned structured-input schema for the bundle/runtime
- installer that copies or refreshes home assets
- home wrapper/runtime that:
  - verifies install/version compatibility
  - targets one repo root explicitly
  - writes run artifacts under `~/.local/state/system/intake/runs/`
  - calls `system`, maps exit codes, and surfaces exact refusals
- launcher script for the supported Codex entrypoint

Files/modules:

- `core/library/authoring/conversational_intake_protocol.md`
- `core/library/authoring/conversational_intake_charter_examples.md`
- `core/schemas/charter_structured_input.schema.json`
- `tools/intake/install.sh`
- `tools/intake/runtime.sh`
- `tools/intake/run_codex_charter_intake.sh`

Exit condition:

- a fresh machine/user can install the intake assets
- the adapter refuses stale installs clearly
- the adapter can execute the happy-path command loop with exact run artifacts

### Workstream 3: Docs, proofs, and cutover

This workstream waits until the CLI contract and adapter contract are real.

Deliverables:

- docs/help/contract alignment for the new surfaces
- install/update smoke coverage
- transcript or fixture proof for happy path, overwrite refusal, and broken-root repair
- one bounded live Codex smoke for the shipped happy path

Files/modules:

- `README.md`
- `docs/SUPPORTED_COMMANDS.md`
- `docs/START_HERE.md`
- `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- `docs/contracts/C-04-resolver-result-and-doctor-blockers.md`
- `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
- `DESIGN.md`
- `tools/ci/install-smoke.sh`
- `tests/fixtures/` or `.implemented/` evidence directory, whichever best fits the shipped proof posture

Exit condition:

- the docs tell the same story as the CLI
- install smoke covers the home runtime boundary
- proof artifacts exist for each required operator path

## Code Quality Review

### Boring-by-default rules

- Do not build a generic "baseline intake engine."
- Do not create a second readiness model in `tools/intake/`.
- Do not introduce a second schema language if the Rust structured-input contract already defines the truth.
- Do split the guided charter path from deterministic `--from-inputs` cleanly. That split is the whole point.

### DRY rules

- Reuse the `project-context` deterministic authoring pattern for charter.
- Reuse existing refusal formatting where new CLI surfaces need operator-facing error output.
- Reuse the existing help drift and smoke harnesses instead of making sidecar proof systems.

### Required inline diagrams during implementation

- `crates/compiler/src/author/charter.rs`
  - guided vs deterministic charter flow
- `crates/compiler/src/doctor.rs`
  - typed readiness report -> human output / JSON output ownership boundary
- `tools/intake/runtime.sh` or equivalent runtime entrypoint
  - install preflight -> doctor -> setup -> validate -> write -> doctor loop

## Test Review

### Framework baseline

No new test framework is needed.

Current layers are already right:

- compiler tests under `crates/compiler/tests/`
- CLI surface tests under `crates/cli/tests/`
- shell smoke under `tools/ci/install-smoke.sh`

### Code path coverage to add

```text
CODE PATH COVERAGE
==================
[+] doctor --json
    |
    ├-- [GAP] valid baseline report serializes exact top-level fields
    ├-- [GAP] scaffolded report serializes with non-zero exit at CLI layer
    ├-- [GAP] invalid-root report still emits JSON on ordinary non-ready states
    └-- [GAP] catastrophic inspection failure emits refusal text instead of partial JSON

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
    ├-- [GAP] existing valid charter -> refuse overwrite
    └-- [GAP] invalid write target -> mutation refused

[+] guided author charter
    |
    ├-- [GAP] remains the only charter path allowed to use Codex
    └-- [GAP] regression: guided/runtime logic must not leak into deterministic path
```

### User flow coverage to add

```text
USER FLOW COVERAGE
==================
[+] Fresh repo intake flow [->E2E]
    |
    ├-- [GAP] doctor --json -> setup -> doctor --json -> validate -> write -> doctor --json
    └-- [GAP] final doctor still reports project-context + environment-inventory as remaining

[+] Existing authored charter flow
    |
    └-- [GAP] adapter refuses overwrite and surfaces compiler-owned next action

[+] Broken .system flow [->E2E]
    |
    └-- [GAP] doctor detects invalid root, adapter routes through setup, reruns doctor

[+] Install/update flow
    |
    ├-- [GAP] missing ~/.local/bin/system shim -> install smoke fails clearly
    └-- [GAP] stale home asset version -> adapter refuses before conversation starts

[+] LLM-backed conversation handoff [->EVAL]
    |
    └-- [GAP] one bounded live Codex smoke proves the shipped happy path
```

### Required tests and proofs

- `crates/compiler/tests/doctor.rs`
  - JSON contract and checklist field coverage
  - blocker and next-safe-action behavior
- `crates/compiler/tests/author.rs`
  - validation-only charter path
  - deterministic charter `--from-inputs`
  - regression that deterministic charter path does not invoke Codex
- `crates/cli/tests/author_cli.rs`
  - new flag parsing, refusal text, and exit behavior
- `crates/cli/tests/help_drift_guard.rs`
  - help/docs parity for `doctor --json` and `author charter --validate --from-inputs`
- `tools/ci/install-smoke.sh`
  - home install/update/runtime checks
- proof artifacts
  - fresh repo happy path
  - existing charter refusal
  - broken-root repair
  - one live Codex happy-path smoke

### Regression rules

- The current Codex dependency in charter `--from-inputs` is a regression gap. The fix is not optional and the regression test is mandatory.
- Any adapter behavior that guesses readiness without `system doctor --json` is a regression against the trust boundary and must be caught by smoke or integration proof.

## Performance and Reliability Review

This milestone is not CPU-heavy, but it still has sharp operational boundaries.

- The adapter must not walk the repo to infer readiness. It should pay the cost of calling `system`, not invent a cache.
- The adapter should execute at most:
  - one install/version preflight
  - one initial `doctor --json`
  - optional `system setup`
  - one follow-up `doctor --json`
  - one validation call
  - one write call
  - one final `doctor --json`
- Run artifacts should stream to disk as they are produced. Do not hold large transcripts in memory.
- No long-lived daemon, file watcher, or persisted readiness cache ships in `M9`.
- Ordinary refusal and non-ready states must remain fast, explicit, and finite.

## Failure Modes Registry

| Codepath | Realistic production failure | Test required | Error handling required | User-visible outcome |
| --- | --- | --- | --- | --- |
| install preflight | stale or missing home asset version | yes | adapter refusal before questioning | explicit compatibility refusal |
| repo targeting | wrong `--repo-root` or missing repo root | yes | explicit repo-root refusal | exact failing path shown |
| doctor JSON | text output accidentally treated as JSON | yes | hard refusal on invalid JSON payload | no guessing, exact failure surfaced |
| setup routing | adapter skips bare `system setup` after doctor says root missing/invalid | yes | regression-blocking smoke or integration test | no write attempt, exact setup action shown |
| charter YAML emission | vague or malformed structured YAML | yes | validation refusal before write | field-specific repair guidance |
| deterministic charter sink | `--from-inputs` still invokes Codex | yes, critical | regression test in compiler/CLI coverage | ship blocker |
| existing canonical truth | overwrite of valid authored charter | yes | compiler-owned refusal | exact next action to inspect existing charter |
| final handoff | adapter claims baseline complete after charter-only completion | yes | final doctor rerun is mandatory | remaining work shown clearly |
| logging | sensitive charter values echoed noisily to stdout | yes | bounded logging/run-artifact policy | sanitized operator output |

Critical gap rule: any path that writes canonically without doctor truth, without validation, or without deterministic compiler ownership is a ship blocker.

## TODOS Cross-Reference

Existing TODOs already capture the correct follow-ons:

- `Claude Code Conversational Intake Surface`
- `Public CLI Distribution`
- `CLI Release Workflow`
- broader `Live Authoring Smoke Coverage`

This plan does not add new TODOs. The right move is to finish the charter-only Codex-first wedge cleanly before widening the surface area.

## Worktree Parallelization Strategy

### Dependency table

| Step | Modules touched | Depends on |
| --- | --- | --- |
| 1. Compiler sink hardening | `crates/compiler/src/author/`, `crates/compiler/src/doctor.rs`, `crates/cli/src/`, compiler/CLI tests | --- |
| 2. Intake bundle and home runtime | `core/library/authoring/`, `core/library/charter/`, `core/schemas/`, `tools/intake/` | Step 1 command contracts written down |
| 3. Docs/help/smoke cutover | `README.md`, `docs/`, `DESIGN.md`, `tools/ci/`, proof artifacts | 1 and 2 |

### Parallel lanes

- Lane A: Step 1
  - sequential inside the lane because CLI routing, doctor output, and charter sink ownership all touch the same trust boundary
- Lane B: Step 2
  - can launch in parallel once Lane A's command contracts are locked in the plan
- Lane C: Step 3
  - sequential after A and B merge because docs, help drift, install smoke, and proof artifacts must reflect the final shipped interfaces

### Execution order

Launch Lane A and Lane B in parallel worktrees after this plan is approved.

Merge both.

Then run Lane C for docs, smoke, and proof cutover.

### Conflict flags

- Lane A and Lane B must not both edit `README.md`, `docs/`, or help snapshots. Reserve those for Lane C.
- Lane A owns `crates/cli/src/` and compiler authoring/doctor code. Lane B must not touch those modules.
- Lane B owns `core/library/authoring/`, `core/schemas/`, and `tools/intake/`. Lane A must not create duplicate runtime assets there.

Result: 3 lanes total, 2 launchable in parallel, 1 final sequential integration lane.

## Acceptance Criteria

1. `system doctor --json` is documented, tested, and emits the exact locked contract.
2. `system author charter --validate --from-inputs <path|->` is documented, tested, and mutation-free.
3. `system author charter --from-inputs <path|->` is deterministic and does not invoke Codex.
4. Guided `system author charter` still works and is clearly separated from the deterministic sink.
5. The Codex intake bundle can complete the fresh-repo charter flow end to end.
6. The adapter persists run artifacts under `~/.local/state/system/intake/runs/`.
7. Install/update checks cover `~/.config/system`, `~/.local/state/system`, and `~/.local/bin/system`.
8. The adapter refuses overwrite of existing valid charter truth.
9. The final doctor handoff truthfully shows remaining baseline work after charter-only completion.
10. Docs, help text, contracts, smoke scripts, and proof artifacts all describe the same shipped command story.

## Unresolved Decisions

None.

The remaining work is execution. The plan is intentionally specific enough that the next choices should be implementation details, not scope debates.

## Completion Summary

- Step 0: Scope Challenge, accepted as charter-only, one-agent, deterministic-sink-first
- Architecture Review: compiler-owned truth boundary retained, adapter kept thin
- Code Quality Review: generic intake framework explicitly rejected
- Test Review: full coverage diagram produced, deterministic-sink regression test required
- Performance Review: no daemon/cache layer allowed, command count bounded
- NOT in scope: written
- What already exists: written
- TODOS.md updates: 0 new items proposed
- Failure modes: written, with deterministic-sink and readiness-guessing blockers called out
- Outside voice: prior `/autoplan` already ran; this consolidation carries forward its accepted scope reductions
- Parallelization: 3 lanes, 2 parallel / 1 sequential
- Lake Score: 7/7 major recommendations chose the complete option over the shortcut

## GSTACK REVIEW REPORT

| Review | Trigger | Why | Runs | Status | Findings |
|--------|---------|-----|------|--------|----------|
| CEO Review | `/plan-ceo-review` | Scope & strategy | 1 | issues_open via `/autoplan` | 5 proposals, 1 accepted, 4 deferred |
| Codex Review | `/codex review` | Independent 2nd opinion | 0 | — | — |
| Eng Review | `/plan-eng-review` | Architecture & tests (required) | 3 | clean | latest run: 29 issues, 0 critical gaps |
| Design Review | `/plan-design-review` | UI/UX gaps | 0 | — | — |

**CROSS-MODEL:** Prior `/autoplan` dual-voice review converged on the main scope reduction: one agent first, charter only, deterministic compiler sink, no generic framework.

**UNRESOLVED:** 0 in the latest direct eng-review run on `2026-04-28`.

**VERDICT:** ENG CLEARED. CEO review narrowed scope but does not block implementation. This plan reflects the narrowed implementation contract and is ready to execute.
