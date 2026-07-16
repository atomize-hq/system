# HCM-0.5 Post-Review-3 Remediation Proof Wall

**Captured at:** `2026-07-16T00:25:23Z`  
**Review-3 subject fingerprint:** `sha256:5ace2cfb049662456e1a3644498ef75189c918b2b26afe4e04b73f2a9dd9553d`  
**Post-remediation lineage fingerprint:** `sha256:269710234c8805742d684b6866a4c624df9aa10bed2ef5a4597f7d518b383cfd`

Parent-owned lineage; not an independent review verdict. Prior dispatches and proof reports remain immutable.

## Accepted findings

| Finding | Classification | Correction | Added proof |
|---|---|---|---|
| nested candidate transplantation could survive as host `completed` | `cross_document_repair` | every completed-result candidate repeats the full evaluation/request/run and producer closure; host validates all nested identities before outcome selection; any mismatch is priority-5 `protocol_error`; membrane admission repeats the check only as defense in depth | independent mutation of each nested identity field must create one protocol-error operational record and zero evidence receipts |
| first draft lifecycle compare basis undefined | `cross_document_repair` | added exact timestamp-free `draft_genesis_lifecycle_fingerprint` preimage; null prior transition is legal only for the first transition with matching genesis; all later transitions require non-null immediate prior identity | valid first edge, wrong genesis, fake first prior, and null later prior assertions |
| runtime closure fingerprint lacked a preimage | `cross_document_repair` | added closed `RuntimeDependencyClosure` schema/ref, member roles/order, dependency edges/order/kinds, bundle-only resolution policy, platform ABI policy, canonical fingerprint, host fixed-point recomputation, and change rules | member/edge/path/role/mode/digest/provider/order/policy/ABI add-remove-substitute and deterministic fingerprint assertions |

## Complete replay

- Static proof exit 0: seven canonical files, 62 balanced fences, 49 duplicate-rejecting JSON/YAML examples, 10 relative links.
- Exact frozen-section, lifecycle/genesis, evidence/verdict/gate, result/candidate identity, typed launch, runtime-closure descriptor, host outcome, mutation inventory, TargetOnly/open-gate, scope, and HCM-0.6 boundary assertions passed.
- Archive boundary and negative self-test passed.
- All three handoff validator modes passed: 30 records, 83 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity.
- `git diff --check` passed.
- GitNexus: seven documentation files, 48 Markdown symbols, zero affected processes, low risk.
- Runtime/schema/Rust/Cargo/catalog/HCM-0.6 work remains absent and all runtime gates remain open.

A different fresh reviewer receives a separate neutral final-proof report and current exact semantic manifest only.
