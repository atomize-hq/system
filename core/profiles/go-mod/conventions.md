# Profile conventions: Go (modules)

## Style
- Format: gofmt
- Lint: go vet (staticcheck optional)
- Keep packages small and cohesive.

## Testing
- `go test ./...` must pass.
- Prefer table-driven tests.

## Security
- `govulncheck` when available.

Core prompts should reference command keys in `commands.yaml`, not hardcode commands.
