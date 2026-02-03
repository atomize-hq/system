# Profile conventions: Node.js (pnpm)

## Assumptions
This profile expects scripts in `package.json`:
- `format`, `format:check`
- `lint`
- `test` (and optionally `test:coverage`)
- `typecheck` (if TypeScript is used)

If a repo does not have these scripts, the Foundation Pack should specify the canonical scripts to add.

## Style / Quality
- Formatting: Prettier (via `pnpm format`)
- Linting: ESLint (via `pnpm lint`)
- Typing: TypeScript `tsc --noEmit` (via `pnpm typecheck`) when applicable
- Tests: Jest/Vitest/etc (via `pnpm test`)

## Layout
- Prefer `src/` for code.
- Tests may be co-located or under `test/` / `tests/`.

Core prompts should reference command keys in `commands.yaml`, not hardcode commands.
