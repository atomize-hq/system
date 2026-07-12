# Proof and Regression Ledger

## Purpose

This ledger records what is actually proven, what remains only architectural intent, and which current behaviors must survive target-architecture work.

It is not a task checklist. Slice-local `tasks/todo.md` files own execution status.

## Proof levels

| Level | Question answered |
|---|---|
| `Exists` | Does an artifact, type, command, crate, or test exist? |
| `SemanticallyCorrect` | Does it encode the approved target meaning? |
| `BoundaryLanded` | Does the correct owner expose/enforce it? |
| `RealPathAdopted` | Does a real product path use that boundary? |
| `RuntimeProven` | Does required runtime/e2e/negative evidence exercise the path? |
| `ReviewClean` | Has an independent review found no remaining actionable issue? |

No lower proof level implies a higher one.

## Current proven baselines

### `PR-001` — Published owner crates

**Current evidence:**

- `handbook-engine = 0.1.1` is published;
- `handbook-flow = 0.1.1` is published;
- `handbook-pipeline = 0.1.2` is published;
- the released pipeline proof rejects path dependencies and checks registry provenance;
- prior dedicated Substrate proofs showed real published engine/flow and pipeline consumption in bounded seams.

**Classification:** `ContractCorrectAndProven` only for the exact published APIs and proof seams exercised.

**Must preserve:**

- registry-only proof;
- exact version assertion;
- real downstream seam;
- distinction between engine/flow proof and pipeline proof;
- no claim that every future membrane API is already importable.

### `PR-002` — Structured baseline input parsing

**Current evidence:** engine exposes typed YAML parse/validate models for Charter, Project Context, and Environment Inventory.

**Classification:** `UsefulPrecursor`.

**Must preserve:** deterministic typed parsing/validation value, not Markdown authority.

### `PR-003` — Deterministic Markdown rendering

**Current evidence:** engine exposes deterministic Markdown renderers for the three baseline authoring families.

**Classification:** `UsefulPrecursor`.

**Must preserve:** deterministic human review projection where still valuable.

**Must not preserve:** independently editable Markdown as canonical truth.

### `PR-004` — Trusted repo-relative artifact access

**Current evidence:** canonical loading/path contracts enforce bounded repo-relative access and reject unsafe states such as disallowed symlinks.

**Classification:** `BoundaryLanded` for current fixed artifacts.

**Must preserve:** trusted path normalization and no-follow behavior when descriptors become dynamic.

### `PR-005` — Work-level scoped rule filtering

**Current evidence:** pipeline stages carry work levels and compiler inclusion filters honor scoped blocks.

**Classification:** `UsefulPrecursor`.

**Must preserve:** the ability to select relevant rule/context sections for a declared working scope.

**Must not preserve:** L0-L3 as the final mixed taxonomy if Context Resolution replaces it.

### `PR-006` — Doctor JSON baseline

**Current evidence:** `handbook doctor --json` emits a typed serialized report.

**Classification:** `UsefulPrecursor`.

**Must preserve:** machine-readable baseline/refusal/next-action semantics.

**Gap:** JSON parity and common envelope do not yet cover all commands.

### `PR-007` — Flow resolver typed decisions

**Current evidence:** flow exposes `resolve_with_contract`, typed selection, refusal, blockers, budget outcome, and next actions.

**Classification:** `BoundaryLanded` for the current reduced request model.

**Must preserve:** typed semantic decisions and consumer-owned rendering.

**Gap:** no profile or Context Resolution input; byte budgets are not semantic projections.

## Open program proof gates

| Gate | Required proof | Current state |
|---|---|---|
| `PG-PROFILE-01` | selected profile resolves complete artifact/vocabulary/Resolution truth with deterministic fingerprint | open |
| `PG-ARTIFACT-01` | custom artifact exists without new Rust enum variant and participates in validation/doctor/flow | open |
| `PG-YAML-01` | one artifact family is canonically YAML, structurally validated, and deterministically rendered | open |
| `PG-YAML-02` | no dual editable Markdown/YAML truth remains for converted families | open |
| `PG-VOCAB-01` | lexical and structural conflation render correctly without losing stable role resolution | open |
| `PG-RES-01` | six-dimension envelope validates inheritance, authority, memory, and validation horizons | open |
| `PG-PROJ-01` | same source truth yields multiple deterministic Resolution projections with provenance | open |
| `PG-PROJ-02` | omitted required claims remain visible and cannot false-pass | open |
| `PG-SDK-01` | CLI and direct Rust consumer call the same use case and receive equivalent typed results | open |
| `PG-JSON-01` | every supported nontrivial CLI operation emits one schema-valid JSON envelope | open |
| `PG-TAURI-01` | thin Tauri command adapter serializes the same SDK DTO without CLI subprocess | open |
| `PG-CONTRACT-01` | locked contract drives claim evaluation and lifecycle-aware gate | open |
| `PG-DOCK-01` | real external process validator emits normalized evidence under declared protocol/Resolution | open |
| `PG-GATE-01` | hard failure blocks regardless of weighted score; required not-observed cannot green | open |
| `PG-SUB-CLI-01` | Substrate uses exact bundled CLI/schema in a real replaceable seam | open |
| `PG-PUBLISH-01` | new downstream-intended API passes exact crates.io external consumer proof | open |
| `PG-SUB-RUST-01` | current-tip Substrate worktree uses exact new crates.io API in a real seam | open |
| `PG-HANDOFF-01` | a blocked slice writes a valid durable handoff; orchestration produces a bounded dispatch without manual report copy | open |

## Greenfield deletion gates

Temporary scaffolding may be introduced only when a row is added here first.

| Bridge ID | Architectural purpose | Allowed lifetime | Deletion proof |
|---|---|---|---|
| none | no temporary bridge approved yet | n/a | n/a |

There is no approved user migration tool, legacy importer, dual-read mode, or compatibility profile.

## Regression rules

Every implementation slice must preserve applicable baselines:

1. trusted repo-relative/no-follow filesystem behavior;
2. deterministic structured parsing and rendering where retained;
3. typed refusal/blocker/next-action semantics;
4. published owner-crate boundaries not explicitly replaced;
5. registry-only released proof for public APIs;
6. consumer-owned product wording;
7. strict separation of docs/artifacts/evidence from contract authority;
8. no human-output parsing by machine consumers;
9. no promotion beyond evidence Resolution;
10. no implicit legacy compatibility commitment.

## Slice closeout evidence record

When a slice closes, update only the affected rows and cite:

- commit/tree state;
- exact source boundary;
- exact tests and commands;
- real-path proof when required;
- negative/fail-closed proof;
- published/downstream evidence when required;
- independent review result;
- handoff record ID.

Do not replace evidence refs with “all tests passed.”

## Control-pack proof gate

Before `HCM-0.1` may close:

- all control-pack files exist and link correctly;
- handoff schema and template are valid JSON;
- README selective-loading and authority rules are complete;
- orchestration prompt can select latest or specified handoff;
- escalation protocol distinguishes local remediation, decomposition, docs repair, broader design, external blocker, and proof gap;
- active docs point to this pack without treating archived docs as authority;
- no Rust files changed;
- `git diff --check` passes;
- independent review is requested before the pack is treated as frozen implementation authority.
