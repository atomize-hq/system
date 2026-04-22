---
contract_id: C-07
seam_id: SEAM-7
owner_seam: SEAM-7
version: reduced-v1
currentness: current
status: published
revalidation_triggers:
  - Any change to `C-01..C-06` semantics, wording, or verification expectations.
  - Any change to supported install-smoke targets (`macOS arm64`, `Linux x86_64`).
  - Any change to CLI help wording that affects the supported-vs-legacy story.
  - Any change to proof-surface ordering, trust header wording, or refusal structure.
  - Any change to the doctor baseline-state set, checklist shape, or baseline artifact vocabulary.
---

# C-07 Conformance Rails and Docs Cutover Contract

## Purpose

This contract defines what “conformance” means for the reduced-v1 Rust-first cutover.

`C-07` exists to ensure the shipped repository continues to tell **one coherent supported story**:

- Rust is the only supported runtime path for reduced v1
- setup scaffolds the `M8` baseline set under canonical repo-local `.system/`
- planning packets resolve deterministically from canonical repo-local `.system/` artifacts
- proof surfaces and refusal semantics remain stable and test-pinned
- the fixture-backed execution demo cannot be mistaken for live slice execution
- direct Codex transport drift on the shipped charter-authoring path cannot land without automated detection

`C-07` is owned by `SEAM-7` and is intentionally **downstream-facing**: it binds tests, CI rails, install smoke, and docs/help parity to the contracts it consumes (`C-01..C-06`).

## Canonical location

- Canonical artifact: `docs/contracts/C-07-conformance-rails-and-docs-cutover.md`
- Producing seam: `SEAM-7`

## Consumed contracts (inputs)

`C-07` consumes:

- [`C-01`](C-01-approved-repo-surface.md) approved repo surface and legacy freeze (supported-vs-legacy messaging; archive/runtime boundary)
- [`C-02`](C-02-rust-workspace-and-cli-command-surface.md) Rust workspace and CLI command surface (verbs, help posture, crate boundaries)
- [`C-03`](C-03-canonical-artifact-manifest-contract.md) canonical artifact manifest contract (`.system/` inputs + freshness)
- [`C-04`](C-04-resolver-result-and-doctor-blockers.md) doctor baseline-readiness and blocker taxonomy (checklist structure; next-safe-action requirements)
- [`C-05`](C-05-renderer-and-proof-surfaces.md) renderer and proof surfaces (trust header ordering; inspect ordering; deterministic JSON/markdown rules)
- [`C-06`](C-06-fixture-execution-demo-boundary.md) fixture execution demo boundary (fixture-only scope; live refusal semantics)

## Owned surface

`C-07` is authoritative about:

- which **local checks** must exist and be runnable deterministically
- which **surfaces must be pinned by tests** (ordering + wording) to prevent drift
- what **install smoke** must prove for supported targets
- what **docs/help parity** must guarantee (no Python-support leakage; help aligns with runtime)
- what constitutes sufficient **closeout evidence** for this conformance seam

`C-07` is not authoritative about upstream contract semantics; it only binds verification rails to upstream truth.

## Normative rules

### Required local checks (developer + CI rails)

Reduced v1 MUST provide deterministic commands that:

- validate formatting (`cargo fmt --all -- --check`)
- validate compilation and tests (`cargo test --workspace`)
- validate CLI surface tests (at minimum: `cargo test -p system-cli`)
- validate compiler tests (at minimum: `cargo test -p system-compiler`)

CI MUST run the same logical checks (or stricter) on supported targets.

For the shipped `system author charter --from-inputs <path|->` surface, CI MUST also run a change-scoped live Codex smoke whenever the compiler-owned `codex exec` contract or charter authoring assets change. That smoke MUST use a fresh temp repo, invoke the real Codex CLI, and prove the `--output-last-message` write path before treating the changed authoring transport as landed.

The live author smoke MUST be model-configurable via repo-owned environment variables. `SYSTEM_AUTHOR_CHARTER_CODEX_MODEL` selects the runtime model for compiler-owned charter synthesis, and the smoke wrapper MAY drive that through `SYSTEM_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL`. CI MUST pin an explicit smoke model rather than inheriting whatever the Codex CLI default happens to be.

For direct `codex exec` use in CI/CD, the live smoke runtime MUST provide `CODEX_API_KEY` to the Codex CLI. The repository MAY continue storing the secret as `OPENAI_API_KEY` in GitHub Actions so long as the workflow maps it into `CODEX_API_KEY` for the smoke step.

### Supported install-smoke targets

Reduced v1 MUST treat these as supported install-smoke targets:

- `macOS arm64`
- `Linux x86_64`

Install smoke MUST, at minimum:

- build and install `system` from this repository (`cargo install --path crates/cli`)
- execute `system --help` (and any required verb-level help) without panicking

### Proof-surface drift must fail fast

Conformance rails MUST fail fast if:

- trust header ordering or wording drifts (`C-05`)
- refusal categories, ordering, or “exact next safe action” semantics drift (`C-04`)
- inspect proof ordering drifts (`C-05`)
- demo-boundary wording drifts (fixture-backed labeling or live refusal semantics; `C-06`)

This is typically enforced by golden-output tests and/or snapshot tests at the compiler or CLI surface.

### Docs and help must reflect the supported story

Docs and CLI help MUST NOT:

- imply Python is a supported runtime path for reduced v1
- imply live slice execution is supported
- treat `archived/` as a runtime input (it is reference-only per `C-01`)

Docs and help SHOULD link directly to the authoritative contracts they reference.

### Drift-guard matrix

The conformance rails owned by `C-07` are expected to pin drift against the upstream contract that is semantically authoritative for that surface:

| Drift risk (what can drift) | Semantic source contract (what defines the truth) |
| --- | --- |
| Supported-vs-legacy wording (no implied Python support; archive/runtime boundary) | [`C-01`](C-01-approved-repo-surface.md) |
| CLI verb vocabulary and help posture (setup/author/pipeline/generate/inspect/doctor ordering and copy) | [`C-02`](C-02-rust-workspace-and-cli-command-surface.md) |
| Canonical `.system/` inputs + freshness fields used by planning packets | [`C-03`](C-03-canonical-artifact-manifest-contract.md) |
| Doctor baseline-state structure + “next safe action” semantics (including checklist and blocker taxonomy) | [`C-04`](C-04-resolver-result-and-doctor-blockers.md) |
| Trust header field set + ordering; proof ordering; deterministic markdown/JSON/inspect rendering | [`C-05`](C-05-renderer-and-proof-surfaces.md) |
| Fixture-backed execution demo boundary (fixture-only posture; live refusal semantics) | [`C-06`](C-06-fixture-execution-demo-boundary.md) |

## Verification checklist

The following checklist is normative for conformance execution and closeout:

- [ ] `cargo fmt --all -- --check`
- [ ] `cargo test --workspace`
- [ ] `cargo test -p system-cli`
- [ ] `cargo test -p system-compiler`
- [ ] change-scoped live author smoke
  - [ ] `SYSTEM_RUN_LIVE_AUTHOR_CHARTER_SMOKE=1 cargo test -p system-cli --test author_cli structured_inputs_author_charter_succeeds_with_live_codex_transport -- --exact`
  - [ ] `SYSTEM_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL=<model>` can override the smoke model locally
  - [ ] CI pins `SYSTEM_LIVE_AUTHOR_CHARTER_SMOKE_CODEX_MODEL=gpt-5.4-mini`
- [ ] `cargo install --path crates/cli` (smoke)
- [ ] `system --help` (smoke)
- [ ] `system generate --help` and `system inspect --help` (help parity)
- [ ] execution demo happy-path
  - [ ] `cargo run -p system-cli -- generate --packet execution.demo.packet --fixture-set basic`
  - [ ] `cargo run -p system-cli -- inspect --packet execution.demo.packet --fixture-set basic`
- [ ] live execution refusal (fixture-backed boundary)
  - [ ] from `tests/fixtures/execution_demo/basic/`: `cargo run -p system-cli -- generate --packet execution.live.packet`
