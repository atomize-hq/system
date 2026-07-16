# HCM-0.5 Final Proof Wall

**Captured at:** `2026-07-16T01:33:29Z`  
**Baseline HEAD:** `fc4e19e84385063c1ca3678e730c7d3b9e604ad8`  
**Selected entry handoff:** `20260715T230600Z--HCM-0-5--orchestration--packet-approved-design-freeze-entry`  
**Approved packet fingerprint:** `sha256:d75e8838ab1be329176a4c49dce8abad1d2001310c7a1981e1040502406aad99`  
**Canonical-plus-packet fingerprint:** `sha256:adeefeda6f14660f578b99ea2cebe1c1535f2038696e08db9bef6e7f1dba2d04`

This is documentation/design verification only. Contract membrane and External docks remain `TargetOnly`; `PG-CONTRACT-01`, `PG-DOCK-01`, and `PG-GATE-01` remain open.

## Exact subject

| Path | SHA-256 |
|---|---|
| `docs/specs/handbook-contract-membrane/00-README.md` | `e200f984dfdc2c8d122cfbfd4155d4162d83f82c80aa042462ba493489b0237f` |
| `docs/specs/handbook-contract-membrane/01-target-architecture.md` | `8e4dc24b0977d21b6012173fbb90f5fbfa7e912bbff70dfbfee3d3c624e8c42c` |
| `docs/specs/handbook-contract-membrane/02-semantic-model.md` | `2956b2f77e23e575aa8809010198d1fd980b6c64431506f681a69b078dfbc9c6` |
| `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` | `55eba6bd55893f2205d80123ab359fa072c08e1cdf0465d068e22d95982f2bb3` |
| `docs/specs/handbook-contract-membrane/04-phase-slice-map.md` | `47a9f4eedef851bbc2bc69d76120579d0d5e668834bae543cd4c1d06c7ab1855` |
| `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` | `148182d2372fd9ef8e1e8696b956c8da55a70c00f9e3e62b5f682b6b7b743b19` |
| `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md` | `987ff0568f891675cb5303c6e04a11747d6aa1a5ac7d698399fee17b97aa4266` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/SPEC.md` | `9c820425077dab95b24b8c039d03a8a8dea7ee2b86f4b21f68fc75bed4811cc8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/plan.md` | `7b4ffdb07a7525d8bbe122d800a8672d811929ce6ea3ca51e5e55459faa755f8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/todo.md` | `9bbd33adf072b206f5a4d212380f12b6a0f743c0df1a74b6258a7fb755baf7b8` |

Encoding: `repo-path-null-sha256-newline-v1`  
Aggregate: `sha256:adeefeda6f14660f578b99ea2cebe1c1535f2038696e08db9bef6e7f1dba2d04`

## Raw verification results

| Surface | Result |
|---|---|
| Entry and packet | exact v1.2 record/ledger parity; packet bytes equal reviewed commit and approved aggregate |
| Dependencies | completed HCM-0.4 evidence verified; HCM-0.9 remains abandoned; HCM-0.6 row unchanged |
| Markdown/examples | 63 balanced fences; 49 duplicate-rejecting JSON/YAML parses; 10 canonical relative links |
| Frozen authority | HCM-0.2/HCM-0.3 `05` prefix and Resolution/Snapshot/Projection section, HCM-0.2/0.3/0.4 proof gates, and frozen phase rows equal baseline |
| Lifecycle | immutable author/admission binding; deterministic draft genesis; exact non-circular transition/resulting fingerprints; content-addressed evaluation basis binds current active transition and immediately prior independent lock through every downstream identity |
| Contract model | exact identity/SemVer table, eight lifecycle edges, applicability split, all-of evidence, three cardinalities, freshness/provenance/consistency/six-dimension Resolution |
| Verdict/gate | seven verdicts, exact claim partition, complete 3x7 matrix, hard/required precedence, score limits, separate local/promotion policies |
| Process identity | request binds parent evaluation run, request ID/fingerprint, and dock run; every result status shares that closure; every nested candidate also binds a non-empty claim-ID subset; identity/claim mismatch is priority-5 `protocol_error` with one operational record and no exposed candidates/evidence |
| Operational record | total expected identity plus `not_created`/`created` process and `absent`/`invalid`/`valid` result branches cover spawn failure, malformed output, and transplantation; valid completion atomically retains fingerprinted complete normalized request and accepted-result admission bases plus the exact bounded untrusted candidate bundle for later crash-resumable replay |
| Runtime closure | closed `RuntimeDependencyClosure` descriptor binds manifest, typed launch, platform ABI, sorted members/roles and edges, bundle-only resolution policy, and canonical closure fingerprint; host fixed-point recomputation is required |
| Host outcomes | ordered priorities 1-8 produce failed/timed_out/cancelled/failed/protocol_error/refused/completed/protocol_error with first-match overlap rules |
| Phase-5 activation | only an exact active contract with a valid prior independent-lock transition may drive evaluation; bare `locked` refuses |
| Operations | exact 12 operations; four mutators classified additively with class, legal condition, exactly-one cardinality, atomic group, and receipt; eight read-only |
| Crosswalk scope | changed `03` table rows are exactly Contract membrane, External docks, and Contract-catalog decomposition; coupled notes only; runtime rows remain `TargetOnly` |
| Scope | canonical `00`-`06` semantic diff only; packet unchanged; no Rust/Cargo/runtime/schema/catalog leaf/HCM-0.6 change; runtime gates open |

## Command exits

- Parent structural/semantic assertion harness: exit 0; includes definition-author authority, draft genesis, exact lifecycle-chain fingerprints, active-basis current-head propagation, exact `03` changed-row allowlist, total execution-record discriminants, retained request/result/candidate-basis crash recovery, active-after-lock refusal, candidate identity/claim failure, recomputable runtime closure, and legal mutation-condition-domain checks.
- `python3 tools/check_archive_boundary.py`: exit 0.
- `python3 tools/check_archive_boundary.py --self-test`: exit 0; forbidden fixture rejected.
- Normal handoff validation: exit 0; 30 records, 87 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity.
- Historical admission self-test: exit 0.
- Orchestration-contract self-test: exit 0.
- `git diff --check`: exit 0.
- `npx gitnexus detect-changes --scope unstaged --repo handbook`: exit 0; seven docs, 48 Markdown symbols, zero affected processes, low risk.

## Required negative/refusal coverage

- exact definition/schema/matcher/fingerprint mismatch; non-author draft transition; author substitution; self-lock; stale basis; wrong genesis; fake first prior; null later prior; transition leaf/prior/order substitution; unlisted lifecycle edge; indeterminate applicability;
- wrong contract/claim/subject/case/source, stale or insufficient-Resolution evidence, missing all-of kind/case, cardinality surplus/shortage, contradictory evidence, incomplete verdict partition;
- hard/required non-pass above score, invalid effect/verdict pairing or weight;
- manifest/bundle/runtime/launch path/digest/kind/argv addition/removal/reorder/substitution; member/edge/path/role/mode/provider/order/policy/ABI closure change; host Python, shell, PATH, shebang, module/package lookup;
- v1 network enablement, inherited secret/config/proxy, unsafe path/output, duplicate/trailing/prose/ANSI/invalid JSON, stderr result, schema/fingerprint mismatch;
- cleanup uncertainty, deadline, host cancellation, crash/signal/nonzero, spawn failure, absent/invalid/valid result record branches, durable recovery after original request/full result/process/workspace/caller-state loss, requested-but-unobserved claim, changed case/Resolution, reordered/substituted claim, post-hoc candidate nonmembership, bare-locked evaluation, stale/deprecated/closed lifecycle head, lock/activation substitution, downstream lifecycle fingerprint mismatch, protocol error, refused/completed, unsolicited cancellation, and catch-all fact combinations;
- completed result or nested candidate transplanted across lifecycle-basis, `evaluation_run_id`, `request_id`, `request_fingerprint`, `run_id`, or claim identity produces priority-5 `protocol_error`, one operational record, and no exposed candidate or evidence receipt;
- mutation conditions outside `always` or a declared exact `data.*` result discriminant fail the combined inventory;
- validator/runner/host/adapter/Rust-native path attempting canonical evidence/verdict/gate/lifecycle/waiver/promotion authority.

A reviewer must replay the exact manifest and independently evaluate current bytes. The result table is verification evidence, not a review verdict.
