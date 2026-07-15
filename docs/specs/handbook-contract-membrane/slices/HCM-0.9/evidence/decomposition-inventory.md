# HCM-0.9 mechanical decomposition inventory

**Status:** planning authority; mechanical topology only

This inventory is derived from the frozen Git blob:

- commit: `214a5b8eb182fce74478df49d4f55d226d65fdf5`;
- path: `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md`;
- file: 343,081 bytes, 3,757 lines,
  `sha256:c7f61db209a81ba20690f365b4069dd01f11e395335bfa10d2ce21143cc2985d`;
- body after exact H1 plus following blank line: 343,048 bytes,
  `sha256:487d698979131bbea319c394d39c27dbd270dc5953f714f30edfe8b7f9ae9202`;
- headings: 48 H2 and 22 H3.

It assigns every frozen payload byte to one of eight contiguous spans. It does
not inventory semantic dependencies and defines no automatic routing behavior.

## Ordered spans

| Order | Leaf | Exact leaf H1 | Frozen lines | Lines | Payload SHA-256 | H2 | H3 |
|---:|---|---|---:|---:|---|---:|---:|
| 1 | `contracts/01-profile-and-artifact-foundations.md` | `# Profile and Artifact Foundations` | 3-533 | 531 | `7196f3cc3c08eb02824ba502bcefcdb7a5437434d20e493bc0314e84431d8e72` | 7 | 3 |
| 2 | `contracts/02-intake-canonical-and-validation.md` | `# Intake, Canonical Artifacts, and Validation` | 534-1317 | 784 | `c9ba7c0c6d4a6737c65c273447d9b2e2b44040e3599600805e010c1d2812a988` | 4 | 0 |
| 3 | `contracts/03-context-resolution-and-projection.md` | `# Vocabulary, Context Resolution, and Projection` | 1318-1932 | 615 | `20e14ffc03548f41c4a89253514e93218c638bf9a6c37bfaa0d70207fac4aa63` | 9 | 0 |
| 4 | `contracts/04-snapshot-memory.md` | `# Snapshot Memory` | 1933-2588 | 656 | `6d5b87eb8715c486937631844f5776c6febe88586b4941bacf1a61cc27cd74ff` | 6 | 5 |
| 5 | `contracts/05-posture-and-synthesis.md` | `# Project Posture and Optional Synthesis` | 2589-2889 | 301 | `b6b0380178c72c88627ea06771ad0f6b2200a36689c494c68361b15da28b819f` | 2 | 6 |
| 6 | `contracts/06-orchestration-and-sdk.md` | `# Development Orchestration and SDK` | 2890-3343 | 454 | `efd41a7e48311f51a02aa4d3d1d42077826ecacdfabf39452ee5337c24da5dae` | 5 | 4 |
| 7 | `contracts/07-transport-adapters-and-publication.md` | `# Transport, Adapters, and Publication` | 3344-3617 | 274 | `dc3c1944b93b82021b0e7a84d985120c5dd3f59344656c077349f8b683ef92d5` | 7 | 2 |
| 8 | `contracts/08-contract-evidence-docks-and-gates.md` | `# Contracts, Evidence, Docks, and Gates` | 3618-3757 | 140 | `08253f11471319f1e519d30c7955dd42ce45a9b5e7e1733ec11101321e86e491` | 8 | 2 |

Totals: 3,755 body lines, 48 H2, and 22 H3.

## Exact H2 ownership

### Leaf 1 — profile and artifact foundations

1. `Status`
2. `Schema policy`
3. `Instance profile contract`
4. `Schema registry entry contract`
5. `Stable-role registry contract`
6. `Artifact kind definition contract`
7. `Artifact instance descriptor contract`

### Leaf 2 — intake, canonical artifacts, and validation

1. `Artifact intake definition contract`
2. `Intake record and artifact candidate contracts`
3. `Charter intake and canonical contract`
4. `Artifact validation layers`
### Leaf 3 — vocabulary, Context Resolution, and Projection

1. `Vocabulary contract`
2. `Context Resolution stack definition`
3. `Context Resolution envelope`
4. `Resolution escalation and memory promotion`
5. `Projection disclosure policy`
6. `Projection support evaluator definition`
7. `Projection definition`
8. `Projection request`
9. `Projection result`

### Leaf 4 — Snapshot Memory

1. `Snapshot capture policy`
2. `Context Memory Snapshot`
3. `Snapshot delta`
4. `Snapshot-grounding Projection definition`
5. `Snapshot projection request/result`
6. `Snapshot redaction and retention`

### Leaf 5 — posture and synthesis

1. `Project posture kernel and recommendation contracts`
2. `Optional synthesis-candidate contract`

### Leaf 6 — orchestration and SDK

1. `Development orchestration contracts`
2. `HCM-0.4 crate ownership and dependency contract`
3. `SDK ordinary-use-case contract`
4. `Operation definition contract`
5. `Capability bootstrap and catalog discovery`

### Leaf 7 — transport, adapters, and publication

1. `Transport request contract`
2. `Transport response and expected-outcome contract`
3. `DTO and JSON Schema generation contract`
4. `CLI JSON contract`
5. `Tauri adapter contract`
6. `Substrate integration contracts`
7. `Published Rust API proof plan`

### Leaf 8 — contract membrane, docks, and gates

1. `Contract record`
2. `Evidence record`
3. `Verdict contract`
4. `Gate contract`
5. `Dock capability manifest`
6. `Dock request/result`
7. `Public API proof gates`
8. `Schema compatibility posture`

## Mechanical derivation requirements

The future verifier derives rather than trusts this table and fails unless:

1. source lines 3-3757 are covered exactly once, contiguously, and in order;
2. every boundary begins at an H2 outside a fence;
3. exact leaf H1 bytes, one following blank line, per-span line count, payload
   digest, H2 count, H3 count, and H2 sequence match;
4. totals equal the frozen body and 48 H2/22 H3 inventory;
5. adding only the inventory's exact leaf H1 plus one blank line, then stripping
   those exact bytes, leaves each payload unchanged; a missing, changed,
   duplicate, or additional H1/scaffold byte fails;
6. concatenating all eight stripped payloads equals the frozen body bytes.

No semantic-reference scan, dependency graph, trigger identity, operation
fixture, co-activation table, fanout metric, or automatic leaf selection is
required or permitted.
