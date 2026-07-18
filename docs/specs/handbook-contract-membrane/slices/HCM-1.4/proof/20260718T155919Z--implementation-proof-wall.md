# HCM-1.4 Implementation Proof Wall

**Captured:** 2026-07-18T15:59:19Z

**Phase / slice:** `HCM-1` / `HCM-1.4`

**Selected handoff:** `20260718T133219Z--HCM-1-4--orchestration--implementation-packet-approved`

**Entry HEAD:** `3d0c57f8be0941457d9121ae5d853dd7bd92fbd7`

**Branch:** `feat/handbook-contract-membrane`

**Classification ceiling:** `BoundaryLanded`

## Entry, authority, and graph risk

The selected v1.2 handoff, subject fingerprint, dependency ancestry, clean entry
tree, and all three handoff-validator modes replayed before implementation. The
repo-owned HCM-1.4 SPEC/plan/todo, control-pack authority, GitNexus instructions,
and required agent skills were loaded before edits. No HCM-2 work was started.

GitNexus was refreshed before implementation. Fresh upstream summaries record:

| Symbol | Direct / total impact | Processes | Risk / containment |
|---|---:|---:|---|
| `resolve_profile_selection` | 20 / 29 | 0 | `HIGH`; full engine profile/registry wall |
| `run_setup` | 15 / 39 | 1 (`cli::setup::run`) | `HIGH`; exact setup/CLI/workspace wall and mutation tests |
| `acquire_authoring_lock` | 3 / 24 | 1 (`cli::author::run`) | `HIGH`; exact portability hunk, MSVC check, 47/22 author suites |
| `doctor_from_artifacts` | 1 / 13 | 1 (`cli::doctor::run`) | `LOW`; doctor/CLI/workspace wall |

The HIGH blast radii were warned before edits. Other touched setup/compiler/CLI
projection symbols were recorded as LOW or MEDIUM and are contained by the same
wall. GitNexus reports its optional FTS extension unavailable; impact results
remain exact callgraph results and no FTS-backed query claim is made.

## TDD and implementation evidence

The implementation was delivered in bounded RED/GREEN increments:

- engine decision API imports and exact contract tests first failed because the
  public decision types/resolver did not exist, then passed after the minimal
  immutable decision closure was added;
- inspection matrix tests first failed against the missing inspector, then
  passed for shipped and generated custom profile/kind/instance repositories;
- compiler setup/doctor contract tests first failed against missing typed APIs,
  then passed after direct closure adoption and shell-helper removal;
- CLI exact snake-case machine rendering first exposed Debug-variant strings,
  then passed after exhaustive transport-owned mappings; and
- the actual compiler MSVC check reproduced the pre-existing
  `libc::LOCK_EX` non-Unix failure, then passed after only the frozen cfg-selected
  local acquisition-operation hunk.

The landed boundary is one resolved profile, selected artifact registry, typed
decision closure, and descriptor-driven structural inspection report. Always
and optional decisions are explicit. Conditional decisions bind the exact
condition definition/fingerprint and remain only
`unresolved`/`evidence_contract_unavailable`/`indeterminate` with a null evidence
fingerprint. No condition evidence, override, observation, assertion, ambient
profile discovery, or bare-boolean request surface was added.

Setup and doctor consume the same closure. Setup writes no selected canonical
artifact, returns typed author-required/indeterminate actions, refuses rewrite
before mutation, and performs complete valid/non-indeterminate preflight before
the unchanged legacy reset owner. Doctor exposes closed typed rows; CLI alone
owns text, exact pretty JSON with one LF, and exit policy. Compiler setup/doctor
shell modules were removed. The fixed pre-membrane product universe remains only
in unmigrated sibling seams and the two expressly authorized author-test legacy
fixture writers.

## Focused, security, and platform proof

All commands below passed against a canonical-LF exact subject clone unless an
actual Windows host is named.

| Command / assertion | Result |
|---|---|
| `cargo fmt --all -- --check` | PASS |
| `cargo test -p handbook-engine --test hcm_1_4_profile_decisions` | PASS; 2 tests |
| `cargo test -p handbook-engine --test hcm_1_4_profile_inspection` | PASS; 6 Unix tests |
| HCM-1.2/HCM-1.3/profile-selection focused suites | PASS |
| `cargo test -p handbook-compiler --test setup` | PASS; 6 tests |
| `cargo test -p handbook-compiler --test doctor` | PASS; 3 tests |
| `cargo test -p handbook-compiler --test author` | PASS; 47 tests |
| `cargo test -p handbook-compiler --doc` | PASS; independent `E0451` and `E0624` privacy doctests |
| `cargo test -p handbook-cli --test cli_surface` | PASS; 98 active tests |
| `cargo test -p handbook-cli --test author_cli` | PASS; 22 tests |
| `cargo test --workspace --all-targets` | PASS |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS |
| `cargo check -p handbook-engine --target x86_64-pc-windows-msvc --all-features` | PASS on Windows x64, Rust 1.89.0 |
| `cargo check -p handbook-compiler --target x86_64-pc-windows-msvc --all-features` | PASS on Windows x64, Rust 1.89.0 |
| named engine `non_unix_repository_inspection_refuses_before_read` | PASS by execution on actual Windows MSVC host |
| named compiler `windows_profile_inspection_refusal_prevents_setup_mutation` | PASS by execution on actual Windows MSVC host |

The inspection wall covers missing required/optional/conditional paths,
present-valid and present-invalid conditional precedence, YAML syntax,
duplicate keys, non-object and wrong-schema documents, 0/1/1 MiB/1 MiB+1
documents, exact 8 MiB and 8 MiB+1 aggregate behavior, unreadable files,
directories, final/intermediate symlinks, custom descriptors, deterministic
order, and Windows fail-closed refusal. The engine unit wall also executes the
trusted-handle intermediate/final substitution-race proof. Results and errors
contain only bounded typed status/reason values and repo-relative selected paths.

The exact `author/mod.rs` diff adds only Unix `LOCK_EX` / non-Unix `0` selection
before the unchanged two-argument helper call. Both helper bodies/signatures and
the Unix `Drop` `LOCK_UN` path remain unchanged. Unix workspace and author tests
plus Windows compiler/runtime proof contain that exception.

## Package, source-tree, and scope proof

`cargo package -p handbook-engine --allow-dirty --no-verify` passed with 98
members, 1.1 MiB uncompressed, and 180.6 KiB compressed. The resulting archive
SHA-256 is
`f8fc9374f27b62fcb56e7bfe20dd8dc15ed1d6f0b1995c14e1ac291aed3181c1`.
The immutable HCM-1.2 package-definition manifest remains SHA-256
`a7678b8f3761e681e98f2b5e94459f4568c4d2ff9851e329bc5f776856829a23`.
Its literal 29 paths match the source tree and archive exactly; every size,
SHA-256, and byte sequence matches. The archive extracted to a fresh temporary
directory, `cargo check --all-features` passed there, and scans found no subject
workspace path.

`cargo metadata --no-deps --format-version 1` passed. `Cargo.toml`,
`Cargo.lock`, and all three affected crate manifests are byte-identical to entry.
No definition, schema, fixture, CI, flow, pipeline, SDK, content-authority, or
HCM-2 file changed. `handbook-compiler` is not claimed as publishable; its proof
is the unchanged manifest, metadata resolution, focused/full/workspace tests,
MSVC checks, and exact changed-source scope.

Production setup/doctor scans contain none of `CanonicalArtifactKind`,
`CANONICAL_ARTIFACT_ORDER`, `canonical_artifact_descriptors`, or fixed starter
template selection. Diff inspection confines `canonical_artifact_descriptors`
and `setup_starter_template_bytes` additions to the two authorized author-test
fixture helpers. `git diff --check`, Cargo-diff equality, fixed-symbol/input-
surface scans, and staged scope inspection pass.

## Classification and review gate

The maximum supported promotion is `BoundaryLanded` for setup/doctor
profile-decision and structural-readiness adoption, plus evidence for the narrow
setup/doctor subset of `PG-PROFILE-01`, `PG-KIND-01`, `PG-KIND-02`, and
`PG-ARTIFACT-01`. Those gates remain open for condition evidence/evaluation,
canonical content authority, materialization, semantic validation, intake,
lifecycle, renderer-derived views, capitalized Projection, publication,
downstream consumers, and HCM-2+.

This wall records no independent-review conclusion. The immutable v1.1 review
dispatch created from the complete staged subject binds the exact manifest and
aggregate fingerprint. A fresh isolated read-only built-in `default` reviewer
must return `CLEAN`; any valid finding requires bounded parent remediation, a
full replay, a new fingerprint/dispatch, and a different fresh reviewer.
