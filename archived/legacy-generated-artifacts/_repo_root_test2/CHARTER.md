# Engineering Charter — InputsFlow

This charter defines how we make engineering tradeoffs (quality vs speed, testing, security, operability). Use it to set defaults and record justified exceptions.

<!-- generated from CHARTER_INPUTS.yaml -->

## Rubric (1–5)
1. Exploratory
2. Prototype
3. Product
4. Production
5. Hardened

## Anti-bikeshedding rules
- Baseline level applies unless overridden.
- Only specify deltas.

## Baseline
- Level: 3 (balanced quality vs speed)

## Project classification + implications
- Classification: Greenfield
- Planning defaults: Back-compat: not required; Migration: not required; Rollout: lightweight; Deprecation: not required yet; Observability: standard

## Operational reality
Nothing is in production today. Internal users. No external contracts to preserve.

## Domains/areas
None.

## Dimensions
(omitted in this fixture)

## Exceptions
Record exceptions in `CHARTER.md#exceptions` with: what/why/scope/risk/owner/expiry.

## Debt tracking
Track debt in issues with label `debt`; review monthly.

## Decision records
Use ADRs in `docs/decisions`.
