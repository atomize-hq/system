# Legacy Mechanism: Environment Inventory (`ENVIRONMENT_INVENTORY.md`)

## What it is

This document describes the legacy Python-harness mechanism that used a repo-root environment inventory as its store-of-record.

In the current reduced-v1 `M8` product story, the shipped baseline canonical path is `.system/environment_inventory/ENVIRONMENT_INVENTORY.md`.

The legacy mechanism used `ENVIRONMENT_INVENTORY.md` as the historical store of record for:

- environment variables
- external services and endpoints
- ports
- runtime assumptions

It exists to prevent hidden environment drift and to make execution reproducible.

## Why it exists

In practice, env vars and runtime assumptions are the #1 source of:
- “works on my machine”
- broken deploys
- invisible dependencies
- accidental security leaks

Making env changes update the inventory is a simple, high-leverage discipline.

## Legacy repo-root location

The legacy harness used a stable repo-root location:

- `${repo_root}/ENVIRONMENT_INVENTORY.md`

The legacy harness could also keep an artifact copy for stage linkage:

- `artifacts/foundation/ENVIRONMENT_INVENTORY.md`

## How it works today

- Rules + runner guidance should require:
  - update the legacy repo-root inventory in the same change
  - keep the artifacts copy in sync when applicable

## Creating/Updating the inventory

- Add new env vars with:
  - name
  - purpose
  - default value (if safe)
  - required/optional
  - where used
  - security notes (secret/non-secret)
- Add services with:
  - name
  - endpoint / port
  - auth expectations
  - local dev notes

## Do / Don’t

✅ Do:
- update inventory whenever you introduce/change env vars or services
- mark secrets as secrets (never include actual secret values)

❌ Don’t:
- let env vars exist only in code or only in deployment configs
- store credentials in the inventory
