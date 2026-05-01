# M10 Task Queue

Authoritative orchestration state for M10. Status values are limited to `READY`, `IN_PROGRESS`, `BLOCKED`, `MERGED_INTEGRATE`, `VERIFIED`, and `LANDED`.

| Task | Branch | Owner | Status | Depends On | Notes |
| --- | --- | --- | --- | --- | --- |
| `M10-P0` Preflight and artifact seeding | `feat/m10` | parent | `LANDED` | - | Confirmed branch, clean status, and seeded orchestration files |
| `M10-P1` Create worktrees and start `M10-W1` | `feat/m10` | parent | `LANDED` | `M10-P0` | Worktrees created and `M10-W1` committed on its lane branch |
| `M10-W1.1` Canonical root/leaf skill truth | `feat/m10-source-gen` | worker | `LANDED` | `M10-P1` | Repo-root and `charter-intake/` authored sources established |
| `M10-W1.2` Generator rewrite | `feat/m10-source-gen` | worker | `LANDED` | `M10-W1.1` | `generate.sh` now emits repo sibling docs and thin projections only |
| `M10-W1.3` Thin-projection guardrails | `feat/m10-source-gen` | worker | `LANDED` | `M10-W1.2` | Repo `.agents/skills/system*` regenerate without runtime payload |
| `M10-G1` Review and merge `M10-W1`, rerun generator | `feat/m10-integrate` | parent | `LANDED` | `M10-W1.3` | Source/generator contract frozen on `feat/m10-integrate` |
| `M10-P2` Rebase downstream branches and launch parallel lanes | `feat/m10-integrate` | parent | `LANDED` | `M10-G1` | Downstream branches rebased; W2/W3/W4 branch results captured |
| `M10-W2.1` Install to `~/system/` | `feat/m10-install-home` | worker | `LANDED` | `M10-P2` | Isolated `HOME` validation created curated `~/system/` |
| `M10-W2.2` Thin Codex discovery | `feat/m10-install-home` | worker | `LANDED` | `M10-W2.1` | `~/.codex/skills/system*` restored as thin links |
| `M10-W2.3` PATH/version gate | `feat/m10-install-home` | worker | `LANDED` | `M10-W2.2` | Install refused stale `0.7.0.0` and accepted repo `0.7.1.0` |
| `M10-W2.4` Helper/runtime payload placement | `feat/m10-install-home` | worker | `LANDED` | `M10-W2.3` | Runtime payload and helper now install under `~/system/` |
| `M10-W2.5` Dev override preservation | `feat/m10-install-home` | worker | `LANDED` | `M10-W2.4` | `dev-setup.sh`/`relink.sh` left as discovery-only override path |
| `M10-W3.1` Installed-home wording | `feat/m10-docs-cutover` | worker | `LANDED` | `M10-P2` | Required docs updated to `~/system/` contract |
| `M10-W3.2` Install ownership wording | `feat/m10-docs-cutover` | worker | `LANDED` | `M10-W3.1` | Required docs only |
| `M10-W3.3` Machine-parsed output wording | `feat/m10-docs-cutover` | worker | `LANDED` | `M10-W3.2` | `system doctor --json` remained the only machine-parsed surface |
| `M10-W4.1` Install smoke topology assertions | `feat/m10-smokes` | worker | `LANDED` | `M10-P2` | Isolated `HOME` validation passed |
| `M10-W4.2` Live smoke runtime-root assertions | `feat/m10-smokes` | worker | `LANDED` | `M10-W4.1` | Live smokes recorded `runtime_root == ~/system/` |
| `M10-W4.3` Dev-setup crossover assertions | `feat/m10-smokes` | worker | `LANDED` | `M10-W4.2` | Dev override kept discovery-only semantics |
| `M10-W4.4` Run-evidence assertions | `feat/m10-smokes` | worker | `LANDED` | `M10-W4.3` | Evidence stayed under `~/.local/state/system/intake/runs/` |
| `M10-G2` Hold docs/smokes until install topology is proven | `feat/m10-integrate` | parent | `LANDED` | `M10-W2.5`, `M10-W3.3`, `M10-W4.4` | Install topology proven before docs/smokes closeout |
| `M10-P3` Parent-only integration and full verification | `feat/m10-integrate` | parent | `LANDED` | `M10-G2` | All worker branches merged locally; verification complete |
| `M10-G3` Land back to `feat/m10` and close session | `feat/m10` | parent | `LANDED` | `M10-P3` | Fast-forwarded `feat/m10`, emitted parent-final sentinel, and removed worktrees |
