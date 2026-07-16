# HCM-0.5 Final Proof Wall

**Captured at:** `2026-07-16T00:00:12Z`  
**Baseline HEAD:** `fc4e19e84385063c1ca3678e730c7d3b9e604ad8`  
**Branch:** `feat/handbook-contract-membrane`  
**Selected entry handoff:** `20260715T230600Z--HCM-0-5--orchestration--packet-approved-design-freeze-entry`  
**Approved packet fingerprint:** `sha256:d75e8838ab1be329176a4c49dce8abad1d2001310c7a1981e1040502406aad99`  
**Canonical-plus-packet fingerprint:** `sha256:153451817e4322754b8ac12aa8f72ff15e3236d7523c56f73119ccb13437b90d`

This captures documentation/design proof only. It makes no runtime, implementation, publication, or consumer-adoption claim. `PG-CONTRACT-01`, `PG-DOCK-01`, and `PG-GATE-01` remain open.

## Exact semantic subject

Encoding: `repo-path-null-sha256-newline-v1`, sorted by repository-relative path.

| Path | SHA-256 |
|---|---|
| `docs/specs/handbook-contract-membrane/00-README.md` | `e200f984dfdc2c8d122cfbfd4155d4162d83f82c80aa042462ba493489b0237f` |
| `docs/specs/handbook-contract-membrane/01-target-architecture.md` | `fdb2195263c87cf5be32f228aaa54b7f0de788d60061c554c50b6cf3085e4500` |
| `docs/specs/handbook-contract-membrane/02-semantic-model.md` | `2ed11ef906a7439c7a5b399d02176a274b5ff19c2ea759f9b49ef66cefe11ed6` |
| `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` | `55eba6bd55893f2205d80123ab359fa072c08e1cdf0465d068e22d95982f2bb3` |
| `docs/specs/handbook-contract-membrane/04-phase-slice-map.md` | `20b72db680cf6ac4af2519e7a40dcca0670336eca59759c793da8fc1dde2e3cc` |
| `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` | `0d625eb404479cb71b315826e06acc2fac7672d0a196e34856d3c2c740963270` |
| `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md` | `4bad6f51e9cf5068382e12efe31d3d8c71fe65d60d93dda647ea7d93299f871d` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/SPEC.md` | `9c820425077dab95b24b8c039d03a8a8dea7ee2b86f4b21f68fc75bed4811cc8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/plan.md` | `7b4ffdb07a7525d8bbe122d800a8672d811929ce6ea3ca51e5e55459faa755f8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/todo.md` | `9bbd33adf072b206f5a4d212380f12b6a0f743c0df1a74b6258a7fb755baf7b8` |

Aggregate: `sha256:153451817e4322754b8ac12aa8f72ff15e3236d7523c56f73119ccb13437b90d`

## Verification results

| Surface | Raw result |
|---|---|
| Entry handoff/ledger | exact v1.2 selected record and ledger parity validated |
| Packet identity | three current files equal reviewed commit `b2c702fa3bca2b0e1b5c0a1ec9bf51bd5f97d23a`; aggregate equals the approved packet fingerprint |
| Dependency boundaries | completed HCM-0.4 record/commit verified; latest HCM-0.9 record remains abandoned/non-authoritative |
| Markdown | 61 balanced fenced blocks; 49 JSON/YAML examples parsed with duplicate-key rejection; 10 canonical relative links/anchors resolved |
| Frozen sections | HCM-0.2/HCM-0.3 canonical `05` prefix, HCM-0.3 Resolution/Snapshot/Projection section, HCM-0.2/0.3/0.4 proof gates, and frozen Phase-0 rows equal baseline |
| Lifecycle | eight admitted edges exactly: draft-review_ready, draft-closed, review_ready-locked, review_ready-closed, locked-active, locked-deprecated, active-deprecated, deprecated-closed |
| Verdict/gate | seven verdicts; three gate-effect rows by seven verdict columns; hard/required and score-precedence assertions present |
| Evidence | all-of kind requirements, `exactly_one`/`at_least_one`/`all_declared_cases`, freshness/provenance, tuple consistency, and six-dimension Resolution assertions present |
| Dock launch | closed native/bundled-interpreter launch binding and fingerprint; exact executable/interpreter/application paths/digests; typed ordered argv; request substitution forbidden; execution captures resolved vector |
| Host outcomes | ordered priorities 1-8 map exactly to failed, timed_out, cancelled, failed, protocol_error, refused, completed, protocol_error; first-match rule and overlap examples present |
| Process security | exact one-document framing, direct bound-vector spawn, default-deny grants, unconditional v1 network denial, bounded resources/output, timeout/cancellation/tree cleanup, and no-partial-evidence assertions present |
| Candidate authority | executor creates one operational record for an admitted run; each valid candidate is separately admitted into one evidence record; invalid/non-completed paths create no evidence receipt |
| First target | future Python/jsonschema dock binds bundled interpreter, application, dependencies, and argv in exact closure; remote/executable/ambient/unsupported/tampered inputs refuse; target remains unimplemented |
| Operation inventory | 12 HCM-0.5 operations: four mutators classified additively with exact class/condition/cardinality/atomic group/receipt; eight read-only; combined inventory required exactly once |
| Classification/scope | Contract membrane and External docks remain `TargetOnly`; runtime gates open; semantic diff is canonical `00`-`06`; no packet/Rust/Cargo/runtime/schema/catalog-leaf/HCM-0.6 change |

## Command results

| Command | Raw result |
|---|---|
| parent structural/semantic assertion harness over the exact subject | exit 0; seven canonical files; 61 fences; 49 parsed JSON/YAML examples; 10 relative links |
| `python3 tools/check_archive_boundary.py` | exit 0 |
| `python3 tools/check_archive_boundary.py --self-test` | exit 0; forbidden archive fixture rejected |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | exit 0; 30 records, 81 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | exit 0 |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | exit 0 |
| `git diff --check` | exit 0 |
| `npx gitnexus detect-changes --scope unstaged --repo handbook` | exit 0; seven documentation files, 48 indexed Markdown symbols, zero affected processes, low risk |

## Required negative/refusal coverage

- exact identity/version/schema/matcher/fingerprint mismatch; stale basis; self-lock; unlisted lifecycle edge; indeterminate applicability;
- wrong contract/claim/subject/case/run/source, stale or insufficient-Resolution evidence, missing all-of kind/case, cardinality surplus/shortage, contradictory repeated evidence, incomplete partition;
- invalid warning/fail effect, hard/required non-pass above score threshold, invalid weight, duplicate/missing verdict;
- manifest/host-map/bundle/runtime/launch substitution, missing/extra/symlink/unsafe member, executable/interpreter/application/argv kind/path/digest/order/add/remove change, host Python or ambient resolver;
- any v1 network enablement, inherited secret/config/proxy state, unsafe input/output path, undeclared/oversized artifact;
- duplicate JSON member, extra/trailing/prose/ANSI stdout, invalid UTF-8/JSON, stderr-only result, schema/fingerprint/status mismatch;
- cleanup uncertainty, deadline, host cancellation, crash/signal/nonzero, malformed result, unsolicited cancellation, typed refusal, valid completion, and catch-all fact vectors each resolve through the first-match table;
- validator/runner/host/adapter/Rust-native path attempting canonical evidence, verdict, gate, lifecycle, waiver, or promotion authority.

A reviewer must independently replay this manifest and enough of the results to determine whether the bytes satisfy the packet. These results are evidence, not an independent review verdict.
