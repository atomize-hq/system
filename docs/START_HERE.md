# Start Here (Supported, Rust-first)

Reduced v1 is a **Rust-first context compiler CLI**.

The legacy Python harness still exists in this repo as **frozen reference material**. It is useful for understanding the old scaffold, but it is not the supported v1 product path.

## What is supported in reduced v1

- **Canonical inputs live in repo-local `.system/`**.
  - Required:
    - `.system/charter/CHARTER.md`
    - `.system/feature_spec/FEATURE_SPEC.md`
  - Optional:
    - `.system/project_context/PROJECT_CONTEXT.md`
- **Planning packet generation** is supported from canonical repo-local `.system/`.
- **Execution packet generation** is fixture-backed demo only via `execution.demo.packet`; live execution is explicitly refused.
- **`inspect`** is the proof surface.
- **`doctor`** is the recovery surface, it explains blockers and safe next actions.
- **`setup`** is still a placeholder entrypoint and is not yet a real Rust setup flow.

## How to navigate this repo

- Supported architecture + cutover plan: [`PLAN.md`](../PLAN.md)
- Operator-facing vocabulary: [`docs/CLI_PRODUCT_VOCABULARY.md`](CLI_PRODUCT_VOCABULARY.md)
- Front door and command hierarchy: [`docs/CLI_COMMAND_HIERARCHY.md`](CLI_COMMAND_HIERARCHY.md)
- Contracts (the authoritative truth): [`docs/contracts/`](contracts/)
- CLI command surface and wording: [`C-02`](contracts/C-02-rust-workspace-and-cli-command-surface.md)
- Canonical `.system/` manifest + freshness: [`C-03`](contracts/C-03-canonical-artifact-manifest-contract.md)
- Refusal + doctor blockers taxonomy: [`C-04`](contracts/C-04-resolver-result-and-doctor-blockers.md)
- Proof surfaces (markdown/json/inspect ordering): [`C-05`](contracts/C-05-renderer-and-proof-surfaces.md)
- Fixture-backed execution demo boundary: [`C-06`](contracts/C-06-fixture-execution-demo-boundary.md)
- Conformance + docs/help parity rails: [`C-07`](contracts/C-07-conformance-rails-and-docs-cutover.md)

Legacy reference docs (Python harness, stage reference, old workflow guides) live under [`docs/legacy/`](legacy/README.md).
