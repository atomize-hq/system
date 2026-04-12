# Changelog

All notable changes to this project are documented in this file.

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
