# HCM-0.6 Post-Review-2 Remediation Proof

**Captured at:** `2026-07-16T15:20:27Z`  
**Baseline HEAD:** `3030b189e573d641cc5d42efa5f54ff189b4c984`  
**Finding dispatch:** `20260716T150927Z--HCM-0-6--fresh-final-default-decision-review-2`  
**Reviewed subject:** `sha256:4616ada7d25d7ff4e9137b6725c41c2ce58c0d30ee6d79a0b6e266626b168a33`  
**Remediated decision subject:** `sha256:4cb4c57d7e9b8410efd23da96ef24f38cafbac25dc9d94ffce2e5e7c4c8a8d53`

Final Review 2 returned one Required finding and no Critical finding. The parent
accepted `HCM-0.6-FR2-001`: two live canonical `05` sentences still described
the shipped-default decision as reserved for future HCM-0.6 work.

The parent changed only those two stale statements and the directly coupled
future-tense Charter-default sentence. The illustrative profile and intake
contract now explicitly select nothing and point to the completed HCM-0.6
decision/tables as the sole shipped-set authority. The Charter-default sentence
now records the approved target selection without claiming that the subordinate
intake definition is approved. No registry, kind, instance, requiredness,
condition, lifecycle, support, or deferral value changed.

## Rerun result

- a fail-closed scan over live canonical `00`-`05` found no pending, reserved,
  or unresolved HCM-0.6 shipped-default statement;
- the 14-file decision subject passed UTF-8, line-ending, NUL, fence, portable-
  path, and relative-link checks;
- all exact kind/instance/role/capability/path/requiredness/condition/lifecycle/
  support/deferral assertions passed;
- both role-registry fingerprints reproduced byte-for-semantic-byte;
- archive boundary plus its negative self-test passed;
- normal handoff validation and both negative self-tests passed with 32 records,
  94 current JSON dispatches, eight admitted legacy dispatches, and exact ledger
  parity;
- `git diff --check` passed; and
- the remediated non-recursive subject aggregate reproduced as
  `sha256:4cb4c57d7e9b8410efd23da96ef24f38cafbac25dc9d94ffce2e5e7c4c8a8d53`.

This is successful typed parent remediation. It is not an independent verdict.
A different-fresh reviewer must receive a newly fingerprinted final subject
without prior finding/remediation discussion.
