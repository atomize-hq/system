# HCM-0.5 Post-Review-6 Remediation Proof Wall

**Captured at:** `2026-07-16T01:18:36Z`  
**Review-6 subject fingerprint:** `sha256:03698b8ad67e9b699c4e9b8b588a84c6b8e7e0639629e342afa164d190e686ad`  
**Post-remediation canonical-plus-packet fingerprint:** `sha256:6eb1217e28ef580eaeaf772cafc01372817ed7ad471d747c4231e5e4dba8dd94`

Parent-owned lineage; not an independent review verdict. Prior dispatches and proof reports remain immutable.

## Accepted finding

| Finding | Classification | Correction | Added proof |
|---|---|---|---|
| crash-resumable admission lost the normalized request/result basis needed for independent replay | `cross_document_repair` | every execution record retains a fingerprinted complete normalized request-admission basis; valid results retain a fingerprinted status/identity/claim-partition/actual-observation basis; the candidate bundle binds both basis fingerprints; evidence append reloads and recomputes all three | after discarding original request/full result/process/workspace/caller state, retained original candidate admits while requested-but-unobserved claim, changed case/Resolution, reordered/substituted claim, and post-hoc nonmember reject |

## Complete replay

- Static proof exit 0: seven canonical files, 63 balanced fences, 49 duplicate-rejecting JSON/YAML examples, 10 canonical relative links.
- Exact frozen-section, author/lifecycle/genesis/fingerprint, durable request/result/candidate recovery, active-after-lock, evidence/verdict/gate, result/candidate identity and claim, total operational-record, typed launch, runtime-closure descriptor, host outcome, mutation inventory, TargetOnly/open-gate, scope, and HCM-0.6 boundary assertions passed.
- Archive boundary and negative self-test passed.
- All three handoff validator modes passed: 30 records, 86 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity.
- `git diff --check` passed.
- GitNexus: seven documentation files, 48 Markdown symbols, zero affected processes, low risk.
- Runtime/schema/Rust/Cargo/catalog/HCM-0.6 work remains absent and all runtime gates remain open.

A different fresh reviewer receives a separate neutral final-proof report and current exact semantic manifest only.
