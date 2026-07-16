# HCM-0.5 Post-Review-4 Remediation Proof Wall

**Captured at:** `2026-07-16T00:43:28Z`  
**Review-4 subject fingerprint:** `sha256:fb07d7e8ebe38825ec160d13f246ba3a6ee67d6936e40458a497fa5df3a2d9ec`  
**Post-remediation canonical-plus-packet fingerprint:** `sha256:19adc667c36300c9d95dd653cbd7c0b97e8e9d89ef603c0816720017a3be0ee1`

Parent-owned lineage; not an independent review verdict. Prior dispatches and proof reports remain immutable.

## Accepted findings

| Finding | Classification | Correction | Added proof |
|---|---|---|---|
| lifecycle author authority was not fingerprint-bound | `cross_document_repair` | the immutable definition now binds authenticated author/admission authority; draft transitions equality-check that binding and lock authority must be distinct | non-author draft transition, self-lock, and author-binding substitution refuse |
| later lifecycle fingerprints lacked a non-circular canonical preimage | `cross_document_repair` | exact RFC 8785 transition and resulting-lifecycle preimages, exclusions, and derivation order now chain the immediate prior lifecycle identity | multi-transition leaf/prior/order substitutions reject deterministically |
| `DockExecutionRecord` was not total for absent or invalid results | `cross_document_repair` | a closed outcome-discriminated shape always binds expected identity and distinguishes `not_created`/`created` process plus `absent`/`invalid`/`valid` result observation | spawn failure, malformed output, and identity transplantation each produce one schema-valid operational record and no evidence |
| nested candidate claim identity was omitted from pre-completion validation | `cross_document_repair` | every candidate fingerprint-binds a non-empty no-duplicate claim-ID subset of the request and result-observed partition; claim mismatch selects priority 5 | independent claim substitution produces one `protocol_error` operational record and zero exposed candidates/evidence receipts |

## Complete replay

- Static proof exit 0: seven canonical files, 63 balanced fences, 49 duplicate-rejecting JSON/YAML examples, 10 relative links.
- Exact frozen-section, author/lifecycle/genesis/fingerprint, evidence/verdict/gate, result/candidate identity and claim, total operational-record, typed launch, runtime-closure descriptor, host outcome, mutation inventory, TargetOnly/open-gate, scope, and HCM-0.6 boundary assertions passed.
- Archive boundary and negative self-test passed.
- All three handoff validator modes passed: 30 records, 84 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity.
- `git diff --check` passed.
- GitNexus: seven documentation files, 48 Markdown symbols, zero affected processes, low risk.
- Runtime/schema/Rust/Cargo/catalog/HCM-0.6 work remains absent and all runtime gates remain open.

A different fresh reviewer receives a separate neutral final-proof report and current exact semantic manifest only.
