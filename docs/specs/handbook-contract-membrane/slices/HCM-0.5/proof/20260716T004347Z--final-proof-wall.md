# HCM-0.5 Final Proof Wall

**Captured at:** `2026-07-16T00:43:47Z`  
**Baseline HEAD:** `fc4e19e84385063c1ca3678e730c7d3b9e604ad8`  
**Selected entry handoff:** `20260715T230600Z--HCM-0-5--orchestration--packet-approved-design-freeze-entry`  
**Approved packet fingerprint:** `sha256:d75e8838ab1be329176a4c49dce8abad1d2001310c7a1981e1040502406aad99`  
**Canonical-plus-packet fingerprint:** `sha256:19adc667c36300c9d95dd653cbd7c0b97e8e9d89ef603c0816720017a3be0ee1`

This is documentation/design verification only. Contract membrane and External docks remain `TargetOnly`; `PG-CONTRACT-01`, `PG-DOCK-01`, and `PG-GATE-01` remain open.

## Exact subject

| Path | SHA-256 |
|---|---|
| `docs/specs/handbook-contract-membrane/00-README.md` | `e200f984dfdc2c8d122cfbfd4155d4162d83f82c80aa042462ba493489b0237f` |
| `docs/specs/handbook-contract-membrane/01-target-architecture.md` | `2d0654bed3ea0d106dcf3d01a90ca984223fd90544f37330175f221350c7090f` |
| `docs/specs/handbook-contract-membrane/02-semantic-model.md` | `c8b24d714065118f532516dec60c5fa4443778d1ee9ed0e010eebd279c6060f3` |
| `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` | `55eba6bd55893f2205d80123ab359fa072c08e1cdf0465d068e22d95982f2bb3` |
| `docs/specs/handbook-contract-membrane/04-phase-slice-map.md` | `a819147e0b8ae53e209aa721cf261ed9308cb4ad05effff2bbbaa3a88832a28a` |
| `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` | `6c69fd1ae00d076783d4191607cb6de85a36015ed5ff9bd47900b6f3251a0270` |
| `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md` | `d2bd57c228ea9e5b9f4c7580cc56f7dd5ba10e95593afad7e2a6d0b862fdf55e` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/SPEC.md` | `9c820425077dab95b24b8c039d03a8a8dea7ee2b86f4b21f68fc75bed4811cc8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/plan.md` | `7b4ffdb07a7525d8bbe122d800a8672d811929ce6ea3ca51e5e55459faa755f8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/todo.md` | `9bbd33adf072b206f5a4d212380f12b6a0f743c0df1a74b6258a7fb755baf7b8` |

Encoding: `repo-path-null-sha256-newline-v1`  
Aggregate: `sha256:19adc667c36300c9d95dd653cbd7c0b97e8e9d89ef603c0816720017a3be0ee1`

## Raw verification results

| Surface | Result |
|---|---|
| Entry and packet | exact v1.2 record/ledger parity; packet bytes equal reviewed commit and approved aggregate |
| Dependencies | completed HCM-0.4 evidence verified; HCM-0.9 remains abandoned; HCM-0.6 row unchanged |
| Markdown/examples | 63 balanced fences; 49 duplicate-rejecting JSON/YAML parses; 10 canonical relative links |
| Frozen authority | HCM-0.2/HCM-0.3 `05` prefix and Resolution/Snapshot/Projection section, HCM-0.2/0.3/0.4 proof gates, and frozen phase rows equal baseline |
| Lifecycle | immutable author/admission binding; deterministic draft genesis; exact non-circular transition and resulting-lifecycle preimages; author/distinct-lock authority and immediate-prior chain checks |
| Contract model | exact identity/SemVer table, eight lifecycle edges, applicability split, all-of evidence, three cardinalities, freshness/provenance/consistency/six-dimension Resolution |
| Verdict/gate | seven verdicts, exact claim partition, complete 3x7 matrix, hard/required precedence, score limits, separate local/promotion policies |
| Process identity | request binds parent evaluation run, request ID/fingerprint, and dock run; every result status shares that closure; every nested candidate also binds a non-empty claim-ID subset; identity/claim mismatch is priority-5 `protocol_error` with one operational record and no exposed candidates/evidence |
| Operational record | total expected identity plus `not_created`/`created` process and `absent`/`invalid`/`valid` result branches cover spawn failure, malformed output, and transplantation without fabricating a valid result closure |
| Runtime closure | closed `RuntimeDependencyClosure` descriptor binds manifest, typed launch, platform ABI, sorted members/roles and edges, bundle-only resolution policy, and canonical closure fingerprint; host fixed-point recomputation is required |
| Host outcomes | ordered priorities 1-8 produce failed/timed_out/cancelled/failed/protocol_error/refused/completed/protocol_error with first-match overlap rules |
| Operations | exact 12 operations; four mutators classified additively with class, legal condition, exactly-one cardinality, atomic group, and receipt; eight read-only |
| Scope | canonical `00`-`06` semantic diff only; packet unchanged; no Rust/Cargo/runtime/schema/catalog leaf/HCM-0.6 change; runtime gates open |

## Command exits

- Parent structural/semantic assertion harness: exit 0; includes definition-author authority, draft genesis, exact lifecycle-chain fingerprints, total execution-record discriminants, candidate identity/claim failure, recomputable runtime closure, and legal mutation-condition-domain checks.
- `python3 tools/check_archive_boundary.py`: exit 0.
- `python3 tools/check_archive_boundary.py --self-test`: exit 0; forbidden fixture rejected.
- Normal handoff validation: exit 0; 30 records, 84 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity.
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
- cleanup uncertainty, deadline, host cancellation, crash/signal/nonzero, spawn failure, absent/invalid/valid result record branches, protocol error, refused/completed, unsolicited cancellation, and catch-all fact combinations;
- completed result or nested candidate transplanted across `evaluation_run_id`, `request_id`, `request_fingerprint`, `run_id`, or claim identity produces priority-5 `protocol_error`, one operational record, and no exposed candidate or evidence receipt;
- mutation conditions outside `always` or a declared exact `data.*` result discriminant fail the combined inventory;
- validator/runner/host/adapter/Rust-native path attempting canonical evidence/verdict/gate/lifecycle/waiver/promotion authority.

A reviewer must replay the exact manifest and independently evaluate current bytes. The result table is verification evidence, not a review verdict.
