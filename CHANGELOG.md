# Changelog

All notable changes to this project are documented in this file.

## [0.3.0.0] - 2026-04-14

### Added
- Added the shipped M2 `pipeline compile` wedge for `pipeline.foundation_inputs` + `stage.10_feature_spec`, including payload-only success output, `--explain` proof mode, and compiler-owned route-basis persistence through `pipeline resolve`.
- Added bounded compile-time document loading and refusal coverage for missing route basis, stale or forged route snapshots, inactive stages, missing required artifacts or variables, and shared golden outputs reused by both compiler and CLI test suites.
- Added proof-corpus and CLI regression coverage for the new compile surface, including refusal goldens, help snapshots, and route-basis state seeds that lock the operator contract in place.

### Changed
- Tightened compile safety by rejecting incomplete profile packs, refusing symlinked compile inputs, honoring the selected stage work level when filtering scoped blocks, and restricting compile selection to the declared M2 stage boundary.
- Normalized compile-facing `repo_root` handling to the stable `${repo_root}` symbolic root and updated route-state contracts so persisted `route_basis` snapshots stay deterministic and auditable across resolve and compile runs.
- Realigned README, DESIGN, support docs, and pipeline contracts around the shipped M2 compile surface so docs, help text, snapshots, and proof-corpus expectations all describe the same supported workflow.

## [0.2.0.0] - 2026-04-13

### Added
- Added the supported reduced-v1 `pipeline` command family with `pipeline list`, `pipeline show`, `pipeline resolve`, and `pipeline state set` so operators can inspect pipeline inventory, compute route truth, and persist narrow routing state without manual repo archaeology.
- Added compiler-owned pipeline catalog loading, canonical ID discovery, deterministic route evaluation, typed route-state persistence, advisory locking, revision-conflict refusals, and shared foundation-family proof corpus goldens for `pipeline resolve` and `pipeline state set`.
- Added dedicated CLI and compiler coverage for pipeline inventory inspection, shorthand ambiguity, activation drift, malformed-state refusals, lock-aware mutation behavior, help snapshots, and proof-corpus rendering.

### Changed
- Tightened the M1 route/state contract so stage `sets`, activation clauses, persisted refs, and runner/profile mutations all validate against explicit reduced-v1 grammar and allowlists instead of drifting into loose stringly-typed behavior.
- Improved the operator proof surface with route-basis summaries, declared route metadata in `pipeline show`, clearer invalid-canonical-id and inventory-refusal behavior, and metadata-only inventory commands that ignore unrelated broken pipeline files during inspection.
- Realigned product docs, contracts, seam/governance artifacts, README guidance, and CLI support docs around the shipped M1 pipeline/routing spine and the shared proof corpus.

## [0.1.0.0] - 2026-04-11

### Added
- Added a compiler-owned Rust pipeline loader that reads the current two-document pipeline YAML shape and exposes typed pipeline definitions for the reduced-v1 foundation-family wedge.
- Added strict parser-gate coverage for duplicate stage ids, stage file validity, activation operators, extra YAML documents, and other refusal paths so unsupported shapes fail loudly instead of becoming compatibility debt.

### Changed
- Tightened reduced-v1 activation handling to boolean equality only, matching the narrow contract the current Rust proof corpus actually uses.
- Clarified glossary and legacy-system docs so the supported Rust loader contract no longer implies broader legacy activation semantics than the compiler ships today.

## [0.0.1.0] - 2026-04-07

### Added
- Introduced the Rust workspace, `system` CLI, and `system-compiler` library as the reduced-v1 product path for planning packet generation, inspect proof surfaces, doctor recovery output, and explicit live-execution refusal.
- Added typed canonical artifact ingestion, manifest/freshness computation, resolver contracts, packet rendering surfaces, and fixture-backed execution demo support with broad Rust test coverage.
- Added CI rails for `rustfmt`, `clippy`, workspace tests, archive-boundary enforcement, and cross-platform install smoke checks on Linux and macOS.
- Added reduced-v1 contracts and Rust-first docs entrypoints covering approved repo surface, command surface, manifest/freshness truth, resolver/blocker behavior, renderer surfaces, fixture-demo boundaries, and conformance rails.

### Changed
- Cut the repo over to a Rust-first supported surface, including updated README guidance, supported-command docs, and archived snapshots of legacy generated artifacts for reference-only use.
- Hardened repo discovery, nested git boundary handling, packet budgeting, omission notes, and help-text drift guards to match shipped CLI behavior.

### Removed
- Removed committed legacy `dist/` generated outputs from the supported repo surface now that the archived snapshot preserves the old scaffold artifacts.
