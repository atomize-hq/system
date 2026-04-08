# Seam Map - Reduced V1 Rust-First CLI Cutover

This seam map uses a workflow-first axis. It treats the reduced v1 as a trust pipeline:
establish the supported repo boundary, stand up the Rust command surface, define trusted inputs,
resolve the planning packet, render proof surfaces, fence the execution demo honestly, and then lock
the result down with validation and docs cutover.

## Horizon policy

- Active seam: `SEAM-2` because milestone `M2` publishes the CLI and workspace contract that downstream resolver seams consume.
- Next seam: `SEAM-3` because milestone `M3` defines the canonical artifact manifest and freshness contract required by the resolver.
- Future seams: `SEAM-4` through `SEAM-7`.
- Only the active seam is eligible for authoritative deep planning by default.
- The next seam may later receive seam-local review and slices, but only provisional deeper planning until its basis is revalidated against the landed upstream closeout it depends on.
- Seam-local slices exist for seams in the forward window; future seams remain seam briefs until promoted.

## Seam summary

| Seam | Name | Type | Horizon | Primary value | Key contracts | Primary touch surface |
| --- | --- | --- | --- | --- | --- | --- |
| `SEAM-1` | Approved Surface and Legacy Freeze | `platform` | `future` | Makes the repo tell one supported Rust-first story and freezes Python as reference-only | `C-01` | root docs, `tools/`, future `archived/python-harness/`, repo layout |
| `SEAM-2` | Rust Workspace and CLI Skeleton | `platform` | `active` | Creates the Rust workspace, crate split, and verb surface downstream seams rely on | `C-02` | `Cargo.toml`, `crates/cli`, `crates/compiler`, CLI help |
| `SEAM-3` | Canonical Artifact Manifest Contract | `integration` | `next` | Defines trusted `.system/` inputs, inherited posture dependencies, freshness, and override rationale | `C-03` | ingest types, manifest schema, canonical artifact rules |
| `SEAM-4` | Planning Packet Resolver and Doctor | `capability` | `future` | Produces deterministic planning packets, budget decisions, refusal semantics, and blocker diagnosis | `C-04` | resolver core, decision log, refusal policy, `doctor` |
| `SEAM-5` | Renderer and Proof Surfaces | `capability` | `future` | Turns one typed resolver result into human and machine proof surfaces without changing selection logic | `C-05` | markdown/JSON/inspect renderers, output ordering, proof copy |
| `SEAM-6` | Fixture Execution Demo Boundary | `risk` | `future` | Demonstrates execution packets honestly while refusing unsupported live slice execution | `C-06` | demo lineage, refusal messaging, fixture path |
| `SEAM-7` | Conformance Rails and Docs Cutover | `conformance` | `future` | Locks in behavior with tests, CI, install smoke, docs parity, and final Python cutover rules | `C-07` | `tests/`, CI config, README/help/docs, cutover validation |

## Why these seams are decomposable

- `SEAM-1` is bounded to repo support truth and the root/runtime boundary; it does not own Rust implementation logic.
- `SEAM-2` is bounded to workspace and command-surface scaffolding; it does not yet decide packet ingest or resolver behavior.
- `SEAM-3` owns the artifact and freshness contract rather than packet selection or rendering.
- `SEAM-4` owns resolver behavior and recovery semantics but not view-layer formatting.
- `SEAM-5` owns presentation and proof ordering while consuming the typed resolver result from `SEAM-4`.
- `SEAM-6` isolates the risky “execution demo vs live capability” boundary instead of burying it in the main resolver seam.
- `SEAM-7` is a genuine conformance seam because its remaining value is cross-seam hardening, regression protection, and doc/help alignment.

## Workstream posture

- `WS-Control`: `SEAM-1` -> `SEAM-5`
- `WS-Demo-Boundary`: `SEAM-6`
- `WS-Conformance`: `SEAM-7`

The workstreams are conflict-safe only after the contract threads from the control-plane seams are published. Until then, the critical path remains serial through `SEAM-1` and `SEAM-2`.
