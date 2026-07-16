# HCM-0.6 Final Proof Wall

**Captured at:** `2026-07-16T15:09:03Z`  
**Baseline HEAD:** `3030b189e573d641cc5d42efa5f54ff189b4c984`  
**Consumed human-input handoff:** `20260716T030358Z--HCM-0-6--orchestration--default-set-decision-input-required`  
**Pre-decision clean subject:** `sha256:cbb2f9dfb04dfe419584b99ea49bb5900bf819e79d23984ec4633631c088f912`  
**Decision-plus-control-pack fingerprint:** `sha256:211053826f4da95c5adc86eab9f57b02b8972d8317fa1d91741085659ae66856`

This is documentation/target-data proof only. It closes `PG-DEFAULT-01` after
the exact subject receives clean independent review and the completed
two-commit closeout lands. It does not prove or publish schemas, kind/profile/
condition definitions, canonical YAML content, setup/doctor behavior, intake,
renderers, Projections, adapters, SDK/CLI/Tauri/Substrate paths, or runtime.

## Exact decision subject

The proof file excludes itself from this internal table to avoid a recursive
hash. The immutable final-review dispatch adds this proof file as the fifteenth
manifest entry and binds the resulting aggregate.

| Path | SHA-256 |
|---|---|
| `docs/specs/handbook-contract-membrane/00-README.md` | `1dc7db93e394823056826313edfc393de4042df225a20497d41a5977abbd659d` |
| `docs/specs/handbook-contract-membrane/01-target-architecture.md` | `c795e532e98b2e88bc4e5afc130c5b8ae9190b5278227a04ba033c08393b48da` |
| `docs/specs/handbook-contract-membrane/02-semantic-model.md` | `2dd65f901a7ce58cebc256a2217eb102933e3a6ccacc10fd49232a73632b9b9a` |
| `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` | `2b92b2928ed218d372607b0c95fd7fa91ae40c5e92fef3bca3125c4754d29112` |
| `docs/specs/handbook-contract-membrane/04-phase-slice-map.md` | `02f357e539abb08499668e485162ae4e3ff67886fabc6d653f8d55c8e33ebc7a` |
| `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` | `e5905f76a3964bb1eda19973baa507c93b2538cfe061972997936cb456dc2c8a` |
| `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md` | `f8f5fbd7623f099fa6360bcabef2a664ab165484c58d8ce097ef203b40cf31bd` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.6/SPEC.md` | `38b88de1596315bda2e12fc96d2068c984f6c7ef8454e5eb39a04b967761d099` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.6/decision/shipped-default-artifact-set-decision.md` | `f6e94ca2d402a7a6351cc572b10b3d3e878b2e209f16b34e81b8dbc12e1fdfda` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.6/proof/20260716T023406Z--pre-decision-proof-wall.md` | `f8f2c260b165954b944d57a9c18853bc270fc6d784106b2d3732ab73889f1c16` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.6/research/candidate-comparison.md` | `e9b7ca7f76fb3ac37a39144751a421703c64fc91d916bb27a26b3ec9abb339f4` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.6/research/shipped-default-artifact-set-research.md` | `8617f8054f92f86dcad5fad8c826bc5d54eb0d9ef45c5d7c77e8de8f8264cacc` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.6/tasks/plan.md` | `8afe9252e822c629fdaa3286d69cfc81cfdebe275ab533c6a8f532a0b0a26a8a` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.6/tasks/todo.md` | `02850983f738f12c9f8e8e7eee13fa89aa65774787bfecfdbb8a118dc808c3ed` |

Encoding: `repo-path-null-sha256-newline-v1`  
Aggregate: `sha256:211053826f4da95c5adc86eab9f57b02b8972d8317fa1d91741085659ae66856`

## Authority and dependency result

- Completed HCM-0.2, HCM-0.3, HCM-0.4, HCM-0.5, and HCM-0.8 closeouts
  were revalidated as dependency evidence. They do not select defaults.
- The pre-decision research subject was committed as
  `a89e0edb2ec70bbbab9b44f45148410b9eeee985`; the parent-owned
  `human_input` handoff and ledger closeout were committed separately as
  `3030b189e573d641cc5d42efa5f54ff189b4c984`.
- The user then resolved the product posture and every catalog/kind/role/
  instance/requiredness/condition/lifecycle/support decision one focused step at
  a time and explicitly approved the consolidated package as authoritative.
- The decision record is the sole new HCM-0.6 selection authority. Research,
  Candidate B, current enums, templates, paths, labels, and examples remain
  provenance or precedent only.
- No active packet exists. The only authorized changes are the HCM-0.6 decision,
  affected control-pack rows, proof/review evidence, and parent closeout.

## Exact approved data proof

| Surface | Proved target decision |
|---|---|
| Stable-role registry | immutable `handbook.roles.core@1.0.0` plus additive `handbook.roles.core@1.1.0`; exact derived fingerprints `sha256:7d9407b43ebdda9ac73206bdfcb0e60e3906bdba980820ed12717d63c28e5c3f` and `sha256:0c85b1b53786e7980c4fd0d7975cd9cde1a3eae2bc8daceb23be1a1731263029`; `environment_context` is an `artifact` role labeled Environment Context |
| Kind catalog | exactly six independent `@1.0.0` refs: project-authority, project-context, environment-context, work-specification, decision-record, and risk-record |
| Root instances | exactly `project_authority`, `project_context`, and `environment_context`; no work, decision, or risk root instance |
| Paths/labels | `.handbook/project/charter.yaml` / Charter; `.handbook/project/context.yaml` / Project Context; `.handbook/project/environment.yaml` / Environment Context |
| Requiredness | `always`, `always`, `conditional`; selection alone never materializes an artifact |
| Constitutional root | only `project_authority`; separate role `constitutional_authority`; capability ID `constitutional_root`; exact contract `handbook.capabilities.constitutional-root@1.0.0` |
| Environment condition | exact ref `handbook.condition.project.managed-operational-surface@1.0.0`; deterministic evidence-gated `true`, `false`, `unknown`, `unresolved`, `stale`, and `refused` outcomes; only true/false establish applicability/inapplicability |
| Work role support | exact allowlist `coordination_horizon`, `delivery_unit`, `implementation_unit`, `execution_envelope`, `atomic_action`; no automatic selection |
| Decision/risk roles | no supported stable role; v1 instance `role_ref` is explicit null; subject/scope/evidence use later typed refs |
| Lifecycle/support | six bounded reassessment responsibilities; at least one schema-backed first-party intake and fixed deterministic human-review renderer per kind before Phase 3; no root-profile Projection selection |
| Deferrals | runbook/procedure, quality-strategy, catalog, and every other unapproved artifact responsibility remain outside the closed v1 first-party catalog |

The exact registry fingerprints were recomputed from the normalized authored
registry content using sorted object keys, preserved authored role order, UTF-8
compact JSON, and SHA-256 after excluding only `registry_fingerprint`. The
1.1.0 YAML block parses to the same normalized preimage and fingerprint.

## Raw verification results

| Check | Result |
|---|---|
| Exact 14-file Markdown byte/structure scan | exit 0; UTF-8, terminal newline, no CR/NUL/absolute machine path, balanced fences, and only valid two-space Markdown hard breaks |
| Relative Markdown links | exit 0; every local link in the exact subject resolves |
| HCM-0.6 semantic assertion harness | exit 0; six exact kind refs, three exact instance IDs/paths/labels, root role/capability closure, five work roles, null decision/risk roles, six condition outcomes, deferrals, and both registry fingerprints present consistently in the decision and canonical contract tables |
| Registry fingerprint replay | exit 0; 1.0.0 and 1.1.0 expected SHA-256 values reproduce; the parsed exact 1.1.0 YAML block reproduces its declared fingerprint |
| `python3 tools/check_archive_boundary.py` | exit 0 |
| `python3 tools/check_archive_boundary.py --self-test` | exit 0; forbidden runtime fixture rejected |
| `python3 docs/specs/handbook-contract-membrane/handoffs/validate_handoffs.py` | exit 0; 32 records, 93 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity |
| `validate_handoffs.py --self-test-v1-admission` | exit 0; unknown/modified/deleted historical records and dispatches rejected |
| `validate_handoffs.py --self-test-orchestration-contract` | exit 0; delegation, lineage, stop/status, fingerprint, reviewed-baseline, and two-commit failures rejected |
| `git diff --check` | exit 0 |
| Pre-review GitNexus tracked-diff check | exit 0; 12 tracked documentation files, 39 Markdown symbols, zero affected processes, low risk; the final staged replay must additionally include the new decision/proof/dispatch files |

The 21 primary-source URLs were already checked in the immutable pre-decision
proof. The approved decision changes no sourced claim, so no fresh network
reachability is claimed here.

## Required negative and scope proof

- Kind, role, capability, instance, label, path, requiredness, condition,
  materialization, authority, schema, and user vocabulary cannot be inferred
  from one another.
- A label, role, requiredness mode, path, kind name, or profile selection cannot
  create constitutional authority without exact capability selection and full
  contract conformance.
- Missing/invalid Project Authority or Project Context fails closed; templates,
  placeholders, inferred content, external systems, or another artifact cannot
  substitute.
- Environment Context cannot satisfy its own condition. Silence/profile opt-in/
  incidental runtime files cannot produce true or false. Unknown, unresolved,
  stale, refused, and contradictory inputs cannot become false.
- Inapplicable Environment Context does not require or create a file. A later
  false result neither deletes retained bytes nor proves them current.
- No empty root Work Specification, Decision Record, or Risk Record is required
  or scaffolded; their absence asserts no absence of work, decisions, or risk.
- Work Specification existence asserts no approval/activity/implementation/
  verification/current truth; Decision Record existence causes no policy or
  work effect; Risk Record certainty cannot exceed evidence or enact posture.
- Project/Environment Context cannot absorb runbooks, exhaustive catalogs,
  constitutional policy, secrets, or volatile/inferred live state.
- Fixed renderers remain derived non-Resolution views; no capitalized Projection
  definition is selected or approximated.
- No Rust, Cargo, runtime, schema/profile asset, setup/doctor, CLI, SDK, Tauri,
  Substrate, dock, adapter, HCM-0.7, or HCM-0.9 file changed.
- Every runtime and publication proof gate remains open. Only the reviewed
  documentation decision may close `PG-DEFAULT-01`.

## Independent review requirement

A fresh isolated built-in `default` reviewer must replay the immutable final
dispatch's complete 15-entry manifest and independently assess the exact user
decision transcription, cross-document consistency, registry fingerprints,
closed-catalog/default-instance/condition/constitutional-root invariants,
deferral boundaries, proof claims, and scope. This proof table is evidence, not
a review verdict. Any valid Critical or Required finding requires bounded
remediation, complete proof rerun, and a different-fresh re-review. No reviewed
subject byte may change after `CLEAN`.
