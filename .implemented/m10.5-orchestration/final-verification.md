# M10.5 Final Verification

## Final Status

- Parent branch: `codex/m10.5-parent`
- Landing branch: `feat/m10`
- Landing result: `git merge --ff-only codex/m10.5-parent` -> `0`

## Parent Verification

- `G1`
  - merged `codex/m10.5-source-gen` into `codex/m10.5-parent`
  - `bash tools/codex/generate.sh` -> `0`
  - verified `install/system-home/` exists
  - verified repo-root authored install-home files are absent
  - verified repo `.agents/skills/system*` remains thin generated projection output
- `G2`
  - merged `codex/m10.5-install-runtime` into `codex/m10.5-parent`
  - isolated-home install verification passed
  - verified `~/system/bin/system` is the only installed executable
  - verified `~/system/bin/system-charter-intake` is absent
  - verified `~/system/share/**` is absent
  - verified `~/system/resources/**` exists
  - verified `~/.codex/skills/system*` points into installed thin projections
- `G3`
  - merged `codex/m10.5-docs-contracts` and `codex/m10.5-smokes` into `codex/m10.5-parent`
  - `cargo fmt --all -- --check` -> `0`
  - `cargo test --workspace` -> initial failure, then `0` after parent restored the exact legacy `M1` boolean-only wording in `PLAN.md`
  - `bash tools/codex/generate.sh` -> `0`
  - `cargo install --locked --force --path crates/cli` -> `0`
  - `bash tools/ci/install-smoke.sh` -> `0`
  - `bash tools/ci/codex-skill-live-smoke.sh` -> `0`
  - real-home `system doctor --json` emitted the locked JSON schema; readiness exit remained `1` as expected on a repo without completed local `.system/` truth
  - verified real-home discovery links, installed executable topology, and `resources/**` layout

## Acceptance Outcome

- `install/system-home/` is the only authored install-home source subtree
- repo root no longer owns active authored install-home files
- repo `.agents/skills/system/**` and `.agents/skills/system-charter-intake/**` remain thin generated projection output only
- `~/system/` is the curated installed product home
- `~/system/bin/system` is the only installed executable for this Codex surface
- `tools/codex/runtime/bin/system-charter-intake.tmpl` is removed
- `~/system/bin/system-charter-intake` does not exist
- `~/system/runtime-manifest.json` exists
- `~/system/resources/**` exists and is the installed runtime guidance root
- `~/system/share/**` does not exist
- `system-charter-intake` invokes `~/system/bin/system` directly
- `~/.codex/skills/*` points to installed thin projections after normal install
- `system doctor --json` remains the only machine-parsed output
- mutable run evidence is constrained to `~/.local/state/system/intake/runs/`
- `tools/codex/relink.sh` remains only as a production relink shim to `install.sh`; it no longer recreates dev-mode ambiguity
