# HCM-0.5 Post-Review-2 Remediation Proof Wall

**Captured at:** `2026-07-16T00:10:58Z`  
**Review-2 subject fingerprint:** `sha256:f6e704231abf5d98e503944d71e43fdb095ada7bc521bb4521f25b39b0d22763`  
**Post-remediation lineage fingerprint:** `sha256:255b97e9f34c54398233733c2fc83250c9f1e9c890cd243a3e9c310284319424`

This is parent-owned remediation lineage and complete proof evidence, not an independent review verdict. Prior proof/review artifacts remain immutable.

## Lineage manifest

| Path | SHA-256 |
|---|---|
| `docs/specs/handbook-contract-membrane/00-README.md` | `e200f984dfdc2c8d122cfbfd4155d4162d83f82c80aa042462ba493489b0237f` |
| `docs/specs/handbook-contract-membrane/01-target-architecture.md` | `fdb2195263c87cf5be32f228aaa54b7f0de788d60061c554c50b6cf3085e4500` |
| `docs/specs/handbook-contract-membrane/02-semantic-model.md` | `e2d7164fd724713ba5dac95695167f01e91432d4c39d16aef8fe9c4d3e0ccf1f` |
| `docs/specs/handbook-contract-membrane/03-seam-crosswalk.md` | `55eba6bd55893f2205d80123ab359fa072c08e1cdf0465d068e22d95982f2bb3` |
| `docs/specs/handbook-contract-membrane/04-phase-slice-map.md` | `20b72db680cf6ac4af2519e7a40dcca0670336eca59759c793da8fc1dde2e3cc` |
| `docs/specs/handbook-contract-membrane/05-contracts-schemas-and-gates.md` | `4aa77ab989d4e819b345ef6550c099152c2198a1a65da78ba3b54e2948a3226d` |
| `docs/specs/handbook-contract-membrane/06-proof-and-regression-ledger.md` | `2b60794c594cc4d0fb2b7b835f8686bec7453d1445d9e522a70bb08c10dbdbae` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260715T232415Z--HCM-0-5--canonical-design-freeze-landing.json` | `66429245a4504f25d0055b7b599981d54ca896ec5c1afe8dfe6e060a1c90c4f3` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260715T234316Z--HCM-0-5--fresh-final-design-freeze-review-1.json` | `c423e697f93990598fb25b64563e73cd4cb0d972470bdf7a8ab0cb16290a638e` |
| `docs/specs/handbook-contract-membrane/handoffs/dispatches/20260716T000057Z--HCM-0-5--fresh-final-design-freeze-review-2.json` | `1a99f47c0c6db02e17fa5c38110c867cb9a7264618269f474b24630b3f5b51d1` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/SPEC.md` | `9c820425077dab95b24b8c039d03a8a8dea7ee2b86f4b21f68fc75bed4811cc8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/proof/20260715T234159Z--design-freeze-proof-wall.md` | `80614e7cf94ace9487eb04ff6e6475eeb07b6d5e10e62e87699859b2d990fe8b` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/proof/20260715T235906Z--post-remediation-proof-wall.md` | `7ad001071cef3afd4192a150d530a71e0f5d7de96b9e639bd27edf58af47966c` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/proof/20260716T000012Z--final-proof-wall.md` | `6434b4f2b7c1a1d43b03da56573d8327356c89a42a792c187b67f1d58df870dc` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/plan.md` | `7b4ffdb07a7525d8bbe122d800a8672d811929ce6ea3ca51e5e55459faa755f8` |
| `docs/specs/handbook-contract-membrane/slices/HCM-0.5/tasks/todo.md` | `9bbd33adf072b206f5a4d212380f12b6a0f743c0df1a74b6258a7fb755baf7b8` |

Aggregate: `sha256:255b97e9f34c54398233733c2fc83250c9f1e9c890cd243a3e9c310284319424`

## Accepted findings and bounded correction

| Finding | Classification | Correction | Added proof |
|---|---|---|---|
| completed results did not close evaluation/request/run identity | `cross_document_repair` | request now binds parent `evaluation_run_id`, request ID/fingerprint, and dock `run_id`; every result status has one common identity closure; execution, candidate, and evidence equality-check it; one evaluation may own multiple distinct runs | completed-result/result-candidate transplantation across evaluation, request, or dock-run must be `protocol_error` with no evidence |
| three additive mutation conditions were prose outside the frozen domain | `cross_document_repair` | `contract.definition.append`, `contract.evidence.append`, and `dock.run` now use `always` after their pre-establishment admission gates; lifecycle transition retains exact `data.transition=applied` | combined inventory parses every condition and accepts only `always` or a declared exact `data.*` discriminant |

## Complete replay result

- Parent static proof: exit 0; seven canonical files; 61 balanced fences; 49 duplicate-rejecting JSON/YAML parses; 10 relative links.
- Exact lifecycle, 3x7 gate, 12-operation/four-mutator, eight-host-outcome, typed-launch, common-result-identity, candidate-transplantation, condition-domain, TargetOnly/open-gate, frozen-section, scope, and no-HCM-0.6 assertions passed.
- Archive boundary and negative self-test passed.
- All three handoff validator modes passed with 30 records, 82 current JSON dispatches, eight admitted legacy dispatches, and exact ledger parity.
- `git diff --check` passed.
- GitNexus unstaged detection: seven documentation files, 48 Markdown symbols, zero affected processes, low risk.
- No runtime/schema/Rust/Cargo/catalog-leaf/HCM-0.6 proof is claimed; all three runtime gates remain open.

A different fresh reviewer must use a separately assembled neutral manifest and independently review current bytes.
