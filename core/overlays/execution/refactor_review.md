# Overlay: Refactor Review (Scope Guardrails)

Use when a slice or PR starts to drift into refactoring.

## Rule of thumb
Refactors are only allowed when they are:
- required to implement the feature safely, or
- explicitly called out in scope.

## Checklist
- Does this change alter public behavior/contracts?
- Does it introduce new dependencies or services?
- Does it make future changes easier (with a concrete reason), or is it stylistic?

## Safety
- Keep refactors behavior-preserving.
- Add/adjust tests before refactor if needed (red→green→refactor).
