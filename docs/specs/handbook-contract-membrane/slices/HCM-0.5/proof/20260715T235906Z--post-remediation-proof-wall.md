# HCM-0.5 Post-Remediation Proof Wall

**Captured at:** `2026-07-15T23:59:06Z`  
**Baseline HEAD:** `fc4e19e84385063c1ca3678e730c7d3b9e604ad8`  
**Branch:** `feat/handbook-contract-membrane`  
**Parent orchestration:** `20260715T232415Z--HCM-0-5--design-freeze-orchestration`  
**Selected entry handoff:** `20260715T230600Z--HCM-0-5--orchestration--packet-approved-design-freeze-entry`  
**Review-1 input fingerprint:** `sha256:6e38b7feb6dce9d476d272076a4399b353cb9d496758f4185d778d618315ba6e`  
**Post-remediation pre-review fingerprint:** `sha256:a76dd6cfcf9124f9edf361f93697eb332847f9127f93181dc007a140cc077ff5`

This proof supersedes the first pre-review proof only for the remediated subject. The earlier report and Fresh Final Review 1 dispatch remain immutable lineage evidence. This remains documentation/design proof; all runtime gates stay open.

## Post-remediation subject manifest

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
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260715T232415Z--HCM-0-5--canonical-design-freeze-landing.json` | `66429245a4504f25d0055b7b599981d54ca896ec5c1afe8dfe6e060a1c90c4f3` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260715T234316Z--HCM-0-5--fresh-final-design-freeze-review-1.json` | `c423e697f93990598fb25b64563e73cd4cb0d972470bdf7a8ab0cb16290a638e` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/SPEC.md` | `9c820425077dab95b24b8c039d03a8a8dea7ee2b86f4b21f68fc75bed4811cc8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/proof/20260715T234159Z--design-freeze-proof-wall.md` | `80614e7cf94ace9487eb04ff6e6475eeb07b6d5e10e62e87699859b2d990fe8b` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/plan.md` | `7b4ffdb07a7525d8bbe122d800a8672d811929ce6ea3ca51e5e55459faa755f8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/todo.md` | `9bbd33adf072b206f5a4d212380f12b6a0f743c0df1a74b6258a7fb755baf7b8` |

Aggregate: `sha256:a76dd6cfcf9124f9edf361f93697eb332847f9127f93181dc007a140cc077ff5`

## Finding disposition and remediation

| Finding | Parent disposition | Bounded remediation | Verification added |
|---|---|---|---|
| F1 bundled-interpreter launch identity incomplete | accepted / `cross_document_repair` | added a closed typed native-or-bundled-interpreter launch binding/fingerprint, exact executable/interpreter/application paths and digests, typed ordered argv, request/execution binding, host revalidation, and bundled Python/jsonschema target rule across canonical `01`-`06` | interpreter/application/path/digest/argv kind/order/add/remove/substitution refusal assertions; no shell/PATH/shebang/host-Python fallback |
| F2 host-outcome mapping not total | accepted / `cross_document_repair` | replaced overlapping outcome prose with an ordered eight-row first-match table over cleanup, deadline, host cancellation, exit/signal, framing/schema/output, refused/completed result, and catch-all facts | exact priority/outcome inventory plus overlap cases: nonzero+malformed, host-cancel+child-result, deadline+cancelled-result, unsolicited cancellation |
| F3 new mutators classified read-only by earlier exhaustive rule | accepted / `cross_document_repair` | added the four HCM-0.5 mutators as an explicit additive extension to the exhaustive HCM-0.4 classification with authority class, condition, exactly-one cardinality, atomic group, and receipt behavior | combined-inventory assertion requires every non-read-only operation exactly once |
| F4 status bytes stale after closeout | accepted / `local_remediation` | changed `00`/`05` to durable closeout-governed authority wording with no post-review status mutation required | forbidden stale phrase assertion and closeout-governed status assertion |
| F5 volatile catalog line count | optional accepted | removed the line count and retained only canonical-monolith truth | volatile-count absence assertion |

## Complete proof replay

| Proof surface | Result | Evidence |
|---|---|---|
| Entry/authority identity | PASS | exact v1.2 entry record/ledger; approved packet commit/path/fingerprint; completed HCM-0.4 dependency; abandoned HCM-0.9 boundary |
| Markdown/examples/links | PASS | 61 balanced fences; 49 JSON/YAML examples parsed with duplicate-key rejection; 10 canonical relative links resolved |
| Frozen dependency authority | PASS | HCM-0.2/HCM-0.3 canonical prefix and Resolution/Snapshot/Projection sections, HCM-0.2/0.3/0.4 proof gates, and Phase-0 frozen rows match baseline |
| Contract/lifecycle/applicability/evidence | PASS | exact identity/SemVer, eight lifecycle edges, immutable authority, selector split, all-of kinds, three cardinalities, freshness/provenance, repeated consistency, six-dimension Resolution, and fixed precedence asserted |
| Verdict/gate | PASS | seven verdicts, exactly-once partition, complete three-by-seven matrix, hard/required precedence, score limits, and separate local/promotion decisions asserted |
| Launch identity | PASS | manifest/runtime closure includes exact typed launch vector; native and bundled-interpreter variants bind all paths/digests/argv; request cannot modify launch; execution record captures resolved vector |
| Host outcome totality | PASS | ordered priorities 1-8 map to `failed`, `timed_out`, `cancelled`, `failed`, `protocol_error`, `refused`, `completed`, `protocol_error`; first-match overlap cases asserted |
| Process/isolation/failure | PASS | exact one-document framing, direct bound launch, default deny, unconditional v1 no-network, bounded resources/output, timeout/cancellation/tree termination, cleanup quarantine, and no partial evidence asserted |
| Candidate/authority boundary | PASS | executor writes operational record only; each valid candidate independently creates one canonical evidence record; rejected/non-completed paths create none |
| First proof target | PASS | future Python/jsonschema dock requires bundled interpreter/application/dependencies/argv in exact closure; remote/executable/ambient/unsupported/tampered inputs refuse; no runtime claim |
| Combined mutation inventory | PASS | exact four additive HCM-0.5 mutators plus eight read-only operations; condition/cardinality/atomic-group/receipt behavior asserted; every non-read-only operation required exactly once |
| Scope/classification | PASS | semantic diff remains canonical `00`-`06`; no packet/Rust/Cargo/runtime/schema/catalog-leaf/HCM-0.6 mutation; Contract membrane and External docks remain `TargetOnly`; runtime gates open |

## Repository command results

| Command | Result |
|---|---|
| parent HCM-0.5 structural/semantic static proof replay | PASS — seven files, 61 fences, 49 parsed JSON/YAML examples, 10 relative links; launch/outcome/mutator/status remediations asserted |
| `python3 tools/check_archive_boundary.py` | PASS |
| `python3 tools/check_archive_boundary.py --self-test` | PASS — forbidden archive fixture rejected |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | PASS — 30 records, 81 current JSON internal dispatches, eight admitted legacy dispatches, exact ledger parity |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-v1-admission` | PASS |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py --self-test-orchestration-contract` | PASS |
| `git diff --check` | PASS |
| `npx gitnexus detect-changes --scope unstaged --repo handbook` | PASS — seven documentation files, 48 indexed Markdown symbols, zero affected processes, low risk |

## Negative/refusal additions from remediation

- Any launch-kind mismatch, interpreter/application/executable path or digest substitution, argv element kind/value/path change, argv addition/removal/reordering, request-supplied launch override, host Python, shebang, module-search-path, ambient package, shell, or PATH resolution refuses before spawn.
- For admitted runs, cleanup uncertainty wins as `failed`; otherwise deadline wins as `timed_out`; otherwise accepted host cancellation wins as `cancelled`; otherwise setup/crash/signal/nonzero wins as `failed`; otherwise framing/schema/fingerprint/result/output violations and unsolicited cancellation are `protocol_error`; exact refused/completed results map only after those rows; catch-all is `protocol_error`.
- The combined mutation classification contains each non-read-only operation exactly once. Pre-admission/refused branches create no mutator receipt; every admitted `dock.run` produces one operational receipt; each valid `contract.evidence.append` produces a distinct one-evidence receipt.

## Re-review boundary

A different fresh isolated built-in default reviewer must replay the next exact manifest and review all accepted remediations plus the complete subject. Any further accepted blocker requires another complete proof capture and another different fresh reviewer. After `CLEAN`, no subject byte may change before byte-identical proof/staging and the primary commit.
