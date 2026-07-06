# Start Here

This repo is in transition.

The active work is moving toward a **handbook contract membrane** / **executable-contract** architecture. The older reduced-v1 CLI/compiler docs are still retained, but they are now archived and should not be treated as the default starting point.

## Read in this order

1. [`docs/ideas/handbook-contract-membrane-architecture-memo.md`](ideas/handbook-contract-membrane-architecture-memo.md)
2. [`docs/ideas/substrate_executable_contracts_architecture.md`](ideas/substrate_executable_contracts_architecture.md)
3. [`docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`](ideas/handbook-substrate-packet-4-2-proof-findings.md) if you want nearby historical proof context

## What is current

- Handbook is being re-centered around contract truth, evidence, docks, verdicts, lifecycle, and gate semantics.
- Substrate is expected to consume that system, not become a second competing contract authority.
- Archived reduced-v1 command/contract docs are historical input only.

## What is not current

Do not start from these unless you are intentionally doing historical archaeology:

- `archived/reduced-v1/`
- `archived/reduced-v1/front-door/`
- `archived/specs/`
- `archived/root-authority/`

## If you need the old story

The prior reduced-v1 CLI/compiler material is still preserved:

- reduced-v1 contracts and legacy docs under [`archived/reduced-v1/`](../archived/reduced-v1/)
- prior root docs under [`archived/root-authority/`](../archived/root-authority/)

## Minimal repo map

- Active docs index: [`docs/README.md`](README.md)
- Current architecture ideas: [`docs/ideas/`](ideas/)
- Rust code: `../crates/`
- Core declarative assets: `../core/`
