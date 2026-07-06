# Handbook

This repo is being cleaned up ahead of the **contract membrane / executable-contract** architecture work.

## Read this first

If you are trying to understand the current intended direction, start here:

1. [`docs/START_HERE.md`](docs/START_HERE.md)
2. [`docs/ideas/handbook-contract-membrane-architecture-memo.md`](docs/ideas/handbook-contract-membrane-architecture-memo.md)
3. [`docs/ideas/substrate_executable_contracts_architecture.md`](docs/ideas/substrate_executable_contracts_architecture.md)

## Current authority posture

- The membrane memo is the current handbook-side architecture starting point.
- The Substrate executable-contracts doc is the companion concept doc.
- Older reduced-v1 CLI/compiler docs have been moved out of the active front door.
- Historical planning/spec artifacts are retained for provenance, not as current execution authority.

## Where things live now

- Active docs index: [`docs/README.md`](docs/README.md)
- Current architecture memos: [`docs/ideas/`](docs/ideas/)
- Archived reduced-v1 docs: [`archived/reduced-v1/`](archived/reduced-v1/)
- Historical root authority docs: [`archived/root-authority/`](archived/root-authority/)
- Historical research dumps: [`archived/research/`](archived/research/)
- Historical specs/plans: [`archived/specs/`](archived/specs/)

## Repo landmarks

- Rust crates: `crates/`
- Declarative/core assets: `core/`
- Tests and fixtures: `tests/`
- Tools and packaging helpers: `tools/`, `install/`

## Important note about local `.handbook/`

You may have a local ignored `.handbook/` working area while developing. Do **not** treat that ignored local state as committed repo authority unless it is deliberately promoted later.
