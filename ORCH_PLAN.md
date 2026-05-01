# M10.5 Orchestration Plan: Installed Home Contract Cutover

## Summary
- This session exists to complete `PLAN.md` for `M10.5` without improvising new milestone semantics.
- The milestone contract is fixed:
  - authored install-home skill truth moves under `install/system-home/`
  - repo `.agents/skills/*` stays thin generated projection output only
  - `~/system/` is the installed home
  - `~/.codex/skills/*` is discovery glue only
  - `~/system/bin/system` is the only installed executable for this Codex surface
  - `tools/codex/runtime/bin/system-charter-intake.tmpl` must be removed
  - `~/system/bin/system-charter-intake` must not exist
  - `~/system/share/**` must not exist
  - `~/system/resources/**` is the installed runtime guidance root
  - `system-charter-intake` must invoke `~/system/bin/system` directly
  - `system doctor --json` remains the only machine-parsed output
  - mutable run evidence belongs only under `~/.local/state/system/intake/runs/`
- Parent is the only integrator, the only scope interpreter, and the only actor allowed to resolve cross-lane contract conflicts.
- Recommended orchestration root under this repo:
  - worktrees: `/Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/`
  - run artifacts: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/`
- Parent integration branch and worktree:
  - branch: `codex/m10.5-parent`
  - worktree: `/Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent`
- Worker branches and worktrees:
  - `codex/m10.5-source-gen` at `/Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/source-gen`
  - `codex/m10.5-install-runtime` at `/Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/install-runtime`
  - `codex/m10.5-docs-contracts` at `/Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/docs-contracts`
  - `codex/m10.5-smokes` at `/Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/smokes`
- Parent-owned orchestration artifacts:
  - queue: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/task-queue.md`
  - session log: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/session-log.md`
  - sentinels: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/`
  - final verification record: `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/final-verification.md`
- Queue discipline is fixed:
  - `task-queue.md` is the only orchestration state authority
  - `session-log.md` is append-only narrative and handoff evidence
  - sentinels are convenience markers only and never override queue state
- Execution order is strict where the contract freezes and parallel where it is safe:
  - parent preflight
  - `W1` source migration plus generator rewrite, serialized
  - parent gate `G1`
  - `W2` install/runtime cutover and `W3` docs/contracts, parallel
  - parent gate `G2`
  - `W4` smokes/regression
  - parent-only merge, real-home verification, and landing back to `feat/m10`

## Hard Guards
- `PLAN.md` is authoritative for milestone scope. Workers execute it; they do not reinterpret it.
- `generate.sh` may read authored inputs and write repo `.agents/skills/*` only. It must not mutate `$HOME`, `~/system/`, `~/.codex/skills/`, or repo-root install-home files.
- `install.sh` owns install-time staging into `~/system/` and refresh of `~/.codex/skills/*`. It must not compile Rust and must not install a repo clone.
- `~/system/bin/system` is the only installed executable for this Codex surface. No worker may preserve or reintroduce `~/system/bin/system-charter-intake`.
- `tools/codex/runtime/bin/system-charter-intake.tmpl` is a required deletion target. If it survives, the milestone is incomplete.
- `~/system/share/**` is forbidden. Installed runtime guidance must resolve from `~/system/resources/**`.
- Repo `.agents/skills/*` is generated projection output only. Workers must not hand-author files there.
- `~/.codex/skills/*` is discovery glue only. It must never become a runtime payload location.
- `system-charter-intake` remains a skill/discovery surface, not a helper binary. Its runtime path must invoke `~/system/bin/system` directly.
- `system doctor --json` is the only machine-parsed output. Any schema or semantics drift is an immediate stop.
- Mutable evidence must land only under `~/.local/state/system/intake/runs/`. Writes under `~/system/` or `~/.codex/skills/` are a hard failure.
- `tools/codex/relink.sh` is ambiguous in the current repo. This plan resolves that by deleting it unless the parent explicitly chooses a tightly controlled production-relink replacement that does not recreate dev-mode ambiguity.
- Any worker lane that runs install, dev-setup, smoke, or live-skill commands must use isolated `HOME` and `XDG_STATE_HOME`. Only the parent may use the real default home, and only during final merged verification.
- Workers stop immediately and hand control back to the parent if they would:
  - edit outside their write ownership
  - preserve the helper-binary contract
  - keep `share/**` alive
  - change `system doctor --json`
  - reinterpret milestone scope

## Workstream Plan
### Parent-Only Critical Path
1. `P0` Preflight on `feat/m10`.
   - Capture `git status --short`.
   - Create orchestration directories.
   - Seed `task-queue.md` and `session-log.md`.
   - Record current repo reality, including that root-authored install-home files still exist and `install/system-home/` may not.
2. `P1` Create worktrees and branches, but launch only `W1`.
   - `W1` is the freeze point for source-of-truth paths and thin projection shape.
   - No downstream lane starts before `G1` passes.
3. `G1` Parent verification after `W1`.
   - Merge `codex/m10.5-source-gen` into `codex/m10.5-parent`.
   - Run generator verification.
   - Confirm repo-root authored install-home files are removed from active contract use.
   - Freeze the canonical source/generator contract before opening downstream work.
4. `P2` Launch `W2` and `W3` in parallel from the updated parent branch.
   - `W2` owns install/runtime behavior.
   - `W3` owns docs/contracts wording.
   - `W4` stays closed because smoke assertions must target the post-`W2` runtime contract, not an in-between state.
5. `G2` Parent verification after `W2`.
   - Merge `codex/m10.5-install-runtime` into `codex/m10.5-parent`.
   - Verify installed-home topology, helper-binary deletion, `resources/**`, and direct `~/system/bin/system` invocation.
   - Only after `G2` passes may `W4` start.
6. `P3` Launch `W4` from the updated parent branch while `W3` finishes or rebases if needed.
7. `G3` Parent integration.
   - Merge `W3` and `W4`.
   - Resolve all conflicts locally in `codex/m10.5-parent`.
   - Run the full verification set, including the real-home pass.
8. `P4` Landing.
   - Fast-forward or merge `codex/m10.5-parent` back to `feat/m10`.
   - Record final results in `final-verification.md`.
   - Mark queue items `LANDED`.

### Setup Commands
```bash
mkdir -p /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5
mkdir -p /Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels

git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent -b codex/m10.5-parent feat/m10
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/source-gen -b codex/m10.5-source-gen feat/m10
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/install-runtime -b codex/m10.5-install-runtime feat/m10
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/docs-contracts -b codex/m10.5-docs-contracts feat/m10
git -C /Users/spensermcconnell/__Active_Code/system worktree add /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/smokes -b codex/m10.5-smokes feat/m10
```

### Worker Lanes
| Lane | Branch / worktree | Locked write ownership | Start gate | Required commands | Exit conditions |
| --- | --- | --- | --- | --- | --- |
| `W1` Source Migration + Generator Freeze | `codex/m10.5-source-gen` / `.worktrees/m10.5/source-gen` | `install/system-home/**` creation; removal of repo-root `SKILL.md.tmpl`, `SKILL.md`, `agents/openai.yaml`, `charter-intake/SKILL.md.tmpl`, `charter-intake/SKILL.md`; `tools/codex/generate.sh`; generated repo `.agents/skills/system/**` and `.agents/skills/system-charter-intake/**` only via `generate.sh` | `P1` | `bash tools/codex/generate.sh`; `find install/system-home -maxdepth 3 -print | sort`; `find .agents/skills/system -maxdepth 3 -print | sort`; `find .agents/skills/system-charter-intake -maxdepth 3 -print | sort` | authored install-home truth exists only under `install/system-home/`; repo-root authored install-home files are removed from the active contract; repo `.agents/skills/*` is thin and generated; generator never writes repo-root install-home files |
| `W2` Install / Runtime Cutover | `codex/m10.5-install-runtime` / `.worktrees/m10.5/install-runtime` | `tools/codex/install.sh`; `tools/codex/dev-setup.sh`; `tools/codex/relink.sh`; `tools/codex/runtime/runtime-manifest.json.tmpl`; deletion of `tools/codex/runtime/bin/system-charter-intake.tmpl`; runtime invocation edits in `install/system-home/charter-intake/SKILL.md.tmpl` after `G1` transfers that file to this lane | `G1` | `export HOME="$PWD/.tmp/home"`; `export XDG_STATE_HOME="$PWD/.tmp/state"`; `export CARGO_HOME="$HOME/.cargo"`; `export PATH="$CARGO_HOME/bin:$PATH"`; `mkdir -p "$HOME" "$XDG_STATE_HOME" "$CARGO_HOME"`; `cargo install --locked --force --path crates/cli`; `bash tools/codex/install.sh`; `system doctor --json`; `test ! -e "$HOME/system/bin/system-charter-intake"`; `test ! -e "$HOME/system/share"` | normal install stages a curated `~/system/` home; `~/system/bin/system` is the only installed executable; `tools/codex/runtime/bin/system-charter-intake.tmpl` is gone; `~/system/bin/system-charter-intake` does not exist; `~/system/resources/**` exists; `~/system/share/**` does not exist; `~/.codex/skills/*` restores to installed thin projections; `system-charter-intake` invokes `~/system/bin/system` directly; `relink.sh` is deleted or parent-approved as a tightly controlled production relink only |
| `W3` Docs / Contracts Cutover | `codex/m10.5-docs-contracts` / `.worktrees/m10.5/docs-contracts` | `README.md`; `DESIGN.md`; `docs/START_HERE.md`; `docs/SUPPORTED_COMMANDS.md`; `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`; `docs/contracts/C-07-conformance-rails-and-docs-cutover.md` | `G1` | `rg -n "system-charter-intake|share/|resources/|install/system-home|~/.codex/skills|~/system/bin/system|doctor --json" README.md DESIGN.md docs/START_HERE.md docs/SUPPORTED_COMMANDS.md docs/contracts/C-02-rust-workspace-and-cli-command-surface.md docs/contracts/C-07-conformance-rails-and-docs-cutover.md` | every packaging-facing doc matches the M10.5 contract exactly; no doc preserves the helper binary, `share/**`, repo-root authored install-home truth, or `~/.codex/skills/*` as installed home |
| `W4` Smokes / Regression Rails | `codex/m10.5-smokes` / `.worktrees/m10.5/smokes` | `tools/ci/install-smoke.sh`; `tools/ci/codex-skill-live-smoke.sh` | `G2` | `export HOME="$PWD/.tmp/home"`; `export XDG_STATE_HOME="$PWD/.tmp/state"`; `export CARGO_HOME="$HOME/.cargo"`; `export PATH="$CARGO_HOME/bin:$PATH"`; `mkdir -p "$HOME" "$XDG_STATE_HOME" "$CARGO_HOME"`; `cargo install --locked --force --path crates/cli`; `bash tools/ci/install-smoke.sh`; `bash tools/ci/codex-skill-live-smoke.sh` | smoke rails fail on any drift from thin repo projections, curated installed home, no helper binary, no `share/**`, direct `~/system/bin/system` invocation, and evidence restricted to `~/.local/state/system/intake/runs/` |

### Lane Subtasks
- `W1` Source Migration + Generator Freeze
  - `W1.1` Create `install/system-home/` as the only authored install-home source subtree.
  - `W1.2` Move authored skill inputs out of repo root and remove repo-root install-home ownership from the active contract.
  - `W1.3` Rewrite `tools/codex/generate.sh` to read only from `install/system-home/`.
  - `W1.4` Regenerate repo `.agents/skills/system*` as thin projections only.
  - `W1.5` Add or tighten exact thin-projection assertions so repo-root generated install-home files cannot reappear.
- `W2` Install / Runtime Cutover
  - `W2.1` Rewrite `tools/codex/install.sh` staging around the curated `~/system/` file set.
  - `W2.2` Preserve PATH/version gate behavior and ensure install refreshes thin discovery into `~/.codex/skills/*`.
  - `W2.3` Remove the helper-binary contract by deleting `tools/codex/runtime/bin/system-charter-intake.tmpl` and preventing `~/system/bin/system-charter-intake`.
  - `W2.4` Move installed runtime guidance resolution to `~/system/resources/**` and prove `~/system/share/**` absence.
  - `W2.5` Resolve `tools/codex/relink.sh` ambiguity by deletion, or by a parent-approved production-only relink behavior that does not recreate dev override semantics.
  - `W2.6` Update the leaf skill template so `system-charter-intake` invokes `~/system/bin/system` directly and preserves evidence-path rules.
- `W3` Docs / Contracts Cutover
  - `W3.1` Update installed-home wording so `~/system/` is the only installed home.
  - `W3.2` Update source-of-truth wording so authored install-home truth exists only under `install/system-home/`.
  - `W3.3` Update discovery wording so repo `.agents/skills/*` stays thin and `~/.codex/skills/*` stays discovery glue only.
  - `W3.4` Remove all helper-binary and `share/**` language from packaging-facing docs.
  - `W3.5` Reconfirm that `system doctor --json` is documented as the only machine-parsed surface.
- `W4` Smokes / Regression Rails
  - `W4.1` Rewrite install smoke assertions for the exact thin repo projection shape.
  - `W4.2` Rewrite install smoke assertions for curated `~/system/`, `resources/**`, and absence of helper binary and `share/**`.
  - `W4.3` Add assertions that normal install after dev override restores installed discovery topology.
  - `W4.4` Rewrite live smoke around direct `~/system/bin/system` invocation.
  - `W4.5` Add evidence-path assertions so mutable run evidence is accepted only under `~/.local/state/system/intake/runs/`.

### Parent Merge Order
1. Merge `W1` into `codex/m10.5-parent`.
2. Open `W2` and `W3`.
3. Merge `W2` into `codex/m10.5-parent`.
4. Open `W4`.
5. Merge `W3` and `W4`.
6. Run full verification in `codex/m10.5-parent`.
7. Land into `feat/m10`.

### Worker Return Contract
- Every worker returns only:
  - changed files
  - commands run with exit codes
  - blockers and unresolved assumptions
- Parent records that return in `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/session-log.md` and updates `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/task-queue.md`.

### Parent Review Discipline
- Parent reviews the worker return contract plus a narrow diff only.
- Parent does not treat full worker transcripts as the review artifact.
- Recommended parent diff review commands:
```bash
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent diff --stat codex/m10.5-parent...codex/m10.5-source-gen
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent diff --stat codex/m10.5-parent...codex/m10.5-install-runtime
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent diff --stat codex/m10.5-parent...codex/m10.5-docs-contracts
git -C /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent diff --stat codex/m10.5-parent...codex/m10.5-smokes
```

### Serialization Boundaries
- `W1` must run alone because it freezes the canonical authored source paths, removes repo-root ownership from the install-home contract, and locks the thin-projection output shape that install logic, docs wording, and smoke assertions all consume.
- `W4` must wait until after `G2` because smoke rails are contract enforcement. They must validate the final install/runtime topology after helper-binary removal, `resources/**` cutover, and direct `~/system/bin/system` invocation are already proven, not an intermediate state.
- Final integration stays parent-only because install/runtime behavior, docs wording, and smoke expectations converge on the same contract. One integrator prevents cross-lane reinterpretation and keeps conflict resolution centralized.

## Context-Control Rules
- Parent is the only owner of:
  - `task-queue.md`
  - `session-log.md`
  - sentinel creation
  - merge decisions
  - scope decisions
  - cross-lane conflict resolution
- Queue states are parent-written only:
  - `READY`
  - `IN_PROGRESS`
  - `BLOCKED`
  - `AWAIT_PARENT_GATE`
  - `MERGED_PARENT`
  - `VERIFIED`
  - `LANDED`
- Sentinel naming is fixed:
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/W1.done`
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/W2.done`
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/W3.done`
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/W4.done`
  - `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/parent-final.done`
- Worker briefs must contain only:
  - current gate
  - owned files
  - required commands
  - exit conditions
  - known blockers
- Workers do not treat prior worker transcripts as authority. The parent queue and current gate are the only execution authority.
- Workers do not hand-edit repo `.agents/skills/*`. They edit source or scripts, then regenerate.
- `W2` is the only lane allowed to touch install/runtime behavior and the moved `install/system-home/charter-intake/SKILL.md.tmpl` after `G1`.
- `W4` may not start early to “get ahead.” Smoke assertions are contract enforcement and must target the post-`W2` runtime topology.
- Blocked-lane procedure:
  - worker stops
  - worker reports changed files, commands run, exit codes, and blocker
  - parent records blocker in `task-queue.md` and `session-log.md`
  - parent writes `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/<lane>.blocked`
  - parent either resolves locally, relaunches the same lane, or creates a replacement lane from the current parent branch
- If a worker blocks after partial progress:
  - parent marks the lane `BLOCKED` in `task-queue.md`
  - parent appends the handoff and current partial state to `session-log.md`
  - parent writes `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/<lane>.blocked`
  - parent chooses exactly one next action:
    - resolve locally in `codex/m10.5-parent`
    - relaunch the same lane with narrowed scope
    - open a replacement lane from the current parent branch
  - partial progress is never silently carried across lanes without a parent decision and queue update

## Tests And Acceptance
### Parent Verification Gates
- `G1` after `W1`:
```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent
git merge --no-ff codex/m10.5-source-gen
bash tools/codex/generate.sh
test -d install/system-home
test ! -e SKILL.md.tmpl
test ! -e SKILL.md
test ! -e agents/openai.yaml
test ! -e charter-intake/SKILL.md.tmpl
test ! -e charter-intake/SKILL.md
find .agents/skills/system -maxdepth 3 -print | sort
find .agents/skills/system-charter-intake -maxdepth 3 -print | sort
```
- `G2` after `W2`:
```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent
git merge --no-ff codex/m10.5-install-runtime
export HOME="$PWD/.tmp/parent-home-g2"
export XDG_STATE_HOME="$PWD/.tmp/parent-state-g2"
export CARGO_HOME="$HOME/.cargo"
export PATH="$CARGO_HOME/bin:$PATH"
mkdir -p "$HOME" "$XDG_STATE_HOME" "$CARGO_HOME"
cargo install --locked --force --path crates/cli
bash tools/codex/install.sh
system doctor --json
test -x "$HOME/system/bin/system"
test ! -e "$HOME/system/bin/system-charter-intake"
test ! -e "$HOME/system/share"
test -d "$HOME/system/resources"
```

### Final Parent Verification
```bash
cd /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent
git merge --no-ff codex/m10.5-docs-contracts
git merge --no-ff codex/m10.5-smokes

cargo fmt --all -- --check
cargo test --workspace
bash tools/codex/generate.sh
cargo install --locked --force --path crates/cli
bash tools/ci/install-smoke.sh
bash tools/ci/codex-skill-live-smoke.sh
system doctor --json
readlink ~/.codex/skills/system
readlink ~/.codex/skills/system-charter-intake
test -x ~/system/bin/system
test ! -e ~/system/bin/system-charter-intake
test ! -e ~/system/share
test -d ~/system/resources
find ~/system -maxdepth 4 -print | sort
```

### Milestone Acceptance Conditions
- `install/system-home/` is the only authored install-home source subtree.
- Repo root no longer owns active authored install-home files.
- Repo `.agents/skills/system/**` and `.agents/skills/system-charter-intake/**` remain thin generated projection output only.
- `~/system/` is the installed product home and is curated by install, not by cloning the repo.
- `~/system/bin/system` is the only installed executable for this Codex surface.
- `tools/codex/runtime/bin/system-charter-intake.tmpl` is removed.
- `~/system/bin/system-charter-intake` does not exist.
- `~/system/runtime-manifest.json` exists.
- `~/system/resources/**` exists and is the installed runtime guidance root.
- `~/system/share/**` does not exist.
- `system-charter-intake` invokes `~/system/bin/system` directly.
- `~/.codex/skills/*` points to installed thin projections after normal install.
- `system doctor --json` remains the only machine-parsed output.
- Mutable run evidence appears only under `~/.local/state/system/intake/runs/`.
- `tools/codex/relink.sh` is deleted, or the parent explicitly records a tighter production-only replacement decision that does not recreate dev-mode ambiguity.
- `task-queue.md`, `session-log.md`, sentinels, and `final-verification.md` all exist and reflect the completed session.

## Closure
1. Land `codex/m10.5-parent` back to `feat/m10`.
2. Append the landing result, verification result, and final branch state to `session-log.md`.
3. Mark all queue items `LANDED` in `task-queue.md`.
4. Write `/Users/spensermcconnell/__Active_Code/system/.implemented/m10.5-orchestration/sentinels/parent-final.done`.
5. Remove orchestration worktrees:
```bash
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/source-gen
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/install-runtime
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/docs-contracts
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/smokes
git -C /Users/spensermcconnell/__Active_Code/system worktree remove /Users/spensermcconnell/__Active_Code/system/.worktrees/m10.5/parent
```
6. Branch retention policy:
  - keep `codex/m10.5-parent` until `feat/m10` is confirmed green after landing
  - delete worker branches after landing unless a blocked follow-up requires one to remain as evidence
7. Session completion requires:
  - `feat/m10` contains the landed parent branch result
  - queue state is terminal and consistent
  - session log contains the landing record
  - final verification record exists
  - worktrees are removed unless an explicit blocked follow-up says otherwise

## Assumptions
- `feat/m10` remains the milestone landing branch.
- The parent may create temporary integration branches and worktrees under this repo without changing the milestone branch target.
- `cargo install --locked --force --path crates/cli` remains the supported way to place a version-matching `system` binary on `PATH` for verification.
- Isolated-home verification is sufficient for worker lanes; the real default home is reserved for the parent’s final merged pass.
- If `relink.sh` is retained for any reason, that requires an explicit parent decision recorded in `session-log.md`; default action is deletion.
- Orchestration artifacts under `.implemented/m10.5-orchestration/` are execution evidence only and are not part of the shipped product contract.
