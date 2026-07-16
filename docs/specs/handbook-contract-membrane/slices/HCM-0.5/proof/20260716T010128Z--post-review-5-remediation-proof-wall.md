# HCM-0.5 Post-Review-5 Remediation Proof Wall

**Captured at:** `2026-07-16T01:01:28Z`  
**Review-5 subject fingerprint:** `sha256:8e553cab5247013b0693ff2f22bf15e88850cb5f38b448b5def3b0180ca4bf77`  
**Post-remediation canonical-plus-packet fingerprint:** `sha256:db08d016c18d06b9ddd7c6b805b38a979e19f81ebb36dfb4132c6dde0499468f`

Parent-owned lineage; not an independent review verdict. Prior dispatches and proof reports remain immutable.

## Accepted findings

| Finding | Classification | Correction | Added proof |
|---|---|---|---|
| execution record did not retain the candidate bundle required for crash-resumable admission | `cross_document_repair` | valid completion now atomically retains a bounded normalized untrusted candidate bundle/fingerprint in the execution record; later evidence append resolves an exact retained index/fingerprint after ephemeral state is discarded | original retained candidate admits after restart; post-hoc nonmember/index/fingerprint substitution rejects |
| Phase-5 exit gate allowed an ambiguous bare-`locked` evaluation path | `cross_document_repair` | Phase 5 now requires an exact active contract with a valid prior independent-lock transition and explicitly forbids bare-`locked` evaluation | cross-document active-after-lock and bare-locked refusal assertions |

## Complete replay

- Static proof exit 0: seven canonical files, 63 balanced fences, 49 duplicate-rejecting JSON/YAML examples, 10 canonical relative links.
- Exact frozen-section, author/lifecycle/genesis/fingerprint, candidate-bundle crash recovery, active-after-lock, evidence/verdict/gate, result/candidate identity and claim, total operational-record, typed launch, runtime-closure descriptor, host outcome, mutation inventory, TargetOnly/open-gate, scope, and HCM-0.6 boundary assertions passed.
- Archive boundary and negative self-test passed.
- All three handoff validator modes passed: 30 records, 85 current JSON dispatches, eight admitted legacy dispatches, exact ledger parity.
- `git diff --check` passed.
- GitNexus: seven documentation files, 48 Markdown symbols, zero affected processes, low risk.
- Runtime/schema/Rust/Cargo/catalog/HCM-0.6 work remains absent and all runtime gates remain open.

A different fresh reviewer receives a separate neutral final-proof report and current exact semantic manifest only.
