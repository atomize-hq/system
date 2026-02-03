# Integration Policy

Defines when “integration” is required vs when a library/module-only change is acceptable.

## Default
- If a slice creates a new capability that must be used by existing entrypoints (CLI/API/service), integration is usually required.
- If a slice is explicitly defined as “library only” (pure module) it may omit wiring—**but must document how it will be wired later**.

## Signals integration is required
- There is an existing entrypoint (CLI command, API route, job runner) that should call the new code.
- A configuration change must be plumbed into runtime.
- A user-visible behavior is specified.

## When library-only is acceptable
- A preparatory slice that adds internal helpers/types needed by later slices.
- The slice explicitly states “no wiring in this slice” and includes:
  - integration point(s)
  - how to test the library behavior now
  - what later slice will wire it in

## Evidence
If integration is required, quality gates must include at least one test that covers the integration boundary.
