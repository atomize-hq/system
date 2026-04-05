---
contract_id: C-02
seam_id: SEAM-2
owner_seam: SEAM-2
version: reduced-v1
currentness: current
status: draft
revalidation_triggers:
  - Any change to the supported verb set or help hierarchy in README.md, PLAN.md, docs/README.md, or this contract.
  - Any change to the workspace member list, crate ownership boundaries, or the location of CLI entrypoints.
  - Any change to the reduced-v1 local install target matrix or the supported-vs-legacy runtime boundary.
---

# C-02 Rust Workspace and CLI Command-Surface Contract

## Purpose

This contract defines the reduced-v1 Rust workspace and CLI command-surface truth for `SEAM-2`. It is the source of truth for crate ownership, supported verbs, help posture, and the runtime boundary between supported Rust code and legacy reference material.

## Canonical Location

- Canonical artifact: `docs/contracts/C-02-rust-workspace-and-cli-command-surface.md`
- Repo-facing truth surfaces that should point here: `README.md`, `PLAN.md`, and `docs/README.md`
- Downstream seams that consume this contract: `SEAM-4`, `SEAM-5`, `SEAM-6`, and `SEAM-7`

## Owned Surface

- Workspace root:
  - `Cargo.toml` at the repo root is the authoritative Rust workspace entrypoint.
  - The workspace members are `crates/cli` and `crates/compiler`.
- Crate ownership:
  - `crates/cli` owns binary entrypoints, argument parsing, command dispatch, and user-facing help text.
  - `crates/compiler` owns shared types, packet/decision-log scaffolding, and core compilation or resolution logic.
  - `crates/cli` MUST NOT become the home for resolver logic, packet selection, or shared domain types beyond thin wiring.
  - `crates/compiler` MUST NOT parse CLI arguments or own the supported help surface.

## Normative Rules

### Workspace and crate boundaries

- The workspace MUST expose one obvious split between the operator-facing CLI and the compiler core.
- The CLI crate MUST remain a thin orchestration layer that delegates shared logic into the compiler crate.
- The compiler crate MUST be the compile-time home for shared packet-result and decision-log types used by downstream seams.
- The Rust CLI MUST be the only supported packet-resolution authority once Rust setup exists.

### CLI verbs and help posture

- The supported reduced-v1 verb surface MUST include only `setup`, `generate`, `inspect`, and `doctor`.
- Help text MUST present the verbs in setup-first order: `setup`, then `generate`, then `inspect`, then `doctor`.
- Help text MUST make clear that setup establishes or refreshes canonical truth, packet generation comes after setup, inspection is a proof surface, and doctor is the recovery surface.
- Help text and placeholder output MUST avoid implying that resolver, renderer, or execution-demo behavior has already landed when it has not.
- `generate` MUST fail with an explicit unimplemented or unsupported placeholder until downstream seams provide real packet behavior.
- `inspect` and `doctor` MUST remain honest placeholders until their owning seams land, but their names and rough operator intent MUST stay stable.

### Runtime boundary

- The legacy Python harness remains legacy reference material only.
- The Rust CLI MUST NOT shell out to, wrap, or depend on the legacy harness as a supported runtime path.
- Docs and help text MUST preserve the distinction between supported Rust code and legacy reference material.

### Reduced-v1 install targets

- Reduced-v1 local install support MUST explicitly cover `macOS arm64` and `Linux x86_64`.
- The contract MUST NOT imply broader package-manager publishing or release automation beyond those local install targets.
- Any expansion of the target matrix or supported distribution story requires a contract revision.

### Compatibility and revalidation

- Changes to verb names, help ordering, crate ownership, or runtime boundary wording MUST trigger revalidation of downstream seams that consume `C-02`.
- Changes to the reduced-v1 install target matrix MUST also trigger revalidation because they affect the supported command story.
- This contract MAY be tightened by downstream implementation details, but it MUST NOT be broadened silently.

## Verification Checklist

- [ ] `Cargo.toml` defines a root workspace with `crates/cli` and `crates/compiler` as members.
- [ ] `crates/cli` owns parsing, dispatch, and help text.
- [ ] `crates/compiler` owns shared types and compiler-core logic.
- [ ] `--help` shows `setup`, `generate`, `inspect`, and `doctor` in setup-first order.
- [ ] Help and placeholder output do not overclaim resolver, renderer, or execution-demo behavior.
- [ ] The Rust CLI does not invoke the legacy Python harness as a supported runtime dependency.
- [ ] The reduced-v1 local install targets are explicitly limited to `macOS arm64` and `Linux x86_64`.
- [ ] `README.md`, `PLAN.md`, and `docs/README.md` each point to this contract.
- [ ] Downstream seams can revalidate against this contract without hidden context.
