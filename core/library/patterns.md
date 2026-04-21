# Canonical Patterns (Library)

These are reusable patterns stages may reference.

## Pattern: Boundary validation
- Validate/parses external inputs at entrypoints (API/CLI/files/webhooks).
- Convert to internal typed structures.
- Fail fast with clear errors.

## Pattern: Capability flags
- When introducing risky changes, use feature flags or staged rollout (Charter-dependent).

## Pattern: Evidence-first gates
- Run gates via profile commands.
- Record evidence in execution/quality gate reports.

## Pattern: Inventory updates
- Any change to env vars/services/ports must update `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`.
