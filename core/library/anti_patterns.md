# Anti-Patterns (Library)

Avoid these failure modes.

## Fake evidence
- Claiming “tests pass” without actually running them (or without evidence).

## Paper-green / scope-lawyering
- Disabling gates to get green.
- Changing requirements after the fact to match implementation.

## Invented facts
- Assuming environments, users, production status, or stack commands not provided by Charter/Context/Profile.

## Unscoped refactors
- Mixing refactors with feature work without explicit scope.
