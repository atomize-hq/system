# HCM-1.4 Planning Proof Wall

## Classification

- phase: `HCM-1`
- slice: `HCM-1.4`
- packet: none at entry; this session creates the packet
- work class: documentation/specification/planning/proof/review/closeout only
- implementation authority: not active
- product adoption: not active
- proof timestamp: `20260717T192217Z`
- planning baseline: `8e3af4694aa0ae30c033c54cd4d13628ba0662e8`

This proof wall binds the candidate planning subject before independent review.
The immutable review dispatch carries the sorted path/hash manifest and exact
aggregate fingerprint. Final `CLEAN`, the primary reviewed-subject commit, and
the separate parent closeout/ledger commit are recorded outside these reviewed
bytes so later evidence does not mutate the subject after review.

## Entry and dependency proof

At entry:

- branch was `feat/handbook-contract-membrane`;
- HEAD was `8e3af4694aa0ae30c033c54cd4d13628ba0662e8`;
- `git status --short` was empty;
- runtime parameters were exactly `PHASE_ID=HCM-1`, `SLICE_ID=HCM-1.4`,
  `ACTIVE_PACKET=none`, and
  `HANDOFF_SELECTOR=20260717T183202Z--HCM-1-3--orchestration--artifact-registry-landed`;
- the selected v1.2 record was completed with no blockers or escalations; and
- record and ledger bound HCM-1.3 implementation commit
  `8194f9f4534b2d27e1077ffab2c89d12da5ff456` and reviewed subject fingerprint
  `sha256:56d8559ffcecf91935a2ce1140f582949ee87408f0291b75e56c17b55a6f8fee`.

Ancestry and transition order were verified as:

```text
0bc51a9cc282581143a5b21f50162456aa32154c  HCM-1.1 primary
  -> 832716a66241bdcf86e2a82ffb3ae72680a7c2cd  HCM-1.2 primary
  -> 8194f9f4534b2d27e1077ffab2c89d12da5ff456  HCM-1.3 primary
  -> 8e3af4694aa0ae30c033c54cd4d13628ba0662e8  HCM-1.3 closeout/planning entry
```

The completed HCM-1.3 record has `resume.execution_target: none`; it is valid
dependency/transition evidence for this explicitly selected HCM-1.4 planning
run, not self-executing implementation authority.

## Handoff and ledger validation

All modes passed before dispatch:

```text
python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py
PASS: 3 record schemas, 2 internal-dispatch schemas, 2 templates, 39 records,
148 current internal dispatches, 8 admitted legacy dispatches, 39 ledger entries

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py \
  --self-test-v1-admission
PASS: unknown/modified/deleted history refuses and exact ledger rebuilds

python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py \
  --self-test-orchestration-contract
PASS: invalid lineage/status/resume/review/two-commit cases refuse and valid
direct/chained remediation plus reviewed-baseline closeout cases pass
```

## Authority and skill proof

The parent re-read `AGENTS.md`, `docs/START_HERE.md`, runner `07`, protocol
`08`, active control pack `00` through `06`, the exact selected HCM-1.3 record
and ledger entry, HCM-1.1 through HCM-1.3 packets/closeouts, current setup and
doctor source/tests, selected-profile/registry owners, definition assets, and
package manifests before drafting.

The required local skill files were re-read in runner order, including
`using-agent-skills`, `context-engineering`, `source-driven-development`,
`spec-driven-development`, `planning-and-task-breakdown`,
`api-and-interface-design`, `security-and-hardening`,
`incremental-implementation`, `test-driven-development`,
`debugging-and-error-recovery`, `documentation-and-adrs`,
`code-review-and-quality`, and `git-workflow-and-versioning`.

## Live-source capsule

Live source establishes:

- setup currently plans fixed `CanonicalArtifactKind` descriptors in
  `CANONICAL_ARTIFACT_ORDER` and may write Markdown starter templates;
- doctor currently reports the fixed canonical artifact baseline and fixed
  C-03/C-04 checks;
- HCM-1.3 exposes selected profile, kinds, instances, roles, capabilities,
  requiredness, condition refs, trusted paths, and structural validation
  through `ResolvedArtifactRegistry`;
- HCM-1.2 owns the exact six condition outcomes and fingerprints but has no
  operation-time evaluator;
- repository silence cannot establish false and Environment Context cannot
  prove its own applicability;
- engine owns pure profile/condition decisions, compiler composes domain
  operations transitionally, and CLI owns wording, arguments, repo discovery,
  JSON selection, and exit behavior; and
- HCM-2, not HCM-1.4, owns canonical YAML content authority, writes, semantic
  loading, intake/promotion, and renderer-derived views.

The packet therefore freezes one profile -> one registry -> one typed decision
closure, structural read-only readiness, setup no-write/refusal behavior,
doctor machine truth, custom-profile proof, and an explicit stop before HCM-2.

## GitNexus and blast-radius proof

`npx gitnexus analyze --index-only` succeeded before drafting: incremental
analysis observed 4 changed and 5 added files plus 11 importers, producing
12,938 nodes, 23,656 edges, 346 clusters, and 300 flows. Free-form search was
degraded by missing FTS, so context/Cypher and live source were used without
claiming semantic-query completeness.

Planning upstream impact results:

| Symbol | Risk | Impact | Packet response |
|---|---:|---:|---|
| compiler `run_setup` | **HIGH** | 39 impacted, 15 direct, setup CLI flow | warning issued; future test-first change preserves complete preflight and atomic reset |
| `build_setup_execution_plan` | MEDIUM | 41 impacted, 2 direct | fixed artifact planning replaced only inside frozen setup contract |
| compiler `doctor` | MEDIUM | 12 impacted, 9 direct | typed report change with focused/compiler/CLI/workspace wall |
| `doctor_from_artifacts` | LOW | 13 impacted, 1 direct | no adapter or fixed fallback survives setup/doctor adoption |
| CLI setup `run` | LOW | no indexed upstream nodes | shell-only responsibility |
| CLI doctor `run` | LOW | no indexed upstream nodes | shell-only responsibility |

No symbol is edited in this planning session. Future implementation must rerun
impact for every existing symbol before edit, warn on HIGH/CRITICAL results,
and stop rather than widening the packet.

## Packet and scope proof

The candidate subject creates:

- `slices/HCM-1.4/SPEC.md`;
- `slices/HCM-1.4/tasks/plan.md`;
- `slices/HCM-1.4/tasks/todo.md`; and
- this proof wall.

It updates only `00-README.md`, `04-phase-slice-map.md`, and
`06-proof-and-regression-ledger.md` to name the landed dependency, packet
authorization boundary, and future implementation proof gate. The todo has no
checked item. `git diff --check`, packet-presence, unchecked-todo, and
documentation-only path checks passed.

The SPEC freezes exact existing/new-file allowlists, one public decision and
inspection contract, condition truth, setup/doctor adoption semantics, TDD
increments, security/determinism matrices, full regression/package proof,
classification ceiling, exit gates, and stop conditions. The plan preserves
RED-before-GREEN packet order. The todo remains entirely future work.

No Rust, Cargo, definition asset, schema, product runtime, CLI grammar,
canonical content, authoring, flow, pipeline, SDK, Tauri, Substrate, adapter,
contract, dock, HCM-2, or unrelated cleanup is performed here.

## Pre-review verdict

| Gate | Result |
|---|---|
| exact selector and dependency proof | PASS |
| handoff/ledger validation | PASS |
| live-source grounding | PASS |
| GitNexus refresh and risk inventory | PASS |
| packet presence and unchecked todo | PASS |
| cross-document planning consistency | PASS |
| docs-only scope and whitespace | PASS |
| implementation separability | PASS |
| independent exact-subject review | PENDING immutable dispatch |

The parent must not claim packet approval until a fresh isolated read-only
built-in `default` reviewer returns `CLEAN` over the exact manifest. Any valid
finding requires bounded parent remediation, a new proof/fingerprint/dispatch,
full relevant checks, and a different fresh reviewer.
