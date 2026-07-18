# Implementation Plan: HCM-1.4 Profile-Aware Setup and Doctor Decisions

## Authority and session fence

This plan is planning-only until the complete HCM-1.4 subject receives a fresh
`CLEAN` review and a parent planning closeout. A future implementation session
must be explicitly selected with that closeout. Packet approval starts no Rust
work and no HCM-2 work.

## Entry gate

Before implementation:

1. verify the selected HCM-1.4 planning closeout, reviewed planning commit,
   subject fingerprint, and clean branch;
2. rerun all three handoff validator modes;
3. refresh GitNexus with `npx gitnexus analyze --index-only` and run upstream
   impact on every existing symbol to change;
4. warn before editing `run_setup` because planning observed HIGH risk;
5. re-read the exact control-pack and packet sections named in the closeout;
6. confirm allowed paths and non-goals against live source; and
7. stop if content authority, new CLI input grammar, Cargo, definition assets,
   a compatibility bridge, or sibling behavior is required.

## Packet 1 — Condition-definition binding and profile decisions

**Files:** new `crates/engine/src/profile_decision.rs`,
`crates/engine/src/profile_selection.rs`,
`crates/engine/src/project_condition_registry.rs`,
`crates/engine/src/lib.rs`, and new
`crates/engine/tests/hcm_1_4_profile_decisions.rs`.

### Task 1 — RED exact public contract and input-surface absence

- write compile/API tests for every frozen type, owned field, accessor, error,
  shipped resolver, order, and return contract;
- write failing tests for exact always/optional/conditional derivation and
  missing condition-definition refusal; and
- prove no observation/evidence/freshness/assertion/boolean/override input API
  exists in engine, compiler, or CLI;
- record the RED command/result before production edits.

### Task 2 — GREEN minimal decision owner

- retain the already-admitted condition registry in the resolved profile;
- bind conditional descriptors to the exact definition/fingerprint and emit
  only `unresolved`/`evidence_contract_unavailable`/`indeterminate` with null
  evidence fingerprint;
- implement the exact package-owned shipped-profile decision resolver without
  cwd/environment/range/latest/repository-profile discovery;
- construct one registry and one immutable `ResolvedProfileDecisions` closure;
- expose exact profile/role/four-field-capability-identity/path/requiredness/
  condition truth; and
- keep all collections ordered and all errors bounded.

**Checkpoint:** focused decision tests pass; HCM-1.2/HCM-1.3 owner and registry
tests remain green; no repository I/O or fixed artifact type enters the module.

## Packet 2 — Descriptor-driven repository inspection

**Files:** new `crates/engine/src/profile_inspection.rs`, existing
`crates/engine/src/canonical_repo_support.rs` only if necessary, existing
`crates/engine/src/lib.rs`, and new
`crates/engine/tests/hcm_1_4_profile_inspection.rs`.

### Task 3 — RED path/read/structural matrix

- generate shipped/custom temporary repositories;
- fail on missing/present/invalid/unsafe/unreadable paths, duplicate YAML keys,
  wrong schemas, size ceilings, aggregate boundaries, and permutations; and
- write one named failing test for every exact low-level outcome to public
  status/reason row, including present-invalid conditional precedence; and
- add Unix component/final symlink substitution proof.

### Task 4 — GREEN minimal inspection

- iterate only selected artifact decisions;
- reuse strict bounded no-follow repository access;
- parse YAML into the JSON data model and validate by instance through the
  HCM-1.3 registry;
- return complete ordered inspection rows with bounded typed categories; and
- make no content-authority or semantic-validation claim.

**Checkpoint:** decision plus inspection suites pass on shipped/custom data;
package-owned definitions and manifests are byte-identical.

## Packet 3 — Setup cutover

**Files:** new `crates/compiler/src/profile_readiness.rs`,
`crates/compiler/src/setup.rs`, `crates/compiler/src/setup_shell.rs` only if
obsolete helpers must be removed, `crates/compiler/src/lib.rs`, and
`crates/compiler/tests/setup.rs`; then the test-only helper in
`crates/compiler/tests/author.rs`.

### Task 5 — RED profile-aware setup contract

- write failing tests for exact profile identity, artifact action table,
  exact compiler types/signatures/rows/errors, condition-unresolved truth, no
  artifact write, rewrite refusal, all setup modes, frozen status precedence,
  total request/root/error-payload mapping, closed two-variant doctor error
  mapping, fail-closed invalid-root behavior,
  exact SetupMode wire values, the closed SetupError code/ALL/accessor surface,
  and complete profile preflight before root/reset mutation; these positive
  contract tests fail because the APIs do not exist yet;
- include a custom descriptor path with no enum variant; and
- prove the same decision/inspection closure is retained in the plan/outcome.

### Task 6 — GREEN setup replacement

- replace fixed descriptor/template planning with HCM-1.4 decisions;
- remove selected-artifact mutation and return typed `author_required` instead;
- refuse rewrite before mutation;
- retain root/mode and the unchanged legacy runtime-state planner/applier,
  report reset failure honestly without claiming transactional rollback; and
- after the error API exists, run the exhaustive 13-row unit replay, one
  `compile_fail,E0451` external struct-literal doctest, and a separate
  `compile_fail,E0624` external crate-private-constructor doctest; and
- delete obsolete fixed setup helpers/imports without touching sibling seams.

### Task 6a — Decouple compiler author fixture from setup

- change only the author test helper/imports: replace its setup call with the
  exact test-local `legacy_authoring_fixture_repo` described in the SPEC;
- leave every production author file and every substantive author assertion
  unchanged except the separately frozen behavior-preserving `author/mod.rs`
  Windows-build portability hunk; and
- run all 47 compiler author tests before continuing.

**Checkpoint:** setup tests pass; fixed starter bytes are unchanged on disk;
invalid/indeterminate profile truth starts no reset; existing route-state tests
remain unchanged/green; setup source has no fixed-artifact decision symbol.

## Packet 4 — Doctor and CLI cutover

**Files:** the already-added `crates/compiler/src/profile_readiness.rs`,
`crates/compiler/src/doctor.rs`,
`crates/compiler/src/doctor_shell.rs` only for obsolete helper removal,
`crates/compiler/src/lib.rs`, `crates/compiler/src/author/mod.rs` only for the
exact frozen Windows-build portability hunk, `crates/compiler/tests/doctor.rs`, then
`crates/cli/src/{main.rs,setup.rs,doctor.rs,doctor_rendering.rs,exit_policy.rs}`
and `crates/cli/tests/{cli_surface.rs,author_cli.rs}` in two sequential
increments; `author_cli.rs` is test-helper-only.

### Task 7 — RED/GREEN doctor domain report

- write the exact closed report, two-variant error mapping, and status-
  precedence tests first;
- replace fixed baseline/checklist/blocker decisions with the shared closure;
- prove setup/doctor profile/condition/applicability/four-field-capability-
  identity equality;
- exhaustively replay `DoctorError::ALL` against the exact two-row derived
  kind/reason mapping and prove every doctor error uses one variant; and
- remove C-03/C-04 fixed-manifest truth from the new report without editing
  unrelated blocker/authoring code.

### Task 8 — RED/GREEN CLI rendering and exit policy

- preserve current command grammar and repo discovery;
- render all human wording in CLI modules;
- serialize the exact typed doctor JSON document with one trailing LF;
- make text, JSON, and exit status agree; and
- update snapshots/tests without adding profile-generated commands or a
  condition/profile input flag.

### Task 8a — Decouple CLI author fixture from setup

- replace only `author_cli.rs`'s setup subprocess helper/imports with the exact
  test-local legacy fixture writer;
- change no production author command behavior or substantive author assertion;
  and
- run all 22 CLI author tests before continuing.

### Task 8b — GREEN exact compiler Windows-build portability prerequisite

- rerun fresh upstream impact for HIGH-risk `acquire_authoring_lock` and warn
  before editing;
- record the current compiler MSVC `libc::LOCK_EX` failure as the RED result;
- change only `acquire_authoring_lock` in
  `crates/compiler/src/author/mod.rs`: cfg-select local `lock_operation` as
  `libc::LOCK_EX` on Unix and ignored integer `0` on non-Unix before passing it
  to the unchanged two-argument helper;
- prove both helper signatures/bodies and the `Drop` `LOCK_UN` call are byte-
  unchanged, Unix acquire/retry/unlock behavior remains exact, non-Unix behavior
  remains the existing no-op, and no other author byte/behavior moves; and
- rerun compiler MSVC check plus all 47 compiler and 22 CLI author tests.

**Checkpoint:** compiler and CLI focused suites pass; CLI source owns wording;
engine decisions contain no command/prose/exit fields.

## Packet 5 — Regression, proof, and closeout

### Task 9 — Full regression and package proof

- run format, focused HCM-1.2/HCM-1.3/HCM-1.4, compiler, CLI, workspace,
  clippy, actual Windows-host runtime, handoff, and engine-package commands
  from the SPEC;
- replay the literal 29-member definition manifest against tree and package;
- extract/check the engine package and run the exact compiler source-tree/
  workspace-metadata boundary proof; do not claim compiler publication;
- prove exact changed-path set equality and no fixture/absolute-path leak; and
- run fixed-symbol/import absence, secret, untracked whitespace, and sibling-
  scope scans.

### Task 10 — Narrow control-pack update

- update only `00`, affected `03` rows, `04`, and the HCM-1.4 gate in `06`;
- record exact real-path evidence and the narrow classification ceiling;
- keep content authority and every later gate open; and
- create the immutable implementation proof wall.

### Task 11 — Fresh review/remediation loop

- create one schema-valid immutable v1.1 review dispatch with the exact complete
  subject manifest and raw proof results;
- spawn a fresh isolated read-only built-in `default` reviewer;
- validate every finding against live authority;
- remediate valid findings in scope, rerun the full proof wall, and use a
  different fresh reviewer on the new fingerprint; and
- repeat until `CLEAN` or a genuine stop condition.

### Task 12 — Two-commit closeout

- replay the final clean manifest byte-for-byte;
- run staged GitNexus detect, diff, scope, secret, and status gates;
- commit the reviewed HCM-1.4 implementation/control-pack/proof subject;
- create one parent v1.2 completed handoff that names but does not start HCM-2;
- rebuild and validate the deterministic ledger in all three modes; and
- commit only the handoff/ledger mechanical closeout artifacts.

## Dependency order

```text
Packet 1 decision contract
  -> Packet 2 repository inspection
  -> Packet 3 setup adoption
  -> compiler author-test fixture decoupling
  -> Packet 4 doctor domain adoption
  -> Packet 4 CLI rendering/exit adoption
  -> CLI author-test fixture decoupling
  -> Packet 5 regression/control proof
  -> fresh review -> remediation -> different fresh review
  -> primary reviewed commit
  -> parent handoff + ledger commit
```

Editing packets are sequential because they share public contracts and final
integration truth. Reviewers are always read-only. An intermediate packet
review does not replace the different-fresh final slice review over the entire
proof wall.

## Risks and mitigations

| Risk | Impact | Mitigation |
|---|---|---|
| HIGH `run_setup` blast radius | setup regression or partial mutation | fresh impact warning, test-first replacement, complete preflight before reset, focused + CLI + workspace wall |
| condition evidence authority is absent | false authority or invented truth | accept no evidence input; bind exact definition and return only unresolved/unavailable/indeterminate |
| Environment Context self-proves applicability | circular authority | file/environment/profile state cannot change unresolved or populate an evidence fingerprint |
| HCM-1.4 steals Phase 2 | premature canonical authority or mutation | structural-only status vocabulary; no artifact writes/intake/rendering/semantic approval |
| old fixed path survives as fallback | permanent dual universe | exact source/import absence scan; no compatibility mode/profile/dispatch |
| capability identity report is mistaken for full registry metadata | false closure claim | freeze only the four-field identity projection; leave bindings/cardinality/validators/rules registry-owned |
| machine report becomes prose/API drift | transport-owned semantics | closed typed report; CLI-only wording; HCM-4 still owns final DTO/schema envelope |
| path race or symlink escape | out-of-repo read | strict descriptor-based no-follow open, size ceilings, race proof, bounded errors |
| compiler Windows build already fails in author lock call | mandatory platform proof impossible | exact behavior-preserving cfg portability hunk, HIGH-risk warning, compiler MSVC check, and full 47/22 author suites |
| Windows fail-closed path is only compiled | unproved platform safety | execute named focused tests on an actual Windows MSVC host or stop |
| compiler package is unpublishable with path-only deps | impossible proof gate | engine package proof plus exact compiler source-tree/workspace proof; publication stays separate |
| author suites assume old setup scaffolding | unavoidable workspace failure | two allowlisted test-helper-only legacy fixture replacements; production authoring unchanged outside the exact portability hunk; assertions unchanged; full 47/22 suites required |
| broad CLI test churn hides sibling regression | scope drift | exact test-helper diff ceiling, named test disposition inventory, and full sibling suite |

## Permanent stop checks

- no code work in the planning session;
- no packet execution without exact reviewed planning selector;
- no Cargo, definition, schema, production authoring beyond the exact behavior-
  preserving `author/mod.rs` portability hunk, flow, pipeline, SDK, or Phase 2
  file; only that hunk and the two exact author-test fixture helpers may move;
- no condition observation/evidence/freshness/assertion/boolean/profile flag or
  ambient inference; no outcome other than explicit unresolved;
- no fixed-enum/profile bridge or Markdown/YAML dual path;
- no canonical artifact write or semantic-authority claim;
- no mutation before complete safe preflight;
- no transactional reset-repair or compiler-publication scope;
- no self-review substitution; and
- no second slice after closeout.
