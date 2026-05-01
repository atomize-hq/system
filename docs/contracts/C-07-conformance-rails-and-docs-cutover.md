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
  - Any change to the installed Codex packaging layout, runtime wrapper behavior, or run-artifact file set.
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
- installed Codex packaging and charter-intake runtime drift cannot land without automated detection

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
- what the installed Codex packaging and charter-intake runtime must prove
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
- validate packaging install smoke (`bash tools/ci/install-smoke.sh`)
- validate installed charter-intake runtime smoke (`bash tools/ci/codex-skill-live-smoke.sh`)

CI MUST run the same logical checks (or stricter) on supported targets.

For the shipped Codex packaging wedge, CI MUST also prove:

- repo `.agents/skills/system-charter-intake/` and `.agents/skills/system/` regenerate deterministically from `install/system-home/` as thin generated projections
- the installed `~/system/` home contains exactly the curated runtime contract:
  - `SKILL.md.tmpl`
  - `SKILL.md`
  - `agents/openai.yaml`
  - `charter-intake/SKILL.md.tmpl`
  - `charter-intake/SKILL.md`
  - `runtime-manifest.json`
  - `bin/system`
  - `resources/authoring/charter_authoring_method.md`
  - `resources/charter/CHARTER_INPUTS.yaml.tmpl`
  - `resources/charter/charter_inputs_directive.md`
  - `.agents/skills/system/SKILL.md`
  - `.agents/skills/system/agents/openai.yaml`
  - `.agents/skills/system-charter-intake/SKILL.md`
  - `.agents/skills/system-charter-intake/agents/openai.yaml`
- `runtime-manifest.json` contains at minimum:
  - `skill_name`
  - `system_release_version`
  - `manifest_version`
  - `generated_at_utc`
- `~/system/` is the installed home, with installed thin projections under `~/system/.agents/skills/*`
- `~/.codex/skills/system*` is discovery glue only and points into `~/system/.agents/skills/*`
- `tools/codex/install.sh` owns the installed `~/system/` home, installs `~/system/bin/system` as the only executable, installs runtime guidance under `~/system/resources/**`, and refreshes the thin Codex discovery glue
- `tools/codex/dev-setup.sh` creates the dev symlink mode only, and normal install after dev setup restores the `~/.codex/skills/system* -> ~/system/.agents/skills/*` discovery topology cleanly
- the installed home does not contain `~/system/bin/system-charter-intake`
- the installed home does not contain `~/system/share/**`
- the installed runtime may machine-parse only `system doctor --json`
- validate/write steps rely on exit code plus persisted stdout/stderr transcripts only, with no new machine-readable authoring contract

### Supported install-smoke targets

Reduced v1 MUST treat these as supported install-smoke targets:

- `macOS arm64`
- `Linux x86_64`

Install smoke MUST, at minimum:

- build and install `system` from this repository (`cargo install --path crates/cli`)
- execute `system --help` (and any required verb-level help) without panicking
- run `bash tools/ci/install-smoke.sh`
- run `bash tools/ci/codex-skill-live-smoke.sh`

### Proof-surface drift must fail fast

Conformance rails MUST fail fast if:

- trust header ordering or wording drifts (`C-05`)
- refusal categories, ordering, or “exact next safe action” semantics drift (`C-04`)
- inspect proof ordering drifts (`C-05`)
- demo-boundary wording drifts (fixture-backed labeling or live refusal semantics; `C-06`)
- installed charter-intake runtime behavior drifts from the locked happy-path sequence, refusal order, or run-artifact contract

This is typically enforced by golden-output tests and/or snapshot tests at the compiler or CLI surface.

### Docs and help must reflect the supported story

Docs and CLI help MUST NOT:

- imply Python is a supported runtime path for reduced v1
- imply live slice execution is supported
- treat `archived/` as a runtime input (it is reference-only per `C-01`)

Docs and CLI help MUST:

- identify `system doctor --json` as the only machine-readable readiness surface for the installed charter-intake skill
- identify `system author charter --validate --from-inputs <path|->` as the mutation-free charter preflight surface
- identify `system author charter --from-inputs <path|->` as deterministic and compiler-owned
- identify `install/system-home/` as the authored source of install-home skill content
- identify `~/system/` as the installed home for the Codex-facing install surface
- identify `~/system/bin/system` as the only installed executable for the Codex-facing install surface
- identify `~/system/runtime-manifest.json` and `~/system/resources/**` as installed runtime contract surfaces
- describe repo `.agents/skills/*` as thin generated projections only
- describe `~/.codex/skills/system*` as discovery glue only, pointing into `~/system/.agents/skills/*`
- describe `tools/codex/install.sh` as owning the installed `~/system/` home, `~/system/bin/system`, `~/system/resources/**`, and Codex discovery-glue refresh
- state that there is no installed `~/system/bin/system-charter-intake` and no installed `~/system/share/**`

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
- [ ] `bash tools/ci/install-smoke.sh`
- [ ] `bash tools/ci/codex-skill-live-smoke.sh`
- [ ] `cargo install --path crates/cli` (smoke)
- [ ] `system --help` (smoke)
- [ ] `system doctor --help` and `system author charter --help` (help parity)
- [ ] `system generate --help` and `system inspect --help` (help parity)
- [ ] generated repo `.agents/skills/system-charter-intake/` and `.agents/skills/system/` are deterministic thin-projection outputs from `install/system-home/`
- [ ] generated runtime root file set matches the locked contract
- [ ] `~/system/` is the installed home and installed thin projections live under `~/system/.agents/skills/*`
- [ ] `~/.codex/skills/system*` is discovery glue only and points into `~/system/.agents/skills/*`
- [ ] normal install after dev setup restores the `~/.codex/skills/system* -> ~/system/.agents/skills/*` discovery topology cleanly
- [ ] `~/system/bin/system` is the only installed executable for this Codex surface
- [ ] `~/system/runtime-manifest.json` and `~/system/resources/**` exist as installed runtime contract surfaces
- [ ] `~/system/bin/system-charter-intake` does not exist
- [ ] `~/system/share/**` does not exist
- [ ] the installed runtime happy path is `doctor --json` -> optional `system setup` -> `doctor --json` -> `author charter --validate --from-inputs` -> `author charter --from-inputs` -> final `doctor --json`
- [ ] outside-git-repo refusal happens before questioning
- [ ] existing-charter refusal is covered as a first-class smoke case
- [ ] execution demo happy-path
  - [ ] `cargo run -p system-cli -- generate --packet execution.demo.packet --fixture-set basic`
  - [ ] `cargo run -p system-cli -- inspect --packet execution.demo.packet --fixture-set basic`
- [ ] live execution refusal (fixture-backed boundary)
  - [ ] from `tests/fixtures/execution_demo/basic/`: `cargo run -p system-cli -- generate --packet execution.live.packet`
