# Quality Gates Specification

## Mandatory Gates
- `cargo test --workspace`
- focused proof-corpus compiler and CLI suites for foundation inputs
- docs/help parity checks for the shipped compile surface

## Gate Policy
- No compile success claim unless both payload and explain scenarios are covered.
- Refusal regressions for stale or inactive route basis are blocking failures.
- Fixture drift between live repo truth and proof corpus is a blocking failure.

## Acceptance Signal
`M2` is ready only when the live pipeline declaration, proof corpus, tests, and docs all agree
on the same bounded compile contract.
