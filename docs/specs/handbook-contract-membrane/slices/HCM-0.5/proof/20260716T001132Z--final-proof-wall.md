# HCM-0.5 Final Proof Wall

**Captured at:** `2026-07-16T00:11:32Z`  
**Baseline HEAD:** `fc4e19e84385063c1ca3678e730c7d3b9e604ad8`  
**Selected entry handoff:** `20260715T230600Z--HCM-0-5--orchestration--packet-approved-design-freeze-entry`  
**Approved packet fingerprint:** `sha256:d75e8838ab1be329176a4c49dce8abad1d2001310c7a1981e1040502406aad99`  
**Canonical-plus-packet fingerprint:** `sha256:fdf4bb8e871a566b9ecd42929c84f624713aa89fd82ac2cc560ab619c7529002`

This is documentation/design verification only. Contract membrane and External docks remain `TargetOnly`; `PG-CONTRACT-01`, `PG-DOCK-01`, and `PG-GATE-01` remain open.

## Exact subject

| Path | SHA-256 |
|---|---|
| `docs/specs/handbook-contract-membrane/00-README.md` | `e200f984dfdc2c8d122cfbfd4155d4162d83f82c80aa042462ba493489b0237f` |
| `docs/specs/handbook-contract-membrane/01-target-architecture.md` | `fdb2195263c87cf5be32f228aaa54b7f0de788d60061c554c50b6cf3085e4500` |
| `docs/specs/handbook-contract-membrane/02-semantic-model.md` | `e2d7164fd724713ba5dac95695167f01e91432d4c39d16aef8fe9c4d3e0ccf1f` |
| `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` | `55eba6bd55893f2205d80123ab359fa072c08e1cdf0465d068e22d95982f2bb3` |
| `docs/specs/handbook-contract-membrane/04-phase-slice-map.md` | `20b72db680cf6ac4af2519e7a40dcca0670336eca59759c793da8fc1dde2e3cc` |
| `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` | `4aa77ab989d4e819b345ef6550c099152c2198a1a65da78ba3b54e2948a3226d` |
| `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md` | `2b60794c594cc4d0fb2b7b835f8686bec7453d1445d9e522a70bb08c10dbdbae` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/SPEC.md` | `9c820425077dab95b24b8c039d03a8a8dea7ee2b86f4b21f68fc75bed4811cc8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/plan.md` | `7b4ffdb07a7525d8bbe122d800a8672d811929ce6ea3ca51e5e55459faa755f8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/todo.md` | `9bbd33adf072b206f5a4d212380f12b6a0f743c0df1a74b6258a7fb755baf7b8` |

Encoding: `repo-path-null-sha256-newline-v1`  
Aggregate: `sha256:fdf4bb8e871a566b9ecd42929c84f624713aa89fd82ac2cc560ab619c7529002`

## Raw verification results

| Surface | Result |
|---|---|
| Entry and packet | exact v1.2 record/ledger parity; packet bytes equal reviewed commit and approved aggregate |
| Dependencies | completed HCM-0.4 evidence verified; HCM-0.9 remains abandoned; HCM-0.6 row unchanged |
| Markdown/examples | 61 balanced fences; 49 duplicate-rejecting JSON/YAML parses; 10 canonical relative links |
| Frozen authority | HCM-0.2/HCM-0.3 `05` prefix and Resolution/Snapshot/Projection section, HCM-0.2/0.3/0.4 proof gates, and frozen phase rows equal baseline |
| Contract model | exact identity/SemVer table, eight lifecycle edges, applicability split, all-of evidence, three cardinalities, freshness/provenance/consistency/six-dimension Resolution |
| Verdict/gate | seven verdicts, exact claim partition, complete 3x7 matrix, hard/required precedence, score limits, separate local/promotion policies |
| Process identity | request binds parent evaluation run, request ID/fingerprint, and dock run; completed/refused/cancelled share common identity closure; result/execution/candidate/evidence equality-check it |
| Launch identity | closed native/bundled-interpreter executable/interpreter/application/typed-argv binding and fingerprint; request substitution/ambient resolution forbidden |
| Host outcomes | ordered priorities 1-8 produce failed/timed_out/cancelled/failed/protocol_error/refused/completed/protocol_error with first-match overlap rules |
| Operations | exact 12 operations; four mutators classified additively with class, legal condition, exactly-one cardinality, atomic group, and receipt; eight read-only |
| Scope | canonical `00`-`06` semantic diff only; packet unchanged; no Rust/Cargo/runtime/schema/catalog leaf/HCM-0.6 change; runtime gates open |

## Command exits

- Parent structural/semantic assertion harness: exit 0; includes evaluation/request/run transplantation and legal mutation-condition-domain checks.
- `python3 tools/check_archive_boundary.py`: exit 0.
- `python3 tools/check_archive_boundary.py --self-test`: exit 0; forbidden fixture rejected.
- Normal handoff validation: exit 0; 30 records, 82 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity.
- Historical admission self-test: exit 0.
- Orchestration-contract self-test: exit 0.
- `git diff --check`: exit 0.
- `npx gitnexus detect-changes --scope unstaged --repo handbook`: exit 0; seven docs, 48 Markdown symbols, zero affected processes, low risk.

## Required negative/refusal coverage

- exact definition/schema/matcher/fingerprint mismatch; stale basis; self-lock; unlisted lifecycle edge; indeterminate applicability;
- wrong contract/claim/subject/case/source, stale or insufficient-Resolution evidence, missing all-of kind/case, cardinality surplus/shortage, contradictory evidence, incomplete verdict partition;
- hard/required non-pass above score, invalid effect/verdict pairing or weight;
- manifest/bundle/runtime/launch path/digest/kind/argv addition/removal/reorder/substitution; host Python, shell, PATH, shebang, module/package lookup;
- v1 network enablement, inherited secret/config/proxy, unsafe path/output, duplicate/trailing/prose/ANSI/invalid JSON, stderr result, schema/fingerprint mismatch;
- cleanup uncertainty, deadline, host cancellation, crash/signal/nonzero, protocol error, refused/completed, unsolicited cancellation, and catch-all fact combinations;
- completed result or candidate transplantation across `evaluation_run_id`, `request_id`, `request_fingerprint`, or `run_id` produces `protocol_error` and no evidence;
- mutation conditions outside `always` or a declared exact `data.*` result discriminant fail the combined inventory;
- validator/runner/host/adapter/Rust-native path attempting canonical evidence/verdict/gate/lifecycle/waiver/promotion authority.

A reviewer must replay the exact manifest and independently evaluate current bytes. The result table is verification evidence, not a review verdict.
