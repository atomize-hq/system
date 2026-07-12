# Start Here

This repo is in transition.

The active work is moving toward a **Handbook contract membrane** / **executable-contract** architecture with configurable canonical artifacts and vocabulary, Context Resolution, deterministic projections, a public SDK facade, full machine transports, and external validator docks. The older reduced-v1 CLI/compiler docs are retained only as provenance.

## Read in this order

1. [`docs/specs/handbook-contract-membrane/00-README.md`](specs/handbook-contract-membrane/00-README.md)
2. the exact control-pack files named by your slice or orchestration handoff
3. [`docs/ideas/handbook-contract-membrane-architecture-memo.md`](ideas/handbook-contract-membrane-architecture-memo.md) for concise architecture lineage
4. [`docs/ideas/substrate_executable_contracts_architecture.md`](ideas/substrate_executable_contracts_architecture.md) for deeper original contract/dock design input
5. [`docs/ideas/handbook-substrate-packet-4-2-proof-findings.md`](ideas/handbook-substrate-packet-4-2-proof-findings.md) only when you need historical downstream proof context

## What is current

- The Handbook Contract Membrane control pack is the active program architecture, sequencing, orchestration, handoff, and proof surface.
- Handbook is being re-centered around canonical structured truth, profiles, vocabulary, Context Resolution, projections, contract truth, evidence, docks, verdicts, lifecycle, and gate semantics.
- Substrate is expected to consume that system, not become a second competing contract authority.
- The current program is greenfield: no user legacy-migration tooling or permanent Markdown/YAML dual authority.
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
- Active control pack: [`docs/specs/handbook-contract-membrane/`](specs/handbook-contract-membrane/)
- Rust code: `../crates/`
- Core declarative assets: `../core/`
