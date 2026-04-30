# M10 Orchestration Plan

## Summary
- Parent integrator starts from the current branch `feat/m10` in `/Users/spensermcconnell/__Active_Code/system`, but does not perform final merges or final verification there.
- Recommended worktree root under this repo: `/Users/spensermcconnell/__Active_Code/system/.worktrees/m10`.
- Parent-only integration branch and worktree:
  - branch: `feat/m10-integrate`
  - worktree: `/Users/spensermcconnell/__Active_Code/system/.worktrees/m10/integrate`
- Worker branches:
  - `feat/m10-source-gen`
  - `feat/m10-install-home`
  - `feat/m10-docs-cutover`
  - `feat/m10-smokes`
- Orchestration run artifacts live only under `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/`. These are run artifacts, not authored source, not product payload, and not long-term contract files.
- Parent-maintained orchestration source of truth:
  - task queue / status tracker: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/task-queue.md`
  - session log / handoff log: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/session-log.md`
  - per-task sentinels / completion markers: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/sentinels/`
  - final verification record: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/final-verification.md`
- Queue discipline:
  - `task-queue.md` is the only orchestration state authority.
  - `session-log.md` is append-only narrative and handoff evidence.
  - sentinel files are convenience markers only. They never override queue state.
- Execution shape:
  - serial setup and `M10-W1`
  - parallel `M10-W2`, `M10-W3`, `M10-W4`
  - parent-only integration and final verification in `feat/m10-integrate`
  - only after green integration does parent fast-forward or merge back to `feat/m10`

## Hard Guards
- `~/system/` is the real installed home. `~/.codex/skills/system*` is never the installed home.
- Repo `.agents/skills/*` must stay thin generated projections. They are never manually authored and never treated as runtime payload.
- `~/.codex/skills/*` must become thin discovery links into `~/system/.agents/skills/*`.
- `bash tools/codex/generate.sh` owns repo outputs only and must never mutate `$HOME`, `~/system/`, or `~/.codex/skills/`.
- `bash tools/codex/install.sh` owns installed-home and discovery outputs only. It must not compile Rust.
- `install.sh` must require `system` on `PATH`, compare that binary version against repo `VERSION`, then copy the verified binary into `~/system/bin/system`.
- Runtime payload belongs only under `~/system/`, including `runtime-manifest.json`, `share/**`, and `bin/system-charter-intake` if that helper remains.
- `system doctor --json` remains the only machine-parsed output. Any schema or contract drift is an immediate stop.
- Mutable run evidence stays under `~/.local/state/system/intake/runs/`, never under `~/system/` and never under `~/.codex/skills/`.
- Required docs and smokes are fixed scope:
  - `README.md`
  - `DESIGN.md`
  - `docs/START_HERE.md`
  - `docs/SUPPORTED_COMMANDS.md`
  - `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
  - `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
  - `tools/ci/install-smoke.sh`
  - `tools/ci/codex-skill-live-smoke.sh`
- Any lane that runs `install.sh`, live smokes, or otherwise mutates install/discovery state must use an isolated `HOME` and isolated `XDG_STATE_HOME`. Worker lanes must not touch the real default home.
- The real default home is reserved strictly for the parent’s final merged verification pass in `feat/m10-integrate`.
- Stop immediately and return to parent if a lane would:
  - touch a file outside its write ownership
  - alter operator-visible CLI behavior instead of topology
  - make `generate.sh` write outside the repo
  - make `install.sh` compile Rust or skip the PATH/version gate
  - change `system doctor --json`
  - put mutable evidence under `~/system/` or `~/.codex/skills/`

## Workstream Plan
Parent-only critical path and gates:
1. `M10-P0` Preflight on `feat/m10`: confirm branch, capture `git status --short`, create orchestration artifact root, and seed `task-queue.md` plus `session-log.md`.
2. `M10-P1` Create all worktrees, but start only `M10-W1`. `M10-W1` must run alone because it freezes the canonical source locations, the generator ownership boundary, and the thin-projection shape that every downstream lane assumes.
3. `M10-G1` Parent gate: review `M10-W1`, merge it into `feat/m10-integrate`, rerun `bash tools/codex/generate.sh`, and freeze the source/generator contract before any install, docs, or smoke work proceeds.
4. `M10-P2` Rebase `feat/m10-install-home`, `feat/m10-docs-cutover`, and `feat/m10-smokes` onto `feat/m10-integrate`, then launch `M10-W2`, `M10-W3`, and `M10-W4` in parallel.
5. `M10-G2` Parent gate: do not merge docs or smokes until install topology is proven against isolated-home verification and the `~/system/` contract is stable.
6. `M10-P3` Parent-only integration in `feat/m10-integrate`: merge worker branches, resolve all conflicts locally, and run the full required verification set there.
7. `M10-G3` Landing gate: only after `feat/m10-integrate` is fully green does parent merge or fast-forward back to `feat/m10`, update final evidence, and declare the session complete.

Recommended setup commands:
```bash
mkdir -p /Users/spensermcconnell/__Active_Code/system/.worktrees/m10
mkdir -p /Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/sentinels

git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/integrate -b feat/m10-integrate feat/m10
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/source-gen -b feat/m10-source-gen feat/m10
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/install-home -b feat/m10-install-home feat/m10
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/docs-cutover -b feat/m10-docs-cutover feat/m10
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/smokes -b feat/m10-smokes feat/m10
```

Bounded worker lanes with disjoint write ownership:

| Lane | Branch | Task ids | Write ownership | Start gate | Exit condition |
| --- | --- | --- | --- | --- | --- |
| `M10-W1` Source + Generator | `feat/m10-source-gen` | `M10-W1.1` canonical root/leaf skill truth, `M10-W1.2` generator rewrite, `M10-W1.3` thin-projection guardrails | `SKILL.md.tmpl`, `SKILL.md`, `charter-intake/SKILL.md.tmpl`, `charter-intake/SKILL.md`, `agents/openai.yaml`, `tools/codex/generate.sh`, `tools/codex/runtime/SKILL.md.tmpl`, `tools/codex/templates/system-charter-intake.SKILL.md.tmpl` | `M10-P1` | `bash tools/codex/generate.sh` regenerates thin repo `.agents/skills/system*` with no `bin/`, `runtime-manifest.json`, or `share/` in repo projections |
| `M10-W2` Install Topology | `feat/m10-install-home` | `M10-W2.1` install to `~/system/`, `M10-W2.2` thin Codex discovery, `M10-W2.3` PATH/version gate, `M10-W2.4` helper/runtime payload placement, `M10-W2.5` dev override preservation | `tools/codex/install.sh`, `tools/codex/dev-setup.sh`, `tools/codex/relink.sh`, `tools/codex/runtime/runtime-manifest.json.tmpl`, `tools/codex/runtime/bin/system-charter-intake.tmpl` | `M10-G1` | isolated-home install flow creates curated `~/system/`, installs thin `~/system/.agents/skills/*`, and restores `~/.codex/skills/* -> ~/system/.agents/skills/*` on normal install |
| `M10-W3` Docs + Contracts | `feat/m10-docs-cutover` | `M10-W3.1` installed-home wording, `M10-W3.2` install ownership wording, `M10-W3.3` machine-parsed output wording | `README.md`, `DESIGN.md`, `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`, `docs/contracts/C-07-conformance-rails-and-docs-cutover.md` | `M10-G1` | every required doc says `~/system/` is the installed home, `.agents/skills/*` is thin, `~/.codex/skills/*` is discovery glue only, and `system doctor --json` is the only machine-parsed surface |
| `M10-W4` Smokes | `feat/m10-smokes` | `M10-W4.1` install smoke topology assertions, `M10-W4.2` live smoke runtime-root assertions, `M10-W4.3` dev-setup crossover assertions, `M10-W4.4` run-evidence assertions | `tools/ci/install-smoke.sh`, `tools/ci/codex-skill-live-smoke.sh` | `M10-G1` | smokes fail on wrong repo projection shape, wrong `~/system/` file set, wrong discovery links, wrong runtime root, or misplaced run evidence |

Lane-local working commands:
- `M10-W1`
```bash
bash tools/codex/generate.sh
find .agents/skills/system -maxdepth 4 -print | sort
find .agents/skills/system-charter-intake -maxdepth 4 -print | sort
```
- `M10-W2`
```bash
export HOME="$PWD/.tmp/home"
export XDG_STATE_HOME="$PWD/.tmp/state"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
mkdir -p "$HOME" "$XDG_STATE_HOME" "$CARGO_HOME"
cargo install --locked --force --path crates/cli
bash tools/codex/install.sh
system doctor --json
```
- `M10-W3`
```bash
rg -n "~/.codex/skills|packaging-only|doctor --json|~/system/|\\.agents/skills/" README.md DESIGN.md docs/START_HERE.md docs/SUPPORTED_COMMANDS.md docs/contracts/C-02-rust-workspace-and-cli-command-surface.md docs/contracts/C-07-conformance-rails-and-docs-cutover.md
```
- `M10-W4`
```bash
export HOME="$PWD/.tmp/home"
export XDG_STATE_HOME="$PWD/.tmp/state"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
mkdir -p "$HOME" "$XDG_STATE_HOME" "$CARGO_HOME"
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh
```

Worker return contract:
- Every worker returns only:
  - changed files
  - commands run with exit codes
  - blockers and assumptions
- Parent records that return into `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/session-log.md` and updates `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/task-queue.md`.
- Parent reviews only the worker summary and a narrow diff:
```bash
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/integrate diff --stat feat/m10-integrate...feat/m10-source-gen
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/integrate diff --stat feat/m10-integrate...feat/m10-install-home
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/integrate diff --stat feat/m10-integrate...feat/m10-docs-cutover
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/integrate diff --stat feat/m10-integrate...feat/m10-smokes
```
- Parent does not rely on full worker transcripts as the review artifact.

Blocked behavior and partial-progress handling:
- If a worker blocks after partial progress, it stops immediately, commits nothing further, and returns its summary under the worker return contract.
- Parent marks the task `BLOCKED` in `task-queue.md`, appends the handoff in `session-log.md`, and writes a sentinel such as `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/sentinels/M10-W2.3.blocked`.
- Parent then chooses exactly one next action:
  - resolve locally in `feat/m10-integrate`
  - narrow the scope and relaunch the same lane
  - open a replacement lane on a new branch from `feat/m10-integrate`
- Partial progress is never silently carried across lanes without a parent decision and queue update.

Why the serialization boundary exists:
- `M10-W1` must run alone because it determines the canonical authored-source paths, whether legacy template paths are removed or retained, and the exact thin-projection contract that install logic, docs wording, and smoke expectations all consume.
- Final integration stays parent-only because install topology, docs vocabulary, helper retention, and smoke assertions all converge on the same contract. One integrator prevents parallel reinterpretation of the milestone and keeps creative conflict resolution local to the parent.

## Context-Control Rules
- Parent is the only agent allowed to reinterpret M10 scope, change file ownership boundaries, decide helper retention for `bin/system-charter-intake`, or resolve cross-lane conflicts.
- Workers get only the plan slice they need plus their owned files. They do not reopen unrelated M10 decisions.
- Workers do not edit `.agents/skills/*` directly. They edit authored sources or scripts, then regenerate.
- Workers do not edit `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/*`. Those orchestration files are parent-maintained run artifacts only.
- Any worker lane that runs `install.sh`, `dev-setup.sh`, `relink.sh`, install smoke, live smoke, or any command that mutates install/discovery state must use an isolated `HOME` and isolated `XDG_STATE_HOME`.
- Worker-safe isolation pattern:
```bash
export HOME="$PWD/.tmp/home"
export XDG_STATE_HOME="$PWD/.tmp/state"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
mkdir -p "$HOME" "$XDG_STATE_HOME" "$CARGO_HOME"
```
- Worker lanes must not inspect or mutate the real `~/system/`, `~/.codex/skills/`, or the real `~/.local/state/system/intake/runs/`.
- The real default home is used once, by the parent only, in the merged `feat/m10-integrate` verification pass.
- Parent updates queue state before and after every lane transition:
  - `READY`
  - `IN_PROGRESS`
  - `BLOCKED`
  - `MERGED_INTEGRATE`
  - `VERIFIED`
  - `LANDED`
- Completion markers are emitted only after parent confirmation:
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/sentinels/M10-W1.done`
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/sentinels/M10-W2.done`
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/sentinels/M10-W3.done`
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/sentinels/M10-W4.done`
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/sentinels/M10-parent-final.done`

## Tests And Acceptance
Required verification commands on merged `feat/m10-integrate`:
```bash
cargo fmt --all -- --check
cargo test --workspace
bash tools/codex/generate.sh
cargo install --locked --force --path crates/cli
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh
```

Parent-only final merged verification pass:
```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/integrate
git merge --no-ff feat/m10-source-gen
git merge --no-ff feat/m10-install-home
git merge --no-ff feat/m10-docs-cutover
git merge --no-ff feat/m10-smokes

cargo fmt --all -- --check
cargo test --workspace
bash tools/codex/generate.sh
cargo install --locked --force --path crates/cli
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh

readlink ~/.codex/skills/system
readlink ~/.codex/skills/system-charter-intake
find ~/system -maxdepth 4 -type f | sort
system doctor --json
```

Acceptance conditions:
- repo `.agents/skills/*` is thin and generated
- `~/system/` exists as the curated installed home
- installed runtime payload is under `~/system/`, not under `.agents/skills/**` and not under `~/.codex/skills/**`
- `~/system/.agents/skills/system*` exists and stays thin
- `~/.codex/skills/system*` points into `~/system/.agents/skills/*`
- `install.sh` does not compile Rust and does enforce the PATH/version gate
- required docs match the shipped topology and no longer describe `~/.codex/skills/system/` as the installed home
- install and live smokes prove `~/system/` as the runtime root and keep evidence under `~/.local/state/system/intake/runs/`
- `system doctor --json` remains the only machine-parsed output
- final verification evidence exists under `/Users/spensermcconnell/__Active_Code/system/.implemented/m10-orchestration/`:
  - `task-queue.md` with all tasks in terminal success state
  - `session-log.md` with the parent merge and verification summary
  - `.implemented/m10-orchestration/sentinels/M10-parent-final.done`
  - `final-verification.md` listing required commands, exit codes, and the final topology checks

End-of-session cleanup and closure:
1. Parent lands `feat/m10-integrate` back onto `feat/m10`.
2. Parent appends the landing result to `session-log.md` and marks all queue items `LANDED`.
3. Parent removes worktrees:
```bash
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/source-gen
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/install-home
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/docs-cutover
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/smokes
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10/integrate
```
4. Branch retention policy:
  - keep `feat/m10-integrate` until `feat/m10` is confirmed green
  - delete worker branches after landing unless a blocked follow-up explicitly needs one preserved
5. Session is complete only when the queue, session log, sentinels, and final verification record all exist and the landing branch is `feat/m10`.

## Assumptions
- `feat/m10` remains the authoritative landing branch for the milestone, but `feat/m10-integrate` is the parent-only branch for all merge resolution and final verification.
- The helper `bin/system-charter-intake` may remain if it shells through `~/system/bin/system`. The parent is the only actor allowed to decide otherwise.
- Worker-local isolated-home verification is sufficient for lane validation before the parent’s final real-home verification pass.
- Existing generated `.agents/skills/*` outputs and orchestration run artifacts are disposable and may be regenerated or recreated during the session.
