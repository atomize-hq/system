# M5 Run Ledger

## Branch And Worktree Topology

| Lane | Branch | Planned worktree | Owner | Status |
| --- | --- | --- | --- | --- |
| WS-INT | `codex/m5-ws-int` | primary repo checkout | main thread | in_progress |
| WS-A | `codex/m5-ws-a` | `../system-m5-ws-a` | worker | completed |
| WS-B | `codex/m5-ws-b` | `../system-m5-ws-b` | worker | completed |
| WS-C | `codex/m5-ws-c` | `../system-m5-ws-c` | worker | completed |
| WS-D | `codex/m5-ws-d` | `../system-m5-ws-d` | worker | completed |
| WS-E | `codex/m5-ws-e` | `../system-m5-ws-e` | main thread | completed |

## Ownership

| Lane | Write scope | Must not touch |
| --- | --- | --- |
| WS-A | `docs/contracts/C-13-pipeline-handoff-and-downstream-trust.md`, narrow `docs/contracts/C-03-canonical-artifact-manifest-contract.md`, `PLAN.md` M5 freeze only | production code, help snapshots, operator docs |
| WS-B | `crates/cli/src/main.rs`, `crates/compiler/src/pipeline_handoff.rs`, `crates/compiler/src/lib.rs`, CLI help snapshots caused by the new command | docs prose, `crates/cli/tests/cli_surface.rs`, fixture corpora |
| WS-C | `tests/fixtures/foundation_flow_demo/**`, `crates/cli/tests/cli_surface.rs`, `crates/cli/tests/feature_spec_contract.rs` if needed, happy-path-only harness support | proof corpus helpers, refusal files, `main.rs`, docs |
| WS-D | new refusal test files, `crates/compiler/tests/support/pipeline_proof_corpus_support.rs`, `crates/cli/tests/pipeline_proof_corpus_support.rs`, `tests/fixtures/pipeline_proof_corpus/foundation_inputs/**` | `crates/cli/tests/cli_surface.rs`, happy-path demo fixtures, docs |
| WS-E | `docs/START_HERE.md`, `docs/SUPPORTED_COMMANDS.md`, `docs/CLI_PRODUCT_VOCABULARY.md`, `docs/CLI_OPERATOR_JOURNEY.md` | contracts, production code, help snapshots, proof corpus logic |

## Merge Order

1. WS-A
2. WS-B
3. WS-C and WS-D in parallel after WS-B freeze
4. WS-E
5. WS-INT final verification

## Verification Gates

- WS-A freezes the authority model:
  - `.system/*` stays canonical
  - `artifacts/*` stays derived
  - `artifacts/handoff/**` is explicit, versioned, and non-canonical
- WS-B must ship:
  - `system pipeline handoff emit --id pipeline.foundation_inputs --consumer feature-slice-decomposer`
  - one compiler-owned handoff manifest surface
  - refusal on stale route basis, missing provenance, tampered inputs, and trust mismatch
- WS-C must ship:
  - happy-path bundle emission from the M4-style fixture
  - bundle-only consumer proof
  - `artifacts/planning/feature_slice/<feature-id>/SLICE_PLAN.md`
  - before/after scorecard plus transcript evidence
- WS-D must ship:
  - stale canonical refusal
  - tampered derived refusal
  - missing or corrupt provenance refusal
  - trust-class mismatch refusal
  - undeclared repo reread refusal
  - canonical-boundary regression for `generate` and `inspect`
- WS-E must keep vocabulary exact:
  - M4 remains journey proof
  - M5 becomes downstream adoption proof
  - canonical vs derived trust stays explicit everywhere

## Worker Return Format

- Changed files
- What was completed
- Blockers or assumptions

Keep summaries short. Do not return long narratives.
