# M10.5 Task Queue

Authority: `/Users/spensermcconnell/__Active_Code/system/PLAN.md`
State authority: this file only

| Item | Owner | Status | Notes |
| --- | --- | --- | --- |
| `P0` Preflight on `feat/m10` | Parent | `LANDED` | Repo state captured and orchestration artifacts seeded |
| `P1` Create parent and worker worktrees, launch `W1` only | Parent | `LANDED` | Worktrees created and `W1` launched in isolation |
| `W1` Source migration + generator freeze | Worker | `LANDED` | Merged into parent and landed to `feat/m10` |
| `G1` Parent verification after `W1` | Parent | `LANDED` | Source subtree and thin projection contract frozen on parent |
| `P2` Launch `W2` and `W3` from updated parent | Parent | `LANDED` | `W2` and `W3` launched from the `G1`-frozen parent state |
| `W2` Install / runtime cutover | Worker | `LANDED` | Merged into parent and landed to `feat/m10` |
| `W3` Docs / contracts cutover | Worker | `LANDED` | Merged into parent and landed to `feat/m10` |
| `G2` Parent verification after `W2` | Parent | `LANDED` | Installed-home topology, helper-binary deletion, and direct invocation verified |
| `P3` Launch `W4` from updated parent | Parent | `LANDED` | `W4` launched from the `G2`-verified parent state |
| `W4` Smokes / regression rails | Worker | `LANDED` | Merged into parent and landed to `feat/m10` |
| `G3` Parent integration | Parent | `LANDED` | `W3` and `W4` merged, blocker fixed, full verification completed |
| `P4` Land parent branch back to `feat/m10` | Parent | `LANDED` | Parent branch fast-forwarded into `feat/m10` |
